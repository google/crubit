// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]
//! Vocabulary types and code generation functions for generating Rust code.

use crate::code_snippet::{Feature, Visibility};
use crate::BindingsGenerator;
use arc_anyhow::Result;
use code_gen_utils::make_rs_ident;
use code_gen_utils::NamespaceQualifier;
use crubit_abi_type::FullyQualifiedPath;
use crubit_feature::CrubitFeature;
use error_report::{anyhow, bail, ensure};
use flagset::FlagSet;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashSet;
use std::rc::Rc;
use token_stream_printer::write_unformatted_tokens;

const SLICE_REF_NAME_RS: &str = "&[]";

/// A struct with information associated with the formatted Rust code snippet.
#[derive(Clone, Debug)]
pub struct RsSnippet {
    pub tokens: TokenStream,
    // The Rust features that are needed for `tokens` to work.
    pub features: FlagSet<Feature>,
}

impl RsSnippet {
    /// Convenience function to initialize RsSnippet with empty `features`.
    pub fn new(tokens: TokenStream) -> RsSnippet {
        RsSnippet { tokens, features: FlagSet::empty() }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mutability {
    Const,
    Mut,
}

impl Mutability {
    pub fn is_const(self) -> bool {
        match self {
            Mutability::Const => true,
            Mutability::Mut => false,
        }
    }

    pub fn format_for_pointer(self) -> TokenStream {
        match self {
            Mutability::Mut => quote! {mut},
            Mutability::Const => quote! {const},
        }
    }

    pub fn format_for_reference(self) -> TokenStream {
        match self {
            Mutability::Mut => quote! {mut},
            Mutability::Const => quote! {},
        }
    }
}

/// Either a named lifetime, or the magic `'_` elided lifetime.
///
/// Warning: elided lifetimes are not always valid, and sometimes named
/// lifetimes are required. In particular, this should never be used for
/// output lifetimes.
///
/// However, because output lifetimes are never elided, a lifetime that only
/// occurs in a single input position can always be elided.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Lifetime(pub Rc<str>);

impl From<&ir::LifetimeName> for Lifetime {
    fn from(lifetime_name: &ir::LifetimeName) -> Self {
        Lifetime(lifetime_name.name.clone())
    }
}

impl Lifetime {
    pub fn new(name: &str) -> Self {
        Lifetime(Rc::from(name))
    }

    pub fn elided() -> Self {
        Lifetime(Rc::from("_"))
    }

    pub fn is_elided(&self) -> bool {
        &*self.0 == "_"
    }

    /// Formats a lifetime for use as a reference lifetime parameter.
    ///
    /// In this case, elided lifetimes are empty.
    pub fn format_for_reference(&self) -> TokenStream {
        match &*self.0 {
            "_" => quote! {},
            _ => quote! {#self},
        }
    }
}

/// Formats a lifetime for use anywhere.
///
/// For the specific context of references, prefer `format_for_reference`, as it
/// gives a more idiomatic formatting for elided lifetimes.
impl ToTokens for Lifetime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(name) = self;
        let lifetime = syn::Lifetime::new(&format!("'{name}"), proc_macro2::Span::call_site());
        lifetime.to_tokens(tokens);
    }
}

/// Qualified path from the root of the crate to the module containing the type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CratePath {
    /// `Some("other_crate")` or `None` for paths within the current crate.
    crate_ident: Option<Ident>,

    crate_root_path: NamespaceQualifier,
    namespace_qualifier: NamespaceQualifier,
}

impl CratePath {
    pub fn new(
        ir: &IR,
        namespace_qualifier: NamespaceQualifier,
        crate_ident: Option<Ident>,
    ) -> CratePath {
        let crate_root_path = NamespaceQualifier::new(ir.crate_root_path());
        CratePath { crate_ident, crate_root_path, namespace_qualifier }
    }

    pub fn to_fully_qualified_path(&self, item: Ident) -> FullyQualifiedPath {
        let crate_ident = self
            .crate_ident
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Ident::new("crate", proc_macro2::Span::call_site()));
        FullyQualifiedPath {
            start_with_colon2: self.crate_ident.is_some(),
            parts: std::iter::once(crate_ident)
                .chain(self.crate_root_path.parts_with_snake_case_record_names())
                .chain(self.namespace_qualifier.parts_with_snake_case_record_names())
                .chain(std::iter::once(item))
                .collect(),
        }
    }
}

impl ToTokens for CratePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ident = match self.crate_ident.as_ref() {
            None => quote! { crate },
            Some(ident) => quote! { #ident },
        };
        let crate_root_path = self.crate_root_path.format_for_rs();
        let namespace_qualifier = self.namespace_qualifier.format_for_rs();
        quote! { #crate_ident :: #crate_root_path #namespace_qualifier }.to_tokens(tokens)
    }
}

pub fn unique_lifetimes<'a>(
    types: impl IntoIterator<Item = &'a RsTypeKind> + 'a,
) -> impl Iterator<Item = Lifetime> + 'a {
    let mut unordered_lifetimes = HashSet::new();
    types
        .into_iter()
        .flat_map(|ty| ty.lifetimes())
        .filter(|lifetime| !lifetime.is_elided())
        .filter(move |lifetime| unordered_lifetimes.insert(lifetime.clone()))
}

pub fn format_generic_params<'a, T: ToTokens>(
    lifetimes: impl IntoIterator<Item = &'a Lifetime>,
    types: impl IntoIterator<Item = T>,
) -> TokenStream {
    let mut lifetimes = lifetimes.into_iter().filter(|lifetime| &*lifetime.0 != "_").peekable();
    let mut types = types.into_iter().peekable();
    if types.peek().is_none() {
        if lifetimes.peek().is_none() {
            quote! {}
        } else {
            quote! { < #( #lifetimes ),* > }
        }
    } else {
        // Note: the comma is inside the lifetimes glob because a trailing comma is
        // needed if there are types that follow.
        quote! { < #( #lifetimes, )* #( #types ),*> }
    }
}

pub fn format_generic_params_replacing_by_self<'a>(
    db: &dyn BindingsGenerator,
    types: impl IntoIterator<Item = &'a RsTypeKind>,
    trait_record: Option<&Record>,
) -> TokenStream {
    format_generic_params(
        [],
        types.into_iter().map(|ty| ty.to_token_stream_replacing_by_self(db, trait_record)),
    )
}

// TODO(jeanpierreda): These functions are at a weird level of abstraction (using
// ir::Record). It's possible that, instead, we should just ask "does the
// RsTypeKind implement clone" (etc.).
//
// Otherwise, these functions should be moved into a separate module.

pub fn should_derive_clone(record: &Record) -> bool {
    match record.trait_derives.clone {
        TraitImplPolarity::Positive => true,
        TraitImplPolarity::Negative => false,
        TraitImplPolarity::None => {
            if record.is_union() {
                // `union`s (unlike `struct`s) should only derive `Clone` if they are `Copy`.
                should_derive_copy(record)
            } else {
                record.is_unpin()
                    && record.copy_constructor == SpecialMemberFunc::Trivial
                    && check_by_value(record).is_ok()
            }
        }
    }
}

pub fn should_derive_copy(record: &Record) -> bool {
    match record.trait_derives.copy {
        TraitImplPolarity::Positive => true,
        TraitImplPolarity::Negative => false,
        TraitImplPolarity::None => {
            record.is_unpin()
                && record.copy_constructor == SpecialMemberFunc::Trivial
                && record.destructor == SpecialMemberFunc::Trivial
                && check_by_value(record).is_ok()
                && record.trait_derives.clone != TraitImplPolarity::Negative
        }
    }
}

pub fn check_by_value(record: &Record) -> Result<()> {
    ensure!(
        record.destructor != SpecialMemberFunc::Unavailable,
        "Can't directly construct values of type `{}` as it has a non-public or deleted destructor",
        record.cc_name
    );
    ensure!(
        !record.is_abstract,
        "Can't directly construct values of type `{}`: it is abstract",
        record.cc_name
    );
    Ok(())
}

/// Location where a type is used.
// TODO: Merge with `TypeLocation` in the other direction.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TypeLocation {
    FnReturn,
    FnParam,
    Other,
}

