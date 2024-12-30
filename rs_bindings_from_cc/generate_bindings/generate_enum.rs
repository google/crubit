// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Generate bindings for C++ enum.
use crate::code_snippet::GeneratedItem;
use crate::db::{BindingsGenerator, Database};
use crate::generate_comment::generate_unsupported;
use arc_anyhow::Result;
use code_gen_utils::make_rs_ident;
use ir::{Enum, UnqualifiedIdentifier, UnsupportedItem, UnsupportedItemPath};
use proc_macro2::Literal;
use quote::{quote, ToTokens};
use std::collections::BTreeSet;

pub fn generate_enum(db: &Database, enum_: &Enum) -> Result<GeneratedItem> {
    let ident = crate::format_cc_ident(&enum_.identifier.identifier);
    let namespace_qualifier = db.ir().namespace_qualifier(enum_)?.format_for_cc()?;
    let fully_qualified_cc_name = quote! { #namespace_qualifier #ident }.to_string();
    let name = make_rs_ident(&enum_.identifier.identifier);
    let underlying_type = db.rs_type_kind(enum_.underlying_type.rs_type.clone())?;
    let Some(enumerators) = &enum_.enumerators else {
        return generate_unsupported(
            db,
            &UnsupportedItem::new_with_static_message(
                &db.ir(),
                enum_,
                Some(UnsupportedItemPath {
                    ident: UnqualifiedIdentifier::Identifier(enum_.identifier.clone()),
                    enclosing_item_id: enum_.enclosing_item_id,
                }),
                "b/322391132: Forward-declared (opaque) enums are not supported yet",
            ),
        );
    };
    let enumerators = enumerators.iter().map(|enumerator| {
        if let Some(unknown_attr) = &enumerator.unknown_attr {
            let comment = format!(
                "Omitting bindings for {ident}\nreason: unknown attribute(s): {unknown_attr}",
                ident = &enumerator.identifier.identifier
            );
            return quote! {
                __COMMENT__ #comment
            };
        }
        let ident = make_rs_ident(&enumerator.identifier.identifier);
        let value = if underlying_type.is_bool() {
            if enumerator.value.wrapped_value == 0 {
                quote! {false}
            } else {
                quote! {true}
            }
        } else {
            if enumerator.value.is_negative {
                Literal::i64_unsuffixed(enumerator.value.wrapped_value as i64).into_token_stream()
            } else {
                Literal::u64_unsuffixed(enumerator.value.wrapped_value).into_token_stream()
            }
        };
        quote! {pub const #ident: #name = #name(#value);}
    });

    let item = quote! {
        #[repr(transparent)]
        #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
        #[__crubit::annotate(cpp_type=#fully_qualified_cc_name)]
        pub struct #name(#underlying_type);
        impl #name {
            #(#enumerators)*
        }
        impl From<#underlying_type> for #name {
            fn from(value: #underlying_type) -> #name {
                #name(value)
            }
        }
        impl From<#name> for #underlying_type {
            fn from(value: #name) -> #underlying_type {
                value.0
            }
        }
    };
    Ok(GeneratedItem {
        item,
        features: BTreeSet::from([make_rs_ident("register_tool")]),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use arc_anyhow::Result;
    use googletest::prelude::*;
    use token_stream_matchers::{assert_rs_matches, assert_rs_not_matches};

    #[gtest]
    fn test_generate_enum_basic() -> Result<()> {
        let ir = ir_from_cc("enum Color { kRed = 5, kBlue };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_uint);
                impl Color {
                    pub const kRed: Color = Color(5);
                    pub const kBlue: Color = Color(6);
                }
                impl From<::core::ffi::c_uint> for Color {
                    fn from(value: ::core::ffi::c_uint) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_uint {
                    fn from(value: Color) -> ::core::ffi::c_uint {
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
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {Color});
        Ok(())
    }

    #[gtest]
    fn test_generate_scoped_enum_basic() -> Result<()> {
        let ir = ir_from_cc("enum class Color { kRed = -5, kBlue };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_int);
                impl Color {
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                }
                impl From<::core::ffi::c_int> for Color {
                    fn from(value: ::core::ffi::c_int) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_int {
                    fn from(value: Color) -> ::core::ffi::c_int {
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
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_long);
                impl Color {
                    pub const kViolet: Color = Color(-9223372036854775808);
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                    pub const kGreen: Color = Color(3);
                    pub const kMagenta: Color = Color(9223372036854775807);
                }
                impl From<::core::ffi::c_long> for Color {
                    fn from(value: ::core::ffi::c_long) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_long {
                    fn from(value: Color) -> ::core::ffi::c_long {
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
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_ulong);
                impl Color {
                    pub const kRed: Color = Color(0);
                    pub const kBlue: Color = Color(1);
                    pub const kLimeGreen: Color = Color(18446744073709551615);
                }
                impl From<::core::ffi::c_ulong> for Color {
                    fn from(value: ::core::ffi::c_ulong) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_ulong {
                    fn from(value: Color) -> ::core::ffi::c_ulong {
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
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_int);
                impl Color {
                    pub const kViolet: Color = Color(-2147483648);
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                    pub const kGreen: Color = Color(3);
                    pub const kMagenta: Color = Color(2147483647);
                }
                impl From<::core::ffi::c_int> for Color {
                    fn from(value: ::core::ffi::c_int) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_int {
                    fn from(value: Color) -> ::core::ffi::c_int {
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
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Color")]
                pub struct Color(::core::ffi::c_uint);
                impl Color {
                    pub const kRed: Color = Color(0);
                    pub const kBlue: Color = Color(1);
                    pub const kLimeGreen: Color = Color(4294967295);
                }
                impl From<::core::ffi::c_uint> for Color {
                    fn from(value: ::core::ffi::c_uint) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for ::core::ffi::c_uint {
                    fn from(value: Color) -> ::core::ffi::c_uint {
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
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Bool")]
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
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                #[__crubit::annotate(cpp_type = "Bool")]
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
}
