// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPEDEF_NAME_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPEDEF_NAME_H_

#include <memory>

#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"

namespace crubit {

// A `DeclImporter` for `TypedefNameDecl`s (e.g. `using x = y`, or
// `typedef y x`) and `UsingShadowDecl`s (`using x::y`).
class TypeAliasImporter : public DeclImporterBase<clang::NamedDecl> {
 public:
  explicit TypeAliasImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::unique_ptr<ir_proto::Item> Import(clang::NamedDecl* decl) override;

 private:
  // Imports a using-declaration that names a function, e.g. `using A::Foo;`
  // inside `namespace B`.
  //
  // The aliased function is imported as a `Func` item whose location,
  // enclosing item and owning target are those of the using-declaration, so
  // that the generated Rust function lives in the module of the
  // using-declaration (`B::Foo`). The C++ thunk calls the function through the
  // using-declaration's qualified name, which is valid C++.
  std::unique_ptr<ir_proto::Item> ImportFunctionAlias(
      clang::UsingShadowDecl& using_decl, clang::FunctionDecl& function_decl);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPEDEF_NAME_H_
