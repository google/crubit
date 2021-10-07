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

#[test]
fn test_type_conversion() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            #include <stdint.h>
            #include <stddef.h>

            struct S {
                bool b;

                char c;
                unsigned char uc;
                signed char sc;
                char16_t c16;
                char32_t c32;
                wchar_t wc;

                short s;
                int i;
                long l;
                long long ll;

                unsigned short us;
                unsigned int ui;
                unsigned long ul;
                unsigned long long ull;

                signed short ss;
                signed int si;
                signed long sl;
                signed long long sll;

                int8_t i8;
                int16_t i16;
                int32_t i32;
                int64_t i64;

                uint8_t u8;
                uint16_t u16;
                uint32_t u32;
                uint64_t u64;

                ptrdiff_t pt;
                size_t st;
                intptr_t ip;
                uintptr_t up;

                float f;
                double d;
            };
        "#,
    )?;
    let fields = ir.records().next().unwrap().fields.iter();
    let type_mapping: HashMap<_, _> =
        fields.map(|f| (f.type_.cc_type.name.as_str(), f.type_.rs_type.name.as_str())).collect();

    assert_eq!(type_mapping["bool"], "bool");

    assert_eq!(type_mapping["char"], "i8");
    assert_eq!(type_mapping["unsigned char"], "u8");
    assert_eq!(type_mapping["signed char"], "i8");
    assert_eq!(type_mapping["char16_t"], "u16");
    // We cannot map C++ char32_t or wchar_t to Rust char,
    // because Rust requires that chars are valid UTF scalar values.
    assert_eq!(type_mapping["char32_t"], "u32");
    assert_eq!(type_mapping["wchar_t"], "i32");

    assert_eq!(type_mapping["short"], "i16");
    assert_eq!(type_mapping["int"], "i32");
    assert_eq!(type_mapping["long"], "i64");
    assert_eq!(type_mapping["long long"], "i64");

    assert_eq!(type_mapping["unsigned short"], "u16");
    assert_eq!(type_mapping["unsigned int"], "u32");
    assert_eq!(type_mapping["unsigned long"], "u64");
    assert_eq!(type_mapping["unsigned long long"], "u64");

    assert_eq!(type_mapping["short"], "i16");
    assert_eq!(type_mapping["int"], "i32");
    assert_eq!(type_mapping["long"], "i64");
    assert_eq!(type_mapping["long long"], "i64");

    assert_eq!(type_mapping["int8_t"], "i8");
    assert_eq!(type_mapping["int16_t"], "i16");
    assert_eq!(type_mapping["int32_t"], "i32");
    assert_eq!(type_mapping["int64_t"], "i64");

    assert_eq!(type_mapping["uint8_t"], "u8");
    assert_eq!(type_mapping["uint16_t"], "u16");
    assert_eq!(type_mapping["uint32_t"], "u32");
    assert_eq!(type_mapping["uint64_t"], "u64");

    assert_eq!(type_mapping["ptrdiff_t"], "isize");
    assert_eq!(type_mapping["size_t"], "usize");
    assert_eq!(type_mapping["intptr_t"], "isize");
    assert_eq!(type_mapping["uintptr_t"], "usize");

    assert_eq!(type_mapping["float"], "f32");
    assert_eq!(type_mapping["double"], "f64");

    Ok(())
}
