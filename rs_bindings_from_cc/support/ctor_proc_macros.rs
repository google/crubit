// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned, ToTokens as _};
use std::borrow::Cow;
use std::collections::HashSet;
use syn::parse::Parse;
use syn::spanned::Spanned as _;
use syn::Token;

// TODO(jeanpierreda): derive constructors and assignment for copy and move.

#[proc_macro_derive(CtorFrom_Default)]
pub fn derive_default(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_name = input.ident;
    let struct_ctor_name =
        Ident::new(&format!("_ctor_derive_{}_CtorType_Default", struct_name), Span::call_site());
    let fields: proc_macro2::TokenStream = match &input.data {
        syn::Data::Struct(data) => {
            if let syn::Fields::Unit = data.fields {
                quote! {}
            } else {
                let filled_fields = data.fields.iter().enumerate().map(|(i, field)| {
                    let field_i = syn::Index::from(i);
                    let field_name = match &field.ident {
                        None => quote! {#field_i},
                        Some(name) => quote! {#name},
                    };

                    let field_type = &field.ty;
                    quote_spanned! {field.span() =>
                        #field_name: <#field_type as ::ctor::CtorNew<()>>::ctor_new(())
                    }
                });
                quote! {{ #(#filled_fields),* }}
            }
        }
        syn::Data::Enum(e) => {
            return syn::Error::new(e.enum_token.span, "Enums are not supported")
                .into_compile_error()
                .into();
        }
        syn::Data::Union(u) => {
            return syn::Error::new(u.union_token.span, "Unions are not supported")
                .into_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        struct #struct_ctor_name();

        impl ::ctor::Ctor for #struct_ctor_name {
            type Output = #struct_name;
            unsafe fn ctor(self, dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self::Output>>) {
                ::ctor::ctor!(
                    #struct_name #fields
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

/// `project_pin_type!(foo::T)` is the name of the type returned by
/// `foo::T::project_pin()`.
///
/// If `foo::T` is not `#[recursively_pinned]`, then this returns the name it
/// would have used, but is essentially useless.
#[proc_macro]
pub fn project_pin_type(name: TokenStream) -> TokenStream {
    let mut name = syn::parse_macro_input!(name as syn::Path);
    match name.segments.last_mut() {
        None => {
            return syn::Error::new(name.span(), "Path must have at least one element")
                .into_compile_error()
                .into();
        }
        Some(last) => {
            if let syn::PathArguments::Parenthesized(p) = &last.arguments {
                return syn::Error::new(
                    p.span(),
                    "Parenthesized paths (e.g. fn, Fn) do not have projected equivalents.",
                )
                .into_compile_error()
                .into();
            }
            last.ident = project_pin_ident(&last.ident);
        }
    }
    TokenStream::from(quote! { #name })
}

fn project_pin_ident(ident: &Ident) -> Ident {
    Ident::new(&format!("__CrubitProjectPin{}", ident), Span::call_site())
}

/// Defines the `project_pin` function, and its return value.
///
/// If the input is a union, this returns nothing, and pin-projection is not
/// implemented.
fn project_pin_impl(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let is_fieldless = match &input.data {
        syn::Data::Struct(data) => data.fields.is_empty(),
        syn::Data::Enum(e) => e.variants.iter().all(|variant| variant.fields.is_empty()),
        syn::Data::Union(_) => {
            return Ok(quote! {});
        }
    };

    let mut projected = input.clone();
    // TODO(jeanpierreda): check attributes for repr(packed)
    projected.attrs.clear();
    projected.ident = project_pin_ident(&projected.ident);

    let lifetime = if is_fieldless {
        quote! {}
    } else {
        add_lifetime(&mut projected.generics, "'proj")
    };

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
    let input_ident = &input.ident;
    let projected_ident = &projected.ident;
    match &mut projected.data {
        syn::Data::Struct(data) => {
            for field in &mut data.fields {
                project_field(field);
            }
            let (pat, project) = pat_project(&mut data.fields);
            project_body = quote! {
                let #input_ident #pat = from;
                #projected_ident #project
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
                    #input_ident::#variant_ident #pat => #projected_ident::#variant_ident #project,
                });
            }
            project_body = quote! {
                match from  {
                    #match_body
                }
            };
        }
        syn::Data::Union(_) => {
            unreachable!("project_pin_impl should early return when it finds a union")
        }
    }

    let (input_impl_generics, input_ty_generics, input_where_clause) =
        input.generics.split_for_impl();
    let (_, projected_generics, _) = projected.generics.split_for_impl();

    Ok(quote! {
        #projected

        impl #input_impl_generics #input_ident #input_ty_generics #input_where_clause {
            #[must_use]
            pub fn project_pin<#lifetime>(self: ::std::pin::Pin<& #lifetime mut Self>) -> #projected_ident #projected_generics {
                unsafe {
                    let from = ::std::pin::Pin::into_inner_unchecked(self);
                    #project_body
                }
            }
        }
    })
}

/// Adds a new lifetime to `generics`, returning the quoted lifetime name.
fn add_lifetime(generics: &mut syn::Generics, prefix: &str) -> proc_macro2::TokenStream {
    let taken_lifetimes: HashSet<&syn::Lifetime> =
        generics.lifetimes().map(|def| &def.lifetime).collect();
    let mut name = Cow::Borrowed(prefix);
    let mut i = 1;
    let lifetime = loop {
        let lifetime = syn::Lifetime::new(&name, Span::call_site());
        if !taken_lifetimes.contains(&lifetime) {
            break lifetime;
        }

        i += 1;
        name = Cow::Owned(format!("{prefix}_{i}"));
    };
    let quoted_lifetime = quote! {#lifetime};
    generics.params.push(syn::GenericParam::Lifetime(syn::LifetimeDef::new(lifetime)));
    quoted_lifetime
}

#[derive(Default)]
struct RecursivelyPinnedArgs {
    is_pinned_drop: bool,
}

impl Parse for RecursivelyPinnedArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = <syn::punctuated::Punctuated<Ident, Token![,]>>::parse_terminated(input)?;
        if args.len() > 1 {
            return Err(syn::Error::new(
                input.span(), // not args.span(), as that is only for the first argument.
                &format!("expected at most 1 argument, got: {}", args.len()),
            ));
        }
        let is_pinned_drop = if let Some(arg) = args.first() {
            if arg != "PinnedDrop" {
                return Err(syn::Error::new(
                    arg.span(),
                    "unexpected argument (wasn't `PinnedDrop`)",
                ));
            }
            true
        } else {
            false
        };
        Ok(RecursivelyPinnedArgs { is_pinned_drop })
    }
}

/// `#[recursively_pinned]` pins every field, similar to `#[pin_project]`, and
/// marks the struct `!Unpin`.
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
///
/// ## Arguments
///
/// ### `PinnedDrop`
///
/// To define a destructor for a recursively-pinned struct, pass `PinnedDrop`
/// and implement the `PinnedDrop` trait.
///
/// `#[recursively_pinned]` prohibits implementing `Drop`, as that would make it
/// easy to violate the `Pin` guarantee. Instead, to define a destructor, one
/// must define a `PinnedDrop` impl, as so:
///
/// ```
/// #[recursively_pinned(PinnedDrop)]
/// struct S {
///   field: i32,
/// }
///
/// impl PinnedDrop for S {
///   unsafe fn pinned_drop(self: Pin<&mut Self>) {
///     println!("I am being destroyed!");
///   }
/// }
/// ```
///
/// (This is analogous to `#[pin_project(PinnedDrop)]`.)
///
/// ## Supported types
///
/// Structs, enums, and unions are all supported. However, unions do not receive
/// a `pin_project` method, as there is no way to implement pin projection for
/// unions. (One cannot know which field is active.)
#[proc_macro_attribute]
pub fn recursively_pinned(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as RecursivelyPinnedArgs);
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let project_pin_impl = match project_pin_impl(&input) {
        Ok(ok) => ok,
        Err(e) => return e.into_compile_error().into(),
    };

    let name = input.ident.clone();

    let (input_impl_generics, input_ty_generics, input_where_clause) =
        input.generics.split_for_impl();

    let drop_impl = if args.is_pinned_drop {
        quote! {
            impl #input_impl_generics Drop for #name #input_ty_generics #input_where_clause {
                fn drop(&mut self) {
                    unsafe {::ctor::PinnedDrop::pinned_drop(::std::pin::Pin::new_unchecked(self))}
                }
            }
        }
    } else {
        quote! {
            impl #input_impl_generics ::ctor::macro_internal::DoNotImplDrop for #name #input_ty_generics #input_where_clause {}
            /// A no-op PinnedDrop that will cause an error if the user also defines PinnedDrop,
            /// due to forgetting to pass `PinnedDrop` to #[recursively_pinned(PinnedDrop)]`.
            impl #input_impl_generics ::ctor::PinnedDrop for #name #input_ty_generics #input_where_clause {
                unsafe fn pinned_drop(self: ::std::pin::Pin<&mut Self>) {}
            }
        }
    };

    let expanded = quote! {
        #input
        #project_pin_impl

        #drop_impl

        unsafe impl #input_impl_generics ::ctor::RecursivelyPinned for #name #input_ty_generics #input_where_clause {
            // TODO(b/200067242): Generate a new type here, which omits a special field for
            // marking types as not to be constructed by value.
            type CtorInitializedFields = Self;
        }
        impl #input_impl_generics !Unpin for #name #input_ty_generics #input_where_clause {}
    };

    TokenStream::from(expanded)
}
