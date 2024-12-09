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

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATIONS_H_