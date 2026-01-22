// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_util.h"

#include <algorithm>
#include <array>
#include <cstddef>
#include <optional>
#include <string>
#include <vector>

#include "absl/algorithm/container.h"
#include "absl/container/flat_hash_set.h"
#include "absl/functional/function_ref.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/annotation_reader.h"
#include "common/status_macros.h"
#include "clang/AST/Attr.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/AST/Type.h"
#include "clang/Basic/AttrKinds.h"
#include "clang/Basic/LLVM.h"

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
    std::string name = attr->getAttrName() ? attr->getNormalizedFullName()
                                           : attr->getSpelling();
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
  if (!clang::isa<clang::CXXRecordDecl>(decl)) {
    return false;
  }

  const auto* cxx_record_decl = clang::dyn_cast<clang::CXXRecordDecl>(&decl);
  constexpr auto kProtoClasses = std::to_array<absl::string_view>({
      "google::protobuf::Message",
      "proto2::internal::ZeroFieldsBase",
      "google::protobuf::MessageLite",
  });

  return cxx_record_decl->isCompleteDefinition() &&
         absl::c_any_of(
             cxx_record_decl->bases(),
             [&](const clang::CXXBaseSpecifier& base) {
               std::string base_name_owned = base.getType().getAsString();
               absl::string_view base_name = base_name_owned;
               if (base_name.starts_with("::")) {
                 base_name.remove_prefix(2);
               }
               return absl::c_linear_search(kProtoClasses, base_name);
             });
}

}  // namespace crubit
