// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_FRIEND_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_FRIEND_H_

#include "rs_bindings_from_cc/decl_importer.h"
#include "clang/AST/DeclFriend.h"

namespace crubit {

// A `DeclImporter` for `FriendDecl`s.
class FriendDeclImporter : public DeclImporterBase<clang::FriendDecl> {
 public:
  FriendDeclImporter(ImportContext& context) : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::FriendDecl*) override;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_FRIEND_H_
