// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use arc_anyhow::{Context, Result};
use code_gen_utils::{format_nonportable_cc_type_name, make_rs_ident};
use cpp_type_name::format_cpp_type;
use database::code_snippet::{ApiSnippets, Thunk, ThunkImpl};
use database::rs_snippet::{CustomizeMethodsKind, RsTypeKind};
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
            generate_absl_flat_hash_map_methods(
                db,
                api_snippets,
                record,
                qualified_ident,
                customize_methods,
                key_type,
                value_type,
            )
        }
    }
}

fn generate_absl_flat_hash_map_methods<'a>(
    db: &BindingsGenerator<'a>,
    api_snippets: &mut ApiSnippets,
    record: &Record,
    qualified_ident: &TokenStream,
    customize_methods: &CustomizeMethodsKind<'a>,
    key_type: &RsTypeKind<'a>,
    value_type: &RsTypeKind<'a>,
) -> Result<()> {
    let member_functions = api_snippets.member_functions.entry(record.id()).or_default();
    let cc_type = format_nonportable_cc_type_name(record.cc_name().as_str())
        .expect("Record has invalid type name");
    // TODO(mvanbem): We need to exclude bridged types here.
    let key_cc_type =
        format_cpp_type(key_type, db).with_context(|| {
            format!(
                "Failed to format C++ type name for absl::flat_hash_map key parameter in {cc_type}",
            )
        })?;
    let value_cc_type = format_cpp_type(value_type, db).with_context(|| {
        format!(
            "Failed to format C++ type name for absl::flat_hash_map value parameter in {cc_type}",
        )
    })?;
    let key_rs_type = key_type.to_token_stream(db);
    let value_rs_type = value_type.to_token_stream(db);
    let value_mut_rs_type = if value_type.is_unpin() {
        quote! { &'a mut #value_rs_type }
    } else {
        quote! { ::core::pin::Pin<&'a mut #value_rs_type> }
    };

    // `len` and `is_empty` methods, which call `size`.
    let thunk_name = make_thunk_name(record, customize_methods, "len");
    member_functions.push(quote! {
        #[doc = " Returns the number of elements currently within the `flat_hash_map`."]
        #[must_use]
        pub fn len(&self) -> usize {
            unsafe { crate::detail::#thunk_name(self) }
        }

        #[doc = " Returns whether or not the `flat_hash_map` is empty."]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
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
    member_functions.push(quote! {
        #[doc = " \
 Returns the number of element slots (assigned, deleted, and empty) available
 within the `flat_hash_map`."
        ]
        #[must_use]
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

    // `try_insert` method, which accepts a key/value pair either by value or by `RvalueReference`
    // and moves them into a `try_emplace` call.
    let thunk_name = make_thunk_name(record, customize_methods, "try_insert");
    let element_mut_rs_type = quote! { (&'a #key_rs_type, #value_mut_rs_type) };
    let element_value_expr = if value_type.is_unpin() {
        quote! { &mut *result_value }
    } else {
        quote! { ::core::pin::Pin::new_unchecked(&mut *result_value) }
    };
    member_functions.push(quote! {
        #[doc = " \
 Inserts an element with the specified key and value by move-constructing them
 into the `flat_hash_map` (copy constructing if there is no move constructor),
 provided that no element with the given key already exists, returning
 references to the newly inserted element.

 If an element with the given key already exists, returns references to the
 existing element along with the provided key and value. In this case if either
 the key or the value is an [`RvalueReference`](::ctor::RvalueReference), it
 was not moved from.

 The key and value are accepted as [`AsRvalue`](::ctor::AsRvalue), which is
 implemented for any `T: Unpin` and for
 [`RvalueReference<T>`](::ctor::RvalueReference).

 Usage passing `Unpin` key and value types by value:

 ```ignore
 use ctor::{emplace, CtorNew};
 let mut map = emplace!(Self::ctor_new());
 map.as_mut().try_insert(\"my_key\", 123).unwrap();
 ```

 Usage passing potentially-`!Unpin` key and value types by rvalue reference:

 ```ignore
 use ctor::{emplace, mov, CtorNew};
 let mut map = emplace!(Self::ctor_new());
 let mut key: Pin<&mut KeyType> = get_key();
 let mut value: Pin<&mut ValueType> = get_value();
 map.as_mut().try_insert(mov!(key), mov!(value)).unwrap();
 ```"
        ]
        pub fn try_insert<'a, K, V>(
            self: ::core::pin::Pin<&'a mut Self>,
            mut key: K,
            mut value: V,
        ) -> Result<
            #element_mut_rs_type,
            ::flat_hash_map::OccupiedError<#element_mut_rs_type, K, V>,
        >
        where
            K: ::ctor::AsRvalue<#key_rs_type>,
            V: ::ctor::AsRvalue<#value_rs_type>,
        {
            let mut result_key: *const #key_rs_type = ::core::ptr::null();
            let mut result_value: *mut #value_rs_type = ::core::ptr::null_mut();
            let was_inserted = unsafe {
                crate::detail::#thunk_name(
                    self,
                    ::ctor::RvalueReference::into_mut_ptr(
                        ::ctor::AsRvalue::as_rvalue(::core::pin::Pin::new(&mut key)),
                    ),
                    ::ctor::RvalueReference::into_mut_ptr(
                        ::ctor::AsRvalue::as_rvalue(::core::pin::Pin::new(&mut value)),
                    ),
                    &raw mut result_key,
                    &raw mut result_value,
                )
            };
            let element = unsafe { (&*result_key, #element_value_expr) };
            if was_inserted {
                ::core::result::Result::Ok(element)
            } else {
                ::core::result::Result::Err(::flat_hash_map::OccupiedError {
                    element,
                    key,
                    value,
                })
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

    Ok(())
}
