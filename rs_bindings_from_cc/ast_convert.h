// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Standalone AST -> IR conversion functions, which can be used independently
// of ast_visitor.
//
// This allows us to, for example, perform a subset of the work of ast_visitor
// inside of a ClangMR or other tool.
#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_CONVERT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_CONVERT_H_

#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"

namespace crubit {

SpecialMemberFunc GetCopyCtorSpecialMemberFunc(ImportContext& ictx,
                                               clang::RecordDecl& record_decl);

SpecialMemberFunc GetMoveCtorSpecialMemberFunc(ImportContext& ictx,
                                               clang::RecordDecl& record_decl);

SpecialMemberFunc GetDestructorSpecialMemberFunc(
    clang::RecordDecl& record_decl);

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_CONVERT_H_
