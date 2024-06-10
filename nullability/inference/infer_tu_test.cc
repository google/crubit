// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/infer_tu.h"

#include <optional>
#include <vector>

#include "nullability/inference/augmented_test_inputs.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/pragma.h"
#include "nullability/proto_matchers.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/ASTMatchers/ASTMatchersMacros.h"
#include "clang/Basic/LLVM.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/SmallString.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ast_matchers::hasName;
using testing::_;
using testing::ElementsAre;
using testing::IsSupersetOf;
using testing::UnorderedElementsAre;

test::EnableSmartPointers Enable;

MATCHER_P2(inferredSlot, I, Nullability, "") {
  return arg.slot() == I && arg.nullability() == Nullability;
}
MATCHER_P3(inferredSlot, I, Nullability, Conflict, "") {
  return arg.slot() == I && arg.nullability() == Nullability &&
         arg.conflict() == Conflict;
}
MATCHER_P2(inferenceMatcher, USR, SlotsMatcher, "") {
  if (arg.symbol().usr() != USR) return false;
  return testing::ExplainMatchResult(SlotsMatcher, arg.slot_inference(),
                                     result_listener);
}

AST_MATCHER(Decl, isCanonical) { return Node.isCanonicalDecl(); }

class InferTUTest : public ::testing::Test {
 protected:
  std::optional<TestAST> AST;
  NullabilityPragmas Pragmas;

  void build(llvm::StringRef Code) {
    AST.emplace(getAugmentedTestInputs(Code, Pragmas));
  }

  auto infer() { return inferTU(AST->context(), Pragmas); }

  // Returns a matcher for an Inference.
  // The DeclMatcher should uniquely identify the symbol being described.
  // (We use this to compute the USR we expect to find in the inference proto).
  // Slots should describe the slots that were inferred.
  template <typename MatcherT>
  testing::Matcher<const Inference &> inference(
      MatcherT DeclMatcher,
      std::vector<testing::Matcher<const Inference::SlotInference &>> Slots) {
    llvm::SmallString<128> USR;
    auto Matches = ast_matchers::match(
        ast_matchers::namedDecl(isCanonical(), DeclMatcher).bind("decl"),
        AST->context());
    EXPECT_EQ(Matches.size(), 1);
    if (auto *D = ast_matchers::selectFirst<Decl>("decl", Matches))
      EXPECT_FALSE(index::generateUSRForDecl(D, USR));
    return inferenceMatcher(USR, testing::ElementsAreArray(Slots));
  }
};

TEST_F(InferTUTest, UncheckedDeref) {
  build(R"cc(
    void target(int *p, bool cond) {
      if (cond) *p;
    }

    void guarded(int *p) {
      if (p) *p;
    }
  )cc");

  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {inferredSlot(1, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, Samples) {
  llvm::StringRef Code =
      "void target(int * p) { *p + *p; }\n"
      "void another(int x) { target(&x); }";
  //   123456789012345678901234567890123456789
  //   0        1         2         3

  build(Code);
  auto Results = infer();
  ASSERT_THAT(Results,
              ElementsAre(inference(hasName("target"),
                                    {inferredSlot(1, Nullability::NONNULL)})));
  EXPECT_THAT(Results.front().slot_inference(0).sample_evidence(),
              testing::UnorderedElementsAre(
                  EqualsProto(R"pb(location: "input.cc:2:30"
                                   kind: NONNULL_ARGUMENT)pb"),
                  EqualsProto(R"pb(location: "input.cc:1:24"
                                   kind: UNCHECKED_DEREFERENCE)pb"),
                  EqualsProto(R"pb(location: "input.cc:1:29"
                                   kind: UNCHECKED_DEREFERENCE)pb")));
}

TEST_F(InferTUTest, Annotations) {
  build(R"cc(
    Nonnull<int *> target(int *a, int *b);
    Nonnull<int *> target(int *a, Nullable<int *> p) { *p; }
  )cc");

  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {
                                        inferredSlot(0, Nullability::NONNULL),
                                        inferredSlot(2, Nullability::NULLABLE),
                                    })));
}

TEST_F(InferTUTest, AnnotationsConflict) {
  build(R"cc(
    Nonnull<int *> target();
    Nullable<int *> target();
  )cc");

  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {inferredSlot(0, Nullability::UNKNOWN)})));
}

