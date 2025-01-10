// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/type_map_override.h"

#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/annotation_reader.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Type.h"
#include "llvm/Support/Casting.h"

namespace crubit {
namespace {

// Gets the crubit_internal_rust_type attribute for `decl`.
// `decl` must not be null.
absl::StatusOr<std::optional<absl::string_view>> GetRustTypeAttribute(
    const clang::Decl* decl) {
  CRUBIT_ASSIGN_OR_RETURN(
      std::optional<AnnotateArgs> args,
      GetAnnotateAttrArgs(*decl, "crubit_internal_rust_type"));
  if (!args.has_value()) return std::nullopt;
  if (args->size() != 1) {
    return absl::InvalidArgumentError(
        "The `crubit_internal_rust_type` attribute requires a single "
        "string literal "
        "argument, the Rust type.");
  }
  return GetExprAsStringLiteral(*args->front(), decl->getASTContext());
}

// Gets the crubit_internal_same_abi attribute for `decl`.
// If the attribute is specified, returns true. If it's unspecified, returns
// false. If the attribute is malformed, returns a bad status.
//
// `decl` must not be null.
absl::StatusOr<bool> GetIsSameAbiAttribute(const clang::Decl* decl) {
  CRUBIT_ASSIGN_OR_RETURN(
      std::optional<AnnotateArgs> args,
      GetAnnotateAttrArgs(*decl, "crubit_internal_same_abi"));
  if (args.has_value() && !args->empty()) {
    return absl::InvalidArgumentError(
        "The `crubit_internal_same_abi` attribute takes no arguments.");
  }
  return args.has_value();
}

// Gathers all instantiated template parameters for `decl` (if any) and converts
// them to `MappedType`s.
//
// `decl` must not be null.
absl::StatusOr<std::optional<std::vector<MappedType>>> GetTemplateParameters(
    ImportContext& ictx, const clang::Decl* decl) {
  const auto* specialization_decl =
      llvm::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(decl);
  if (!specialization_decl) {
    return std::nullopt;
  }

  std::vector<MappedType> result;
  for (const auto& arg : specialization_decl->getTemplateArgs().asArray()) {
    auto mapped_type =
        ictx.ConvertQualType(arg.getAsType(), /*lifetimes=*/nullptr,
                             /*ref_qualifier_kind=*/std::nullopt,
                             /*nullable=*/false);
    if (!mapped_type.ok()) return mapped_type.status();

    result.push_back(*mapped_type);
  }

  return result;
}

}  // namespace

std::optional<IR::Item> TypeMapOverrideImporter::Import(
    clang::TypeDecl* type_decl) {
  absl::StatusOr<std::optional<absl::string_view>> rust_type =
      GetRustTypeAttribute(type_decl);
  if (!rust_type.ok()) {
    return ictx_.ImportUnsupportedItem(
        type_decl,
        // Failure here indicates that there was an incorrect attempt to use the
        // `crubit_internal_rust_type` attribute. This attribute should never
        // result in the generation of a Rust type, so we use the unnameable
        // kind.
        UnsupportedItem::Kind::kUnnameable, std::nullopt,
        FormattedError::PrefixedStrCat(
            "Invalid crubit_internal_rust_type attribute",
            rust_type.status().message()));
  }
  if (!rust_type->has_value()) {
    return std::nullopt;
  }
  absl::StatusOr<bool> is_same_abi = GetIsSameAbiAttribute(type_decl);
  if (!is_same_abi.ok()) {
    return ictx_.ImportUnsupportedItem(
        type_decl, UnsupportedItem::Kind::kUnnameable, std::nullopt,
        FormattedError::PrefixedStrCat(
            "Invalid crubit_internal_is_same_abi attribute",
            is_same_abi.status().message()));
  }

  auto rs_name = std::string(**rust_type);

  clang::ASTContext& context = type_decl->getASTContext();
  clang::QualType cc_qualtype = context.getTypeDeclType(type_decl);
  const clang::Type* cpp_type = cc_qualtype.getTypePtr();
  if (cpp_type == nullptr) return std::nullopt;
  std::string cc_name = cc_qualtype.getAsString();

  absl::StatusOr<std::optional<std::vector<MappedType>>> type_parameters =
      GetTemplateParameters(ictx_, type_decl);
  if (!type_parameters.ok()) {
    return ictx_.ImportUnsupportedItem(
        type_decl, UnsupportedItem::Kind::kUnnameable, std::nullopt,
        FormattedError::PrefixedStrCat("Error fetching template parameters",
                                       type_parameters.status().message()));
  }

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
      .type_parameters = type_parameters->value_or(std::vector<MappedType>()),
      .owning_target = ictx_.GetOwningTarget(type_decl),
      .size_align = std::move(size_align),
      .is_same_abi = *is_same_abi,
      .id = ictx_.GenerateItemId(type_decl),
  };
}

}  // namespace crubit
