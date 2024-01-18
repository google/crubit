// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![allow(clippy::collapsible_else_if)]
//! Vocabulary types and code generation functions for generating Rust code.

use arc_anyhow::Result;
use code_gen_utils::make_rs_ident;
use code_gen_utils::NamespaceQualifier;
use error_report::bail;
use ir::*;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashSet;
use std::iter::Iterator;
use std::rc::Rc;
use token_stream_printer::write_unformatted_tokens;

/// A struct with information associated with the formatted Rust code snippet.
#[derive(Clone, Debug)]
pub struct RsSnippet {
    pub tokens: TokenStream,
    // The Rust features that are needed for `tokens` to work.
    pub features: HashSet<Ident>,
}

impl RsSnippet {
    /// Convenience function to initialize RsSnippet with empty `features`.
    pub fn new(tokens: TokenStream) -> RsSnippet {
        RsSnippet { tokens, features: HashSet::<Ident>::new() }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mutability {
    Const,
    Mut,
}

impl Mutability {
    pub fn format_for_pointer(&self) -> TokenStream {
        match self {
            Mutability::Mut => quote! {mut},
            Mutability::Const => quote! {const},
        }
    }

    pub fn format_for_reference(&self) -> TokenStream {
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

pub fn format_generic_params<'a, T: ToTokens>(
    lifetimes: impl IntoIterator<Item = &'a Lifetime>,
    types: impl IntoIterator<Item = T>,
) -> TokenStream {
    let mut lifetimes = lifetimes.into_iter().filter(|lifetime| &*lifetime.0 != "_").peekable();
    let mut types = types.into_iter().peekable();
    if lifetimes.peek().is_none() && types.peek().is_none() {
        quote! {}
    } else {
        quote! { < #( #lifetimes ),* #( #types ),*> }
    }
}

pub fn format_generic_params_replacing_by_self<'a>(
    types: impl IntoIterator<Item = &'a RsTypeKind>,
    trait_record: Option<&Record>,
) -> TokenStream {
    format_generic_params(
        [],
        types.into_iter().map(|ty| ty.to_token_stream_replacing_by_self(trait_record)),
    )
}

// TODO(jeanpierreda): These functions are at a weird level of abstraction (using
// ir::Record). It's possible that, instead, we should just ask "does the
// RsTypeKind implement clone" (etc.).
//
// Otherwise, these functions should be moved into a separate module.

pub fn should_derive_clone(record: &Record) -> bool {
    if record.is_union() {
        // `union`s (unlike `struct`s) should only derive `Clone` if they are `Copy`.
        should_derive_copy(record)
    } else {
        record.is_unpin()
            && record.copy_constructor == SpecialMemberFunc::Trivial
            && check_by_value(record).is_ok()
    }
}

pub fn should_derive_copy(record: &Record) -> bool {
    // TODO(b/202258760): Make `Copy` inclusion configurable.
    record.is_unpin()
        && record.copy_constructor == SpecialMemberFunc::Trivial
        && record.destructor == ir::SpecialMemberFunc::Trivial
        && check_by_value(record).is_ok()
}

pub fn check_by_value(record: &Record) -> Result<()> {
    if record.destructor == SpecialMemberFunc::Unavailable {
        bail!(
            "Can't directly construct values of type `{}` as it has a non-public or deleted destructor",
            record.cc_name.as_ref()
        )
    }
    if record.is_abstract {
        bail!(
            "Can't directly construct values of type `{}`: it is abstract",
            record.cc_name.as_ref()
        );
    }
    Ok(())
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    /// (), void
    Unit,
    bool,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    usize,
    isize,
    f32,
    f64,
    c_uchar,
    c_schar,
    c_ushort,
    c_short,
    c_uint,
    c_int,
    c_ulong,
    c_long,
    c_ulonglong,
    c_longlong,
}

impl PrimitiveType {
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "()" => Self::Unit,
            "bool" => Self::bool,
            "u8" => Self::u8,
            "i8" => Self::i8,
            "u16" => Self::u16,
            "i16" => Self::i16,
            "u32" => Self::u32,
            "i32" => Self::i32,
            "u64" => Self::u64,
            "i64" => Self::i64,
            "usize" => Self::usize,
            "isize" => Self::isize,
            "f32" => Self::f32,
            "f64" => Self::f64,
            "::core::ffi::c_uchar" => Self::c_uchar,
            "::core::ffi::c_schar" => Self::c_schar,
            "::core::ffi::c_ushort" => Self::c_ushort,
            "::core::ffi::c_short" => Self::c_short,
            "::core::ffi::c_uint" => Self::c_uint,
            "::core::ffi::c_int" => Self::c_int,
            "::core::ffi::c_ulong" => Self::c_ulong,
            "::core::ffi::c_long" => Self::c_long,
            "::core::ffi::c_ulonglong" => Self::c_ulonglong,
            "::core::ffi::c_longlong" => Self::c_longlong,
            _ => return None,
        })
    }
}

