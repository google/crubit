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
#include "nullability/inference/eligible_ranges_for_test.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/pragma.h"
#include "nullability/proto_matchers.h"
#include "nullability/type_nullability.h"
#include "clang/include/clang/AST/Decl.h"
#include "clang/include/clang/AST/DeclTemplate.h"
#include "clang/include/clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/include/clang/ASTMatchers/ASTMatchers.h"
#include "clang/include/clang/Basic/LLVM.h"
#include "clang/include/clang/Testing/TestAST.h"
#include "llvm/include/llvm/ADT/StringRef.h"
#include "llvm/include/llvm/Support/raw_ostream.h"
#include "llvm/include/llvm/Testing/Annotations/Annotations.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

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
using ::testing::ExplainMatchResult;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::Pointwise;
using ::testing::UnorderedElementsAre;
using ::testing::UnorderedElementsAreArray;

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

// Used in matcher message strings below, which is not detected.
[[maybe_unused]] std::string printSlot(unsigned Slot) {
  return absl::StrCat(Slot);
}

// Used in matcher message strings below, which is not detected.
[[maybe_unused]] std::string printSlot(std::optional<Slot> Slot) {
  if (Slot) return absl::StrCat(*Slot);
  return "nullopt";
}

MATCHER_P2(eligibleRange, SlotInDecl, InsertionOffset,
           absl::StrCat("has a SlotRange for slot ", printSlot(SlotInDecl),
                        " with insertion offset at ", InsertionOffset)) {
  std::optional<Slot> Slot = arg.Slot;
  const SlotRange &ArgRange = arg.Range;
  return ExplainMatchResult(SlotInDecl, Slot, result_listener) &&
         ExplainMatchResult(InsertionOffset,
                            ArgRange.qualifier_annotation_insertion_offset(),
                            result_listener);
}

MATCHER_P2(eligibleRangeWithNoExistingAnnotation, SlotInDecl, InsertionOffset,
           "") {
  return ExplainMatchResult(false, arg.Range.has_existing_annotation(),
                            result_listener) &&
         ExplainMatchResult(eligibleRange(SlotInDecl, InsertionOffset), arg,
                            result_listener);
}

MATCHER_P3(eligibleRange, SlotInDecl, InsertionOffset, ExistingAnnotation,
           absl::StrCat("has a SlotRange for slot ", printSlot(SlotInDecl),
                        " with insertion offset at ", InsertionOffset,
                        " and existing annotation ",
                        Nullability_Name(ExistingAnnotation))) {
  return ExplainMatchResult(eligibleRange(SlotInDecl, InsertionOffset), arg,
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

// Matcher for 2-tuple where the first element is expected to be a RemovalRange
// and the second an Annotations::Range.
MATCHER(areEquivalentRanges, "") {
  return std::get<0>(arg).begin() == std::get<1>(arg).Begin &&
         std::get<0>(arg).end() == std::get<1>(arg).End;
}

MATCHER_P(hasRemovalRanges, RemovalRanges, "") {
  return ExplainMatchResult(Pointwise(areEquivalentRanges(), RemovalRanges),
                            arg.Range.existing_annotation_removal_range(),
                            result_listener);
}

auto removalRanges(std::vector<Annotations::Range> RemovalRanges) {
  return hasRemovalRanges(RemovalRanges);
}

// Convenience version of `removalRanges({})`, which makes it more clear that
// the argument is intentionally empty and has better messaging during failure.
MATCHER(hasNoRemovalRanges, "") {
  return ExplainMatchResult(IsEmpty(),
                            arg.Range.existing_annotation_removal_range(),
                            result_listener);
}

TEST(EligibleRangesTest, RawPointersReturnsAndParametersHaveRanges) {
  auto Input =
      Annotations("int *$r^target(int *$p^P, bool *$q^Q) { return P; }");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRangeWithNoExistingAnnotation(0, Input.point("r")),
                eligibleRangeWithNoExistingAnnotation(1, Input.point("p")),
                eligibleRangeWithNoExistingAnnotation(2, Input.point("q")))));
}

TEST(EligibleRangesTest, NonPointerReturnsAndParametersHaveNoRanges) {
  auto Input = Annotations("void target(int *^P1, int P2) { return; }");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, DeclWithoutBodyHasRanges) {
  auto Input = Annotations("void target(int *^P1, int P2);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, AllNestedPointersEligible) {
  auto Input =
      Annotations("void target(int *$inner^*$middle^*$outer^P1, int P2);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("outer")),
                eligibleRange(std::nullopt, Input.point("middle")),
                eligibleRange(std::nullopt, Input.point("inner")))));
}

TEST(EligibleRangesTest, DeclConstFollowsInsertionOffset) {
  auto Input = Annotations(R"(
  void target(int *$one^ const P1,
           int *$two_i^ const *$two_o^ const P2);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("one")),
                eligibleRange(2, Input.point("two_o")),
                eligibleRange(std::nullopt, Input.point("two_i")))));
}

TEST(EligibleRangesTest, NestedPointeeConst) {
  auto Input = Annotations("void target(const int *$i^ const *$o^P);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(1, Input.point("o")),
                               eligibleRange(std::nullopt, Input.point("i")))));
}

