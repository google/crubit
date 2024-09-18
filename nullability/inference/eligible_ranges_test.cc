// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "nullability/inference/augmented_test_inputs.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/pragma.h"
#include "nullability/proto_matchers.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/raw_ostream.h"
#include "llvm/Testing/Annotations/Annotations.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"  // IWYU pragma: keep
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"
#include "third_party/protobuf/message.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::classTemplateDecl;
using ::clang::ast_matchers::classTemplateSpecializationDecl;
using ::clang::ast_matchers::fieldDecl;
using ::clang::ast_matchers::findAll;
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::functionTemplateDecl;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::selectFirst;
using ::clang::ast_matchers::varDecl;
using ::llvm::Annotations;
using ::testing::AllOf;
using ::testing::Contains;
using ::testing::ExplainMatchResult;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::Optional;
using ::testing::Pointwise;
using ::testing::UnorderedElementsAre;
using ::testing::UnorderedElementsAreArray;

test::EnableSmartPointers Enable;

constexpr char MainFileName[] = "input.cc";

MATCHER_P2(SlotRange, SlotID, Range,
           absl::StrCat("is a SlotRange with ID ", SlotID,
                        " and range equivalent to [", Range.Begin, ",",
                        Range.End, ")")) {
  return ((SlotID == -1 && !arg.has_slot()) || arg.slot() == SlotID) &&
         Range.Begin == arg.begin() && Range.End == arg.end();
}

MATCHER_P2(SlotRangeWithNoExistingAnnotation, SlotID, Range, "") {
  return !arg.has_existing_annotation() &&
         ExplainMatchResult(SlotRange(SlotID, Range), arg, result_listener);
}

MATCHER_P3(SlotRange, SlotID, Range, ExistingAnnotation,
           absl::StrCat("is a SlotRange with ID ", SlotID,
                        " and range equivalent to [", Range.Begin, ",",
                        Range.End, ") and existing annotation ",
                        ExistingAnnotation)) {
  return ExplainMatchResult(SlotRange(SlotID, Range), arg, result_listener) &&
         arg.has_existing_annotation() &&
         arg.existing_annotation() == ExistingAnnotation;
}

MATCHER_P2(TypeLocRanges, Path, Ranges, "") {
  return ExplainMatchResult(Path, arg.path(), result_listener) &&
         ExplainMatchResult(Ranges, arg.range(), result_listener);
}

MATCHER_P2(TypeLocRangesWithNoPragmaNullability, Path, Ranges, "") {
  return !arg.has_pragma_nullability() &&
         ExplainMatchResult(TypeLocRanges(Path, Ranges), arg, result_listener);
}

MATCHER_P3(TypeLocRanges, Path, Ranges, PragmaNullability, "") {
  return ExplainMatchResult(Path, arg.path(), result_listener) &&
         ExplainMatchResult(Ranges, arg.range(), result_listener) &&
         ExplainMatchResult(PragmaNullability, arg.pragma_nullability(),
                            result_listener);
}

template <typename DeclT, typename MatcherT>
std::optional<clang::tidy::nullability::TypeLocRanges> getRanges(
    ASTContext &Ctx, MatcherT Matcher,
    const TypeNullabilityDefaults &Defaults) {
  const auto *D = selectFirst<DeclT>("d", match(Matcher.bind("d"), Ctx));
  CHECK(D != nullptr);
  return clang::tidy::nullability::getEligibleRanges(*D, Defaults);
}

template <typename DeclT, typename MatcherT>
std::optional<clang::tidy::nullability::TypeLocRanges> getRanges(
    llvm::StringRef Input, MatcherT Matcher) {
  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  return getRanges<DeclT>(TU.context(), Matcher,
                          TypeNullabilityDefaults(TU.context(), Pragmas));
}

std::optional<clang::tidy::nullability::TypeLocRanges> getFunctionRanges(
    llvm::StringRef Input, llvm::StringRef FunctionName = "target") {
  return getRanges<FunctionDecl>(Input, functionDecl(hasName(FunctionName)));
}

std::optional<clang::tidy::nullability::TypeLocRanges> getFieldRanges(
    llvm::StringRef Input, llvm::StringRef FieldName = "Target") {
  return getRanges<FieldDecl>(
      Input, FieldName.empty() ? fieldDecl() : fieldDecl(hasName(FieldName)));
}

std::optional<clang::tidy::nullability::TypeLocRanges> getVarRanges(
    llvm::StringRef Input, llvm::StringRef VarName = "Target") {
  return getRanges<VarDecl>(Input, varDecl(hasName(VarName)));
}

TEST(EligibleRangesTest, ReturnAndOneParameterIdentified) {
  auto Input = Annotations("$r[[int *]]target($p[[int *]]P) { return P; }");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName,
          UnorderedElementsAre(
              SlotRangeWithNoExistingAnnotation(0, Input.range("r")),
              SlotRangeWithNoExistingAnnotation(1, Input.range("p"))))));
}

TEST(EligibleRangesTest, OnlyFirstParameterIdentified) {
  auto Input = Annotations("void target([[int *]]P1, int P2) { return; }");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

// Checks that a function decl without a body is handled correctly.
TEST(EligibleRangesTest, DeclHandled) {
  auto Input = Annotations("void target([[int *]]P1, int P2);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, AllNestedPointersEligible) {
  auto Input =
      Annotations("void target($three[[$two[[$one[[int *]]*]]*]]P1, int P2);");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(1, Input.range("three"))))));
}

TEST(EligibleRangesTest, DeclConstExcluded) {
  auto Input = Annotations(R"(
  void target($one[[int *]] const P1,
           $two_o[[$two_i[[int *]] const *]] const P2);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two_o")),
                                       SlotRange(-1, Input.range("two_i"))))));
}

TEST(EligibleRangesTest, PointeeConstIncluded) {
  auto Input = Annotations(R"(
  void target([[const int *]]P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, NestedPointeeConstIncluded) {
  auto Input = Annotations("void target($o[[$i[[const int *]] const *]]P);");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("o")),
                                       SlotRange(-1, Input.range("i"))))));
}

