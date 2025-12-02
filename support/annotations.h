// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATIONS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATIONS_H_

#include "support/annotations_internal.h"

// Marks a function or type as requiring Rust binding.
//
// If Crubit fails to generate bindings for a function or type annotated with
// `CRUBIT_MUST_BIND`, bindings generation for the entire target will fail
// with a hard error.
//
// This can be useful when developing C++ API surfaces that are intended to be
// used from Rust, as it produces a clear error message when a function is
// missing bindings.
//
// For example, this C++ header will silently not produce bindings for
// `foo` because overloads are not supported.
//
// ```c++
// void foo();
// void foo(int x);
// void bar();
// ```
//
// This default behavior allows `bar` to still receive bindings.
//
// By contrast, this C++ header will fail at binding generation time with an
// error message describing that overloads are not supported:
//
// ```c++
// CRUBIT_MUST_BIND void foo();
// void foo(int x);
// void bar();
// ```
//
// The annotation can also be applied to a type:
//
// ```c++
// struct CRUBIT_MUST_BIND Foo {
//   int x;
// };
// ```
#define CRUBIT_MUST_BIND CRUBIT_INTERNAL_ANNOTATE("crubit_must_bind")

// Prevents a function or type from receiving Rust bindings.
//
// Use of this annotation should be avoided where-possible as usage of it can
// prevent Rust from accessing useful C++ functionality.
//
// Declarations using this annotation must be registered in the
// `do_not_bind_allowlist` or bindings generation will fail with a hard error.
#define CRUBIT_DO_NOT_BIND CRUBIT_INTERNAL_ANNOTATE("crubit_do_not_bind")

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
// CRUBIT_RUST_NAME("foo")
// int bar() { return 42; }
// ```
//
// Becomes this Rust interface:
//
// ```rust
// pub fn foo() -> i32;  // returns 42
// ```
//
// There's a special case for operators: when an operator symbol is given, for
// example `CRUBIT_RUST_NAME("!")`, the function is mapped to the corresponding
// Rust operator trait, in this case `core::ops::Not`.
//
// ```c++
// CRUBIT_RUST_NAME("!")
// MyInt operator~(MyInt x);
// ```
//
// ```rust
// impl core::ops::Not for MyInt {
//   type Output = Self;
//
//   fn not(self) -> Self::Output { /* calls operator~ */ }
// }
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

// Marks a type as unsafely implementing one or more marker traits.
//
// This can be applied to a struct, class, or enum.
//
// For example, this C++ header:
//
// ```c++
// struct
//   CRUBIT_UNSAFE_IMPL("Send", "Sync") MyStruct {
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
// unsafe impl Sync for MyStruct {}
// ```
#define CRUBIT_UNSAFE_IMPL(...)                                         \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_unsafe_impl" __VA_OPT__(, ) \
                               __VA_ARGS__)

// Declare a Rust type as the bridge type for binding generation.
//
// This can be applied to a struct, class, or enum.
//
// Let's walk through an example, starting with the BUILD file:
//
// ```bzl
// cc_library(
//   name = "example",
//   hdrs = ["example.h"],
//   additional_rust_srcs_for_crubit_bindings = [":additional_example_src"],
// )
//
// additional_rust_srcs_for_crubit_bindings(
//     name = "additional_example_src",
//     srcs = ["additional_example_src.rs"],
// )
//
// rust_library(
//     name = "user",
//     srcs = ["user.rs"],
//     cc_deps = [":example"],
// )
// ```
//
// There are two main targets: :example and :additional_example_src. If the Rust
// side of the bridge type isn't provided by Rust std, then it must be provided
// in the additional_rust_srcs_for_crubit_bindings. :user is simply a Rust
// library that shows how a Rust library can consume the C++ bindings and its
// bridge type.
//
// Here's the C++ bridge type with the annotation, as well as a function that
// returns it:
//
// ```c++
// // example.h
// struct CRUBIT_BRIDGE_VOID_CONVERTERS("MyRustStruct", "rust_to_cpp",
// "cpp_to_rust")
//   MyCppStruct {
//     std::string name;
// };
// MyCppStruct foo();
// ```
//
// For this example, we'll make a native Rust struct that contains a Rust String
// field, defined in additional_example_src.rs:
//
// ```rust
// // additional_example_src.rs
// pub struct MyRustStruct {
//   pub name: String,
// }
// ```
//
// With the provided BUILD configuration above, we can now use the bridge type
// and methods that use the bridge type in Rust.
//
// ```rust
// // user.rs
// pub fn print_foo() {
//   let s: example::MyRustStruct = example::foo();
//   println!("{}", s.name);
// }
// ```
//
// `ty` must be a path to a Rust type. If it starts with `::`, it will be
// treated as a fully-qualified path, i.e. it can refer to types defined outside
// of (but still visible to) the current build target. Otherwise, it will assume
// the Rust type is defined in the Rust bindings by prefixing it with `crate::`
// in all references, which requires defining the type in an
// `additional_rust_srcs_for_crubit_bindings`.
//
// SAFETY:
//   * `rust_to_cpp` must be a valid function name, and its signature must be
//     `void rust_to_cpp (void* rust_struct, MyCppStruct* cpp_struct)`.
//   * `cpp_to_rust` must be valid function name and its signature must be
//     `void cpp_to_rust (MyCppStruct* cpp_struct, void* rust_struct)`.
#define CRUBIT_BRIDGE_VOID_CONVERTERS(ty, ...) \
  CRUBIT_INTERNAL_BRIDGE_SUPPORT(ty __VA_OPT__(, ) __VA_ARGS__)

