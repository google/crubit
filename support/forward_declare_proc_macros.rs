// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn symbol(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::LitStr);
    let string = input.value();
    let parts = string.chars().map(|c| {
        let c = syn::LitChar::new(c, input.span());
        quote! {
          ::forward_declare::C<#c>
        }
    });

    TokenStream::from(quote! {
      ::forward_declare::Symbol<(
          #(#parts,)*
      )>
    })
}

struct ForwardDeclareArgs {
    vis: syn::Visibility,
    ident: syn::Ident,
    #[allow(unused)] // honestly just here to make parsing code simpler.
    sep: syn::Token![=],
    symbol: syn::Type,
}

impl syn::parse::Parse for ForwardDeclareArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(ForwardDeclareArgs {
            vis: input.parse()?,
            ident: input.parse()?,
            sep: input.parse()?,
            symbol: input.parse()?,
        })
    }
}

/// Generates a unique forward-declared IncompleteType.
///
/// Usage: `forward_declare!(Alias = "path");`.
///
/// This creates a type alias `type Alias = IncompleteType<symbol!("path"),
/// ???>`, with some unique type for `???`.
#[proc_macro]
pub fn forward_declare(input: TokenStream) -> TokenStream {
    let ForwardDeclareArgs { vis, ident, sep: _, symbol } =
        syn::parse_macro_input!(input as ForwardDeclareArgs);
    // Generate a unique struct for the second type parameter to Incomplete.
    // This is the real reason we need to use a proc macro -- there's no way to do
    // this in a non-proc macro. See https://github.com/rust-lang/rust/issues/29599.
    //
    // It basically suffices to just prefix forward_declare_ at the front and assume
    // nobody else will do that. If someone forward-declares the same type twice
    // in the same scope then this will be an error, but that would be true even
    // if we made a totally random identifier. (Because the type alias we create
    // with this will be duplicated.)
    let param_ident = syn::Ident::new(
        &format!("_forward_declare_{}", ident.to_string()),
        proc_macro2::Span::call_site(),
    );

    TokenStream::from(quote! {
      #vis struct #param_ident;
      #vis type #ident = ::forward_declare::Incomplete<#symbol, #param_ident>;
    })
}
