// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/object_repository.h"

#include <cassert>
#include <cstddef>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/strings/str_cat.h"
#include "lifetime_analysis/builtin_lifetimes.h"
#include "lifetime_analysis/object.h"
#include "lifetime_analysis/object_set.h"
#include "lifetime_analysis/points_to_map.h"
#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/pointee_type.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/ExprObjC.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/raw_ostream.h"

namespace clang {
namespace tidy {
namespace lifetimes {

FunctionLifetimesOrError GetFunctionLifetimes(
    const FunctionDecl* decl, const FunctionLifetimesMap& known_lifetimes) {
  // Note that this cannot be a simple .find() on the map because `decl` might
  // be a builtin.
  bool is_builtin = decl->getBuiltinID() != 0;
  if (is_builtin) {
    return GetBuiltinLifetimes(decl);
  }
  if (!known_lifetimes.count(decl->getCanonicalDecl())) {
    return FunctionAnalysisError(
        absl::StrCat("Unknown callee ", decl->getNameAsString()));
  }
  return known_lifetimes.lookup(decl->getCanonicalDecl());
}

class ObjectRepository::VarDeclVisitor
    : public clang::RecursiveASTVisitor<VarDeclVisitor> {
 public:
  explicit VarDeclVisitor(ObjectRepository& object_repository,
                          const FunctionLifetimesMap& callee_lifetimes)
      : object_repository_(object_repository),
        callee_lifetimes_(callee_lifetimes) {}

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
      PropagateResultObject(expr, object_repository_.return_object_);
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
    // have walked down from the result object to find its terminating
    // expressions, which should have found this expression and connected it to
    // that object already.
    if (!object_repository_.result_objects_.count(msg_expr)) {
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
      assert(ResultObjectWasPropagatedTo(call_expr));
    }

    FunctionLifetimeFactorySingleCallback lifetime_factory(
        [](auto) { return Lifetime::CreateVariable(); });

    // If we have a direct callee, construct a FunctionLifetimes out of the
    // function/method definition.
    if (auto callee = call_expr->getDirectCallee()) {
      bool is_operator_call = clang::isa<clang::CXXOperatorCallExpr>(call_expr);
      bool is_method = clang::isa<clang::CXXMethodDecl>(callee);
      object_repository_.call_expr_virtual_lifetimes_[call_expr] =
          FunctionLifetimes::CreateForDecl(callee, lifetime_factory).get();
      PrepareFunctionCall(
          call_expr, /*index_shift=*/is_operator_call && is_method ? 1 : 0);
    } else {
      // Always a function pointer.
      // TODO(veluca): pointers-to-members are not supported (yet?)
      clang::QualType callee_type =
          call_expr->getCallee()->getType()->getPointeeType().IgnoreParens();
      // TODO(veluca): what about FunctionNoProtoType??
      object_repository_.call_expr_virtual_lifetimes_[call_expr] =
          FunctionLifetimes::CreateForFunctionType(
              clang::cast<clang::FunctionProtoType>(callee_type),
              lifetime_factory)
              .get();
      PrepareFunctionCall(call_expr, /*index_shift=*/0);
    }

    return true;
  }

  bool VisitStringLiteral(clang::StringLiteral* lit_expr) {
    object_repository_.string_literal_object_ = object_repository_.CreateObject(
        lit_expr->getType(), Lifetime::Static(),
        [](const clang::Expr*) { return Lifetime::Static(); });
    return true;
  }

  bool VisitCXXConstructExpr(clang::CXXConstructExpr* construct_expr) {
    assert(ResultObjectWasPropagatedTo(construct_expr));

    FunctionLifetimeFactorySingleCallback lifetime_factory(
        [](auto) { return Lifetime::CreateVariable(); });
    const clang::FunctionDecl* constructor = construct_expr->getConstructor();
    object_repository_.call_expr_virtual_lifetimes_[construct_expr] =
        FunctionLifetimes::CreateForDecl(constructor, lifetime_factory).get();
    PrepareFunctionCall(construct_expr,
                        /*index_shift=*/0);
    return true;
  }

