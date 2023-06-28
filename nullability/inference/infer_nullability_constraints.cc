// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/infer_nullability_constraints.h"

#include <optional>
#include <vector>

#include "absl/log/check.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/resolve_constraints.h"
#include "nullability/inference/safety_constraint_generator.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/ControlFlowContext.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/Support/Errc.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {

namespace {
using ::clang::dataflow::DataflowAnalysisContext;
using ::clang::dataflow::Environment;
using ::clang::dataflow::PointerValue;
using ::clang::dataflow::Value;

std::optional<NullabilityConstraint> inferNullabilityConstraint(
    const clang::ParmVarDecl &P, const Environment &Environment,
    const SafetyConstraintGenerator &SafetyConstraintGenerator) {
  auto *Val = clang::dyn_cast_or_null<PointerValue>(Environment.getValue(P));
  // If there's no PointerValue or its null state wasn't evaluated by the
  // analysis, produce no facts.
  if (!Val || !hasPointerNullState(*Val)) {
    return std::nullopt;
  }

  // Parameters already annotated NonNull must be NonNull. This enables
  // production of edits to synchronize the annotation from definition to all
  // declarations.
  if (isNonNullAnnotated(P.getType())) {
    NullabilityConstraint Constraint;
    Constraint.set_must_be_nonnull(true);
    return Constraint;
  }

  return resolveConstraints(SafetyConstraintGenerator.constraints(), *Val);
}
}  // namespace

bool isNonNullAnnotated(clang::QualType Type) {
  CHECK(Type.getNonReferenceType()->isPointerType());
  if (const TypeNullability Nullability =
          getNullabilityAnnotationsFromType(Type);
      !Nullability.empty()) {
    return Nullability[0] == clang::NullabilityKind::NonNull;
  }
  return false;
}

llvm::Expected<llvm::DenseMap<const clang::NamedDecl *, NullabilityConstraint>>
inferNullabilityConstraints(const clang::FunctionDecl &Func,
                            clang::ASTContext &Context) {
  // We want to make sure we use the declaration that the body comes from,
  // otherwise we will see references to `ParmVarDecl`s from a different
  // declaration.
  const clang::FunctionDecl *DeclWithBody = nullptr;
  if (!Func.getBody(DeclWithBody)) {
    return llvm::make_error<llvm::StringError>(llvm::errc::invalid_argument,
                                               "Function has no body.");
  }
  CHECK(DeclWithBody);

  llvm::DenseMap<const clang::NamedDecl *, NullabilityConstraint> Results;
  llvm::Expected<clang::dataflow::ControlFlowContext> ControlFlowContext =
      clang::dataflow::ControlFlowContext::build(*DeclWithBody);
  if (!ControlFlowContext) return Results;

  DataflowAnalysisContext AnalysisContext(
      std::make_unique<clang::dataflow::WatchedLiteralsSolver>());
  Environment Environment(AnalysisContext, *DeclWithBody);
  PointerNullabilityAnalysis Analysis(Context);
  SafetyConstraintGenerator SafetyConstraintGenerator;

  llvm::Expected<std::vector<std::optional<
      clang::dataflow::DataflowAnalysisState<PointerNullabilityLattice>>>>
      BlockToOutputStateOrError = clang::dataflow::runDataflowAnalysis(
          *ControlFlowContext, Analysis, Environment,
          [&SafetyConstraintGenerator, &Context](
              const clang::CFGElement &Element,
              const clang::dataflow::DataflowAnalysisState<
                  PointerNullabilityLattice> &State) {
            SafetyConstraintGenerator.collectConstraints(Element, State.Lattice,
                                                         State.Env, Context);
          });
  if (!BlockToOutputStateOrError) {
    return Results;
  }

  for (const auto *P : DeclWithBody->parameters()) {
    CHECK(P != nullptr);
    if (std::optional<NullabilityConstraint> Constraint =
            inferNullabilityConstraint(*P, Environment,
                                       SafetyConstraintGenerator))
      Results[P] = *Constraint;
  }
  return Results;
}
}  // namespace clang::tidy::nullability
