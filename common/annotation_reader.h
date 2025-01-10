// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ANNOTATION_READER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ANNOTATION_READER_H_

#include <optional>
#include <string>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Expr.h"

namespace crubit {

using AnnotateArgs = llvm::SmallVector<clang::Expr*>;

// Returns the arguments of the `[clang::annotate(annotation_name)]`
// annotation on `decl`, if it exists.
//
// Returns an error if there is more than one annotation with the given name,
// or `std::nullopt` if the annotation was not found.
absl::StatusOr<std::optional<AnnotateArgs>> GetAnnotateAttrArgs(
    const clang::Decl& decl, absl::string_view annotation_name);

// Evaluates `expr` as a boolean.
absl::StatusOr<bool> GetExprAsBool(const clang::Expr& expr,
                                   const clang::ASTContext& ast_context);

// Evaluates `expr` as a string literal.
absl::StatusOr<absl::string_view> GetExprAsStringLiteral(
    const clang::Expr& expr, const clang::ASTContext& ast_context);

std::optional<std::string> GetAnnotateArgAsStringByAttribute(
    const clang::Decl* decl, absl::string_view attribute);

absl::Status RequireSingleStringArgIfExists(const clang::Decl* decl,
                                            absl::string_view attribute);
}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ANNOTATION_READER_H_
