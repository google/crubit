// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_SUPPORT_INTERNAL_ATTRIBUTES_H_
#define CRUBIT_SUPPORT_INTERNAL_ATTRIBUTES_H_

#include "absl/base/attributes.h"

// Style waiver granted in <internal link>
#if ABSL_HAVE_CPP_ATTRIBUTE(clang::annotate) && \
    ABSL_HAVE_CPP_ATTRIBUTE(clang::annotate_type)
#define CRUBIT_INTERNAL_ANNOTATE(...) [[clang::annotate(__VA_ARGS__)]]
#define CRUBIT_INTERNAL_ANNOTATE_TYPE(...) [[clang::annotate_type(__VA_ARGS__)]]
#else
#define CRUBIT_INTERNAL_ANNOTATE(...)
#define CRUBIT_INTERNAL_ANNOTATE_TYPE(...)
#endif

// By default, <internal link> will infer Rust safety based on the types of the
// function's parameters. This annotation can be used to override that
// inference.
#define CRUBIT_OVERRIDE_UNSAFE(function_is_unsafe) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_override_unsafe", function_is_unsafe)

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

#define CRUBIT_INTERNAL_BRIDGE_TYPE(t) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_bridge_type", t)

#define CRUBIT_INTERNAL_BRIDGE_TYPE_RUST_TO_CPP_CONVERTER(t) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_bridge_type_rust_to_cpp_converter", t)

#define CRUBIT_INTERNAL_BRIDGE_TYPE_CPP_TO_RUST_CONVERTER(t) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_bridge_type_cpp_to_rust_converter", t)

// Declare a Rust type as the bridge type for binding generation.
//
// This can be applied to a struct, class, or enum.
//
// For example, We have the following C++ struct and its Rust bridge
// counterpart:
//
// ```c++
// struct
//   CRUBIT_INTERNAL_BRIDGE_SUPPORT("MyRustStruct", "rust_to_cpp",
//   "cpp_to_rust") MyCppStruct {
//     std::string name;
// };
// ```
//
// ```rust
// pub struct MyRustStruct {
//   name: String,
// }
// ```
// Then the following C++ api will be translated to Rust:
//
// ```c++
// MyCppStruct foo();
// ```
//
// ```rust
// pub fn foo() -> MyRustStruct;
// ```
//
// SAFETY:
//   * `ty` must be a fully-qualified valid Rust type name.
//   * `rust_to_cpp` must be a valid function name, and its signature must be
//     `void rust_to_cpp (void* rust_struct, MyCppStruct* cpp_struct)`.
//   * `cpp_to_rust` must be valid function name and its signature must be
//     `void cpp_to_rust (MyCppStruct* cpp_struct, void* rust_struct)`.
#define CRUBIT_INTERNAL_BRIDGE_SUPPORT(ty, rust_to_cpp, cpp_to_rust) \
  CRUBIT_INTERNAL_BRIDGE_TYPE(ty)                                    \
  CRUBIT_INTERNAL_BRIDGE_TYPE_RUST_TO_CPP_CONVERTER(rust_to_cpp)     \
  CRUBIT_INTERNAL_BRIDGE_TYPE_CPP_TO_RUST_CONVERTER(cpp_to_rust)

// Mark a type as deriving a trait.
//
// This can be applied to a struct, class, or enum.
//
// For example, we have the following C++ struct and its Rust bridge
// counterpart:
//
// ```c++
// struct
//   CRUBIT_TRAIT_DERIVE("Copy", "Debug") MyCppStruct {
//     bool enable_foo;
// };
// ```
//
// ```rust
// #[derive(Copy, Debug)]
// pub struct MyRustStruct {
//   enable_foo: bool,
// }
// ```
#define CRUBIT_TRAIT_DERIVE(...)                                         \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_trait_derive" __VA_OPT__(, ) \
                               __VA_ARGS__)

#endif  // CRUBIT_SUPPORT_INTERNAL_ATTRIBUTES_H_
