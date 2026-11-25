// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ABSL_FUNCTIONAL_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ABSL_FUNCTIONAL_H_

#include "absl/functional/any_invocable.h"
#include "absl/functional/function_ref.h"

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

// A type alias to an AnyInvocable becomes a type alias to the bridged Rust
// type.
using IntMapper = absl::AnyInvocable<int(int) const>;

// Type aliases are transparent: the parameter is spelled as the bridged Rust
// type, not as `IntMapper`.
int CallIntMapper(IntMapper f, int i);

// Member type aliases to an AnyInvocable also receive bindings.
struct Widget {
  using DoneCallback = absl::AnyInvocable<void() &&>;
};

// IncompleteRecord
struct Incomplete;

// Should not receive bindings because the type is incomplete.
absl::AnyInvocable<Incomplete(Incomplete) const> ReturnIncompleteMapper();
// Should not receive bindings because the type is incomplete.
Incomplete CallIncompleteMapper(
    absl::AnyInvocable<Incomplete(Incomplete) const> f, Incomplete i);

void InvokeFunctionRef(absl::FunctionRef<void()> f);
int ApplyFunctionRef(absl::FunctionRef<int(int)> f, int x);
int ApplyConstFunctionRef(absl::FunctionRef<int(int) const> f, int x);
using IntTransformRef = absl::FunctionRef<int(int)>;
int ApplyAliasFunctionRef(IntTransformRef f, int x);

// Return-lifetime elision ties the returned reference to `v`, not `pred`.
const int& FindIf(const int& v, absl::FunctionRef<bool(int)> pred);

// Constructors taking `absl::FunctionRef` on both `Unpin` (`From`) and
// `!Unpin` (`CtorNew`) classes.
struct UnpinCallbackHolder {
  explicit UnpinCallbackHolder(absl::FunctionRef<void()> f);
  int value;
};

struct PinnedCallbackHolder {
  explicit PinnedCallbackHolder(absl::FunctionRef<void()> f);
  PinnedCallbackHolder(const PinnedCallbackHolder&) = delete;
  int value;
};

// Escape-route rejections: return types that are or contain borrowed callables.
absl::FunctionRef<void()> ReturnFunctionRef();

struct StructWithFunctionRef {
  absl::FunctionRef<void()> f;
};

StructWithFunctionRef MakeStructWithFunctionRef(absl::FunctionRef<void()> f);
MyOption<absl::FunctionRef<void()>> ReturnMyOptionFunctionRef();

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ABSL_FUNCTIONAL_H_
