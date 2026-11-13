// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_PER_FILE_COPT_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_PER_FILE_COPT_LIB_H_

#include "rs_bindings_from_cc/test/bazel/per_file_copt/dep.h"

inline Dep MakeDep(int value) { return Dep{value}; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_PER_FILE_COPT_LIB_H_
