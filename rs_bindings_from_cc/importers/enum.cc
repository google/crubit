// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/enum.h"

namespace crubit {

std::optional<IR::Item> EnumDeclImporter::Import(clang::EnumDecl* enum_decl) {
  std::optional<Identifier> enum_name =
      ictx_.GetTranslatedIdentifier(enum_decl);
  if (!enum_name.has_value()) {
    // TODO(b/208945197): This corresponds to an unnamed enum declaration like
    // `enum { kFoo = 1 }`, which only exists to provide constants into the
    // surrounding scope and doesn't actually introduce an enum namespace. It
    // seems like it should probably be handled with other constants.
    return ictx_.ImportUnsupportedItem(enum_decl,
                                       "Unnamed enums are not supported yet");
  }

  clang::QualType cc_type = enum_decl->getIntegerType();
  if (cc_type.isNull()) {
    // According to https://clang.llvm.org/doxygen/classclang_1_1EnumDecl.html,
    // getIntegerType "returns a null QualType for an enum forward definition
    // with no fixed underlying type." The same page implies that this can't
    // occur in C++ nor in standard C, but clang supports enums like this
    // in C "as an extension".
    return ictx_.ImportUnsupportedItem(
        enum_decl,
        "Forward declared enums without type specifiers are not supported");
  }
  std::optional<clang::tidy::lifetimes::ValueLifetimes> no_lifetimes;
  absl::StatusOr<MappedType> type =
      ictx_.ConvertQualType(cc_type, no_lifetimes);
  if (!type.ok()) {
    return ictx_.ImportUnsupportedItem(enum_decl, type.status().ToString());
  }

  std::vector<Enumerator> enumerators;
  enumerators.reserve(std::distance(enum_decl->enumerators().begin(),
                                    enum_decl->enumerators().end()));
  for (clang::EnumConstantDecl* enumerator : enum_decl->enumerators()) {
    std::optional<Identifier> enumerator_name =
        ictx_.GetTranslatedIdentifier(enumerator);
    if (!enumerator_name.has_value()) {
      // It's not clear that this case is possible
      return ictx_.ImportUnsupportedItem(
          enum_decl, "importing enum failed: missing enumerator name");
    }

    enumerators.push_back(Enumerator{
        .identifier = *enumerator_name,
        .value = IntegerConstant(enumerator->getInitVal()),
    });
  }

  return Enum{
      .identifier = *enum_name,
      .id = GenerateItemId(enum_decl),
      .owning_target = ictx_.GetOwningTarget(enum_decl),
      .source_loc = ictx_.ConvertSourceLocation(enum_decl->getBeginLoc()),
      .underlying_type = *std::move(type),
      .enumerators = enumerators,
      .enclosing_namespace_id = GetEnclosingNamespaceId(enum_decl),
  };
}

}  // namespace crubit
