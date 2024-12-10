// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "absl/strings/str_cat.h"
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

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::classTemplateSpecializationDecl;
using ::clang::ast_matchers::fieldDecl;
using ::clang::ast_matchers::findAll;
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::isTemplateInstantiation;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::selectFirst;
using ::clang::ast_matchers::varDecl;
using ::llvm::Annotations;
using ::testing::AllOf;
using ::testing::Contains;
using ::testing::Each;
using ::testing::ElementsAre;
using ::testing::ExplainMatchResult;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::Pointwise;
using ::testing::UnorderedElementsAre;
using ::testing::UnorderedElementsAreArray;

test::EnableSmartPointers Enable;

constexpr char MainFileName[] = "input.cc";

template <typename DeclT, typename MatcherT>
EligibleRanges getRanges(ASTContext &Ctx, MatcherT Matcher,
                         const TypeNullabilityDefaults &Defaults) {
  const auto *D = selectFirst<DeclT>("d", match(Matcher.bind("d"), Ctx));
  CHECK(D != nullptr);
  return clang::tidy::nullability::getEligibleRanges(*D, Defaults);
}

template <typename DeclT, typename MatcherT>
EligibleRanges getRanges(llvm::StringRef Input, MatcherT Matcher) {
  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  return getRanges<DeclT>(TU.context(), Matcher,
                          TypeNullabilityDefaults(TU.context(), Pragmas));
}

EligibleRanges getFunctionRanges(llvm::StringRef Input,
                                 llvm::StringRef FunctionName = "target") {
  return getRanges<FunctionDecl>(Input, functionDecl(hasName(FunctionName)));
}

EligibleRanges getFieldRanges(llvm::StringRef Input,
                              llvm::StringRef FieldName = "Target") {
  return getRanges<FieldDecl>(
      Input, FieldName.empty() ? fieldDecl() : fieldDecl(hasName(FieldName)));
}

EligibleRanges getVarRanges(llvm::StringRef Input,
                            llvm::StringRef VarName = "Target") {
  return getRanges<VarDecl>(Input, varDecl(hasName(VarName)));
}

std::string printSlot(unsigned Slot) { return absl::StrCat(Slot); }

std::string printSlot(std::optional<Slot> Slot) {
  if (Slot) return absl::StrCat(*Slot);
  return "nullopt";
}

MATCHER_P3(eligibleRange, SlotInDecl, SlotInType, Range,
           absl::StrCat("has a SlotRange for slot ", printSlot(SlotInDecl),
                        " with slot_in_type ", SlotInType,
                        " and range equivalent to [", Range.Begin, ",",
                        Range.End, ")")) {
  std::optional<Slot> Slot = arg.Slot;
  const SlotRange &ArgRange = arg.Range;
  return ExplainMatchResult(SlotInDecl, Slot, result_listener) &&
         ExplainMatchResult(SlotInType, ArgRange.slot_in_type(),
                            result_listener) &&
         ExplainMatchResult(Range.Begin, ArgRange.begin(), result_listener) &&
         ExplainMatchResult(Range.End, ArgRange.end(), result_listener);
}

MATCHER_P3(eligibleRangeWithNoExistingAnnotation, SlotInDecl, SlotInType, Range,
           "") {
  return ExplainMatchResult(false, arg.Range.has_existing_annotation(),
                            result_listener) &&
         ExplainMatchResult(eligibleRange(SlotInDecl, SlotInType, Range), arg,
                            result_listener);
}

MATCHER_P4(eligibleRange, SlotInDecl, SlotInType, Range, ExistingAnnotation,
           absl::StrCat("has a SlotRange for slot ", printSlot(SlotInDecl),
                        " with slot_in_type ", SlotInType,
                        " and range equivalent to [", Range.Begin, ",",
                        Range.End, ") and existing annotation ",
                        ExistingAnnotation)) {
  return ExplainMatchResult(eligibleRange(SlotInDecl, SlotInType, Range), arg,
                            result_listener) &&
         ExplainMatchResult(true, arg.Range.has_existing_annotation(),
                            result_listener) &&
         ExplainMatchResult(ExistingAnnotation, arg.Range.existing_annotation(),
                            result_listener);
}

MATCHER_P(hasPath, Path, "") {
  return ExplainMatchResult(Path, arg.Range.path(), result_listener);
}

MATCHER(hasNoPragmaNullability, "") {
  return !arg.Range.has_pragma_nullability();
}

MATCHER_P(hasPragmaNullability, PragmaNullability, "") {
  return ExplainMatchResult(true, arg.Range.has_pragma_nullability(),
                            result_listener) &&
         ExplainMatchResult(PragmaNullability, arg.Range.pragma_nullability(),
                            result_listener);
}

TEST(EligibleRangesTest, ReturnAndOneParameterIdentified) {
  auto Input = Annotations("$r[[int *]]target($p[[int *]]P) { return P; }");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              eligibleRangeWithNoExistingAnnotation(0, 0, Input.range("r")),
              eligibleRangeWithNoExistingAnnotation(1, 0, Input.range("p")))));
}

TEST(EligibleRangesTest, OnlyFirstParameterIdentified) {
  auto Input = Annotations("void target([[int *]]P1, int P2) { return; }");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
}

