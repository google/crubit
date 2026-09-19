// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGE_WRAPPING_ALIAS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGE_WRAPPING_ALIAS_H_

#include "support/bridge.h"

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

// Note: this deliberately avoids standard library headers (b/269489158), so
// `static_cast<T&&>` is used in place of `std::move`.
template <typename Abi>
  requires(crubit::is_crubit_abi<Abi>)
struct BridgeAbi {
  using Value = Bridge<typename Abi::Value>;
  static constexpr decltype(Abi::kSize) kSize = Abi::kSize;
  void Encode(Value value, crubit::Encoder& encoder) && {
    static_cast<Abi&&>(abi).Encode(
        static_cast<typename Abi::Value&&>(value.value), encoder);
  }
  Value Decode(crubit::Decoder& decoder) && {
    return {.value = static_cast<Abi&&>(abi).Decode(decoder)};
  }

  Abi abi;
};

// `AliasToInst` is an alias to a template instantiation. With the
// `template_instantiation` feature enabled (see the aspect hints in BUILD),
// the instantiation is bound and the bridge type wrapping it can be bridged
// too, as `Bridge<__CcTemplateInst12TemplateTypeIiE>`.
Bridge<AliasToInst> bridge_alias_to_inst();

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGE_WRAPPING_ALIAS_H_
