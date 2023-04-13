// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Macros for testing, at runtime, whether an item exists at runtime.
//! Note that these must be procedural macros because they are fundamentally
//! non-hygienic in how they attempts to test whether an item exists: each tries
//! to introduce shadowing from inside the macro, which hygiene normally
//! prevents.

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Pair;
use syn::spanned::Spanned as _;

/// Returns whether the named type exists.
///
/// For example, `type_exists!(std::num::NonZeroU8) == true`, but at the time
/// of writing, `type_exists!(std::num::ThisDoesntExist) == false`.
///
/// If the provided name does not refer to a type, it returns `false`.
///
/// This is a handy macro to have at hand for integration tests of the bindings
/// generator, as it allows one to assert at _runtime_ whether bindings were
/// generated for a type, instead of at compile-time.
///
/// Known limitations:
///
/// * Does not support names that are already in scope (e.g. `i32`).
/// * Does not support generics (e.g. `std::vec::Vec<i32>`).
/// * Does not support associated types (e.g. `Option<i32>::Item`).
#[proc_macro]
pub fn type_exists(path: TokenStream) -> TokenStream {
    let (path, name) = match extract_last(syn::parse_macro_input!(path as syn::Path)) {
        Ok((path, name)) => (path, name),
        Err(e) => return e.into_compile_error().into(),
    };

    TokenStream::from(quote! {
        {
            #[allow(non_camel_case_types)]
            struct #name;
            let fallback_id = ::core::any::TypeId::of::<#name>();

            // introduce a new scope, so that we can shadow `#name`.
            let type_id = {
                #[allow(unused_imports)]
                use #path *;
                ::core::any::TypeId::of::<#name>()
            };
            fallback_id != type_id
        }
    })
}

/// Returns whether the named static, constant, or function exists.
///
/// For example, `value_exists!(std::f32::consts::E) == true`, but at the time
/// of writing, `value_exists!(std::f32::consts::DOES_NOT_EXIST) == false`.
///
/// If the provided name does not refer to a runtime value
/// (such as a non-templated function, constant, or static), it returns `false`.
///
/// This is a handy macro to have at hand for integration tests of the bindings
/// generator, as it allows one to assert at _runtime_ whether bindings were
/// generated for a function or constant, instead of at compile-time.
///
/// Known limitations:
///
/// * Does not support names that are already in scope (e.g. `i32`).
/// * Does not support generics (e.g. `std::vec::Vec::<i32>`).
/// * Does not support methods or associated constants (e.g.
///   `Option<i32>::default`).
#[proc_macro]
pub fn value_exists(path: TokenStream) -> TokenStream {
    let (path, name) = match extract_last(syn::parse_macro_input!(path as syn::Path)) {
        Ok((path, name)) => (path, name),
        Err(e) => return e.into_compile_error().into(),
    };

    TokenStream::from(quote! {
        {
            /// A new type, introduced to differentiate types.
            /// If we made it a non-ZST, we could also differentiate based on address.
            /// Technically, that'd have a slight performance cost, which this avoids. ;)))
            #[allow(non_camel_case_types)]
            struct #name {}
            #[allow(non_upper_case_globals)]
            static #name : #name = #name {};
            // Alternatively: let fallback_address = &#name as *const _ as usize;
            let fallback_id = ::std::any::Any::type_id(&#name);

            // introduce a new scope, so that we can shadow `#name`.
            let type_id = {
                #[allow(unused_imports)]
                use #path *;
                // Alternatively: &#name as *const _ as usize
                ::std::any::Any::type_id(&#name)
            };
            fallback_id != type_id
        }
    })
}

fn extract_last(mut path: syn::Path) -> Result<(syn::Path, syn::PathSegment), syn::Error> {
    let name = match path.segments.pop() {
        None => {
            return Err(syn::Error::new(
                path.span(),
                "Path must have at least two elements (e.g. A::B)",
            ));
        }
        Some(Pair::Punctuated(..)) => {
            return Err(syn::Error::new(path.span(), "Path must not end in `::`"));
        }
        Some(Pair::End(name)) => name,
    };

    if path.segments.is_empty() {
        // TODO(jeanpierreda): If it were desirable, we could allow `type_exists!(X)` etc, by
        // creating a `mod` to hold it for shadowing purposes.
        return Err(syn::Error::new(
            path.span(),
            "Path must have at least two elements (e.g. A::B)",
        ));
    }
    Ok((path, name))
}
