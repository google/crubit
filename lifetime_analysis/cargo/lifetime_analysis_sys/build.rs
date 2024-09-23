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
    "lifetime_analysis/analyze.cc",
    "lifetime_analysis/builtin_lifetimes.cc",
    "lifetime_analysis/lifetime_analysis.cc",
    "lifetime_analysis/lifetime_constraints.cc",
    "lifetime_analysis/lifetime_lattice.cc",
    "lifetime_analysis/object.cc",
    "lifetime_analysis/object_repository.cc",
    "lifetime_analysis/object_set.cc",
    "lifetime_analysis/pointer_compatibility.cc",
    "lifetime_analysis/points_to_map.cc",
    "lifetime_analysis/template_placeholder_support.cc",
];
