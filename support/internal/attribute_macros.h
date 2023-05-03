// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_ATTRIBUTES_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_ATTRIBUTES_H_

// TODO(jeanpierreda): Get a waiver for these, or else a waiver for each caller.
#define CRUBIT_INTERNAL_ANNOTATE(...) [[clang::annotate(__VA_ARGS__)]]
#define CRUBIT_INTERNAL_ANNOTATE_TYPE(...) [[clang::annotate_type(__VA_ARGS__)]]

// Unsafe: disables bindings, and reinterprets all uses of this type as `t`.
//
// This attribute completely disables automated bindings for the type which it
// appertains to. All uses of that type are replaced with uses of `t`, which
// must be a rust type which exists and is guaranteed to be available by that
// name.
//
// This can be applied to a struct, class, or enum.
//
// TODO(b/274834739): also support type aliases.
//
// For example, this C++ header:
//
// ```c++
// struct CRUBIT_INTERNAL_RUST_TYPE("char") CharT {std::uint32_t c; };
// CharT foo() { return {0};}
// ```
//
// Becomes this Rust interface:
//
// ```rust
// pub fn foo() -> char;  // returns '\0'
// ```
//
// SAFETY:
//   If the type is not ABI-compatible with `t`, the behavior is undefined.
#define CRUBIT_INTERNAL_RUST_TYPE(t) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_rust_type", t)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_ATTRIBUTES_H_
