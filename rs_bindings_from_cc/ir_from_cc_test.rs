// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![cfg(test)]

use arc_anyhow::Result;
use googletest::prelude::*;
use ir::*;
use ir_matchers::{assert_ir_matches, assert_ir_not_matches, assert_items_match};
use ir_testing::{ir_id, retrieve_func, retrieve_record, DEPENDENCY_TARGET, TESTING_TARGET};
use itertools::Itertools;
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::rc::Rc;

fn ir_from_cc(header: &str) -> Result<IR> {
    ir_testing::ir_from_cc(multiplatform_testing::test_platform(), header)
}

fn ir_from_cc_dependency(header: &str, dep_header: &str) -> Result<IR> {
    ir_testing::ir_from_cc_dependency(multiplatform_testing::test_platform(), header, dep_header)
}

#[gtest]
fn test_function() {
    let ir = ir_from_cc("int f(int a, int b);").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f",
                owning_target: BazelLabel("//test:testing_target"),
                mangled_name: "_Z1fii",
                doc_comment: None,
                return_type: CcType {
                    variant: Primitive(Int),
                    is_const: false, ...
                },
                params: [
                    FuncParam {
                        type_: CcType {
                            variant: Primitive(Int),
                            is_const: false, ...
                        },
                        identifier: "a",
                        unknown_attr: None,
                    },
                    FuncParam {
                        type_: CcType {
                            variant: Primitive(Int),
                            is_const: false, ...
                        },
                        identifier: "b",
                        unknown_attr: None,
                    },
                ],
                lifetime_params: [],
                is_inline: false,
                member_func_metadata: None,
                is_extern_c: false,
                is_noreturn: false,
                is_variadic: false,
                is_consteval: false,
                nodiscard: None,
                deprecated: None,
                unknown_attr: None,
                has_c_calling_convention: true,
                is_member_or_descendant_of_class_template: false,
                safety_annotation: Unannotated,
                source_loc: "Generated from: ir_from_cc_virtual_header.h;l=3",
                id: ItemId(...),
                enclosing_item_id: None,
                adl_enclosing_record: None,
                must_bind: false,
            }
        }
    );
}

#[gtest]
fn test_function_with_asm_label() {
    let ir = ir_from_cc("int f(int a, int b) asm(\"foo\");").unwrap();
    match multiplatform_testing::test_platform() {
        // If a declaration uses an asm label, the Clang mangler adds a '\u{1}' prefix on some
        // platforms to signify that LLVM should not perform any LLVM-level mangling on it.
        multiplatform_testing::Platform::ArmMacOS | multiplatform_testing::Platform::X86MacOS => {
            assert_ir_matches!(
                ir,
                quote! {
                    Func {
                        cc_name: "f",
                        rs_name: "f", ...
                        mangled_name: "\u{1}foo", ...
                    }
                }
            )
        }
        _ => assert_ir_matches!(
            ir,
            quote! {
                Func {
                    cc_name: "f",
                    rs_name: "f", ...
                    mangled_name: "foo", ...
                }
            }
        ),
    }
}

#[gtest]
fn test_function_with_unnamed_parameters() {
    let ir = ir_from_cc("int f(int, int);").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                mangled_name: "_Z1fii", ...
                params: [
                    FuncParam {
                        ... identifier: "__param_0", ...
                    },
                    FuncParam {
                        ... identifier: "__param_1", ...
                    },
                ], ...
            }
        }
    );
}

#[gtest]
fn test_unescapable_rust_keywords_in_function_parameters() {
    let ir = ir_from_cc("int f(int self, int crate, int super);").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Func {
                cc_name: "f",
                rs_name: "f", ...
                params: [
                    FuncParam {
                        ... identifier: "__param_0", ...
                    },
                    FuncParam {
                        ... identifier: "__param_1", ...
                    },
                    FuncParam {
                        ... identifier: "__param_2", ...
                    },
                ], ...
            }
        }
    );
}

#[gtest]
fn test_unescapable_rust_keywords_in_struct_name() {
    let ir = ir_from_cc("struct Self{ int field; };").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "Self", ...
            errors: [FormattedError {
                ... message: "Record name is not supported: Unescapable identifier: Self", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_unescapable_rust_keywords_in_enum_name() {
    let ir = ir_from_cc("enum Self{ kFoo = 1 };").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "Self", ...
            errors: [FormattedError {
                ... message: "Enum name is not supported: Unescapable identifier: Self", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_unescapable_rust_keywords_in_enumerator_name() {
    let ir = ir_from_cc("enum SomeEnum { self = 1 };").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "SomeEnum", ...
            errors: [FormattedError {
                ..., message: "Enumerator name is not supported: Unescapable identifier: self", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_unescapable_rust_keywords_in_anonymous_struct_type_alias() {
    let ir = ir_from_cc("typedef struct { int field; } Self;").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "Self", ...
            errors: [FormattedError {
                ..., message: "Record name is not supported: Unescapable identifier: Self", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_unescapable_rust_keywords_in_field_name() {
    let ir = ir_from_cc("struct SomeStruct { int self; };").unwrap();
    let record = ir.records().find(|record| record.rs_name == "SomeStruct").unwrap();
    assert_eq!(record.fields.len(), 1);
    let field = &record.fields[0];
    assert_eq!(field.rust_identifier.as_ref().map(|x| x.as_str()), Some("__field_0"));
    assert_eq!(field.cpp_identifier.as_ref().map(|x| x.as_str()), Some("self"));
}

#[gtest]
fn test_unescapable_rust_keywords_in_namespace_name() {
    let ir = ir_from_cc("namespace self { void foo(); }").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "self", ...
            errors: [FormattedError {
                ..., message: "Namespace name is not supported: Unescapable identifier: self", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_unescapable_rust_keywords_in_function_name() {
    let ir = ir_from_cc("void self();").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "self", ...
            errors: [FormattedError {
                ..., message: "Function name is not supported: Unescapable identifier: self", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_unescapable_rust_keywords_in_type_alias_name() {
    let ir = ir_from_cc("using Self = int;").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "Self", ...
            errors: [FormattedError {
                ..., message: "Type alias name is not supported: Unescapable identifier: Self", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_function_with_custom_calling_convention() {
    if multiplatform_testing::test_platform() != multiplatform_testing::Platform::X86Linux {
        return; // vectorcall only exists on x86_64, not e.g. aarch64
    }
    let ir = ir_from_cc("int f_vectorcall(int, int) [[clang::vectorcall]];").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Func {
                cc_name: "f_vectorcall", ...
                mangled_name: "_Z12f_vectorcallii", ...
                has_c_calling_convention: false, ...
            }
        }
    );
}

#[gtest]
fn test_consteval_function() {
    let ir = ir_from_cc("consteval int add(int, int);").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Func {
            cc_name: "add", ...
            is_consteval: true, ...
          }
        }
    );
}

#[gtest]
fn test_variadic_function() {
    let ir = ir_from_cc("int sprintf(char* str, const char* format, ...);").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Func {
            cc_name: "sprintf", ...
            is_variadic: true, ...
          }
        }
    );
}

#[gtest]
fn test_functions_from_dependency_are_not_emitted() -> Result<()> {
    let ir = ir_from_cc_dependency("int Add(int a, int b);", "int Multiply(int a, int b);")?;
    assert_ir_matches!(ir, quote! { Func { cc_name: "Add" ... } });
    assert_ir_not_matches!(ir, quote! { Func { cc_name: "Multiply" ... } });
    Ok(())
}

#[gtest]
fn test_dont_import_record_nested_in_func() {
    let ir = ir_from_cc("inline void f() { struct S{}; }").unwrap();
    assert_ir_not_matches!(ir, quote! { Record { ... "S" ... } });
}

#[gtest]
fn test_explicit_class_template_instantiation_declaration_not_supported_yet() {
    let ir = ir_from_cc(
        "
        template <class T> struct MyTemplate{};
        extern template struct MyTemplate<int>;
      ",
    )
    .unwrap();
    assert_ir_not_matches!(ir, quote! { Record });
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "MyTemplate",
            kind: Class,
            path: Some(UnsupportedItemPath { ident: "MyTemplate", enclosing_item_id: None, }),
            errors: [FormattedError {
                ..., message: "Class templates are not supported yet", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_function_template_not_supported_yet() {
    let ir = ir_from_cc("template<typename SomeParam> void SomeFunctionTemplate() {};").unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "SomeFunctionTemplate",
            kind: Func,
            path: Some(UnsupportedItemPath { ident: "SomeFunctionTemplate", enclosing_item_id: None, }),
            errors: [FormattedError {
                ..., message: "Function templates are not supported yet", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_function_with_rvalue_reference_parameter_without_lifetime_analysis_has_no_lifetime(
) -> googletest::Result<()> {
    let ir = ir_from_cc("void f(int&& a) {};").unwrap();
    let function = ir.functions().find(|func| func.cc_name == "f").or_fail()?;
    let [param] = &function.params[..] else { return fail!("expected exactly one parameter") };
    let &PointerType { kind, lifetime, .. } = param.type_.variant.as_pointer().or_fail()?;
    expect_eq!(kind, PointerTypeKind::RValueRef);
    expect_eq!(lifetime, None);
    Ok(())
}

#[gtest]
fn test_record_member_variable_access_specifiers() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
            int default_access_int;
          public:
            int public_int;
          protected:
            int protected_int;
          private:
            int private_int;
        };

        class SomeClass {
          int default_access_int;
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "SomeStruct", ...
                fields: [
                    Field {
                        rust_identifier: Some("default_access_int") ...
                        access: Public ...
                    },
                    Field {
                        rust_identifier: Some("public_int") ...
                        access: Public ...
                    },
                    Field {
                        rust_identifier: Some("protected_int") ...
                        access: Protected ...
                    },
                    Field {
                        rust_identifier: Some("private_int") ...
                        access: Private ...
                    },
                ] ...
            }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "SomeClass", ...
                fields: [
                    Field {
                        rust_identifier: Some("default_access_int") ...
                        access: Private ...
                    }
                ] ...
            }
        }
    );
}

#[gtest]
fn test_bitfields() {
    let ir = ir_from_cc(
        r#"
        struct Bitfields {
            int b1: 1;
            int b2: 2;
            int b3: 13;
            int b4: 14;
        };"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "Bitfields", ...
                fields: [
                       Field {
                           rust_identifier: Some("b1"), ...
                           type_: Ok(CcType { variant: Primitive(Int), ... }), ...
                           offset: 0,
                           size: 1, ...
                           is_bitfield: true, ...
                       },
                       Field {
                           rust_identifier: Some("b2"), ...
                           type_: Ok(CcType { variant: Primitive(Int), ... }), ...
                           offset: 1,
                           size: 2, ...
                           is_bitfield: true, ...
                       },
                       Field {
                           rust_identifier: Some("b3"), ...
                           type_: Ok(CcType { variant: Primitive(Int), ... }), ...
                           offset: 3,
                           size: 13, ...
                           is_bitfield: true, ...
                       },
                       Field {
                           rust_identifier: Some("b4"), ...
                           type_: Ok(CcType { variant: Primitive(Int), ... }), ...
                           offset: 16,
                           size: 14, ...
                           is_bitfield: true, ...
                       },
                ] ...
            }
        }
    );
}

/// This is a regression test for b/270748945.
#[gtest]
fn test_struct_with_packed_attribute() {
    let ir = ir_from_cc(
        r#"
        struct __attribute__((packed)) PackedStruct {
          char char_var;
          int int_var;
        };"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "PackedStruct", ...
            errors: [FormattedError {
                ..., message: "Records with packed layout are not supported", ...
            }], ...
        }}
    );
}

/// This is a regression test for b/270748945.
#[gtest]
fn test_struct_with_packed_field() {
    let ir = ir_from_cc(
        r#"
        struct PackedStruct {
          char char_var;
          __attribute__((packed)) int int_var;
        };"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "PackedStruct", ...
            errors: [FormattedError {
                ..., message: "Records with packed layout are not supported", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_struct_with_unnamed_bitfield_member() {
    // This test input causes `field_decl->getName()` to return an empty string.
    // This example is based on `struct timex` from bits/timex.h
    let ir = ir_from_cc(
        r#"
        struct WithUnnamedFields {
            int foo;
            int :32;  // <- unnamed bitfield
        };"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "WithUnnamedFields", ...
                fields: [
                    Field { rust_identifier: Some("foo") ... },
                    Field { rust_identifier: None ... },
                ] ...
            }
        }
    );
}

#[gtest]
fn test_struct_with_bridge_type_annotation() {
    let ir = ir_from_cc(
        r#"
        struct [[clang::annotate("crubit_bridge_type", "SomeBridgeType"),
                 clang::annotate("crubit_bridge_type_rust_to_cpp_converter", "rust_to_cpp_converter"),
                 clang::annotate("crubit_bridge_type_cpp_to_rust_converter", "cpp_to_rust_converter")]]
                RecordWithBridgeType {
            int foo;
        };"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "RecordWithBridgeType", ...
                bridge_type: Some(BridgeVoidConverters {
                  rust_name: "SomeBridgeType",
                  rust_to_cpp_converter: "rust_to_cpp_converter",
                  cpp_to_rust_converter: "cpp_to_rust_converter", ...
                }), ...
            }
        }
    );
}

#[gtest]
fn test_struct_with_trait_derive_annotation() {
    let ir = ir_from_cc(
        r#"
        struct [[clang::annotate("crubit_internal_trait_derive", "Debug"),
                 clang::annotate("crubit_internal_unsafe_impl", "Send")]]
                RecordWithDerives {
            int foo;
        };"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "RecordWithDerives", ...
                trait_derives: TraitDerives { ...
                  debug: Positive, ...
                  send: true, ...
                }, ...
            }
        }
    );
}

#[gtest]
fn test_struct_with_unsafe_annotation() {
    let ir = ir_from_cc(
        r#"
        struct [[clang::annotate("crubit_override_unsafe", true)]]
                UnsafeType {
            int foo;
        };"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "UnsafeType", ...
                is_unsafe_type: true, ...
            }
        }
    );
}

#[gtest]
fn test_conflicting_unsafe_annotation() {
    let ir = ir_from_cc(
        r#"
        struct [[clang::annotate("crubit_override_unsafe", true)]] S;
        struct [[clang::annotate("crubit_override_unsafe", false)]] S {
            int foo;
        };
        "#,
    )
    .unwrap();

    assert_ir_not_matches!(ir, quote! { Record { ... "S" ... } });
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
            name: "S",
            kind: Struct,
            path: Some(UnsupportedItemPath { ident: "S", enclosing_item_id: None, }),
            errors: [FormattedError {
                ..., message: "Different declarations have inconsistent `crubit_override_unsafe` annotations.", ...
            }], ...
        }}
    );
}

#[gtest]
fn test_struct_with_unnamed_struct_and_union_members() {
    // This test input causes `field_decl->getName()` to return an empty string.
    // See also:
    // - https://en.cppreference.com/w/c/language/struct: "[...] an unnamed member
    //   of a struct whose type is a struct without name is known as anonymous
    //   struct."
    // - https://rust-lang.github.io/rfcs/2102-unnamed-fields.html
    let ir = ir_from_cc(
        r#"
        struct StructWithUnnamedMembers {
          struct {
            int anonymous_struct_field_1;
            int anonymous_struct_field_2;
          };
          union {
            int anonymous_union_field_1;
            int anonymous_union_field_2;
          };
        }; "#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "StructWithUnnamedMembers" ...
                cc_name: "StructWithUnnamedMembers" ...
                fields: [
                    Field {
                        rust_identifier: None, ...
                        type_ : Err(...), ...
                        offset: 0, ...
                    } ...
                    Field {
                        rust_identifier: None, ...
                        type_ : Err(...), ...
                        offset: 64, ...
                    } ...
                ], ...
                size_align: SizeAlign {
                    size: 12,
                    alignment: 4,
                } ...
            }
        }
    );
}

#[gtest]
fn test_record_private_member_functions_not_present() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          public:
            int public_method();
          protected:
            int protected_method();
          private:
            int private_method();
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(ir, quote! { Func { cc_name: "public_method" ... } });
    assert_ir_not_matches!(ir, quote! { Func { cc_name: "protected_method" ... } });
    assert_ir_not_matches!(ir, quote! { Func { cc_name: "private_method" ... } });
}

#[gtest]
fn test_record_private_static_member_functions_not_present() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          public:
            static int public_method();
          protected:
            static int protected_method();
          private:
            static int private_method();
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(ir, quote! { Func { cc_name: "public_method" ... } });
    assert_ir_not_matches!(ir, quote! { Func { cc_name: "protected_method" ... } });
    assert_ir_not_matches!(ir, quote! { Func { cc_name: "private_method" ... } });
}

#[gtest]
fn test_record_special_member_access_specifiers() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          private:
            SomeStruct(SomeStruct& s);
          protected:
            SomeStruct(SomeStruct&& s);
          public:
            ~SomeStruct();
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "SomeStruct" ...
                copy_constructor: Unavailable,
                move_constructor: Unavailable,
                destructor: NontrivialUserDefined ...
            }
        }
    );
}

