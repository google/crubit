// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <optional>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/test/test_headers.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/CommandLineArgs.h"
#include "clang/Testing/TestAST.h"
#include "llvm/Testing/Annotations/Annotations.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"  // IWYU pragma: keep
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::functionDecl;
using ::llvm::Annotations;
using ::testing::ExplainMatchResult;
using ::testing::Optional;
using ::testing::UnorderedElementsAre;

constexpr char MainFileName[] = "main.cpp";

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

std::optional<clang::tidy::nullability::TypeLocRanges> getEligibleRanges(
    llvm::StringRef Input) {
  auto Inputs = TestInputs(Input);
  for (const auto &Entry :
       llvm::ArrayRef(test_headers_create(), test_headers_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.push_back("nullability_annotations.h");
  Inputs.ExtraArgs.push_back("-I.");
  Inputs.Language = TestLanguage::Lang_CXX17;
  Inputs.FileName = std::string(MainFileName);
  auto TU = TestAST(std::move(Inputs));
  ASTContext &Context = TU.context();
  const auto *FunDecl = ast_matchers::selectFirst<FunctionDecl>(
      "fun", ast_matchers::match(functionDecl().bind("fun"), Context));
  CHECK(FunDecl != nullptr);
  return clang::tidy::nullability::getEligibleRanges(*FunDecl);
}

TEST(GenEditsTest, ReturnAndOneParameterIdentified) {
  auto Input = Annotations("$r[[int *]]foo($p[[int *]]p) { return p; }");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(0, Input.range("r")),
                                             SlotRange(1, Input.range("p"))))));
}

TEST(GenEditsTest, OnlyFirstParameterIdentified) {
  auto Input = Annotations("void foo([[int *]]p1, int p2) { return; }");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

// Checks that a function decl without a body is handled correctly.
TEST(GenEditsTest, DeclHandled) {
  auto Input = Annotations("void foo([[int *]]p1, int p2);");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, AllNestedPointersEligible) {
  auto Input =
      Annotations("void foo($three[[$two[[$one[[int *]]*]]*]]p1, int p2);");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(1, Input.range("three"))))));
}

TEST(GenEditsTest, DeclConstExcluded) {
  auto Input = Annotations(R"(
  void foo($one[[int *]] const p1,
           $two_o[[$two_i[[int *]] const *]] const p2);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two_o")),
                                       SlotRange(-1, Input.range("two_i"))))));
}

TEST(GenEditsTest, PointeeConstIncluded) {
  auto Input = Annotations(R"(
  void foo([[const int *]]p);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, NestedPointeeConstIncluded) {
  auto Input = Annotations("void foo($o[[$i[[const int *]] const *]]p);");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("o")),
                                       SlotRange(-1, Input.range("i"))))));
}

TEST(GenEditsTest, FunctionPointerTypeIgnored) {
  std::string Input = "void foo(int (*p)(int));";
  EXPECT_EQ(getEligibleRanges(Input), std::nullopt);
}

TEST(GenEditsTest, ArrayTypeIgnored) {
  std::string Input = "void foo(int p[]);";
  EXPECT_EQ(getEligibleRanges(Input), std::nullopt);
}

TEST(GenEditsTest, FunctionAndArrayTypeIgnored) {
  std::string Input = "void foo(int (*z[3])(float));";
  EXPECT_EQ(getEligibleRanges(Input), std::nullopt);
}

TEST(GenEditsTest, AnnotatedSlotsGetRangesForPointerTypeOnly) {
  auto Input = Annotations(R"(
  void foo(Nonnull<$one[[int *]]> nonnull,
           Nullable<$two[[int *]]> nullable,
           NullabilityUnknown<$three[[int *]]> unknown);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two")),
                                       SlotRange(3, Input.range("three"))))));
}

TEST(GenEditsTest, NamespacedAliasAnnotatedSlotsGetNoRange) {
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
  void foo(custom::CustomNonnull<$one[[int *]]> nonnull,
           custom::CustomNullable<$two[[int *]]> nullable,
           custom::CustomUnknown<$three[[int *]]> unknown);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(2, Input.range("two")),
                                       SlotRange(3, Input.range("three"))))));
}