// Checks that a function decl without a body is handled correctly.
TEST(EligibleRangesTest, DeclHandled) {
  auto Input = Annotations("void target([[int *]]P1, int P2);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
}

TEST(EligibleRangesTest, AllNestedPointersEligible) {
  auto Input = Annotations(
      "void target($outer[[$middle[[$inner[[int *]]*]]*]]P1, int P2);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, 0, Input.range("outer")),
                eligibleRange(std::nullopt, 1, Input.range("middle")),
                eligibleRange(std::nullopt, 2, Input.range("inner")))));
}

TEST(EligibleRangesTest, DeclConstExcluded) {
  auto Input = Annotations(R"(
  void target($one[[int *]] const P1,
           $two_o[[$two_i[[int *]] const *]] const P2);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, 0, Input.range("one")),
                eligibleRange(2, 0, Input.range("two_o")),
                eligibleRange(std::nullopt, 1, Input.range("two_i")))));
}

TEST(EligibleRangesTest, PointeeConstIncluded) {
  auto Input = Annotations(R"(
  void target([[const int *]]P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
}

TEST(EligibleRangesTest, NestedPointeeConstIncluded) {
  auto Input = Annotations("void target($o[[$i[[const int *]] const *]]P);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, 0, Input.range("o")),
                eligibleRange(std::nullopt, 1, Input.range("i")))));
}

TEST(EligibleRangesTest, SmartPointerDeclConstExcluded) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target(const [[std::unique_ptr<int>]] P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
}

TEST(EligibleRangesTest, AnnotatedSlotsGetRangesForPointerTypeOnly) {
  auto Input = Annotations(R"(
  void target(Nonnull<$one[[int *]]> NonnullP,
           Nullable<$two[[int *]]> NullableP,
           NullabilityUnknown<$three[[int *]]> UnknownP);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, 0, Input.range("one"), Nullability::NONNULL),
                eligibleRange(2, 0, Input.range("two"), Nullability::NULLABLE),
                eligibleRange(3, 0, Input.range("three"),
                              Nullability::UNKNOWN))));
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
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range("one")),
                                 eligibleRange(2, 0, Input.range("two")),
                                 eligibleRange(3, 0, Input.range("three")))));
}

TEST(EligibleRangesTest, NestedAnnotationsGetOneRange) {
  auto Input = Annotations(R"(void target(Nonnull<Nonnull<[[int *]]>> P);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
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
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, 0, Input.range("one_o")),
                eligibleRange(std::nullopt, 1, Input.range("one_i")),
                eligibleRange(2, 0, Input.range("two_o")),
                eligibleRange(std::nullopt, 1, Input.range("two_i")),
                eligibleRange(3, 0, Input.range("three_o")),
                eligibleRange(std::nullopt, 1, Input.range("three_i")),
                eligibleRange(4, 0, Input.range("four_o")),
                eligibleRange(std::nullopt, 1, Input.range("four_i")))));
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
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, 0, Input.range("one_o")),
                eligibleRange(std::nullopt, 1, Input.range("one_i")),
                eligibleRange(2, 0, Input.range("two_o")),
                eligibleRange(std::nullopt, 1, Input.range("two_i")),
                eligibleRange(3, 0, Input.range("three_o")),
                eligibleRange(std::nullopt, 1, Input.range("three_i")),
                eligibleRange(4, 0, Input.range("four_o")),
                eligibleRange(std::nullopt, 1, Input.range("four_i")))));
}

TEST(EligibleRangesTest, RefToPointer) {
  auto Input = Annotations("void target([[int *]]&P);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
}

TEST(EligibleRangesTest, TemplateOfPointers) {
  auto Input = Annotations(R"(
  template <typename One, typename Two>
  struct S {}; 

  void target(S<$one[[int *]], $two[[$two_inner[[bool *]]*]]> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, 0, Input.range("one")),
                eligibleRange(std::nullopt, 1, Input.range("two")),
                eligibleRange(std::nullopt, 2, Input.range("two_inner")))));
}

TEST(EligibleRangesTest, TemplateOfConstPointers) {
  auto Input = Annotations(R"(
  template <typename One, typename Two>
  struct S {};

  void target(
      S<$one[[const int *]], $two_o[[$two_i[[const int *]] const *]]> P,
      S<$three[[int *]] const, $four_o[[$four_i[[int *]] const *]] const> Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, 0, Input.range("one")),
                eligibleRange(std::nullopt, 1, Input.range("two_o")),
                eligibleRange(std::nullopt, 2, Input.range("two_i")),
                eligibleRange(std::nullopt, 0, Input.range("three")),
                eligibleRange(std::nullopt, 1, Input.range("four_o")),
                eligibleRange(std::nullopt, 2, Input.range("four_i")))));
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
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range("one")),
                                 eligibleRange(2, 0, Input.range("two")))));
}

TEST(EligibleRangesTest, UserDefinedSmartPointer) {
  auto Input = Annotations(R"(
  struct _Nullable MySmartIntPtr {
    using pointer = int *;
  };

  void target($one[[MySmartIntPtr]] UserDefinedSmart,
           Nonnull<$two[[MySmartIntPtr]]> NonnullUserDefinedSmart);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range("one")),
                                 eligibleRange(2, 0, Input.range("two")))));
}

