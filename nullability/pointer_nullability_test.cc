// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability.h"

#include <memory>

#include "clang/AST/Type.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(NullabilityPropertiesTest, Test) {
  dataflow::DataflowAnalysisContext DACtx(
      std::make_unique<dataflow::WatchedLiteralsSolver>());
  dataflow::Environment Env(DACtx);

  auto &True = DACtx.arena().makeLiteral(true);
  auto &False = DACtx.arena().makeLiteral(false);
  auto &False2 = DACtx.arena().makeNot(True);

  auto MakePointer =
      [&](const dataflow::Formula &FromNullable,
          const dataflow::Formula &Null) -> dataflow::PointerValue & {
    auto &P = Env.create<dataflow::PointerValue>(
        DACtx.createStorageLocation(QualType()));
    initPointerNullState(P, Env, &Env.arena().makeBoolValue(FromNullable),
                         &Env.arena().makeBoolValue(Null));
    return P;
  };

  EXPECT_TRUE(
      isNullable(MakePointer(/*FromNullable=*/True, /*Null=*/True), Env));
  EXPECT_FALSE(
      isNullable(MakePointer(/*FromNullable=*/True, /*Null=*/False), Env));
  EXPECT_FALSE(
      isNullable(MakePointer(/*FromNullable=*/False, /*Null=*/True), Env));
  EXPECT_FALSE(
      isNullable(MakePointer(/*FromNullable=*/False, /*Null=*/False), Env));

  EXPECT_FALSE(
      isNullable(MakePointer(/*FromNullable=*/True, /*Null=*/False2), Env));
  EXPECT_FALSE(
      isNullable(MakePointer(/*FromNullable=*/False2, /*Null=*/True), Env));
}

}  // namespace
}  // namespace clang::tidy::nullability