TEST(GenEditsTest, NestedAnnotationsGetOneRange) {
  auto Input = Annotations(R"(void foo(Nonnull<Nonnull<[[int *]]>> a);)");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, NestedPointersOuterAnnotated) {
  test::EnableSmartPointers Enable;
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }
  void foo(
      Nonnull<$one_o[[$one_i[[int *]]*]]> p,
      Nonnull<$two_o[[std::unique_ptr<$two_i[[int*]]>]]> q,
      Nonnull<$three_o[[$three_i[[std::unique_ptr<int>]]*]]> r,
      Nonnull<$four_o[[std::unique_ptr<$four_i[[std::unique_ptr<int>]]>]]> s);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName, UnorderedElementsAre(
                                    SlotRange(1, Input.range("one_o")),
                                    SlotRange(-1, Input.range("one_i")),
                                    // TODO(b/330702908) When supported, Slot 2.
                                    SlotRange(-1, Input.range("two_o")),
                                    SlotRange(-1, Input.range("two_i")),
                                    SlotRange(3, Input.range("three_o")),
                                    SlotRange(-1, Input.range("three_i")),
                                    // TODO(b/330702908) When supported, Slot 4.
                                    SlotRange(-1, Input.range("four_o")),
                                    SlotRange(-1, Input.range("four_i"))))));
}

TEST(GenEditsTest, NestedPointersInnerAnnotated) {
  test::EnableSmartPointers Enable;
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void foo(
      $one_o[[Nonnull<$one_i[[int *]]>*]] p,
      $two_o[[std::unique_ptr<Nonnull<$two_i[[int*]]>>]] q,
      $three_o[[Nonnull<$three_i[[std::unique_ptr<int>]]>*]] r,
      $four_o[[std::unique_ptr<Nonnull<$four_i[[std::unique_ptr<int>]]>>]] s);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName, UnorderedElementsAre(
                                    SlotRange(1, Input.range("one_o")),
                                    SlotRange(-1, Input.range("one_i")),
                                    // TODO(b/330702908) When supported, Slot 2.
                                    SlotRange(-1, Input.range("two_o")),
                                    SlotRange(-1, Input.range("two_i")),
                                    SlotRange(3, Input.range("three_o")),
                                    SlotRange(-1, Input.range("three_i")),
                                    // TODO(b/330702908) When supported, Slot 4.
                                    SlotRange(-1, Input.range("four_o")),
                                    SlotRange(-1, Input.range("four_i"))))));
}

TEST(GenEditsTest, RefToPointer) {
  auto Input = Annotations("void foo([[int *]]&p);");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, TemplateOfPointers) {
  auto Input = Annotations(R"(
  template <typename One, typename Two>
  struct S {}; 

  void foo(S<$one[[int *]], $two[[$two_inner[[bool *]]*]]> p);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName, UnorderedElementsAre(
                                    SlotRange(-1, Input.range("one")),
                                    SlotRange(-1, Input.range("two")),
                                    SlotRange(-1, Input.range("two_inner"))))));
}

TEST(GenEditsTest, TemplateOfConstPointers) {
  auto Input = Annotations(R"(
  template <typename One, typename Two>
  struct S {};

  void foo(
      S<$one[[const int *]], $two_o[[$two_i[[const int *]] const *]]> p,
      S<$three[[int *]] const, $four_o[[$four_i[[int *]] const *]] const> q);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two_o")),
                                       SlotRange(-1, Input.range("two_i")),
                                       SlotRange(-1, Input.range("three")),
                                       SlotRange(-1, Input.range("four_o")),
                                       SlotRange(-1, Input.range("four_i"))))));
}

TEST(GenEditsTest, UniquePtr) {
  test::EnableSmartPointers Enable;
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  void foo($one[[std::unique_ptr<int>]] std_smart,
           Nonnull<$two[[std::unique_ptr<int>]]> nonnull_std_smart);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(
                            // TODO(b/330702908) When supported, Slots 1 and 2.
                            SlotRange(-1, Input.range("one")),
                            SlotRange(-1, Input.range("two"))))));
}

