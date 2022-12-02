// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{anyhow, bail, ensure, Context, Result};
use code_gen_utils::{
    format_cc_ident, format_cc_includes, make_rs_ident, CcInclude, NamespaceQualifier,
};
use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use rustc_hir::definitions::{DefPathData, DisambiguatedDefPathData};
use rustc_hir::{Item, ItemKind, Node, Unsafety};
use rustc_interface::Queries;
use rustc_middle::dep_graph::DepContext;
use rustc_middle::ty::{self, Ty, TyCtxt}; // See <internal link>/ty.html#import-conventions
use rustc_span::def_id::{DefId, LocalDefId, LOCAL_CRATE};
use rustc_span::symbol::Symbol;
use rustc_target::abi::Layout;
use rustc_target::spec::abi::Abi;
use rustc_target::spec::PanicStrategy;
use std::collections::BTreeSet;
use std::iter::Sum;
use std::ops::{Add, AddAssign};

pub struct GeneratedBindings {
    pub h_body: TokenStream,
    pub rs_body: TokenStream,
}

impl GeneratedBindings {
    pub fn generate(tcx: TyCtxt) -> Result<Self> {
        match tcx.sess().panic_strategy() {
            PanicStrategy::Unwind => bail!("No support for panic=unwind strategy (b/254049425)"),
            PanicStrategy::Abort => (),
        };

        let top_comment = {
            let crate_name = tcx.crate_name(LOCAL_CRATE);
            let txt = format!(
                "Automatically @generated C++ bindings for the following Rust crate:\n\
                 {crate_name}"
            );
            quote! { __COMMENT__ #txt __NEWLINE__ }
        };

        let Self { h_body, rs_body } = format_crate(tcx).unwrap_or_else(|err| {
            let txt = format!("Failed to generate bindings for the crate: {err}");
            let src = quote! { __COMMENT__ #txt };
            Self { h_body: src.clone(), rs_body: src }
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

        Ok(Self { h_body, rs_body })
    }
}

/// Helper (used by `bindings_driver` and `test::run_compiler`) for invoking
/// functions operating on `TyCtxt`.
pub fn enter_tcx<'tcx, F, T>(
    queries: &'tcx Queries<'tcx>,
    f: F,
) -> rustc_interface::interface::Result<T>
where
    F: FnOnce(TyCtxt<'tcx>) -> T + Send,
    T: Send,
{
    let query_context = queries.global_ctxt()?;
    Ok(query_context.peek_mut().enter(f))
}

#[derive(Debug, Default)]
struct CcSnippet {
    snippet: TokenStream,

    /// Set of `#include`s that the `snippet` depends on.  For example if
    /// `snippet` expands to `std::int32_t`, then `includes` need to cover
    /// the `cstdint`.
    includes: BTreeSet<CcInclude>,
}

impl CcSnippet {
    /// Consumes `self` and returns the main `snippet`, while preserving
    /// `includes` into the `external_includes` out parameter.
    fn into_tokens(mut self, external_includes: &mut BTreeSet<CcInclude>) -> TokenStream {
        external_includes.append(&mut self.includes);
        self.snippet
    }
}

/// Represents the fully qualified name of a Rust item (e.g. a `struct` or a
/// function).
struct FullyQualifiedName {
    /// Name of the crate that defines the item.
    /// For example, this would be `std` for `std::cmp::Ordering`.
    krate: Symbol,

    /// Path to the module where the item is located.
    /// For example, this would be `cmp` for `std::cmp::Ordering`.
    /// The path can contain multiple modules - e.g. `foo::bar::baz`.
    mod_path: NamespaceQualifier,

    /// Name of the item.
    name: Symbol,
}

impl FullyQualifiedName {
    fn new(tcx: TyCtxt, def_id: DefId) -> Self {
        fn get_symbol(path_component: DisambiguatedDefPathData) -> Symbol {
            match path_component.data {
                DefPathData::TypeNs(symbol) | DefPathData::ValueNs(symbol) => symbol,
                other_data => panic!("Unexpected `path_component`: {other_data}"),
            }
        }

        let krate = tcx.crate_name(def_id.krate);

        let mut full_path = tcx.def_path(def_id).data; // mod_path + name
        let name = full_path.pop().expect("At least the item's name should be present");
        let name = get_symbol(name);

        let mod_path = full_path.into_iter().map(get_symbol).map(|s| s.as_str().into()).collect();
        let mod_path = NamespaceQualifier(mod_path);

        Self { krate, mod_path, name }
    }

    fn format_for_cc(&self) -> Result<TokenStream> {
        let top_level_ns = format_cc_ident(self.krate.as_str())?;
        let ns_path = self.mod_path.format_for_cc()?;
        let name = format_cc_ident(self.name.as_str())?;
        Ok(quote! { :: #top_level_ns :: #ns_path #name })
    }

    fn format_for_rs(&self) -> TokenStream {
        let top_level_ns = make_rs_ident(self.krate.as_str());
        let ns_path = self.mod_path.format_for_rs();
        let name = make_rs_ident(self.name.as_str());
        quote! { :: #top_level_ns :: #ns_path #name }
    }
}

fn format_ret_ty_for_cc(tcx: TyCtxt, ty: Ty) -> Result<CcSnippet> {
    let void = Ok(CcSnippet { snippet: quote! { void }, includes: BTreeSet::new() });
    match ty.kind() {
        ty::TyKind::Never => void,  // `!`
        ty::TyKind::Tuple(types) if types.len() == 0 => void,  // `()`
        _ => format_ty_for_cc(tcx, ty),
    }
}

/// Formats an argument of a thunk.  For example:
/// - most primitive types are passed as-is - e.g. `123`
/// - structs need to be moved: `std::move(value)`
/// - in the future additional processing may be needed for other types (this is
///   speculative so please take these examples with a grain of salt):
///     - str: utf-8 verification
///     - &T: calling into `crubit::MutRef::unsafe_get_ptr`
fn format_cc_thunk_arg<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, value: TokenStream) -> CcSnippet {
    let mut includes = BTreeSet::new();
    if ty.is_copy_modulo_regions(tcx, ty::ParamEnv::empty()) {
        CcSnippet { includes, snippet: value }
    } else {
        includes.insert(CcInclude::utility());
        CcSnippet { includes, snippet: quote! { std::move(#value) } }
    }
}

/// Formats `ty` into a `CcSnippet` that represents how the type should be
/// spelled in a C++ declaration of an `extern "C"` function.
fn format_ty_for_cc(tcx: TyCtxt, ty: Ty) -> Result<CcSnippet> {
    fn cstdint(snippet: TokenStream) -> CcSnippet {
        let mut includes = BTreeSet::new();
        includes.insert(CcInclude::cstdint());
        CcSnippet { snippet, includes }
    }
    fn keyword(snippet: TokenStream) -> CcSnippet {
        CcSnippet { snippet, includes: BTreeSet::new() }
    }
    Ok(match ty.kind() {
        ty::TyKind::Never => {
            // TODO(b/254507801): Maybe translate into `crubit::Never`?
            bail!("The never type `!` is only supported as a return type (b/254507801)");
        },
        ty::TyKind::Tuple(types) => {
            if types.len() == 0 {
                // TODO(b/254507801): Maybe translate into `crubit::Unit`?
                bail!("The unit type `()` / `void` is only supported as a return type");
            } else {
                // TODO(b/254099023): Add support for tuples.
                bail!("Tuples are not supported yet: {} (b/254099023)", ty);
            }
        }

        ty::TyKind::Bool => keyword(quote! { bool }),

        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#fixed-width-floating-point-types
        // documents that "When the platforms' "math.h" header defines the __STDC_IEC_559__ macro,
        // Rust's floating-point types are safe to use directly in C FFI where the appropriate C
        // types are expected (f32 for float, f64 for double)."
        //
        // TODO(b/255768062): Generated bindings should explicitly check `__STDC_IEC_559__`
        ty::TyKind::Float(ty::FloatTy::F32) => keyword(quote! { float }),
        ty::TyKind::Float(ty::FloatTy::F64) => keyword(quote! { double }),

        ty::TyKind::Char => {
            // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#char
            // documents that "Rust char is 32-bit wide and represents an unicode scalar value".
            //
            // We don't map Rust's `char` to C++ `char32_t` because
            // - It may be wider than 32 bits - <internal link>/c/string/multibyte/char32_t says that
            //   "char32_t is an unsigned integer type used for 32-bit wide characters and is the
            //   same type as uint_least32_t. uint_least32_t is the smallest unsigned integer type
            //   with width of at least 32 bits"
            // - It is problematic on MacOS - https://github.com/eqrion/cbindgen/issues/423
            //   points out that `uchar.h` is missing on that platform.
            cstdint(quote!{ std::uint32_t })
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
            if substs.len() != 0 {
                bail!("Generic types are not supported yet (b/259749095)");
            }

            // Verify if definition of `ty` can be succesfully imported and bail otherwise.
            let def_id = adt.did();
            format_adt_core(tcx, def_id)
                .with_context(|| format!(
                        "Failed to generate bindings for the definition of `{ty}`"))?;

            let includes = if def_id.krate == LOCAL_CRATE {
                BTreeSet::new()  // No extra `#include`s needed.
            } else {
                bail!("Cross-crate dependencies are not supported yet (b/258261328)");
            };

            CcSnippet {
                snippet: FullyQualifiedName::new(tcx, def_id).format_for_cc()?,
                includes
            }
        },
        ty::TyKind::Foreign(..)
        | ty::TyKind::Str
        | ty::TyKind::Array(..)
        | ty::TyKind::Slice(..)
        | ty::TyKind::RawPtr(..)
        | ty::TyKind::Ref(..)
        | ty::TyKind::FnPtr(..)
        | ty::TyKind::Dynamic(..)
        | ty::TyKind::Generator(..)
        | ty::TyKind::GeneratorWitness(..)
        | ty::TyKind::Projection(..)
        | ty::TyKind::Opaque(..)
        | ty::TyKind::Param(..)
        | ty::TyKind::Bound(..)
        | ty::TyKind::Placeholder(..) => {
            bail!("The following Rust type is not supported yet: {ty}")
        }
        ty::TyKind::Closure(..)
        | ty::TyKind::FnDef(..)
        | ty::TyKind::Infer(..)
        | ty::TyKind::Error(..) => {
            // `Closure` types are assumed to never appear in a public API of a crate (only
            // function-body-local variables/values should be able to have a closure type).
            //
            // `FnDef` is assumed to never appear in a public API of a crate - this seems to
            // be an internal, compiler-only type similar to `Closure` (e.g.
            // based on the statement from https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/ty/enum.TyKind.html#variant.FnDef
            // that "each function has a unique type"
            //
            // `Infer` and `Error` types should be impossible at the time when Crubit's code
            // runs (after the "analysis" phase of the Rust compiler).
            panic!("Unexpected TyKind: {:?}", ty.kind());
        }
    })
}

/// Formats `ty` for Rust - to be used in `..._cc_api_impl.rs` (e.g. as a type
/// of a parameter in a Rust thunk).  Because `..._cc_api_impl.rs` is a
/// distinct, separate crate, the returned `TokenStream` uses crate-qualified
/// names whenever necessary - for example: `target_crate::SomeStruct` rather
/// than just `SomeStruct`.
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
            if substs.len() != 0 {
                bail!("Generic types are not supported yet (b/259749095)");
            }

            FullyQualifiedName::new(tcx, adt.did()).format_for_rs()
        },
        ty::TyKind::Foreign(..)
        | ty::TyKind::Str
        | ty::TyKind::Array(..)
        | ty::TyKind::Slice(..)
        | ty::TyKind::RawPtr(..)
        | ty::TyKind::Ref(..)
        | ty::TyKind::FnPtr(..)
        | ty::TyKind::Dynamic(..)
        | ty::TyKind::Generator(..)
        | ty::TyKind::GeneratorWitness(..)
        | ty::TyKind::Projection(..)
        | ty::TyKind::Opaque(..)
        | ty::TyKind::Param(..)
        | ty::TyKind::Bound(..)
        | ty::TyKind::Placeholder(..) => {
            bail!("The following Rust type is not supported yet: {ty}")
        }
        ty::TyKind::Closure(..)
        | ty::TyKind::FnDef(..)
        | ty::TyKind::Infer(..)
        | ty::TyKind::Error(..) => {
            // See the comment inside the similar fallback branch in `format_ty_for_cc`.
            panic!("Unexpected TyKind: {:?}", ty.kind());
        }
    })
}

#[derive(Debug, Default)]
struct MixedSnippet {
    cc: CcSnippet,
    rs: TokenStream,
}

impl AddAssign for MixedSnippet {
    fn add_assign(&mut self, rhs: Self) {
        let Self { cc: mut rhs_cc, rs: rhs_rs } = rhs;

        self.cc.includes.append(&mut rhs_cc.includes);
        self.cc.snippet.extend(rhs_cc.snippet);
        self.rs.extend(rhs_rs);
    }
}

impl Add for MixedSnippet {
    type Output = MixedSnippet;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl Sum for MixedSnippet {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Default::default(), Add::add)
    }
}

/// Formats a function with the given `local_def_id`.
///
/// Will panic if `local_def_id`
/// - is invalid
/// - doesn't identify a function,
/// - has generic parameters of any kind - lifetime parameters (see also
///   b/258235219), type parameters, or const parameters.
fn format_fn(tcx: TyCtxt, local_def_id: LocalDefId) -> Result<MixedSnippet> {
    let def_id: DefId = local_def_id.to_def_id(); // Convert LocalDefId to DefId.

    let item_name = tcx.item_name(def_id);
    let mut symbol_name = {
        // Call to `mono` is ok - doc comment requires no generic parameters (although
        // lifetime parameters would have been okay).
        let instance = ty::Instance::mono(tcx, def_id);
        tcx.symbol_name(instance)
    };

    let sig = tcx
        .fn_sig(def_id)
        .no_bound_vars()
        .expect("Doc comment points out there should be no generic parameters");

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

    let doc_comment = {
        let doc_comment = format_doc_comment(tcx, local_def_id);
        if doc_comment.is_empty() {
            quote!{}
        } else {
            quote! { __NEWLINE__ #doc_comment }
        }
    };

    let mut includes = BTreeSet::new();
    let cc_snippet = {
        let ret_type = format_ret_ty_for_cc(tcx, sig.output())
            .context("Error formatting function return type")?
            .into_tokens(&mut includes);
        let fn_name =
            format_cc_ident(item_name.as_str()).context("Error formatting function name")?;
        let arg_names = tcx
            .fn_arg_names(def_id)
            .iter()
            .enumerate()
            .map(|(index, ident)| {
                format_cc_ident(ident.as_str())
                    .unwrap_or_else(|_err| format_cc_ident(&format!("__param_{index}")).unwrap())
            })
            .collect_vec();
        let arg_types = sig
            .inputs()
            .iter()
            .enumerate()
            .map(|(index, ty)| {
                Ok(format_ty_for_cc(tcx, *ty)
                    .with_context(|| format!("Error formatting the type of parameter #{index}"))?
                    .into_tokens(&mut includes))
            })
            .collect::<Result<Vec<_>>>()?;
        if item_name.as_str() == symbol_name.name {
            quote! {
                #doc_comment
                extern "C" #ret_type #fn_name (
                        #( #arg_types #arg_names ),*
                );
            }
        } else {
            let exported_name =
                format_cc_ident(symbol_name.name).context("Error formatting exported name")?;
            let thunk_args = arg_names
                .clone()
                .into_iter()
                .zip(sig.inputs().iter())
                .map(|(arg, &ty)| format_cc_thunk_arg(tcx, ty, arg).into_tokens(&mut includes))
                .collect_vec();
            quote! {
                namespace __crubit_internal {
                    extern "C" #ret_type #exported_name (
                            #( #arg_types #arg_names ),*
                    );
                }
                #doc_comment
                inline #ret_type #fn_name (
                        #( #arg_types #arg_names ),* ) {
                    return __crubit_internal :: #exported_name( #( #thunk_args ),* );
                }
            }
        }
    };

    let rs_snippet = if !needs_thunk {
        quote! {}
    } else {
        let crate_name = make_rs_ident(tcx.crate_name(LOCAL_CRATE).as_str());
        let fn_name = make_rs_ident(item_name.as_str());
        let exported_name = make_rs_ident(symbol_name.name);
        let ret_type = format_ty_for_rs(tcx, sig.output())?;
        let arg_names = tcx
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
        let arg_types = sig
            .inputs()
            .iter()
            .copied()
            .map(|ty| format_ty_for_rs(tcx, ty))
            .collect::<Result<Vec<_>>>()?;
        quote! {
            #[no_mangle]
            extern "C" fn #exported_name( #( #arg_names: #arg_types ),* ) -> #ret_type {
                :: #crate_name :: #fn_name( #( #arg_names ),* )
            }
        }
    };
    Ok(MixedSnippet { cc: CcSnippet { includes, snippet: cc_snippet }, rs: rs_snippet })
}

/// Gets the layout of the algebraic data type (an ADT - a struct, an enum, or a
/// union) represented by `def_id`.
fn get_adt_layout<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Result<Layout<'tcx>> {
    // TODO(b/259749095): Support non-empty set of generic parameters.  (One
    // scenario where the `layout_of` call below returns an error is when it
    // can't compute the layout for generic ADTs with unsubstituted types / with
    // empty ParamEnv.)
    let param_env = ty::ParamEnv::empty();