#[gtest]
fn test_record_special_member_definition() {
    let ir = ir_from_cc(
        "
        struct SomeStruct {
          private:
            SomeStruct(SomeStruct& s);
          protected:
            SomeStruct(SomeStruct&& s) = delete;
        };
    ",
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "SomeStruct" ...
                copy_constructor: Unavailable,
                move_constructor: Unavailable,
                destructor: Trivial ...
            }
        }
    );
}

#[gtest]
fn test_pointer_member_variable() {
    let ir = ir_from_cc(
        "struct SomeStruct {
            SomeStruct* ptr;
        };",
    )
    .unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Field {
                rust_identifier: Some("ptr") ...
                type_: Ok(CcType {
                    variant: Pointer(PointerType {
                        kind: Nullable,
                        lifetime: None,
                        pointee_type: CcType {
                            variant: Record(...), ...
                        },
                    }), ...
                }) ...
            }
        }
    );
}

#[gtest]
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
        .map(|r| (r.rs_name.identifier.as_ref(), r.doc_comment.as_ref().unwrap().as_ref()))
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

#[gtest]
fn test_doc_comment_vs_tooling_directives() -> Result<()> {
    let ir = ir_from_cc(concat!(
        " // Doc comment for `f1`\n",
        " // NOLINTNEXTLINE(directive-name)\n",
        r#" void f1();

            // Doc comment for `f2`
            // // NOLINT
            void f2();

            // // NOLINT
            static void f3();

            // Mid-sentence usage: [...] this is why we need NOLINT / virtual [...].
            void f4();

            // No closing paren still suppresses
        "#,
        " // NOLINTNEXTLINE(directive-name]\n",
        r#" void f5();

            // Multiple, comma-separated directives listed in parens
            // NOLINTNEXTLINE(foo,bar)
            void f6();
        "#,
    ))?;

    let comments: HashMap<&str, Option<&str>> = ir
        .functions()
        .map(|f| {
            if let UnqualifiedIdentifier::Identifier(id) = &f.rs_name {
                (id.identifier.as_ref(), f.doc_comment.as_deref())
            } else {
                panic!("No constructors/destructors expected in this test.")
            }
        })
        .collect();

    assert_eq!(comments["f1"], Some("Doc comment for `f1`"));
    assert_eq!(comments["f2"], Some("Doc comment for `f2`"));
    assert_eq!(comments["f3"], None);
    assert_eq!(
        comments["f4"],
        Some("Mid-sentence usage: [...] this is why we need NOLINT / virtual [...].")
    );
    assert_eq!(comments["f5"], Some("No closing paren still suppresses"));
    assert_eq!(comments["f6"], Some("Multiple, comma-separated directives listed in parens"));
    Ok(())
}

#[gtest]
fn test_must_bind_annotation_on_record() -> googletest::Result<()> {
    let ir = ir_from_cc(r#"struct [[clang::annotate("crubit_must_bind")]] S {};"#).or_fail()?;
    let record = ir.records().find(|record| record.rs_name == "S").or_fail()?;
    expect_that!(&**record, field!(&Record.must_bind, eq(true)));
    Ok(())
}

#[gtest]
fn test_must_bind_annotation_on_function() -> googletest::Result<()> {
    let ir = ir_from_cc(r#"[[clang::annotate("crubit_must_bind")]] void f() {}"#).or_fail()?;
    let func = ir.functions().find(|func| func.rs_name == "f").or_fail()?;
    expect_that!(&**func, field!(&Func.must_bind, eq(true)));
    Ok(())
}

#[gtest]
fn test_must_bind_annotation_on_unbindable_type_produces_must_bind_error() -> googletest::Result<()>
{
    let ir = ir_from_cc(
        r#"template<typename T> struct [[clang::annotate("crubit_must_bind", 7)]] S {};"#,
    )
    .or_fail()?;
    let record = ir.unsupported_items().find(|item| &*item.name == "S").or_fail()?;
    expect_that!(&**record, field!(&UnsupportedItem.must_bind, eq(true)));
    Ok(())
}

#[gtest]
fn test_typedef() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            // Doc comment for MyTypedefDecl.
            typedef int MyTypedefDecl;

            // Doc comment for MyTypeAliasDecl.
            using MyTypeAliasDecl = int;
        "#,
    )?;

    let int = quote! {
      CcType {
        variant: Primitive(Int),
        is_const: false, ...
      }
    };
    assert_ir_matches!(
        ir,
        quote! {
          TypeAlias {
            cc_name: "MyTypedefDecl",
            rs_name: "MyTypedefDecl",
            id: ItemId(...),
            owning_target: BazelLabel("//test:testing_target"),
            doc_comment: Some("Doc comment for MyTypedefDecl."),
            unknown_attr: None,
            underlying_type: #int,
            source_loc: ...
            enclosing_item_id: None,
            must_bind: false,
          }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
          TypeAlias {
            cc_name: "MyTypeAliasDecl",
            rs_name: "MyTypeAliasDecl",
            id: ItemId(...),
            owning_target: BazelLabel("//test:testing_target"),
            doc_comment: Some("Doc comment for MyTypeAliasDecl."),
            unknown_attr: None,
            underlying_type: #int,
            source_loc: ...,
            enclosing_item_id: None,
            must_bind: false,
          }
        }
    );

    Ok(())
}

#[gtest]
fn test_typedef_duplicate() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            struct MyStruct {};
            // First doc comment.
            using MyTypeAlias = MyStruct;
            // Second doc comment.
            using MyTypeAlias = MyStruct;
        "#,
    )?;
    // TODO(b/200064504): Figure out if we can (and want to) merge the doc
    // comments from both C++ declarations above. (Currently only the first doc
    // comment makes it through - maybe this is also okay in the long term?)
    assert_ir_matches!(
        ir,
        quote! {
          TypeAlias {
            cc_name: "MyTypeAlias",
            ...
            doc_comment: Some("First doc comment."),
            ...
          }
        }
    );
    // Emitting duplicated TypeAliases is undesirable, because Rust disallows
    // redefining a type alias even when the underlying type matches.  See
    // https://play.rust-lang.org/?edition=2021&gist=1c6f79ed41994fa6c89472742ded2f14
    //
    // The implementation avoids duplicated TypeAliases in the following way:
    // 1) LookupDecl gets called with `decl->getCanonicalDecl()`,
    // 2) LookupDecl deduplicates via `lookup_cache_`.
    assert_ir_not_matches!(
        ir,
        quote! {
          TypeAlias {
            cc_name: "MyTypeAlias",
            ...
          }
          ...
          TypeAlias {
            cc_name: "MyTypeAlias",
            ...
          }
        }
    );
    Ok(())
}

