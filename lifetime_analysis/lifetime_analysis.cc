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
#include "lifetime_analysis/visit_lifetimes.h"
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
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/Optional.h"
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
      const clang::FunctionDecl* func,
      const llvm::DenseMap<const clang::FunctionDecl*,
                           FunctionLifetimesOrError>& callee_lifetimes,
      const DiagnosticReporter& diag_reporter)
      : object_repository_(object_repository),
        points_to_map_(points_to_map),
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
  const clang::FunctionDecl* func_;
  const llvm::DenseMap<const clang::FunctionDecl*, FunctionLifetimesOrError>&
      callee_lifetimes_;
  const DiagnosticReporter& diag_reporter_;
};

}  // namespace

void TransferInitializer(Object dest, clang::QualType type,
                         const ObjectRepository& object_repository,
                         const clang::Expr* init_expr,
                         PointsToMap& points_to_map) {
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
        TransferInitializer(*object_repository.GetFieldObject(dest, f),
                            f->getType(), object_repository, field_init,
                            points_to_map);
      }
      return;
    }
  }

  if (type->isPointerType() || type->isReferenceType() ||
      type->isStructureOrClassType()) {
    ObjectSet init_points_to = points_to_map.GetExprObjectSet(init_expr);
    // It's important to use "Extend" (not "Set") here because we process
    // initializers for member variables only _after_ the dataflow analysis has
    // run.
    points_to_map.ExtendPointerPointsToSet(dest, init_points_to);
  }
}