TEST(EligibleRangesTest, AnnotatedSlotsGetRangesForPointerTypeOnly) {
  auto Input = Annotations(R"(
  void target(Nonnull<$one[[int *]]> NonnullP,
           Nullable<$two[[int *]]> NullableP,
           NullabilityUnknown<$three[[int *]]> UnknownP);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName,
          UnorderedElementsAre(
              SlotRange(1, Input.range("one"), Nullability::NONNULL),
              SlotRange(2, Input.range("two"), Nullability::NULLABLE),
              SlotRange(3, Input.range("three"), Nullability::UNKNOWN)))));
}

TEST(EligibleRangesTest, NamespacedAliasAnnotatedSlotsGetNoRange) {
  auto Input = Annotations(R"(
  namespace custom {
  template <typename T>
  using CustomNonnull = Nonnull<T>;
  template <typename T>
  using CustomNullable = Nullable<T>;
  template <typename T>
  using CustomUnknown = NullabilityUnknown<T>;
  }

  // Note also that these custom annotations are aliases for the nullability
  // annotations, not themselves annotated. Aliases of any depth for a
  // nullability annotation are considered an annotation.
  void target(custom::CustomNonnull<$one[[int *]]> NonnullP,
           custom::CustomNullable<$two[[int *]]> NullableP,
           custom::CustomUnknown<$three[[int *]]> UnknownP);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two")),
                                       SlotRange(3, Input.range("three"))))));
}

TEST(EligibleRangesTest, NestedAnnotationsGetOneRange) {
  auto Input = Annotations(R"(void target(Nonnull<Nonnull<[[int *]]>> P);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, NestedPointersOuterAnnotated) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }
  void target(
      Nonnull<$one_o[[$one_i[[int *]]*]]> P,
      Nonnull<$two_o[[std::unique_ptr<$two_i[[int*]]>]]> Q,
      Nonnull<$three_o[[$three_i[[std::unique_ptr<int>]]*]]> R,
      Nonnull<$four_o[[std::unique_ptr<$four_i[[std::unique_ptr<int>]]>]]> S);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one_o")),
                                       SlotRange(-1, Input.range("one_i")),
                                       SlotRange(2, Input.range("two_o")),
                                       SlotRange(-1, Input.range("two_i")),
                                       SlotRange(3, Input.range("three_o")),
                                       SlotRange(-1, Input.range("three_i")),
                                       SlotRange(4, Input.range("four_o")),
                                       SlotRange(-1, Input.range("four_i"))))));
}

TEST(EligibleRangesTest, NestedPointersInnerAnnotated) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target(
      $one_o[[Nonnull<$one_i[[int *]]>*]] P,
      $two_o[[std::unique_ptr<Nonnull<$two_i[[int*]]>>]] Q,
      $three_o[[Nonnull<$three_i[[std::unique_ptr<int>]]>*]] R,
      $four_o[[std::unique_ptr<Nonnull<$four_i[[std::unique_ptr<int>]]>>]] S);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one_o")),
                                       SlotRange(-1, Input.range("one_i")),
                                       SlotRange(2, Input.range("two_o")),
                                       SlotRange(-1, Input.range("two_i")),
                                       SlotRange(3, Input.range("three_o")),
                                       SlotRange(-1, Input.range("three_i")),
                                       SlotRange(4, Input.range("four_o")),
                                       SlotRange(-1, Input.range("four_i"))))));
}

TEST(EligibleRangesTest, RefToPointer) {
  auto Input = Annotations("void target([[int *]]&P);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, TemplateOfPointers) {
  auto Input = Annotations(R"(
  template <typename One, typename Two>
  struct S {}; 

  void target(S<$one[[int *]], $two[[$two_inner[[bool *]]*]]> P);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName, UnorderedElementsAre(
                                    SlotRange(-1, Input.range("one")),
                                    SlotRange(-1, Input.range("two")),
                                    SlotRange(-1, Input.range("two_inner"))))));
}

TEST(EligibleRangesTest, TemplateOfConstPointers) {
  auto Input = Annotations(R"(
  template <typename One, typename Two>
  struct S {};

  void target(
      S<$one[[const int *]], $two_o[[$two_i[[const int *]] const *]]> P,
      S<$three[[int *]] const, $four_o[[$four_i[[int *]] const *]] const> Q);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two_o")),
                                       SlotRange(-1, Input.range("two_i")),
                                       SlotRange(-1, Input.range("three")),
                                       SlotRange(-1, Input.range("four_o")),
                                       SlotRange(-1, Input.range("four_i"))))));
}

TEST(EligibleRangesTest, UniquePtr) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target($one[[std::unique_ptr<int>]] StdSmart,
           Nonnull<$two[[std::unique_ptr<int>]]> NonnullStdSmart);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two"))))));
}

