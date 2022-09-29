// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{bail, Result};
use proc_macro2::TokenStream;
use quote::quote;
use rustc_interface::Queries;
use rustc_middle::dep_graph::DepContext;
use rustc_middle::middle::exported_symbols::ExportedSymbol;
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::{LocalDefId, LOCAL_CRATE};
use std::fmt::Display;

pub struct GeneratedBindings {
    pub h_body: TokenStream,
}

impl GeneratedBindings {
    pub fn generate(tcx: TyCtxt) -> Self {
        let top_comment = {
            let crate_name = tcx.crate_name(LOCAL_CRATE);
            let txt = format!(
                "Automatically @generated C++ bindings for the following Rust crate:\n\
                 {crate_name}"
            );
            quote! { __COMMENT__ #txt __NEWLINE__ }
        };

        let h_body = {
            let main_content = format_crate(tcx);
            quote! { #top_comment #main_content }
        };

        Self { h_body }
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

fn format_def(_tcx: TyCtxt, _def_id: LocalDefId) -> Result<TokenStream> {
    bail!("Nothing works yet!")
}

fn format_unsupported_def(
    tcx: TyCtxt,
    local_def_id: LocalDefId,
    err_msg: impl Display,
) -> TokenStream {
    let span = tcx.sess().source_map().span_to_embeddable_string(tcx.def_span(local_def_id));
    let name = tcx.def_path_str(local_def_id.to_def_id());
    let msg = format!("Error while generating bindings for `{name}` defined at {span}: {err_msg}");
    quote! { __COMMENT__ #msg }
}

fn format_crate(tcx: TyCtxt) -> TokenStream {
    let snippets =
        tcx.exported_symbols(LOCAL_CRATE).iter().filter_map(move |(symbol, _)| match symbol {
            ExportedSymbol::NonGeneric(def_id) => {
                let def_id = def_id.expect_local(); // Exports are always from the local crate.
                match format_def(tcx, def_id) {
                    Ok(snippet) => Some(snippet),
                    Err(err) => Some(format_unsupported_def(tcx, def_id, err)),
                }
            }
            ExportedSymbol::Generic(def_id, ..) => {
                let def_id = def_id.expect_local(); // Exports are always from the local crate.
                Some(format_unsupported_def(tcx, def_id, "Generics are not supported yet."))
            }
            ExportedSymbol::DropGlue(_) | ExportedSymbol::NoDefId(_) => None,
        });
    quote! { #( #snippets )* }
}

#[cfg(test)]
pub mod tests {
    use super::GeneratedBindings;

    use anyhow::Result;
    use std::path::PathBuf;

    use token_stream_printer::tokens_to_string;

    pub fn get_sysroot_for_testing() -> PathBuf {
        let runfiles = runfiles::Runfiles::create().unwrap();
        runfiles.rlocation(if std::env::var("LEGACY_TOOLCHAIN_RUST_TEST").is_ok() {
            "google3/third_party/unsupported_toolchains/rust/toolchains/nightly"
        } else {
            "google3/nowhere/llvm/rust"
        })
    }

    #[test]
    #[should_panic]
    fn test_panic_when_syntax_errors_in_test_inputs() {
        run_compiler("syntax error here", |_tcx| ())
    }

    #[test]
    fn test_generated_bindings_happy_path() -> Result<()> {
        let test_src = r#"
                pub fn public_function() {
                    private_function()
                }

                fn private_function() {}
            "#;
        test_generated_bindings(test_src, |bindings| {
            // TODO(lukasza): Use `assert_cc_matches!` from
            // `rs_bindings_from_cc/token_stream_matchers.rs` here.
            let h_body = tokens_to_string(bindings.h_body)?;
            assert_eq!(
                h_body,
                "// Automatically @generated C++ bindings for the following Rust crate:\n\
                 // rust_out\n\
                 \n\
                 // Error while generating bindings for `public_function` \
                         defined at <crubit_unittests.rs>:2:17: 2:41: \
                         Nothing works yet!\n"
            );
            Ok(())
        })
    }

    fn test_generated_bindings<F, T>(source: &str, f: F) -> T
    where
        F: FnOnce(GeneratedBindings) -> T + Send,
        T: Send,
    {
        run_compiler(source, |tcx| f(GeneratedBindings::generate(tcx)))
    }

    /// Compiles Rust `source` then calls `f` on the `TyCtxt` representation
    /// of the compiled `source`.
    fn run_compiler<F, T>(source: impl Into<String>, f: F) -> T
    where
        F: for<'tcx> FnOnce(rustc_middle::ty::TyCtxt<'tcx>) -> T + Send,
        T: Send,
    {
        use rustc_session::config::{CrateType, Input, Options, OutputType, OutputTypes};

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
            diagnostic_output: rustc_session::DiagnosticOutput::Default,
            lint_caps: Default::default(),
            parse_sess_created: None,
            register_lints: None,
            override_queries: None,
            make_codegen_backend: None,
            registry: rustc_errors::registry::Registry::new(rustc_error_codes::DIAGNOSTICS),
        };

        rustc_interface::interface::run_compiler(config, |compiler| {
            compiler.enter(|queries| super::enter_tcx(queries, f))
        })
        .expect("Test inputs shouldn't cause compilation errors.")
    }
}
