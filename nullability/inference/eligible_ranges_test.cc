// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/eligible_ranges.h"

#include <optional>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Basic/LLVM.h"
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
  return arg.slot() == SlotID && Range.Begin == arg.begin() &&
         Range.End == arg.end();
}

MATCHER_P2(TypeLocRanges, Path, Ranges, "") {
  return ExplainMatchResult(Path, arg.path(), result_listener) &&
         ExplainMatchResult(Ranges, arg.range(), result_listener);
}

std::optional<clang::tidy::nullability::TypeLocRanges> getEligibleRanges(
    llvm::StringRef Input) {
  auto TI = TestInputs(Input);
  TI.FileName = std::string(MainFileName);
  auto TU = TestAST(std::move(TI));
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

TEST(GenEditsTest, DeclCovered) {
  auto Input = Annotations("void foo([[int *]]p1, int p2);");
  EXPECT_THAT(
      getEligibleRanges(Input.code()),
      Optional(TypeLocRanges(
          MainFileName, UnorderedElementsAre(SlotRange(1, Input.range())))));
}

TEST(GenEditsTest, ConstIncludeInType) {
  std::string Input = "void foo(const int *p1, int p2);";
  // TODO: fix, once `const` is supported.
  EXPECT_EQ(getEligibleRanges(Input), std::nullopt);
}

// TODO: Add additional tests for type corner cases, e.g.
//
// int (*foo)(int (*a))
// void foo(int **x())
// void foo(int x[])
// void foo(int (*z[3])(float))

}  // namespace
}  // namespace clang::tidy::nullability
