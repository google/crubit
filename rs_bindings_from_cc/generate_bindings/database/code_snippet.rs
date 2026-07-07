// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Generate the final bindings, including structures for code snippet, feature
/// gating, etc.
use crate::db::BindingsGenerator;
use crate::rs_snippet::{LifetimeOptions, PrimitiveName, RsTypeKind};
use arc_anyhow::{Error, Result};
use code_gen_utils::{expect_format_cc_type_name, make_rs_ident, CcInclude};
use crubit_feature::CrubitFeature;
use error_report::{anyhow, bail, ensure};
use ffi_types::FfiU8SliceBox;
use flagset::FlagSet;
use heck::ToSnakeCase;
use ir::{
    BazelLabel, GenericItem, IntegerConstant, Item, ItemId, Namespace, RecordType,
    UnqualifiedIdentifier,
};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::format_ident;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::fmt::Display;
use std::num::NonZeroUsize;
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
    /// Mapping from an item to the abstract representation of its generated bindings.
    ///
    /// The ordering of the items is irrelevant; the GeneratedItems are quoted by traversing the
    /// IR's top level items in order which is deterministic, and traversing their children in
    /// order, which is also deterministic.
    pub generated_items: HashMap<ItemId, GeneratedItem>,

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

    pub member_functions: HashMap<ItemId, Vec<TokenStream>>,

    pub free_functions: HashMap<ItemId, Vec<TokenStream>>,
}

