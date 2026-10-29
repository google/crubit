// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_MACRO_CONSTANTS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_MACRO_CONSTANTS_H_

#include "absl/base/nullability.h"
#include "clang/AST/Decl.h"
#include "clang/Basic/IdentifierTable.h"
#include "clang/Lex/MacroInfo.h"
#include "clang/Sema/Sema.h"

namespace crubit {

// Returns a `constexpr` variable equivalent to the object-like macro `name`
// (defined by `macro_info`), e.g. `constexpr int FOO = 4;` for
// `#define FOO 4`, or null if the macro isn't a supported constant expression
// (see `MacroExpressionParser`). Reports no diagnostics.
//
// The variable belongs to the translation unit, but isn't added to it: callers
// can add it with `addHiddenDecl`, so that name lookup doesn't find it.
clang::VarDecl* absl_nullable SynthesizeMacroConstant(
    clang::Sema& sema, const clang::IdentifierInfo& name,
    const clang::MacroInfo& macro_info);

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_MACRO_CONSTANTS_H_
