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
    let struct_ctor_name =
        Ident::new(&format!("_ctor_derive_{}_CtorType_Default", struct_name), Span::call_site());
    let filled_fields: Vec<_> = match &input.data {
        syn::Data::Struct(data) => {
            if let syn::Fields::Unit = data.fields {
                // The generated code will not work as is for `struct Foo;` because we would
                // create the literal `Foo {}`, which is not valid.
                // TODO(jeanpierreda): special-case this.
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

/// `projected!(foo::T)` is the name of the type returned by
/// `foo::T::project()`.
///
/// If `foo::T` is not `#[recursively_pinned]`, then this returns the name it
/// would have used, but is essentially useless.
#[proc_macro]
pub fn projected(name: TokenStream) -> TokenStream {
    let mut name = syn::parse_macro_input!(name as syn::Path);
    {
        let last = name.segments.last_mut().expect("Path must have at least one element");
        if let syn::PathArguments::Parenthesized(..) = &last.arguments {
            panic!("Parenthesized patns (e.g. fn, Fn) do not have projected equivalents.")
        }
        last.ident = projected_ident(&last.ident);
    }
    TokenStream::from(quote! { #name })
}

fn projected_ident(ident: &Ident) -> Ident {
    Ident::new(&format!("__CrubitProjected{}", ident), Span::call_site())
}

fn projected_struct(s: syn::DeriveInput) -> (syn::DeriveInput, proc_macro2::TokenStream) {
    let mut projected = s;
    // TODO(jeanpierreda): check attributes for repr(packed)
    projected.attrs.clear();

    let original_ident = projected.ident.clone();
    projected.ident = projected_ident(&projected.ident);
    let projected_ident = &projected.ident;

    assert_eq!(
        projected.generics.params.len(),
        0,
        "pin projection is currently not implemented for generic structs"
    );

    let is_fieldless = match &projected.data {
        syn::Data::Struct(data) => data.fields.is_empty(),
        syn::Data::Enum(e) => e.variants.iter().all(|variant| variant.fields.is_empty()),
        syn::Data::Union(_) => unimplemented!(),
    };

    let lifetime;
    if is_fieldless {
        lifetime = quote! {};
    } else {
        let syn_lifetime = syn::Lifetime::new("'proj", Span::call_site());
        projected
            .generics
            .params
            .push(syn::GenericParam::Lifetime(syn::LifetimeDef::new(syn_lifetime.clone())));
        lifetime = quote! {#syn_lifetime};
    }

    let project_field = |field: &mut syn::Field| {
        field.attrs.clear();
        let field_ty = &field.ty;
        let pin_ty = syn::parse_quote!(::std::pin::Pin<& #lifetime mut #field_ty>);
        field.ty = syn::Type::Path(pin_ty);
    };
    // returns the braced parts of a projection pattern and return value.
    // e.g. {foo, bar}, {foo: Pin::new_unchecked(foo), bar: Pin::new_unchecked(bar)}
    let pat_project = |fields: &mut syn::Fields| {
        let mut pat = quote! {};
        let mut project = quote! {};
        for (i, field) in fields.iter_mut().enumerate() {
            // TODO(jeanpierreda): check attributes for e.g. #[unpin]
            field.attrs.clear();
            let lhs;
            let rhs;
            if let Some(ident) = &field.ident {
                lhs = quote! {#ident};
                rhs = ident.clone();
                pat.extend(quote! {#lhs,});
            } else {
                lhs = proc_macro2::Literal::usize_unsuffixed(i).into_token_stream();
                rhs = Ident::new(&format!("item_{i}"), Span::call_site());
                pat.extend(quote! {#lhs: #rhs,});
            }
            project.extend(quote! {#lhs: ::std::pin::Pin::new_unchecked(#rhs),});
        }
        (quote! {{#pat}}, quote! {{#project}})
    };
    let project_body;
    match &mut projected.data {
        syn::Data::Struct(data) => {
            for field in &mut data.fields {
                project_field(field);
            }
            let (pat, project) = pat_project(&mut data.fields);
            project_body = quote! {
                let #original_ident #pat = from;
                Self #project
            };
        }
        syn::Data::Enum(e) => {
            let mut match_body = quote! {};
            for variant in &mut e.variants {
                for field in &mut variant.fields {
                    project_field(field);
                }
                let (pat, project) = pat_project(&mut variant.fields);
                let variant_ident = &variant.ident;
                match_body.extend(quote! {
                    #original_ident::#variant_ident #pat => Self::#variant_ident #project,
                });
            }
            project_body = quote! {
                match from  {
                    #match_body
                }
            };
        }
        syn::Data::Union(_) => unimplemented!("Unions are not supported"),
    }
    let impl_block = quote! {
        impl<#lifetime> #projected_ident<#lifetime> {
            fn new(from: ::std::pin::Pin<& #lifetime mut #original_ident>) -> Self {
                unsafe {
                    let from = ::std::pin::Pin::into_inner_unchecked(from);
                    #project_body
                }
            }
        }
    };
    (projected, impl_block)
}

/// #[recursively_pinned] pins every field, similar to #[pin_project], and marks the struct `!Unpin`.
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
/// This is analogous to using pin_project, pinning every field, as so:
///
/// ```
/// #[pin_project(!Unpin)]
/// struct S {
///   #[pin]
///   field: i32,
/// }
/// ```
#[proc_macro_attribute]
pub fn recursively_pinned(args: TokenStream, item: TokenStream) -> TokenStream {
    assert!(args.is_empty(), "recursively_pinned does not take any arguments.");
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let (projected, projected_impl) = projected_struct(input.clone());
    let projected_ident = &projected.ident;

    let name = input.ident.clone();

    let expanded = quote! {
        #input
        #projected
        #projected_impl

        impl #name {
            #[must_use]
            #[inline(always)]
            fn project(self: ::std::pin::Pin<&mut Self>) -> #projected_ident {
                #projected_ident::new(self)
            }
        }

        unsafe impl ::ctor::RecursivelyPinned for #name {}
        impl !Unpin for #name {}
    };

    TokenStream::from(expanded)
}