TEST(EligibleRangesTest, NamespacedAliasAnnotatedSlotsGetRanges) {
  auto Input = Annotations(R"(
  namespace custom {
  namespace internal {
  template <typename T>
  using CustomNonnull = absl_nonnull T;
  template <typename T>
  using CustomNullable = absl_nullable T;
  template <typename T>
  using CustomUnknown = absl_nullability_unknown T;
  }
  
  template <typename T>
  using CustomNonnull = internal::CustomNonnull<T>;
  template <typename T>
  using CustomNullable = internal::CustomNullable<T>;
  template <typename T>
  using CustomUnknown = internal::CustomUnknown<T>;
  }

  // Aliases of any depth that apply a nullability annotation are detected.
  void target(custom::CustomNonnull<int *$one^> NonnullP,
           custom::CustomNullable<int *$two^> NullableP,
           custom::CustomUnknown<int *$three^> UnknownP);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("one"), Nullability::NONNULL),
                eligibleRange(2, Input.point("two"), Nullability::NULLABLE),
                eligibleRange(3, Input.point("three"), Nullability::UNKNOWN))));
}

TEST(EligibleRangesTest, DuplicateAnnotationsGetOneRange) {
  auto Input =
      Annotations(R"(void target(int *^ absl_nonnull absl_nonnull P);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point(), Nullability::NONNULL))));
}

TEST(EligibleRangesTest, NestedPointersOuterAnnotated) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }
  void target(
      int *$one_i^*$one_o^ absl_nonnull P,
      absl_nonnull $two_o^std::unique_ptr<int*$two_i^> Q,
      $three_i^std::unique_ptr<int>*$three_o^ absl_nonnull R,
      absl_nonnull $four_o^std::unique_ptr<$four_i^std::unique_ptr<int>> S);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("one_o"), Nullability::NONNULL),
                eligibleRange(std::nullopt, Input.point("one_i")),
                eligibleRange(2, Input.point("two_o"), Nullability::NONNULL),
                eligibleRange(std::nullopt, Input.point("two_i")),
                eligibleRange(3, Input.point("three_o"), Nullability::NONNULL),
                eligibleRange(std::nullopt, Input.point("three_i")),
                eligibleRange(4, Input.point("four_o"), Nullability::NONNULL),
                eligibleRange(std::nullopt, Input.point("four_i")))));
}

TEST(EligibleRangesTest, NestedPointersInnerAnnotated) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target(
      int *$one_i^absl_nonnull *$one_o^P,
      $two_o^std::unique_ptr<int *$two_i^absl_nonnull> Q,
      absl_nonnull $three_i^std::unique_ptr<int> *$three_o^R,
      $four_o^std::unique_ptr<absl_nonnull $four_i^std::unique_ptr<int>> S);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("one_o")),
                eligibleRange(std::nullopt, Input.point("one_i"),
                              Nullability::NONNULL),
                eligibleRange(2, Input.point("two_o")),
                eligibleRange(std::nullopt, Input.point("two_i"),
                              Nullability::NONNULL),
                eligibleRange(3, Input.point("three_o")),
                eligibleRange(std::nullopt, Input.point("three_i"),
                              Nullability::NONNULL),
                eligibleRange(4, Input.point("four_o")),
                eligibleRange(std::nullopt, Input.point("four_i"),
                              Nullability::NONNULL))));
}

TEST(EligibleRangesTest, RefToPointer) {
  auto Input = Annotations("void target(int *^&P);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, TemplateOfPointers) {
  auto Input = Annotations(R"(
  template <typename One, typename Two>
  struct S {}; 

  void target(S<int *$one^, bool *$two_inner^*$two^> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("one")),
                eligibleRange(std::nullopt, Input.point("two")),
                eligibleRange(std::nullopt, Input.point("two_inner")))));
}

TEST(EligibleRangesTest, TemplateOfConstPointers) {
  auto Input = Annotations(R"(
  template <typename One, typename Two>
  struct S {};

  void target(
      S<const int *$one^, const int *$two_i^ const *$two_o^> P,
      S<int *$three^ const, int *$four_i^ const *$four_o^ const> Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("one")),
                eligibleRange(std::nullopt, Input.point("two_o")),
                eligibleRange(std::nullopt, Input.point("two_i")),
                eligibleRange(std::nullopt, Input.point("three")),
                eligibleRange(std::nullopt, Input.point("four_o")),
                eligibleRange(std::nullopt, Input.point("four_i")))));
}

TEST(EligibleRangesTest, SmartPointer) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target($one^std::unique_ptr<int> StdSmart,
           absl_nonnull $two^std::unique_ptr<int> NonnullStdSmart);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("one")),
                eligibleRange(2, Input.point("two"), Nullability::NONNULL))));
}

TEST(EligibleRangesTest,
     SmartPointerPrecedingDeclConstPrecedesInsertionOffset) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target(const ^std::unique_ptr<int> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, UserDefinedSmartPointer) {
  auto Input = Annotations(R"(
  struct _Nullable MySmartIntPtr {
    using pointer = int *;
  };

  void target($one^MySmartIntPtr UserDefinedSmart,
           absl_nonnull $two^MySmartIntPtr NonnullUserDefinedSmart);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("one")),
                eligibleRange(2, Input.point("two"), Nullability::NONNULL))));
}

TEST(EligibleRangesTest, UserDefinedTemplatedSmartPointer) {
  auto Input = Annotations(R"(
  template <typename T>
  struct _Nullable MySmartPtr {};

  void target($one^MySmartPtr<int> UserDefinedSmart,
           absl_nonnull $two^MySmartPtr<int> NonnullUserDefinedSmart);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("one")),
                eligibleRange(2, Input.point("two"), Nullability::NONNULL))));
}

TEST(EligibleRangesTest, SimpleAlias) {
  auto Input = Annotations(R"(
  using IntPtr = int *;

  void target(^IntPtr P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
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

  void target(^Nested P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, AliasTemplate) {
  auto Input = Annotations(R"(
  template <typename T>
  using AliasTemplate = T;

  void target(AliasTemplate<int*^> P, AliasTemplate<int> Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, DependentAliasSimple) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  template <typename T>
  struct S {
    using Type = T;
  };

  void target(S<int *$raw^>::Type P, S<int>::Type Q, S<$smart^std::unique_ptr<int>>::Type R);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("raw")),
                                 eligibleRange(3, Input.point("smart")))));
}

