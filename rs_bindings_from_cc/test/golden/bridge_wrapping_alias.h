// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGE_WRAPPING_ALIAS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGE_WRAPPING_ALIAS_H_

template <typename T>
struct TemplateType {
  T value;
};

using AliasToInst = TemplateType<int>;

template <typename T>
struct                                                          //
    [[clang::annotate("crubit_bridge_rust_name", "Bridge")]]    //
    [[clang::annotate("crubit_bridge_abi_rust", "BridgeAbi")]]  //
    [[clang::annotate("crubit_bridge_abi_cpp", "BridgeAbi")]]   //
    Bridge {
  T value;
};

// This function should fail to generate, and should provide a helpful error
// message describing not only that it's because a type is an alias to a
// template instantiation and therefore cannot be bridged, but specifically that
// it is AliasToInst's fault.
Bridge<AliasToInst> bridge_alias_to_inst();

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGE_WRAPPING_ALIAS_H_
