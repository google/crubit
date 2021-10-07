#![cfg(test)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use anyhow::Result;
use ffi_types::{FfiU8Slice, FfiU8SliceBox};
use ir::*;
use ir_testing::*;
use std::collections::HashMap;

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

#[test]
fn test_doc_comment() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            /// Doc comment
            ///
            ///  * with three slashes
            struct DocCommentSlashes {};

            //! Doc comment
            //!
            //!  * with slashes and bang
            struct DocCommentBang {};

            /** Multiline comment

                * with two stars */
            struct MultilineCommentTwoStars {};

            // Line comment
            //
            //  * with two slashes
            struct LineComment {};

            /* Multiline comment

                * with one star */
            struct MultilineOneStar {};
        "#,
    )?;
    let comments: HashMap<_, _> = ir
        .records()
        .map(|r| (r.identifier.identifier.as_str(), r.doc_comment.as_ref().unwrap()))
        .collect();

    assert_eq!(comments["DocCommentSlashes"], "Doc comment\n\n * with three slashes");
    assert_eq!(comments["DocCommentBang"], "Doc comment\n\n * with slashes and bang");

    // TODO(forster): The bullet point is not retained in this
    // case. Instead we get the space at the end. Not sure if this
    // can be fixed easily...
    assert_eq!(comments["MultilineCommentTwoStars"], "Multiline comment\n\n with two stars ");
    assert_eq!(comments["LineComment"], "Line comment\n\n * with two slashes");

    // TODO(forster): The bullet point is not retained in this
    // case. Instead we get the space at the end. Not sure if this
    // can be fixed easily...
    assert_eq!(comments["MultilineOneStar"], "Multiline comment\n\n with one star ");

    Ok(())
}
