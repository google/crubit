// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_LIFETIME_ANALYSIS_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_LIFETIME_ANALYSIS_H_

#include <functional>
#include <string>
#include <variant>

#include "lifetime_analysis/lifetime_lattice.h"
#include "lifetime_analysis/object_repository.h"
#include "lifetime_analysis/points_to_map.h"
#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/Type.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang {
namespace tidy {
namespace lifetimes {

void TransferInitializer(const Object* dest, clang::QualType type,
                         const ObjectRepository& object_repository,
                         const clang::Expr* init_expr,
                         PointsToMap& points_to_map,
                         LifetimeConstraints& constraints);

struct FunctionParameter {
  clang::QualType param_type;
  ValueLifetimes param_lifetimes;
  const Object* arg_object;
};

std::optional<ObjectSet> TransferLifetimesForCall(
    const clang::Expr* call, const std::vector<FunctionParameter>& fn_params,
    const ValueLifetimes& return_lifetimes, ObjectRepository& object_repository,
    PointsToMap& points_to_map, clang::ASTContext& ast_context);

// Function to call to report a diagnostic.
// This has the same interface as ClangTidyCheck::diag().
using DiagnosticReporter = std::function<clang::DiagnosticBuilder(
    clang::SourceLocation, clang::StringRef, clang::DiagnosticIDs::Level)>;

class LifetimeAnalysis
    : public clang::dataflow::DataflowAnalysis<LifetimeAnalysis,
                                               LifetimeLattice> {
 public:
  explicit LifetimeAnalysis(
      const clang::FunctionDecl* func, ObjectRepository& object_repository,
      const llvm::DenseMap<const clang::FunctionDecl*,
                           FunctionLifetimesOrError>& callee_lifetimes,
      const DiagnosticReporter& diag_reporter)
      : clang::dataflow::DataflowAnalysis<LifetimeAnalysis, LifetimeLattice>(
            func->getASTContext(), /*ApplyBuiltinTransfer=*/false),
        func_(func),
        object_repository_(object_repository),
        callee_lifetimes_(callee_lifetimes),
        diag_reporter_(diag_reporter) {}

  LifetimeLattice initialElement();

  std::string ToString(const LifetimeLattice& state);

  bool IsEqual(const LifetimeLattice& state1, const LifetimeLattice& state2);

  void transfer(const clang::Stmt* stmt, LifetimeLattice& state,
                clang::dataflow::Environment& environment);

 private:
  const clang::FunctionDecl* func_;
  ObjectRepository& object_repository_;
  const llvm::DenseMap<const clang::FunctionDecl*, FunctionLifetimesOrError>&
      callee_lifetimes_;
  const DiagnosticReporter& diag_reporter_;
};

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_LIFETIME_ANALYSIS_H_