#[gtest]
fn test_typedef_of_full_template_specialization() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            namespace test_namespace_bindings {
                // Doc comment of MyStruct template.
                template <typename T>
                struct MyStruct {
                  // Doc comment of GetValue method.
                  const T& GetValue() const { return value; }

                  // Doc comment of `value` field.
                  T value;
                };

                // Doc comment of MyTypeAlias.
                using MyTypeAlias = MyStruct<int>;
            }"#,
    )?;
    // Instantiation of MyStruct<int> specialization:
    assert_ir_matches!(
        ir,
        quote! {
          Record {
            rs_name: "__CcTemplateInstN23test_namespace_bindings8MyStructIiEE", ...
            cc_name: "test_namespace_bindings::MyStruct<int>", ...
            owning_target: BazelLabel("//test:testing_target"), ...
            doc_comment: Some("Doc comment of MyStruct template."), ...
            fields: [Field {
                rust_identifier: Some("value"), ...
                doc_comment: Some("Doc comment of `value` field."), ...
                type_: Ok(CcType { variant: Primitive(Int), ... }),
                access: Public,
                offset: 0, ...
            }], ...
            enclosing_item_id: None, ...
          }
        }
    );
    let record_id = retrieve_record(&ir, "test_namespace_bindings::MyStruct<int>").id;
    // Make sure the instantiation of the class template appears exactly once in the
    // `top_level_item_ids`.
    assert_eq!(1, ir.top_level_item_ids().iter().filter(|&&id| id == record_id).count());
    // Type alias for the class template specialization.
    assert_ir_matches!(
        ir,
        quote! {
          TypeAlias {
            cc_name: "MyTypeAlias", ...
            owning_target: BazelLabel("//test:testing_target"), ...
            doc_comment: Some("Doc comment of MyTypeAlias."), ...
            underlying_type: CcType {
              variant: Record(ItemId(#record_id)), ...
            }, ...
          }
        }
    );
    // Member function of the MyTemplate<int> specialization:
    assert_ir_matches!(
        ir,
        quote! {
          Func {
            cc_name: "GetValue",
            rs_name: "GetValue",
            owning_target: BazelLabel("//test:testing_target"),
            mangled_name: "_ZNK23test_namespace_bindings8MyStructIiE8GetValueEv", ...
            doc_comment: Some("Doc comment of GetValue method."), ...
            is_inline: true, ...
            member_func_metadata: Some(MemberFuncMetadata {
                record_id: ItemId(#record_id),
                instance_method_metadata: Some(InstanceMethodMetadata { ... }), ...
            }), ...
          }
        }
    );
    // Implicitly defined assignment operator inside the struct template is
    // represented in the AST slightly differently (not marked as instantiated)
    // because it is generated by the compiler for the complete, instantiated type
    // according to general rules.
    assert_ir_matches!(
        ir,
        quote! {
          Func {
              cc_name: "operator=",
              rs_name: "operator=",
              owning_target: BazelLabel("//test:testing_target"),
              mangled_name: "_ZN23test_namespace_bindings8MyStructIiEaSERKS1_", ...
              doc_comment: None, ...
          }
        }
    );
    Ok(())
}

#[gtest]
fn test_typedef_for_explicit_template_specialization() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            namespace test_namespace_bindings {
                template <typename T>
                struct MyStruct final {};

                // Doc comment for template specialization for T=int.
                template<>
                struct MyStruct<int> final {
                  // Doc comment of the GetValue method specialization for T=int.
                  const int& GetValue() const { return value * 42; }

                  // Doc comment of the `value` field specialization for T=int.
                  int value;
                };

                // Doc comment of MyTypeAlias.
                using MyTypeAlias = MyStruct<int>;
              }"#,
    )?;
    // Instantiation of the explicit MyStruct<int> specialization:
    assert_ir_matches!(
        ir,
        quote! {
          Record {
            rs_name: "__CcTemplateInstN23test_namespace_bindings8MyStructIiEE", ...
            cc_name: "test_namespace_bindings::MyStruct<int>", ...
            owning_target: BazelLabel("//test:testing_target"),
            defining_target: Some(BazelLabel("//test:testing_target")), ...
            doc_comment: Some("Doc comment for template specialization for T=int."), ...
            fields: [Field {
                rust_identifier: Some("value"), ...
                doc_comment: Some("Doc comment of the `value` field specialization for T=int."), ...
                type_: Ok(CcType { variant: Primitive(Int), ... }),
                access: Public,
                offset: 0, ...
            }], ...
            enclosing_item_id: None, ...
          }
        }
    );
    let record_id = retrieve_record(&ir, "test_namespace_bindings::MyStruct<int>").id;

    // TODO(b/200067826) This assertion worked because the template specialization
    // was top level already.
    // Make sure the explicit specialization of the struct template appears exactly
    // once in the `top_level_item_ids`.
    // assert_eq!(1, ir.top_level_item_ids().filter(|&&id| id ==
    // record_id).count());

    // Instance method inside the explicit MyStruct<int> specialization:
    assert_ir_matches!(
        ir,
        quote! {
          Func {
            cc_name: "GetValue",
            rs_name: "GetValue",
            owning_target: BazelLabel("//test:testing_target"),
            mangled_name: "_ZNK23test_namespace_bindings8MyStructIiE8GetValueEv", ...
            doc_comment: Some("Doc comment of the GetValue method specialization for T=int."), ...
            is_inline: true, ...
            member_func_metadata: Some(MemberFuncMetadata {
                record_id: ItemId(#record_id),
                instance_method_metadata: Some(InstanceMethodMetadata { ... }), ...
            }), ...
          }
        }
    );
    Ok(())
}

#[gtest]
fn test_multiple_typedefs_to_same_specialization() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            struct MyStruct {
              void MyMethod() {}
            };
            using MyIntAlias = MyStruct<int>;
            using MyIntAlias = MyStruct<int>;
            using MyIntAlias2 = MyStruct<int>;
            using MyFloatAlias = MyStruct<float>;
            "#,
    )?;

    // Verify that there is only 1 record for each specialization.
    assert_eq!(
        1,
        ir.records().filter(|r| r.cc_name.identifier.as_ref() == "MyStruct<int>").count()
    );
    assert_eq!(
        1,
        ir.records().filter(|r| r.cc_name.identifier.as_ref() == "MyStruct<float>").count()
    );
    let functions = ir
        .functions()
        .filter(|f| f.rs_name == UnqualifiedIdentifier::Identifier(ir_id("MyMethod")))
        .collect_vec();

    // Verify that there is only 1 function per instantiation.
    assert_eq!(2, functions.len());
    let rec_id1 = functions[0].member_func_metadata.as_ref().unwrap().record_id;
    let rec_id2 = functions[1].member_func_metadata.as_ref().unwrap().record_id;
    assert_ne!(rec_id1, rec_id2);
    Ok(())
}

#[gtest]
fn test_implicit_specialization_items_are_deterministically_ordered() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            struct MyStruct {
              void MyMethod();
            };
            struct Str {};
            using Alias1 = MyStruct<int>;
            using Alias2 = MyStruct<long long>;
            using Alias3 = MyStruct<Str>;
            namespace test_namespace_bindings {
              using Alias4 = MyStruct<MyStruct<int>>;
              using Alias5 = MyStruct<bool>;
            }
            "#,
    )?;

    // Implicit class template specializations and their methods all have the same
    // source location. Test that they are sorted deterministically. (Implementation
    // detail: the ordering is by mangled name).
    let class_template_specialization_names = ir
        .top_level_item_ids()
        .iter()
        .filter_map(|id| match ir.find_decl(*id).unwrap() {
            ir::Item::Record(r) if r.rs_name.identifier.contains("__CcTemplateInst") => {
                Some(r.rs_name.identifier.as_ref())
            }
            _ => None,
        })
        .collect_vec();
    assert_eq!(
        vec![
            "__CcTemplateInst8MyStructI3StrE",
            "__CcTemplateInst8MyStructIS_IiEE",
            "__CcTemplateInst8MyStructIbE",
            "__CcTemplateInst8MyStructIiE",
            "__CcTemplateInst8MyStructIxE"
        ],
        class_template_specialization_names
    );

    let method_mangled_names = ir
        .functions()
        .filter_map(|f| match &f.rs_name {
            UnqualifiedIdentifier::Identifier(id) if id.identifier.as_ref() == "MyMethod" => {
                Some(f.mangled_name.as_ref())
            }
            _ => None,
        })
        .collect_vec();
    assert_eq!(
        vec![
            "_ZN8MyStructI3StrE8MyMethodEv",
            "_ZN8MyStructIS_IiEE8MyMethodEv",
            "_ZN8MyStructIbE8MyMethodEv",
            "_ZN8MyStructIiE8MyMethodEv",
            "_ZN8MyStructIxE8MyMethodEv"
        ],
        method_mangled_names
    );

    Ok(())
}

#[gtest]
fn test_templates_inheritance() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            class BaseTemplate {
             protected:
              BaseTemplate(T base_value) : base_value_(base_value) {}
              const T& base_value() const { return base_value_; }
             private:
              T base_value_;
            };

            template <typename T>
            class ClassTemplateDerivedFromClassTemplate : public BaseTemplate<T> {
             public:
              ClassTemplateDerivedFromClassTemplate(T base_value, T derived_value)
                  : BaseTemplate<T>(base_value), derived_value_(derived_value) {}
              T combined_value() const {
                return 1000 * BaseTemplate<T>::base_value() + derived_value_;
              }
             private:
              T derived_value_;
            };

            using TypeAliasForClassTemplateDerivedFromClassTemplate =
                    ClassTemplateDerivedFromClassTemplate<int>;
            "#,
    )?;

    // ClassTemplateDerivedFromClassTemplate is instantiated because of
    // TypeAliasForClassTemplateDerivedFromClassTemplate..
    assert_eq!(
        1,
        ir.records()
            .filter(|r| r.cc_name.identifier.contains("ClassTemplateDerivedFromClassTemplate"))
            .count()
    );

    // BaseTemplate is *not* instantiated in the generated bindings/IR.  The derived
    // class's bindings work fine without the bindings for the base class (this
    // is also true for non-templated base/derived classes).
    assert_eq!(0, ir.records().filter(|r| r.cc_name.identifier.contains("BaseTemplate")).count());
    Ok(())
}

#[gtest]
fn test_aliased_class_template_instantiated_in_header() -> Result<()> {
    // This aliased class template specialization is instantiated due to the code
    // that is present in the header. We should not corrupt the AST by
    // instantiating again.
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            struct MyTemplate {
                const T& GetValue() { return field; }
                T field;
            };

            inline void my_full_instantiation() {
                MyTemplate<int> t;
                t.field = 123;
                t.field = t.GetValue() * 123;
            }

            using MyAlias = MyTemplate<int>; "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! {
          Record {
            rs_name: "__CcTemplateInst10MyTemplateIiE", ...
            cc_name: "MyTemplate<int>", ...
            fields: [Field { rust_identifier: Some("field"), ... }], ...
          }
        }
    );
    assert_ir_matches!(ir, quote! { Func { ...rs_name: "GetValue", ...  } });
    Ok(())
}

#[gtest]
fn test_aliased_class_template_partially_instantiated_in_header() -> Result<()> {
    // Similar to `test_aliased_class_template_instantiated_in_header`, but doesn't
    // instantiate all members.
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            struct MyTemplate {
                const T& GetValue() { return field; }
                T field;
            };

            inline void my_instantiation() {
                MyTemplate<int> t;
                // Members of MyTemplate are not used/instantiated.
            }

            using MyAlias = MyTemplate<int>; "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! {
          Record {
            rs_name: "__CcTemplateInst10MyTemplateIiE", ...
            cc_name: "MyTemplate<int>", ...
            fields: [Field { rust_identifier: Some("field"), ... }], ...
          }
        }
    );
    assert_ir_matches!(ir, quote! { Func { cc_name: "GetValue", rs_name: "GetValue", ...  } });
    Ok(())
}

#[gtest]
fn test_subst_template_type_parm_pack_type() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename... TArgs>
            struct MyStruct {
                static int GetSum(TArgs... my_args) { return (0 + ... + my_args); }
            };
            using MyTypeAlias = MyStruct<int, int>; "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! {
          Record(Record {
            rs_name: "__CcTemplateInst8MyStructIJiiEE", ...
            cc_name: "MyStruct<int, int>", ...
          }),
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            Func {
                cc_name: "GetSum", ...
                mangled_name: "_ZN8MyStructIJiiEE6GetSumEii", ...
                params: [
                    FuncParam {
                        type_: CcType { variant: Primitive(Int), ... },
                        identifier: "__my_args_0",
                        unknown_attr: None,
                    },
                    FuncParam {
                        type_: CcType { variant: Primitive(Int), ... },
                        identifier: "__my_args_1",
                        unknown_attr: None,
                    },
                ], ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_fully_instantiated_template_in_function_return_type() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision

            template <typename T>
            struct MyStruct { T value; };

            MyStruct<int> MyFunction(); "#,
    )?;
    // Instantiation of the struct template:
    assert_ir_matches!(
        ir,
        quote! {
          Record {
            rs_name: "__CcTemplateInst8MyStructIiE", ...
            cc_name: "MyStruct<int>", ...
            owning_target: BazelLabel("//test:testing_target"), ...
          }
        }
    );
    let record_id = retrieve_record(&ir, "MyStruct<int>").id;
    // Function that used the class template as a return type.
    assert_ir_matches!(
        ir,
        quote! {
          Func {
            cc_name: "MyFunction",
            rs_name: "MyFunction",
            owning_target: BazelLabel("//test:testing_target"), ...
            return_type: CcType {
                variant: Record(ItemId(#record_id)), ...
            },
            params: [], ...
            is_inline: false, ...
            member_func_metadata: None, ...
            has_c_calling_convention: true, ...
            is_member_or_descendant_of_class_template: false, ...
          }
        }
    );
    Ok(())
}

#[gtest]
fn test_fully_instantiated_template_in_function_param_type() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision

            template <typename T>
            struct MyStruct { T value; };

            void MyFunction(const MyStruct<int>& my_param); "#,
    )?;
    // Instantiation of the struct template:
    assert_ir_matches!(
        ir,
        quote! {
          Record {
            rs_name: "__CcTemplateInst8MyStructIiE", ...
            cc_name: "MyStruct<int>", ...
            owning_target: BazelLabel("//test:testing_target"), ...
          }
        }
    );
    let record_id = retrieve_record(&ir, "MyStruct<int>").id;
    // Function that used the class template as a param type:
    assert_ir_matches!(
        ir,
        quote! {
          Func {
            cc_name: "MyFunction",
            rs_name: "MyFunction",
            owning_target: BazelLabel("//test:testing_target"), ...
            params: [FuncParam {
                type_: CcType {
                    variant: Pointer(PointerType {
                        kind: LValueRef,
                        lifetime: Some(...),
                        pointee_type: CcType {
                            variant: Record(ItemId(#record_id)), ...
                        },
                    }),
                    is_const: false, ...
                },
                identifier: "my_param",
                unknown_attr: None,
            }], ...
            is_inline: false, ...
            member_func_metadata: None, ...
            has_c_calling_convention: true, ...
            is_member_or_descendant_of_class_template: false, ...
          }
        }
    );
    Ok(())
}

#[gtest]
fn test_fully_instantiated_template_in_public_field() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            struct MyTemplate { T field; };

            class MyStruct {
             public:
              MyTemplate<int> public_field;
            }; "#,
    )?;
    // Instantiation of the struct template:
    assert_ir_matches!(
        ir,
        quote! {
          Record {
            rs_name: "__CcTemplateInst10MyTemplateIiE", ...
            cc_name: "MyTemplate<int>", ...
            owning_target: BazelLabel("//test:testing_target"), ...
          }
        }
    );
    let record_id = retrieve_record(&ir, "MyTemplate<int>").id;
    // Struct that used the class template as a type of a public field:
    assert_ir_matches!(
        ir,
        quote! {
               Record {
                   rs_name: "MyStruct",
                   cc_name: "MyStruct", ...
                   owning_target: BazelLabel("//test:testing_target"), ...
                   fields: [Field {
                       rust_identifier: Some("public_field"), ...
                       type_: Ok(CcType {
                           variant: Record(ItemId(#record_id)), ...
                       }),
                       access: Public,
                       offset: 0,
                       size: 32,
                       unknown_attr: None,
                       is_no_unique_address: false,
                       is_bitfield: false,
                       is_inheritable: true,
                   }], ...
               }
        }
    );
    Ok(())
}

#[gtest]
fn test_fully_instantiated_template_in_private_field() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            struct MyTemplate { T field; };

            class MyStruct {
             private:
              MyTemplate<int> private_field_;
            }; "#,
    )?;
    // There should be no instantiated template, just because of the private field.
    // To some extent this test is an early enforcement of the long-term plan for
    // b/226580208 and <internal link>.
    assert_ir_not_matches!(ir, quote! { "field" });
    // Struct that used the class template as a type of a private field:
    assert_ir_matches!(
        ir,
        quote! {
               Record {
                   rs_name: "MyStruct",
                   cc_name: "MyStruct", ...
                   owning_target: BazelLabel("//test:testing_target"), ...
                   fields: [Field {
                       rust_identifier: Some("private_field_"), ...
                       type_: Err("Types of non-public C++ fields can be elided away"), ...
                       access: Private,
                       offset: 0,
                       size: 32,
                       unknown_attr: None,
                       is_no_unique_address: false,
                       is_bitfield: false,
                       is_inheritable: false,
                   }], ...
               }
        }
    );
    Ok(())
}

