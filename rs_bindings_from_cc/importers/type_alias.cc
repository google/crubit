// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/type_alias.h"

#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/algorithm/container.h"
#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/status/statusor.h"
#include "absl/strings/ascii.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "absl/strings/string_view.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "llvm/Support/MD5.h"

namespace crubit {
namespace {

std::string ProtoMessageToRustModName(absl::string_view message_name) {
  std::string mod_name = "";
  for (char c : message_name) {
    // Underscore in MessageName implies nested item which maps to a nested
    // namespace in Rust.
    // If someone actually used an underscore in a proto message name, this
    // heuristic will fail. Whoops.
    if (c == '_' && !mod_name.empty()) {
      absl::StrAppend(&mod_name, "::");
      continue;
    } else if (absl::ascii_isupper(c) && !mod_name.empty() &&
               mod_name.back() != ':') {
      absl::StrAppend(&mod_name, "_");
    }
    // StrAppend doesn't accept `char`. :'(
    c = absl::ascii_tolower(static_cast<unsigned char>(c));
    absl::StrAppend(&mod_name, absl::string_view(&c, 1));
  }
  return mod_name;
}

// Returns the relative name of a Rust Proto Enum corresponding to the
// C++ Proto Enum. For example, the C++ enum `MyMessage::MyEnum` would become
// `my_message::MyEnum`.
std::string ProtoEnumToRustName(clang::NamedDecl& decl) {
  std::vector<std::string> mod_chain;
  for (clang::DeclContext* decl_context = decl.getDeclContext();
       decl_context->isRecord(); decl_context = decl_context->getParent()) {
    auto* record_decl = clang::dyn_cast<clang::RecordDecl>(decl_context);

    mod_chain.push_back(ProtoMessageToRustModName(record_decl->getName()));
  }
  absl::c_reverse(mod_chain);
  mod_chain.push_back(std::string(decl.getName()));
  return absl::StrJoin(mod_chain, "::");
}
}  // namespace

std::optional<IR::Item> crubit::TypeAliasImporter::Import(
    clang::NamedDecl* decl) {
  // Special-case proto enums. We handle them under the alias, rather than
  // the enum declaration, because the enum declaration gives no useful
  // way to obtain the message names that it is a part of unless/until
  // `_` is forbidden in enum identifiers.
  if (auto* alias_decl = clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
    if (ictx_.IsFromProtoTarget(*alias_decl) &&
        alias_decl->getUnderlyingType()->isEnumeralType()) {
      ictx_.MarkAsSuccessfullyImported(decl);
      return ExistingRustType{
          .rs_name = ProtoEnumToRustName(*decl),
          .cc_name = decl->getQualifiedNameAsString(),
          .unique_name = ictx_.GetUniqueName(*decl),
          .owning_target = ictx_.GetOwningTarget(decl),
          .size_align = std::nullopt,
          // To be paranoid, assume Rust proto enums are not ABI compatible.
          .is_same_abi = false,
          .id = ictx_.GenerateItemId(decl),
      };
    }
  }

  clang::QualType underlying_qualtype;
  if (auto* typedef_name_decl = clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
    if (typedef_name_decl->getAnonDeclWithTypedefName()) {
      // Anonymous declarations with typedefs just incorporate the typedef name
      // into their item, instead of having a separate TypeAlias item in
      // addition.
      return std::nullopt;
    }
    underlying_qualtype = typedef_name_decl->getUnderlyingType();
    if (const clang::TagDecl* tag_decl =
            StripCStyleNameIntroducingTypedef(typedef_name_decl)) {
      return ictx_.ImportUnsupportedItem(
          *decl, std::nullopt,
          {FormattedError::Static(
              "Typedef only used to introduce a name in C. Not importing.")});
    }
  } else if (auto* using_decl = clang::dyn_cast<clang::UsingShadowDecl>(decl)) {
    clang::NamedDecl* target = using_decl->getTargetDecl();
    if (auto* function_decl = clang::dyn_cast<clang::FunctionDecl>(target)) {
      return ictx_.ImportUnsupportedItem(
          *decl, std::nullopt,
          {FormattedError::Static("Function aliases are not yet supported.")});
    }
    auto* target_type = clang::dyn_cast<clang::TypeDecl>(target);
    if (target_type == nullptr) {
      // Not a type.
      return std::nullopt;
    }
    underlying_qualtype =
        target_type->getASTContext().getTypeDeclType(target_type);
    decl = using_decl;
  } else {
    // Neither a typedef nor a using decl.
    return std::nullopt;
  }

  absl::StatusOr<TranslatedIdentifier> identifier =
      ictx_.GetTranslatedIdentifier(decl);
  if (!identifier.ok()) {
    return ictx_.ImportUnsupportedItem(
        *decl, std::nullopt,
        {FormattedError::PrefixedStrCat("Type alias name is not supported",
                                        identifier.status().message())});
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        *decl, std::nullopt,
        {FormattedError::FromStatus(std::move(enclosing_item_id.status()))});
  }

