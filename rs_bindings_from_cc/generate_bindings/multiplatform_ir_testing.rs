// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Utilities for generating IR for the `multiplatformat_testing::test_platform` in tests.

use arc_anyhow::Result;
use ir::{Record, IR};
use ir_testing::with_full_lifetime_macros;
use multiplatform_testing::test_platform;

pub fn ir_from_cc(header: &str) -> Result<IR> {
    ir_testing::ir_from_cc(test_platform(), header)
}

pub fn ir_from_cc_dependency(header: &str, dep_header: &str) -> Result<IR> {
    ir_testing::ir_from_cc_dependency(test_platform(), header, dep_header, None)
}

pub fn ir_record(name: &str) -> Record {
    ir_testing::ir_record(test_platform(), name)
}

pub fn ir_from_assumed_lifetimes_cc(program: &str) -> Result<IR> {
    let mut full_program = with_full_lifetime_macros();
    full_program.push_str(program);
    ir_testing::ir_from_cc_dependency(
        test_platform(),
        &full_program,
        "// empty header",
        Some("assume_lifetimes"),
    )
}

pub fn ir_from_fmt_cc(program: &str) -> Result<IR> {
    ir_testing::ir_from_cc_dependency(
        multiplatform_testing::test_platform(),
        program,
        "// empty header",
        Some("fmt"),
    )
}