TEST(EligibleRangesTest, UserDefinedSmartPointer) {
  auto Input = Annotations(R"(
  struct MySmartIntPtr {
    using absl_nullability_compatible = void;
    using pointer = int *;
  };

  void target($one[[MySmartIntPtr]] UserDefinedSmart,
           Nonnull<$two[[MySmartIntPtr]]> NonnullUserDefinedSmart);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two"))))));
}

TEST(EligibleRangesTest, UserDefinedTemplatedSmartPointer) {
  auto Input = Annotations(R"(
  template <typename T>
  struct MySmartPtr {
    using absl_nullability_compatible = void;
  };

  void target($one[[MySmartPtr<int>]] UserDefinedSmart,
           Nonnull<$two[[MySmartPtr<int>]]> NonnullUserDefinedSmart);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two"))))));
}

TEST(EligibleRangesTest, SimpleAlias) {
  auto Input = Annotations(R"(
  using IntPtr = int *;

  void target([[IntPtr]] P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, InaccessibleAlias) {
  auto Input = Annotations(R"(
  template <typename T>
  class TemplateClass {};
  using Inaccessible = TemplateClass<int *>;

  void target(Inaccessible P);
  )");
  EXPECT_EQ(getFunctionRanges(Input.code()), std::nullopt);
}

TEST(EligibleRangesTest, NestedAlias) {
  auto Input = Annotations(R"(
  using Nested = int **;

  void target($[[Nested]] P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, AliasTemplate) {
  auto Input = Annotations(R"(
  template <typename T>
  using AliasTemplate = T;

  void target(AliasTemplate<[[int*]]> P, AliasTemplate<int> Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, DependentAliasSimple) {
  auto Input = Annotations(R"(
  template <typename T>
  struct S {
    using Type = T;
  };

  void target(S<[[int *]]>::Type P, S<int>::Type Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, DependentAliasAnnotated) {
  auto Input = Annotations(R"(
  template <typename T>
  struct S {
    using type = T;
  };

  void target(S<Nullable<[[int *]]>>::type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, DependentAliasOfDependentAlias) {
  auto Input = Annotations(R"(
  template <typename T>
  struct vector {
    using value_type = T;
  };
  template <typename T>
  struct S {
    using type = vector<T>::value_type;
  };

  void target(S<[[int *]]>::type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, DependentAliasTemplate) {
  auto Input = Annotations(R"(
  template <typename V>
  struct vector {};
  template <typename T>
  struct S {
    template <template<typename> typename U>
    using type = U<T>;
  };

  void target(S<[[int*]]>::type<vector> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(-1, Input.range())))));
}

TEST(EligibleRangesTest, DependentAliasNested) {
  auto Input = Annotations(R"(
  template <typename V>
  struct vector {
    using value_type = V;
  };

  void target(vector<$one[[$two[[$three[[int*]]*]]*]]>::value_type P);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(-1, Input.range("three"))))));
}

TEST(EligibleRangesTest, NoreturnAliasLosesFunctionTypeSourceInfo) {
  // This previously crashed because the noreturn attribute causes the
  // TypedefType to be unwrapped and rewritten without the Typedef layer and
  // the source information below that layer to be dropped.
  auto Input = Annotations(R"(
    typedef void (*Alias)(const char *, ...);

    __attribute__((__noreturn__)) [[Alias]] Target;
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(0, Input.range(""))))));
}

TEST(EligibleRangesTest, TemplatedClassContext) {
  auto Input = Annotations(R"(
  template <typename T>
  struct Outer {
    struct Inner {};
  };

  void target(Outer<[[int *]]>::Inner P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(-1, Input.range())))));
}

TEST(EligibleRangesTest, NestedTemplatedClasses) {
  auto Input = Annotations(R"(
  template <typename S>
  struct Outermost {
    template <typename T>
    struct Outer {
      template <typename U>
      struct Inner {};
    };
  };

  void target(
      Outermost<$three[[char *]]>::Outer<$two[[int *]]>::Inner<$one[[bool *]]>
          P);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(-1, Input.range("three"))))));
}

TEST(EligibleRangesTest, DependentAliasReferencingFurtherOutTemplateParam) {
  auto Input = Annotations(R"(
  template <typename S>
  struct Outermost {
    template <typename T>
    struct Outer {
      template <typename U>
      using Inner = S;
    };
  };

  void target(Outermost<[[int*]]>::Outer<bool>::Inner<char*> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, DependentAliasForwardingMultipleTemplateArguments) {
  auto Input = Annotations(R"(
  template <typename T, class U>
  struct Pair;
  template <typename T, class U>
  struct PairWrapper {
    using type = Pair<T , U>;
  };

  void target(PairWrapper<$one[[int *]], $two[[bool *]]>::type P);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two"))))));
}

TEST(EligibleRangesTest, DependentAliasInMultipleNestedClassContexts) {
  auto Input = Annotations(R"(
  template <typename A, class B>
  struct Pair;

  template <typename T>
  struct Outer {
    template <typename U>
    struct Inner {
      using type = Pair<T, U>;
    };
  };

  void target(Outer<$one[[int *]]>::Inner<$two[[bool *]]>::type P);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two"))))));
}

TEST(EligibleRangesTest, AliasTemplateInNestedClassContext) {
  auto Input = Annotations(R"(
  template <typename A, class B>
  struct Pair;

  template <typename T>
  struct Outer {
    template <typename U>
    using Inner = Pair<T, U>;
  };

  void target(Outer<$one[[int *]]>::Inner<$two[[bool *]]> P);
  )");

  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two"))))));
}

TEST(EligibleRangesTest, DependentAliasOfSmartPointer) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  template <typename T>
  struct S {
    using type = std::unique_ptr<T>;
  };

  void target($unique_ptr[[S<$inner[[int*]]>::type]] P);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("unique_ptr")),
                                       SlotRange(-1, Input.range("inner"))))));
}

TEST(EligibleRangesTest, DependentlyNamedTemplate) {
  auto Input = Annotations(R"(
  struct Wrapper {
    template <typename T>
    using Alias = T;
  };

  template <typename U, class WrapT>
  struct S {
    using type = typename WrapT::template Alias<U> *;
  };

  // P's canonical type is int**. The outer pointer's range is the whole type,
  // and the inner pointer's range is the first template argument to S.
  void target($outer[[S<$inner[[int *]], Wrapper>::type]] P);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("outer")),
                                       SlotRange(-1, Input.range("inner"))))));
}

TEST(EligibleRangesTest, PartialSpecialization) {
  auto Input = Annotations(R"(
  template <typename T>
  struct S {
  };
  template <typename P>
  struct S<P *> {
    using Alias = P;
  };

  // P's canonical type is int * and derives its nullability from the template
  // argument minus a layer of pointer indirection. But NullabilityWalker
  // doesn't support resugaring template arguments in partial specializations,
  // so we only see the pointer type at the alias' Loc.
  void target([[S<int **>::Alias]] P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, TypeTemplateParamPack) {
  auto Input = Annotations(R"(
  template <typename... T>
  struct Tuple {
    using type = int;
  };

  void target(Tuple<$one[[int *]], $two[[$three[[int *]]*]]> P,
           Tuple<int *, int **>::type Q);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(-1, Input.range("three"))))));
}

TEST(EligibleRangesTest, DefaultTemplateArgs) {
  auto Input = Annotations(R"(
  template <typename T1, typename T2 = int*>
  struct S {};
  template <typename T1, typename T2 = T1>
  using Alias = T2;

  void target(S<$one[[int *]]> P, $two[[Alias<int *>]] Q);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(
                      SlotRange(-1, Input.range("one")),
                      // TODO(b/281474380) Collect the template
                      // argument instead of the whole alias, when we can see
                      // through the layers of default argument redirection
                      SlotRange(2, Input.range("two"))))));
}

