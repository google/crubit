// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A wrapper around `run_compiler` for testing only.

#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_feature;
extern crate rustc_interface;
extern crate rustc_lint_defs;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

use rustc_middle::ty::TyCtxt;
use rustc_session::config::{CodegenOptions, CrateType, Input, Options, OutputType, OutputTypes};

use std::path::PathBuf;

#[cfg(oss)]
const TOOLCHAIN_ROOT: &str = "rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools/rust_toolchain/lib/rustlib/x86_64-unknown-linux-gnu";
#[cfg(not(oss))]
const TOOLCHAIN_ROOT: &str = "google3/nowhere/llvm/rust/main_sysroot";

/// Returns the `rustc` sysroot that is suitable for the environment where
/// unit tests run.
///
/// The sysroot is used internally by `run_compiler_for_testing`, but it may
/// also be passed as `--sysroot=...` in `rustc_args` argument of
/// `run_compiler`
pub fn get_sysroot_for_testing() -> PathBuf {
    let runfiles = runfiles::Runfiles::create().unwrap();
    let loc = runfiles.rlocation(std::path::Path::new(TOOLCHAIN_ROOT));
    assert!(loc.exists(), "Sysroot directory '{}' doesn't exist", loc.display());
    assert!(loc.is_dir(), "Provided sysroot '{}' is not a directory", loc.display());
    loc
}

/// `run_compiler_for_testing` is similar to `run_compiler`: it invokes the
/// `callback` after parsing and analysis are done, but instead of taking
/// `rustc_args` it:
///
/// * Invokes the Rust compiler on the given Rust `source`
/// * Hardcodes other compiler flags (e.g. picks Rust 2021 edition, and opts
///   into treating all warnings as errors).
pub fn run_compiler_for_testing<F, T>(source: impl Into<String>, callback: F) -> T
where
    F: for<'tcx> FnOnce(TyCtxt<'tcx>) -> T + Send,
    T: Send,
{
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
        output_file: None,
        output_dir: None,
        file_loader: None,
        lint_caps: Default::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: rustc_errors::registry::Registry::new(rustc_error_codes::DIAGNOSTICS),
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES,
        ice_file: None,
        expanded_args: vec![],
    };

    rustc_interface::interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            use rustc_interface::interface::Result;
            let try_func = || -> Result<T> {
                let mut query_context = queries.global_ctxt()?;
                query_context.enter(|tcx| {
                    // Explicitly force full `analysis` stage to detect compilation
                    // errors that the earlier stages might miss.  This helps ensure that the
                    // test inputs are valid Rust (even if `callback` wouldn't
                    // have triggered full analysis).
                    tcx.analysis(())
                })?;

                // `analysis` might succeed even if there are some lint / warning errors.
                // Detecting these requires explicitly checking `compile_status`.
                compiler.session().compile_status()?;

                // Run the provided callback.
                Ok(query_context.enter(callback))
            };
            try_func().expect("Test inputs shouldn't cause compilation errors")
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Test inputs shouldn't cause compilation errors")]
    fn test_run_compiler_for_testing_panic_when_test_input_contains_syntax_errors() {
        run_compiler_for_testing("syntax error here", |_tcx| panic!("This part shouldn't execute"))
    }

    #[test]
    #[should_panic(expected = "Test inputs shouldn't cause compilation errors")]
    fn test_run_compiler_for_testing_panic_when_test_input_triggers_analysis_errors() {
        run_compiler_for_testing("#![feature(no_such_feature)]", |_tcx| {
            panic!("This part shouldn't execute")
        })
    }

    #[test]
    #[should_panic(expected = "Test inputs shouldn't cause compilation errors")]
    fn test_run_compiler_for_testing_panic_when_test_input_triggers_warnings() {
        run_compiler_for_testing("pub fn foo(unused_parameter: i32) {}", |_tcx| {
            panic!("This part shouldn't execute")
        })
    }

    #[test]
    fn test_run_compiler_for_testing_nightly_features_ok_in_test_input() {
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
        run_compiler_for_testing(test_src, |_tcx| ())
    }

    #[test]
    fn test_run_compiler_for_testing_stabilized_features_ok_in_test_input() {
        // This test arbitrarily picks `const_ptr_offset_from` as an example of a
        // feature that has been already stabilized.
        run_compiler_for_testing("#![feature(const_ptr_offset_from)]", |_tcx| ())
    }
}
