// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated build.rs for the cc_library existing_rust_type.

const PATH_TO_SRC_ROOT: &str = "../../../..";

fn main() {
    crubit_build::compile_cc_lib(PATH_TO_SRC_ROOT, SOURCES).unwrap();
}
const SOURCES: &[&str] = &["rs_bindings_from_cc/importers/existing_rust_type.cc"];
