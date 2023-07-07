// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability.h"

#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using testing::ElementsAre;

TEST(NullabilityPropertiesTest, Test) {
  dataflow::DataflowAnalysisContext DACtx(
      std::make_unique<dataflow::WatchedLiteralsSolver>());
  dataflow::Environment Env(DACtx);

  auto &True = DACtx.arena().makeLiteral(true);
  auto &False = DACtx.arena().makeLiteral(false);
  auto &False2 = DACtx.arena().makeNot(True);

  auto MakePointer =
      [&](const dataflow::Formula &Known,
          const dataflow::Formula &Null) -> dataflow::PointerValue & {
    auto &P = Env.create<dataflow::PointerValue>(
        DACtx.createStorageLocation(QualType()));
    initPointerNullState(P, Env, &Env.arena().makeBoolValue(Known),
                         &Env.arena().makeBoolValue(Null));
    return P;
  };

  EXPECT_TRUE(isNullable(MakePointer(/*Known=*/True, /*Null=*/True), Env));
  EXPECT_FALSE(isNullable(MakePointer(/*Known=*/True, /*Null=*/False), Env));
  EXPECT_FALSE(isNullable(MakePointer(/*Known=*/False, /*Null=*/True), Env));
  EXPECT_FALSE(isNullable(MakePointer(/*Known=*/False, /*Null=*/False), Env));

  EXPECT_FALSE(isNullable(MakePointer(/*Known=*/True, /*Null=*/False2), Env));
  EXPECT_FALSE(isNullable(MakePointer(/*Known=*/False2, /*Null=*/True), Env));
}

}  // namespace
}  // namespace clang::tidy::nullability
