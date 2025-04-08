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
#include "clang/include/clang/AST/ASTContext.h"
#include "clang/include/clang/AST/Attr.h"
#include "clang/include/clang/AST/Decl.h"
#include "clang/include/clang/AST/DeclBase.h"
#include "clang/include/clang/AST/Expr.h"

namespace crubit {

using AnnotateArgs = llvm::SmallVector<clang::Expr*>;

// Returns the arguments of the [[clang::annotate(annotation_name, ...)]]
// annotation on `decl`, or none if the annotation does not exist.
//
// Returns an error if there there are any conflicting annotation arguments
// across all redeclarations of `decl`, or `std::nullopt` if the annotation does
// not exist.
//
// For example, given the following C++ code:
//
// class [[clang::annotate("crubit_annotation_foo", "bar")]] MyClass;
//
// class [[clang::annotate("crubit_annotation_foo", "bar")]] MyClass {}
//   ...
// };
//
// GetAnnotateAttrArgs(my_class_decl, "crubit_annotation_foo") will return
// ["bar"] because 1) it found it, and 2) the annotations are consistent across
// all redecls. But if the C++ code is:
//
// class [[clang::annotate("crubit_annotation_foo", "bar")]] MyClass;
//
// class [[clang::annotate("crubit_annotation_foo", "baz")]] MyClass {}
//   ...
// };
//
// GetAnnotateAttrArgs(my_class_decl, "crubit_annotation_foo") will return an
// error, because the annotations are inconsistent across redeclarations.
//
// Finally, if the C++ code is:
//
// class MyClass {
//   ...
// };
//
// GetAnnotateAttrArgs(my_class_decl, "crubit_annotation_foo") will return
// std::nullopt, because the annotation does not exist.
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

// Returns true if `decl` has an annotation with the given name.
//
// Returns an error if an annotation with the given name exists, but it has
// arguments.
absl::StatusOr<bool> HasAnnotationWithoutArgs(
    const clang::Decl& decl, absl::string_view annotation_name);

absl::Status RequireSingleStringArgIfExists(const clang::Decl* decl,
                                            absl::string_view attribute);
}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ANNOTATION_READER_H_
