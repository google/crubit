// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use googletest::prelude::gtest;
use multiplatform_ir_testing::ir_from_cc;
use quote::quote;
use test_generators::generate_bindings_tokens_for_test;
use token_stream_matchers::{assert_rs_matches, assert_rs_not_matches};

#[gtest]
fn test_generate_enum_basic() -> Result<()> {
    let ir = ir_from_cc("enum Color { kRed = 5, kBlue };")?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(transparent)]
            #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Color"]
            pub struct Color(::ffi_11::c_uint);
            impl Color {
                pub const kRed: Color = Color(5);
                pub const kBlue: Color = Color(6);
            }
            impl From<::ffi_11::c_uint> for Color {
                fn from(value: ::ffi_11::c_uint) -> Color {
                    Color(value)
                }
            }
            impl From<Color> for ::ffi_11::c_uint {
                fn from(value: Color) -> ::ffi_11::c_uint {
                    value.0
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_generate_opaque_enum() -> Result<()> {
    let ir = ir_from_cc("enum Color : int;")?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {Color});
    Ok(())
}

#[gtest]
fn test_generate_scoped_enum_basic() -> Result<()> {
    let ir = ir_from_cc("enum class Color { kRed = -5, kBlue };")?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(transparent)]
            #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Color"]
            pub struct Color(::ffi_11::c_int);
            impl Color {
                pub const kRed: Color = Color(-5);
                pub const kBlue: Color = Color(-4);
            }
            impl From<::ffi_11::c_int> for Color {
                fn from(value: ::ffi_11::c_int) -> Color {
                    Color(value)
                }
            }
            impl From<Color> for ::ffi_11::c_int {
                fn from(value: Color) -> ::ffi_11::c_int {
                    value.0
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_generate_enum_with_64_bit_signed_vals() -> Result<()> {
    let ir = ir_from_cc(
        r#"enum Color : long {
                kViolet = -9223372036854775807 - 1LL,
                kRed = -5,
                kBlue,
                kGreen = 3,
                kMagenta = 9223372036854775807
            };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(transparent)]
            #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Color"]
            pub struct Color(::ffi_11::c_long);
            impl Color {
                pub const kViolet: Color = Color(-9223372036854775808);
                pub const kRed: Color = Color(-5);
                pub const kBlue: Color = Color(-4);
                pub const kGreen: Color = Color(3);
                pub const kMagenta: Color = Color(9223372036854775807);
            }
            impl From<::ffi_11::c_long> for Color {
                fn from(value: ::ffi_11::c_long) -> Color {
                    Color(value)
                }
            }
            impl From<Color> for ::ffi_11::c_long {
                fn from(value: Color) -> ::ffi_11::c_long {
                    value.0
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_generate_enum_with_64_bit_unsigned_vals() -> Result<()> {
    let ir = ir_from_cc(
        r#" enum Color: unsigned long {
                kRed,
                kBlue,
                kLimeGreen = 18446744073709551615
            }; "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(transparent)]
            #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Color"]
            pub struct Color(::ffi_11::c_ulong);
            impl Color {
                pub const kRed: Color = Color(0);
                pub const kBlue: Color = Color(1);
                pub const kLimeGreen: Color = Color(18446744073709551615);
            }
            impl From<::ffi_11::c_ulong> for Color {
                fn from(value: ::ffi_11::c_ulong) -> Color {
                    Color(value)
                }
            }
            impl From<Color> for ::ffi_11::c_ulong {
                fn from(value: Color) -> ::ffi_11::c_ulong {
                    value.0
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_generate_enum_with_32_bit_signed_vals() -> Result<()> {
    let ir = ir_from_cc(
        "enum Color { kViolet = -2147483647 - 1, kRed = -5, kBlue, kGreen = 3, kMagenta = 2147483647 };",
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(transparent)]
            #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Color"]
            pub struct Color(::ffi_11::c_int);
            impl Color {
                pub const kViolet: Color = Color(-2147483648);
                pub const kRed: Color = Color(-5);
                pub const kBlue: Color = Color(-4);
                pub const kGreen: Color = Color(3);
                pub const kMagenta: Color = Color(2147483647);
            }
            impl From<::ffi_11::c_int> for Color {
                fn from(value: ::ffi_11::c_int) -> Color {
                    Color(value)
                }
            }
            impl From<Color> for ::ffi_11::c_int {
                fn from(value: Color) -> ::ffi_11::c_int {
                    value.0
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_generate_enum_with_32_bit_unsigned_vals() -> Result<()> {
    let ir = ir_from_cc("enum Color: unsigned int { kRed, kBlue, kLimeGreen = 4294967295 };")?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(transparent)]
            #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Color"]
            pub struct Color(::ffi_11::c_uint);
            impl Color {
                pub const kRed: Color = Color(0);
                pub const kBlue: Color = Color(1);
                pub const kLimeGreen: Color = Color(4294967295);
            }
            impl From<::ffi_11::c_uint> for Color {
                fn from(value: ::ffi_11::c_uint) -> Color {
                    Color(value)
                }
            }
            impl From<Color> for ::ffi_11::c_uint {
                fn from(value: Color) -> ::ffi_11::c_uint {
                    value.0
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_generate_enum_bool() -> Result<()> {
    let ir = ir_from_cc("enum Bool : bool { kFalse, kTrue };")?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(transparent)]
            #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Bool"]
            pub struct Bool(bool);
            impl Bool {
                pub const kFalse: Bool = Bool(false);
                pub const kTrue: Bool = Bool(true);
            }
            impl From<bool> for Bool {
                fn from(value: bool) -> Bool {
                    Bool(value)
                }
            }
            impl From<Bool> for bool {
                fn from(value: Bool) -> bool {
                    value.0
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_generate_enum_bool_alias() -> Result<()> {
    let ir = ir_from_cc("using MyBool = bool; enum Bool : MyBool { kFalse, kTrue };")?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(transparent)]
            #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Bool"]
            pub struct Bool(crate::MyBool);
            impl Bool {
                pub const kFalse: Bool = Bool(false);
                pub const kTrue: Bool = Bool(true);
            }
            impl From<crate::MyBool> for Bool {
                fn from(value: crate::MyBool) -> Bool {
                    Bool(value)
                }
            }
            impl From<Bool> for crate::MyBool {
                fn from(value: Bool) -> crate::MyBool {
                    value.0
                }
            }
        }
    );
    Ok(())
}
