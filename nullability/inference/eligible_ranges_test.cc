// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <optional>
#include <string>

#include "absl/log/check.h"
#include "nullability/inference/augmented_test_inputs.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/pragma.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Testing/Annotations/Annotations.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"  // IWYU pragma: keep
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::fieldDecl;
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::selectFirst;
using ::clang::ast_matchers::varDecl;
using ::llvm::Annotations;
using ::testing::ExplainMatchResult;
using ::testing::Optional;
using ::testing::UnorderedElementsAre;

test::EnableSmartPointers Enable;

constexpr char MainFileName[] = "input.cc";

MATCHER_P2(SlotRange, SlotID, Range,
           absl::StrCat("is a SlotRange with ID ", SlotID,
                        " and range equivalent to [", Range.Begin, ",",
                        Range.End, ")")) {
  return ((SlotID == -1 && !arg.has_slot()) || arg.slot() == SlotID) &&
         Range.Begin == arg.begin() && Range.End == arg.end();
}

MATCHER_P2(TypeLocRanges, Path, Ranges, "") {
  return ExplainMatchResult(Path, arg.path(), result_listener) &&
         ExplainMatchResult(Ranges, arg.range(), result_listener);
}

template <typename DeclT, typename MatcherT>
std::optional<clang::tidy::nullability::TypeLocRanges> getRanges(
    llvm::StringRef Input, MatcherT Matcher) {
  NullabilityPragmas UnusedPragmas;
  TestAST TU(getAugmentedTestInputs(Input, UnusedPragmas));
  const auto *D =
      selectFirst<DeclT>("d", match(Matcher.bind("d"), TU.context()));
  CHECK(D != nullptr);
  return clang::tidy::nullability::getEligibleRanges(*D);
}

std::optional<clang::tidy::nullability::TypeLocRanges> getFunctionRanges(
    llvm::StringRef Input, llvm::StringRef FunctionName = "target") {
  return getRanges<FunctionDecl>(Input, functionDecl(hasName(FunctionName)));
}

std::optional<clang::tidy::nullability::TypeLocRanges> getFieldRanges(
    llvm::StringRef Input, llvm::StringRef FieldName = "target") {
  return getRanges<FieldDecl>(Input, fieldDecl(hasName(FieldName)));
}

std::optional<clang::tidy::nullability::TypeLocRanges> getVarRanges(
    llvm::StringRef Input, llvm::StringRef VarName = "target") {
  return getRanges<VarDecl>(Input, varDecl(hasName(VarName)));
}

TEST(EligibleRangesTest, ReturnAndOneParameterIdentified) {
  auto Input = Annotations("$r[[int *]]target($p[[int *]]p) { return p; }");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(0, Input.range("r")),
                                             SlotRange(1, Input.range("p"))))));
}

TEST(EligibleRangesTest, OnlyFirstParameterIdentified) {
  auto Input = Annotations("void target([[int *]]p1, int p2) { return; }");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

// Checks that a function decl without a body is handled correctly.
TEST(EligibleRangesTest, DeclHandled) {
  auto Input = Annotations("void target([[int *]]p1, int p2);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, AllNestedPointersEligible) {
  auto Input =
      Annotations("void target($three[[$two[[$one[[int *]]*]]*]]p1, int p2);");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(1, Input.range("three"))))));
}

TEST(EligibleRangesTest, DeclConstExcluded) {
  auto Input = Annotations(R"(
  void target($one[[int *]] const p1,
           $two_o[[$two_i[[int *]] const *]] const p2);
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
  void target([[const int *]]p);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, NestedPointeeConstIncluded) {
  auto Input = Annotations("void target($o[[$i[[const int *]] const *]]p);");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("o")),
                                       SlotRange(-1, Input.range("i"))))));
}

TEST(EligibleRangesTest, FunctionPointerTypeIgnored) {
  std::string Input = "void target(int (*p)(int));";
  EXPECT_EQ(getFunctionRanges(Input), std::nullopt);
}

TEST(EligibleRangesTest, ArrayTypeIgnored) {
  std::string Input = "void target(int p[]);";
  EXPECT_EQ(getFunctionRanges(Input), std::nullopt);
}

TEST(EligibleRangesTest, FunctionAndArrayTypeIgnored) {
  std::string Input = "void target(int (*z[3])(float));";
  EXPECT_EQ(getFunctionRanges(Input), std::nullopt);
}

TEST(EligibleRangesTest, AnnotatedSlotsGetRangesForPointerTypeOnly) {
  auto Input = Annotations(R"(
  void target(Nonnull<$one[[int *]]> nonnull,
           Nullable<$two[[int *]]> nullable,
           NullabilityUnknown<$three[[int *]]> unknown);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two")),
                                       SlotRange(3, Input.range("three"))))));
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
  void target(custom::CustomNonnull<$one[[int *]]> nonnull,
           custom::CustomNullable<$two[[int *]]> nullable,
           custom::CustomUnknown<$three[[int *]]> unknown);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two")),
                                       SlotRange(3, Input.range("three"))))));
}