TEST(EligibleRangesTest, UserDefinedTemplatedSmartPointer) {
  auto Input = Annotations(R"(
  template <typename T>
  struct _Nullable MySmartPtr {};

  void target($one[[MySmartPtr<int>]] UserDefinedSmart,
           Nonnull<$two[[MySmartPtr<int>]]> NonnullUserDefinedSmart);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range("one")),
                                 eligibleRange(2, 0, Input.range("two")))));
}

TEST(EligibleRangesTest, SimpleAlias) {
  auto Input = Annotations(R"(
  using IntPtr = int *;

  void target([[IntPtr]] P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
}

TEST(EligibleRangesTest, InaccessibleAlias) {
  auto Input = Annotations(R"(
  template <typename T>
  class TemplateClass {};
  using Inaccessible = TemplateClass<int *>;

  void target(Inaccessible P);
  )");
  EXPECT_THAT(getFunctionRanges(Input.code()), IsEmpty());
}

TEST(EligibleRangesTest, NestedAlias) {
  auto Input = Annotations(R"(
  using Nested = int **;

  void target($[[Nested]] P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
}

TEST(EligibleRangesTest, AliasTemplate) {
  auto Input = Annotations(R"(
  template <typename T>
  using AliasTemplate = T;

  void target(AliasTemplate<[[int*]]> P, AliasTemplate<int> Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
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
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(std::nullopt, 0, Input.range()))));
}

TEST(EligibleRangesTest, DependentAliasNested) {
  auto Input = Annotations(R"(
  template <typename V>
  struct vector {
    using value_type = V;
  };

  void target(vector<$outer[[$middle[[$inner[[int*]]*]]*]]>::value_type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, 0, Input.range("outer")),
                eligibleRange(std::nullopt, 1, Input.range("middle")),
                eligibleRange(std::nullopt, 2, Input.range("inner")))));
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(0, 0, Input.range("")))));
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
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(std::nullopt, 0, Input.range()))));
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
      Outermost<$zero[[char *]]>::Outer<$one[[int *]]>::Inner<$two[[bool *]]>
          P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, 0, Input.range("zero")),
                eligibleRange(std::nullopt, 1, Input.range("one")),
                eligibleRange(std::nullopt, 2, Input.range("two")))));
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
}

TEST(EligibleRangesTest, DependentAliasForwardingMultipleTemplateArguments) {
  auto Input = Annotations(R"(
  template <typename T, class U>
  struct Pair;
  template <typename T, class U>
  struct PairWrapper {
    using type = Pair<T , U>;
  };

  void target(PairWrapper<$zero[[int *]], $one[[bool *]]>::type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, 0, Input.range("zero")),
                eligibleRange(std::nullopt, 1, Input.range("one")))));
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

  void target(Outer<$zero[[int *]]>::Inner<$one[[bool *]]>::type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, 0, Input.range("zero")),
                eligibleRange(std::nullopt, 1, Input.range("one")))));
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

  void target(Outer<$zero[[int *]]>::Inner<$one[[bool *]]> P);
  )");

  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, 0, Input.range("zero")),
                eligibleRange(std::nullopt, 1, Input.range("one")))));
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
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, 0, Input.range("unique_ptr")),
                eligibleRange(std::nullopt, 1, Input.range("inner")))));
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
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, 0, Input.range("outer")),
                eligibleRange(std::nullopt, 1, Input.range("inner")))));
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range()))));
}

TEST(EligibleRangesTest, TypeTemplateParamPack) {
  auto Input = Annotations(R"(
  template <typename... T>
  struct Tuple {
    using type = int;
  };

  void target(Tuple<$zero[[int *]], $one[[$two[[int *]]*]]> P,
           Tuple<int *, int **>::type Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, 0, Input.range("zero")),
                eligibleRange(std::nullopt, 1, Input.range("one")),
                eligibleRange(std::nullopt, 2, Input.range("two")))));
}

TEST(EligibleRangesTest, DefaultTemplateArgs) {
  auto Input = Annotations(R"(
  template <typename T1, typename T2 = int*>
  struct S {};
  template <typename T1, typename T2 = T1>
  using Alias = T2;

  void target(S<$one[[int *]]> P, $two[[Alias<int *>]] Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, 0, Input.range("one")),
                // TODO(b/281474380) Collect the template
                // argument instead of the whole alias, when we can see
                // through the layers of default argument redirection
                eligibleRange(2, 0, Input.range("two")))));
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
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          // Eventually, two different valid slot values for the two
          // ranges, but for now, inference looks at neither of
          // them, so both have no slot.
          UnorderedElementsAre(eligibleRange(std::nullopt, 0, Input.range()),
                               eligibleRange(std::nullopt, 1, Input.range()))));
}

TEST(EligibleRangesTest, Field) {
  auto Input = Annotations(R"(
  struct S {
    $zero[[$one[[int *]]*]] Target;
  };
  )");
  EXPECT_THAT(
      getFieldRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, 0, Input.range("zero")),
                eligibleRange(std::nullopt, 1, Input.range("one")))));
}