/// Options for how lifetimes can be elided in function parameters.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ElisionOptions {
    /// If true, references will be elided. This option is only set for select function parameters
    /// and return types.
    pub elide_references: bool,

    /// Are there any reference parameters to the function whose return type we are interested in?
    pub have_reference_param: bool,

    pub is_return_type: bool,
}

/// A type with template type arguments that has a uniform representation regardless of `T` and
/// should be mapped to a handwritten Rust type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UniformReprTemplateType {
    /// std::vector<T, std::allocator<T>>
    StdVector {
        element_type: RsTypeKind,
    },
    /// std::unique_ptr<T, std::default_delete<T>>
    StdUniquePtr {
        element_type: RsTypeKind,
    },
    AbslSpan {
        is_const: bool,
        include_lifetime: bool,
        element_type: RsTypeKind,
    },
}

impl UniformReprTemplateType {
    /// Returns the `UniformReprTemplateType` for a `TemplateSpecialization`.
    /// Returns an error if the template arguments (if any) fail to db.rs_type_kind(T).
    /// Returns none if the template specialization is not for a known type corresponding with
    /// one of `UniformReprTemplateType`s variants.
    fn new(
        db: &dyn BindingsGenerator,
        template_specialization: Option<&TemplateSpecialization>,
        have_reference_param: bool,
        is_return_type: bool,
    ) -> Result<Option<Rc<Self>>> {
        let Some(template_specialization) = template_specialization else {
            return Ok(None);
        };

        let mut type_args = template_specialization
            .template_args
            .iter()
            .map(|arg| {
                let arg_type = match &arg.type_ {
                    Ok(arg_type) => arg_type.clone(),
                    Err(e) => bail!("{e}"),
                };
                // Importantly, `is_return_type` is not propagated through inner types.
                let arg_type_kind = db.rs_type_kind(arg_type)?;
                ensure!(
                    !arg_type_kind.is_bridge_type(),
                    "Bridge types cannot be used as template arguments"
                );
                Ok(arg_type_kind)
            })
            .collect::<Result<Vec<RsTypeKind>>>()?;

        let this = match (template_specialization.template_name.as_ref(), &type_args[..]) {
            ("std::unique_ptr", [_t, RsTypeKind::Record { record, .. }]) => {
                let has_std_deleter =
                    record.template_specialization.as_ref().is_some_and(|deleter| {
                        deleter.template_name.as_ref() == "std::default_delete"
                            && deleter.template_args.len() == 1
                            && deleter.template_args[0] == template_specialization.template_args[0]
                    });
                if !has_std_deleter {
                    return Ok(None);
                }
                Self::StdUniquePtr { element_type: type_args.remove(0) }
            }
            ("std::vector", [_t, RsTypeKind::Record { record, .. }]) => {
                let has_std_allocator =
                    record.template_specialization.as_ref().is_some_and(|allocator| {
                        allocator.template_name.as_ref() == "std::allocator"
                            && allocator.template_args.len() == 1
                            && allocator.template_args[0]
                                == template_specialization.template_args[0]
                    });
                if !has_std_allocator {
                    return Ok(None);
                }
                Self::StdVector { element_type: type_args.remove(0) }
            }
            ("absl::Span", [_t]) => {
                // Revisit the CcType of _t to see if it is const.
                let is_const = template_specialization.template_args[0].type_.as_ref().expect("should be valid because type_args is the successful result of get_template_args").is_const;
                Self::AbslSpan {
                    is_const,

                    // We always accept lifetime-bound spans as parameters. A C++ function
                    // shouldn't be using an array referenced by a span after it returns.
                    //
                    // Spans returned by a C++ function have an unclear lifetime, and so must be
                    // returned as a raw span.
                    include_lifetime: !is_return_type,

                    element_type: type_args.remove(0),
                }
            }
            _ => return Ok(None),
        };
        Ok(Some(Rc::new(this)))
    }