TEST(EligibleRangesTest, DependentAliasAnnotated) {
  auto Input = Annotations(R"(
  template <typename T>
  struct S {
    using type = T;
  };

  void target(S<int *$macro^absl_nullable>::type P, S<int *$qualifier^ _Nullable>::type Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("macro"), Nullability::NULLABLE),
                eligibleRange(2, Input.point("qualifier"),
                              Nullability::NULLABLE))));
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

  void target(S<int *^>::type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
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

  void target(S<int*^>::type<vector> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(std::nullopt, Input.point()))));
}

TEST(EligibleRangesTest, DependentAliasNested) {
  auto Input = Annotations(R"(
  template <typename V>
  struct vector {
    using value_type = V;
  };

  void target(vector<int *$inner^*$middle^*$outer^>::value_type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("outer")),
                eligibleRange(std::nullopt, Input.point("middle")),
                eligibleRange(std::nullopt, Input.point("inner")))));
}

TEST(EligibleRangesTest, NoreturnAliasLosesFunctionTypeSourceInfo) {
  // This previously crashed because the noreturn attribute causes the
  // TypedefType to be unwrapped and rewritten without the Typedef layer and
  // the source information below that layer to be dropped.
  //
  // Because we see it as a function pointer type, we place the annotation
  // following the type, which is allowed, but not our preferred style.
  auto Input = Annotations(R"(
    typedef void (*Alias)(const char *, ...);

    __attribute__((__noreturn__)) Alias^ Target;
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(0, Input.point("")))));
}

TEST(EligibleRangesTest, TemplatedClassContext) {
  auto Input = Annotations(R"(
  template <typename T>
  struct Outer {
    struct Inner {};
  };

  void target(Outer<int *^>::Inner P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(std::nullopt, Input.point()))));
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
      Outermost<char *$zero^>::Outer<int *$one^>::Inner<bool *$two^> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("zero")),
                eligibleRange(std::nullopt, Input.point("one")),
                eligibleRange(std::nullopt, Input.point("two")))));
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

  void target(Outermost<int*^>::Outer<bool>::Inner<char*> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, DependentAliasForwardingMultipleTemplateArguments) {
  auto Input = Annotations(R"(
  template <typename T, class U>
  struct Pair;
  template <typename T, class U>
  struct PairWrapper {
    using type = Pair<T , U>;
  };

  void target(PairWrapper<int *$zero^, bool *$one^>::type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("zero")),
                eligibleRange(std::nullopt, Input.point("one")))));
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

  void target(Outer<int *$zero^>::Inner<bool *$one^>::type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("zero")),
                eligibleRange(std::nullopt, Input.point("one")))));
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

  void target(Outer<int *$zero^>::Inner<bool *$one^> P);
  )");

  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("zero")),
                eligibleRange(std::nullopt, Input.point("one")))));
}

TEST(EligibleRangesTest, DependentAliasWrappingTemplateArg) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  template <typename T>
  struct S {
    using type = std::unique_ptr<T>;
  };

  void target($unique_ptr_outer^S<int *$inner^>::type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("unique_ptr_outer")),
                eligibleRange(std::nullopt, Input.point("inner")))));
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

  // P's canonical type is int**. The outer pointer's source is the whole type,
  // and the inner pointer's source is the first template argument to S.
  void target($outer^S<int *$inner^, Wrapper>::type P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("outer")),
                eligibleRange(std::nullopt, Input.point("inner")))));
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
  void target(^S<int **>::Alias P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, TypeTemplateParamPack) {
  auto Input = Annotations(R"(
  template <typename... T>
  struct Tuple {
    using type = int;
  };

  void target(Tuple<int *$zero^, int *$two^*$one^> P,
           Tuple<int *, int **>::type Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("zero")),
                eligibleRange(std::nullopt, Input.point("one")),
                eligibleRange(std::nullopt, Input.point("two")))));
}

TEST(EligibleRangesTest, DefaultTemplateArgs) {
  auto Input = Annotations(R"(
  template <typename T1, typename T2 = int*>
  struct S {};
  template <typename T1, typename T2 = T1>
  using Alias = T2;

  void target(S<int *$one^> P, Alias<int *$two^> Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("one")),
                eligibleRange(2, Input.point("two")))));
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

  void target(Couple<int *^> P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            // Eventually, two different valid slot values for the two
            // ranges, but for now, inference looks at neither of
            // them, so both have no slot.
            UnorderedElementsAre(eligibleRange(std::nullopt, Input.point()),
                                 eligibleRange(std::nullopt, Input.point()))));
}

TEST(EligibleRangesTest, Field) {
  auto Input = Annotations(R"(
  struct S {
    int *$one^*$zero^ Target;
  };
  )");
  EXPECT_THAT(
      getFieldRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, Input.point("zero")),
                eligibleRange(std::nullopt, Input.point("one")))));
}

TEST(EligibleRangesTest, StaticFieldAkaGlobal) {
  auto Input = Annotations(R"(
  struct S {
    static int *$one^*$zero^ Target;
  };
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, Input.point("zero")),
                eligibleRange(std::nullopt, Input.point("one")))));
}

TEST(EligibleRangesTest, GlobalVariable) {
  auto Input = Annotations(R"(
    int *$one^*$zero^ Target;
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, Input.point("zero")),
                eligibleRange(std::nullopt, Input.point("one")))));
}

