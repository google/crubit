// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A wrapper around `run_compiler` for testing only.

#![feature(rustc_private)]
#![allow(unexpected_cfgs)]

extern crate rustc_driver;
extern crate rustc_feature;
extern crate rustc_interface;
extern crate rustc_lint_defs;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
#[rustversion::since(2026-08-22)]
extern crate rustc_structures;
extern crate rustc_target;
use itertools::Itertools;

use rustc_middle::ty::TyCtxt;
use rustc_session::config::{Input, Options, Sysroot};
use rustc_span::def_id::LocalDefId;
use rustc_target::spec::TargetTuple;
use std::path::Path;
use std::path::PathBuf;

rustversion::select! {
    before(2026-08-22) => {
        use rustc_session::config::CrateType;
    }
    _ => {
        use rustc_structures::CrateType;
    }
}

rustversion::select! {
    before(2026-11-23) => {
        use rustc_session::config::{OutputType, OutputTypes};
    }
    _ => {
        use rustc_session::output::{OutputType, OutputTypes};
    }
}

/// Returns the `rustc` sysroot that is suitable for the environment where
/// unit tests run.
///
/// The sysroot is used internally by `run_compiler_for_testing`, but it may
/// also be passed as `--sysroot=...` in `rustc_args` argument of
/// `run_compiler`
pub fn sysroot_path() -> Option<PathBuf> {
    #[cfg(not(bazel))]
    {
        None
    }

    #[cfg(bazel)]
    {
        #[cfg(oss)]
        let loc = PathBuf::from(
            std::env::var_os("RUNFILES_DIR")
                .or_else(|| std::env::var_os("TEST_SRCDIR"))
                .expect("Neither RUNFILES_DIR nor TEST_SRCDIR was set"),
        )
        .join("+crubit_toolchains+rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools/rust_toolchain");
        #[cfg(not(oss))]
        let loc = {
            const TOOLCHAIN_ROOT: &str = env!("G3_SYSROOT_PATH");
            let runfiles = runfiles::Runfiles::create().unwrap();
            runfiles.rlocation(Path::new(TOOLCHAIN_ROOT)).expect("Failed to locate runfile")
        };
        assert!(loc.exists(), "Sysroot directory '{}' doesn't exist", loc.display());
        assert!(loc.is_dir(), "Provided sysroot '{}' is not a directory", loc.display());
        Some(loc)
    }
}