#[gtest]
fn test_template_with_decltype_and_with_auto() -> Result<()> {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T1, typename T2>
            struct MyTemplate {
                static decltype(auto) TemplatedAdd(T1 a, T2 b) { return a + b; }
            };
            using MyAlias = MyTemplate<unsigned int, long long>; "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! {
            Func {
              cc_name: "TemplatedAdd",
               rs_name: "TemplatedAdd", ...
               return_type: CcType { variant: Primitive(LongLong), ... }, ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_subst_template_type_parm_type_vs_const_when_non_const_template_param() -> Result<()> {
    // This test (and
    // `test_subst_template_type_parm_type_vs_const_when_const_template_param`)
    // verifies that `importer.cc` preserves `const` qualifier attached *both* to
    // QualType associated with:
    // 1) SubstTemplateTypeParm (i.e. the template *argument* has `const`:
    // `MyTemplate<const int>`) 2) TemplateTypeParmType used inside the template
    // definition: `const T& GetConstRef()`
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            struct MyTemplate {
                const T& GetConstRef() const { return value; }
                T& GetRef() { return value; }
                T value;
            };

            // Just like the other test_subst_template_type_parm_type_vs_const...
            // test, but using non-*const* int template parameter.
            using MyAlias = MyTemplate<int>; "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! {
            Func {
                cc_name: "GetConstRef",
                rs_name: "GetConstRef", ...
                return_type: CcType {
                    variant: Pointer(PointerType {
                        kind: LValueRef,
                        lifetime: Some(...),
                        pointee_type: CcType {
                            variant: Primitive(Int),
                            is_const: true, ...
                        }, ...
                    }),
                    is_const: false, ...
                }, ...
            }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            Func {
                cc_name: "GetRef",
                rs_name: "GetRef", ...
                return_type: CcType {
                    variant: Pointer(PointerType {
                        kind: LValueRef,
                        lifetime: Some(...),
                        pointee_type: CcType {
                            variant: Primitive(Int),
                            is_const: false, ...
                        },
                    }),
                    is_const: false, ...
                }, ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_subst_template_type_parm_type_vs_const_when_const_template_param() -> Result<()> {
    // This test (and
    // `test_subst_template_type_parm_type_vs_const_when_non_const_template_param`)
    // verifies that `importer.cc` preserves `const` qualifier attached *both* to
    // QualType associated with:
    // 1) SubstTemplateTypeParm (i.e. the template *argument* has `const`:
    // `MyTemplate<const int>`) 2) TemplateTypeParmType used inside the template
    // definition: `const T& GetConstRef()`
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            struct MyTemplate {
                const T& GetConstRef() const { return value; }
                T& GetRef() { return value; }
                T value;
            };

            // Just like the other test_subst_template_type_parm_type_vs_const...
            // test, but using *const* int template parameter.
            using MyAlias = MyTemplate<const int>; "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! {
            Func {
                cc_name: "GetConstRef",
                rs_name: "GetConstRef", ...
                return_type: CcType {
                    variant: Pointer(PointerType {
                        kind: LValueRef,
                        lifetime: Some(...),
                        pointee_type: CcType {
                            variant: Primitive(Int),
                            is_const: true, ...
                        },
                    }),
                    is_const: false, ...
                }, ...
            }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            Func {
                cc_name: "GetRef",
                rs_name: "GetRef", ...
                return_type: CcType {
                    variant: Pointer(PointerType {
                        kind: LValueRef,
                        lifetime: Some(...),
                        pointee_type: CcType {
                            variant: Primitive(Int),
                            is_const: true, ...
                        },
                    }),
                    is_const: false, ...
                }, ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_template_and_alias_are_both_in_dependency() -> Result<()> {
    // See also the `test_template_in_dependency_and_alias_in_current_target` test.
    let ir = {
        let dependency_src = r#" #pragma clang lifetime_elision
                template <typename T>
                struct MyTemplate {
                    T GetValue();
                    T field;
                };
                using MyAliasOfTemplate = MyTemplate<int>;
                struct StructInDependency {}; "#;
        let current_target_src = r#" #pragma clang lifetime_elision
                /* no references to MyTemplate or MyAliasOfTemplate */
                struct StructInCurrentTarget {}; "#;
        ir_from_cc_dependency(current_target_src, dependency_src)?
    };

    // Just double-checking the test inputs VS target names.
    let current_target = ir_testing::TESTING_TARGET;
    let dependency = ir_testing::DEPENDENCY_TARGET;
    assert_ir_matches!(
        ir,
        quote! {
            Record { ...
                cc_name: "StructInCurrentTarget", ...
                owning_target: BazelLabel(#current_target), ...
            }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            Record { ...
                cc_name: "StructInDependency", ...
                owning_target: BazelLabel(#dependency), ...
            }
        }
    );

    // Type alias is only defined in `dependency`.
    assert_ir_matches!(
        ir,
        quote! {
            TypeAlias { ...
                cc_name: "MyAliasOfTemplate", ...
                owning_target: BazelLabel(#dependency), ...
            }
        }
    );
    assert_ir_not_matches!(
        ir,
        quote! {
            TypeAlias { ...
                cc_name: "MyAliasOfTemplate", ...
                owning_target: BazelLabel(#current_target), ...
            }
        }
    );

    // The template should be instantiated in `dependency`, rather than in
    // `current_target`.
    // TODO(b/222001243): Fix which target contains the instantiations and then flip
    // the test assertions below.  Tentative fix: cl/438580040.
    assert_ir_not_matches!(
        ir,
        quote! {
            Record { ...
                cc_name: "MyTemplate<int>", ...
                owning_target: BazelLabel(#dependency), ...
            }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            Record { ...
                cc_name: "MyTemplate<int>", ...
                owning_target: BazelLabel(#current_target), ...
            }
        }
    );

    // The template instantiations in the `dependency` should only produce type
    // information (e.g. TypeAlias, Record) and don't need to produce Func
    // items.
    assert_ir_not_matches!(
        ir,
        quote! {
            Func { ...
                rs_name: "GetValue", ...
                owning_target: BazelLabel(#dependency), ...
            }
        }
    );
    // There should be nothing template-instantiation-related in the main test
    // target. TODO(b/222001243): Fix which target contains the instantiations
    // and then flip the test assertions below to `assert_ir_not_matches`.
    assert_ir_matches!(
        ir,
        quote! {
            Func { ...
                rs_name: "GetValue", ...
                owning_target: BazelLabel(#current_target), ...
            }
        }
    );

    Ok(())
}

#[gtest]
fn test_template_in_dependency_and_alias_in_current_target() -> Result<()> {
    // See also the `test_template_and_alias_are_both_in_dependency` test.
    let ir = {
        let dependency_src = r#" #pragma clang lifetime_elision
                template <typename T>
                struct MyTemplate {
                    T GetValue();
                    T field;
                };
                struct StructInDependency{}; "#;
        let current_target_src = r#" #pragma clang lifetime_elision
                using MyAliasOfTemplate = MyTemplate<int>;
                struct StructInCurrentTarget{}; "#;
        ir_from_cc_dependency(current_target_src, dependency_src)?
    };

    // Just double-checking the test inputs VS target names.
    let current_target = ir_testing::TESTING_TARGET;
    let dependency = ir_testing::DEPENDENCY_TARGET;
    assert_ir_matches!(
        ir,
        quote! {
            Record { ...
                cc_name: "StructInCurrentTarget", ...
                owning_target: BazelLabel(#current_target), ...
            }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            Record { ...
                cc_name: "StructInDependency", ...
                owning_target: BazelLabel(#dependency), ...
            }
        }
    );

    // Type alias is only defined in `current_target`
    assert_ir_not_matches!(
        ir,
        quote! {
            TypeAlias { ...
                cc_name: "MyAliasOfTemplate", ...
                owning_target: BazelLabel(#dependency), ...
            }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            TypeAlias { ...
                cc_name: "MyAliasOfTemplate", ...
                owning_target: BazelLabel(#current_target), ...
            }
        }
    );

    // The template should be instantiated in `current_target`, rather than in
    // `dependency`.
    assert_ir_not_matches!(
        ir,
        quote! {
            Record { ...
                cc_name: "MyTemplate<int>", ...
                owning_target: BazelLabel(#dependency), ...
            }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            Record { ...
                cc_name: "MyTemplate<int>", ...
                owning_target: BazelLabel(#current_target), ...
            }
        }
    );

    // There should be nothing template-instantiation-related in the dependency
    // (since there is no instantiation there).
    assert_ir_not_matches!(
        ir,
        quote! {
            Func { ...
                rs_name: "GetValue", ...
                owning_target: BazelLabel(#dependency), ...
            }
        }
    );
    // The template instantiations in the current target should produce not only
    // type information (e.g. TypeAlias, Record) but also Func items (for
    // methods of the instantiated class template).
    assert_ir_matches!(
        ir,
        quote! {
            Func { ...
                rs_name: "GetValue", ...
                owning_target: BazelLabel(#current_target), ...
            }
        }
    );

    Ok(())
}

#[gtest]
fn test_well_known_types_check_namespaces() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            namespace my_namespace {
              using int32_t = int;
            }
            void f(my_namespace::int32_t i);
        "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! {
          Func { ...
            rs_name: "f", ...
            params: [
             FuncParam {
              type_: CcType { variant: Record(...), ... },
              identifier: "i",
              unknown_attr: None,
             }], ...
          }
        }
    );
    Ok(())
}

#[gtest]
fn test_dont_import_typedef_nested_in_func() {
    let ir = ir_from_cc("inline void f() { typedef int MyTypedefDecl; }").unwrap();
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyTypedefDecl" ... } });
}

#[gtest]
fn test_dont_import_typedef_for_structs_from_c() {
    let ir = ir_from_cc("struct MyStruct {}; typedef struct MyStruct MyStruct;").unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyStruct" ...}});
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyStruct" ... } });
}

#[gtest]
fn test_ignore_typedef_but_import_struct_from_c() {
    let ir = ir_from_cc("typedef struct {} MyStruct;").unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyStruct" ...}});
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyStruct" ... } });
}

#[gtest]
fn test_typedef_and_import_struct_from_c() {
    let ir = ir_from_cc("typedef struct MyStruct {} MyTypedef;").unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyStruct" ...}});
    assert_ir_matches!(ir, quote! { TypeAlias { cc_name: "MyTypedef" ... } });
}

#[gtest]
fn test_import_struct_typedef_from_different_decl_context() {
    let ir = ir_from_cc(
        "struct MyStruct {}; namespace test_namespace_bindings { typedef MyStruct MyStruct; }",
    )
    .unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyStruct" ...}});
    assert_ir_matches!(ir, quote! { TypeAlias { cc_name: "MyStruct" ... } });
}

// TODO(b/214901011): This only worked because we didn't generate bindings for
// the second reopened namespace.
// #[gtest]
#[allow(dead_code)]
fn test_ignore_struct_typedef_from_decl_context_redecl() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings { struct MyStruct {}; }
        namespace test_namespace_bindings { typedef MyStruct MyStruct; }
    "#,
    )
    .unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyStruct" ...}});
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyStruct" ... } });
}

// TODO(b/214901011): This only worked because we didn't generate IR for the
// namespace coming from the dependency.
// #[gtest]
#[allow(dead_code)]
fn test_ignore_struct_typedef_from_decl_context_redecl_from_multiple_targets() {
    let ir = ir_from_cc_dependency(
        "namespace test_namespace_bindings { typedef MyStruct MyStruct; }",
        "namespace test_namespace_bindings { struct MyStruct {}; }",
    )
    .unwrap();
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyStruct" ... } });
}

#[gtest]
fn test_dont_import_typedef_for_unions_from_c() {
    let ir = ir_from_cc("union MyUnion {}; typedef union MyUnion MyUnion;").unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyUnion" ...}});
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyUnion" ... } });
}

#[gtest]
fn test_ignore_typedef_but_import_union_from_c() {
    let ir = ir_from_cc("typedef union {} MyUnion;").unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyUnion" ...}});
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyUnion" ... } });
}

#[gtest]
fn test_typedef_and_import_union_from_c() {
    let ir = ir_from_cc("typedef union MyUnion {} MyTypedef;").unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyUnion" ...}});
    assert_ir_matches!(ir, quote! { TypeAlias { cc_name: "MyTypedef" ... } });
}

#[gtest]
fn test_import_union_typedef_from_different_decl_context() {
    let ir = ir_from_cc(
        "union MyUnion {}; namespace test_namespace_bindings { typedef MyUnion MyUnion; }",
    )
    .unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyUnion" ...}});
    assert_ir_matches!(ir, quote! { TypeAlias { cc_name: "MyUnion" ... } });
}

// TODO(b/214901011): This only worked because we didn't generate bindings for
// the second reopened namespace.
// #[gtest]
#[allow(dead_code)]
fn test_ignore_union_typedef_from_decl_context_redecl() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings { union MyUnion {}; }
        namespace test_namespace_bindings { typedef MyUnion MyUnion; }
    "#,
    )
    .unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyUnion" ...}});
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyUnion" ... } });
}

// TODO(b/214901011): This only worked because we didn't generate IR for the
// namespace coming from the dependency.
// #[gtest]
#[allow(dead_code)]
fn test_ignore_union_typedef_from_decl_context_redecl_from_multiple_targets() {
    let ir = ir_from_cc_dependency(
        "namespace test_namespace_bindings { typedef MyUnion MyUnion; }",
        "namespace test_namespace_bindings { union MyUnion {}; }",
    )
    .unwrap();
    assert_ir_not_matches!(ir, quote! { TypeAlias { identifier: "MyUnion" ... } });
}

#[gtest]
fn test_record_with_unsupported_field_type() -> Result<()> {
    // Using a packed struct because it's currently not supported.
    // But... any other unsupported type would also work for this test.
    let ir = ir_from_cc(
        r#"
        struct __attribute__((packed)) Packed {};
        struct StructWithUnsupportedField {
          // Doc comment for `my_field`.
          Packed my_field;
        };
    "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! {
           Record {
               rs_name: "StructWithUnsupportedField",
               cc_name: "StructWithUnsupportedField",
               ...
               fields: [Field {
                   rust_identifier: Some("my_field"),
                   cpp_identifier: Some("my_field"),
                   doc_comment: Some("Doc comment for `my_field`."),
                   type_: Err(
                       "Unsupported type 'struct Packed': No generated bindings found for 'Packed'",
                   ),
                   access: Public,
                   offset: 0,
                   size: 8,
                   unknown_attr: None,
                   is_no_unique_address: false,
                   is_bitfield: false,
                   is_inheritable: false,
               }],
               ...
                size_align: SizeAlign {
                    size: 1,
                    alignment: 1,
                } ...
               ...
           }
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            UnsupportedItem {
                name: "Packed", ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_record_with_unsupported_base() -> Result<()> {
    let ir = ir_from_cc(
        r#" 
            struct __attribute__((packed)) IllegalBaseClass {
              // Having a field here avoids empty base class optimization
              // and forces `derived_field` to be at a non-zero offset.
              // See also: https://en.cppreference.com/w/cpp/language/ebo
              char nested_field;
            };

            // Using a packed struct as a base class because packed structs are
            // currently unsupported.  But... any other unsupported base class
            // would also work for this test.
            struct DerivedClass : public IllegalBaseClass {
              int derived_field;
            }; "#,
    )?;
    // Verify that `unambiguous_public_bases` are empty (instead of containing a
    // dangling `ItemId` of the `IllegalBaseClass` (which got imported as
    // `UnsupportedItem` rather than as a `Record`).
    assert_ir_matches!(
        ir,
        quote! {
           Record {
              rs_name: "DerivedClass",
              cc_name: "DerivedClass",
              cc_preferred_name: "",
              mangled_cc_name: "12DerivedClass",
              id: ItemId(...),
              owning_target: BazelLabel("//test:testing_target"),
              defining_target: None,
              template_specialization: None,
              unknown_attr: None,
              doc_comment: Some(...),
              bridge_type: None, ...
              unambiguous_public_bases: [],
              fields: [Field {
                  rust_identifier: Some("derived_field"), ...
                  offset: 32, ...
              }], ...
              size_align: SizeAlign {
                  size: 8,
                  alignment: 4,
              }, ...
              is_derived_class: true,
              override_alignment: true,
              ...
           }
        }
    );
    // Verify that the IllegalBaseClass is unsupported (this is mostly verification
    // that the test input correctly sets up the test scenario;  the real
    // verification is above).
    assert_ir_matches!(
        ir,
        quote! {
           UnsupportedItem {
                name: "IllegalBaseClass", ...
           }
        }
    );
    Ok(())
}

#[gtest]
fn test_do_not_import_static_member_functions_when_record_not_supported_yet() {
    // only using __attribute__((packed)) as an example of a record we cannot import yet.
    let ir = ir_from_cc(
        "
        struct __attribute__((packed)) SomeStruct {
          static void StaticMemberFunction();
        };",
    )
    .unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
          name: "SomeStruct::StaticMemberFunction" ...
        }}
    );
}

#[gtest]
fn test_do_not_import_nonstatic_member_functions_when_record_not_supported_yet() {
    // only using __attribute__((packed)) as an example of a record we cannot import yet.
    let ir = ir_from_cc(
        "
        struct __attribute__((packed)) SomeStruct {
          void NonStaticMemberFunction();
        };",
    )
    .unwrap();
    assert_ir_matches!(
        ir,
        quote! { UnsupportedItem {
          name: "SomeStruct::NonStaticMemberFunction" ...
        }}
    );
}

#[gtest]
fn test_dont_import_injected_class_name() {
    let ir = ir_from_cc("struct SomeStruct {};").unwrap();
    let names =
        ir.records().map(|r| r.rs_name.identifier.as_ref()).filter(|n| n.contains("SomeStruct"));
    // we support nested structs, so we should not emit record for injected class name
    assert_eq!(names.count(), 1);
}

#[gtest]
fn test_integer_typedef_usage() -> Result<()> {
    // This is a regression test. We used to incorrectly desugar typedefs of
    // builtin types and treat them as if they were the underlying builtin type.
    // As a result, this test would produce a binding for f(MyTypedef) with a
    // parameter of type `int` instead of `MyTypedef`. This test therefore
    // checks that the type has a `decl_id` but doesn't have a `name`. More
    // specific checks are done in the code generation tests.
    let ir = ir_from_cc(
        r#"
            typedef int MyTypedef;
            void f(MyTypedef my_typedef);
        "#,
    )?;
    assert_ir_matches!(
        ir,
        quote! { Func {
         cc_name: "f",
         rs_name: "f", ...
         params: [
           FuncParam {
            type_: CcType { variant: Record(...), ... },
            identifier: "my_typedef",
            unknown_attr: None,
           }], ...
        } }
    );

    Ok(())
}

#[gtest]
fn test_struct() {
    let ir = ir_from_cc("struct SomeStruct { int first_field; int second_field; };").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "SomeStruct" ...
                cc_name: "SomeStruct" ...
                mangled_cc_name: "10SomeStruct" ...
                fields: [
                    Field {
                        rust_identifier: Some("first_field"), ...
                        type_: Ok(CcType { variant: Primitive(Int), ... }), ...
                        offset: 0, ...
                        size: 32, ...
                        is_bitfield: false, ...
                    },
                    Field {
                        rust_identifier: Some("second_field"), ...
                        type_: Ok(CcType { variant: Primitive(Int), ... }), ...
                        offset: 32, ...
                        size: 32, ...
                        is_bitfield: false, ...
                    },
                ], ...
                size_align: SizeAlign {
                  size: 8,
                  alignment: 4,
                }, ...
                record_type: Struct, ...
            }
        }
    );
}

#[gtest]
fn test_class() {
    // This test verifies that `record_type` correectly captures whether the C++
    // RecordDecl was for a `struct` VS for a `class`.
    let ir = ir_from_cc("class SomeClass { int field; };").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "SomeClass" ...
                cc_name: "SomeClass" ...
                record_type: Class, ...
            }
        }
    );
}

#[gtest]
fn test_struct_forward_declaration() {
    let ir = ir_from_cc("struct Struct;").unwrap();
    assert!(!ir.records().any(|r| r.rs_name.identifier.as_ref() == "Struct"));
}

#[gtest]
fn test_struct_forward_declaration_in_namespace() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        namespace MyNamespace {
        struct FwdDeclared;
        }
        "#,
    )?;

    assert_eq!(1, ir.namespaces().count());
    let ns = ir.namespaces().next().unwrap();
    assert_eq!("MyNamespace", ns.rs_name.identifier.as_ref());
    assert_eq!(1, ns.child_item_ids.len());

    let ns_id = ns.id;
    let child_id = ns.child_item_ids[0];
    assert_ir_matches!(
        ir,
        quote! {
            Namespace(Namespace {
                cc_name: "MyNamespace",
                rs_name: "MyNamespace" ...
                id: ItemId(#ns_id) ...
                child_item_ids: [ItemId(#child_id)] ...
                enclosing_item_id: None ...
            }),
            IncompleteRecord(IncompleteRecord {
                cc_name: "FwdDeclared" ...
                rs_name: "FwdDeclared" ...
                id: ItemId(#child_id) ...
                ...
                enclosing_item_id: Some(ItemId(#ns_id)) ...
            }),
        }
    );

    Ok(())
}

#[gtest]
fn test_union() {
    let ir = ir_from_cc("union SomeUnion { int first_field; int second_field; };").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            Record {
                rs_name: "SomeUnion" ...
                cc_name: "SomeUnion" ...
                fields: [
                    Field {
                        rust_identifier: Some("first_field"), ...
                        type_: Ok(CcType { variant: Primitive(Int), ... }), ...
                        offset: 0, ...
                        size: 32, ...
                        is_bitfield: false, ...
                    },
                    Field {
                        rust_identifier: Some("second_field"), ...
                        type_: Ok(CcType { variant: Primitive(Int), ... }), ...
                        offset: 0, ...
                        size: 32, ...
                        is_bitfield: false, ...
                    },
                ], ...
                size_align: SizeAlign {
                    size: 4,
                    alignment: 4,
                }, ...
                record_type: Union, ...
            }
        }
    );
}

#[gtest]
fn test_union_with_data_members_with_different_sizes() {
    let ir = ir_from_cc(
        r#"
    union MyUnion {
      char first_field[56];
      int second_field;
    };
  "#,
    )
    .unwrap();
    assert_ir_matches!(
        ir,
        quote! {
              Record { ...
                rs_name: "MyUnion"...
                fields: [
                  Field {
                    rust_identifier: Some("first_field") ...
                    offset: 0 ...
                    size: 448 ...
                  },
                  Field {
                    rust_identifier: Some("second_field") ...
                    offset: 0 ...
                    size: 32 ...
                  } ...
                ] ...
              }
        }
    );
}

#[gtest]
fn test_member_function_params() {
    let ir = ir_from_cc(
        r#"
            struct Struct {
                void Foo(int x, int y);
            };
        "#,
    )
    .unwrap();
    let foo_func = ir
        .functions()
        .find(|f| f.rs_name == UnqualifiedIdentifier::Identifier(ir_id("Foo")))
        .unwrap();
    let param_names: Vec<_> =
        foo_func.params.iter().map(|p| p.identifier.identifier.as_ref()).collect();
    assert_eq!(param_names, vec!["__this", "x", "y"]);
}

fn assert_member_function_with_predicate_has_instance_method_metadata<F: FnMut(&Func) -> bool>(
    ir: &IR,
    record_name: &str,
    mut func_predicate: F,
    expected_metadata: &Option<ir::InstanceMethodMetadata>,
) {
    let record = ir
        .records()
        .find(|r| r.rs_name.identifier.as_ref() == record_name)
        .expect("Struct not found");
    let function = ir.functions().find(|f| func_predicate(f));
    let meta = function
        .expect("Function not found")
        .member_func_metadata
        .as_ref()
        .expect("Member function should specify member_func_metadata");
    assert_eq!(meta.record_id, record.id);
    assert_eq!(&meta.instance_method_metadata, expected_metadata);
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

    assert_member_function_with_predicate_has_instance_method_metadata(
        &ir,
        "Struct",
        |f| f.rs_name == UnqualifiedIdentifier::Identifier(ir_id(name)),
        expected_metadata,
    );
}

#[gtest]
fn test_member_function_static() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "static void Function();",
        &None,
    );
}

#[gtest]
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

#[gtest]
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

#[gtest]
fn test_member_function_virtual() {
    assert_member_function_has_instance_method_metadata(
        "Function",
        "virtual void Function();",
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::Unqualified,
            is_const: false,
            is_virtual: true,
        }),
    );
}

#[gtest]
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

#[gtest]
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

#[gtest]
fn test_member_function_rvalue_ref_qualified_this_param_type() {
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            struct StructWithRvalueRefQualifiedMethod final {
                void rvalue_ref_qualified_method() &&;
                void rvalue_ref_const_qualified_method() const &&;
            };
        "#,
    )
    .unwrap();

    let rvalue_ref_method = ir
        .functions()
        .find(|f| {
            f.rs_name == UnqualifiedIdentifier::Identifier(ir_id("rvalue_ref_qualified_method"))
        })
        .unwrap();
    let this_param = &rvalue_ref_method.params[0].type_.variant.as_pointer().unwrap();
    assert_eq!(this_param.kind, PointerTypeKind::RValueRef);
    assert!(!this_param.pointee_type.is_const);

    let rvalue_ref_const_method = ir
        .functions()
        .find(|f| {
            f.rs_name
                == UnqualifiedIdentifier::Identifier(ir_id("rvalue_ref_const_qualified_method"))
        })
        .unwrap();
    let const_this_param = rvalue_ref_const_method.params[0].type_.variant.as_pointer().unwrap();
    assert_eq!(const_this_param.kind, PointerTypeKind::RValueRef);
    assert!(const_this_param.pointee_type.is_const);
}

#[gtest]
fn test_member_function_explicit_constructor() {
    let ir = ir_from_cc(
        r#"
        struct SomeStruct {
          explicit SomeStruct(int i);
          SomeStruct() = delete;
          SomeStruct(const SomeStruct&) = delete;
        }; "#,
    )
    .unwrap();
    assert_member_function_with_predicate_has_instance_method_metadata(
        &ir,
        "SomeStruct",
        |f| f.rs_name == UnqualifiedIdentifier::Constructor,
        &Some(ir::InstanceMethodMetadata {
            reference: ir::ReferenceQualification::Unqualified,
            is_const: false,
            is_virtual: false,
        }),
    );
}

#[gtest]
fn test_member_function_constructor() {
    for explicit_prefix in ["", "explicit"] {
        let ir = ir_from_cc(&format!(
            r#"
                struct SomeStruct {{
                  {explicit_prefix} SomeStruct(int i);
                }}; "#
        ))
        .unwrap();
        assert_member_function_with_predicate_has_instance_method_metadata(
            &ir,
            "SomeStruct",
            |f| f.rs_name == UnqualifiedIdentifier::Constructor,
            &Some(ir::InstanceMethodMetadata {
                reference: ir::ReferenceQualification::Unqualified,
                is_const: false,
                is_virtual: false,
            }),
        );
    }
}

fn get_func_names(definition: &str) -> Vec<ir::UnqualifiedIdentifier> {
    let ir = ir_from_cc(definition).unwrap();
    ir.functions().map(|f| f.rs_name.clone()).collect()
}

#[gtest]
fn test_identifier_function_name() {
    assert_eq!(
        get_func_names("void Function();"),
        vec![ir::UnqualifiedIdentifier::Identifier(ir::Identifier {
            identifier: "Function".into()
        })],
    );
}

#[gtest]
fn test_constructor_function_name() {
    assert!(get_func_names("struct Struct {Struct();};")
        .contains(&ir::UnqualifiedIdentifier::Constructor));
}

#[gtest]
fn test_destructor_function_name() {
    assert!(get_func_names("struct Struct {~Struct();};")
        .contains(&ir::UnqualifiedIdentifier::Destructor));
}

#[gtest]
fn test_unsupported_items_are_emitted() -> Result<()> {
    // We will have to rewrite this test to use something else that is unsupported
    // once we start importing nested structs.
    let ir = ir_from_cc("struct __attribute__((packed)) PackedStruct {};")?;
    assert_strings_contain(
        ir.unsupported_items().map(|i| i.name.as_ref()).collect_vec().as_slice(),
        "PackedStruct",
    );
    Ok(())
}

#[gtest]
fn test_unsupported_items_from_dependency_are_not_emitted() -> Result<()> {
    let ir = ir_from_cc_dependency(
        "struct MyOtherStruct { ImportantLibaryTypeThatsUnsupported my_field; };",
        "struct __attribute__((packed)) ImportantLibaryTypeThatsUnsupported {};",
    )?;
    let names = ir.unsupported_items().map(|i| i.name.as_ref()).collect_vec();
    assert_strings_dont_contain(names.as_slice(), "ImportantLibaryTypeThatsUnsupported");
    Ok(())
}

#[gtest]
fn test_user_of_unsupported_type_is_unsupported() -> Result<()> {
    let ir = ir_from_cc(
        r#"struct __attribute__((packed)) Packed {};
           void f(Packed n);
        "#,
    )?;
    let names = ir.unsupported_items().map(|i| i.name.as_ref()).collect_vec();
    assert_strings_contain(names.as_ref(), "Packed");
    assert_strings_contain(names.as_ref(), "f");
    Ok(())
}

fn assert_strings_contain(strings: &[&str], expected_string: &str) {
    assert!(
        strings.iter().any(|s| *s == expected_string),
        "Value '{}' was unexpectedly missing from {:?}",
        expected_string,
        strings
    );
}

fn assert_strings_dont_contain(strings: &[&str], unexpected_pattern: &str) {
    assert!(
        strings.iter().all(|s| !s.contains(unexpected_pattern)),
        "Pattern {:?} was unexpectedly found in {:?}",
        unexpected_pattern,
        strings
    );
}

#[gtest]
fn test_elided_lifetimes() {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct S {
          int& f(int& i);
        };"#,
    )
    .unwrap();
    let func = retrieve_func(&ir, "f");
    let lifetime_params = &func.lifetime_params;
    assert_eq!(lifetime_params.iter().map(|p| p.name.as_ref()).collect_vec(), vec!["a", "b"]);
    let a_id = lifetime_params[0].id;
    let b_id = lifetime_params[1].id;
    assert_eq!(func.return_type.variant.as_pointer().unwrap().lifetime.unwrap(), a_id);

    assert_eq!(func.params[0].identifier, ir_id("__this"));
    let ptr = &func.params[0].type_.variant.as_pointer().unwrap();
    assert!(!ptr.pointee_type.is_const);
    assert_eq!(ptr.lifetime.unwrap(), a_id);

    assert_eq!(func.params[1].identifier, ir_id("i"));
    let ptr = &func.params[1].type_.variant.as_pointer().unwrap();
    assert!(!ptr.pointee_type.is_const);
    assert_eq!(ptr.lifetime.unwrap(), b_id);
}

fn verify_elided_lifetimes_in_default_constructor(ir: &IR) {
    let r = ir.records().next().expect("IR should contain `struct S`");
    assert_eq!(r.rs_name.identifier.as_ref(), "S");
    assert!(r.is_trivial_abi);

    let f = ir
        .functions()
        .find(|f| matches!(&f.rs_name, UnqualifiedIdentifier::Constructor) && f.params.len() == 1)
        .expect("IR should contain the default constructor");
    assert_eq!(f.lifetime_params.len(), 1);

    let p = f.params.first().expect("IR should contain `__this` parameter");
    assert_eq!(p.identifier, ir_id("__this"));

    let p_ptr = p.type_.variant.as_pointer().unwrap();
    assert_eq!(p_ptr.lifetime.unwrap(), f.lifetime_params[0].id);
    assert!(!p_ptr.pointee_type.is_const);
}

#[gtest]
fn test_operator_names() {
    let ir = ir_from_cc(
        r#"
        // TOOD(b/208377928): Use #include <stddef.h> instead of declaring `size_t` ourselves...
        using size_t = unsigned long;
        #pragma clang lifetime_elision
        struct SomeStruct {
          // There is an implicit/default `oparator=` hidden here as well.
          void* operator new(size_t size);
          void* operator new[](size_t size);
          bool operator==(const SomeStruct& other) const;
        };"#,
    )
    .unwrap();
    let operator_names: HashSet<&str> = ir
        .functions()
        .filter(|f| {
            // Only SomeStruct member functions (excluding stddef.h stuff).
            if let Some(Ok(r)) = ir.record_for_member_func(f).map(<&Rc<Record>>::try_from) {
                r.rs_name.identifier.as_ref() == "SomeStruct"
            } else {
                false
            }
        })
        .flat_map(|f| match &f.rs_name {
            UnqualifiedIdentifier::Operator(op) => Some(op.name.as_ref()),
            _ => None,
        })
        .collect();
    assert!(operator_names.contains("="));
    assert!(operator_names.contains("new"));
    assert!(operator_names.contains("new[]"));
    assert!(operator_names.contains("=="));
}

#[gtest]
fn test_elided_lifetimes_in_default_constructor_with_implicit_default() {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct S {
          int i;
        };"#,
    )
    .unwrap();
    verify_elided_lifetimes_in_default_constructor(&ir);
}

#[gtest]
fn test_elided_lifetimes_in_default_constructor_with_explicit_default() {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct S {
          S() = default;
          int i;
        };"#,
    )
    .unwrap();
    verify_elided_lifetimes_in_default_constructor(&ir);
}

#[gtest]
fn test_no_aligned_attr() {
    let ir = ir_from_cc("struct SomeStruct {};").unwrap();

    assert_ir_matches! {ir, quote! {
      Record {
        ... rs_name: "SomeStruct" ...
        ... size_align: SizeAlign {
            size: 1,
            alignment: 1,
        } ...
        ... override_alignment: false ...
      }}
    };
}

#[gtest]
fn test_aligned_attr() {
    let ir = ir_from_cc("struct SomeStruct {} __attribute__((aligned(64)));").unwrap();
    assert_ir_matches! {ir, quote! {
      Record {
        ... rs_name: "SomeStruct" ...
        ... size_align: SizeAlign {
            size: 64,
            alignment: 64,
        } ...
        ... override_alignment: true ...
      }}
    };
}

#[gtest]
fn test_template_with_preferred_name_attribute() {
    let ir = ir_from_cc(
        r#"
      template<typename T> struct basic_string;
      using string = basic_string<char>;
      template<typename T> struct [[clang::preferred_name(string)]] basic_string {
      };
      "#,
    )
    .unwrap();
    assert_ir_matches! {ir, quote! {
      ...
      Record {
        ... cc_name: "basic_string<char>" ...
        ... cc_preferred_name: "string" ...
        ... unknown_attr: None ...
      }
      ...
    }
    };
}

#[gtest]
fn test_template_without_preferred_name_attribute() {
    let ir = ir_from_cc(
        r#"
      template<typename T> struct basic_string {};
      using string = basic_string<char>;
      "#,
    )
    .unwrap();
    assert_ir_matches! {ir, quote! {
      ...
      Record {
        ... cc_name: "basic_string<char>" ...
        ... cc_preferred_name: "basic_string<char>" ...
        ... unknown_attr: None ...
      }
      ...
    }
    };
}

#[gtest]
fn test_class_template_specialization_information_collection() {
    let ir = ir_from_cc(
        r#"
      namespace ns {
      template<typename T, typename U> struct basic_string {};

      using string = basic_string<char, int>;
      }
      "#,
    )
    .unwrap();
    assert_ir_matches! {ir, quote! {
      ...
      Record {
        ... cc_name: "ns::basic_string<char, int>" ...
        ... template_specialization: Some(TemplateSpecialization {
            template_name: "ns::basic_string",
            template_args: [
                TemplateArg {
                    type_: Ok(CcType { variant: Primitive(Char), ...  }),
                },
                TemplateArg {
                    type_: Ok(CcType { variant: Primitive(Int), ...  }),
                },
            ],
        }) ...
      }
      ...
    }
    };
}

#[gtest]
fn test_record_with_pointer_attribute() {
    let ir = ir_from_cc(
        r#"
      class [[gsl::Pointer(int)]] Struct {};
      "#,
    )
    .unwrap();
    assert_ir_matches! {ir, quote! {
      Record {
        ... rs_name: "Struct" ...
        ... unknown_attr: None ...
      }
    }
    };
}

#[gtest]
fn test_record_with_owner_attribute() {
    let ir = ir_from_cc(
        r#"
      class [[gsl::Owner(int)]] Struct {};
      "#,
    )
    .unwrap();
    assert_ir_matches! {ir, quote! {
      Record {
        ... rs_name: "Struct" ...
        ... unknown_attr: None ...
      }
    }
    };
}

#[gtest]
fn test_c_style_struct_with_typedef_and_aligned_attr() {
    let ir = ir_from_cc("typedef struct {} SomeStruct __attribute__((aligned(64)));").unwrap();

    assert_ir_matches! {ir, quote! {
      Record {
        ... rs_name: "SomeStruct" ...
        ... size_align: SizeAlign {
            size: 64,
            alignment: 64,
        } ...
        ... override_alignment: true ...
      }}
    };
}

#[gtest]
fn test_volatile_is_unsupported() {
    let ir = ir_from_cc("volatile int* foo();").unwrap();
    let f = ir
        .unsupported_items()
        .find(|i| i.errors().iter().any(|e| e.to_string().contains("volatile")))
        .unwrap();
    assert_eq!("foo", f.name.as_ref());
}

#[gtest]
fn test_unnamed_enum_unsupported() {
    let ir = ir_from_cc("enum { kFoo = 1, kBar = 2 };").unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            UnsupportedItem {
                name: "(unnamed enum at ./ir_from_cc_virtual_header.h:3:1)",
                kind: Enum,
                path: None,
                errors: [FormattedError {
                    ..., message: "Unnamed enums are not supported yet", ...
                }], ...
            }
        }
    );
}

#[gtest]
fn test_literal_operator_unsupported() {
    let ir = ir_from_cc(
        r#"
        // clang::DeclarationName::NameKind::CXXLiteralOperatorName
        unsigned operator ""_foobar(const char*);
    "#,
    )
    .unwrap();
    assert_ir_matches!(
        ir,
        quote! {
            UnsupportedItem {
                name: "operator\"\"_foobar",
                kind: Func,
                path: None,
                errors: [FormattedError {
                    ..., message: "Function name is not supported: Unsupported name: operator\"\"_foobar", ...
                }], ...
            }
        }
    );
}

#[gtest]
fn test_unsupported_item_has_item_id() {
    let ir = ir_from_cc("struct __attribute__((packed)) SomeStruct {};").unwrap();
    let unsupported = ir.unsupported_items().find(|i| i.name.as_ref() == "SomeStruct").unwrap();
    assert_ne!(unsupported.id, ItemId::new_for_testing(0));
}

#[gtest]
fn test_comment_has_item_id() {
    let ir = ir_from_cc("// Comment").unwrap();
    let comment = ir.comments().find(|i| i.text.as_ref() == "Comment").unwrap();
    assert_ne!(comment.id, ItemId::new_for_testing(0));
}

#[gtest]
fn test_function_has_item_id() {
    let ir = ir_from_cc("int foo();").unwrap();
    let function = ir
        .functions()
        .find(|i| i.rs_name == UnqualifiedIdentifier::Identifier(ir_id("foo")))
        .unwrap();
    assert_ne!(function.id, ItemId::new_for_testing(0));
}

#[gtest]
fn test_top_level_items() {
    let ir = ir_from_cc(
        r#"
        struct ForwardDeclaredTopLevelStruct;
        struct TopLevelStruct;
        struct TopLevelStruct {};
        // Top level comment

        // Function comment
        void top_level_func();
        namespace top_level_namespace {
        struct Nested {};
        // free nested comment

        // nested_func comment
        void nested_func();
        }  // namespace top_level_namespace"#,
    )
    .unwrap();

    let top_level_items =
        ir.top_level_item_ids().iter().map(|id| ir.find_decl(*id).unwrap()).collect_vec();

    assert_items_match!(
        top_level_items,
        vec![
            quote! {
              IncompleteRecord {
                ... cc_name: "ForwardDeclaredTopLevelStruct" ...
              }
            },
            quote! {
              Record {
                ... rs_name: "TopLevelStruct" ...
              }
            },
            quote! {
              Comment {
                ... text: "Top level comment" ...
              }
            },
            quote! {
              Func { ... rs_name: "top_level_func" ... }
            },
            quote! {
              Namespace { ... rs_name: "top_level_namespace" ... }
            },
            quote! {
              Comment {
                ... text: "namespace top_level_namespace" ...
              }
            },
        ]
    );
}

#[gtest]
fn test_record_items() {
    let ir = ir_from_cc(
        r#"
        struct TopLevelStruct {
          // A free comment

          // foo comment
          int foo;

          int bar();
          struct Nested {};
          int baz();

          // clang::DeclarationName::NameKind::CXXConversionFunctionName
          explicit operator int() const { return 123; }
        };"#,
    )
    .unwrap();

    let record = ir.records().find(|i| i.rs_name == "TopLevelStruct").unwrap();
    let record_items =
        record.child_item_ids.iter().map(|id| ir.find_decl(*id).unwrap()).collect_vec();

    assert_items_match!(
        record_items,
        vec![
            quote! {
              Func { ... rs_name: Constructor ... }
            },
            quote! {
              Func { ... rs_name: Constructor ... }
            },
            quote! {
              Func { ... rs_name: Constructor, ... }
            },
            quote! {
              Func { ... rs_name: Destructor ... }
            },
            quote! {
              Func { ... rs_name: "operator=" ... }
            },
            quote! {
              Func { ... rs_name: "operator=" ... }
            },
            quote! {
              ...Comment {
                ... text: "A free comment" ...
              }
            },
            quote! {
              ... Func { ... rs_name: "bar" ... }
            },
            quote! {
              ... Record { ... rs_name: "Nested" ... }
            },
            quote! {
              ...Func {
                ... rs_name: "baz" ...
              }
            },
            quote! {
              ... UnsupportedItem {
                  name: "TopLevelStruct::operator int",
                  kind: Func,
                  path: None,
                  errors: [FormattedError {
                    ..., message: "Function name is not supported: Unsupported name: operator int",
                  }],
                  ...
              }
            },
        ]
    );
}

#[gtest]
fn test_namespaces() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings {
          // A free comment

          // Struct comment
          struct StructWithinNamespace {};

          void function_within_namespace();

          namespace inner_namespace {
          struct InnerStruct {};
          }  // namespace inner_namespace
          }  // namespace test_namespace_bindings"#,
    )
    .unwrap();

    let namespace =
        ir.namespaces().find(|n| n.rs_name == ir_id("test_namespace_bindings")).unwrap();
    let namespace_items =
        namespace.child_item_ids.iter().map(|id| ir.find_decl(*id).unwrap()).collect_vec();

    assert_ir_matches!(
        ir,
        quote! {
            ...
            Namespace {
                cc_name: "test_namespace_bindings",
                rs_name: "test_namespace_bindings" ...
                id: ItemId(...) ...
                canonical_namespace_id: ItemId(...) ...
                owning_target: BazelLabel("//test:testing_target") ...
            }
            ...
        }
    );

    assert_items_match!(
        namespace_items,
        vec![
            quote! {
              Comment {
                ... text: "A free comment" ...
              }
            },
            quote! {
              Record {
                ... rs_name : "StructWithinNamespace" ...
              }
            },
            quote! {
              Func { ... rs_name: "function_within_namespace" ... }
            },
            quote! {
              Namespace { ... rs_name: "inner_namespace" ... }
            },
            quote! {
              Comment {
                ... text: "namespace inner_namespace" ...
              }
            },
        ]
    );
}

