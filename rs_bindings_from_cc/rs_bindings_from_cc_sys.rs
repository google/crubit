// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::ffi::{c_char, c_int};

extern "C" {
    pub fn crubit_rs_bindings_from_cc_main(argc: c_int, argv: *mut *mut c_char) -> std::ffi::c_int;
}

// Dependencies from C++ code, avoid cargo/rustc pruning them.
//
// TODO(danakj): How do we keep cargo/rustc from pruning these dependencies from
// the linker line without adding these statements? Bazel does not have this
// issue.
#[cfg(feature = "crubit_cargo_build")]
extern crate collect_instantiations;
#[cfg(feature = "crubit_cargo_build")]
extern crate common_sys;
#[cfg(feature = "crubit_cargo_build")]
extern crate generate_bindings;
#[cfg(feature = "crubit_cargo_build")]
extern crate lifetime_analysis_sys;
#[cfg(feature = "crubit_cargo_build")]
extern crate lifetime_annotations_sys;
