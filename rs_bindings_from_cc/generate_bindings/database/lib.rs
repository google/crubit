// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Definitions of types and database APIs used to generate Rust bindings from C++ APIs.

pub mod code_snippet;
pub mod db;
pub mod function_types;
pub mod rs_snippet;
pub use db::{BindingsGenerator, Database};