#define CRUBIT_BRIDGE(rust_name, abi_rust, abi_cpp)              \
  CRUBIT_INTERNAL_ANNOTATE("crubit_bridge_rust_name", rust_name) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_bridge_abi_rust", abi_rust)   \
  CRUBIT_INTERNAL_ANNOTATE("crubit_bridge_abi_cpp", abi_cpp)

// Prevents Crubit from interpreting one or more named attributes on this
// declaration.
//
// TODO: b/444508414 - This currently only ignores unknown attributes, but
// should be extended to ignore attributes that Crubit interprets.
//
// ```c++
// // example.h
// // SAFETY: `my_attribute` does not affect ABI.
// struct CRUBIT_UNSAFE_IGNORE_ATTR(my_attribute) [[my_attribute]] MyStruct {};
// ```
#define CRUBIT_UNSAFE_IGNORE_ATTR(name) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_unsafe_ignore_attr", #name)

// The CRUBIT_OWNED_POINTER AND CRUBIT_OWNED_POINTEE annotations work together
// to map conventionally "owned" C++ pointer usages to a Rust type that provides
// Rust-style ownership wrapping a raw pointer.
//
// Think carefully before using these annotations. These annotations exist to
// support existing APIs that use raw pointers directly, and can not be
// modified; if your API can be improved to avoid dealing with raw pointers (for
// example, by using the managed C++ pointer types that crubit supports) that's
// almost certainly the preferable solution.
//
// Types annotated with `CRUBIT_OWNED_PTR` indicate a transfer of ownership: for
// example, in the return position, they indicate that the function is passing
// ownership of the pointed-to object to the caller. Similarly, in a parameter
// position, the callee receiving ownership of the pointed-to object.
//
// The annotation is only meaningful on pointer types. When a pointer is
// annotated with `CRUBIT_OWNED_PTR`, the pointee type must also be annotated
// with `CRUBIT_OWNED_PTR_TYPE`, indicating the Rust type that will manage the
// ownership of the object in Rust bindings.
//
// The annotation pair is meant to associate Rust types that simply contain a
// pointer to the associated C++ type.
//
// For example:
//
// ```c++
// struct CRUBIT_OWNED_POINTEE("WrapperTypeName") MyType;
//
// MyType* CRUBIT_OWNED_POINTER ReturnOwnedPtr() { ... }
// void AcceptOwnedPtr(MyType* CRUBIT_OWNED_POINTER owned_ptr) { ... }
// ```
//
// This will generate a Rust struct called `WrapperTypeName` that simply
// contains a pointer to the underlying object. This type will be used in
// positions that are annotated with `CRUBIT_OWNED_POINTER`.

#define CRUBIT_OWNED_POINTER \
  CRUBIT_INTERNAL_ANNOTATE_TYPE("crubit_owned_pointer")
#define CRUBIT_OWNED_POINTEE(name) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_owned_pointee", name)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATIONS_H_
