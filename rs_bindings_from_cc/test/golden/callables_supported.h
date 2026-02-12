// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CALLABLES_SUPPORTED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CALLABLES_SUPPORTED_H_

#include "support/rs_std/dyn_callable.h"

void invoke_once(rs_std::DynCallable<void() &&> f);

void invoke(rs_std::DynCallable<void()> f);

void invoke_const(rs_std::DynCallable<void() const> f);

int map_int(rs_std::DynCallable<int(int) const> f, int arg);

struct [[clang::annotate("crubit_bridge_rust_name", "RustBridged")]]
[[clang::annotate("crubit_bridge_abi_rust", "RustBridgedAbi")]]
[[clang::annotate("crubit_bridge_abi_cpp", "::crubit::BridgedAbi")]] Bridged {};

Bridged map_bridged(rs_std::DynCallable<Bridged(Bridged) const> f, Bridged arg);

struct ABICompatible {
  int x;
};

ABICompatible map_abi_compatible(
    rs_std::DynCallable<ABICompatible(ABICompatible) const> f,
    ABICompatible arg);

class LayoutCompatible {
 private:
  explicit LayoutCompatible(int x) : private_(x) {}

 public:
  static LayoutCompatible Create(int x) { return LayoutCompatible(x); }

  int get() const { return private_; }

 private:
  int private_;
};

LayoutCompatible map_layout_compatible(
    rs_std::DynCallable<LayoutCompatible(LayoutCompatible) const> f,
    LayoutCompatible arg);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CALLABLES_SUPPORTED_H_
