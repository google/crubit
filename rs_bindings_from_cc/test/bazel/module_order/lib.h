// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MODULE_ORDER_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MODULE_ORDER_LIB_H_

#include <cstdint>

// `a.h` and `b.h` define `ABox` and `BBox`, and their precompiled modules
// contain separate instantiations of `Box`.
#include "rs_bindings_from_cc/test/bazel/module_order/a.h"
#include "rs_bindings_from_cc/test/bazel/module_order/b.h"
#include "rs_bindings_from_cc/test/bazel/module_order/template.h"

namespace lib_ns {

using IntBox = template_ns::Box<int>;
using Int16Box = template_ns::Box<int16_t>;

using IntABox = a_ns::ABox<int>;
using IntBBox = b_ns::BBox<int>;

inline IntBox MakeIntBox(int value) { return IntBox{.value = value}; }
inline Int16Box MakeInt16Box(int16_t value) { return Int16Box{.value = value}; }

}  // namespace lib_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MODULE_ORDER_LIB_H_
