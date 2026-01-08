// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::borrow::Cow;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::vec;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned, ToTokens as _};
use syn::parse::Parse;
use syn::spanned::Spanned as _;
use syn::Token;

struct CrateRename(pub Ident);

impl Parse for CrateRename {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: Token![crate] = input.parse()?;
        let _: Token![=] = input.parse()?;
        let ident = input.parse()?;
        Ok(CrateRename(ident))
    }
}

fn derive_crate_name(input: &syn::DeriveInput) -> syn::Result<Ident> {
    let mut result = Ident::new("ctor", Span::call_site());
    for attr in &input.attrs {
        if !attr.path.is_ident("ctor") {
            continue;
        }
        CrateRename(result) = attr.parse_args()?;
    }
    Ok(result)
}

// TODO(jeanpierreda): derive constructors and assignment for copy and move.

const FIELD_FOR_MUST_USE_CTOR: &'static str = "__must_use_ctor_to_initialize";

#[proc_macro_derive(CtorFrom_Default, attributes(ctor))]
pub fn derive_default(item: TokenStream) -> TokenStream {
    match derive_default_impl(syn::parse_macro_input!(item as syn::DeriveInput)) {
        Ok(t) => t.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn derive_default_impl(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ctor = derive_crate_name(&input)?;

    let struct_name = input.ident;
    let struct_ctor_name =
        Ident::new(&format!("_ctor_derive_{}_CtorType_Default", struct_name), Span::call_site());
    let fields: proc_macro2::TokenStream = match &input.data {
        syn::Data::Struct(data) => {
            if let syn::Fields::Unit = data.fields {
                quote! {}
            } else {
                let filled_fields = data.fields.iter().enumerate().filter_map(|(i, field)| {
                    let field_i = syn::Index::from(i);
                    let field_name;
                    // This logic is here in case you derive default on the output of
                    // `#[recursively_pinned]`, but it's obviously not very flexible. For example,
                    // maybe we want to compute a non-colliding field name, and maybe there are
                    // other ordering problems.
                    match &field.ident {
                        Some(name) if name == FIELD_FOR_MUST_USE_CTOR => return None,
                        Some(name) => field_name = quote! {#name},
                        None => field_name = quote! {#field_i},
                    };

                    let field_type = &field.ty;
                    Some(quote_spanned! {field.span() =>
                        #field_name: <#field_type as ::ctor::CtorNew<()>>::ctor_new(())
                    })
                });
                quote! {{ #(#filled_fields),* }}
            }
        }
        syn::Data::Enum(e) => {
            return Err(syn::Error::new(e.enum_token.span, "Enums are not supported"));
        }
        syn::Data::Union(u) => {
            return Err(syn::Error::new(u.union_token.span, "Unions are not supported"));
        }
    };

    Ok(quote! {
        struct #struct_ctor_name();

        // SAFETY: unconditionally initializes dest.
        unsafe impl ::#ctor::Ctor for #struct_ctor_name {
            type Output = #struct_name;
            // TODO(jeanpierreda): This only handles the Infallible case,
            // but a derive should also handle non-infallible cases.
            type Error = ::#ctor::Infallible;
            unsafe fn ctor(self, dest: *mut Self::Output) -> ::core::result::Result<(), Self::Error> {
                ::#ctor::ctor!(
                    #struct_name #fields
                ).ctor(dest)
            }
        }

        impl !::core::marker::Unpin for #struct_ctor_name {}

        impl ::#ctor::CtorNew<()> for #struct_name {
            type CtorType = #struct_ctor_name;
            type Error = ::#ctor::Infallible;

            fn ctor_new(_args: ()) -> #struct_ctor_name { #struct_ctor_name() }
        }
    })
}

/// Derives `Ctor`-based move and copy constructors and assignment for a `Copy` type.
///
/// Specifically, this will provide an implementation of:
///
/// * `From<RvalueReference<_, Self>>`
/// * `CtorNew<RvalueReference<_, Self>>`
/// * `UnpinAssign<&Self>`
/// * `UnpinAssign<RvalueReference<_, Self>>`
///
/// To override the crate name used by the ctor crate, pass in `#[ctor(crate = something_else)]`,
/// as so:
///
/// ```
/// #[derive(MoveAndAssignViaCopy))]
/// #[ctor(crate = renamed_ctor)]
/// struct MyType;
/// ```
#[proc_macro_derive(MoveAndAssignViaCopy, attributes(ctor))]
pub fn derive_move_and_assign_via_copy(item: TokenStream) -> TokenStream {
    match derive_move_and_assign_via_copy_impl(syn::parse_macro_input!(item as syn::DeriveInput)) {
        Ok(t) => t.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn derive_move_and_assign_via_copy_impl(
    input: syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let ctor = derive_crate_name(&input)?;
    let type_name = input.ident;
    Ok(quote! {
        // Move construct.
        impl From<::#ctor::RvalueReference<'_, Self>> for #type_name {
            #[inline(always)]
            fn from(this: ::#ctor::RvalueReference<'_, Self>) -> Self { *this.get_ref() }
        }

        // Ctor move construct.
        impl ::#ctor::CtorNew<::#ctor::RvalueReference<'_, Self>> for #type_name {
            type CtorType = Self;
            type Error = ::#ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::#ctor::RvalueReference<'_, Self>) -> Self::CtorType {
                <Self as From<::#ctor::RvalueReference<'_, Self>>>::from(args)
            }
        }

        // Copy assign.
        impl ::#ctor::UnpinAssign<&Self> for #type_name {
            #[inline(always)]
            fn unpin_assign(&mut self, other: &Self) {
                *self = *other;
            }
        }

        // Move assign.
        impl ::#ctor::UnpinAssign<::#ctor::RvalueReference<'_, Self>> for #type_name {
            #[inline(always)]
            fn unpin_assign(&mut self, other: ::#ctor::RvalueReference<'_, Self>) {
                *self = *other.get_ref();
            }
        }
    })
}

/// `project_pin_type!(foo::T)` is the name of the type returned by
/// `foo::T::project_pin()`.
///
/// If `foo::T` is not `#[recursively_pinned]`, then this returns the name it
/// would have used, but is essentially useless.
#[proc_macro]
pub fn project_pin_type(name: TokenStream) -> TokenStream {
    project_type_impl(name, project_pin_ident)
}

fn project_pin_ident(ident: &Ident) -> Ident {
    Ident::new(&format!("__CrubitProjectPin{}", ident), Span::call_site())
}

/// `project_ref_type!(foo::T)` is the name of the type returned by
/// `foo::T::project_ref()`.
///
/// If `foo::T` is not `#[recursively_pinned]`, then this returns the name it
/// would have used, but is essentially useless.
#[proc_macro]
pub fn project_ref_type(name: TokenStream) -> TokenStream {
    project_type_impl(name, project_ref_ident)
}

fn project_ref_ident(ident: &Ident) -> Ident {
    Ident::new(&format!("__CrubitProjectRef{}", ident), Span::call_site())
}

fn project_type_impl(name: TokenStream, project_ident: fn(&Ident) -> Ident) -> TokenStream {
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
            last.ident = project_ident(&last.ident);
        }
    }
    TokenStream::from(quote! { #name })
}

/// Defines the `project_pin`/`project_ref` function, and its return type.
///
/// If the input is a union, this returns nothing, and pin-projection is not
/// implemented.
///
/// Args:
///   * input: the input type to project.
///   * method_name: the name of the method to define (`project_pin` or
///     `project_ref`).
///   * mut_: Mutability qualifier: `mut` for `project_pin`, empty for `project_ref`.
///   * project_ident: a function that takes an identifier for the type and returns the
///     identifier to use for the projection type.
fn project_method_impl(
    input: &syn::DeriveInput,
    method_name: proc_macro2::TokenStream,
    mut_: proc_macro2::TokenStream,
    project_ident: fn(&Ident) -> Ident,
) -> syn::Result<proc_macro2::TokenStream> {
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
    projected.ident = project_ident(&projected.ident);

    let lifetime = if is_fieldless {
        quote! {}
    } else {
        add_lifetime(&mut projected.generics, "'proj")
    };

    let project_field = |field: &mut syn::Field| {
        field.attrs.clear();
        let field_ty = &field.ty;
        let pin_ty = syn::parse_quote!(::core::pin::Pin<& #lifetime #mut_ #field_ty>);
        field.ty = syn::Type::Path(pin_ty);
    };
    // returns the braced parts of a projection pattern and return value.
    // e.g. {foo, bar, ..}, {foo: Pin::new_unchecked(foo), bar:
    // Pin::new_unchecked(bar)}
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
            project.extend(quote! {#lhs: ::core::pin::Pin::new_unchecked(#rhs),});
        }
        // Also ignore the __must_use_ctor_to_initialize field, if present.
        pat.extend(quote! {..});
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
            unreachable!("project_method_impl should early return when it finds a union")
        }
    }

    let (input_impl_generics, input_ty_generics, input_where_clause) =
        input.generics.split_for_impl();
    let (_, projected_generics, _) = projected.generics.split_for_impl();

    Ok(quote! {
        #projected

        impl #input_impl_generics #input_ident #input_ty_generics #input_where_clause {
            #[must_use]
            pub fn #method_name<#lifetime>(self: ::core::pin::Pin<& #lifetime #mut_ Self>) -> #projected_ident #projected_generics {
                unsafe {
                    let from = ::core::pin::Pin::into_inner_unchecked(self);
                    #project_body
                }
            }
        }
    })
}