TEST(GenEditsTest, UserDefinedSmartPointer) {
  test::EnableSmartPointers Enable;
  auto Input = Annotations(R"(
  struct MySmartIntPtr {
    using absl_nullability_compatible = void;
    using pointer = int;
  };

  void foo($one[[MySmartIntPtr]] user_defined_smart,
           Nonnull<$two[[MySmartIntPtr]]> nonnull_user_defined_smart);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(
                            // TODO(b/330702908) When supported, Slots 1 and 2.
                            SlotRange(-1, Input.range("one")),
                            SlotRange(-1, Input.range("two"))))));
}

TEST(GenEditsTest, UserDefinedTemplatedSmartPointer) {
  test::EnableSmartPointers Enable;
  auto Input = Annotations(R"(
  template <typename T>
  struct MySmartPtr {
    using absl_nullability_compatible = void;
  };

  void foo($one[[MySmartPtr<int>]] user_defined_smart,
           Nonnull<$two[[MySmartPtr<int>]]> nonnull_user_defined_smart);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(
                            // TODO(b/330702908) When supported, Slots 1 and 2.
                            SlotRange(-1, Input.range("one")),
                            SlotRange(-1, Input.range("two"))))));
}

TEST(GenEditsTest, SimpleAlias) {
  auto Input = Annotations(R"(
  using IntPtr = int *;

  void foo([[IntPtr]] a);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, InaccessibleAlias) {
  auto Input = Annotations(R"(
  template <typename T>
  class TemplateClass {};
  using Inaccessible = TemplateClass<int *>;

  void foo(Inaccessible a);
  )");
  EXPECT_EQ(getEligibleRanges(Input.code()), std::nullopt);
}

TEST(GenEditsTest, NestedAlias) {
  auto Input = Annotations(R"(
  using Nested = int **;

  void foo($[[Nested]] a);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, AliasTemplate) {
  auto Input = Annotations(R"(
  template <typename T>
  using AliasTemplate = T;

  void foo(AliasTemplate<[[int*]]> a, AliasTemplate<int> b);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, DependentAliasSimple) {
  auto Input = Annotations(R"(
  template <typename T>
  struct S {
    using type = T;
  };

  void foo(S<[[int *]]>::type a, S<int>::type b);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, DependentAliasAnnotated) {
  auto Input = Annotations(R"(
  template <typename T>
  struct S {
    using type = T;
  };

  void foo(S<Nullable<[[int *]]>>::type a);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, DependentAliasOfDependentAlias) {
  auto Input = Annotations(R"(
  template <typename T>
  struct vector {
    using value_type = T;
  };
  template <typename T>
  struct S {
    using type = vector<T>::value_type;
  };

  void foo(S<[[int *]]>::type a);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, DependentAliasTemplate) {
  auto Input = Annotations(R"(
  template <typename V>
  struct vector {};
  template <typename T>
  struct S {
    template <template<typename> typename U>
    using type = U<T>;
  };

  void foo(S<[[int*]]>::type<vector> a);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(-1, Input.range())))));
}

TEST(GenEditsTest, DependentAliasNested) {
  auto Input = Annotations(R"(
  template <typename V>
  struct vector {
    using value_type = V;
  };

  void foo(vector<$one[[$two[[$three[[int*]]*]]*]]>::value_type a);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(-1, Input.range("three"))))));
}

TEST(GenEditsTest, TemplatedClassContext) {
  auto Input = Annotations(R"(
  template <typename T>
  struct Outer {
    struct Inner {};
  };

  void foo(Outer<[[int *]]>::Inner a);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(-1, Input.range())))));
}

TEST(GenEditsTest, NestedTemplatedClasses) {
  auto Input = Annotations(R"(
  template <typename S>
  struct Outermost {
    template <typename T>
    struct Outer {
      template <typename U>
      struct Inner {};
    };
  };

  void foo(
      Outermost<$three[[char *]]>::Outer<$two[[int *]]>::Inner<$one[[bool *]]>
          a);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(-1, Input.range("three"))))));
}

