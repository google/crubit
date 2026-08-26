// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_util.h"

#include <algorithm>
#include <cstddef>
#include <optional>
#include <string>
#include <vector>

#include "absl/container/flat_hash_set.h"
#include "absl/functional/function_ref.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/annotation_reader.h"
#include "common/status_macros.h"
#include "common/string_view_conversion.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/recording_diagnostic_consumer.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/Type.h"
#include "clang/Basic/AttrKinds.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/Scope.h"
#include "clang/Sema/Sema.h"
#include "llvm/Support/Casting.h"

namespace crubit {

bool IsFullClassTemplateSpecializationOrChild(const clang::Decl* decl) {
  if (clang::isa<clang::ClassTemplatePartialSpecializationDecl>(decl)) {
    return false;
  }
  if (clang::isa<clang::ClassTemplateSpecializationDecl>(decl)) {
    return true;
  }

  if (const auto* decl_context = decl->getDeclContext()) {
    return IsFullClassTemplateSpecializationOrChild(
        clang::dyn_cast<clang::Decl>(decl_context));
  }

  return false;
}

bool IsClangLifetimeAnnotation(const clang::Attr& attr) {
  return clang::isa<clang::LifetimeBoundAttr>(attr) ||
         clang::isa<clang::LifetimeCaptureByAttr>(attr);
}

bool IsClangCoroAnnotation(const clang::Attr& attr) {
  return clang::isa<clang::CoroReturnTypeAttr>(attr) ||
         clang::isa<clang::CoroWrapperAttr>(attr) ||
         clang::isa<clang::CoroLifetimeBoundAttr>(attr) ||
         clang::isa<clang::CoroDisableLifetimeBoundAttr>(attr) ||
         clang::isa<clang::CoroAwaitElidableAttr>(attr) ||
         clang::isa<clang::CoroAwaitElidableArgumentAttr>(attr) ||
         clang::isa<clang::CoroOnlyDestroyWhenCompleteAttr>(attr);
}

absl::StatusOr<std::optional<std::string>> CollectUnknownAttrs(
    const clang::Decl& decl,
    absl::FunctionRef<bool(const clang::Attr&)> is_known) {
  std::optional<std::string> unknown_attr;
  if (!decl.hasAttrs()) {
    // Surprisingly, getAttrs() does not return an empty vec if there are no
    // attrs, it crashes.
    return unknown_attr;
  }

  absl::flat_hash_set<absl::string_view> ignored_attr_names;
  {
    CRUBIT_ASSIGN_OR_RETURN(
        std::optional<AnnotateArgs> args,
        GetAnnotateAttrArgs(decl, "crubit_unsafe_ignore_attr"));
    if (args.has_value()) {
      clang::ASTContext& ast_context = decl.getASTContext();
      for (const clang::Expr* arg : *args) {
        CRUBIT_ASSIGN_OR_RETURN(absl::string_view name,
                                GetExprAsStringLiteral(*arg, ast_context));
        ignored_attr_names.insert(name);
      }
    }
  }

  for (clang::Attr* attr : decl.getAttrs()) {
    if (is_known(*attr)) {
      continue;
    }
    if (IsClangCoroAnnotation(*attr)) {
      continue;
    }
    // Regardless of the callback, always ignore annotate attributes.
    if (clang::isa<clang::AnnotateAttr>(attr) ||
        clang::isa<clang::AnnotateTypeAttr>(attr)) {
      continue;
    }
    // The available attribute is handled centrally by importer.cc,
    // by checking Decl::isUnavailable.
    if (clang::isa<clang::UnavailableAttr>(attr)) {
      continue;
    }
    // Ignore attributes we have been instructed to ignore.
    std::string name_buf;
    absl::string_view name;
    if (attr->getAttrName()) {
      name_buf = attr->getNormalizedFullName();
      name = name_buf;
    } else {
      name = attr->getSpelling();
    }
    if (ignored_attr_names.contains(name)) {
      continue;
    }

    if (unknown_attr.has_value()) {
      absl::StrAppend(&*unknown_attr, ", ");
    } else {
      unknown_attr.emplace("");
    }
    absl::StrAppend(&*unknown_attr, name);
  }
  return unknown_attr;
}

absl::string_view DebugAttrName(clang::attr::Kind attr_kind) {
  // TODO(jeanpierreda): Give some more human-readable name, e.g. using
  // ParsedAttrInfo::getAllBuiltin.  Unfortunately, we don't have a TypeLoc,
  // so we only have access to a Kind, which doesn't specify how it is spelled.
  //
  // For now, we use the symbol name, and prefix it with `clang::attr` to make
  // it obvious it's an internal symbol and not something the user typed.
  switch (attr_kind) {
    // (Yes, the X-macro is really the only way to do it. Party like it's 1969!)
#define ATTR(X)        \
  case clang::attr::X: \
    return "clang::attr::Kind::" #X;
#include "clang/Basic/AttrList.inc"
#undef ATTR
  }
}

std::optional<std::string> CollectUnknownTypeAttrs(
    const clang::Type& t, absl::FunctionRef<bool(clang::attr::Kind)> is_known) {
  std::optional<std::string> unknown_attr;
  const clang::Type* type = &t;
  while (const auto* attributed_type = type->getAs<clang::AttributedType>()) {
    clang::attr::Kind attr_kind = attributed_type->getAttrKind();
    if (!is_known(attr_kind)) {
      if (unknown_attr.has_value()) {
        absl::StrAppend(&*unknown_attr, ", ");
      } else {
        unknown_attr.emplace("");
      }
      absl::StrAppend(&*unknown_attr, DebugAttrName(attr_kind));
    }
    type = attributed_type->getEquivalentType().getTypePtr();
  }
  return unknown_attr;
}

absl::StatusOr<CallingConv> ConvertCcCallConvToSupportedCallingConv(
    clang::CallingConv cc_call_conv) {
  switch (cc_call_conv) {
    case clang::CC_C:  // __attribute__((cdecl))
      return CallingConv::C_DECL;
    case clang::CC_X86FastCall:  // __attribute__((fastcall))
      return CallingConv::FAST_CALL;
    case clang::CC_X86VectorCall:  // __attribute__((vectorcall))
      return CallingConv::VECTOR_CALL;
    case clang::CC_X86ThisCall:  // __attribute__((thiscall))
      return CallingConv::THIS_CALL;
    case clang::CC_X86StdCall:  // __attribute__((stdcall))
      return CallingConv::STD_CALL;
    case clang::CC_Win64:  // __attribute__((ms_abi))
      return CallingConv::MS_ABI;
    case clang::CC_AAPCS:      // __attribute__((pcs("aapcs")))
    case clang::CC_AAPCS_VFP:  // __attribute__((pcs("aapcs-vfp")))
      // TODO(lukasza): Should both map to "aapcs"?
      break;
    case clang::CC_X86_64SysV:  // __attribute__((sysv_abi))
      // TODO(lukasza): Maybe this is "sysv64"?
      break;
    case clang::CC_X86Pascal:     // __attribute__((pascal))
    case clang::CC_X86RegCall:    // __attribute__((regcall))
    case clang::CC_IntelOclBicc:  // __attribute__((intel_ocl_bicc))
    case clang::CC_SpirFunction:  // default for OpenCL functions on SPIR target
    case clang::CC_DeviceKernel:  // __attribute__((device_kernel))
    case clang::CC_Swift:         // __attribute__((swiftcall))
    case clang::CC_SwiftAsync:    // __attribute__((swiftasynccall))
    case clang::CC_PreserveMost:  // __attribute__((preserve_most))
    case clang::CC_PreserveAll:   // __attribute__((preserve_all))
    case clang::CC_AArch64VectorCall:  // __attribute__((aarch64_vector_pcs))
      // TODO(hlopko): Uncomment once we integrate the upstream change that
      // introduced it:
      // case clang::CC_AArch64SVEPCS: __attribute__((aarch64_sve_pcs))

      // These don't seem to have any Rust equivalents.
      break;
    default:
      break;
  }
  return absl::UnimplementedError(
      absl::StrCat("Unsupported calling convention: ",
                   StringViewFromStringRef(
                       clang::FunctionType::getNameForCallConv(cc_call_conv))));
}

// Attempts to evaluate and return the value of `expr` as a string literal under
// `ast_context`. The returned `string_view` is owned by the `ast_context`.
absl::StatusOr<absl::string_view> EvaluateAsStringLiteral(
    const clang::ASTContext& ast_context, const clang::Expr* expr) {
  clang::Expr::EvalResult eval_result;
  if (!expr->EvaluateAsConstantExpr(eval_result, ast_context) ||
      !eval_result.Val.isLValue()) {
    return absl::InvalidArgumentError("expression is not a string literal");
  }

  const auto* eval_result_expr =
      eval_result.Val.getLValueBase().dyn_cast<const clang::Expr*>();
  if (!eval_result_expr) {
    return absl::InvalidArgumentError("expression is not a string literal");
  }

  const auto* strlit = clang::dyn_cast<clang::StringLiteral>(eval_result_expr);
  if (!strlit) {
    return absl::InvalidArgumentError("expression is not a string literal");
  }

  return strlit->getString();
}

absl::StatusOr<ClangLifetimeAnnotations>
CollectClangLifetimeAnnotationsForMemberFunctionType(
    const clang::ASTContext& ast_context, const clang::Type& t) {
  ClangLifetimeAnnotations annotations;
  const clang::Type* type = &t;
  while (const auto* attributed_type = type->getAs<clang::AttributedType>()) {
    if (auto lifetimebound_attr =
            clang::dyn_cast_or_null<clang::LifetimeBoundAttr>(
                attributed_type->getAttr());
        lifetimebound_attr != nullptr) {
      annotations.lifetimebound = true;
    } else if (auto lifetime_capture_by_attr =
                   clang::dyn_cast_or_null<clang::LifetimeCaptureByAttr>(
                       attributed_type->getAttr());
               lifetime_capture_by_attr != nullptr) {
      for (auto& param : lifetime_capture_by_attr->params()) {
        annotations.lifetime_capture_by.push_back(param);
      }
    }
    type = attributed_type->getEquivalentType().getTypePtr();
    if (type == nullptr) break;
  }
  return annotations;
}

absl::StatusOr<std::vector<absl::string_view>> CollectLifetimeInputs(
    const clang::ASTContext& ast_context, const clang::Decl* decl) {
  // TODO(b/454627672): How do we reconcile multiple param binding lists
  // (possible with forward declarations)?
  std::vector<absl::string_view> lifetimes;
  // In contrast with type annotations, here the syntax tree looks like
  // (decl [($a, $b), ($c, $d)])
  for (const auto* attr : decl->specific_attrs<clang::AnnotateAttr>()) {
    if (attr == nullptr || attr->getAnnotation() != "lifetime_params") {
      continue;
    }
    for (const auto* arg : attr->args()) {
      CRUBIT_ASSIGN_OR_RETURN(absl::string_view lifetime,
                              EvaluateAsStringLiteral(ast_context, arg));
      lifetimes.push_back(lifetime);
    }
  }
  return lifetimes;
}

absl::StatusOr<std::vector<absl::string_view>> CollectExplicitLifetimes(
    const clang::ASTContext& ast_context, const clang::Type& t) {
  std::vector<absl::string_view> lifetimes;
  const clang::Type* type = &t;
  // We're looking at ((t ($a, $b)) ($c, $d)) and want to get a flattened list
  // of [a; b; c; d].
  while (const auto* attributed_type = type->getAs<clang::AttributedType>()) {
    if (attributed_type->getAttr() == nullptr) {
      type = attributed_type->getEquivalentType().getTypePtr();
      if (type == nullptr) break;
      continue;
    }
    auto annotate_type_attr =
        clang::dyn_cast<clang::AnnotateTypeAttr>(attributed_type->getAttr());
    if (annotate_type_attr == nullptr ||
        annotate_type_attr->getAnnotation() != "lifetime") {
      type = attributed_type->getEquivalentType().getTypePtr();
      if (type == nullptr) break;
      continue;
    }
    if (size_t n_args = annotate_type_attr->args_size(); n_args != 0) {
      lifetimes.resize(lifetimes.size() + n_args);
      size_t arg_index = 0;
      for (const clang::Expr* arg : annotate_type_attr->args()) {
        CRUBIT_ASSIGN_OR_RETURN(absl::string_view lifetime,
                                EvaluateAsStringLiteral(ast_context, arg));
        lifetimes[lifetimes.size() - arg_index - 1] = lifetime;
        ++arg_index;
      }
    }
    type = attributed_type->getEquivalentType().getTypePtr();
    if (type == nullptr) break;
  }
  std::reverse(lifetimes.begin(), lifetimes.end());
  return lifetimes;
}

bool IsProto2Message(const clang::Decl& decl) {
  const auto* cxx_record_decl = clang::dyn_cast<clang::CXXRecordDecl>(&decl);
  if (cxx_record_decl == nullptr || !cxx_record_decl->isCompleteDefinition()) {
    return false;
  }
  // A forward-compatible way to check this is to see whether the record derives
  // from google::protobuf::MessageLite. (Note that because the record is a complete
  // definition, we have the full inheritance hierarchy available.)
  return !cxx_record_decl->forallBases([](const clang::CXXRecordDecl* base) {
    std::string base_name_owned = base->getQualifiedNameAsString();
    absl::string_view base_name = base_name_owned;
    // It's not clear that the default formatting rules will give us an absolute
    // qualified name, but popping off the first :: is benign.
    if (base_name.starts_with("::")) {
      base_name.remove_prefix(2);
    }
    return base_name != "google::protobuf::MessageLite";
  });
}

const clang::TagDecl* StripCStyleNameIntroducingTypedef(
    const clang::TypedefNameDecl* alias_decl) {
  if (alias_decl == nullptr) {
    return nullptr;
  }

  // If the typedef is giving a name to an anonymous tag type, then we just
  // return that anonymous tag type directly.
  // Example: typedef struct { ... } Foo;
  if (clang::TagDecl* anon_tag = alias_decl->getAnonDeclWithTypedefName()) {
    return anon_tag;
  }

  clang::QualType aliased_type = alias_decl->getASTContext().getTypedefType(
      clang::ElaboratedTypeKeyword::None, /*Qualifier=*/std::nullopt,
      const_cast<clang::TypedefNameDecl*>(alias_decl));

  if (aliased_type.isNull()) {
    return nullptr;
  }

  const clang::TagDecl* tag_decl = aliased_type->getAsTagDecl();

  if (tag_decl == nullptr) {
    // Not aliasing a tag type, not interested.
    return nullptr;
  }

  // Not interested in `typedef Bar Foo;`, so filter out cases where the alias
  // name is different from the tag name.
  if (tag_decl->getName() != alias_decl->getName()) {
    return nullptr;
  }

  // Not interested in `typedef other_context::Foo Foo;`, so filter out cases
  // where the tag decl is not in the same redecl context as the alias decl.
  if (tag_decl->getDeclContext()->getRedeclContext() !=
      alias_decl->getDeclContext()->getRedeclContext()) {
    return nullptr;
  }

  // The aliased type is a tag decl that has the same name and is in the same
  // redecl context as the alias decl, so we know it's shaped like
  // `typedef struct Foo Foo;`.
  return tag_decl;
}

bool ForceDefineImplicitFunction(ImportContext& ictx,
                                 clang::FunctionDecl* function_decl) {
  if (auto defaulted_kind = function_decl->getDefaultedFunctionKind();
      defaulted_kind.isSpecialMember()) {
    auto special_member_kind = defaulted_kind.asSpecialMember();
    if (!function_decl->isDeleted() &&
        (function_decl->isImplicit() || function_decl->isDefaulted()) &&
        !function_decl->doesThisDeclarationHaveABody()) {
      crubit::RecordingDiagnosticConsumer diagnostic_recorder =
          crubit::RecordDiagnostics(ictx.sema_.getDiagnostics(), [&] {
            FakeTUScope fake_tu_scope(ictx);
            clang::Sema::SynthesizedFunctionScope synthesized_function_scope(
                ictx.sema_, function_decl);
            // This is slightly different from DefineDefaultedFunction in that
            // we only consider certain special member kinds and we clear
            // the WillHaveBody flag. (We also run it in our diagnostic
            // sandbox.)
            switch (special_member_kind) {
              case clang::CXXSpecialMemberKind::CopyAssignment:
                function_decl->setWillHaveBody(false);
                ictx.sema_.DefineImplicitCopyAssignment(
                    function_decl->getLocation(),
                    llvm::cast<clang::CXXMethodDecl>(function_decl));
                break;
              case clang::CXXSpecialMemberKind::MoveAssignment:
                function_decl->setWillHaveBody(false);
                ictx.sema_.DefineImplicitMoveAssignment(
                    function_decl->getLocation(),
                    llvm::cast<clang::CXXMethodDecl>(function_decl));
                break;
              case clang::CXXSpecialMemberKind::MoveConstructor:
                function_decl->setWillHaveBody(false);
                ictx.sema_.DefineImplicitMoveConstructor(
                    function_decl->getLocation(),
                    llvm::cast<clang::CXXConstructorDecl>(function_decl));
                break;
              case clang::CXXSpecialMemberKind::CopyConstructor:
                function_decl->setWillHaveBody(false);
                ictx.sema_.DefineImplicitCopyConstructor(
                    function_decl->getLocation(),
                    llvm::cast<clang::CXXConstructorDecl>(function_decl));
                break;
              default:
                break;
            }
            ictx.sema_.PerformPendingInstantiations(/*LocalOnly=*/false,
                                                    /*AtEndOfTU=*/false);
          });
      if (diagnostic_recorder.getNumErrors() != 0) {
        return false;
      }
    }
  }
  return true;
}

bool EnsureFunctionDefined(ImportContext& ictx, clang::FunctionDecl* fn) {
  if (fn->isDeleted() || fn->getBody() != nullptr) {
    return true;
  }
  if (fn->isImplicit() || fn->isDefaulted()) {
    return ForceDefineImplicitFunction(ictx, fn);
  }
  if (fn->isTemplateInstantiation()) {
    crubit::RecordingDiagnosticConsumer diagnostic_recorder =
        crubit::RecordDiagnostics(ictx.sema_.getDiagnostics(), [&] {
          FakeTUScope fake_tu_scope(ictx);
          auto poi = fn->getPointOfInstantiation();
          if (poi.isInvalid()) poi = fn->getLocation();
          ictx.sema_.InstantiateFunctionDefinition(poi, fn);
          ictx.sema_.PerformPendingInstantiations(/*LocalOnly=*/false,
                                                  /*AtEndOfTU=*/false);
        });
    if (diagnostic_recorder.getNumErrors() != 0) {
      return false;
    }
  }
  return true;
}

std::string GetInvalidCallTarget(ImportContext& ictx,
                                 clang::FunctionDecl* function_decl) {
  // TODO(zarko): Should we ForceDefineImplicitFunction each function we
  // encounter?
  std::string invalid_decl_name;
  absl::flat_hash_set<clang::FunctionDecl*> visited_decls;
  struct BodyVisitor : public clang::RecursiveASTVisitor<BodyVisitor> {
    std::string& invalid_decl_name;
    absl::flat_hash_set<clang::FunctionDecl*>& visited_decls;
    ImportContext& ictx;
    clang::NamedDecl* blame_decl;
    BodyVisitor(std::string& invalid_decl_name,
                absl::flat_hash_set<clang::FunctionDecl*>& visited_decls,
                ImportContext& ictx, clang::NamedDecl* blame_decl)
        : invalid_decl_name(invalid_decl_name),
          visited_decls(visited_decls),
          ictx(ictx),
          blame_decl(blame_decl) {}
    bool CheckAndRecurse(clang::FunctionDecl* fn, bool force = false) {
      if (force || fn->isTemplateInstantiation() || fn->isDefaulted() ||
          fn->isImplicit()) {
        if (fn->getBody() == nullptr) {
          if (!EnsureFunctionDefined(ictx, fn)) {
            invalid_decl_name = fn->getQualifiedNameAsString();
            return false;
          }
        }
        bool is_satisfied = true;
        if (fn->getTrailingRequiresClause()) {
          clang::ConstraintSatisfaction satisfaction;
          if (ictx.sema_.CheckFunctionConstraints(fn, satisfaction)) {
            is_satisfied = false;
          } else {
            is_satisfied = satisfaction.IsSatisfied;
          }
        }
        if (fn->isInvalidDecl() || fn->isDeleted() || !is_satisfied) {
          invalid_decl_name = fn->getQualifiedNameAsString();
          return false;
        }
        if (fn->getBody() != nullptr) {
          if (!visited_decls.insert(fn).second) {
            return true;
          }
          BodyVisitor really_recurse(invalid_decl_name, visited_decls, ictx,
                                     blame_decl);
          really_recurse.TraverseStmt(fn->getBody());
          if (auto* ctor = clang::dyn_cast<clang::CXXConstructorDecl>(fn)) {
            for (clang::CXXCtorInitializer* init : ctor->inits()) {
              if (init->getInit()) {
                really_recurse.TraverseStmt(init->getInit());
              }
            }
          }
        }
      }
      return true;
    }
    bool VisitCXXConstructExpr(clang::CXXConstructExpr* expr) {
      if (auto* mfn = clang::dyn_cast<clang::CXXConstructorDecl>(
              expr->getConstructor())) {
        return CheckAndRecurse(mfn);
      }
      return true;
    }
    bool VisitMemberExpr(clang::MemberExpr* expr) {
      if (auto* mfn =
              clang::dyn_cast<clang::CXXMethodDecl>(expr->getMemberDecl())) {
        return CheckAndRecurse(mfn);
      }
      return true;
    }
    bool VisitStaticAssertDecl(clang::StaticAssertDecl* decl) {
      if (decl->isFailed()) {
        invalid_decl_name = blame_decl->getQualifiedNameAsString();
        return false;
      }
      return true;
    }
    bool VisitRecoveryExpr(clang::RecoveryExpr* expr) {
      // Clang appears to insert a RecoveryExpr in template bodies that fail
      // to instantiate properly.
      if (expr->containsErrors()) {
        invalid_decl_name = blame_decl->getQualifiedNameAsString();
        return false;
      }
      return true;
    }
    bool VisitDeclRefExpr(clang::DeclRefExpr* expr) {
      if (auto* fn = clang::dyn_cast<clang::FunctionDecl>(expr->getDecl())) {
        return CheckAndRecurse(fn);
      }
      return true;
    }
  } visitor(invalid_decl_name, visited_decls, ictx, function_decl);
  auto qnas = function_decl->getQualifiedNameAsString();
  if (!EnsureFunctionDefined(ictx, function_decl)) {
    return qnas;
  }
  if (function_decl->isInvalidDecl()) {
    return qnas;
  }
  visitor.CheckAndRecurse(function_decl, true);
  return invalid_decl_name;
}

absl::flat_hash_set<absl::string_view> GetIgnoredAttrs(
    const clang::Decl* decl) {
  absl::flat_hash_set<absl::string_view> ignored_attr_names;
  if (!decl) return ignored_attr_names;

  if (auto args = GetAnnotateAttrArgs(*decl, "crubit_unsafe_ignore_attr");
      args.ok() && args->has_value()) {
    clang::ASTContext& ast_context = decl->getASTContext();
    for (const clang::Expr* arg : **args) {
      if (auto name = GetExprAsStringLiteral(*arg, ast_context); name.ok()) {
        ignored_attr_names.insert(*name);
      }
    }
  }
  return ignored_attr_names;
}

}  // namespace crubit
