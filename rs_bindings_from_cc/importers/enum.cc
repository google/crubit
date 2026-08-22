// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/enum.h"

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/algorithm/container.h"
#include "absl/status/statusor.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclarationName.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"

namespace crubit {

std::unique_ptr<ir_proto::Item> EnumDeclImporter::Import(
    clang::EnumDecl* enum_decl) {
  if (enum_decl->getName().empty()) {
    // Anonymous enums are handled by `EnumConstantDeclImporter`.
    return nullptr;
  }
  absl::StatusOr<TranslatedIdentifier> enum_name =
      ictx_.GetTranslatedIdentifier(*enum_decl);
  if (!enum_name.ok()) {
    return ictx_.ImportUnsupportedItem(
        *enum_decl, std::nullopt,
        {FormattedError::PrefixedStrCat("Enum name is not supported",
                                        enum_name.status().message())});
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(enum_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        *enum_decl, std::nullopt,
        {FormattedError::FromStatus(std::move(enclosing_item_id.status()))});
  }

  // Reports an unsupported enum with the given error.
  //
  // This is preferred to invoking `ImportUnsupportedItem` directly because it
  // ensures that the path is set correctly. Note that this cannot be used above
  // because the enclosing item ID and translated name are not yet available.
  auto unsupported = [this, &enum_name = *enum_name,
                      &enclosing_item_id = *enclosing_item_id,
                      enum_decl](FormattedError error) {
    return ictx_.ImportUnsupportedItem(*enum_decl, enum_name.rs_identifier(),
                                       enclosing_item_id, {std::move(error)});
  };

  clang::QualType cpp_type = enum_decl->getIntegerType();
  if (cpp_type.isNull()) {
    // According to https://clang.llvm.org/doxygen/classclang_1_1EnumDecl.html,
    // getIntegerType "returns a null QualType for an enum forward definition
    // with no fixed underlying type." The same page implies that this can't
    // occur in C++ nor in standard C, but clang supports enums like this
    // in C "as an extension".
    return unsupported(
        FormattedError::Static("Forward declared enums without type "
                               "specifiers are not supported"));
  }
  const clang::tidy::lifetimes::ValueLifetimes* no_lifetimes = nullptr;
  absl::StatusOr<CcType> type =
      ictx_.ConvertQualType(cpp_type, no_lifetimes, /*nullable=*/true,
                            ictx_.AreAssumedLifetimesEnabledForTarget(
                                ictx_.GetOwningTarget(*enum_decl)));
  if (!type.ok()) {
    return unsupported(FormattedError::FromStatus(std::move(type.status())));
  }

  std::vector<ir_proto::Enumerator> enumerators;
  enumerators.reserve(absl::c_distance(enum_decl->enumerators()));
  for (clang::EnumConstantDecl* enumerator : enum_decl->enumerators()) {
    absl::StatusOr<TranslatedIdentifier> enumerator_name =
        ictx_.GetTranslatedIdentifier(*enumerator);
    if (!enumerator_name.ok()) {
      // It's not clear that this case is possible
      return unsupported(
          FormattedError::PrefixedStrCat("Enumerator name is not supported",
                                         enumerator_name.status().message()));
    }

    std::optional<std::string> deprecated;
    absl::StatusOr<std::optional<std::string>> unknown_attr =
        CollectUnknownAttrs(*enumerator, [&](const clang::Attr& attr) {
          if (auto* deprecated_attr =
                  clang::dyn_cast<clang::DeprecatedAttr>(&attr)) {
            deprecated.emplace(deprecated_attr->getMessage());
            return true;
          }
          return false;
        });
    if (!unknown_attr.ok()) {
      return unsupported(
          FormattedError::FromStatus(std::move(unknown_attr.status())));
    }

    absl::StatusOr<IntegerConstant> value =
        IntegerConstant::FromAPValue(enumerator->getInitVal());
    if (!value.ok()) {
      return unsupported(FormattedError::FromStatus(std::move(value.status())));
    }
    ir_proto::Enumerator proto_enum_val;
    proto_enum_val.mutable_identifier()->set_identifier(
        (*enumerator_name).rs_identifier().Ident());
    value->WriteToProto(*proto_enum_val.mutable_value());
    if (unknown_attr->has_value()) {
      proto_enum_val.set_unknown_attr(std::move(**unknown_attr));
    }
    if (deprecated.has_value()) {
      proto_enum_val.set_deprecated(std::move(*deprecated));
    }
    if (auto doc = ictx_.GetComment(*enumerator); doc.has_value()) {
      proto_enum_val.set_doc_comment(std::move(*doc));
    }
    enumerators.push_back(std::move(proto_enum_val));
  }

  std::optional<std::string> nodiscard;
  std::optional<std::string> deprecated;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*enum_decl, [&](const clang::Attr& attr) {
        if (auto* unused_attr =
                clang::dyn_cast<clang::WarnUnusedResultAttr>(&attr)) {
          nodiscard.emplace(unused_attr->getMessage());
          return true;
        } else if (auto* deprecated_attr =
                       clang::dyn_cast<clang::DeprecatedAttr>(&attr)) {
          deprecated.emplace(deprecated_attr->getMessage());
          return true;
        } else if (clang::isa<clang::VisibilityAttr>(attr)) {
          // Visibility attributes on enums do not affect Rust enum bindings.
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return unsupported(
        FormattedError::FromStatus(std::move(unknown_attr.status())));
  }

  if (ictx_.IsFromProtoTarget(*enum_decl)) {
    // Supporting a top-level `Foo_Bar_Baz` enum is hard! It could be any of
    // these four things:
    // * A top-level `enum Foo_Bar_Baz`
    // * A nested `message Foo { enum Bar_Baz }`
    // * A differently nested `message Foo_Bar { enum Baz }`
    // * A deeply nested `message Foo { message Bar { message Baz } }`
    //
    // There is no signal on the enum itself to distinguish, so we would need to
    // iterate over all the records in the header file to find any aliases that
    // refer to this enum, and from that, learn the name.
    //
    // At least for now, we're going to forgo that exercise. If the name does
    // not contain an underscore, then we know it's the first case.
    // If the name does contain an underscore, but is retrieved via the alias,
    // then we can know the case above perfectly, and can handle this
    // in type_alias.cc. But if the name contains an underscore, and is
    // accessed at the top level: give up!
    if (enum_decl->getName().contains('_')) {
      return unsupported(FormattedError::Static(
          "b/406221412: Proto enums with underscores are not supported "
          "except via Message::Enum syntax."));
    }
    ictx_.MarkAsSuccessfullyImported(*enum_decl);
    auto item = std::make_unique<ir_proto::Item>();
    auto* existing = item->mutable_existing_rust_type();
    existing->set_rs_name(std::string(enum_decl->getName()));
    existing->set_cc_name(enum_decl->getQualifiedNameAsString());
    existing->set_unique_name(ictx_.GetUniqueName(*enum_decl));
    existing->set_owning_target(ictx_.GetOwningTarget(*enum_decl).value());
    existing->set_is_same_abi(false);
    existing->set_id(ictx_.GenerateItemId(*enum_decl).value());
    return item;
  }

  BazelLabel owning_target = ictx_.GetOwningTarget(*enum_decl);
  absl::StatusOr<bool> detected_formatter = ictx_.DetectFormatter(*enum_decl);
  if (!detected_formatter.ok()) {
    return unsupported(
        FormattedError::FromStatus(std::move(detected_formatter).status()));
  }

  std::optional<std::string> doc_comment = ictx_.GetComment(*enum_decl);

  ictx_.MarkAsSuccessfullyImported(*enum_decl);
  clang::DeclarationNameInfo name_info(enum_decl->getDeclName(),
                                       enum_decl->getLocation());

  auto item = std::make_unique<ir_proto::Item>();
  auto* proto_enum = item->mutable_enum_decl();
  proto_enum->mutable_cc_name()->set_identifier(
      (*enum_name).cc_identifier.Ident());
  proto_enum->mutable_rs_name()->set_identifier(
      (*enum_name).rs_identifier().Ident());
  proto_enum->set_unique_name(ictx_.GetUniqueName(*enum_decl));
  proto_enum->set_mangled_cc_name(ictx_.GetMangledName(*enum_decl));
  proto_enum->set_id(ictx_.GenerateItemId(*enum_decl).value());
  proto_enum->set_owning_target(std::move(owning_target).value());
  proto_enum->set_source_loc(
      ictx_.ConvertSourceLocation(enum_decl->getBeginLoc(), &name_info));
  type->WriteToProto(*proto_enum->mutable_underlying_type());
  if (enum_decl->isCompleteDefinition()) {
    for (auto& enumerator : enumerators) {
      *proto_enum->add_enumerators() = std::move(enumerator);
    }
  } else {
    // Forward declared enums must be distinguished from complete empty enums.
    proto_enum->set_is_incomplete(true);
  }
  if (unknown_attr->has_value()) {
    proto_enum->set_unknown_attr(std::move(**unknown_attr));
  }
  if (enclosing_item_id->has_value()) {
    proto_enum->set_enclosing_item_id((*enclosing_item_id)->value());
  }
  proto_enum->set_detected_formatter(*detected_formatter);
  if (nodiscard.has_value()) {
    proto_enum->set_nodiscard(std::move(*nodiscard));
  }
  if (deprecated.has_value()) {
    proto_enum->set_deprecated(std::move(*deprecated));
  }
  if (doc_comment.has_value()) {
    proto_enum->set_doc_comment(std::move(*doc_comment));
  }
  return item;
}

}  // namespace crubit