#[gtest]
fn test_nested_namespace_definition() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings::inner {
        void func();
        }"#,
    )
    .unwrap();

    let namespace =
        ir.namespaces().find(|n| n.rs_name == ir_id("test_namespace_bindings")).unwrap();
    let namespace_items =
        namespace.child_item_ids.iter().map(|id| ir.find_decl(*id).unwrap()).collect_vec();

    assert_items_match!(
        namespace_items,
        vec![quote! {
          Namespace { ... rs_name: "inner" ... }
        },]
    );

    let inner_namespace = ir.namespaces().find(|n| n.rs_name == ir_id("inner")).unwrap();
    let inner_namespace_items =
        inner_namespace.child_item_ids.iter().map(|id| ir.find_decl(*id).unwrap()).collect_vec();

    assert_items_match!(
        inner_namespace_items,
        vec![quote! {
          Func { ... rs_name: "func" ... }
        },]
    );
}

#[gtest]
fn test_enclosing_item_ids() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings {
          struct T {};
          struct S {
            void processT();
          };
          void f();
          enum E {};
          typedef int TypedefDecl;
          using TypeAliasDecl = int;
          namespace inner {
            struct InnerS {};
            void inner_f();
            enum InnerE {};
            typedef int InnerTypedefDecl;
            using InnerTypeAliasDecl = int;
          }
        }"#,
    )
    .unwrap();

    let namespace =
        ir.namespaces().find(|n| n.rs_name == ir_id("test_namespace_bindings")).unwrap();
    let namespace_items: Vec<&Item> =
        namespace.child_item_ids.iter().map(|id| ir.find_decl(*id).unwrap()).collect_vec();

    assert_eq!(namespace.enclosing_item_id, None);
    assert!(namespace_items.iter().all(|item| item.enclosing_item_id() == Some(namespace.id)));

    let inner_namespace = ir.namespaces().find(|n| n.rs_name == ir_id("inner")).unwrap();
    let inner_namespace_items: Vec<&Item> =
        inner_namespace.child_item_ids.iter().map(|id| ir.find_decl(*id).unwrap()).collect_vec();

    assert!(inner_namespace_items
        .iter()
        .all(|item| item.enclosing_item_id() == Some(inner_namespace.id)));

    let record = ir.records().find(|r| r.rs_name.identifier.as_ref() == "S").unwrap();
    let record_items: Vec<&Item> =
        record.child_item_ids.iter().map(|id| ir.find_decl(*id).unwrap()).collect_vec();
    for item in record_items.iter() {
        match item {
            Item::UnsupportedItem(_) => {}
            Item::Comment(_) => {}
            _ => {
                assert!(item.enclosing_item_id() == Some(record.id));
            }
        }
    }
}

