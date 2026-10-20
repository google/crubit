// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]

use arc_anyhow::{Context, Result};
use code_gen_utils::make_rs_ident;
use cpp_type_name::{cpp_type_name_for_record, format_cpp_type};
use database::code_snippet::{ApiSnippets, Thunk, ThunkImpl};
use database::rs_snippet::{CustomizeMethodsKind, RsTypeKind};
use database::BindingsGenerator;
use error_report::ensure;
use ir::*;
use proc_macro2::{Ident, TokenStream, TokenTree};
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

/// Generates the trait implementation and inherent forwarding methods of a record whose methods
/// are customized, rather than bound directly from its C++ methods.
///
/// Failing to generate the customized methods does not prevent the record from being bound.
/// Instead, they are replaced by a comment explaining why they could not be generated, and the
/// error is reported.
pub(crate) fn generate_customized_methods<'a>(
    db: &BindingsGenerator<'a>,
    api_snippets: &mut ApiSnippets,
    record: &Record,
    qualified_ident: &TokenStream,
    customize_methods: &CustomizeMethodsKind,
) -> Option<TokenStream> {
    // The crate of the target that defines the class template re-exports the support types used by
    // the generated methods, such as `Map` and `OccupiedError`. Spell it the same way as
    // `fully_qualify_type`, because its name may be mangled (e.g. by
    // `use_label_encoded_names_for_deps`).
    let defining_target =
        db.defining_target(record.id()).unwrap_or_else(|| record.owning_target().clone());
    let template_crate = match rs_imported_crate_name(&defining_target, db.ir()) {
        Some(crate_name) => quote! { ::#crate_name },
        None => quote! { crate },
    };

    let result = match customize_methods {
        CustomizeMethodsKind::AbslFlatHashMap { key_type, value_type } => {
            generate_absl_flat_hash_map(
                db,
                api_snippets,
                record,
                qualified_ident,
                &template_crate,
                customize_methods,
                key_type,
                value_type,
            )
        }
    };

    match result {
        Ok(trait_impl) => Some(trait_impl),
        Err(err) => {
            db.errors().report(&err);
            let message = format!(
                "error: `absl::flat_hash_map` methods could not be generated\n{}",
                format!("  {err:#}").replace('\n', "\n  ")
            );
            api_snippets
                .member_functions
                .entry(record.id())
                .or_default()
                .push(quote! { __NEWLINE__ __COMMENT__ #message });
            None
        }
    }
}

/// The Rust and C++ spellings of an element type that can be accessed in place.
struct ElementType {
    rs: TokenStream,
    cc: TokenStream,
}

/// Checks that elements of type `ty` can be accessed in place from Rust, and returns its Rust
/// and C++ spellings.
fn element_type(
    db: &BindingsGenerator<'_>,
    ty: &RsTypeKind,
    description: &str,
) -> Result<ElementType> {
    ensure!(
        !ty.pass_by_value_bridges(),
        "the {description} `{}` can't be accessed in place because it is a non-layout-compatible bridged type\nSee crubit.rs/types.",
        ty.display(db),
    );
    let rs = ty.to_token_stream(db);
    // TODO(b/478868411): Support element types with lifetimes, by adding them as parameters of
    // the record's type.
    ensure!(
        !mentions_non_static_lifetime(rs.clone()),
        "the {description} `{}` has a lifetime, which is not yet supported",
        ty.display(db),
    );
    let cc = format_cpp_type(ty, db).with_context(|| {
        format!("Failed to format the C++ spelling of the {description} `{}`", ty.display(db))
    })?;
    Ok(ElementType { rs, cc })
}

/// Checks that elements of type `ty` can be constructed from an rvalue of the same type in C++.
fn ensure_constructible_from_rvalue(
    db: &BindingsGenerator<'_>,
    ty: &RsTypeKind,
    description: &str,
) -> Result<()> {
    let constructible = match ty.unalias() {
        RsTypeKind::Record { record, .. } => {
            !record.is_abstract()
                && (record.move_constructor() != SpecialMemberFunc::Unavailable
                    || record.copy_constructor() != SpecialMemberFunc::Unavailable)
        }
        _ => ty.is_move_constructible(),
    };
    ensure!(
        constructible,
        "the {description} `{}` can't be constructed from an rvalue because it has no \
         public move or copy constructor",
        ty.display(db),
    );
    Ok(())
}

fn generate_absl_flat_hash_map(
    db: &BindingsGenerator<'_>,
    api_snippets: &mut ApiSnippets,
    record: &Record,
    qualified_ident: &TokenStream,
    template_crate: &TokenStream,
    customize_methods: &CustomizeMethodsKind,
    key_type: &RsTypeKind,
    value_type: &RsTypeKind,
) -> Result<TokenStream> {
    let cc_type = cpp_type_name_for_record(record, db)?;
    let ElementType { rs: key_rs, cc: key_cc } = element_type(db, key_type, "key type")?;
    let ElementType { rs: val_rs, cc: val_cc } = element_type(db, value_type, "value type")?;
    ensure_constructible_from_rvalue(db, key_type, "key type")?;
    ensure_constructible_from_rvalue(db, value_type, "value type")?;

    let mut add_thunk = |method_name: &str,
                         params: &[(&str, TokenStream, TokenStream)],
                         ret: Option<(TokenStream, TokenStream)>,
                         conversion_stmts: TokenStream,
                         return_stmt: TokenStream|
     -> Ident {
        let thunk_ident = make_thunk_name(record, customize_methods, method_name);
        let param_idents: Vec<Ident> =
            params.iter().map(|(name, _, _)| make_rs_ident(name)).collect();
        let rs_param_types: Vec<TokenStream> = params.iter().map(|(_, rs, _)| rs.clone()).collect();
        let cc_param_types: Vec<TokenStream> = params.iter().map(|(_, _, cc)| cc.clone()).collect();
        let (return_type_fragment, return_type_name) = match ret {
            Some((rs, cc)) => (Some(rs), cc),
            None => (None, quote! { void }),
        };
        api_snippets.thunks.push(Thunk::Function {
            mangled_name: None,
            thunk_ident: thunk_ident.clone(),
            generic_params: quote! {},
            param_idents: param_idents.clone(),
            param_types: rs_param_types,
            return_type_fragment,
        });
        api_snippets.cc_details.push(ThunkImpl::Function {
            return_type_name,
            thunk_ident: thunk_ident.clone(),
            param_types: cc_param_types,
            param_idents,
            conversion_stmts,
            return_stmt,
            needs_slot_header: false,
            is_always_inline: false,
        });
        thunk_ident
    };

    let this_ref = ("__this", quote! { &#qualified_ident }, quote! { const #cc_type* });
    let this_pin_mut =
        ("__this", quote! { ::core::pin::Pin<&mut #qualified_ident> }, quote! { #cc_type* });
    let key_ref = ("key", quote! { &#key_rs }, quote! { const #key_cc* });

    let len_thunk = add_thunk(
        "len",
        &[this_ref.clone()],
        Some((quote! { usize }, quote! { size_t })),
        quote! {},
        quote! { return __this->size() },
    );
    let capacity_thunk = add_thunk(
        "capacity",
        &[this_ref.clone()],
        Some((quote! { usize }, quote! { size_t })),
        quote! {},
        quote! { return __this->capacity() },
    );
    let clear_thunk =
        add_thunk("clear", &[this_pin_mut.clone()], None, quote! {}, quote! { __this->clear() });
    let reserve_thunk = add_thunk(
        "reserve",
        &[this_pin_mut.clone(), ("count", quote! { usize }, quote! { size_t })],
        None,
        quote! {},
        quote! { __this->reserve(count) },
    );
    let get_key_value_thunk = add_thunk(
        "get_key_value",
        &[
            this_ref,
            key_ref.clone(),
            ("result_key", quote! { *mut *const #key_rs }, quote! { const #key_cc** }),
            ("result_value", quote! { *mut *const #val_rs }, quote! { const #val_cc** }),
        ],
        Some((quote! { bool }, quote! { bool })),
        quote! {
            auto it = __this->find(*key);
            if (it == __this->end()) { return false; }
            *result_key = &it->first;
            *result_value = &it->second;
        },
        quote! { return true },
    );
    let get_mut_thunk = add_thunk(
        "get_mut",
        &[this_pin_mut.clone(), key_ref.clone()],
        Some((quote! { *mut #val_rs }, quote! { #val_cc* })),
        quote! { auto it = __this->find(*key); },
        quote! { return it == __this->end() ? nullptr : &it->second },
    );
    let insert_thunk = add_thunk(
        "insert",
        &[
            this_pin_mut.clone(),
            ("key", quote! { *mut #key_rs }, quote! { #key_cc* }),
            ("value", quote! { *mut #val_rs }, quote! { #val_cc* }),
            ("overwrite", quote! { bool }, quote! { bool }),
            ("result_key", quote! { *mut *const #key_rs }, quote! { const #key_cc** }),
            ("result_value", quote! { *mut *mut #val_rs }, quote! { #val_cc** }),
        ],
        Some((quote! { bool }, quote! { bool })),
        quote! {
            auto it = __this->try_emplace(std::move(*key), std::move(*value));
            if (!it.second && overwrite) {
                std::destroy_at(&it.first->second);
                crubit::construct_at(&it.first->second, std::move(*value));
            }
            *result_key = &it.first->first;
            *result_value = &it.first->second;
        },
        quote! { return it.second || overwrite },
    );
    let remove_thunk = add_thunk(
        "remove",
        &[this_pin_mut, key_ref],
        Some((quote! { bool }, quote! { bool })),
        quote! {},
        quote! { return __this->erase(*key) != 0 },
    );

    let val_mut = if value_type.is_unpin() {
        quote! { &'a mut #val_rs }
    } else {
        quote! { ::core::pin::Pin<&'a mut #val_rs> }
    };

    let inherent_methods = quote! {
        #[inline(always)]
        #[must_use]
        pub fn len(&self) -> usize {
            <Self as #template_crate::Map>::len(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            <Self as #template_crate::Map>::is_empty(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn capacity(&self) -> usize {
            <Self as #template_crate::Map>::capacity(self)
        }
        #[inline(always)]
        pub fn clear(self: ::core::pin::Pin<&mut Self>) {
            <Self as #template_crate::Map>::clear(self)
        }
        #[inline(always)]
        pub fn reserve(self: ::core::pin::Pin<&mut Self>, additional: usize) {
            <Self as #template_crate::Map>::reserve(self, additional)
        }
        #[inline(always)]
        #[must_use]
        pub fn contains_key(&self, key: &#key_rs) -> bool {
            <Self as #template_crate::Map>::contains_key(self, key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get<'a>(&'a self, key: &#key_rs) -> ::core::option::Option<&'a #val_rs> {
            <Self as #template_crate::Map>::get(self, key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_key_value<'a>(
            &'a self,
            key: &#key_rs,
        ) -> ::core::option::Option<(&'a #key_rs, &'a #val_rs)> {
            <Self as #template_crate::Map>::get_key_value(self, key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_mut<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: &#key_rs,
        ) -> ::core::option::Option<#val_mut> {
            <Self as #template_crate::Map>::get_mut(self, key)
        }
        #[inline(always)]
        pub fn insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![#key_rs],
            value: ::ctor::Ctor![#val_rs],
        ) -> (&'a #key_rs, #val_mut) {
            <Self as #template_crate::Map>::insert(self, key, value)
        }
        #[inline(always)]
        #[must_use]
        pub fn try_insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![#key_rs],
            value: ::ctor::Ctor![#val_rs],
        ) -> ::core::result::Result<
            (&'a #key_rs, #val_mut),
            #template_crate::OccupiedError<(&'a #key_rs, #val_mut)>,
        > {
            <Self as #template_crate::Map>::try_insert(self, key, value)
        }
        #[inline(always)]
        pub fn remove(self: ::core::pin::Pin<&mut Self>, key: &#key_rs) -> bool {
            <Self as #template_crate::Map>::remove(self, key)
        }
    };

    api_snippets.member_functions.entry(record.id()).or_default().push(inherent_methods);

    Ok(quote! {
        __NEWLINE__
        unsafe impl #template_crate::Map for #qualified_ident {
            type Key = #key_rs;
            type Value = #val_rs;
            type ValueMut<'a> = #val_mut where Self: 'a;

            #[doc(hidden)]
            #[inline(always)]
            unsafe fn raw_len(&self) -> usize {
                unsafe { crate::detail::#len_thunk(self) }
            }
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn raw_capacity(&self) -> usize {
                unsafe { crate::detail::#capacity_thunk(self) }
            }
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn raw_clear(self: ::core::pin::Pin<&mut Self>) {
                unsafe { crate::detail::#clear_thunk(self) }
            }
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn raw_reserve(self: ::core::pin::Pin<&mut Self>, count: usize) {
                unsafe { crate::detail::#reserve_thunk(self, count) }
            }
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn raw_get_key_value(
                &self,
                key: &Self::Key,
                result_key: *mut *const Self::Key,
                result_value: *mut *const Self::Value,
            ) -> bool {
                unsafe { crate::detail::#get_key_value_thunk(self, key, result_key, result_value) }
            }
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn raw_get_mut(
                self: ::core::pin::Pin<&mut Self>,
                key: &Self::Key,
            ) -> *mut Self::Value {
                unsafe { crate::detail::#get_mut_thunk(self, key) }
            }
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn raw_insert(
                self: ::core::pin::Pin<&mut Self>,
                key: *mut Self::Key,
                value: *mut Self::Value,
                overwrite: bool,
                result_key: *mut *const Self::Key,
                result_value: *mut *mut Self::Value,
            ) -> bool {
                unsafe { crate::detail::#insert_thunk(self, key, value, overwrite, result_key, result_value) }
            }
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn raw_remove(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> bool {
                unsafe { crate::detail::#remove_thunk(self, key) }
            }
        }
    })
}

/// Returns whether `tokens`, which spell a Rust type, mention any lifetime other than `'static`.
///
/// That includes named lifetimes such as `'a`, the anonymous lifetime `'_`, and the elided
/// lifetimes of references such as `&str`.
fn mentions_non_static_lifetime(tokens: TokenStream) -> bool {
    let mut tokens = tokens.into_iter().peekable();
    while let Some(token) = tokens.next() {
        let mentions_lifetime = match &token {
            TokenTree::Group(group) => mentions_non_static_lifetime(group.stream()),
            // A lifetime is spelled as a `'` followed by an identifier.
            TokenTree::Punct(punct) if punct.as_char() == '\'' => {
                !matches!(tokens.peek(), Some(TokenTree::Ident(ident)) if ident == "static")
            }
            // A reference has an elided lifetime unless its `&` is followed by a lifetime.
            TokenTree::Punct(punct) if punct.as_char() == '&' => {
                !matches!(tokens.peek(), Some(TokenTree::Punct(next)) if next.as_char() == '\'')
            }
            _ => false,
        };
        if mentions_lifetime {
            return true;
        }
    }
    false
}
