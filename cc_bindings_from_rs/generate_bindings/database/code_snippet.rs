// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_abi;
extern crate rustc_middle;
extern crate rustc_span;

use crate::FineGrainedFeature;
use arc_anyhow::Result;
use code_gen_utils::CcInclude;
use crubit_abi_type::CrubitAbiType;
use error_report::bail;
use itertools::Itertools;
use proc_macro2::TokenStream;
use rustc_middle::ty::Ty;
use rustc_span::def_id::DefId;
use rustc_span::Symbol;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fmt;
use std::ops::{Add, AddAssign};

/// A CrubitAbiType that has C++ prerequisites.
/// This type just serves as a named pair for CrubitAbiType and CcPrerequisites, and is useful
/// for when we build up a CrubitAbiType where some of the inner types (e.g. std::string_view)
/// might have C++ prerequisites (e.g. <string>) that need to be included to work.
#[derive(Clone)]
pub struct CrubitAbiTypeWithCcPrereqs<'tcx> {
    pub crubit_abi_type: CrubitAbiType,
    pub prereqs: CcPrerequisites<'tcx>,
}

impl<'tcx> CrubitAbiTypeWithCcPrereqs<'tcx> {
    /// Extracts the CrubitAbiType and adds the C++ prerequisites to the given `prereqs`.
    pub fn crubit_abi_type(self, prereqs: &mut CcPrerequisites<'tcx>) -> CrubitAbiType {
        *prereqs += self.prereqs;
        self.crubit_abi_type
    }
}
impl<'tcx> From<CrubitAbiType> for CrubitAbiTypeWithCcPrereqs<'tcx> {
    fn from(crubit_abi_type: CrubitAbiType) -> Self {
        Self { crubit_abi_type, prereqs: Default::default() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CcPrerequisites<'tcx> {
    /// Set of `#include`s that a `CcSnippet` depends on.  For example if
    /// `CcSnippet::tokens` expands to `std::int32_t`, then `includes`
    /// need to cover the `#include <cstdint>`.
    pub includes: BTreeSet<CcInclude>,

    /// Set of local definitions that a `CcSnippet` depends on.  For example if
    /// `CcSnippet::tokens` expands to `void foo(S s) { ... }` then the
    /// definition of `S` should have appeared earlier - in this case `defs`
    /// will include the `DefId` corresponding to `S`.  Note that the
    /// definition of `S` is covered by `ApiSnippets::main_api` (i.e. the
    /// predecessor of a toposort edge is `ApiSnippets::main_api` - it is not
    /// possible to depend on `ApiSnippets::cc_details`).
    pub defs: HashSet<DefId>,

    /// Set of forward declarations that a `CcSnippet` depends on.  For example
    /// if `CcSnippet::tokens` expands to `void foo(S* s)` then a forward
    /// declaration of `S` should have appeared earlier - in this case
    /// `fwd_decls` will include the `DefId` corresponding to `S`.
    /// Note that in this particular example the *definition* of `S` does
    /// *not* need to appear earlier (and therefore `defs` will *not*
    /// contain `DefId` corresponding to `S`).
    pub fwd_decls: HashSet<DefId>,

    /// Set of Crubit feature flags required for the CcSnippet to be valid.
    pub required_features: flagset::FlagSet<FineGrainedFeature>,

    // Set of template specializations our snippet requires.
    pub template_specializations: HashSet<TemplateSpecialization<'tcx>>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum TemplateSpecialization<'tcx> {
    /// Our layout compatible type for `std::option::Option`.
    RsStdOption { arg_ty: Ty<'tcx>, self_ty: Ty<'tcx> },
}

impl<'tcx> CcPrerequisites<'tcx> {
    pub fn is_empty(&self) -> bool {
        let Self { includes, defs, fwd_decls, required_features, template_specializations } = self;
        includes.is_empty()
            && defs.is_empty()
            && fwd_decls.is_empty()
            && required_features.is_empty()
            && template_specializations.is_empty()
    }

    /// Weakens all dependencies to only require a forward declaration. Example
    /// usage scenarios:
    /// - Computing prerequisites of pointer types (the pointee type can just be
    ///   forward-declared),
    /// - Computing prerequisites of function declarations (parameter types and
    ///   return type can just be forward-declared).
    pub fn move_defs_to_fwd_decls(&mut self) {
        self.fwd_decls.extend(std::mem::take(&mut self.defs))
    }
}

impl<'tcx> AddAssign for CcPrerequisites<'tcx> {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, rhs: Self) {
        let Self { mut includes, defs, fwd_decls, required_features, template_specializations } =
            rhs;

        // `BTreeSet::append` is used because it _seems_ to be more efficient than
        // calling `extend`.  This is because `extend` takes an iterator
        // (processing each `rhs` include one-at-a-time) while `append` steals
        // the whole backing data store from `rhs.includes`. OTOH, this is a bit
        // speculative, since the (expected / guessed) performance difference is
        // not documented at
        // https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.append
        self.includes.append(&mut includes);

        self.defs.extend(defs);
        self.fwd_decls.extend(fwd_decls);
        self.required_features |= required_features;
        self.template_specializations.extend(template_specializations);
    }
}

impl<'tcx> Add for CcPrerequisites<'tcx> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

#[derive(Clone, Default)]
pub struct CcSnippet<'tcx> {
    pub tokens: TokenStream,
    pub prereqs: CcPrerequisites<'tcx>,
}
// Override debug to use the Display impl for tokens, as the Debug impl for TokenStream is rarely
// useful (it shows the structure of the tokens, not the actual text).
impl<'tcx> fmt::Debug for CcSnippet<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("CcSnippet")
            .field("tokens", &self.tokens.to_string())
            .field("prereqs", &self.prereqs)
            .finish()
    }
}