#[gtest]
fn test_namespace_canonical_id() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings {
          struct T {};
        }
        int i;
        namespace test_namespace_bindings {
          struct Y {};
        }"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            ...
            Namespace {
                cc_name: "test_namespace_bindings",
                rs_name: "test_namespace_bindings" ...
                id: ItemId(...) ...
                canonical_namespace_id: ItemId(...) ...
            }
            ...
        }
    );

    let namespaces = ir.namespaces().collect_vec();
    assert_eq!(namespaces.len(), 2);
    assert_eq!(namespaces[0].id, namespaces[0].canonical_namespace_id);
    assert_eq!(namespaces[0].canonical_namespace_id, namespaces[1].canonical_namespace_id);
}

#[gtest]
fn test_reopened_namespaces() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings {
        namespace inner {}
        }

        namespace test_namespace_bindings {
        namespace inner {}
        }"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            ...
            Namespace(Namespace {
                cc_name: "test_namespace_bindings" ...
            })
            ...
            Namespace(Namespace {
              cc_name: "inner" ...
            })
            ...
            Namespace(Namespace {
              cc_name: "test_namespace_bindings" ...
            })
            ...
            Namespace(Namespace {
              cc_name: "inner" ...
            })
            ...
        }
    );
}

#[gtest]
fn test_namespace_stored_data_in_ir() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings {
          namespace inner {}
        }
        namespace test_namespace_bindings {
          namespace inner {}
          namespace inner {}
        }"#,
    )
    .unwrap();

    let outer_namespaces =
        ir.namespaces().filter(|ns| ns.rs_name == ir_id("test_namespace_bindings")).collect_vec();
    assert_eq!(outer_namespaces.len(), 2);

    assert_eq!(ir.get_reopened_namespace_idx(outer_namespaces[0].id).unwrap(), 0);
    assert_eq!(ir.get_reopened_namespace_idx(outer_namespaces[1].id).unwrap(), 1);

    assert!(!ir
        .is_last_reopened_namespace(
            outer_namespaces[0].id,
            outer_namespaces[0].canonical_namespace_id
        )
        .unwrap());
    assert!(ir
        .is_last_reopened_namespace(
            outer_namespaces[1].id,
            outer_namespaces[1].canonical_namespace_id
        )
        .unwrap());

    let inner_namespaces = ir.namespaces().filter(|ns| ns.rs_name == ir_id("inner")).collect_vec();
    assert_eq!(inner_namespaces.len(), 3);

    assert_eq!(ir.get_reopened_namespace_idx(inner_namespaces[0].id).unwrap(), 0);
    assert_eq!(ir.get_reopened_namespace_idx(inner_namespaces[1].id).unwrap(), 1);
    assert_eq!(ir.get_reopened_namespace_idx(inner_namespaces[2].id).unwrap(), 2);

    assert!(!ir
        .is_last_reopened_namespace(
            inner_namespaces[0].id,
            inner_namespaces[0].canonical_namespace_id
        )
        .unwrap());
    assert!(!ir
        .is_last_reopened_namespace(
            inner_namespaces[1].id,
            inner_namespaces[1].canonical_namespace_id
        )
        .unwrap());
    assert!(ir
        .is_last_reopened_namespace(
            inner_namespaces[2].id,
            inner_namespaces[2].canonical_namespace_id
        )
        .unwrap());
}