    let ty = tcx.type_of(def_id);
    let layout = tcx
        .layout_of(param_env.and(ty))
        // Have to use `.map_err` instead of `.with_context`, because `LayoutError` doesn't
        // satisfy the `anyhow::context::ext::StdError` trait bound.
        .map_err(|layout_err| {
            let item_name = tcx.item_name(def_id);
            anyhow!("Error computing the layout of #{item_name}: {layout_err}")
        })?
        .layout;
    Ok(layout)
}

/// Represents bindings for the "core" part of an algebraic data type (an ADT -
/// a struct, an enum, or a union) in a way that supports later injecting the
/// other parts like so:
///
/// ```
/// quote! {
///     #header {
///         #core
///         #other_parts  // (e.g. struct fields)
///     }
/// }
/// ```
struct AdtCoreBindings {
    /// `header` of the C++ declaration of the ADT.
    /// Example: `struct alignas(4) SomeStruct final`
    header: TokenStream,

    /// `core` contains declarations of
    /// - the default constructor
    /// - the copy constructor
    /// - the move constructor
    /// - the copy assignment operator
    /// - the move assignment operator
    /// - the destructor
    core: TokenStream,

    /// Assertions that we want C++ to make about the ADT.
    /// Example: `static_assert`s about the ADT size and alignment.
    cc_assertions: TokenStream,

