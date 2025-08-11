// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use code_gen_utils::expect_format_cc_type_name;
use error_report::{anyhow, bail};
use ir::{CcCallingConv, CcType, CcTypeVariant, Item, PointerTypeKind, Record, IR};
use proc_macro2::TokenStream;
use quote::quote;

pub fn format_cpp_type(ty: &CcType, ir: &IR) -> Result<TokenStream> {
    // Formatting *both* pointers *and* references as pointers, because:
    // - Pointers and references have the same representation in the ABI.
    // - Clang's `-Wreturn-type-c-linkage` warns when using references in C++
    //   function thunks declared as `extern "C"` (see b/238681766).
    format_cpp_type_inner(ty, ir, /* references_ok= */ false)
}

pub fn format_cpp_type_with_references(ty: &CcType, ir: &IR) -> Result<TokenStream> {
    format_cpp_type_inner(ty, ir, /* references_ok= */ true)
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
    match &ty.variant {
        CcTypeVariant::Primitive(primitive) => Ok(quote! { #primitive #const_fragment }),
        CcTypeVariant::Pointer(pointer) => {
            let nested_type = format_cpp_type_inner(&pointer.pointee_type, ir, references_ok)?;
            let ptr = match (references_ok, pointer.kind) {
                (true, PointerTypeKind::LValueRef) => quote! {&},
                (true, PointerTypeKind::RValueRef) => quote! {&&},
                _ => quote! {*},
            };
            Ok(quote! {#nested_type #ptr #const_fragment})
        }
        CcTypeVariant::FuncPointer { non_null, call_conv, param_and_return_types } => {
            let (ret_type, param_types) = param_and_return_types.split_last().expect(
                "funcValue should always have a return type, this is a crubit implementation bug",
            );

            // Function pointer types don't ignore references, but luckily,
            // `-Wreturn-type-c-linkage` does. So we can just re-enable references now
            // so that the function type is exactly correct.
            let ret_type = format_cpp_type_inner(ret_type, ir, /* references_ok= */ true)?;
            let param_types = param_types
                .iter()
                .map(|t| format_cpp_type_inner(t, ir, /* references_ok= */ true))
                .collect::<Result<Vec<_>>>()?;
            let attr = match call_conv {
                CcCallingConv::C => quote! {},
                other => quote! { __attribute__((#other)) },
            };
            let ptr = if *non_null && references_ok {
                quote! {&}
            } else {
                quote! {*}
            };
            // `type_identity_t` is used below to avoid having to
            // emit spiral-like syntax where some syntax elements of
            // an inner type (e.g. function type as below) can
            // surround syntax elements of an outer type (e.g. a
            // pointer type). Compare: `int (*foo)(int, int)` VS
            // `type_identity_t<int(int, int)>* foo`.
            Ok(quote! {
                crubit::type_identity_t<
                    #ret_type ( #( #param_types ),* ) #attr
                > #ptr #const_fragment
            })
        }
        CcTypeVariant::Record(id) => {
            let item = ir.find_untyped_decl(*id);
            let type_name = cpp_type_name_for_item(item, ir)?;
            Ok(quote! {#const_fragment #type_name})
        }
    }
}

/// Returns the fully-qualified name for an item, not including the type tag.
pub fn tagless_cpp_type_name_for_item(item: &ir::Item, ir: &IR) -> Result<TokenStream> {
    if let ir::Item::Record(record) = item {
        cpp_tagless_type_name_for_record(record, ir)
    } else {
        cpp_type_name_for_item(item, ir)
    }
}

/// Returns the fully qualified name for an item.
///
/// For example, for `namespace x { struct Y { using X = int; }; }`, the name
/// for `X` is `x::Y::X`.
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
