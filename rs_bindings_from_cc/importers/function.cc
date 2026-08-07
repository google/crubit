// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/function.h"

#include <cstdint>
#include <iterator>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "clang/Sema/Initialization.h"
#include "clang/Sema/Template.h"
#include "absl/algorithm/container.h"
#include "absl/container/btree_set.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "common/annotation_reader.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "lifetime_annotations/lifetime_error.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir.proto.h"
#include "rs_bindings_from_cc/recording_diagnostic_consumer.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclarationName.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/RecordLayout.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Basic/DiagnosticIDs.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Lex/Lexer.h"
#include "clang/Rewrite/Core/Rewriter.h"
#include "clang/Sema/Scope.h"
#include "clang/Sema/Sema.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/Path.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {
namespace {

// A collection of `FormattedError` values that enforces uniqueness.
struct Errors {
  // `btree_set` is used to ensure stable ordering.
  absl::btree_set<FormattedError> error_set;
  void Add(FormattedError error) {
    auto result = error_set.insert(std::move(error));
    CHECK(result.second) << "Duplicated error message";
  }
  void AddStatus(absl::Status status) {
    Add(FormattedError::FromStatus(status));
  }
};

SafetyAnnotation GetCrubitSafetyAnnotation(const clang::Decl& decl,
                                           Errors& errors) {
  absl::StatusOr<std::optional<AnnotateArgs>> maybe_args =
      GetAnnotateAttrArgs(decl, "crubit_override_unsafe");
  if (!maybe_args.ok()) {
    errors.AddStatus(std::move(maybe_args).status());
    return SafetyAnnotation::SAFETY_ANNOTATION_UNANNOTATED;
  }
  if (!maybe_args->has_value()) {
    return SafetyAnnotation::SAFETY_ANNOTATION_UNANNOTATED;
  }
  const AnnotateArgs& args = **maybe_args;
  if (args.size() != 1) {
    errors.AddStatus(absl::InvalidArgumentError(
        "`crubit_override_unsafe` annotation must have exactly one argument"));
    return SafetyAnnotation::SAFETY_ANNOTATION_UNANNOTATED;
  }
  absl::StatusOr<bool> is_unsafe =
      GetExprAsBool(*args[0], decl.getASTContext());
  if (!is_unsafe.ok()) {
    errors.AddStatus(std::move(is_unsafe).status());
    return SafetyAnnotation::SAFETY_ANNOTATION_UNANNOTATED;
  } else if (*is_unsafe) {
    return SafetyAnnotation::SAFETY_ANNOTATION_UNSAFE;
  } else {
    return SafetyAnnotation::SAFETY_ANNOTATION_DISABLE_UNSAFE;
  }
}

template <typename Attr>
void CollectUnsafeAttr(const clang::Decl& decl, Errors& errors, bool is_safe,
                       SafetyAnnotation& safety) {
  auto attrs = decl.specific_attrs<Attr>();
  if (attrs.empty()) {
    return;
  }
  if (is_safe) {
    errors.Add(
        FormattedError::Substitute("Function is annotated with both `$0` and "
                                   "`CRUBIT_OVERRIDE_UNSAFE(false)`",
                                   attrs.begin()->getSpelling()));
    return;
  }
  safety = SafetyAnnotation::SAFETY_ANNOTATION_UNSAFE;
}

SafetyAnnotation GetSafetyAnnotation(const clang::Decl& decl, Errors& errors) {
  SafetyAnnotation safety = GetCrubitSafetyAnnotation(decl, errors);
  bool is_safe = safety == SafetyAnnotation::SAFETY_ANNOTATION_DISABLE_UNSAFE;
  CollectUnsafeAttr<clang::UnsafeBufferUsageAttr>(decl, errors, is_safe,
                                                  safety);

  CollectUnsafeAttr<clang::RequiresCapabilityAttr>(decl, errors, is_safe,
                                                   safety);
  return safety;
}

// Applies the ref qualifier to the `this` pointer.
//
// Assume `f` is a method in `void f() && $a`. Converting the `this` parameter
// type of `f` will result in a pointer type, even though the method is rvalue
// ref qualified and has a lifetime. This function will update the `this`
// parameter type to be an rvalue reference instead.
void ApplyRefQualifierToThisPointer(CcType& this_param_type,
                                    clang::RefQualifierKind ref_qualifier_kind,
                                    bool assumed_lifetimes_enabled) {
  auto* pointer = std::get_if<CcType::PointerType>(&this_param_type.variant);
  // The CcType of `this` should always be a pointer.
  CHECK(pointer != nullptr);

  // Now we go back and fix the `this` parameter type to be a reference
  // if it was a rvalue ref qualified and had a lifetime.
  if ((assumed_lifetimes_enabled || pointer->lifetime.has_value()) &&
      ref_qualifier_kind == clang::RefQualifierKind::RQ_RValue) {
    // It was just a non null pointer, but because of the rvalue ref
    // qualification, it should be an rvalue reference.
    CHECK(pointer->kind == PointerTypeKind::NON_NULL);
    pointer->kind = PointerTypeKind::R_VALUE_REF;
  }
}

}  // namespace

static bool IsInStdNamespace(const clang::FunctionDecl* decl) {
  const clang::DeclContext* context = decl->getDeclContext();
  while (context) {
    if (context->isStdNamespace()) {
      return true;
    }
    context = context->getParent();
  }
  return false;
}

