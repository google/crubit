#![cfg(test)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use anyhow::Result;
use ir::*;
use ir_testing::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::Iterator;

// TODO(mboehme): If we start needing to match on parts of the IR in tests,
// check out the crate https://crates.io/crates/galvanic-assert.

fn assert_cc_produces_ir_items_ignoring_decl_ids(cc_src: &str, mut expected: Vec<Item>) {
    let actual = ir_from_cc(cc_src).unwrap();

    for (ref mut expected_item, actual_item) in expected.iter_mut().zip(actual.items()) {
        // TODO(hlopko): Handle MappedTypes as well.
        match (expected_item, actual_item) {
            (Item::Func(ref mut expected_func), Item::Func(actual_func)) => {
                expected_func.decl_id = actual_func.decl_id;
            }
            (Item::Record(ref mut expected_record), Item::Record(actual_record)) => {
                expected_record.decl_id = actual_record.decl_id;
            }
            (_, _) => (),
        }
    }

    assert_eq!(actual.items().collect_vec(), expected.iter().collect_vec());
}

#[test]
fn test_function() {
    assert_cc_produces_ir_items_ignoring_decl_ids(
        "int Add(int a, int b);",
        vec![Item::Func(Func {
                name: UnqualifiedIdentifier::Identifier(ir_id("Add")),
                decl_id: /* ignored */ DeclId(42),
                owning_target: "//test:testing_target".into(),
                mangled_name: "_Z3Addii".to_string(),
                doc_comment: None,
                return_type: ir_int(),
                params: vec![ir_int_param("a"), ir_int_param("b")],
                is_inline: false,
                member_func_metadata: None,
            })],
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
    let type_mapping: HashMap<_, _> = fields
        .map(|f| {
            (
                f.type_.cc_type.name.as_ref().unwrap().as_str(),
                f.type_.rs_type.name.as_ref().unwrap().as_str(),
            )
        })
        .collect();

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

#[test]
fn test_member_function_params() {
    let ir = ir_from_cc(
        r#"
            struct Struct {
                void Foo(int x, int y);
            };
        "#,
    )
    .unwrap();
    let foo_func =
        ir.functions().find(|f| f.name == UnqualifiedIdentifier::Identifier(ir_id("Foo"))).unwrap();
    let param_names: Vec<_> = foo_func.params.iter().map(|p| &p.identifier.identifier).collect();
    assert_eq!(param_names, vec!["__this", "x", "y"]);
}

fn assert_member_function_has_instance_method_metadata(
    name: &str,
    definition: &str,
    expected_metadata: &Option<ir::InstanceMethodMetadata>,
) {
    let mut file = String::new();
    file += "struct Struct {\n  ";
    file += definition;
    file += "\n};";
    let ir = ir_from_cc(&file).unwrap();

    let function =
        ir.functions().find(|f| f.name == UnqualifiedIdentifier::Identifier(ir_id(name)));
    let meta = function
        .expect("Function not found")
        .member_func_metadata
        .as_ref()
        .expect("Member function should specify member_func_metadata");
    assert_eq!(&meta.for_type.identifier, "Struct");
    assert_eq!(&meta.instance_method_metadata, expected_metadata);
}

#[test]
fn test_member_function_static() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "static void Function();",
        &None,
    );
}

#[test]
fn test_member_function() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "void Function();",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::Unqualified,
            is_const: false,
            is_virtual: false,
        }),
    );
}

#[test]
fn test_member_function_const() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "void Function() const;",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::Unqualified,
            is_const: true,
            is_virtual: false,
        }),
    );
}

// TODO(b/202853028): Support virtual functions.
#[test]
fn test_member_function_virtual() {
    let file = r#"
    struct Struct {
        virtual void Function();
    };
    "#;
    let ir = ir_from_cc(file).unwrap();

    for func in ir.functions() {
        assert!(func.name != UnqualifiedIdentifier::Identifier(ir_id("Function")));
    }
}

#[test]
fn test_member_function_lvalue() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "void Function() &;",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::LValue,
            is_const: false,
            is_virtual: false,
        }),
    );
}

#[test]
fn test_member_function_rvalue() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "void Function() &&;",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::RValue,
            is_const: false,
            is_virtual: false,
        }),
    );
}

fn get_func_names(definition: &str) -> Vec<ir::UnqualifiedIdentifier> {
    let ir = ir_from_cc(definition).unwrap();
    ir.functions().map(|f| f.name.clone()).collect()
}

#[test]
fn test_identifier_function_name() {
    assert_eq!(
        get_func_names("void Function();"),
        vec![ir::UnqualifiedIdentifier::Identifier(ir::Identifier {
            identifier: "Function".into()
        })],
    );
}

#[test]
fn test_constructor_function_name() {
    assert_eq!(
        get_func_names("struct Struct {Struct();};"),
        vec![ir::UnqualifiedIdentifier::Constructor],
    );
}

#[test]
fn test_destructor_function_name() {
    assert!(
        get_func_names("struct Struct {~Struct();};")
            .contains(&ir::UnqualifiedIdentifier::Destructor)
    );
}