TEST(EligibleRangesTest, StaticFieldAkaGlobal) {
  auto Input = Annotations(R"(
  struct S {
    static $zero[[$one[[int *]]*]] Target;
  };
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, 0, Input.range("zero")),
                eligibleRange(std::nullopt, 1, Input.range("one")))));
}

TEST(EligibleRangesTest, GlobalVariable) {
  auto Input = Annotations(R"(
    $zero[[$one[[int *]]*]] Target;
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, 0, Input.range("zero")),
                eligibleRange(std::nullopt, 1, Input.range("one")))));
}

TEST(EligibleRangesTest, Lambda) {
  auto Input = Annotations(R"(
  auto Lambda = []($one[[int *]]) -> $zero[[int *]] {};
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "operator()"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(0, 0, Input.range("zero")),
                                 eligibleRange(1, 0, Input.range("one")))));
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
  EXPECT_THAT(getFieldRanges(Input, ""), IsEmpty());
}

TEST(EligibleRangesTest, Pragma) {
  auto Input = Annotations(R"(
  #pragma nullability file_default nonnull

  $zero[[$one[[int *]]*]] target($param_one[[int *]], $param_two[[int *]]);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName),
                       hasPragmaNullability(Nullability::NONNULL))),
            UnorderedElementsAre(
                eligibleRange(0, 0, Input.range("zero"), Nullability::NONNULL),
                eligibleRange(std::nullopt, 1, Input.range("one"),
                              Nullability::NONNULL),
                eligibleRange(1, 0, Input.range("param_one"),
                              Nullability::NONNULL),
                eligibleRange(2, 0, Input.range("param_two"),
                              Nullability::NONNULL))));

  Input = Annotations(R"(
  #pragma nullability file_default nullable
  [[int*]] Target;
  )");
  EXPECT_THAT(getVarRanges(Input.code()),
              AllOf(Each(AllOf(hasPath(MainFileName),
                               hasPragmaNullability(Nullability::NULLABLE))),
                    UnorderedElementsAre(eligibleRange(
                        0, 0, Input.range(), Nullability::NULLABLE))));

  Input = Annotations(R"(
  [[int*]] Target;
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRangeWithNoExistingAnnotation(0, 0, Input.range()))));
}

TEST(EligibleRangesTest, RangesEntirelyWithinMacro) {
  auto Input = Annotations(R"(
  #define MACRO(IpGlobalVar, funcName) $within[[int*]] funcName##Func() {return IpGlobalVar;}
  int* GlobalVar1;
  int* GlobalVar2;
  MACRO(GlobalVar1, getVar1);
  MACRO(GlobalVar2, getVar2);

  #define A_TYPE int*
  $whole_type_1[[A_TYPE]] GlobalVar3;
  $whole_type_2[[A_TYPE]] GlobalVar4;
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "getVar1Func"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(0, 0, Input.range("within")))));
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "getVar2Func"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(0, 0, Input.range("within")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GlobalVar3"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, 0, Input.range("whole_type_1")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GlobalVar4"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, 0, Input.range("whole_type_2")))));
}

// We saw specific difficulty finding previous tokens during our search for
// `const`s when passing over the line continuation in multi-line macros.
TEST(EligibleRangesTest, RangesInMultiLineMacro) {
  auto Input = Annotations(R"(
  #define MACRO(IpGlobalVar, funcName) $return[[int*]] funcName##Func(      \
                                        $param_one[[const char*]] p,        \
                                        $param_two[[const                   \
                                        int*]] p2)                          \
                                           {return IpGlobalVar;}
  int* GlobalVar;
  MACRO(GlobalVar, getVar);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "getVarFunc"),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(0, 0, Input.range("return")),
                               eligibleRange(1, 0, Input.range("param_one")),
                               eligibleRange(2, 0, Input.range("param_two")))));
}

// We saw specific difficulty finding the post-line-continuation start of a
// const or similar that begins with no unescaped whitespace between the `\` and
// the token's first character. Clang considers each of the `const`s in this
// tests to begin at the preceding `\` location, but we'd rather add the
// nullability annotation after the newline.
TEST(EligibleRangesTest, RangesInMultiLineMacroNoIndentation) {
  auto Input = Annotations(R"(
  #define MACRO(IpGlobalVar, funcName) $return[[int*]] funcName##Func(  \
\
$param_one[[const char*]] p,        \
$param_two[[const                   \
int*]] p2)                          \
    {return IpGlobalVar;}
  int* GlobalVar;
  MACRO(GlobalVar, getVar);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "getVarFunc"),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(0, 0, Input.range("return")),
                               eligibleRange(1, 0, Input.range("param_one")),
                               eligibleRange(2, 0, Input.range("param_two")))));
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
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "noStar"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            Not(Contains(eligibleRangeWithNoExistingAnnotation(
                0, 0, Input.range("func_auto"))))));
  EXPECT_THAT(getVarRanges(Input.code(), "GNoStar"), IsEmpty());
  EXPECT_THAT(getVarRanges(Input.code(), "GNoStarNullable"), IsEmpty());
}

