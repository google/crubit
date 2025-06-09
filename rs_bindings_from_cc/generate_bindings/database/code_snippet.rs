// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Generate the final bindings, including structures for code snippet, feature
/// gating, etc.
use crate::db::BindingsGenerator;
use crate::rs_snippet::RsTypeKind;
use arc_anyhow::{anyhow, Error, Result};
use code_gen_utils::{expect_format_cc_type_name, CcInclude};
use ffi_types::FfiU8SliceBox;
use flagset::FlagSet;
use ir::{BazelLabel, GenericItem, Item, RecordType, UnqualifiedIdentifier};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{quote, ToTokens};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

/// FFI equivalent of `Bindings`.
#[repr(C)]
pub struct FfiBindings {
    pub rs_api: FfiU8SliceBox,
    pub rs_api_impl: FfiU8SliceBox,
    pub error_report: FfiU8SliceBox,
    pub fatal_errors: FfiU8SliceBox,
}

#[derive(Clone, Debug, Default)]
pub struct ApiSnippets {
    /// Main API - for example:
    /// - A Rust definition of a function (with a doc comment),
    /// - A Rust definition of a struct (with a doc comment).
    pub main_api: Vec<MainApi>,

    /// Rust implementation details - for example:
    /// - A Rust declaration of an `extern "C"` thunk,
    /// - Rust static assertions about struct size, aligment, and field offsets.
    pub thunks: Vec<Thunk>,
    pub assertions: Vec<Assertion>,

    /// C++ implementation details - for example:
    /// - A C++ implementation of an `extern "C"` thunk,
    /// - C++ static assertions about struct size, aligment, and field offsets.
    pub cc_details: Vec<ThunkImpl>,

    pub features: FlagSet<Feature>,
}

impl From<MainApi> for ApiSnippets {
    fn from(main_api: MainApi) -> Self {
        ApiSnippets { main_api: vec![main_api], ..Default::default() }
    }
}

/// Source code for generated bindings.
pub struct Bindings {
    // Rust source code.
    pub rs_api: String,
    // C++ source code.
    pub rs_api_impl: String,
}

/// Source code for generated bindings, as tokens.
///
/// This is public within the crate for testing purposes.
pub struct BindingsTokens {
    // Rust source code.
    pub rs_api: TokenStream,
    // C++ source code.
    pub rs_api_impl: TokenStream,
}

/// A missing set of crubit features caused by a capability that requires that
/// feature.
///
/// For example, if addition is not implemented due to missing the Experimental
/// feature on //foo, then we might have something like:
///
/// ```
/// RequiredCrubitFeature {
///   target: "//foo".into(),
///   item: "kFoo".into(),
///   missing_features: CrubitFeature::Experimental.into(),
///   capability_description: "int addition".into(),
/// }
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RequiredCrubitFeature {
    pub target: BazelLabel,
    pub item: Rc<str>,
    pub missing_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    pub capability_description: Rc<str>,
}

impl Display for RequiredCrubitFeature {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self { target, item, missing_features, capability_description } = self;
        let feature_strings: Vec<&str> =
            missing_features.into_iter().map(|feature| feature.aspect_hint()).collect();
        write!(f, "{target} needs [{features}] for {item}", features = feature_strings.join(", "),)?;
        if !capability_description.is_empty() {
            write!(f, " ({capability_description})")?;
        }
        Ok(())
    }
}

