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
use itertools::Itertools;

use rustc_middle::ty::TyCtxt;
use rustc_session::config::{CrateType, Input, Options, OutputType, OutputTypes};
use rustc_span::def_id::LocalDefId;
use rustc_target::spec::TargetTriple;

use std::path::{Path, PathBuf};

#[cfg(oss)]
const TOOLCHAIN_ROOT: &str = "rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools/rust_toolchain/lib/rustlib/x86_64-unknown-linux-gnu";
#[cfg(not(oss))]
const TOOLCHAIN_ROOT: &str = env!("G3_SYSROOT_PATH");

/// Returns the `rustc` sysroot that is suitable for the environment where
/// unit tests run.
///
/// The sysroot is used internally by `run_compiler_for_testing`, but it may
/// also be passed as `--sysroot=...` in `rustc_args` argument of
/// `run_compiler`
pub fn get_sysroot_for_testing() -> PathBuf {
    let runfiles = runfiles::Runfiles::create().unwrap();
    let loc = runfiles.rlocation(Path::new(TOOLCHAIN_ROOT)).expect("Failed to locate runfile");
    assert!(loc.exists(), "Sysroot directory '{}' doesn't exist", loc.display());
    assert!(loc.is_dir(), "Provided sysroot '{}' is not a directory", loc.display());
    loc
}

/// If a rustc --target arg is necessary, sets it up and returns its value.
///
/// In google3, we use a target json in some configurations. Its filename needs
/// to match the one used to build the rust standard libraries, and it must be
/// a "real" file (not a symlink). This function sets this up by copying the
/// target path passed via the RUSTC_TARGET_PATH env var to a file with the
/// expected name in a temporary directory.
/// See cs/GOOGLE3_RUSTC_TARGET_JSON for code related to this.
pub fn setup_rustc_target_for_testing(target_dir: &Path) -> Option<String> {
    assert!(target_dir.exists(), "target dir '{}' doesn't exist", target_dir.display());
    if let Ok(original_target) = &std::env::var("RUSTC_TARGET_PATH") {
        let loc = &Path::new(original_target);
        assert!(loc.exists(), "target json path '{}' doesn't exist", loc.display());
        assert!(loc.is_file(), "target json path '{}' doesn't point to a file", loc.display());
        let target = target_dir.join("google3_rustc_target.json");
        std::fs::copy(loc, &target).unwrap_or_else(|_| {
            panic!("failed to copy target_json from '{}' to '{}'", loc.display(), target.display())
        });
        Some(target.clone().into_os_string().into_string().unwrap_or_else(|_| {
            panic!("failed to convert target path '{}' to string", target.display())
        }))
    } else {
        None
    }
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

    let mut opts = Options {
        crate_types: vec![CrateType::Rlib], // Test inputs simulate library crates.
        maybe_sysroot: Some(get_sysroot_for_testing()),
        output_types,
        edition: rustc_span::edition::Edition::Edition2021,
        unstable_features: rustc_feature::UnstableFeatures::Allow,
        lint_opts: vec![
            ("warnings".to_string(), rustc_lint_defs::Level::Deny),
            ("stable_features".to_string(), rustc_lint_defs::Level::Allow),
        ],
        ..Default::default()
    };

    let target_dir = tempfile::TempDir::new().unwrap();

    if let Some(target) = &setup_rustc_target_for_testing(target_dir.path()) {
        let target_path = &Path::new(target);
        opts.target_triple = TargetTriple::from_path(target_path).unwrap_or_else(|_| {
            panic!("failed to construct a TargetTriple from target: '{}'", target_path.display())
        });
    }

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
        psess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        hash_untracked_state: None,
        registry: rustc_errors::registry::Registry::new(rustc_errors::DIAGNOSTICS),
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES.to_vec(),
        ice_file: None,
        expanded_args: vec![],
        using_internal_features: std::sync::Arc::new(false.into()),
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
                // Detecting these requires explicitly checking.
                if let Some(guar) = compiler.sess.dcx().has_errors() {
                    return Err(guar);
                }

                // Run the provided callback.
                Ok(query_context.enter(callback))
            };
            try_func().expect("Test inputs shouldn't cause compilation errors")
        })
    })
}

/// Finds the definition id of a Rust item with the specified `name`.
/// Panics if no such item is found, or if there is more than one match.
pub fn find_def_id_by_name(tcx: TyCtxt, name: &str) -> LocalDefId {
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
}
