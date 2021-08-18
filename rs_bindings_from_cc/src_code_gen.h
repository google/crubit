// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_H_

#include <string>

#include "rs_bindings_from_cc/ir.h"

namespace rs_bindings_from_cc {

// Generates Rust bindings source code from the given `IR`.
std::string GenerateRustApi(const IR &ir);

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_H_
