// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/resolve_constraints.h"

#include <memory>

#include "nullability/inference/inference.proto.h"
#include "nullability/pointer_nullability.h"
#include "nullability/proto_matchers.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "llvm/ADT/DenseSet.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

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
  const llvm::DenseSet<const clang::dataflow::Formula *> Constraints;
  EXPECT_FALSE(resolveConstraints(Constraints, Pointer).must_be_nonnull());
}

TEST_F(ResolveConstraintsTest, ArbitraryBooleanConstraintsDoNotImplyNonNull) {
  auto &A = Environment.arena();
  auto &Atom1 = A.makeAtomRef(A.makeAtom());
  auto &Atom2 = A.makeAtomRef(A.makeAtom());
  const llvm::DenseSet<const clang::dataflow::Formula *> Constraints = {&Atom1,
                                                                        &Atom2};
  EXPECT_FALSE(resolveConstraints(Constraints, Pointer).must_be_nonnull());
}

TEST_F(ResolveConstraintsTest, UnsatisfiableConstraintsProducesDefaultValues) {
  auto &A = Environment.arena();
  auto &Atom1 = A.makeAtomRef(A.makeAtom());
  const llvm::DenseSet<const clang::dataflow::Formula *> Constraints = {
      &Atom1, &A.makeNot(Atom1)};
  EXPECT_THAT(resolveConstraints(Constraints, Pointer), EqualsProto(""));
}

TEST_F(ResolveConstraintsTest, NotIsNullConstraintImpliesNonNull) {
  auto &A = Environment.arena();
  auto &is_null = getPointerNullState(Pointer).second.formula();
  const llvm::DenseSet<const clang::dataflow::Formula *> Constraints = {
      &A.makeNot(is_null)};
  EXPECT_TRUE(resolveConstraints(Constraints, Pointer).must_be_nonnull());
}

}  // namespace
}  // namespace clang::tidy::nullability
