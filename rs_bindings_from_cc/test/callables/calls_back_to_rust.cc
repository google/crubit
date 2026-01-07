// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/callables/calls_back_to_rust.h"

#include <optional>
#include <utility>

#include "support/rs_std/dyn_callable.h"

void invoke_once(rs_std::DynCallable<void() &&> f) { std::move(f)(); }

void invoke(rs_std::DynCallable<void()> f) { f(); }

void invoke_const(rs_std::DynCallable<void() const> f) { f(); }

int map_int(rs_std::DynCallable<int(int) const> f, int arg) { return f(arg); }

std::optional<int> map_optional_int(
    rs_std::DynCallable<std::optional<int>(std::optional<int>) const> f,
    std::optional<int> arg) {
  return f(arg);
}

ABICompatible map_abi_compatible(
    rs_std::DynCallable<ABICompatible(ABICompatible) const> f,
    ABICompatible arg) {
  return f(arg);
}

LayoutCompatible map_layout_compatible(
    rs_std::DynCallable<LayoutCompatible(LayoutCompatible) const> f,
    LayoutCompatible arg) {
  return f(arg);
}
