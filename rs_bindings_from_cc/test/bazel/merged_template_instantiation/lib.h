// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MERGED_TEMPLATE_INSTANTIATION_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MERGED_TEMPLATE_INSTANTIATION_LIB_H_

#include <cstdint>

// `a.h` and `b.h` aren't used directly, but they make the precompiled modules
// of `:a` and `:b` contain separate instantiations of the same templates.
#include "rs_bindings_from_cc/test/bazel/merged_template_instantiation/a.h"  // IWYU pragma: keep
#include "rs_bindings_from_cc/test/bazel/merged_template_instantiation/b.h"  // IWYU pragma: keep
#include "rs_bindings_from_cc/test/bazel/merged_template_instantiation/template.h"

namespace lib_ns {

using IntBox = template_ns::Box<int>;
using Int16Box = template_ns::Box<int16_t>;

inline IntBox MakeIntBox(int value) { return IntBox{.value = value}; }
inline Int16Box MakeInt16Box(int16_t value) { return Int16Box{.value = value}; }

}  // namespace lib_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_MERGED_TEMPLATE_INSTANTIATION_LIB_H_