impl<'tcx> CcSnippet<'tcx> {
    /// Consumes `self` and returns its `tokens`, while preserving
    /// its `prereqs` into `prereqs_accumulator`.
    pub fn into_tokens(self, prereqs_accumulator: &mut CcPrerequisites<'tcx>) -> TokenStream {
        let Self { tokens, prereqs } = self;
        *prereqs_accumulator += prereqs;
        tokens
    }

    /// Creates a new CcSnippet (with no `CcPrerequisites`).
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens, ..Default::default() }
    }

    /// Creates a CcSnippet that depends on a single `CcInclude`.
    pub fn with_include(tokens: TokenStream, include: CcInclude) -> Self {
        let mut prereqs = CcPrerequisites::default();
        prereqs.includes.insert(include);
        Self { tokens, prereqs }
    }

    /// Resolves the feature requirements. If the required features of `self`
    /// are in `crubit_features`, then this returns a version of `self` with
    /// the feature requirements removed. Otherwise, this returns an error.
    pub fn resolve_feature_requirements(
        mut self,
        crubit_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    ) -> Result<Self> {
        let mut errs = Vec::new();
        for feature in self.prereqs.required_features {
            if let Err(e) = feature.ensure_crubit_feature(crubit_features) {
                errs.push(e);
            }
        }
        match errs.len() {
            0 => {
                self.prereqs.required_features.clear();
                Ok(self)
            }
            1 => Err(errs.pop().unwrap()),
            _ => {
                let mut errs = errs.into_iter().map(|e| e.to_string());
                bail!(errs.join(", "))
            }
        }
    }

    pub fn into_main_api(self) -> ApiSnippets<'tcx> {
        ApiSnippets { main_api: self, ..Default::default() }
    }
}

impl<'tcx> AddAssign for CcSnippet<'tcx> {
    fn add_assign(&mut self, rhs: Self) {
        self.tokens.extend(rhs.into_tokens(&mut self.prereqs));
    }
}

/// Holds the declaration of an extern "C" function.
///
/// ADTs can be annotated with conversion functions that adhere to the C calling
/// conventions. Crubit needs to declare these functions within an extern "C"
/// block, so rustc knows that the conversion function is defined elsewhere.
///
/// This type implements PartialEq and PartialOrd based on the `symbol` field.
/// The `decl` field is ignored. All `ExternCDecl` instances from all thunks are
/// eventually put in BTreeSet to remove duplicates and to achieve deterministic
/// ordering of the generated extern "C" { ... } block.
#[derive(Clone, Debug)]
pub struct ExternCDecl {
    /// The name of the function.
    pub symbol: Symbol,

