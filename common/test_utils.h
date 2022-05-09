// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// A collection of utility functions for Gunit tests.

#ifndef THIRD_PARTY_CRUBIT_COMMON_TEST_UTILS_H_
#define THIRD_PARTY_CRUBIT_COMMON_TEST_UTILS_H_

#include <string>
#include <vector>

#include "absl/strings/string_view.h"
namespace crubit {

// Writes a file for a current gunit test in a temporary directory specific to
// the current test. Returns the path of the newly written file.
std::string WriteFileForCurrentTest(absl::string_view filename,
                                    absl::string_view content);

// Gets Clang flags for gunit tests (for example -I flag that tells Clang to
// search for headers in the temporary directory where `WriteFileForCurrentTest`
// writes files).
std::vector<std::string> DefaultClangArgs();

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_COMMON_TEST_UTILS_H_