TEST(EligibleRangesTest, RangesWithBareAutoAsTemplateParameterNotReturned) {
  auto Input = Annotations(R"(
    namespace std {
    template <typename T>
    class unique_ptr;
    }

    void func(auto P, auto& Q) { }

    void lambdaRecipient($A[[void (*A)(const $B[[std::unique_ptr<int>]]& B)]]) {}

    void usage() {
      $I[[int*]] I;
      func(I, I);
      lambdaRecipient([](const auto& X) {});
    }
  )");
  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input.code(), Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      // The only eligible ranges in the entire snippet are the function pointer
      // parameter `A`, it's unique_ptr parameter `B`, and the local variable
      // `I`. No ranges are collected for the bare `auto` parameters of `func`
      // or the bare `auto` parameter of the lambda.
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(1, 0, Input.range("A")),
                               eligibleRange(std::nullopt, 1, Input.range("B")),
                               eligibleRange(0, 0, Input.range("I")))));
}

MATCHER_P3(autoEligibleRangeWithNoExistingAnnotation, SlotInDecl, SlotInType,
           Range, "") {
  return ExplainMatchResult(true, arg.Range.contains_auto_star(),
                            result_listener) &&
         ExplainMatchResult(false, arg.Range.has_existing_annotation(),
                            result_listener) &&
         ExplainMatchResult(eligibleRange(SlotInDecl, SlotInType, Range), arg,
                            result_listener);
}

MATCHER_P4(autoEligibleRange, SlotInDecl, SlotInType, Range, ExistingAnnotation,
           "") {
  return ExplainMatchResult(true, arg.Range.contains_auto_star(),
                            result_listener) &&
         ExplainMatchResult(
             eligibleRange(SlotInDecl, SlotInType, Range, ExistingAnnotation),
             arg, result_listener);
}

TEST(EligibleRangesTest, RangesWithAutoStarTypeReturnedWithMarker) {
  auto Input = Annotations(R"(
     $func_auto[[auto*]] star($func_not_auto[[int*]] P) {
      P = nullptr;
      return P;
    }
    
    int* getPtr();
    $var_auto[[auto*]] GStar = getPtr();
    $var_auto_const[[auto*]] const GStarConst = getPtr();
    $var_const_auto[[const auto*]]  GConstStar = getPtr();
    $var_auto_const_ref[[auto*]] const& GStarConstRef = getPtr();
    $var_const_auto_ref[[const auto*]]& GConstStarRef = GConstStar;
    $var_auto_attributed[[auto*]] _Nullable GStarNullable = getPtr();
    $var_auto_star_star[[$var_auto_star_inner[[auto*]]*]] GStarStar = &GStar;
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "star"),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(autoEligibleRangeWithNoExistingAnnotation(
                                   0, 0, Input.range("func_auto")),
                               AllOf(eligibleRangeWithNoExistingAnnotation(
                                         1, 0, Input.range("func_not_auto")),
                                     ResultOf(
                                         [](const class EligibleRange &ER) {
                                           return ER.Range.contains_auto_star();
                                         },
                                         testing::IsFalse())))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStar"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(autoEligibleRangeWithNoExistingAnnotation(
                0, 0, Input.range("var_auto")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStarConst"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(autoEligibleRangeWithNoExistingAnnotation(
                0, 0, Input.range("var_auto_const")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GConstStar"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(autoEligibleRangeWithNoExistingAnnotation(
                0, 0, Input.range("var_const_auto")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStarConstRef"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(autoEligibleRangeWithNoExistingAnnotation(
                0, 0, Input.range("var_auto_const_ref")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GConstStarRef"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(autoEligibleRangeWithNoExistingAnnotation(
                0, 0, Input.range("var_const_auto_ref")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStarNullable"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                autoEligibleRange(0, 0, Input.range("var_auto_attributed"),
                                  Nullability::NULLABLE))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStarStar"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                autoEligibleRangeWithNoExistingAnnotation(
                    0, 0, Input.range("var_auto_star_star")),
                autoEligibleRangeWithNoExistingAnnotation(
                    std::nullopt, 1, Input.range("var_auto_star_inner")))));
}

MATCHER(noPreRangeLength, "") {
  return !arg.Range.has_existing_annotation_pre_range_length();
}

MATCHER(noPostRangeLength, "") {
  return !arg.Range.has_existing_annotation_post_range_length();
}

MATCHER_P(preRangeLength, Length, "") {
  return arg.Range.has_existing_annotation_pre_range_length() &&
         arg.Range.existing_annotation_pre_range_length() == Length;
}

