// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/friend.h"

#include <optional>
#include <variant>

#include "absl/log/check.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/Basic/LLVM.h"

namespace crubit {

std::optional<IR::Item> FriendDeclImporter::Import(
    clang::FriendDecl* friend_decl) {
  if (!ictx_.IsFromCurrentTarget(friend_decl)) return std::nullopt;

  // Check if this is a `friend` declaration for a function (and not for a
  // type).
  clang::NamedDecl* named_decl = clang::dyn_cast_or_null<clang::FunctionDecl>(
      friend_decl->getFriendDecl());
  if (!named_decl) return std::nullopt;

  // Skip non-canonical decls, similarly to GetCanonicalChildren in importer.cc
  if (named_decl != named_decl->getCanonicalDecl()) return std::nullopt;

  // Get the enclosing record declaration.
  clang::DeclContext* decl_context = friend_decl->getDeclContext();
  if (!decl_context) {
    return ictx_.ImportUnsupportedItem(
        friend_decl, UnsupportedItem::Kind::kFunc, std::nullopt,
        FormattedError::Static("DeclContext was unexpectedly null"));
  }
  clang::CXXRecordDecl* enclosing_record_decl =
      clang::dyn_cast<clang::CXXRecordDecl>(decl_context);
  if (!enclosing_record_decl) {
    return ictx_.ImportUnsupportedItem(
        friend_decl, UnsupportedItem::Kind::kFunc, std::nullopt,
        FormattedError::Static(
            "DeclContext was unexpectedly not a CXXRecordDecl"));
  }

  // If `!is_redeclared_outside_of_friend_decl`, then we need to emit a new item
  // to support the following case from
  // https://en.cppreference.com/w/cpp/language/adl: "ADL can find a friend
  // function (typically, an overloaded operator) that is defined entirely
  // within a class or class template, even if it was never declared at
  // namespace level."
  std::optional<IR::Item> item = ictx_.ImportDecl(named_decl);
  if (!item.has_value()) return std::nullopt;
  if (std::holds_alternative<UnsupportedItem>(*item)) return std::nullopt;
  Func* func_item = std::get_if<Func>(&*item);
  CHECK(func_item);  // Guaranteed by `isa<clang::FunctionDecl>` above.
  // Return the recursively generated function item almost as-is. It needs a
  // fresh item ID because it came from this friend_decl.
  //
  // We also set the `adl_enclosing_record` field to the enclosing record. This
  // allows us to prevent generation of bindings in the case that the enclosing
  // record is not visible, as ADL is necessary for the friend function to be
  // found.
  Func result = *func_item;
  result.id = ictx_.GenerateItemId(friend_decl);
  result.adl_enclosing_record = ictx_.GenerateItemId(enclosing_record_decl);
  return result;
}

}  // namespace crubit