/// Adds a new lifetime to `generics`, returning the quoted lifetime name.
fn add_lifetime(generics: &mut syn::Generics, prefix: &str) -> proc_macro2::TokenStream {
    let taken_lifetimes: BTreeSet<&syn::Lifetime> =
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

enum RecursivelyPinnedArg {
    PinnedDrop,
    RenamedCrate(Ident),
}

impl Parse for RecursivelyPinnedArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![crate]) {
            let CrateRename(new_crate) = input.parse()?;
            Ok(RecursivelyPinnedArg::RenamedCrate(new_crate))
        } else if input.parse::<Ident>()? == "PinnedDrop" {
            Ok(RecursivelyPinnedArg::PinnedDrop)
        } else {
            Err(syn::Error::new(
                input.span(),
                format!("unexpected argument: expected PinnedDrop or crate=..., but got: {input}"),
            ))
        }
    }
}

#[derive(Default)]
struct RecursivelyPinnedArgs {
    is_pinned_drop: bool,
    renamed_crate: Option<Ident>,
}

impl Parse for RecursivelyPinnedArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut result = RecursivelyPinnedArgs::default();
        let args =
            <syn::punctuated::Punctuated<RecursivelyPinnedArg, Token![,]>>::parse_terminated(
                input,
            )?;
        for arg in args {
            match arg {
                RecursivelyPinnedArg::PinnedDrop => {
                    result.is_pinned_drop = true;
                }
                RecursivelyPinnedArg::RenamedCrate(ident) => {
                    result.renamed_crate = Some(ident);
                }
            }
        }
        Ok(result)
    }
}

