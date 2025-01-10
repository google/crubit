// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/annotation_reader.h"

#include <optional>
#include <string>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Expr.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/APSInt.h"
#include "llvm/ADT/StringRef.h"

namespace crubit {

absl::StatusOr<bool> GetExprAsBool(const clang::Expr& expr,
                                   const clang::ASTContext& ast_context) {
  clang::Expr::EvalResult eval_result;
  if (!expr.EvaluateAsConstantExpr(eval_result, ast_context)) {
    return absl::InvalidArgumentError(
        "failed to evaluate annotation expression as a constant");
  }
  if (eval_result.Val.getKind() != clang::APValue::Int) {
    return absl::InvalidArgumentError(
        "annotation expression must evaluate to a bool");
  }
  const llvm::APSInt& int_value = eval_result.Val.getInt();
  if (int_value.isZero()) {
    return false;
  } else {
    // Non-zero values are treated as true.
    return true;
  }
}

// TODO(yongheng): Merge with lifetime_annotations/type_lifetimes.cc.
absl::StatusOr<absl::string_view> GetExprAsStringLiteral(
    const clang::Expr& expr, const clang::ASTContext& ast_context) {
  auto error = []() {
    return absl::InvalidArgumentError(
        "cannot evaluate argument as a string literal");
  };

  clang::Expr::EvalResult eval_result;
  if (!expr.EvaluateAsConstantExpr(eval_result, ast_context) ||
      !eval_result.Val.isLValue()) {
    return error();
  }

  const auto* eval_result_expr =
      eval_result.Val.getLValueBase().dyn_cast<const clang::Expr*>();
  if (!eval_result_expr) {
    return error();
  }

  const auto* string_literal =
      clang::dyn_cast<clang::StringLiteral>(eval_result_expr);
  if (!string_literal) {
    return error();
  }

  return {string_literal->getString()};
}

absl::StatusOr<std::optional<AnnotateArgs>> GetAnnotateAttrArgs(
    const clang::Decl& decl, absl::string_view annotation_name) {
  std::optional<AnnotateArgs> result;

  for (clang::AnnotateAttr* attr : decl.specific_attrs<clang::AnnotateAttr>()) {
    if (attr->getAnnotation() != llvm::StringRef(annotation_name)) continue;

    if (result.has_value()) {
      return absl::InvalidArgumentError(
          absl::StrCat("Only one `", annotation_name,
                       "` annotation may be placed on a declaration."));
    }

    result.emplace(attr->args_begin(), attr->args_end());
  }

  return result;
}

std::optional<std::string> GetAnnotateArgAsStringByAttribute(
    const clang::Decl* decl, absl::string_view attribute) {
  absl::StatusOr<std::optional<AnnotateArgs>> maybe_args =
      GetAnnotateAttrArgs(*decl, attribute);
  if (!maybe_args.ok() || !maybe_args->has_value()) {
    return std::nullopt;
  }
  const AnnotateArgs& args = **maybe_args;
  if (args.size() != 1) {
    return std::nullopt;
  }
  absl::StatusOr<absl::string_view> maybe_val =
      GetExprAsStringLiteral(*args[0], decl->getASTContext());
  if (!maybe_val.ok()) {
    return std::nullopt;
  }
  return std::string(*maybe_val);
}

absl::Status RequireSingleStringArgIfExists(const clang::Decl* decl,
                                            absl::string_view attribute) {
  absl::StatusOr<std::optional<AnnotateArgs>> maybe_args =
      GetAnnotateAttrArgs(*decl, attribute);
  if (!maybe_args.ok() || !maybe_args->has_value()) {
    return absl::OkStatus();
  }
  const AnnotateArgs& args = **maybe_args;
  if (args.size() != 1) {
    return absl::InvalidArgumentError(absl::StrCat(
        "Attribute ", attribute, " must have a single string argument."));
  }
  absl::StatusOr<absl::string_view> arg =
      GetExprAsStringLiteral(*args[0], decl->getASTContext());
  if (!arg.ok()) {
    return absl::InvalidArgumentError(absl::StrCat(
        "Attribute ", attribute, " must have a single string argument."));
  }
  return absl::OkStatus();
}

}  // namespace crubit