#[gtest]
fn test_items_inside_linkage_spec_decl_are_imported() {
    let ir = ir_from_cc(
        r#"
          extern "C" {
            struct MyStruct {};
          }
      "#,
    )
    .unwrap();
    assert_ir_matches!(ir, quote! { Record { ... cc_name: "MyStruct" ... } })
}

#[gtest]
fn test_items_inside_linkage_spec_decl_are_considered_toplevel() {
    // The test below assumes the first top_level_item_ids element is the one added
    // by the the source code under test. Let's double check that assumption here.
    assert!(ir_from_cc("").unwrap().top_level_item_ids().is_empty());

    let ir = ir_from_cc(
        r#"
    extern "C" {
      struct MyStruct {};
    }"#,
    )
    .unwrap();
    let item_id = ir.top_level_item_ids()[0];

    assert_ir_matches!(
        ir,
        quote! {
          ...
          Record {
            ... cc_name: "MyStruct" ...
            ... id: ItemId(#item_id) ...
          }
          ...
        }
    );
}

#[gtest]
fn test_inline_namespace() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings {
          inline namespace inner {
            struct MyStruct {};
          }
        }"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            ...
            Namespace(Namespace {
                cc_name: "test_namespace_bindings" ...
            }) ...
            Namespace(Namespace {
                cc_name: "inner" ...
            }) ...
        }
    );
}