TEST_F(InferTUTest, ParamsFromCallSite) {
  build(R"cc(
    void callee(int* p, int* q, int* r);
    void target(int* a, Nonnull<int*> b, Nullable<int*> c) { callee(a, b, c); }
  )cc");

  ASSERT_THAT(infer(),
              Contains(inference(hasName("callee"),
                                 {
                                     inferredSlot(1, Nullability::UNKNOWN),
                                     inferredSlot(2, Nullability::NONNULL),
                                     inferredSlot(3, Nullability::NULLABLE),
                                 })));
}

TEST_F(InferTUTest, ReturnTypeNullable) {
  build(R"cc(
    int* target() { return nullptr; }
  )cc");
  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {inferredSlot(0, Nullability::NULLABLE)})));
}

TEST_F(InferTUTest, ReturnTypeNonnull) {
  build(R"cc(
    Nonnull<int*> providesNonnull();
    int* target() { return providesNonnull(); }
  )cc");
  EXPECT_THAT(infer(),
              Contains(inference(hasName("target"),
                                 {inferredSlot(0, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, ReturnTypeNonnullAndUnknown) {
  build(R"cc(
    Nonnull<int*> providesNonnull();
    int* target(bool b, int* q) {
      if (b) return q;
      return providesNonnull();
    }
  )cc");
  EXPECT_THAT(infer(),
              Contains(inference(hasName("target"),
                                 {inferredSlot(0, Nullability::UNKNOWN)})));
}

TEST_F(InferTUTest, ReturnTypeNonnullAndNullable) {
  build(R"cc(
    Nonnull<int*> providesNonnull();
    int* target(bool b) {
      if (b) return nullptr;
      return providesNonnull();
    }
  )cc");
  EXPECT_THAT(infer(),
              Contains(inference(hasName("target"),
                                 {inferredSlot(0, Nullability::NULLABLE)})));
}

TEST_F(InferTUTest, ReturnTypeDereferenced) {
  build(R"cc(
    struct S {
      void member();
    };

    S* makePtr();
    void target() { makePtr()->member(); }
  )cc");
  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("makePtr"),
                                    {inferredSlot(0, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, PassedToNonnull) {
  build(R"cc(
    void takesNonnull(Nonnull<int*>);
    void target(int* p) { takesNonnull(p); }
  )cc");
  EXPECT_THAT(infer(),
              Contains(inference(hasName("target"),
                                 {inferredSlot(1, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, PassedToMutableNullableRef) {
  build(R"cc(
    void takesMutableNullableRef(Nullable<int*>&);
    void target(int* p) { takesMutableNullableRef(p); }
  )cc");
  EXPECT_THAT(infer(),
              Contains(inference(hasName("target"),
                                 {inferredSlot(1, Nullability::NULLABLE)})));
}

TEST_F(InferTUTest, AssignedFromNullable) {
  build(R"cc(
    void target(int* p) { p = nullptr; }
  )cc");
  EXPECT_THAT(infer(),
              Contains(inference(hasName("target"),
                                 {inferredSlot(1, Nullability::NULLABLE)})));
}

TEST_F(InferTUTest, CHECKMacro) {
  build(R"cc(
    // macro must use the parameter, but otherwise body doesn't matter
#define CHECK(x) x
    void target(int* p) { CHECK(p); }
  )cc");
  EXPECT_THAT(infer(),
              Contains(inference(hasName("target"),
                                 {inferredSlot(1, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, CHECKNEMacro) {
  build(R"cc(
    // macro must use the first parameter, but otherwise body doesn't matter
#define CHECK_NE(x, y) x
    void target(int* p, int* q, int* r, int* s) {
      CHECK_NE(p, nullptr);
      CHECK_NE(nullptr, q);
      int* a = nullptr;
      CHECK_NE(a, r);
      CHECK_NE(s, a);
    }
  )cc");
  EXPECT_THAT(infer(),
              UnorderedElementsAre(inference(
                  hasName("target"), {inferredSlot(1, Nullability::NONNULL),
                                      inferredSlot(2, Nullability::NONNULL),
                                      inferredSlot(3, Nullability::NONNULL),
                                      inferredSlot(4, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, Fields) {
  build(R"cc(
    int* getIntPtr();
    struct S {
      int* unchecked_deref;
      int* default_null_and_unchecked_deref = nullptr;
      int* uninitialized;
      int NotATarget = *getIntPtr();

      void method() {
        *unchecked_deref;
        *default_null_and_unchecked_deref;
      }
    };

    void foo() {
      // Use the implicitly-declared default constructor so that it will be
      // generated.
      S s;
    }

    class C {
     public:
      C() : null_constructor_init(nullptr) {
        null_in_constructor_and_unchecked_deref = nullptr;
        null_in_constructor = nullptr;
      }

      void method() { *null_in_constructor_and_unchecked_deref; }

     private:
      int* null_in_constructor_and_unchecked_deref;
      int* null_constructor_init;
      int* null_in_constructor;
    };
  )cc");
  EXPECT_THAT(
      infer(),
      UnorderedElementsAre(
          inference(hasName("unchecked_deref"),
                    {inferredSlot(0, Nullability::NONNULL)}),
          // Unchecked deref is strong evidence and a default null
          // member initializer is weak.
          inference(hasName("default_null_and_unchecked_deref"),
                    {inferredSlot(0, Nullability::NONNULL)}),
          // No inference for uninitialized.,
          inference(hasName("getIntPtr"),
                    {inferredSlot(0, Nullability::NONNULL)}),
          // Initialization to null in the constructor or another
          // function body is strong, producing a conflict.
          inference(hasName("null_in_constructor_and_unchecked_deref"),
                    {inferredSlot(0, Nullability::NONNULL, /*Conflict*/ true)}),
          inference(hasName("null_constructor_init"),
                    {inferredSlot(0, Nullability::NULLABLE)}),
          inference(hasName("null_in_constructor"),
                    {inferredSlot(0, Nullability::NULLABLE)})));
}

TEST_F(InferTUTest, FieldsImplicitlyDeclaredConstructorNeverUsed) {
  build(R"cc(
    Nullable<bool *> getNullable();
    struct S {
      int *I = nullptr;
      bool *B = getNullable();
      char *C = static_cast<char *>(nullptr);
    };

    void foo(S s);
  )cc");
  // Because the implicitly-declared default constructor is never used, it is
  // not present in the AST and we never analyze it. So, we collect no evidence
  // from default member initializers.
  EXPECT_THAT(infer(),
              AllOf(AllOf(Not(Contains(inference(hasName("I"), {_}))),
                          Not(Contains(inference(hasName("B"), {_}))),
                          Not(Contains(inference(hasName("C"), {_}))))));
}

TEST_F(InferTUTest, FieldsImplicitlyDeclaredConstructorUsed) {
  build(R"cc(
    Nullable<bool *> getNullable();
    struct S {
      int *I = nullptr;
      bool *B = getNullable();
      char *C = static_cast<char *>(nullptr);
    };
    // A use of the implicitly-declared default constructor, so it is generated
    // and included in the AST for us to analyze, allowing us to infer from
    // default member initializers.
    void foo() { S s; }
  )cc");
  EXPECT_THAT(
      infer(),
      IsSupersetOf(
          {inference(hasName("I"), {inferredSlot(0, Nullability::NULLABLE)}),
           inference(hasName("B"), {inferredSlot(0, Nullability::NULLABLE)}),
           inference(hasName("C"), {inferredSlot(0, Nullability::NULLABLE)})}));
}

TEST_F(InferTUTest, GlobalVariables) {
  build(R"cc(
    int* getIntPtr();

    int* I;
    bool* B;
    int NotATarget = *getIntPtr();

    void target() {
      I = nullptr;
      *B;
    }
  )cc");
  EXPECT_THAT(
      infer(),
      UnorderedElementsAre(
          inference(hasName("I"), {inferredSlot(0, Nullability::NULLABLE)}),
          inference(hasName("B"), {inferredSlot(0, Nullability::NONNULL)}),
          inference(hasName("getIntPtr"),
                    {inferredSlot(0, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, StaticMemberVariables) {
  build(R"cc(
    struct S {
      static int* SI;
      static bool* SB;
    };

    void target() {
      *S::SI;
      S::SB = nullptr;
    }
  )cc");
  EXPECT_THAT(
      infer(),
      UnorderedElementsAre(
          inference(hasName("SI"), {inferredSlot(0, Nullability::NONNULL)}),
          inference(hasName("SB"), {inferredSlot(0, Nullability::NULLABLE)})));
}

TEST_F(InferTUTest, Filter) {
  build(R"cc(
    int* target1() { return nullptr; }
    int* target2() { return nullptr; }
  )cc");
  EXPECT_THAT(inferTU(AST->context(), Pragmas, /*Iterations=*/1,
                      [&](const Decl &D) {
                        return cast<NamedDecl>(D).getNameAsString() !=
                               "target2";
                      }),
              ElementsAre(inference(hasName("target1"), {_})));
}

TEST_F(InferTUTest, IterationsPropagateInferences) {
  build(R"cc(
    void takesToBeNonnull(int* x) { *x; }
    int* returnsToBeNonnull(int* a) { return a; }
    int* target(int* p, int* q, int* r) {
      *p;
      takesToBeNonnull(q);
      q = r;
      return returnsToBeNonnull(p);
    }
  )cc");
  EXPECT_THAT(
      inferTU(AST->context(), Pragmas, /*Iterations=*/1),
      UnorderedElementsAre(
          inference(hasName("target"), {inferredSlot(0, Nullability::UNKNOWN),
                                        inferredSlot(1, Nullability::NONNULL)}),
          inference(hasName("returnsToBeNonnull"),
                    {inferredSlot(0, Nullability::UNKNOWN),
                     inferredSlot(1, Nullability::UNKNOWN)}),
          inference(hasName("takesToBeNonnull"),
                    {inferredSlot(1, Nullability::NONNULL)})));
  EXPECT_THAT(
      inferTU(AST->context(), Pragmas, /*Iterations=*/2),
      UnorderedElementsAre(
          inference(hasName("target"), {inferredSlot(0, Nullability::UNKNOWN),
                                        inferredSlot(1, Nullability::NONNULL),
                                        inferredSlot(2, Nullability::NONNULL)}),
          inference(hasName("returnsToBeNonnull"),
                    {inferredSlot(0, Nullability::UNKNOWN),
                     inferredSlot(1, Nullability::NONNULL)}),
          inference(hasName("takesToBeNonnull"),
                    {inferredSlot(1, Nullability::NONNULL)})));
  EXPECT_THAT(
      inferTU(AST->context(), Pragmas, /*Iterations=*/3),
      UnorderedElementsAre(
          inference(hasName("target"), {inferredSlot(0, Nullability::UNKNOWN),
                                        inferredSlot(1, Nullability::NONNULL),
                                        inferredSlot(2, Nullability::NONNULL),
                                        inferredSlot(3, Nullability::NONNULL)}),
          inference(hasName("returnsToBeNonnull"),
                    {inferredSlot(0, Nullability::NONNULL),
                     inferredSlot(1, Nullability::NONNULL)}),
          inference(hasName("takesToBeNonnull"),
                    {inferredSlot(1, Nullability::NONNULL)})));
  EXPECT_THAT(
      inferTU(AST->context(), Pragmas, /*Iterations=*/4),
      UnorderedElementsAre(
          inference(hasName("target"), {inferredSlot(0, Nullability::NONNULL),
                                        inferredSlot(1, Nullability::NONNULL),
                                        inferredSlot(2, Nullability::NONNULL),
                                        inferredSlot(3, Nullability::NONNULL)}),
          inference(hasName("returnsToBeNonnull"),
                    {inferredSlot(0, Nullability::NONNULL),
                     inferredSlot(1, Nullability::NONNULL)}),
          inference(hasName("takesToBeNonnull"),
                    {inferredSlot(1, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, Pragma) {
  build(R"cc(
#pragma nullability file_default nonnull
    void target(int* default_nonnull, NullabilityUnknown<int*> inferred_nonnull,
                Nullable<int*> nullable,
                NullabilityUnknown<int*> inferred_nullable,
                NullabilityUnknown<int*> unknown) {
      default_nonnull = inferred_nonnull;
      default_nonnull = nullptr;
      inferred_nullable = nullable;
    }
  )cc");
  EXPECT_THAT(infer(),
              UnorderedElementsAre(inference(
                  hasName("target"),
                  {
                      // annotation by pragma beats assignment from null, so
                      // default_nonnull should still be inferred NONNULL
                      inferredSlot(1, Nullability::NONNULL),
                      // an explicit unknown does not override a Nonnull
                      // inference, even if it overrides the pragma
                      inferredSlot(2, Nullability::NONNULL),
                      // an explicit nullable overrides pragma default
                      inferredSlot(3, Nullability::NULLABLE),
                      // an explicit unknown does not override a Nullable
                      // inference, which does override the pragma
                      inferredSlot(4, Nullability::NULLABLE)
                      // an explicit unknown overrides the pragma, but produces
                      // no inference, so nothing for slot 5.
                  })));
}

using InferTUSmartPointerTest = InferTUTest;

TEST_F(InferTUSmartPointerTest, Annotations) {
  build(R"cc(
#include <memory>
    Nonnull<std::unique_ptr<int>> target(std::unique_ptr<int> a,
                                         std::unique_ptr<int> b);
    Nonnull<std::unique_ptr<int>> target(std::unique_ptr<int> a,
                                         Nullable<std::unique_ptr<int>> p) {
      *p;
    }
  )cc");

  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {
                                        inferredSlot(0, Nullability::NONNULL),
                                        inferredSlot(2, Nullability::NULLABLE),
                                    })));
}

TEST_F(InferTUSmartPointerTest, ParamsFromCallSite) {
  build(R"cc(
#include <memory>
#include <utility>
    void callee(std::unique_ptr<int> p, std::unique_ptr<int> q,
                std::unique_ptr<int> r);
    void target(std::unique_ptr<int> a, Nonnull<std::unique_ptr<int>> b,
                Nullable<std::unique_ptr<int>> c) {
      callee(std::move(a), std::move(b), std::move(c));
    }
  )cc");

  EXPECT_THAT(infer(),
              Contains(inference(hasName("callee"),
                                 {
                                     inferredSlot(1, Nullability::UNKNOWN),
                                     inferredSlot(2, Nullability::NONNULL),
                                     inferredSlot(3, Nullability::NULLABLE),
                                 })));
}

TEST_F(InferTUSmartPointerTest, ReturnTypeNullable) {
  build(R"cc(
#include <memory>
    std::unique_ptr<int> target() { return std::unique_ptr<int>(); }
  )cc");
  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {inferredSlot(0, Nullability::NULLABLE)})));
}

TEST_F(InferTUSmartPointerTest, ReturnTypeNonnull) {
  build(R"cc(
#include <memory>
    std::unique_ptr<int> target() { return std::make_unique<int>(0); }
  )cc");
  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {inferredSlot(0, Nullability::NONNULL)})));
}

}  // namespace
}  // namespace clang::tidy::nullability
