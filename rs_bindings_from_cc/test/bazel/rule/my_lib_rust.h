// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef MY_LIB_RUST_H_
#define MY_LIB_RUST_H_

#include <stddef.h>

#include "rs_bindings_from_cc/test/bazel/rule/cpp_add.h"

inline size_t CppAdd(size_t a, size_t b) { return Add(a, b); }

#endif  // MY_LIB_RUST_H_
