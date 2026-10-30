// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_CONST_GENERIC_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_CONST_GENERIC_H_

#include <cstddef>
#include <cstdint>

#include "support/annotations_internal.h"

template <typename T, size_t N, int32_t Tag>
struct CRUBIT_INTERNAL_RUST_TYPE("crate::RustTaggedArray<{}, {}, {}>", T,
                                 crubit::rust_type::Const<N>,
                                 crubit::rust_type::Const<Tag>)
    CppTaggedArray final {
  T elements[N];
};

inline CppTaggedArray<int32_t, 3, -5> MakeTaggedArray() {
  return CppTaggedArray<int32_t, 3, -5>{{10, 20, 30}};
}

inline int32_t SumTaggedArrayWithTag(CppTaggedArray<int32_t, 3, -5> arr) {
  return arr.elements[0] + arr.elements[1] + arr.elements[2] + (-5);
}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_CONST_GENERIC_H_
