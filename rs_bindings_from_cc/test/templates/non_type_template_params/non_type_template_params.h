// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NON_TYPE_TEMPLATE_PARAMS_NON_TYPE_TEMPLATE_PARAMS_H_
#define DEVTOOLS_RUST_CC_INTEROP_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NON_TYPE_TEMPLATE_PARAMS_NON_TYPE_TEMPLATE_PARAMS_H_

#include <stdint.h>

#pragma clang lifetime_elision

template <int multiplier>
class MyTemplate final {
 public:
  static int Multiply(int value) { return value * multiplier; }
};

using MyMultiplierX100 = MyTemplate<100>;
using MyMultiplierX1000 = MyTemplate<1000>;

template <uint64_t x>
struct NumericConst {
  static uint64_t GetValue() { return x; }
};

// This is a regression test for b/244616557.  This test mimics the problem that
// used to be encounteded when trying to build bindings for the standard
// `<random>` header which uses big constants for some of the predefined/builtin
// random number generators.  Before the fix, this test would fail to build,
// with errors like:
//
//   .../non_type_template_params/non_type_template_params_rust_api_impl.cc:75:
//   error: integer literal is too large to be represented in a signed integer
//   type, interpreting as unsigned [-Werror,-Wimplicitly-unsigned-literal]
//   static_assert(sizeof(class NumericConst<18446744073709551615>) == 1);
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wimplicitly-unsigned-literal"
using BigNumericConst = NumericConst<18446744073709551615>;
#pragma clang diagnostic pop

#endif  // DEVTOOLS_RUST_CC_INTEROP_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NON_TYPE_TEMPLATE_PARAMS_NON_TYPE_TEMPLATE_PARAMS_H_