impl ApiSnippets {
    pub fn append(&mut self, other: ApiSnippets) {
        for (item_id, generated_item) in other.generated_items {
            use std::collections::hash_map::Entry::*;
            match self.generated_items.entry(item_id) {
                Vacant(vacant) => {
                    // Other has generated bindings for an Item that self hasn't.
                    vacant.insert(generated_item);
                }
                Occupied(mut occupied) => {
                    occupied.get_mut().merge(generated_item);
                }
            }
        }
        for (item_id, member_functions) in other.member_functions {
            self.member_functions.entry(item_id).or_default().extend(member_functions);
        }
        for (item_id, free_methods) in other.free_functions {
            self.free_functions.entry(item_id).or_default().extend(free_methods);
        }
        self.thunks.extend(other.thunks);
        self.assertions.extend(other.assertions);
        self.cc_details.extend(other.cc_details);
        self.features |= other.features;
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

/// Returns the list of features required to use the item which are not yet
/// enabled.
///
/// If the item doesn't have a defining target, the return value is meaningless,
/// and bindings will always be generated.
///
/// If the item does have a defining target, and it doesn't enable the specified
/// features, then bindings are suppressed for this item.
pub fn missing_feature_descriptions(db: &BindingsGenerator, item: &Item) -> Result<Vec<String>> {
    let mut missing_features = vec![];

    let ir = &db.ir();

    struct TargetAndFeatures {
        target: BazelLabel,
        features: flagset::FlagSet<CrubitFeature>,
    }
    let defining_and_owning_target: Vec<TargetAndFeatures> = db
        .defining_target(item.id())
        .into_iter()
        .chain(item.owning_target())
        .map(|target| TargetAndFeatures { features: ir.target_crubit_features(&target), target })
        .collect();

    let have_feature = |feature: CrubitFeature| -> bool {
        // We refuse to generate bindings if either the definition of an item, or
        // instantiation (if it is a template) of an item are in a translation unit
        // which doesn't have the required Crubit features.
        for TargetAndFeatures { target, .. } in &defining_and_owning_target {
            let enabled_features = ir.target_crubit_features(target);
            if !enabled_features.contains(feature) {
                return false;
            }
        }
        true
    };

    let missing_features_of_type = |rs_type_kind: &RsTypeKind| -> Option<Vec<String>> {
        for TargetAndFeatures { target, features } in &defining_and_owning_target {
            let descriptions = rs_type_kind.missing_feature_descriptions_of_type(target, *features);
            if !descriptions.is_empty() {
                return Some(descriptions);
            }
        }
        None
    };

    let missing_features_of_cc_type = |cc_type: ir::CcType| -> Option<Vec<String>> {
        match db.rs_type_kind(cc_type) {
            Ok(rs_type_kind) => missing_features_of_type(&rs_type_kind),
            Err(e) => Some(vec![e.to_string()]),
        }
    };

    let join_missing_with_context = |context: &str, missing: &[String]| -> String {
        let desc = missing.join("\n").replace('\n', "\n  ");
        format!("{context}:\n  {desc}")
    };

    if !have_feature(CrubitFeature::Experimental)
        && let Some(unknown_attr) = item.unknown_attr()
    {
        missing_features.push(format!(
            "crubit.rs/errors/unknown_attribute: unknown attribute(s): {unknown_attr}"
        ));
    }

    match item {
        Item::Constant(_)
        | Item::Comment { .. }
        | Item::GlobalVar(_)
        | Item::Namespace(_)
        | Item::UnsupportedItem(..)
        | Item::UseMod { .. } => {}

        Item::Func(func) => {
            if func.rs_name == UnqualifiedIdentifier::Destructor {
                // We support destructors in supported even though they use some features we
                // don't generally support with that feature set, because in this
                // particular case, it's safe.
                if !have_feature(CrubitFeature::Types) {
                    missing_features.push("destructors".to_string());
                }
            } else {
                for param in &func.params {
                    if let Some(missing) = missing_features_of_cc_type(param.type_.clone()) {
                        missing_features.push(join_missing_with_context(
                            &format!(
                                "Unsupported parameter type `{} {}`",
                                db.cc_type_debug_name(&param.type_),
                                param.identifier
                            ),
                            &missing,
                        ));
                    }
                }
                if let Some(missing) = missing_features_of_cc_type(func.return_type.clone()) {
                    missing_features.push(join_missing_with_context(
                        &format!(
                            "Unsupported return type `{}`",
                            db.cc_type_debug_name(&func.return_type)
                        ),
                        &missing,
                    ));
                }
                if !have_feature(CrubitFeature::Experimental) {
                    if !func.has_c_calling_convention {
                        missing_features.push("non-C calling convention".to_string());
                    }
                    if func.is_variadic {
                        missing_features.push("variadic function".to_string());
                    }
                    if func.is_noreturn {
                        missing_features.push("[[noreturn]] attribute".to_string());
                    }
                    for param in &func.params {
                        if let Some(unknown_attr) = &param.unknown_attr {
                            missing_features.push(format!(
                                "crubit.rs/errors/unknown_attribute: param {param} has unknown attribute(s): {unknown_attr}",
                                param = &param.identifier.identifier
                            ));
                        }
                    }
                }
            }
        }
        Item::Record(_) | Item::TypeAlias(_) | Item::Enum(_) | Item::ExistingRustType(_) => {
            // We use from_item_raw here because missing_feature_descriptions is itself called
            // by `BindingsGenerator::rs_type_kind()` in order to decide if it should return
            // an error.
            if let Some(missing) = missing_features_of_type(&RsTypeKind::from_item_raw(
                db,
                item.clone(),
                &LifetimeOptions { is_return_type: true, ..LifetimeOptions::default() },
                /*template_args=*/ &None,
                /*lifetimes=*/ &[],
            )?) {
                missing_features.extend(missing);
            }
        }
        Item::IncompleteRecord(_) => {
            if !have_feature(CrubitFeature::Wrapper) {
                missing_features.push("incomplete type".to_string());
            }
        }
    }
    Ok(missing_features)
}

/// Visibility of an item.
///
/// Generally speaking, if an error occurs (e.g. a bindings doesn't exist), then
/// the way to "keep going" to catch more errors is to pretend that the missing
/// item is `Public`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum Visibility {
    /// The item has `pub` visibility.
    #[default]
    Public,
    /// The item has `pub(crate)` visibility.
    PubCrate,
}

impl Visibility {
    /// Returns the least of two visibilities.
    #[must_use]
    pub fn or(self, other: Visibility) -> Visibility {
        match (self, other) {
            (Visibility::Public, Visibility::Public) => Visibility::Public,
            (Visibility::Public, Visibility::PubCrate)
            | (Visibility::PubCrate, Visibility::Public)
            | (Visibility::PubCrate, Visibility::PubCrate) => Visibility::PubCrate,
        }
    }
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
        missing_features: Vec<String>,
    },
    DependencyFailed {
        type_name: String,
        reason: String,
    },
    LeadingDunder {
        name: String,
    },
    Visibility(Error),
    /// This is directly unsupported.
    Unsupported(Error),
    /// This item's parent was a record, but no nested items module could be generated because there
    /// were other records with nested items whose nested items module mapped to the same name.
    ParentModuleNameNotUnique {
        conflicting_name: String,
        /// Invariant: more than 1 element.
        parent_names_that_map_to_same_name: Vec<String>,
    },
    /// This item's parent was a record, but no nested items module could be generated because there
    /// were other items that occupied the name in that parent's namespace. For example, a struct
    /// called `foo` would not be able to receive nested items because its nested module name would
    /// also be `foo`. `Foo` would be fine though, because it gets `foo`.
    ParentModuleNameOverwritten {
        conflicting_name: Rc<str>,
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
            NoBindingsReason::MissingRequiredFeatures { missing_features } => {
                anyhow!(missing_features.join("\n"))
            }
            NoBindingsReason::DependencyFailed { type_name, reason } => {
                anyhow!("depends on `{type_name}` which cannot be bound because {reason}")
            }
            NoBindingsReason::LeadingDunder { name } => {
                anyhow!("Skipping generating bindings for '{name}' because it has a leading `__`")
            }
            NoBindingsReason::Visibility(error) | NoBindingsReason::Unsupported(error) => error,
            NoBindingsReason::ParentModuleNameNotUnique {
                conflicting_name,
                parent_names_that_map_to_same_name,
            } => {
                anyhow!(
                    "crubit.rs/errors/nested_type: records {parent_names_that_map_to_same_name:?} all have nested items, but all map to the same nested module name: `{conflicting_name}`",
                )
            }
            NoBindingsReason::ParentModuleNameOverwritten { conflicting_name } => {
                anyhow!(
                    "crubit.rs/errors/nested_type: parent record has nested items, but the module to contain them could not be generated because another item named `{conflicting_name}` already exists",
                )
            }
        }
    }
}

/// The thing that a name resolves to.
#[derive(Debug)]
pub enum ResolvedName {
    Namespace {
        /// Namespaces with the same canonical namespace id are coalesced together.
        canonical_namespace_id: ItemId,
    },
    /// An item that is explicitly generated (as opposed to RecordNestedItems, which is implicitly
    /// generated).
    ExplicitItem(ItemId),
    /// An item that is in the value namespace (functions, constants, etc.).
    ValueItem(ItemId),
    /// The module that is generated to hold the nested items of a record.
    /// If there's more than one, that means the multiple records with nested items, when
    /// snake_cased, map to the same name. For now, this is treated as an error, but we may want to
    /// behave differently in the future.
    RecordNestedItems {
        /// Invariant: at least one item.
        parent_records_that_map_to_this_name: Vec<ItemId>,
    },
}

