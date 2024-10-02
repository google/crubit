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
        enum_decl,
        FormattedError::Static("Unnamed enums are not supported yet"));
  }
  absl::StatusOr<Identifier> enum_name =
      ictx_.GetTranslatedIdentifier(enum_decl);
  if (!enum_name.ok()) {
    return ictx_.ImportUnsupportedItem(
        enum_decl,
        FormattedError::PrefixedStrCat("Enum name is not supported",
                                       enum_name.status().message()));
  }

  clang::QualType cpp_type = enum_decl->getIntegerType();
  if (cpp_type.isNull()) {
    // According to https://clang.llvm.org/doxygen/classclang_1_1EnumDecl.html,
    // getIntegerType "returns a null QualType for an enum forward definition
    // with no fixed underlying type." The same page implies that this can't
    // occur in C++ nor in standard C, but clang supports enums like this
    // in C "as an extension".
    return ictx_.ImportUnsupportedItem(
        enum_decl, FormattedError::Static("Forward declared enums without type "
                                          "specifiers are not supported"));
  }
  const clang::tidy::lifetimes::ValueLifetimes* no_lifetimes = nullptr;
  absl::StatusOr<MappedType> type =
      ictx_.ConvertQualType(cpp_type, no_lifetimes, std::nullopt);
  if (!type.ok()) {
    return ictx_.ImportUnsupportedItem(
        enum_decl, FormattedError::FromStatus(std::move(type.status())));
  }

  std::vector<Enumerator> enumerators;
  enumerators.reserve(absl::c_distance(enum_decl->enumerators()));
  for (clang::EnumConstantDecl* enumerator : enum_decl->enumerators()) {
    absl::StatusOr<Identifier> enumerator_name =
        ictx_.GetTranslatedIdentifier(enumerator);
    if (!enumerator_name.ok()) {
      // It's not clear that this case is possible
      return ictx_.ImportUnsupportedItem(
          enum_decl,
          FormattedError::PrefixedStrCat("Enumerator name is not supported",
                                         enumerator_name.status().message()));
    }

    enumerators.push_back(Enumerator{
        .identifier = *enumerator_name,
        .value = IntegerConstant(enumerator->getInitVal()),
        .unknown_attr = CollectUnknownAttrs(*enumerator),
    });
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(enum_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        enum_decl,
        FormattedError::FromStatus(std::move(enclosing_item_id.status())));
  }

  ictx_.MarkAsSuccessfullyImported(enum_decl);
  return Enum{
      .identifier = *enum_name,
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
