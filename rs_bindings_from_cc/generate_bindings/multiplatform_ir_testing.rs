// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Utilities for generating IR for the `multiplatformat_testing::test_platform` in tests.

use arc_anyhow::Result;
use ir_rust_proto::IRProto;
use ir_testing::with_full_lifetime_macros;
use multiplatform_testing::test_platform;

pub fn ir_proto_from_cc(header: &str) -> Result<IRProto> {
    ir_testing::ir_proto_from_cc(test_platform(), header)
}

pub fn ir_proto_from_cc_annotated(header: &str) -> Result<IRProto> {
    ir_testing::ir_proto_from_cc_annotated(test_platform(), header)
}

pub fn ir_proto_from_cc_dependency(header: &str, dep_header: &str) -> Result<IRProto> {
    ir_testing::ir_proto_from_cc_dependency(
        test_platform(),
        header,
        dep_header,
        None,
        /*kythe_annotations=*/ false,
        /*carcinize=*/ false,
    )
}

pub fn ir_proto_from_cc_with_inline_cpp(header: &str) -> Result<IRProto> {
    ir_testing::ir_proto_from_cc_dependency(
        test_platform(),
        header,
        "// empty header",
        None,
        /*kythe_annotations=*/ false,
        /*carcinize=*/ true,
    )
}

pub fn ir_proto_from_assumed_lifetimes_cc(program: &str) -> Result<IRProto> {
    let mut full_program = with_full_lifetime_macros();
    full_program.push_str(program);
    ir_testing::ir_proto_from_cc_dependency(
        test_platform(),
        &full_program,
        "// empty header",
        Some("assume_lifetimes"),
        /*kythe_annotations=*/ false,
        /*carcinize=*/ false,
    )
}
