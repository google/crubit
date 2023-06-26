// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/resolve_constraints.h"

#include <memory>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/pointer_nullability.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "llvm/ADT/DenseSet.h"

namespace clang::tidy::nullability {
namespace {
using ::testing::EqualsProto;

class ResolveConstraintsTest : public testing::Test {
 public:
  ResolveConstraintsTest()
      : StorageLocation(clang::dataflow::StorageLocation::Kind::Scalar,
                        clang::QualType()),
        Pointer(StorageLocation),
        DataflowAnalysisContext(
            std::make_unique<clang::dataflow::WatchedLiteralsSolver>()),
        Environment(DataflowAnalysisContext) {
    initUnknownPointer(Pointer, Environment);
  }

  clang::dataflow::StorageLocation StorageLocation;
  clang::dataflow::PointerValue Pointer;
  clang::dataflow::DataflowAnalysisContext DataflowAnalysisContext;
  clang::dataflow::Environment Environment;
};

TEST_F(ResolveConstraintsTest, EmptyConstraintsDoNotImplyNonNull) {
  const llvm::DenseSet<clang::dataflow::BoolValue*> Constraints;
  EXPECT_FALSE(resolveConstraints(Constraints, Pointer).must_be_nonnull());
}

TEST_F(ResolveConstraintsTest, ArbitraryBooleanConstraintsDoNotImplyNonNull) {
  clang::dataflow::AtomicBoolValue Atom1;
  clang::dataflow::AtomicBoolValue Atom2;
  const llvm::DenseSet<clang::dataflow::BoolValue*> Constraints = {&Atom1,
                                                                   &Atom2};
  EXPECT_FALSE(resolveConstraints(Constraints, Pointer).must_be_nonnull());
}

TEST_F(ResolveConstraintsTest, UnsatisfiableConstraintsProducesDefaultValues) {
  clang::dataflow::AtomicBoolValue Atom1;
  clang::dataflow::BoolValue& NotAtom1 = Environment.makeNot(Atom1);
  const llvm::DenseSet<clang::dataflow::BoolValue*> Constraints = {&Atom1,
                                                                   &NotAtom1};
  EXPECT_THAT(resolveConstraints(Constraints, Pointer),
              EqualsProto(NullabilityConstraint::default_instance()));
}

TEST_F(ResolveConstraintsTest, NotIsNullConstraintImpliesNonNull) {
  auto& is_null = getPointerNullState(Pointer).second;
  auto& not_is_null = Environment.makeNot(is_null);
  const llvm::DenseSet<clang::dataflow::BoolValue*> Constraints = {
      &not_is_null};
  EXPECT_TRUE(resolveConstraints(Constraints, Pointer).must_be_nonnull());
}

}  // namespace
}  // namespace clang::tidy::nullability
