// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_lattice.h"

#include <cassert>
#include <memory>

#include "absl/base/nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ast_matchers::callee;
using ast_matchers::cxxMemberCallExpr;
using ast_matchers::functionDecl;
using ast_matchers::hasName;
using ast_matchers::match;
using ast_matchers::selectFirst;

using dataflow::DataflowAnalysisContext;
using dataflow::Environment;
using dataflow::LatticeJoinEffect;
using dataflow::RecordStorageLocation;
using dataflow::Value;
using dataflow::WatchedLiteralsSolver;

absl::Nonnull<NamedDecl *> lookup(StringRef Name, const DeclContext &DC) {
  auto Result = DC.lookup(&DC.getParentASTContext().Idents.get(Name));
  EXPECT_TRUE(Result.isSingleResult()) << Name;
  return Result.front();
}

class PointerNullabilityLatticeTest : public ::testing::Test {
 protected:
  DataflowAnalysisContext DACtx{std::make_unique<WatchedLiteralsSolver>()};
  Environment Env{DACtx};
  PointerNullabilityLattice::NonFlowSensitiveState NFS;
};

TEST_F(PointerNullabilityLatticeTest, ConstMethodProducesNewValueAfterJoin) {
  TestAST AST(R"cpp(
    struct S {
      int *property() const;
    };
    void target() {
      S s;
      s.property();
    }
  )cpp");

  auto *SDecl =
      cast<CXXRecordDecl>(lookup("S", *AST.context().getTranslationUnitDecl()));
  QualType SType = AST.context().getRecordType(SDecl);

  auto *CE = selectFirst<CallExpr>(
      "call", match(cxxMemberCallExpr(callee(functionDecl(hasName("property"))))
                        .bind("call"),
                    AST.context()));
  assert(CE != nullptr);

  RecordStorageLocation Loc(SType, RecordStorageLocation::FieldToLoc(), {});

  PointerNullabilityLattice Lattice1(NFS);
  Value *Val1 = Lattice1.getOrCreateConstMethodReturnValue(Loc, CE, Env);

  PointerNullabilityLattice Lattice2(NFS);
  Value *Val2 = Lattice2.getOrCreateConstMethodReturnValue(Loc, CE, Env);

  EXPECT_EQ(Lattice1.join(Lattice2), LatticeJoinEffect::Changed);
  Value *ValAfterJoin =
      Lattice1.getOrCreateConstMethodReturnValue(Loc, CE, Env);

  EXPECT_NE(ValAfterJoin, Val1);
  EXPECT_NE(ValAfterJoin, Val2);
}

}  // namespace
}  // namespace clang::tidy::nullability