  clang::tidy::lifetimes::ValueLifetimes* no_lifetimes = nullptr;
  // TODO(mboehme): Once lifetime_annotations supports retrieving lifetimes in
  // type aliases, pass these to ConvertQualType().
  absl::StatusOr<CcType> underlying_type = ictx_.ConvertQualType(
      underlying_qualtype, no_lifetimes, /*nullable=*/true,
      ictx_.AreAssumedLifetimesEnabledForTarget(ictx_.GetOwningTarget(decl)));

  if (!underlying_type.ok()) {
    return ictx_.ImportUnsupportedItem(
        *decl,
        UnsupportedItem::Path{.ident = (*identifier).cc_identifier,
                              .enclosing_item_id = *enclosing_item_id},
        {FormattedError::FromStatus(std::move(underlying_type.status()))});
  }
  bool is_cc_template_instantiation = false;
  if (const auto* ns_decl =
          clang::dyn_cast<clang::NamespaceDecl>(decl->getDeclContext())) {
    // Explicitly support `cc_template!`: see support/cc_template/cc_template.rs
    if (ns_decl->getName() == "__cc_template_instantiations") {
      is_cc_template_instantiation = true;
    }
  }

  // TODO(zarko): This check is overly conservative; we should see if we can
  // come up with a better way to filter out problematic cases where we disagree
  // about completeness in different targets, or we could also decide that this
  // is not a valid way to use the tool and require that users include the
  // necessary headers to get proper output.
  if (!ictx_.IsFromCurrentTarget(decl) && !is_cc_template_instantiation) {
    const clang::CXXRecordDecl* record_decl =
        underlying_qualtype->getAsCXXRecordDecl();
    if (auto* tst =
            underlying_qualtype->getAs<clang::TemplateSpecializationType>()) {
      if (record_decl == nullptr) record_decl = tst->getAsCXXRecordDecl();
    }
    if (record_decl != nullptr && ictx_.RefersToOwnedDefinition(record_decl)) {
      return ictx_.ImportUnsupportedItem(
          *decl,
          UnsupportedItem::Path{.ident = (*identifier).cc_identifier,
                                .enclosing_item_id = *enclosing_item_id},
          {FormattedError::Static(
              "Cannot import across-target type alias because its underlying "
              "type requires a record definition owned by a downstream "
              "target")});
    }
  }
  ictx_.MarkAsSuccessfullyImported(decl);

