// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability.h"

#include <memory>

#include "nullability/type_nullability.h"
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
  dataflow::PointerValue &makePointer(PointerTypeNullability N) {
    auto &P = Env.create<dataflow::PointerValue>(
        DACtx.createStorageLocation(QualType()));
    initPointerNullState(P, DACtx, N);
    return P;
  }

  dataflow::PointerValue &makeNullPointer() {
    auto &P = Env.create<dataflow::PointerValue>(
        DACtx.createStorageLocation(QualType()));
    initNullPointer(P, DACtx);
    return P;
  }

  dataflow::DataflowAnalysisContext DACtx = dataflow::DataflowAnalysisContext(
      std::make_unique<dataflow::WatchedLiteralsSolver>());
  dataflow::Environment Env = dataflow::Environment(DACtx);
};

TEST_F(NullabilityPropertiesTest, Test) {
  auto &A = DACtx.arena();

  EXPECT_TRUE(isNullable(makeNullPointer(), Env));

  {
    auto &NullableButNotNull = makePointer(NullabilityKind::Nullable);
    EXPECT_TRUE(isNullable(NullableButNotNull, Env));
    Env.addToFlowCondition(
        A.makeNot(getPointerNullState(NullableButNotNull).IsNull));
    EXPECT_FALSE(isNullable(NullableButNotNull, Env));
  }

  {
    auto &NullableAndNull = makePointer(NullabilityKind::Nullable);
    Env.addToFlowCondition(getPointerNullState(NullableAndNull).IsNull);
    EXPECT_TRUE(isNullable(NullableAndNull, Env));
  }

  {
    auto &NonnullAndNotNull = makePointer(NullabilityKind::NonNull);
    EXPECT_FALSE(isNullable(NonnullAndNotNull, Env));
    Env.addToFlowCondition(
        A.makeNot(getPointerNullState(NonnullAndNotNull).IsNull));
    EXPECT_FALSE(isNullable(NonnullAndNotNull, Env));
  }

  {
    // This is a little surprising: if a pointer comes from a non-null source
    // but is dynamically discovered to be definitely null, we still don't
    // consider it nullable.
    auto &NonnullAndNull = makePointer(NullabilityKind::NonNull);
    Env.addToFlowCondition(getPointerNullState(NonnullAndNull).IsNull);
    EXPECT_FALSE(isNullable(NonnullAndNull, Env));
  }
}

TEST_F(NullabilityPropertiesTest, IsNullableAdditionalConstraints) {
  auto &P = makePointer(NullabilityKind::Nullable);
  EXPECT_TRUE(isNullable(P, Env));
  auto *NotNull = &DACtx.arena().makeNot(getPointerNullState(P).IsNull);
  EXPECT_FALSE(isNullable(P, Env, NotNull));
}

TEST_F(NullabilityPropertiesTest, GetNullabilityAdditionalConstraints) {
  auto &P = makePointer(NullabilityKind::Nullable);
  EXPECT_EQ(getNullability(P, Env), NullabilityKind::Nullable);
  auto *NotNull = &DACtx.arena().makeNot(getPointerNullState(P).IsNull);
  EXPECT_EQ(getNullability(P, Env, NotNull), NullabilityKind::NonNull);
}

}  // namespace
}  // namespace clang::tidy::nullability