namespace {

void SetPointerPointsToSetRespectingTypes(Object pointer,
                                          const ObjectSet& points_to,
                                          PointsToMap& points_to_map,
                                          clang::ASTContext& ast_context) {
  assert(pointer.Type()->isPointerType() || pointer.Type()->isReferenceType());

  ObjectSet points_to_filtered;

  for (auto object : points_to) {
    if (MayPointTo(pointer.Type(), object.Type(), ast_context)) {
      points_to_filtered.Add(object);
    }
  }

  points_to_map.SetPointerPointsToSet(pointer, points_to_filtered);
}

void SetAllPointersPointsToSetRespectingTypes(const ObjectSet& pointers,
                                              const ObjectSet& points_to,
                                              PointsToMap& points_to_map,
                                              clang::ASTContext& ast_context) {
  for (auto pointer : pointers) {
    SetPointerPointsToSetRespectingTypes(pointer, points_to, points_to_map,
                                         ast_context);
  }
}

void CollectLifetimes(
    Object arg_object, clang::QualType type,
    const ValueLifetimes& value_lifetimes, const PointsToMap& points_to_map,
    const ObjectRepository& object_repository,
    llvm::DenseMap<Lifetime, ObjectSet>& lifetime_to_object_set) {
  class Visitor : public LifetimeVisitor {
   public:
    Visitor(const ObjectRepository& object_repository,
            const PointsToMap& points_to_map,
            llvm::DenseMap<Lifetime, ObjectSet>& lifetime_to_object_set)
        : object_repository_(object_repository),
          points_to_map_(points_to_map),
          lifetime_to_object_set_(lifetime_to_object_set) {}

    Object GetFieldObject(const ObjectSet& objects,
                          const clang::FieldDecl* field) override {
      // All the objects have the same field.
      assert(!objects.empty());
      return *object_repository_.GetFieldObject(*objects.begin(), field);
    }

    Object GetBaseClassObject(const ObjectSet& objects,
                              clang::QualType base) override {
      // All the objects have the same base.
      assert(!objects.empty());
      return *object_repository_.GetBaseClassObject(*objects.begin(), base);
    }

    ObjectSet Traverse(const ObjectLifetimes& lifetimes,
                       const ObjectSet& objects,
                       int /*pointee_depth*/) override {
      lifetime_to_object_set_[lifetimes.GetLifetime()].Add(objects);
      return points_to_map_.GetPointerPointsToSet(objects);
    }

   private:
    const ObjectRepository& object_repository_;
    const PointsToMap& points_to_map_;
    llvm::DenseMap<Lifetime, ObjectSet>& lifetime_to_object_set_;
  };
  Visitor visitor(object_repository, points_to_map, lifetime_to_object_set);
  VisitLifetimes({arg_object}, type,
                 ObjectLifetimes(arg_object.GetLifetime(), value_lifetimes),
                 visitor);
}

void PropagateLifetimesToPointees(
    Object arg_object, clang::QualType type,
    const ValueLifetimes& value_lifetimes, PointsToMap& points_to_map,
    ObjectRepository& object_repository,
    const llvm::DenseMap<Lifetime, ObjectSet>& lifetime_to_object_set,
    clang::ASTContext& ast_context) {
  class Visitor : public LifetimeVisitor {
   public:
    Visitor(ObjectRepository& object_repository, PointsToMap& points_to_map,
            const llvm::DenseMap<Lifetime, ObjectSet>& lifetime_to_object_set,
            clang::ASTContext& ast_context)
        : object_repository_(object_repository),
          points_to_map_(points_to_map),
          lifetime_to_object_set_(lifetime_to_object_set),
          ast_context_(ast_context) {}

    Object GetFieldObject(const ObjectSet& objects,
                          const clang::FieldDecl* field) override {
      // All the objects have the same field.
      assert(!objects.empty());
      return *object_repository_.GetFieldObject(*objects.begin(), field);
    }

    Object GetBaseClassObject(const ObjectSet& objects,
                              clang::QualType base) override {
      // All the objects have the same base.
      assert(!objects.empty());
      return *object_repository_.GetBaseClassObject(*objects.begin(), base);
    }

    ObjectSet Traverse(const ObjectLifetimes& lifetimes,
                       const ObjectSet& objects,
                       int /*pointee_depth*/) override {
      clang::QualType type = lifetimes.GetValueLifetimes().Type();
      ObjectSet points_to_original =
          points_to_map_.GetPointerPointsToSet(objects);
      if (!type.isConstQualified() && !PointeeType(type).isNull()) {
        Lifetime pointee_lifetime =
            lifetimes.GetValueLifetimes().GetPointeeLifetimes().GetLifetime();
        ObjectSet points_to = lifetime_to_object_set_.lookup(pointee_lifetime);
        // If this is pointer-to-static, assume the callee can modify it to
        // point to a static object that we don't know about.
        if (pointee_lifetime == Lifetime::Static()) {
          points_to.Add(
              object_repository_.CreateStaticObject(PointeeType(type)));
        }
        SetAllPointersPointsToSetRespectingTypes(objects, points_to,
                                                 points_to_map_, ast_context_);
        assert(points_to_map_.GetPointerPointsToSet(objects).Contains(
            points_to_original));
      }
      // Return the original points-to set, not the modified one. The original
      // points-to set is sufficient because it captures the arguments that
      // were passed to the function, but it doesn't contain any possibly
      // spurious edges that may have been inserted by the logic above, which
      // can reduce the precision of the analysis.
      return points_to_original;
    }

   private:
    ObjectRepository& object_repository_;
    PointsToMap& points_to_map_;
    const llvm::DenseMap<Lifetime, ObjectSet>& lifetime_to_object_set_;
    clang::ASTContext& ast_context_;
  };
  Visitor visitor(object_repository, points_to_map, lifetime_to_object_set,
                  ast_context);
  VisitLifetimes({arg_object}, type,
                 ObjectLifetimes(arg_object.GetLifetime(), value_lifetimes),
                 visitor);
}

bool AllStatic(const ValueLifetimes& lifetimes) {
  return !lifetimes.HasAny([](Lifetime l) { return l != Lifetime::Static(); });
}

}  // namespace

