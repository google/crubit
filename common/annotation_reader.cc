// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/annotation_reader.h"

#include <functional>
#include <optional>
#include <string>

#include "absl/base/attributes.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/status_macros.h"
#include "clang/AST/APValue.h"
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

// Returns the string literal value of `expr`.
//
// `ast_context` is the associated `ASTContext`. If `expr` is not a string
// literal, `error()` is returned.
//
// As `StringLiteral` expressions are always allocated on the `ASTContext (see
// `StringLiteral::Create()`), the returned `string_view` has the same lifetime
// as `ast_context`.
//
// TODO(yongheng): Merge with lifetime_annotations/type_lifetimes.cc.
static absl::StatusOr<absl::string_view> GetExprAsStringLiteral(
    const clang::Expr& expr,
    const clang::ASTContext& ast_context ABSL_ATTRIBUTE_LIFETIME_BOUND,
    std::function<absl::Status()> error) {
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

absl::StatusOr<absl::string_view> GetExprAsStringLiteral(
    const clang::Expr& expr,
    const clang::ASTContext& ast_context ABSL_ATTRIBUTE_LIFETIME_BOUND) {
  return GetExprAsStringLiteral(expr, ast_context, []() {
    return absl::InvalidArgumentError(
        "cannot evaluate argument as a string literal");
  });
}

static absl::StatusOr<const clang::AnnotateAttr*> GetAnnotateAttrSingleDecl(
    const clang::Decl& decl, absl::string_view annotation_name) {
  const clang::AnnotateAttr* found_attr = nullptr;
  for (clang::AnnotateAttr* attr : decl.specific_attrs<clang::AnnotateAttr>()) {
    if (attr->getAnnotation() != llvm::StringRef(annotation_name)) continue;

    if (found_attr != nullptr) {
      return absl::InvalidArgumentError(
          absl::StrCat("Only one `", annotation_name,
                       "` annotation may be placed on a declaration."));
    }
    found_attr = attr;
  }
  return found_attr;
}

static absl::Status InconsistentAnnotationsError(
    absl::string_view annotation_name) {
  return absl::InvalidArgumentError(
      absl::StrCat("Different declarations have inconsistent `",
                   annotation_name, "` annotations."));
}

static absl::Status CheckExpressionsAreSameConstant(
    const clang::Expr& expr1, const clang::Expr& expr2,
    absl::string_view annotation_name, const clang::ASTContext& ast_context) {
  clang::Expr::EvalResult eval_result_1, eval_result_2;
  if (!expr1.EvaluateAsConstantExpr(eval_result_1, ast_context) ||
      !expr2.EvaluateAsConstantExpr(eval_result_2, ast_context)) {
    return absl::InvalidArgumentError(
        absl::StrCat("Arguments of `", annotation_name,
                     "` annotation must be constant expressions."));
  }

  const clang::APValue& value1 = eval_result_1.Val;
  const clang::APValue& value2 = eval_result_2.Val;

  if (value1.getKind() != value2.getKind()) {
    return InconsistentAnnotationsError(annotation_name);
  }

  auto must_be_int_or_string_error = [annotation_name]() {
    return absl::InvalidArgumentError(absl::StrCat(
        "Arguments of `", annotation_name,
        "` annotation must be of integral type or string literals."));
  };

  switch (value1.getKind()) {
    case clang::APValue::Int:
      if (value1.getInt() != value2.getInt()) {
        return InconsistentAnnotationsError(annotation_name);
      }
      break;
    case clang::APValue::LValue: {
      CRUBIT_ASSIGN_OR_RETURN(
          absl::string_view value1_string,
          GetExprAsStringLiteral(expr1, ast_context,
                                 must_be_int_or_string_error));
      CRUBIT_ASSIGN_OR_RETURN(
          absl::string_view value2_string,
          GetExprAsStringLiteral(expr2, ast_context,
                                 must_be_int_or_string_error));
      if (value1_string != value2_string) {
        return InconsistentAnnotationsError(annotation_name);
      }
      break;
    }
    default:
      return must_be_int_or_string_error();
  }

  return absl::OkStatus();
}

static absl::Status CheckAnnotationsConsistent(
    const clang::AnnotateAttr* annotate1, const clang::AnnotateAttr* annotate2,
    const clang::ASTContext& ast_context) {
  if (annotate1->args_size() != annotate2->args_size())
    return InconsistentAnnotationsError(annotate1->getAnnotation());
  for (int i = 0; i < annotate1->args_size(); ++i) {
    CRUBIT_RETURN_IF_ERROR(CheckExpressionsAreSameConstant(
        *annotate1->args_begin()[i], *annotate2->args_begin()[i],
        annotate1->getAnnotation(), ast_context));
  }
  return absl::OkStatus();
}

absl::StatusOr<std::optional<AnnotateArgs>> GetAnnotateAttrArgs(
    const clang::Decl& decl, absl::string_view annotation_name) {
  const clang::AnnotateAttr* found_attr = nullptr;

  int num_found = 0;
  for (const clang::Decl* redecl : decl.redecls()) {
    if (redecl == nullptr) continue;

    CRUBIT_ASSIGN_OR_RETURN(
        const clang::AnnotateAttr* attr,
        GetAnnotateAttrSingleDecl(*redecl, annotation_name));

    if (attr != nullptr) {
      ++num_found;
      if (found_attr == nullptr) {
        found_attr = attr;
      } else {
        CRUBIT_RETURN_IF_ERROR(
            CheckAnnotationsConsistent(found_attr, attr, decl.getASTContext()));
      }
    }
  }

  // If only one redeclaration had an annotation, check the annotation against
  // itself. This checks that all arguments have the expected type.
  if (num_found == 1) {
    CRUBIT_RETURN_IF_ERROR(CheckAnnotationsConsistent(found_attr, found_attr,
                                                      decl.getASTContext()));
  }

  if (found_attr == nullptr) {
    return std::nullopt;
  }

  return AnnotateArgs(found_attr->args_begin(), found_attr->args_end());
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