/// Prevents this type from being directly created outside of this crate in safe
/// code.
///
/// For enums and unit structs, this uses the `#[non_exhaustive]` attribute.
/// This leads to unfortunate error messages, but there is no other way to
/// prevent creation of an enum or a unit struct at this time.
///
/// For tuple structs, we also use `#[non_exhaustive]`, as it's no worse than
/// the alternative. Both adding a private field and adding `#[non_exhaustive]`
/// lead to indirect error messages, but `#[non_exhaustive]` is the more likely
/// of the two to ever get custom error message support.
///
/// Finally, for structs with named fields, we actually *cannot* use
/// `#[non_exhaustive]`, because it would make the struct not FFI-safe, and
/// structs with named fields are specifically supported for C++ interop.
/// Instead, we use a private field with a name that indicates the error.
/// (`__must_use_ctor_to_initialize`).
///
/// Unions are not yet implemented properly.
///
/// ---
///
/// Note that the use of `#[non_exhaustive]` also has other effects. At the
/// least: tuple variants and tuple structs marked with `#[non_exhaustive]`
/// cannot be pattern matched using the "normal" syntax. Instead, one must use
/// curly braces. (Broken: `T(x, ..)`; woken: `T{0: x, ..}`).
///
/// (This does not seem very intentional, and with all luck will be fixed before
/// too long.)
fn forbid_initialization(s: &mut syn::DeriveInput) {
    let non_exhaustive_attr = syn::parse_quote!(#[non_exhaustive]);
    match &mut s.data {
        // TODO(b/232969667): prevent creation of unions from safe code.
        // (E.g. hide inside a struct.)
        syn::Data::Union(_) => return,
        syn::Data::Struct(data) => {
            match &mut data.fields {
                syn::Fields::Unit | syn::Fields::Unnamed(_) => {
                    s.attrs.insert(0, non_exhaustive_attr);
                }
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field {
                        attrs: vec![],
                        vis: syn::Visibility::Inherited,
                        // TODO(jeanpierreda): better hygiene: work even if a field has the same name.
                        ident: Some(Ident::new(FIELD_FOR_MUST_USE_CTOR, Span::call_site())),
                        colon_token: Some(<syn::Token![:]>::default()),
                        ty: syn::parse_quote!([u8; 0]),
                    });
                }
            }
        }
        syn::Data::Enum(e) => {
            // Enums can't have private fields. Instead, we need to add #[non_exhaustive] to
            // every variant -- this makes it impossible to construct the
            // variants.
            for variant in &mut e.variants {
                variant.attrs.insert(0, non_exhaustive_attr.clone());
            }
        }
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
/// `#[recursively_pinned]` also provides `project_pin()` and `project_ref()` methods
/// which return a struct containing pinned references to each field, see below.
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
/// ### `crate=<ident>`
///
/// If the `ctor` crate is renamed or wrapped, you may need to pass the new
/// crate name to `#[recursively_pinned]`. For example:
///
/// ```
/// // We depend on `ctor` under the name `ctor2` so that we can also depend on another
/// // crate called `ctor`. :|
/// #[recursively_pinned(crate=ctor2)]
/// struct S {
///   field: i32,
/// }
/// ```
///
/// ## Direct initialization
///
/// Use the `ctor!` macro to instantiate recursively pinned types. For example:
///
/// ```
/// // equivalent to `let x = Point {x: 3, y: 4}`, but uses pinned construction.
/// let x = emplace!(ctor!(Point {x: 3, y: 4}));
/// ```
///
/// Recursively pinned types cannot be created directly in safe code, as they
/// are pinned from the very moment of their creation.
///
/// This is prevented either using `#[non_exhaustive]` or using a private field,
/// depending on the type in question. For example, enums use
/// `#[non_exhaustive]`, and structs with named fields use a private field named
/// `__must_use_ctor_to_initialize`. This can lead to confusing error messages,
/// so watch out!
///
/// ## Pin-projection
///
/// `#[recursively_pinned]` adds `project_pin` and `project_ref` methods to the
/// type, shaped like this:
///
/// ```
/// pub fn project_pin(self: Pin<&mut Self>) -> <Projected>;
/// pub fn project_ref(self: Pin<&Self>) -> <Projected>;
/// ```
///
/// `project_pin()` is a method on `Pin<&mut Self>`, and returns a type
/// containing pinned mutable references to each field. Similarly,
/// `project_ref()` projects to a type with pinned shared reference fields.
///
/// For example, given:
///
/// ```
/// #[recursively_pinned]
/// struct Point {
///   x: i32,
///   y: i32,
/// }
/// ```
///
/// One can mutate the `x` field of a `Pin<&mut Point>` as follows:
///
/// ```
/// p.as_mut().project_pin().x.set(42);
/// ```
///
/// ## Supported types
///
/// Structs, enums, and unions are all supported. However, unions do not receive
/// a `project_{pin,ref}` methods, as there is no way to implement pin projection for
/// unions. (One cannot know which field is active.)
#[proc_macro_attribute]
pub fn recursively_pinned(args: TokenStream, item: TokenStream) -> TokenStream {
    match recursively_pinned_impl(args.into(), item.into()) {
        Ok(t) => t.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

/// A separate function for calling from tests.
///
/// See e.g. https://users.rust-lang.org/t/procedural-macro-api-is-used-outside-of-a-procedural-macro/30841
fn recursively_pinned_impl(
    args: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let args = syn::parse2::<RecursivelyPinnedArgs>(args)?;
    let ctor = args.renamed_crate.unwrap_or(Ident::new("ctor", Span::call_site()));
    let mut input = syn::parse2::<syn::DeriveInput>(item)?;

    let project_pin_impl =
        project_method_impl(&input, quote! {project_pin}, quote! {mut}, project_pin_ident)?;
    let project_ref_impl =
        project_method_impl(&input, quote! {project_ref}, quote! {}, project_ref_ident)?;
    let name = input.ident.clone();

    // Create two copies of input: one (public) has a private field that can't be
    // instantiated. The other (only visible via
    // RecursivelyPinned::CtorInitializedFields) doesn't have this field.
    // This causes `ctor!(Foo {})` to work, but `Foo{}` to complain of a missing
    // field.
    let mut ctor_initialized_input = input.clone();
    // Removing repr(C) triggers dead-code detection.
    ctor_initialized_input.attrs = vec![syn::parse_quote!(#[allow(dead_code)])];
    // TODO(jeanpierreda): This should really check for name collisions with any types
    // used in the fields. Collisions with other names don't matter, because the
    // type is locally defined within a narrow scope.
    ctor_initialized_input.ident = syn::Ident::new(&format!("__CrubitCtor{name}"), name.span());
    let ctor_initialized_name = &ctor_initialized_input.ident;
    forbid_initialization(&mut input);

    let (input_impl_generics, input_ty_generics, input_where_clause) =
        input.generics.split_for_impl();

    let drop_impl = if args.is_pinned_drop {
        quote! {
            impl #input_impl_generics Drop for #name #input_ty_generics #input_where_clause {
                fn drop(&mut self) {
                    unsafe {::#ctor::PinnedDrop::pinned_drop(::core::pin::Pin::new_unchecked(self))}
                }
            }
        }
    } else {
        quote! {
            impl #input_impl_generics ::#ctor::macro_internal::DoNotImplDrop for #name #input_ty_generics #input_where_clause {}
            /// A no-op PinnedDrop that will cause an error if the user also defines PinnedDrop,
            /// due to forgetting to pass `PinnedDrop` to #[recursively_pinned(PinnedDrop)]`.
            impl #input_impl_generics ::#ctor::PinnedDrop for #name #input_ty_generics #input_where_clause {
                unsafe fn pinned_drop(self: ::core::pin::Pin<&mut Self>) {}
            }
        }
    };

    Ok(quote! {
        #input
        #project_pin_impl
        #project_ref_impl

        #drop_impl
        impl #input_impl_generics !Unpin for #name #input_ty_generics #input_where_clause {}

        // Introduce a new scope to limit the blast radius of the CtorInitializedFields type.
        // This lets us use relatively readable names: while the impl is visible outside the scope,
        // type is otherwise not visible.
        const _ : () = {
            #ctor_initialized_input

            unsafe impl #input_impl_generics ::#ctor::RecursivelyPinned for #name #input_ty_generics #input_where_clause {
                type CtorInitializedFields = #ctor_initialized_name #input_ty_generics;
            }
        };
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use googletest::prelude::*;
    use token_stream_matchers::assert_rs_matches;

    /// Essentially a change detector, but handy for debugging.
    ///
    /// At time of writing, we can't write negative compilation tests, so
    /// asserting on the output is as close as we can get. Once negative
    /// compilation tests are added, it would be better to test various
    /// safety features that way.
    #[gtest]
    fn test_recursively_pinned_struct() {
        let definition =
            recursively_pinned_impl(quote! {}, quote! {#[repr(C)] struct S {x: i32}}).unwrap();

        // The struct can't be directly created, but can be created via
        // CtorInitializedFields:
        assert_rs_matches!(
            definition,
            quote! {
                #[repr(C)]
                struct S {
                    x: i32,
                    __must_use_ctor_to_initialize: [u8; 0]
                }
            }
        );
        assert_rs_matches!(
            definition,
            quote! {
                const _: () = {
                   #[allow(dead_code)]
                   struct __CrubitCtorS {x: i32}
                   unsafe impl ::ctor::RecursivelyPinned for S {
                       type CtorInitializedFields = __CrubitCtorS;
                   }
                };
            }
        );

        // The type is non-Unpin:
        assert_rs_matches!(
            definition,
            quote! {
                impl !Unpin for S {}
            }
        );

        // The remaining features of the generated output are better tested via
        // real tests that exercise the code.
    }

    /// The enum version of `test_recursively_pinned_struct`.
    #[gtest]
    fn test_recursively_pinned_enum() {
        let definition = recursively_pinned_impl(
            quote! {},
            quote! {
                #[repr(C)]
                enum E {
                    A,
                    B(i32),
                }
            },
        )
        .unwrap();

        // The enum variants can't be directly created, but can be created via
        // CtorInitializedFields:
        assert_rs_matches!(
            definition,
            quote! {
                #[repr(C)]
                enum E {
                    #[non_exhaustive]
                    A,
                    #[non_exhaustive]
                    B(i32),
                }
            }
        );
        assert_rs_matches!(
            definition,
            quote! {
                const _: () = {
                    #[allow(dead_code)]
                    enum __CrubitCtorE {
                        A,
                        B(i32),
                    }
                    unsafe impl ::ctor::RecursivelyPinned for E {
                        type CtorInitializedFields = __CrubitCtorE;
                    }
                };
            }
        );

        // The type is non-Unpin:
        assert_rs_matches!(
            definition,
            quote! {
                impl !Unpin for E {}
            }
        );

        // The remaining features of the generated output are better tested via
        // real tests that exercise the code.
    }
}