TEST(EligibleRangesTest, Lambda) {
  auto Input = Annotations(R"(
  auto Lambda = [](int *$one^) -> int *$zero^ {};
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "operator()"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(0, Input.point("zero")),
                                 eligibleRange(1, Input.point("one")))));
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

  int *$one^*$zero^ target(int *$param_one^, int *$param_two^);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName),
                     hasPragmaNullability(Nullability::NONNULL))),
          UnorderedElementsAre(
              eligibleRange(0, Input.point("zero"), Nullability::NONNULL),
              eligibleRange(std::nullopt, Input.point("one"),
                            Nullability::NONNULL),
              eligibleRange(1, Input.point("param_one"), Nullability::NONNULL),
              eligibleRange(2, Input.point("param_two"),
                            Nullability::NONNULL))));

  Input = Annotations(R"(
  #pragma nullability file_default nullable
  int*^ Target;
  )");
  EXPECT_THAT(getVarRanges(Input.code()),
              AllOf(Each(AllOf(hasPath(MainFileName),
                               hasPragmaNullability(Nullability::NULLABLE))),
                    UnorderedElementsAre(eligibleRange(
                        0, Input.point(), Nullability::NULLABLE))));

  Input = Annotations(R"(
  int*^ Target;
  )");
  EXPECT_THAT(
      getVarRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRangeWithNoExistingAnnotation(0, Input.point()))));
}

TEST(EligibleRangesTest, RangesEntirelyWithinMacro) {
  auto Input = Annotations(R"(
  #define MACRO(IpGlobalVar, funcName) int *$within^ funcName##Func() {return IpGlobalVar;}
  int* GlobalVar1;
  int* GlobalVar2;
  MACRO(GlobalVar1, getVar1);
  MACRO(GlobalVar2, getVar2);

  #define A_TYPE int*
  A_TYPE$whole_type_1^ GlobalVar3;
  A_TYPE$whole_type_2^ GlobalVar4;
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "getVar1Func"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(0, Input.point("within")))));
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "getVar2Func"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(0, Input.point("within")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GlobalVar3"),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(0, Input.point("whole_type_1")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GlobalVar4"),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(0, Input.point("whole_type_2")))));
}

// We saw specific difficulty finding previous tokens during our search for
// `const`s when passing over the line continuation in multi-line macros.
TEST(EligibleRangesTest, RangesInMultiLineMacro) {
  auto Input = Annotations(R"(
  using IntPtr = int*;

  #define MACRO(IpGlobalVar, funcName) int *$return^ funcName##Func(                             \
                                        const char*$param_one^ p,                                \
                                        const                                                    \
                                        int*$param_two^ p2,                                      \
                                        $removal_before_3[[absl_nonnull ]]const                  \
                                                     $param_three^IntPtr p3)                     \
                                           {return IpGlobalVar;}
  int* GlobalVar;
  MACRO(GlobalVar, getVar);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "getVarFunc"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, Input.point("return")),
                eligibleRange(1, Input.point("param_one")),
                eligibleRange(2, Input.point("param_two")),
                AllOf(eligibleRange(3, Input.point("param_three"),
                                    Nullability::NONNULL),
                      removalRanges({Input.range("removal_before_3")})))));
}

// We saw specific difficulty finding starts of tokens that begin with no
// unescaped whitespace between the `\` and the token's first character. Clang
// considers the tokens to begin at the preceding `\` location. Any oddities
// produced by edits using these locations are fixed by auto-formatting after
// applying edits.
TEST(EligibleRangesTest, RangesInMultiLineMacroNoIndentation) {
  auto Input = Annotations(R"(
  using IntPtr = int*;

  #define MACRO(IpGlobalVar, funcName) int *$return^ funcName##Func( $removal_before_west[[\
absl_nonnull                                                                             ]]\
const                                                                          $param_west^\
IntPtr                                                                                     \
p,                                                                                         \
int*$param_east^                                                                           \
const$removal_before_east[[                                                                \
absl_nullable]]                                                                            \
p2)                                                                                        \
    {return IpGlobalVar;}

  int* GlobalVar;
  MACRO(GlobalVar, getVar);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "getVarFunc"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(0, Input.point("return")),
                AllOf(eligibleRange(1, Input.point("param_west"),
                                    Nullability::NONNULL),
                      removalRanges({Input.range("removal_before_west")})),
                AllOf(eligibleRange(2, Input.point("param_east"),
                                    Nullability::NULLABLE),
                      removalRanges({Input.range("removal_before_east")})))));
}

TEST(EligibleRangesTest, BareAutoTypeGetsNoRanges) {
  auto Input = Annotations(R"(
    auto$func_auto^
        noStar(int* P) {
          P = nullptr;
          return P;
        }

        int* getPtr();
    auto GNoStar = getPtr();
    auto _Nullable GNoStarNullable = getPtr();
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "noStar"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            Not(Contains(eligibleRangeWithNoExistingAnnotation(
                0, Input.point("func_auto"))))));
  EXPECT_THAT(getVarRanges(Input.code(), "GNoStar"), IsEmpty());
  EXPECT_THAT(getVarRanges(Input.code(), "GNoStarNullable"), IsEmpty());
}

TEST(EligibleRangesTest, BareAutoAsTemplateParameterGetsNoRanges) {
  auto Input = Annotations(R"(
    namespace std {
    template <typename T>
    class unique_ptr;
    }

    void func(auto P, auto& Q) { }

    void lambdaRecipient(void (*$A^A)(const $B^std::unique_ptr<int>& B)) {}

    void usage() {
      int *$I^ I;
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
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("A")),
                                 eligibleRange(std::nullopt, Input.point("B")),
                                 eligibleRange(0, Input.point("I")))));
}

