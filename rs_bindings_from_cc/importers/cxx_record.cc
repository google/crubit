// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/cxx_record.h"

#include <algorithm>
#include <array>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "clang/Basic/SourceLocation.h"
#include "absl/base/nullability.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/log/die_if_null.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/annotation_reader.h"
#include "common/status_macros.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_convert.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/CXXInheritance.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/PrettyPrinter.h"
#include "clang/AST/RecordLayout.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/Sema.h"
#include "llvm/Support/ErrorHandling.h"
namespace crubit {

namespace {

std::string GetClassTemplateSpecializationCcName(
    const clang::ASTContext& ast_context,
    const clang::ClassTemplateSpecializationDecl* specialization_decl,
    bool use_preferred_names) {
  clang::PrintingPolicy policy(ast_context.getLangOpts());
  policy.IncludeTagDefinition = false;
  // Canonicalize types -- in particular, the template parameter types must be
  // desugared out of an `ElaboratedType` so that their namespaces are written
  // down.
  policy.PrintAsCanonical = true;
  policy.UsePreferredNames = use_preferred_names;
  // Use type suffix (e.g. `123u` rather than just `123`) to avoid the
  // `-Wimplicitly-unsigned-literal` warning.  See also b/244616557.
  policy.AlwaysIncludeTypeForTemplateArgument = true;

  return clang::QualType(specialization_decl->getTypeForDecl(), 0)
      .getAsString(policy);
}

AccessSpecifier TranslateAccessSpecifier(clang::AccessSpecifier access) {
  switch (access) {
    case clang::AS_public:
      return kPublic;
    case clang::AS_protected:
      return kProtected;
    case clang::AS_private:
      return kPrivate;
    case clang::AS_none:
      LOG(FATAL)
          << "We should never be encoding a 'none' access specifier in IR.";
      // We have to return something. Conservatively return private so we don't
      // inadvertently make a private member variable accessible in Rust.
      return kPrivate;
  }
}

absl::StatusOr<RecordType> TranslateRecordType(
    const clang::RecordDecl& record_decl) {
  switch (record_decl.getTagKind()) {
    case clang::TagTypeKind::Struct:
      return RecordType::kStruct;
    case clang::TagTypeKind::Union:
      return RecordType::kUnion;
    case clang::TagTypeKind::Class:
      return RecordType::kClass;
    case clang::TagTypeKind::Enum:
      llvm::report_fatal_error(
          "clang::RecordDecl::getTagKind shouldn't return TagTypeKind::Enum");
    case clang::TagTypeKind::Interface:
      // Some docs about `__interface` can be found here:
      // https://docs.microsoft.com/en-us/cpp/cpp/interface?view=msvc-170
      return absl::UnimplementedError(
          "`__interface` / clang::TagTypeKind::Interface is not supported");
  }

  llvm::report_fatal_error("Unrecognized clang::TagKind");
}

// Returns the values of the given keys in `record_decl` if they all exist and
// are strings. Otherwise, returns `std::nullopt`.
template <size_t N>
std::optional<std::array<std::string, N>> GetKeyValues(
    const clang::RecordDecl& record_decl,
    std::array<absl::string_view, N> keys) {
  std::array<std::string, N> values;
  for (int i = 0; i < N; ++i) {
    absl::StatusOr<std::optional<std::string>> value =
        GetAnnotationWithStringArg(record_decl, keys[i]);
    CHECK_OK(value);
    if (!value->has_value()) {
      CHECK_EQ(i, 0) << "Missing value for key: '" << keys[i]
                     << "'; use the provided annotation to ensure that all "
                        "keys are present";
      return std::nullopt;
    }
    values[i] = **std::move(value);
  }
  return values;
}

// Returns the bridge type annotation for the given `record_decl` if it exists.
std::optional<BridgeType> GetBridgeTypeAnnotation(
    const clang::RecordDecl& record_decl) {
  auto void_converter_values = GetKeyValues<3>(
      record_decl,
      {"crubit_bridge_type", "crubit_bridge_type_rust_to_cpp_converter",
       "crubit_bridge_type_cpp_to_rust_converter"});
  auto crubit_abi_values = GetKeyValues<3>(
      record_decl, {"crubit_bridge_rust_name", "crubit_bridge_abi_rust",
                    "crubit_bridge_abi_cpp"});
  auto crubit_slice_ptr_abi_values =
      GetKeyValues<1>(record_decl, {"crubit_bridge_slice_ptr_abi_cpp"});
  CHECK(1 >= void_converter_values.has_value() + crubit_abi_values.has_value() +
                 crubit_slice_ptr_abi_values.has_value())
      << "CRUBIT_BRIDGE_VOID_CONVERTERS, CRUBIT_BRIDGE, and "
         "CRUBIT_BRIDGE_SLICE_PTR are mutually exclusive, and cannot be used "
         "on the same type.";

  if (crubit::IsProto2Message(record_decl)) {
    return BridgeType{BridgeType::ProtoMessageBridge{
        .rust_name = record_decl.getNameAsString(),
        .abi_rust = "ProtoMessageRustBridge",
        .abi_cpp = "::crubit::BoxedAbi"}};
  }

  if (void_converter_values.has_value()) {
    auto [rust_name, rust_to_cpp_converter, cpp_to_rust_converter] =
        *void_converter_values;
    return BridgeType{BridgeType::BridgeVoidConverters{
        .rust_name = std::move(rust_name),
        .rust_to_cpp_converter = std::move(rust_to_cpp_converter),
        .cpp_to_rust_converter = std::move(cpp_to_rust_converter),
    }};
  }
  if (crubit_abi_values.has_value()) {
    auto [rust_name, abi_rust, abi_cpp] = *crubit_abi_values;
    return BridgeType{BridgeType::Bridge{
        .rust_name = std::move(rust_name),
        .abi_rust = std::move(abi_rust),
        .abi_cpp = std::move(abi_cpp),
    }};
  }
  if (crubit_slice_ptr_abi_values.has_value()) {
    auto [abi_cpp] = *crubit_slice_ptr_abi_values;
    return BridgeType{BridgeType::SlicePointer{
        .abi_cpp = std::move(abi_cpp),
    }};
  }
  return std::nullopt;
}

// Helper function for `GetTraitDerives` to populate the derived traits in
// `TraitDerives`.
absl::Status AddTraitDerives(const clang::Decl& decl, TraitDerives& result) {
  CRUBIT_ASSIGN_OR_RETURN(
      std::optional<AnnotateArgs> args,
      GetAnnotateAttrArgs(decl, "crubit_internal_trait_derive"));

  if (!args.has_value()) return absl::OkStatus();
  clang::ASTContext& ast_context = decl.getASTContext();

  absl::flat_hash_set<absl::string_view> custom_traits;
  for (const clang::Expr* arg : *args) {
    CRUBIT_ASSIGN_OR_RETURN(const absl::string_view derived_trait,
                            GetExprAsStringLiteral(*arg, ast_context));
    absl::string_view trait;
    TraitImplPolarity polarity;
    if (derived_trait.starts_with("!")) {
      trait = derived_trait.substr(1);
      polarity = TraitImplPolarity::kNegative;
    } else {
      trait = derived_trait;
      polarity = TraitImplPolarity::kPositive;
    }

    if (trait == "Send" || trait == "Sync") {
      if (polarity == TraitImplPolarity::kNegative) {
        return absl::InvalidArgumentError(absl::StrCat(
            "Trait '", trait, "' is negatively derived by default."));
      }
      return absl::InvalidArgumentError(
          absl::StrCat("Trait '", trait,
                       "' is an unsafe trait, and must be implemented with the "
                       "`CRUBIT_UNSAFE_IMPL(\"",
                       trait, "\")` macro."));
    }

    TraitImplPolarity* absl_nullable selected = result.Polarity(trait);
    if (selected == nullptr) {
      // Custom (unrecognized) trait.
      if (polarity == TraitImplPolarity::kNegative) {
        return absl::InvalidArgumentError(absl::StrCat(
            "Custom trait '", trait, "' cannot be negatively derived."));
      }
      auto [it, did_insert] = custom_traits.insert(trait);
      if (did_insert) continue;

      return absl::InvalidArgumentError(absl::StrCat(
          "Custom trait '", trait, "' is derived multiple times."));
    }

    if (*selected == TraitImplPolarity::kNone) {
      // Trait is not yet derived, happy path :)
      *selected = polarity;
      continue;
    }

    if (*selected != polarity) {
      return absl::InvalidArgumentError(absl::StrCat(
          "Trait '", trait, "' is derived both positively and negatively."));
    }
    return absl::InvalidArgumentError(
        absl::StrCat("Trait '", trait, "' is derived multiple times."));
  }
  result.custom.reserve(custom_traits.size());
  for (const absl::string_view trait : custom_traits) {
    result.custom.emplace_back(trait);
  }
  return absl::OkStatus();
}

// Helper function for `GetTraitDerives` to populate the unsafe implementation
// fields in `TraitDerives`.
absl::Status AddUnsafeImpls(const clang::Decl& decl, TraitDerives& result) {
  CRUBIT_ASSIGN_OR_RETURN(
      std::optional<AnnotateArgs> args,
      GetAnnotateAttrArgs(decl, "crubit_internal_unsafe_impl"));

  if (!args.has_value()) return absl::OkStatus();
  clang::ASTContext& ast_context = decl.getASTContext();

  for (const clang::Expr* arg : *args) {
    CRUBIT_ASSIGN_OR_RETURN(const absl::string_view unsafe_impl,
                            GetExprAsStringLiteral(*arg, ast_context));
    if (unsafe_impl == "Send") {
      if (result.send) {
        return absl::InvalidArgumentError(
            "Unsafe implementation 'Send' is derived multiple times.");
      }
      result.send = true;
    } else if (unsafe_impl == "Sync") {
      if (result.sync) {
        return absl::InvalidArgumentError(
            "Unsafe implementation 'Sync' is derived multiple times.");
      }
      result.sync = true;
    } else {
      return absl::InvalidArgumentError(absl::StrCat(
          "Unsafe implementation '", unsafe_impl, "' is not supported."));
    }
  }
  return absl::OkStatus();
}

// Returns the set of traits to derive on the Rust type.
absl::StatusOr<TraitDerives> GetTraitDerives(const clang::Decl& decl) {
  TraitDerives result;
  CRUBIT_RETURN_IF_ERROR(AddTraitDerives(decl, result));
  CRUBIT_RETURN_IF_ERROR(AddUnsafeImpls(decl, result));
  return result;
}

absl::StatusOr<bool> IsUnsafeType(const clang::Decl& decl) {
  CRUBIT_ASSIGN_OR_RETURN(std::optional<AnnotateArgs> args,
                          GetAnnotateAttrArgs(decl, "crubit_override_unsafe"));
  if (!args.has_value()) return false;
  if (args->size() != 1) {
    return absl::InvalidArgumentError(
        "`crubit_override_unsafe` annotation must have exactly one argument");
  }

  return GetExprAsBool(*args->front(), decl.getASTContext());
}

std::optional<Identifier> StringRefToOptionalIdentifier(llvm::StringRef name) {
  if (name.empty()) {
    return std::nullopt;
  }
  return Identifier(std::string(name));
}

}  // namespace

std::optional<Identifier> CXXRecordDeclImporter::GetTranslatedFieldName(
    const clang::FieldDecl* field_decl) {
  if (field_decl->getName().empty()) {
    CHECK(!field_decl->hasAttr<clang::NoUniqueAddressAttr>() &&
          "Unnamed fields can't be annotated with [[no_unique_address]]");
    // We don't just conjure an artificial name for an unnamed field, because
    // in the future such fields may be elided entirely - see unnamed members
    // in:
    // - https://en.cppreference.com/w/c/language/struct
    // - https://rust-lang.github.io/rfcs/2102-unnamed-fields.html
    return std::nullopt;
  }

  absl::StatusOr<TranslatedIdentifier> name =
      ictx_.GetTranslatedIdentifier(field_decl);
  if (!name.ok()) {
    unsigned field_pos = field_decl->getFieldIndex();
    return {Identifier(absl::StrCat("__field_", field_pos))};
  }
  return (*name).rs_identifier();
}

bool IsKnownAttr(const clang::Attr& attr) {
  return clang::isa<clang::AlignedAttr>(attr) ||
         clang::isa<clang::CoroLifetimeBoundAttr>(attr) ||
         clang::isa<clang::CoroReturnTypeAttr>(attr) ||
         clang::isa<clang::FinalAttr>(attr) ||
         clang::isa<clang::OwnerAttr>(attr) ||
         clang::isa<clang::PointerAttr>(attr) ||
         clang::isa<clang::PreferredNameAttr>(attr) ||
         clang::isa<clang::TrivialABIAttr>(attr) ||
         clang::isa<clang::WarnUnusedResultAttr>(attr);
}

std::optional<IR::Item> CXXRecordDeclImporter::Import(
    clang::CXXRecordDecl* record_decl) {
  const clang::DeclContext* decl_context = record_decl->getDeclContext();
  if (decl_context->isFunctionOrMethod()) {
    return std::nullopt;
  }
  if (ictx_.HasBeenAlreadySuccessfullyImported(record_decl)) {
    LOG(FATAL)
        << ("THIS IS A BUG: the type was marked as imported, so we "
            "short-circuited evaluation here. However, instead of the fully "
            "imported type being used, apparently this empty stub was used "
            "instead. Report this upstream.");
  }
  if (record_decl->isInjectedClassName()) {
    return std::nullopt;
  }
  if (record_decl->isImplicit()) {
    return std::nullopt;
  }
  if (clang::isa<clang::ClassTemplatePartialSpecializationDecl>(record_decl)) {
    return ictx_.ImportUnsupportedItem(
        *record_decl, std::nullopt,
        FormattedError::Static(
            "Partially-specialized class templates are not supported"));
  }

  if (record_decl->isInvalidDecl()) {
    return std::nullopt;
  }

  std::optional<IR::Item> attr_error_item;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*record_decl, [&](const clang::Attr& attr) {
        if (IsKnownAttr(attr)) {
          return true;
        } else if (auto* visibility =
                       clang::dyn_cast<clang::VisibilityAttr>(&attr);
                   visibility && record_decl->isInStdNamespace()) {
          if (visibility->getVisibility() ==
              clang::VisibilityAttr::VisibilityType::Hidden) {
            attr_error_item = ictx_.ImportUnsupportedItem(
                *record_decl, std::nullopt,
                FormattedError::Static("Records from the standard library with "
                                       "hidden visibility are not supported"));
          }
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItem(
        *record_decl, std::nullopt,
        FormattedError::FromStatus(std::move(unknown_attr.status())));
  }
  if (attr_error_item.has_value()) {
    return attr_error_item;
  }

  std::string rs_name, cc_name;
  clang::SourceLocation source_loc;
  std::optional<std::string> doc_comment;
  bool is_explicit_class_template_instantiation_definition = false;
  std::optional<TemplateSpecialization> template_specialization;
  std::optional<BridgeType> bridge_type = GetBridgeTypeAnnotation(*record_decl);
  BazelLabel owning_target = ictx_.GetOwningTarget(record_decl);
  if (auto* specialization_decl =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
              record_decl)) {
    is_explicit_class_template_instantiation_definition =
        specialization_decl->getSpecializationKind() ==
        clang::TSK_ExplicitInstantiationDefinition;
    rs_name = ictx_.GetMangledName(specialization_decl);
    // use_preferred_names = false so that this returns e.g.
    // `basic_string_view<char16_t>` instead of 'u16string_view' despite
    // `_LIBCPP_PREFERRED_NAME(u16string_view)`.  See also b/244350186.
    cc_name = GetClassTemplateSpecializationCcName(
        ictx_.ctx_, specialization_decl, /*use_preferred_names=*/false);
    std::string cc_preferred_name =
        GetClassTemplateSpecializationCcName(ictx_.ctx_, specialization_decl,
                                             /*use_preferred_names=*/true);
    template_specialization.emplace();
    template_specialization->is_string_view =
        cc_preferred_name == "std::string_view";
    template_specialization->is_wstring_view =
        cc_preferred_name == "std::wstring_view";
    doc_comment = ictx_.GetComment(specialization_decl);
    if (!doc_comment.has_value()) {
      doc_comment =
          ictx_.GetComment(specialization_decl->getSpecializedTemplate());
    }
    source_loc = specialization_decl->getBeginLoc();
    // Specify defining_target if it's a template instantiation.
    if (auto instantiation_source =
            specialization_decl->getSpecializedTemplateOrPartial()) {
      clang::NamedDecl* decl;
      if (auto* template_decl =
              instantiation_source.dyn_cast<clang::ClassTemplateDecl*>()) {
        decl = template_decl;
      } else {
        decl = instantiation_source
                   .get<clang::ClassTemplatePartialSpecializationDecl*>();
      }
      BazelLabel target = ictx_.GetOwningTarget(decl);
      // TODO(okabayashi): File a bug for generalizing "canonical insts".
      // When a template like `std::string_view` is instantiated, it will be
      // owned by whatever target it was instantiated in. The C++ compiler is
      // then responsible for unifying identical instantiations. However, this
      // is a pain for Crubit because we aren't able to generally unify these.
      // In the case of `std::string_view`, however, we know that there's an
      // instantiation in the `cc_std` target, so I've chosen that as the
      // canonical instantiation, and am mapping all other instantiations to
      // that instantiation.
      // A problem with this is it's not _actually_ the same ItemId, and it
      // really should be. This ensures that when we refer to this Item, it's
      // spelled `cc_std::__CcTemplateInst...`. But a major downside is that we
      // still generate this template inst struct...
      if (template_specialization->is_string_view) {
        owning_target = target;
      }
      template_specialization->defining_target = std::move(target);
    }
    template_specialization->template_name =
        specialization_decl->getQualifiedNameAsString();
    // preferred_cc_name.substr(0, preferred_cc_name.find('<'));
    for (const clang::TemplateArgument& template_arg :
         specialization_decl->getTemplateArgs().asArray()) {
      if (template_arg.getKind() == clang::TemplateArgument::ArgKind::Type) {
        template_specialization->template_args.emplace_back(
            TemplateArg{ictx_.ConvertQualType(template_arg.getAsType(),
                                              /*lifetimes=*/nullptr)});
      }
    }

    if (!bridge_type.has_value()) {
      absl::StatusOr<std::optional<BridgeType>> builtin_bridge_type =
          GetBuiltinBridgeType(specialization_decl);
      if (!builtin_bridge_type.ok()) {
        return ictx_.ImportUnsupportedItem(
            *record_decl, std::nullopt,
            FormattedError::FromStatus(
                std::move(builtin_bridge_type).status()));
      }
      bridge_type = *std::move(builtin_bridge_type);
    }

    // TODO(b/436862191): Remove this once the migration is complete.
    if (!bridge_type.has_value()) {
      const clang::CXXRecordDecl* cxx_record_decl =
          specialization_decl->getSpecializedTemplate()->getTemplatedDecl();
      if (ictx_.GetOwningTarget(cxx_record_decl) ==
              BazelLabel("@abseil-cpp//absl/status:statusor") &&
          cxx_record_decl->getName() == "StatusOr") {
        bridge_type = BridgeType{BridgeType::Bridge{
            .rust_name = "::status::absl::StatusOr",
            .abi_rust = "::status::absl::StatusOrAbi",
            .abi_cpp = "::crubit::StatusOrAbi",
        }};
      }
    }
  } else {
    const clang::NamedDecl* named_decl = record_decl;
    if (record_decl->getName().empty()) {
      if (auto* typedef_decl = record_decl->getTypedefNameForAnonDecl()) {
        named_decl = typedef_decl;
      } else {
        // Skip anonymous structs that don't get a name via typedecl.
        return std::nullopt;
      }
    }
    CHECK(!named_decl->getName().empty());

    absl::StatusOr<TranslatedIdentifier> record_name =
        ictx_.GetTranslatedIdentifier(named_decl);
    if (record_name.ok()) {
      rs_name = (*record_name).rs_identifier().Ident();
      cc_name = (*record_name).cc_identifier.Ident();
      doc_comment = ictx_.GetComment(record_decl);
      source_loc = record_decl->getBeginLoc();
    } else {
      return ictx_.ImportUnsupportedItem(
          *record_decl, std::nullopt,
          FormattedError::PrefixedStrCat("Record name is not supported",
                                         record_name.status().message()));
    }
  }

  // TODO(b/436862191): Remove this once the migration is complete.
  if (!bridge_type.has_value()) {
    if (ictx_.GetOwningTarget(record_decl) ==
            BazelLabel("@abseil-cpp//absl/status:status") &&
        record_decl->getName() == "Status") {
      bridge_type = BridgeType{BridgeType::Bridge{
          .rust_name = "absl::Status",
          .abi_rust = "absl::StatusAbi",
          .abi_cpp = "::crubit::StatusAbi",
      }};
    }
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(record_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        *record_decl, std::nullopt,
        FormattedError::FromStatus(std::move(enclosing_item_id.status())));
  }

  // Reports an unsupported type with the given error.
  //
  // This is preferred to invoking `ImportUnsupportedItem` directly because it
  // ensures that the path is set correctly. Note that this cannot be used above
  // because the enclosing item ID and translated name are not yet available.
  auto unsupported = [this, &record_decl, &rs_name,
                      &enclosing_item_id](FormattedError error) {
    return ictx_.ImportUnsupportedItem(
        *record_decl,
        UnsupportedItem::Path{.ident = Identifier(rs_name),
                              .enclosing_item_id = *enclosing_item_id},
        error);
  };

  if (record_decl->isDependentContext()) {
    // We can't pass this to getASTRecordLayout() or it'll segfault.
    // TODO(jeanpierreda): investigate what we can do to support dependent records?
    // All I know is that I saw other code calling getASTRecordLayout() do the
    // same check. But getASTRecordLayout() itself doesn't actually document
    // this.
    return unsupported(
        FormattedError::Static("Dependent records are not supported"));
  }

  absl::StatusOr<RecordType> record_type = TranslateRecordType(*record_decl);
  if (!record_type.ok()) {
    return unsupported(
        FormattedError::FromStatus(std::move(record_type.status())));
  }

  if (record_decl->hasAttr<clang::PackedAttr>() ||
      std::any_of(record_decl->field_begin(), record_decl->field_end(),
                  [](const clang::FieldDecl* field_decl) {
                    return field_decl->hasAttr<clang::PackedAttr>();
                  })) {
    return unsupported(
        FormattedError::Static("Records with packed layout are not supported"));
  }

  ictx_.MarkAsSuccessfullyImported(record_decl);
  if (!record_decl->isCompleteDefinition()) {
    return IncompleteRecord{.cc_name = Identifier(cc_name),
                            .rs_name = Identifier(rs_name),
                            .id = ictx_.GenerateItemId(record_decl),
                            .owning_target = ictx_.GetOwningTarget(record_decl),
                            .unknown_attr = std::move(*unknown_attr),
                            .record_type = *record_type,
                            .enclosing_item_id = *std::move(enclosing_item_id)};
  }

  ictx_.sema_.ForceDeclarationOfImplicitMembers(record_decl);

  const clang::ASTRecordLayout& layout =
      ictx_.ctx_.getASTRecordLayout(record_decl);

  bool is_derived_class = record_decl->getNumBases() != 0;
  bool override_alignment = record_decl->hasAttr<clang::AlignedAttr>() ||
                            is_derived_class || layout.hasOwnVFPtr();

  bool is_effectively_final =
      record_decl->isEffectivelyFinal() || record_decl->isUnion();

  std::optional<std::string> nodiscard;
  if (const auto* attr = record_decl->getAttr<clang::WarnUnusedResultAttr>();
      attr != nullptr) {
    nodiscard.emplace(attr->getMessage());
  }

  auto item_ids = ictx_.GetItemIdsInSourceOrder(record_decl);
  item_ids.erase(std::remove_if(item_ids.begin(), item_ids.end(),
                                [&](ItemId item_id) {
                                  return ictx_.IsUnsupportedAndAlien(item_id);
                                }),
                 item_ids.end());
  const clang::TypedefNameDecl* anon_typedef =
      record_decl->getTypedefNameForAnonDecl();

  absl::StatusOr<TraitDerives> trait_derives = GetTraitDerives(*record_decl);
  if (!trait_derives.ok()) {
    return unsupported(
        FormattedError::FromStatus(std::move(trait_derives).status()));
  }

  absl::StatusOr<bool> is_unsafe_type = IsUnsafeType(*record_decl);
  if (!is_unsafe_type.ok()) {
    return unsupported(
        FormattedError::FromStatus(std::move(is_unsafe_type).status()));
  }

  auto record = Record{
      .rs_name = Identifier(rs_name),
      .cc_name = Identifier(cc_name),
      .mangled_cc_name = ictx_.GetMangledName(record_decl),
      .id = ictx_.GenerateItemId(record_decl),
      .owning_target = std::move(owning_target),
      .template_specialization = std::move(template_specialization),
      .unknown_attr = std::move(*unknown_attr),
      .doc_comment = std::move(doc_comment),
      .bridge_type = std::move(bridge_type),
      .source_loc = ictx_.ConvertSourceLocation(source_loc),
      .unambiguous_public_bases = GetUnambiguousPublicBases(*record_decl),
      .fields = ImportFields(record_decl),
      .size_align =
          {
              .size = layout.getSize().getQuantity(),
              .alignment = layout.getAlignment().getQuantity(),
          },
      .trait_derives = *std::move(trait_derives),
      .is_derived_class = is_derived_class,
      .override_alignment = override_alignment,
      .is_unsafe_type = *is_unsafe_type,
      .copy_constructor = GetCopyCtorSpecialMemberFunc(*record_decl),
      .move_constructor = GetMoveCtorSpecialMemberFunc(*record_decl),
      .destructor = GetDestructorSpecialMemberFunc(*record_decl),
      .is_trivial_abi = record_decl->canPassInRegisters(),
      .is_inheritable = !is_effectively_final,
      .is_abstract = record_decl->isAbstract(),
      .nodiscard = std::move(nodiscard),
      .record_type = *record_type,
      .is_aggregate = record_decl->isAggregate(),
      .is_anon_record_with_typedef = anon_typedef != nullptr,
      .is_explicit_class_template_instantiation_definition =
          is_explicit_class_template_instantiation_definition,
      .child_item_ids = std::move(item_ids),
      .enclosing_item_id = *std::move(enclosing_item_id),
  };

  // If the align attribute was attached to the typedef decl, we should
  // apply it to the generated record.
  //
  // TODO(jeanpierreda): We also need this logic for non-anonymous structs, where we
  // instead copy the struct into a new decl with this typedef's decl id.
  // So this part probably needs to be factored out somewhere that
  // typedef_name.cc can get at it.
  if (anon_typedef != nullptr) {
    auto* aligned = anon_typedef->getAttr<clang::AlignedAttr>();
    if (aligned) {
      int64_t& size = record.size_align.size;
      int64_t& alignment = record.size_align.alignment;
      alignment =
          ictx_.ctx_.toCharUnitsFromBits(aligned->getAlignment(ictx_.ctx_))
              .getQuantity();
      record.override_alignment = true;

      // If it has alignment, update the `record->size` to the aligned
      // one, because that size is going to be used as this record's
      // canonical size in IR and in the binding code.

      // Make sure that `alignment` is a power of 2.
      CHECK(!(alignment & (alignment - 1)));

      // Given that `alignment` is a power of 2, we can round it up by
      // a bit arithmetic: `alignment - 1` clears the single bit of it
      // while turning all the zeros in the right to 1s. Adding
      // `alignment - 1` and doing &~ with it effectively rounds it up
      // to the next multiple of the alignment.
      size = (size + alignment - 1) & ~(alignment - 1);
    }
  }
  return record;
}

std::vector<Field> CXXRecordDeclImporter::ImportFields(
    clang::CXXRecordDecl* record_decl) {
  clang::AccessSpecifier default_access =
      record_decl->isClass() ? clang::AS_private : clang::AS_public;
  std::vector<Field> fields;
  const clang::ASTRecordLayout& layout =
      ictx_.ctx_.getASTRecordLayout(record_decl);
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    clang::AccessSpecifier access = field_decl->getAccess();
    if (access == clang::AS_none) {
      access = default_access;
    }

    const clang::tidy::lifetimes::ValueLifetimes* no_lifetimes = nullptr;
    absl::StatusOr<CcType> type;
    switch (access) {
      case clang::AS_public:
        // TODO(mboehme): Once lifetime_annotations supports retrieving
        // lifetimes in field types, pass these to ConvertQualType().
        type = ictx_.ConvertQualType(field_decl->getType(), no_lifetimes);
        break;
      case clang::AS_protected:
      case clang::AS_private:
      case clang::AS_none:
        // As a performance optimization (i.e. to keep the generated code
        // small) we can emit private fields as opaque blobs of bytes.  This
        // may avoid the need to include supporting types in the generated
        // code (e.g. avoiding extra template instantiations).  See also
        // b/226580208 and <internal link>.
        type = absl::UnavailableError(
            "Types of non-public C++ fields can be elided away");
        break;
    }

    bool is_inheritable = false;
    auto* field_record = field_decl->getType()->getAsCXXRecordDecl();
    if (field_record) {
      // If it is a record as a direct member, its item must be already
      // imported.
      auto item = ictx_.GetImportedItem(field_record);
      if (item.has_value()) {
        if (const auto* record = std::get_if<Record>(&item.value())) {
          is_inheritable = record->is_inheritable;
        }
      }
    }

    uint64_t size;
    if (field_decl->isZeroSize(ictx_.ctx_)) {
      size = 0;
    } else if (field_decl->isBitField()) {
      size = field_decl->getBitWidthValue();
    } else {
      size = ictx_.ctx_.getTypeSize(field_decl->getType());
    }

    fields.push_back(
        {.rust_identifier = GetTranslatedFieldName(field_decl),
         .cpp_identifier = StringRefToOptionalIdentifier(field_decl->getName()),
         .doc_comment = ictx_.GetComment(field_decl),
         .type = std::move(type),
         .access = TranslateAccessSpecifier(access),
         .offset = layout.getFieldOffset(field_decl->getFieldIndex()),
         .size = size,
         .unknown_attr = CollectUnknownAttrs(*field_decl),
         .is_no_unique_address =
             field_decl->hasAttr<clang::NoUniqueAddressAttr>(),
         .is_bitfield = field_decl->isBitField(),
         .is_inheritable = is_inheritable});
  }
  return fields;
}

