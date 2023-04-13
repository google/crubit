// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

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
    // Note that this has to be a procedural macro because it is fundamentally
    // non-hygienic in how it attempts to test whether a type exists: it tries to
    // introduce shadowing from inside the macro, which hygiene normally
    // prevents.
    let mut path = syn::parse_macro_input!(path as syn::Path);
    let name = match path.segments.pop() {
        None => {
            return syn::Error::new(
                path.span(),
                "Path must have at least two elements (e.g. A::B)",
            )
            .into_compile_error()
            .into();
        }
        Some(Pair::Punctuated(..)) => {
            return syn::Error::new(path.span(), "Path must not end in `::`")
                .into_compile_error()
                .into();
        }
        Some(Pair::End(name)) => name,
    };

    if path.segments.is_empty() {
        // TODO(jeanpierreda): If it were desirable, we could allow `type_exists!(X)`, by
        // creating a `mod` to hold it for shadowing purposes.
        return syn::Error::new(path.span(), "Path must have at least two elements (e.g. A::B)")
            .into_compile_error()
            .into();
    }

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