impl ToTokens for PrimitiveType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            // This doesn't affect void in function return values, as those are special-cased to be
            // omitted.
            Self::Unit => quote! {::core::ffi::c_void},
            Self::bool => quote! {bool},
            Self::u8 => quote! {u8},
            Self::i8 => quote! {i8},
            Self::u16 => quote! {u16},
            Self::i16 => quote! {i16},
            Self::u32 => quote! {u32},
            Self::i32 => quote! {i32},
            Self::u64 => quote! {u64},
            Self::i64 => quote! {i64},
            Self::usize => quote! {usize},
            Self::isize => quote! {isize},
            Self::f32 => quote! {f32},
            Self::f64 => quote! {f64},
            Self::c_uchar => quote! {::core::ffi::c_uchar},
            Self::c_schar => quote! {::core::ffi::c_schar},
            Self::c_ushort => quote! {::core::ffi::c_ushort},
            Self::c_short => quote! {::core::ffi::c_short},
            Self::c_uint => quote! {::core::ffi::c_uint},
            Self::c_int => quote! {::core::ffi::c_int},
            Self::c_ulong => quote! {::core::ffi::c_ulong},
            Self::c_long => quote! {::core::ffi::c_long},
            Self::c_ulonglong => quote! {::core::ffi::c_ulonglong},
            Self::c_longlong => quote! {::core::ffi::c_longlong},
        }
        .to_tokens(tokens)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RsTypeKind {
    Pointer {
        pointee: Rc<RsTypeKind>,
        mutability: Mutability,
    },
    Reference {
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
    },
    TypeAlias {
        type_alias: Rc<TypeAlias>,
        underlying_type: Rc<RsTypeKind>,
        crate_path: Rc<CratePath>,
    },
    Primitive(PrimitiveType),
    Other {
        name: Rc<str>,
        type_args: Rc<[RsTypeKind]>,
        is_same_abi: bool,
    },
}

impl RsTypeKind {
    pub fn new_record(record: Rc<Record>, ir: &IR) -> Result<Self> {
        let crate_path = Rc::new(CratePath::new(
            ir,
            ir.namespace_qualifier(&record)?,
            rs_imported_crate_name(&record.owning_target, ir),
        ));
        Ok(RsTypeKind::Record { record, crate_path })
    }

    pub fn new_type_map_override(type_map_override: &TypeMapOverride) -> Self {
        RsTypeKind::Other {
            name: type_map_override.rs_name.clone(),
            type_args: Rc::from([]),
            is_same_abi: type_map_override.is_same_abi,
        }
    }

    /// Returns true if the type is known to be `Unpin`, false otherwise.
    pub fn is_unpin(&self) -> bool {
        match self {
            RsTypeKind::IncompleteRecord { .. } => false,
            RsTypeKind::Record { record, .. } => record.is_unpin(),
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.is_unpin(),
            _ => true,
        }
    }

