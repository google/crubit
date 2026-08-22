// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use arc_anyhow::{Context, Result};
use code_gen_utils::{format_nonportable_cc_type_name, make_rs_ident};
use cpp_type_name::format_cpp_type;
use database::code_snippet::{ApiSnippets, DocCommentAttr, Thunk, ThunkImpl};
use database::rs_snippet::CustomizeMethodsKind;
use database::BindingsGenerator;
use ir::*;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

fn make_thunk_name(
    record: &Record,
    customize_methods: &CustomizeMethodsKind,
    method: &str,
) -> Ident {
    make_rs_ident(&format!(
        "__crubit_{kind}_{method}_{mangled_cc_name}_{owning_target}",
        kind = match customize_methods {
            CustomizeMethodsKind::AbslFlatHashMap { .. } => "flat_hash_map",
        },
        mangled_cc_name = record.mangled_cc_name(),
        owning_target = Record::owning_target(record).convert_to_cc_identifier(),
    ))
}

pub(crate) fn generate_customized_methods<'a>(
    db: &BindingsGenerator<'a>,
    api_snippets: &mut ApiSnippets,
    record: &Record,
    qualified_ident: &TokenStream,
    customize_methods: &CustomizeMethodsKind<'a>,
) -> Result<()> {
    match customize_methods {
        CustomizeMethodsKind::AbslFlatHashMap { key_type, value_type } => {
            let member_functions = api_snippets.member_functions.entry(record.id()).or_default();
            let cc_type = format_nonportable_cc_type_name(record.cc_name().as_str())
                .expect("Record has invalid type name");
            let key_cc_type = format_cpp_type(key_type, db).with_context(|| format!(
                    "Failed to format C++ type name for absl::flat_hash_map key parameter in {cc_type}",
                ))?;
            let value_cc_type = format_cpp_type(value_type, db).with_context(|| format!(
                    "Failed to format C++ type name for absl::flat_hash_map value parameter in {cc_type}",
                ))?;
            let key_rs_type = key_type.to_token_stream(db);
            let value_rs_type = value_type.to_token_stream(db);
            let value_mut_rs_type = if value_type.is_unpin() {
                quote! { &'a mut #value_rs_type }
            } else {
                quote! { ::core::pin::Pin<&'a mut #value_rs_type> }
            };
            let pair_mut_rs_type = quote! { (&'a #key_rs_type, #value_mut_rs_type) };

            // `len` method, which just calls `size`.
            let thunk_name = make_thunk_name(record, customize_methods, "len");
            let doc_comment = DocCommentAttr(
                " \
 Returns the number of elements currently within the `flat_hash_map`."
                    .into(),
            );
            member_functions.push(quote! {
                #doc_comment
                pub fn len(&self) -> usize {
                    unsafe { crate::detail::#thunk_name(self) }
                }
            });
            api_snippets.thunks.push(Thunk::Function {
                mangled_name: None,
                thunk_ident: thunk_name.clone(),
                generic_params: quote! { <'a> },
                param_idents: vec![make_rs_ident("__this")],
                param_types: vec![quote! { &'a #qualified_ident }],
                return_type_fragment: Some(quote! { usize }),
            });
            api_snippets.cc_details.push(ThunkImpl::Function {
                return_type_name: quote! { size_t },
                thunk_ident: thunk_name,
                param_types: vec![quote! { const #cc_type* }],
                param_idents: vec![make_rs_ident("__this")],
                conversion_stmts: quote! {},
                return_stmt: quote! { return __this->size() },
            });

            // `capacity` method, which just calls `capacity`.
            let thunk_name = make_thunk_name(record, customize_methods, "capacity");
            let doc_comment = DocCommentAttr(
                " \
 Returns the number of element slots (assigned, deleted, and empty) available
 within the `flat_hash_map`."
                    .into(),
            );
            member_functions.push(quote! {
                #doc_comment
                pub fn capacity(&self) -> usize {
                    unsafe { crate::detail::#thunk_name(self) }
                }
            });
            api_snippets.thunks.push(Thunk::Function {
                mangled_name: None,
                thunk_ident: thunk_name.clone(),
                generic_params: quote! { <'a> },
                param_idents: vec![make_rs_ident("__this")],
                param_types: vec![quote! { &'a #qualified_ident }],
                return_type_fragment: Some(quote! { usize }),
            });
            api_snippets.cc_details.push(ThunkImpl::Function {
                return_type_name: quote! { size_t },
                thunk_ident: thunk_name,
                param_types: vec![quote! { const #cc_type* }],
                param_idents: vec![make_rs_ident("__this")],
                conversion_stmts: quote! {},
                return_stmt: quote! { return __this->capacity() },
            });

            // `is_empty` method, which just calls `empty`.
            let doc_comment = DocCommentAttr(
                " \
 Returns whether or not the `flat_hash_map` is empty."
                    .into(),
            );

            let thunk_name = make_thunk_name(record, customize_methods, "is_empty");
            member_functions.push(quote! {
                #doc_comment
                pub fn is_empty(&self) -> bool {
                    unsafe { crate::detail::#thunk_name(self) }
                }
            });
            api_snippets.thunks.push(Thunk::Function {
                mangled_name: None,
                thunk_ident: thunk_name.clone(),
                generic_params: quote! { <'a> },
                param_idents: vec![make_rs_ident("__this")],
                param_types: vec![quote! { &'a #qualified_ident }],
                return_type_fragment: Some(quote! { bool }),
            });
            api_snippets.cc_details.push(ThunkImpl::Function {
                return_type_name: quote! { bool },
                thunk_ident: thunk_name,
                param_types: vec![quote! { const #cc_type* }],
                param_idents: vec![make_rs_ident("__this")],
                conversion_stmts: quote! {},
                return_stmt: quote! { return __this->empty() },
            });

            // `try_insert` and `try_insert_mov` methods, which accept a key/value pair either by
            // value or by `RvalueReference` and share a thunk that moves them into a `try_emplace`
            // call.
            let thunk_name = make_thunk_name(record, customize_methods, "try_insert");
            let element_value_expr = if value_type.is_unpin() {
                quote! { &mut *result_value.assume_init() }
            } else {
                quote! { ::core::pin::Pin::new_unchecked(&mut *result_value.assume_init()) }
            };
            if key_type.is_unpin() && value_type.is_unpin() {
                let try_insert_doc_comment = DocCommentAttr(
                    " \
 Inserts an element with the specified key and value by move- (or copy-)
 constructing them into the `flat_hash_map`, provided that no element with the
 given key already exists, returning references to the newly inserted element.

 If an element with the given key already exists, returns references to the
 existing element along with the provided key and value."
                        .into(),
                );
                member_functions.push(quote! {
                    #try_insert_doc_comment
                    pub fn try_insert<'a>(
                        self: ::core::pin::Pin<&'a mut Self>,
                        mut key: #key_rs_type,
                        mut value: #value_rs_type,
                    ) -> ::core::result::Result<
                        #pair_mut_rs_type,
                        ::absl_container::OccupiedError<'a, #key_rs_type, #value_rs_type>,
                    > {
                        let mut result_key: ::core::mem::MaybeUninit<*const #key_rs_type> =
                            ::core::mem::MaybeUninit::uninit();
                        let mut result_value: ::core::mem::MaybeUninit<*mut #value_rs_type> =
                            ::core::mem::MaybeUninit::uninit();
                        let was_inserted = unsafe {
                            crate::detail::#thunk_name(
                                self,
                                &raw mut key,
                                &raw mut value,
                                result_key.as_mut_ptr(),
                                result_value.as_mut_ptr(),
                            )
                        };
                        let element = unsafe { (&*result_key.assume_init(), #element_value_expr) };
                        if was_inserted {
                            ::core::result::Result::Ok(element)
                        } else {
                            ::core::result::Result::Err(::absl_container::OccupiedError{
                                element,
                                key,
                                value,
                            })
                        }
                    }
                });
            }
            let try_insert_mov_doc_comment = DocCommentAttr(
                " \
 Inserts an element with the specified key and value by move- (or copy-)
 constructing them into the `flat_hash_map`, provided that no element with the
 given key already exists, returning references to the newly inserted element.

 If an element with the given key already exists, returns references to the
 existing element; the provided key and value references are not moved from."
                    .into(),
            );
            member_functions.push(quote! {
                #try_insert_mov_doc_comment
                pub fn try_insert_mov<'a>(
                    self: ::core::pin::Pin<&'a mut Self>,
                    mut key: ::ctor::RvalueReference<#key_rs_type>,
                    mut value: ::ctor::RvalueReference<#value_rs_type>,
                ) -> Result<
                    #pair_mut_rs_type,
                    ::absl_container::OccupiedMovError<#pair_mut_rs_type>,
                > {
                    let mut result_key: ::core::mem::MaybeUninit<*const #key_rs_type> =
                        ::core::mem::MaybeUninit::uninit();
                    let mut result_value: ::core::mem::MaybeUninit<*mut #value_rs_type> =
                        ::core::mem::MaybeUninit::uninit();
                    let was_inserted = unsafe {
                        crate::detail::#thunk_name(
                            self,
                            ::core::pin::Pin::into_inner_unchecked(key.as_mut()) as _,
                            ::core::pin::Pin::into_inner_unchecked(value.as_mut()) as _,
                            result_key.as_mut_ptr(),
                            result_value.as_mut_ptr(),
                        )
                    };
                    let element = unsafe { (&*result_key.assume_init(), #element_value_expr) };
                    if was_inserted {
                        ::core::result::Result::Ok(element)
                    } else {
                        ::core::result::Result::Err(::absl_container::OccupiedMovError{ element })
                    }
                }
            });
            api_snippets.thunks.push(Thunk::Function {
                mangled_name: None,
                thunk_ident: thunk_name.clone(),
                generic_params: quote! {},
                param_idents: vec![
                    make_rs_ident("__this"),
                    make_rs_ident("key"),
                    make_rs_ident("value"),
                    make_rs_ident("result_key"),
                    make_rs_ident("result_value"),
                ],
                param_types: vec![
                    quote! { ::core::pin::Pin<&mut #qualified_ident> },
                    quote! { *mut #key_rs_type },
                    quote! { *mut #value_rs_type },
                    quote! { *mut *const #key_rs_type },
                    quote! { *mut *mut #value_rs_type },
                ],
                return_type_fragment: Some(quote! { bool }),
            });
            api_snippets.cc_details.push(ThunkImpl::Function {
                return_type_name: quote! { bool },
                thunk_ident: thunk_name,
                param_types: vec![
                    quote! { #cc_type* },
                    quote! { #key_cc_type* },
                    quote! { #value_cc_type* },
                    quote! { #key_cc_type const** },
                    quote! { #value_cc_type** },
                ],
                param_idents: vec![
                    make_rs_ident("__this"),
                    make_rs_ident("key"),
                    make_rs_ident("value"),
                    make_rs_ident("result_key"),
                    make_rs_ident("result_value"),
                ],
                conversion_stmts: quote! {
                    auto it = __this->try_emplace(std::move(*key), std::move(*value));
                    *result_key = &it.first->first;
                    *result_value = &it.first->second;
                },
                return_stmt: quote! { return it.second },
            });
        }
    }
    Ok(())
}