Identifier FunctionDeclImporter::GetTranslatedParamName(
    const clang::ParmVarDecl* param_decl) {
  int param_pos = param_decl->getFunctionScopeIndex();
  absl::StatusOr<TranslatedIdentifier> name =
      ictx_.GetTranslatedIdentifier(param_decl);
  if (!name.ok()) {
    return {Identifier(absl::StrCat("__param_", param_pos))};
  }
  if (auto* sttpt =
          param_decl->getType()->getAs<clang::SubstTemplateTypeParmType>();
      sttpt && sttpt->getReplacedParameter()->isParameterPack()) {
    // Avoid giving the same name to all parameters expanded from a pack.
    return {Identifier(
        absl::StrCat("__", name->rs_identifier().Ident(), "_", param_pos))};
  }
  return Identifier(std::string((*name).rs_identifier().Ident()));
}

namespace {

bool FunctionInStdOrSystemHeaderWithReservedName(
    const clang::FunctionDecl& function_decl) {
  bool is_in_system_hdr =
      function_decl.getASTContext().getSourceManager().isInSystemHeader(
          function_decl.getLocation());
  if (IsInStdNamespace(&function_decl) || is_in_system_hdr) {
    if (clang::IdentifierInfo* id = function_decl.getIdentifier()) {
      llvm::StringRef name = id->getName();
      bool underscore_underscore = name.starts_with("__");
      bool underscore_capital = name.starts_with("_") && name.size() > 1 &&
                                'A' <= name[1] && name[1] <= 'Z';
      if (underscore_underscore || underscore_capital) {
        return true;
      }
    }
  }
  return false;
}

bool FunctionNameIsIdentifier(clang::FunctionDecl& function_decl) {
  return function_decl.getDeclName().getNameKind() ==
         clang::DeclarationName::Identifier;
}

bool FunctionBodyIsDefinedInHeader(const clang::FunctionDecl* function_decl) {
  if (function_decl == nullptr) {
    return false;
  }
  const clang::FunctionDecl* def = nullptr;
  if (!function_decl->hasBody(def) || def == nullptr) {
    return false;
  }
  const clang::FunctionDecl* pattern = def->getTemplateInstantiationPattern();
  const clang::FunctionDecl* target_decl = def;
  if (pattern != nullptr) {
    const clang::FunctionDecl* pattern_def = nullptr;
    if (pattern->hasBody(pattern_def) && pattern_def != nullptr) {
      target_decl = pattern_def;
    } else {
      target_decl = pattern;
    }
  }

  clang::SourceManager& source_manager =
      function_decl->getASTContext().getSourceManager();
  clang::SourceLocation loc = target_decl->getLocation();
  if (loc.isInvalid()) {
    return false;
  }
  if (loc.isMacroID()) {
    loc = source_manager.getExpansionLoc(loc);
  }
  if (source_manager.isInMainFile(loc) ||
      source_manager.isWrittenInMainFile(loc) ||
      source_manager.isWrittenInCommandLineFile(loc) ||
      source_manager.isWrittenInScratchSpace(loc)) {
    return false;
  }
  if (source_manager.isInSystemHeader(loc)) {
    return true;
  }
  std::optional<llvm::StringRef> filename =
      source_manager.getNonBuiltinFilenameForID(source_manager.getFileID(loc));
  if (!filename.has_value()) {
    return false;
  }
  llvm::StringRef ext = llvm::sys::path::extension(*filename);
  static constexpr llvm::StringRef kHeaderExtensions[] = {
      ".h", ".H", ".hh", ".hpp", ".hxx", ".h++", ".inc", ".inl", ".def", ".tcc",
  };
  return absl::c_contains(kHeaderExtensions, ext);
}

struct PropertyFieldInfo {
  CcType type;
  int64_t offset;
  clang::QualType qual_type;
};

std::optional<PropertyFieldInfo> GetPropertyFieldInfo(ImportContext& ictx,
                                                      const clang::Expr* expr) {
  auto* member_expr = clang::dyn_cast_or_null<clang::MemberExpr>(expr);
  if (!member_expr) {
    return std::nullopt;
  }

  if (!member_expr->isImplicitAccess()) {
    const clang::Expr* base = member_expr->getBase()->IgnoreParenImpCasts();
    if (!clang::isa<clang::CXXThisExpr>(base)) {
      return std::nullopt;
    }
  }

  auto* field_decl =
      clang::dyn_cast_or_null<clang::FieldDecl>(member_expr->getMemberDecl());
  if (!field_decl || field_decl->isBitField()) {
    return std::nullopt;
  }

  auto* record_decl = field_decl->getParent();
  if (!record_decl || record_decl->isDependentContext() ||
      !record_decl->isCompleteDefinition()) {
    return std::nullopt;
  }
  CcType type = ictx.ConvertQualType(field_decl->getType(),
                                     /*lifetimes=*/nullptr, /*nullable=*/true,
                                     ictx.AreAssumedLifetimesEnabledForTarget(
                                         ictx.GetOwningTarget(record_decl)));

  const clang::ASTRecordLayout& layout =
      record_decl->getASTContext().getASTRecordLayout(record_decl);
  int64_t offset =
      static_cast<int64_t>(layout.getFieldOffset(field_decl->getFieldIndex()));

  return PropertyFieldInfo{
      .type = std::move(type),
      .offset = offset,
      .qual_type = field_decl->getType(),
  };
}

std::optional<ir_proto::MemberFuncSemantic> GetMemberFuncSemantic(
    ImportContext& ictx, const clang::FunctionDecl* function_decl) {
  auto* method_decl = clang::dyn_cast<clang::CXXMethodDecl>(function_decl);
  if (!method_decl || !method_decl->isInstance()) {
    return std::nullopt;
  }

  bool is_getter_candidate =
      method_decl->param_empty() && !method_decl->getReturnType()->isVoidType();
  bool is_setter_candidate = !method_decl->isConst() &&
                             method_decl->getNumParams() == 1 &&
                             method_decl->getReturnType()->isVoidType();
  if (!is_getter_candidate && !is_setter_candidate) {
    return std::nullopt;
  }

  const clang::FunctionDecl* def = nullptr;
  if (!method_decl->hasBody(def)) {
    return std::nullopt;
  }
  auto* method_def = clang::dyn_cast_or_null<clang::CXXMethodDecl>(def);
  if (!method_def || !method_def->isInstance()) {
    return std::nullopt;
  }

  // Extra checks against the definition.
  if (is_getter_candidate) {
    if (!method_def->param_empty() ||
        method_def->getReturnType()->isVoidType()) {
      return std::nullopt;
    }
  } else {
    if (method_def->isConst() || method_def->getNumParams() != 1 ||
        !method_def->getReturnType()->isVoidType()) {
      return std::nullopt;
    }
  }

  if (!FunctionBodyIsDefinedInHeader(method_def)) {
    return std::nullopt;
  }

  const clang::Stmt* body = method_def->getBody();
  auto* compound_stmt = clang::dyn_cast_or_null<clang::CompoundStmt>(body);
  if (!compound_stmt || compound_stmt->size() != 1) {
    return std::nullopt;
  }

  if (is_getter_candidate) {
    auto* return_stmt =
        clang::dyn_cast<clang::ReturnStmt>(compound_stmt->body_back());
    if (!return_stmt || !return_stmt->getRetValue()) {
      return std::nullopt;
    }

    const clang::Expr* ret_expr =
        return_stmt->getRetValue()->IgnoreParenImpCasts();
    std::optional<PropertyFieldInfo> field_info =
        GetPropertyFieldInfo(ictx, ret_expr);
    if (!field_info) {
      return std::nullopt;
    }
    // TODO(b/482092715): relax this restriction and allow conversions where
    // possible.
    if (method_def->getReturnType().getCanonicalType() !=
        field_info->qual_type.getCanonicalType()) {
      return std::nullopt;
    }
    ir_proto::MemberFuncSemantic semantic;
    field_info->type.WriteToProto(*semantic.mutable_getter()->mutable_type());
    semantic.mutable_getter()->set_offset(field_info->offset);
    return semantic;
  } else {
    const clang::Expr* expr =
        clang::dyn_cast<clang::Expr>(compound_stmt->body_back());
    if (!expr) {
      return std::nullopt;
    }
    if (auto* ewc = clang::dyn_cast<clang::ExprWithCleanups>(expr)) {
      expr = ewc->getSubExpr();
    }
    if (!expr) {
      return std::nullopt;
    }
    expr = expr->IgnoreParenImpCasts();

    const clang::Expr* lhs = nullptr;
    const clang::Expr* rhs = nullptr;
    if (auto* bin_op = clang::dyn_cast<clang::BinaryOperator>(expr)) {
      if (bin_op->getOpcode() == clang::BO_Assign) {
        lhs = bin_op->getLHS()->IgnoreParenImpCasts();
        rhs = bin_op->getRHS()->IgnoreParenImpCasts();
      }
    } else if (auto* op_call =
                   clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
      if (op_call->getOperator() == clang::OO_Equal &&
          op_call->getNumArgs() == 2) {
        lhs = op_call->getArg(0)->IgnoreParenImpCasts();
        rhs = op_call->getArg(1)->IgnoreParenImpCasts();
      }
    }
    if (!lhs || !rhs) {
      return std::nullopt;
    }

    auto* decl_ref = clang::dyn_cast<clang::DeclRefExpr>(rhs);
    if (!decl_ref) {
      return std::nullopt;
    }
    if (decl_ref->getDecl() != method_def->getParamDecl(0) &&
        decl_ref->getDecl() != method_decl->getParamDecl(0)) {
      return std::nullopt;
    }

    std::optional<PropertyFieldInfo> field_info =
        GetPropertyFieldInfo(ictx, lhs);
    if (!field_info) {
      return std::nullopt;
    }
    // TODO(b/482092715): relax this restriction and allow conversions where
    // possible.
    if (method_def->getParamDecl(0)->getType().getCanonicalType() !=
        field_info->qual_type.getCanonicalType()) {
      return std::nullopt;
    }
    ir_proto::MemberFuncSemantic semantic;
    field_info->type.WriteToProto(*semantic.mutable_setter()->mutable_type());
    semantic.mutable_setter()->set_offset(field_info->offset);
    return semantic;
  }
}

class MemberAccessRewriter
    : public clang::RecursiveASTVisitor<MemberAccessRewriter> {
 public:
  explicit MemberAccessRewriter(clang::Rewriter& rewriter)
      : rewriter_(rewriter) {}

  bool VisitMemberExpr(clang::MemberExpr* member_expr) {
    if (member_expr->isImplicitAccess()) {
      rewriter_.InsertText(member_expr->getMemberLoc(), "__this->");
    }
    return true;
  }

  bool VisitCXXThisExpr(clang::CXXThisExpr* this_expr) {
    if (!this_expr->isImplicit()) {
      rewriter_.ReplaceText(this_expr->getSourceRange(), "__this");
    }
    return true;
  }

 private:
  clang::Rewriter& rewriter_;
};

// Formats a C++ function declaration's signature and body for inline_cpp!
// token generation.
// For non-static member functions, rewrites implicit member accesses and
// explicit `this` expressions to access members through `__this->`.
std::optional<std::string> GetFunctionSourceText(
    const clang::ASTContext& ast_ctx, const clang::SourceManager& sm,
    const clang::FunctionDecl* function_decl, bool carcinize) {
  const clang::Stmt* body = function_decl->getBody();
  if (!carcinize || body == nullptr || function_decl->isImplicit()) {
    return std::nullopt;
  }

  auto* method_decl = clang::dyn_cast<clang::CXXMethodDecl>(function_decl);
  if (method_decl != nullptr && !method_decl->isStatic()) {
    clang::Rewriter rewriter(const_cast<clang::SourceManager&>(sm),
                             ast_ctx.getLangOpts());
    MemberAccessRewriter visitor(rewriter);
    visitor.TraverseStmt(const_cast<clang::Stmt*>(body));

    clang::CharSourceRange range = clang::CharSourceRange::getTokenRange(
        body->getBeginLoc(), body->getEndLoc());
    std::string rewritten_body = rewriter.getRewrittenText(range);
    if (!rewritten_body.empty()) {
      return rewritten_body;
    }
  }

  bool invalid = false;
  llvm::StringRef body_text =
      clang::Lexer::getSourceText(clang::CharSourceRange::getTokenRange(
                                      body->getBeginLoc(), body->getEndLoc()),
                                  sm, ast_ctx.getLangOpts(), &invalid);
  if (invalid) {
    return std::nullopt;
  }

  return body_text.str();
}

}  // namespace