MATCHER_P(postRangeLength, Length, "") {
  return arg.Range.has_existing_annotation_post_range_length() &&
         arg.Range.existing_annotation_post_range_length() == Length;
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            ElementsAre(AllOf(eligibleRange(1, 0, Input.range("no")),
                              noPreRangeLength(), noPostRangeLength()),
                        AllOf(eligibleRange(2, 0, Input.range("yes")),
                              preRangeLength(25), postRangeLength(1)),
                        AllOf(eligibleRange(3, 0, Input.range("with_comments")),
                              preRangeLength(70), postRangeLength(19)),
                        AllOf(eligibleRange(4, 0, Input.range("nullable")),
                              preRangeLength(15), postRangeLength(1)),
                        AllOf(eligibleRange(5, 0, Input.range("nonnull")),
                              preRangeLength(14), postRangeLength(1)))));
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(1, 0, Input.range("")),
                      // The token checks looking for annotations are done
                      // without expansion of macros, so we see a left
                      // paren as the preceding token and report no
                      // existing pre-range/post-range annotation.
                      noPreRangeLength(), noPostRangeLength()))));
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
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(AllOf(eligibleRange(1, 0, Input.range("")),
                                     preRangeLength(25), postRangeLength(1)))));
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(1, 0, Input.range("nothing")),
                      preRangeLength(25), postRangeLength(1)),
                AllOf(eligibleRange(2, 0, Input.range("comment")),
                      preRangeLength(25), postRangeLength(1)),
                AllOf(eligibleRange(3, 0, Input.range("whitespace")),
                      preRangeLength(25), postRangeLength(1)))));
}

TEST(ExistingAnnotationLengthTest, ClangAttribute) {
  auto Input = Annotations(R"(
  void target($no[[int*]] P, $yes[[int*]] _Null_unspecified Q,
              $no_space[[int*]]_Null_unspecified R,
              $with_comment[[int*]]/* a comment */_Null_unspecified S,
              $nullable[[int*]] _Nullable T, $nonnull[[int*]] _Nonnull U);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(1, 0, Input.range("no")),
                      noPreRangeLength(), noPostRangeLength()),
                AllOf(eligibleRange(2, 0, Input.range("yes")),
                      preRangeLength(0), postRangeLength(18)),
                AllOf(eligibleRange(3, 0, Input.range("no_space")),
                      preRangeLength(0), postRangeLength(17)),
                AllOf(eligibleRange(4, 0, Input.range("with_comment")),
                      preRangeLength(0), postRangeLength(32)),
                AllOf(eligibleRange(5, 0, Input.range("nullable")),
                      preRangeLength(0), postRangeLength(10)),
                AllOf(eligibleRange(6, 0, Input.range("nonnull")),
                      preRangeLength(0), postRangeLength(9)))));
}

MATCHER(equivalentRanges, "") {
  return std::get<0>(arg).begin() == std::get<1>(arg).Begin &&
         std::get<0>(arg).end() == std::get<1>(arg).End;
}

MATCHER_P2(complexDeclaratorImpl, FollowingAnnotation, Ranges, "") {
  if (!arg.Range.has_complex_declarator_ranges()) {
    *result_listener << "no complex declarator ranges present";
    return false;
  }
  ComplexDeclaratorRanges ArgRanges = arg.Range.complex_declarator_ranges();
  return ExplainMatchResult(FollowingAnnotation,
                            ArgRanges.following_annotation(),
                            result_listener) &&
         ExplainMatchResult(Pointwise(equivalentRanges(), Ranges),
                            ArgRanges.removal(), result_listener);
}

auto complexDeclarator(llvm::StringRef FollowingAnnotation,
                       std::vector<Annotations::Range> Ranges) {
  return complexDeclaratorImpl(FollowingAnnotation, Ranges);
}

MATCHER(noComplexDeclarator, "") {
  return !arg.Range.has_complex_declarator_ranges();
}

TEST(ComplexDeclaratorTest, FunctionPointer) {
  auto Input = Annotations(R"(
  void target($func_pointer[[int (*$remove_from_type[[P]])(int, $pointer_param[[int*]])]]);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, 0, Input.range("func_pointer")),
                    complexDeclarator("P", {Input.range("remove_from_type")})),
              AllOf(
                  eligibleRange(std::nullopt, 1, Input.range("pointer_param")),
                  noComplexDeclarator()))));

  Input = Annotations("void target($unnamed[[int (*)(int)]]);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(1, 0, Input.range("unnamed")),
                      noComplexDeclarator()))));
}

TEST(ComplexDeclaratorTest, ArrayOfNonPointersHasNoRanges) {
  std::string Input = "void target(int P[]);";
  EXPECT_THAT(getFunctionRanges(Input), IsEmpty());
}

TEST(ComplexDeclaratorTest, ArrayOfSimplePointers) {
  auto Input = Annotations("void target([[int*]] P[]);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(std::nullopt, 0, Input.range()),
                      noComplexDeclarator()))));
}

TEST(ComplexDeclaratorTest, ArrayOfFunctionPointers) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input = Annotations("void target([[int (*$1^P[3]$2^)(float)]]);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(std::nullopt, 0, Input.range()),
                      complexDeclarator(
                          "P[3]", {Annotations::Range(Input.point("1"),
                                                      Input.point("2"))})))));

  // An unnamed array of function pointers. The array brackets are still moved.
  Input = Annotations("void target([[void(*$1^[]$2^)(int)]]);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(std::nullopt, 0, Input.range()),
                      complexDeclarator(
                          "[]", {Annotations::Range(Input.point("1"),
                                                    Input.point("2"))})))));
}

TEST(ComplexDeclaratorTest, ArrayOfArrayOfPointersToArray) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input = Annotations(R"(
  void target($1^$range[[int*]] (*$3^P[3][2]$4^)[1]$2^);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(std::nullopt, 1, Input.range("range")),
                      noComplexDeclarator()),
                AllOf(eligibleRange(std::nullopt, 0,
                                    Annotations::Range(Input.point("1"),
                                                       Input.point("2"))),
                      complexDeclarator("P[3][2]", {Annotations::Range(
                                                       Input.point("3"),
                                                       Input.point("4"))})))));
}

