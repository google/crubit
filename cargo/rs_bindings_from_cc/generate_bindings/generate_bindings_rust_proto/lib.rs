// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated lib.rs for the proto crate.

pub use rs_bindings_from_cc_ir_rust_proto as ir;

#[allow(unused_imports)]
pub mod generate_bindings {
    include!(concat!(env!("OUT_DIR"), "/generate_bindings.rs"));
}
pub use generate_bindings::*;