impl ResolvedName {
    /// If two names resolve to different ResolvedNames, try and coalesce them together.
    /// For example, namespaces that correspond to the same canonical namespace id can be flattened.
    pub fn coalesce(&mut self, other: ResolvedName) -> Result<()> {
        match (self, other) {
            // RecordNestedItems coalesce together. Right now, this is just to provide a better
            // error message ("several record nested items modules map to the same name"), but we
            // can later change this to actually coalesce them (may lead to nested name conflicts
            // though) or pick one or something else.
            (
                Self::RecordNestedItems { parent_records_that_map_to_this_name },
                Self::RecordNestedItems {
                    parent_records_that_map_to_this_name:
                        mut other_parent_records_that_map_to_this_name,
                },
            ) => {
                parent_records_that_map_to_this_name
                    .append(&mut other_parent_records_that_map_to_this_name);
                Ok(())
            }
            // RecordNestedItems are always overwritten by other resolved names, because they are
            // implicitly generated and therefore have low priority.
            (_, Self::RecordNestedItems { .. }) => Ok(()),
            (this @ Self::RecordNestedItems { .. }, other) => {
                *this = other;
                Ok(())
            }
            // Namespaces with the same canonical namespace id flatten into a single resolved name.
            (
                Self::Namespace { canonical_namespace_id },
                Self::Namespace { canonical_namespace_id: other_canonical_namespace_id },
            ) => {
                ensure!(
                    *canonical_namespace_id == other_canonical_namespace_id,
                    "multiple namespaces with the same name but differing canonical namespace ids"
                );
                Ok(())
            }
            (Self::ValueItem(_), Self::ValueItem(_)) => Ok(()), // Keep first one
            (Self::ExplicitItem(_), Self::ValueItem(_)) => Ok(()), // Keep ExplicitItem
            (this @ Self::ValueItem(_), other @ Self::ExplicitItem(_)) => {
                *this = other; // Overwrite with ExplicitItem!
                Ok(())
            }
            // Everything else is a conflict, and should never happen.
            _ => bail!("conflicting name occupants"),
        }
    }
}

pub fn generated_items_to_token_stream<'db>(
    generated_items: &HashMap<ItemId, GeneratedItem>,
    db: &'db crate::BindingsGenerator<'db>,
    elements: &[ItemId],
) -> TokenStream {
    let mut tokens = quote! {};
    generated_items_to_tokens(generated_items, db, elements, &mut tokens);
    tokens
}