TEST(ComplexDeclaratorTest, PointerToArray) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input =
      Annotations(R"(void target($1^int (*$remove_from_type[[P]])[]$2^);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(AllOf(
              eligibleRange(
                  1, 0, Annotations::Range(Input.point("1"), Input.point("2"))),
              complexDeclarator("P", {Input.range("remove_from_type")})))));

  // An unnamed pointer to an array. There's nothing to move.
  Input = Annotations(R"(void target($1^int (*)[]$2^);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(AllOf(
              eligibleRange(
                  1, 0, Annotations::Range(Input.point("1"), Input.point("2"))),
              noComplexDeclarator()))));
}

TEST(ComplexDeclaratorTest,
     ArrayOfPointersWithExtraParensAroundNameAndInSizeBrackets) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input = Annotations(R"(void target([[int (*$3^((P))[(1 + 2)]$4^)]]);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(AllOf(
                eligibleRange(std::nullopt, 0, Input.range()),
                complexDeclarator("((P))[(1 + 2)]",
                                  {Annotations::Range(Input.point("3"),
                                                      Input.point("4"))})))));
}

TEST(ComplexDeclaratorTest, PointerToPointerToArray) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input =
      Annotations(R"(void target($1^int (*$star[[*]]$q[[Q]])[1]$2^);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(1, 0,
                                    Annotations::Range(Input.point("1"),
                                                       Input.point("2"))),
                      complexDeclarator("Q", {Input.range("q")})),
                AllOf(eligibleRange(std::nullopt, 1,
                                    Annotations::Range(Input.point("1"),
                                                       Input.point("2"))),
                      complexDeclarator("*", {Input.range("star")})))));
}

TEST(ComplexDeclaratorTest, PointerToArrayOfFunctionPointers) {
  // Can't use ranges marked by [[...]] around arrays because of the adjacent
  // closing square bracket at the end of the array length and the unfortunate
  // syntax of Annotations, so use individual points.
  auto Input = Annotations(
      R"(void target($whole[[void (*$1^(*$p[[(P)]])[]$2^)(int)]]);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(1, 0, Input.range("whole")),
                      complexDeclarator("(P)", {Input.range("p")})),
                AllOf(eligibleRange(std::nullopt, 1, Input.range("whole")),
                      complexDeclarator(
                          "(*)[]", {Annotations::Range(Input.point("1"),
                                                       Input.range("p").Begin),
                                    Annotations::Range(Input.range("p").End,
                                                       Input.point("2"))})))));
}

MATCHER_P(eligibleRange, Expected, "") {
  return ExplainMatchResult(Expected.Slot, arg.Slot, result_listener) &&
         ExplainMatchResult(EqualsProto(Expected.Range), arg.Range,
                            result_listener);
}

template <typename DeclT, typename MatcherT>
std::vector<testing::Matcher<EligibleRange>> rangesFor(
    std::vector<std::pair<std::string, MatcherT>> DeclMatchers, ASTContext &Ctx,
    const TypeNullabilityDefaults &Defaults) {
  std::vector<testing::Matcher<EligibleRange>> RangeMatchers;
  for (const auto &[Name, Matcher] : DeclMatchers) {
    llvm::errs() << "Getting ranges for " << Name << "\n";
    EligibleRanges Ranges = getRanges<DeclT>(Ctx, Matcher, Defaults);
    if (Ranges.empty()) {
      ADD_FAILURE() << "No ranges found for " << Name << "!";
      return {};
    }
    for (const EligibleRange &ER : Ranges) {
      RangeMatchers.push_back(eligibleRange(ER));
    }
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

  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range("param")),
                                 eligibleRange(0, 0, Input.range("return")))));
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
    struct _Nullable custom_smart_ptr {
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

  // No ranges without an instantiation.
  EXPECT_THAT(getEligibleRanges(TU.context(), Defaults), IsEmpty());
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

  auto &InstantiationDecl = *selectFirst<ClassTemplateSpecializationDecl>(
      "b",
      match(classTemplateSpecializationDecl(hasName("CTemplate")).bind("b"),
            TU.context()));

  // Matches the ranges for decls in the instantiation.
  std::vector<testing::Matcher<EligibleRange>> Expected;
  for (const EligibleRange &ER : getEligibleRanges(
           *selectFirst<FunctionDecl>(
               "b",
               match(findAll(functionDecl(hasName("methodWithPtr")).bind("b")),
                     InstantiationDecl, TU.context())),
           Defaults)) {
    Expected.push_back(eligibleRange(ER));
  }
  for (const EligibleRange &ER : getEligibleRanges(
           *selectFirst<FieldDecl>(
               "b", match(findAll(fieldDecl(hasName("PtrField")).bind("b")),
                          InstantiationDecl, TU.context())),
           Defaults)) {
    Expected.push_back(eligibleRange(ER));
  }
  for (const EligibleRange &ER : getEligibleRanges(
           *selectFirst<VarDecl>(
               "b", match(findAll(varDecl(hasName("StaticField")).bind("b")),
                          InstantiationDecl, TU.context())),
           Defaults)) {
    Expected.push_back(eligibleRange(ER));
  }
  EXPECT_THAT(getEligibleRanges(TU.context(), Defaults),
              UnorderedElementsAreArray(Expected));
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

  auto &ExplicitSpecialization = *selectFirst<ClassTemplateSpecializationDecl>(
      "b", match(classTemplateSpecializationDecl().bind("b"), TU.context()));

  // Matches only the ranges in the explicit specialization.
  std::vector<testing::Matcher<EligibleRange>> Expected;
  for (const EligibleRange &ER : getEligibleRanges(
           *selectFirst<FunctionDecl>(
               "b",
               match(findAll(functionDecl(hasName("methodWithPtr")).bind("b")),
                     ExplicitSpecialization, TU.context())),
           Defaults)) {
    Expected.push_back(eligibleRange(ER));
  }
  for (const EligibleRange &ER : getEligibleRanges(
           *selectFirst<FieldDecl>(
               "b", match(findAll(fieldDecl(hasName("PtrField")).bind("b")),
                          ExplicitSpecialization, TU.context())),
           Defaults)) {
    Expected.push_back(eligibleRange(ER));
  }
  for (const EligibleRange &ER : getEligibleRanges(
           *selectFirst<VarDecl>(
               "b", match(findAll(varDecl(hasName("StaticField")).bind("b")),
                          ExplicitSpecialization, TU.context())),
           Defaults)) {
    Expected.push_back(eligibleRange(ER));
  }
  for (const EligibleRange &ER : getEligibleRanges(
           *selectFirst<FieldDecl>(
               "b",
               match(findAll(fieldDecl(hasName("ExtraFieldInSpecialization"))
                                 .bind("b")),
                     ExplicitSpecialization, TU.context())),
           Defaults)) {
    Expected.push_back(eligibleRange(ER));
  }
  EXPECT_THAT(getEligibleRanges(TU.context(), Defaults),
              UnorderedElementsAreArray(Expected));
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

  EXPECT_THAT(getEligibleRanges(TU.context(), Defaults), IsEmpty());
}

