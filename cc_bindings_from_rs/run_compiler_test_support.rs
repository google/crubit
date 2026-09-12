// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! A wrapper around `run_compiler` for testing only.

#![feature(rustc_private)]
#![allow(unexpected_cfgs)]

extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_feature;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint_defs;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
use itertools::Itertools;

use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::DefId;

use std::path::Path;
use std::path::PathBuf;

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
        const TOOLCHAIN_ROOT: &str = "rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools/rust_toolchain/lib/rustlib/x86_64-unknown-linux-gnu";
        #[cfg(not(oss))]
        const TOOLCHAIN_ROOT: &str = env!("G3_SYSROOT_PATH");
        let runfiles = runfiles::Runfiles::create().unwrap();
        let loc = runfiles.rlocation(Path::new(TOOLCHAIN_ROOT)).expect("Failed to locate runfile");
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
/// * Hardcodes other compiler flags (e.g. picks Rust 2021 edition, and opts
///   into treating all warnings as errors).
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
    let target_dir = tempfile::TempDir::new().unwrap();
    let src_path = target_dir.path().join("rust_out.rs");
    std::fs::write(&src_path, &source).expect("Failed to write test source file");

    let rmeta_path = target_dir.path().join("librust_out.rmeta");

    let target = setup_rustc_target_for_testing(target_dir.path());

    // Stage 1: Compile the test input to an .rmeta file.
    let mut stage1_args = vec![
        "rustc".to_string(),
        src_path.display().to_string(),
        format!("--remap-path-prefix={}=<crubit_unittests.rs>", src_path.display()),
        "--crate-name=rust_out".to_string(),
        "--crate-type=lib".to_string(),
        "--emit=metadata".to_string(),
        "-o".to_string(),
        rmeta_path.display().to_string(),
        "--edition=2021".to_string(),
        "-Dwarnings".to_string(),
        "-Astable_features".to_string(),
        "-Aunused".to_string(),
        "-Aunused_features".to_string(),
    ];
    if let Some(sysroot) = sysroot_path() {
        stage1_args.push(format!("--sysroot={}", sysroot.display()));
    }
    if let Some(target) = &target {
        stage1_args.push(format!("--target={target}"));
        stage1_args.push("-Zunstable-options".to_string());
    }

    struct Stage1Callbacks;
    impl rustc_driver::Callbacks for Stage1Callbacks {}

    let stage1_res = rustc_driver::catch_fatal_errors(|| {
        rustc_driver::run_compiler(&stage1_args, &mut Stage1Callbacks);
    });

    if stage1_res.is_err() || !rmeta_path.exists() {
        panic!("Test input compilation failed");
    }

    // Stage 2: Analyze a stub crate importing the metadata.
    let mut stage2_args = vec![
        "rustc".to_string(),
        "unused_input.rs".to_string(),
        "--crate-name=rust_out_wrapper".to_string(),
        "--crate-type=lib".to_string(),
        format!("--extern=rust_out={}", rmeta_path.display()),
        "--edition=2021".to_string(),
    ];
    if let Some(sysroot) = sysroot_path() {
        stage2_args.push(format!("--sysroot={}", sysroot.display()));
    }
    if let Some(target) = &target {
        stage2_args.push(format!("--target={target}"));
        stage2_args.push("-Zunstable-options".to_string());
    }

    let stub_input = "extern crate rust_out;\n".to_string();

    let stage2_res =
        run_compiler::run_compiler_with_input(&stage2_args, stub_input, |tcx| Ok(callback(tcx)));

    if let Err(err) = stage2_res {
        panic!("Failed in stage 2 compilation: {err:#}");
    }
}