/// If a rustc --target arg is necessary, sets it up and returns its value.
///
/// We use a target json in some configurations. Its filename needs to match the
/// one used to build the rust standard libraries, and it must be a "real" file
/// (not a symlink). This function sets this up by copying the target path
/// passed via the RUSTC_TARGET_PATH env var to a file with the expected name in
/// a temporary directory.
/// See (internal link) for code related to this.
pub fn setup_rustc_target_for_testing(target_dir: &Path) -> Option<String> {
    assert!(target_dir.exists(), "target dir '{}' doesn't exist", target_dir.display());
    if let Ok(original_target) = &std::env::var("RUSTC_TARGET_PATH") {
        let loc = &Path::new(original_target);
        assert!(loc.exists(), "target json path '{}' doesn't exist", loc.display());
        assert!(loc.is_file(), "target json path '{}' doesn't point to a file", loc.display());
        let target = target_dir.join("rustc_target.json");
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
/// * Hardcodes other compiler flags (e.g. picks Rust 2021 edition, and opts into treating all
///   warnings as errors).
pub fn run_compiler_for_testing<F, T>(source: impl Into<String>, callback: F) -> T
where
    F: for<'tcx> FnOnce(TyCtxt<'tcx>) -> T + Send,
    T: Send,
{
    let mut output: Option<T> = None;
    let output_ref = &mut output;
    run_compiler_for_testing_impl(
        source.into(),
        Box::new(move |tcx| {
            *output_ref = Some(callback(tcx));
        }),
    );
    output.unwrap()
}

static USING_INTERNAL_FEATURES: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

const TEST_FILENAME: &str = "crubit_unittests.rs";

fn rustc_interface_config(opts: Options, source: String) -> rustc_interface::interface::Config {
    rustc_interface::interface::Config {
        opts,
        crate_cfg: Default::default(),
        crate_check_cfg: Default::default(),
        input: Input::Str {
            name: rustc_span::FileName::Custom(TEST_FILENAME.to_string()),
            input: source,
        },
        output_file: None,
        output_dir: None,
        file_loader: None,
        lint_caps: Default::default(),
        psess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        track_state: None,
        ice_file: None,
        extra_symbols: vec![],
        using_internal_features: &USING_INTERNAL_FEATURES,
    }
}

fn run_full_analysis(tcx: TyCtxt<'_>) {
    rustversion::select! {
        before(2026-11-23) => tcx.ensure_ok().analysis(()),
        _ => rustc_interface::passes::full_analysis(tcx),
    }
}

/// A non-generic implementation of `run_compiler_for_testing`.
///
/// This is used to ensure that the body of `run_compiler_for_testing` is not recompiled for every
/// invocation. This saves some targets
/// (e.g. `//cc_bindings_from_rs/generate_bindings:bindings_test`)
/// several minutes of compilation time.
#[allow(rustc::internal)]
fn run_compiler_for_testing_impl(
    source: String,
    callback: Box<dyn for<'tcx> FnOnce(TyCtxt<'tcx>) + Send + '_>,
) {
    // Setting `output_types` that will trigger code gen - otherwise some parts of
    // the analysis will be missing (e.g. `tcx.exported_symbols()`).
    // The choice of `Bitcode` is somewhat arbitrary (e.g. `Assembly`,
    // `Mir`, etc. would also trigger code gen).
    let output_types = OutputTypes::new(&[(OutputType::Bitcode, None /* PathBuf */)]);

    let mut opts = Options {
        crate_types: vec![CrateType::Rlib], // Test inputs simulate library crates.
        sysroot: Sysroot::new(sysroot_path()),
        output_types,
        edition: rustc_span::edition::Edition::Edition2021,
        unstable_features: rustc_feature::UnstableFeatures::Allow,
        lint_opts: vec![
            ("warnings".to_string(), rustc_lint_defs::Level::Deny),
            ("stable_features".to_string(), rustc_lint_defs::Level::Allow),
        ],
        ..Default::default()
    };

    // Needed for using target.json; avoids:
    // error loading target specification: custom targets are unstable and require
    // `-Zunstable-options`
    //
    // TODO: use `Session::unstable_options` instead of
    // `unstable_opts.unstable_options` and remove the function #[allow(rustc::internal)].
    opts.unstable_opts.unstable_options = true;
    // Ignore target modifiers; see `run_compiler::target_modifier_flag_names`.
    opts.cg.unsafe_allow_abi_mismatch = run_compiler::target_modifier_flag_names();

    let target_dir = tempfile::TempDir::new().unwrap();

    if let Some(target) = &setup_rustc_target_for_testing(target_dir.path()) {
        let target_path = &Path::new(target);
        opts.target_triple = TargetTuple::from_path(target_path).unwrap_or_else(|_| {
            panic!("failed to construct a TargetTuple from target: '{}'", target_path.display())
        });
    }

    let config = rustc_interface_config(opts, source);

    rustc_interface::interface::run_compiler(config, |compiler| {
        use std::panic::{catch_unwind, AssertUnwindSafe};
        let sess = &compiler.sess;
        let krate = catch_unwind(AssertUnwindSafe(|| rustc_interface::passes::parse(sess)))
            .expect("Test input compilation failed while parsing");

        let result = rustc_interface::create_and_enter_global_ctxt(compiler, krate, |tcx| {
            if sess.dcx().has_errors().is_some() {
                sess.dcx().fatal("Compilation failed, aborting");
            }
            // Explicitly force full `analysis` stage to detect compilation
            // errors that the earlier stages might miss.  This helps ensure that the
            // test inputs are valid Rust (even if `callback` wouldn't
            // have triggered full analysis).
            catch_unwind(AssertUnwindSafe(|| run_full_analysis(tcx)))
                .expect("Test input compilation failed while analyzing");
            callback(tcx)
        });

        // TODO(b/390671870): somehow this is failing to reject warnings.
        if let Some(_error_guaranteed) = compiler.sess.dcx().has_errors() {
            panic!("Test input compilation failed while linting")
        }
        result
    });
}

/// Finds the definition id of a Rust item with the specified `name`.
/// Panics if no such item is found, or if there is more than one match.
pub fn find_def_id_by_name(tcx: TyCtxt, name: &str) -> LocalDefId {
    let hir_items = || tcx.hir_free_items().map(|item_id| tcx.hir_item(item_id));
    let items_with_matching_name = hir_items()
        .filter(|item| item.kind.ident().map_or(false, |ident| ident.name.as_str() == name))
        .collect_vec();
    match *items_with_matching_name.as_slice() {
        [] => {
            let found_names = hir_items()
                .map(|item| item.kind.ident())
                .filter(|ident| ident.is_some())
                .map(|ident| ident.unwrap().name.as_str().to_owned())
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