TEST(GetEligibleRangesFromASTTest, FunctionTemplateHasInstantiation) {
  auto Input = Annotations(R"(
    template <typename T>
    int funcTemplate($param[[T*]] A, T B) {
      $local[[T*]] LocalPointerInTemplate;
      T LocalInTemplate;
      return 0;
    }

    int I = funcTemplate<int>(nullptr, 0);
  )");

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input.code(), Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  auto &InstantiationDecl = *selectFirst<FunctionDecl>(
      "b",
      match(functionDecl(isTemplateInstantiation(), hasName("funcTemplate"))
                .bind("b"),
            TU.context()));

  std::vector<testing::Matcher<EligibleRange>> Expected;
  for (const EligibleRange &ER : getEligibleRanges(
           *selectFirst<FunctionDecl>(
               "b",
               match(findAll(functionDecl(hasName("funcTemplate")).bind("b")),
                     InstantiationDecl, TU.context())),
           Defaults)) {
    Expected.push_back(eligibleRange(ER));
  }
  for (const EligibleRange &ER : getEligibleRanges(
           *selectFirst<VarDecl>(
               "b",
               match(findAll(
                         varDecl(hasName("LocalPointerInTemplate")).bind("b")),
                     InstantiationDecl, TU.context())),
           Defaults)) {
    Expected.push_back(eligibleRange(ER));
  }
  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      AllOf(UnorderedElementsAreArray(Expected),
            Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, 0, Input.range("param")),
                                 eligibleRange(0, 0, Input.range("local")))));
}

TEST(GetEligibleRangesFromASTTest, AutoFunctionTemplateSyntax) {
  auto Input = Annotations(R"(
    void funcTemplate(auto P, $star[[auto*]] Q) {}

    void usage() {
      $local_one[[int*]] A = nullptr;
      $local_two[[int*]] B = nullptr;
      funcTemplate(A, B);
    }
  )");

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input.code(), Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(2, 0, Input.range("star")),
                               eligibleRange(0, 0, Input.range("local_one")),
                               eligibleRange(0, 0, Input.range("local_two")))));
}

TEST(skipEscapedNewLinePrefixesTest, NoPrefix) {
  EXPECT_EQ(skipEscapedNewLinePrefixes(""), "");
  EXPECT_EQ(skipEscapedNewLinePrefixes("foo"), "foo");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\too"), "\too");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\too"), "\\too");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\foo"), "\foo");
}

TEST(skipEscapedNewLinePrefixesTest, EscapedNewlinePresent) {
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\\nfoo"), "foo");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\\rfoo"), "foo");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\\n foo"), " foo");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\  \t \nfoo"), "foo");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\     \nfoo"), "foo");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\     \n\rfoo"), "foo");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\     \r\nfoo"), "foo");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\     \n\nfoo"), "\nfoo");
  EXPECT_EQ(skipEscapedNewLinePrefixes("\\     \r\n\\\r\n\\   \r\nfoo"), "foo");
}
}  // namespace
}  // namespace clang::tidy::nullability