TEST(EligibleRangesTest, NestedAnnotationsGetOneRange) {
  auto Input = Annotations(R"(void target(Nonnull<Nonnull<[[int *]]>> a);)");
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
      Nonnull<$one_o[[$one_i[[int *]]*]]> p,
      Nonnull<$two_o[[std::unique_ptr<$two_i[[int*]]>]]> q,
      Nonnull<$three_o[[$three_i[[std::unique_ptr<int>]]*]]> r,
      Nonnull<$four_o[[std::unique_ptr<$four_i[[std::unique_ptr<int>]]>]]> s);
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
      $one_o[[Nonnull<$one_i[[int *]]>*]] p,
      $two_o[[std::unique_ptr<Nonnull<$two_i[[int*]]>>]] q,
      $three_o[[Nonnull<$three_i[[std::unique_ptr<int>]]>*]] r,
      $four_o[[std::unique_ptr<Nonnull<$four_i[[std::unique_ptr<int>]]>>]] s);
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
  auto Input = Annotations("void target([[int *]]&p);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(EligibleRangesTest, TemplateOfPointers) {
  auto Input = Annotations(R"(
  template <typename One, typename Two>
  struct S {}; 

  void target(S<$one[[int *]], $two[[$two_inner[[bool *]]*]]> p);
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
      S<$one[[const int *]], $two_o[[$two_i[[const int *]] const *]]> p,
      S<$three[[int *]] const, $four_o[[$four_i[[int *]] const *]] const> q);
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

  void target($one[[std::unique_ptr<int>]] std_smart,
           Nonnull<$two[[std::unique_ptr<int>]]> nonnull_std_smart);
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

  void target($one[[MySmartIntPtr]] user_defined_smart,
           Nonnull<$two[[MySmartIntPtr]]> nonnull_user_defined_smart);
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

  void target($one[[MySmartPtr<int>]] user_defined_smart,
           Nonnull<$two[[MySmartPtr<int>]]> nonnull_user_defined_smart);
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

  void target([[IntPtr]] a);
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

  void target(Inaccessible a);
  )");
  EXPECT_EQ(getFunctionRanges(Input.code()), std::nullopt);
}

TEST(EligibleRangesTest, NestedAlias) {
  auto Input = Annotations(R"(
  using Nested = int **;

  void target($[[Nested]] a);
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

  void target(AliasTemplate<[[int*]]> a, AliasTemplate<int> b);
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
    using type = T;
  };

  void target(S<[[int *]]>::type a, S<int>::type b);
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

  void target(S<Nullable<[[int *]]>>::type a);
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

  void target(S<[[int *]]>::type a);
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

  void target(S<[[int*]]>::type<vector> a);
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

  void target(vector<$one[[$two[[$three[[int*]]*]]*]]>::value_type a);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(-1, Input.range("three"))))));
}

TEST(EligibleRangesTest, TemplatedClassContext) {
  auto Input = Annotations(R"(
  template <typename T>
  struct Outer {
    struct Inner {};
  };

  void target(Outer<[[int *]]>::Inner a);
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
          a);
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

  void target(Outermost<[[int*]]>::Outer<bool>::Inner<char*> a);
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

  void target(PairWrapper<$one[[int *]], $two[[bool *]]>::type a);
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

  void target(Outer<$one[[int *]]>::Inner<$two[[bool *]]>::type a);
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

  void target(Outer<$one[[int *]]>::Inner<$two[[bool *]]> a);
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

  void target($unique_ptr[[S<$inner[[int*]]>::type]] a);
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

  // a's canonical type is int**. The outer pointer's range is the whole type,
  // and the inner pointer's range is the first template argument to S.
  void target($outer[[S<$inner[[int *]], Wrapper>::type]] a);
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

  // a's canonical type is int * and derives its nullability from the template
  // argument minus a layer of pointer indirection. But NullabilityWalker
  // doesn't support resugaring template arguments in partial specializations,
  // so we only see the pointer type at the alias' Loc.
  void target([[S<int **>::Alias]] a);
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

  void target(Tuple<$one[[int *]], $two[[$three[[int *]]*]]> a,
           Tuple<int *, int **>::type b);
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

  void target(S<$one[[int *]]> a, $two[[Alias<int *>]] b);
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

  void target(Couple<[[int *]]> c);
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
    $zero[[$one[[int *]]*]] target;
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
    static $zero[[$one[[int *]]*]] target;
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
    $zero[[$one[[int *]]*]] target;
  )");
  EXPECT_THAT(getVarRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(0, Input.range("zero")),
                                       SlotRange(-1, Input.range("one"))))));
}

}  // namespace
}  // namespace clang::tidy::nullability
