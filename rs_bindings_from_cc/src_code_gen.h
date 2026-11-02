// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_H_

#include <string>

#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/profiling.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"

namespace crubit {

// Source code for generated bindings.
struct Bindings {
  // Rust source code.
  std::string rs_api;
  // C++ source code.
  std::string rs_api_impl;
  // Optional JSON error report.
  std::string error_report;
};

// Generates bindings from the given `IR`.
CRUBIT_PROFILE_NOINLINE llvm::Expected<Bindings> GenerateBindings(
    IR ir, llvm::StringRef crubit_support_path_format,
    llvm::StringRef crubit_support_versioned_path_format,
    llvm::StringRef clang_format_exe_path, llvm::StringRef rustfmt_exe_path,
    llvm::StringRef rustfmt_config_path, bool generate_error_report,
    bool is_golden_test, bool kythe_annotations,
    llvm::StringRef kythe_default_corpus);

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_H_