/// Returns the list of features required to use the item which are not yet
/// enabled.
///
/// If the item doesn't have a defining target, the return value is meaningless,
/// and bindings will always be generated.
///
/// If the item does have a defining target, and it doesn't enable the specified
/// features, then bindings are suppressed for this item.
pub fn required_crubit_features(
    db: &dyn BindingsGenerator,
    item: &Item,
) -> Result<Vec<RequiredCrubitFeature>> {
    let mut missing_features = vec![];

    let ir = &db.ir();

    let require_any_feature =
        |missing_features: &mut Vec<RequiredCrubitFeature>,
         alternative_required_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
         capability_description: &dyn Fn() -> Rc<str>| {
            // We refuse to generate bindings if either the definition of an item, or
            // instantiation (if it is a template) of an item are in a translation unit
            // which doesn't have the required Crubit features.
            for target in item.defining_target().into_iter().chain(item.owning_target().as_ref()) {
                let enabled_features = ir.target_crubit_features(target);
                if (alternative_required_features & enabled_features).is_empty() {
                    missing_features.push(RequiredCrubitFeature {
                        target: target.clone(),
                        item: item.debug_name(ir),
                        missing_features: alternative_required_features,
                        capability_description: capability_description(),
                    });
                }
            }
        };

    let require_rs_type_kind = |missing_features: &mut Vec<RequiredCrubitFeature>,
                                rs_type_kind: &RsTypeKind,
                                context: &dyn Fn() -> Rc<str>| {
        for target in item.defining_target().into_iter().chain(item.owning_target().as_ref()) {
            let (missing, desc) =
                rs_type_kind.required_crubit_features(db, ir.target_crubit_features(target));
            if !missing.is_empty() {
                let context = context();
                let capability_description = if desc.is_empty() {
                    context
                } else if context.is_empty() {
                    desc.into()
                } else {
                    format!("{context}: {desc}").into()
                };
                missing_features.push(RequiredCrubitFeature {
                    target: target.clone(),
                    item: item.debug_name(ir),
                    missing_features: missing,
                    capability_description,
                });
            }
        }
    };

    if let Some(unknown_attr) = item.unknown_attr() {
        require_any_feature(
            &mut missing_features,
            crubit_feature::CrubitFeature::Experimental.into(),
            &|| format!("unknown attribute(s): {unknown_attr}").into(),
        );
    }
    match item {
        Item::UnsupportedItem(..) => {}
        Item::Func(func) => {
            if func.rs_name == UnqualifiedIdentifier::Destructor {
                // We support destructors in supported even though they use some features we
                // don't generally support with that feature set, because in this
                // particular case, it's safe.
                require_any_feature(
                    &mut missing_features,
                    crubit_feature::CrubitFeature::Supported.into(),
                    &|| "destructors".into(),
                );
            } else {
                let return_type = db.rs_type_kind(func.return_type.clone())?;
                require_rs_type_kind(&mut missing_features, &return_type, &|| "return type".into());
                for (i, param) in func.params.iter().enumerate() {
                    let param_type = db.rs_type_kind(param.type_.clone())?;
                    require_rs_type_kind(&mut missing_features, &param_type, &|| {
                        format!("the type of {} (parameter #{i})", &param.identifier).into()
                    });
                }
                if func.is_extern_c {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Supported.into(),
                        &|| "extern \"C\" function".into(),
                    );
                } else {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Supported.into(),
                        &|| "non-extern \"C\" function".into(),
                    );
                }
                if !func.has_c_calling_convention {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Experimental.into(),
                        &|| "non-C calling convention".into(),
                    );
                }
                if func.is_noreturn {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Experimental.into(),
                        &|| "[[noreturn]] attribute".into(),
                    );
                }
                if func.nodiscard.is_some() {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Experimental.into(),
                        &|| "[[nodiscard]] attribute".into(),
                    );
                }
                if func.deprecated.is_some() {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Experimental.into(),
                        &|| "[[deprecated]] attribute".into(),
                    );
                }
                for param in &func.params {
                    if let Some(unknown_attr) = &param.unknown_attr {
                        require_any_feature(
                            &mut missing_features,
                            crubit_feature::CrubitFeature::Experimental.into(),
                            &|| {
                                format!(
                                    "param {param} has unknown attribute(s): {unknown_attr}",
                                    param = &param.identifier.identifier
                                )
                                .into()
                            },
                        );
                    }
                }
            }
        }
        Item::Record(_) | Item::TypeAlias(_) | Item::Enum(_) => {
            require_rs_type_kind(
                &mut missing_features,
                // We use from_item_raw here because required_crubit_features is itself called
                // by `BindingsGenerator::rs_type_kind()` in order to decide if it should return
                // an error.
                &RsTypeKind::from_item_raw(db, item.clone())?,
                &|| "".into(),
            );
        }
        Item::GlobalVar(_) => {}
        Item::Namespace(_) => {
            require_any_feature(
                &mut missing_features,
                crubit_feature::CrubitFeature::Supported.into(),
                &|| "namespace".into(),
            );
        }
        Item::IncompleteRecord(_) => {
            require_any_feature(
                &mut missing_features,
                crubit_feature::CrubitFeature::Wrapper.into(),
                &|| "incomplete type".into(),
            );
        }
        Item::Comment { .. } | Item::UseMod { .. } => {}
        Item::TypeMapOverride { .. } => {
            require_any_feature(
                &mut missing_features,
                crubit_feature::CrubitFeature::Experimental.into(),
                &|| "type map override".into(),
            );
        }
    }
    Ok(missing_features)
}

