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
#include "clang/Basic/Specifiers.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

class NullabilityPropertiesTest : public ::testing::Test {
 protected:
  dataflow::PointerValue &makePointer(const dataflow::Formula &FromNullable,
                                      const dataflow::Formula &Null) {
    auto &P = Env.create<dataflow::PointerValue>(
        DACtx.createStorageLocation(QualType()));
    initPointerNullState(P, Env, &Env.arena().makeBoolValue(FromNullable),
                         &Env.arena().makeBoolValue(Null));
    return P;
  }

  dataflow::DataflowAnalysisContext DACtx = dataflow::DataflowAnalysisContext(
      std::make_unique<dataflow::WatchedLiteralsSolver>());
  dataflow::Environment Env = dataflow::Environment(DACtx);
};

TEST_F(NullabilityPropertiesTest, Test) {
  auto &True = DACtx.arena().makeLiteral(true);
  auto &False = DACtx.arena().makeLiteral(false);
  auto &False2 = DACtx.arena().makeNot(True);

  EXPECT_TRUE(
      isNullable(makePointer(/*FromNullable=*/True, /*Null=*/True), Env));
  EXPECT_FALSE(
      isNullable(makePointer(/*FromNullable=*/True, /*Null=*/False), Env));
  EXPECT_FALSE(
      isNullable(makePointer(/*FromNullable=*/False, /*Null=*/True), Env));
  EXPECT_FALSE(
      isNullable(makePointer(/*FromNullable=*/False, /*Null=*/False), Env));

  EXPECT_FALSE(
      isNullable(makePointer(/*FromNullable=*/True, /*Null=*/False2), Env));
  EXPECT_FALSE(
      isNullable(makePointer(/*FromNullable=*/False2, /*Null=*/True), Env));
}

TEST_F(NullabilityPropertiesTest, IsNullableAdditionalConstraints) {
  auto &FromNullable = Env.makeAtomicBoolValue().formula();
  auto &Null = Env.makeAtomicBoolValue().formula();
  EXPECT_TRUE(isNullable(makePointer(FromNullable, Null), Env));

  auto *NotNull = &DACtx.arena().makeNot(Null);
  EXPECT_FALSE(isNullable(makePointer(FromNullable, Null), Env, NotNull));
}

TEST_F(NullabilityPropertiesTest, GetNullabilityAdditionalConstraints) {
  auto &FromNullable = Env.makeAtomicBoolValue().formula();
  auto &Null = Env.makeAtomicBoolValue().formula();
  EXPECT_EQ(getNullability(makePointer(FromNullable, Null), Env),
            NullabilityKind::Nullable);

  auto *NotNull = &DACtx.arena().makeNot(Null);
  EXPECT_EQ(getNullability(makePointer(FromNullable, Null), Env, NotNull),
            NullabilityKind::NonNull);
}

}  // namespace
}  // namespace clang::tidy::nullability
