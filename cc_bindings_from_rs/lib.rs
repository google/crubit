// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use rustc_hir::{Item, ItemKind, Node};
use rustc_interface::Queries;
use rustc_middle::middle::exported_symbols::ExportedSymbol;
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::LOCAL_CRATE;

// TODO(lukasza): Replace `get_names_of_exported_fns` with something that can
// generate C++ bindings.
pub fn get_names_of_exported_fns(tcx: TyCtxt) -> impl Iterator<Item = String> + '_ {
    tcx.exported_symbols(LOCAL_CRATE).iter().filter_map(move |(symbol, _)| match symbol {
        ExportedSymbol::NonGeneric(def_id) => {
            match tcx.hir().find_by_def_id(def_id.expect_local()).unwrap() {
                Node::Item(Item { kind: ItemKind::Fn { .. }, .. }) => {
                    Some(tcx.def_path_str(*def_id))
                }
                _ => None,
            }
        }
        ExportedSymbol::Generic(..) | ExportedSymbol::DropGlue(_) | ExportedSymbol::NoDefId(_) => {
            None
        }
    })
}

/// Helper (used by `main` and `test::run_compiler`) for invokind functions
/// operating on `TyCtxt`.
pub fn enter_tcx<'tcx, F, T>(
    queries: &'tcx Queries<'tcx>,
    f: F,
) -> rustc_interface::interface::Result<T>
where
    F: FnOnce(rustc_middle::ty::TyCtxt<'tcx>) -> T + Send,
    T: Send,
{
    let query_context = queries.global_ctxt()?;
    Ok(query_context.peek_mut().enter(f))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn test_get_names_of_exported_fns_public_vs_private() {
        let test_src = r#"
                pub fn public_function() {
                    private_function()
                }

                fn private_function() {}
            "#;
        let exported_functions = get_names_of_exported_fns(test_src);
        assert_eq!(1, exported_functions.len());
        assert_eq!("public_function", exported_functions[0]);
    }

    #[test]
    #[should_panic]
    fn test_panic_when_syntax_errors_in_test_inputs() {
        get_names_of_exported_fns("syntax error here");
    }

    fn get_names_of_exported_fns(source: &str) -> Vec<String> {
        run_compiler(source, |tcx| super::get_names_of_exported_fns(tcx).collect_vec())
    }

    /// Compiles Rust `source` then calls `f` on the `TyCtxt` representation
    /// of the compiled `source`.
    fn run_compiler<F, T>(source: impl Into<String>, f: F) -> T
    where
        F: for<'tcx> FnOnce(rustc_middle::ty::TyCtxt<'tcx>) -> T + Send,
        T: Send,
    {
        use lazy_static::lazy_static;
        use rustc_session::config::{CrateType, Input, Options, OutputType, OutputTypes};
        use std::path::PathBuf;

        // TODO(lukasza): This probably won't work in Bazel...
        lazy_static! {
            static ref RUSTC_SYSROOT: String = {
                let output = std::process::Command::new("rustc")
                    .arg("--print=sysroot")
                    .current_dir(".")
                    .output()
                    .expect("For now we depend on `rustc` invocation to succeed... sorry...");
                std::str::from_utf8(&output.stdout)
                    .expect("Only UTF-8 compatible rustc sysroot is supported... sorry...")
                    .trim()
                    .into()
            };
        }

        const TEST_FILENAME: &str = "crubit_unittests.rs";

        // Setting `output_types` that will trigger code gen - otherwise some parts of
        // the analysis will be missing (e.g. `tcx.exported_symbols()`).
        // The choice of `Bitcode` is somewhat arbitrary (e.g. `Assembly`,
        // `Mir`, etc. would also trigger code gen).
        let output_types = OutputTypes::new(&[(OutputType::Bitcode, None /* PathBuf */)]);

        let opts = Options {
            crate_types: vec![CrateType::Rlib], // Test inputs simulate library crates.
            maybe_sysroot: Some(PathBuf::from(RUSTC_SYSROOT.clone())),
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
