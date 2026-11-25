// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CALLABLES_CALLS_BACK_TO_RUST_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CALLABLES_CALLS_BACK_TO_RUST_H_

#include <optional>

#include "support/annotations.h"
#include "support/rs_std/dyn_callable.h"
#include "support/rs_std/fn_ref.h"

void invoke_once(rs::DynCallable<void() &&> f);

void invoke(rs::DynCallable<void()> f);

void invoke_const(rs::DynCallable<void() const> f);

int map_int(rs::DynCallable<int(int) const> f, int arg);

std::optional<int> map_optional_int(
    rs::DynCallable<std::optional<int>(std::optional<int>) const> f,
    std::optional<int> arg);

struct ABICompatible {
  int x;
};

ABICompatible map_abi_compatible(
    rs::DynCallable<ABICompatible(ABICompatible) const> f, ABICompatible arg);

class LayoutCompatible {
 private:
  explicit LayoutCompatible(int x) : private_(x) {}

 public:
  static LayoutCompatible Create(int x) { return LayoutCompatible(x); }

  CRUBIT_UNSAFE_MARK_SAFE
  int get() const { return private_; }

 private:
  int private_;
};

LayoutCompatible map_layout_compatible(
    rs::DynCallable<LayoutCompatible(LayoutCompatible) const> f,
    LayoutCompatible arg);

rs::DynCallable<void() &&> bridge_to_and_from_cpp(rs::DynCallable<void() &&> f);

void invoke_as_absl_anyinvocable(rs::DynCallable<void()> f);

void invoke_fn_ref(rs::FnRef<void()> f);
void invoke_fn_ref_const(rs::FnRef<void() const> f);

int map_int_fn_ref(rs::FnRef<int(int) const> f, int arg);

std::optional<int> map_optional_int_fn_ref(
    rs::FnRef<std::optional<int>(std::optional<int>) const> f,
    std::optional<int> arg);

ABICompatible map_abi_compatible_fn_ref(
    rs::FnRef<ABICompatible(ABICompatible) const> f, ABICompatible arg);

LayoutCompatible map_layout_compatible_fn_ref(
    rs::FnRef<LayoutCompatible(LayoutCompatible) const> f,
    LayoutCompatible arg);

struct HasFnRefMethod {
  int unused;
  void invoke(rs::FnRef<void()> f);
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CALLABLES_CALLS_BACK_TO_RUST_H_