  std::optional<std::string> deprecated;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*decl, [&](const clang::Attr& attr) {
        if (auto* deprecated_attr =
                clang::dyn_cast<clang::DeprecatedAttr>(&attr)) {
          deprecated.emplace(deprecated_attr->getMessage());
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItem(
        *decl,
        UnsupportedItem::Path{.ident = (*identifier).cc_identifier,
                              .enclosing_item_id = *enclosing_item_id},
        {FormattedError::FromStatus(std::move(unknown_attr.status()))});
  }

  std::string rs_name = std::string(identifier->rs_identifier().Ident());
  if (rs_name.size() > 160) {
    // rustdoc generates filenames using these names, so they can't be too long.
    // Preserve the prefix of the name for debugging. The hash must be stable
    // across runs. (llvm::MD5Hash returns the lower 64 bits of the MD5 hash,
    // and we need some extra headroom for the full file name.)
    rs_name = absl::StrCat(rs_name.substr(0, 160 - 17), "_",
                           absl::Hex(llvm::MD5Hash(rs_name)));
  }

  return TypeAlias{
      .cc_name = identifier->cc_identifier,
      .rs_name = Identifier(rs_name),
      .unique_name = ictx_.GetUniqueName(*decl),
      .id = ictx_.GenerateItemId(decl),
      .owning_target = ictx_.GetOwningTarget(decl),
      .doc_comment = ictx_.GetComment(decl),
      .unknown_attr = std::move(*unknown_attr),
      .underlying_type = *underlying_type,
      .source_loc = ictx_.ConvertSourceLocation(decl->getBeginLoc(), nullptr),
      .enclosing_item_id = *std::move(enclosing_item_id),
      .deprecated = std::move(deprecated),
  };
}

absl::StatusOr<std::unique_ptr<ir_proto::Item>>
TypeAliasImporter::ImportToProto(clang::NamedDecl* decl) {
  clang::QualType underlying_qualtype;
  if (const auto* typedef_name_decl =
          clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
    underlying_qualtype = typedef_name_decl->getUnderlyingType();
  } else if (const auto* using_decl =
                 clang::dyn_cast<clang::UsingShadowDecl>(decl)) {
    const auto* target_type =
        clang::dyn_cast<clang::TypeDecl>(using_decl->getTargetDecl());
    if (target_type == nullptr) {
      return nullptr;
    }
    underlying_qualtype =
        target_type->getASTContext().getTypeDeclType(target_type);
    decl = const_cast<clang::UsingShadowDecl*>(using_decl);
  } else {
    return nullptr;
  }

  absl::StatusOr<TranslatedIdentifier> identifier =
      ictx_.GetTranslatedIdentifier(decl);
  if (!identifier.ok()) {
    return ictx_.ImportUnsupportedItemToProto(
        *decl, std::nullopt,
        {FormattedError::PrefixedStrCat("Type alias name is not supported",
                                        identifier.status().message())});
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItemToProto(
        *decl, std::nullopt,
        {FormattedError::FromStatus(std::move(enclosing_item_id.status()))});
  }

  clang::tidy::lifetimes::ValueLifetimes* no_lifetimes = nullptr;
  absl::StatusOr<CcType> underlying_type = ictx_.ConvertQualType(
      underlying_qualtype, no_lifetimes, /*nullable=*/true,
      ictx_.AreAssumedLifetimesEnabledForTarget(ictx_.GetOwningTarget(decl)));

  if (!underlying_type.ok()) {
    return ictx_.ImportUnsupportedItemToProto(
        *decl,
        UnsupportedItem::Path{.ident = (*identifier).cc_identifier,
                              .enclosing_item_id = *enclosing_item_id},
        {FormattedError::FromStatus(std::move(underlying_type.status()))});
  }
  bool is_cc_template_instantiation = false;
  if (const auto* ns_decl =
          clang::dyn_cast<clang::NamespaceDecl>(decl->getDeclContext())) {
    if (ns_decl->getName() == "__cc_template_instantiations") {
      is_cc_template_instantiation = true;
    }
  }

  if (!ictx_.IsFromCurrentTarget(decl) && !is_cc_template_instantiation) {
    const clang::CXXRecordDecl* record_decl =
        underlying_qualtype->getAsCXXRecordDecl();
    if (auto* tst =
            underlying_qualtype->getAs<clang::TemplateSpecializationType>()) {
      if (record_decl == nullptr) record_decl = tst->getAsCXXRecordDecl();
    }
    if (record_decl != nullptr && ictx_.RefersToOwnedDefinition(record_decl)) {
      return ictx_.ImportUnsupportedItemToProto(
          *decl,
          UnsupportedItem::Path{.ident = (*identifier).cc_identifier,
                                .enclosing_item_id = *enclosing_item_id},
          {FormattedError::Static(
              "Cannot import across-target type alias because its underlying "
              "type requires a record definition owned by a downstream "
              "target")});
    }
  }
  ictx_.MarkAsSuccessfullyImported(decl);

  std::optional<std::string> deprecated;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*decl, [&](const clang::Attr& attr) {
        if (auto* deprecated_attr =
                clang::dyn_cast<clang::DeprecatedAttr>(&attr)) {
          deprecated.emplace(deprecated_attr->getMessage());
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItemToProto(
        *decl,
        UnsupportedItem::Path{.ident = (*identifier).cc_identifier,
                              .enclosing_item_id = *enclosing_item_id},
        {FormattedError::FromStatus(std::move(unknown_attr.status()))});
  }

  std::string rs_name = std::string(identifier->rs_identifier().Ident());
  if (rs_name.size() > 160) {
    rs_name = absl::StrCat(rs_name.substr(0, 160 - 17), "_",
                           absl::Hex(llvm::MD5Hash(rs_name)));
  }

  auto item = std::make_unique<ir_proto::Item>();
  ir_proto::TypeAlias* type_alias = item->mutable_type_alias();

  *type_alias->mutable_cc_name() = identifier->cc_identifier.ToFlatProto();
  *type_alias->mutable_rs_name() = Identifier(rs_name).ToFlatProto();
  type_alias->set_unique_name(ictx_.GetUniqueName(*decl));
  type_alias->set_id(ictx_.GenerateItemId(decl).value());
  type_alias->set_owning_target(ictx_.GetOwningTarget(decl).value());
  if (auto doc_comment = ictx_.GetComment(decl)) {
    type_alias->set_doc_comment(*doc_comment);
  }
  if (unknown_attr->has_value()) {
    type_alias->set_unknown_attr(**unknown_attr);
  }
  *type_alias->mutable_underlying_type() = underlying_type->ToFlatProto();
  type_alias->set_source_loc(
      ictx_.ConvertSourceLocation(decl->getBeginLoc(), nullptr));
  if (enclosing_item_id->has_value()) {
    type_alias->set_enclosing_item_id((*enclosing_item_id)->value());
  }
  type_alias->set_must_bind(must_bind_);
  if (deprecated) {
    type_alias->set_deprecated(*deprecated);
  }

  return item;
}

}  // namespace crubit