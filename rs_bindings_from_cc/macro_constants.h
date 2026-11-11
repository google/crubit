// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_MACRO_CONSTANTS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_MACRO_CONSTANTS_H_

#include <optional>

#include "absl/base/nullability.h"
#include "clang/AST/Decl.h"
#include "clang/Basic/IdentifierTable.h"
#include "clang/Lex/MacroInfo.h"
#include "clang/Parse/Parser.h"
#include "clang/Sema/Sema.h"

namespace crubit {

// A `constexpr` variable synthesized from an object-like macro.
struct MacroConstant {
  clang::VarDecl* absl_nonnull var_decl;
  // Whether the macro's expansion contains a numeric literal with a suffix
  // (e.g. `42u`), a `char16_t` or `char32_t` literal (e.g. `u'x'`), or a cast
  // (e.g. `(uint32_t)1`). If so,
  // the macro is bound with the type of `var_decl`. Otherwise, it's bound as an
  // untyped literal.
  bool is_typed;
};

// Returns a `constexpr` variable equivalent to the object-like macro `name`
// (defined by `macro_info`), e.g. `constexpr int FOO = 4;` for
// `#define FOO 4`, or nullopt if the macro isn't supported. The macro is
// supported if its expansion is a single C++ constant expression of a supported
// type, parsed by `parser` (the parser of the translation unit) after the end
// of the translation unit. Reports no diagnostics.
//
// The variable belongs to the translation unit, but isn't added to it: callers
// can add it with `addHiddenDecl`, so that name lookup doesn't find it.
std::optional<MacroConstant> SynthesizeMacroConstant(
    clang::Sema& sema, clang::Parser& parser, const clang::IdentifierInfo& name,
    const clang::MacroInfo& macro_info);

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_MACRO_CONSTANTS_H_
