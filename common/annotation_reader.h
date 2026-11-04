// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ANNOTATION_READER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ANNOTATION_READER_H_

#include <optional>
#include <string>

#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Expr.h"
#include "clang/AST/TypeBase.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"

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
llvm::Expected<std::optional<AnnotateArgs>> GetAnnotateAttrArgs(
    const clang::Decl& decl, llvm::StringRef annotation_name);

// Evaluates `expr` as a boolean.
llvm::Expected<bool> GetExprAsBool(const clang::Expr& expr,
                                   const clang::ASTContext& ast_context);

// Evaluates `expr` as a string literal.
llvm::Expected<llvm::StringRef> GetExprAsStringLiteral(
    const clang::Expr& expr, const clang::ASTContext& ast_context);

// Returns the `string_arg` of [[clang::annotate(annotation_name, string_arg)]]
// annotation on `decl`, or none if the annotation does not exist.
//
// Returns an error if there are conflicting annotations or if the argument is
// not a single string.
llvm::Expected<std::optional<std::string>> GetAnnotationWithStringArg(
    const clang::Decl& decl, llvm::StringRef annotation_name);

// Returns the string arguments of [[clang::annotate(annotation_name,
// string_arg1, string_arg2, ...)]] annotation on `decl`, or none if the
// annotation does not exist.
//
// Returns an error if there are conflicting annotations or if any argument is
// not a string.
llvm::Expected<std::optional<std::vector<std::string>>>
GetAnnotationWithStringArgs(const clang::Decl& decl,
                            llvm::StringRef annotation_name);

// Returns true if `decl` has an annotation with the given name.
//
// Returns an error if an annotation with the given name exists, but it has
// arguments.
llvm::Expected<bool> HasAnnotationWithoutArgs(const clang::Decl& decl,
                                              llvm::StringRef annotation_name);

// Returns the `AnnotateTypeAttr` with the given `annotation_name` if it exists.
// If there are multiple annotations with the given name, returns an error.
llvm::Expected<const clang::AnnotateTypeAttr* _Nullable>
GetTypeAnnotationSingleDecl(const clang::Type* _Nonnull type
                            [[clang::lifetimebound]],
                            llvm::StringRef annotation_name);

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ANNOTATION_READER_H_
