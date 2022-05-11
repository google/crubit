// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_MIGRATOR_RS_FROM_CC_RS_FROM_CC_LIB_H_
#define CRUBIT_MIGRATOR_RS_FROM_CC_RS_FROM_CC_LIB_H_

#include <string>

#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"

namespace crubit_rs_from_cc {

// Converts C++ source code into Rust.
//
// Parameters:
// * `cc_file_content`: a string with the C++ source code to convert.
// * `cc_file_name`: name of the C++ file we're converting. Can be omitted for
//   tests.
// * `args`: additional command line arguments for Clang (if any)
//
absl::StatusOr<std::string> RsFromCc(
    absl::string_view cc_file_content,
    absl::string_view cc_file_name = "testing/file_name.cc",
    absl::Span<const absl::string_view> args = {});

}  // namespace crubit_rs_from_cc

#endif  // CRUBIT_MIGRATOR_RS_FROM_CC_RS_FROM_CC_LIB_H_