    fn to_token_stream(&self, db: &dyn BindingsGenerator) -> TokenStream {
        match self {
            Self::StdVector { element_type } => {
                let element_type_tokens = element_type.to_token_stream(db);
                quote! { ::cc_std::std::vector::<#element_type_tokens> }
            }
            Self::StdUniquePtr { element_type } => {
                let element_type_tokens = element_type.to_token_stream(db);
                quote! { ::cc_std::std::unique_ptr::<#element_type_tokens> }
            }
            Self::AbslSpan { is_const, include_lifetime, element_type } => {
                let element_type_tokens = element_type.to_token_stream(db);

                // Use Span when we have a lifetime parameter, and RawSpan otherwise.
                //
                // See http://<internal link>.
                match (*is_const, *include_lifetime) {
                    (true, true) => quote! { ::span::absl::Span<'_, #element_type_tokens> },
                    (false, true) => quote! { ::span::absl::SpanMut<'_, #element_type_tokens> },
                    (true, false) => quote! { ::span::absl::RawSpan<#element_type_tokens> },
                    (false, false) => quote! { ::span::absl::RawSpanMut<#element_type_tokens> },
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RsTypeKind {
    /// An error occurred while generating the type.
    ///
    /// Error types are only exposed in `:wrapper` mode, where they become
    /// opaque types, or where a visibility override is specified.
    Error {
        symbol: Rc<str>,
        error: arc_anyhow::Error,
        visibility_override: Option<Visibility>,
    },
    Pointer {
        pointee: Rc<RsTypeKind>,
        is_slice: bool,
        mutability: Mutability,
    },
    Reference {
        option: bool,
        referent: Rc<RsTypeKind>,
        mutability: Mutability,
        lifetime: Lifetime,
    },
    RvalueReference {
        referent: Rc<RsTypeKind>,
        mutability: Mutability,
        lifetime: Lifetime,
    },
    FuncPtr {
        option: bool,
        abi: Rc<str>,
        return_type: Rc<RsTypeKind>,
        param_types: Rc<[RsTypeKind]>,
    },
    /// An incomplete record type.
    IncompleteRecord {
        incomplete_record: Rc<IncompleteRecord>,
        crate_path: Rc<CratePath>,
    },
    /// A complete record type.
    Record {
        record: Rc<Record>,
        crate_path: Rc<CratePath>,
        /// If this record is an instantiation of a `UniformReprTemplateType`, this will be set.
        uniform_repr_template_type: Option<Rc<UniformReprTemplateType>>,
    },
    Enum {
        enum_: Rc<Enum>,
        crate_path: Rc<CratePath>,
    },
    TypeAlias {
        type_alias: Rc<TypeAlias>,
        underlying_type: Rc<RsTypeKind>,
        crate_path: Rc<CratePath>,
    },
    Primitive(Primitive),
    /// Types that require custom logic to translate.
    BridgeType {
        bridge_type: BridgeRsTypeKind,
        original_type: Rc<Record>,
    },
    /// Types that can be reinterpreted in place, e.g., signed char <-> i8
    /// This variant comes from the `CRUBIT_INTERNAL_RUST_TYPE` attribute macro in C++,
    /// which is used on types like `SliceRef`, `StrRef`, and C++ types generated from Rust
    /// types by cc_bindings_from_rs.
    TypeMapOverride(Rc<TypeMapOverride>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BridgeRsTypeKind {
    BridgeVoidConverters {
        rust_name: Rc<str>,
        cpp_to_rust_converter: Rc<str>,
        rust_to_cpp_converter: Rc<str>,
    },
    Bridge {
        rust_name: Rc<str>,
        abi_rust: Rc<str>,
        abi_cpp: Rc<str>,
        generic_types: Rc<[RsTypeKind]>,
    },
    ProtoMessageBridge {
        rust_name: Rc<str>,
        abi_rust: Rc<str>,
        abi_cpp: Rc<str>,
    },
    /// Bridges to a `*mut [T]` or `*const [T]`.
    SlicePointer {
        mutability: Mutability,
        pointee: CcType,
        abi_cpp: Rc<str>,
    },
    StdOptional(Rc<RsTypeKind>),
    StdPair(Rc<RsTypeKind>, Rc<RsTypeKind>),
    StdString {
        in_cc_std: bool,
    },
}

impl BridgeRsTypeKind {
    /// If the record is a bridge type, returns the corresponding BridgeRsTypeKind.
    /// Otherwise, returns None. This may also return an error if db.rs_type_kind fails, or if the
    /// record has template parameters that cannot be translated.
    pub fn new(record: &Record, db: &dyn BindingsGenerator) -> Result<Option<BridgeRsTypeKind>> {
        let Some(bridge_type) = &record.bridge_type else {
            return Ok(None);
        };

        let bridge_rs_type_kind = match bridge_type.clone() {
            BridgeType::BridgeVoidConverters {
                rust_name,
                cpp_to_rust_converter,
                rust_to_cpp_converter,
            } => BridgeRsTypeKind::BridgeVoidConverters {
                rust_name,
                cpp_to_rust_converter,
                rust_to_cpp_converter,
            },
            BridgeType::ProtoMessageBridge { rust_name, abi_rust, abi_cpp } => {
                BridgeRsTypeKind::ProtoMessageBridge { rust_name, abi_rust, abi_cpp }
            }
            BridgeType::Bridge { rust_name, abi_rust, abi_cpp } => BridgeRsTypeKind::Bridge {
                rust_name,
                abi_rust,
                abi_cpp,
                generic_types: record
                    .template_specialization
                    .as_ref()
                    .map(|template_spec| &template_spec.template_args[..])
                    .unwrap_or_default()
                    .iter()
                    .map(|template_arg: &TemplateArg| {
                        let type_ = template_arg.type_.as_ref().map_err(|err| {
                            anyhow!("Failed to get type from template arg: {}", err)
                        })?;

                        db.rs_type_kind(type_.clone())
                    })
                    .collect::<Result<Rc<[RsTypeKind]>>>()?,
            },
            BridgeType::SlicePointer { abi_cpp } => {
                let Some([type_in_slice]) = record
                    .template_specialization
                    .as_ref()
                    .map(|template_spec| &template_spec.template_args[..])
                else {
                    // This might break for std::span if we ever support it, which can have a second
                    // template argument for the size.
                    bail!("SlicePointer expects a single template argument");
                };
                let pointee: CcType = type_in_slice
                    .type_
                    .as_ref()
                    .map_err(|err| anyhow!("Failed to get type from template arg: {}", err))?
                    .clone();
                let pointee_rs_type_kind = db.rs_type_kind(pointee.clone())?;
                ensure!(
                    pointee_rs_type_kind.is_c_abi_compatible_by_value(),
                    "{} is not C ABI compatible, so it cannot be passed behind a slice",
                    pointee_rs_type_kind.display(db)
                );
                let mutability = if pointee.is_const { Mutability::Const } else { Mutability::Mut };
                BridgeRsTypeKind::SlicePointer { mutability, pointee, abi_cpp }
            }
            BridgeType::StdOptional(t) => {
                BridgeRsTypeKind::StdOptional(Rc::new(db.rs_type_kind(t)?))
            }
            BridgeType::StdPair(t1, t2) => BridgeRsTypeKind::StdPair(
                Rc::new(db.rs_type_kind(t1)?),
                Rc::new(db.rs_type_kind(t2)?),
            ),
            BridgeType::StdString => {
                let in_cc_std = db.ir().is_current_target(&record.owning_target)
                    && record.owning_target.target_name_escaped() == "cc_std";

                BridgeRsTypeKind::StdString { in_cc_std }
            }
        };

        Ok(Some(bridge_rs_type_kind))
    }

    pub fn is_void_converters_bridge_type(&self) -> bool {
        matches!(self, BridgeRsTypeKind::BridgeVoidConverters { .. })
    }
}

impl RsTypeKind {
    /// Directly creates an `RsTypeKind` from an `Item` that defines a type.
    ///
    /// WARNING: this is a low-level function that bypasses the validity checks
    /// that are normally performed by `BindingsGenerator::rs_type_kind()`. In particular,
    /// this function should only be called by functions that are themselves called by
    /// `rs_type_kind()`, in order to avoid cycles while introspecting types.
    ///
    /// Returns an error if the item does not define a type (e.g. it is a function declaration),
    /// or if the `RsTypeKind` cannot be created (e.g. a type alias which points to a type that
    /// cannot receive an `RsTypeKind`).
    pub fn from_item_raw(
        db: &dyn BindingsGenerator,
        item: Item,
        have_reference_param: bool,
        is_return_type: bool,
    ) -> Result<Self> {
        match item {
            Item::IncompleteRecord(incomplete_record) => {
                RsTypeKind::new_incomplete_record(db, incomplete_record)
            }
            Item::Record(record) => {
                RsTypeKind::new_record(db, record, have_reference_param, is_return_type)
            }
            Item::Enum(enum_) => RsTypeKind::new_enum(db, enum_),
            Item::TypeAlias(type_alias) => RsTypeKind::new_type_alias(db, type_alias),
            Item::TypeMapOverride(type_map_override) => {
                RsTypeKind::new_type_map_override(db, type_map_override)
            }
            other_item => bail!("Item does not define a type: {other_item:?}"),
        }
    }

    fn new_type_alias(db: &dyn BindingsGenerator, type_alias: Rc<TypeAlias>) -> Result<Self> {
        let ir = db.ir();
        let underlying_type = db.rs_type_kind(type_alias.underlying_type.clone())?;
        // Note: we don't need to call `.unalias()` for these checks, because we already checked
        // this, recursively.

        // Bridge types cannot be aliased
        if underlying_type.is_bridge_type() {
            return Ok(underlying_type);
        }
        // Records cannot be aliased unless they are part of the same translation unit as the alias.
        // This is not for an especially principled reason: unless it's in the same translation
        // unit, we don't know if the record is incomplete from the point of view of the alias.
        // For example, perhaps the alias is to a forward declaration, and then later, we completed
        // the forward declaration.
        if let RsTypeKind::Record { record, .. } = &underlying_type {
            if record.owning_target != type_alias.owning_target
                && record.defining_target.as_ref() != Some(&type_alias.owning_target)
            {
                return Ok(underlying_type);
            }
        }
        let crate_path = Rc::new(CratePath::new(
            &ir,
            ir.namespace_qualifier(&type_alias),
            rs_imported_crate_name(&type_alias.owning_target, &ir),
        ));
        Ok(RsTypeKind::TypeAlias {
            type_alias,
            crate_path,
            underlying_type: Rc::new(underlying_type),
        })
    }

    fn new_record(
        db: &dyn BindingsGenerator,
        record: Rc<Record>,
        have_reference_param: bool,
        is_return_type: bool,
    ) -> Result<Self> {
        let ir = db.ir();
        if let Some(bridge_type) = BridgeRsTypeKind::new(&record, db)? {
            return Ok(RsTypeKind::BridgeType { bridge_type, original_type: record });
        }
        let crate_path = Rc::new(CratePath::new(
            ir,
            ir.namespace_qualifier(&record),
            rs_imported_crate_name(&record.owning_target, ir),
        ));
        Ok(RsTypeKind::Record {
            uniform_repr_template_type: UniformReprTemplateType::new(
                db,
                record.template_specialization.as_ref(),
                have_reference_param,
                is_return_type,
            )?,
            record,
            crate_path,
        })
    }

    fn new_incomplete_record(
        db: &dyn BindingsGenerator,
        incomplete_record: Rc<IncompleteRecord>,
    ) -> Result<Self> {
        let ir = db.ir();
        let crate_path = Rc::new(CratePath::new(
            ir,
            ir.namespace_qualifier(&incomplete_record),
            rs_imported_crate_name(&incomplete_record.owning_target, ir),
        ));
        Ok(RsTypeKind::IncompleteRecord { incomplete_record, crate_path })
    }

    fn new_enum(db: &dyn BindingsGenerator, enum_: Rc<Enum>) -> Result<Self> {
        let ir = db.ir();
        let crate_path = Rc::new(CratePath::new(
            ir,
            ir.namespace_qualifier(&enum_),
            rs_imported_crate_name(&enum_.owning_target, ir),
        ));
        Ok(RsTypeKind::Enum { enum_, crate_path })
    }

    fn new_type_map_override(
        db: &dyn BindingsGenerator,
        type_map_override: Rc<TypeMapOverride>,
    ) -> Result<Self> {
        if type_map_override.rs_name.as_ref() == SLICE_REF_NAME_RS {
            let [slice_type_inner] = &type_map_override.type_parameters[..] else {
                bail!(
                    "SliceRef has {} type parameters, expected 1",
                    type_map_override.type_parameters.len()
                );
            };

            return Ok(RsTypeKind::Pointer {
                pointee: Rc::new(db.rs_type_kind(slice_type_inner.clone())?),
                is_slice: true,
                mutability: if slice_type_inner.is_const {
                    Mutability::Const
                } else {
                    Mutability::Mut
                },
            });
        }

        Ok(RsTypeKind::TypeMapOverride(type_map_override))
    }

    /// Returns true if the type is known to be `Unpin`, false otherwise.
    pub fn is_unpin(&self) -> bool {
        match self.unalias() {
            RsTypeKind::Error { .. } | RsTypeKind::IncompleteRecord { .. } => false,
            RsTypeKind::Record { record, uniform_repr_template_type, .. } => {
                uniform_repr_template_type.is_some() || record.is_unpin()
            }
            RsTypeKind::BridgeType { .. } => true,
            _ => true,
        }
    }

    /// Recursively follows type aliases until an underlying nonalias type is reached.
    pub fn unalias(&self) -> &Self {
        let mut unaliased = self;
        while let RsTypeKind::TypeAlias { underlying_type, .. } = unaliased {
            unaliased = underlying_type;
        }
        unaliased
    }

    pub fn is_bridge_type(&self) -> bool {
        matches!(self.unalias(), RsTypeKind::BridgeType { .. })
    }

    pub fn is_pointer_bridge_type(&self) -> bool {
        matches!(
            self.unalias(),
            RsTypeKind::BridgeType {
                bridge_type: BridgeRsTypeKind::BridgeVoidConverters { .. },
                ..
            }
        )
    }

    pub fn is_crubit_abi_bridge_type(&self) -> bool {
        self.is_bridge_type() && !self.is_pointer_bridge_type()
    }

    pub fn is_proto_message_bridge_type(&self) -> bool {
        matches!(
            self.unalias(),
            RsTypeKind::BridgeType { bridge_type: BridgeRsTypeKind::ProtoMessageBridge { .. }, .. }
        )
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self.unalias(), RsTypeKind::Primitive(_))
    }

    /// Returns the features required to use this type which are not already
    /// enabled, which might depend on where the type is used.
    ///
    /// If a function accepts or returns this type, or an alias refers to this
    /// type, then the function or type alias will itself also require this
    /// feature. However, in the case of fields inside compound data types,
    /// only those fields require the feature, not the entire type.
    ///
    /// This isn't inlined into `db.rs_type_kind()` because `db.rs_type_kind()`
    /// does not know which target is requesting the type, and it's a bit
    /// tricky. Consider that a templated item needs to perform this check
    /// for both the template definition and its instantiation, and so both
    /// would need to be passed in to rs_type_kind() in order to be able to
    /// merge these two functions.
    pub fn required_crubit_features(
        &self,
        db: &dyn BindingsGenerator,
        enabled_features: flagset::FlagSet<CrubitFeature>,
    ) -> (flagset::FlagSet<CrubitFeature>, String) {
        let mut missing_features = <flagset::FlagSet<CrubitFeature>>::default();
        let mut reasons = <std::collections::BTreeSet<std::borrow::Cow<'static, str>>>::new();
        let mut require_feature =
            |required_feature: CrubitFeature,
             reason: Option<&dyn Fn() -> std::borrow::Cow<'static, str>>| {
                let required_features = <flagset::FlagSet<CrubitFeature>>::from(required_feature);
                let missing = required_features - enabled_features;
                if !missing.is_empty() {
                    missing_features |= missing;
                    if let Some(reason) = reason {
                        reasons.insert(reason());
                    }
                }
            };

        for rs_type_kind in self.dfs_iter() {
            match rs_type_kind {
                RsTypeKind::Error { error, visibility_override, .. } => {
                    if visibility_override.is_some() {
                        require_feature(CrubitFeature::Supported, None)
                    } else {
                        require_feature(
                            CrubitFeature::Wrapper,
                            Some(&|| std::borrow::Cow::from(format!("error: {error}"))),
                        )
                    }
                }
                RsTypeKind::Pointer { .. } => require_feature(CrubitFeature::Supported, None),
                RsTypeKind::Reference { .. } | RsTypeKind::RvalueReference { .. } => {
                    require_feature(
                        CrubitFeature::Experimental,
                        Some(&|| "references are not supported".into()),
                    );
                }
                RsTypeKind::FuncPtr { abi, .. } => {
                    if &**abi == "C" {
                        require_feature(CrubitFeature::Supported, None);
                    } else {
                        require_feature(
                            CrubitFeature::Experimental,
                            Some(&|| "functions must be not use a non-C calling convention".into()),
                        );
                    }
                }
                RsTypeKind::IncompleteRecord { .. } => require_feature(
                    CrubitFeature::Wrapper,
                    Some(&|| {
                        format!("{} is not a complete type)", rs_type_kind.display(db)).into()
                    }),
                ),
                // Here, we can very carefully be non-recursive into the _structure_ of the type.
                //
                // Whether a record type is supported in rust does _not_ depend on whether each
                // field is supported in Rust -- we can, if those fields are unsupported, replace
                // them with opaque blobs.
                //
                // Instead, what matters is the abstract properties of the struct itself!
                RsTypeKind::Record { record, .. } => {
                    // Types which aren't rust-movable, or which are general template
                    // instantiations, are only supported experimentally.
                    // But we do want to allow some commonly used template instantiations such as
                    // std::string_view so we create an allow list fo them. This is just a temporary
                    // solution until we have a better way to handle template
                    // instantiations.
                    if record.defining_target.is_none()
                        || record.is_allowed_template_instantiation()
                    {
                        require_feature(CrubitFeature::Supported, None)
                    } else if record.defining_target.is_some() {
                        require_feature(
                            CrubitFeature::Wrapper,
                            Some(&|| {
                                format!("{} is a template instantiation", rs_type_kind.display(db),)
                                    .into()
                            }),
                        )
                    }
                }
                RsTypeKind::Enum { .. } => require_feature(CrubitFeature::Supported, None),
                // the alias itself is supported, but the overall features require depends on the
                // aliased type, which is also visited by dfs_iter.
                RsTypeKind::TypeAlias { .. } => require_feature(CrubitFeature::Supported, None),
                RsTypeKind::Primitive { .. } => require_feature(CrubitFeature::Supported, None),
                RsTypeKind::BridgeType { bridge_type, original_type } => {
                    let is_pointer_bridge =
                        matches!(bridge_type, BridgeRsTypeKind::BridgeVoidConverters { .. });

                    if original_type.template_specialization.is_none()
                        || original_type.is_allowed_template_instantiation()
                        || !is_pointer_bridge
                    {
                        require_feature(CrubitFeature::Supported, None)
                    } else {
                        require_feature(
                            CrubitFeature::Experimental,
                            Some(&|| {
                                format!(
                                    "{} is a bridged template instantiation",
                                    rs_type_kind.display(db),
                                )
                                .into()
                            }),
                        )
                    }
                }
                RsTypeKind::TypeMapOverride(_) => require_feature(CrubitFeature::Supported, None),
            }
        }
        (missing_features, reasons.into_iter().join(", "))
    }

    /// Returns true if the type can be passed by value through `extern "C"` ABI
    /// thunks.
    pub fn is_c_abi_compatible_by_value(&self) -> bool {
        match self.unalias() {
            RsTypeKind::IncompleteRecord { .. } => {
                // Incomplete record (forward declaration) as parameter type or return type is
                // unusual but it's a valid cc_library and such a header can be made to work
                // when its user code includes headers that define the forward-declared type.
                // Thus we don't panic here and simply return false, to allow
                // Crubit to generate bindings for other un-impacted APIs.
                false
            }
            // `rs_bindings_from_cc` can change the type of fields (e.g. using a blob of bytes for
            // unsupported field types, or for no_unique_address fields).  Changing the type
            // of fields may change the ABI, which means that we can no longer assume
            // that `extern "C"` ABI thunks can pass such types by value.
            //
            // TODO(b/274177296): Return `true` for structs where bindings replicate the type of
            // all the fields.
            RsTypeKind::Record { .. } => false,
            RsTypeKind::BridgeType { .. } => false,
            RsTypeKind::TypeMapOverride(type_map_override) => type_map_override.is_same_layout,
            _ => true,
        }
    }

    /// Returns true if the type is known to be move-constructible, false
    /// otherwise.
    ///
    /// For the purposes of this method, references are considered
    /// move-constructible (as if they were pointers).
    pub fn is_move_constructible(&self) -> bool {
        match self.unalias() {
            RsTypeKind::IncompleteRecord { .. } => false,
            RsTypeKind::Record { record, .. } => {
                record.move_constructor != ir::SpecialMemberFunc::Unavailable
            }
            RsTypeKind::BridgeType { .. } => true,
            _ => true,
        }
    }

    /// Returns Ok if the type can be used by value, or an error describing why
    /// it can't.
    pub fn check_by_value(&self) -> Result<()> {
        match self.unalias() {
            RsTypeKind::Error { error, .. } => bail!("Cannot use an error type by value: {error}"),
            RsTypeKind::Record { record, .. } => check_by_value(record),
            RsTypeKind::IncompleteRecord { incomplete_record, .. } => {
                bail!(
                    "Attempted to pass incomplete record type `{}` by-value",
                    incomplete_record.cc_name
                )
            }
            _ => Ok(()),
        }
    }

    pub fn format_as_return_type_fragment(
        &self,
        db: &dyn BindingsGenerator,
        self_record: Option<&Record>,
    ) -> Option<TokenStream> {
        match self.unalias() {
            RsTypeKind::Primitive(Primitive::Void) => None,
            _ => Some(self.to_token_stream_replacing_by_self(db, self_record)),
        }
    }

    /// Formats this RsTypeKind as the `self` parameter: usually, `&'a self` or
    /// `&'a mut self`.
    ///
    /// If this is !Unpin, however, it uses `self: Pin<&mut Self>` instead.
    ///
    /// If `self` is formatted as RvalueReference or ConstRvalueReference, then
    /// `arbitrary_self_types` feature flag is returned in the feature flags.
    pub fn format_as_self_param(&self) -> Result<RsSnippet> {
        match self {
            RsTypeKind::Pointer { .. } => {
                // TODO(jeanpierreda): provide end-user-facing docs, and insert a link to e.g.
                // something like <internal link>
                bail!(
                    "`self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function."
                )
            }
            RsTypeKind::Reference { option, referent, lifetime, mutability } => {
                assert!(!*option, "Optional self type is not valid, this should never happen");
                let mut_ = mutability.format_for_reference();
                let lifetime = lifetime.format_for_reference();
                if mutability == &Mutability::Mut && !referent.is_unpin() {
                    // TODO(b/239661934): Add a `use ::core::pin::Pin` to the crate, and use
                    // `Pin`.
                    Ok(RsSnippet::new(quote! {self: ::core::pin::Pin< & #lifetime #mut_ Self>}))
                } else {
                    Ok(RsSnippet::new(quote! { & #lifetime #mut_ self }))
                }
            }
            RsTypeKind::RvalueReference { referent: _, lifetime, mutability } => {
                let lifetime = lifetime.format_for_reference();
                // TODO(b/239661934): Add `use ::ctor::{RvalueReference, ConstRvalueReference}`.
                match mutability {
                    Mutability::Mut => Ok(RsSnippet {
                        tokens: quote! {self: ::ctor::RvalueReference<#lifetime, Self>},
                        features: Feature::arbitrary_self_types.into(),
                    }),
                    Mutability::Const => Ok(RsSnippet {
                        tokens: quote! {self: ::ctor::ConstRvalueReference<#lifetime, Self>},
                        features: Feature::arbitrary_self_types.into(),
                    }),
                }
            }
            RsTypeKind::Record { .. } => {
                // This case doesn't happen for methods, but is needed for free functions mapped
                // to a trait impl that take the first argument by value.
                Ok(RsSnippet::new(quote! { self }))
            }
            _ => bail!("Unexpected type of `self` parameter: {:?}", self),
        }
    }

    /// Returns whether the type represented by `self` implements the `Copy`
    /// trait.
    pub fn implements_copy(&self) -> bool {
        match self {
            RsTypeKind::Error { .. } => false,
            RsTypeKind::Primitive { .. } => true,
            RsTypeKind::Pointer { .. } => true,
            RsTypeKind::FuncPtr { .. } => true,
            RsTypeKind::Reference { mutability: Mutability::Const, .. } => true,
            RsTypeKind::Reference { mutability: Mutability::Mut, .. } => false,
            RsTypeKind::RvalueReference { .. } => false,
            RsTypeKind::IncompleteRecord { .. } => false,
            RsTypeKind::Record { record, .. } => should_derive_copy(record),
            RsTypeKind::Enum { .. } => true,
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.implements_copy(),
            RsTypeKind::BridgeType { bridge_type, .. } => match bridge_type {
                // We cannot get the information of the Rust type so we assume it is not Copy.
                BridgeRsTypeKind::BridgeVoidConverters { .. }
                | BridgeRsTypeKind::Bridge { .. }
                | BridgeRsTypeKind::ProtoMessageBridge { .. } => false,
                BridgeRsTypeKind::SlicePointer { mutability, .. } => mutability.is_const(),
                BridgeRsTypeKind::StdOptional(t) => t.implements_copy(),
                BridgeRsTypeKind::StdPair(t1, t2) => t1.implements_copy() && t2.implements_copy(),
                BridgeRsTypeKind::StdString { .. } => false,
            },
            RsTypeKind::TypeMapOverride(_) => true,
        }
    }

    pub fn is_ref_to(&self, expected_record: &Record) -> bool {
        match self.unalias() {
            RsTypeKind::Reference { referent, .. } => referent.is_record(expected_record),
            RsTypeKind::RvalueReference { referent, .. } => referent.is_record(expected_record),
            _ => false,
        }
    }

    pub fn is_rvalue_ref_to(&self, expected_record: &Record) -> bool {
        match self.unalias() {
            RsTypeKind::RvalueReference { referent, .. } => referent.is_record(expected_record),
            _ => false,
        }
    }

    pub fn is_shared_ref_to(&self, expected_record: &Record) -> bool {
        match self.unalias() {
            RsTypeKind::Reference { referent, mutability: Mutability::Const, .. } => {
                referent.is_record(expected_record)
            }
            _ => false,
        }
    }

    pub fn is_record(&self, expected_record: &Record) -> bool {
        match self.unalias() {
            RsTypeKind::Record { record: actual_record, .. } => {
                actual_record.id == expected_record.id
            }
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self.unalias(), RsTypeKind::Primitive(Primitive::Bool))
    }

    /// Iterates over `self` and all the nested types (e.g. pointees, generic
    /// type args, etc.) in DFS order.
    pub fn dfs_iter(&self) -> impl Iterator<Item = &RsTypeKind> + '_ {
        RsTypeKindIter::new(self)
    }

    /// Iterates over all `LifetimeId`s in `self` and in all the nested types.
    /// Note that the results might contain duplicate LifetimeId values (e.g.
    /// if the same LifetimeId is used in two `type_args`).
    pub fn lifetimes(&self) -> impl Iterator<Item = Lifetime> + use<'_> {
        self.dfs_iter().filter_map(Self::lifetime)
    }

    /// Returns the pointer or reference target.
    pub fn referent(&self) -> Option<&RsTypeKind> {
        match self.unalias() {
            Self::Pointer { pointee: p, .. }
            | Self::Reference { referent: p, .. }
            | Self::RvalueReference { referent: p, .. } => Some(&**p),
            _ => None,
        }
    }

    /// Returns the reference lifetime, or None if this is not a reference.
    pub fn lifetime(&self) -> Option<Lifetime> {
        match self.unalias() {
            Self::Reference { lifetime, .. } | Self::RvalueReference { lifetime, .. } => {
                Some(lifetime.clone())
            }
            _ => None,
        }
    }

    /// Similar to to_token_stream, but replacing RsTypeKind:Record with Self
    /// when the underlying Record matches the given one.
    pub fn to_token_stream_replacing_by_self(
        &self,
        db: &dyn BindingsGenerator,
        self_record: Option<&Record>,
    ) -> TokenStream {
        match self {
            RsTypeKind::Pointer { pointee, is_slice, mutability } => {
                let mutability = mutability.format_for_pointer();
                let pointee_ = pointee.to_token_stream_replacing_by_self(db, self_record);
                if *is_slice {
                    quote! {* #mutability [#pointee_] }
                } else {
                    quote! {* #mutability #pointee_ }
                }
            }
            RsTypeKind::Reference { option, referent, mutability, lifetime } => {
                let mut_ = mutability.format_for_reference();
                let lifetime = lifetime.format_for_reference();
                let referent_ = referent.to_token_stream_replacing_by_self(db, self_record);
                let mut tokens = quote! {& #lifetime #mut_ #referent_};
                if mutability == &Mutability::Mut && !referent.is_unpin() {
                    // TODO(b/239661934): Add a `use ::core::pin::Pin` to the crate, and use
                    // `Pin`. This either requires deciding how to qualify pin at
                    // RsTypeKind-creation time, or returning a non-TokenStream type from here (and
                    // not implementing ToTokens, but instead some other interface.)
                    tokens = quote! {::core::pin::Pin< #tokens >};
                }
                if *option {
                    tokens = quote! {Option< #tokens >};
                }
                tokens
            }
            RsTypeKind::RvalueReference { referent, mutability, lifetime } => {
                let referent_ = referent.to_token_stream_replacing_by_self(db, self_record);
                // TODO(b/239661934): Add a `use ::ctor::RvalueReference` (etc.) to the crate.
                if mutability == &Mutability::Mut {
                    quote! {::ctor::RvalueReference<#lifetime, #referent_>}
                } else {
                    quote! {::ctor::ConstRvalueReference<#lifetime, #referent_>}
                }
            }
            RsTypeKind::FuncPtr { option, abi, return_type, param_types } => {
                let param_types_ = param_types
                    .iter()
                    .map(|type_| type_.to_token_stream_replacing_by_self(db, self_record));
                let mut tokens = quote! { extern #abi fn( #( #param_types_ ),* ) };
                if let Some(return_frag) =
                    return_type.format_as_return_type_fragment(db, self_record)
                {
                    quote! { -> #return_frag }.to_tokens(&mut tokens);
                }
                if param_types.iter().any(|p| db.is_rs_type_kind_unsafe(p.clone())) {
                    tokens = quote! { unsafe #tokens }
                }
                if *option {
                    tokens = quote! {Option< #tokens >}
                }
                tokens
            }
            RsTypeKind::Record { record, .. } => {
                if self_record == Some(record) {
                    quote! { Self }
                } else {
                    self.to_token_stream(db)
                }
            }
            RsTypeKind::BridgeType { .. } => self.to_token_stream(db),
            RsTypeKind::TypeMapOverride(_) => self.to_token_stream(db),
            _ => self.to_token_stream(db),
        }
    }

    /// Returns a `Display`able type for this `RsTypeKind`.
    pub fn display<'a, 'db>(
        &'a self,
        db: &'a dyn BindingsGenerator<'db>,
    ) -> impl std::fmt::Display + use<'a, 'db> {
        DisplayRsTypeKind { rs_type_kind: self, db }
    }
}

/// A type that implements [`std::fmt::Display`] for [`RsTypeKind`], which
/// requires a [`BindingsGenerator`] to be able to format the type.
pub struct DisplayRsTypeKind<'a, 'db> {
    rs_type_kind: &'a RsTypeKind,
    db: &'a dyn BindingsGenerator<'db>,
}

impl std::fmt::Display for DisplayRsTypeKind<'_, '_> {
    // Formats the token stream of the RsTypeKind to a string. Note that this can
    // include extra whitespace, where we'd ideally remove it, but it is hard to
    // remove whitespace without invoking rustfmt.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match write_unformatted_tokens(f, self.rs_type_kind.to_token_stream(self.db)) {
            Ok(_) => Ok(()),
            Err(e) => {
                // Honestly this should never happen, but we should spit out something.
                write!(f, "<error: {e}>")
            }
        }
    }
}

impl RsTypeKind {
    pub fn to_token_stream(&self, db: &dyn BindingsGenerator) -> TokenStream {
        match self {
            // errors become opaque blobs
            RsTypeKind::Error { symbol, .. } => {
                // We use `()` as the crate identifier for convenience, and because
                // the only types using `()` like this are all pub(crate) (generated here.)
                quote! { ::forward_declare::Incomplete<::forward_declare::symbol!(#symbol), ()> }
            }
            RsTypeKind::Pointer { pointee, is_slice, mutability } => {
                let mutability = mutability.format_for_pointer();
                let pointee_tokens = pointee.to_token_stream(db);
                if *is_slice {
                    quote! {* #mutability [#pointee_tokens] }
                } else {
                    quote! {* #mutability #pointee_tokens }
                }
            }
            RsTypeKind::Reference { option, referent, mutability, lifetime } => {
                let mut_ = mutability.format_for_reference();
                let lifetime = lifetime.format_for_reference();
                let referent_tokens = referent.to_token_stream(db);
                let mut tokens = quote! {& #lifetime #mut_ #referent_tokens};
                if mutability == &Mutability::Mut && !referent.is_unpin() {
                    // TODO(b/239661934): Add a `use ::core::pin::Pin` to the crate, and use
                    // `Pin`. This either requires deciding how to qualify pin at
                    // RsTypeKind-creation time, or returning a non-TokenStream type from here (and
                    // not implementing ToTokens, but instead some other interface.)
                    tokens = quote! { ::core::pin::Pin< #tokens > };
                }
                if *option {
                    tokens = quote! { Option< #tokens > };
                }
                tokens
            }
            RsTypeKind::RvalueReference { referent, mutability, lifetime } => {
                // TODO(b/239661934): Add a `use ::ctor::RvalueReference` (etc.) to the crate.
                let referent_tokens = referent.to_token_stream(db);
                if mutability == &Mutability::Mut {
                    quote! {::ctor::RvalueReference<#lifetime, #referent_tokens>}
                } else {
                    quote! {::ctor::ConstRvalueReference<#lifetime, #referent_tokens>}
                }
            }
            RsTypeKind::FuncPtr { option, abi, return_type, param_types } => {
                let param_types_tokens = param_types.iter().map(|ty| ty.to_token_stream(db));
                let mut tokens = quote! { extern #abi fn( #( #param_types_tokens ),* ) };
                if let Some(return_frag) = return_type.format_as_return_type_fragment(db, None) {
                    quote! { -> #return_frag }.to_tokens(&mut tokens);
                }
                if param_types.iter().any(|p| db.is_rs_type_kind_unsafe(p.clone())) {
                    tokens = quote! { unsafe #tokens }
                }
                if *option {
                    tokens = quote! {Option< #tokens >}
                }
                tokens
            }
            RsTypeKind::IncompleteRecord { incomplete_record, crate_path } => {
                let record_ident = make_rs_ident(incomplete_record.rs_name.identifier.as_ref());
                quote! { #crate_path #record_ident }
            }
            RsTypeKind::Record { record, crate_path, uniform_repr_template_type } => {
                if let Some(generic_monomorphization) = uniform_repr_template_type {
                    return generic_monomorphization.to_token_stream(db);
                }
                let ident = make_rs_ident(record.rs_name.identifier.as_ref());
                quote! { #crate_path #ident }
            }
            RsTypeKind::Enum { enum_, crate_path } => {
                let ident = make_rs_ident(&enum_.rs_name.identifier);
                quote! { #crate_path #ident }
            }
            RsTypeKind::TypeAlias { type_alias, crate_path, .. } => {
                let ident = make_rs_ident(&type_alias.rs_name.identifier);
                quote! { #crate_path #ident }
            }
            RsTypeKind::Primitive(primitive) => match primitive {
                Primitive::Bool => quote! { bool },
                Primitive::Void => quote! { ::core::ffi::c_void },
                Primitive::Float => quote! { f32 },
                Primitive::Double => quote! { f64 },
                Primitive::Char => quote! { ::core::ffi::c_char },
                Primitive::SignedChar => quote! { ::core::ffi::c_schar },
                Primitive::UnsignedChar => quote! { ::core::ffi::c_uchar },
                Primitive::Short => quote! { ::core::ffi::c_short },
                Primitive::Int => quote! { ::core::ffi::c_int },
                Primitive::Long => quote! { ::core::ffi::c_long },
                Primitive::LongLong => quote! { ::core::ffi::c_longlong },
                Primitive::UnsignedShort => quote! { ::core::ffi::c_ushort },
                Primitive::UnsignedInt => quote! { ::core::ffi::c_uint },
                Primitive::UnsignedLong => quote! { ::core::ffi::c_ulong },
                Primitive::UnsignedLongLong => quote! { ::core::ffi::c_ulonglong },
                Primitive::Char16T => quote! { u16 },
                Primitive::Char32T => quote! { u32 },
                Primitive::PtrdiffT
                | Primitive::StdPtrdiffT
                | Primitive::IntptrT
                | Primitive::StdIntptrT => quote! { isize },
                Primitive::SizeT
                | Primitive::StdSizeT
                | Primitive::UintptrT
                | Primitive::StdUintptrT => quote! { usize },
                Primitive::Int8T | Primitive::StdInt8T => quote! { i8 },
                Primitive::Int16T | Primitive::StdInt16T => quote! { i16 },
                Primitive::Int32T | Primitive::StdInt32T => quote! { i32 },
                Primitive::Int64T | Primitive::StdInt64T => quote! { i64 },
                Primitive::Uint8T | Primitive::StdUint8T => quote! { u8 },
                Primitive::Uint16T | Primitive::StdUint16T => quote! { u16 },
                Primitive::Uint32T | Primitive::StdUint32T => quote! { u32 },
                Primitive::Uint64T | Primitive::StdUint64T => quote! { u64 },
            },
            RsTypeKind::BridgeType { bridge_type, original_type } => {
                let make_path = |rust_name: &str| {
                    let is_absolute_path = rust_name.starts_with("::");
                    // If the name starts with "::", then it is an absolute path. In this case, we
                    // need to skip the first part of the split, since it returns the empty string.
                    let name_parts =
                        rust_name.split("::").skip(is_absolute_path as usize).map(make_rs_ident);
                    let target = original_type
                        .defining_target
                        .as_ref()
                        .unwrap_or(&original_type.owning_target);

                    let prefix = if is_absolute_path {
                        quote! {}
                    } else if db.ir().is_current_target(target) {
                        quote! {crate}
                    } else {
                        let ident = make_rs_ident(target.target_name());
                        quote! { :: #ident }
                    };
                    quote! { #prefix :: #(#name_parts)::* }
                };
                match bridge_type {
                    BridgeRsTypeKind::BridgeVoidConverters { rust_name, .. } => {
                        make_path(rust_name)
                    }
                    BridgeRsTypeKind::Bridge { rust_name, generic_types, .. } => {
                        let path = make_path(rust_name);

                        // If there are no generic types, then we're done.
                        if generic_types.is_empty() {
                            return path;
                        }

                        let generic_types_tokens =
                            generic_types.iter().map(|t| t.to_token_stream(db));
                        quote! { #path < #(#generic_types_tokens),* > }
                    }
                    BridgeRsTypeKind::ProtoMessageBridge { rust_name, .. } => make_path(rust_name),
                    BridgeRsTypeKind::SlicePointer { mutability, pointee, .. } => {
                        let mutability = mutability.format_for_pointer();
                        let pointee = db
                            .rs_type_kind(pointee.clone())
                            .expect("pointee was already validated as a valid RsTypeKind in BridgeRsTypeKind::new")
                            .to_token_stream(db);
                        quote! { * #mutability [#pointee] }
                    }
                    BridgeRsTypeKind::StdOptional(inner) => {
                        let inner = inner.to_token_stream(db);
                        quote! { ::core::option::Option< #inner > }
                    }
                    BridgeRsTypeKind::StdPair(first, second) => {
                        let first = first.to_token_stream(db);
                        let second = second.to_token_stream(db);
                        quote! { (#first, #second) }
                    }
                    BridgeRsTypeKind::StdString { in_cc_std } => {
                        if *in_cc_std {
                            quote! { crate::std::string }
                        } else {
                            quote! { ::cc_std::std::string }
                        }
                    }
                }
            }
            RsTypeKind::TypeMapOverride(type_map_override) => {
                type_map_override.rs_name.parse().expect("Invalid RsType::name in the IR")
            }
        }
    }
}

struct RsTypeKindIter<'ty> {
    todo: Vec<&'ty RsTypeKind>,
}

impl<'ty> RsTypeKindIter<'ty> {
    pub fn new(ty: &'ty RsTypeKind) -> Self {
        Self { todo: vec![ty] }
    }
}

impl<'ty> Iterator for RsTypeKindIter<'ty> {
    type Item = &'ty RsTypeKind;

    fn next(&mut self) -> Option<Self::Item> {
        match self.todo.pop() {
            None => None,
            Some(curr) => {
                match curr {
                    RsTypeKind::Error { .. }
                    | RsTypeKind::Primitive { .. }
                    | RsTypeKind::IncompleteRecord { .. }
                    | RsTypeKind::Record { .. }
                    | RsTypeKind::Enum { .. } => {}
                    RsTypeKind::Pointer { pointee, .. } => self.todo.push(pointee),
                    RsTypeKind::Reference { referent, .. } => self.todo.push(referent),
                    RsTypeKind::RvalueReference { referent, .. } => self.todo.push(referent),
                    RsTypeKind::TypeAlias { underlying_type: t, .. } => self.todo.push(t),
                    RsTypeKind::FuncPtr { return_type, param_types, .. } => {
                        self.todo.push(return_type);
                        self.todo.extend(param_types.iter().rev());
                    }
                    RsTypeKind::BridgeType { bridge_type, .. } => match bridge_type {
                        BridgeRsTypeKind::BridgeVoidConverters { .. }
                        | BridgeRsTypeKind::ProtoMessageBridge { .. }
                        | BridgeRsTypeKind::Bridge { .. }
                        | BridgeRsTypeKind::SlicePointer { .. } => {}
                        BridgeRsTypeKind::StdOptional(t) => self.todo.push(t),
                        BridgeRsTypeKind::StdPair(t1, t2) => {
                            self.todo.push(t2);
                            self.todo.push(t1);
                        }
                        BridgeRsTypeKind::StdString { .. } => {}
                    },
                    RsTypeKind::TypeMapOverride(_) => {}
                };
                Some(curr)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arc_anyhow::Result;
    use googletest::prelude::*;
    use token_stream_matchers::assert_rs_matches;

    fn make_type_map_override(name: Rc<str>, is_same_layout: bool) -> RsTypeKind {
        RsTypeKind::TypeMapOverride(Rc::new(TypeMapOverride {
            rs_name: name,
            cc_name: "".into(),
            type_parameters: Vec::new(),
            owning_target: BazelLabel("//new/for/testing".into()),
            size_align: None,
            is_same_layout,
            id: ItemId::new_for_testing(0),
            must_bind: false,
        }))
    }

    #[gtest]
    fn test_dfs_iter_ordering_for_func_ptr() {
        // Set up a test input representing: fn(A, B) -> C
        let f = {
            let a = make_type_map_override("A".into(), true);
            let b = make_type_map_override("B".into(), true);
            let c = make_type_map_override("C".into(), true);
            RsTypeKind::FuncPtr {
                option: false,
                abi: "blah".into(),
                param_types: Rc::from([a, b]),
                return_type: Rc::new(c),
            }
        };
        let dfs_names = f
            .dfs_iter()
            .map(|t| match t {
                RsTypeKind::FuncPtr { .. } => "fn".to_string(),
                RsTypeKind::TypeMapOverride(type_map_override) => {
                    type_map_override.rs_name.to_string()
                }
                _ => unreachable!("Only FuncPtr and TypeMapOverride kinds are used in this test"),
            })
            .collect_vec();
        assert_eq!(vec!["fn", "A", "B", "C"], dfs_names);
    }

    struct EmptyDatabase;
    impl<'db> BindingsGenerator<'db> for EmptyDatabase {}

    #[gtest]
    fn test_lifetime_elision_for_references() {
        let referent = Rc::new(make_type_map_override("T".into(), true));
        let reference = RsTypeKind::Reference {
            option: false,
            referent,
            mutability: Mutability::Const,
            lifetime: Lifetime::new("_"),
        };
        assert_rs_matches!(reference.to_token_stream(&EmptyDatabase), quote! {&T});
    }

    #[gtest]
    fn test_lifetime_elision_for_rvalue_references() {
        let referent = Rc::new(make_type_map_override("T".into(), true));
        let reference = RsTypeKind::RvalueReference {
            referent,
            mutability: Mutability::Mut,
            lifetime: Lifetime::new("_"),
        };
        assert_rs_matches!(
            reference.to_token_stream(&EmptyDatabase),
            quote! {RvalueReference<'_, T>}
        );
    }

    #[gtest]
    fn test_format_as_self_param_rvalue_reference() -> Result<()> {
        let referent = Rc::new(make_type_map_override("T".into(), true));
        let result = RsTypeKind::RvalueReference {
            referent,
            mutability: Mutability::Mut,
            lifetime: Lifetime::new("a"),
        }
        .format_as_self_param()?;
        assert_rs_matches!(result.tokens, quote! {self: ::ctor::RvalueReference<'a, Self>});
        assert_eq!(result.features, Feature::arbitrary_self_types);
        Ok(())
    }

    #[gtest]
    fn test_format_as_self_param_const_rvalue_reference() -> Result<()> {
        let referent = Rc::new(make_type_map_override("T".into(), true));
        let result = RsTypeKind::RvalueReference {
            referent,
            mutability: Mutability::Const,
            lifetime: Lifetime::new("a"),
        }
        .format_as_self_param()?;
        assert_rs_matches!(result.tokens, quote! {self: ::ctor::ConstRvalueReference<'a, Self>});
        assert_eq!(result.features, Feature::arbitrary_self_types);
        Ok(())
    }

    /// Basic unit test of required_crubit_features on a compound data type.
    ///
    /// If a nested type within it requires a feature, then the whole feature
    /// does. This is done automatically via dfs_iter().
    #[gtest]
    fn test_required_crubit_features() {
        let no_types: &[RsTypeKind] = &[];
        let int = RsTypeKind::Primitive(Primitive::Int32T);
        let reference = RsTypeKind::Reference {
            option: false,
            referent: Rc::new(int.clone()),
            mutability: Mutability::Const,
            lifetime: Lifetime::new("_"),
        };
        for func_ptr in [
            RsTypeKind::FuncPtr {
                option: false,
                abi: "C".into(),
                return_type: Rc::new(reference.clone()),
                param_types: no_types.into(),
            },
            RsTypeKind::FuncPtr {
                option: false,
                abi: "C".into(),
                return_type: Rc::new(int),
                param_types: Rc::from([reference]),
            },
        ] {
            let (all_required_features, reason) = func_ptr.required_crubit_features(
                &EmptyDatabase,
                <flagset::FlagSet<CrubitFeature>>::default(),
            );
            assert_eq!(
                all_required_features,
                CrubitFeature::Experimental | CrubitFeature::Supported
            );
            assert_eq!(reason, "references are not supported");
        }
    }
}
