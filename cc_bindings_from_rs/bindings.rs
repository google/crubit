// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{anyhow, bail, ensure, Context, Result};
use code_gen_utils::{
    format_cc_ident, format_cc_includes, format_namespace_bound_cc_tokens, make_rs_ident,
    CcInclude, NamespaceQualifier,
};
use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use rustc_hir::definitions::{DefPathData, DisambiguatedDefPathData};
use rustc_hir::{AssocItemKind, ImplItemKind, ImplicitSelfKind, Item, ItemKind, Node, Unsafety};
use rustc_middle::dep_graph::DepContext;
use rustc_middle::mir::Mutability;
use rustc_middle::ty::{self, Ty, TyCtxt}; // See <internal link>/ty.html#import-conventions
use rustc_span::def_id::{DefId, LocalDefId, LOCAL_CRATE};
use rustc_span::symbol::Symbol;
use rustc_target::spec::abi::Abi;
use rustc_target::spec::PanicStrategy;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::iter::once;
use std::ops::AddAssign;
use std::rc::Rc;

pub struct Input<'tcx> {
    /// Compilation context for the crate that the bindings should be generated
    /// for.
    pub tcx: TyCtxt<'tcx>,

    /// Path to a the `crubit/support` directory in a format that should be used
    /// in the `#include` directives inside the generated C++ files.
    /// Example: "crubit/support".
    pub crubit_support_path: Rc<str>,

    // TODO(b/262878759): Provide a set of enabled/disabled Crubit features.
    pub _features: (),

    // TODO(b/258261328): Provide a map from crate name into C++ header path with crate bindings.
    pub _crate_to_include_map: (),
}

pub struct Output {
    pub h_body: TokenStream,
    pub rs_body: TokenStream,
}

