// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/infer_tu.h"

#include <optional>
#include <string>
#include <vector>

#include "nullability/inference/ctn_replacement_macros.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/replace_macros.h"
#include "nullability/proto_matchers.h"
#include "nullability/test/test_headers.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/ASTMatchers/ASTMatchersMacros.h"
#include "clang/Basic/LLVM.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Testing/CommandLineArgs.h"
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
using testing::IsEmpty;
using testing::UnorderedElementsAre;

MATCHER_P2(inferredSlot, I, Nullability, "") {
  return arg.slot() == I && arg.nullability() == Nullability;
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

  void build(llvm::StringRef Code) {
    TestInputs Inputs = Code;
    Inputs.Language = TestLanguage::Lang_CXX17;
    for (const auto &Entry :
         llvm::ArrayRef(test_headers_create(), test_headers_size()))
      Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
    for (const auto &Entry : llvm::ArrayRef(ctn_replacement_macros_create(),
                                            ctn_replacement_macros_size()))
      Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
    Inputs.ExtraArgs.push_back("-include");
    Inputs.ExtraArgs.push_back("nullability_annotations.h");
    Inputs.ExtraArgs.push_back("-include");
    Inputs.ExtraArgs.push_back(std::string(ReplacementMacrosHeaderFileName));
    Inputs.ExtraArgs.push_back("-I.");

    Inputs.MakeAction = [&]() {
      return std::make_unique<ReplaceMacrosAction>();
    };
    AST.emplace(Inputs);
  }

  auto infer() { return inferTU(AST->context()); }

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

TEST_F(InferTUTest, Field) {
  build(R"cc(
    struct S {
      int *I;
      bool *B;
    };
    void target() {
      S S;
      S.I = nullptr;
      *S.B;
    }
  )cc");
  EXPECT_THAT(
      infer(),
      UnorderedElementsAre(
          inference(hasName("I"), {inferredSlot(0, Nullability::NULLABLE)}),
          inference(hasName("B"), {inferredSlot(0, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, Globals) {
  build(R"cc(
    int* I;
    bool* B;

    void target() {
      I = nullptr;
      *B;
    }
  )cc");
  EXPECT_THAT(
      infer(),
      UnorderedElementsAre(
          inference(hasName("I"), {inferredSlot(0, Nullability::NULLABLE)}),
          inference(hasName("B"), {inferredSlot(0, Nullability::NONNULL)})));
}

TEST_F(InferTUTest, Filter) {
  build(R"cc(
    int* target1() { return nullptr; }
    int* target2() { return nullptr; }
  )cc");
  EXPECT_THAT(inferTU(AST->context(), /*Iterations=*/1,
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
      inferTU(AST->context(), /*Iterations=*/1),
      UnorderedElementsAre(
          inference(hasName("target"), {inferredSlot(0, Nullability::UNKNOWN),
                                        inferredSlot(1, Nullability::NONNULL)}),
          inference(hasName("returnsToBeNonnull"),
                    {inferredSlot(0, Nullability::UNKNOWN),
                     inferredSlot(1, Nullability::UNKNOWN)}),
          inference(hasName("takesToBeNonnull"),
                    {inferredSlot(1, Nullability::NONNULL)})));
  EXPECT_THAT(
      inferTU(AST->context(), /*Iterations=*/2),
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
      inferTU(AST->context(), /*Iterations=*/3),
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
      inferTU(AST->context(), /*Iterations=*/4),
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

using InferTUSmartPointerTest = InferTUTest;

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

  // TODO(b/304963199): Currently not inferring anything because we don't
  // support smart pointers. The expected result is the same as for the
  // `ParamsFromCallSite` test.
  ASSERT_THAT(infer(), IsEmpty());
}

TEST_F(InferTUSmartPointerTest, ReturnTypeNullable) {
  build(R"cc(
#include <memory>
    std::unique_ptr<int> target() { return std::unique_ptr<int>(); }
  )cc");
  // TODO(b/304963199): Currently not inferring anything because we don't
  // support smart pointers. The expected result is a nullable return type.
  EXPECT_THAT(infer(), IsEmpty());
}

TEST_F(InferTUSmartPointerTest, ReturnTypeNonnull) {
  build(R"cc(
#include <memory>
    std::unique_ptr<int> target() { return std::make_unique<int>(0); }
  )cc");
  // TODO(b/304963199): Currently not inferring anything because we don't
  // support smart pointers. The expected result is a nonnull return type.
  EXPECT_THAT(infer(), IsEmpty());
}

}  // namespace
}  // namespace clang::tidy::nullability