std::unique_ptr<ir_proto::Item> FunctionDeclImporter::Import(
    clang::FunctionDecl* function_decl) {
  if (!ictx_.IsFromCurrentTarget(function_decl)) return nullptr;
  if (function_decl->isDeleted()) return nullptr;
  const bool only_import_types =
      ictx_.IsFeatureEnabledForCurrentTarget("types") &&
      !ictx_.IsFeatureEnabledForCurrentTarget("supported");
  if (!must_bind_ && only_import_types &&
      FunctionNameIsIdentifier(*function_decl)) {
    return nullptr;
  }
  if (FunctionInStdOrSystemHeaderWithReservedName(*function_decl)) {
    return ictx_.ImportUnsupportedItem(
        *function_decl, std::nullopt,
        {FormattedError::Static("Internal functions from the standard "
                                "library are not supported")},
        must_bind_);
  }
  // Method is private, we don't need to import it.
  if (auto* method_decl =
          clang::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    switch (method_decl->getAccess()) {
      case clang::AS_public:
        break;
      case clang::AS_protected:
      case clang::AS_private:
      case clang::AS_none:
        // No need for IR to include Func representing private methods.
        // TODO(b/475810473): Revisit this for protected methods.
        return nullptr;
    }
  }

  absl::StatusOr<TranslatedUnqualifiedIdentifier> translated_name =
      ictx_.GetTranslatedName(function_decl);
  if (!translated_name.ok()) {
    return ictx_.ImportUnsupportedItem(
        *function_decl, std::nullopt,
        {FormattedError::PrefixedStrCat("Function name is not supported",
                                        translated_name.status().message())},
        must_bind_);
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(function_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        *function_decl, std::nullopt,
        {FormattedError::FromStatus(std::move(enclosing_item_id).status())},
        must_bind_);
  }

  // Reports an unsupported function with the given error.
  //
  // This is preferred to invoking `ImportUnsupportedItem` directly because it
  // ensures that the path is set correctly. Note that this cannot be used above
  // because the enclosing item ID and translated name are not yet available.
  auto unsupported = [this, &translated_name = *translated_name,
                      &enclosing_item_id = *enclosing_item_id,
                      function_decl](FormattedError error) {
    return ictx_.ImportUnsupportedItem(
        *function_decl, translated_name.cc_identifier, enclosing_item_id,
        {std::move(error)}, must_bind_);
  };

  if (function_decl->isWeakImported()) {
    return unsupported(FormattedError::Static("Function is weakly imported"));
  }

  // We should only import methods of class template specializations
  // that can be instantiated: the template may spell out the method,
  // but it's not guaranteed to be instantiable for the template parameter(s);
  // importing an un-instantiable method causes Crubit to generate a thunk to
  // invoke this method, which triggers instantiation when compiling the
  // generated bindings, which fails the build.
  clang::FunctionDecl* template_decl_for_method =
      function_decl->getInstantiatedFromMemberFunction();
  if (template_decl_for_method) {
    if (!ictx_.invocation_.should_instantiate_template_from_path(
            ictx_.sema_.getSourceManager(),
            template_decl_for_method->getLocation())) {
      return unsupported(FormattedError::Static(
          "Function template instantiation forbidden by blocklist"));
    }
    // It turns out that some of the functions marked with
    // `__attribute__((exclude_from_explicit_instantiation))` can cause us to
    // generate invalid bindings. For example, std::forward_list::sort() is
    // marked with _LIBCPP_HIDE_FROM_ABI (which includes
    // exclude_from_explicit_instantiations); sort() does not compile for types
    // that don't work with __less<>(). Previously, we avoided instantiating
    // these entirely because of some alleged instability in clang (though we
    // still tried to generate bindings for them!).
    //
    // We can either instantiate these functions (as we do now) or possibly
    // allowlist them. We can't just unconditionally try to bind them because
    // we will generate bad code.
    //
    // Note that inline definitions (`void sort() { sort(__less<>()); }`)
    // are considered to be "defined" even if we don't try and instantiate
    // their dependencies.
    //
    // We attempt to instantiate as many transitive templates as we can.
    // The first time a template is instantiated, we'll get diagnostics from
    // Clang (for substitution failure, bad static_asserts, etc). We only get
    // these once. However, we can (and, as it turns out, we must) still crawl
    // the AST of already-instantiated templates to check for error conditions.
    //
    // We have also explored using typechecking functions in Sema directly to
    // detect bad instantiations (e.g., by building up a function definition
    // that looks exactly like a Crubit thunk and trying to typecheck that).
    // This does not work, presumably because Clang does not re-check for
    // deeply broken instantiations (like ones that have RecoveryExprs in them)
    // until some subsequent full AST traversal like lowering to LLVM.
    auto point_of_instantiation = function_decl->getPointOfInstantiation();
    // Point of instantiation is invalid if Crubit is eagerly
    // instantiating a method of a class template specialization.
    if (point_of_instantiation.isInvalid()) {
      point_of_instantiation = function_decl->getLocation();
    }
    crubit::RecordingDiagnosticConsumer diagnostic_recorder =
        crubit::RecordDiagnostics(ictx_.sema_.getDiagnostics(), [&] {
          // Generally, clang is able to instantiate templates like this even
          // after parsing completes. However, in rare cases it accesses
          // transient parsing state (Scope) which was already cleaned up.
          //
          // HACK: We need to create a fake TU scope to avoid a crash in
          // `clang::Sema::InstantiateFunctionDefinition` when it tries to
          // access the translation unit scope, which it incorrectly assumes
          // is always non-null. This should be fixed in clang.
          //
          // Specifically, the crash happens when Crubit instantiates a
          // class template with a defaulted copy constructor, introducing
          // lazily-injected builtins such as `memcpy` to be introduced to the
          // TU scope.
          //
          // See b/401857961 where this was observed and cl/265779405 where a
          // similar issue was fixed in CLIF.
          FakeTUScope fake_tu_scope(ictx_);
          ictx_.sema_.InstantiateFunctionDefinition(point_of_instantiation,
                                                    function_decl);
          // Check that any newly discovered dependencies can be instantiated.
          ictx_.sema_.PerformPendingInstantiations();
        });
    std::string diagnostics =
        diagnostic_recorder.ConcatenatedDiagnostics("Diagnostics emitted:\n");
    if (diagnostic_recorder.getNumErrors() != 0) {
      // Clang considers the function decl valid even fatal diagnostics is
      // emitted during instantiation. However, such diagnostics would fail
      // compilation of generated bindings, so it's invalid as far as Crubit
      // is concerned, thus set it as invalid here.
      function_decl->setInvalidDecl();
      return unsupported(FormattedError::PrefixedStrCat(
          "Failed to instantiate the function/method template", diagnostics));
    } else if (auto invalid_decl_name =
                   GetInvalidCallTarget(ictx_, function_decl);
               !invalid_decl_name.empty()) {
      // TODO(zarko): This error message doesn't appear in the output code,
      return unsupported(FormattedError::PrefixedStrCat(
          "Instantiating this template relies on an invalid decl",
          invalid_decl_name));
    }
  }
  if (function_decl->isInvalidDecl()) {
    return unsupported(
        FormattedError::Static("Function declaration is considered invalid"));
  }
  // See DefineDefaultedFunction in SemaDeclCXX.cpp.
  // TODO(zarko): This is intentionally very narrow in scope (just for
  // copy assignments) right now. See b/436870965.
  if (auto defaulted_kind = ictx_.sema_.getDefaultedFunctionKind(function_decl);
      defaulted_kind.isSpecialMember()) {
    // TODO(zarko): Possibly eliminate a redundant check here (have we done this
    // already if function_decl is a function template that is also defaulted?)
    if (auto target = GetInvalidCallTarget(ictx_, function_decl);
        !target.empty()) {
      return unsupported(FormattedError::PrefixedStrCat(
          "Defaulted function relies on an invalid decl", target));
    }
  }

  bool assumed_lifetimes_enabled = ictx_.AreAssumedLifetimesEnabledForTarget(
      ictx_.GetOwningTarget(function_decl));
  clang::tidy::lifetimes::LifetimeSymbolTable lifetime_symbol_table;
  std::optional<clang::tidy::lifetimes::FunctionLifetimes> lifetimes;
  std::vector<std::string> lifetime_inputs;
  Errors errors;

  if (assumed_lifetimes_enabled) {
    auto lifetime_inputs_or_err =
        CollectLifetimeInputs(ictx_.sema_.getASTContext(), function_decl);
    if (!lifetime_inputs_or_err.ok()) {
      errors.Add(FormattedError::FromStatus(
          std::move(lifetime_inputs_or_err).status()));
    } else {
      lifetime_inputs.reserve(lifetime_inputs_or_err->size());
      absl::c_transform(*lifetime_inputs_or_err,
                        std::back_inserter(lifetime_inputs),
                        [](absl::string_view lifetime_view) {
                          return std::string(lifetime_view);
                        });
    }
  } else {
    llvm::Expected<clang::tidy::lifetimes::FunctionLifetimes> lifetimes_or_err =
        clang::tidy::lifetimes::GetLifetimeAnnotations(
            function_decl, *ictx_.invocation_.lifetime_context_,
            &lifetime_symbol_table);
    if (lifetimes_or_err) {
      lifetimes = std::move(*lifetimes_or_err);
    } else {
      using clang::tidy::lifetimes::LifetimeError;
      llvm::Error remaining_err = llvm::handleErrors(
          lifetimes_or_err.takeError(),
          [](std::unique_ptr<LifetimeError> lifetime_err) -> llvm::Error {
            switch (lifetime_err->type()) {
              case LifetimeError::Type::ElisionNotEnabled:
              case LifetimeError::Type::CannotElideOutputLifetimes:
                // If elision is not enabled or output lifetimes cannot be
                // elided, we want to import the function with raw lifetime-less
                // pointers. Just return success here; this will leave the
                // `lifetimes` optional empty, and we will then handle this
                // accordingly below.
                return llvm::Error::success();
                break;
              default:
                return llvm::Error(std::move(lifetime_err));
                break;
            }
          });
      if (remaining_err) {
        return unsupported(FormattedError::PrefixedStrCat(
            "Unable to get lifetime annotations",
            llvm::toString(std::move(remaining_err))));
      }
    }
  }

  std::vector<ir_proto::FuncParam> params;
  if (auto* method_decl =
          clang::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    if (!ictx_.HasBeenAlreadySuccessfullyImported(method_decl->getParent())) {
      return unsupported(FormattedError::Static("Couldn't import the parent"));
    }

    // non-static member functions receive an implicit `this` parameter.
    if (method_decl->isInstance()) {
      const clang::tidy::lifetimes::ValueLifetimes* this_lifetimes = nullptr;
      if (lifetimes) {
        this_lifetimes = &lifetimes->GetThisLifetimes();
      }
      CcType this_param_type =
          ictx_.ConvertQualType(method_decl->getThisType(), this_lifetimes,
                                /*nullable=*/false, assumed_lifetimes_enabled);
      if (assumed_lifetimes_enabled) {
        if (auto qual_type = method_decl->getType(); !qual_type.isNull()) {
          // Since getThisType desugars `this` (among other nontrivial
          // transformations), we need to post-hoc crawl through annotations
          // on the CXXMethodDecl to collect lifetimes. For example, for a
          // function like `int* $b f() && $a [[clang::lifetimebound]]`, we'll
          // see a type like:
          //
          // (AttributedType (AttributedType (FunctionProtoType
          //     (AttributedType (PointerType ...)))))
          //
          // where the first two AttributedTypes are meant to bind to `this`.
          auto this_lifetime_views = CollectExplicitLifetimes(
              ictx_.sema_.getASTContext(), *qual_type.getTypePtr());
          if (!this_lifetime_views.ok()) {
            errors.Add(FormattedError::PrefixedStrCat(
                "Can't collect lifetimes for `this`",
                this_lifetime_views.status().message()));
          } else if (!this_lifetime_views->empty() &&
                     !this_param_type.explicit_lifetimes.empty()) {
            errors.Add(FormattedError::PrefixedStrCat(
                "Extra explicit lifetimes on `this`",
                absl::StrJoin(*this_lifetime_views, ", ")));
          } else {
            this_param_type.explicit_lifetimes.reserve(
                this_lifetime_views->size());
            absl::c_transform(
                *this_lifetime_views,
                std::back_inserter(this_param_type.explicit_lifetimes),
                [](absl::string_view lifetime_view) {
                  return std::string(lifetime_view);
                });
          }
        }
      }
      ApplyRefQualifierToThisPointer(this_param_type,
                                     method_decl->getRefQualifier(),
                                     assumed_lifetimes_enabled);
      ir_proto::FuncParam this_param;
      this_param_type.WriteToProto(*this_param.mutable_type());
      this_param.mutable_identifier()->set_identifier("__this");
      // TODO(b/319524852): catch `[[clang::lifetimebound]]` on `this`.
      if (assumed_lifetimes_enabled && !method_decl->getType().isNull()) {
        auto clang_annotations =
            CollectClangLifetimeAnnotationsForMemberFunctionType(
                ictx_.sema_.getASTContext(),
                *method_decl->getType().getTypePtr());
        if (!clang_annotations.ok()) {
          errors.Add(FormattedError::PrefixedStrCat(
              "Can't collect clang lifetime annotations for `this`",
              clang_annotations.status().message()));
        } else {
          if (clang_annotations->lifetimebound) {
            this_param.set_clang_lifetimebound(true);
          }
          for (const auto& cap : clang_annotations->lifetime_capture_by) {
            this_param.add_clang_lifetime_capture_by(cap);
          }
        }
      }
      params.push_back(std::move(this_param));
    }
  }

  if (lifetimes) {
    CHECK(lifetimes->IsValidForDecl(function_decl));
  }

  for (unsigned i = 0; i < function_decl->getNumParams(); ++i) {
    const clang::ParmVarDecl* param = function_decl->getParamDecl(i);
    const clang::tidy::lifetimes::ValueLifetimes* param_lifetimes = nullptr;
    if (lifetimes) {
      param_lifetimes = &lifetimes->GetParamLifetimes(i);
    }
    CcType param_type =
        ictx_.ConvertQualType(param->getType(), param_lifetimes,
                              /*nullable=*/true, assumed_lifetimes_enabled);

    std::optional<Identifier> param_name = GetTranslatedParamName(param);
    CHECK(param_name.has_value());  // No known failure cases.

    absl::StatusOr<std::optional<std::string>> unknown_attr =
        CollectUnknownAttrs(
            *param, assumed_lifetimes_enabled
                        ? IsClangLifetimeAnnotation
                        : [](const clang::Attr& attr) { return false; });
    if (!unknown_attr.ok()) {
      errors.Add(FormattedError::FromStatus(std::move(unknown_attr).status()));
      continue;
    }

    ir_proto::FuncParam proto_param;
    param_type.WriteToProto(*proto_param.mutable_type());
    proto_param.mutable_identifier()->set_identifier(param_name->Ident());
    if (unknown_attr->has_value()) {
      proto_param.set_unknown_attr(std::move(**unknown_attr));
    }

    if (assumed_lifetimes_enabled) {
      auto lifetimebound = param->specific_attrs<clang::LifetimeBoundAttr>();
      if (!lifetimebound.empty()) {
        proto_param.set_clang_lifetimebound(true);
      }
      auto lifetime_capture_by =
          param->specific_attrs<clang::LifetimeCaptureByAttr>();
      if (!lifetime_capture_by.empty()) {
        for (const clang::LifetimeCaptureByAttr* attr : lifetime_capture_by) {
          for (const auto& p : attr->params()) {
            proto_param.add_clang_lifetime_capture_by(p);
          }
        }
      }
    }
    params.push_back(std::move(proto_param));
  }

  bool undeduced_return_type =
      function_decl->getReturnType()->isUndeducedType();
  if (undeduced_return_type) {
    // Use a custom diagnoser as the `DeduceReturnType` call may fail, which
    // is OK if this is a method of a class template, since Crubit
    // instantiates the members of the class templates eagerly.
    crubit::RecordingDiagnosticConsumer diagnostic_recorder =
        crubit::RecordDiagnostics(ictx_.sema_.getDiagnostics(), [&] {
          undeduced_return_type = ictx_.sema_.DeduceReturnType(
              function_decl, function_decl->getLocation());
        });
    if (undeduced_return_type) {
      errors.Add(FormattedError::PrefixedStrCat(
          "Couldn't deduce the return type",
          diagnostic_recorder.ConcatenatedDiagnostics(
              "Diagnostics emitted:\n")));
    }
  }
  absl::StatusOr<CcType> return_type;
  if (!undeduced_return_type) {
    const clang::tidy::lifetimes::ValueLifetimes* return_lifetimes = nullptr;
    if (lifetimes) {
      return_lifetimes = &lifetimes->GetReturnLifetimes();
    }
    return_type =
        ictx_.ConvertQualType(function_decl->getReturnType(), return_lifetimes,
                              /*nullable=*/true, assumed_lifetimes_enabled);
    if (!return_type.ok()) {
      errors.Add(FormattedError::PrefixedStrCat(
          "Return type is not supported", return_type.status().message()));
    }
  }

  llvm::DenseSet<clang::tidy::lifetimes::Lifetime> all_free_lifetimes;
  if (lifetimes) {
    all_free_lifetimes = lifetimes->AllFreeLifetimes();
  }

  std::vector<ir_proto::LifetimeName> lifetime_params;
  lifetime_params.reserve(all_free_lifetimes.size());
  for (clang::tidy::lifetimes::Lifetime lifetime : all_free_lifetimes) {
    std::optional<llvm::StringRef> name =
        lifetime_symbol_table.LookupLifetime(lifetime);
    CHECK(name.has_value());
    ir_proto::LifetimeName l_name;
    l_name.set_name(name->str());
    l_name.set_id(lifetime.Id());
    lifetime_params.push_back(std::move(l_name));
  }
  llvm::sort(lifetime_params, [](const ir_proto::LifetimeName& l1,
                                 const ir_proto::LifetimeName& l2) {
    return l1.name() < l2.name();
  });

  bool is_inline = false;
  bool is_defined = false;
  for (auto* def : function_decl->redecls()) {
    if (def->isInlined()) is_inline = true;
    if (def->isThisDeclarationADefinition()) is_defined = true;
  }
  if (!is_defined) {
    // Template members may not be defined until instantiation.
    if (auto* pat = function_decl->getTemplateInstantiationPattern()) {
      if (pat->isThisDeclarationADefinition()) {
        is_defined = true;
      }
    }
  }
  // It is valid to declare an inline function but not define it, as long as it
  // is not odr-used. Our thunk can't call it, so it is not callable from Rust.
  if (is_inline && !is_defined) {
    errors.Add(FormattedError::Static("Inline function is not defined"));
  }

  std::optional<ir_proto::InstanceMethodMetadata> instance_metadata;
  if (auto* method_decl =
          clang::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    if (method_decl->isInstance()) {
      ir_proto::InstanceMethodMetadata meta;
      switch (method_decl->getRefQualifier()) {
        case clang::RQ_LValue:
          meta.set_reference(ir_proto::InstanceMethodMetadata::L_VALUE);
          break;
        case clang::RQ_RValue:
          meta.set_reference(ir_proto::InstanceMethodMetadata::R_VALUE);
          break;
        case clang::RQ_None:
          meta.set_reference(ir_proto::InstanceMethodMetadata::UNQUALIFIED);
          break;
      }
      meta.set_is_const(method_decl->isConst());
      meta.set_is_virtual(method_decl->isVirtual());
      instance_metadata = std::move(meta);
    }
  }

  if (!errors.error_set.empty()) {
    return ictx_.ImportUnsupportedItem(
        *function_decl, translated_name->cc_identifier, *enclosing_item_id,
        std::vector(errors.error_set.begin(), errors.error_set.end()),
        must_bind_);
  }

  bool has_c_calling_convention =
      function_decl->getType()->getAs<clang::FunctionType>()->getCallConv() ==
      clang::CC_C;
  bool is_member_or_descendant_of_class_template =
      IsFullClassTemplateSpecializationOrChild(function_decl);

  SafetyAnnotation safety_annotation =
      GetSafetyAnnotation(*function_decl, errors);

  std::optional<std::string> doc_comment = ictx_.GetComment(function_decl);
  if (!doc_comment.has_value() && is_member_or_descendant_of_class_template) {
    // Despite `is_member_or_descendant_of_class_template` check above, we are
    // not guaranteed that a `func_pattern` exists below.  For example, it may
    // be missing when `function_decl` is an implicitly defined constructor of
    // a class template -- such decls are generated, not instantiated.
    if (clang::FunctionDecl* func_pattern =
            function_decl->getTemplateInstantiationPattern()) {
      doc_comment = ictx_.GetComment(func_pattern);
    }
  }

  std::optional<std::string> nodiscard;
  std::optional<std::string> deprecated;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*function_decl, [&](const clang::Attr& attr) {
        if (auto* unused_attr =
                clang::dyn_cast<clang::WarnUnusedResultAttr>(&attr)) {
          nodiscard.emplace(unused_attr->getMessage());
          return true;
        } else if (auto* deprecated_attr =
                       clang::dyn_cast<clang::DeprecatedAttr>(&attr)) {
          deprecated.emplace(deprecated_attr->getMessage());
          return true;
        } else if (clang::isa<clang::NoReturnAttr>(attr)) {
          return true;  // we call isNoReturn below, instead
        } else if (clang::isa<clang::UnsafeBufferUsageAttr>(attr) ||
                   clang::isa<clang::RequiresCapabilityAttr>(attr)) {
          return true;  // Handled in `GetSafetyAnnotation()`
        } else if (clang::isa<clang::AsmLabelAttr>(attr) ||
                   clang::isa<clang::ConstAttr>(attr) ||
                   clang::isa<clang::FinalAttr>(attr) ||
                   clang::isa<clang::ExcludeFromExplicitInstantiationAttr>(
                       attr) ||
                   clang::isa<clang::NoThrowAttr>(attr) ||
                   clang::isa<clang::OverrideAttr>(attr) ||
                   clang::isa<clang::PureAttr>(attr) ||
                   clang::isa<clang::ReinitializesAttr>(attr) ||
                   clang::isa<clang::UnusedAttr>(attr) ||
                   clang::isa<clang::AlwaysInlineAttr>(attr) ||
                   clang::isa<clang::AssertCapabilityAttr>(attr) ||
                   clang::isa<clang::AcquireCapabilityAttr>(attr) ||
                   clang::isa<clang::TryAcquireCapabilityAttr>(attr) ||
                   clang::isa<clang::ReleaseCapabilityAttr>(attr) ||
                   clang::isa<clang::NoThreadSafetyAnalysisAttr>(attr) ||
                   clang::isa<clang::LockReturnedAttr>(attr) ||
                   clang::isa<clang::AbiTagAttr>(attr) ||
                   clang::isa<clang::LocksExcludedAttr>(attr)) {
          // These attributes don't affect Rust.
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItem(
        *function_decl, translated_name->cc_identifier, *enclosing_item_id,
        {FormattedError::FromStatus(std::move(unknown_attr).status())},
        must_bind_);
  }

  // Silence ClangTidy, checked above: calling `errors.Add` if
  // `!return_type.ok()` and returning early if `!errors.empty()`.
  CHECK_OK(return_type);

  // We should unify FunctionDecl* generated from friend declarations with the
  // correct enclosing record.
  std::optional<ItemId> adl_enclosing_record;
  if (function_decl->getFriendObjectKind() != clang::Decl::FOK_None) {
    if (auto* enclosing_record = clang::dyn_cast_or_null<clang::CXXRecordDecl>(
            function_decl->getLexicalDeclContext())) {
      adl_enclosing_record = ictx_.GenerateItemId(enclosing_record);
    }
  }

  std::optional<std::string> source_text =
      GetFunctionSourceText(ictx_.ctx_, ictx_.ctx_.getSourceManager(),
                            function_decl, ictx_.invocation_.is_carcinize());

  auto name_info = function_decl->getNameInfo();
  auto item = std::make_unique<ir_proto::Item>();
  auto* func = item->mutable_func();
  WriteToProto(translated_name->cc_identifier, *func->mutable_cc_name());
  WriteToProto(translated_name->rs_identifier(), *func->mutable_rs_name());
  func->set_unique_name(ictx_.GetUniqueName(*function_decl));
  func->set_owning_target(ictx_.GetOwningTarget(function_decl).value());
  if (doc_comment.has_value()) {
    func->set_doc_comment(*doc_comment);
  }
  func->set_mangled_name(ictx_.GetMangledName(function_decl));
  return_type->WriteToProto(*func->mutable_return_type());
  for (auto& param : params) {
    *func->add_params() = std::move(param);
  }
  for (auto& l_param : lifetime_params) {
    *func->add_lifetime_params() = std::move(l_param);
  }
  func->set_is_inline(is_inline);
  if (instance_metadata.has_value()) {
    *func->mutable_instance_method_metadata() = std::move(*instance_metadata);
  }
  func->set_is_extern_c(function_decl->isExternC());
  func->set_is_noreturn(function_decl->isNoReturn());
  func->set_is_variadic(function_decl->isVariadic());
  func->set_is_consteval(function_decl->isConsteval());
  if (nodiscard.has_value()) {
    func->set_nodiscard(std::move(*nodiscard));
  }
  if (deprecated.has_value()) {
    func->set_deprecated(std::move(*deprecated));
  }
  if (unknown_attr->has_value()) {
    func->set_unknown_attr(std::move(**unknown_attr));
  }
  func->set_has_c_calling_convention(has_c_calling_convention);
  func->set_is_member_or_descendant_of_class_template(
      is_member_or_descendant_of_class_template);
  func->set_safety_annotation(safety_annotation);
  func->set_source_loc(
      ictx_.ConvertSourceLocation(function_decl->getBeginLoc(), &name_info));
  func->set_id(ictx_.GenerateItemId(function_decl).value());
  if (enclosing_item_id->has_value()) {
    func->set_enclosing_item_id((*enclosing_item_id)->value());
  }
  if (adl_enclosing_record.has_value()) {
    func->set_adl_enclosing_record(adl_enclosing_record->value());
  }
  for (const auto& lifetime_input : lifetime_inputs) {
    func->add_lifetime_inputs(lifetime_input);
  }
  if (source_text.has_value()) {
    func->set_inline_cpp_source_text(*source_text);
  }
  auto semantic = GetMemberFuncSemantic(ictx_, function_decl);
  if (semantic.has_value()) {
    *func->mutable_semantic() = std::move(*semantic);
  }
  return item;
}

}  // namespace crubit