TEST(EligibleRangesTest, MultipleSlotsOneRange) {
  auto Input = Annotations(R"(
  template <typename T1, typename T2>
  struct Pair {
    T1 first;
    T2 second;
  };
  template <typename T>
  using Couple = Pair<T, T>;

  void target(Couple<[[int *]]> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          // Eventually, two different valid slot values for the two
          // ranges, but for now, inference looks at neither of
          // them, so both have no slot.
          MainFileName, UnorderedElementsAre(SlotRange(-1, Input.range()),
                                             SlotRange(-1, Input.range())))));
}

TEST(EligibleRangesTest, Field) {
  auto Input = Annotations(R"(
  struct S {
    $zero[[$one[[int *]]*]] Target;
  };
  )");
  EXPECT_THAT(getFieldRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(0, Input.range("zero")),
                                       SlotRange(-1, Input.range("one"))))));
}

TEST(EligibleRangesTest, StaticFieldAkaGlobal) {
  auto Input = Annotations(R"(
  struct S {
    static $zero[[$one[[int *]]*]] Target;
  };
  )");
  EXPECT_THAT(getVarRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(0, Input.range("zero")),
                                       SlotRange(-1, Input.range("one"))))));
}

TEST(EligibleRangesTest, GlobalVariable) {
  auto Input = Annotations(R"(
    $zero[[$one[[int *]]*]] Target;
  )");
  EXPECT_THAT(getVarRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(0, Input.range("zero")),
                                       SlotRange(-1, Input.range("one"))))));
}

TEST(EligibleRangesTest, Lambda) {
  auto Input = Annotations(R"(
  auto Lambda = []($one[[int *]]) -> $zero[[int *]] {};
  )");
  EXPECT_THAT(getFunctionRanges(Input.code(), "operator()"),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(0, Input.range("zero")),
                                       SlotRange(1, Input.range("one"))),
                  Nullability::UNKNOWN)));
}

TEST(EligibleRangesTest, LambdaCaptureWithFunctionTypeInTemplateArg) {
  std::string Input = R"cc(
    template <typename T>
    using ATemplate = T;

    void func(ATemplate<void(const int)> *P) {
      [&P]() { P(0); }();
    }
  )cc";
  // We expect no ranges for the lambda's implicit FieldDecl, which for some
  // reason has an incomplete FunctionTypeLoc that has only nullptrs where the
  // ParmVarDecls should be for the function parameters.
  EXPECT_EQ(getFieldRanges(Input, ""), std::nullopt);
}

TEST(EligibleRangesTest, Pragma) {
  auto Input = Annotations(R"(
  #pragma nullability file_default nonnull

  $zero[[$one[[int *]]*]] target($param_one[[int *]], $param_two[[int *]]);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName,
          UnorderedElementsAre(
              SlotRange(0, Input.range("zero"), Nullability::NONNULL),
              SlotRange(-1, Input.range("one"), Nullability::NONNULL),
              SlotRange(1, Input.range("param_one"), Nullability::NONNULL),
              SlotRange(2, Input.range("param_two"), Nullability::NONNULL)),
          Nullability::NONNULL)));

  Input = Annotations(R"(
  #pragma nullability file_default nullable
  [[int*]] Target;
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      Optional(TypeLocRanges(MainFileName,
                             UnorderedElementsAre(SlotRange(
                                 0, Input.range(), Nullability::NULLABLE)),
                             Nullability::NULLABLE)));

  Input = Annotations(R"(
  [[int*]] Target;
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      Optional(TypeLocRangesWithNoPragmaNullability(
          MainFileName, UnorderedElementsAre(SlotRangeWithNoExistingAnnotation(
                            0, Input.range())))));
}

TEST(EligibleRangesTest, RangesWithBareAutoTypeNotReturned) {
  auto Input = Annotations(R"cc(
    $func_auto[[auto]] noStar(int* P) {
      P = nullptr;
      return P;
    }

    int* getPtr();
    auto GNoStar = getPtr();
    auto _Nullable GNoStarNullable = getPtr();
  )cc");
  EXPECT_THAT(getFunctionRanges(Input.code(), "noStar"),
              Optional(TypeLocRangesWithNoPragmaNullability(
                  MainFileName, Not(Contains(SlotRangeWithNoExistingAnnotation(
                                    0, Input.range("func_auto")))))));
  EXPECT_EQ(getVarRanges(Input.code(), "GNoStar"), std::nullopt);
  EXPECT_EQ(getVarRanges(Input.code(), "GNoStarNullable"), std::nullopt);
}

MATCHER_P2(AutoSlotRangeWithNoExistingAnnotation, SlotID, Range, "") {
  return arg.contains_auto_star() && !arg.has_existing_annotation() &&
         ExplainMatchResult(SlotRange(SlotID, Range), arg, result_listener);
}

MATCHER_P3(AutoSlotRange, SlotID, Range, ExistingAnnotation,
           absl::StrCat("is a SlotRange with ID ", SlotID,
                        " and range equivalent to [", Range.Begin, ",",
                        Range.End, ") and existing annotation ",
                        ExistingAnnotation)) {
  return arg.contains_auto_star() &&
         ExplainMatchResult(SlotRange(SlotID, Range, ExistingAnnotation), arg,
                            result_listener);
}