/// Visibility of an item.
///
/// Generally speaking, if an error occurs (e.g. a bindings doesn't exist), then
/// the way to "keep going" to catch more errors is to pretend that the missing
/// item is `Public`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Visibility {
    /// The item has `pub` visibility.
    #[default]
    Public,
    /// The item has `pub(crate)` visibility.
    PubCrate,
}

impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Visibility::Public => quote! {pub}.to_tokens(tokens),
            Visibility::PubCrate => quote! {pub(crate)}.to_tokens(tokens),
        }
    }
}

/// Information about the bindings that this item will have.
///
/// When this is returned, and the item is not a `Func`, then the bindings are
/// guaranteed to exist.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct BindingsInfo {
    pub visibility: Visibility,
}

/// Information about why bindings do not exist.
#[derive(Clone, PartialEq, Eq)]
pub enum NoBindingsReason {
    MissingRequiredFeatures {
        context: Rc<str>,
        missing_features: Vec<RequiredCrubitFeature>,
    },
    DependencyFailed {
        context: Rc<str>,
        error: Error,
    },
    /// This is directly unsupported.
    Unsupported {
        context: Rc<str>,
        error: Error,
    },
}

impl Display for NoBindingsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Error as Display>::fmt(&Error::from(self.clone()), f)
    }
}

impl std::fmt::Debug for NoBindingsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Error as std::fmt::Debug>::fmt(&Error::from(self.clone()), f)
    }
}

