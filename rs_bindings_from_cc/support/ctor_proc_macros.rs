// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned, ToTokens as _};
use syn::spanned::Spanned as _;

// TODO(jeanpierreda): derive constructors and assignment for copy and move.

#[proc_macro_derive(CtorFrom_Default)]
pub fn derive_default(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_name = input.ident;
    let struct_ctor_name = Ident::new(
        &format!("_ctor_derive_{}_CtorType_Default", struct_name),
        Span::call_site(),
    );
    let filled_fields: Vec<_> = match &input.data {
        syn::Data::Struct(data) => {
            if let syn::Fields::Unit = data.fields {
                // The generated code will not work as is for `struct Foo;` because we would
                // create the literal `Foo {}`, which is not valid.
                // TODO(jeanpierreda): special-case this.
                // Note: this is not important right now, because pin_project doesn't support
                // fieldless structs to begin with, so either way it fails to compile.
                // We could generate the code for Foo {} and have it fail inside pin_project,
                // but if pin_project ever added support for empty structs, this
                // would unexpectedly start working, but with unexpected
                // results.
                todo!();
            }
            data.fields
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let field_i = syn::Index::from(i);
                    let field_name = match &field.ident {
                        None => quote! {#field_i},
                        Some(name) => quote! {#name},
                    };

                    let field_type = &field.ty;
                    quote_spanned! {field.span() =>
                        #field_name: <#field_type as ::ctor::CtorNew<()>>::ctor_new(())
                    }
                })
                .collect()
        }
        _ => unimplemented!(),
    };

    let expanded = quote! {
        struct #struct_ctor_name();

        impl ::ctor::Ctor for #struct_ctor_name {
            type Output = #struct_name;
            unsafe fn ctor(self, dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self::Output>>) {
                ::ctor::ctor!(
                    #struct_name { #(#filled_fields),* }
                ).ctor(dest)
            }
        }

        impl !::std::marker::Unpin for #struct_ctor_name {}

        impl ::ctor::CtorNew<()> for #struct_name {
            type CtorType = #struct_ctor_name;

            fn ctor_new(_args: ()) -> #struct_ctor_name { #struct_ctor_name() }
        }
    };
    TokenStream::from(expanded)
}

/// #[recursively_pinned] pins every field using #[pin_project]. All macro
/// arguments are forwarded.
///
/// To use this macro, you must depend directly on pin_project. (This is due to
/// limitations of procedural macros, which do not have a `$crate` equivalent.)
///
/// Example:
///
/// ```
/// #[recursively_pinned]
/// struct S {
///   field: i32,
/// }
/// ```
///
/// This is equivalent to using pin_project directly, but pinning every field,
/// as so:
///
/// ```
/// #[pin_project]
/// struct S {
///   #[pin]
///   field: i32,
/// }
/// ```
///
/// Note that recursively_pinned doesn't mark a struct as `!Unpin` -- to do
/// that, use a `!Unpin` member such as `PhantomPinned` member (initialized via
/// e.g. `PhantomPinnedCtor`), or pass in `UnsafeUnpin`.
#[proc_macro_attribute]
pub fn recursively_pinned(args: TokenStream, item: TokenStream) -> TokenStream {
    let args: proc_macro2::TokenStream = args.into();
    let mut input = syn::parse_macro_input!(item as syn::DeriveInput);

    input.attrs.insert(0, syn::parse_quote!(#[::pin_project::pin_project(#args)]));
    let pin: syn::Attribute = syn::parse_quote!(#[pin]);
    match &mut input.data {
        syn::Data::Struct(data) => {
            for field in &mut data.fields {
                field.attrs.push(pin.clone());
            }
        }
        syn::Data::Enum(e) => {
            for variant in &mut e.variants {
                for field in &mut variant.fields {
                    field.attrs.push(pin.clone());
                }
            }
        }
        syn::Data::Union(_) => unimplemented!(),
    }

    let name = input.ident.clone();
    let input = input.into_token_stream();

    let expanded = quote! {
        #input

        unsafe impl ::ctor::RecursivelyPinned for #name {}
    };

    TokenStream::from(expanded)
}