TEST(EligibleRangesTest, RangesWithAutoStarTypeReturnedWithMarker) {
  auto Input = Annotations(R"(
     $func_auto[[auto*]] star($func_not_auto[[int*]] P) {
      P = nullptr;
      return P;
    }
    
    int* getPtr();
    $var_auto[[auto*]] GStar = getPtr();
    $var_auto_attributed[[auto*]] _Nullable GStarNullable = getPtr();
    $var_auto_star_star[[$var_auto_star_inner[[auto*]]*]] GStarStar = &GStar;
    )");
  EXPECT_THAT(getFunctionRanges(Input.code(), "star"),
              Optional(TypeLocRangesWithNoPragmaNullability(
                  MainFileName, UnorderedElementsAre(
                                    AutoSlotRangeWithNoExistingAnnotation(
                                        0, Input.range("func_auto")),
                                    AllOf(SlotRangeWithNoExistingAnnotation(
                                              1, Input.range("func_not_auto")),
                                          ResultOf(
                                              [](const class SlotRange& SR) {
                                                return SR.contains_auto_star();
                                              },
                                              testing::IsFalse()))))));
  EXPECT_THAT(getVarRanges(Input.code(), "GStar"),
              Optional(TypeLocRangesWithNoPragmaNullability(
                  MainFileName,
                  UnorderedElementsAre(AutoSlotRangeWithNoExistingAnnotation(
                      0, Input.range("var_auto"))))));
  EXPECT_THAT(getVarRanges(Input.code(), "GStarNullable"),
              Optional(TypeLocRangesWithNoPragmaNullability(
                  MainFileName, UnorderedElementsAre(AutoSlotRange(
                                    0, Input.range("var_auto_attributed"),
                                    Nullability::NULLABLE)))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStarStar"),
      Optional(TypeLocRangesWithNoPragmaNullability(
          MainFileName,
          UnorderedElementsAre(AutoSlotRangeWithNoExistingAnnotation(
                                   0, Input.range("var_auto_star_star")),
                               AutoSlotRangeWithNoExistingAnnotation(
                                   -1, Input.range("var_auto_star_inner"))))));
}

MATCHER(NoPreRangeLength, "") {
  return !arg.has_existing_annotation_pre_range_length();
}

MATCHER(NoPostRangeLength, "") {
  return !arg.has_existing_annotation_post_range_length();
}

MATCHER_P(PreRangeLength, Length, "") {
  return arg.has_existing_annotation_pre_range_length() &&
         arg.existing_annotation_pre_range_length() == Length;
}

MATCHER_P(PostRangeLength, Length, "") {
  return arg.has_existing_annotation_post_range_length() &&
         arg.existing_annotation_post_range_length() == Length;
}

TEST(ExistingAnnotationLengthTest, AbslTemplate) {
  auto Input = Annotations(R"(
  namespace absl {
  template <typename T>
  using NullabilityUnknown = ::NullabilityUnknown<T>;
  template <typename T>
  using Nullable = ::Nullable<T>;
  template <typename T>
  using Nonnull = ::Nonnull<T>;
  }
  void target($no[[int*]] P, absl::NullabilityUnknown<$yes[[int*]]> Q,
              absl::/* a comment*/NullabilityUnknown< /* a comment */
              $with_comments[[int*]] /* a comment */  > R,
              absl::Nullable<$nullable[[int*]]> S,
              absl::Nonnull<$nonnull[[int*]]> T);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, testing::ElementsAre(
                            AllOf(SlotRange(1, Input.range("no")),
                                  NoPreRangeLength(), NoPostRangeLength()),
                            AllOf(SlotRange(2, Input.range("yes")),
                                  PreRangeLength(25), PostRangeLength(1)),
                            AllOf(SlotRange(3, Input.range("with_comments")),
                                  PreRangeLength(70), PostRangeLength(19)),
                            AllOf(SlotRange(4, Input.range("nullable")),
                                  PreRangeLength(15), PostRangeLength(1)),
                            AllOf(SlotRange(5, Input.range("nonnull")),
                                  PreRangeLength(14), PostRangeLength(1))))));
}

TEST(ExistingAnnotationLengthTest, AnnotationInMacro) {
  auto Input = Annotations(R"(
  namespace absl {
  template <typename T>
  using NullabilityUnknown = ::NullabilityUnknown<T>;
  }

  #define UNKNOWN(T) absl::NullabilityUnknown<T>

  void target(UNKNOWN([[int *]]) P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(AllOf(
                            SlotRange(1, Input.range("")),
                            // The token checks looking for annotations are done
                            // without expansion of macros, so we see a left
                            // paren as the preceding token and report no
                            // existing pre-range/post-range annotation.
                            NoPreRangeLength(), NoPostRangeLength())))));
}

TEST(ExistingAnnotationLengthTest, UniquePtr) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }
  namespace absl {
  template <typename T>
  using NullabilityUnknown = ::NullabilityUnknown<T>;
  }
  
  void target(absl::NullabilityUnknown<[[std::unique_ptr<int>]]> P);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName, UnorderedElementsAre(AllOf(
                                    SlotRange(1, Input.range("")),
                                    PreRangeLength(25), PostRangeLength(1))))));
}

TEST(ExistingAnnotationLengthTest, DoubleClosingAngleBrackets) {
  auto Input = Annotations(R"(
  namespace absl {
  template <typename T>
  using NullabilityUnknown = ::NullabilityUnknown<T>;
  }
  
  template <typename T>
  using MyTemplateAlias = T;
  
  void target(MyTemplateAlias<absl::NullabilityUnknown<$nothing[[int *]]>> P,
  MyTemplateAlias<absl::NullabilityUnknown<$comment[[int *]]>/* a comment */> Q,
  MyTemplateAlias<absl::NullabilityUnknown<$whitespace[[int *]]>
  > R);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(
                            AllOf(SlotRange(1, Input.range("nothing")),
                                  PreRangeLength(25), PostRangeLength(1)),
                            AllOf(SlotRange(2, Input.range("comment")),
                                  PreRangeLength(25), PostRangeLength(1)),
                            AllOf(SlotRange(3, Input.range("whitespace")),
                                  PreRangeLength(25), PostRangeLength(1))))));
}

TEST(ExistingAnnotationLengthTest, ClangAttribute) {
  auto Input = Annotations(R"(
  void target($no[[int*]] P, $yes[[int*]] _Null_unspecified Q,
              $with_comment[[int*]]/* a comment */_Null_unspecified R,
              $nullable[[int*]] _Nullable S, $nonnull[[int*]] _Nonnull T);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName,
          UnorderedElementsAre(AllOf(SlotRange(1, Input.range("no")),
                                     NoPreRangeLength(), NoPostRangeLength()),
                               AllOf(SlotRange(2, Input.range("yes")),
                                     PreRangeLength(0), PostRangeLength(18)),
                               AllOf(SlotRange(3, Input.range("with_comment")),
                                     PreRangeLength(0), PostRangeLength(32)),
                               AllOf(SlotRange(4, Input.range("nullable")),
                                     PreRangeLength(0), PostRangeLength(10)),
                               AllOf(SlotRange(5, Input.range("nonnull")),
                                     PreRangeLength(0), PostRangeLength(9))))));
}

MATCHER(EquivalentRanges, "") {
  return std::get<0>(arg).begin() == std::get<1>(arg).Begin &&
         std::get<0>(arg).end() == std::get<1>(arg).End;
}