std::vector<BaseClass> CXXRecordDeclImporter::GetUnambiguousPublicBases(
    const clang::CXXRecordDecl& record_decl) const {
  // This function is unfortunate: the only way to correctly get information
  // about the bases is lookupInBases. It runs a complex O(N^3) algorithm for
  // e.g. correctly determining virtual base paths, etc.
  //
  // However, lookupInBases does not recurse into a class once it's found.
  // So we need to call lookupInBases once per class, making this O(N^4).

  llvm::SmallPtrSet<const clang::CXXRecordDecl*, 4> seen;
  std::vector<BaseClass> bases;
  clang::CXXBasePaths paths;
  // the const cast is a common pattern, apparently, see e.g.
  // https://clang.llvm.org/doxygen/CXXInheritance_8cpp_source.html#l00074
  paths.setOrigin(const_cast<clang::CXXRecordDecl*>(&record_decl));

  auto next_class = [&]() {
    const clang::CXXRecordDecl* found = nullptr;

    // Matches the first new class it encounters (and adds it to `seen`, so
    // that future runs don't rediscover it.)
    auto is_new_class = [&](const clang::CXXBaseSpecifier* base_specifier,
                            clang::CXXBasePath&) {
      const auto* record_decl = base_specifier->getType()->getAsCXXRecordDecl();
      if (found) {
        return record_decl == found;
      }

      if (record_decl && seen.insert(record_decl).second) {
        found = record_decl;
        return true;
      }
      return false;
    };
    return record_decl.lookupInBases(is_new_class, paths);
  };

  for (; next_class(); paths.clear()) {
    for (const clang::CXXBasePath& path : paths) {
      if (path.Access != clang::AS_public) {
        continue;
      }
      const clang::CXXBaseSpecifier& base_specifier =
          *path[path.size() - 1].Base;
      const clang::QualType& base = base_specifier.getType();
      if (paths.isAmbiguous(ictx_.ctx_.getCanonicalType(base))) {
        continue;
      }

      clang::CXXRecordDecl* base_record_decl =
          ABSL_DIE_IF_NULL(base_specifier.getType()->getAsCXXRecordDecl());
      if (!ictx_.HasBeenAlreadySuccessfullyImported(base_record_decl)) {
        continue;
      }

      std::optional<int64_t> offset = {0};
      for (const clang::CXXBasePathElement& base_path_element : path) {
        if (base_path_element.Base->isVirtual()) {
          offset.reset();
          break;
        }
        *offset +=
            ictx_.ctx_.getASTRecordLayout(base_path_element.Class)
                .getBaseClassOffset(ABSL_DIE_IF_NULL(
                    base_path_element.Base->getType()->getAsCXXRecordDecl()))
                .getQuantity();
      }
      CHECK((!offset.has_value() || *offset >= 0) &&
            "Concrete base classes should have non-negative offsets.");
      bases.push_back(
          BaseClass{.base_record_id = ictx_.GenerateItemId(base_record_decl),
                    .offset = offset});
      break;
    }
  }
  return bases;
}