#[gtest]
fn test_function_redeclared_as_friend() {
    let ir = ir_from_cc(
        r#"
            class SomeClass final {
              friend constexpr int bar();
            };
            constexpr int bar() { return 123; }
        "#,
    )
    .unwrap();

    // The function should appear only once in IR items.  (This is a bit redundant
    // with the assert below, but double-checks that `...` didn't miss a Func
    // item.)
    let functions = ir
        .functions()
        .filter(|f| f.rs_name == UnqualifiedIdentifier::Identifier(ir_id("bar")))
        .collect_vec();
    assert_eq!(1, functions.len());
    let function_id = functions[0].id;

    // There should only be a single Func item.
    //
    // Additionally, this assert also verifies that `child_item_ids` and
    // `top_level_item_ids` have the right length, which indirectly verifies
    // that the `function_id` is not included in `top_level_item_ids` and is
    // included in the record's `child_item_ids`).
    assert_ir_matches!(
        ir,
        quote! {
            items: [
                ...
                Record(Record {
                    rs_name: "SomeClass" ...
                    child_item_ids: [
                        ItemId(...),
                        ItemId(...),
                        ItemId(...),
                        ItemId(...),
                        ItemId(...),
                        ItemId(...),
                        ItemId(#function_id),
                    ] ...
                    enclosing_item_id: None ...
                }),
                Func(Func { cc_name: Constructor, rs_name: Constructor ...  }),
                Func(Func { cc_name: Constructor, rs_name: Constructor ...  }),
                Func(Func { cc_name: Constructor, rs_name: Constructor ...  }),
                Func(Func { cc_name: Destructor, rs_name: Destructor ...  }),
                Func(Func { cc_name: "operator=", rs_name: "operator=" ...  }),
                Func(Func { cc_name: "operator=", rs_name: "operator=" ...  }),
                Func(Func {
                    cc_name: "bar",
                    rs_name: "bar" ...
                    enclosing_item_id: None ...
                    adl_enclosing_record: Some(ItemId(...)) ...
                }),
            ],
            top_level_item_ids: map! { ... BazelLabel(#TESTING_TARGET): [ItemId(...)] ... }
        }
    );
}

#[gtest]
fn test_function_redeclared_in_separate_namespace_chunk() {
    let ir = ir_from_cc(
        r#"
        namespace ns { inline void f(); }
        namespace ns { inline void f() {} }
        "#,
    )
    .unwrap();

    // The function should appear only once in IR items.  (This is a bit redundant
    // with the assert below, but double-checks that `...` didn't miss a Func
    // item.)
    let functions = ir
        .functions()
        .filter(|f| f.rs_name == UnqualifiedIdentifier::Identifier(ir_id("f")))
        .collect_vec();
    assert_eq!(1, functions.len());
    let function_id = functions[0].id;

    // The function should appear only once.  This assert not only verifies that the
    // `Func` item appears only once, but it also verifies that it also only
    // appears once in `child_item_ids`.
    assert_ir_matches!(
        ir,
        quote! {
            items: [
                ...
                Namespace(Namespace {
                    cc_name: "ns",
                    rs_name: "ns" ...
                    child_item_ids: [ItemId(#function_id)] ...
                    enclosing_item_id: None ...
                }),
                Func(Func {
                    cc_name: "f",
                    rs_name: "f" ...
                    enclosing_item_id: Some(ItemId(...)) ...
                }),
                Namespace(Namespace {
                    cc_name: "ns",
                    rs_name: "ns" ...
                    child_item_ids: [] ...
                }),
            ]
        }
    );
}

#[gtest]
fn test_forward_declared_specialization_has_rs_name() {
    let ir = ir_from_cc(
        r#"
        namespace test_namespace_bindings {
          template <typename T>
          struct MyTemplate {
            void processT(T t);
          };

          struct Param {};

          template<> struct MyTemplate<Param>;

          using MyTypeAlias = MyTemplate<Param>;
        }"#,
    )
    .unwrap();

    assert_ir_matches!(
        ir,
        quote! {
            ...
            IncompleteRecord {
              cc_name: "test_namespace_bindings::MyTemplate<test_namespace_bindings::Param>",
              rs_name: "__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_5ParamEEE",
              ...
            } ...
        }
    );
}

#[gtest]
fn test_friend() {
    let ir = ir_from_cc(
        r#"
        struct MyStruct {
          friend int Invisible();
          friend int VisibleByADL(MyStruct& x);
        };"#,
    )
    .unwrap();

    // NOTE: Actual ADL visibility determination is handled by the IR consumer.
    // These two friend functions have similar IR representations.
    assert_ir_matches!(
        ir,
        quote! {
            ...
            Func { ... rs_name: "Invisible", ... adl_enclosing_record: Some(...) ... } ...
        }
    );
    assert_ir_matches!(
        ir,
        quote! {
            ...
            Func { ... rs_name: "VisibleByADL", ... adl_enclosing_record: Some(...) ... } ...
        }
    );
}

fn generate_member_func_with_visibility(record_type: &str, visibility: &str) -> String {
    format!(
        r#"
{record_type} Bar {{
 {visibility}
  void myfunc() {{}}
}};"#
    )
}

#[gtest]
fn test_private_method() {
    let ir_with_function = quote! {
      ...
      Func { ... rs_name: "myfunc", ... }
      ...
    };
    for (record_type, visibility, expect_function) in vec![
        ("struct", "public:", true),
        ("struct", "protected:", false),
        ("struct", "private:", false),
        // tests without visiblity keyword, public is the default for struct
        ("struct", "", true),
        ("class", "public:", true),
        ("class", "protected:", false),
        ("class", "private:", false),
        // tests without visiblity keyword, private is the default for class
        ("class", "", false),
    ] {
        let record = generate_member_func_with_visibility(record_type, visibility);
        let ir = ir_from_cc(&record).unwrap();
        eprintln!("{}", record);
        if expect_function {
            assert_ir_matches!(ir, ir_with_function);
        } else {
            assert_ir_not_matches!(ir, ir_with_function);
        }
    }
}

#[gtest]
fn test_source_location_with_macro() {
    let assert_matches = |cc_snippet: &str, expected: proc_macro2::TokenStream| {
        let ir = ir_from_cc(cc_snippet).unwrap();
        assert_ir_matches!(ir, expected);
    };
    let loc = "Generated from: ir_from_cc_virtual_header.h;l=5\n\
        Expanded at: ir_from_cc_virtual_header.h;l=7";
    assert_matches(
        r#"
#define NO_OP_FUNC(func_name) \
  void fun_name();

NO_OP_FUNC(no_op_func_to_test_source_location_with_macro);"#,
        quote! {Func { ..., source_loc: #loc, ... } },
    );

    let loc = "Generated from: ir_from_cc_virtual_header.h;l=4\n\
        Expanded at: ir_from_cc_virtual_header.h;l=5";
    assert_matches(
        r#"
#define TYPE_ALIAS_TO_INT(type_alias) using type_alias = int;
TYPE_ALIAS_TO_INT(MyIntToTestSourceLocationWithMacro);"#,
        quote! {TypeAlias { ..., source_loc: #loc, ... } },
    );
    let loc = "Generated from: ir_from_cc_virtual_header.h;l=4\n\
        Expanded at: ir_from_cc_virtual_header.h;l=6";
    assert_matches(
        r#"
#define TEMPLATE_NO_OP_FUNC(func_name) \
template <typename T> void func_name() {};
  TEMPLATE_NO_OP_FUNC(unsupported_templated_no_op_func_to_test_source_location_with_macro);"#,
        quote! {UnsupportedItem { ..., source_loc: Some(#loc,), ... } },
    );

    let loc = "Generated from: ir_from_cc_virtual_header.h;l=5\n\
        Expanded at: ir_from_cc_virtual_header.h;l=6";
    assert_matches(
        r#"
#define DEFINE_EMPTY_ENUM(enum_name) \
  enum enum_name {};
DEFINE_EMPTY_ENUM(EmptyEnumToTestSourceLocationWithMacro);"#,
        quote! {Enum { ..., source_loc: #loc, ... } },
    );

    let loc = "Generated from: ir_from_cc_virtual_header.h;l=5\n\
        Expanded at: ir_from_cc_virtual_header.h;l=6";
    assert_matches(
        r#"
#define DEFINE_EMPTY_STRUCT(struct_name) \
  struct struct_name {};
DEFINE_EMPTY_STRUCT(EmptyStructToTestSourceLocationWithMacro);"#,
        quote! {Record { ..., source_loc: #loc, ... } },
    );
}

#[gtest]
fn test_source_location() {
    let assert_matches = |cc_snippet: &str, expected: proc_macro2::TokenStream| {
        let ir = ir_from_cc(cc_snippet).unwrap();
        assert_ir_matches!(ir, expected);
    };
    assert_matches(
        "void no_op_func_to_test_source_location();",
        quote! {Func { ..., source_loc: "Generated from: ir_from_cc_virtual_header.h;l=3", ... } },
    );
    assert_matches(
        r#"typedef float SomeTypedefToTestSourceLocation;"#,
        quote! {TypeAlias { ..., source_loc: "Generated from: ir_from_cc_virtual_header.h;l=3", ... } },
    );
    assert_matches(
        r#"  template <typename T> void unsupported_templated_func_to_test_source_location() {}"#,
        quote! {UnsupportedItem { ..., source_loc: Some("Generated from: ir_from_cc_virtual_header.h;l=3"), ... } },
    );
    assert_matches(
        r#"enum SomeEmptyEnumToTestSourceLocation {};"#,
        quote! {Enum { ..., source_loc: "Generated from: ir_from_cc_virtual_header.h;l=3", ... } },
    );
    assert_matches(
        r#"struct SomeEmptyStructToTestSourceLocation {};"#,
        quote! {Record { ..., source_loc: "Generated from: ir_from_cc_virtual_header.h;l=3", ... } },
    );
}

#[gtest]
fn test_source_location_with_macro_defined_in_another_file() {
    let dependency_header = r#"
#define MyIntTypeAliasToTestSourceLocation(type_alias_name) using type_alias_name = int;"#;
    let header = "MyIntTypeAliasToTestSourceLocation(my_int);";
    let ir = ir_from_cc_dependency(header, dependency_header).unwrap();
    let expected_source_loc = "Generated from: test/dependency_header.h;l=2\n\
                               Expanded at: ir_from_cc_virtual_header.h;l=3";
    assert_ir_matches!(
        ir,
        quote! {
        ...
        TypeAlias {
          ...
            source_loc: #expected_source_loc,
          ...
          }
        ...
        }
    );
}

#[gtest]
fn test_source_location_class_template_specialization() {
    let cc_snippet = "template <typename T> class MyClassTemplateToTestSourceLocation { T t_; };
    using MyClassTemplateSpecializationToTestSourceLocation = MyClassTemplateToTestSourceLocation<bool>;";
    let ir = ir_from_cc(cc_snippet).unwrap();
    let expected_source_loc = "Generated from: ir_from_cc_virtual_header.h;l=4";
    assert_ir_matches!(
        ir,
        quote! {
        ...
        TypeAlias {
          ...
            source_loc: #expected_source_loc,
          ...
          }
        ...
        }
    );
}

#[gtest]
fn test_top_level_item_ids_from_multiple_targets() {
    let dependency_header = r#"struct FromDependency {};"#;
    let header = "struct FromHeader {};";
    let ir = ir_from_cc_dependency(header, dependency_header).unwrap();
    assert_ir_matches!(
        ir,
        quote! {
          top_level_item_ids: map! {
            ...
            BazelLabel(#DEPENDENCY_TARGET): [...]
            ...
            BazelLabel(#TESTING_TARGET): [...]
            ...
          }
        }
    );
}