MATCHER_P2(ComplexDeclaratorImpl, FollowingAnnotation, Ranges, "") {
  if (!arg.has_complex_declarator_ranges()) {
    *result_listener << "no complex declarator ranges present";
    return false;
  }
  ComplexDeclaratorRanges ArgRanges = arg.complex_declarator_ranges();
  return ExplainMatchResult(FollowingAnnotation,
                            ArgRanges.following_annotation(),
                            result_listener) &&
         ExplainMatchResult(Pointwise(EquivalentRanges(), Ranges),
                            ArgRanges.removal(), result_listener);
}

auto ComplexDeclarator(llvm::StringRef FollowingAnnotation,
                       std::vector<Annotations::Range> Ranges) {
  return ComplexDeclaratorImpl(FollowingAnnotation, Ranges);
}

MATCHER(NoComplexDeclarator, "") {
  return !arg.has_complex_declarator_ranges();
}

TEST(ComplexDeclaratorTest, FunctionPointer) {
  auto Input = Annotations(R"(
  void target($func_pointer[[int (*$remove_from_type[[P]])(int, $pointer_param[[int*]])]]);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName,
          UnorderedElementsAre(
              AllOf(SlotRange(1, Input.range("func_pointer")),
                    ComplexDeclarator("P", {Input.range("remove_from_type")})),
              AllOf(SlotRange(-1, Input.range("pointer_param")),
                    NoComplexDeclarator())))));

  Input = Annotations("void target($unnamed[[int (*)(int)]]);");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName, UnorderedElementsAre(
                                    AllOf(SlotRange(1, Input.range("unnamed")),
                                          NoComplexDeclarator())))));
}

TEST(ComplexDeclaratorTest, ArrayOfNonPointersHasNoRanges) {
  std::string Input = "void target(int P[]);";
  EXPECT_EQ(getFunctionRanges(Input), std::nullopt);
}

TEST(ComplexDeclaratorTest, ArrayOfSimplePointers) {
  auto Input = Annotations("void target([[int*]] P[]);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(AllOf(SlotRange(-1, Input.range()),
                                                   NoComplexDeclarator())))));
}

TEST(ComplexDeclaratorTest, ArrayOfFunctionPointers) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input = Annotations("void target([[int (*$1^P[3]$2^)(float)]]);");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(AllOf(
                      SlotRange(-1, Input.range()),
                      ComplexDeclarator(
                          "P[3]", {Annotations::Range(Input.point("1"),
                                                      Input.point("2"))}))))));

  // An unnamed array of function pointers. The array brackets are still moved.
  Input = Annotations("void target([[void(*$1^[]$2^)(int)]]);");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(AllOf(
                      SlotRange(-1, Input.range()),
                      ComplexDeclarator(
                          "[]", {Annotations::Range(Input.point("1"),
                                                    Input.point("2"))}))))));
}

TEST(ComplexDeclaratorTest, ArrayOfArrayOfPointersToArray) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input = Annotations(R"(
  void target($1^$range[[int*]] (*$3^P[3][2]$4^)[1]$2^);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName,
          UnorderedElementsAre(
              AllOf(SlotRange(-1, Input.range("range")), NoComplexDeclarator()),
              AllOf(SlotRange(-1, Annotations::Range(Input.point("1"),
                                                     Input.point("2"))),
                    ComplexDeclarator(
                        "P[3][2]", {Annotations::Range(Input.point("3"),
                                                       Input.point("4"))}))))));
}

TEST(ComplexDeclaratorTest, PointerToArray) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input =
      Annotations(R"(void target($1^int (*$remove_from_type[[P]])[]$2^);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName,
          UnorderedElementsAre(AllOf(
              SlotRange(1,
                        Annotations::Range(Input.point("1"), Input.point("2"))),
              ComplexDeclarator("P", {Input.range("remove_from_type")}))))));

  // An unnamed pointer to an array. There's nothing to move.
  Input = Annotations(R"(void target($1^int (*)[]$2^);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(AllOf(
                            SlotRange(1, Annotations::Range(Input.point("1"),
                                                            Input.point("2"))),
                            NoComplexDeclarator())))));
}

TEST(ComplexDeclaratorTest,
     ArrayOfPointersWithExtraParensAroundNameAndInSizeBrackets) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input = Annotations(R"(void target([[int (*$3^((P))[(1 + 2)]$4^)]]);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName,
          UnorderedElementsAre(AllOf(
              SlotRange(-1, Input.range()),
              ComplexDeclarator("((P))[(1 + 2)]",
                                {Annotations::Range(Input.point("3"),
                                                    Input.point("4"))}))))));
}

TEST(ComplexDeclaratorTest, PointerToPointerToArray) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input =
      Annotations(R"(void target($1^int (*$star[[*]]$q[[Q]])[1]$2^);)");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(
                      AllOf(SlotRange(1, Annotations::Range(Input.point("1"),
                                                            Input.point("2"))),
                            ComplexDeclarator("Q", {Input.range("q")})),
                      AllOf(SlotRange(-1, Annotations::Range(Input.point("1"),
                                                             Input.point("2"))),
                            ComplexDeclarator("*", {Input.range("star")}))))));
}

TEST(ComplexDeclaratorTest, PointerToArrayOfFunctionPointers) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input = Annotations(
      R"(void target($whole[[void (*$1^(*$p[[(P)]])[]$2^)(int)]]);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName,
          UnorderedElementsAre(
              AllOf(SlotRange(1, Input.range("whole")),
                    ComplexDeclarator("(P)", {Input.range("p")})),
              AllOf(SlotRange(-1, Input.range("whole")),
                    ComplexDeclarator(
                        "(*)[]", {Annotations::Range(Input.point("1"),
                                                     Input.range("p").Begin),
                                  Annotations::Range(Input.range("p").End,
                                                     Input.point("2"))}))))));
}

template <typename DeclT, typename MatcherT>
std::vector<testing::Matcher<const proto2::Message &>> rangesFor(
    std::vector<std::pair<std::string, MatcherT>> DeclMatchers, ASTContext &Ctx,
    const TypeNullabilityDefaults &Defaults) {
  std::vector<testing::Matcher<const proto2::Message &>> RangeMatchers;
  for (const auto &[Name, Matcher] : DeclMatchers) {
    llvm::errs() << "Getting ranges for " << Name << "\n";
    auto Ranges = getRanges<DeclT>(Ctx, Matcher, Defaults);
    if (!Ranges) {
      ADD_FAILURE() << "No ranges found for " << Name << "!";
      return {};
    }
    RangeMatchers.push_back(EqualsProto(*Ranges));
  }
  return RangeMatchers;
}

