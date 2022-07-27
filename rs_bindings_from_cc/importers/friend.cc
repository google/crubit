// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/friend.h"

#include <iostream>
#include <optional>

#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"

namespace crubit {

std::optional<IR::Item> FriendDeclImporter::Import(
    clang::FriendDecl* friend_decl) {
  if (!ictx_.IsFromCurrentTarget(friend_decl)) return std::nullopt;

  // Recurse to import the function declaration.
  clang::NamedDecl* named_decl = friend_decl->getFriendDecl();
  if (!named_decl) {
    // This friend declaration refers to a type. We don't need to import it.
    return std::nullopt;
  }
  std::optional<IR::Item> item = ictx_.ImportDecl(named_decl);
  if (!item.has_value()) return std::nullopt;
  Func* func_item = std::get_if<Func>(&*item);
  if (!func_item) return std::nullopt;

  // Get the enclosing record declaration.
  clang::DeclContext* decl_context = friend_decl->getDeclContext();
  if (!decl_context) {
    return ictx_.ImportUnsupportedItem(friend_decl,
                                       "DeclContext was unexpectedly null");
  }
  clang::CXXRecordDecl* enclosing_record_decl =
      clang::dyn_cast<clang::CXXRecordDecl>(decl_context);
  if (!enclosing_record_decl) {
    return ictx_.ImportUnsupportedItem(
        friend_decl, "DeclContext was unexpectedly not a CXXRecordDecl");
  }

  // Return the recursively generated function item almost as-is. It needs a
  // fresh item ID because it came from this friend_decl. And it needs an ADL
  // enclosing record note because as a friend function it is not visible at top
  // level.
  Func result = *func_item;
  result.id = GenerateItemId(friend_decl);
  result.adl_enclosing_record = GenerateItemId(enclosing_record_decl);
  return result;
}

}  // namespace crubit