    /// Returns the features required to use this type.
    ///
    /// If a function accepts or returns this type, or an alias refers to this
    /// type, then the function or type alias will itself also require this
    /// feature. However, in the case of fields inside compound data types,
    /// only those fields require the feature, not the entire type.
    pub fn required_crubit_features(&self, ir: &IR) -> Result<flagset::FlagSet<CrubitFeature>> {
        /// Required features, sans recursion.
        fn required_crubit_features_flat(
            rs_type_kind: &RsTypeKind,
            ir: &IR,
        ) -> Result<flagset::FlagSet<CrubitFeature>> {
            match rs_type_kind {
                RsTypeKind::Pointer { .. } => Ok(CrubitFeature::ExternC.into()),
                RsTypeKind::Reference { .. } | RsTypeKind::RvalueReference { .. } => {
                    Ok(CrubitFeature::Experimental.into())
                }
                // TODO(b/314382764): Carve out some function pointer types that can be ExternC.
                RsTypeKind::FuncPtr { .. } => Ok(CrubitFeature::Experimental.into()),
                RsTypeKind::IncompleteRecord { .. } => Ok(CrubitFeature::Experimental.into()),
                // Here, we can very carefully be non-recursive into the _structure_ of the type.
                //
                // Whether a record type is supported in rust does _not_ depend on whether each
                // field is supported in Rust -- we can, if those fields are unsupported, replace
                // them with opaque blobs.
                //
                // Instead, what matters is the abstract properties of the struct itself!
                RsTypeKind::Record { record, .. } => {
                    let record = RsTypeKind::new_record(record.clone(), ir)?;
                    if record.is_unpin() {
                        Ok(CrubitFeature::ExternC.into())
                    } else {
                        Ok(CrubitFeature::Experimental.into())
                    }
                }
                // TODO(b/314382764): Carve out some aliases that can be ExternC.
                RsTypeKind::TypeAlias { .. } => Ok(CrubitFeature::Experimental.into()),
                RsTypeKind::Primitive { .. } => Ok(CrubitFeature::ExternC.into()),
                RsTypeKind::Other { .. } => Ok(CrubitFeature::Experimental.into()),
            }
        }

        let mut features = flagset::FlagSet::<CrubitFeature>::default();
        for rs_type_kind in self.dfs_iter() {
            features |= required_crubit_features_flat(rs_type_kind, ir)?;
        }
        Ok(features)
    }

    /// Returns true if the type can be passed by value through `extern "C"` ABI
    /// thunks.
    pub fn is_c_abi_compatible_by_value(&self) -> bool {
        match self {
            RsTypeKind::TypeAlias { underlying_type, .. } => {
                underlying_type.is_c_abi_compatible_by_value()
            }
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
            RsTypeKind::Other { is_same_abi, .. } => *is_same_abi,
            _ => true,
        }
    }

    /// Returns true if the type is known to be move-constructible, false
    /// otherwise.
    ///
    /// For the purposes of this method, references are considered
    /// move-constructible (as if they were pointers).
    pub fn is_move_constructible(&self) -> bool {
        match self {
            RsTypeKind::IncompleteRecord { .. } => false,
            RsTypeKind::Record { record, .. } => {
                record.move_constructor != ir::SpecialMemberFunc::Unavailable
            }
            RsTypeKind::TypeAlias { underlying_type, .. } => {
                underlying_type.is_move_constructible()
            }
            _ => true,
        }
    }

    /// Returns Ok if the type can be used by value, or an error describing why
    /// it can't.
    pub fn check_by_value(&self) -> Result<()> {
        match self {
            RsTypeKind::Record { record, .. } => check_by_value(record),
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.check_by_value(),
            _ => Ok(()),
        }
    }