auto rangesForFunctions(std::vector<std::string> FunctionNames, ASTContext &Ctx,
                        const TypeNullabilityDefaults &Defaults) {
  std::vector<std::pair<std::string, decltype(functionDecl())>>
      FunctionMatchers;
  for (const auto &Name : FunctionNames) {
    FunctionMatchers.push_back({Name, functionDecl(hasName(Name))});
  }
  return rangesFor<FunctionDecl>(FunctionMatchers, Ctx, Defaults);
}

auto rangesForVars(std::vector<std::string> VarNames, ASTContext &Ctx,
                   const TypeNullabilityDefaults &Defaults) {
  std::vector<std::pair<std::string, decltype(varDecl())>> VarMatchers;
  for (const auto &Name : VarNames) {
    VarMatchers.push_back({Name, varDecl(hasName(Name))});
  }
  return rangesFor<VarDecl>(VarMatchers, Ctx, Defaults);
}

auto rangesForFields(std::vector<std::string> FieldNames, ASTContext &Ctx,
                     const TypeNullabilityDefaults &Defaults) {
  std::vector<std::pair<std::string, decltype(fieldDecl())>> FieldMatchers;
  for (const auto &Name : FieldNames) {
    FieldMatchers.push_back({Name, fieldDecl(hasName(Name))});
  }
  return rangesFor<FieldDecl>(FieldMatchers, Ctx, Defaults);
}

TEST(GetEligibleRangesFromASTTest, Functions) {
  std::string Input = R"cc(
    namespace std {
    template <typename T>
    class unique_ptr;
    }  // namespace std

    void ptrParam(int* P);
    int* ptrReturn();
    void smartPtrParam(std::unique_ptr<int> P);
    void noPtrs();
    auto autoReturn() { return ptrReturn(); }
    auto* autoStarReturn() { return ptrReturn(); }
  )cc";
  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  EXPECT_THAT(getEligibleRanges(TU.context(), Defaults),
              UnorderedElementsAreArray(rangesForFunctions(
                  {"ptrParam", "smartPtrParam", "ptrReturn", "autoStarReturn"},
                  TU.context(), Defaults)));
}

TEST(GetEligibleRangesFromASTTest, Variables) {
  std::string Input = R"cc(
    int* IntPtr;
    int** IntPtrPtr = nullptr;
    auto Auto = IntPtr;
    auto* AutoStar = IntPtr;

    void func() {
      int* LocalIntPtr;
      int** LocalIntPtrPtr = nullptr;
      auto LocalAuto = LocalIntPtr;
      auto* LocalAutoStar = LocalIntPtr;
      static int* StaticInFuncIntPtr;
    }

    namespace std {
    template <typename T>
    class unique_ptr {
      // we need a more complete type for the pointer to use it for variables
      // than we do to use it for functions
      using pointer = T*;
    };
    }  // namespace std

    std::unique_ptr<int> UniquePtr;
    auto AutoUniquePtr =
        UniquePtr;  // not realistic, but just to get the auto deduced
    void smartFunc() {
      std::unique_ptr<int> LocalUniquePtr;
      auto LocalAutoUniquePtr = LocalUniquePtr;
    }
  )cc";
  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  EXPECT_THAT(getEligibleRanges(TU.context(), Defaults),
              UnorderedElementsAreArray(rangesForVars(
                  {"IntPtr", "IntPtrPtr", "AutoStar", "LocalIntPtr",
                   "LocalIntPtrPtr", "LocalAutoStar", "StaticInFuncIntPtr",
                   "UniquePtr", "LocalUniquePtr"},
                  TU.context(), Defaults)));
}

TEST(GetEligibleRangesFromASTTest, Lambda) {
  auto Input = Annotations(R"(
    auto Lambda = []() {};
    auto LambdaWithPtrParam = []($param[[int*]]) {};
    auto LambdaWithPtrReturn = []() -> $return[[int*]] { return nullptr; };
  )");
  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input.code(), Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  EXPECT_THAT(getEligibleRanges(TU.context(), Defaults),
              UnorderedElementsAre(
                  TypeLocRanges(MainFileName, UnorderedElementsAre(SlotRange(
                                                  1, Input.range("param")))),
                  TypeLocRanges(MainFileName, UnorderedElementsAre(SlotRange(
                                                  0, Input.range("return"))))));
}

TEST(GetEligibleRangesFromASTTest, ClassMembers) {
  std::string Input = R"cc(
    namespace std {
    template <typename T>
    struct unique_ptr {
      using pointer = T*;
    };
    }  // namespace std

    template <typename T>
    struct custom_smart_ptr {
      using absl_nullability_compatible = void;
      using pointer = T*;
    };

    class C {
      void method();
      int* methodWithPtr();
      int NonPtrField;
      int* PtrField;
      static int* StaticField;
      std::unique_ptr<int> StdSmartField;
      custom_smart_ptr<int> CustomSmartField;
    };
  )cc";

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  auto Expected = rangesForFunctions({"methodWithPtr"}, TU.context(), Defaults);
  auto AlsoExpected =
      rangesForFields({"PtrField", "StdSmartField", "CustomSmartField"},
                      TU.context(), Defaults);
  Expected.insert(Expected.end(), AlsoExpected.begin(), AlsoExpected.end());
  AlsoExpected = rangesForVars({"StaticField"}, TU.context(), Defaults);
  Expected.insert(Expected.end(), AlsoExpected.begin(), AlsoExpected.end());

  EXPECT_THAT(getEligibleRanges(TU.context(), Defaults),
              UnorderedElementsAreArray(Expected));
}

