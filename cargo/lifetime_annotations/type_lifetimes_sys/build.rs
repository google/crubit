// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated build.rs for the cc_library type_lifetimes.

const PATH_TO_SRC_ROOT: &str = "../../..";

fn main() {
    crubit_build::compile_cc_lib(PATH_TO_SRC_ROOT, SOURCES).unwrap();
}
const SOURCES: &[&str] =
    &["lifetime_annotations/function_lifetimes.cc", "lifetime_annotations/type_lifetimes.cc"];
