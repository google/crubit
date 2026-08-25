// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Rust bindings for the C++ standard library (`std`).
//!
//! To use C++ standard library types from Rust, depend on `//support:cpp_std`
//! in `deps` (not `cc_deps`) and use types such as `cpp_std::unique_ptr<T>`.

pub use cc_std::std::*;
