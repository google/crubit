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
#include "rs_bindings_from_cc/ir.pb.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
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

std::unique_ptr<ir_proto::Item> crubit::TypeAliasImporter::Import(
    clang::NamedDecl* decl) {
  // Special-case proto enums. We handle them under the alias, rather than
  // the enum declaration, because the enum declaration gives no useful
  // way to obtain the message names that it is a part of unless/until
  // `_` is forbidden in enum identifiers.
  if (auto* alias_decl = clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
    if (ictx_.IsFromProtoTarget(*alias_decl) &&
        alias_decl->getUnderlyingType()->isEnumeralType()) {
      ictx_.MarkAsSuccessfullyImported(*decl);
      auto item = std::make_unique<ir_proto::Item>();
      auto* existing = item->mutable_existing_rust_type();
      existing->set_rs_name(ProtoEnumToRustName(*decl));
      existing->set_cc_name(decl->getQualifiedNameAsString());
      existing->set_unique_name(ictx_.GetUniqueName(*decl));
      existing->set_owning_target(ictx_.GetOwningTarget(*decl).value());
      // To be paranoid, assume Rust proto enums are not ABI compatible.
      existing->set_is_same_abi(false);
      existing->set_id(ictx_.GenerateItemId(*decl).value());
      return item;
    }
  }

  clang::QualType underlying_qualtype;
  if (auto* typedef_name_decl = clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
    if (typedef_name_decl->getAnonDeclWithTypedefName()) {
      // Anonymous declarations with typedefs just incorporate the typedef name
      // into their item, instead of having a separate TypeAlias item in
      // addition.
      return nullptr;
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
      return ImportFunctionAlias(*using_decl, *function_decl);
    }
    auto* target_type = clang::dyn_cast<clang::TypeDecl>(target);
    if (target_type == nullptr) {
      // Not a type.
      return nullptr;
    }
    underlying_qualtype =
        target_type->getASTContext().getTypeDeclType(target_type);
    decl = using_decl;
  } else {
    // Neither a typedef nor a using decl.
    return nullptr;
  }

  absl::StatusOr<TranslatedIdentifier> identifier =
      ictx_.GetTranslatedIdentifier(*decl);
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
      ictx_.AreAssumedLifetimesEnabledForTarget(ictx_.GetOwningTarget(*decl)));

  if (!underlying_type.ok()) {
    return ictx_.ImportUnsupportedItem(
        *decl, (*identifier).cc_identifier, *enclosing_item_id,
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
  if (!ictx_.IsFromCurrentTarget(*decl) && !is_cc_template_instantiation) {
    const clang::CXXRecordDecl* record_decl =
        underlying_qualtype->getAsCXXRecordDecl();
    if (auto* tst =
            underlying_qualtype->getAs<clang::TemplateSpecializationType>()) {
      if (record_decl == nullptr) record_decl = tst->getAsCXXRecordDecl();
    }
    if (record_decl != nullptr && ictx_.RefersToOwnedDefinition(*record_decl)) {
      return ictx_.ImportUnsupportedItem(
          *decl, (*identifier).cc_identifier, *enclosing_item_id,
          {FormattedError::Static(
              "Cannot import across-target type alias because its underlying "
              "type requires a record definition owned by a downstream "
              "target")});
    }
  }
  ictx_.MarkAsSuccessfullyImported(*decl);

  std::optional<std::string> deprecated;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*decl, [&](const clang::Attr& attr) {
        if (auto* deprecated_attr =
                clang::dyn_cast<clang::DeprecatedAttr>(&attr)) {
          deprecated.emplace(deprecated_attr->getMessage());
          return true;
        } else if (clang::isa<clang::VisibilityAttr>(attr)) {
          // Visibility attributes on typedef/using declarations, from
          // export macros for example, do not affect Rust type aliases,
          // but Clang still creates the AST attribute node.
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItem(
        *decl, (*identifier).cc_identifier, *enclosing_item_id,
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

  auto item = std::make_unique<ir_proto::Item>();
  auto* type_alias = item->mutable_type_alias();
  type_alias->mutable_cc_name()->set_identifier(
      identifier->cc_identifier.Ident());
  type_alias->mutable_rs_name()->set_identifier(rs_name);
  type_alias->set_unique_name(ictx_.GetUniqueName(*decl));
  type_alias->set_id(ictx_.GenerateItemId(*decl).value());
  type_alias->set_owning_target(ictx_.GetOwningTarget(*decl).value());
  if (auto doc = ictx_.GetComment(*decl); doc.has_value()) {
    type_alias->set_doc_comment(std::move(*doc));
  }
  if (unknown_attr->has_value()) {
    type_alias->set_unknown_attr(std::move(**unknown_attr));
  }
  underlying_type->WriteToProto(*type_alias->mutable_underlying_type());
  type_alias->set_source_loc(
      ictx_.ConvertSourceLocation(decl->getBeginLoc(), nullptr));
  if (enclosing_item_id->has_value()) {
    type_alias->set_enclosing_item_id((*enclosing_item_id)->value());
  }
  if (deprecated.has_value()) {
    type_alias->set_deprecated(std::move(*deprecated));
  }
  return item;
}

std::unique_ptr<ir_proto::Item> TypeAliasImporter::ImportFunctionAlias(
    clang::UsingShadowDecl& using_decl, clang::FunctionDecl& function_decl) {
  // `using Base::Method;` inside a class. Binding these requires adjusting the
  // `this` type of the method to the derived class, which is not done yet.
  if (clang::isa<clang::CXXMethodDecl>(function_decl)) {
    return ictx_.ImportUnsupportedItem(
        using_decl, std::nullopt,
        {FormattedError::Static(
            "Using-declarations of member functions are not yet supported.")});
  }
  if (!ictx_.IsFromCurrentTarget(using_decl)) {
    return nullptr;
  }
  // `FunctionDeclImporter` only imports functions from the current target, so
  // report a specific error rather than silently dropping the alias.
  if (!ictx_.IsFromCurrentTarget(function_decl)) {
    return ictx_.ImportUnsupportedItem(
        using_decl, std::nullopt,
        {FormattedError::Static("Using-declarations of functions from other "
                                "targets are not yet supported.")});
  }

  absl::StatusOr<std::optional<ItemId>> enclosing_item_id =
      ictx_.GetEnclosingItemId(&using_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        using_decl, std::nullopt,
        {FormattedError::FromStatus(std::move(enclosing_item_id).status())});
  }

  // Import the function itself, and then re-home the resulting item onto the
  // using-declaration. The name is intentionally kept: a using-declaration
  // cannot rename, and the target's name already accounts for
  // `CRUBIT_RUST_NAME`. Two items with the same mangled name may exist in the
  // IR afterwards (the function and its alias); bindings generation handles
  // that by generating a distinct C++ thunk for each.
  std::unique_ptr<ir_proto::Item> item = ictx_.ImportDecl(&function_decl);
  if (item == nullptr) {
    return nullptr;
  }
  const std::string unique_name = ictx_.GetUniqueName(using_decl);
  const ItemId id = ictx_.GenerateItemId(using_decl);
  const std::string source_loc =
      ictx_.ConvertSourceLocation(using_decl.getBeginLoc(), nullptr);
  const BazelLabel owning_target = ictx_.GetOwningTarget(using_decl);

  if (item->has_unsupported_item()) {
    ir_proto::UnsupportedItem* unsupported = item->mutable_unsupported_item();
    unsupported->set_name(using_decl.getQualifiedNameAsString());
    unsupported->set_unique_name(unique_name);
    unsupported->set_id(id.value());
    unsupported->set_source_loc(source_loc);
    unsupported->set_defining_target(owning_target.value());
    if (unsupported->has_path()) {
      if (enclosing_item_id->has_value()) {
        unsupported->mutable_path()->set_enclosing_item_id(
            (*enclosing_item_id)->value());
      } else {
        unsupported->mutable_path()->clear_enclosing_item_id();
      }
    }
    return item;
  }

  CHECK(item->has_func()) << "Importing a FunctionDecl should produce a Func";
  ir_proto::Func* func = item->mutable_func();
  func->set_unique_name(unique_name);
  func->set_id(id.value());
  func->set_source_loc(source_loc);
  func->set_owning_target(owning_target.value());
  if (enclosing_item_id->has_value()) {
    func->set_enclosing_item_id((*enclosing_item_id)->value());
  } else {
    func->clear_enclosing_item_id();
  }
  // A hidden friend becomes reachable by qualified name through the alias.
  func->clear_adl_enclosing_record();
  // Comments are attached to the written `using` declaration (the introducer),
  // not to the implicit shadow declaration.
  if (std::optional<std::string> doc =
          ictx_.GetComment(*using_decl.getIntroducer());
      doc.has_value()) {
    func->set_doc_comment(*std::move(doc));
  }
  return item;
}

}  // namespace crubit