  bool VisitInitListExpr(clang::InitListExpr* init_list_expr) {
    // We only want to visit in Semantic form, we ignore Syntactic form.
    if (IsInitExprInitializingARecordObject(init_list_expr) &&
        init_list_expr->isSemanticForm() && !init_list_expr->isTransparent()) {
      assert(ResultObjectWasPropagatedTo(init_list_expr));
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

  void PrepareFunctionCall(const clang::Expr* expr, size_t index_shift) {
    const auto& func_lifetimes =
        object_repository_.call_expr_virtual_lifetimes_[expr];
    auto make_object = [this](const ValueLifetimes& lifetime) {
      return object_repository_.CreateObject(
          ObjectLifetimes(Lifetime::CreateLocal(), lifetime),
          object_repository_.initial_points_to_map_);
    };
    for (size_t i = 0; i < func_lifetimes.GetNumParams(); ++i) {
      object_repository_
          .call_expr_args_objects_[std::make_pair(expr, i + index_shift)] =
          make_object(func_lifetimes.GetParamLifetimes(i));
    }
    if (func_lifetimes.IsNonStaticMethod()) {
      object_repository_.call_expr_this_pointers_[expr] =
          make_object(func_lifetimes.GetThisLifetimes());
    }
    object_repository_.call_expr_ret_objects_[expr] =
        make_object(func_lifetimes.GetReturnLifetimes());
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

    const Object* object = object_repository_.CreateObject(
        var->getType(), lifetime, lifetime_factory);

    object_repository_.object_repository_[var] = object;
    if (!var->getType()->isArrayType()) {
      object_repository_.initial_single_valued_objects_.Add(object);
    }

    // Remember the original value of function parameters.
    if (auto parm_var_decl = clang::dyn_cast<const clang::ParmVarDecl>(var)) {
      object_repository_.initial_parameter_object_[parm_var_decl] =
          object_repository_.CloneObject(object);
    }

    if (var->hasInit() && var->getType()->isRecordType()) {
      PropagateResultObject(var->getInit(), object);
    }
  }

  void AddObjectForFunc(clang::FunctionDecl* func) {
    if (object_repository_.object_repository_.count(func)) {
      return;
    }

    FunctionLifetimesOrError func_lifetimes =
        GetFunctionLifetimes(func, callee_lifetimes_);
    if (std::holds_alternative<FunctionAnalysisError>(func_lifetimes)) {
      error_ = "No lifetimes for callee '" + func->getNameAsString() +
               "': " + std::get<FunctionAnalysisError>(func_lifetimes).message;
    } else {
      object_repository_.object_repository_[func] =
          object_repository_.ConstructObject(
              Lifetime::Static(), func->getType(),
              std::get<FunctionLifetimes>(func_lifetimes));
    }
  }

  const Object* AddTemporaryObjectForExpression(clang::Expr* expr) {
    clang::QualType type = expr->getType().getCanonicalType();
    const Object* object = object_repository_.CreateObject(
        type, Lifetime::CreateLocal(),
        [](const clang::Expr*) { return Lifetime::CreateVariable(); });

    if (type->isRecordType()) {
      PropagateResultObject(expr, object);
    }
    return object;
  }

  // Propagates a result object `object` of record type to the expressions that
  // actually perform the initialization (we call these "terminating
  // expressions").
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
  // The mapping from a terminating expression to the result object it
  // initializes is stored in `object_repository_.result_objects_`.
  void PropagateResultObject(const clang::Expr* expr, const Object* object) {
    // TODO(danakj): Use StmtVisitor to implement this method.

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
        object_repository_.result_objects_[expr] = object;
        return;
      }
      if (auto* e = clang::dyn_cast<clang::InitListExpr>(expr)) {
        if (!e->isSemanticForm()) return;
        if (e->isTransparent()) {
          // A field initializer like `S s{cond ? S{} : S{}}` is considered
          // transparent, and the actual initializer is within.
          for (const clang::Expr* init : e->inits()) {
            PropagateResultObject(init, object);
          }
        } else {
          object_repository_.result_objects_[e] = object;
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
      PropagateResultObject(e->getSubExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::UnaryOperator>(expr)) {
      PropagateResultObject(e->getSubExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::SubstNonTypeTemplateParmExpr>(expr)) {
      PropagateResultObject(e->getReplacement(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::CastExpr>(expr)) {
      PropagateResultObject(e->getSubExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::CXXDefaultArgExpr>(expr)) {
      PropagateResultObject(e->getExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::CXXDefaultInitExpr>(expr)) {
      PropagateResultObject(e->getExpr(), object);
      return;
    }
    if (auto* e = clang::dyn_cast<clang::ExprWithCleanups>(expr)) {
      PropagateResultObject(e->getSubExpr(), object);
      return;
    }

    // Expressions that produce a temporary object.
    if (auto* e = clang::dyn_cast<clang::BinaryOperator>(expr)) {
      if (e->isCommaOp()) {
        AddTemporaryObjectForExpression(e->getLHS());
        PropagateResultObject(e->getRHS(), object);
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
      PropagateResultObject(e->getTrueExpr(), object);
      PropagateResultObject(e->getFalseExpr(), object);
      return;
    }

    expr->dump();
    llvm::report_fatal_error(
        "Unexpected expression in initializer expression tree");
  }

  bool ResultObjectWasPropagatedTo(clang::Expr* terminating_expr) {
    // An expression that initializes an object should have already been
    // connected to the result object it initializes. We should have walked down
    // from the result object to find its terminating expressions.
    if (!object_repository_.result_objects_.count(terminating_expr)) {
      llvm::errs() << "Missing result object for terminating expression, "
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
        std::optional<const Object*> this_object =
            object_repository_.GetThisObject();
        assert(this_object.has_value());

        const Object* field_object =
            object_repository_.GetFieldObject(*this_object, init->getMember());
        PropagateResultObject(init_expr, field_object);
      } else if (init->getBaseClass()) {
        std::optional<const Object*> this_object =
            object_repository_.GetThisObject();
        assert(this_object.has_value());

        const Object* base_object = object_repository_.GetBaseClassObject(
            *this_object, init->getBaseClass());
        PropagateResultObject(init_expr, base_object);
      }

      // Traverse after finishing with the outer expression, including
      // connecting the initializer (constructor) to its object.
      TraverseStmt(const_cast<clang::Expr*>(init_expr));
    }
  }

  ObjectRepository& object_repository_;
  const FunctionLifetimesMap& callee_lifetimes_;
  std::optional<std::string> error_;
};

llvm::Expected<ObjectRepository> ObjectRepository::Create(
    const clang::FunctionDecl* func,
    const llvm::DenseMap<const clang::FunctionDecl*, FunctionLifetimesOrError>&
        callee_lifetimes) {
  ObjectRepository object_repository;
  const auto* method_decl = clang::dyn_cast<clang::CXXMethodDecl>(func);

  const auto* definition = func->getDefinition();
  assert(definition || (method_decl && method_decl->isPureVirtual()));
  if (definition) func = definition;
  object_repository.func_ = func;

  object_repository.return_object_ = object_repository.CreateObject(
      func->getReturnType(), Lifetime::CreateVariable(),
      [](const clang::Expr*) { return Lifetime::CreateVariable(); });

  if (method_decl) {
    if (!method_decl->isStatic()) {
      object_repository.this_object_ = object_repository.CreateObject(
          method_decl->getFunctionObjectParameterType(),
          Lifetime::CreateVariable(),
          [](const clang::Expr*) { return Lifetime::CreateVariable(); });
    }
  }

  VarDeclVisitor decl_visitor(object_repository, callee_lifetimes);
  if (auto* constructor = clang::dyn_cast<clang::CXXConstructorDecl>(func)) {
    decl_visitor.TraverseCXXMemberInitializers(constructor);
  }
  decl_visitor.TraverseFunctionDecl(const_cast<clang::FunctionDecl*>(func));

  if (decl_visitor.error_.has_value()) {
    return llvm::createStringError(llvm::inconvertibleErrorCode(),
                                   *decl_visitor.error_);
  }
  return object_repository;
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
    os << ") object: " << object->DebugString() << "\n";
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
    field.second->printName(os,
                            field.second->getASTContext().getPrintingPolicy());
    os << "' on " << field.first->DebugString()
       << " object: " << object->DebugString() << "\n";
  }
  for (const auto& [base, object] : base_object_map_) {
    os << "Base of type " << clang::QualType(base.second, 0).getAsString()
       << " of " << base.first->DebugString()
       << " object: " << object->DebugString() << "\n";
  }
  os << "Return " << return_object_->DebugString() << "\n";
  os.flush();
  return result;
}

const Object* ObjectRepository::GetDeclObject(
    const clang::ValueDecl* decl) const {
  auto iter = object_repository_.find(decl);
  if (iter == object_repository_.end()) {
    llvm::errs() << "Didn't find object for Decl:\n";
    decl->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find object for Decl");
  }
  return iter->second;
}

const Object* ObjectRepository::GetTemporaryObject(
    const clang::MaterializeTemporaryExpr* expr) const {
  auto iter = temporary_objects_.find(expr);
  if (iter == temporary_objects_.end()) {
    llvm::errs() << "Didn't find object for temporary expression:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find object for temporary expression");
  }
  return iter->second;
}

const Object* ObjectRepository::GetOriginalParameterValue(
    const clang::ParmVarDecl* var_decl) const {
  auto iter = initial_parameter_object_.find(var_decl);
  if (iter == initial_parameter_object_.end()) {
    llvm::errs() << "Didn't find caller object for parameter:\n";
    var_decl->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find caller object for parameter");
  }
  return iter->second;
}

FunctionLifetimes ObjectRepository::GetOriginalFunctionLifetimes() const {
  FunctionLifetimes ret;
  auto get_initial_lifetimes_or_die = [&](const Object* object) {
    auto iter = initial_object_lifetimes_.find(object);
    if (iter == initial_object_lifetimes_.end()) {
      llvm::errs() << "Didn't find lifetimes for object "
                   << object->DebugString();
      llvm::report_fatal_error("Didn't find lifetimes for object");
    }
    return iter->second;
  };
  ret.return_lifetimes_ =
      get_initial_lifetimes_or_die(GetReturnObject()).GetValueLifetimes();
  if (this_object_.has_value()) {
    ret.this_lifetimes_ = ValueLifetimes::PointerTo(
        clang::dyn_cast<clang::CXXMethodDecl>(func_)->getThisType(),
        get_initial_lifetimes_or_die(*this_object_));
  }
  ret.param_lifetimes_.reserve(func_->getNumParams());
  for (size_t i = 0; i < func_->getNumParams(); i++) {
    ret.param_lifetimes_.push_back(
        get_initial_lifetimes_or_die(
            GetOriginalParameterValue(func_->getParamDecl(i)))
            .GetValueLifetimes());
  }
  if (!ret.IsValidForDecl(func_)) {
    llvm::errs() << "Internal error: did not produce valid function lifetimes";
    llvm::report_fatal_error(
        "Internal error: did not produce valid function lifetimes");
  }
  return ret;
}

const Object* ObjectRepository::GetCallExprArgumentObject(
    const clang::CallExpr* expr, size_t arg_index) const {
  auto iter = call_expr_args_objects_.find(std::make_pair(expr, arg_index));
  if (iter == call_expr_args_objects_.end()) {
    llvm::errs() << "Didn't find object for argument " << arg_index
                 << " of call:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find object for argument");
  }
  return iter->second;
}

const Object* ObjectRepository::GetCallExprRetObject(
    const clang::Expr* expr) const {
  auto iter = call_expr_ret_objects_.find(expr);
  if (iter == call_expr_ret_objects_.end()) {
    llvm::errs() << "Didn't find object for return value of call:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find object for return value");
  }
  return iter->second;
}

const FunctionLifetimes& ObjectRepository::GetCallExprVirtualLifetimes(
    const clang::Expr* expr) const {
  auto iter = call_expr_virtual_lifetimes_.find(expr);
  if (iter == call_expr_virtual_lifetimes_.end()) {
    llvm::errs() << "Didn't find object for return value of call:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find object for return value");
  }
  return iter->second;
}

const Object* ObjectRepository::GetCallExprThisPointer(
    const clang::CallExpr* expr) const {
  auto iter = call_expr_this_pointers_.find(expr);
  if (iter == call_expr_this_pointers_.end()) {
    llvm::errs() << "Didn't find `this` object for call:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find `this` object for call");
  }
  return iter->second;
}

const Object* ObjectRepository::GetCXXConstructExprArgumentObject(
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
  return iter->second;
}

const Object* ObjectRepository::GetCXXConstructExprThisPointer(
    const clang::CXXConstructExpr* expr) const {
  auto iter = call_expr_this_pointers_.find(expr);
  if (iter == call_expr_this_pointers_.end()) {
    llvm::errs() << "Didn't find `this` object for constructor:\n";
    expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find `this` object for constructor");
  }
  return iter->second;
}

const Object* ObjectRepository::GetResultObject(
    const clang::Expr* initializer_expr) const {
  assert(clang::isa<clang::CXXConstructExpr>(initializer_expr) ||
         clang::isa<clang::InitListExpr>(initializer_expr) ||
         clang::isa<clang::CallExpr>(initializer_expr));

  auto iter = result_objects_.find(initializer_expr);
  if (iter == result_objects_.end()) {
    llvm::errs() << "Didn't find result object for initializer:\n";
    initializer_expr->dump();
    llvm::errs() << "\n" << DebugString();
    llvm::report_fatal_error("Didn't find result object for initializer");
  }
  return iter->second;
}

const Object* ObjectRepository::GetFieldObject(
    const Object* struct_object, const clang::FieldDecl* field) const {
  std::optional<const Object*> field_object =
      GetFieldObjectInternal(struct_object, field);
  if (!field_object.has_value()) {
    llvm::errs() << "On an object of type "
                 << struct_object->Type().getAsString()
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
  for (const Object* object : struct_objects) {
    ret.Add(GetFieldObject(object, field));
  }
  return ret;
}

const Object* ObjectRepository::GetBaseClassObject(
    const Object* struct_object, const clang::Type* base) const {
  base = base->getCanonicalTypeInternal().getTypePtr();
  auto iter = base_object_map_.find(std::make_pair(struct_object, base));
  if (iter == base_object_map_.end()) {
    llvm::errs() << "On object " << struct_object->DebugString()
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
  for (const Object* object : struct_objects) {
    ret.Add(GetBaseClassObject(object, base));
  }
  return ret;
}

namespace {

llvm::SmallVector<std::string> GetFieldLifetimeArguments(
    const clang::FieldDecl* field) {
  // TODO(mboehme): Report errors as Clang diagnostics, not through
  // llvm::report_fatal_error().

  const clang::AnnotateAttr* member_lifetimes_attr = nullptr;
  for (auto annotate : field->specific_attrs<clang::AnnotateAttr>()) {
    if (annotate->getAnnotation() == "member_lifetimes") {
      if (member_lifetimes_attr) {
        llvm::report_fatal_error("repeated lifetime annotation");
      }
      member_lifetimes_attr = annotate;
    }
  }
  if (!member_lifetimes_attr) {
    return {};
  }

  llvm::SmallVector<std::string> ret;
  for (const auto& arg : member_lifetimes_attr->args()) {
    llvm::StringRef lifetime;
    if (llvm::Error err = EvaluateAsStringLiteral(arg, field->getASTContext())
                              .moveInto(lifetime)) {
      llvm::report_fatal_error(llvm::StringRef(toString(std::move(err))));
    }
    ret.push_back(lifetime.str());
  }

  return ret;
}

template <typename CallbackField, typename CallackBase>
void ForEachFieldAndBase(clang::QualType record_type,
                         const ObjectLifetimes& object_lifetimes,
                         const CallbackField& callback_field,
                         const CallackBase& callback_base) {
  assert(record_type->isRecordType());
  for (clang::FieldDecl* f :
       record_type->getAs<clang::RecordType>()->getOriginalDecl()->fields()) {
    ObjectLifetimes field_lifetimes = object_lifetimes.GetFieldOrBaseLifetimes(
        f->getType(), GetFieldLifetimeArguments(f));
    callback_field(field_lifetimes, f);
  }
  if (auto* cxxrecord = clang::dyn_cast<clang::CXXRecordDecl>(
          record_type->getAs<clang::RecordType>()->getOriginalDecl())) {
    for (const clang::CXXBaseSpecifier& base : cxxrecord->bases()) {
      clang::QualType base_type = base.getType();
      auto base_object_lifetimes = object_lifetimes.GetFieldOrBaseLifetimes(
          base_type, GetLifetimeParameters(base_type));
      callback_base(base_object_lifetimes, &*base_type.getCanonicalType());
      ForEachFieldAndBase(base.getType(), base_object_lifetimes, callback_field,
                          callback_base);
    }
  }
}

}  // namespace

class ObjectRepository::ObjectCreator {
 public:
  ObjectCreator(ObjectRepository& object_repository, PointsToMap& points_to_map)
      : object_repository_(object_repository), points_to_map_(points_to_map) {}

  const Object* CreateObjectsRecursively(
      const ObjectLifetimes& object_lifetimes) {
    if (auto it = object_cache_.find(object_lifetimes);
        it != object_cache_.end()) {
      return it->second;
    }

    const clang::QualType type = object_lifetimes.Type();
    const ValueLifetimes& value_lifetimes =
        object_lifetimes.GetValueLifetimes();

    std::optional<FunctionLifetimes> function_lifetimes;

    if (type->getAs<clang::FunctionType>()) {
      function_lifetimes = value_lifetimes.GetFuncLifetimes();
    }

    const Object* obj = object_repository_.ConstructObject(
        object_lifetimes.GetLifetime(), object_lifetimes.Type(),
        function_lifetimes);
    object_cache_[object_lifetimes] = obj;

    object_repository_.initial_object_lifetimes_[obj] = object_lifetimes;

    if (type->isIncompleteType()) {
      // Nothing we can do.
      return obj;
    }

    // Pointer type.
    if (!PointeeType(type).isNull()) {
      points_to_map_.ExtendPointerPointsToSet(
          obj,
          {CreateObjectsRecursively(value_lifetimes.GetPointeeLifetimes())});
      return obj;
    }

    // Record type.
    if (type->getAs<clang::RecordType>()) {
      ForEachFieldAndBase(
          type, object_lifetimes,
          [this, obj](const ObjectLifetimes& field_lifetimes,
                      const clang::FieldDecl* f) {
            const Object* field = CreateObjectsRecursively(field_lifetimes);
            object_repository_.field_object_map_[std::make_pair(obj, f)] =
                field;
          },
          [this, obj](const ObjectLifetimes& base_lifetimes,
                      const clang::Type* base_type) {
            const Object* base_obj = CreateObjectsRecursively(base_lifetimes);
            object_repository_
                .base_object_map_[std::make_pair(obj, base_type)] = base_obj;
          }

      );
    }

    return obj;
  }

 private:
  ObjectRepository& object_repository_;
  PointsToMap& points_to_map_;
  // We re-use the same Object for all the sub-objects with the same type and
  // lifetimes. This avoids infinite loops in the case of structs like lists.
  llvm::DenseMap<ObjectLifetimes, const Object*> object_cache_;
};

const Object* ObjectRepository::CreateObject(
    const ObjectLifetimes& object_lifetimes, PointsToMap& points_to_map) {
  ObjectCreator object_creator(*this, points_to_map);
  return object_creator.CreateObjectsRecursively(object_lifetimes);
}

const Object* ObjectRepository::CreateObject(clang::QualType type,
                                             Lifetime root_object_lifetime,
                                             LifetimeFactory lifetime_factory) {
  return CreateObject(
      ObjectLifetimes(root_object_lifetime,
                      ValueLifetimes::Create(type, lifetime_factory).get()),
      initial_points_to_map_);
}

template <typename... Args>
const Object* ObjectRepository::ConstructObject(Args&&... args) {
  return new (object_allocator_.Allocate()) Object(args...);
}

// Clones an object and its base classes and fields, if any.
const Object* ObjectRepository::CloneObject(const Object* object) {
  struct ObjectPair {
    const Object* orig_object;
    const Object* new_object;
  };
  auto clone = [this](const Object* obj) {
    auto new_obj = ConstructObject(obj->GetLifetime(), obj->Type(),
                                   obj->GetFuncLifetimes());
    initial_points_to_map_.SetPointerPointsToSet(
        new_obj, initial_points_to_map_.GetPointerPointsToSet(obj));
    return new_obj;
  };
  const Object* new_root = clone(object);
  initial_object_lifetimes_[new_root] = initial_object_lifetimes_[object];
  std::vector<ObjectPair> object_stack{{object, new_root}};
  while (!object_stack.empty()) {
    auto [orig_object, new_object] = object_stack.back();
    assert(orig_object->Type() == new_object->Type());
    object_stack.pop_back();
    auto record_type = orig_object->Type()->getAs<clang::RecordType>();
    if (!record_type) {
      continue;
    }

    // Base classes.
    if (auto* cxxrecord = clang::dyn_cast<clang::CXXRecordDecl>(
            record_type->getOriginalDecl())) {
      for (const clang::CXXBaseSpecifier& base : cxxrecord->bases()) {
        const Object* base_obj =
            GetBaseClassObject(orig_object, base.getType());
        const Object* new_base_obj = clone(base_obj);
        base_object_map_[std::make_pair(
            new_object, base.getType().getCanonicalType().getTypePtr())] =
            new_base_obj;
        object_stack.push_back(ObjectPair{base_obj, new_base_obj});
      }
    }

    // Fields.
    for (auto f : record_type->getOriginalDecl()->fields()) {
      const Object* field_obj = GetFieldObject(orig_object, f);
      const Object* new_field_obj = clone(field_obj);
      field_object_map_[std::make_pair(new_object, f)] = new_field_obj;
      object_stack.push_back(ObjectPair{field_obj, new_field_obj});
    }
  }
  return new_root;
}

std::optional<const Object*> ObjectRepository::GetFieldObjectInternal(
    const Object* struct_object, const clang::FieldDecl* field) const {
  auto iter = field_object_map_.find(std::make_pair(struct_object, field));
  if (iter != field_object_map_.end()) {
    return iter->second;
  }
  if (auto* cxxrecord =
          clang::dyn_cast<clang::CXXRecordDecl>(struct_object->Type()
                                                    ->getAs<clang::RecordType>()
                                                    ->getOriginalDecl())) {
    for (const clang::CXXBaseSpecifier& base : cxxrecord->bases()) {
      std::optional<const Object*> field_object = GetFieldObjectInternal(
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