pub fn integer_constant_to_token_stream(
    db: &crate::BindingsGenerator,
    integer_constant: IntegerConstant,
    underlying_type: &RsTypeKind,
) -> Result<TokenStream> {
    let RsTypeKind::Primitive(primitive) = *underlying_type.unalias() else {
        bail!(
            "integer_constant_to_token_stream called with non-primitive underlying type:\n  {}",
            underlying_type.display(db),
        )
    };
    let IntegerConstant { is_negative, wrapped_value } = integer_constant;
    Ok(if underlying_type.is_bool() {
        if wrapped_value == 0 {
            quote! {false}
        } else {
            quote! {true}
        }
    } else {
        let mut value = if is_negative {
            Literal::i64_unsuffixed(wrapped_value as i64).into_token_stream()
        } else {
            Literal::u64_unsuffixed(wrapped_value).into_token_stream()
        };
        if underlying_type.is_char() {
            value = quote! {
                #value as u8
            };
        }
        match PrimitiveName::from_primitive(primitive) {
            PrimitiveName::NativeType(_) => value,
            PrimitiveName::Ffi11Type(type_name) => {
                // This is a bit of trickery. In order to have a standard way for the compiler to
                // produce ffi_11 values of types which are possibly wrapped depending on the
                // target platform, ffi_11 provides `new_c_int`, `new_c_long`, etc.
                let new_fn_name =
                    Ident::new(&format!("new_{type_name}"), proc_macro2::Span::call_site());
                quote! { ::ffi_11::#new_fn_name(#value) }
            }
        }
    })
}

pub fn generated_items_to_tokens<'db>(
    generated_items: &HashMap<ItemId, GeneratedItem>,
    db: &'db crate::BindingsGenerator<'db>,
    elements: &[ItemId],
    tokens: &mut TokenStream,
) {
    for &id in elements {
        let Some(generated_item) = generated_items.get(&id) else {
            continue;
        };

        match generated_item {
            GeneratedItem::Comment { message } => quote! { __COMMENT__ #message }.to_tokens(tokens),
            GeneratedItem::Enum(enum_item) => enum_item.to_tokens(tokens),
            GeneratedItem::Func(function_tokens) => function_tokens.to_tokens(tokens),
            GeneratedItem::Record(record_item) => {
                let Record {
                    doc_comment_attr,
                    derive_attr,
                    recursively_pinned_attr,
                    must_use_attr,
                    deprecated_attr,
                    align,
                    internally_mutable_unknown_fields,
                    crubit_annotation,
                    visibility,
                    struct_or_union,
                    ident,
                    id,
                    head_padding,
                    field_definitions,
                    implements_send,
                    implements_sync,
                    cxx_impl,
                    incomplete_definition,
                    upcast_impls,
                    display_impl,
                    no_unique_address_accessors,
                    items,
                    nested_items,
                    indirect_functions,
                    delete,
                    owned_ptr_config,
                    member_methods,
                    free_functions,
                    lifetime_params,
                    is_thread_safe,
                    size,
                } = record_item.as_ref();

                let type_param_tokens = if !lifetime_params.is_empty() {
                    quote! { < #( #lifetime_params ),* > }
                } else {
                    quote! {}
                };

                let repr_attrs = std::iter::once(quote! { C }).chain(align.map(|align| {
                    let align = Literal::usize_unsuffixed(align);
                    quote! { align(#align) }
                }));

                // For thread-safe types, generate an opaque UnsafeCell body instead of
                // individual fields. This enables interior mutability, allowing non-const
                // C++ methods to be called through shared references (&self).
                let struct_body = if *is_thread_safe {
                    let size_literal = Literal::usize_unsuffixed(*size);
                    quote! {
                        __opaque: ::core::cell::UnsafeCell<[::core::mem::MaybeUninit<u8>; #size_literal]>,
                    }
                } else {
                    let head_padding = head_padding.map(|n| {
                        let n = Literal::usize_unsuffixed(n);
                        // TODO(b/481405536): Do this unconditionally.
                        if *internally_mutable_unknown_fields {
                            quote! { __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; #n], }
                        } else {
                            quote! { __non_field_data: [::core::mem::MaybeUninit<u8>; #n], }
                        }
                    });
                    let lifetime_markers: Vec<TokenStream> = lifetime_params
                        .iter()
                        .map(|lt| {
                            let field_name = format_ident!("__marker_{}", lt.ident);
                            quote! { #field_name: ::core::marker::PhantomData<& #lt ()> }
                        })
                        .collect();
                    quote! {
                        #head_padding
                        #( #field_definitions )*
                        #( #lifetime_markers )*
                    }
                };

                let send_impl = match implements_send {
                    true => {
                        quote! { unsafe impl #type_param_tokens Send for #ident #type_param_tokens {} }
                    }
                    false => {
                        quote! { impl #type_param_tokens !Send for #ident #type_param_tokens {} }
                    }
                };
                let sync_impl = match implements_sync {
                    true => {
                        quote! { unsafe impl #type_param_tokens Sync for #ident #type_param_tokens {} }
                    }
                    false => {
                        quote! { impl #type_param_tokens !Sync for #ident #type_param_tokens {} }
                    }
                };

                let cxx_impl = match cxx_impl {
                    Some(CxxExternTypeImpl { id, kind }) => quote! {
                        unsafe impl #type_param_tokens ::cxx::ExternType for #ident #type_param_tokens {
                            type Id = ::cxx::type_id!(#id);
                            type Kind = #kind;
                        }
                    },
                    _ => quote! {},
                };

                let no_unique_address_accessors_impl = if !no_unique_address_accessors.is_empty() {
                    Some(quote! {
                        impl #type_param_tokens #ident #type_param_tokens {
                            #( #no_unique_address_accessors )*
                        }
                    })
                } else {
                    None
                };

                let owned_type_def = owned_ptr_config.as_ref().map(|cfg| {
                    let owned_type_name = &cfg.owned_type_name;
                    let drop_meth = &cfg.drop_impl;
                    let doc_comment = format!(
                        "Wrapper for a C++ {} owned by Rust. \n\n Style guide: The C++ type to which this refers should be wrapped in an `Arc` or `Mutex` if it is not already thread-safe. \n\n THIS TYPE REQUIRES A MANUAL DROP IMPLEMENTATION. \n You MUST provide an `impl {} {{ pub fn {}(&mut self) {{ ... }} }}` block in a separate Rust file (e.g., via `additional_rust_srcs`). Failure to do so will result in a compile-time error: `method not found in `{}``.",
                        ident, owned_type_name, drop_meth, owned_type_name
                    );
                    quote! {
                        __NEWLINE__ __NEWLINE__
                        __COMMENT__ "Generated due to CRUBIT_OWNED_POINTEE annotation."
                        #[doc = #doc_comment]
                        #[repr(transparent)]
                        pub struct #owned_type_name(::core::ptr::NonNull<#ident>);

                        impl Drop for #owned_type_name {
                            fn drop(&mut self) {
                                __COMMENT__ "IMPORTANT: The drop method MUST be implemented in a user-written .rs file (e.g., using `additional_rust_srcs`)."
                                __COMMENT__ "Crubit cannot automatically generate the destruction logic for this type."
                                __COMMENT__ "See the struct documentation for more details."
                                self.#drop_meth();
                            }
                        }
                    }
                });

                let member_methods_impl = if !member_methods.is_empty() {
                    Some(quote! {
                        impl #type_param_tokens #ident #type_param_tokens {
                            #( #member_methods )*
                        }
                    })
                } else {
                    None
                };

                quote! {
                    #doc_comment_attr
                    #derive_attr
                    #recursively_pinned_attr
                    #must_use_attr
                    #deprecated_attr
                    #[repr(#(#repr_attrs),*)]
                    #crubit_annotation
                    #visibility #struct_or_union #ident #type_param_tokens {
                        #struct_body
                    }

                    #send_impl
                    #sync_impl
                    #cxx_impl
                    #display_impl

                    #incomplete_definition

                    #no_unique_address_accessors_impl

                    #member_methods_impl

                    #owned_type_def

                    __NEWLINE__ __NEWLINE__
                }
                .to_tokens(tokens);

                generated_items_to_tokens(generated_items, db, items, tokens);

                quote! { #( #indirect_functions __NEWLINE__ __NEWLINE__ )* }.to_tokens(tokens);

                for upcast_impl_or_err in upcast_impls {
                    match upcast_impl_or_err {
                        Ok(UpcastImpl { base_name, derived_name, body }) => {
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
                                        unsafe { #body }
                                    }
                                }
                                __NEWLINE__
                                __NEWLINE__
                            }
                            .to_tokens(tokens);
                        }
                        Err(err) => {
                            quote! {
                                __NEWLINE__
                                __COMMENT__ #err
                                __NEWLINE__
                            }
                            .to_tokens(tokens);
                        }
                    }
                }

                if let Some(DeleteImpl { record_type, thunk_ident, crate_root_path }) = delete {
                    quote! {
                        unsafe impl ::operator::Delete for #record_type {
                            #[inline(always)]
                            unsafe fn delete(p: *mut Self) {
                                unsafe { #crate_root_path::detail::#thunk_ident(p); }
                            }
                        }
                        __NEWLINE__
                        __NEWLINE__
                    }
                    .to_tokens(tokens);
                }
                let record_ir = db.find_decl::<Rc<ir::Record>>(*id).unwrap();
                let module_name = db.record_to_associated_module_name(record_ir.clone()).unwrap();
                if !free_functions.is_empty() || !nested_items.is_empty() {
                    let nested_items_to_tokens =
                        generated_items_to_token_stream(generated_items, db, nested_items);
                    quote! {
                        pub mod #module_name {
                            __NEWLINE__
                            #( #free_functions )*
                            #nested_items_to_tokens
                        }
                    }
                    .to_tokens(tokens);
                }
            }
            GeneratedItem::NonCanonicalNamespace | GeneratedItem::CanonicalNamespace { .. } => {
                // For a given namespace, canonical_namespace_id is not necessarily in this target,
                // meaning it may never be visited if we branch down just from the top level items in
                // this target. To mitigate this issue, we instead agree to pick the last reopened
                // namespace _in this target_ as the representative that gets to generate all the items
                // in the canonical namespace.
                // The reason this occurs is because although Crubit only generates items for this
                // target, Clang looks at all the includes, meaning it can see the same namespace in
                // headers from different targets. The canonical namespace is picked by Clang, resulting
                // in sometimes getting a canonical namespace that's not in our target.
                // We do not have to worry about getting items from other targets though because Crubit
                // only generates items for this target.
                let current_namespace: &Rc<ir::Namespace> =
                    db.find_decl::<Rc<Namespace>>(id).expect("should always be a namespace");
                let is_last_reopened_namespace_in_this_target = db
                    .ir()
                    .is_last_reopened_namespace(id, current_namespace.canonical_namespace_id)
                    .expect("should always be a namespace");

                if !is_last_reopened_namespace_in_this_target {
                    // It is not the representative, so we don't generate any items in order to
                    // avoid generating duplicate bindings.
                    continue;
                }
                // It is the representative, so we generate all the items keyed under the
                // canonical namespace id.

                let Some(GeneratedItem::CanonicalNamespace { items, deprecated_attr }) =
                    generated_items.get(&current_namespace.canonical_namespace_id)
                else {
                    panic!("the entry we generated for the canonical namespace should be a GeneratedItem::CanonicalNamespace");
                };

                let namespace_tokens = generated_items_to_token_stream(generated_items, db, items);

                let canonical_namespace: &Rc<ir::Namespace> = db
                    .find_decl(current_namespace.canonical_namespace_id)
                    .unwrap_or_else(|_| panic!("Namespace canonical_namespace_id {:?} not found as a valid Namespace item.", current_namespace.canonical_namespace_id));
                let name = make_rs_ident(&canonical_namespace.rs_name.identifier);

                quote! {
                    #deprecated_attr
                    pub mod #name {
                        #namespace_tokens
                    }
                    __NEWLINE__
                }
                .to_tokens(tokens);

                if canonical_namespace.is_inline {
                    // TODO(b/308949532): Skip re-export if the canonical module is empty
                    // (transitively).
                    quote! {
                        __HASH_TOKEN__ [allow(unused_imports)]
                        pub use #name::*;
                    }
                    .to_tokens(tokens);
                }
            }
            GeneratedItem::ForwardDeclare { visibility, ident, symbol } => {
                quote! {
                    forward_declare::forward_declare!(
                        #visibility #ident __SPACE__ = __SPACE__ forward_declare::symbol!(#symbol)
                    );
                }
                .to_tokens(tokens);
            }
            GeneratedItem::UseMod { path, mod_name } => quote! {
                #[path = #path]
                mod #mod_name;
                __HASH_TOKEN__ [allow(unused_imports)]
                pub use #mod_name::*;
            }
            .to_tokens(tokens),
            GeneratedItem::Constant { ident, type_tokens, value, deprecated_attr } => quote! {
                #deprecated_attr
                pub const #ident: #type_tokens = #value;
            }
            .to_tokens(tokens),
            GeneratedItem::GlobalVar {
                link_name,
                visibility,
                is_mut,
                ident,
                type_tokens,
                deprecated_attr,
            } => {
                let link_name_attr =
                    link_name.as_deref().map(|link_name| quote! { #[link_name = #link_name] });
                let mut_kw = if *is_mut { Some(quote! { mut }) } else { None };
                quote! {
                    unsafe extern "C" {
                        #link_name_attr
                        #deprecated_attr
                        #visibility static #mut_kw #ident: #type_tokens;
                    }
                }
                .to_tokens(tokens);
            }
            GeneratedItem::TypeAlias {
                doc_comment,
                visibility,
                ident,
                underlying_type,
                underlying_nested_module_path,
                deprecated_attr,
                lifetime_params,
            } => {
                let type_param_tokens = if !lifetime_params.is_empty() {
                    quote! { < #( #lifetime_params ),* > }
                } else {
                    quote! {}
                };

                quote! {
                    #doc_comment
                    #deprecated_attr
                    #visibility type #ident #type_param_tokens = #underlying_type;
                }
                .to_tokens(tokens);

                // If we alias a record with nested types, underlying_nested_module_name will be
                // Some. In this case, we need to re-export the underlying module with the snake
                // case name of the alias.
                if let Some(underlying_nested_module_path) = underlying_nested_module_path {
                    let aliased_nested_module_name =
                        make_rs_ident(&ident.to_string().to_snake_case());

                    if &aliased_nested_module_name != ident {
                        quote! {
                            #visibility use #underlying_nested_module_path as #aliased_nested_module_name;
                        }
                        .to_tokens(tokens);
                    }
                }
            }
        }
        quote! {
            __NEWLINE__
            __NEWLINE__
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub enum GeneratedItem {
    Comment {
        message: Rc<str>,
    },
    Constant {
        ident: Ident,
        type_tokens: TokenStream,
        value: TokenStream,
        deprecated_attr: Option<DeprecatedAttr>,
    },
    Enum(TokenStream),
    Func(TokenStream),
    // Box used to mitigate disproportionaly large enum variant lint
    Record(Box<Record>),
    NonCanonicalNamespace,
    CanonicalNamespace {
        /// List of all the items from all the namespaces
        items: Vec<ItemId>,
        deprecated_attr: Option<DeprecatedAttr>,
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
        deprecated_attr: Option<DeprecatedAttr>,
    },
    TypeAlias {
        doc_comment: Option<DocCommentAttr>,
        visibility: Visibility,
        ident: Ident,
        underlying_type: TokenStream,
        underlying_nested_module_path: Option<TokenStream>,
        deprecated_attr: Option<DeprecatedAttr>,
        lifetime_params: Vec<syn::Lifetime>,
    },
}

impl GeneratedItem {
    fn merge(&mut self, other: GeneratedItem) {
        fn merge_deprecated_text(s1: &str, s2: &str) -> String {
            match (s1, s2) {
                ("", s2) => s2.to_string(),
                (s1, "") => s1.to_string(),
                (s1, s2) => format!("{}, {}", s1, s2),
            }
        }
        match (self, other) {
            (
                GeneratedItem::CanonicalNamespace { items, deprecated_attr },
                GeneratedItem::CanonicalNamespace {
                    items: other_items,
                    deprecated_attr: other_deprecated_attr,
                },
            ) => {
                items.extend(other_items);
                match (&deprecated_attr, &other_deprecated_attr) {
                    (Some(a), Some(b)) => {
                        if a.0 != b.0 {
                            *deprecated_attr =
                                Some(DeprecatedAttr(merge_deprecated_text(&a.0, &b.0).into()))
                        }
                    }
                    (_, None) => (),
                    (None, _) => *deprecated_attr = other_deprecated_attr,
                };
            }
            (
                GeneratedItem::Comment { message },
                GeneratedItem::Comment { message: other_message },
            ) => {
                assert_eq!(message.as_ref(), other_message.as_ref());
            }
            (this, other) => {
                // The bindings are not mergable, this should never happen.
                unreachable!("Two ApiSnippets generated bindings for the same ItemId that's not a canonical namespace: {this:#?} and {other:#?}");
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct OwnedPtrConfig {
    pub owned_type_name: Ident,
    pub drop_impl: Ident,
}

#[derive(Clone, Debug)]
pub struct Record {
    pub doc_comment_attr: Option<DocCommentAttr>,
    pub derive_attr: DeriveAttr,
    pub recursively_pinned_attr: Option<RecursivelyPinnedAttr>,
    pub must_use_attr: Option<MustUseAttr>,
    pub deprecated_attr: Option<DeprecatedAttr>,
    pub align: Option<usize>,
    pub internally_mutable_unknown_fields: bool,
    pub crubit_annotation: DocCommentAttr,
    pub visibility: Visibility,
    pub struct_or_union: StructOrUnion,
    pub ident: Ident,
    pub id: ItemId,
    pub head_padding: Option<usize>,
    pub field_definitions: Vec<FieldDefinition>,
    pub implements_send: bool,
    pub implements_sync: bool,
    pub cxx_impl: Option<CxxExternTypeImpl>,
    pub incomplete_definition: Option<TokenStream>,
    pub upcast_impls: Vec<Result<UpcastImpl, String>>,
    pub display_impl: Option<DisplayImpl>,
    pub no_unique_address_accessors: Vec<NoUniqueAddressAccessor>,
    pub items: Vec<ItemId>,
    pub nested_items: Vec<ItemId>,
    /// Functions that get attached either by a trait or from a base class.
    pub indirect_functions: Vec<TokenStream>,
    pub delete: Option<DeleteImpl>,
    /// The owning wrapper type configuration when the type was annotated with CRUBIT_OWNED_POINTEE.
    pub owned_ptr_config: Option<OwnedPtrConfig>,
    pub member_methods: Vec<TokenStream>,
    pub free_functions: Vec<TokenStream>,
    pub lifetime_params: Vec<syn::Lifetime>,
    /// Whether this type is annotated as thread-safe (CRUBIT_THREAD_SAFE).
    pub is_thread_safe: bool,
    /// The size of the type in bytes (needed for opaque UnsafeCell wrapping).
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct UpcastImpl {
    pub base_name: TokenStream,
    pub derived_name: TokenStream,
    pub body: UpcastImplBody,
}

#[derive(Clone, Debug)]
pub struct DeleteImpl {
    pub record_type: TokenStream,
    pub thunk_ident: Ident,
    pub crate_root_path: TokenStream,
}

#[derive(Clone, Debug)]
pub enum UpcastImplBody {
    PointerOffset { offset: i64 },
    CastThunk { crate_root_path: Option<Ident>, cast_fn_name: Ident },
}

#[derive(Clone, Debug)]
pub struct DisplayImpl {
    pub type_name: TokenStream,
    pub fmt_fn_name: Ident,
}

impl ToTokens for DisplayImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { type_name, fmt_fn_name } = self;
        quote! {
            impl ::core::fmt::Display for #type_name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = ::lossy_formatter::LossyFormatter::new(f);
                    if unsafe { crate::detail::#fmt_fn_name(self, &mut f) } {
                        ::core::result::Result::Ok(())
                    } else {
                        ::core::result::Result::Err(::core::fmt::Error)
                    }
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocCommentAttr(pub Rc<str>);

impl ToTokens for DocCommentAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(doc_comment) = self;
        quote! { #[doc = #doc_comment] }.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct DeriveAttr(pub Vec<TokenStream>);

impl ToTokens for DeriveAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(derives) = self;
        if !derives.is_empty() {
            quote! { #[derive(#(#derives),*)] }.to_tokens(tokens);
        }
    }
}

#[derive(Clone, Debug)]
pub struct RecursivelyPinnedAttr {
    pub pinned_drop: bool,
}

impl ToTokens for RecursivelyPinnedAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.pinned_drop {
            true => quote! { #[::ctor::recursively_pinned(PinnedDrop)] },
            false => quote! { #[::ctor::recursively_pinned] },
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct MustUseAttr(pub Rc<str>);

impl ToTokens for MustUseAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0.as_ref() {
            "" => quote! { #[must_use] },
            message => quote! { #[must_use = #message] },
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct DeprecatedAttr(pub Rc<str>);

impl ToTokens for DeprecatedAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0.as_ref() {
            "" => quote! { #[deprecated] },
            message => quote! { #[deprecated = #message] },
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct CfiEncodingAttr(pub Rc<str>);

impl ToTokens for CfiEncodingAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.0.is_empty() {
            let encoding = self.0.as_ref();
            quote! { #[cfi_encoding = #encoding] }.to_tokens(tokens);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CxxExternTypeImpl {
    pub id: Rc<str>,
    pub kind: CxxKind,
}

#[derive(Copy, Clone, Debug)]
pub enum CxxKind {
    Opaque,
    Trivial,
}

impl ToTokens for CxxKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CxxKind::Opaque => quote! {::cxx::kind::Opaque},
            CxxKind::Trivial => quote! {::cxx::kind::Trivial},
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub enum StructOrUnion {
    Struct,
    Union,
}

impl ToTokens for StructOrUnion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            StructOrUnion::Struct => quote! { struct },
            StructOrUnion::Union => quote! { union },
        }
        .to_tokens(tokens);
    }
}

/// Quotes as the type of a type-less, unaligned block of memory that can hold a
/// specified number of bits, rounded up to the next multiple of 8.
#[derive(Copy, Clone, Debug)]
pub struct BitPadding {
    pub size: NonZeroUsize,
    pub internally_mutable: bool,
}

impl BitPadding {
    fn padding_size_in_bytes(self) -> usize {
        self.size.get().div_ceil(8)
    }
}

impl ToTokens for BitPadding {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let n = Literal::usize_unsuffixed(self.padding_size_in_bytes());
        if self.internally_mutable {
            quote! { [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; #n] }.to_tokens(tokens);
        } else {
            quote! { [::core::mem::MaybeUninit<u8>; #n] }.to_tokens(tokens);
        }
    }
}

#[derive(Clone, Debug)]
pub struct BitfieldComment {
    pub field_name: Option<Rc<str>>,
    pub bits: NonZeroUsize,
}

impl ToTokens for BitfieldComment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = format!("{} : {} bits", self.field_name.as_deref().unwrap_or(""), self.bits.get());
        quote! { __COMMENT__ #s }.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub enum FieldDefinition {
    Bitfield {
        field_index: usize,
        desc: Vec<BitfieldComment>,
        padding: Option<BitPadding>,
        bits: BitPadding,
    },
    Field {
        field_index: usize,
        padding: Option<BitPadding>,
        doc_comment: Option<DocCommentAttr>,
        deprecated_attr: Option<DeprecatedAttr>,
        visibility: Visibility,
        ident: Ident,
        field_type: FieldType,
    },
}

impl ToTokens for FieldDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FieldDefinition::Bitfield { field_index, desc, padding, bits } => {
                let padding_field = padding.map(|padding| {
                    let padding_name =
                        syn::parse_str::<Ident>(&format!("__padding{field_index}")).unwrap();
                    quote! { #padding_name: #padding, }
                });
                let bitfield_name =
                    syn::parse_str::<Ident>(&format!("__bitfields{field_index}")).unwrap();
                quote! {
                    __NEWLINE__ #( #desc )*
                    #padding_field
                    #bitfield_name: #bits,
                }
                .to_tokens(tokens);
            }
            FieldDefinition::Field {
                field_index,
                padding,
                doc_comment,
                deprecated_attr,
                visibility,
                ident,
                field_type,
            } => {
                let padding_field = padding.map(|padding| {
                    let padding_name =
                        syn::parse_str::<Ident>(&format!("__padding{field_index}")).unwrap();
                    quote! { #padding_name: #padding, }
                });
                quote! {
                    #padding_field
                    #doc_comment
                    #deprecated_attr
                    #visibility #ident: #field_type,
                }
                .to_tokens(tokens);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Erased(BitPadding),
    Type { needs_manually_drop: bool, needs_cell: bool, ty: TokenStream },
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FieldType::Erased(padding) => padding.to_tokens(tokens),
            FieldType::Type { needs_manually_drop, needs_cell, ty } => {
                let mut ty = ty.clone();
                if *needs_manually_drop {
                    ty = quote! { ::core::mem::ManuallyDrop<#ty> };
                }
                if *needs_cell {
                    ty = quote! { ::core::cell::Cell<#ty> };
                }
                ty.to_tokens(tokens)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoUniqueAddressAccessor {
    pub doc_comment: Option<DocCommentAttr>,
    pub field: Ident,
    pub type_: TokenStream,
    pub byte_offset: usize,
}

impl ToTokens for NoUniqueAddressAccessor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { doc_comment, field, type_, byte_offset } = self;
        let byte_offset = Literal::usize_unsuffixed(*byte_offset);

        // SAFETY: even if there is a named field in Rust for this subobject, it is not
        // safe to just cast the pointer. A `struct S {[[no_unique_address]] A a;
        // char b};` will be represented in Rust using a too-short field `a` (e.g.
        // with `[MaybeUninit<u8>; 3]`, where the trailing fourth byte is actually
        // `b`). We cannot cast this to something wider, which includes `b`, even
        // though the `a` object does in fact include `b` in C++. This is Rust, and
        // these are distinct object allocations. We don't have provenance.
        //
        // However, we can start from the pointer to **S** and perform pointer
        // arithmetic on it to get a correctly-sized `A` reference. This is
        // equivalent to transmuting the type to one where the potentially-overlapping
        // subobject exists, but the fields next to it, which it overlaps, do not.
        // As if it were `struct S {A a;};`. However, we do not use transmutes, and
        // instead reimplement field access using pointer arithmetic.
        //
        // The resulting pointer is valid and correctly aligned, and does not violate
        // provenance. It also does not result in mutable aliasing, because this
        // borrows `self`, not just `a`.
        quote! {
            #doc_comment
            pub fn #field(&self) -> &#type_ {
                unsafe {
                    let ptr = (self as *const Self as *const u8).offset(#byte_offset);
                    &*(ptr as *const #type_)
                }
            }
        }
        .to_tokens(tokens);
    }
}

flagset::flags! {
    #[allow(non_camel_case_types)]
    pub enum Feature: u32 {
        // <internal link> start
        allocator_api,
        arbitrary_self_types,
        cfg_sanitize,
        cfi_encoding,
        custom_inner_attributes,
        impl_trait_in_assoc_type,
        negative_impls,
        // <internal link> end
    }
}

impl ToTokens for Feature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Feature::allocator_api => quote! { allocator_api },
            Feature::arbitrary_self_types => quote! { arbitrary_self_types },
            Feature::cfg_sanitize => quote! { cfg_sanitize },
            Feature::cfi_encoding => quote! { cfi_encoding },
            Feature::custom_inner_attributes => quote! { custom_inner_attributes },
            Feature::impl_trait_in_assoc_type => quote! { impl_trait_in_assoc_type },
            Feature::negative_impls => quote! { negative_impls },
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
    /// Generates a thunk for formatting in C++.
    Fmt { fmt_fn_name: Ident, param_type: TokenStream },
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

impl Thunk {
    pub fn name(&self) -> &proc_macro2::Ident {
        match self {
            Thunk::Upcast { cast_fn_name, .. } => cast_fn_name,
            Thunk::Fmt { fmt_fn_name, .. } => fmt_fn_name,
            Thunk::Function { thunk_ident, .. } => thunk_ident,
        }
    }
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
            Thunk::Fmt { fmt_fn_name, param_type } => {
                quote! {
                    pub(crate) unsafe fn #fmt_fn_name(
                        value: &#param_type,
                        formatter: &mut ::lossy_formatter::LossyFormatter) -> bool;
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
    Upcast {
        base_cc_name: TokenStream,
        cast_fn_name: Ident,
        derived_cc_name: TokenStream,
    },
    /// A function that formats in C++.
    Fmt {
        fmt_fn_name: Ident,
        param_type: TokenStream,
    },
    /// A function that implements a Rust function thunk.
    Function {
        return_type_name: TokenStream,
        thunk_ident: Ident,
        param_types: Vec<TokenStream>,
        param_idents: Vec<Ident>,
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
    FunctionTypeAssertion {
        implementation_function: TokenStream,
        cc_function_type: TokenStream,
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
            ThunkImpl::Fmt { fmt_fn_name, param_type } => {
                quote! {
                    extern "C" bool #fmt_fn_name(
                        const #param_type& value, ::lossy_formatter::LossyFormatter& formatter) {
                        return ::crubit::Fmt(value, formatter);
                    }
                }
                .to_tokens(tokens);
            }
            ThunkImpl::Function {
                return_type_name,
                thunk_ident,
                param_types,
                param_idents,
                conversion_stmts,
                return_stmt,
            } => {
                quote! {
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
            ThunkImpl::FunctionTypeAssertion { implementation_function, cc_function_type } => {
                quote! {
                    static_assert( ( #cc_function_type ) & #implementation_function);
                }
                .to_tokens(tokens);
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
    pub includes: CppIncludes,
    // The "pragma clang diagnostic push/pop" is automatically inserted around the thunks.
    pub dyn_callable_cpp_decls: TokenStream,
    pub thunks: Vec<ThunkImpl>,
}

impl ToTokens for CppDetails {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let CppDetails { includes, dyn_callable_cpp_decls, thunks } = self;
        quote! {
            #includes
            __NEWLINE__
            __HASH_TOKEN__ pragma clang diagnostic push __NEWLINE__
            // Disable Clang thread-safety-analysis warnings that would otherwise
            // complain about thunks that call mutex locking functions in an unpaired way.
            __HASH_TOKEN__ pragma clang diagnostic ignored "-Wthread-safety-analysis" __NEWLINE__ __NEWLINE__

            #dyn_callable_cpp_decls
            #( #thunks __NEWLINE__ __NEWLINE__ )*

            __HASH_TOKEN__ pragma clang diagnostic pop __NEWLINE__
        }
        .to_tokens(tokens);
    }
}