    pub fn format_as_return_type_fragment(&self, self_record: Option<&Record>) -> TokenStream {
        match self {
            RsTypeKind::Primitive(PrimitiveType::Unit) => quote! {},
            other_type => {
                let other_type_ = other_type.to_token_stream_replacing_by_self(self_record);
                quote! { -> #other_type_ }
            }
        }
    }

    /// Formats this RsTypeKind as `&'a mut MaybeUninit<SomeStruct>`. This is
    /// used to format `__this` parameter in a constructor thunk.
    pub fn format_mut_ref_as_uninitialized(&self) -> Result<TokenStream> {
        match self {
            RsTypeKind::Reference { referent, lifetime, mutability: Mutability::Mut } => {
                let lifetime = lifetime.format_for_reference();
                Ok(quote! { & #lifetime mut ::core::mem::MaybeUninit< #referent > })
            }
            _ => bail!("Expected reference to format as MaybeUninit, got: {:?}", self),
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
            RsTypeKind::Reference { referent, lifetime, mutability } => {
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
                let arbitrary_self_types = make_rs_ident("arbitrary_self_types");
                // TODO(b/239661934): Add `use ::ctor::{RvalueReference, ConstRvalueReference}`.
                match mutability {
                    Mutability::Mut => Ok(RsSnippet {
                        tokens: quote! {self: ::ctor::RvalueReference<#lifetime, Self>},
                        features: [arbitrary_self_types].into_iter().collect(),
                    }),
                    Mutability::Const => Ok(RsSnippet {
                        tokens: quote! {self: ::ctor::ConstRvalueReference<#lifetime, Self>},
                        features: [arbitrary_self_types].into_iter().collect(),
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
        // TODO(b/212696226): Verify results of `implements_copy` via static
        // assertions in the generated Rust code (because incorrect results
        // can silently lead to unsafe behavior).
        match self {
            RsTypeKind::Primitive { .. } => true,
            RsTypeKind::Pointer { .. } => true,
            RsTypeKind::FuncPtr { .. } => true,
            RsTypeKind::Reference { mutability: Mutability::Const, .. } => true,
            RsTypeKind::Reference { mutability: Mutability::Mut, .. } => false,
            RsTypeKind::RvalueReference { .. } => false,
            RsTypeKind::IncompleteRecord { .. } => false,
            RsTypeKind::Record { record, .. } => should_derive_copy(record),
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.implements_copy(),
            RsTypeKind::Other { type_args, .. } => {
                // All types that may appear here without `type_args` (e.g.
                // primitive types like `i32`) implement `Copy`. Generic types
                // that may be present here (e.g. Option<...>) are `Copy` if all
                // of their `type_args` are `Copy`.
                type_args.iter().all(|t| t.implements_copy())
            }
        }
    }

    pub fn is_ref_to(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Reference { referent, .. } => referent.is_record(expected_record),
            RsTypeKind::RvalueReference { referent, .. } => referent.is_record(expected_record),
            _ => false,
        }
    }

    pub fn is_shared_ref_to(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Reference { referent, mutability: Mutability::Const, .. } => {
                referent.is_record(expected_record)
            }
            _ => false,
        }
    }

    pub fn is_record(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Record { record: actual_record, .. } => {
                actual_record.id == expected_record.id
            }
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            RsTypeKind::Primitive(PrimitiveType::bool) => true,
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.is_bool(),
            _ => false,
        }
    }

    /// Iterates over `self` and all the nested types (e.g. pointees, generic
    /// type args, etc.) in DFS order.
    pub fn dfs_iter(&self) -> impl Iterator<Item = &RsTypeKind> + '_ {
        RsTypeKindIter::new(self)
    }

    /// Iterates over all `LifetimeId`s in `self` and in all the nested types.
    /// Note that the results might contain duplicate LifetimeId values (e.g.
    /// if the same LifetimeId is used in two `type_args`).
    pub fn lifetimes(&self) -> impl Iterator<Item = Lifetime> + '_ {
        self.dfs_iter().filter_map(Self::lifetime)
    }

    /// Returns the pointer or reference target.
    pub fn referent(&self) -> Option<&RsTypeKind> {
        match self {
            Self::Pointer { pointee: p, .. }
            | Self::Reference { referent: p, .. }
            | Self::RvalueReference { referent: p, .. } => Some(&**p),
            _ => None,
        }
    }

    /// Returns the reference lifetime, or None if this is not a reference.
    pub fn lifetime(&self) -> Option<Lifetime> {
        match self {
            Self::Reference { lifetime, .. } | Self::RvalueReference { lifetime, .. } => {
                Some(lifetime.clone())
            }
            _ => None,
        }
    }
    /// Similar to to_token_stream, but replacing RsTypeKind:Record with Self
    /// when the underlying Record matches the given one.
    pub fn to_token_stream_replacing_by_self(&self, self_record: Option<&Record>) -> TokenStream {
        match self {
            RsTypeKind::Pointer { pointee, mutability } => {
                let mutability = mutability.format_for_pointer();
                let pointee_ = pointee.to_token_stream_replacing_by_self(self_record);
                quote! {* #mutability #pointee_}
            }
            RsTypeKind::Reference { referent, mutability, lifetime } => {
                let mut_ = mutability.format_for_reference();
                let lifetime = lifetime.format_for_reference();
                let referent_ = referent.to_token_stream_replacing_by_self(self_record);
                let reference = quote! {& #lifetime #mut_ #referent_};
                if mutability == &Mutability::Mut && !referent.is_unpin() {
                    // TODO(b/239661934): Add a `use ::core::pin::Pin` to the crate, and use
                    // `Pin`. This either requires deciding how to qualify pin at
                    // RsTypeKind-creation time, or returning a non-TokenStream type from here (and
                    // not implementing ToTokens, but instead some other interface.)
                    quote! {::core::pin::Pin< #reference >}
                } else {
                    reference
                }
            }
            RsTypeKind::RvalueReference { referent, mutability, lifetime } => {
                let referent_ = referent.to_token_stream_replacing_by_self(self_record);
                // TODO(b/239661934): Add a `use ::ctor::RvalueReference` (etc.) to the crate.
                if mutability == &Mutability::Mut {
                    quote! {::ctor::RvalueReference<#lifetime, #referent_>}
                } else {
                    quote! {::ctor::ConstRvalueReference<#lifetime, #referent_>}
                }
            }
            RsTypeKind::FuncPtr { abi, return_type, param_types } => {
                let param_types_: Vec<TokenStream> = param_types
                    .iter()
                    .map(|type_| type_.to_token_stream_replacing_by_self(self_record))
                    .collect();
                let return_frag = return_type.format_as_return_type_fragment(self_record);
                quote! { extern #abi fn( #( #param_types_ ),* ) #return_frag }
            }
            RsTypeKind::Record { record, crate_path } => {
                if self_record == Some(record) {
                    quote! { Self }
                } else {
                    let ident = make_rs_ident(record.rs_name.as_ref());
                    quote! { #crate_path #ident }
                }
            }
            RsTypeKind::Other { name, type_args, .. } => {
                let name: TokenStream = name.parse().expect("Invalid RsType::name in the IR");
                let generic_params =
                    format_generic_params_replacing_by_self(type_args.iter(), self_record);
                quote! {#name #generic_params}
            }
            _ => self.to_token_stream(),
        }
    }
}

impl std::fmt::Display for RsTypeKind {
    // Formats the token stream of the RsTypeKind to a string. Note that this can
    // include extra whitespace, where we'd ideally remove it, but it is hard to
    // remove whitespace without invoking rustfmt.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match write_unformatted_tokens(f, self.to_token_stream()) {
            Ok(_) => Ok(()),
            Err(e) => {
                // Honestly this should never happen, but we should spit out something.
                write!(f, "<error: {e}>")
            }
        }
    }
}

impl ToTokens for RsTypeKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_token_stream().to_tokens(tokens)
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            RsTypeKind::Pointer { pointee, mutability } => {
                let mutability = mutability.format_for_pointer();
                quote! {* #mutability #pointee}
            }
            RsTypeKind::Reference { referent, mutability, lifetime } => {
                let mut_ = mutability.format_for_reference();
                let lifetime = lifetime.format_for_reference();
                let reference = quote! {& #lifetime #mut_ #referent};
                if mutability == &Mutability::Mut && !referent.is_unpin() {
                    // TODO(b/239661934): Add a `use ::core::pin::Pin` to the crate, and use
                    // `Pin`. This either requires deciding how to qualify pin at
                    // RsTypeKind-creation time, or returning a non-TokenStream type from here (and
                    // not implementing ToTokens, but instead some other interface.)
                    quote! {::core::pin::Pin< #reference >}
                } else {
                    reference
                }
            }
            RsTypeKind::RvalueReference { referent, mutability, lifetime } => {
                // TODO(b/239661934): Add a `use ::ctor::RvalueReference` (etc.) to the crate.
                if mutability == &Mutability::Mut {
                    quote! {::ctor::RvalueReference<#lifetime, #referent>}
                } else {
                    quote! {::ctor::ConstRvalueReference<#lifetime, #referent>}
                }
            }
            RsTypeKind::FuncPtr { abi, return_type, param_types } => {
                let return_frag = return_type.format_as_return_type_fragment(None);
                quote! { extern #abi fn( #( #param_types ),* ) #return_frag }
            }
            RsTypeKind::IncompleteRecord { incomplete_record, crate_path } => {
                let record_ident = make_rs_ident(incomplete_record.rs_name.as_ref());
                quote! { #crate_path #record_ident }
            }
            RsTypeKind::Record { record, crate_path } => {
                let ident = make_rs_ident(record.rs_name.as_ref());
                quote! { #crate_path #ident }
            }
            RsTypeKind::TypeAlias { type_alias, crate_path, .. } => {
                let ident = make_rs_ident(&type_alias.identifier.identifier);
                quote! { #crate_path #ident }
            }
            RsTypeKind::Primitive(primitive) => quote! {#primitive},
            RsTypeKind::Other { name, type_args, .. } => {
                let name: TokenStream = name.parse().expect("Invalid RsType::name in the IR");
                let generic_params =
                    format_generic_params(/* lifetimes= */ &[], type_args.iter());
                quote! {#name #generic_params}
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
                    RsTypeKind::Primitive { .. }
                    | RsTypeKind::IncompleteRecord { .. }
                    | RsTypeKind::Record { .. } => {}
                    RsTypeKind::Pointer { pointee, .. } => self.todo.push(pointee),
                    RsTypeKind::Reference { referent, .. } => self.todo.push(referent),
                    RsTypeKind::RvalueReference { referent, .. } => self.todo.push(referent),
                    RsTypeKind::TypeAlias { underlying_type: t, .. } => self.todo.push(t),
                    RsTypeKind::FuncPtr { return_type, param_types, .. } => {
                        self.todo.push(return_type);
                        self.todo.extend(param_types.iter().rev());
                    }
                    RsTypeKind::Other { type_args, .. } => self.todo.extend(type_args.iter().rev()),
                };
                Some(curr)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use token_stream_matchers::assert_rs_matches;

    #[test]
    fn test_dfs_iter_ordering() {
        // Set up a test input representing: A<B<C>, D<E>>.
        let a = {
            let b = {
                let c = RsTypeKind::Other {
                    name: "C".into(),
                    type_args: Rc::from([]),
                    is_same_abi: true,
                };
                RsTypeKind::Other { name: "B".into(), type_args: Rc::from([c]), is_same_abi: true }
            };
            let d = {
                let e = RsTypeKind::Other {
                    name: "E".into(),
                    type_args: Rc::from([]),
                    is_same_abi: true,
                };
                RsTypeKind::Other { name: "D".into(), type_args: Rc::from([e]), is_same_abi: true }
            };
            RsTypeKind::Other { name: "A".into(), type_args: Rc::from([b, d]), is_same_abi: true }
        };
        let dfs_names = a
            .dfs_iter()
            .map(|t| match t {
                RsTypeKind::Other { name, .. } => &**name,
                _ => unreachable!("Only 'other' types are used in this test"),
            })
            .collect_vec();
        assert_eq!(vec!["A", "B", "C", "D", "E"], dfs_names);
    }

    #[test]
    fn test_dfs_iter_ordering_for_func_ptr() {
        // Set up a test input representing: fn(A, B) -> C
        let f = {
            let a = RsTypeKind::Other {
                name: "A".into(),
                type_args: Rc::from(&[][..]),
                is_same_abi: true,
            };
            let b = RsTypeKind::Other {
                name: "B".into(),
                type_args: Rc::from(&[][..]),
                is_same_abi: true,
            };
            let c = RsTypeKind::Other {
                name: "C".into(),
                type_args: Rc::from(&[][..]),
                is_same_abi: true,
            };
            RsTypeKind::FuncPtr {
                abi: "blah".into(),
                param_types: Rc::from([a, b]),
                return_type: Rc::new(c),
            }
        };
        let dfs_names = f
            .dfs_iter()
            .map(|t| match t {
                RsTypeKind::FuncPtr { .. } => "fn",
                RsTypeKind::Other { name, .. } => &**name,
                _ => unreachable!("Only FuncPtr and Other kinds are used in this test"),
            })
            .collect_vec();
        assert_eq!(vec!["fn", "A", "B", "C"], dfs_names);
    }

    #[test]
    fn test_lifetime_elision_for_references() {
        let type_args: &[RsTypeKind] = &[];
        let referent = Rc::new(RsTypeKind::Other {
            name: "T".into(),
            type_args: type_args.into(),
            is_same_abi: true,
        });
        let reference = RsTypeKind::Reference {
            referent,
            mutability: Mutability::Const,
            lifetime: Lifetime::new("_"),
        };
        assert_rs_matches!(quote! {#reference}, quote! {&T});
    }

    #[test]
    fn test_lifetime_elision_for_rvalue_references() {
        let type_args: &[RsTypeKind] = &[];
        let referent = Rc::new(RsTypeKind::Other {
            name: "T".into(),
            type_args: type_args.into(),
            is_same_abi: true,
        });
        let reference = RsTypeKind::RvalueReference {
            referent,
            mutability: Mutability::Mut,
            lifetime: Lifetime::new("_"),
        };
        assert_rs_matches!(quote! {#reference}, quote! {RvalueReference<'_, T>});
    }

    #[test]
    fn test_format_as_self_param_rvalue_reference() -> Result<()> {
        let type_args: &[RsTypeKind] = &[];
        let referent = Rc::new(RsTypeKind::Other {
            name: "T".into(),
            type_args: type_args.into(),
            is_same_abi: true,
        });
        let result = RsTypeKind::RvalueReference {
            referent,
            mutability: Mutability::Mut,
            lifetime: Lifetime::new("a"),
        }
        .format_as_self_param()?;
        assert_rs_matches!(result.tokens, quote! {self: ::ctor::RvalueReference<'a, Self>});
        assert_eq!(result.features, [make_rs_ident("arbitrary_self_types")].into_iter().collect());
        Ok(())
    }

    #[test]
    fn test_format_as_self_param_const_rvalue_reference() -> Result<()> {
        let type_args: &[RsTypeKind] = &[];
        let referent = Rc::new(RsTypeKind::Other {
            name: "T".into(),
            type_args: type_args.into(),
            is_same_abi: true,
        });
        let result = RsTypeKind::RvalueReference {
            referent,
            mutability: Mutability::Const,
            lifetime: Lifetime::new("a"),
        }
        .format_as_self_param()?;
        assert_rs_matches!(result.tokens, quote! {self: ::ctor::ConstRvalueReference<'a, Self>});
        assert_eq!(result.features, [make_rs_ident("arbitrary_self_types")].into_iter().collect());
        Ok(())
    }
}