TEST(EligibleRangesTest, AutoStarGetsRanges) {
  auto Input = Annotations(R"(
     auto*$func_auto^ star(int*$func_not_auto^ P) {
      P = nullptr;
      return P;
    }

    int* getPtr();
    auto*$var_auto^ GStar = getPtr();
    auto*$var_auto_const^ const GStarConst = getPtr();
    const auto*$var_const_auto^  GConstStar = getPtr();
    auto*$var_auto_const_ref^ const& GStarConstRef = getPtr();
    const auto*$var_const_auto_ref^& GConstStarRef = GConstStar;
    auto*$var_auto_attributed^ _Nullable GStarNullable = getPtr();
    auto*$var_auto_star_inner^*$var_auto_star_star^ GStarStar = &GStar;
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code(), "star"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRangeWithNoExistingAnnotation(
                                     0, Input.point("func_auto")),
                                 eligibleRangeWithNoExistingAnnotation(
                                     1, Input.point("func_not_auto")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStar"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRangeWithNoExistingAnnotation(
                0, Input.point("var_auto")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStarConst"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRangeWithNoExistingAnnotation(
                0, Input.point("var_auto_const")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GConstStar"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRangeWithNoExistingAnnotation(
                0, Input.point("var_const_auto")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStarConstRef"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRangeWithNoExistingAnnotation(
                0, Input.point("var_auto_const_ref")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GConstStarRef"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRangeWithNoExistingAnnotation(
                0, Input.point("var_const_auto_ref")))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStarNullable"),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(eligibleRange(
              0, Input.point("var_auto_attributed"), Nullability::NULLABLE))));
  EXPECT_THAT(
      getVarRanges(Input.code(), "GStarStar"),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRangeWithNoExistingAnnotation(
                    0, Input.point("var_auto_star_star")),
                eligibleRangeWithNoExistingAnnotation(
                    std::nullopt, Input.point("var_auto_star_inner")))));
}

TEST(RemovalRangesTest, AnnotationInMacro) {
  auto Input = Annotations(R"(

  #define UNKNOWN(T) T absl_nullability_unknown

  void target(UNKNOWN(int *^) P);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(1, Input.point("")),
                      // The token checks looking for annotations are done
                      // without expansion of macros, so we see a left
                      // paren as the preceding token and report no
                      // existing ranges to remove.
                      hasNoRemovalRanges()))));
}

TEST(RemovalRangesTest, ClangAttribute) {
  auto Input = Annotations(R"(
  void target(int *$no^ P, int *$yes^$yes_removal[[ _Null_unspecified]] Q,
              int*$no_space^$no_space_removal[[_Null_unspecified]] R,
              int*$with_comment^$with_comment_removal[[/* a comment */_Null_unspecified]] S,
              int *$nullable^$nullable_removal[[ _Nullable]] T,
              int *$nonnull^$nonnull_removal[[ _Nonnull]] U);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, Input.point("no")), hasNoRemovalRanges()),
              AllOf(eligibleRange(2, Input.point("yes")),
                    removalRanges({Input.range("yes_removal")})),
              AllOf(eligibleRange(3, Input.point("no_space")),
                    removalRanges({Input.range("no_space_removal")})),
              AllOf(eligibleRange(4, Input.point("with_comment")),
                    removalRanges({Input.range("with_comment_removal")})),
              AllOf(eligibleRange(5, Input.point("nullable")),
                    removalRanges({Input.range("nullable_removal")})),
              AllOf(eligibleRange(6, Input.point("nonnull")),
                    removalRanges({Input.range("nonnull_removal")})))));
}

TEST(RemovalRangesTest, ClangAttributeSmartPointersEast) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target($no^std::unique_ptr<int> P,
              $yes^std::unique_ptr<int>$yes_removal[[ _Null_unspecified]] Q,
              $no_space^std::unique_ptr<int>$no_space_removal[[_Null_unspecified]] R,
              $with_comment^std::unique_ptr<int>$with_comment_removal[[/* a comment */_Null_unspecified]] S,
              $nullable^std::unique_ptr<int>$nullable_removal[[ _Nullable]] T,
              $nonnull^std::unique_ptr<int>$nonnull_removal[[ _Nonnull]] U);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, Input.point("no")), hasNoRemovalRanges()),
              AllOf(eligibleRange(2, Input.point("yes")),
                    removalRanges({Input.range("yes_removal")})),
              AllOf(eligibleRange(3, Input.point("no_space")),
                    removalRanges({Input.range("no_space_removal")})),
              AllOf(eligibleRange(4, Input.point("with_comment")),
                    removalRanges({Input.range("with_comment_removal")})),
              AllOf(eligibleRange(5, Input.point("nullable")),
                    removalRanges({Input.range("nullable_removal")})),
              AllOf(eligibleRange(6, Input.point("nonnull")),
                    removalRanges({Input.range("nonnull_removal")})))));
}

TEST(RemovalRangesTest, ClangAttributeSmartPointersWest) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target($no^std::unique_ptr<int> P,
              $yes_removal[[_Null_unspecified ]]$yes^std::unique_ptr<int> Q,
              $with_comment_removal[[_Null_unspecified/* a comment */]]$with_comment^std::unique_ptr<int> R,
              $nullable_removal[[_Nullable ]]$nullable^std::unique_ptr<int> S,
              $nonnull_removal[[_Nonnull ]]$nonnull^std::unique_ptr<int> T);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, Input.point("no")), hasNoRemovalRanges()),
              AllOf(eligibleRange(2, Input.point("yes")),
                    removalRanges({Input.range("yes_removal")})),
              AllOf(eligibleRange(3, Input.point("with_comment")),
                    removalRanges({Input.range("with_comment_removal")})),
              AllOf(eligibleRange(4, Input.point("nullable")),
                    removalRanges({Input.range("nullable_removal")})),
              AllOf(eligibleRange(5, Input.point("nonnull")),
                    removalRanges({Input.range("nonnull_removal")})))));
}

