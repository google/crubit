// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATIONS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATIONS_H_

#include "support/internal/attribute_macros.h"

// By default, <internal link> will infer Rust safety based on the types of the
// function's parameters. This annotation can be used to override that
// inference.
#define CRUBIT_OVERRIDE_UNSAFE(function_is_unsafe) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_override_unsafe", function_is_unsafe)

// This annotation configures a user-defined rust name for a C++ declaration.
// If the user-defined rust name conflicts with an existing name, bindings for
// both the annotated decl and the conflicting decl will fail, and be treated as
// overloads.
//
// For example, this C++ header:
//
// ```c++
// CRUBIT_RUST_NAME(foo)
// int bar() { return 42; }
// ```
//
// Becomes this Rust interface:
//
// ```rust
// pub fn foo() -> i32;  // returns 42
// ```
#define CRUBIT_RUST_NAME(crubit_rust_name) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_rust_name", crubit_rust_name)

// This annotation is used to mark a function as `unsafe` to Rust callers.
//
// For example, this C++ header:
//
// ```c++
// CRUBIT_UNSAFE
// int foo() { return 42; }
// ```
//
// Becomes this Rust interface:
//
// ```rust
// pub unsafe fn foo() -> i32;  // returns 42
// ```
//
// By default, <internal link> will infer Rust safety based on the types of the
// function's parameters. This annotation can be used to override that
// inference.
#define CRUBIT_UNSAFE CRUBIT_OVERRIDE_UNSAFE(true)

// This annotation is used to mark a function as `safe` to Rust callers
// regardless of the types of the function's parameters.
//
// For example, this C++ header:
//
// ```c++
// CRUBIT_UNSAFE_MARK_SAFE
// uint64_t foo(uint64_t* unsafe_ptr) { return 42; }
// ```
//
// Becomes this Rust interface:
//
// ```rust
// pub fn foo(*mut u64) -> u64;  // returns 42
// ```
//
// By default, <internal link> will infer Rust safety based on the types of the
// function's parameters. This annotation can be used to override that
// inference.
#define CRUBIT_UNSAFE_MARK_SAFE CRUBIT_OVERRIDE_UNSAFE(false)

// Marks a type as deriving a trait.
//
// This can be applied to a struct, class, or enum.
//
// For example, this C++ header:
//
// ```c++
// struct CRUBIT_TRAIT_DERIVE("Debug") MyStruct {
//     bool enable_foo;
// };
// ```
//
// Becomes this Rust interface:
//
// ```rust
// #[derive(..., Debug)]
// pub struct MyStruct {
//   enable_foo: bool,
// }
// ```
#define CRUBIT_TRAIT_DERIVE(...)                                         \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_trait_derive" __VA_OPT__(, ) \
                               __VA_ARGS__)

// Marks a type as unsafely implementing a marker trait.
//
// This can be applied to a struct, class, or enum.
//
// For example, this C++ header:
//
// ```c++
// struct
//   CRUBIT_UNSAFE_IMPL("Send") MyStruct {
//     bool enable_foo;
// };
// ```
//
// Becomes this Rust interface:
//
// ```rust
// pub struct MyStruct {
//   enable_foo: bool,
// }
//
// unsafe impl Send for MyStruct {}
// ```
#define CRUBIT_UNSAFE_IMPL(...)                                         \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_unsafe_impl" __VA_OPT__(, ) \
                               __VA_ARGS__)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATIONS_H_
