// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ABSL_FUNCTIONAL_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ABSL_FUNCTIONAL_H_

#include "absl/functional/any_invocable.h"

template <typename T>
struct                                                                     //
    [[clang::annotate("crubit_bridge_rust_name", "MyOption")]]             //
    [[clang::annotate("crubit_bridge_abi_rust", "MyOptionAbi")]]           //
    [[clang::annotate("crubit_bridge_abi_cpp", "::crubit::MyOptionAbi")]]  //
    MyOption {
  // A hypothetical implementation would have an optional field here, but it's
  // unnecessary for golden tests and including <optional> bloats the codegen.
};

// Calls the invocable and returns void.
void CallVoidVoid(absl::AnyInvocable<void() &&> f);

// Returns an invocable that returns 42.
absl::AnyInvocable<int(int) const> ReturnIntMapper();

// Returns an AnyInvocable that takes a MyOption<int> and returns a
// MyOption<int>.
absl::AnyInvocable<MyOption<int>(MyOption<int>) const> MyOptionIntMapper();

// Returns an AnyInvocable without const qualifier, which Crubit promotes to
// Fn in Rust.
absl::AnyInvocable<int(int)> ReturnNonConstIntMapper();

// IncompleteRecord
struct Incomplete;

// Should not receive bindings because the type is incomplete.
absl::AnyInvocable<Incomplete(Incomplete) const> ReturnIncompleteMapper();
// Should not receive bindings because the type is incomplete.
Incomplete CallIncompleteMapper(
    absl::AnyInvocable<Incomplete(Incomplete) const> f, Incomplete i);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ABSL_FUNCTIONAL_H_
