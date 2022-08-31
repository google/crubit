// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TEMPLATE_TEMPLATE_PARAMS_TEMPLATE_TEMPLATE_PARAMS_H_
#define DEVTOOLS_RUST_CC_INTEROP_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TEMPLATE_TEMPLATE_PARAMS_TEMPLATE_TEMPLATE_PARAMS_H_

#pragma clang lifetime_elision

template <typename TPolicyType>
struct Policy {
  static constexpr TPolicyType policy = TPolicyType();
};

template <>
struct Policy<int> {
  static constexpr int policy = 42;
};

template <template <class> class TPolicy>
class MyTemplate {
 public:
  static int GetPolicy() { return TPolicy<int>::policy; }
};

using MyTypeAlias = MyTemplate<Policy>;

#endif  // DEVTOOLS_RUST_CC_INTEROP_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_TEMPLATE_TEMPLATE_PARAMS_TEMPLATE_TEMPLATE_PARAMS_H_
