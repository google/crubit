// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Utilities for generating IR for the `multiplatformat_testing::test_platform` in tests.

use arc_anyhow::Result;
use ir::{Record, IR};
use multiplatform_testing::test_platform;

pub fn ir_from_cc(header: &str) -> Result<IR> {
    ir_testing::ir_from_cc(test_platform(), header)
}

pub fn ir_from_cc_dependency(header: &str, dep_header: &str) -> Result<IR> {
    ir_testing::ir_from_cc_dependency(test_platform(), header, dep_header)
}

pub fn ir_record(name: &str) -> Record {
    ir_testing::ir_record(test_platform(), name)
}
