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

namespace crubit {

// Gets the requested attribute for `decl`.
absl::StatusOr<const clang::AnnotateAttr*> GetAnnotateAttr(
    const clang::Decl& decl, absl::string_view attribute);

// Evaluates `expr` as a string literal.
absl::StatusOr<absl::string_view> GetAnnotateArgAsStringLiteral(
    const clang::AnnotateAttr& attr, const clang::ASTContext& ast_context);

std::optional<std::string> GetAnnotateArgAsStringByAttribute(
    const clang::Decl* decl, absl::string_view attribute);

absl::Status RequireSingleStringArgIfExists(const clang::Decl* decl,
                                            absl::string_view attribute);
}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ANNOTATION_READER_H_
