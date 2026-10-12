// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Unified runtime support library for Crubit generated bindings.

#![no_std]

// TODO(b/572850571): Evaluate splitting runtime support libraries based on global
// allocator requirements (alloc vs pure core).
extern crate alloc;

pub mod array;
pub mod bridge;
pub mod dyn_erased_future;
pub mod erased_future;
pub mod hash;