    /// Assertions that we want Rust to make about the ADT.
    /// Example: `const` evaluations of `assert`s about the ADT size and
    /// alignment.
    rs_assertions: TokenStream,
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
// TODO(b/259724276): Memoize `format_adt_core` calls - either via `salsa`, or
// (if it doesn't work) then falling back to manually authored caching code.
fn format_adt_core(tcx: TyCtxt, def_id: DefId) -> Result<AdtCoreBindings> {
    // TODO(b/259749095): Support non-empty set of generic parameters.
    let param_env = ty::ParamEnv::empty();

    let cc_name = {
        let item_name = tcx.item_name(def_id);
        format_cc_ident(item_name.as_str()).context("Error formatting item name")?
    };

    let ty = tcx.type_of(def_id);
    if ty.needs_drop(tcx, param_env) {
        // TODO(b/258251148): Support custom `Drop` impls.
        bail!("`Drop` trait and \"drop glue\" are not supported yet (b/258251148)");
    }

    let layout = get_adt_layout(tcx, def_id)?;
    let alignment = {
        // Only the ABI-mandated alignment is considered (i.e. `AbiAndPrefAlign::pref`
        // is ignored), because 1) Rust's `std::mem::align_of` returns the
        // ABI-mandated alignment and 2) the generated C++'s `alignas(...)`
        // should specify the minimal/mandatory alignment.
        let alignment = layout.align().abi.bytes();
        Literal::u64_unsuffixed(alignment)
    };
    let size = {
        let size = layout.size().bytes();
        ensure!(size != 0, "Zero-sized types (ZSTs) are not supported (b/258259459)");
        Literal::u64_unsuffixed(size)
    };

    let header = quote! { struct alignas(#alignment) #cc_name final };
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
            #cc_name& operator=(const #cc_name&) = delete;

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
            #cc_name& operator=(#cc_name&&) = default;

            // TODO(b/258251148): Support custom `Drop` impls and drop glue.
            ~#cc_name() = default;
    };
    let cc_assertions = quote! {
        static_assert(
            sizeof(#cc_name) == #size,
            "Verify that struct layout didn't change since this header got generated");
        static_assert(
            alignof(#cc_name) == #alignment,
            "Verify that struct layout didn't change since this header got generated");
    };
    let rs_assertions = {
        let rs_type = format_ty_for_rs(tcx, ty)?;
        quote! {
            const _: () = assert!(::std::mem::size_of::<#rs_type>() == #size);
            const _: () = assert!(::std::mem::align_of::<#rs_type>() == #alignment);
        }
    };
    Ok(AdtCoreBindings { header, core, cc_assertions, rs_assertions })
}

/// Formats the data (e.g. the fields) of an algebraic data type (an ADT - a
/// struct, an enum, or a union).
///
/// This function needs to remain infallible (see the doc comment of
/// `format_adt_core`).
fn format_adt_data(tcx: TyCtxt, def_id: LocalDefId) -> TokenStream {
    let def_id = def_id.to_def_id(); // LocalDefId -> DefId conversion.
    let size = get_adt_layout(tcx, def_id)
        .expect("`format_adt_data` should only be called if `format_adt_core` succeeded")
        .size()
        .bytes();
    let size = Literal::u64_unsuffixed(size);
    quote! {
        private:
            // TODO(b/258233850): Emit individual fields.
            unsigned char opaque_blob_of_bytes[#size];
    }
}

/// Formats an algebraic data type (an ADT - a struct, an enum, or a union)
/// represented by `def_id`.
///
/// Will panic if `def_id`
/// - is invalid
/// - doesn't identify an ADT,
fn format_adt(tcx: TyCtxt, local_def_id: LocalDefId) -> Result<MixedSnippet> {
    let AdtCoreBindings { header, core, cc_assertions, rs_assertions } =
        format_adt_core(tcx, local_def_id.to_def_id())?;

    let includes = BTreeSet::new();
    let data = format_adt_data(tcx, local_def_id);
    let doc_comment = format_doc_comment(tcx, local_def_id);
    let cc_snippet = quote! {
        __NEWLINE__ #doc_comment
        #header {
            #core
            #data
        };
        #cc_assertions
    };

    Ok(MixedSnippet { cc: CcSnippet { includes, snippet: cc_snippet }, rs: rs_assertions })
}

/// Formats the doc comment associated with the item identified by
/// `local_def_id`.
/// If there is no associated doc comment, an empty `TokenStream` is returned.
fn format_doc_comment(tcx: TyCtxt, local_def_id: LocalDefId) -> TokenStream {
    let hir_id = tcx.local_def_id_to_hir_id(local_def_id);
    let doc_comment: String = tcx
        .hir()
        .attrs(hir_id)
        .iter()
        .filter_map(|attr| match &attr.doc_str() {
            None => None,
            Some(symbol) => Some(symbol.as_str().to_owned()),
        })
        .collect_vec()
        .join("\n\n");
    if doc_comment.is_empty() {
        quote! {}
    } else {
        quote! { __COMMENT__ #doc_comment}
    }
}

/// Formats a Rust item idenfied by `def_id`.
///
/// Will panic if `def_id` is invalid (i.e. doesn't identify a Rust node or
/// item).
fn format_def(tcx: TyCtxt, def_id: LocalDefId) -> Result<MixedSnippet> {
    match tcx.hir().get_by_def_id(def_id) {
        Node::Item(item) => match item {
            Item { kind: ItemKind::Fn(_, generics, _) |
                         ItemKind::Struct(_, generics) |
                         ItemKind::Enum(_, generics) |
                         ItemKind::Union(_, generics),
                   .. } if !generics.params.is_empty() => {
                // TODO(b/258235219): Supporting function parameter types (or return types) that
                // are references requires adding support for generic lifetime parameters.  The
                // required changes may cascade into `format_fn`'s usage of `no_bound_vars`.
                bail!("Generics are not supported yet (b/259749023 and b/259749095)");
            },
            Item { kind: ItemKind::Fn(..), .. } => format_fn(tcx, def_id),
            Item { kind: ItemKind::Struct(..) | ItemKind::Enum(..) | ItemKind::Union(..), .. } =>
                format_adt(tcx, def_id),
            Item { kind, .. } => bail!("Unsupported rustc_hir::hir::ItemKind: {}", kind.descr()),
        },
        _unsupported_node => bail!("Unsupported rustc_hir::hir::Node"),
    }
}

/// Formats a C++ comment explaining why no bindings have been generated for
/// `local_def_id`.
fn format_unsupported_def(
    tcx: TyCtxt,
    local_def_id: LocalDefId,
    err: anyhow::Error,
) -> MixedSnippet {
    let span = tcx.sess().source_map().span_to_embeddable_string(tcx.def_span(local_def_id));
    let name = tcx.def_path_str(local_def_id.to_def_id());

    // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
    // says: To print causes as well [...], use the alternate selector “{:#}”.
    let msg = format!("Error generating bindings for `{name}` defined at {span}: {err:#}");
    let comment = quote! { __NEWLINE__ __NEWLINE__ __COMMENT__ #msg __NEWLINE__ };

    MixedSnippet { cc: CcSnippet { snippet: comment, ..Default::default() }, ..Default::default() }
}

/// Formats all public items from the Rust crate being compiled.
fn format_crate(tcx: TyCtxt) -> Result<GeneratedBindings> {
    let MixedSnippet{ cc, rs } = tcx
        .hir()
        .items()
        .filter_map(|item_id| {
            let def_id: LocalDefId = item_id.owner_id.def_id;
            if !tcx.local_visibility(def_id).is_public() {
                None
            } else {
                Some(
                    format_def(tcx, def_id)
                        .unwrap_or_else(|err| format_unsupported_def(tcx, def_id, err)),
                )
            }
        })
        .sum();

    let h_body = {
        // TODO(b/254690602): Decide whether using `#crate_name` as the name of the
        // top-level namespace is okay (e.g. investigate if this name is globally
        // unique + ergonomic).
        let crate_name = format_cc_ident(tcx.crate_name(LOCAL_CRATE).as_str())?;

        let CcSnippet { includes, snippet } = cc;
        let includes = format_cc_includes(&includes);
        quote! {
            #includes __NEWLINE__
            namespace #crate_name {
                #snippet
            }
        }
    };
    Ok(GeneratedBindings { h_body, rs_body: rs })
}

#[cfg(test)]
pub mod tests {
    use super::{
        format_cc_thunk_arg, format_def, format_ret_ty_for_cc, format_ty_for_cc, format_ty_for_rs,
        GeneratedBindings, MixedSnippet,
    };

    use anyhow::Result;
    use code_gen_utils::{format_cc_ident, format_cc_includes};
    use itertools::Itertools;
    use proc_macro2::TokenStream;
    use quote::quote;
    use rustc_middle::ty::{Ty, TyCtxt};
    use rustc_span::def_id::LocalDefId;
    use std::path::PathBuf;

    use token_stream_matchers::{
        assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
    };

    pub fn get_sysroot_for_testing() -> PathBuf {
        let runfiles = runfiles::Runfiles::create().unwrap();
        runfiles.rlocation(if std::env::var("LEGACY_TOOLCHAIN_RUST_TEST").is_ok() {
            "google3/third_party/unsupported_toolchains/rust/toolchains/nightly"
        } else {
            "google3/nowhere/llvm/rust"
        })
    }

    #[test]
    #[should_panic(expected = "Test inputs shouldn't cause compilation errors")]
    fn test_infra_panic_when_test_input_contains_syntax_errors() {
        run_compiler("syntax error here", |_tcx| panic!("This part shouldn't execute"))
    }

    #[test]
    #[should_panic(expected = "Test inputs shouldn't cause compilation errors")]
    fn test_infra_panic_when_test_input_triggers_analysis_errors() {
        run_compiler("#![feature(no_such_feature)]", |_tcx| panic!("This part shouldn't execute"))
    }

    #[test]
    #[should_panic(expected = "Test inputs shouldn't cause compilation errors")]
    fn test_infra_panic_when_test_input_triggers_warnings() {
        run_compiler("pub fn foo(unused_parameter: i32) {}", |_tcx| {
            panic!("This part shouldn't execute")
        })
    }

    #[test]
    fn test_infra_nightly_features_ok_in_test_input() {
        // This test arbitrarily picks `yeet_expr` as an example of a feature that
        // hasn't yet been stabilized.
        let test_src = r#"
                // This test is supposed to test that *nightly* features are ok
                // in the test input.  The `forbid` directive below helps to
                // ensure that we'll realize in the future when the `yeet_expr`
                // feature gets stabilized, making it not quite fitting for use
                // in this test.
                #![forbid(stable_features)]

                #![feature(yeet_expr)]
            "#;
        run_compiler(test_src, |_tcx| ())
    }

    #[test]
    fn test_infra_stabilized_features_ok_in_test_input() {
        // This test arbitrarily picks `const_ptr_offset_from` as an example of a
        // feature that has been already stabilized.
        run_compiler("#![feature(const_ptr_offset_from)]", |_tcx| ())
    }

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
        run_compiler(test_src, |tcx| find_def_id_by_name(tcx, "missing_name"));
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
        run_compiler(test_src, |tcx| find_def_id_by_name(tcx, "some_name"));
    }

    #[test]
    fn test_generated_bindings_fn_no_mangle_extern_c() {
        // This test covers only a single example of a function that should get a C++
        // binding. Additional coverage of how items are formatted is provided by
        // `test_format_def_..._fn_...` tests.
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn public_function() {
                    println!("foo");
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.expect("Test expects success");
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    extern "C" void public_function();
                }
            );
            // TODO(b/254097223): Verify Rust thunks here (once they actually get generated).
        });
    }

    #[test]
    fn test_generated_bindings_fn_export_name() {
        // Coverage of how `MixedSnippet::cc` are propagated when there is no
        // `MixedSnippet::rs` (e.g. no Rust thunks are needed).
        let test_src = r#"
                #[export_name = "export_name"]
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.expect("Test expects success");
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
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

    #[test]
    fn test_generated_bindings_struct() {
        // This test covers only a single example of an ADT (struct/enum/union) that
        // should get a C++ binding. Additional coverage of how items are
        // formatted is provided by `test_format_def_..._struct_...`,
        // `test_format_def_..._enum_...`, and `test_format_def_..._union_...`
        // tests.
        //
        // We don't want to duplicate coverage already provided by
        // `test_format_def_struct_with_fields`, but we do want to verify that
        // * `format_crate` will actually find and process the struct
        //   (`test_format_def_...` doesn't cover this aspect - it uses a test-only
        //   `find_def_id_by_name` instead)
        // * The overall shape of the bindings is present.
        let test_src = r#"
                pub struct Point {
                    pub x: i32,
                    pub y: i32,
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.expect("Test expects success");
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    namespace rust_out {
                        struct alignas(4) Point final {
                            // No point replicating test coverage of
                            // `test_format_def_struct_with_fields`.
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
            let bindings = bindings.expect("Test expects success");
            assert_cc_matches!(
                bindings.h_body,
                quote! {
                    __HASH_TOKEN__ include <cstdint> ...
                    namespace ... {
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
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.expect("Test expects success");

            // Non-public items should not be present in the generated bindings.
            assert_cc_not_matches!(bindings.h_body, quote! { private_function });
            assert_rs_not_matches!(bindings.rs_body, quote! { private_function });
            assert_cc_not_matches!(bindings.h_body, quote! { PrivateStruct });
            assert_rs_not_matches!(bindings.rs_body, quote! { PrivateStruct });
        });
    }

    #[test]
    fn test_generated_bindings_top_level_items() {
        let test_src = "pub fn public_function() {}";
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.expect("Test expects success");
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

    #[test]
    fn test_generated_bindings_unsupported_item() {
        // This test verifies how `Err` from `format_def` is formatted as a C++ comment
        // (in `format_crate` and `format_unsupported_def`).
        // - This test covers only a single example of an unsupported item.  Additional
        //   coverage is provided by `test_format_def_unsupported_...` tests.
        // - This test somewhat arbitrarily chooses an example of an unsupported item,
        //   trying to pick one that 1) will never be supported (b/254104998 has some extra
        //   notes about APIs named after reserved C++ keywords) and 2) tests that the
        //   full error chain is included in the message.
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn reinterpret_cast() {}
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.expect("Test expects success");
            let expected_comment_txt = "Error generating bindings for `reinterpret_cast` \
                 defined at <crubit_unittests.rs>:3:17: 3:53: \
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
    fn test_format_def_fn_extern_c_no_mangle_no_params_no_return_type() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn public_function() {}
            "#;
        test_format_def(test_src, "public_function", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    extern "C" void public_function();
                }
            );
        });
    }

    #[test]
    fn test_format_def_fn_explicit_unit_return_type() {
        // This test is very similar to the
        // `test_format_def_fn_extern_c_no_mangle_no_params_no_return_type` above, except that the
        // return type is explicitly spelled out.  There is no difference in `ty::FnSig` so our
        // code behaves exactly the same, but the test has been planned based on earlier,
        // hir-focused approach and having this extra test coverage shouldn't hurt. (`hir::FnSig`
        // and `hir::FnRetTy` _do_ see a difference between the two tests).
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn explicit_unit_return_type() -> () {}
            "#;
        test_format_def(test_src, "explicit_unit_return_type", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    extern "C" void explicit_unit_return_type();
                }
            );
        });
    }

    #[test]
    fn test_format_def_fn_never_return_type() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn never_returning_function() -> ! {
                    panic!("This function panics and therefore never returns");
                }
            "#;
        test_format_def(test_src, "never_returning_function", |result| {
            // TODO(b/254507801): The function should be annotated with the `[[noreturn]]`
            // attribute.
            // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
            // details).
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    extern "C" void never_returning_function();
                }
            );
        })
    }