TEST(RemovalRangesTest, ClangAttributeSmartPointersWithNearerConst) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target($const_west_removal[[_Nullable ]]const $insert_const_west^std::unique_ptr<int> P,
              $insert_const_east^std::unique_ptr<int> const$const_east_removal[[ _Nullable]] Q,
              $insert_const_east_no_space^std::unique_ptr<int>const$const_east_removal_no_space[[ _Nullable]] R,
              $insert_const_east_comment^std::unique_ptr<int>/* a comment */const$const_east_removal_comment[[ _Nullable]] S);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, Input.point("insert_const_west")),
                    removalRanges({Input.range("const_west_removal")})),
              AllOf(eligibleRange(2, Input.point("insert_const_east")),
                    removalRanges({Input.range("const_east_removal")})),
              AllOf(
                  eligibleRange(3, Input.point("insert_const_east_no_space")),
                  removalRanges({Input.range("const_east_removal_no_space")})),
              AllOf(eligibleRange(4, Input.point("insert_const_east_comment")),
                    removalRanges(
                        {Input.range("const_east_removal_comment")})))));
}

TEST(RemovalRangesTest, ClangAttributeWithFunctionPointer) {
  auto Input = Annotations(R"(
  void target(void (*^[[ _Nonnull]] P)(int));
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(1, Input.point(""), Nullability::NONNULL),
                      removalRanges({Input.range("")})))));
}

TEST(RemovalRangesTest, AbslMacro) {
  auto Input = Annotations(R"(
  void target(int *$no^ P, int *$yes^$yes_removal[[ absl_nullability_unknown]] Q,
              int*$no_space^$no_space_removal[[absl_nullability_unknown]] R,
              int*$with_comment^$with_comment_removal[[/* a comment */absl_nullability_unknown]] S,
              int *$nullable^$nullable_removal[[ absl_nullable]] T,
              int *$nonnull^$nonnull_removal[[ absl_nonnull]] U);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, Input.point("no")), hasNoRemovalRanges()),
              AllOf(eligibleRange(2, Input.point("yes")),
                    removalRanges({Input.range("yes_removal")})),
              AllOf(eligibleRange(3, Input.point("no_space")),
                    removalRanges({Input.range("no_space_removal")})),
              AllOf(eligibleRange(4, Input.point("with_comment")),
                    removalRanges({Input.range("with_comment_removal")})),
              AllOf(eligibleRange(5, Input.point("nullable")),
                    removalRanges({Input.range("nullable_removal")})),
              AllOf(eligibleRange(6, Input.point("nonnull")),
                    removalRanges({Input.range("nonnull_removal")})))));
}

TEST(RemovalRangesTest, AbslMacroSmartPointersEast) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target($no^std::unique_ptr<int> P,
              $yes^std::unique_ptr<int>$yes_removal[[ absl_nullability_unknown]] Q,
              $no_space^std::unique_ptr<int>$no_space_removal[[absl_nullability_unknown]] R,
              $with_comment^std::unique_ptr<int>$with_comment_removal[[/* a comment */absl_nullability_unknown]] S,
              $nullable^std::unique_ptr<int>$nullable_removal[[ absl_nullable]] T,
              $nonnull^std::unique_ptr<int>$nonnull_removal[[ absl_nonnull]] U);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, Input.point("no")), hasNoRemovalRanges()),
              AllOf(eligibleRange(2, Input.point("yes")),
                    removalRanges({Input.range("yes_removal")})),
              AllOf(eligibleRange(3, Input.point("no_space")),
                    removalRanges({Input.range("no_space_removal")})),
              AllOf(eligibleRange(4, Input.point("with_comment")),
                    removalRanges({Input.range("with_comment_removal")})),
              AllOf(eligibleRange(5, Input.point("nullable")),
                    removalRanges({Input.range("nullable_removal")})),
              AllOf(eligibleRange(6, Input.point("nonnull")),
                    removalRanges({Input.range("nonnull_removal")})))));
}

TEST(RemovalRangesTest, AbslMacroSmartPointersWest) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target($no^std::unique_ptr<int> P,
              $yes_removal[[absl_nullability_unknown ]]$yes^std::unique_ptr<int> Q,
              $with_comment_removal[[absl_nullability_unknown/* a comment */]]$with_comment^std::unique_ptr<int> R,
              $nullable_removal[[absl_nullable ]]$nullable^std::unique_ptr<int> S,
              $nonnull_removal[[absl_nonnull ]]$nonnull^std::unique_ptr<int> T);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, Input.point("no")), hasNoRemovalRanges()),
              AllOf(eligibleRange(2, Input.point("yes")),
                    removalRanges({Input.range("yes_removal")})),
              AllOf(eligibleRange(3, Input.point("with_comment")),
                    removalRanges({Input.range("with_comment_removal")})),
              AllOf(eligibleRange(4, Input.point("nullable")),
                    removalRanges({Input.range("nullable_removal")})),
              AllOf(eligibleRange(5, Input.point("nonnull")),
                    removalRanges({Input.range("nonnull_removal")})))));
}