pub fn generate_bindings(input: &Input) -> Result<Output> {
    match input.tcx.sess().panic_strategy() {
        PanicStrategy::Unwind => bail!("No support for panic=unwind strategy (b/254049425)"),
        PanicStrategy::Abort => (),
    };

    let top_comment = {
        let crate_name = input.tcx.crate_name(LOCAL_CRATE);
        let txt = format!(
            "Automatically @generated C++ bindings for the following Rust crate:\n\
             {crate_name}"
        );
        quote! { __COMMENT__ #txt __NEWLINE__ }
    };

    let Output { h_body, rs_body } = format_crate(input).unwrap_or_else(|err| {
        let txt = format!("Failed to generate bindings for the crate: {err}");
        let src = quote! { __COMMENT__ #txt };
        Output { h_body: src.clone(), rs_body: src }
    });

    let h_body = quote! {
        #top_comment

        // TODO(b/251445877): Replace `#pragma once` with include guards.
        __HASH_TOKEN__ pragma once __NEWLINE__
        __NEWLINE__

        #h_body
    };

    let rs_body = quote! {
        #top_comment

        // Rust warns about non-`#[repr(C)]` structs being used as parameter types or return
        // type of `extern "C"` functions (such as thunks that might be present in `rs_body`).
        // This warning makes sense, because in absence of a guaranteed / well-defined ABI
        // for this structs, one can't author C/C++ definitions compatible with that ABI.
        // Unless... the author is `cc_bindings_from_rs` invoked with exactly the same version
        // and cmdline flags as `rustc`.  Given this, we just disable warnings like the one
        // in the example below:
        //
        //   warning: `extern` fn uses type `DefaultReprPoint`, which is not FFI-safe
        //   --> .../cc_bindings_from_rs/test/structs/structs_cc_api_impl.rs:25:6
        //       |
        //    25 | ) -> structs::DefaultReprPoint {
        //       |      ^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
        //       |
        //       = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute...
        //       = note: this struct has unspecified layout
        //       = note: `#[warn(improper_ctypes_definitions)]` on by default
        #![allow(improper_ctypes_definitions)] __NEWLINE__
        __NEWLINE__

        #rs_body
    };

    Ok(Output { h_body, rs_body })
}

#[derive(Clone, Debug, Default)]
struct CcPrerequisites {
    /// Set of `#include`s that a `CcSnippet` depends on.  For example if
    /// `CcSnippet::tokens` expands to `std::int32_t`, then `includes`
    /// need to cover the `#include <cstdint>`.
    includes: BTreeSet<CcInclude>,

    /// Set of local definitions that a `CcSnippet` depends on.  For example if
    /// `CcSnippet::tokens` expands to `void foo(S s)` then the definition
    /// of `S` should have appeared earlier - in this case `defs` will
    /// include the `LocalDefId` corresponding to `S`.
    defs: HashSet<LocalDefId>,

    /// Set of forward declarations that a `CcSnippet` depends on.  For example
    /// if `CcSnippet::tokens` expands to `void foo(S* s)` then a forward
    /// declaration of `S` should have appeared earlier - in this case
    /// `fwd_decls` will include the `LocalDefId` corresponding to `S`.
    /// Note that in this particular example the *definition* of `S` does
    /// *not* need to appear earlier (and therefore `defs` will *not*
    /// contain `LocalDefId` corresponding to `S`).
    fwd_decls: HashSet<LocalDefId>,
}

impl CcPrerequisites {
    #[cfg(test)]
    fn is_empty(&self) -> bool {
        let &Self { ref includes, ref defs, ref fwd_decls } = self;
        includes.is_empty() && defs.is_empty() && fwd_decls.is_empty()
    }

    /// Weakens all dependencies to only require a forward declaration. Example
    /// usage scenarios:
    /// - Computing prerequisites of pointer types (the pointee type can just be
    ///   forward-declared),
    /// - Computing prerequisites of function declarations (parameter types and
    ///   return type can just be forward-declared).
    fn move_defs_to_fwd_decls(&mut self) {
        self.fwd_decls.extend(std::mem::take(&mut self.defs))
    }
}

impl AddAssign for CcPrerequisites {
    fn add_assign(&mut self, rhs: Self) {
        let Self { mut includes, defs, fwd_decls } = rhs;

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
    }
}

#[derive(Debug, Default)]
struct CcSnippet {
    tokens: TokenStream,
    prereqs: CcPrerequisites,
}

impl CcSnippet {
    /// Consumes `self` and returns its `tokens`, while preserving
    /// its `prereqs` into `prereqs_accumulator`.
    fn into_tokens(self, prereqs_accumulator: &mut CcPrerequisites) -> TokenStream {
        let Self { tokens, prereqs } = self;
        *prereqs_accumulator += prereqs;
        tokens
    }

    /// Creates a new CcSnippet (with no `CcPrerequisites`).
    fn new(tokens: TokenStream) -> Self {
        Self { tokens, ..Default::default() }
    }

    /// Creates a CcSnippet that depends on a single `CcInclude`.
    fn with_include(tokens: TokenStream, include: CcInclude) -> Self {
        let mut prereqs = CcPrerequisites::default();
        prereqs.includes.insert(include);
        Self { tokens, prereqs }
    }
}

/// Represents the fully qualified name of a Rust item (e.g. of a `struct` or a
/// function).
struct FullyQualifiedName {
    /// Name of the crate that defines the item.
    /// For example, this would be `std` for `std::cmp::Ordering`.
    krate: Symbol,

    /// Path to the module where the item is located.
    /// For example, this would be `cmp` for `std::cmp::Ordering`.
    /// The path may contain multiple modules - e.g. `foo::bar::baz`.
    mod_path: NamespaceQualifier,

    /// Name of the item.
    /// For example, this would be `Ordering` for `std::cmp::Ordering`.
    name: Symbol,
}

impl FullyQualifiedName {
    /// Computes a `FullyQualifiedName` for `def_id`.
    ///
    /// May panic if `def_id` is an invalid id, or identifies an unnamed item.
    /// Examples of supported items:
    /// - `Node::Item` with `ItemKind::Fn`
    /// - `Node::Item` with `ItemKind::Struct`
    /// - Any `Node::ImplItem` (e.g. `ImplItemKind::Fn`)
    /// Examples of unsupported items:
    /// - `Node::Item` with `ItemKind::Impl` (the `impl` block itself doesn't
    ///   have a name)
    // TODO(b/259724276): This function's results should be memoized.
    fn new(tcx: TyCtxt, def_id: DefId) -> Self {
        fn get_symbol(path_component: DisambiguatedDefPathData) -> Option<Symbol> {
            match path_component.data {
                DefPathData::TypeNs(symbol) | DefPathData::ValueNs(symbol) => Some(symbol),

                // `Impl` and `ImplTrait` variants can appear in `full_path` of
                // a method - e.g. (pseudocode): `module1::module2::impl::method`.
                // Return `None` to skip these when calculating the `mod_path`.
                DefPathData::Impl | DefPathData::ImplTrait => None,

                other_data => panic!("Unexpected `path_component`: {other_data:?}"),
            }
        }

        let krate = tcx.crate_name(def_id.krate);

        let mut full_path = tcx.def_path(def_id).data; // mod_path + name
        let name = full_path.pop().expect("At least the item's name should be present");
        let name = get_symbol(name).expect("Caller should ensure `def_id` maps to a named item");

        let mod_path = NamespaceQualifier::new(
            full_path.into_iter().filter_map(get_symbol).map(|s| Rc::<str>::from(s.as_str())),
        );

        Self { krate, mod_path, name }
    }

    fn format_for_cc(&self) -> Result<TokenStream> {
        let top_level_ns = format_cc_ident(self.krate.as_str())?;
        let ns_path = self.mod_path.format_for_cc()?;
        let name = format_cc_ident(self.name.as_str())?;
        Ok(quote! { :: #top_level_ns :: #ns_path #name })
    }

    fn format_for_rs(&self) -> TokenStream {
        let krate = make_rs_ident(self.krate.as_str());
        let mod_path = self.mod_path.format_for_rs();
        let name = make_rs_ident(self.name.as_str());
        quote! { :: #krate :: #mod_path #name }
    }
}

fn format_ret_ty_for_cc(input: &Input, ty: Ty) -> Result<CcSnippet> {
    let void = Ok(CcSnippet::new(quote! { void }));
    match ty.kind() {
        ty::TyKind::Never => void,  // `!`
        ty::TyKind::Tuple(types) if types.len() == 0 => void,  // `()`
        _ => format_ty_for_cc(input, ty),
    }
}

/// Formats an argument of a thunk.  For example:
/// - most primitive types are passed as-is - e.g. `123`
/// - structs need to be moved: `std::move(value)`
/// - in the future additional processing may be needed for other types (this is
///   speculative so please take these examples with a grain of salt):
///     - `&str`: utf-8 verification (see b/262580415)
///     - `&T`: calling into `crubit::MutRef::unsafe_get_ptr` (see b/258235219)
fn format_cc_thunk_arg<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, value: TokenStream) -> CcSnippet {
    if ty.is_copy_modulo_regions(tcx, ty::ParamEnv::empty()) {
        CcSnippet::new(value)
    } else {
        CcSnippet::with_include(quote! { std::move(#value) }, CcInclude::utility())
    }
}

/// Formats `ty` into a `CcSnippet` that represents how the type should be
/// spelled in a C++ declaration of a function parameter or field.
//
// TODO(b/259724276): This function's results should be memoized.
fn format_ty_for_cc(input: &Input, ty: Ty) -> Result<CcSnippet> {
    fn cstdint(tokens: TokenStream) -> CcSnippet {
        CcSnippet::with_include(tokens, CcInclude::cstdint())
    }
    fn keyword(tokens: TokenStream) -> CcSnippet {
        CcSnippet::new(tokens)
    }
    Ok(match ty.kind() {
        ty::TyKind::Never => {
            // TODO(b/254507801): Maybe translate into `crubit::Never`?
            bail!("The never type `!` is only supported as a return type (b/254507801)");
        },
        ty::TyKind::Tuple(types) => {
            if types.len() == 0 {
                // TODO(b/254507801): Maybe translate into `crubit::Unit`?
                bail!("`()` / `void` is only supported as a return type (b/254507801)");
            } else {
                // TODO(b/254099023): Add support for tuples.
                bail!("Tuples are not supported yet: {} (b/254099023)", ty);
            }
        }

        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#bool documents
        // that "Rust's bool has the same layout as C17's _Bool".  The details (e.g. size, valid
        // bit patterns) are implementation-defined, but this is okay, because `bool` in the
        // `extern "C"` functions in the generated `..._cc_api.h` will also be the C17's _Bool.
        ty::TyKind::Bool => keyword(quote! { bool }),

        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#fixed-width-floating-point-types
        // documents that "When the platforms' "math.h" header defines the __STDC_IEC_559__ macro,
        // Rust's floating-point types are safe to use directly in C FFI where the appropriate C
        // types are expected (f32 for float, f64 for double)."
        //
        // TODO(b/255768062): Generated bindings should explicitly check `__STDC_IEC_559__`
        ty::TyKind::Float(ty::FloatTy::F32) => keyword(quote! { float }),
        ty::TyKind::Float(ty::FloatTy::F64) => keyword(quote! { double }),

        // ABI compatibility and other details are described in the doc comments in
        // `crubit/support/rstd/char.h` and `crubit/support/rstd/char_test.cc` (search for "Layout
        // tests").
        ty::TyKind::Char => {
            let rstd_char_path = format!("{}/rstd/char.h", &*input.crubit_support_path);
            CcSnippet::with_include(
                quote! { rstd::Char },
                CcInclude::user_header(rstd_char_path.into()),
            )
        },

        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#isize-and-usize
        // documents that "Rust's signed and unsigned fixed-width integer types {i,u}{8,16,32,64}
        // have the same layout the C fixed-width integer types from the <stdint.h> header
        // {u,}int{8,16,32,64}_t. These fixed-width integer types are therefore safe to use
        // directly in C FFI where the corresponding C fixed-width integer types are expected.
        //
        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#layout-compatibility-with-c-native-integer-types
        // documents that "Rust does not support C platforms on which the C native integer type are
        // not compatible with any of Rust's fixed-width integer type (e.g. because of
        // padding-bits, lack of 2's complement, etc.)."
        ty::TyKind::Int(ty::IntTy::I8) => cstdint(quote!{ std::int8_t }),
        ty::TyKind::Int(ty::IntTy::I16) => cstdint(quote!{ std::int16_t }),
        ty::TyKind::Int(ty::IntTy::I32) => cstdint(quote!{ std::int32_t }),
        ty::TyKind::Int(ty::IntTy::I64) => cstdint(quote!{ std::int64_t }),
        ty::TyKind::Uint(ty::UintTy::U8) => cstdint(quote!{ std::uint8_t }),
        ty::TyKind::Uint(ty::UintTy::U16) => cstdint(quote!{ std::uint16_t }),
        ty::TyKind::Uint(ty::UintTy::U32) => cstdint(quote!{ std::uint32_t }),
        ty::TyKind::Uint(ty::UintTy::U64) => cstdint(quote!{ std::uint64_t }),

        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#isize-and-usize
        // documents that "The isize and usize types are [...] layout compatible with C's uintptr_t
        // and intptr_t types.".
        ty::TyKind::Int(ty::IntTy::Isize) => cstdint(quote!{ std::intptr_t }),
        ty::TyKind::Uint(ty::UintTy::Usize) => cstdint(quote!{ std::uintptr_t }),

        ty::TyKind::Int(ty::IntTy::I128) | ty::TyKind::Uint(ty::UintTy::U128) => {
            // Note that "the alignment of Rust's {i,u}128 is unspecified and allowed to
            // change" according to
            // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#fixed-width-integer-types
            //
            // TODO(b/254094650): Consider mapping this to Clang's (and GCC's) `__int128`
            // or to `absl::in128`.
            bail!("C++ doesn't have a standard equivalent of `{ty}` (b/254094650)");
        }

        ty::TyKind::Adt(adt, substs) => {
            ensure!(substs.len() == 0, "Generic types are not supported yet (b/259749095)");

            // Verify if definition of `ty` can be succesfully imported and bail otherwise.
            let def_id = adt.did();
            format_adt_core(input.tcx, def_id)
                .with_context(|| format!(
                        "Failed to generate bindings for the definition of `{ty}`"))?;

            let mut prereqs = CcPrerequisites::default();
            if def_id.krate == LOCAL_CRATE {
                prereqs.defs.insert(def_id.expect_local());
            } else {
                // TODO(b/258261328): Add `#include` of other crate's `..._cc_api.h`.
                bail!("Cross-crate dependencies are not supported yet (b/258261328)");
            };

            CcSnippet {
                tokens: FullyQualifiedName::new(input.tcx, def_id).format_for_cc()?,
                prereqs
            }
        },

        ty::TyKind::RawPtr(ty::TypeAndMut{ty, mutbl}) => {
            let const_qualifier = match mutbl {
                Mutability::Mut => quote!{},
                Mutability::Not => quote!{ const },
            };
            let CcSnippet{ tokens, mut prereqs } = format_ty_for_cc(input, *ty)
                .with_context(|| format!(
                        "Failed to format the pointee of the pointer type `{ty}`"))?;
            prereqs.move_defs_to_fwd_decls();
            CcSnippet {
                prereqs,
                tokens: quote!{ #const_qualifier #tokens * },
            }
        },

        // TODO(b/260268230, b/260729464): When recursively processing nested types (e.g. an
        // element type of an Array, a referent of a Ref, a parameter type of an FnPtr, etc), one
        // should also 1) propagate `CcPrerequisites::defs`, 2) cover `CcPrerequisites::defs` in
        // `test_format_ty_for_cc...`.  For ptr/ref it might be possible to use
        // `CcPrerequisites::move_defs_to_fwd_decls`.
        _ => bail!("The following Rust type is not supported yet: {ty}"),
    })
}

/// Formats `ty` for Rust - to be used in `..._cc_api_impl.rs` (e.g. as a type
/// of a parameter in a Rust thunk).  Because `..._cc_api_impl.rs` is a
/// distinct, separate crate, the returned `TokenStream` uses crate-qualified
/// names whenever necessary - for example: `target_crate::SomeStruct` rather
/// than just `SomeStruct`.
//
// TODO(b/259724276): This function's results should be memoized.
fn format_ty_for_rs(tcx: TyCtxt, ty: Ty) -> Result<TokenStream> {
    Ok(match ty.kind() {
        ty::TyKind::Bool
        | ty::TyKind::Float(_)
        | ty::TyKind::Char
        | ty::TyKind::Int(_)
        | ty::TyKind::Uint(_)
        | ty::TyKind::Never => ty
            .to_string()
            .parse()
            .expect("rustc_middle::ty::Ty::to_string() should produce no parsing errors"),
        ty::TyKind::Tuple(types) => {
            if types.len() == 0 {
                quote! { () }
            } else {
                // TODO(b/254099023): Add support for tuples.
                bail!("Tuples are not supported yet: {} (b/254099023)", ty);
            }
        }
        ty::TyKind::Adt(adt, substs) => {
            ensure!(substs.len() == 0, "Generic types are not supported yet (b/259749095)");
            FullyQualifiedName::new(tcx, adt.did()).format_for_rs()
        },
        ty::TyKind::RawPtr(ty::TypeAndMut{ty, mutbl}) => {
            let qualifier = match mutbl {
                Mutability::Mut => quote!{ mut },
                Mutability::Not => quote!{ const },
            };
            let ty = format_ty_for_rs(tcx, *ty)
                .with_context(|| format!(
                        "Failed to format the pointee of the pointer type `{ty}`"))?;
            quote!{ * #qualifier #ty }
        },
        _ => bail!("The following Rust type is not supported yet: {ty}"),
    })
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum SnippetKind {
    /// Main API - for example:
    /// - A C++ declaration of a function (with a doc comment),
    /// - A C++ definition of a struct (with a doc comment).
    MainApi,

    /// Implementation details - for example:
    /// - A C++ declaration of an `extern "C"` thunk,
    /// - A Rust implementation of an `extern "C"` thunk,
    /// - C++ or Rust assertions about struct size and aligment.
    ImplDetails,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct SnippetKey {
    def_id: LocalDefId,
    kind: SnippetKind,
}

fn preferred_snippet_order(tcx: TyCtxt) -> impl Fn(&SnippetKey, &SnippetKey) -> Ordering + '_ {
    move |lhs: &SnippetKey, rhs: &SnippetKey| {
        let to_ordering_key = |x: &SnippetKey| (x.kind.clone(), tcx.def_span(x.def_id));
        let lhs = to_ordering_key(lhs);
        let rhs = to_ordering_key(rhs);
        lhs.cmp(&rhs)
    }
}

/// A C++ snippet (e.g. function declaration for `..._cc_api.h`) and a Rust
/// snippet (e.g. a thunk definition for `..._cc_api_impl.rs`).
#[derive(Debug, Default)]
struct MixedSnippet {
    cc: CcSnippet,
    rs: TokenStream,
}

impl From<CcSnippet> for MixedSnippet {
    fn from(cc: CcSnippet) -> Self {
        Self { cc, rs: quote! {} }
    }
}

/// Formats a function with the given `local_def_id`.
///
/// Returns multiple snippets, so that a function declaration can be emitted
/// separately from a function definition (and thunk declaration).  This is
/// mostly needed to handle method declarations (which need to be emitted
/// separately from method definitions;  they also need to be reordered
/// separately - see the `non_contiguous_method_decls_and_defs` module in
/// `cc_bindings_from_rs/test/impls/impls.rs`).  Secondary motivation is to keep
/// implementation details out of the way (to improve readability of the main
/// apis).
///
/// Multiple snippets are returned as a `Vec` for consistency with
/// `format_item`.  This is a somewhat arbitrary choice - in theory the return
/// value could be represented as a pair/tuple or a struct that explicitly only
/// holds two snippets: a declaration and an (optional) definition.
///
/// Will panic if `local_def_id`
/// - is invalid
/// - doesn't identify a function,
fn format_fn(input: &Input, local_def_id: LocalDefId) -> Result<Vec<(SnippetKey, MixedSnippet)>> {
    let tcx = input.tcx;
    let def_id: DefId = local_def_id.to_def_id(); // Convert LocalDefId to DefId.

    ensure!(
        tcx.generics_of(def_id).count() == 0,
        "Generic functions are not supported yet (b/259749023)"
    );
    let sig = match tcx.fn_sig(def_id).subst_identity().no_bound_vars() {
        None => bail!("Generic functions are not supported yet (b/259749023)"),
        Some(sig) => sig,
    };

    let mut symbol_name = {
        // Call to `mono` is ok - `generics_of` have been checked above.
        let instance = ty::Instance::mono(tcx, def_id);
        tcx.symbol_name(instance)
    };

    if sig.c_variadic {
        // TODO(b/254097223): Add support for variadic functions.
        bail!("C variadic functions are not supported (b/254097223)");
    }

    match sig.unsafety {
        Unsafety::Normal => (),
        Unsafety::Unsafe => {
            // TODO(b/254095482): Figure out how to handle `unsafe` functions.
            bail!("Bindings for `unsafe` functions are not fully designed yet (b/254095482)");
        }
    }

    let needs_thunk: bool;
    match sig.abi {
        // "C" ABI is okay: Before https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html a Rust
        // panic that "escapes" a "C" ABI function leads to Undefined Behavior.  This is
        // unfortunate, but Crubit's `panics_and_exceptions.md` documents that `-Cpanic=abort` is
        // the only supported configuration.
        //
        // After https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html a Rust panic that
        // tries to "escape" a "C" ABI function will terminate the program.  This is okay.
        Abi::C { unwind: false } => {
            needs_thunk = false;
        },

        // "C-unwind" ABI is okay: After https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html a
        // new "C-unwind" ABI may be used by Rust functions that want to safely propagate Rust
        // panics through frames that may belong to another language.
        Abi::C { unwind: true } => {
            needs_thunk = false;
        },

        // All other ABIs trigger thunk generation.  This covers Rust ABI functions, but
        // also ABIs that theoretically are understood both by C++ and Rust (e.g. see
        // `format_cc_call_conv_as_clang_attribute` in `rs_bindings_from_cc/src_code_gen.rs`).
        _ => {
            let thunk_name = format!("__crubit_thunk_{}", symbol_name.name);
            symbol_name = ty::SymbolName::new(tcx, &thunk_name);
            needs_thunk = true;
        }
    };

    let FullyQualifiedName { krate, mod_path, name, .. } = FullyQualifiedName::new(tcx, def_id);

    let mut prereqs = CcPrerequisites::default();
    let cc_ret_type = format_ret_ty_for_cc(input, sig.output())
        .context("Error formatting function return type")?
        .into_tokens(&mut prereqs);
    let cc_fn_name = format_cc_ident(name.as_str()).context("Error formatting function name")?;
    let cc_arg_names = tcx
        .fn_arg_names(def_id)
        .iter()
        .enumerate()
        .map(|(index, ident)| {
            format_cc_ident(ident.as_str())
                .unwrap_or_else(|_err| format_cc_ident(&format!("__param_{index}")).unwrap())
        })
        .collect_vec();
    let cc_arg_types = sig
        .inputs()
        .iter()
        .enumerate()
        .map(|(index, ty)| {
            Ok(format_ty_for_cc(input, *ty)
                .with_context(|| format!("Error formatting the type of parameter #{index}"))?
                .into_tokens(&mut prereqs))
        })
        .collect::<Result<Vec<_>>>()?;

    let struct_name = match tcx.impl_of_method(def_id) {
        Some(impl_id) => match tcx.impl_subject(impl_id) {
            ty::ImplSubject::Inherent(ty) => match ty.kind() {
                ty::TyKind::Adt(adt, substs) => {
                    assert_eq!(0, substs.len(), "Callers should filter out generics");
                    Some(tcx.item_name(adt.did()))
                }
                _ => panic!("Non-ADT `impl`s should be filtered by caller"),
            },
            ty::ImplSubject::Trait(_) => panic!("Trait methods should be filtered by caller"),
        },
        None => None,
    };
    let needs_definition = name.as_str() != symbol_name.name;
    let main_api = {
        let doc_comment = {
            let doc_comment = format_doc_comment(tcx, local_def_id);
            quote! { __NEWLINE__ #doc_comment }
        };

        let static_ = match tcx.hir().get_by_def_id(local_def_id) {
            Node::ImplItem(impl_item) => match &impl_item.kind {
                ImplItemKind::Fn(fn_sig, _) => match fn_sig.decl.implicit_self {
                    ImplicitSelfKind::None => quote! { static },
                    _ => bail!("`self` parameter is not supported yet"),
                },
                _ => panic!("`format_fn` can only work with functions"),
            },
            Node::Item(_) => quote! {}, // Free function ==> no `static` qualifier.
            other => panic!("Unexpected HIR node kind: {other:?}"),
        };

        let mut prereqs = prereqs.clone();
        prereqs.move_defs_to_fwd_decls();
        let extern_c_or_inline = if !needs_definition {
            quote! { extern "C" }
        } else {
            quote! { inline }
        };
        CcSnippet {
            prereqs,
            tokens: quote! {
                __NEWLINE__
                #doc_comment
                #static_ #extern_c_or_inline #cc_ret_type #cc_fn_name (
                        #( #cc_arg_types #cc_arg_names ),*
                );
                __NEWLINE__
            },
        }
    };
    let impl_details = if !needs_definition {
        None
    } else {
        let cc_exported_name =
            format_cc_ident(symbol_name.name).context("Error formatting exported name")?;
        let cc_struct_name = match struct_name.as_ref() {
            None => quote! {},
            Some(symbol) => {
                let name = format_cc_ident(symbol.as_str())
                    .expect("Caller of format_fn should verify struct name via format_adt_core");
                quote! { #name :: }
            }
        };
        let thunk_args = cc_arg_names
            .clone()
            .into_iter()
            .zip(sig.inputs().iter())
            .map(|(arg, &ty)| format_cc_thunk_arg(tcx, ty, arg).into_tokens(&mut prereqs))
            .collect_vec();
        let cc = CcSnippet {
            prereqs,
            tokens: quote! {
                __NEWLINE__
                namespace __crubit_internal {
                    extern "C" #cc_ret_type #cc_exported_name (
                            #( #cc_arg_types #cc_arg_names ),*
                    );
                }
                inline #cc_ret_type #cc_struct_name #cc_fn_name (
                        #( #cc_arg_types #cc_arg_names ),* ) {
                    return __crubit_internal :: #cc_exported_name( #( #thunk_args ),* );
                }
                __NEWLINE__
            },
        };

        let rs = if !needs_thunk {
            quote! {}
        } else {
            let crate_name = make_rs_ident(krate.as_str());
            let mod_path = mod_path.format_for_rs();
            let rs_fn_name = make_rs_ident(name.as_str());
            let rs_exported_name = make_rs_ident(symbol_name.name);
            let rs_struct_name = match struct_name.as_ref() {
                None => quote! {},
                Some(symbol) => {
                    let name = make_rs_ident(symbol.as_str());
                    quote! { #name :: }
                }
            };
            let rs_ret_type = format_ty_for_rs(tcx, sig.output())?;
            let rs_arg_names = tcx
                .fn_arg_names(def_id)
                .iter()
                .enumerate()
                .map(|(index, ident)| {
                    if ident.as_str().is_empty() {
                        format_ident!("__param_{index}")
                    } else {
                        make_rs_ident(ident.as_str())
                    }
                })
                .collect_vec();
            let rs_arg_types = sig
                .inputs()
                .iter()
                .copied()
                .map(|ty| format_ty_for_rs(tcx, ty))
                .collect::<Result<Vec<_>>>()?;
            quote! {
                #[no_mangle]
                extern "C" fn #rs_exported_name( #( #rs_arg_names: #rs_arg_types ),* )
                        -> #rs_ret_type {
                    :: #crate_name :: #mod_path #rs_struct_name #rs_fn_name(
                        #( #rs_arg_names ),*
                    )
                }
            }
        };

        Some(MixedSnippet { cc, rs })
    };

    let mut result =
        vec![(SnippetKey { def_id: local_def_id, kind: SnippetKind::MainApi }, main_api.into())];
    if let Some(impl_details) = impl_details {
        result.push((
            SnippetKey { def_id: local_def_id, kind: SnippetKind::ImplDetails },
            impl_details,
        ));
    }
    Ok(result)
}

/// Represents bindings for the "core" part of an algebraic data type (an ADT -
/// a struct, an enum, or a union) in a way that supports later injecting the
/// other parts like so:
///
/// ```
/// quote! {
///     #keyword #alignment #name final {
///         #core
///         #decls_of_other_parts  // (e.g. struct fields, methods, etc.)
///     }
/// }
/// ```
///
/// `keyword`, `name` are stored separately, to support formatting them as a
/// forward declaration - e.g. `struct SomeStruct`.
struct AdtCoreBindings {
    /// DefId of the ADT.
    def_id: DefId,

    /// C++ tag - e.g. `struct`, `class`, `enum`, or `union`.  This isn't always
    /// a direct mapping from Rust (e.g. a Rust `enum` might end up being
    /// represented as an opaque C++ `struct`).
    keyword: TokenStream,

    /// C++ translation of the ADT identifier - e.g. `SomeStruct`.
    cc_name: TokenStream,

    /// Rust spelling of the ADT type - e.g.
    /// `some_crate::some_module::SomeStruct`.
    rs_name: TokenStream,

    /// `core` contains declarations of
    /// - the default constructor
    /// - the copy constructor
    /// - the move constructor
    /// - the copy assignment operator
    /// - the move assignment operator
    /// - the destructor
    core: TokenStream,

    alignment_in_bytes: u64,
    size_in_bytes: u64,
}

/// Formats the core of an algebraic data type (an ADT - a struct, an enum, or a
/// union) represented by `def_id`.
///
/// The "core" means things that are necessary for a succesful binding (e.g.
/// inability to generate a correct C++ destructor means that the ADT cannot
/// have any bindings).  "core" excludes things that are A) infallible (e.g.
/// struct or union fields which can always be translated into private, opaque
/// blobs of bytes) or B) optional (e.g. a problematic instance method
/// can just be ignored, unlike a problematic destructor).  The split between
/// fallible "core" and non-fallible "rest" is motivated by the need to avoid
/// cycles / infinite recursion (e.g. when processing fields that refer back to
/// the struct type, possible with an indirection of a pointer).
///
/// `format_adt_core` is used both to 1) format bindings for the core of an ADT,
/// and 2) check if formatting would have succeeded (e.g. when called from
/// `format_ty`).  The 2nd case is needed for ADTs defined in any crate - this
/// is why the `def_id` parameter is a DefId rather than LocalDefId.
//
// TODO(b/259724276): This function's results should be memoized.
fn format_adt_core(tcx: TyCtxt, def_id: DefId) -> Result<AdtCoreBindings> {
    // TODO(b/259749095): Support non-empty set of generic parameters.
    let param_env = ty::ParamEnv::empty();

    let ty = tcx.type_of(def_id);
    if ty.needs_drop(tcx, param_env) {
        // TODO(b/258251148): Support custom `Drop` impls.
        bail!("`Drop` trait and \"drop glue\" are not supported yet (b/258251148)");
    }

    let rs_name = format_ty_for_rs(tcx, ty)?;
    let cc_name = {
        let item_name = tcx.item_name(def_id);
        format_cc_ident(item_name.as_str()).context("Error formatting item name")?
    };

    let layout = tcx
        .layout_of(param_env.and(ty))
        // Have to use `.map_err` instead of `.with_context`, because `LayoutError` doesn't
        // satisfy the `anyhow::context::ext::StdError` trait bound.
        .map_err(|layout_err| {
            let item_name = tcx.item_name(def_id);
            anyhow!("Error computing the layout of #{item_name}: {layout_err}")
        })?
        .layout;
    let alignment_in_bytes = {
        // Only the ABI-mandated alignment is considered (i.e. `AbiAndPrefAlign::pref`
        // is ignored), because 1) Rust's `std::mem::align_of` returns the
        // ABI-mandated alignment and 2) the generated C++'s `alignas(...)`
        // should specify the minimal/mandatory alignment.
        layout.align().abi.bytes()
    };
    let size_in_bytes = layout.size().bytes();
    ensure!(size_in_bytes != 0, "Zero-sized types (ZSTs) are not supported (b/258259459)");

    let core = quote! {
        public:
            // TODO(b/258249980): If the wrapped type implements the `Default` trait, then we
            // should call its `impl` from the default C++ constructor (instead of `delete`ing
            // the default C++ constructor).
            #cc_name() = delete;

            // TODO(b/258249993): Provide `default` copy constructor and assignment operator if
            // the wrapped type is `Copy` on Rust side.
            // TODO(b/259741191): If the wrapped type implements the `Clone` trait, then we should
            // *consider* calling `clone` from the copy constructor and `clone_from` from the copy
            // assignment operator.
            #cc_name(const #cc_name&) = delete;

            // The generated bindings have to follow Rust move semantics:
            // * All Rust types are memcpy-movable (e.g. <internal link>/constructors.html says
            //   that "Every type must be ready for it to be blindly memcopied to somewhere else
            //   in memory")
            // * The only valid operation on a moved-from non-`Copy` Rust struct is to assign to
            //   it.
            //
            // The generated C++ bindings match the required semantics because they:
            // * Generate trivial` C++ move constructor and move assignment operator. Per
            //   <internal link>/cpp/language/move_constructor#Trivial_move_constructor: "A trivial move
            //   constructor is a constructor that performs the same action as the trivial copy
            //   constructor, that is, makes a copy of the object representation as if by
            //   std::memmove."
            // * Generate trivial C++ destructor. (Types that implement `Drop` trait or require
            //   "drop glue" are not *yet* supported - this might eventually change as part of the
            //   work tracked under b/258251148). Per
            //   <internal link>/cpp/language/destructor#Trivial_destructor: "A trivial destructor is a
            //   destructor that performs no action."
            //
            // In particular, note that the following C++ code and Rust code are exactly equivalent
            // (except that in Rust, reuse of `y` is forbidden at compile time, whereas in C++,
            // it's only prohibited by convention):
            // * C++, assumming trivial move constructor and trivial destructor:
            //   `auto x = std::move(y);`
            // * Rust, assumming non-`Copy`, no custom `Drop` or drop glue:
            //   `let x = y;`
            //
            // TODO(b/258251148): If the ADT provides a custom `Drop` impls or requires drop glue,
            // then extra care should be taken to ensure the C++ destructor can handle the
            // moved-from object in a way that meets Rust move semantics.  For example, the
            // generated C++ move constructor might need to assign `Default::default()` to the
            // moved-from object.
            #cc_name(#cc_name&&) = default;

            // TODO(b/258235219): Providing assignment operators enables mutation which
            // may negatively interact with support for references.  Therefore until we
            // have more confidence in our reference-handling-plans, we are deleting the
            // assignment operators.
            //
            // (Move assignment operator has another set of concerns and constraints - see the
            // comment for the move constructor above).
            #cc_name& operator=(const #cc_name&) = delete;
            #cc_name& operator=(#cc_name&&) = delete;

            // TODO(b/258251148): Support custom `Drop` impls and drop glue.
            ~#cc_name() = default;
    };
    Ok(AdtCoreBindings {
        def_id,
        keyword: quote! { struct },
        cc_name,
        rs_name,
        core,
        alignment_in_bytes,
        size_in_bytes,
    })
}

/// Formats an algebraic data type (an ADT - a struct, an enum, or a union)
/// represented by `core`.  This function is infallible - after
/// `format_adt_core` returns success we have committed to emitting C++ bindings
/// for the ADT.
///
/// Returns multiple snippets to support independent reordering of 1) the struct
/// definition (including declarations of methods - aka member functions), and
/// 2) an arbitrary number of method definitions.  For motivation see the
/// `non_contiguous_method_decls_and_defs` module in `cc_bindings_from_rs/test/
/// impls/impls.rs`).  Secondary motivation is to keep implementation
/// details out of the way (to improve readability of the main apis).
fn format_adt(input: &Input, core: &AdtCoreBindings) -> Vec<(SnippetKey, MixedSnippet)> {
    let tcx = input.tcx;

    // `format_adt` should only be called for local ADTs.
    let local_def_id = core.def_id.expect_local();

    let (impl_item_main_apis, impl_item_other_snippets) = tcx
        .inherent_impls(core.def_id)
        .iter()
        .map(|impl_id| tcx.hir().expect_item(impl_id.expect_local()))
        .flat_map(|item| match &item.kind {
            ItemKind::Impl(impl_) => impl_.items,
            other => panic!("Unexpected `ItemKind` from `inherent_impls`: {other:?}"),
        })
        .flat_map(|impl_item_ref| {
            let def_id = impl_item_ref.id.owner_id.def_id;
            if !tcx.effective_visibilities(()).is_directly_public(def_id) {
                return vec![];
            }
            let result = match impl_item_ref.kind {
                AssocItemKind::Fn { .. } => format_fn(input, def_id),
                other => Err(anyhow!("Unsupported `impl` item kind: {other:?}")),
            };
            result.unwrap_or_else(|err| vec![format_unsupported_def(tcx, def_id, err)])
        })
        .partition::<Vec<_>, _>(|(SnippetKey { kind, .. }, _)| *kind == SnippetKind::MainApi);

    let alignment = Literal::u64_unsuffixed(core.alignment_in_bytes);
    let size = Literal::u64_unsuffixed(core.size_in_bytes);
    let cc_name = &core.cc_name;
    let main_api = {
        let doc_comment = format_doc_comment(tcx, core.def_id.expect_local());
        let keyword = &core.keyword;
        let core = &core.core;

        let mut prereqs = CcPrerequisites::default();
        let impl_item_decls = if impl_item_main_apis.is_empty() {
            quote! {}
        } else {
            let cmp = preferred_snippet_order(tcx);
            let tokens = impl_item_main_apis
                .into_iter()
                .sorted_by(|(key1, _), (key2, _)| cmp(key1, key2))
                .map(|(_key, snippet)| snippet.cc.into_tokens(&mut prereqs));
            quote! {
                public:
                    #( #tokens )*
            }
        };
        prereqs.fwd_decls.remove(&local_def_id);

        CcSnippet {
            prereqs,
            tokens: quote! {
                __NEWLINE__ #doc_comment
                #keyword alignas(#alignment) #cc_name final {
                    #core
                    #impl_item_decls
                    private:
                        // TODO(b/258233850): Emit individual fields.
                        unsigned char opaque_blob_of_bytes[#size];
                };
                __NEWLINE__
            },
        }
    };
    let impl_details = {
        let mut cc = CcSnippet::new(quote! {
            __NEWLINE__
            static_assert(
                sizeof(#cc_name) == #size,
                "Verify that struct layout didn't change since this header got generated");
            static_assert(
                alignof(#cc_name) == #alignment,
                "Verify that struct layout didn't change since this header got generated");
            __NEWLINE__
        });
        cc.prereqs.defs.insert(local_def_id);
        let rs = {
            let rs_name = &core.rs_name;
            quote! {
                const _: () = assert!(::std::mem::size_of::<#rs_name>() == #size);
                const _: () = assert!(::std::mem::align_of::<#rs_name>() == #alignment);
            }
        };
        MixedSnippet { cc, rs }
    };

    let mut result = vec![
        (SnippetKey { def_id: local_def_id, kind: SnippetKind::MainApi }, main_api.into()),
        (SnippetKey { def_id: local_def_id, kind: SnippetKind::ImplDetails }, impl_details),
    ];
    result.extend(impl_item_other_snippets);
    result
}

/// Formats the forward declaration of an algebraic data type (an ADT - a
/// struct, an enum, or a union), returning something like
/// `quote!{ struct SomeStruct; }`.
///
/// Will panic if `def_id` doesn't identify an ADT that can be successfully
/// handled by `format_adt_core`.
fn format_fwd_decl(tcx: TyCtxt, def_id: LocalDefId) -> TokenStream {
    let def_id = def_id.to_def_id(); // LocalDefId -> DefId conversion.

    // `format_fwd_decl` should only be called for items from
    // `CcPrerequisites::fwd_decls` and `fwd_decls` should only contain ADTs
    // that `format_adt_core` succeeds for.
    let AdtCoreBindings { keyword, cc_name, .. } = format_adt_core(tcx, def_id)
        .expect("`format_fwd_decl` should only be called if `format_adt_core` succeeded");

    quote! { #keyword #cc_name; }
}

fn format_source_location(tcx: TyCtxt, local_def_id: LocalDefId) -> String {
    let def_span = tcx.def_span(local_def_id);
    let rustc_span::FileLines { file, lines } =
        match tcx.sess().source_map().span_to_lines(def_span) {
            Ok(filelines) => filelines,
            Err(_) => return "unknown location".to_string(),
        };
    let file_name = file.name.prefer_local().to_string();
    // Note: line_index starts at 0, while CodeSearch starts indexing at 1.
    let line_number = lines[0].line_index + 1;
    let google3_prefix = {
        // If rustc_span::FileName isn't a 'real' file, then it's surrounded by by angle
        // brackets, thus don't prepend "google3/" prefix.
        if file.name.is_real() { "google3/" } else { "" }
    };
    format!("{google3_prefix}{file_name};l={line_number}")
}

/// Formats the doc comment (if any) associated with the item identified by
/// `local_def_id`, and appends the source location at which the item is
/// defined.
fn format_doc_comment(tcx: TyCtxt, local_def_id: LocalDefId) -> TokenStream {
    let hir_id = tcx.local_def_id_to_hir_id(local_def_id);
    let doc_comment = tcx
        .hir()
        .attrs(hir_id)
        .iter()
        .filter_map(|attr| attr.doc_str())
        .map(|symbol| symbol.to_string())
        .chain(once(format!("Generated from: {}", format_source_location(tcx, local_def_id))))
        .join("\n\n");
    quote! { __COMMENT__ #doc_comment}
}

/// Formats a HIR item idenfied by `def_id`.  Returns `None` if the item
/// can be ignored. Returns an `Err` if the definition couldn't be formatted.
///
/// Will panic if `def_id` is invalid (i.e. doesn't identify a HIR item).
fn format_item(input: &Input, def_id: LocalDefId) -> Result<Vec<(SnippetKey, MixedSnippet)>> {
    let tcx = input.tcx;

    // TODO(b/262052635): When adding support for re-exports we may need to change
    // `is_directly_public` below into `is_exported`.  (OTOH such change *alone* is
    // undesirable, because it would mean exposing items from a private module.)
    if !tcx.effective_visibilities(()).is_directly_public(def_id) {
        return Ok(vec![]);
    }

    match input.tcx.hir().expect_item(def_id) {
        Item { kind: ItemKind::Struct(_, generics) |
                     ItemKind::Enum(_, generics) |
                     ItemKind::Union(_, generics),
               .. } if !generics.params.is_empty() => {
            bail!("Generic types are not supported yet (b/259749095)");
        },
        Item { kind: ItemKind::Fn(..), .. } => format_fn(input, def_id),
        Item { kind: ItemKind::Struct(..) | ItemKind::Enum(..) | ItemKind::Union(..), .. } =>
            format_adt_core(tcx, def_id.to_def_id())
                .map(|core| format_adt(input, &core)),
        Item { kind: ItemKind::Impl(_), .. } |  // Handled by `format_adt`
        Item { kind: ItemKind::Mod(_), .. } =>  // Handled by `format_crate`
            Ok(vec![]),
        Item { kind, .. } => bail!("Unsupported rustc_hir::hir::ItemKind: {}", kind.descr()),
    }
}

/// Formats a C++ comment explaining why no bindings have been generated for
/// `local_def_id`.
fn format_unsupported_def(
    tcx: TyCtxt,
    local_def_id: LocalDefId,
    err: anyhow::Error,
) -> (SnippetKey, MixedSnippet) {
    let source_loc = format_source_location(tcx, local_def_id);
    let name = tcx.def_path_str(local_def_id.to_def_id());

    // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
    // says: To print causes as well [...], use the alternate selector “{:#}”.
    let msg = format!("Error generating bindings for `{name}` defined at {source_loc}: {err:#}");
    let cc = CcSnippet::new(quote! { __NEWLINE__ __NEWLINE__ __COMMENT__ #msg __NEWLINE__ });

    (SnippetKey { def_id: local_def_id, kind: SnippetKind::MainApi }, cc.into())
}

/// Formats all public items from the Rust crate being compiled.
fn format_crate(input: &Input) -> Result<Output> {
    let tcx = input.tcx;
    let mut bindings: HashMap<SnippetKey, MixedSnippet> = tcx
        .hir()
        .items()
        .flat_map(|item_id| {
            let def_id: LocalDefId = item_id.owner_id.def_id;
            format_item(input, def_id)
                .unwrap_or_else(|err| vec![format_unsupported_def(tcx, def_id, err)])
                .into_iter()
        })
        .fold(HashMap::new(), |mut map, (key, value)| {
            let old_item = map.insert(key, value);
            assert!(old_item.is_none(), "Duplicated key: {key:?}");
            map
        });

    // Find the order of `bindings` that 1) meets the requirements of
    // `CcPrerequisites::defs` and 2) makes a best effort attempt to keep the
    // `bindings` in the same order as the source order of the Rust APIs.
    let ordered_ids = {
        let toposort::TopoSortResult { ordered: ordered_ids, failed: failed_ids } = {
            let nodes = bindings.keys().copied();
            let deps = bindings.iter().flat_map(|(&successor, snippet)| {
                let predecessors = snippet.cc.prereqs.defs.iter().map(|&def_id|
                    SnippetKey { def_id, kind: SnippetKind::MainApi }
                );
                predecessors.map(move |predecessor| toposort::Dependency { predecessor, successor })
            });
            toposort::toposort(nodes, deps, preferred_snippet_order(tcx))
        };
        assert_eq!(
            0,
            failed_ids.len(),
            "There are no known scenarios where CcPrerequisites::defs can form \
                    a dependency cycle. These `LocalDefId`s form an unexpected cycle: {}",
            failed_ids.into_iter().map(|id| format!("{:?}", id)).join(",")
        );
        ordered_ids
    };

    // Destructure/rebuild `bindings` (in the same order as `ordered_ids`) into
    // `includes`, and into separate C++ snippets and Rust snippets.
    let (includes, ordered_cc, rs_body) = {
        let mut already_declared = HashSet::new();
        let mut fwd_decls = HashSet::new();
        let mut includes = BTreeSet::new();
        let mut ordered_cc = Vec::new();
        let mut rs_body = quote! {};
        for key in ordered_ids.into_iter() {
            let mod_path = FullyQualifiedName::new(tcx, key.def_id.to_def_id()).mod_path;
            let MixedSnippet {
                rs: inner_rs,
                cc: CcSnippet {
                    tokens: cc_tokens,
                    prereqs: CcPrerequisites {
                        includes: mut inner_includes,
                        fwd_decls: inner_fwd_decls,
                        .. // `defs` have already been utilized by `toposort` above
                    }
                }
            } = bindings.remove(&key).unwrap();

            fwd_decls.extend(inner_fwd_decls.difference(&already_declared).copied());
            already_declared.insert(key.def_id);
            already_declared.extend(inner_fwd_decls.into_iter());

            includes.append(&mut inner_includes);
            ordered_cc.push((mod_path, cc_tokens));
            rs_body.extend(inner_rs);
        }

        // Prepend `fwd_decls` (in the original source order) to `ordered_cc`.
        let fwd_decls = fwd_decls
            .into_iter()
            .sorted_by_key(|def_id| tcx.def_span(*def_id))
            .map(|local_def_id| {
                let mod_path = FullyQualifiedName::new(tcx, local_def_id.to_def_id()).mod_path;
                (mod_path, format_fwd_decl(tcx, local_def_id))
            })
            .collect_vec();
        let ordered_cc = fwd_decls.into_iter().chain(ordered_cc.into_iter()).collect_vec();

        (includes, ordered_cc, rs_body)
    };

    // Generate top-level elements of the C++ header file.
    let h_body = {
        // TODO(b/254690602): Decide whether using `#crate_name` as the name of the
        // top-level namespace is okay (e.g. investigate if this name is globally
        // unique + ergonomic).
        let crate_name = format_cc_ident(tcx.crate_name(LOCAL_CRATE).as_str())?;

        let includes = format_cc_includes(&includes);
        let ordered_cc = format_namespace_bound_cc_tokens(ordered_cc);
        quote! {
            #includes
            __NEWLINE__ __NEWLINE__
            namespace #crate_name {
                __NEWLINE__
                #ordered_cc
                __NEWLINE__
            }
            __NEWLINE__
        }
    };

    Ok(Output { h_body, rs_body })
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use anyhow::Result;
    use itertools::Itertools;
    use proc_macro2::TokenStream;
    use quote::quote;
    use rustc_middle::ty::{Ty, TyCtxt};
    use rustc_span::def_id::LocalDefId;

    use crate::run_compiler::tests::run_compiler_for_testing;
    use code_gen_utils::{format_cc_ident, format_cc_includes};
    use token_stream_matchers::{
        assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
    };

    #[test]
    #[should_panic(expected = "No items named `missing_name`.\n\
                               Instead found:\n`bar`,\n`foo`,\n`m1`,\n`m2`,\n`std`")]
    fn test_find_def_id_by_name_panic_when_no_item_with_matching_name() {
        let test_src = r#"
                pub extern "C" fn foo() {}

                pub mod m1 {
                    pub fn bar() {}
                }
                pub mod m2 {
                    pub fn bar() {}
                }
            "#;
        run_compiler_for_testing(test_src, |tcx| find_def_id_by_name(tcx, "missing_name"));
    }

    #[test]
    #[should_panic(expected = "More than one item named `some_name`")]
    fn test_find_def_id_by_name_panic_when_multiple_items_with_matching_name() {
        let test_src = r#"
                pub mod m1 {
                    pub fn some_name() {}
                }
                pub mod m2 {
                    pub fn some_name() {}
                }
            "#;
        run_compiler_for_testing(test_src, |tcx| find_def_id_by_name(tcx, "some_name"));
    }

    /// This test covers only a single example of a function that should get a
    /// C++ binding. The test focuses on verification that the output from
    /// `format_fn` gets propagated all the way to `GenerateBindings::new`.
    /// Additional coverage of how functions are formatted is provided
    /// by `test_format_item_..._fn_...` tests (which work at the `format_fn`
    /// level).
    #[test]
    fn test_generated_bindings_fn_no_mangle_extern_c() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn public_function() {
                    println!("foo");
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    extern "C" void public_function();
                }
            );

            // No Rust thunks should be generated in this test scenario.
            assert_rs_not_matches!(bindings.rs_body, quote!{ public_function });
        });
    }

    /// `test_generated_bindings_fn_export_name` covers a scenario where
    /// `MixedSnippet::cc` is present but `MixedSnippet::rs` is empty
    /// (because no Rust thunks are needed).
    #[test]
    fn test_generated_bindings_fn_export_name() {
        let test_src = r#"
                #[export_name = "export_name"]
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        ...
                        inline double public_function(double x, double y);
                        namespace __crubit_internal {
                            extern "C" double export_name(double x, double y);
                        }
                        inline double public_function(double x, double y) {
                            return __crubit_internal::export_name(x, y);
                        }
                    }
                }
            );
        });
    }

    /// The `test_generated_bindings_struct` test covers only a single example
    /// of an ADT (struct/enum/union) that should get a C++ binding.
    /// Additional coverage of how items are formatted is provided by
    /// `test_format_item_..._struct_...`, `test_format_item_..._enum_...`,
    /// and `test_format_item_..._union_...` tests.
    ///
    /// We don't want to duplicate coverage already provided by
    /// `test_format_item_struct_with_fields`, but we do want to verify that
    /// * `format_crate` will actually find and process the struct
    ///   (`test_format_item_...` doesn't cover this aspect - it uses a
    ///   test-only `find_def_id_by_name` instead)
    /// * The actual shape of the bindings still looks okay at this level.
    #[test]
    fn test_generated_bindings_struct() {
        let test_src = r#"
                pub struct Point {
                    pub x: i32,
                    pub y: i32,
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        ...
                        struct alignas(4) Point final {
                            // No point replicating test coverage of
                            // `test_format_item_struct_with_fields`.
                            ...
                        };
                        static_assert(sizeof(Point) == 8, ...);
                        static_assert(alignof(Point) == 4, ...);
                    }  // namespace rust_out
                }
            );
            assert_rs_matches!(
                bindings.rs_body,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::Point>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::Point>() == 4);
                }
            );
        });
    }

    /// The `test_generated_bindings_impl` test covers only a single example of
    /// a non-trait `impl`. Additional coverage of how items are formatted
    /// should be provided in the future by `test_format_item_...` tests.
    ///
    /// We don't want to duplicate coverage already provided by
    /// `test_format_item_static_method`, but we do want to verify that
    /// * `format_crate` won't process the `impl` as a standalone HIR item
    /// * The actual shape of the bindings still looks okay at this level.
    #[test]
    fn test_generated_bindings_impl() {
        let test_src = r#"
                pub struct SomeStruct(i32);

                impl SomeStruct {
                    pub fn public_static_method() -> i32 { 123 }

                    #[allow(dead_code)]
                    fn private_static_method() -> i32 { 123 }
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        ...
                        struct ... SomeStruct ... {
                            // No point replicating test coverage of
                            // `test_format_item_static_method`.
                            ...
                            std::int32_t public_static_method();
                            ...
                        };
                        ...
                        std::int32_t SomeStruct::public_static_method() {
                            ...
                        }
                    }  // namespace rust_out
                }
            );
            assert_rs_matches!(
                bindings.rs_body,
                quote! {
                    extern "C" fn ...() -> i32 {
                        ::rust_out::SomeStruct::public_static_method()
                    }
                }
            );
        });
    }

    #[test]
    fn test_generated_bindings_includes() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn public_function(i: i32, d: isize, u: u64) {
                    dbg!(i);
                    dbg!(d);
                    dbg!(u);
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    __HASH_TOKEN__ include <cstdint> ...
                    namespace ... {
                        ...
                        extern "C" void public_function(
                            std::int32_t i,
                            std::intptr_t d,
                            std::uint64_t u);
                    }
                }
            );
        });
    }

    #[test]
    fn test_generated_bindings_prereq_defs_require_different_order() {
        let test_src = r#"
                // In the generated bindings `f` needs to come *after* `S`.
                pub fn f(s: S) -> bool { s.0 }
                pub struct S(bool);
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        ...
                        struct ... S final {
                            // No point replicating test coverage of
                            // `test_format_item_struct_with_fields`.
                            ...
                        };
                        ...
                        namespace __crubit_internal {
                            extern "C" bool ...(::rust_out::S s);
                        }
                        ...
                        inline bool f(::rust_out::S s) { ... }
                        ...
                        static_assert(sizeof(S) == ..., ...);
                        static_assert(alignof(S) == ..., ...);
                        ...
                    }  // namespace rust_out
                }
            );
            assert_rs_matches!(
                bindings.rs_body,
                quote! {
                    #[no_mangle]
                    extern "C"
                    fn ...(s: ::rust_out::S) -> bool { ...  }
                    ...
                    const _: () = assert!(::std::mem::size_of::<::rust_out::S>() == ...);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::S>() == ...);
                }
            );
        });
    }

    /// Tests that a forward declaration is present when it is required to
    /// preserve the original source order.  In this test the
    /// `CcPrerequisites::fwd_decls` dependency comes from a pointer parameter.
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_for_ptr_param() {
        let test_src = r#"
                // To preserve original API order we need to forward declare S.
                pub fn f(_: *const S) {}
                pub struct S(bool);
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        ...
                        // Verifing the presence of this forward declaration
                        // it the essence of this test.  The order of the items
                        // below also matters.
                        struct S;
                        ...
                        inline void f(const ::rust_out::S* __param_0);
                        ...
                        struct alignas(...) S final { ... }
                        ...
                        inline void f(const ::rust_out::S* __param_0) { ... }
                        ...
                    }  // namespace rust_out
                }
            );
        });
    }

    /// Tests that a forward declaration is present when it is required to
    /// preserve the original source order.  In this test the
    /// `CcPrerequisites::fwd_decls` dependency comes from a parameter
    /// that takes a struct by value, but is only needed in a C++ function
    /// declaration (not in a C++ function definition).
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_for_cpp_fn_decl() {
        let test_src = r#"
                // `f` will only have a C++ declaration (and no C++ definition)
                // in the generated `..._cc_api.h` header.
                //
                // Therefore `f`'s `CcPrerequisites` should include `S`
                // as a `fwd_decls` edge, rather than as a `defs` edge.
                #[no_mangle]
                pub extern "C" fn f(s: S) -> bool { s.0 }

                #[repr(C)]
                pub struct S(bool);
            "#;

        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        ...
                        // Verifing the presence of this forward declaration
                        // it the essence of this test.  The order also matters:
                        // 1. The fwd decl of `S` should come first,
                        // 2. Declaration of `f` and definition of `S` should come next
                        //    (in their original order - `f` first and then `S`).
                        struct S;
                        ...
                        extern "C" bool f(::rust_out::S s);
                        ...
                        struct alignas(...) S final { ... }
                        ...
                    }  // namespace rust_out
                }
            );
        });
    }

    /// This test verifies that a forward declaration for a given ADT is only
    /// emitted once (and not once for every API item that requires the
    /// forward declaration as a prerequisite).
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_no_duplication() {
        let test_src = r#"
                // All three functions below require a forward declaration of S.
                pub fn f1(_: *const S) {}
                pub fn f2(_: *const S) {}
                pub fn f3(_: *const S) {}

                pub struct S(bool);

                // This function also includes S in its CcPrerequisites::fwd_decls
                // (although here it is not required, because the definition of S
                // is already available above).
                pub fn f4(_: *const S) {}
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap().h_body.to_string();

            // Only a single forward declaration is expected.
            assert_eq!(1, bindings.matches("struct S ;").count(), "bindings = {bindings}");
        });
    }

    /// This test verifies that forward declarations are emitted in a
    /// deterministic order. The particular order doesn't matter _that_
    /// much, but it definitely shouldn't change every time
    /// `cc_bindings_from_rs` is invoked again.  The current order preserves
    /// the original source order of the Rust API items.
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_deterministic_order() {
        let test_src = r#"
                // To try to mix things up, the bindings for the functions below
                // will *ask* for forward declarations in a different order:
                // * Different from the order in which the forward declarations
                //   are expected to be *emitted* (the original source order).
                // * Different from alphabetical order.
                pub fn f1(_: *const b::S3) {}
                pub fn f2(_: *const a::S2) {}
                pub fn f3(_: *const a::S1) {}

                pub mod a {
                    pub struct S1(bool);
                    pub struct S2(bool);
                }

                pub mod b {
                    pub struct S3(bool);
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        ...
                        // Verifying that we get the same order in each test
                        // run is the essence of this test.
                        namespace a {
                        struct S1;
                        struct S2;
                        }
                        namespace b {
                        struct S3;
                        }
                        ...
                        inline void f1 ...
                        inline void f2 ...
                        inline void f3 ...

                        namespace a { ...
                        struct alignas(...) S1 final { ... } ...
                        struct alignas(...) S2 final { ... } ...
                        } ...
                        namespace b { ...
                        struct alignas(...) S3 final { ... } ...
                        } ...
                    }  // namespace rust_out
                }
            );
        });
    }

    /// This test verifies that forward declarations are not emitted if they are
    /// not needed (e.g. if bindings the given `struct` or other ADT have
    /// already been defined earlier).  In particular, we don't want to emit
    /// forward declarations for *all* `structs` (regardless if they are
    /// needed or not).
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_not_needed_because_of_initial_order() {
        let test_src = r#"
                pub struct S(bool);

                // S is already defined above - no need for forward declaration in C++.
                pub fn f(_s: *const S) {}
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_not_matches!(bindings.h_body, quote! { struct S; });
            assert_cc_matches!(bindings.h_body, quote! { void f(const ::rust_out::S* _s); });
        });
    }

    /// This test verifies that a method declaration doesn't ask for a forward
    /// declaration to the struct.
    #[test]
    fn test_generated_bindings_prereq_fwd_decls_not_needed_inside_struct_definition() {
        let test_src = r#"
                pub struct S(bool);

                impl S {
                    // This shouldn't require a fwd decl of S.
                    pub fn create() -> S { Self(true) }
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_not_matches!(bindings.h_body, quote! { struct S; });
            assert_cc_matches!(bindings.h_body, quote! { S create(); });
        });
    }

    #[test]
    fn test_generated_bindings_module_basics() {
        let test_src = r#"
                pub mod some_module {
                    pub fn some_func() {}
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        namespace some_module {
                            ...
                            inline void some_func() { ... }
                            ...
                        }  // namespace some_module
                    }  // namespace rust_out
                }
            );
            assert_rs_matches!(
                bindings.rs_body,
                quote! {
                    #[no_mangle]
                    extern "C"
                    fn ...() -> () {
                        ::rust_out::some_module::some_func()
                    }
                }
            );
        });
    }

    #[test]
    fn test_generated_bindings_module_name_is_cpp_reserved_keyword() {
        let test_src = r#"
                pub mod working_module {
                    pub fn working_module_f1() {}
                    pub fn working_module_f2() {}
                }
                pub mod reinterpret_cast {
                    pub fn broken_module_f1() {}
                    pub fn broken_module_f2() {}
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();

            // Items in the broken module should be replaced with a comment explaining the
            // problem.
            let broken_module_msg = "Failed to format namespace name `reinterpret_cast`: \
                                     `reinterpret_cast` is a C++ reserved keyword \
                                     and can't be used as a C++ identifier";
            assert_cc_not_matches!(bindings.h_body, quote! { namespace reinterpret_cast });
            assert_cc_not_matches!(bindings.h_body, quote! { broken_module_f1 });
            assert_cc_not_matches!(bindings.h_body, quote! { broken_module_f2 });

            // Items in the other module should still go through.
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        namespace working_module {
                            ...
                            inline void working_module_f1();
                            ...
                            inline void working_module_f2();
                            ...
                        }  // namespace some_module

                        __COMMENT__ #broken_module_msg
                        ...
                    }  // namespace rust_out
                }
            );
        });
    }

    /// `test_generated_bindings_non_pub_items` verifies that non-public items
    /// are not present/propagated into the generated bindings.
    #[test]
    fn test_generated_bindings_non_pub_items() {
        let test_src = r#"
                #![allow(dead_code)]

                extern "C" fn private_function() {
                    println!("foo");
                }

                struct PrivateStruct {
                    x: i32,
                    y: i32,
                }

                pub struct PublicStruct(i32);

                impl PublicStruct {
                    fn private_method() {}
                }

                pub mod public_module {
                    fn priv_func_in_pub_module() {}
                }

                mod private_module {
                    pub fn pub_func_in_priv_module() { priv_func_in_priv_module() }
                    fn priv_func_in_priv_module() {}
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_not_matches!(bindings.h_body, quote! { private_function });
            assert_rs_not_matches!(bindings.rs_body, quote! { private_function });
            assert_cc_not_matches!(bindings.h_body, quote! { PrivateStruct });
            assert_rs_not_matches!(bindings.rs_body, quote! { PrivateStruct });
            assert_cc_not_matches!(bindings.h_body, quote! { private_method });
            assert_rs_not_matches!(bindings.rs_body, quote! { private_method });
            assert_cc_not_matches!(bindings.h_body, quote! { priv_func_in_priv_module });
            assert_rs_not_matches!(bindings.rs_body, quote! { priv_func_in_priv_module });
            assert_cc_not_matches!(bindings.h_body, quote! { priv_func_in_pub_module });
            assert_rs_not_matches!(bindings.rs_body, quote! { priv_func_in_pub_module });
            assert_cc_not_matches!(bindings.h_body, quote! { private_module });
            assert_rs_not_matches!(bindings.rs_body, quote! { private_module });
            assert_cc_not_matches!(bindings.h_body, quote! { pub_func_in_priv_module });
            assert_rs_not_matches!(bindings.rs_body, quote! { pub_func_in_priv_module });
        });
    }

    #[test]
    fn test_generated_bindings_top_level_items() {
        let test_src = "pub fn public_function() {}";
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            let expected_comment_txt =
                "Automatically @generated C++ bindings for the following Rust crate:\n\
                 rust_out";
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    __COMMENT__ #expected_comment_txt
                    ...
                    __HASH_TOKEN__ pragma once
                    ...
                    namespace rust_out {
                        ...
                    }
                }
            );
            assert_cc_matches!(
                bindings.rs_body,
                quote! {
                    __COMMENT__ #expected_comment_txt
                }
            );
        })
    }

    /// The `test_generated_bindings_unsupported_item` test verifies how `Err`
    /// from `format_item` is formatted as a C++ comment (in `format_crate`
    /// and `format_unsupported_def`):
    /// - This test covers only a single example of an unsupported item.
    ///   Additional coverage is provided by `test_format_item_unsupported_...`
    ///   tests.
    /// - This test somewhat arbitrarily chooses an example of an unsupported
    ///   item, trying to pick one that 1) will never be supported (b/254104998
    ///   has some extra notes about APIs named after reserved C++ keywords) and
    ///   2) tests that the full error chain is included in the message.
    #[test]
    fn test_generated_bindings_unsupported_item() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn reinterpret_cast() {}
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            let expected_comment_txt = "Error generating bindings for `reinterpret_cast` \
                 defined at <crubit_unittests.rs>;l=3: \
                 Error formatting function name: \
                 `reinterpret_cast` is a C++ reserved keyword \
                 and can't be used as a C++ identifier";
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    __COMMENT__ #expected_comment_txt
                }
            );
        })
    }

    #[test]
    fn test_format_item_fn_extern_c_no_mangle_no_params_no_return_type() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn public_function() {}
            "#;
        test_format_item(test_src, "public_function", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void public_function();
                }
            );
        });
    }

    /// The `test_format_item_fn_explicit_unit_return_type` test below is very
    /// similar to the
    /// `test_format_item_fn_extern_c_no_mangle_no_params_no_return_type` above,
    /// except that the return type is explicitly spelled out.  There is no
    /// difference in `ty::FnSig` so our code behaves exactly the same, but the
    /// test has been planned based on earlier, hir-focused approach and having
    /// this extra test coverage shouldn't hurt. (`hir::FnSig`
    /// and `hir::FnRetTy` _would_ see a difference between the two tests, even
    /// though there is no different in the current `bindings.rs` code).
    #[test]
    fn test_format_item_fn_explicit_unit_return_type() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn explicit_unit_return_type() -> () {}
            "#;
        test_format_item(test_src, "explicit_unit_return_type", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void explicit_unit_return_type();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_never_return_type() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn never_returning_function() -> ! {
                    panic!("This function panics and therefore never returns");
                }
            "#;
        test_format_item(test_src, "never_returning_function", |result| {
            // TODO(b/254507801): The function should be annotated with the `[[noreturn]]`
            // attribute.
            // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
            // details).
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void never_returning_function();
                }
            );
        })
    }

    /// `test_format_item_fn_mangling` checks that bindings can be generated for
    /// `extern "C"` functions that do *not* have `#[no_mangle]` attribute.  The
    /// test elides away the mangled name in the `assert_cc_matches` checks
    /// below, but end-to-end test coverage should eventually be provided by
    /// `test/functions` (see b/262904507).
    #[test]
    fn test_format_item_fn_mangling() {
        let test_src = r#"
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_format_item(test_src, "public_function", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    inline double public_function(double x, double y);
                }
            );
            assert!(impl_details.rs.is_empty());
            assert!(impl_details.cc.prereqs.is_empty());
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double x, double y);
                    }
                    ...
                    inline double public_function(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_export_name() {
        let test_src = r#"
                #[export_name = "export_name"]
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_format_item(test_src, "public_function", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    inline double public_function(double x, double y);
                }
            );
            assert!(impl_details.rs.is_empty());
            assert!(impl_details.cc.prereqs.is_empty());
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double export_name(double x, double y);
                    }
                    ...
                    inline double public_function(double x, double y) {
                        return __crubit_internal::export_name(x, y);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_unsafe() {
        let test_src = r#"
                #[no_mangle]
                pub unsafe extern "C" fn foo() {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Bindings for `unsafe` functions \
                             are not fully designed yet (b/254095482)"
            );
        });
    }

    /// `test_format_item_fn_const` tests how bindings for an `const fn` are
    /// generated.
    ///
    /// Right now the `const` qualifier is ignored, but one can imagine that in
    /// the (very) long-term future such functions (including their bodies)
    /// could be translated into C++ `consteval` functions.
    #[test]
    fn test_format_item_fn_const() {
        let test_src = r#"
                pub const fn foo(i: i32) -> i32 { i * 42 }
            "#;
        test_format_item(test_src, "foo", |result| {
            // TODO(b/254095787): Update test expectations below once `const fn` from Rust
            // is translated into a `consteval` C++ function.
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    inline std::int32_t foo(std::int32_t i);
                }
            );
            assert!(!impl_details.cc.prereqs.is_empty());
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...( std::int32_t i);
                    }
                    ...
                    inline std::int32_t foo(std::int32_t i) {
                        return __crubit_internal::...(i);
                    }
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    #[no_mangle]
                    extern "C"
                    fn ...(i: i32) -> i32 {
                        ::rust_out::foo(i)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_c_unwind_abi() {
        // See also https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
        let test_src = r#"
                #![feature(c_unwind)]

                #[no_mangle]
                pub extern "C-unwind" fn may_throw() {}
            "#;
        test_format_item(test_src, "may_throw", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void may_throw();
                }
            );
        });
    }

    /// This test mainly verifies that `format_item` correctly propagates
    /// `CcPrerequisites` of parameter types and return type.
    #[test]
    fn test_format_item_fn_cc_prerequisites_if_cpp_definition_needed() {
        let test_src = r#"
                pub fn foo(_i: i32) -> S { panic!("foo") }
                pub struct S(i32);
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);

            // Minimal coverage, just to double-check that the test setup works.
            //
            // Note that this is a definition, and therefore `S` should be defined
            // earlier (not just forward declared).
            assert_cc_matches!(main_api.tokens, quote! { S foo(std::int32_t _i);});
            assert_cc_matches!(impl_details.cc.tokens, quote! { S foo(std::int32_t _i) { ... }});

            // Main checks: `CcPrerequisites::includes`.
            assert_cc_matches!(
                format_cc_includes(&main_api.prereqs.includes),
                quote! { include <cstdint> }
            );
            assert_cc_matches!(
                format_cc_includes(&impl_details.cc.prereqs.includes),
                quote! { include <cstdint> }
            );

            // Main checks: `CcPrerequisites::defs` and `CcPrerequisites::fwd_decls`.
            //
            // Verifying the actual def_id is tricky, because `test_format_item` doesn't
            // expose `tcx` to the verification function (and therefore calling
            // `find_def_id_by_name` is not easily possible).
            //
            // Note that `main_api` and `impl_details` have different expectations.
            assert_eq!(0, main_api.prereqs.defs.len());
            assert_eq!(1, main_api.prereqs.fwd_decls.len());
            assert_eq!(1, impl_details.cc.prereqs.defs.len());
            assert_eq!(0, impl_details.cc.prereqs.fwd_decls.len());
        });
    }

    /// This test verifies that `format_item` uses `CcPrerequisites::fwd_decls`
    /// rather than `CcPrerequisites::defs` for functions that only need a
    /// C++ declaration (and don't need a C++ definition).
    #[test]
    fn test_format_item_fn_cc_prerequisites_if_only_cpp_declaration_needed() {
        let test_src = r#"
                // `foo` will only have a C++ declaration (and no C++ definition)
                // in the generated `..._cc_api.h` header.
                //
                // Therefore `foo`'s `CcPrerequisites` should include `S`
                // as a `fwd_decls` edge, rather than as a `defs` edge.
                #[no_mangle]
                pub extern "C" fn foo(s: S) -> bool { s.0 }

                #[repr(C)]
                pub struct S(bool);
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);

            // Minimal coverage, just to double-check that the test setup works.
            //
            // Note that this is only a function *declaration* (not a function definition -
            // there is no function body), and therefore `S` just needs to be
            // forward-declared earlier.
            assert_cc_matches!(main_api.tokens, quote! { extern "C" bool foo(::rust_out::S s); });

            // Main checks: `CcPrerequisites::defs` and `CcPrerequisites::fwd_decls`.
            //
            // Verifying the actual def_id is tricky, because `test_format_item` doesn't
            // expose `tcx` to the verification function (and therefore calling
            // `find_def_id_by_name` is not easily possible).
            assert_eq!(0, main_api.prereqs.defs.len());
            assert_eq!(1, main_api.prereqs.fwd_decls.len());
        });
    }

    #[test]
    fn test_format_item_fn_with_type_aliased_return_type() {
        // Type aliases disappear at the `rustc_middle::ty::Ty` level and therefore in
        // the short-term the generated bindings also ignore type aliases.
        //
        // TODO(b/254096006): Consider preserving `type` aliases when generating
        // bindings.
        let test_src = r#"
                type MyTypeAlias = f64;

                #[no_mangle]
                pub extern "C" fn type_aliased_return() -> MyTypeAlias { 42.0 }
            "#;
        test_format_item(test_src, "type_aliased_return", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" double type_aliased_return();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_doc_comment_with_unmangled_name() {
        let test_src = r#"
            /// Outer line doc.
            /** Outer block doc that spans lines.
             */
            #[doc = "Doc comment via doc attribute."]
            #[no_mangle]
            pub extern "C" fn fn_with_doc_comment_with_unmangled_name() {}
          "#;
        test_format_item(test_src, "fn_with_doc_comment_with_unmangled_name", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            let doc_comments = [
                " Outer line doc.",
                "",
                " Outer block doc that spans lines.",
                "             ",
                "",
                "Doc comment via doc attribute.",
                "",
                "Generated from: <crubit_unittests.rs>;l=7",
            ]
            .join("\n");
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #doc_comments
                    extern "C" void fn_with_doc_comment_with_unmangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_inner_doc_comment_with_unmangled_name() {
        let test_src = r#"
            /// Outer doc comment.
            #[no_mangle]
            pub extern "C" fn fn_with_inner_doc_comment_with_unmangled_name() {
                //! Inner doc comment.
            }
          "#;
        test_format_item(test_src, "fn_with_inner_doc_comment_with_unmangled_name", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            let doc_comments = [
                " Outer doc comment.",
                " Inner doc comment.",
                "Generated from: <crubit_unittests.rs>;l=4",
            ]
            .join("\n\n");
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #doc_comments
                    extern "C" void fn_with_inner_doc_comment_with_unmangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_doc_comment_with_mangled_name() {
        let test_src = r#"
                /// Doc comment of a function with mangled name.
                pub extern "C" fn fn_with_doc_comment_with_mangled_name() {}
            "#;
        test_format_item(test_src, "fn_with_doc_comment_with_mangled_name", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            let comment = " Doc comment of a function with mangled name.\n\n\
                           Generated from: <crubit_unittests.rs>;l=3";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    inline void fn_with_doc_comment_with_mangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_name_is_reserved_cpp_keyword() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn reinterpret_cast() -> () {}
            "#;
        test_format_item(test_src, "reinterpret_cast", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error formatting function name: \
                       `reinterpret_cast` is a C++ reserved keyword \
                       and can't be used as a C++ identifier"
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_ret_type() {
        let test_src = r#"
                pub fn foo() -> (i32, i32) { (123, 456) }
            "#;
        test_format_item(test_src, "foo", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error formatting function return type: \
                       Tuples are not supported yet: (i32, i32) (b/254099023)"
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_with_late_bound_lifetimes() {
        // TODO(b/258235219): Expect success after adding support for references.
        let test_src = r#"
                pub fn foo(arg: &i32) -> &i32 { arg }

                // Lifetime inference translates the above into:
                //     pub fn foo<'a>(arg: &'a i32) -> &'a i32 { ... }
                // leaving 'a lifetime late-bound (it is bound with a lifetime
                // taken from each of the callsites).  In other words, we can't
                // just call `no_bound_vars` on this `FnSig`'s `Binder`.
            "#;
        test_format_item(test_src, "foo", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic functions are not supported yet (b/259749023)");
        });
    }

    #[test]
    fn test_format_item_unsupported_generic_fn() {
        let test_src = r#"
                use std::default::Default;
                use std::fmt::Display;
                pub fn generic_function<T: Default + Display>() {
                    println!("{}", T::default());
                }
            "#;
        test_format_item(test_src, "generic_function", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic functions are not supported yet (b/259749023)");
        });
    }

    #[test]
    fn test_format_item_unsupported_generic_struct() {
        let test_src = r#"
                pub struct Point<T> {
                    pub x: T,
                    pub y: T,
                }
            "#;
        test_format_item(test_src, "Point", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic types are not supported yet (b/259749095)");
        });
    }

    #[test]
    fn test_format_item_unsupported_generic_enum() {
        let test_src = r#"
                pub enum Point<T> {
                    Cartesian{x: T, y: T},
                    Polar{angle: T, dist: T},
                }
            "#;
        test_format_item(test_src, "Point", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic types are not supported yet (b/259749095)");
        });
    }

    #[test]
    fn test_format_item_unsupported_generic_union() {
        let test_src = r#"
                pub union SomeUnion<T> {
                    pub x: std::mem::ManuallyDrop<T>,
                    pub y: i32,
                }
            "#;
        test_format_item(test_src, "SomeUnion", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic types are not supported yet (b/259749095)");
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_async() {
        let test_src = r#"
                pub async fn async_function() {}
            "#;
        test_format_item(test_src, "async_function", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Error formatting function return type: \
                             The following Rust type is not supported yet: \
                             impl std::future::Future<Output = ()>");
        });
    }

    #[test]
    fn test_format_item_fn_rust_abi() {
        let test_src = r#"
                pub fn add(x: f64, y: f64) -> f64 { x * y }
            "#;
        test_format_item(test_src, "add", |result| {
            // TODO(b/261074843): Re-add thunk name verification once we are using stable name
            // mangling (which may be coming in Q1 2023).  (This might mean reverting cl/492333432
            // + manual review and tweaks.)
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    inline double add(double x, double y);
                }
            );
            assert!(impl_details.cc.prereqs.is_empty());
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double x, double y);
                    }
                    ...
                    inline double add(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    #[no_mangle]
                    extern "C"
                    fn ...(x: f64, y: f64) -> f64 {
                        ::rust_out::add(x, y)
                    }
                }
            );
        });
    }

    /// `test_format_item_fn_rust_abi` tests a function call that is not a
    /// C-ABI, and is not the default Rust ABI.  It can't use `"stdcall"`,
    /// because it is not supported on the targets where Crubit's tests run.
    /// So, it ended up using `"vectorcall"`.
    ///
    /// This test almost entirely replicates `test_format_item_fn_rust_abi`,
    /// except for the `extern "vectorcall"` part in the `test_src` test
    /// input.
    ///
    /// This test verifies the current behavior that gives reasonable and
    /// functional FFI bindings.  OTOH, in the future we may decide to avoid
    /// having the extra thunk for cases where the given non-C-ABI function
    /// call convention is supported by both C++ and Rust
    /// (see also `format_cc_call_conv_as_clang_attribute` in
    /// `rs_bindings_from_cc/src_code_gen.rs`)
    #[test]
    fn test_format_item_fn_vectorcall_abi() {
        let test_src = r#"
                #![feature(abi_vectorcall)]
                pub extern "vectorcall" fn add(x: f64, y: f64) -> f64 { x * y }
            "#;
        test_format_item(test_src, "add", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    inline double add(double x, double y);
                }
            );
            assert!(impl_details.cc.prereqs.is_empty());
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double x, double y);
                    }
                    ...
                    inline double add(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    #[no_mangle]
                    extern "C"
                    fn ...(x: f64, y: f64) -> f64 {
                        ::rust_out::add(x, y)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_variadic() {
        let test_src = r#"
                #![feature(c_variadic)]

                #[no_mangle]
                pub unsafe extern "C" fn variadic_function(_fmt: *const u8, ...) {}
            "#;
        test_format_item(test_src, "variadic_function", |result| {
            // TODO(b/254097223): Add support for variadic functions.
            let err = result.unwrap_err();
            assert_eq!(err, "C variadic functions are not supported (b/254097223)");
        });
    }

    #[test]
    fn test_format_item_fn_params() {
        let test_src = r#"
                #[allow(unused_variables)]
                #[no_mangle]
                pub extern "C" fn foo(b: bool, f: f64) {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    extern "C" void foo(bool b, double f);
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_param_name_reserved_keyword() {
        let test_src = r#"
                #[allow(unused_variables)]
                #[no_mangle]
                pub extern "C" fn some_function(reinterpret_cast: f64) {}
            "#;
        test_format_item(test_src, "some_function", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    extern "C" void some_function(double __param_0);
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_multiple_anonymous_parameter_names() {
        let test_src = r#"
                pub fn foo(_: f64, _: f64) {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    inline void foo(double __param_0, double __param_1);
                }
            );
            assert!(impl_details.cc.prereqs.is_empty());
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void ...(
                            double __param_0, double __param_1);
                    }
                    ...
                    inline void foo(double __param_0, double __param_1) {
                        return __crubit_internal::...(__param_0, __param_1);
                    }
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    #[no_mangle]
                    extern "C" fn ...(__param_0: f64, __param_1: f64) -> () {
                        ::rust_out::foo(__param_0, __param_1)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_destructuring_parameter_name() {
        let test_src = r#"
                pub struct S {
                    pub f1: i32,
                    pub f2: i32,
                }

                // This test mostly focuses on the weird parameter "name" below.
                // See also
                // https://doc.rust-lang.org/reference/items/functions.html#function-parameters
                // which points out that function parameters are just irrefutable patterns.
                pub fn func(S{f1, f2}: S) -> i32 { f1 + f2 }
            "#;
        test_format_item(test_src, "func", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    inline std::int32_t func(::rust_out::S __param_0);
                }
            );
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...(::rust_out::S __param_0);
                    }
                    ...
                    inline std::int32_t func(::rust_out::S __param_0) {
                        return __crubit_internal::...(std::move(__param_0));
                    }
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    #[no_mangle]
                    extern "C" fn ...(__param_0: ::rust_out::S) -> i32 {
                        ::rust_out::func(__param_0)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_param_type() {
        let test_src = r#"
                pub fn foo(_param: (i32, i32)) {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Error formatting the type of parameter #0: \
                             Tuples are not supported yet: (i32, i32) (b/254099023)");
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_param_type_unit() {
        let test_src = r#"
                #[no_mangle]
                pub fn fn_with_params(_param: ()) {}
            "#;
        test_format_item(test_src, "fn_with_params", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Error formatting the type of parameter #0: \
                             `()` / `void` is only supported as a return type (b/254507801)");
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_param_type_never() {
        let test_src = r#"
                #![feature(never_type)]

                #[no_mangle]
                pub extern "C" fn fn_with_params(_param: !) {}
            "#;
        test_format_item(test_src, "fn_with_params", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error formatting the type of parameter #0: \
                 The never type `!` is only supported as a return type (b/254507801)"
            );
        });
    }

    /// This is a test for a regular struct - a struct with named fields.
    /// https://doc.rust-lang.org/reference/items/structs.html refers to this kind of struct as
    /// `StructStruct` or "nominal struct type".
    #[test]
    fn test_format_item_struct_with_fields() {
        let test_src = r#"
                pub struct SomeStruct {
                    pub x: i32,
                    pub y: i32,
                }

                const _: () = assert!(std::mem::size_of::<SomeStruct>() == 8);
                const _: () = assert!(std::mem::align_of::<SomeStruct>() == 4);
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct alignas(4) SomeStruct final {
                        public:
                            // In this test there is no `Default` implementation.
                            SomeStruct() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            SomeStruct(const SomeStruct&) = delete;

                            // All Rust types are trivially-movable.
                            SomeStruct(SomeStruct&&) = default;

                            // Assignment operators are disabled for now.
                            SomeStruct& operator=(const SomeStruct&) = delete;
                            SomeStruct& operator=(SomeStruct&&) = delete;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~SomeStruct() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[8];
                    };
                }
            );
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    static_assert(sizeof(SomeStruct) == 8, ...);
                    static_assert(alignof(SomeStruct) == 4, ...);
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                }
            );
        });
    }

    /// This is a test for `TupleStruct` or "tuple struct" - for more details
    /// please refer to https://doc.rust-lang.org/reference/items/structs.html
    #[test]
    fn test_format_item_struct_with_tuple() {
        let test_src = r#"
                pub struct TupleStruct(i32, i32);
                const _: () = assert!(std::mem::size_of::<TupleStruct>() == 8);
                const _: () = assert!(std::mem::align_of::<TupleStruct>() == 4);
            "#;
        test_format_item(test_src, "TupleStruct", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct alignas(4) TupleStruct final {
                        public:
                            // In this test there is no `Default` implementation.
                            TupleStruct() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            TupleStruct(const TupleStruct&) = delete;

                            // All Rust types are trivially-movable.
                            TupleStruct(TupleStruct&&) = default;

                            // Assignment operators are disabled for now.
                            TupleStruct& operator=(const TupleStruct&) = delete;
                            TupleStruct& operator=(TupleStruct&&) = delete;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~TupleStruct() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[8];
                    };
                }
            );
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    static_assert(sizeof(TupleStruct) == 8, ...);
                    static_assert(alignof(TupleStruct) == 4, ...);
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::TupleStruct>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::TupleStruct>() == 4);
                }
            );
        });
    }

    #[test]
    fn test_format_item_static_method() {
        let test_src = r#"
                /// No-op `f32` placeholder is used, because ZSTs are not supported
                /// (b/258259459).
                pub struct Math(f32);

                impl Math {
                    pub fn add_i32(x: f32, y: f32) -> f32 {
                        x + y
                    }
                }
            "#;
        test_format_item(test_src, "Math", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... Math final {
                        ...
                        public:
                          ...
                          static inline float add_i32(float x, float y);
                        ...
                    };
                }
            );
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C" float ... (float x, float y);
                    }
                    inline float Math::add_i32(float x, float y) {
                      return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    #[no_mangle]
                    extern "C" fn ...(x: f32, y: f32) -> f32 {
                        ::rust_out::Math::add_i32(x, y)
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_static_method_with_generic_type_parameters() {
        let test_src = r#"
                /// No-op `f32` placeholder is used, because ZSTs are not supported
                /// (b/258259459).
                pub struct SomeStruct(f32);

                impl SomeStruct {
                    // To make this testcase distinct / non-overlapping wrt
                    // test_format_item_static_method_with_generic_lifetime_parameters
                    // `t` is taken by value below.
                    pub fn generic_method<T: Clone>(t: T) -> T {
                        t.clone()
                    }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            let unsupported_msg = "Error generating bindings for `SomeStruct::generic_method` \
                                   defined at <crubit_unittests.rs>;l=10: \
                                   Generic functions are not supported yet (b/259749023)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
            assert_cc_not_matches!(impl_details.cc.tokens, quote! { SomeStruct::generic_method },);
            assert_rs_not_matches!(impl_details.rs, quote! { generic_method },);
        });
    }

    #[test]
    fn test_format_item_static_method_with_generic_lifetime_parameters() {
        let test_src = r#"
                /// No-op `f32` placeholder is used, because ZSTs are not supported
                /// (b/258259459).
                pub struct SomeStruct(f32);

                impl SomeStruct {
                    pub fn fn_taking_reference<'a>(x: &'a i32) -> i32 { *x }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            let unsupported_msg = "Error generating bindings for `SomeStruct::fn_taking_reference` \
                                   defined at <crubit_unittests.rs>;l=7: \
                                   Generic functions are not supported yet (b/259749023)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
            assert_cc_not_matches!(
                impl_details.cc.tokens,
                quote! { SomeStruct::fn_taking_reference },
            );
            assert_rs_not_matches!(impl_details.rs, quote! { fn_taking_reference },);
        });
    }

    #[test]
    fn test_format_item_method_taking_self_by_value() {
        let test_src = r#"
                pub struct SomeStruct(f32);

                impl SomeStruct {
                    pub fn into_f32(self) -> f32 {
                        self.0
                    }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            let unsupported_msg = "Error generating bindings for `SomeStruct::into_f32` \
                                   defined at <crubit_unittests.rs>;l=5: \
                                   `self` parameter is not supported yet";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
            assert_cc_not_matches!(impl_details.cc.tokens, quote! { SomeStruct::into_f32 },);
            assert_rs_not_matches!(impl_details.rs, quote! { into_f32 },);
        });
    }

    #[test]
    fn test_format_item_method_taking_self_by_const_ref() {
        let test_src = r#"
                pub struct SomeStruct(f32);

                impl SomeStruct {
                    pub fn get_f32(&self) -> f32 {
                        self.0
                    }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            let unsupported_msg = "Error generating bindings for `SomeStruct::get_f32` \
                                   defined at <crubit_unittests.rs>;l=5: \
                                   Generic functions are not supported yet (b/259749023)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
            assert_cc_not_matches!(impl_details.cc.tokens, quote! { SomeStruct::get_f32 },);
            assert_rs_not_matches!(impl_details.rs, quote! { get_f32 },);
        });
    }

    #[test]
    fn test_format_item_method_taking_self_by_mutable_ref() {
        let test_src = r#"
                pub struct SomeStruct(f32);

                impl SomeStruct {
                    pub fn set_f32(&mut self, new_value: f32) {
                        self.0 = new_value;
                    }
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            let unsupported_msg = "Error generating bindings for `SomeStruct::set_f32` \
                                   defined at <crubit_unittests.rs>;l=5: \
                                   Generic functions are not supported yet (b/259749023)";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct ... SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
            assert_cc_not_matches!(impl_details.cc.tokens, quote! { SomeStruct::set_f32 },);
            assert_rs_not_matches!(impl_details.rs, quote! { set_f32 },);
        });
    }

    #[test]
    fn test_format_item_unsupported_struct_with_name_that_is_reserved_keyword() {
        let test_src = r#"
                #[allow(non_camel_case_types)]
                pub struct reinterpret_cast {
                    pub x: i32,
                    pub y: i32,
                }
            "#;
        test_format_item(test_src, "reinterpret_cast", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error formatting item name: \
                             `reinterpret_cast` is a C++ reserved keyword \
                             and can't be used as a C++ identifier"
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_struct_with_custom_drop_impl() {
        let test_src = r#"
                pub struct StructWithCustomDropImpl {
                    pub x: i32,
                    pub y: i32,
                }

                impl Drop for StructWithCustomDropImpl {
                    fn drop(&mut self) {}
                }
            "#;
        test_format_item(test_src, "StructWithCustomDropImpl", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "`Drop` trait and \"drop glue\" are not supported yet (b/258251148)");
        });
    }

    #[test]
    fn test_format_item_unsupported_struct_with_custom_drop_glue() {
        let test_src = r#"
                #![allow(dead_code)]

                // `i32` is present to avoid hitting the ZST checks related to (b/258259459)
                struct StructWithCustomDropImpl(i32);

                impl Drop for StructWithCustomDropImpl {
                    fn drop(&mut self) {
                        println!("dropping!");
                    }
                }

                pub struct StructRequiringCustomDropGlue {
                    field: StructWithCustomDropImpl,
                }
            "#;
        test_format_item(test_src, "StructRequiringCustomDropGlue", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "`Drop` trait and \"drop glue\" are not supported yet (b/258251148)");
        });
    }

    /// This test covers how ZSTs (zero-sized-types) are handled.
    /// https://doc.rust-lang.org/reference/items/structs.html refers to this kind of struct as a
    /// "unit-like struct".
    #[test]
    fn test_format_item_unsupported_struct_zero_sized_type() {
        let test_src = r#"
                pub struct ZeroSizedType1;
                pub struct ZeroSizedType2();
                pub struct ZeroSizedType3{}
            "#;
        for name in ["ZeroSizedType1", "ZeroSizedType2", "ZeroSizedType3"] {
            test_format_item(test_src, name, |result| {
                let err = result.unwrap_err();
                assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)");
            });
        }
    }

    /// This is a test for an enum that only has `EnumItemDiscriminant` items
    /// (and doesn't have `EnumItemTuple` or `EnumItemStruct` items).  See
    /// also https://doc.rust-lang.org/reference/items/enumerations.html
    #[test]
    fn test_format_item_enum_with_only_discriminant_items() {
        let test_src = r#"
                pub enum SomeEnum {
                    Red,
                    Green = 123,
                    Blue,
                }

                const _: () = assert!(std::mem::size_of::<SomeEnum>() == 1);
                const _: () = assert!(std::mem::align_of::<SomeEnum>() == 1);
            "#;
        test_format_item(test_src, "SomeEnum", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct alignas(1) SomeEnum final {
                        public:
                            // In this test there is no `Default` implementation.
                            SomeEnum() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            SomeEnum(const SomeEnum&) = delete;

                            // All Rust types are trivially-movable.
                            SomeEnum(SomeEnum&&) = default;

                            // Assignment operators are disabled for now.
                            SomeEnum& operator=(const SomeEnum&) = delete;
                            SomeEnum& operator=(SomeEnum&&) = delete;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~SomeEnum() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[1];
                    };
                }
            );
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    static_assert(sizeof(SomeEnum) == 1, ...);
                    static_assert(alignof(SomeEnum) == 1, ...);
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeEnum>() == 1);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeEnum>() == 1);
                }
            );
        });
    }

    /// This is a test for an enum that has `EnumItemTuple` and `EnumItemStruct`
    /// items. See also https://doc.rust-lang.org/reference/items/enumerations.html
    #[test]
    fn test_format_item_enum_with_tuple_and_struct_items() {
        let test_src = r#"
                pub enum Point {
                    Cartesian(f32, f32),
                    Polar{ dist: f32, angle: f32 },
                }

                const _: () = assert!(std::mem::size_of::<Point>() == 12);
                const _: () = assert!(std::mem::align_of::<Point>() == 4);
            "#;
        test_format_item(test_src, "Point", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct alignas(4) Point final {
                        public:
                            // In this test there is no `Default` implementation.
                            Point() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            Point(const Point&) = delete;

                            // All Rust types are trivially-movable.
                            Point(Point&&) = default;

                            // Assignment operators are disabled for now.
                            Point& operator=(const Point&) = delete;
                            Point& operator=(Point&&) = delete;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~Point() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[12];
                    };
                }
            );
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    static_assert(sizeof(Point) == 12, ...);
                    static_assert(alignof(Point) == 4, ...);
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::Point>() == 12);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::Point>() == 4);
                }
            );
        });
    }

    /// This test covers how zero-variant enums are handled.  See also
    /// https://doc.rust-lang.org/reference/items/enumerations.html#zero-variant-enums
    #[test]
    fn test_format_item_unsupported_enum_zero_variants() {
        let test_src = r#"
                pub enum ZeroVariantEnum {}
            "#;
        test_format_item(test_src, "ZeroVariantEnum", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)");
        });
    }

    /// This is a test for a `union`.  See also
    /// https://doc.rust-lang.org/reference/items/unions.html
    #[test]
    fn test_format_item_union() {
        let test_src = r#"
                pub union SomeUnion {
                    pub i: i32,
                    pub f: f64,
                }

                const _: () = assert!(std::mem::size_of::<SomeUnion>() == 8);
                const _: () = assert!(std::mem::align_of::<SomeUnion>() == 8);
            "#;
        test_format_item(test_src, "SomeUnion", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let impl_details = get_impl_details_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct alignas(8) SomeUnion final {
                        public:
                            // In this test there is no `Default` implementation.
                            SomeUnion() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            SomeUnion(const SomeUnion&) = delete;

                            // All Rust types are trivially-movable.
                            SomeUnion(SomeUnion&&) = default;

                            // Assignment operators are disabled for now.
                            SomeUnion& operator=(const SomeUnion&) = delete;
                            SomeUnion& operator=(SomeUnion&&) = delete;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~SomeUnion() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[8];
                    };
                }
            );
            assert_cc_matches!(
                impl_details.cc.tokens,
                quote! {
                    static_assert(sizeof(SomeUnion) == 8, ...);
                    static_assert(alignof(SomeUnion) == 8, ...);
                }
            );
            assert_rs_matches!(
                impl_details.rs,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 8);
                }
            );
        });
    }

    #[test]
    fn test_format_item_doc_comments_union() {
        let test_src = r#"
            /// Doc for some union.
            pub union SomeUnionWithDocs {
                /// Doc for a field in a union.
                pub i: i32,
                pub f: f64
            }
        "#;
        test_format_item(test_src, "SomeUnionWithDocs", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let comment = " Doc for some union.\n\n\
                           Generated from: <crubit_unittests.rs>;l=3";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    struct ... SomeUnionWithDocs final {
                        ...
                    }
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_doc_comments_enum() {
        let test_src = r#"
            /** Doc for some enum. */
            pub enum SomeEnumWithDocs {
                Kind1(i32),
            }
        "#;
        test_format_item(test_src, "SomeEnumWithDocs", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let comment = " Doc for some enum. \n\n\
                            Generated from: <crubit_unittests.rs>;l=3";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    struct ... SomeEnumWithDocs final {
                        ...
                    }
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_doc_comments_struct() {
        let test_src = r#"
            #![allow(dead_code)]
            #[doc = "Doc for some struct."]
            pub struct SomeStructWithDocs {
                some_field : i32,
            }
        "#;
        test_format_item(test_src, "SomeStructWithDocs", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let comment = "Doc for some struct.\n\n\
                           Generated from: <crubit_unittests.rs>;l=4";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    struct ... SomeStructWithDocs final {
                        ...
                    }
                    ...
                }
            );
        });
    }

    #[test]
    fn test_format_item_doc_comments_tuple_struct() {
        let test_src = r#"
            /// Doc for some tuple struct.
            pub struct SomeTupleStructWithDocs(i32);
        "#;
        test_format_item(test_src, "SomeTupleStructWithDocs", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let comment = " Doc for some tuple struct.\n\n\
                           Generated from: <crubit_unittests.rs>;l=3";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    struct ... SomeTupleStructWithDocs final {
                        ...
                    }
                    ...
                },
            );
        });
    }

    #[test]
    fn test_format_item_source_loc_macro_rules() {
        let test_src = r#"
            macro_rules! some_tuple_struct_macro_for_testing_source_loc {
                () => {
                    /// Some doc on SomeTupleStructMacroForTesingSourceLoc.
                    pub struct SomeTupleStructMacroForTesingSourceLoc(i32);
                };
            }

            some_tuple_struct_macro_for_testing_source_loc!();
        "#;
        test_format_item(test_src, "SomeTupleStructMacroForTesingSourceLoc", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let source_loc_comment = " Some doc on SomeTupleStructMacroForTesingSourceLoc.\n\n\
                                      Generated from: <crubit_unittests.rs>;l=5";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #source_loc_comment
                    struct ... SomeTupleStructMacroForTesingSourceLoc final {
                        ...
                    }
                    ...
                },
            );
        });
    }

    #[test]
    fn test_format_item_source_loc_with_no_doc_comment() {
        let test_src = r#"
            pub struct SomeTupleStructWithNoDocComment(i32);
        "#;
        test_format_item(test_src, "SomeTupleStructWithNoDocComment", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            let comment = "Generated from: <crubit_unittests.rs>;l=2";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    struct ... SomeTupleStructWithNoDocComment final {
                        ...
                    }
                    ...
                },
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_static_value() {
        let test_src = r#"
                #[no_mangle]
                pub static STATIC_VALUE: i32 = 42;
            "#;
        test_format_item(test_src, "STATIC_VALUE", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Unsupported rustc_hir::hir::ItemKind: static item");
        });
    }

    #[test]
    fn test_format_item_unsupported_const_value() {
        let test_src = r#"
                pub const CONST_VALUE: i32 = 42;
            "#;
        test_format_item(test_src, "CONST_VALUE", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Unsupported rustc_hir::hir::ItemKind: constant item");
        });
    }

    #[test]
    fn test_format_item_unsupported_type_alias() {
        let test_src = r#"
                pub type TypeAlias = i32;
            "#;
        test_format_item(test_src, "TypeAlias", |result| {
            // TODO(b/254096006): Add support for type alias definitions.
            let err = result.unwrap_err();
            assert_eq!(err, "Unsupported rustc_hir::hir::ItemKind: type alias");
        });
    }

    #[test]
    fn test_format_item_unsupported_impl_item_const_value() {
        let test_src = r#"
                pub struct SomeStruct(i32);

                impl SomeStruct {
                    pub const CONST_VALUE: i32 = 42;
                }
            "#;
        test_format_item(test_src, "SomeStruct", |result| {
            let result = result.unwrap();
            let main_api = get_main_api_snippet(&result);
            assert!(main_api.prereqs.is_empty());
            let unsupported_msg = "Error generating bindings for `SomeStruct::CONST_VALUE` \
                                   defined at <crubit_unittests.rs>;l=5: \
                                   Unsupported `impl` item kind: Const";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    struct alignas(4) SomeStruct final {
                        ...
                        __COMMENT__ #unsupported_msg
                        ...
                    };
                    ...
                }
            );
        });
    }

    /// `test_format_ret_ty_for_cc_successes` provides test coverage for cases
    /// where `format_ret_ty_for_cc` returns an `Ok(...)`.  Additional
    /// testcases are covered by `test_format_ty_for_cc_successes` (because
    /// `format_ret_ty_for_cc` delegates most cases to `format_ty_for_cc`).
    #[test]
    fn test_format_ret_ty_for_cc_successes() {
        let testcases = [
            // ( <Rust type>, <expected C++ type> )
            ("bool", "bool"), // TyKind::Bool
            ("()", "void"),
            // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
            // details).
            ("!", "void"),
        ];
        test_ty(&testcases, quote! {}, |desc, tcx, ty, expected| {
            let actual = {
                let input = bindings_input_for_tests(tcx);
                let cc_snippet = format_ret_ty_for_cc(&input, ty).unwrap();
                assert!(cc_snippet.prereqs.is_empty());
                cc_snippet.tokens.to_string()
            };
            let expected = expected.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual, expected, "{desc}");
        });
    }

    /// `test_format_ty_for_cc_successes` provides test coverage for cases where
    /// `format_ty_for_cc` returns an `Ok(...)`.
    ///
    /// Note that using `std::int8_t` (instead of `::std::int8_t`) has been an
    /// explicit decision. The "Google C++ Style Guide" suggests to "avoid
    /// nested namespaces that match well-known top-level namespaces" and "in
    /// particular, [...] not create any nested std namespaces.".  It
    /// seems desirable if the generated bindings conform to this aspect of the
    /// style guide, because it makes things easier for *users* of these
    /// bindings.
    #[test]
    fn test_format_ty_for_cc_successes() {
        let testcases = [
            // ( <Rust type>, (<expected C++ type>,
            //                 <expected #include>,
            //                 <expected prereq def>,
            //                 <expected prereq fwd decl>) )
            ("bool", ("bool", "", "", "")),
            ("f32", ("float", "", "", "")),
            ("f64", ("double", "", "", "")),
            ("i8", ("std::int8_t", "<cstdint>", "", "")),
            ("i16", ("std::int16_t", "<cstdint>", "", "")),
            ("i32", ("std::int32_t", "<cstdint>", "", "")),
            ("i64", ("std::int64_t", "<cstdint>", "", "")),
            ("isize", ("std::intptr_t", "<cstdint>", "", "")),
            ("u8", ("std::uint8_t", "<cstdint>", "", "")),
            ("u16", ("std::uint16_t", "<cstdint>", "", "")),
            ("u32", ("std::uint32_t", "<cstdint>", "", "")),
            ("u64", ("std::uint64_t", "<cstdint>", "", "")),
            ("usize", ("std::uintptr_t", "<cstdint>", "", "")),
            ("char", ("rstd::Char", "\"crubit/support/for/tests/rstd/char.h\"", "", "")),
            ("SomeStruct", ("::rust_out::SomeStruct", "", "SomeStruct", "")),
            ("SomeEnum", ("::rust_out::SomeEnum", "", "SomeEnum", "")),
            ("SomeUnion", ("::rust_out::SomeUnion", "", "SomeUnion", "")),
            ("*const i32", ("const std::int32_t*", "<cstdint>", "", "")),
            ("*mut i32", ("std::int32_t*", "<cstdint>", "", "")),
            // `SomeStruct` is a `fwd_decls` prerequisite (not `defs` prerequisite):
            ("*mut SomeStruct", ("::rust_out::SomeStruct*", "", "", "SomeStruct")),
            // Testing propagation of deeper/nested `fwd_decls`:
            ("*mut *mut SomeStruct", (":: rust_out :: SomeStruct * *", "", "", "SomeStruct")),
            // Extra parens/sugar are expected to be ignored:
            ("(bool)", ("bool", "", "", "")),
        ];
        let preamble = quote! {
            #![allow(unused_parens)]

            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }
            pub enum SomeEnum {
                Cartesian{x: f64, y: f64},
                Polar{angle: f64, dist: f64},
            }
            pub union SomeUnion {
                pub x: i32,
                pub y: i32,
            }
        };
        test_ty(
            &testcases,
            preamble,
            |desc, tcx, ty,
             (expected_tokens, expected_include, expected_prereq_def, expected_prereq_fwd_decl)| {
                let (actual_tokens, actual_prereqs) = {
                    let input = bindings_input_for_tests(tcx);
                    let s = format_ty_for_cc(&input, ty).unwrap();
                    (s.tokens.to_string(), s.prereqs)
                };
                let (actual_includes, actual_prereq_defs, actual_prereq_fwd_decls) =
                    (actual_prereqs.includes, actual_prereqs.defs, actual_prereqs.fwd_decls);

                let expected_tokens = expected_tokens.parse::<TokenStream>().unwrap().to_string();
                assert_eq!(actual_tokens, expected_tokens, "{desc}");

                if expected_include.is_empty() {
                    assert!(actual_includes.is_empty());
                } else {
                    let expected_include: TokenStream = expected_include.parse().unwrap();
                    assert_cc_matches!(
                        format_cc_includes(&actual_includes),
                        quote! { __HASH_TOKEN__ include #expected_include }
                    );
                }

                if expected_prereq_def.is_empty() {
                    assert!(actual_prereq_defs.is_empty());
                } else {
                    let expected_def_id = find_def_id_by_name(tcx, expected_prereq_def);
                    assert_eq!(1, actual_prereq_defs.len());
                    assert_eq!(expected_def_id, actual_prereq_defs.into_iter().next().unwrap());
                }

                if expected_prereq_fwd_decl.is_empty() {
                    assert!(actual_prereq_fwd_decls.is_empty());
                } else {
                    let expected_def_id = find_def_id_by_name(tcx, expected_prereq_fwd_decl);
                    assert_eq!(1, actual_prereq_fwd_decls.len());
                    assert_eq!(expected_def_id,
                               actual_prereq_fwd_decls.into_iter().next().unwrap());
                }
            },
        );
    }

    /// `test_format_ty_for_cc_failures` provides test coverage for cases where
    /// `format_ty_for_cc` returns an `Err(...)`.
    ///
    /// It seems okay to have no test coverage for now for the following types
    /// (which should never be encountered when generating bindings and where
    /// `format_ty_for_cc` should panic):
    /// - TyKind::Closure
    /// - TyKind::Error
    /// - TyKind::FnDef
    /// - TyKind::Infer
    ///
    /// TODO(lukasza): Add test coverage (here and in the "for_rs" flavours)
    /// for:
    /// - TyKind::Bound
    /// - TyKind::Dynamic (`dyn Eq`)
    /// - TyKind::Foreign (`extern type T`)
    /// - https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html:
    ///   TyKind::Generator, TyKind::GeneratorWitness
    /// - TyKind::Param
    /// - TyKind::Placeholder
    #[test]
    fn test_format_ty_for_cc_failures() {
        let testcases = [
            // ( <Rust type>, <expected error message> )
            (
                "()", // Empty TyKind::Tuple
                "`()` / `void` is only supported as a return type (b/254507801)"
            ),
            (
                // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
                // details).
                "!", // TyKind::Never
                "The never type `!` is only supported as a return type (b/254507801)"
            ),
            (
                "(i32, i32)", // Non-empty TyKind::Tuple
                "Tuples are not supported yet: (i32, i32) (b/254099023)",
            ),
            (
                "&'static i32", // TyKind::Ref
                "The following Rust type is not supported yet: &'static i32",
            ),
            (
                "[i32; 42]", // TyKind::Array
                "The following Rust type is not supported yet: [i32; 42]",
            ),
            (
                "&'static [i32]", // TyKind::Slice (nested underneath TyKind::Ref)
                "The following Rust type is not supported yet: &'static [i32]",
            ),
            (
                "&'static str", // TyKind::Str (nested underneath TyKind::Ref)
                "The following Rust type is not supported yet: &'static str",
            ),
            (
                "impl Eq", // TyKind::Alias
                "The following Rust type is not supported yet: impl std::cmp::Eq",
            ),
            (
                "fn(i32) -> i32", // TyKind::FnPtr
                "The following Rust type is not supported yet: fn(i32) -> i32",
            ),
            // TODO(b/254094650): Consider mapping this to Clang's (and GCC's) `__int128`
            // or to `absl::in128`.
            ("i128", "C++ doesn't have a standard equivalent of `i128` (b/254094650)"),
            ("u128", "C++ doesn't have a standard equivalent of `u128` (b/254094650)"),
            (
                "StructWithCustomDrop",
                "Failed to generate bindings for the definition of `StructWithCustomDrop`: \
                 `Drop` trait and \"drop glue\" are not supported yet (b/258251148)"
            ),
            (
                "ConstGenericStruct<42>",
                "Generic types are not supported yet (b/259749095)",
            ),
            (
                "TypeGenericStruct<u8>",
                "Generic types are not supported yet (b/259749095)",
            ),
            (
                // This double-checks that TyKind::Adt(..., substs) are present
                // even if the type parameter argument is not explicitly specified
                // (here it comes from the default: `...Struct<T = u8>`).
                "TypeGenericStruct",
                "Generic types are not supported yet (b/259749095)",
            ),
            (
                "LifetimeGenericStruct<'static>",
                "Generic types are not supported yet (b/259749095)",
            ),
            (
                "std::cmp::Ordering",
                "Cross-crate dependencies are not supported yet (b/258261328)",
            ),
            (
                "Option<i8>",
                "Generic types are not supported yet (b/259749095)",
            ),
        ];
        let preamble = quote! {
            #![feature(never_type)]

            pub struct StructWithCustomDrop {
                pub x: i32,
                pub y: i32,
            }

            impl Drop for StructWithCustomDrop {
                fn drop(&mut self) {}
            }

            pub struct ConstGenericStruct<const N: usize> {
                pub arr: [u8; N],
            }

            pub struct TypeGenericStruct<T = u8> {
                pub t: T,
            }

            pub struct LifetimeGenericStruct<'a> {
                pub reference: &'a u8,
            }
        };
        test_ty(&testcases, preamble, |desc, tcx, ty, expected_msg| {
            let input = bindings_input_for_tests(tcx);
            let anyhow_err = format_ty_for_cc(&input, ty).unwrap_err();
            let actual_msg = format!("{anyhow_err:#}");
            assert_eq!(&actual_msg, *expected_msg, "{desc}");
        });
    }

    #[test]
    fn test_format_ty_for_rs_successes() {
        // Test coverage for cases where `format_ty_for_rs` returns an `Ok(...)`.
        let testcases = [
            // ( <Rust type>, <expected Rust spelling for ..._cc_api_impl.rs> )
            ("bool", "bool"),
            ("f32", "f32"),
            ("f64", "f64"),
            ("i8", "i8"),
            ("i16", "i16"),
            ("i32", "i32"),
            ("i64", "i64"),
            ("i128", "i128"),
            ("isize", "isize"),
            ("u8", "u8"),
            ("u16", "u16"),
            ("u32", "u32"),
            ("u64", "u64"),
            ("u128", "u128"),
            ("usize", "usize"),
            ("char", "char"),
            ("!", "!"),
            ("()", "()"),
            // ADTs:
            ("SomeStruct", "::rust_out::SomeStruct"),
            ("SomeEnum", "::rust_out::SomeEnum"),
            ("SomeUnion", "::rust_out::SomeUnion"),
            // Type from another crate:
            ("std::cmp::Ordering", "::core::cmp::Ordering"),
            // `const` and `mut` pointers:
            ("*const i32", "*const i32"),
            ("*mut i32", "*mut i32"),
            // Pointer to an ADT:
            ("*mut SomeStruct", "* mut :: rust_out :: SomeStruct"),
        ];
        let preamble = quote! {
            #![feature(never_type)]

            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }
            pub enum SomeEnum {
                Cartesian{x: f64, y: f64},
                Polar{angle: f64, dist: f64},
            }
            pub union SomeUnion {
                pub x: i32,
                pub y: i32,
            }
        };
        test_ty(&testcases, preamble, |desc, tcx, ty, expected_tokens| {
            let actual_tokens = format_ty_for_rs(tcx, ty).unwrap().to_string();
            let expected_tokens = expected_tokens.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual_tokens, expected_tokens, "{desc}");
        });
    }

    #[test]
    fn test_format_ty_for_rs_failures() {
        // This test provides coverage for cases where `format_ty_for_rs` returns an
        // `Err(...)`.
        let testcases = [
            // ( <Rust type>, <expected error message> )
            (
                "(i32, i32)", // Non-empty TyKind::Tuple
                "Tuples are not supported yet: (i32, i32) (b/254099023)",
            ),
            (
                "&'static i32", // TyKind::Ref
                "The following Rust type is not supported yet: &'static i32",
            ),
            (
                "[i32; 42]", // TyKind::Array
                "The following Rust type is not supported yet: [i32; 42]",
            ),
            (
                "&'static [i32]", // TyKind::Slice (nested underneath TyKind::Ref)
                "The following Rust type is not supported yet: &'static [i32]",
            ),
            (
                "&'static str", // TyKind::Str (nested underneath TyKind::Ref)
                "The following Rust type is not supported yet: &'static str",
            ),
            (
                "impl Eq", // TyKind::Alias
                "The following Rust type is not supported yet: impl std::cmp::Eq",
            ),
            (
                "fn(i32) -> i32", // TyKind::FnPtr
                "The following Rust type is not supported yet: fn(i32) -> i32",
            ),
            (
                "Option<i8>", // TyKind::Adt - generic + different crate
                "Generic types are not supported yet (b/259749095)",
            ),
        ];
        let preamble = quote! {};
        test_ty(&testcases, preamble, |desc, tcx, ty, expected_err| {
            let anyhow_err = format_ty_for_rs(tcx, ty).unwrap_err();
            let actual_err = format!("{anyhow_err:#}");
            assert_eq!(&actual_err, *expected_err, "{desc}");
        });
    }

    #[test]
    fn test_format_cc_thunk_arg() {
        let testcases = [
            // ( <Rust type>, (<expected C++ type>, <expected #include>) )
            ("i32", ("value", "")),
            ("SomeStruct", ("std::move(value)", "utility")),
        ];
        let preamble = quote! {
            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }
        };
        test_ty(&testcases, preamble, |desc, tcx, ty, (expected_tokens, expected_include)| {
            let (actual_tokens, actual_includes) = {
                let cc_snippet = format_cc_thunk_arg(tcx, ty, quote! { value });
                (cc_snippet.tokens.to_string(), cc_snippet.prereqs.includes)
            };

            let expected_tokens = expected_tokens.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual_tokens, expected_tokens, "{desc}");

            if expected_include.is_empty() {
                assert!(actual_includes.is_empty());
            } else {
                let expected_header = format_cc_ident(expected_include).unwrap();
                assert_cc_matches!(
                    format_cc_includes(&actual_includes),
                    quote! { include <#expected_header> }
                );
            }
        });
    }

    fn test_ty<TestFn, Expectation>(
        testcases: &[(&str, Expectation)],
        preamble: TokenStream,
        test_fn: TestFn,
    ) where
        TestFn: for<'tcx> Fn(
                /* testcase_description: */ &str,
                TyCtxt<'tcx>,
                Ty<'tcx>,
                &Expectation,
            ) + Sync,
        Expectation: Sync,
    {
        for (index, (input, expected)) in testcases.iter().enumerate() {
            let desc = format!("test #{index}: test input: `{input}`");
            let input = {
                let ty_tokens: TokenStream = input.parse().unwrap();
                let input = quote! {
                    #preamble
                    pub fn test_function() -> #ty_tokens { panic!("") }
                };
                input.to_string()
            };
            run_compiler_for_testing(input, |tcx| {
                let def_id = find_def_id_by_name(tcx, "test_function");
                let ty = tcx
                    .fn_sig(def_id.to_def_id())
                    .subst_identity()
                    .no_bound_vars()
                    .unwrap()
                    .output();
                test_fn(&desc, tcx, ty, expected);
            });
        }
    }

    fn get_main_api_snippet(format_item_result: &Vec<(SnippetKey, MixedSnippet)>) -> &CcSnippet {
        format_item_result
            .iter()
            .filter_map(|(key, snippet)| match key.kind {
                SnippetKind::MainApi => {
                    assert!(snippet.rs.is_empty(), "MainApi should be C++-only");
                    Some(&snippet.cc)
                }
                _ => None,
            })
            .exactly_one()
            .expect("Expecting exactly 1 MainApi snippet")
    }

    /// Combines all ImplDetails snippets into a single MixedSnippet.
    fn get_impl_details_snippet(
        format_item_result: &Vec<(SnippetKey, MixedSnippet)>,
    ) -> MixedSnippet {
        format_item_result
            .iter()
            .filter_map(|(key, snippet)| match key.kind {
                SnippetKind::ImplDetails => Some(snippet),
                _ => None,
            })
            .fold(Default::default(), |mut acc, snippet| {
                acc.cc.prereqs += snippet.cc.prereqs.clone();
                acc.cc.tokens.extend(snippet.cc.tokens.clone());
                acc.rs.extend(snippet.rs.clone());
                acc
            })
    }

    /// Tests invoking `format_item` on the item with the specified `name` from
    /// the given Rust `source`.  Returns the result of calling
    /// `test_function` with `format_item`'s result as an argument.
    /// (`test_function` should typically `assert!` that it got the expected
    /// result from `format_item`.)
    fn test_format_item<F, T>(source: &str, name: &str, test_function: F) -> T
    where
        F: FnOnce(Result<Vec<(SnippetKey, MixedSnippet)>, String>) -> T + Send,
        T: Send,
    {
        run_compiler_for_testing(source, |tcx| {
            let def_id = find_def_id_by_name(tcx, name);
            let result = format_item(&bindings_input_for_tests(tcx), def_id);

            // Sort the vector of results to make the tests more deterministic.  Below (i.e. in
            // tests) we use a somewhat arbitrary SnippetKey-based order.  The order of these
            // intermediate results doesn't matter in prod, because they will later get toposorted
            // based on CcPrerequisites.
            let result = result.map(|mut vec| {
                let cmp = preferred_snippet_order(tcx);
                vec.sort_by(|(key1, _), (key2, _)| cmp(key1, key2));
                vec
            });

            // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations says:
            // To print causes as well [...], use the alternate selector “{:#}”.
            let result = result.map_err(|anyhow_err| format!("{anyhow_err:#}"));

            test_function(result)
        })
    }

    /// Finds the definition id of a Rust item with the specified `name`.
    /// Panics if no such item is found, or if there is more than one match.
    fn find_def_id_by_name(tcx: TyCtxt, name: &str) -> LocalDefId {
        let hir_items = || tcx.hir().items().map(|item_id| tcx.hir().item(item_id));
        let items_with_matching_name =
            hir_items().filter(|item| item.ident.name.as_str() == name).collect_vec();
        match *items_with_matching_name.as_slice() {
            [] => {
                let found_names = hir_items()
                    .map(|item| item.ident.name.as_str())
                    .filter(|s| !s.is_empty())
                    .sorted()
                    .dedup()
                    .map(|name| format!("`{name}`"))
                    .join(",\n");
                panic!("No items named `{name}`.\nInstead found:\n{found_names}");
            }
            [item] => item.owner_id.def_id,
            _ => panic!("More than one item named `{name}`"),
        }
    }

    fn bindings_input_for_tests(tcx: TyCtxt) -> Input {
        Input {
            tcx,
            crubit_support_path: "crubit/support/for/tests".into(),
            _features: (),
            _crate_to_include_map: (),
        }
    }

    /// Tests invoking `generate_bindings` on the given Rust `source`.
    /// Returns the result of calling `test_function` with the generated
    /// bindings as an argument. (`test_function` should typically `assert!`
    /// that it got the expected `GeneratedBindings`.)
    fn test_generated_bindings<F, T>(source: &str, test_function: F) -> T
    where
        F: FnOnce(Result<Output>) -> T + Send,
        T: Send,
    {
        run_compiler_for_testing(source, |tcx| {
            test_function(generate_bindings(&bindings_input_for_tests(tcx)))
        })
    }
}