    #[test]
    fn test_format_def_fn_mangling() {
        // This test checks that bindings can be generated for `extern "C"` functions
        // that do *not* have `#[no_mangle]` attribute.  The test elides away
        // the mangled name in the `assert_cc_matches` checks below, but
        // end-to-end test coverage is provided by `test/functions`.
        let test_src = r#"
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_format_def(test_src, "public_function", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double x, double y);
                    }
                    inline double public_function(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_def_fn_export_name() {
        let test_src = r#"
                #[export_name = "export_name"]
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_format_def(test_src, "public_function", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double export_name(double x, double y);
                    }
                    inline double public_function(double x, double y) {
                        return __crubit_internal::export_name(x, y);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_def_unsupported_fn_unsafe() {
        // This tests how bindings for an `unsafe fn` are generated.
        let test_src = r#"
                #[no_mangle]
                pub unsafe extern "C" fn foo() {}
            "#;
        test_format_def(test_src, "foo", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(
                err,
                "Bindings for `unsafe` functions \
                             are not fully designed yet (b/254095482)"
            );
        });
    }

    #[test]
    fn test_format_def_fn_const() {
        // This tests how bindings for an `const fn` are generated.
        //
        // Right now the `const` qualifier is ignored, but one can imagine that in the
        // (very) long-term future such functions (including their bodies) could
        // be translated into C++ `consteval` functions.
        let test_src = r#"
                pub const fn foo(i: i32) -> i32 { i * 42 }
            "#;
        test_format_def(test_src, "foo", |result| {
            // TODO(b/254095787): Update test expectations below once `const fn` from Rust
            // is translated into a `consteval` C++ function.
            let result = result.expect("Test expects success here");
            assert!(!result.cc.includes.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...( std::int32_t i);
                    }
                    inline std::int32_t foo(std::int32_t i) {
                        return __crubit_internal::...(i);
                    }
                }
            );
            assert_rs_matches!(
                result.rs,
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
    fn test_format_def_fn_with_c_unwind_abi() {
        // See also https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
        let test_src = r#"
                #![feature(c_unwind)]

                #[no_mangle]
                pub extern "C-unwind" fn may_throw() {}
            "#;
        test_format_def(test_src, "may_throw", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    extern "C" void may_throw();
                }
            );
        });
    }

    #[test]
    fn test_format_def_fn_with_type_aliased_return_type() {
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
        test_format_def(test_src, "type_aliased_return", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    extern "C" double type_aliased_return();
                }
            );
        });
    }

    #[test]
    fn test_format_def_fn_with_doc_comment_with_unmangled_name() {
        let test_src = r#"
            /// Outer line doc.
            /** Outer block doc that spans lines.
             */
            #[doc = "Doc comment via doc attribute."]
            #[no_mangle]
            pub extern "C" fn fn_with_doc_comment_with_unmangled_name() {}
          "#;
        test_format_def(test_src, "fn_with_doc_comment_with_unmangled_name", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            let doc_comments = [
                " Outer line doc.",
                "",
                " Outer block doc that spans lines.",
                "             ",
                "",
                "Doc comment via doc attribute.",
            ]
            .join("\n");
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    __COMMENT__ #doc_comments
                    extern "C" void fn_with_doc_comment_with_unmangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_def_fn_with_inner_doc_comment_with_unmangled_name() {
        let test_src = r#"
            /// Outer doc comment.
            #[no_mangle]
            pub extern "C" fn fn_with_inner_doc_comment_with_unmangled_name() {
                //! Inner doc comment.
            }
          "#;
        test_format_def(test_src, "fn_with_inner_doc_comment_with_unmangled_name", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            let doc_comments = [" Outer doc comment.", " Inner doc comment."].join("\n\n");
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    __COMMENT__ #doc_comments
                    extern "C" void fn_with_inner_doc_comment_with_unmangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_def_fn_with_doc_comment_with_mangled_name() {
        let test_src = r#"
                /// Doc comment of a function with mangled name.
                pub extern "C" fn fn_with_doc_comment_with_mangled_name() {}
            "#;
        test_format_def(test_src, "fn_with_doc_comment_with_mangled_name", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            let comment = " Doc comment of a function with mangled name.";
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void ...();
                    }
                    __COMMENT__ #comment
                    inline void fn_with_doc_comment_with_mangled_name() {
                        return __crubit_internal::...();
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_def_unsupported_fn_name_is_reserved_cpp_keyword() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn reinterpret_cast() -> () {}
            "#;
        test_format_def(test_src, "reinterpret_cast", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(
                err,
                "Error formatting function name: \
                       `reinterpret_cast` is a C++ reserved keyword \
                       and can't be used as a C++ identifier"
            );
        });
    }

    #[test]
    fn test_format_def_unsupported_fn_ret_type() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn foo() -> *const i32 { std::ptr::null() }
            "#;
        test_format_def(test_src, "foo", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(
                err,
                "Error formatting function return type: \
                       The following Rust type is not supported yet: *const i32"
            );
        });
    }

    #[test]
    fn test_format_def_unsupported_fn_with_late_bound_lifetimes() {
        // TODO(b/258235219): Expect success after adding support for references.
        let test_src = r#"
                pub fn foo(arg: &i32) -> &i32 { arg }

                // Lifetime inference translates the above into:
                //     pub fn foo<'a>(arg: &'a i32) -> &'a i32 { ... }
                // leaving 'a lifetime late-bound (it is bound with a lifetime
                // taken from each of the callsites).  In other words, we can't
                // just call `no_bound_vars` on this `FnSig`'s `Binder`.
            "#;
        test_format_def(test_src, "foo", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Generics are not supported yet (b/259749023 and b/259749095)");
        });
    }

    #[test]
    fn test_format_def_unsupported_generic_fn() {
        let test_src = r#"
                use std::default::Default;
                use std::fmt::Display;
                pub fn generic_function<T: Default + Display>() {
                    println!("{}", T::default());
                }
            "#;
        test_format_def(test_src, "generic_function", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Generics are not supported yet (b/259749023 and b/259749095)");
        });
    }

    #[test]
    fn test_format_def_unsupported_generic_struct() {
        let test_src = r#"
                pub struct Point<T> {
                    pub x: T,
                    pub y: T,
                }
            "#;
        test_format_def(test_src, "Point", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Generics are not supported yet (b/259749023 and b/259749095)");
        });
    }

    #[test]
    fn test_format_def_unsupported_generic_enum() {
        let test_src = r#"
                pub enum Point<T> {
                    Cartesian{x: T, y: T},
                    Polar{angle: T, dist: T},
                }
            "#;
        test_format_def(test_src, "Point", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Generics are not supported yet (b/259749023 and b/259749095)");
        });
    }

    #[test]
    fn test_format_def_unsupported_generic_union() {
        let test_src = r#"
                pub union SomeUnion<T> {
                    pub x: std::mem::ManuallyDrop<T>,
                    pub y: i32,
                }
            "#;
        test_format_def(test_src, "SomeUnion", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Generics are not supported yet (b/259749023 and b/259749095)");
        });
    }

    #[test]
    fn test_format_def_unsupported_fn_async() {
        let test_src = r#"
                pub async fn async_function() {}
            "#;
        test_format_def(test_src, "async_function", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Error formatting function return type: \
                             The following Rust type is not supported yet: \
                             impl std::future::Future<Output = ()>");
        });
    }

    #[test]
    fn test_format_def_fn_rust_abi() {
        let test_src = r#"
                pub fn add(x: f64, y: f64) -> f64 { x * y }
            "#;
        test_format_def(test_src, "add", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double x, double y);
                    }
                    inline double add(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                result.rs,
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
    fn test_format_def_fn_vectorcall_abi() {
        // This tests a function call that is not a C-ABI, and is not the default Rust
        // ABI.  It can't use "stdcall", because it is not supported on the
        // targets where Crubit's tests run.  So, it ended up using
        // "vectorcall".
        //
        // This test almost entirely replicates `test_format_def_fn_rust_abi`, except
        // for the `extern "vectorcall"` part in the `test_src` test input.
        //
        // This test verifies the current behavior that gives reasonable and functional
        // FFI bindings.  OTOH, in the future we may decide to avoid having the
        // extra thunk for cases where the given non-C-ABI function call
        // convention is supported by both C++ and Rust
        // (see also `format_cc_call_conv_as_clang_attribute` in
        // `rs_bindings_from_cc/src_code_gen.rs`)
        let test_src = r#"
                #![feature(abi_vectorcall)]
                pub extern "vectorcall" fn add(x: f64, y: f64) -> f64 { x * y }
            "#;
        test_format_def(test_src, "add", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double x, double y);
                    }
                    inline double add(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                result.rs,
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
    fn test_format_def_unsupported_fn_variadic() {
        let test_src = r#"
                #![feature(c_variadic)]

                #[no_mangle]
                pub unsafe extern "C" fn variadic_function(_fmt: *const u8, ...) {}
            "#;
        test_format_def(test_src, "variadic_function", |result| {
            // TODO(b/254097223): Add support for variadic functions.
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "C variadic functions are not supported (b/254097223)");
        });
    }

    #[test]
    fn test_format_def_fn_params() {
        let test_src = r#"
                #[allow(unused_variables)]
                #[no_mangle]
                pub extern "C" fn foo(b: bool, f: f64) {}
            "#;
        test_format_def(test_src, "foo", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    extern "C" void foo(bool b, double f);
                }
            );
        });
    }

    #[test]
    fn test_format_def_fn_param_name_reserved_keyword() {
        let test_src = r#"
                #[allow(unused_variables)]
                #[no_mangle]
                pub extern "C" fn some_function(reinterpret_cast: f64) {}
            "#;
        test_format_def(test_src, "some_function", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert!(result.rs.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    extern "C" void some_function(double __param_0);
                }
            );
        });
    }

    #[test]
    fn test_format_def_fn_with_multiple_anonymous_parameter_names() {
        let test_src = r#"
                pub fn foo(_: f64, _: f64) {}
            "#;
        test_format_def(test_src, "foo", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void ...(
                            double __param_0, double __param_1);
                    }
                    inline void foo(double __param_0, double __param_1) {
                        return __crubit_internal::...(__param_0, __param_1);
                    }
                }
            );
            assert_rs_matches!(
                result.rs,
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
    fn test_format_def_fn_with_destructuring_parameter_name() {
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
        test_format_def(test_src, "func", |result| {
            let result = result.expect("Test expects success here");
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...(::rust_out::S __param_0);
                    }
                    inline std::int32_t func(::rust_out::S __param_0) {
                        return __crubit_internal::...(std::move(__param_0));
                    }
                }
            );
            assert_rs_matches!(
                result.rs,
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
    fn test_format_def_unsupported_fn_param_type() {
        let test_src = r#"
                #[no_mangle]
                pub extern "C" fn fn_with_params(_param: *const i32) {}
            "#;
        test_format_def(test_src, "fn_with_params", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Error formatting the type of parameter #0: \
                             The following Rust type is not supported yet: \
                             *const i32");
        });
    }

    #[test]
    fn test_format_def_unsupported_fn_param_type_unit() {
        let test_src = r#"
                #[no_mangle]
                pub fn fn_with_params(_param: ()) {}
            "#;
        test_format_def(test_src, "fn_with_params", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Error formatting the type of parameter #0: \
                             The unit type `()` / `void` is only supported as a return type");
        });
    }

    #[test]
    fn test_format_def_unsupported_fn_param_type_never() {
        let test_src = r#"
                #![feature(never_type)]

                #[no_mangle]
                pub extern "C" fn fn_with_params(_param: !) {}
            "#;
        test_format_def(test_src, "fn_with_params", |result| {
            let err = result.expect_err("Test expects an error here");
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
    fn test_format_def_struct_with_fields() {
        let test_src = r#"
                pub struct SomeStruct {
                    pub x: i32,
                    pub y: i32,
                }

                const _: () = assert!(std::mem::size_of::<SomeStruct>() == 8);
                const _: () = assert!(std::mem::align_of::<SomeStruct>() == 4);
            "#;
        test_format_def(test_src, "SomeStruct", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    struct alignas(4) SomeStruct final {
                        public:
                            // In this test there is no `Default` implementation.
                            SomeStruct() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            SomeStruct(const SomeStruct&) = delete;
                            SomeStruct& operator=(const SomeStruct&) = delete;

                            // All Rust types are trivially-movable.
                            SomeStruct(SomeStruct&&) = default;
                            SomeStruct& operator=(SomeStruct&&) = default;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~SomeStruct() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[8];
                    };
                    static_assert(sizeof(SomeStruct) == 8, ...);
                    static_assert(alignof(SomeStruct) == 4, ...);
                }
            );
            assert_rs_matches!(
                result.rs,
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
    fn test_format_def_struct_with_tuple() {
        let test_src = r#"
                pub struct TupleStruct(i32, i32);
                const _: () = assert!(std::mem::size_of::<TupleStruct>() == 8);
                const _: () = assert!(std::mem::align_of::<TupleStruct>() == 4);
            "#;
        test_format_def(test_src, "TupleStruct", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    struct alignas(4) TupleStruct final {
                        public:
                            // In this test there is no `Default` implementation.
                            TupleStruct() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            TupleStruct(const TupleStruct&) = delete;
                            TupleStruct& operator=(const TupleStruct&) = delete;

                            // All Rust types are trivially-movable.
                            TupleStruct(TupleStruct&&) = default;
                            TupleStruct& operator=(TupleStruct&&) = default;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~TupleStruct() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[8];
                    };
                    static_assert(sizeof(TupleStruct) == 8, ...);
                    static_assert(alignof(TupleStruct) == 4, ...);
                }
            );
            assert_rs_matches!(
                result.rs,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::TupleStruct>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::TupleStruct>() == 4);
                }
            );
        });
    }

    #[test]
    fn test_format_def_unsupported_struct_with_name_that_is_reserved_keyword() {
        let test_src = r#"
                #[allow(non_camel_case_types)]
                pub struct reinterpret_cast {
                    pub x: i32,
                    pub y: i32,
                }
            "#;
        test_format_def(test_src, "reinterpret_cast", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(
                err,
                "Error formatting item name: \
                             `reinterpret_cast` is a C++ reserved keyword \
                             and can't be used as a C++ identifier"
            );
        });
    }

    #[test]
    fn test_format_def_unsupported_struct_with_custom_drop_impl() {
        let test_src = r#"
                pub struct StructWithCustomDropImpl {
                    pub x: i32,
                    pub y: i32,
                }

                impl Drop for StructWithCustomDropImpl {
                    fn drop(&mut self) {}
                }
            "#;
        test_format_def(test_src, "StructWithCustomDropImpl", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "`Drop` trait and \"drop glue\" are not supported yet (b/258251148)");
        });
    }

    #[test]
    fn test_format_def_unsupported_struct_with_custom_drop_glue() {
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
        test_format_def(test_src, "StructRequiringCustomDropGlue", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "`Drop` trait and \"drop glue\" are not supported yet (b/258251148)");
        });
    }

    // This test covers how ZSTs (zero-sized-types) are handled.
    // https://doc.rust-lang.org/reference/items/structs.html refers to this kind of struct as a
    // "unit-like struct".
    #[test]
    fn test_format_def_unsupported_struct_zero_sized_type() {
        let test_src = r#"
                pub struct ZeroSizedType1;
                pub struct ZeroSizedType2();
                pub struct ZeroSizedType3{}
            "#;
        for name in ["ZeroSizedType1", "ZeroSizedType2", "ZeroSizedType3"] {
            test_format_def(test_src, name, |result| {
                let err = result.expect_err("Test expects an error here");
                assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)");
            });
        }
    }

    /// This is a test for an enum that only has `EnumItemDiscriminant` items
    /// (and doesn't have `EnumItemTuple` or `EnumItemStruct` items).  See
    /// also https://doc.rust-lang.org/reference/items/enumerations.html
    #[test]
    fn test_format_def_enum_with_only_discriminant_items() {
        let test_src = r#"
                pub enum SomeEnum {
                    Red,
                    Green = 123,
                    Blue,
                }

                const _: () = assert!(std::mem::size_of::<SomeEnum>() == 1);
                const _: () = assert!(std::mem::align_of::<SomeEnum>() == 1);
            "#;
        test_format_def(test_src, "SomeEnum", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    struct alignas(1) SomeEnum final {
                        public:
                            // In this test there is no `Default` implementation.
                            SomeEnum() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            SomeEnum(const SomeEnum&) = delete;
                            SomeEnum& operator=(const SomeEnum&) = delete;

                            // All Rust types are trivially-movable.
                            SomeEnum(SomeEnum&&) = default;
                            SomeEnum& operator=(SomeEnum&&) = default;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~SomeEnum() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[1];
                    };
                    static_assert(sizeof(SomeEnum) == 1, ...);
                    static_assert(alignof(SomeEnum) == 1, ...);
                }
            );
            assert_rs_matches!(
                result.rs,
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
    fn test_format_def_enum_with_tuple_and_struct_items() {
        let test_src = r#"
                pub enum Point {
                    Cartesian(f32, f32),
                    Polar{ dist: f32, angle: f32 },
                }

                const _: () = assert!(std::mem::size_of::<Point>() == 12);
                const _: () = assert!(std::mem::align_of::<Point>() == 4);
            "#;
        test_format_def(test_src, "Point", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    struct alignas(4) Point final {
                        public:
                            // In this test there is no `Default` implementation.
                            Point() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            Point(const Point&) = delete;
                            Point& operator=(const Point&) = delete;

                            // All Rust types are trivially-movable.
                            Point(Point&&) = default;
                            Point& operator=(Point&&) = default;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~Point() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[12];
                    };
                    static_assert(sizeof(Point) == 12, ...);
                    static_assert(alignof(Point) == 4, ...);
                }
            );
            assert_rs_matches!(
                result.rs,
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
    fn test_format_def_unsupported_enum_zero_variants() {
        let test_src = r#"
                pub enum ZeroVariantEnum {}
            "#;
        test_format_def(test_src, "ZeroVariantEnum", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)");
        });
    }

    /// This is a test for a `union`.  See also
    /// https://doc.rust-lang.org/reference/items/unions.html
    #[test]
    fn test_format_def_union() {
        let test_src = r#"
                pub union SomeUnion {
                    pub i: i32,
                    pub f: f64,
                }

                const _: () = assert!(std::mem::size_of::<SomeUnion>() == 8);
                const _: () = assert!(std::mem::align_of::<SomeUnion>() == 8);
            "#;
        test_format_def(test_src, "SomeUnion", |result| {
            let result = result.expect("Test expects success here");
            assert!(result.cc.includes.is_empty());
            assert_cc_matches!(
                result.cc.snippet,
                quote! {
                    struct alignas(8) SomeUnion final {
                        public:
                            // In this test there is no `Default` implementation.
                            SomeUnion() = delete;

                            // In this test there is no `Copy` implementation / derive.
                            SomeUnion(const SomeUnion&) = delete;
                            SomeUnion& operator=(const SomeUnion&) = delete;

                            // All Rust types are trivially-movable.
                            SomeUnion(SomeUnion&&) = default;
                            SomeUnion& operator=(SomeUnion&&) = default;

                            // In this test there is no custom `Drop`, so C++ can also
                            // just use the `default` destructor.
                            ~SomeUnion() = default;
                        private:
                            unsigned char opaque_blob_of_bytes[8];
                    };
                    static_assert(sizeof(SomeUnion) == 8, ...);
                    static_assert(alignof(SomeUnion) == 8, ...);
                }
            );
            assert_rs_matches!(
                result.rs,
                quote! {
                    const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 8);
                    const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 8);
                }
            );
        });
    }

    #[test]
    fn test_format_def_doc_comments_union() {
        let test_src = r#"
            /// Doc for some union.
            pub union SomeUnionWithDocs {
                /// Doc for a field in a union.
                pub i: i32,
                pub f: f64
            }
        "#;
        test_format_def(test_src, "SomeUnionWithDocs", |result| {
            let result = result.expect("Test expects success here");
            let comment = " Doc for some union.";
            assert_cc_matches!(
                result.cc.snippet,
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
    fn test_format_def_doc_comments_enum() {
        let test_src = r#"
            /** Doc for some enum. */
            pub enum SomeEnumWithDocs {
                Kind1(i32),
            }
        "#;
        test_format_def(test_src, "SomeEnumWithDocs", |result| {
            let result = result.expect("Test expects success here");
            let comment = " Doc for some enum. ";
            assert_cc_matches!(
                result.cc.snippet,
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
    fn test_format_def_doc_comments_struct() {
        let test_src = r#"
            #![allow(dead_code)]
            #[doc = "Doc for some struct."]
            pub struct SomeStructWithDocs {
                some_field : i32,
            }
        "#;
        test_format_def(test_src, "SomeStructWithDocs", |result| {
            let result = result.expect("Test expects success here");
            let comment = "Doc for some struct.";
            assert_cc_matches!(
                result.cc.snippet,
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
    fn test_format_def_doc_comments_tuple_struct() {
        let test_src = r#"
            /// Doc for some tuple struct.
            pub struct SomeTupleStructWithDocs(i32);
        "#;
        test_format_def(test_src, "SomeTupleStructWithDocs", |result| {
            let result = result.expect("Test expects success here");
            let comment = " Doc for some tuple struct.";
            assert_cc_matches!(
                result.cc.snippet,
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
    fn test_format_def_unsupported_hir_item_kind() {
        let test_src = r#"
                #[no_mangle]
                pub static STATIC_VALUE: i32 = 42;
            "#;
        test_format_def(test_src, "STATIC_VALUE", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Unsupported rustc_hir::hir::ItemKind: static item");
        });
    }

    #[test]
    fn test_format_ret_ty_for_cc_successes() {
        // Test coverage for cases where `format_ret_ty_for_cc` returns an `Ok(...)`.
        // Additional testcases are covered by `test_format_ty_for_cc_successes`
        // (because `format_ret_ty_for_cc` delegates most cases to `format_ty_for_cc`).
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
                let cc_snippet = format_ret_ty_for_cc(tcx, ty).unwrap();
                assert!(cc_snippet.includes.is_empty());
                cc_snippet.snippet.to_string()
            };
            let expected = expected.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual, expected, "{desc}");
        });
    }

    #[test]
    fn test_format_ty_for_cc_successes() {
        // Test coverage for cases where `format_ty_for_cc` returns an `Ok(...)`.
        //
        // Using `std::int8_t` (instead of `::std::int8_t`) has been an explicit decision.  The
        // "Google C++ Style Guide" suggests to "avoid nested namespaces that match well-known
        // top-level namespaces" and "in particular, [...] not create any nested std namespaces.".
        // It seems desirable if the generated bindings conform to this aspect of the style guide,
        // because it makes things easier for *users* of these bindings.
        let testcases = [
            // ( <Rust type>, (<expected C++ type>, <expected #include>) )
            ("bool", ("bool", "")),
            ("f32", ("float", "")),
            ("f64", ("double", "")),
            ("i8", ("std::int8_t", "cstdint")),
            ("i16", ("std::int16_t", "cstdint")),
            ("i32", ("std::int32_t", "cstdint")),
            ("i64", ("std::int64_t", "cstdint")),
            ("isize", ("std::intptr_t", "cstdint")),
            ("u8", ("std::uint8_t", "cstdint")),
            ("u16", ("std::uint16_t", "cstdint")),
            ("u32", ("std::uint32_t", "cstdint")),
            ("u64", ("std::uint64_t", "cstdint")),
            ("usize", ("std::uintptr_t", "cstdint")),
            ("char", ("std::uint32_t", "cstdint")),
            ("SomeStruct", ("::rust_out::SomeStruct", "")),
            ("SomeEnum", ("::rust_out::SomeEnum", "")),
            ("SomeUnion", ("::rust_out::SomeUnion", "")),
            // Extra parens/sugar are expected to be ignored:
            ("(bool)", ("bool", "")),
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
        test_ty(&testcases, preamble, |desc, tcx, ty, (expected_snippet, expected_include)| {
            let (actual_snippet, actual_includes) = {
                let cc_snippet = format_ty_for_cc(tcx, ty).unwrap();
                (cc_snippet.snippet.to_string(), cc_snippet.includes)
            };

            let expected_snippet = expected_snippet.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual_snippet, expected_snippet, "{desc}");

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

    #[test]
    fn test_format_ty_for_cc_failures() {
        // This test provides coverage for cases where `format_ty_for_cc` returns an
        // `Err(...)`.
        //
        // TODO(lukasza): Add test coverage (here and in the "for_rs" flavours) for:
        // - TyKind::Bound
        // - TyKind::Dynamic (`dyn Eq`)
        // - TyKind::Foreign (`extern type T`)
        // - https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html:
        //   TyKind::Generator, TyKind::GeneratorWitness
        // - TyKind::Param
        // - TyKind::Placeholder
        // - TyKind::Projection
        //
        // It seems okay to have no test coverage for now for the following types (which
        // should never be encountered when generating bindings and where
        // `format_ty_for_cc` should panic):
        // - TyKind::Closure
        // - TyKind::Error
        // - TyKind::FnDef
        // - TyKind::Infer */
        let testcases = [
            // ( <Rust type>, <expected error message> )
            (
                "()", // Empty TyKind::Tuple
                "The unit type `()` / `void` is only supported as a return type"
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
                "*const i32", // TyKind::Ptr
                "The following Rust type is not supported yet: *const i32",
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
                "impl Eq", // TyKind::Opaque
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
        test_ty(&testcases, preamble, |desc, tcx, ty, expected_err| {
            let anyhow_err = format_ty_for_cc(tcx, ty).unwrap_err();
            let actual_err = format!("{anyhow_err:#}");
            assert_eq!(&actual_err, *expected_err, "{desc}");
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
            ("SomeStruct", "::rust_out::SomeStruct"),
            ("SomeEnum", "::rust_out::SomeEnum"),
            ("SomeUnion", "::rust_out::SomeUnion"),
            ("std::cmp::Ordering", "::core::cmp::Ordering"),
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
        test_ty(&testcases, preamble, |desc, tcx, ty, expected_snippet| {
            let actual_snippet = format_ty_for_rs(tcx, ty).unwrap().to_string();
            let expected_snippet = expected_snippet.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual_snippet, expected_snippet, "{desc}");
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
                "*const i32", // TyKind::Ptr
                "The following Rust type is not supported yet: *const i32",
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
                "impl Eq", // TyKind::Opaque
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
        test_ty(&testcases, preamble, |desc, tcx, ty, (expected_snippet, expected_include)| {
            let (actual_snippet, actual_includes) = {
                let cc_snippet = format_cc_thunk_arg(tcx, ty, quote! { value });
                (cc_snippet.snippet.to_string(), cc_snippet.includes)
            };

            let expected_snippet = expected_snippet.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual_snippet, expected_snippet, "{desc}");

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
            run_compiler(input, |tcx| {
                let def_id = find_def_id_by_name(tcx, "test_function");
                let ty = tcx.fn_sig(def_id.to_def_id()).no_bound_vars().unwrap().output();
                test_fn(&desc, tcx, ty, expected);
            });
        }
    }

    /// Tests invoking `format_def` on the item with the specified `name` from
    /// the given Rust `source`.  Returns the result of calling
    /// `test_function` with `format_def`'s result as an argument.
    /// (`test_function` should typically `assert!` that it got the expected
    /// result from `format_def`.)
    fn test_format_def<F, T>(source: &str, name: &str, test_function: F) -> T
    where
        F: FnOnce(Result<MixedSnippet, String>) -> T + Send,
        T: Send,
    {
        run_compiler(source, |tcx| {
            let def_id = find_def_id_by_name(tcx, name);
            let result = format_def(tcx, def_id);

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

    /// Tests invoking `GeneratedBindings::generate` on the given Rust `source`.
    /// Returns the result of calling `test_function` with the generated
    /// bindings as an argument. (`test_function` should typically `assert!`
    /// that it got the expected `GeneratedBindings`.)
    fn test_generated_bindings<F, T>(source: &str, test_function: F) -> T
    where
        F: FnOnce(Result<GeneratedBindings>) -> T + Send,
        T: Send,
    {
        run_compiler(source, |tcx| test_function(GeneratedBindings::generate(tcx)))
    }

    /// Invokes the Rust compiler on the given Rust `source` and then calls `f`
    /// on the `TyCtxt` representation of the compiled `source`.
    fn run_compiler<F, T>(source: impl Into<String>, f: F) -> T
    where
        F: for<'tcx> FnOnce(TyCtxt<'tcx>) -> T + Send,
        T: Send,
    {
        use rustc_session::config::{
            CodegenOptions, CrateType, Input, Options, OutputType, OutputTypes,
            SymbolManglingVersion,
        };

        const TEST_FILENAME: &str = "crubit_unittests.rs";

        // Setting `output_types` that will trigger code gen - otherwise some parts of
        // the analysis will be missing (e.g. `tcx.exported_symbols()`).
        // The choice of `Bitcode` is somewhat arbitrary (e.g. `Assembly`,
        // `Mir`, etc. would also trigger code gen).
        let output_types = OutputTypes::new(&[(OutputType::Bitcode, None /* PathBuf */)]);

        let opts = Options {
            crate_types: vec![CrateType::Rlib], // Test inputs simulate library crates.
            maybe_sysroot: Some(get_sysroot_for_testing()),
            output_types,
            edition: rustc_span::edition::Edition::Edition2021,
            unstable_features: rustc_feature::UnstableFeatures::Allow,
            lint_opts: vec![
                ("warnings".to_string(), rustc_lint_defs::Level::Deny),
                ("stable_features".to_string(), rustc_lint_defs::Level::Allow),
            ],
            cg: CodegenOptions {
                // As pointed out in `panics_and_exceptions.md` the tool only supports `-C
                // panic=abort` and therefore we explicitly opt into this config for tests.
                panic: Some(rustc_target::spec::PanicStrategy::Abort),
                // To simplify how unit tests are authored we force a specific mangling algorithm -
                // this way the tests can hardcode mangled-name-depdendent expectations (e.g. names
                // of thunks expected in test output).  The value below has been chosen based on
                // <internal link>/llvm-coverage-instrumentation.html#rust-symbol-mangling which
                // points out that `v0` mangling can be used to "ensure consistent and reversible
                // name mangling" in situations when "mangled names must be consistent across
                // compilations".
                symbol_mangling_version: Some(SymbolManglingVersion::V0),
                ..Default::default()
            },
            ..Default::default()
        };

        let config = rustc_interface::interface::Config {
            opts,
            crate_cfg: Default::default(),
            crate_check_cfg: Default::default(),
            input: Input::Str {
                name: rustc_span::FileName::Custom(TEST_FILENAME.to_string()),
                input: source.into(),
            },
            input_path: None,
            output_file: None,
            output_dir: None,
            file_loader: None,
            lint_caps: Default::default(),
            parse_sess_created: None,
            register_lints: None,
            override_queries: None,
            make_codegen_backend: None,
            registry: rustc_errors::registry::Registry::new(rustc_error_codes::DIAGNOSTICS),
        };

        rustc_interface::interface::run_compiler(config, |compiler| {
            compiler.enter(|queries| {
                use rustc_interface::interface::Result;
                let result: Result<Result<()>> = super::enter_tcx(queries, |tcx| {
                    // Explicitly force full `analysis` stage to detect compilation
                    // errors that the earlier stages might miss.  This helps ensure that the
                    // test inputs are valid Rust (even if `f` wouldn't
                    // have triggered full analysis).
                    tcx.analysis(())
                });

                // Flatten the outer and inner results into a single result.  (outer result
                // comes from `enter_tcx`; inner result comes from `analysis`).
                //
                // TODO(lukasza): Use `Result::flatten` API when it gets stabilized.  See also
                // https://github.com/rust-lang/rust/issues/70142
                let result: Result<()> = result.and_then(|result| result);

                // `analysis` might succeed even if there are some lint / warning errors.
                // Detecting these requires explicitly checking `compile_status`.
                let result: Result<()> = result.and_then(|()| compiler.session().compile_status());

                // Run the provided callback.
                let result: Result<T> = result.and_then(|()| super::enter_tcx(queries, f));
                result.expect("Test inputs shouldn't cause compilation errors")
            })
        })
    }
}
