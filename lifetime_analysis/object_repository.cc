// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/object_repository.h"

#include <functional>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "lifetime_analysis/object.h"
#include "lifetime_analysis/visit_lifetimes.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/pointee_type.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/ErrorHandling.h"

namespace clang {
namespace tidy {
namespace lifetimes {

class ObjectRepository::VarDeclVisitor
    : public clang::RecursiveASTVisitor<VarDeclVisitor> {
 public:
  explicit VarDeclVisitor(ObjectRepository& object_repository)
      : object_repository_(object_repository) {}

  // We need to visit implicitly-defined constructors and assignment operators.
  bool shouldVisitImplicitCode() { return true; }

  bool VisitVarDecl(clang::VarDecl* var) {
    // Add objects for any local variables declared in this function.
    AddObjectForVar(var);
    return true;
  }

  bool VisitReturnStmt(clang::ReturnStmt* stmt) {
    const clang::Expr* expr = stmt->getRetValue();
    if (IsInitExprInitializingARecordObject(expr)) {
      PropagateInitializedObject(expr, *object_repository_.return_object_);
    }
    return true;
  }

  bool VisitMemberExpr(clang::MemberExpr* member) {
    if (auto* method =
            clang::dyn_cast<clang::CXXMethodDecl>(member->getMemberDecl());
        method && method->isStatic()) {
      // Create objects for static member functions.
      AddObjectForFunc(method);
    }
    return true;
  }

  bool VisitDeclRefExpr(clang::DeclRefExpr* decl_ref) {
    // Add objects for any global variables referenced in this function.
    // This also runs for local variables, but we don't have to treat those
    // differently as AddObjectForVar() protects against duplication.
    if (auto* var_decl = clang::dyn_cast<clang::VarDecl>(decl_ref->getDecl())) {
      AddObjectForVar(var_decl);
    }
    // Add objects for any function referenced in this function.
    if (auto* function_decl =
            clang::dyn_cast<clang::FunctionDecl>(decl_ref->getDecl())) {
      AddObjectForFunc(function_decl);
    }
    return true;
  }

  bool VisitObjCMessageExpr(clang::ObjCMessageExpr* msg_expr) {
    // ObjCMessageExpr is an initializer expression terminator, so we should
    // have walked down from the object which requires initialization to find
    // its terminating expressions, which should have found this expression and
    // connected it to that object already.
    if (!object_repository_.initialized_objects_.count(msg_expr)) {
      msg_expr->dump();
      llvm::report_fatal_error(
          "Missing initializer for ObjCMessageExpr, we did not record it "
          "when we visited something earlier in the tree yet?");
    }
    return true;
  }

  // Create objects for function call arguments.
  bool VisitCallExpr(clang::CallExpr* call_expr) {
    if (IsInitExprInitializingARecordObject(call_expr)) {
      assert(InitializedObjectWasPropagatedTo(call_expr));
    }

    // For calls to members, the type of the callee is a "bound member function
    // type", so we look at the declaration instead.
    if (auto member_call =
            clang::dyn_cast<clang::CXXMemberCallExpr>(call_expr)) {
      const clang::FunctionDecl* callee = call_expr->getDirectCallee();
      // TODO(veluca): pointers-to-members are not supported (yet?)
      assert(callee);
      AddObjectsForArguments(call_expr, callee->getType(),
                             /*index_shift=*/0);
      auto method = clang::cast<clang::CXXMethodDecl>(callee);
      clang::QualType type = method->getThisType();
      object_repository_.call_expr_this_pointers_[call_expr] =
          CreateLocalObject(type);
    } else if (auto op_call =
                   clang::dyn_cast<clang::CXXOperatorCallExpr>(call_expr)) {
      const clang::FunctionDecl* callee = call_expr->getDirectCallee();
      auto method = clang::dyn_cast<clang::CXXMethodDecl>(callee);
      AddObjectsForArguments(call_expr, callee->getType(),
                             /*index_shift=*/method ? 1 : 0);
      if (method) {
        clang::QualType type = method->getThisType();
        object_repository_.call_expr_this_pointers_[call_expr] =
            CreateLocalObject(type);
      }
    } else {
      // Always a function pointer.
      clang::QualType callee_type = call_expr->getCallee()->getType();
      AddObjectsForArguments(call_expr, callee_type, /*index_shift=*/0);
    }

    return true;
  }

  bool VisitCXXConstructExpr(clang::CXXConstructExpr* construct_expr) {
    assert(InitializedObjectWasPropagatedTo(construct_expr));

    // Create objects for constructor arguments.
    const clang::FunctionDecl* constructor = construct_expr->getConstructor();
    AddObjectsForArguments(construct_expr, constructor->getType(),
                           /*index_shift=*/0);
    clang::QualType type = construct_expr->getConstructor()->getThisType();
    object_repository_.call_expr_this_pointers_[construct_expr] =
        CreateLocalObject(type);
    return true;
  }

  bool VisitInitListExpr(clang::InitListExpr* init_list_expr) {
    // We only want to visit in Semantic form, we ignore Syntactic form.
    if (IsInitExprInitializingARecordObject(init_list_expr) &&
        init_list_expr->isSemanticForm() && !init_list_expr->isTransparent()) {
      assert(InitializedObjectWasPropagatedTo(init_list_expr));
    }
    return true;
  }

  bool VisitMaterializeTemporaryExpr(
      clang::MaterializeTemporaryExpr* temporary_expr) {
    object_repository_.temporary_objects_[temporary_expr] =
        AddTemporaryObjectForExpression(temporary_expr->getSubExpr());
    return true;
  }

  bool VisitCompoundStmt(clang::CompoundStmt* compound) {
    // Create temporary objects for any top-level `CXXTemporaryObjectExpr`s,
    // i.e. ones that are used as statements.
    for (clang::Stmt* stmt : compound->body()) {
      if (auto* temporary = clang::dyn_cast<CXXTemporaryObjectExpr>(stmt)) {
        AddTemporaryObjectForExpression(temporary);
      }
    }
    return true;
  }

  const Object* CreateLocalObject(clang::QualType type) {
    const Object* object =
        object_repository_.CreateObject(Lifetime::CreateLocal(), type);
    object_repository_.CreateObjects(
        *object, type,
        [](const clang::Expr*) { return Lifetime::CreateVariable(); },
        /*transitive=*/false);
    return object;
  }

  void AddObjectsForArguments(const clang::Expr* expr,
                              clang::QualType callee_type, size_t index_shift) {
    if (callee_type->isDependentType()) {
      // TODO(veluca): the fact that we reach this point is a clang bug: it
      // should not be possible to reach dependent types from a template
      // instantiation. See also the following discussion, where richardsmith@
      // agrees this looks like a Clang bug and suggests how it might be fixed:
      // https://chat.google.com/room/AAAAb6i7WDQ/OvLC9NgO91A
      return;
    }
    if (callee_type->isPointerType()) {
      callee_type = callee_type->getPointeeType();
    }
    // TODO(veluca): figure out how to create a test where the callee is a
    // ParenType.
    // For reference, this was triggered in the implementation of `bsearch`.
    callee_type = callee_type.IgnoreParens();
    assert(callee_type->isFunctionType());
    // TODO(veluca): could this be a clang::FunctionNoProtoType??
    const auto* fn_type = clang::cast<clang::FunctionProtoType>(callee_type);
    for (size_t i = 0; i < fn_type->getNumParams(); ++i) {
      object_repository_
          .call_expr_args_objects_[std::make_pair(expr, i + index_shift)] =
          CreateLocalObject(fn_type->getParamType(i));
    }
  }

  void AddObjectForVar(clang::VarDecl* var) {
    if (object_repository_.object_repository_.count(var)) {
      return;
    }

    Lifetime lifetime;
    LifetimeFactory lifetime_factory;

    switch (var->getStorageClass()) {
      case clang::SC_Extern:
      case clang::SC_Static:
      case clang::SC_PrivateExtern:
        lifetime = Lifetime::Static();
        lifetime_factory = [](const clang::Expr*) {
          return Lifetime::Static();
        };
        break;
      default:
        lifetime = Lifetime::CreateLocal();
        lifetime_factory = [](const clang::Expr*) {
          return Lifetime::CreateVariable();
        };
        break;
    }

    const Object* object =
        object_repository_.CreateObject(lifetime, var->getType());

    object_repository_.CreateObjects(
        *object, var->getType(), lifetime_factory,
        /*transitive=*/clang::isa<clang::ParmVarDecl>(var) ||
            lifetime == Lifetime::Static());

    object_repository_.object_repository_[var] = *object;
    object_repository_.object_value_types_[*object] =
        var->getType()->isArrayType() ? ObjectValueType::kMultiValued
                                      : ObjectValueType::kSingleValued;

    // Remember the original value of function parameters.
    if (auto parm_var_decl = clang::dyn_cast<const clang::ParmVarDecl>(var)) {
      object_repository_.initial_parameter_object_[parm_var_decl] =
          object_repository_.CloneObject(object);
    }

    if (var->hasInit() && var->getType()->isRecordType()) {
      PropagateInitializedObject(var->getInit(), *object);
    }
  }

  void AddObjectForFunc(clang::FunctionDecl* func) {
    if (object_repository_.object_repository_.count(func)) {
      return;
    }

    object_repository_.object_repository_[func] =
        *object_repository_.CreateObjectFromFunctionDecl(*func);
  }

  const Object* AddTemporaryObjectForExpression(clang::Expr* expr) {
    clang::QualType type = expr->getType().getCanonicalType();
    const Object* object =
        object_repository_.CreateObject(Lifetime::CreateLocal(), type);

    object_repository_.CreateObjects(
        *object, type,
        [](const clang::Expr*) { return Lifetime::CreateVariable(); },
        /*transitive=*/false);

    if (type->isRecordType()) {
      PropagateInitializedObject(expr, *object);
    }
    return object;
  }

  // Propagates an `object` of record type that is to be initialized to the
  // expressions that actually perform the initialization (we call these
  // "terminating expressions").
  //
  // `expr` is the initializer for a variable; this will contain one or
  // several terminating expressions (such as a CXXConstructExpr, InitListExpr,
  // or CallExpr).
  //
  // Note that not all terminating expressions below `expr` necessarily
  // initialize `object`; some of these terminating expressions may also
  // initialize temporary objects. This function takes care to propagate
  // `object` only to the appropriate terminating expressions.
  //
  // The mapping from a terminating expression to the object it initializes
  // is stored in `object_repository_.initialized_objects_`.
  void PropagateInitializedObject(const clang::Expr* expr, Object object) {
    // TODO(danakj): Use StmtVisitor to implement this method.
    // copybara:begin_strip
    // Context and hints:
    // http://cl/414017975/depot/lifetime_analysis/var_decl_objects.cc?version=s3#324
    // copybara:end_strip

    // Terminating expressions. Expressions that don't initialize a record
    // object can not be such, and their existence is unexpected as we should
    // be converting to and initializing a record object from such expressions
    // further up in the initializer expression's AST. We will assert later in
    // this function if we find this situation somehow due to incorrect
    // expectations in this comment.
    if (IsInitExprInitializingARecordObject(expr)) {
      if (clang::isa<clang::CXXConstructExpr>(expr) ||
          clang::isa<clang::CallExpr>(expr) ||
          clang::isa<clang::ObjCMessageExpr>(expr) ||
          clang::isa<clang::LambdaExpr>(expr)) {
        object_repository_.initialized_objects_[expr] = object;
        return;
      }
      if (auto* e = clang::dyn_cast<clang::InitListExpr>(expr)) {
        if (!e->isSemanticForm()) return;
        if (e->isTransparent()) {
          // A field initializer like `S s{cond ? S{} : S{}}` is considered
          // transparent, and the actual initializer is within.
          for (const clang::Expr* init : e->inits()) {
            PropagateInitializedObject(init, object);
          }
        } else {
          object_repository_.initialized_objects_[e] = object;
        }
        return;
      }
    }

    // Expressions to walk through. Logic is similar to the AggExprEmitter in
    // clang third_party/llvm-project/clang/lib/CodeGen/CGExprAgg.cpp though we
    // don't have to visit all the sub-expressions that clang codegen needs to,
    // as we can stop at terminating expressions and ignore many expressions
    // that don't occur in the code we're analyzing.
    if (auto* e = clang::dyn_cast<clang::ParenExpr>(expr)) {
      PropagateInitializedObject(e->getSubExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::UnaryOperator>(expr)) {
      PropagateInitializedObject(e->getSubExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::SubstNonTypeTemplateParmExpr>(expr)) {
      PropagateInitializedObject(e->getReplacement(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::CastExpr>(expr)) {
      PropagateInitializedObject(e->getSubExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::CXXDefaultArgExpr>(expr)) {
      PropagateInitializedObject(e->getExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::CXXDefaultInitExpr>(expr)) {
      PropagateInitializedObject(e->getExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::ExprWithCleanups>(expr)) {
      PropagateInitializedObject(e->getSubExpr(), object);
      return;
    }

    // Expressions that produce a temporary object.
    if (auto* e = clang::dyn_cast<clang::BinaryOperator>(expr)) {
      if (e->isCommaOp()) {
        AddTemporaryObjectForExpression(e->getLHS());
        PropagateInitializedObject(e->getRHS(), object);
        return;
      }

      // Any other binary operator should not produce a record type, it would be
      // used to construct a record further up the AST, so we should not arrive
      // here.
      expr->dump();
      llvm::report_fatal_error(
          "Unexpected binary operator in initializer expression tree");
    }
    if (auto* e = clang::dyn_cast<clang::AbstractConditionalOperator>(expr)) {
      AddTemporaryObjectForExpression(e->getCond());
      PropagateInitializedObject(e->getTrueExpr(), object);
      PropagateInitializedObject(e->getFalseExpr(), object);
      return;
    }

    expr->dump();
    llvm::report_fatal_error(
        "Unexpected expression in initializer expression tree");
  }

  bool InitializedObjectWasPropagatedTo(clang::Expr* terminating_expr) {
    // An expression that initializes an object should have already been
    // connected to the object it initializes. We should have walked down from
    // the object which requires initialization to find its terminating
    // expressions.
    if (!object_repository_.initialized_objects_.count(terminating_expr)) {
      llvm::errs() << "Missing initialized object for terminating expression, "
                      "we did not record it when we visited something earlier "
                      "in the tree yet?\n";
      terminating_expr->dump();
      return false;
    } else {
      return true;
    }
  }

  void TraverseCXXMemberInitializers(
      const clang::CXXConstructorDecl* constructor) {
    // For constructors, we also need to create lifetimes for variables
    // referenced by in-class member initializers; the visitor by default only
    // visits expressions in the initializer list.
    // We also need to associate member initializers with the members they
    // initialize.
    for (const auto* init : constructor->inits()) {
      const auto* init_expr = init->getInit();
      if (const auto* default_init =
              clang::dyn_cast<clang::CXXDefaultInitExpr>(init_expr)) {
        init_expr = default_init->getExpr();
      }

      if (init->getMember() && init->getMember()->getType()->isRecordType()) {
        std::optional<Object> this_object = object_repository_.GetThisObject();
        assert(this_object.has_value());

        Object field_object =
            object_repository_.GetFieldObject(*this_object, init->getMember());
        PropagateInitializedObject(init_expr, field_object);
      } else if (init->getBaseClass()) {
        std::optional<Object> this_object = object_repository_.GetThisObject();
        assert(this_object.has_value());

        Object base_object = object_repository_.GetBaseClassObject(
            *this_object, init->getBaseClass());
        PropagateInitializedObject(init_expr, base_object);
      }

      // Traverse after finishing with the outer expression, including
      // connecting the initializer (constructor) to its object.
      TraverseStmt(const_cast<clang::Expr*>(init_expr));
    }
  }

  ObjectRepository& object_repository_;
};

ObjectRepository::ObjectRepository(const clang::FunctionDecl* func) {
  const auto* method_decl = clang::dyn_cast<clang::CXXMethodDecl>(func);

  const auto* definition = func->getDefinition();
  assert(definition || (method_decl && method_decl->isPure()));
  if (definition) func = definition;

  // For the return value, we only need to create field objects.
  return_object_ = CreateObject(Lifetime::CreateLocal(), func->getReturnType());
  CreateObjects(
      *return_object_, func->getReturnType(),
      [](const clang::Expr*) { return Lifetime::CreateLocal(); },
      /*transitive=*/false);

  if (method_decl) {
    if (!method_decl->isStatic()) {
      this_object_ = CreateObject(Lifetime::CreateVariable(),
                                  method_decl->getThisObjectType());
      CreateObjects(
          **this_object_, method_decl->getThisObjectType(),
          [](const clang::Expr*) { return Lifetime::CreateVariable(); },
          /*transitive=*/true);
    }
  }

  VarDeclVisitor decl_visitor(*this);
  if (auto* constructor = clang::dyn_cast<clang::CXXConstructorDecl>(func)) {
    decl_visitor.TraverseCXXMemberInitializers(constructor);
  }
  decl_visitor.TraverseFunctionDecl(const_cast<clang::FunctionDecl*>(func));
}

std::string ObjectRepository::DebugString() const {
  std::string result;
  llvm::raw_string_ostream os(result);

  if (this_object_) {
    os << "This " << (*this_object_)->DebugString() << "\n";
  }
  for (const auto& [decl, object] : object_repository_) {
    os << decl->getDeclKindName() << " " << decl << " (";
    decl->printName(os);
    os << ") object: " << object.DebugString() << "\n";
  }
  for (const auto& [expr_i, object] : call_expr_args_objects_) {
    const auto& [expr, i] = expr_i;
    os << "Call " << expr << " (arg " << i
       << ") object: " << object->DebugString() << "\n";
  }
  for (const auto& [expr, object] : call_expr_this_pointers_) {
    os << "Call " << expr << " (this) pointer: " << object->DebugString()
       << "\n";
  }
  os << "InitialPointsToMap:\n" << initial_points_to_map_.DebugString() << "\n";
  for (const auto& [field, object] : field_object_map_) {
    os << "Field '";
    field.second->printName(os);
    os << "' on " << field.first.Type().getAsString()
       << " object: " << object->DebugString() << "\n";
  }
  os << "Return " << return_object_->DebugString() << "\n";
  os.flush();
  return result;
}

const Object* ObjectRepository::CreateObject(Lifetime lifetime,
                                             clang::QualType type) {
  return new (object_allocator_.Allocate()) Object(lifetime, type);
}

const Object* ObjectRepository::CreateObjectFromFunctionDecl(
    const clang::FunctionDecl& func) {
  return new (object_allocator_.Allocate()) Object(func);
}

Object ObjectRepository::GetDeclObject(const clang::ValueDecl* decl) const {
  auto iter = object_repository_.find(decl);
  if (iter == object_repository_.end()) {
    llvm::errs() << "Didn't find object for Decl:\n";
    decl->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find object for Decl");
  }
  return iter->second;
}

Object ObjectRepository::GetTemporaryObject(
    const clang::MaterializeTemporaryExpr* expr) const {
  auto iter = temporary_objects_.find(expr);
  if (iter == temporary_objects_.end()) {
    llvm::errs() << "Didn't find object for temporary expression:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find object for temporary expression");
  }
  return *iter->second;
}

Object ObjectRepository::GetOriginalParameterValue(
    const clang::ParmVarDecl* var_decl) const {
  auto iter = initial_parameter_object_.find(var_decl);
  if (iter == initial_parameter_object_.end()) {
    llvm::errs() << "Didn't find caller object for parameter:\n";
    var_decl->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find caller object for parameter");
  }
  return *iter->second;
}

Object ObjectRepository::GetCallExprArgumentObject(const clang::CallExpr* expr,
                                                   size_t arg_index) const {
  auto iter = call_expr_args_objects_.find(std::make_pair(expr, arg_index));
  if (iter == call_expr_args_objects_.end()) {
    llvm::errs() << "Didn't find object for argument " << arg_index
                 << " of call:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find object for argument");
  }
  return *iter->second;
}

Object ObjectRepository::GetCallExprThisPointer(
    const clang::CallExpr* expr) const {
  auto iter = call_expr_this_pointers_.find(expr);
  if (iter == call_expr_this_pointers_.end()) {
    llvm::errs() << "Didn't find `this` object for call:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find `this` object for call");
  }
  return *iter->second;
}

Object ObjectRepository::GetCXXConstructExprArgumentObject(
    const clang::CXXConstructExpr* expr, size_t arg_index) const {
  auto iter = call_expr_args_objects_.find(std::make_pair(expr, arg_index));
  if (iter == call_expr_args_objects_.end()) {
    llvm::errs() << "Didn't find object for argument " << arg_index
                 << " of constructor call:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error(
        "Didn't find object for argument of constructor call");
  }
  return *iter->second;
}

Object ObjectRepository::GetCXXConstructExprThisPointer(
    const clang::CXXConstructExpr* expr) const {
  auto iter = call_expr_this_pointers_.find(expr);
  if (iter == call_expr_this_pointers_.end()) {
    llvm::errs() << "Didn't find `this` object for constructor:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find `this` object for constructor");
  }
  return *iter->second;
}

Object ObjectRepository::GetInitializedObject(
    const clang::Expr* initializer_expr) const {
  assert(clang::isa<clang::CXXConstructExpr>(initializer_expr) ||
         clang::isa<clang::InitListExpr>(initializer_expr) ||
         clang::isa<clang::CallExpr>(initializer_expr));

  auto iter = initialized_objects_.find(initializer_expr);
  if (iter == initialized_objects_.end()) {
    llvm::errs() << "Didn't find object for initializer:\n";
    initializer_expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find object for initializer");
  }
  return iter->second;
}

ObjectRepository::ObjectValueType ObjectRepository::GetObjectValueType(
    Object object) const {
  auto iter = object_value_types_.find(object);
  // If we don't know this lifetime, we conservatively assume it to be
  // multi-valued.
  if (iter == object_value_types_.end()) {
    return ObjectValueType::kMultiValued;
  }
  return iter->second;
}

Object ObjectRepository::GetFieldObject(Object struct_object,
                                        const clang::FieldDecl* field) const {
  std::optional<Object> field_object =
      GetFieldObjectInternal(struct_object, field);
  if (!field_object.has_value()) {
    llvm::errs() << "On an object of type "
                 << struct_object.Type().getAsString()
                 << ", trying to get field:\n";
    field->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find field object");
  }
  return *field_object;
}

ObjectSet ObjectRepository::GetFieldObject(
    const ObjectSet& struct_objects, const clang::FieldDecl* field) const {
  ObjectSet ret;
  for (Object object : struct_objects) {
    ret.Add(GetFieldObject(object, field));
  }
  return ret;
}

Object ObjectRepository::GetBaseClassObject(Object struct_object,
                                            const clang::Type* base) const {
  base = base->getCanonicalTypeInternal().getTypePtr();
  auto iter = base_object_map_.find(std::make_pair(struct_object, base));
  if (iter == base_object_map_.end()) {
    llvm::errs() << "On object " << struct_object.DebugString()
                 << ", trying to get base:\n";
    base->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find base object");
  }
  return iter->second;
}

ObjectSet ObjectRepository::GetBaseClassObject(const ObjectSet& struct_objects,
                                               const clang::Type* base) const {
  ObjectSet ret;
  for (Object object : struct_objects) {
    ret.Add(GetBaseClassObject(object, base));
  }
  return ret;
}

Object ObjectRepository::CreateStaticObject(clang::QualType type) {
  auto iter = static_objects_.find(type);
  if (iter != static_objects_.end()) {
    return *iter->second;
  }

  const Object* object = CreateObject(Lifetime::Static(), type);
  static_objects_[type] = object;

  CreateObjects(
      *object, type, [](const clang::Expr*) { return Lifetime::Static(); },
      true);

  return *object;
}

void ObjectRepository::CreateObjects(Object root_object, clang::QualType type,
                                     LifetimeFactory lifetime_factory,
                                     bool transitive) {
  class Visitor : public LifetimeVisitor {
   public:
    Visitor(ObjectRepository& object_repository, bool create_transitive_objects)
        : object_repository_(object_repository),
          create_transitive_objects_(create_transitive_objects) {}

    Object GetFieldObject(const ObjectSet& objects,
                          const clang::FieldDecl* field) override {
      assert(!objects.empty());
      std::optional<const Object*> field_object = std::nullopt;

      for (Object object : objects) {
        if (auto iter = object_repository_.field_object_map_.find(
                std::make_pair(object, field));
            iter != object_repository_.field_object_map_.end()) {
          field_object = iter->second;
        }
      }
      if (!field_object.has_value()) {
        field_object = object_repository_.CreateObject(
            (*objects.begin()).GetLifetime(), field->getType());
      }
      for (Object object : objects) {
        object_repository_.field_object_map_[std::make_pair(object, field)] =
            *field_object;
      }
      return **field_object;
    }

    Object GetBaseClassObject(const ObjectSet& objects,
                              clang::QualType base) override {
      assert(!objects.empty());
      base = base.getCanonicalType();
      std::optional<Object> base_object = std::nullopt;

      for (Object object : objects) {
        if (auto iter = object_repository_.base_object_map_.find(
                std::make_pair(object, &*base));
            iter != object_repository_.base_object_map_.end()) {
          base_object = iter->second;
        }
      }
      if (!base_object.has_value()) {
        base_object = *object_repository_.CreateObject(
            (*objects.begin()).GetLifetime(), base);
      }
      for (Object object : objects) {
        object_repository_.base_object_map_[std::make_pair(object, &*base)] =
            *base_object;
      }
      return *base_object;
    }

    ObjectSet Traverse(const ObjectLifetimes& lifetimes,
                       const ObjectSet& objects,
                       int /*pointee_depth*/) override {
      if (!create_transitive_objects_) return {};
      if (PointeeType(lifetimes.GetValueLifetimes().Type()).isNull()) {
        return {};
      }

      const auto& cache_key =
          lifetimes.GetValueLifetimes().GetPointeeLifetimes();

      Object child_pointee;
      if (auto iter = object_cache_.find(cache_key);
          iter == object_cache_.end()) {
        child_pointee = *object_repository_.CreateObject(
            lifetimes.GetValueLifetimes().GetPointeeLifetimes().GetLifetime(),
            PointeeType(lifetimes.GetValueLifetimes().Type()));
        object_cache_[cache_key] = child_pointee;
      } else {
        child_pointee = iter->second;
      }

      object_repository_.initial_points_to_map_.SetPointerPointsToSet(
          objects, {child_pointee});
      return ObjectSet{child_pointee};
    }

   private:
    ObjectRepository& object_repository_;
    bool create_transitive_objects_;
    // Inside of a given VarDecl, we re-use the same Object for all the
    // sub-objects with the same type and lifetimes. This avoids infinite loops
    // in the case of structs like lists.
    llvm::DenseMap<ObjectLifetimes, Object> object_cache_;
  };
  Visitor visitor(*this, transitive);
  VisitLifetimes(
      {root_object}, type,
      ObjectLifetimes(root_object.GetLifetime(),
                      ValueLifetimes::Create(type, lifetime_factory).get()),
      visitor);
}

// Clones an object and its base classes and fields, if any.
const Object* ObjectRepository::CloneObject(const Object* object) {
  struct ObjectPair {
    Object orig_object;
    const Object* new_object;
  };
  auto clone = [this](Object obj) {
    auto new_obj = CreateObject(obj.GetLifetime(), obj.Type());
    initial_points_to_map_.SetPointerPointsToSet(
        *new_obj, initial_points_to_map_.GetPointerPointsToSet(obj));
    return new_obj;
  };
  const Object* new_root = clone(*object);
  std::vector<ObjectPair> object_stack{{*object, new_root}};
  while (!object_stack.empty()) {
    auto [orig_object, new_object] = object_stack.back();
    assert(orig_object.Type() == new_object->Type());
    object_stack.pop_back();
    auto record_type = orig_object.Type()->getAs<clang::RecordType>();
    if (!record_type) {
      continue;
    }

    // Base classes.
    if (auto* cxxrecord =
            clang::dyn_cast<clang::CXXRecordDecl>(record_type->getDecl())) {
      for (const clang::CXXBaseSpecifier& base : cxxrecord->bases()) {
        auto base_obj = GetBaseClassObject(orig_object, base.getType());
        const Object* new_base_obj = clone(base_obj);
        base_object_map_[std::make_pair(
            *new_object, base.getType().getCanonicalType().getTypePtr())] =
            *new_base_obj;
        object_stack.push_back(ObjectPair{base_obj, new_base_obj});
      }
    }

    // Fields.
    for (auto f : record_type->getDecl()->fields()) {
      auto field_obj = GetFieldObject(orig_object, f);
      const Object* new_field_obj = clone(field_obj);
      field_object_map_[std::make_pair(*new_object, f)] = new_field_obj;
      object_stack.push_back(ObjectPair{field_obj, new_field_obj});
    }
  }
  return new_root;
}

std::optional<Object> ObjectRepository::GetFieldObjectInternal(
    Object struct_object, const clang::FieldDecl* field) const {
  auto iter = field_object_map_.find(std::make_pair(struct_object, field));
  if (iter != field_object_map_.end()) {
    return *iter->second;
  }
  if (auto* cxxrecord = clang::dyn_cast<clang::CXXRecordDecl>(
          struct_object.Type()->getAs<clang::RecordType>()->getDecl())) {
    for (const clang::CXXBaseSpecifier& base : cxxrecord->bases()) {
      std::optional<Object> field_object = GetFieldObjectInternal(
          GetBaseClassObject(struct_object, base.getType()), field);
      if (field_object.has_value()) {
        return field_object;
      }
    }
  }
  return std::nullopt;
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