/// Finds the definition id of a Rust item with the specified `name`.
/// Panics if no such item is found, or if there is more than one match.
pub fn find_def_id_by_name(tcx: TyCtxt, name: &str) -> DefId {
    let mut matches = Vec::new();
    let mut all_names = Vec::new();

    fn search_module(
        tcx: TyCtxt,
        mod_def_id: DefId,
        target_name: &str,
        matches: &mut Vec<DefId>,
        all_names: &mut Vec<String>,
    ) {
        let children: &[rustc_middle::metadata::ModChild] =
            if let Some(local_def_id) = mod_def_id.as_local() {
                tcx.module_children_local(local_def_id)
            } else {
                tcx.module_children(mod_def_id)
            };
        for child in children {
            let child_name = child.ident.name.as_str();
            if !child_name.is_empty() {
                all_names.push(child_name.to_string());
            }
            if let Some(def_id) = child.res.opt_def_id() {
                let def_kind = tcx.def_kind(def_id);
                if matches!(def_kind, rustc_hir::def::DefKind::Ctor(..)) {
                    continue;
                }
                if child_name == target_name {
                    matches.push(def_id);
                }
                if matches!(
                    def_kind,
                    rustc_hir::def::DefKind::Struct
                        | rustc_hir::def::DefKind::Enum
                        | rustc_hir::def::DefKind::Union
                ) {
                    for &impl_def_id in tcx.inherent_impls(def_id) {
                        for &assoc_id in tcx.associated_item_def_ids(impl_def_id) {
                            let sym = tcx.item_name(assoc_id);
                            let assoc_name = sym.as_str();
                            if !assoc_name.is_empty() {
                                all_names.push(assoc_name.to_string());
                            }
                            if assoc_name == target_name {
                                matches.push(assoc_id);
                            }
                        }
                    }
                } else if def_kind == rustc_hir::def::DefKind::Trait {
                    for &assoc_id in tcx.associated_item_def_ids(def_id) {
                        let sym = tcx.item_name(assoc_id);
                        let assoc_name = sym.as_str();
                        if !assoc_name.is_empty() {
                            all_names.push(assoc_name.to_string());
                        }
                        if assoc_name == target_name {
                            matches.push(assoc_id);
                        }
                    }
                } else if def_kind == rustc_hir::def::DefKind::Mod
                    && def_id.krate == mod_def_id.krate
                {
                    search_module(tcx, def_id, target_name, matches, all_names);
                }
            }
        }
    }

    for krate in
        tcx.used_crates(()).iter().copied().chain(std::iter::once(rustc_span::def_id::LOCAL_CRATE))
    {
        if krate != rustc_span::def_id::LOCAL_CRATE && tcx.crate_name(krate).as_str() != "rust_out"
        {
            continue;
        }
        if krate == rustc_span::def_id::LOCAL_CRATE
            && tcx.crate_name(krate).as_str() == "rust_out_wrapper"
        {
            continue;
        }
        search_module(tcx, krate.as_def_id(), name, &mut matches, &mut all_names);
    }

    matches.dedup();
    match matches.as_slice() {
        [] => {
            let found_names =
                all_names.into_iter().sorted().dedup().map(|n| format!("`{n}`")).join(",\n");
            panic!("No items named `{name}`.\nInstead found:\n{found_names}");
        }
        [def_id] => *def_id,
        _ => panic!("More than one item named `{name}`: {matches:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Test input compilation failed")]
    fn test_run_compiler_for_testing_panic_when_test_input_contains_syntax_errors() {
        run_compiler_for_testing("syntax error here", |_tcx| panic!("This part shouldn't execute"))
    }

    #[test]
    #[should_panic(expected = "Test input compilation failed")]
    fn test_run_compiler_for_testing_panic_when_test_input_triggers_analysis_errors() {
        run_compiler_for_testing("#![feature(no_such_feature)]", |_tcx| {
            panic!("This part shouldn't execute")
        })
    }

    // TODO(b/390671870): when rejecting warnings is fixed, enable this test.
    /*#[test]
    #[should_panic(expected = "Test input compilation failed while linting")]
    fn test_run_compiler_for_testing_panic_when_test_input_triggers_warnings() {
        run_compiler_for_testing("pub fn foo(unused_parameter: i32) {}", |_tcx| {
            panic!("This part shouldn't execute")
        })
    }*/

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
