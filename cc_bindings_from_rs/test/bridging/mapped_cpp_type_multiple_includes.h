// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_TEST_BRIDGING_MAPPED_CPP_TYPE_MULTIPLE_INCLUDES_H
#define CRUBIT_TEST_BRIDGING_MAPPED_CPP_TYPE_MULTIPLE_INCLUDES_H

#include <cstdint>

namespace mapped_cpp_type {
struct MultiHeaderType {
  MappedCppType base;
  int32_t extra;
};
}  // namespace mapped_cpp_type

#endif  // CRUBIT_TEST_BRIDGING_MAPPED_CPP_TYPE_MULTIPLE_INCLUDES_H
