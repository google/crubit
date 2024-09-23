// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

const PATH_TO_SRC_ROOT: &str = "../../..";

fn main() {
    crubit_build::compile_cc_lib(PATH_TO_SRC_ROOT, SOURCES).unwrap();
}

// TODO(danakj): Pull this out of the BUILD somehow?
//
// TODO(danakj): Split these up into separate Cargo targets so incremental
// builds of C++ changes are fast?
const SOURCES: &[&str] = &[
    "lifetime_annotations/function_lifetimes.cc",
    "lifetime_annotations/lifetime_annotations.cc",
    "lifetime_annotations/lifetime.cc",
    "lifetime_annotations/lifetime_substitutions.cc",
    "lifetime_annotations/lifetime_symbol_table.cc",
    "lifetime_annotations/pointee_type.cc",
    "lifetime_annotations/type_lifetimes.cc",
];
