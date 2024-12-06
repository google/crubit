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

absl::StatusOr<bool> GetAnnotateArgAsBool(
    const clang::AnnotateAttr& attr, const clang::ASTContext& ast_context) {
  if (attr.args_size() != 1) {
    return absl::InvalidArgumentError(
        "annotation must have exactly one argument");
  }
  const clang::Expr& expr = **attr.args_begin();
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
absl::StatusOr<absl::string_view> GetAnnotateArgAsStringLiteral(
    const clang::AnnotateAttr& attr, const clang::ASTContext& ast_context) {
  if (attr.args_size() != 1) {
    return absl::InvalidArgumentError(
        "annotation must have exactly one argument");
  }
  const clang::Expr& expr = **attr.args_begin();
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

absl::StatusOr<const clang::AnnotateAttr*> GetAnnotateAttr(
    const clang::Decl& decl, absl::string_view attribute) {
  const clang::AnnotateAttr* found_attr = nullptr;
  for (clang::AnnotateAttr* attr : decl.specific_attrs<clang::AnnotateAttr>()) {
    if (attr->getAnnotation() != llvm::StringRef(attribute)) continue;

    if (found_attr != nullptr)
      return absl::InvalidArgumentError(
          absl::StrCat("Only one `", attribute,
                       "` attribute may be placed on a declaration."));
    found_attr = attr;
  }
  return found_attr;
}

std::optional<std::string> GetAnnotateArgAsStringByAttribute(
    const clang::Decl* decl, absl::string_view attribute) {
  absl::StatusOr<const clang::AnnotateAttr*> bridging_type_annotation =
      GetAnnotateAttr(*decl, attribute);
  if (!bridging_type_annotation.ok() || *bridging_type_annotation == nullptr) {
    return std::nullopt;
  }
  absl::StatusOr<absl::string_view> bridging_type =
      GetAnnotateArgAsStringLiteral(**bridging_type_annotation,
                                    decl->getASTContext());
  if (!bridging_type.ok()) {
    return std::nullopt;
  }
  return std::string(*bridging_type);
}

absl::Status RequireSingleStringArgIfExists(const clang::Decl* decl,
                                            absl::string_view attribute) {
  absl::StatusOr<const clang::AnnotateAttr*> attr =
      GetAnnotateAttr(*decl, attribute);
  if (!attr.ok() || *attr == nullptr) {
    return absl::OkStatus();
  }
  if (attr.value()->args_size() != 1) {
    return absl::InvalidArgumentError(absl::StrCat(
        "Attribute ", attribute, " must have a single string argument."));
  }
  absl::StatusOr<absl::string_view> arg =
      GetAnnotateArgAsStringLiteral(**attr, decl->getASTContext());
  if (!arg.ok()) {
    return absl::InvalidArgumentError(absl::StrCat(
        "Attribute ", attribute, " must have a single string argument."));
  }
  return absl::OkStatus();
}

}  // namespace crubit