TEST(GetEligibleRangesFromASTTest, ClassTemplateMembersNoInstantiation) {
  std::string Input = R"cc(
    template <typename T>
    class CTemplate {
      void method();
      T* methodWithPtr();
      T NonPtrField;
      T* PtrField;
      static T* StaticField;
    };
  )cc";

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  auto &TemplatedDecl =
      *selectFirst<ClassTemplateDecl>(
           "b", match(classTemplateDecl(hasName("CTemplate")).bind("b"),
                      TU.context()))
           ->getTemplatedDecl();

  // Matches the ranges for decls in the template.
  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      UnorderedElementsAre(
          EqualsProto(*getEligibleRanges(
              *selectFirst<FunctionDecl>(
                  "b",
                  match(
                      findAll(functionDecl(hasName("methodWithPtr")).bind("b")),
                      TemplatedDecl, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<FieldDecl>(
                  "b", match(findAll(fieldDecl(hasName("PtrField")).bind("b")),
                             TemplatedDecl, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<VarDecl>(
                  "b", match(findAll(varDecl(hasName("StaticField")).bind("b")),
                             TemplatedDecl, TU.context())),
              Defaults))));
}

TEST(GetEligibleRangesFromASTTest, ClassTemplateMembersHasInstantiation) {
  std::string Input = R"cc(
    template <typename T>
    class CTemplate {
      void method();
      T* methodWithPtr();
      T NonPtrField;
      T* PtrField;
      static T* StaticField;
    };

    CTemplate<int> Int;
  )cc";

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  auto &TemplatedDecl =
      *selectFirst<ClassTemplateDecl>(
           "b", match(classTemplateDecl(hasName("CTemplate")).bind("b"),
                      TU.context()))
           ->getTemplatedDecl();

  // Matches the ranges for decls in the template.
  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      UnorderedElementsAre(
          EqualsProto(*getEligibleRanges(
              *selectFirst<FunctionDecl>(
                  "b",
                  match(
                      findAll(functionDecl(hasName("methodWithPtr")).bind("b")),
                      TemplatedDecl, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<FieldDecl>(
                  "b", match(findAll(fieldDecl(hasName("PtrField")).bind("b")),
                             TemplatedDecl, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<VarDecl>(
                  "b", match(findAll(varDecl(hasName("StaticField")).bind("b")),
                             TemplatedDecl, TU.context())),
              Defaults))));
}

TEST(GetEligibleRangesFromASTTest, ClassTemplateExplicitSpecializationMembers) {
  std::string Input = R"cc(
    template <typename T>
    class CTemplate {
      void method();
      T* methodWithPtr();
      T NonPtrField;
      T* PtrField;
      static T* StaticField;
    };

    template <>
    class CTemplate<int> {
      void method();
      int* methodWithPtr();
      int NonPtrField;
      int* PtrField;
      static int* StaticField;

      int* ExtraFieldInSpecialization;
    };
  )cc";

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  auto &TemplatedDecl =
      *selectFirst<ClassTemplateDecl>(
           "b", match(classTemplateDecl(hasName("CTemplate")).bind("b"),
                      TU.context()))
           ->getTemplatedDecl();
  auto &ExplicitSpecialization = *selectFirst<ClassTemplateSpecializationDecl>(
      "b", match(classTemplateSpecializationDecl().bind("b"), TU.context()));

  // Matches the ranges for decls in the template and the ranges in the explicit
  // specialization.
  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      UnorderedElementsAre(
          EqualsProto(*getEligibleRanges(
              *selectFirst<FunctionDecl>(
                  "b",
                  match(
                      findAll(functionDecl(hasName("methodWithPtr")).bind("b")),
                      TemplatedDecl, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<FieldDecl>(
                  "b", match(findAll(fieldDecl(hasName("PtrField")).bind("b")),
                             TemplatedDecl, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<VarDecl>(
                  "b", match(findAll(varDecl(hasName("StaticField")).bind("b")),
                             TemplatedDecl, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<FunctionDecl>(
                  "b",
                  match(
                      findAll(functionDecl(hasName("methodWithPtr")).bind("b")),
                      ExplicitSpecialization, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<FieldDecl>(
                  "b", match(findAll(fieldDecl(hasName("PtrField")).bind("b")),
                             ExplicitSpecialization, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<VarDecl>(
                  "b", match(findAll(varDecl(hasName("StaticField")).bind("b")),
                             ExplicitSpecialization, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<FieldDecl>(
                  "b",
                  match(findAll(fieldDecl(hasName("ExtraFieldInSpecialization"))
                                    .bind("b")),
                        ExplicitSpecialization, TU.context())),
              Defaults))));
}

TEST(GetEligibleRangesFromASTTest, FunctionTemplateNoInstantiation) {
  std::string Input = R"cc(
    template <typename T>
    int funcTemplate(T*) {
      T* LocalInTemplate;
      return 0;
    }
  )cc";

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  auto &TemplatedFuncDecl =
      *selectFirst<FunctionTemplateDecl>(
           "b", match(functionTemplateDecl(hasName("funcTemplate")).bind("b"),
                      TU.context()))
           ->getTemplatedDecl();

  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      UnorderedElementsAre(
          EqualsProto(*getEligibleRanges(
              *selectFirst<FunctionDecl>(
                  "b",
                  match(
                      findAll(functionDecl(hasName("funcTemplate")).bind("b")),
                      TemplatedFuncDecl, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<VarDecl>(
                  "b",
                  match(findAll(varDecl(hasName("LocalInTemplate")).bind("b")),
                        TemplatedFuncDecl, TU.context())),
              Defaults))));
}

TEST(GetEligibleRangesFromASTTest, FunctionTemplateHasInstantiation) {
  std::string Input = R"cc(
    template <typename T>
    int funcTemplate(T*) {
      T* LocalInTemplate;
      return 0;
    }

    int I = funcTemplate<int>(nullptr);
  )cc";

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  auto &TemplatedFuncDecl =
      *selectFirst<FunctionTemplateDecl>(
           "b", match(functionTemplateDecl(hasName("funcTemplate")).bind("b"),
                      TU.context()))
           ->getTemplatedDecl();

  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      UnorderedElementsAre(
          EqualsProto(*getEligibleRanges(
              *selectFirst<FunctionDecl>(
                  "b",
                  match(
                      findAll(functionDecl(hasName("funcTemplate")).bind("b")),
                      TemplatedFuncDecl, TU.context())),
              Defaults)),
          EqualsProto(*getEligibleRanges(
              *selectFirst<VarDecl>(
                  "b",
                  match(findAll(varDecl(hasName("LocalInTemplate")).bind("b")),
                        TemplatedFuncDecl, TU.context())),
              Defaults))));
}
}  // namespace
}  // namespace clang::tidy::nullability
