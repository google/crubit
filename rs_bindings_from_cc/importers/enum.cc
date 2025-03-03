// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/enum.h"

#include <optional>
#include <utility>
#include <vector>

#include "absl/algorithm/container.h"
#include "absl/status/statusor.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"

namespace crubit {

std::optional<IR::Item> EnumDeclImporter::Import(clang::EnumDecl* enum_decl) {
  if (enum_decl->getName().empty()) {
    // TODO(b/208945197): This corresponds to an unnamed enum declaration like
    // `enum { kFoo = 1 }`, which only exists to provide constants into the
    // surrounding scope and doesn't actually introduce an enum namespace. It
    // seems like it should probably be handled with other constants.
    return ictx_.ImportUnsupportedItem(
        enum_decl, UnsupportedItem::Kind::kUnnameable, std::nullopt,
        FormattedError::Static("Unnamed enums are not supported yet"));
  }
  absl::StatusOr<TranslatedIdentifier> enum_name =
      ictx_.GetTranslatedIdentifier(enum_decl);
  if (!enum_name.ok()) {
    return ictx_.ImportUnsupportedItem(
        enum_decl, UnsupportedItem::Kind::kType, std::nullopt,
        FormattedError::PrefixedStrCat("Enum name is not supported",
                                       enum_name.status().message()));
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(enum_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        enum_decl, UnsupportedItem::Kind::kType, std::nullopt,
        FormattedError::FromStatus(std::move(enclosing_item_id.status())));
  }

  // Reports an unsupported enum with the given error.
  //
  // This is preferred to invoking `ImportUnsupportedItem` directly because it
  // ensures that the path is set correctly. Note that this cannot be used above
  // because the enclosing item ID and translated name are not yet available.
  auto unsupported = [this, &enum_name, &enclosing_item_id,
                      enum_decl](FormattedError error) {
    return ictx_.ImportUnsupportedItem(
        enum_decl, UnsupportedItem::Kind::kType,
        UnsupportedItem::Path{.ident = (*enum_name).rs_identifier(),
                              .enclosing_item_id = *enclosing_item_id},
        error);
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
  absl::StatusOr<MappedType> type =
      ictx_.ConvertQualType(cpp_type, no_lifetimes, std::nullopt);
  if (!type.ok()) {
    return unsupported(FormattedError::FromStatus(std::move(type.status())));
  }

  std::vector<Enumerator> enumerators;
  enumerators.reserve(absl::c_distance(enum_decl->enumerators()));
  for (clang::EnumConstantDecl* enumerator : enum_decl->enumerators()) {
    absl::StatusOr<TranslatedIdentifier> enumerator_name =
        ictx_.GetTranslatedIdentifier(enumerator);
    if (!enumerator_name.ok()) {
      // It's not clear that this case is possible
      return unsupported(
          FormattedError::PrefixedStrCat("Enumerator name is not supported",
                                         enumerator_name.status().message()));
    }

    enumerators.push_back(Enumerator{
        .identifier = (*enumerator_name).rs_identifier(),
        .value = IntegerConstant(enumerator->getInitVal()),
        .unknown_attr = CollectUnknownAttrs(*enumerator),
    });
  }

  ictx_.MarkAsSuccessfullyImported(enum_decl);
  return Enum{
      .cc_name = (*enum_name).cc_identifier,
      .rs_name = (*enum_name).rs_identifier(),
      .id = ictx_.GenerateItemId(enum_decl),
      .owning_target = ictx_.GetOwningTarget(enum_decl),
      .source_loc = ictx_.ConvertSourceLocation(enum_decl->getBeginLoc()),
      .underlying_type = *std::move(type),
      .enumerators = enum_decl->isCompleteDefinition()
                         ? std::make_optional(std::move(enumerators))
                         : std::nullopt,
      .unknown_attr = CollectUnknownAttrs(*enum_decl),
      .enclosing_item_id = *std::move(enclosing_item_id),
  };
}

}  // namespace crubit
