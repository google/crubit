// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a dependency of `test_api.rs` - types exported by
//! `other_crate.rs` are used in public API exposed by `test_api.rs`.

pub struct SomeStruct(pub i32);
