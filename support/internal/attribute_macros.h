// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "absl/base/attributes.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_ATTRIBUTES_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_ATTRIBUTES_H_

// Style waiver granted in <internal link>
#if ABSL_HAVE_CPP_ATTRIBUTE(clang::annotate) && \
    ABSL_HAVE_CPP_ATTRIBUTE(clang::annotate_type)
#define CRUBIT_INTERNAL_ANNOTATE(...) [[clang::annotate(__VA_ARGS__)]]
#define CRUBIT_INTERNAL_ANNOTATE_TYPE(...) [[clang::annotate_type(__VA_ARGS__)]]
#else
#define CRUBIT_INTERNAL_ANNOTATE(...)
#define CRUBIT_INTERNAL_ANNOTATE_TYPE(...)
#endif

// Unsafe: disables bindings, and reinterprets all uses of this type as `t`.
//
// This attribute completely disables automated bindings for the type which it
// appertains to. All uses of that type are replaced with uses of `t`, which
// must be a rust type which exists and is guaranteed to be available by that
// name.
//
// This can be applied to a struct, class, or enum.
//
// If CRUBIT_INTERNAL_SAME_ABI is also specified, then the Rust
// type will be assumed to be exactly C ABI compatible (not just of identical
// layout) with `t`.
//
// TODO(b/274834739): also support type aliases.
//
// For example, this C++ header:
//
// ```c++
// struct
//   CRUBIT_INTERNAL_RUST_TYPE("char")
//   CRUBIT_INTERNAL_SAME_ABI
//   CharT {
//     std::uint32_t c;
// };
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
//   If the type is not layout-compatible with `t`, the behavior is undefined.
#define CRUBIT_INTERNAL_RUST_TYPE(t) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_rust_type", t)

// Unsafe: forces a type to be treated as C abi compatible with its rust
// equivalent.
//
// This only has any effect when used in combination with
// CRUBIT_INTERNAL_RUST_TYPE. For all other types, C ABI compatibility can be
// inferred automatically, and overriding the decision is always a bug.
//
// TODO(b/284512049): This also currently assumes copyability. It should only
// assume movability.
//
// SAFETY:
//   If the type is not ABI-compatible with its Rust equivalent, the behavior is
//   undefined.
#define CRUBIT_INTERNAL_SAME_ABI \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_same_abi")

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_ATTRIBUTES_H_