std::optional<ObjectSet> TransferLifetimesForCall(
    const clang::Expr* call, const std::vector<FunctionParameter>& fn_params,
    const ValueLifetimes& return_lifetimes, ObjectRepository& object_repository,
    PointsToMap& points_to_map, clang::ASTContext& ast_context) {
  // TODO(mboehme): The following description says what we _want_ to do, but
  // this isn't what we actually do right now. Modify the code so that it
  // corresponds to the description, then remove this TODO.
  //
  // Overall approach:
  // - Step 1: Find all objects accessible by the callee.
  //   This means finding all objects transitively accessible from the argument
  //   pointees passed to the callee. As part of this step, we establish a
  //   mapping from callee lifetimes to caller lifetimes, which will be used in
  //   subsequent steps to determine whether a given object (whose lifetime is
  //   a caller lifetime) has a given callee lifetime. Note that, in general, a
  //   single callee lifetime may correspond to multiple caller lifetimes.
  //
  // - Step 2: Perform all modifications the callee could make to the points-to
  //   map that are permissible from a lifetime and type system point of view.
  //   Specifically, for every non-const pointer accessible by the callee:
  //   - Determine the callee lifetime 'l associated with that pointer.
  //   - For each object accessible by the callee, determine whether it has
  //     callee lifetime 'l (using the mapping established in step 1) and
  //     and whether the type of the pointer is compatible with the type of the
  //     object. If both of these conditions are met, add an edge from the
  //     pointer to the object into the points-to map.
  //   It remains to be explained what "compatible" means above. The most
  //   principled approach would be to use C++'s strict aliasing rules, but some
  //   real-world code unfortunately violates the strict aliasing rules.
  //   Instead, we make the compatibility rule more permissive than strict
  //   aliasing; we expect we will need some experimentation to achieve a
  //   good tradeoff between the following considerations:
  //   - If we make the compatibility rule too strict, we miss some points-to
  //     edges that may be introduced by real-world code (even though that code
  //     is in violation of the strict aliasing rule), and the analysis result
  //     becomes wrong.
  //   - If we make the compatibility rule too permissive, we allow spurious
  //     edges in the points-to map, and the analysis result becomes overly
  //     restrictive.
  //   We also need to consider that the type returned by Object::Type() might
  //   not be identical to the actual dynamic type of the object. If the object
  //   was passed in to the function through a pointer or reference to class
  //   type, the dynamic type of the object might be a derived class of the
  //   type we assumed for the object.
  //
  // - Step 3: Determine points-to set for the return value.
  //   This is the set of all objects accessible by the callee that
  //   - are compatible with the callee's return type, and
  //   - conform to the lifetime annotations on the return type.
  //   The latter point means that every object that is transitively reachable
  //   from the original object has a lifetime that corresponds to the callee
  //   lifetime implied by the annotation.
  //
  // Some additional considerations apply if the callee signature contains the
  // 'static lifetime, either in the parameters or the return value:
  // - Any objects that are associated with the static lifetime in the callee
  //   must be forced to have static lifetime.
  //   We have no way of doing this directly, as we cannot mutate the lifetime
  //   of the object (and, in any case, such a mutation would be global and not
  //   limited to the current point in the program flow).
  //   Instead, for each such object, we synthesize a pointer with static
  //   lifetime and make it point at the object. Later, in
  //   PropagateStaticToPointees(), this will cause us to assign static lifetime
  //   to the object.
  //   A cleaner solution to this would be to explicitly express "outlives"
  //   constraints in the lattice. This might also help more generally to
  //   simplify the logic associated with static lifetimes, but it would also be
  //   a more invasive change.
  //
  // - Any pointer or reference may point to an object of static lifetime. This
  //   has the following implications:
  //   - In step 2, when adding edges to the points-to map, we always add edges
  //     to objects of static lifetime if their type is compatible with the
  //     type of the pointer.
  //   - In step 3, an object of static lifetime conforms to any callee lifetime
  //     if that lifetime occurs in covariant position.
  //
  // - The callee may have access to objects of static lifetime that are not
  //   passed as arguments, in addition to the ones that are accessible from the
  //   arguments.
  //   Because of this, for any non-const pointer accessible by the callee, we
  //   add a points-to edge to a newly created static object of the appropriate
  //   type.
  //   This does cause us to add a lot of static objects to the graph that we
  //   do not expect to occur in reality. If this turns out to have undesired
  //   effects, we could use the following alternative approach as a compromise:
  //   - In step 2, if the non-const pointer is associated with static lifetime,
  //     does not already point to an object of static lifetime and would not
  //     gain an edge to an existing object of static lifetime, create a new
  //     object of static lifetime and the appropriate type and add an edge
  //     from the pointer to the newly created object.
  //   - In step 3, if we obtain an empty points-to set for the return value
  //     because the return type contains 'static lifetime annotations and the
  //     existing objects do not conform to these annotations, add newly
  //     created static objects to the points-to map in suitable places so that
  //     we can return a non-empty points-to set.
  //   TODO(mboehme): Investigate whether it's really so bad to add newly
  //   created static objects in all the places they could theoretically occur.
  //   If this turns out not to have any adverse effect on the analysis, it
  //   would be the more principled and simpler thing to do.

  assert(call || !return_lifetimes.HasLifetimes());

  // Step 1: Create mapping from callee lifetimes to points-to sets.
  llvm::DenseMap<Lifetime, ObjectSet> lifetime_to_object_set;
  for (auto [type, param_lifetimes, arg_object] : fn_params) {
    CollectLifetimes({arg_object}, type, param_lifetimes, points_to_map,
                     object_repository, lifetime_to_object_set);
  }

  // Force any objects associated with the static lifetime in the callee to have
  // static lifetime (see more detailed explanation above).
  if (auto iter = lifetime_to_object_set.find(Lifetime::Static());
      iter != lifetime_to_object_set.end()) {
    for (const Object& object : iter->second) {
      Object pointer = object_repository.CreateStaticObject(
          ast_context.getPointerType(object.Type()));
      points_to_map.ExtendPointerPointsToSet(pointer, {object});
    }
  }

  // Step 2: Propagate points-to sets to output parameters.
  for (auto [type, param_lifetimes, arg_object] : fn_params) {
    PropagateLifetimesToPointees({arg_object}, type, param_lifetimes,
                                 points_to_map, object_repository,
                                 lifetime_to_object_set, ast_context);
  }

  // Step 3: Determine points-to set for the return value.
  if (return_lifetimes.HasLifetimes()) {
    if (IsInitExprInitializingARecordObject(call)) {
      Object init_object = object_repository.GetInitializedObject(call);
      PropagateLifetimesToPointees(
          {init_object}, call->getType(), return_lifetimes, points_to_map,
          object_repository, lifetime_to_object_set, ast_context);
    } else {
      ObjectSet rval_points_to;

      rval_points_to = lifetime_to_object_set.lookup(
          return_lifetimes.GetPointeeLifetimes().GetLifetime());
      // If this return value is a pointer-to-static, assume the callee can
      // return a static object that we don't know about.
      if (return_lifetimes.GetPointeeLifetimes().GetLifetime() ==
          Lifetime::Static()) {
        bool all_static = AllStatic(return_lifetimes);
        (void)all_static;
        assert(all_static);
        rval_points_to.Add(
            object_repository.CreateStaticObject(PointeeType(call->getType())));
      }
      return rval_points_to;
    }
  }
  return std::nullopt;
}

