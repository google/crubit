// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use code_gen_utils::expect_format_cc_type_name;
use error_report::{anyhow, bail};
use ir::{CcType, Item, Record, IR};
use proc_macro2::TokenStream;
use quote::quote;

pub fn format_cpp_type(ty: &CcType, ir: &IR) -> Result<TokenStream> {
    // Formatting *both* pointers *and* references as pointers, because:
    // - Pointers and references have the same representation in the ABI.
    // - Clang's `-Wreturn-type-c-linkage` warns when using references in C++
    //   function thunks declared as `extern "C"` (see b/238681766).
    format_cpp_type_inner(ty, ir, /* references_ok= */ false)
}

pub fn cpp_type_name_for_record(record: &Record, ir: &IR) -> Result<TokenStream> {
    let tagless = cpp_tagless_type_name_for_record(record, ir)?;
    let tag_kind = record.cc_tag_kind();
    Ok(quote! { #tag_kind #tagless })
}

pub fn cpp_tagless_type_name_for_record(record: &Record, ir: &IR) -> Result<TokenStream> {
    let ident = expect_format_cc_type_name(record.cc_name.identifier.as_ref());
    let namespace_qualifier = ir.namespace_qualifier(record).format_for_cc()?;
    Ok(quote! { #namespace_qualifier #ident })
}

fn format_cpp_type_inner(ty: &CcType, ir: &IR, references_ok: bool) -> Result<TokenStream> {
    let const_fragment = if ty.is_const {
        quote! {const}
    } else {
        quote! {}
    };
    if let Some(ref name) = ty.name {
        match name.as_ref() {
            mut name @ ("*" | "&" | "&&") => {
                if ty.type_args.len() != 1 {
                    bail!("Invalid pointer type (need exactly 1 type argument): {:?}", ty);
                }
                let nested_type = format_cpp_type_inner(&ty.type_args[0], ir, references_ok)?;
                if !references_ok {
                    name = "*";
                }
                let ptr = match name {
                    "*" => quote! {*},
                    "&" => quote! {&},
                    "&&" => quote! {&&},
                    _ => unreachable!(),
                };
                Ok(quote! {#nested_type #ptr #const_fragment})
            }
            cpp_type_name => match cpp_type_name.strip_prefix("#funcValue ") {
                None => {
                    if !ty.type_args.is_empty() {
                        bail!("Type not yet supported: {:?}", ty);
                    }
                    // Not using `code_gen_utils::format_cc_ident`, because
                    // `cpp_type_name` may be a C++ reserved keyword (e.g.
                    // `int`).
                    let cc_ident: TokenStream = cpp_type_name.parse().unwrap();
                    Ok(quote! { #cc_ident #const_fragment })
                }
                Some(abi) => match ty.type_args.split_last() {
                    None => bail!("funcValue type without a return type: {:?}", ty),
                    Some((ret_type, param_types)) => {
                        // Function pointer types don't ignore references, but luckily,
                        // `-Wreturn-type-c-linkage` does. So we can just re-enable references now
                        // so that the function type is exactly correct.
                        let ret_type =
                            format_cpp_type_inner(ret_type, ir, /* references_ok= */ true)?;
                        let param_types = param_types
                            .iter()
                            .map(|t| format_cpp_type_inner(t, ir, /* references_ok= */ true))
                            .collect::<Result<Vec<_>>>()?;
                        let attr = format_cc_call_conv_as_clang_attribute(abi)?;
                        // `type_identity_t` is used below to avoid having to
                        // emit spiral-like syntax where some syntax elements of
                        // an inner type (e.g. function type as below) can
                        // surround syntax elements of an outer type (e.g. a
                        // pointer type). Compare: `int (*foo)(int, int)` VS
                        // `type_identity_t<int(int, int)>* foo`.
                        Ok(quote! { crubit::type_identity_t<
                            #ret_type ( #( #param_types ),* ) #attr
                        >  })
                    }
                },
            },
        }
    } else {
        let item = ir.item_for_type(ty)?;
        let type_name = cpp_type_name_for_item(item, ir)?;
        Ok(quote! {#const_fragment #type_name})
    }
}

// Maps a Rust ABI [1] into a Clang attribute. See also
// `ConvertCcCallConvIntoRsApi` in importer.cc.
// [1]
// https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier
fn format_cc_call_conv_as_clang_attribute(rs_abi: &str) -> Result<TokenStream> {
    match rs_abi {
        "cdecl" => Ok(quote! {}),
        "fastcall" => Ok(quote! { __attribute__((fastcall)) }),
        "stdcall" => Ok(quote! { __attribute__((stdcall)) }),
        "thiscall" => Ok(quote! { __attribute__((thiscall)) }),
        "vectorcall" => Ok(quote! { __attribute__((vectorcall)) }),
        _ => bail!("Unsupported ABI: {}", rs_abi),
    }
}

fn cpp_type_name_for_item(item: &ir::Item, ir: &IR) -> Result<TokenStream> {
    /// Returns the namespace / class qualifiers necessary to access the item.
    ///
    /// For example, for `namespace x { struct Y { using X = int; }; }`, the prefix
    /// for `X` is `x::Y::`.
    fn cpp_qualified_path_prefix(item: &ir::Item, ir: &ir::IR) -> Result<TokenStream> {
        let Some(parent) = item.enclosing_item_id() else {
            return Ok(quote! {});
        };
        let parent: &ir::Item = ir.find_decl(parent)?;
        match parent {
            ir::Item::Namespace(_) => Ok(ir.namespace_qualifier(item).format_for_cc()?),
            ir::Item::Record(r) => {
                let name = cpp_tagless_type_name_for_record(r, ir)?;
                Ok(quote! {#name ::})
            }
            _ => bail!("Unexpected enclosing item: {item:?}"),
        }
    }

    match item {
        Item::IncompleteRecord(incomplete_record) => {
            let ident = expect_format_cc_type_name(incomplete_record.cc_name.identifier.as_ref());
            let namespace_qualifier = ir.namespace_qualifier(incomplete_record).format_for_cc()?;
            let tag_kind = incomplete_record.record_type;
            Ok(quote! { #tag_kind #namespace_qualifier #ident })
        }
        Item::Record(record) => cpp_type_name_for_record(record, ir),
        Item::Enum(enum_) => {
            let ident = expect_format_cc_type_name(&enum_.rs_name.identifier);
            let qualifier = cpp_qualified_path_prefix(item, ir)?;
            Ok(quote! { #qualifier #ident })
        }
        Item::TypeAlias(type_alias) => {
            let ident = expect_format_cc_type_name(&type_alias.cc_name.identifier);
            let qualifier = cpp_qualified_path_prefix(item, ir)?;
            Ok(quote! { #qualifier #ident })
        }
        Item::TypeMapOverride(type_map_override) => type_map_override
            .cc_name
            .parse::<TokenStream>()
            .map_err(|_| anyhow!("malformed type name: {:?}", type_map_override.cc_name)),
        _ => bail!("Item does not define a type: {:?}", item),
    }
}
