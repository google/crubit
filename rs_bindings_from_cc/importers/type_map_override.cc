// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/type_map_override.h"

#include <optional>
#include <string>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/StringRef.h"

namespace crubit {
namespace {

// Copied from lifetime_annotations/type_lifetimes.cc, which is expected to move
// into ClangTidy. See:
// https://discourse.llvm.org/t/rfc-lifetime-annotations-for-c/61377
absl::StatusOr<absl::string_view> EvaluateAsStringLiteral(
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

// Gets the requested attribute for `decl`.
// `decl` must not be null.
absl::StatusOr<const clang::AnnotateAttr*> GetAnnotateAttr(
    const clang::Decl* decl, absl::string_view attribute) {
  const clang::AnnotateAttr* found_attr = nullptr;
  for (clang::AnnotateAttr* attr :
       decl->specific_attrs<clang::AnnotateAttr>()) {
    if (attr->getAnnotation() != llvm::StringRef(attribute)) continue;

    if (found_attr != nullptr)
      return absl::InvalidArgumentError(
          absl::StrCat("Only one `", attribute,
                       "` attribute may be placed on a declaration."));
    found_attr = attr;
  }
  return found_attr;
}

// Gets the crubit_internal_rust_type attribute for `decl`.
// `decl` must not be null.
absl::StatusOr<std::optional<absl::string_view>> GetRustTypeAttribute(
    const clang::Decl* decl) {
  CRUBIT_ASSIGN_OR_RETURN(const clang::AnnotateAttr* attr,
                          GetAnnotateAttr(decl, "crubit_internal_rust_type"));
  if (attr == nullptr) return std::nullopt;
  if (attr->args_size() != 1)
    return absl::InvalidArgumentError(
        "The `crubit_internal_rust_type` attribute requires a single "
        "string literal "
        "argument, the Rust type.");
  const clang::Expr& arg = **attr->args_begin();
  return EvaluateAsStringLiteral(arg, decl->getASTContext());
}

// Gets the crubit_internal_same_abi attribute for `decl`.
// If the attribute is specified, returns true. If it's unspecified, returns
// false. If the attribute is malformed, returns a bad status.
//
// `decl` must not be null.
absl::StatusOr<bool> GetIsSameAbiAttribute(const clang::Decl* decl) {
  CRUBIT_ASSIGN_OR_RETURN(const clang::AnnotateAttr* attr,
                          GetAnnotateAttr(decl, "crubit_internal_same_abi"));
  if (attr != nullptr && attr->args_size() != 0)
    return absl::InvalidArgumentError(
        "The `crubit_internal_same_abi` attribute takes no arguments.");
  return attr != nullptr;
}
}  // namespace

std::optional<IR::Item> TypeMapOverrideImporter::Import(
    clang::TypeDecl* type_decl) {
  absl::StatusOr<std::optional<absl::string_view>> rust_type =
      GetRustTypeAttribute(type_decl);
  if (!rust_type.ok()) {
    return ictx_.ImportUnsupportedItem(
        type_decl, absl::StrCat("Invalid crubit_internal_rust_type attribute: ",
                                rust_type.status().message()));
  }
  if (!rust_type->has_value()) {
    return std::nullopt;
  }
  absl::StatusOr<bool> is_same_abi = GetIsSameAbiAttribute(type_decl);
  if (!is_same_abi.ok()) {
    return ictx_.ImportUnsupportedItem(
        type_decl,
        absl::StrCat("Invalid crubit_internal_is_same_abi attribute: ",
                     is_same_abi.status().message()));
  }

  auto rs_name = std::string(**rust_type);

  clang::ASTContext& context = type_decl->getASTContext();
  clang::QualType cc_qualtype = context.getTypeDeclType(type_decl);
  const clang::Type* cpp_type = cc_qualtype.getTypePtr();
  if (cpp_type == nullptr) return std::nullopt;
  std::string cc_name = cc_qualtype.getAsString();

  ictx_.MarkAsSuccessfullyImported(type_decl);

  std::optional<SizeAlign> size_align;
  if (!cpp_type->isIncompleteType()) {
    size_align = SizeAlign{
        .size = context.getTypeSizeInChars(cpp_type).getQuantity(),
        .alignment = context.getTypeAlignInChars(cpp_type).getQuantity(),
    };
  }
  return TypeMapOverride{
      .rs_name = std::move(rs_name),
      .cc_name = std::move(cc_name),
      .owning_target = ictx_.GetOwningTarget(type_decl),
      .size_align = std::move(size_align),
      .is_same_abi = *is_same_abi,
      .id = ictx_.GenerateItemId(type_decl),
  };
}

}  // namespace crubit
