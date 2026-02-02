// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMPOSABLE_BRIDGING_TEMPLATE_TYPE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMPOSABLE_BRIDGING_TEMPLATE_TYPE_H_

template <typename T>
// clang-format off
struct
    [[clang::annotate("crubit_bridge_rust_name", "MyOption")]]
    [[clang::annotate("crubit_bridge_abi_rust", "MyOptionAbi")]]
    [[clang::annotate("crubit_bridge_abi_cpp", "::crubit::MyOptionAbi")]]
// clang-format on
MyOption {
 private:
  bool present;
  union {
    T value;
  };
};

// A basic templated type that does nothing fancy.
template <typename T>
struct Value {
  T value;
};

MyOption<Value<int>> ReturnsValue();

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMPOSABLE_BRIDGING_TEMPLATE_TYPE_H_