TEST(RemovalRangesTest, AbslMacroSmartPointersWithNearerConst) {
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void target($const_west_removal[[absl_nullable ]]const $insert_const_west^std::unique_ptr<int> P,
              $insert_const_east^std::unique_ptr<int> const$const_east_removal[[ absl_nullable]] Q,
              $insert_const_east_no_space^std::unique_ptr<int>const$const_east_removal_no_space[[ absl_nullable]] R,
              $insert_const_east_comment^std::unique_ptr<int>/* a comment */const$const_east_removal_comment[[ absl_nullable]] S);
    )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, Input.point("insert_const_west")),
                    removalRanges({Input.range("const_west_removal")})),
              AllOf(eligibleRange(2, Input.point("insert_const_east")),
                    removalRanges({Input.range("const_east_removal")})),
              AllOf(
                  eligibleRange(3, Input.point("insert_const_east_no_space")),
                  removalRanges({Input.range("const_east_removal_no_space")})),
              AllOf(eligibleRange(4, Input.point("insert_const_east_comment")),
                    removalRanges(
                        {Input.range("const_east_removal_comment")})))));
}

TEST(RemovalRangesTest, AbslMacroWithFunctionPointer) {
  auto Input = Annotations(R"(
  void target(void (*^[[ absl_nonnull]] P)(int));
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                AllOf(eligibleRange(1, Input.point(""), Nullability::NONNULL),
                      removalRanges({Input.range("")})))));
}

TEST(RemovalRangesTest, SimpleAlias) {
  auto Input = Annotations(R"(
  using IntPtr = int *;

  void target($one_removal[[_Nullable ]]$one^IntPtr P,
              $two^IntPtr$two_removal[[ _Nullable]] Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(
          Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
          UnorderedElementsAre(
              AllOf(eligibleRange(1, Input.point("one"), Nullability::NULLABLE),
                    removalRanges({Input.range("one_removal")})),
              AllOf(eligibleRange(2, Input.point("two"), Nullability::NULLABLE),
                    removalRanges({Input.range("two_removal")})))));
}

TEST(EligibleRangesTest, FunctionPointer) {
  auto Input = Annotations(R"(
  void target(int (*$after_star_0^P)(int, int*$pointer_param^));
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("after_star_0")),
                eligibleRange(std::nullopt, Input.point("pointer_param")))));

  Input = Annotations("void target(int (*$after_star^)(int));");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("after_star")))));
}

TEST(EligibleRangesTest, ArrayOfNonPointersHasNoRanges) {
  std::string Input = "void target(int P[]);";
  EXPECT_THAT(getFunctionRanges(Input), IsEmpty());
}

TEST(EligibleRangesTest, ArrayOfSimplePointers) {
  auto Input = Annotations("void target(int*^ P[]);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(std::nullopt, Input.point()))));
}

TEST(EligibleRangesTest, ArrayOfFunctionPointers) {
  auto Input = Annotations("void target([[int (*$after_star^P[3])(float)]]);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("after_star")))));

  // An unnamed array of function pointers.
  Input = Annotations("void target([[void(*$after_star^[])(int)]]);");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("after_star")))));
}

TEST(EligibleRangesTest, ArrayOfArrayOfPointersToArray) {
  auto Input = Annotations(R"(
  void target(int *$element^ (*$ptr_to_array^P[3][2])[1]);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("element")),
                eligibleRange(std::nullopt, Input.point("ptr_to_array")))));
}

TEST(EligibleRangesTest, PointerToArray) {
  auto Input = Annotations(R"(void target(int (*$after_star^P)[]);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("after_star")))));

  // An unnamed pointer to an array.
  Input = Annotations(R"(void target(int (*$after_star^)[]);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("after_star")))));
}

TEST(EligibleRangesTest,
     ArrayOfPointersWithExtraParensAroundNameAndInSizeBrackets) {
  auto Input =
      Annotations(R"(void target(int (*$after_star^((P))[(1 + 2)]));)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(std::nullopt, Input.point("after_star")))));
}

TEST(EligibleRangesTest, PointerToPointerToArray) {
  auto Input =
      Annotations(R"(void target(int (*$after_star_2^*$after_star_1^Q)[1]);)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("after_star_1")),
                eligibleRange(std::nullopt, Input.point("after_star_2")))));
}

TEST(EligibleRangesTest, PointerToArrayOfFunctionPointers) {
  auto Input = Annotations(
      R"(void target(void (*$after_star_2^(*$after_star_1^(P))[])(int));)");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(
                eligibleRange(1, Input.point("after_star_1")),
                eligibleRange(std::nullopt, Input.point("after_star_2")))));
}

TEST(EligibleRangesTest, StarInMacroAtEndOfDefinition) {
  auto Input = Annotations(R"(
  #define MY_TYPE int*

  void target(MY_TYPE$one^ P, MY_TYPE$two^ Q);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("one")),
                                 eligibleRange(2, Input.point("two")))));
}

TEST(EligibleRangesTest, StarInFunctionPointerMacro) {
  auto Input = Annotations(R"(
  #define FUNCTION_POINTER void(*^)(int)

  void target(FUNCTION_POINTER, FUNCTION_POINTER);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()),
                                 eligibleRange(2, Input.point()))));
}

TEST(EligibleRangesTest, StarInArrayPointerMacro) {
  auto Input = Annotations(R"(
  #define ARRAY_POINTER int(*$after_star^)[]

  void target(ARRAY_POINTER, ARRAY_POINTER);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("after_star")),
                                 eligibleRange(2, Input.point("after_star")))));
}

TEST(EligibleRangesTest, StarInPointerArrayMacro) {
  auto Input = Annotations(R"(
  #define POINTER_ARRAY int*^[]

  void target(POINTER_ARRAY, POINTER_ARRAY);
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(std::nullopt, Input.point()),
                                 eligibleRange(std::nullopt, Input.point()))));
}

