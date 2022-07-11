// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NON_TYPE_TEMPLATE_PARAMS_NON_TYPE_TEMPLATE_PARAMS_H_
#define DEVTOOLS_RUST_CC_INTEROP_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NON_TYPE_TEMPLATE_PARAMS_NON_TYPE_TEMPLATE_PARAMS_H_

#pragma clang lifetime_elision

template <int multiplier>
class MyTemplate final {
 public:
  static int Multiply(int value) { return value * multiplier; }
};

using MyMultiplierX100 = MyTemplate<100>;
using MyMultiplierX1000 = MyTemplate<1000>;

#endif  // DEVTOOLS_RUST_CC_INTEROP_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NON_TYPE_TEMPLATE_PARAMS_NON_TYPE_TEMPLATE_PARAMS_H_
