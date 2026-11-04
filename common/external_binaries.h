// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Rust and sh libraries can use environment variables exported from Bazel,
// but C++ doesn't have that feature, so instead we call into Rust to obtain
// the paths.
//
// First of all, this keeps them guaranteed in sync. But, also, it reuses the
// option_env logic of Rust, so that the value is potentially different in
// Cargo.

#ifndef THIRD_PARTY_CRUBIT_COMMON_EXTERNAL_BINARIES_H_
#define THIRD_PARTY_CRUBIT_COMMON_EXTERNAL_BINARIES_H_

#include <string_view>

extern "C" const char* crubit_rustfmt_exe_path();
extern "C" const char* crubit_clang_format_exe_path();

namespace crubit {

inline std::string_view RustfmtExePath() {
  static std::string_view kRustfmtExePath = crubit_rustfmt_exe_path();
  return kRustfmtExePath;
}

inline std::string_view ClangFormatExePath() {
  static std::string_view kClangFormatExePath = crubit_clang_format_exe_path();
  return kClangFormatExePath;
}

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_COMMON_EXTERNAL_BINARIES_H_
