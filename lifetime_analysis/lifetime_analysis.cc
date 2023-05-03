// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/lifetime_analysis.h"

#include <iostream>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "lifetime_analysis/builtin_lifetimes.h"
#include "lifetime_analysis/object.h"
#include "lifetime_analysis/object_repository.h"
#include "lifetime_analysis/object_set.h"
#include "lifetime_analysis/pointer_compatibility.h"
#include "lifetime_analysis/points_to_map.h"
#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/pointee_type.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/StmtVisitor.h"
#include "clang/AST/TemplateBase.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/ErrorHandling.h"

namespace clang {
namespace tidy {
namespace lifetimes {

namespace {

class TransferStmtVisitor
    : public clang::StmtVisitor<TransferStmtVisitor,
                                std::optional<std::string>> {
 public:
  TransferStmtVisitor(
      ObjectRepository& object_repository, PointsToMap& points_to_map,
      LifetimeConstraints& constraints, ObjectSet& single_valued_objects,
      const clang::FunctionDecl* func,
      const llvm::DenseMap<const clang::FunctionDecl*,
                           FunctionLifetimesOrError>& callee_lifetimes,
      const DiagnosticReporter& diag_reporter)
      : object_repository_(object_repository),
        points_to_map_(points_to_map),
        constraints_(constraints),
        single_valued_objects_(single_valued_objects),
        func_(func),
        callee_lifetimes_(callee_lifetimes),
        diag_reporter_(diag_reporter) {}

  std::optional<std::string> VisitExpr(const clang::Expr* expr);
  std::optional<std::string> VisitDeclRefExpr(
      const clang::DeclRefExpr* decl_ref);
  std::optional<std::string> VisitStringLiteral(
      const clang::StringLiteral* strlit);
  std::optional<std::string> VisitCastExpr(const clang::CastExpr* cast);
  std::optional<std::string> VisitReturnStmt(
      const clang::ReturnStmt* return_stmt);
  std::optional<std::string> VisitDeclStmt(const clang::DeclStmt* decl_stmt);
  std::optional<std::string> VisitUnaryOperator(const clang::UnaryOperator* op);
  std::optional<std::string> VisitArraySubscriptExpr(
      const clang::ArraySubscriptExpr* subscript);
  std::optional<std::string> VisitBinaryOperator(
      const clang::BinaryOperator* op);
  std::optional<std::string> VisitConditionalOperator(
      const clang::ConditionalOperator* op);
  std::optional<std::string> VisitInitListExpr(
      const clang::InitListExpr* init_list);
  std::optional<std::string> VisitMaterializeTemporaryExpr(
      const clang::MaterializeTemporaryExpr* temporary_expr);
  std::optional<std::string> VisitMemberExpr(const clang::MemberExpr* member);
  std::optional<std::string> VisitCXXThisExpr(
      const clang::CXXThisExpr* this_expr);
  std::optional<std::string> VisitCallExpr(const clang::CallExpr* call);
  std::optional<std::string> VisitCXXConstructExpr(
      const clang::CXXConstructExpr* construct_expr);

 private:
  ObjectRepository& object_repository_;
  PointsToMap& points_to_map_;
  LifetimeConstraints& constraints_;
  ObjectSet& single_valued_objects_;
  const clang::FunctionDecl* func_;
  const llvm::DenseMap<const clang::FunctionDecl*, FunctionLifetimesOrError>&
      callee_lifetimes_;
  const DiagnosticReporter& diag_reporter_;
};

void GenerateConstraintsForSingleAssignment(const Object* oldp,
                                            const Object* newp,
                                            LifetimeConstraints& constraints) {
  if (oldp->GetFuncLifetimes().has_value() &&
      newp->GetFuncLifetimes().has_value()) {
    // The order of `newp` and `oldp` here may seem surprising. However, this
    // can be thought of as: "I am assigning `newp` where before I had `oldp`,
    // therefore `oldp` needs to be able to represent a call to whatever it is
    // that `newp` represents, hence I need to generate constraints for
    // replacing `newp` with `oldp`". At least this is why veluca@ thinks this
    // is the correct order (the opposite order generates incorrect results).
    constraints.join(LifetimeConstraints::ForCallableSubstitutionFull(
        *newp->GetFuncLifetimes(), *oldp->GetFuncLifetimes()));
  }
  constraints.AddOutlivesConstraint(oldp->GetLifetime(), newp->GetLifetime());
}

void GenerateConstraintsForAssignmentNonRecursive(
    const ObjectSet& old_pointees, const ObjectSet& new_pointees,
    bool is_in_invariant_context, LifetimeConstraints& constraints) {
  // The new pointees must always outlive the old pointees.
  for (const Object* old : old_pointees) {
    for (const Object* newp : new_pointees) {
      GenerateConstraintsForSingleAssignment(old, newp, constraints);
    }
  }

  // If we are in an invariant context, we need to insert constraints in the
  // opposite direction too (i.e. we need equality).
  if (is_in_invariant_context) {
    for (const Object* old : old_pointees) {
      for (const Object* newp : new_pointees) {
        GenerateConstraintsForSingleAssignment(newp, old, constraints);
      }
    }
  }
}

// TODO(veluca): this is quadratic.
void GenerateConstraintsForAssignmentRecursive(
    const ObjectSet& pointers, const ObjectSet& new_pointees,
    clang::QualType pointer_type, const ObjectRepository& object_repository,
    const PointsToMap& points_to_map, bool is_in_invariant_context,
    LifetimeConstraints& constraints,
    llvm::DenseSet<std::pair<const Object*, const Object*>>& seen_pairs) {
  // Check for cycles.
  {
    size_t num_seen_pairs = seen_pairs.size();
    for (auto pointer : pointers) {
      for (auto pointee : new_pointees) {
        seen_pairs.insert({pointer, pointee});
      }
    }
    // All done: all the pairs we have were already seen.
    if (num_seen_pairs == seen_pairs.size()) return;
  }

  if (pointer_type->isIncompleteType()) {
    // Nothing we *can* do.
    return;
  }
  assert(!pointer_type->isRecordType());
  if (!pointer_type->isPointerType() && !pointer_type->isReferenceType()) {
    // Nothing to do.
    return;
  }

  ObjectSet old_pointees = points_to_map.GetPointerPointsToSet(pointers);

  GenerateConstraintsForAssignmentNonRecursive(
      old_pointees, new_pointees, is_in_invariant_context, constraints);

  // See https://doc.rust-lang.org/nomicon/subtyping.html for an explanation of
  // variance; here in particular, we use the fact that the pointee of a pointer
  // is covariant if the pointer points to a const-qualified type, and invariant
  // otherwise.
  is_in_invariant_context = !pointer_type->getPointeeType().isConstQualified();

  // Recurse in pointees. As the pointee might be of struct type, we need first
  // to extract all field pointers from it.
  struct RecursiveVisitInfo {
    clang::QualType type;
    ObjectSet old_pointees;
    ObjectSet new_pointees;
  };

  std::vector<RecursiveVisitInfo> calls_to_make;
  calls_to_make.push_back(
      {pointer_type->getPointeeType(), old_pointees, new_pointees});
  while (!calls_to_make.empty()) {
    RecursiveVisitInfo call = std::move(calls_to_make.back());
    calls_to_make.pop_back();

    if (call.type->isIncompleteType()) {
      // Nothing we *can* do.
      continue;
    }

    if (const auto* record_type = call.type->getAs<clang::RecordType>()) {
      for (auto field : record_type->getDecl()->fields()) {
        calls_to_make.push_back(
            {field->getType(),
             object_repository.GetFieldObject(call.old_pointees, field),
             object_repository.GetFieldObject(call.new_pointees, field)});
      }
      if (auto* cxxrecord =
              clang::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl())) {
        for (const clang::CXXBaseSpecifier& base : cxxrecord->bases()) {
          calls_to_make.push_back({base.getType(),
                                   object_repository.GetBaseClassObject(
                                       call.old_pointees, base.getType()),
                                   object_repository.GetBaseClassObject(
                                       call.new_pointees, base.getType())});
        }
      }
    } else {
      GenerateConstraintsForAssignmentRecursive(
          call.old_pointees,
          points_to_map.GetPointerPointsToSet(call.new_pointees), call.type,
          object_repository, points_to_map, is_in_invariant_context,
          constraints, seen_pairs);
    }
  }
}

void GenerateConstraintsForAssignment(const ObjectSet& pointers,
                                      const ObjectSet& new_pointees,
                                      clang::QualType pointer_type,
                                      const ObjectRepository& object_repository,
                                      PointsToMap& points_to_map,
                                      LifetimeConstraints& constraints) {
  llvm::DenseSet<std::pair<const Object*, const Object*>> seen_pairs;
  // Outer-most pointers are never invariant.
  GenerateConstraintsForAssignmentRecursive(
      pointers, new_pointees, pointer_type, object_repository, points_to_map,
      /*is_in_invariant_context=*/false, constraints, seen_pairs);
}

void GenerateConstraintsForObjectLifetimeEquality(
    const ObjectSet& a, const ObjectSet& b, const clang::QualType& type,
    const PointsToMap& points_to_map, const ObjectRepository& object_repository,
    LifetimeConstraints& constraints) {
  GenerateConstraintsForAssignmentNonRecursive(
      a, b, /*is_in_invariant_context=*/true, constraints);
  if (const auto* record_type = type->getAs<clang::RecordType>()) {
    for (auto field : record_type->getDecl()->fields()) {
      GenerateConstraintsForObjectLifetimeEquality(
          object_repository.GetFieldObject(a, field),
          object_repository.GetFieldObject(b, field), field->getType(),
          points_to_map, object_repository, constraints);
    }
    if (auto* cxxrecord =
            clang::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl())) {
      for (const clang::CXXBaseSpecifier& base : cxxrecord->bases()) {
        GenerateConstraintsForObjectLifetimeEquality(
            object_repository.GetBaseClassObject(a, base.getType()),
            object_repository.GetBaseClassObject(b, base.getType()),
            base.getType(), points_to_map, object_repository, constraints);
      }
    }
  } else if (!PointeeType(type).isNull()) {
    GenerateConstraintsForObjectLifetimeEquality(
        points_to_map.GetPointerPointsToSet(a),
        points_to_map.GetPointerPointsToSet(b), PointeeType(type),
        points_to_map, object_repository, constraints);
  }
}

}  // namespace

void HandlePointsToSetExtension(const ObjectSet& pointers,
                                const ObjectSet& new_pointees,
                                clang::QualType pointer_type,
                                const ObjectRepository& object_repository,
                                PointsToMap& points_to_map,
                                LifetimeConstraints& constraints) {
  // Record types should not get to this point at all, as
  // their initialization is done by constructor calls.
  assert(!pointer_type->isRecordType());
  GenerateConstraintsForAssignment(pointers, new_pointees, pointer_type,
                                   object_repository, points_to_map,
                                   constraints);
  for (const Object* pointer : pointers) {
    points_to_map.ExtendPointerPointsToSet(pointer, new_pointees);
  }
}

void TransferInitializer(const Object* dest, clang::QualType type,
                         const ObjectRepository& object_repository,
                         const clang::Expr* init_expr,
                         TargetPointeeBehavior pointee_behavior,
                         PointsToMap& points_to_map,
                         LifetimeConstraints& constraints) {
  type = type.getCanonicalType();
  if (type->isArrayType()) {
    type = type->castAsArrayTypeUnsafe()->getElementType();
  }

  // Initializer lists are handled one member/field at a time.
  if (type->isRecordType()) {
    if (auto init_list_expr = clang::dyn_cast<clang::InitListExpr>(init_expr)) {
      // We assume that initializers are always the semantic form of
      // InitListExpr.
      assert(init_list_expr->isSemanticForm());
      size_t init = 0;
      for (auto f : type->getAs<clang::RecordType>()->getDecl()->fields()) {
        assert(init < init_list_expr->getNumInits());
        auto field_init = init_list_expr->getInit(init);
        ++init;
        TransferInitializer(object_repository.GetFieldObject(dest, f),
                            f->getType(), object_repository, field_init,
                            pointee_behavior, points_to_map, constraints);
      }
      return;
    }
  }

  if (type->isPointerType() || type->isReferenceType() ||
      type->isStructureOrClassType()) {
    ObjectSet init_points_to = points_to_map.GetExprObjectSet(init_expr);
    if (pointee_behavior == TargetPointeeBehavior::kKeep) {
      // It's important to use "Extend" (not "Set") here because we process
      // initializers for member variables only _after_ the dataflow analysis
      // has run.
      HandlePointsToSetExtension({dest}, init_points_to, type,
                                 object_repository, points_to_map, constraints);
    } else {
      points_to_map.SetPointerPointsToSet(dest, init_points_to);
    }
  }
}

LifetimeLattice LifetimeAnalysis::initialElement() {
  return LifetimeLattice(object_repository_.InitialPointsToMap(),
                         object_repository_.InitialSingleValuedObjects());
}

std::string LifetimeAnalysis::ToString(const LifetimeLattice& state) {
  return state.ToString();
}

bool LifetimeAnalysis::IsEqual(const LifetimeLattice& state1,
                               const LifetimeLattice& state2) {
  return state1 == state2;
}

void LifetimeAnalysis::transfer(const clang::CFGElement& elt,
                                LifetimeLattice& state,
                                clang::dataflow::Environment& /*environment*/) {
  if (state.IsError()) return;

  auto cfg_stmt = elt.getAs<clang::CFGStmt>();
  if (!cfg_stmt) return;
  auto stmt = cfg_stmt->getStmt();

  TransferStmtVisitor visitor(object_repository_, state.PointsTo(),
                              state.Constraints(), state.SingleValuedObjects(),
                              func_, callee_lifetimes_, diag_reporter_);
  if (std::optional<std::string> err =
          visitor.Visit(const_cast<clang::Stmt*>(stmt))) {
    state = LifetimeLattice(*err);
  }
}

namespace {

std::optional<std::string> TransferStmtVisitor::VisitExpr(
    const clang::Expr* expr) {
  // Ensure that we don't attempt to analyze code that contains errors.
  // This is triggered by TypoExpr and RecoveryExpr, but rather than handling
  // these particular expression types individually, we just check
  // Expr::containsErrors().
  if (expr->containsErrors()) {
    return "encountered an expression containing errors";
  }
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitDeclRefExpr(
    const clang::DeclRefExpr* decl_ref) {
  auto* decl = decl_ref->getDecl();
  if (!clang::isa<clang::VarDecl>(decl) &&
      !clang::isa<clang::FunctionDecl>(decl)) {
    return std::nullopt;
  }

  const Object* object = object_repository_.GetDeclObject(decl);

  assert(decl_ref->isGLValue() || decl_ref->getType()->isBuiltinType());

  clang::QualType type = decl->getType().getCanonicalType();

  if (type->isReferenceType()) {
    points_to_map_.SetExprObjectSet(
        decl_ref, points_to_map_.GetPointerPointsToSet(object));
  } else {
    points_to_map_.SetExprObjectSet(decl_ref, {object});
  }

  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitStringLiteral(
    const clang::StringLiteral* strlit) {
  points_to_map_.SetExprObjectSet(
      strlit, {object_repository_.GetStringLiteralObject()});
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitCastExpr(
    const clang::CastExpr* cast) {
  switch (cast->getCastKind()) {
    case clang::CK_LValueToRValue: {
      if (cast->getType()->isPointerType()) {
        // Converting from a glvalue to a prvalue means that we need to perform
        // a dereferencing operation because the objects associated with
        // glvalues and prvalues have different meanings:
        // - A glvalue is associated with the object identified by the glvalue.
        // - A prvalue is only associated with an object if the prvalue is of
        //   pointer type; the object it is associated with is the object the
        //   pointer points to.
        // See also documentation for PointsToMap.
        ObjectSet points_to = points_to_map_.GetPointerPointsToSet(
            points_to_map_.GetExprObjectSet(cast->getSubExpr()));
        points_to_map_.SetExprObjectSet(cast, points_to);
      }
      break;
    }
    case clang::CK_NullToPointer: {
      points_to_map_.SetExprObjectSet(cast, {});
      break;
    }
    // These casts are just no-ops from a Object point of view.
    case clang::CK_FunctionToPointerDecay:
    case clang::CK_BuiltinFnToFnPtr:
    case clang::CK_ArrayToPointerDecay:
    case clang::CK_UserDefinedConversion:
      // Note on CK_UserDefinedConversion: The actual conversion happens in a
      // CXXMemberCallExpr that is a subexpression of this CastExpr. The
      // CK_UserDefinedConversion is just used to mark the fact that this is a
      // user-defined conversion; it's therefore a no-op for our purposes.
    case clang::CK_NoOp: {
      clang::QualType type = cast->getType().getCanonicalType();
      if (type->isPointerType() || cast->isGLValue()) {
        points_to_map_.SetExprObjectSet(
            cast, points_to_map_.GetExprObjectSet(cast->getSubExpr()));
      }
      break;
    }
    case clang::CK_DerivedToBase:
    case clang::CK_UncheckedDerivedToBase:
    case clang::CK_BaseToDerived:
    case clang::CK_Dynamic: {
      // These need to be mapped to what the subexpr points to.
      // (Simple cases just work okay with this; may need to be revisited when
      // we add more inheritance support.)
      ObjectSet points_to = points_to_map_.GetExprObjectSet(cast->getSubExpr());
      points_to_map_.SetExprObjectSet(cast, points_to);
      break;
    }
    case clang::CK_BitCast:
    case clang::CK_LValueBitCast:
    case clang::CK_IntegralToPointer: {
      // We don't support analyzing functions that perform a reinterpret_cast.
      diag_reporter_(
          func_->getBeginLoc(),
          "cannot infer lifetimes because function uses a type-unsafe cast",
          clang::DiagnosticIDs::Warning);
      diag_reporter_(cast->getBeginLoc(), "type-unsafe cast occurs here",
                     clang::DiagnosticIDs::Note);
      return "type-unsafe cast prevents analysis";
    }
    default: {
      if (cast->isGLValue() ||
          cast->getType().getCanonicalType()->isPointerType()) {
        llvm::errs() << "Unknown cast type:\n";
        cast->dump();
        // No-noop casts of pointer types are not handled yet.
        llvm::report_fatal_error("unknown cast type encountered");
      }
    }
  }
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitReturnStmt(
    const clang::ReturnStmt* return_stmt) {
  clang::QualType return_type = func_->getReturnType();
  // We only need to handle pointers and references.
  // For record types, initialization of the return value has already been
  // handled in VisitCXXConstructExpr() or VisitInitListExpr(), so nothing
  // to do here.
  if (!return_type->isPointerType() && !return_type->isReferenceType()) {
    return std::nullopt;
  }

  const clang::Expr* ret_expr = return_stmt->getRetValue();
  // This occurs when computing `ret_expr`s result includes creating temporary
  // objects with destructors. We want to find the value to be returned inside
  // the ExprWithCleanups.
  //
  // The PointsToMap::GetExprObjectSet() function could do this but it doesn't
  // understand the context from which it is being called. This operation needs
  // to be done only in cases where we are leaving scope - that is, the return
  // statement. And the return statement also needs to look for initializers in
  // its sub expressions, after looking inside ExprWithCleanups.
  //
  // That means GetExprObjectSet() would need to also look for initializers but
  // we don't want to do this on every call to GetExprObjectSet().
  if (auto cleanups = clang::dyn_cast<clang::ExprWithCleanups>(ret_expr)) {
    ret_expr = cleanups->getSubExpr();
  }

  ObjectSet expr_points_to = points_to_map_.GetExprObjectSet(ret_expr);
  GenerateConstraintsForAssignment(
      {object_repository_.GetReturnObject()}, expr_points_to, return_type,
      object_repository_, points_to_map_, constraints_);
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitDeclStmt(
    const clang::DeclStmt* decl_stmt) {
  for (const clang::Decl* decl : decl_stmt->decls()) {
    if (const auto* var_decl = clang::dyn_cast<clang::VarDecl>(decl)) {
      const Object* var_object = object_repository_.GetDeclObject(var_decl);

      // Don't need to record initializers because initialization has already
      // happened in VisitCXXConstructExpr(), VisitInitListExpr(), or
      // VisitCallExpr().
      if (var_decl->hasInit() && !var_decl->getType()->isRecordType()) {
        TransferInitializer(var_object, var_decl->getType(), object_repository_,
                            var_decl->getInit(), TargetPointeeBehavior::kIgnore,
                            points_to_map_, constraints_);
      }
    }
  }
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitUnaryOperator(
    const clang::UnaryOperator* op) {
  if (!op->isGLValue() && !op->getType()->isPointerType() &&
      !op->getType()->isArrayType()) {
    return std::nullopt;
  }

  ObjectSet sub_points_to = points_to_map_.GetExprObjectSet(op->getSubExpr());

  // Maybe surprisingly, the code here doesn't do any actual address-taking or
  // dereferencing.
  // This is because AddrOf and Deref really only do a reinterpretation:
  // - AddrOf reinterprets a glvalue of type T as a prvalue of type T*
  // - Deref reinterprets an prvalue of type T* as a glvalue of type T
  // (See also the assertions below.)
  // The actual dereferencing happens in the LValueToRValue CastExpr,
  // see TransferCastExpr().

  switch (op->getOpcode()) {
    case clang::UO_AddrOf:
      assert(!op->isGLValue());
      assert(op->getSubExpr()->isGLValue());
      points_to_map_.SetExprObjectSet(op, sub_points_to);
      break;

    case clang::UO_Deref:
      assert(op->isGLValue());
      assert(!op->getSubExpr()->isGLValue());
      points_to_map_.SetExprObjectSet(op, sub_points_to);
      break;

    case clang::UO_PostInc:
    case clang::UO_PostDec:
      assert(!op->isGLValue());
      assert(op->getSubExpr()->isGLValue());
      points_to_map_.SetExprObjectSet(
          op, points_to_map_.GetPointerPointsToSet(sub_points_to));
      break;

    case clang::UO_PreInc:
    case clang::UO_PreDec:
      assert(op->isGLValue());
      assert(op->getSubExpr()->isGLValue());
      points_to_map_.SetExprObjectSet(op, sub_points_to);
      break;

    default:
      break;
  }
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitArraySubscriptExpr(
    const clang::ArraySubscriptExpr* subscript) {
  // For our purposes here, a subscripting operation is equivalent to a
  // dereference on its base - we don't make a distinction between different
  // lifetimes in an array. This effectively merges the points-to sets of all
  // elements in the array. See [/docs/lifetimes_static_analysis.md](/docs/lifetimes_static_analysis.md) for why we
  // don't track individual array elements.

  ObjectSet sub_points_to =
      points_to_map_.GetExprObjectSet(subscript->getBase());

  assert(subscript->isGLValue());
  assert(!subscript->getBase()->isGLValue());
  points_to_map_.SetExprObjectSet(subscript, sub_points_to);
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitBinaryOperator(
    const clang::BinaryOperator* op) {
  switch (op->getOpcode()) {
    case clang::BO_Assign: {
      assert(op->getLHS()->isGLValue());
      ObjectSet lhs_points_to = points_to_map_.GetExprObjectSet(op->getLHS());
      points_to_map_.SetExprObjectSet(op, lhs_points_to);
      // Because of how we handle reference-like structs, a member access to a
      // non-reference-like field in a struct might still produce lifetimes. We
      // don't want to change points-to sets in those cases.
      if (!op->getLHS()->getType()->isPointerType()) break;
      ObjectSet rhs_points_to = points_to_map_.GetExprObjectSet(op->getRHS());
      // We can overwrite (instead of extend) the destination points-to-set
      // only in very specific circumstances:
      // - We need to know unambiguously what the LHS refers to, so that we
      //   know we're definitely writing to a particular object, and
      // - That destination object needs to be "single-valued" (see docstring of
      //   LifetimeLattice::SingleValuedObjects for the definition of this
      //   term).
      if (lhs_points_to.size() == 1 &&
          single_valued_objects_.Contains(*lhs_points_to.begin())) {
        // Replacing the points-to-set entirely does not generate any
        // constraints.
        points_to_map_.SetPointerPointsToSet(lhs_points_to, rhs_points_to);
      } else {
        HandlePointsToSetExtension(lhs_points_to, rhs_points_to,
                                   op->getLHS()->getType(), object_repository_,
                                   points_to_map_, constraints_);
      }
      break;
    }

    case clang::BO_Add:
    case clang::BO_Sub: {
      // Pointer arithmetic.
      // We are only interested in the case in which exactly one of the two
      // operands is a pointer (in particular we want to exclude int* - int*).
      if (op->getLHS()->getType()->isPointerType() ^
          op->getRHS()->getType()->isPointerType()) {
        if (op->getLHS()->getType()->isPointerType()) {
          points_to_map_.SetExprObjectSet(
              op, points_to_map_.GetExprObjectSet(op->getLHS()));
        } else {
          points_to_map_.SetExprObjectSet(
              op, points_to_map_.GetExprObjectSet(op->getRHS()));
        }
      }
      break;
    }

    default:
      break;
  }
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitConditionalOperator(
    const clang::ConditionalOperator* op) {
  clang::QualType type = op->getType().getCanonicalType();

  if (op->isGLValue() || type->isPointerType()) {
    // It is possible that either of the expressions may not have an ObjectSet
    // if the node is pruned as it is considered unreachable.
    assert(points_to_map_.ExprHasObjectSet(op->getTrueExpr()) ||
           points_to_map_.ExprHasObjectSet(op->getFalseExpr()));
    ObjectSet points_to_true =
        points_to_map_.ExprHasObjectSet(op->getTrueExpr())
            ? points_to_map_.GetExprObjectSet(op->getTrueExpr())
            : ObjectSet();
    ObjectSet points_to_false =
        points_to_map_.ExprHasObjectSet(op->getFalseExpr())
            ? points_to_map_.GetExprObjectSet(op->getFalseExpr())
            : ObjectSet();
    points_to_map_.SetExprObjectSet(op, points_to_true.Union(points_to_false));
  }
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitInitListExpr(
    const clang::InitListExpr* init_list) {
  if (init_list->isSyntacticForm()) {
    // We are only interested in the semantic form, which is fully realized,
    // and is the one considered to be the initializer.
    return std::nullopt;
  }
  if (IsInitExprInitializingARecordObject(init_list)) {
    if (init_list->isTransparent()) {
      // A transparent initializer list does nothing, the actual initializer
      // terminating expression is within, and has already transferred lifetimes
      // up to the object being initialized.
      return std::nullopt;
    }
    // The object set for each field should be pointing to the initializers.
    const Object* init_object =
        object_repository_.GetInitializedObject(init_list);
    TransferInitializer(init_object, init_list->getType(), object_repository_,
                        init_list, TargetPointeeBehavior::kKeep, points_to_map_,
                        constraints_);
  } else {
    // If the InitListExpr is not initializing a record object, we assume it's
    // initializing an array or a reference and hence associate the InitListExpr
    // with the union of the points-to sets of the initializers (as the analysis
    // is array-insensitive).
    ObjectSet targets;
    for (clang::Expr* expr : init_list->inits()) {
      // If we are constructing an initializer list of non-pointer types, we
      // don't need to do anything here. Note that initializer list elements
      // must all have the same type in this case.
      if (PointeeType(expr->getType()).isNull() && !expr->isGLValue()) {
        return std::nullopt;
      }
      targets.Add(points_to_map_.GetExprObjectSet(expr));
    }
    points_to_map_.SetExprObjectSet(init_list, std::move(targets));
  }
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitMaterializeTemporaryExpr(
    const clang::MaterializeTemporaryExpr* temporary_expr) {
  const Object* temp_object =
      object_repository_.GetTemporaryObject(temporary_expr);
  points_to_map_.SetExprObjectSet(temporary_expr, {temp_object});
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitMemberExpr(
    const clang::MemberExpr* member) {
  ObjectSet struct_points_to =
      points_to_map_.GetExprObjectSet(member->getBase());

  if (const auto* method =
          clang::dyn_cast<clang::CXXMethodDecl>(member->getMemberDecl())) {
    // It doesn't really make sense to associate an object set with a non-static
    // member function.
    // If the member function is being called, we're not interested in its
    // "value" anyway. If the non-static member function is used outside of a
    // function call, then, it's a pointer-to-member, but those aren't
    // really pointers anyway, and we'll need special treatment for them.
    if (method->isStatic()) {
      points_to_map_.SetExprObjectSet(
          member, {object_repository_.GetDeclObject(method)});
    }
    return std::nullopt;
  }

  auto field = clang::dyn_cast<clang::FieldDecl>(member->getMemberDecl());
  if (field == nullptr) {
    llvm::report_fatal_error("indirect member access is not supported yet");
  }
  ObjectSet expr_points_to =
      object_repository_.GetFieldObject(struct_points_to, field);
  if (field->getType()->isReferenceType()) {
    expr_points_to = points_to_map_.GetPointerPointsToSet(expr_points_to);
  }
  points_to_map_.SetExprObjectSet(member, expr_points_to);
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitCXXThisExpr(
    const clang::CXXThisExpr* this_expr) {
  std::optional<const Object*> this_object = object_repository_.GetThisObject();
  assert(this_object.has_value());
  points_to_map_.SetExprObjectSet(this_expr, ObjectSet{this_object.value()});
  return std::nullopt;
}

namespace {
void ConstrainFunctionLifetimesForCall(
    const FunctionLifetimes& callee_lifetimes,
    const FunctionLifetimes& placeholder_lifetimes,
    LifetimeConstraints& constraints) {
  // We handle function calls as follows:
  // - We create a placeholder FunctionLifetimes for each call location, meant
  // to indicate the concrete lifetimes that the callee is instantiated with for
  // that specific call.
  // - When we analyze the call, we constrain the concrete lifetimes so that
  // they are compatible with the lifetimes of the arguments.
  // - We then constrain the placeholder lifetimes so that the actual callee
  // could be used in place of the placeholder callee.
  // - As a consequence, the lifetimes of the return object are also constrained
  // correctly (ie. in such a way that they are compatible with the callee and
  // the call arguments).
  // TODO(veluca): this code assumes that the lifetime of variables cannot
  // change across function call boundaries. It is not an intrinsic limitation
  // (could potentially be resolved by updating the PointsToMap after the fact,
  // or - even better - by converting the entire CFG into SSA form), but for
  // simplicity we are leaving things as they are for now; if real-world usage
  // shows this to be an important limitation, we should revisit this decision.
  constraints.join(LifetimeConstraints::ForCallableSubstitutionFull(
      callee_lifetimes, placeholder_lifetimes));
}
}  // namespace

std::optional<std::string> TransferStmtVisitor::VisitCallExpr(
    const clang::CallExpr* call) {
  struct CalleeInfo {
    bool is_member_operator;
    FunctionLifetimes lifetimes;
    // Type of the function being called. Note that this might not know anything
    // about the `this` argument for non-static methods.
    const clang::FunctionProtoType* type;
  };

  llvm::SmallVector<CalleeInfo, 1> callees;

  auto add_callee_from_decl =
      [&callees, call,
       this](const clang::FunctionDecl* decl) -> std::optional<std::string> {
    const FunctionLifetimesOrError& callee_lifetimes_or_error =
        GetFunctionLifetimes(decl, callee_lifetimes_);

    if (!std::holds_alternative<FunctionLifetimes>(callee_lifetimes_or_error)) {
      // Note: It is possible that this does not have an entry if the function
      // is not analyzed (because its body is not defined in this TU). If this
      // happens, we currently bail without analyzing further.
      return "No lifetimes for callee '" + decl->getNameAsString() + "': " +
             std::get<FunctionAnalysisError>(callee_lifetimes_or_error).message;
    }
    FunctionLifetimes callee_lifetimes =
        std::get<FunctionLifetimes>(callee_lifetimes_or_error);

    bool is_member_operator = clang::isa<clang::CXXOperatorCallExpr>(call) &&
                              clang::isa<clang::CXXMethodDecl>(decl);
    callees.push_back(
        CalleeInfo{is_member_operator, callee_lifetimes,
                   decl->getType()->getAs<clang::FunctionProtoType>()});
    return std::nullopt;
  };

  const clang::FunctionDecl* direct_callee = call->getDirectCallee();
  if (direct_callee) {
    // This code path is needed for non-static member functions, as those don't
    // have an `Object` for their callees.
    if (auto err = add_callee_from_decl(direct_callee); err.has_value()) {
      return err;
    }
  } else {
    const clang::Expr* callee = call->getCallee();
    for (const auto& object : points_to_map_.GetExprObjectSet(callee)) {
      if (const std::optional<FunctionLifetimes>& func_lifetimes =
              object->GetFuncLifetimes();
          func_lifetimes.has_value()) {
        callees.push_back(
            {.is_member_operator = false,
             .lifetimes = *func_lifetimes,
             .type = object->Type()->getAs<clang::FunctionProtoType>()});
      }
    }
  }

  for (const CalleeInfo& callee : callees) {
    ConstrainFunctionLifetimesForCall(
        callee.lifetimes, object_repository_.GetCallExprVirtualLifetimes(call),
        constraints_);

    for (size_t i = callee.is_member_operator ? 1 : 0; i < call->getNumArgs();
         i++) {
      // We can't just use SetPointerPointsToSet here because call->getArg(i)
      // might not have an ObjectSet (for example for integer constants); it
      // also may be needed for struct initialization.
      // Note that we don't need to worry about possibly extending the
      // PointsToSet more than needed, as dataflow analysis relies on points-to
      // sets never shrinking.
      TransferInitializer(
          object_repository_.GetCallExprArgumentObject(call, i),
          callee.type->getParamType(callee.is_member_operator ? i - 1 : i),
          object_repository_, call->getArg(i), TargetPointeeBehavior::kKeep,
          points_to_map_, constraints_);
    }

    std::optional<ObjectSet> this_object_set;
    if (callee.is_member_operator) {
      this_object_set = points_to_map_.GetExprObjectSet(call->getArg(0));
    } else if (const auto* member_call =
                   clang::dyn_cast<clang::CXXMemberCallExpr>(call)) {
      this_object_set = points_to_map_.GetExprObjectSet(
          member_call->getImplicitObjectArgument());
    }
    if (this_object_set.has_value()) {
      const Object* this_ptr = object_repository_.GetCallExprThisPointer(call);
      HandlePointsToSetExtension({this_ptr}, *this_object_set, this_ptr->Type(),
                                 object_repository_, points_to_map_,
                                 constraints_);
    }
  }

  if (IsInitExprInitializingARecordObject(call)) {
    const Object* init_object = object_repository_.GetInitializedObject(call);
    GenerateConstraintsForObjectLifetimeEquality(
        {init_object}, {object_repository_.GetCallExprRetObject(call)},
        init_object->Type(), points_to_map_, object_repository_, constraints_);
  } else {
    ObjectSet ret_pts = points_to_map_.GetPointerPointsToSet(
        object_repository_.GetCallExprRetObject(call));
    // SetExprObjectSet will assert-fail if `call` does not have a type that can
    // have an object set; this `if` guards against that.
    if (!ret_pts.empty()) {
      points_to_map_.SetExprObjectSet(call, ret_pts);
    }
  }
  return std::nullopt;
}

std::optional<std::string> TransferStmtVisitor::VisitCXXConstructExpr(
    const clang::CXXConstructExpr* construct_expr) {
  const clang::CXXConstructorDecl* constructor =
      construct_expr->getConstructor();

  assert(callee_lifetimes_.count(constructor->getCanonicalDecl()));
  const FunctionLifetimesOrError& callee_lifetimes_or_error =
      callee_lifetimes_.lookup(constructor->getCanonicalDecl());
  if (!std::holds_alternative<FunctionLifetimes>(callee_lifetimes_or_error)) {
    return "No lifetimes for constructor " + constructor->getNameAsString();
  }
  const FunctionLifetimes& callee_lifetimes =
      std::get<FunctionLifetimes>(callee_lifetimes_or_error);

  ConstrainFunctionLifetimesForCall(
      callee_lifetimes,
      object_repository_.GetCallExprVirtualLifetimes(construct_expr),
      constraints_);

  // We check <= instead of == because of default arguments.
  assert(construct_expr->getNumArgs() <= constructor->getNumParams());

  for (size_t i = 0; i < construct_expr->getNumArgs(); i++) {
    TransferInitializer(
        object_repository_.GetCXXConstructExprArgumentObject(construct_expr, i),
        constructor->getParamDecl(i)->getType(), object_repository_,
        construct_expr->getArg(i), TargetPointeeBehavior::kKeep, points_to_map_,
        constraints_);
  }

  // Handle the `this` parameter, which should point to the object getting
  // initialized.
  HandlePointsToSetExtension(
      {object_repository_.GetCXXConstructExprThisPointer(construct_expr)},
      {object_repository_.GetInitializedObject(construct_expr)},
      constructor->getThisType(), object_repository_, points_to_map_,
      constraints_);

  return std::nullopt;
}

}  // namespace

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