TEST(EligibleRangesTest, StarInFunctionPointerAlias) {
  auto Input = Annotations(R"(
  typedef void(*FunctionPointer)(int);
  using FunctionPointerUsing = void(*)(int);

  void target($typedef^FunctionPointer, $using^FunctionPointerUsing);
  )");
  // It's legal to place the annotation immediately before (or after) an alias
  // for a complex declarator type, instead of immediately after the `*` inside
  // the type, and it allows for more precise annotations, so we supply the
  // range for doing so, preferring to place the annotation before the alias.
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("typedef")),
                                 eligibleRange(2, Input.point("using")))));
}

TEST(EligibleRangesTest, StarInArrayPointerAlias) {
  auto Input = Annotations(R"(
  typedef int(*ArrayPointer)[];
  using ArrayPointerUsing = int(*)[];

  void target($typedef^ArrayPointer, $using^ArrayPointerUsing);
  )");
  // It's legal to place the annotation immediately before (or after) an alias
  // for a complex declarator type, instead of immediately after the `*` inside
  // the type, and it allows for more precise annotations, so we supply the
  // range for doing so, preferring to place the annotation before the alias.
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("typedef")),
                                 eligibleRange(2, Input.point("using")))));
}

TEST(EligibleRangesTest, StarInPointerArrayAlias) {
  auto Input = Annotations(R"(
  typedef int* PointerArray[];
  using PointerArrayUsing = int*[];

  void target(PointerArray, PointerArrayUsing);
  )");
  // We don't dig into the alias for sub-ranges, and the array types are not
  // eligible pointers.
  EXPECT_THAT(getFunctionRanges(Input.code()), IsEmpty());
}

TEST(EligibleRangesTest, StarInFunctionPointerInMacroArg) {
  auto Input = Annotations(R"(
  #define MACRO(ARG) void target(ARG)

  MACRO(void (*^)(int));
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, StarInFunctionPointerDeclEntirelyInMacro) {
  auto Input = Annotations(R"(
  #define MACRO void target(void (*^)(int))

  MACRO;
  )");
  EXPECT_THAT(
      getFunctionRanges(Input.code()),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point()))));
}

TEST(EligibleRangesTest, DotsRemovedFromPath) {
  auto Header1 = Annotations(R"(
    void target1(int *$param^);
  )");
  auto Header2 = Annotations(R"(
    int*$return^ target2(int);
  )");
  auto Input = Annotations(R"cc(
#include "./header1.h"
#include "./dir/../header2.h"
  )cc");

  NullabilityPragmas Pragmas;
  TestInputs Inputs = getAugmentedTestInputs(Input.code(), Pragmas);
  Inputs.ExtraFiles["header1.h"] = Header1.code();
  Inputs.ExtraFiles["dir/../header2.h"] = Header2.code();
  TestAST TU(Inputs);
  auto Ranges1 =
      getRanges<FunctionDecl>(TU.context(), functionDecl(hasName("target1")),
                              TypeNullabilityDefaults(TU.context(), Pragmas));
  auto Ranges2 =
      getRanges<FunctionDecl>(TU.context(), functionDecl(hasName("target2")),
                              TypeNullabilityDefaults(TU.context(), Pragmas));

  EXPECT_THAT(
      Ranges1,
      AllOf(Each(AllOf(hasPath("header1.h"), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Header1.point("param")))));
  EXPECT_THAT(
      Ranges2,
      AllOf(Each(AllOf(hasPath("header2.h"), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(0, Header2.point("return")))));
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
    auto LambdaWithPtrParam = [](int *$param^) {};
    auto LambdaWithPtrReturn = []() -> int *$return^ { return nullptr; };
  )");
  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input.code(), Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(1, Input.point("param")),
                                 eligibleRange(0, Input.point("return")))));
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
    int funcTemplate(T*$param^ A, T B) {
      T*$local^ LocalPointerInTemplate;
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
            UnorderedElementsAre(eligibleRange(1, Input.point("param")),
                                 eligibleRange(0, Input.point("local")))));
}

TEST(GetEligibleRangesFromASTTest, AutoFunctionTemplateSyntax) {
  auto Input = Annotations(R"(
    void funcTemplate(auto P, auto*$star^ Q) {}

    void usage() {
      int*$local_one^ A = nullptr;
      int*$local_two^ B = nullptr;
      funcTemplate(A, B);
    }
  )");

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input.code(), Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  EXPECT_THAT(
      getEligibleRanges(TU.context(), Defaults),
      AllOf(Each(AllOf(hasPath(MainFileName), hasNoPragmaNullability())),
            UnorderedElementsAre(eligibleRange(2, Input.point("star")),
                                 eligibleRange(0, Input.point("local_one")),
                                 eligibleRange(0, Input.point("local_two")))));
}

// This is a crash repro related to a function template's return type that is
// somehow not itself dependent, but has a nested name specifier which is a
// dependent type.
TEST(GetEligibleRangesFromASTTest, NonDependentTypeWithDependentTypeNamespace) {
  std::string Input = R"cc(
    template <typename T>
    struct Inner {
      typedef int Alias;
    };

    template <typename T>
    struct Outer {
      typedef Inner<bool> InnerAlias;
    };

    template <typename T>
    Outer<T>::InnerAlias::Alias* Foo() {
      return nullptr;
    }
  )cc";

  NullabilityPragmas Pragmas;
  TestAST TU(getAugmentedTestInputs(Input, Pragmas));
  TypeNullabilityDefaults Defaults(TU.context(), Pragmas);

  EXPECT_THAT(getEligibleRanges(TU.context(), Defaults), IsEmpty());
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
