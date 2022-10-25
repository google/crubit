// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{anyhow, bail, Context, Result};
use code_gen_utils::format_cc_ident;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use rustc_hir::{Item, ItemKind, Node, Unsafety};
use rustc_interface::Queries;
use rustc_middle::dep_graph::DepContext;
use rustc_middle::middle::exported_symbols::ExportedSymbol;
use rustc_middle::ty::{self, Ty, TyCtxt}; // See <internal link>/ty.html#import-conventions
use rustc_span::def_id::{DefId, LocalDefId, LOCAL_CRATE};
use rustc_target::spec::abi::Abi;
use rustc_target::spec::PanicStrategy;

pub struct GeneratedBindings {
    pub h_body: TokenStream,
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

        let h_body = {
            let crate_content = format_crate(tcx).unwrap_or_else(|err| {
                let txt = format!("Failed to generate bindings for the crate: {}", err);
                quote! { __COMMENT__ #txt }
            });
            // TODO(b/251445877): Replace `#pragma once` with include guards.
            quote! {
                #top_comment
                __HASH_TOKEN__ pragma once __NEWLINE__
                __NEWLINE__
                #crate_content
            }
        };

        Ok(Self { h_body })
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

fn format_ret_ty(ty: Ty) -> Result<TokenStream> {
    match ty.kind() {
        ty::TyKind::Never => Ok(quote! { void }), // `!`
        ty::TyKind::Tuple(types) if types.len() == 0 => Ok(quote! { void }), // `()`
        _ => format_ty(ty),
    }
}

fn format_ty(ty: Ty) -> Result<TokenStream> {
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
        ty::TyKind::Bool => quote! { bool },
        ty::TyKind::Float(ty::FloatTy::F32) => quote! { float },
        ty::TyKind::Float(ty::FloatTy::F64) => quote! { double },

        ty::TyKind::Char
        | ty::TyKind::Int(
            ty::IntTy::Isize | ty::IntTy::I8 | ty::IntTy::I16 | ty::IntTy::I32 | ty::IntTy::I64,
        )
        | ty::TyKind::Uint(
            ty::UintTy::Usize
            | ty::UintTy::U8
            | ty::UintTy::U16
            | ty::UintTy::U32
            | ty::UintTy::U64,
        ) => {
            // TODO(b/254094545): Add support for returning TokenStream *and* include paths.
            bail!("No support yet for `#include`ing C++ equivalent of `{ty}` (b/254094545)")
        }

        ty::TyKind::Int(ty::IntTy::I128) | ty::TyKind::Uint(ty::UintTy::U128) => {
            // TODO(b/254094650): Consider mapping this to Clang's (and GCC's) `__int128`
            // or to `absl::in128`.
            bail!("C++ doesn't have a standard equivalent of `{ty}` (b/254094650)");
        }

        ty::TyKind::Adt(..)
        | ty::TyKind::Foreign(..)
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

/// Formats a function with the given `def_id`.
///
/// Will panic if `def_id`
/// - is invalid
/// - doesn't identify a function,
/// - has generic parameters (any kind - lifetime parameters, type parameters,
///   or const parameters).
fn format_fn(tcx: TyCtxt, def_id: LocalDefId) -> Result<TokenStream> {
    let def_id: DefId = def_id.to_def_id(); // Convert LocalDefId to DefId.

    let item_name = tcx.item_name(def_id);
    let symbol_name = {
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

    match sig.abi {
        // "C" ABI is okay: Before https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html a Rust
        // panic that "escapes" a "C" ABI function leads to Undefined Behavior.  This is
        // unfortunate, but Crubit's `panics_and_exceptions.md` documents that `-Cpanic=abort` is
        // the only supported configuration.
        //
        // After https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html a Rust panic that
        // tries to "escape" a "C" ABI function will terminate the program.  This is okay.
        Abi::C { unwind: false } => (),

        // "C-unwind" ABI is okay: After https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html a
        // new "C-unwind" ABI may be used by Rust functions that want to safely propagate Rust
        // panics through frames that may belong to another language.
        Abi::C { unwind: true } => (),

        // TODO(b/254097223): Add support for Rust thunks.
        _ => bail!("Non-C ABI is not supported yet (b/254097223)"),
    };
    if item_name.as_str() != symbol_name.name {
        bail!("Mangled or renamed symbols are not supported yet");
    }

    let ret_type = format_ret_ty(sig.output()).context("Error formatting function return type")?;
    let fn_name = format_cc_ident(item_name.as_str()).context("Error formatting function name")?;
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
            format_ty(*ty)
                .with_context(|| format!("Error formatting the type of parameter #{index}"))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(quote! {
        extern "C" #ret_type #fn_name (
                #( #arg_types #arg_names ),*
            );
    })
}

/// Formats a Rust item idenfied by `def_id`.
///
/// Will panic if `def_id` is invalid (i.e. doesn't identify a Rust node or
/// item).
fn format_def(tcx: TyCtxt, def_id: LocalDefId) -> Result<TokenStream> {
    match tcx.hir().get_by_def_id(def_id) {
        Node::Item(item) => match item {
            Item { kind: ItemKind::Fn(_hir_fn_sig, generics, _body), .. } => {
                if generics.params.len() == 0 {
                    format_fn(tcx, def_id)
                } else {
                    bail!(
                        "Generic functions (lifetime-generic or type-generic) are not supported yet"
                    )
                }
            }
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
) -> TokenStream {
    let span = tcx.sess().source_map().span_to_embeddable_string(tcx.def_span(local_def_id));
    let name = tcx.def_path_str(local_def_id.to_def_id());

    // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
    // says: To print causes as well [...], use the alternate selector “{:#}”.
    let msg = format!("Error generating bindings for `{name}` defined at {span}: {err:#}");
    quote! { __NEWLINE__ __NEWLINE__ __COMMENT__ #msg __NEWLINE__ }
}

/// Formats all public items from the Rust crate being compiled (aka the
/// `LOCAL_CRATE`).
fn format_crate(tcx: TyCtxt) -> Result<TokenStream> {
    // TODO(lukasza): We probably shouldn't be using `exported_symbols` as the main
    // entry point for finding Rust definitions that need to be wrapping in C++
    // bindings.  For example, it _seems_ that things like `type` aliases or
    // `struct`s (without an `impl`) won't be visible to a linker and therefore
    // won't have exported symbols.  Additionally, walking Rust's modules top-down
    // might result in easier translation into C++ namespaces.
    let snippets =
        tcx.exported_symbols(LOCAL_CRATE).iter().filter_map(move |(symbol, _)| match symbol {
            ExportedSymbol::NonGeneric(def_id) => {
                // It seems that non-generic exported symbols should all be defined in the
                // `LOCAL_CRATE`.  Furthermore, `def_id` seems to be a `LocalDefId`.  OTOH, it
                // isn't clear why `ExportedSymbol::NonGeneric` holds a `DefId` rather than a
                // `LocalDefId`.  For now, we assert `expect_local` below (and if it fails, then
                // hopefully it will help us understand these things better and maybe add
                // extra unit tests against out code).
                let local_id = def_id.expect_local();

                Some(match format_def(tcx, local_id) {
                    Ok(snippet) => snippet,
                    Err(err) => format_unsupported_def(tcx, local_id, err),
                })
            }
            ExportedSymbol::Generic(def_id, _substs) => {
                // Ignore non-local defs.  Map local defs to an unsupported comment.
                //
                // We are guessing that a non-local `def_id` can happen when the `LOCAL_CRATE`
                // exports a monomorphization/specialization of a generic defined in a different
                // crate.  One specific example (covered via `async fn` in one of the tests) is
                // `DefId(2:14250 ~ core[ef75]::future::from_generator)`.
                def_id.as_local().map(|local_id| {
                    format_unsupported_def(tcx, local_id, anyhow!("Generics are not supported yet."))
                })
            }
            ExportedSymbol::DropGlue(..) | ExportedSymbol::NoDefId(..) => None,
        });

    // TODO(b/254690602): Decide whether using `#crate_name` as the name of the
    // top-level namespace is okay (e.g. investigate if this name is globally
    // unique + ergonomic).
    let crate_name = format_cc_ident(tcx.crate_name(LOCAL_CRATE).as_str())?;
    Ok(quote! {
        namespace #crate_name {
            #( #snippets )*
        }
    })
}

#[cfg(test)]
pub mod tests {
    use super::{format_def, format_ret_ty, format_ty, GeneratedBindings};

    use anyhow::Result;
    use itertools::Itertools;
    use proc_macro2::TokenStream;
    use quote::quote;
    use rustc_middle::ty::{Ty, TyCtxt};
    use rustc_span::def_id::LocalDefId;
    use std::path::PathBuf;

    use token_stream_matchers::{assert_cc_matches, assert_cc_not_matches};

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
    fn test_generated_bindings_fn_success() {
        // This test covers only a single example of a function that should get a C++
        // binding. Additional coverage of how items are formatted is provided by
        // `test_format_def_...` tests.
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
        });
    }

    #[test]
    fn test_generated_bindings_fn_non_pub() {
        let test_src = r#"
                #![allow(dead_code)]
                extern "C" fn private_function() {
                    println!("foo");
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.expect("Test expects success");

            // Non-public functions should not be present in the generated bindings.
            assert_cc_not_matches!(bindings.h_body, quote! { private_function });
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
            assert_cc_matches!(
                result.expect("Test expects success here"),
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
            assert_cc_matches!(
                result.expect("Test expects success here"),
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
            assert_cc_matches!(
                result.expect("Test expects success here"),
                quote! {
                    extern "C" void never_returning_function();
                }
            );
        })
    }

    #[test]
    fn test_format_def_fn_mangling() {
        let test_src = r#"
                pub extern "C" fn public_function() {}
            "#;
        test_format_def(test_src, "public_function", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Mangled or renamed symbols are not supported yet");
        });
    }

    #[test]
    fn test_format_def_fn_export_name() {
        let test_src = r#"
                #[export_name = "exported_function"]
                #[no_mangle]
                pub extern "C" fn public_function() {}
            "#;
        test_format_def(test_src, "public_function", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Mangled or renamed symbols are not supported yet");
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
            // TODO(lukasza): Update test expectations below once `const fn` example from
            // the testcase doesn't just error out (and is instead supported as
            // a non-`consteval` binding).
            // TODO(b/254095787): Update test expectations below once `const fn` from Rust
            // is translated into a `consteval` C++ function.
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Non-C ABI is not supported yet (b/254097223)");
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
            assert_cc_matches!(
                result.expect("Test expects success here"),
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
            assert_cc_matches!(
                result.expect("Test expects success here"),
                quote! {
                    extern "C" double type_aliased_return();
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
            assert_eq!(
                err,
                "Generic functions (lifetime-generic or type-generic) are not supported yet"
            );
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
            assert_eq!(
                err,
                "Generic functions (lifetime-generic or type-generic) are not supported yet"
            );
        });
    }

    #[test]
    fn test_format_def_unsupported_fn_async() {
        let test_src = r#"
                pub async fn async_function() {}
            "#;
        test_format_def(test_src, "async_function", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Non-C ABI is not supported yet (b/254097223)");
        });
    }

    #[test]
    fn test_format_def_unsupported_fn_non_c_abi() {
        let test_src = r#"
                pub fn default_rust_abi_function() {}
            "#;
        test_format_def(test_src, "default_rust_abi_function", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Non-C ABI is not supported yet (b/254097223)");
        })
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
            assert_cc_matches!(
                result.expect("Test expects success here"),
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
            assert_cc_matches!(
                result.expect("Test expects success here"),
                quote! {
                    extern "C" void some_function(double __param_0);
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
            // TODO(b/254097223): Change the expectations once Rust-ABI functions are
            // supported. Note that the test cannot use `extern "C"` in the
            // meantime, because `()` is not FFI-safe (i.e. Rust won't allow
            // using it with `extern "C"`).
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Non-C ABI is not supported yet (b/254097223)");
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

    #[test]
    fn test_format_def_unsupported_hir_item_kind() {
        let test_src = r#"
                pub struct SomeStruct(i32);
            "#;
        test_format_def(test_src, "SomeStruct", |result| {
            let err = result.expect_err("Test expects an error here");
            assert_eq!(err, "Unsupported rustc_hir::hir::ItemKind: struct");
        });
    }

    #[test]
    fn test_format_ret_ty_successes() {
        // Test coverage for cases where `format_ret_ty` returns an `Ok(...)`.
        // Additional testcases are covered by `test_format_ty_successes`
        // (because `format_ret_ty` delegates most cases to `format_ty`).
        let testcases = [
            // ( <Rust type>, <expected C++ type> )
            ("bool", "bool"), // TyKind::Bool
            ("()", "void"),
            // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
            // details).
            ("!", "void"),
        ];
        test_ty(&testcases, |desc, ty, expected| {
            let actual = format_ret_ty(ty).unwrap().to_string();
            let expected = expected.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual, expected, "{desc}");
        });
    }

    #[test]
    fn test_format_ty_successes() {
        // Test coverage for cases where `format_ty` returns an `Ok(...)`.
        let testcases = [
            // ( <Rust type>, <expected C++ type> )
            ("bool", "bool"),  // TyKind::Bool
            ("f32", "float"),  // TyKind::Float(ty::FloatTy::F32)
            ("f64", "double"), // TyKind::Float(ty::FloatTy::F64)
            // Extra parens/sugar are expected to be ignored:
            ("(bool)", "bool"),
        ];
        test_ty(&testcases, |desc, ty, expected| {
            let actual = format_ty(ty).unwrap().to_string();
            let expected = expected.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual, expected, "{desc}");
        });
    }

    #[test]
    fn test_format_ty_failures() {
        // This test provides coverage for cases where `format_ty` returns an
        // `Err(...)`.
        //
        // TODO(lukasza): Add test coverage for:
        // - TyKind::Adt (structs, unions, enums, etc.)
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
        // `format_ty` should panic):
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
                "char", // TyKind::Char
                "No support yet for `#include`ing C++ equivalent of `char` (b/254094545)",
            ),
            (
                "i32", // TyKind::Int
                "No support yet for `#include`ing C++ equivalent of `i32` (b/254094545)",
            ),
            (
                "u32", // TyKind::UInt
                "No support yet for `#include`ing C++ equivalent of `u32` (b/254094545)",
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
        ];
        test_ty(&testcases, |desc, ty, expected_err| {
            let anyhow_err = format_ty(ty).unwrap_err();
            let actual_err = format!("{anyhow_err:#}");
            assert_eq!(&actual_err, *expected_err, "{desc}");
        });
    }

    fn test_ty<TestFn, Expectation>(testcases: &[(&str, Expectation)], test_fn: TestFn)
    where
        TestFn: Fn(/* testcase_description: */ &str, Ty, &Expectation) -> () + Sync,
        Expectation: Sync,
    {
        for (index, (input, expected)) in testcases.into_iter().enumerate() {
            let desc = format!("test #{index}: test input: `{input}`");
            let input = {
                let ty_tokens: TokenStream = input.parse().unwrap();
                let input = quote! {
                    #![allow(unused_parens)]
                    pub fn test_function() -> #ty_tokens { panic!("") }
                };
                input.to_string()
            };
            run_compiler(input, |tcx| {
                let def_id = find_def_id_by_name(tcx, "test_function");
                let ty = tcx.fn_sig(def_id.to_def_id()).no_bound_vars().unwrap().output();
                test_fn(&desc, ty, expected);
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
        F: FnOnce(Result<TokenStream, String>) -> T + Send,
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
        match items_with_matching_name.as_slice() {
            &[] => {
                let found_names = hir_items()
                    .map(|item| item.ident.name.as_str())
                    .filter(|s| !s.is_empty())
                    .sorted()
                    .dedup()
                    .map(|name| format!("`{name}`"))
                    .collect_vec();
                panic!("No items named `{}`.\nInstead found:\n{}", name, found_names.join(",\n"));
            }
            &[item] => item.def_id.def_id,
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
        };

        const TEST_FILENAME: &str = "crubit_unittests.rs";

        // Setting `output_types` that will trigger code gen - otherwise some parts of
        // the analysis will be missing (e.g. `tcx.exported_symbols()`).
        // The choice of `Bitcode` is somewhat arbitrary (e.g. `Assembly`,
        // `Mir`, etc. would also trigger code gen).
        let output_types = OutputTypes::new(&[(OutputType::Bitcode, None /* PathBuf */)]);

        // Ensure that Rust compiler's output is associated with the test being run
        // (instead of haphazardly being interleaved with random output from
        // the test harness or other tests). This is done by forwarding all the
        // diagnostic writes to `eprint!`, so that they can be captured by the
        // test harness (unlike directly writing to `std::io::stdout()` or
        // `std::io::stderr()`; see also doc-hidden `std::io::set_output_capture` and
        // the `cargo test --nocapture` flag).
        let diagnostic_output = rustc_session::DiagnosticOutput::Raw(Box::new(
            string_callback_writer::StringCallbackWriter::new(|s| eprint!("{s}")),
        ));

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
                panic: Some(rustc_target::spec::PanicStrategy::Abort),
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
            diagnostic_output,
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

    mod string_callback_writer {
        use std::io::Write;

        /// `StringCallbackWriter` provides an implementation of the
        /// `std::io::Write` trait that decodes the written bytes as
        /// strings, and forwards them to the provided callback.
        pub struct StringCallbackWriter<F: FnMut(&str)> {
            callback: F,
            buf: Vec<u8>,
        }

        impl<F: FnMut(&str)> StringCallbackWriter<F> {
            pub fn new(callback: F) -> Self {
                Self { callback, buf: Vec::new() }
            }

            fn drain_buf_into_callback(&mut self) {
                if self.buf.len() == 0 {
                    return;
                }
                loop {
                    match std::str::from_utf8(self.buf.as_slice()) {
                        Ok(s) => {
                            (self.callback)(s);
                            self.buf.clear();
                            break;
                        }
                        Err(err) => {
                            let pos = err.valid_up_to();
                            if pos == 0 {
                                if self.buf.len() < 4 {
                                    // Up to 3 first bytes might still turn out to be a start of a
                                    // valid UTF-8 character - let's wait for more bytes.
                                    break;
                                } else {
                                    // Otherwise, there is some invalid UTF-8 at the start of the
                                    // buffer.  Try to recover by chopping off the invalid bytes
                                    // one-at-a-time.
                                    //
                                    // Vec::remove is O(n) so overall this may take O(n^2) time
                                    // which is a bit undesirable, but this should never happen in
                                    // practice + this code is only used for test support.
                                    (self.callback)("�");
                                    self.buf.remove(0);
                                    continue; // Retry.
                                }
                            } else {
                                let s = std::str::from_utf8(&self.buf[..pos])
                                    .expect("`valid_up_to` should ensure success here");
                                (self.callback)(s);
                                self.buf.drain(0..pos);

                                // Retry in case a valid UTF-8 is still present later in the buffer.
                                continue;
                            }
                        }
                    }
                }
                assert!(self.buf.len() < 4);
            }
        }

        impl<F: FnMut(&str)> Write for StringCallbackWriter<F> {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                let written_bytes = self.buf.write(buf)?;
                self.drain_buf_into_callback();
                Ok(written_bytes)
            }

            fn flush(&mut self) -> std::io::Result<()> {
                // Not calling `self.buf.flush()` because it is a no-op for `Vec<u8>`.
                self.drain_buf_into_callback();
                Ok(())
            }
        }

        mod test {
            use super::StringCallbackWriter;

            use itertools::{chain, Itertools};
            use std::cell::{Ref, RefCell};
            use std::io::Write;
            use std::iter::once;
            use std::rc::Rc;

            struct StringCallbackWriterTestHelper {
                output: Rc<RefCell<Vec<String>>>,
                writer: Box<dyn Write>,
            }

            impl StringCallbackWriterTestHelper {
                fn new() -> Self {
                    let buf = Rc::new(RefCell::new(Vec::new()));
                    Self {
                        output: buf.clone(),
                        writer: Box::new(StringCallbackWriter::new(move |s| {
                            buf.borrow_mut().push(s.to_string())
                        })),
                    }
                }

                fn output(&self) -> Ref<'_, Vec<String>> {
                    self.output.borrow()
                }
            }

            #[test]
            fn test_valid_utf8() {
                let mut test = StringCallbackWriterTestHelper::new();

                write!(&mut test.writer, "foo").unwrap();
                write!(&mut test.writer, "bar").unwrap();

                assert_eq!(2, test.output().len());
                assert_eq!("foo", &test.output()[0]);
                assert_eq!("bar", &test.output()[1]);
            }

            #[test]
            fn test_multibyte_character_broken_across_writes() {
                let mut test = StringCallbackWriterTestHelper::new();

                // UTF-8 byte/hex encoding:
                // - Crab character / U+1F980: f0 9f a6 80
                // - Exclamation mark / U+0021: 21

                // 2 first writes are incomplete - they shouldn't trigger the callback.
                test.writer.write(&[0xf0]).unwrap();
                assert_eq!(0, test.output().len());
                test.writer.write(&[0x9f, 0xa6]).unwrap();
                assert_eq!(0, test.output().len());

                // The next write completes the utf8 character.
                test.writer.write(&[0x80, 0x21]).unwrap();
                assert_eq!(1, test.output().len());
                assert_eq!("🦀!", &test.output()[0]);
            }

            #[test]
            fn test_invalid_utf8() {
                let mut test = StringCallbackWriterTestHelper::new();

                let input = chain!(
                    "foo".as_bytes().into_iter().copied(),
                    once(0xff), // This byte can never appear in UTF-8.
                    "bar".as_bytes().into_iter().copied(),
                );
                test.writer.write(input.collect_vec().as_slice()).unwrap();

                assert_eq!(3, test.output().len());
                assert_eq!("foo", &test.output()[0]);
                assert_eq!("�", &test.output()[1]);
                assert_eq!("bar", &test.output()[2]);
            }
        }
    }
}
