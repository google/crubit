// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/existing_rust_type.h"

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
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Type.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/Support/Casting.h"

namespace crubit {
namespace {

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

std::string_view ArgKindToString(clang::TemplateArgument::ArgKind kind) {
  switch (kind) {
    case clang::TemplateArgument::Null:
      return "Null";
    case clang::TemplateArgument::Type:
      return "Type";
    case clang::TemplateArgument::Declaration:
      return "Declaration";
    case clang::TemplateArgument::NullPtr:
      return "NullPtr";
    case clang::TemplateArgument::Integral:
      return "Integral";
    case clang::TemplateArgument::StructuralValue:
      return "StructuralValue";
    case clang::TemplateArgument::Template:
      return "Template";
    case clang::TemplateArgument::TemplateExpansion:
      return "TemplateExpansion";
    case clang::TemplateArgument::Expression:
      return "Expression";
    case clang::TemplateArgument::Pack:
      return "Pack";
    default:
      return "unknown";
  }
}

// Returns the ClassTemplateSpecializationDecl of `crubit::{name}` if `type` is
// an instantiation of it, or nullptr otherwise.
const clang::ClassTemplateSpecializationDecl*
GetCrubitClassTemplateSpecializationDecl(clang::QualType type,
                                         absl::string_view name) {
  const auto* record_type = type->getAs<clang::RecordType>();
  if (record_type == nullptr) {
    return nullptr;
  }
  const clang::RecordDecl* record_decl = record_type->getDecl();
  if (absl::string_view(record_decl->getName()) != name) {
    return nullptr;
  }
  if (!record_decl->getDeclContext()->isNamespace() ||
      llvm::cast<clang::NamespaceDecl>(record_decl->getDeclContext())
              ->getName() != "crubit") {
    return nullptr;
  }
  return llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(record_decl);
}

struct CrubitInternalRustType {
  std::string format_string;
  std::vector<TemplateArg> format_args;
};

// Gets the crubit_internal_rust_type attribute for `decl`.
absl::StatusOr<std::optional<CrubitInternalRustType>>
GetCrubitInternalRustTypeAttr(ImportContext& ictx, const clang::Decl& decl) {
  CRUBIT_ASSIGN_OR_RETURN(
      std::optional<AnnotateArgs> opt_args,
      GetAnnotateAttrArgs(decl, "crubit_internal_rust_type"));
  if (!opt_args.has_value()) return std::nullopt;
  const AnnotateArgs& args = *opt_args;
  if (args.empty()) {
    return absl::InvalidArgumentError(
        "crubit.rs-bug: The `CRUBIT_INTERNAL_RUST_TYPE` attribute is malformed."
        "Crubit expects the annotation to expand to the form "
        "`[[clang::annotate\"crubit_internal_rust_type\", \"RustType\", "
        "crubit::crubit_internal_rust_type_args<T1, T2, ...>())]]`, but "
        "instead only found `[[clang::annotate\"crubit_internal_rust_type\", "
        "\"RustType\")]]`");
  }
  CRUBIT_ASSIGN_OR_RETURN(
      absl::string_view format_string,
      GetExprAsStringLiteral(*args.front(), decl.getASTContext()));

  if (args.size() < 2) {
    return CrubitInternalRustType{.format_string = std::string(format_string)};
  }
  if (args.size() > 2) {
    return absl::InvalidArgumentError(
        "crubit.rs-bug: The `CRUBIT_INTERNAL_RUST_TYPE` attribute is malformed."
        "Crubit expects the annotation to expand to the form "
        "`[[clang::annotate\"crubit_internal_rust_type\", \"RustType\", "
        "crubit::crubit_internal_rust_type_args<T1, T2, ...>())]]`, but "
        "instead found `[[clang::annotate\"crubit_internal_rust_type\", "
        "\"RustType\", /* more than 1 trailing argument */)]]`");
  }

  const clang::ClassTemplateSpecializationDecl* spec =
      GetCrubitClassTemplateSpecializationDecl(
          args[1]->getType(), "crubit_internal_rust_type_args");
  if (spec == nullptr) {
    return absl::InvalidArgumentError(
        "crubit.rs-bug: The `CRUBIT_INTERNAL_RUST_TYPE` attribute is malformed."
        "Crubit expects the annotation to expand to the form "
        "`[[clang::annotate\"crubit_internal_rust_type\", \"RustType\", "
        "crubit::crubit_internal_rust_type_args<T1, T2, ...>())]]`, but "
        "instead found `[[clang::annotate\"crubit_internal_rust_type\", "
        "\"RustType\", /* something other than "
        "`crubit::crubit_internal_rust_type_args<...>()` */)]]`");
  }

  // In `crubit::crubit_internal_rust_type_args<A, B, C>`, there's only one
  // template argument: a Pack. To get A, B, and C, we need to call
  // `getPackAsArray()` on that template argument.
  if (spec->getTemplateArgs().size() != 1) {
    return absl::InvalidArgumentError(absl::StrCat(
        "crubit.rs-bug: The `CRUBIT_INTERNAL_RUST_TYPE` attribute is malformed."
        "Crubit expects the annotation to expand to the form "
        "`[[clang::annotate\"crubit_internal_rust_type\", \"RustType\", "
        "crubit::crubit_internal_rust_type_args<T1, T2, ...>())]]`, but "
        "instead found `[[clang::annotate\"crubit_internal_rust_type\", "
        "\"RustType\", crubit::crubit_internal_rust_type_args<...>())]]`, "
        "where the inner `<...>` has ",
        spec->getTemplateArgs().size(),
        " template arguments instead of a single pack argument."));
  }

  const clang::TemplateArgument& spec_template_arg =
      spec->getTemplateArgs().get(0);
  if (spec_template_arg.getKind() != clang::TemplateArgument::Pack) {
    return absl::InvalidArgumentError(absl::StrCat(
        "crubit.rs-bug: The `CRUBIT_INTERNAL_RUST_TYPE` attribute is malformed."
        "Crubit expects the annotation to expand to the form "
        "`[[clang::annotate\"crubit_internal_rust_type\", \"RustType\", "
        "crubit::crubit_internal_rust_type_args<T1, T2, ...>())]]`, but "
        "instead found `[[clang::annotate\"crubit_internal_rust_type\", "
        "\"RustType\", crubit::crubit_internal_rust_type_args<...>())]]`, "
        "where the inner `<...>` has a single argument of kind ",
        ArgKindToString(spec_template_arg.getKind()),
        " instead of a Pack argument."));
  }

  llvm::ArrayRef<clang::TemplateArgument> pack =
      spec_template_arg.getPackAsArray();

  std::vector<TemplateArg> format_args;
  format_args.reserve(pack.size());
  for (const auto& arg : pack) {
    if (arg.getKind() != clang::TemplateArgument::Type) {
      return absl::InvalidArgumentError(absl::StrCat(
          "The template arguments of `CRUBIT_INTERNAL_RUST_TYPE` must be "
          "types, found ",
          ArgKindToString(arg.getKind()),
          ". For const generics, use `crubit::const_generic<N>`."));
    }
    clang::QualType type = arg.getAsType();

    if (const auto* const_generic =
            GetCrubitClassTemplateSpecializationDecl(type, "const_generic")) {
      // The user wrote `crubit::const_generic<N>`, so we need to extract the
      // value of N from this.

      // Ensure that there is exactly one template argument (anything else
      // should be impossible).
      if (const_generic->getTemplateArgs().size() != 1) {
        return absl::InvalidArgumentError(
            "crubit.rs-bug: `crubit::const_generic` must have exactly one "
            "template argument.");
      }

      const clang::TemplateArgument& const_generic_arg =
          const_generic->getTemplateArgs().get(0);

      if (const_generic_arg.getKind() != clang::TemplateArgument::Integral) {
        return absl::InvalidArgumentError(absl::StrCat(
            "`crubit::const_generic` template argument must be an integral "
            "constant, found: ",
            ArgKindToString(const_generic_arg.getKind())));
      }
      format_args.push_back(
          const_generic_arg.getIntegralType()->isBooleanType()
              ? TemplateArg(const_generic_arg.getAsIntegral().getBoolValue())
              : TemplateArg(const_generic_arg.getAsIntegral().getExtValue()));
    } else {
      // TODO(b/454627672): is specialization_decl the right decl to check
      // for assumed_lifetimes?
      format_args.push_back(TemplateArg(
          ictx.ConvertQualType(type, /*lifetimes=*/nullptr, /*nullable=*/true,
                               ictx.AreAssumedLifetimesEnabledForTarget(
                                   ictx.GetOwningTarget(spec)))));
    }
  }

  return CrubitInternalRustType{
      .format_string = std::string(format_string),
      .format_args = std::move(format_args),
  };
}

// Gathers all template parameter names for `decl` (if any).
//
// `decl` must not be null.
std::optional<std::vector<std::string>> GetTemplateParameterNames(
    ImportContext& ictx, const clang::Decl* decl) {
  const auto* specialization_decl =
      llvm::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(decl);
  if (!specialization_decl) {
    return std::nullopt;
  }

  std::vector<std::string> result;
  result.reserve(specialization_decl->getTemplateArgs().size());
  for (const auto* template_param :
       specialization_decl->getSpecializedTemplate()
           ->getTemplateParameters()
           ->asArray()) {
    result.push_back(template_param->getDeclName().getAsString());
  }
  return result;
}

}  // namespace

std::optional<IR::Item> ExistingRustTypeImporter::Import(
    clang::TypeDecl* type_decl) {
  absl::StatusOr<std::optional<CrubitInternalRustType>> status_or_opt_attr =
      GetCrubitInternalRustTypeAttr(ictx_, *type_decl);
  if (!status_or_opt_attr.ok()) {
    return ictx_.HardError(
        *type_decl,
        // Failure here indicates that there was an incorrect attempt to use the
        // `crubit_internal_rust_type` attribute. This attribute should never
        // result in the generation of a Rust type, so we use the unnameable
        // kind.
        FormattedError::PrefixedStrCat(
            "Invalid CRUBIT_INTERNAL_RUST_TYPE attribute",
            std::move(status_or_opt_attr).status().message()));
  }
  if (!status_or_opt_attr->has_value()) {
    return std::nullopt;
  }
  const auto [format_string, format_args] = **std::move(status_or_opt_attr);
  absl::StatusOr<bool> is_same_abi = GetIsSameAbiAttribute(type_decl);
  if (!is_same_abi.ok()) {
    return ictx_.HardError(*type_decl,
                           FormattedError::PrefixedStrCat(
                               "Invalid crubit_internal_is_same_abi attribute",
                               is_same_abi.status().message()));
  }

  clang::ASTContext& context = type_decl->getASTContext();
  clang::QualType cc_qualtype = context.getTypeDeclType(type_decl);
  const clang::Type* cpp_type = cc_qualtype.getTypePtr();
  if (cpp_type == nullptr) return std::nullopt;
  // Tag keywords (e.g. `struct`, `class`, `union`) are suppressed so we get
  // `Foo` instead of `struct Foo`.
  clang::PrintingPolicy policy(context.getLangOpts());
  policy.SuppressTagKeyword = true;
  std::string cc_name = cc_qualtype.getAsString(policy);

  std::optional<std::vector<std::string>> type_parameter_names =
      GetTemplateParameterNames(ictx_, type_decl);

  ictx_.MarkAsSuccessfullyImported(type_decl);

  std::optional<SizeAlign> size_align;
  if (!cpp_type->isIncompleteType()) {
    size_align = SizeAlign{
        .size = context.getTypeSizeInChars(cpp_type).getQuantity(),
        .alignment = context.getTypeAlignInChars(cpp_type).getQuantity(),
    };
  }
  return ExistingRustType{
      .rs_name = std::move(format_string),
      .cc_name = std::move(cc_name),
      .unique_name = ictx_.GetUniqueName(*type_decl),
      .template_args = std::move(format_args),
      .template_arg_names =
          type_parameter_names.value_or(std::vector<std::string>()),
      .owning_target = ictx_.GetOwningTarget(type_decl),
      .size_align = std::move(size_align),
      .is_same_abi = *is_same_abi,
      .id = ictx_.GenerateItemId(type_decl),
  };
}

}  // namespace crubit