impl From<NoBindingsReason> for Error {
    fn from(reason: NoBindingsReason) -> Error {
        match reason {
            NoBindingsReason::MissingRequiredFeatures { context, missing_features } => {
                // This maybe could use .context(), but the ordering is backward.
                let mut all_missing = vec![];
                for missing in missing_features {
                    all_missing.push(missing.to_string());
                }
                anyhow!(
                    "Can't generate bindings for {context}, because of missing required features (<internal link>):\n{}",
                    all_missing.join("\n")
                )
            }
            NoBindingsReason::DependencyFailed { context, error } => error.context(format!(
                "Can't generate bindings for {context} due to missing bindings for its dependency"
            )),
            NoBindingsReason::Unsupported { context, error } => error.context(format!(
                "Can't generate bindings for {context}, because it is unsupported"
            )),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MainApi {
    Comment {
        message: Rc<str>,
    },
    Enum(TokenStream),
    Func(TokenStream),
    Record(TokenStream),
    Newline,
    UpcastImpl {
        base_name: TokenStream,
        derived_name: TokenStream,
        body: UpcastImplBody,
    },
    Namespace {
        name: Ident,
        previous_namespace_to_use: Option<Ident>,
        items: Vec<MainApi>,
        insert_use_stmt_for_inline_namespace: bool,
    },
    ForwardDeclare {
        visibility: Visibility,
        ident: Ident,
        symbol: String,
    },
    UseMod {
        path: Rc<str>,
        mod_name: Ident,
    },
    GlobalVar {
        link_name: Option<Rc<str>>,
        visibility: Visibility,
        is_mut: bool,
        ident: Ident,
        type_tokens: TokenStream,
    },
    TypeAlias {
        doc_comment: Option<DocCommentAttr>,
        visibility: Visibility,
        ident: Ident,
        underlying_type: TokenStream,
    },
}

impl ToTokens for MainApi {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            MainApi::Comment { message } => quote! { __COMMENT__ #message }.to_tokens(tokens),
            MainApi::Enum(enum_item) => enum_item.to_tokens(tokens),
            MainApi::Func(func_item) => func_item.to_tokens(tokens),
            MainApi::Record(record_item) => record_item.to_tokens(tokens),
            MainApi::Newline => quote! { __NEWLINE__ }.to_tokens(tokens),
            MainApi::UpcastImpl { base_name, derived_name, body } => {
                let body = match body {
                    UpcastImplBody::PointerOffset { offset } => {
                        let offset = Literal::i64_unsuffixed(*offset);
                        quote! { (derived as *const _ as *const u8).offset(#offset) as *const #base_name }
                    }
                    UpcastImplBody::CastThunk { crate_root_path, cast_fn_name } => {
                        let path = if let Some(crate_root_path) = crate_root_path {
                            quote! { crate :: #crate_root_path }
                        } else {
                            quote! { crate }
                        };
                        quote! { #path::detail::#cast_fn_name(derived) }
                    }
                };

                quote! {
                    unsafe impl oops::Inherits<#base_name> for #derived_name {
                        unsafe fn upcast_ptr(derived: *const Self) -> *const #base_name {
                            #body
                        }
                    }
                }
                .to_tokens(tokens);
            }
            MainApi::Namespace {
                name,
                previous_namespace_to_use,
                items,
                insert_use_stmt_for_inline_namespace,
            } => {
                let use_stmt_for_previous_namespace =
                    previous_namespace_to_use.as_ref().map(|previous_namespace_ident| {
                        quote! {
                          __HASH_TOKEN__ [allow(unused_imports)]
                          pub use super::#previous_namespace_ident::*; __NEWLINE__ __NEWLINE__
                        }
                    });

                quote! {
                    pub mod #name {
                        #use_stmt_for_previous_namespace

                        #( #items __NEWLINE__ __NEWLINE__ )*
                    }
                    __NEWLINE__
                }
                .to_tokens(tokens);

                if *insert_use_stmt_for_inline_namespace {
                    // TODO(b/308949532): Skip re-export if the canonical module is empty
                    // (transitively).
                    quote! {
                        __HASH_TOKEN__ [allow(unused_imports)]
                        pub use #name::*;
                    }
                    .to_tokens(tokens);
                }
            }
            MainApi::ForwardDeclare { visibility, ident, symbol } => {
                quote! {
                    forward_declare::forward_declare!(
                        #visibility #ident __SPACE__ = __SPACE__ forward_declare::symbol!(#symbol)
                    );
                }
                .to_tokens(tokens);
            }
            MainApi::UseMod { path, mod_name } => quote! {
                #[path = #path]
                mod #mod_name;
                __HASH_TOKEN__ [allow(unused_imports)]
                pub use #mod_name::*;
            }
            .to_tokens(tokens),
            MainApi::GlobalVar { link_name, visibility, is_mut, ident, type_tokens } => {
                let link_name_attr =
                    link_name.as_deref().map(|link_name| quote! { #[link_name = #link_name] });
                let mut_kw = if *is_mut { Some(quote! { mut }) } else { None };
                quote! {
                    extern "C" {
                        #link_name_attr
                        #visibility static #mut_kw #ident: #type_tokens;
                    }
                }
                .to_tokens(tokens);
            }
            MainApi::TypeAlias { doc_comment, visibility, ident, underlying_type } => {
                quote! {
                    #doc_comment
                    #visibility type #ident = #underlying_type;
                }
                .to_tokens(tokens);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum UpcastImplBody {
    PointerOffset { offset: i64 },
    CastThunk { crate_root_path: Option<Ident>, cast_fn_name: Ident },
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocCommentAttr(pub Rc<str>);

impl ToTokens for DocCommentAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(doc_comment) = self;
        quote! { #[doc = #doc_comment] }.to_tokens(tokens);
    }
}

flagset::flags! {
    #[allow(non_camel_case_types)]
    pub enum Feature: u32 {
        // <internal link> start
        allocator_api,
        arbitrary_self_types,
        cfg_sanitize,
        custom_inner_attributes,
        impl_trait_in_assoc_type,
        negative_impls,
        register_tool,
        // <internal link> end
    }
}

impl ToTokens for Feature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Feature::allocator_api => quote! { allocator_api },
            Feature::arbitrary_self_types => quote! { arbitrary_self_types },
            Feature::cfg_sanitize => quote! { cfg_sanitize },
            Feature::custom_inner_attributes => quote! { custom_inner_attributes },
            Feature::impl_trait_in_assoc_type => quote! { impl_trait_in_assoc_type },
            Feature::negative_impls => quote! { negative_impls },
            Feature::register_tool => quote! { register_tool },
        }
        .to_tokens(tokens);
    }
}

/// Compile-time assertions for the generated bindings.
#[derive(Clone, Debug)]
pub enum Assertion {
    /// Asserts that a type has a certain size and alignment.
    SizeAlign { type_name: TokenStream, size: usize, alignment: usize },
    /// Asserts that a type implements (or does not implement) certain traits.
    Impls {
        type_name: TokenStream,
        /// Assert that all of these traits are implemented.
        all_of: FlagSet<AssertableTrait>,
        /// Assert that none of these traits are implemented.
        none_of: FlagSet<AssertableTrait>,
    },
    /// Asserts that a list of fields have their expected offsets using the [`core::mem::offset_of`]
    /// macro.
    FieldOffsets { qualified_ident: TokenStream, fields_and_expected_offsets: Vec<(Ident, usize)> },
}

flagset::flags! {
    /// Traits that can be asserted as implemented or not implemented on a type, using the
    /// `static_assertions` macros.
    // Note: ordering of variants determines iteration order on `FlagSet<AssertableTrait>`.
    pub enum AssertableTrait: u32 {
        Copy,
        Clone,
        Drop,
    }
}

impl ToTokens for Assertion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Assertion::SizeAlign { type_name, size, alignment } => {
                let size = Literal::usize_unsuffixed(*size);
                let alignment = Literal::usize_unsuffixed(*alignment);
                quote! {
                    assert!(::core::mem::size_of::<#type_name>() == #size);
                    assert!(::core::mem::align_of::<#type_name>() == #alignment);
                }
                .to_tokens(tokens);
            }
            Assertion::Impls { type_name, all_of, none_of } => {
                assert!(
                    all_of.is_disjoint(*none_of),
                    "Found contradicting impl assertions, this is a bug"
                );
                if !all_of.is_empty() {
                    let all_of_iter = all_of.into_iter();
                    quote! {
                        static_assertions::assert_impl_all! (#type_name: #(#all_of_iter),*);
                    }
                    .to_tokens(tokens);
                }
                if !none_of.is_empty() {
                    let none_of_iter = none_of.into_iter();
                    quote! {
                        static_assertions::assert_not_impl_any! (#type_name: #(#none_of_iter),*);
                    }
                    .to_tokens(tokens);
                }
            }
            Assertion::FieldOffsets { qualified_ident, fields_and_expected_offsets } => {
                for (field_ident, expected_offset) in fields_and_expected_offsets {
                    let expected_offset = Literal::usize_unsuffixed(*expected_offset);
                    quote! {
                        assert!(::core::mem::offset_of!(#qualified_ident, #field_ident) == #expected_offset);
                    }
                    .to_tokens(tokens);
                }
            }
        }
    }
}

impl ToTokens for AssertableTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AssertableTrait::Copy => quote! { Copy },
            AssertableTrait::Clone => quote! { Clone },
            AssertableTrait::Drop => quote! { Drop },
        }
        .to_tokens(tokens);
    }
}

/// A Rust function thunk that appears in an `unsafe extern "C"` block.
#[derive(Clone, Debug)]
pub enum Thunk {
    /// Generates a thunk for upcasting from a derived type to a base type.
    Upcast { cast_fn_name: Ident, derived_name: TokenStream, base_name: TokenStream },
    /// Generates a thunk for a function.
    Function {
        mangled_name: Option<Rc<str>>,
        thunk_ident: Ident,
        generic_params: TokenStream,
        param_idents: Vec<Ident>,
        param_types: Vec<TokenStream>,
        return_type_fragment: Option<TokenStream>,
    },
}

impl ToTokens for Thunk {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Thunk::Upcast { cast_fn_name, derived_name, base_name } => {
                quote! {
                    pub fn #cast_fn_name(from: *const #derived_name) -> *const #base_name;
                }
                .to_tokens(tokens);
            }
            Thunk::Function {
                mangled_name,
                thunk_ident,
                generic_params,
                param_idents,
                param_types,
                return_type_fragment,
            } => {
                if let Some(mangled_name) = mangled_name {
                    quote! {
                        #[link_name = #mangled_name]
                    }
                    .to_tokens(tokens);
                }

                let return_type_fragment =
                    return_type_fragment.as_ref().map(|return_type| quote! { -> #return_type });

                // Note: some of these are `safe`, but _all_ of them are currently wrapped by a
                // (possibly safe) function, so we leave them all `unsafe` for convenience.
                quote! {
                    pub(crate) unsafe fn #thunk_ident #generic_params(
                        #( #param_idents: #param_types ),*
                    ) #return_type_fragment ;
                }
                .to_tokens(tokens);
            }
        }
    }
}

/// Abstract representation of generated C++ code that implements the Rust thunk.
#[derive(Clone, Debug)]
pub enum ThunkImpl {
    /// A function that upcasts from a derived type to a base type.
    Upcast { base_cc_name: TokenStream, cast_fn_name: Ident, derived_cc_name: TokenStream },
    /// A function that implements a Rust function thunk.
    Function {
        conversion_externs: TokenStream,
        return_type_name: TokenStream,
        thunk_ident: Ident,
        param_types: Vec<TokenStream>,
        param_idents: Vec<TokenStream>,
        conversion_stmts: TokenStream,
        return_stmt: TokenStream,
    },
    /// A set of `static_assert`s that check the layout of a record.
    LayoutAssertion {
        tag_kind: Option<RecordType>,
        namespace_qualifier: TokenStream,
        record_ident: Rc<str>,
        sizeof_impl: SizeofImpl,
        size: usize,
        alignment: usize,
        fields_and_expected_offsets: Vec<(TokenStream, usize)>,
    },
}

impl ToTokens for ThunkImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ThunkImpl::Upcast { base_cc_name, cast_fn_name, derived_cc_name } => {
                quote! {
                    extern "C" const #base_cc_name& #cast_fn_name(const #derived_cc_name& from) {
                        return from;
                    }
                }
                .to_tokens(tokens);
            }
            ThunkImpl::Function {
                conversion_externs,
                return_type_name,
                thunk_ident,
                param_types,
                param_idents,
                conversion_stmts,
                return_stmt,
            } => {
                quote! {
                    #conversion_externs

                    extern "C" #return_type_name #thunk_ident( #( #param_types #param_idents ),* ) {
                        #conversion_stmts
                        #return_stmt;
                    }
                }
                .to_tokens(tokens);
            }
            ThunkImpl::LayoutAssertion {
                tag_kind,
                namespace_qualifier,
                record_ident,
                sizeof_impl,
                size,
                alignment,
                fields_and_expected_offsets,
            } => {
                let size = Literal::usize_unsuffixed(*size);
                let alignment = Literal::usize_unsuffixed(*alignment);

                let record_ident = expect_format_cc_type_name(record_ident.as_ref());
                quote! {
                    static_assert(#sizeof_impl(#tag_kind #namespace_qualifier #record_ident) == #size);
                    static_assert(alignof(#tag_kind #namespace_qualifier #record_ident) == #alignment);
                }.to_tokens(tokens);

                for (field_ident, expected_offset) in fields_and_expected_offsets {
                    let expected_offset = Literal::usize_unsuffixed(*expected_offset);

                    quote! {
                        static_assert(CRUBIT_OFFSET_OF(#field_ident, #tag_kind #namespace_qualifier #record_ident) == #expected_offset);
                    }.to_tokens(tokens);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CppIncludes {
    pub internal_includes: TokenStream,
    pub ir_includes: Vec<CcInclude>,
}

impl ToTokens for CppIncludes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let CppIncludes { internal_includes, ir_includes } = self;
        quote! {
            #internal_includes
            __NEWLINE__
            __COMMENT__ "Public headers of the C++ library being wrapped."
            #( #ir_includes )* __NEWLINE__
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub enum SizeofImpl {
    /// The `sizeof` keyword.
    Builtin,
    /// Like sizeof, but rounds up to alignment in case the type has a strange
    /// sizeof.
    ///
    /// In particular, this is true of type aliases which override alignment but
    /// not size, as in e.g. `typedef __attribute__((aligned(N)) struct {} MyAlias;`.
    RoundUpToAlignment,
}

impl ToTokens for SizeofImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            SizeofImpl::Builtin => quote! { sizeof },
            SizeofImpl::RoundUpToAlignment => quote! { CRUBIT_SIZEOF },
        }
        .to_tokens(tokens);
    }
}

/// Abstract representation of a *_rs_api_impl file.
pub struct CppDetails {
    includes: CppIncludes,
    // The "pragma clang diagnostic push/pop" is automatically inserted around the thunks.
    thunks: Vec<ThunkImpl>,
}

impl CppDetails {
    pub fn new(includes: CppIncludes) -> Self {
        CppDetails { includes, thunks: vec![] }
    }
}

impl Extend<ThunkImpl> for CppDetails {
    fn extend<T: IntoIterator<Item = ThunkImpl>>(&mut self, iter: T) {
        self.thunks.extend(iter);
    }
}

impl ToTokens for CppDetails {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let CppDetails { includes, thunks } = self;
        quote! {
            #includes
            __NEWLINE__
            __HASH_TOKEN__ pragma clang diagnostic push __NEWLINE__
            // Disable Clang thread-safety-analysis warnings that would otherwise
            // complain about thunks that call mutex locking functions in an unpaired way.
            __HASH_TOKEN__ pragma clang diagnostic ignored "-Wthread-safety-analysis" __NEWLINE__ __NEWLINE__

            #( #thunks __NEWLINE__ __NEWLINE__ )*

            __HASH_TOKEN__ pragma clang diagnostic pop __NEWLINE__
        }
        .to_tokens(tokens);
    }
}