absl::StatusOr<std::optional<BridgeType>>
CXXRecordDeclImporter::GetBuiltinBridgeType(
    const clang::ClassTemplateSpecializationDecl* decl) {
  const clang::CXXRecordDecl* cxx_record_decl =
      decl->getSpecializedTemplate()->getTemplatedDecl();
  const clang::DeclContext* context = cxx_record_decl->getDeclContext();
  bool is_std_namespace = false;
  while (context) {
    if (context->isStdNamespace()) {
      is_std_namespace = true;
      break;
    }
    context = context->getParent();
  }

  if (!is_std_namespace) {
    return std::nullopt;
  }

  clang::StringRef name = cxx_record_decl->getName();
  auto cc_type_of_arg = [&](int index) {
    return ictx_.ConvertQualType(
        /*qual_type=*/decl->getTemplateArgs()[index].getAsType(),
        /*lifetimes=*/nullptr);
  };

  if (name == "optional") {
    CRUBIT_ASSIGN_OR_RETURN(CcType inner, cc_type_of_arg(0));
    return BridgeType{BridgeType::StdOptional{
        .inner_type = std::make_shared<CcType>(std::move(inner)),
    }};
  }

  if (name == "pair") {
    CRUBIT_ASSIGN_OR_RETURN(CcType first, cc_type_of_arg(0));
    CRUBIT_ASSIGN_OR_RETURN(CcType second, cc_type_of_arg(1));
    return BridgeType{BridgeType::StdPair{
        .first_type = std::make_shared<CcType>(std::move(first)),
        .second_type = std::make_shared<CcType>(std::move(second)),
    }};
  }

  if (name == "basic_string") {
    CRUBIT_ASSIGN_OR_RETURN(CcType char_type, cc_type_of_arg(0));
    if (const auto* primitive =
            std::get_if<CcType::Primitive>(&char_type.variant);
        primitive != nullptr && primitive->spelling == "char") {
      return BridgeType{BridgeType::StdString{}};
    }
  }
  // Add builtin bridge types here as needed.

  return std::nullopt;
}

}  // namespace crubit