TEST(GenEditsTest, DependentAliasReferencingFurtherOutTemplateParam) {
  auto Input = Annotations(R"(
  template <typename S>
  struct Outermost {
    template <typename T>
    struct Outer {
      template <typename U>
      using Inner = S;
    };
  };

  void foo(Outermost<[[int*]]>::Outer<bool>::Inner<char*> a);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, DependentAliasForwardingMultipleTemplateArguments) {
  auto Input = Annotations(R"(
  template <typename T, class U>
  struct Pair;
  template <typename T, class U>
  struct PairWrapper {
    using type = Pair<T , U>;
  };

  void foo(PairWrapper<$one[[int *]], $two[[bool *]]>::type a);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two"))))));
}

TEST(GenEditsTest, DependentAliasInMultipleNestedClassContexts) {
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

  void foo(Outer<$one[[int *]]>::Inner<$two[[bool *]]>::type a);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two"))))));
}

TEST(GenEditsTest, AliasTemplateInNestedClassContext) {
  auto Input = Annotations(R"(
  template <typename A, class B>
  struct Pair;

  template <typename T>
  struct Outer {
    template <typename U>
    using Inner = Pair<T, U>;
  };

  void foo(Outer<$one[[int *]]>::Inner<$two[[bool *]]> a);
  )");

  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two"))))));
}

TEST(GenEditsTest, DependentAliasOfSmartPointer) {
  test::EnableSmartPointers Enable;
  auto Input = Annotations(R"(
  namespace std {
  template <typename T>
  class unique_ptr;
  }

  template <typename T>
  struct S {
    using type = std::unique_ptr<T>;
  };

  void foo($unique_ptr[[S<$inner[[int*]]>::type]] a);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("unique_ptr")),
                                       SlotRange(-1, Input.range("inner"))))));
}

TEST(GenEditsTest, DependentlyNamedTemplate) {
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
  void foo($outer[[S<$inner[[int *]], Wrapper>::type]] a);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(1, Input.range("outer")),
                                       SlotRange(-1, Input.range("inner"))))));
}

TEST(GenEditsTest, PartialSpecialization) {
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
  void foo([[S<int **>::Alias]] a);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, TypeTemplateParamPack) {
  auto Input = Annotations(R"(
  template <typename... T>
  struct Tuple {
    using type = int;
  };

  void foo(Tuple<$one[[int *]], $two[[$three[[int *]]*]]> a,
           Tuple<int *, int **>::type b);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(SlotRange(-1, Input.range("one")),
                                       SlotRange(-1, Input.range("two")),
                                       SlotRange(-1, Input.range("three"))))));
}

TEST(GenEditsTest, DefaultTemplateArgs) {
  auto Input = Annotations(R"(
  template <typename T1, typename T2 = int*>
  struct S {};
  template <typename T1, typename T2 = T1>
  using Alias = T2;

  void foo(S<$one[[int *]]> a, $two[[Alias<int *>]] b);
  )");
  EXPECT_THAT(getEligibleRanges(Input.code()),
              Optional(TypeLocRanges(
                  MainFileName,
                  UnorderedElementsAre(
                      SlotRange(-1, Input.range("one")),
                      // TODO(b/281474380) Collect the template
                      // argument instead of the whole alias, when we can see
                      // through the layers of default argument redirection
                      SlotRange(2, Input.range("two"))))));
}

TEST(GenEditsTest, MultipleSlotsOneRange) {
  auto Input = Annotations(R"(
  template <typename T1, typename T2>
  struct Pair {
    T1 first;
    T2 second;
  };
  template <typename T>
  using Couple = Pair<T, T>;

  void foo(Couple<[[int *]]> c);
  )");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          // Eventually, two different valid slot values for the two
          // ranges, but for now, inference looks at neither of
          // them, so both have no slot.
          MainFileName, UnorderedElementsAre(SlotRange(-1, Input.range()),
                                             SlotRange(-1, Input.range())))));
}

}  // namespace
}  // namespace clang::tidy::nullability