    /// The full function declaration as can be placed in an extern "C" block.
    pub decl: TokenStream,
}

impl Ord for ExternCDecl {
    fn cmp(&self, other: &Self) -> Ordering {
        self.symbol.cmp(&other.symbol)
    }
}

impl PartialOrd for ExternCDecl {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExternCDecl {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl Eq for ExternCDecl {}

#[derive(Clone, Default)]
pub struct RsSnippet {
    pub tokens: TokenStream,

    /// Set of extern "C" declarations needed by `tokens`.
    pub extern_c_decls: BTreeSet<ExternCDecl>,
}

// Override debug to use the Display impl for tokens, as the Debug impl for TokenStream is rarely
// useful (it shows the structure of the tokens, not the actual text).
impl fmt::Debug for RsSnippet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("RsSnippet")
            .field("tokens", &self.tokens.to_string())
            .field("extern_c_decls", &self.extern_c_decls)
            .finish()
    }
}

impl FromIterator<RsSnippet> for RsSnippet {
    fn from_iter<I: IntoIterator<Item = RsSnippet>>(iter: I) -> Self {
        let mut result = RsSnippet::default();
        for RsSnippet { tokens, extern_c_decls } in iter.into_iter() {
            result.tokens.extend(tokens);
            result.extern_c_decls.extend(extern_c_decls);
        }
        result
    }
}

impl RsSnippet {
    // Creates a new RsSnippet from a TokenStream.
    pub fn new(tokens: TokenStream) -> Self {
        Self { tokens, ..Default::default() }
    }

    /// Consumes `self` and returns its `tokens`, while preserving
    /// its `extern_c_decls` into `extern_c_decls_accumulator`.
    pub fn into_tokens(
        self,
        extern_c_decls_accumulator: &mut BTreeSet<ExternCDecl>,
    ) -> TokenStream {
        extern_c_decls_accumulator.extend(self.extern_c_decls);
        self.tokens
    }
}

impl AddAssign for RsSnippet {
    fn add_assign(&mut self, rhs: Self) {
        self.tokens.extend(rhs.tokens);
        self.extern_c_decls.extend(rhs.extern_c_decls);
    }
}

#[derive(Clone, Debug, Default)]
pub struct ApiSnippets<'tcx> {
    /// Main API - for example:
    /// - A C++ declaration of a function (with a doc comment),
    /// - A C++ definition of a struct (with a doc comment).
    pub main_api: CcSnippet<'tcx>,

    /// C++ implementation details - for example:
    /// - A C++ declaration of an `extern "C"` thunk,
    /// - C++ `static_assert`s about struct size, aligment, and field offsets.
    pub cc_details: CcSnippet<'tcx>,

    /// Rust implementation details - for example:
    /// - A Rust implementation of an `extern "C"` thunk,
    /// - Rust `assert!`s about struct size, aligment, and field offsets.
    pub rs_details: RsSnippet,
}

impl<'tcx> ApiSnippets<'tcx> {
    /// Resolves the feature requirements. If the required features of `self`
    /// are in `crubit_features`, then this returns a version of `self` with
    /// the feature requirements removed. Otherwise, this returns an error.
    pub fn resolve_feature_requirements(
        self,
        crubit_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    ) -> Result<Self> {
        Ok(Self {
            main_api: self.main_api.resolve_feature_requirements(crubit_features)?,
            cc_details: self.cc_details.resolve_feature_requirements(crubit_features)?,
            rs_details: self.rs_details,
        })
    }
}

impl<'tcx> FromIterator<ApiSnippets<'tcx>> for ApiSnippets<'tcx> {
    fn from_iter<I: IntoIterator<Item = ApiSnippets<'tcx>>>(iter: I) -> Self {
        let mut result = ApiSnippets::default();
        for ApiSnippets { main_api, cc_details, rs_details } in iter.into_iter() {
            result.main_api += main_api;
            result.cc_details += cc_details;
            result.rs_details += rs_details;
        }
        result
    }
}
