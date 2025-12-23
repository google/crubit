// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use code_gen_utils::expect_format_cc_type_name;
use database::rs_snippet::{RsTypeKind, RustPtrKind};
use error_report::{anyhow, bail};
use ir::{CcCallingConv, Item, PointerTypeKind, Record, IR};
use proc_macro2::TokenStream;
use quote::quote;
use std::rc::Rc;

pub fn format_cpp_type(ty: &RsTypeKind, ir: &IR) -> Result<TokenStream> {
    // Formatting *both* pointers *and* references as pointers, because:
    // - Pointers and references have the same representation in the ABI.
    // - Clang's `-Wreturn-type-c-linkage` warns when using references in C++
    //   function thunks declared as `extern "C"` (see b/238681766).
    format_cpp_type_inner(ty, ir, /* references_ok= */ false)
}

pub fn format_cpp_type_with_references(ty: &RsTypeKind, ir: &IR) -> Result<TokenStream> {
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

pub fn format_cpp_type_inner(
    rs_type_kind: &RsTypeKind,
    ir: &IR,
    references_ok: bool,
) -> Result<TokenStream> {
    match rs_type_kind {
        RsTypeKind::Error { symbol, .. } => symbol.parse().map_err(|_| {
            anyhow!("malformed type name, this is a crubit implementation bug: {:?}", symbol)
        }),
        RsTypeKind::Pointer { pointee, kind, mutability } => {
            let nested_type = format_cpp_type_inner(pointee, ir, references_ok)?;
            let const_fragment = mutability.is_const().then(|| quote! { const });
            match kind {
                RustPtrKind::CcPtr(kind) => {
                    let ptr = match (references_ok, kind) {
                        (true, PointerTypeKind::LValueRef) => quote! {&},
                        (true, PointerTypeKind::RValueRef) => quote! {&&},
                        _ => quote! {*},
                    };
                    Ok(quote! { #nested_type #const_fragment #ptr })
                }
                RustPtrKind::Slice => {
                    Ok(quote! { ::rs_std::SliceRef<#const_fragment #nested_type> })
                }
            }
        }
        RsTypeKind::Reference { referent, mutability, .. } => {
            let const_fragment = mutability.is_const().then(|| quote! { const });
            let nested_type = format_cpp_type_inner(referent, ir, references_ok)?;
            let pointer_kind = if !references_ok {
                quote! { * }
            } else {
                quote! { & }
            };
            Ok(quote! { #nested_type #const_fragment #pointer_kind })
        }
        RsTypeKind::RvalueReference { referent, mutability, .. } => {
            let const_fragment = mutability.is_const().then(|| quote! { const });
            let nested_type = format_cpp_type_inner(referent, ir, references_ok)?;
            let pointer_kind = if !references_ok {
                quote! { * }
            } else {
                quote! { && }
            };
            Ok(quote! { #nested_type #const_fragment #pointer_kind })
        }
        RsTypeKind::FuncPtr { option, cc_calling_conv, return_type, param_types } => {
            // Function pointer types don't ignore references, but luckily,
            // `-Wreturn-type-c-linkage` does. So we can just re-enable references now
            // so that the function type is exactly correct.
            let ret_type = format_cpp_type_inner(return_type, ir, /* references_ok= */ true)?;
            let param_types = param_types
                .iter()
                .map(|t| format_cpp_type_inner(t, ir, /* references_ok= */ true))
                .collect::<Result<Vec<_>>>()?;
            let attr = match cc_calling_conv {
                CcCallingConv::C => quote! {},
                other => quote! { __attribute__((#other)) },
            };
            let ptr = if !*option && references_ok {
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
                > #ptr
            })
        }
        RsTypeKind::IncompleteRecord { incomplete_record, .. } => tagless_cpp_type_name_for_item(
            &Item::IncompleteRecord(Rc::clone(incomplete_record)),
            ir,
        ),
        RsTypeKind::Record { record, .. } => cpp_type_name_for_record(record, ir),
        RsTypeKind::Enum { enum_, .. } => {
            tagless_cpp_type_name_for_item(&Item::Enum(Rc::clone(enum_)), ir)
        }
        RsTypeKind::TypeAlias { type_alias, .. } => {
            tagless_cpp_type_name_for_item(&Item::TypeAlias(Rc::clone(type_alias)), ir)
        }
        RsTypeKind::Primitive(primitive) => Ok(quote! { #primitive }),
        RsTypeKind::BridgeType { original_type, .. } => cpp_type_name_for_record(original_type, ir),
        RsTypeKind::ExistingRustType(existing_rust_type) => tagless_cpp_type_name_for_item(
            &Item::ExistingRustType(Rc::clone(existing_rust_type)),
            ir,
        ),
        RsTypeKind::C9Co { original_type, .. } => cpp_type_name_for_record(original_type, ir),
    }
}

/// Returns the fully qualified name for an item (not including type tags).
///
/// For example, for `namespace x { struct Y { using X = int; }; }`, the name
/// for `X` is `x::Y::X`.
pub fn tagless_cpp_type_name_for_item(item: &ir::Item, ir: &IR) -> Result<TokenStream> {
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
            Ok(quote! { #namespace_qualifier #ident })
        }
        Item::Record(record) => cpp_tagless_type_name_for_record(record, ir),
        Item::Enum(enum_) => {
            let ident = expect_format_cc_type_name(&enum_.rs_name.identifier);
            let namespace_qualifier = cpp_qualified_path_prefix(item, ir)?;
            Ok(quote! { #namespace_qualifier #ident })
        }
        Item::TypeAlias(type_alias) => {
            let ident = expect_format_cc_type_name(&type_alias.cc_name.identifier);
            let namespace_qualifier = cpp_qualified_path_prefix(item, ir)?;
            Ok(quote! { #namespace_qualifier #ident })
        }
        Item::ExistingRustType(existing_rust_type) => existing_rust_type
            .cc_name
            .parse::<TokenStream>()
            .map_err(|_| anyhow!("malformed type name: {:?}", existing_rust_type.cc_name)),
        _ => bail!("Item does not define a type: {:?}", item),
    }
}