LifetimeLattice LifetimeAnalysis::initialElement() {
  return LifetimeLattice(object_repository_.InitialPointsToMap());
}

std::string LifetimeAnalysis::ToString(const LifetimeLattice& state) {
  return state.ToString();
}

bool LifetimeAnalysis::IsEqual(const LifetimeLattice& state1,
                               const LifetimeLattice& state2) {
  return state1 == state2;
}

void LifetimeAnalysis::transfer(const clang::Stmt* stmt, LifetimeLattice& state,
                                clang::dataflow::Environment& /*environment*/) {
  if (state.IsError()) return;

  TransferStmtVisitor visitor(object_repository_, state.PointsTo(), func_,
                              callee_lifetimes_, diag_reporter_);
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
  Object obj = object_repository_.CreateStaticObject(strlit->getType());
  points_to_map_.SetExprObjectSet(strlit, {obj});
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
  points_to_map_.ExtendPointerPointsToSet(object_repository_.GetReturnObject(),
                                          expr_points_to);
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
        TransferInitializer(*var_object, var_decl->getType(),
                            object_repository_, var_decl->getInit(),
                            points_to_map_);
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
  // elements in the array. See <internal link> for why we
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
      for (Object pointer : lhs_points_to) {
        // We can overwrite (instead of extend) the destination points-to-set
        // only in very specific circumstances:
        // - We need to know unambiguously what the LHS refers to, so that we
        //   know we're definitely writing to a particular object, and
        // - That destination object needs to be "single-valued" (it can't be
        //   an array, for example).
        if (lhs_points_to.size() == 1 &&
            object_repository_.GetObjectValueType(pointer) ==
                ObjectRepository::ObjectValueType::kSingleValued) {
          points_to_map_.SetPointerPointsToSet(pointer, rhs_points_to);
        } else {
          points_to_map_.ExtendPointerPointsToSet(pointer, rhs_points_to);
        }
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
    ObjectSet points_to_true =
        points_to_map_.GetExprObjectSet(op->getTrueExpr());
    ObjectSet points_to_false =
        points_to_map_.GetExprObjectSet(op->getFalseExpr());
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
    Object init_object = object_repository_.GetInitializedObject(init_list);
    TransferInitializer(init_object, init_list->getType(), object_repository_,
                        init_list, points_to_map_);
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
  Object temp_object = object_repository_.GetTemporaryObject(temporary_expr);
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
  std::optional<Object> this_object = object_repository_.GetThisObject();
  assert(this_object.has_value());
  points_to_map_.SetExprObjectSet(this_expr, ObjectSet{this_object.value()});
  return std::nullopt;
}

// Collects all function parameters, including (if this is a member call) the
// implicit this argument.
std::vector<FunctionParameter> CollectFunctionParameters(
    const clang::CallExpr* call, const clang::FunctionDecl* callee,
    const FunctionLifetimes& callee_lifetimes,
    const ObjectRepository& object_repository) {
  std::vector<FunctionParameter> fn_params;

  if (clang::isa<clang::CXXOperatorCallExpr>(call) &&
      clang::isa<clang::CXXMethodDecl>(callee)) {
    // `this` is considered an argument in this case (but not a parameter on its
    // definition).
    assert(call->getNumArgs() == callee->getNumParams() + 1);

    // Handle the `this` argument.
    {
      fn_params.push_back(FunctionParameter{
          clang::dyn_cast<clang::CXXMethodDecl>(callee)->getThisType(),
          callee_lifetimes.GetThisLifetimes(),
          object_repository.GetCallExprThisPointer(call)});
    }

    // Handle all other arguments.
    for (size_t i = 1; i < call->getNumArgs(); i++) {
      fn_params.push_back(FunctionParameter{
          callee->getParamDecl(i - 1)->getType().getCanonicalType(),
          callee_lifetimes.GetParamLifetimes(i - 1),
          object_repository.GetCallExprArgumentObject(call, i)});
    }
  } else {
    // We check <= instead of == because of default arguments.
    assert(call->getNumArgs() <= callee->getNumParams());

    for (size_t i = 0; i < call->getNumArgs(); i++) {
      fn_params.push_back(FunctionParameter{
          callee->getParamDecl(i)->getType().getCanonicalType(),
          callee_lifetimes.GetParamLifetimes(i),
          object_repository.GetCallExprArgumentObject(call, i)});
    }
    if (const auto* member_call =
            clang::dyn_cast<clang::CXXMemberCallExpr>(call)) {
      // The callee is always a MemberExpr.
      // - If the call uses `->`, the object argument should be a prvalue that
      //   is a pointer to the struct.
      // - If the call uses `.`, the object argument should be a glvalue of
      //   struct type.
      assert(clang::isa<clang::MemberExpr>(member_call->getCallee()));
      assert(clang::dyn_cast<clang::MemberExpr>(member_call->getCallee())
                 ->isArrow() ^
             member_call->getImplicitObjectArgument()->isGLValue());
      // This is the type of the function *parameter*, not of the argument.
      // This is always a pointer, even if the argument is a reference, but as
      // we don't treat pointers or references differently, this is not an
      // issue.
      fn_params.push_back(
          FunctionParameter{member_call->getMethodDecl()->getThisType(),
                            callee_lifetimes.GetThisLifetimes(),
                            object_repository.GetCallExprThisPointer(call)});
    }
  }
  return fn_params;
}

void SetExprObjectSetRespectingType(const clang::Expr* expr,
                                    const ObjectSet& points_to,
                                    PointsToMap& points_to_map,
                                    clang::ASTContext& ast_context) {
  ObjectSet points_to_filtered;

  for (auto object : points_to) {
    if (expr->isGLValue()) {
      if (PointeesCompatible(expr->getType(), object.Type(), ast_context)) {
        points_to_filtered.Add(object);
      }
    } else {
      clang::QualType expr_type = expr->getType();
      // CXXConstructExpr is a special case -- it is a non-glvalue with the type
      // of the constructed object itself. Non-pointer, non-glvalue expressions
      // like this are not usually allowed to be associated with a points-to
      // set, but CXXConstructExpr is an exception. We need to associate it with
      // an `Object` representing the newly constructed object so that
      // TransferInitializer() can then retrieve this object. So we pretend that
      // the type is actually "pointer to object" to give MayPointTo() what it
      // expects.
      //
      // Note that we will not see clang::InitListExpr here, which is the other
      // form of initializer along with CXXConstructExpr. That is because we
      // come here through a "call" and we don't consider an initializer list to
      // be a "call" or treat it as such.
      assert(!clang::isa<clang::InitListExpr>(expr));
      if (clang::isa<clang::CXXConstructExpr>(expr)) {
        expr_type = ast_context.getPointerType(expr_type);
      }

      if (MayPointTo(expr_type, object.Type(), ast_context)) {
        points_to_filtered.Add(object);
      }
    }
  }

  points_to_map.SetExprObjectSet(expr, points_to_filtered);
}

std::optional<std::string> TransferStmtVisitor::VisitCallExpr(
    const clang::CallExpr* call) {
  llvm::SmallVector<const clang::FunctionDecl*> callees;

  const clang::FunctionDecl* direct_callee = call->getDirectCallee();
  if (direct_callee) {
    // This code path is needed for non-static member functions, as those don't
    // have an `Object` for their callees.
    callees.push_back(direct_callee);
  } else {
    const clang::Expr* callee = call->getCallee();
    for (const auto& object : points_to_map_.GetExprObjectSet(callee)) {
      const clang::FunctionDecl* func = object.GetFunc();
      assert(func);
      callees.push_back(func);
    }
  }

  std::optional<ObjectSet> call_points_to;

  for (const auto* callee : callees) {
    bool is_builtin = callee->getBuiltinID() != 0;

    FunctionLifetimesOrError builtin_callee_lifetimes_or_error;
    if (is_builtin) {
      builtin_callee_lifetimes_or_error = GetBuiltinLifetimes(callee);
    } else {
      assert(callee_lifetimes_.count(callee->getCanonicalDecl()));
    }
    const FunctionLifetimesOrError& callee_lifetimes_or_error =
        is_builtin ? builtin_callee_lifetimes_or_error
                   : callee_lifetimes_.lookup(callee->getCanonicalDecl());

    if (!std::holds_alternative<FunctionLifetimes>(callee_lifetimes_or_error)) {
      return "No lifetimes for callee '" + callee->getNameAsString() + "': " +
             std::get<FunctionAnalysisError>(callee_lifetimes_or_error).message;
    }
    FunctionLifetimes callee_lifetimes =
        std::get<FunctionLifetimes>(callee_lifetimes_or_error);

    bool is_member_operator = clang::isa<clang::CXXOperatorCallExpr>(call) &&
                              clang::isa<clang::CXXMethodDecl>(callee);
    for (size_t i = is_member_operator ? 1 : 0; i < call->getNumArgs(); i++) {
      // We can't just use SetPointerPointsToSet here because call->getArg(i)
      // might not have an ObjectSet (for example for integer constants); it
      // also may be needed for struct initialization.
      // Note that we don't need to worry about possibly extending the
      // PointsToSet more than needed, as dataflow analysis relies on points-to
      // sets never shrinking.
      TransferInitializer(
          object_repository_.GetCallExprArgumentObject(call, i),
          callee->getParamDecl(is_member_operator ? i - 1 : i)->getType(),
          object_repository_, call->getArg(i), points_to_map_);
    }
    if (is_member_operator) {
      points_to_map_.SetPointerPointsToSet(
          object_repository_.GetCallExprThisPointer(call),
          points_to_map_.GetExprObjectSet(call->getArg(0)));
    }
    if (const auto* member_call =
            clang::dyn_cast<clang::CXXMemberCallExpr>(call)) {
      points_to_map_.SetPointerPointsToSet(
          object_repository_.GetCallExprThisPointer(call),
          points_to_map_.GetExprObjectSet(
              member_call->getImplicitObjectArgument()));
    }

    std::vector<FunctionParameter> fn_params = CollectFunctionParameters(
        call, callee, callee_lifetimes, object_repository_);

    std::optional<ObjectSet> single_call_points_to = TransferLifetimesForCall(
        call, fn_params, callee_lifetimes.GetReturnLifetimes(),
        object_repository_, points_to_map_, callee->getASTContext());
    if (single_call_points_to) {
      if (call_points_to) {
        call_points_to.value().Add(std::move(single_call_points_to).value());
      } else {
        call_points_to = std::move(single_call_points_to);
      }
    }
  }

  if (call_points_to) {
    SetExprObjectSetRespectingType(call, call_points_to.value(), points_to_map_,
                                   callees[0]->getASTContext());
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

  // We check <= instead of == because of default arguments.
  assert(construct_expr->getNumArgs() <= constructor->getNumParams());

  for (size_t i = 0; i < construct_expr->getNumArgs(); i++) {
    TransferInitializer(
        object_repository_.GetCXXConstructExprArgumentObject(construct_expr, i),
        construct_expr->getArg(i)->getType(), object_repository_,
        construct_expr->getArg(i), points_to_map_);
  }

  // Handle the `this` parameter, which should point to the object getting
  // initialized.
  points_to_map_.SetPointerPointsToSet(
      object_repository_.GetCXXConstructExprThisPointer(construct_expr),
      {object_repository_.GetInitializedObject(construct_expr)});

  // Populate fn_params for the constructor call.
  std::vector<FunctionParameter> fn_params;

  for (size_t i = 0; i < construct_expr->getNumArgs(); i++) {
    clang::QualType arg_type =
        constructor->getParamDecl(i)->getType().getCanonicalType();
    fn_params.push_back(
        FunctionParameter{arg_type, callee_lifetimes.GetParamLifetimes(i),
                          object_repository_.GetCXXConstructExprArgumentObject(
                              construct_expr, i)});
  }

  clang::QualType type = constructor->getThisType();
  fn_params.push_back(FunctionParameter{
      type, callee_lifetimes.GetThisLifetimes(),
      object_repository_.GetCXXConstructExprThisPointer(construct_expr)});

  TransferLifetimesForCall(
      construct_expr, fn_params,
      ValueLifetimes::ForLifetimeLessType(constructor->getReturnType()),
      object_repository_, points_to_map_, constructor->getASTContext());
  return std::nullopt;
}

}  // namespace

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
