#![cfg(test)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use anyhow::Result;
use ffi_types::{FfiU8Slice, FfiU8SliceBox};
use ir::*;
use ir_testing::*;

extern "C" {
    fn json_from_cc(cc_source: FfiU8Slice) -> FfiU8SliceBox;
}

// TODO(mboehme): If we start needing to match on parts of the IR in tests,
// check out the crate https://crates.io/crates/galvanic-assert.

fn ir_from_cc(src: &str) -> Result<IR> {
    let src_u8 = src.as_bytes();
    let json_utf8 = unsafe { json_from_cc(FfiU8Slice::from_slice(src_u8)).into_boxed_slice() };
    deserialize_ir(&*json_utf8)
}

fn assert_cc_produces_ir(cc_src: &str, mut expected: IR) {
    let actual = ir_from_cc(cc_src).unwrap();

    // ir_from_cc() always sets `used_headers` this way, so add it to the
    // expected IR.
    expected.used_headers = vec![HeaderName { name: "test/testing_header_0.h".to_string() }];

    assert_eq!(actual, expected);
}

#[test]
fn test_function() {
    assert_cc_produces_ir(
        "int Add(int a, int b);",
        IR {
            items: vec![Item::Func(Func {
                identifier: ir_id("Add"),
                mangled_name: "_Z3Addii".to_string(),
                doc_comment: None,
                return_type: ir_int(),
                params: vec![ir_int_param("a"), ir_int_param("b")],
                is_inline: false,
            })],
            ..Default::default()
        },
    );
}
