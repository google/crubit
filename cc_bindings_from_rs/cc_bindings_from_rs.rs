// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(never_type)]
#![feature(rustc_private)]
#![deny(rustc::internal)]

extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

use rustc_errors::registry;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::{self as config, ErrorOutputType, Sysroot, UnstableOptions};
use rustc_session::search_paths::SearchPath;
use rustc_session::EarlyDiagCtxt;

use arc_anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use cmdline::Cmdline;
use code_gen_utils::CcInclude;
use error_report::{ErrorReport, ErrorReporting, FatalErrors, ReportFatalError};
use generate_bindings::{Database, IncludeGuard};
use kythe_metadata::cc_embed_provenance_map;
use run_compiler::run_compiler;
use token_stream_printer::{
    cc_tokens_to_formatted_string, cc_tokens_to_formatted_string_with_provenance,
    rs_tokens_to_formatted_string, RustfmtConfig,
};

fn turn_off_clang_format(mut cc_api: String) -> String {
    cc_api.insert_str(
        cc_api.find("#ifndef").unwrap_or_else(|| cc_api.find("#pragma once").unwrap()),
        "// clang-format off\n",
    );
    cc_api
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    std::fs::write(path, content)
        .with_context(|| format!("Error when writing to {}", path.display()))
}

fn new_db<'tcx>(
    cmdline: &Cmdline,
    tcx: TyCtxt<'tcx>,
    errors: Rc<dyn ErrorReporting>,
    fatal_errors: Rc<dyn ReportFatalError>,
) -> Database<'tcx> {
    let mut crate_name_to_include_paths = <HashMap<Rc<str>, Vec<CcInclude>>>::new();
    for (crate_name, include_path) in &cmdline.crate_headers {
        let paths = crate_name_to_include_paths.entry(crate_name.as_str().into()).or_default();
        paths.push(CcInclude::user_header(include_path.as_str().into()));
    }

    let mut crate_name_to_features =
        <HashMap<Rc<str>, flagset::FlagSet<crubit_feature::CrubitFeature>>>::new();
    for (crate_name, features) in &cmdline.crate_features {
        let accumulated_features = crate_name_to_features
            .entry(crate_name.as_str().into())
            .or_insert(cmdline.default_crate_features);
        *accumulated_features |= *features
    }
    for (crate_name, features) in &cmdline.crate_disabled_features {
        let accumulated_features = crate_name_to_features
            .entry(crate_name.as_str().into())
            .or_insert(cmdline.default_crate_features);
        *accumulated_features -= *features
    }
    let include_guard = if let Some(include_guard) = &cmdline.h_out_include_guard {
        IncludeGuard::Guard(include_guard.clone())
    } else {
        IncludeGuard::PragmaOnce
    };
    let mut crate_name_to_namespace = <HashMap<Rc<str>, Rc<str>>>::new();
    for (crate_name, namespace) in &cmdline.crate_namespaces {
        // TODO: Check dup.
        crate_name_to_namespace.insert(crate_name.as_str().into(), namespace.as_str().into());
    }
    let mut crate_renames = <HashMap<Rc<str>, Rc<str>>>::new();
    for (name, renamed) in &cmdline.crate_rename {
        crate_renames.insert(name.as_str().into(), renamed.as_str().into());
    }
    generate_bindings::new_database(
        tcx,
        cmdline.source_crate_name.as_ref().map(|s| s.clone().into()),
        cmdline.crubit_support_path_format.clone(),
        cmdline.crubit_debug_path_format.clone(),
        cmdline.default_crate_features,
        cmdline.enable_hir_types,
        cmdline.kythe_annotations,
        cmdline.enable_rmeta_interface,
        crate_name_to_include_paths.into(),
        crate_name_to_features.into(),
        crate_name_to_namespace.into(),
        crate_renames.into(),
        errors,
        fatal_errors,
        cmdline.no_thunk_name_mangling,
        include_guard,
    )
}

fn run_with_tcx(cmdline: &Cmdline, tcx: TyCtxt) -> Result<()> {
    use generate_bindings::{generate_bindings, BindingsTokens};

    let generate_error_report = cmdline.error_report_out.is_some();
    let (error_report, errors) = ErrorReport::new_rc_or_ignore(generate_error_report);
    let fatal_errors = Rc::new(FatalErrors::new());

    let BindingsTokens { cc_api, cc_api_impl } = {
        let db = new_db(cmdline, tcx, errors, fatal_errors.clone());
        generate_bindings(&db)?
    };

    let fatal_error_message = fatal_errors.take_string();
    if !fatal_error_message.is_empty() {
        return Err(arc_anyhow::Error::msg(fatal_error_message));
    }

    if cmdline.kythe_annotations {
        let (cc_api, provenance_map) = cc_tokens_to_formatted_string_with_provenance(
            cc_api,
            cmdline.clang_format_exe_path.as_deref(),
        )?;
        let cc_api = turn_off_clang_format(cc_api);
        let cc_api = cc_embed_provenance_map(
            &provenance_map,
            cmdline
                .kythe_default_corpus
                .as_deref()
                .expect("kythe_default_corpus is required when kythe_annotations is enabled"),
            cc_api,
        );
        write_file(&cmdline.h_out, &cc_api)?;
    } else {
        let cc_api =
            cc_tokens_to_formatted_string(cc_api, cmdline.clang_format_exe_path.as_deref())?;
        let cc_api = turn_off_clang_format(cc_api);
        write_file(&cmdline.h_out, &cc_api)?;
    }

    {
        let rustfmt_config = cmdline.rustfmt_exe_path.as_ref().map(|rustfmt_path| {
            RustfmtConfig::new(rustfmt_path, cmdline.rustfmt_config_path.as_deref())
        });
        let cc_api_impl = rs_tokens_to_formatted_string(cc_api_impl, rustfmt_config.as_ref())?;
        write_file(&cmdline.rs_out, &cc_api_impl)?;
    }

    if let Some(error_report_out) = &cmdline.error_report_out {
        write_file(error_report_out, &error_report.unwrap().to_json_string())?;
    }

    Ok(())
}

/// Generates bindings using rmeta input files rather than source files and rustc args.
/// This function exists so we can guard it behind a feature flag and should be cleaned up once
/// we've migrated to rmetas entirely.
fn run_with_rmetas(cmdline: &Cmdline) -> Result<()> {
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    rustc_driver::init_rustc_env_logger(&early_dcx);
    let at_args = &cmdline.rustc_args;
    let at_args = at_args.get(1..).unwrap_or_default();

    let mut at_args = at_args.to_owned();
    for (crate_name, metadata) in &cmdline.r#extern {
        at_args.push(format!("--extern={crate_name}={metadata}"));
    }

    if let Some(target) = &cmdline.target {
        at_args.push(format!("--target={}", target));
    }

    if let Some(sysroot) = &cmdline.sysroot {
        at_args.push(format!("--sysroot={}", sysroot));
    }

    for library_path in &cmdline.library_dirs {
        at_args.push(format!("-L={}", library_path));
    }

    let Some(matches) = rustc_driver::handle_options(&early_dcx, &at_args) else {
        bail!("rustc failed to parse provided arguments")
    };

    // We do not support unstable options at the moment.
    // We just need an instance of this struct so we can reuse the logic in rustc rather than
    // duplicate it.
    let unstable_opts = UnstableOptions::default();

    let externs = config::parse_externs(&early_dcx, &matches, &unstable_opts);

    let sysroot = Sysroot::new(matches.opt_str("sysroot").map(PathBuf::from));
    let target_triple = config::parse_target_triple(&early_dcx, &matches);

    // We eagerly scan all files in each passed -L path. If the same directory is passed multiple
    // times, and the directory contains a lot of files, this can take a lot of time.
    // So we remove -L paths that were passed multiple times, and keep only the first occurrence.
    // We still have to keep the original order of the -L arguments.
    let search_paths: Vec<SearchPath> = {
        let mut seen_search_paths: HashSet<&String> = HashSet::default();
        let search_path_matches: Vec<String> = matches.opt_strs("L");
        search_path_matches
            .iter()
            .filter(|p| seen_search_paths.insert(*p))
            .map(|path| {
                SearchPath::from_cli_opt(
                    sysroot.path(),
                    &target_triple,
                    &early_dcx,
                    &path,
                    unstable_opts.unstable_options,
                )
            })
            .collect()
    };

    let Some(crate_name) = cmdline.source_crate_name.as_ref() else {
        bail!("--source-crate-name must be provided when using rmetas")
    };

    let input = config::Input::Str {
        name: rustc_span::FileName::Custom("main.rs".into()),
        // This tells rustc to load our crate from it's rmeta.
        input: format!(
            r#"
extern crate r#{};

fn main() {{ }}
"#,
            crate_name
        )
        .into(),
    };
    let config = rustc_interface::Config {
        // Command line options
        opts: config::Options {
            externs,
            // Fix this up
            search_paths,
            target_triple,
            ..config::Options::default()
        },
        // cfg! configuration in addition to the default ones
        crate_cfg: Vec::new(),       // FxHashSet<(String, Option<String>)>
        crate_check_cfg: Vec::new(), // CheckCfg
        input,
        output_dir: None,  // Option<PathBuf>
        output_file: None, // Option<PathBuf>
        file_loader: None, // Option<Box<dyn FileLoader + Send + Sync>>
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES.to_owned(),
        lint_caps: Default::default(), // FxHashMap<lint::LintId, lint::Level>
        // This is a callback from the driver that is called when [`ParseSess`] is created.
        psess_created: None, //Option<Box<dyn FnOnce(&mut ParseSess) + Send>>
        // This is a callback from the driver that is called when we're registering lints;
        // it is called during plugin registration when we have the LintStore in a non-shared state.
        //
        // Note that if you find a Some here you probably want to call that function in the new
        // function being registered.
        register_lints: None, // Option<Box<dyn Fn(&Session, &mut LintStore) + Send + Sync>>
        // This is a callback from the driver that is called just after we have populated
        // the list of queries.
        //
        // The second parameter is local providers and the third parameter is external providers.
        override_queries: None, // Option<fn(&Session, &mut ty::query::Providers<'_>, &mut ty::query::Providers<'_>)>
        // Registry of diagnostics codes.
        registry: registry::Registry::new(rustc_errors::codes::DIAGNOSTICS),
        make_codegen_backend: None,
        ice_file: None,
        hash_untracked_state: None,
        using_internal_features: &rustc_driver::USING_INTERNAL_FEATURES,
        extra_symbols: vec![],
    };
    rustc_interface::run_compiler(config, |compiler| {
        // Parse the program and print the syntax tree.
        let krate = rustc_interface::passes::parse(&compiler.sess);
        // Analyze the program and inspect the types of definitions.
        rustc_interface::create_and_enter_global_ctxt(&compiler, krate, |tcx| {
            // Make sure name resolution and macro expansion is run.
            let _ = tcx.resolver_for_lowering();

            tcx.ensure_ok().analysis(());
            // Now that we've run analysis to prime the compiler, run our callback.
            run_with_tcx(&cmdline, tcx).expect("Binding generation failed")
        });
    });
    Ok(())
}

/// Main entrypoint that (unlike `main`) doesn't do any intitializations that
/// should only happen once for the binary (e.g. it doesn't call
/// `init_env_logger`) and therefore can be used from the tests module below.
fn run_with_cmdline_args(raw_args: &[String]) -> Result<()> {
    let cmdline = Cmdline::new(raw_args)?;
    if cmdline.enable_rmeta_interface {
        run_with_rmetas(&cmdline)
    } else {
        run_compiler(&cmdline.rustc_args, |tcx| run_with_tcx(&cmdline, tcx))
    }
}

fn main() -> Result<()> {
    // TODO: Investigate if we should install a signal handler here.  See also how
    // compiler/rustc_driver/src/lib.rs calls `signal_handler::install()`.

    // TODO(b/254689400): Provide Crubit-specific panic hook message (we shouldn't
    // use `rustc_driver::install_ice_hook` because it's message asks to file
    // bugs at https://github.com/rust-lang/rust/issues/new.

    // `std::env::args()` will panic if any of the cmdline arguments are not valid
    // Unicode.  This seems okay.
    let args = std::env::args().collect_vec();

    run_with_cmdline_args(&args).map_err(|err| match err.downcast_ref::<clap::Error>() {
        // Explicitly call `clap::Error::exit`, because 1) it results in *colored* output and
        // 2) it uses a zero exit code for specific "errors" (e.g. for `--help` output).
        Some(clap_err) => {
            let _: ! = clap_err.exit();
        }

        // Return `err` from `main`.  This will print the error message (no color codes
        // though) and terminate the process with a non-zero exit code.
        None => err,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use external_binaries::{CLANG_FORMAT_EXE_PATH, RUSTFMT_EXE_PATH};
    use itertools::Itertools;
    use regex::{Regex, RegexBuilder};
    use run_compiler_test_support::setup_rustc_target_for_testing;
    use run_compiler_test_support::sysroot_path;
    use std::path::PathBuf;
    use tempfile::{tempdir, TempDir};

    /// Test data builder (see also
    /// https://testing.googleblog.com/2018/02/testing-on-toilet-cleanly-create-test.html).
    struct TestArgs {
        rs_input: Option<String>,
        h_path: Option<String>,
        error_report_out: Option<String>,
        extra_crubit_args: Vec<String>,

        /// Arg for the following `rustc` flag: `--codegen=panic=<arg>`.
        panic_mechanism: String,

        /// Other `rustc` flags.
        extra_rustc_args: Vec<String>,

        tempdir: TempDir,
        include_guard: Option<String>,
    }

    /// Result of `TestArgs::run` that helps tests access test outputs (e.g. the
    /// internally generated `h_path` and/or `rs_input_path`).
    #[derive(Debug)]
    struct TestResult {
        h_path: PathBuf,
        rs_path: PathBuf,
        error_report_out_path: Option<PathBuf>,
    }

    impl TestArgs {
        fn default_args() -> Result<Self> {
            Ok(Self {
                rs_input: None,
                h_path: None,
                error_report_out: None,
                extra_crubit_args: vec![],
                panic_mechanism: "abort".to_string(),
                extra_rustc_args: vec![],
                tempdir: tempdir()?,
                include_guard: None,
            })
        }

        /// Use the specified `h_path` rather than auto-generating one in
        /// `self`-managed temporary directory.
        fn with_h_path(mut self, h_path: &str) -> Self {
            self.h_path = Some(h_path.to_string());
            self
        }

        /// Specify the path to the error report output file.
        fn with_error_report_out(mut self, error_report_out: &str) -> Self {
            self.error_report_out = Some(error_report_out.to_string());
            self
        }

        /// Specify the test Rust input.
        fn with_rs_input(mut self, rs_input: &str) -> Self {
            self.rs_input = Some(rs_input.to_string());
            self
        }

        /// Replaces the default `--codegen=panic=abort` with the specified
        /// `panic_mechanism`.
        fn with_panic_mechanism(mut self, panic_mechanism: &str) -> Self {
            self.panic_mechanism = panic_mechanism.to_string();
            self
        }

        /// Appends `extra_rustc_args` at the end of the cmdline (i.e. as
        /// additional rustc args, in addition to `--sysroot`,
        /// `--crate-type=...`, etc.).
        fn with_extra_rustc_args(mut self, extra_rustc_args: &[&str]) -> Self {
            self.extra_rustc_args = extra_rustc_args.iter().map(|t| t.to_string()).collect_vec();
            self
        }

        /// Appends `extra_crubit_args` before the first `--`.
        fn with_extra_crubit_args(mut self, extra_crubit_args: &[&str]) -> Self {
            self.extra_crubit_args = extra_crubit_args.iter().map(|t| t.to_string()).collect_vec();
            self
        }

        fn with_include_guard(mut self, include_guard: &str) -> Self {
            self.include_guard = Some(include_guard.to_string());
            self
        }

        /// Invokes `super::run_with_cmdline_args` with default `test_crate.rs`
        /// input (and with other default args + args gathered by
        /// `self`).
        ///
        /// Returns the path to the `h_out` file.  The file's lifetime is the
        /// same as `&self`.
        fn run(&self) -> Result<TestResult> {
            let h_path = match self.h_path.as_ref() {
                None => self.tempdir.path().join("test_crate_cc_api.h"),
                Some(s) => PathBuf::from(s),
            };

            let rs_path = self.tempdir.path().join("test_crate_cc_api_impl.rs");

            let rs_input_path = self.tempdir.path().join("test_crate.rs");
            let rs_input = if let Some(rs_input) = &self.rs_input {
                rs_input
            } else {
                r#" pub mod public_module {
                        pub fn public_function() {
                            private_function()
                        }

                        fn private_function() {}
                    }
                "#
            };
            std::fs::write(&rs_input_path, rs_input)?;

            let mut args = vec![
                "cc_bindings_from_rs_unittest_executable".to_string(),
                format!("--h-out={}", h_path.display()),
                format!("--rs-out={}", rs_path.display()),
                "--crubit-support-path-format=<crubit/support/{header}>".to_string(),
                format!("--clang-format-exe-path={CLANG_FORMAT_EXE_PATH}"),
                format!("--rustfmt-exe-path={RUSTFMT_EXE_PATH}"),
            ];

            let mut error_report_out_path = None;
            if let Some(error_report_out) = self.error_report_out.as_ref() {
                error_report_out_path = Some(self.tempdir.path().join(error_report_out));
                args.push(format!(
                    "--error-report-out={}",
                    error_report_out_path.as_ref().unwrap().display()
                ));
            }

            if let Some(include_guard) = &self.include_guard {
                args.push(format!("--h-out-include-guard={include_guard}"));
            }

            args.extend(self.extra_crubit_args.iter().cloned());
            args.extend([
                "--".to_string(),
                format!("--codegen=panic={}", &self.panic_mechanism),
                "--crate-type=lib".to_string(),
                rs_input_path.display().to_string(),
            ]);

            if let Some(sysroot) = sysroot_path() {
                args.push(format!("--sysroot={}", sysroot.display()));
            }

            if let Some(target) = &setup_rustc_target_for_testing(self.tempdir.path()) {
                args.push(format!("--target={}", target));
            }

            args.extend(self.extra_rustc_args.iter().cloned());

            run_with_cmdline_args(&args)?;

            Ok(TestResult { h_path, rs_path, error_report_out_path })
        }
    }

    // TODO(lukasza): b/261074843 - Go back to exact string matching (and hardcoding thunk
    // names) once we are using stable name mangling.
    // ("Go back" = more or less revert cl/492292910 + manual review and
    // tweaks.)
    fn assert_body_matches(actual: &str, expected: &str) {
        fn build_regex(expected_body: &str) -> Regex {
            let patt = regex::escape(expected_body);
            let patt = format!("^{patt}"); // Not always matching $ enables prefix checks below.
            let patt = patt.replace("ANY_IDENTIFIER_CHARACTERS", "[a-zA-Z0-9_]*");
            RegexBuilder::new(&patt).multi_line(false).dot_matches_new_line(false).build().unwrap()
        }
        let is_whole_h_body_matching = {
            match build_regex(expected).shortest_match(&actual) {
                None => false,
                Some(len) => len == actual.len(),
            }
        };
        if !is_whole_h_body_matching {
            let longest_matching_expectation_len = (0..=expected.len())
                .rev() // Iterating from longest to shortest prefix
                .filter(|&len| {
                    expected
                        .get(0..len) // Only valid UTF-8 boundaries
                        .filter(|prefix| build_regex(prefix).is_match(&actual))
                        .is_some()
                })
                .next() // Getting the first regex that matched
                .unwrap(); // We must get a match at least for 0-length expected body
            let longest_matching_regex =
                build_regex(&expected[0..longest_matching_expectation_len]);
            let len_of_longest_match = longest_matching_regex.shortest_match(&actual).unwrap(); // Again - we must get a match at least for 0-length expected body
            let mut marked_body = actual.to_string();
            marked_body.insert_str(len_of_longest_match, "!!!>>>");
            let mut marked_pattern = expected.to_string();
            marked_pattern.insert_str(longest_matching_expectation_len, "!!!>>>");
            panic!(
                "Mismatched expectations:\n\
                    #### Actual body (first mismatch follows the \"!!!>>>\" marker):\n\
                    {marked_body}\n\
                    #### Mismatched pattern (mismatch follows the \"!!!>>>\" marker):\n\
                    {marked_pattern}"
            );
        }
    }

    // TODO: b/455963829 - Replace this with golden test so it's easier to update.
    #[test]
    fn test_error_reporting_generation() -> Result<()> {
        let test_args =
            TestArgs::default_args()?.with_error_report_out("error_report.json").with_rs_input(
                r#"
                pub struct Unsupported<T> {
                    pub field: T,
                }
                "#,
            );

        let test_result = test_args.run().expect("Error report generation should succeed");
        assert!(test_result.error_report_out_path.is_some());
        let error_report_out_path = test_result.error_report_out_path.as_ref().unwrap();
        assert!(error_report_out_path.exists());
        let error_report = std::fs::read_to_string(&error_report_out_path)?;
        let expected_error_report = r#"{
  "Generic types are not supported yet (b/259749095)": {
    "count": 1
  }
}"#;
        assert_eq!(expected_error_report, error_report);
        Ok(())
    }

    #[test]
    fn test_with_include_guard() -> Result<()> {
        let test_args =
            TestArgs::default_args()?.with_include_guard("CRUBIT_GENERATED_HEADER_FOR_test_crate_");
        let test_result = test_args.run().expect("Customized include guard should succeed");

        assert!(test_result.h_path.exists());
        let temp_dir_str = test_args.tempdir.path().to_str().unwrap();
        let cc_api = std::fs::read_to_string(&test_result.h_path)?;
        #[rustfmt::skip]
        assert_body_matches(
            &cc_api,
            &format!(
                "{}\n{}\n{}",
r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate
// Features: <none>

// clang-format off
#ifndef CRUBIT_GENERATED_HEADER_FOR_test_crate_
#define CRUBIT_GENERATED_HEADER_FOR_test_crate_

namespace test_crate {

namespace public_module {
"#,
format!("// Generated from: {temp_dir_str}/test_crate.rs;l=2"),
r#"void public_function();

namespace __crubit_internal {
extern "C" void
__crubit_thunk_ANY_IDENTIFIER_CHARACTERS();
}
inline void public_function() {
  return __crubit_internal::
      __crubit_thunk_ANY_IDENTIFIER_CHARACTERS();
}

}  // namespace public_module

}  // namespace test_crate
#endif  // CRUBIT_GENERATED_HEADER_FOR_test_crate_
"#
            ),
        );
        Ok(())
    }

    #[test]
    fn test_happy_path() -> Result<()> {
        let test_args = TestArgs::default_args()?;
        let test_result = test_args.run().expect("Default args should succeed");

        assert!(test_result.h_path.exists());
        let temp_dir_str = test_args.tempdir.path().to_str().unwrap();
        let cc_api = std::fs::read_to_string(&test_result.h_path)?;
        #[rustfmt::skip]
        assert_body_matches(
            &cc_api,
            &format!(
                "{}\n{}\n{}",
r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate
// Features: <none>

// clang-format off
#pragma once

namespace test_crate {

namespace public_module {
"#,
format!("// Generated from: {temp_dir_str}/test_crate.rs;l=2"),
r#"void public_function();

namespace __crubit_internal {
extern "C" void
__crubit_thunk_ANY_IDENTIFIER_CHARACTERS();
}
inline void public_function() {
  return __crubit_internal::
      __crubit_thunk_ANY_IDENTIFIER_CHARACTERS();
}

}  // namespace public_module

}  // namespace test_crate
"#
            ),
        );

        assert!(test_result.rs_path.exists());
        let cc_api_impl = std::fs::read_to_string(&test_result.rs_path)?;
        assert_body_matches(
            &cc_api_impl,
            r#"// Automatically @generated C++ bindings for the following Rust crate:
// test_crate
// Features: <none>

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ANY_IDENTIFIER_CHARACTERS()
-> () {
    unsafe { ::test_crate::public_module::public_function() }
}
"#,
        );
        Ok(())
    }

    /// `test_cmdline_error_propagation` tests that errors from `Cmdline::new`
    /// get propagated. More detailed test coverage of various specific
    /// error types can be found in tests in `cmdline.rs`.
    #[test]
    fn test_cmdline_error_propagation() -> Result<()> {
        let err = TestArgs::default_args()?
            .with_extra_crubit_args(&["--unrecognized-crubit-flag"])
            .run()
            .expect_err("--unrecognized_crubit_flag should trigger an error");

        let msg = format!("{err:#}");
        assert!(msg.contains("unexpected argument '--unrecognized-crubit-flag'"), "msg = {}", msg,);
        Ok(())
    }

    /// `test_run_compiler_error_propagation` tests that errors from
    /// `run_compiler` get propagated. More detailed test coverage of
    /// various specific error types can be found in tests in `run_compiler.
    /// rs`.
    #[test]
    fn test_run_compiler_error_propagation() -> Result<()> {
        let err = TestArgs::default_args()?
            .with_extra_rustc_args(&["--unrecognized-rustc-flag"])
            .run()
            .expect_err("--unrecognized-rustc-flag should trigger an error");

        let msg = format!("{err:#}");
        assert_eq!("Errors reported by Rust compiler.", msg);
        Ok(())
    }

    /// `test_rustc_with_panic_unwind` tests that `panic=unwind`
    /// is supported at least nominally.
    ///
    /// This is tested at the `cc_bindings_from_rs.rs` level instead of at the
    /// `bindings.rs` level, because `run_compiler_test_support` doesn't
    /// support specifying a custom panic mechanism.
    #[test]
    fn test_rustc_with_panic_unwind() -> Result<()> {
        let _ = TestArgs::default_args()?
            .with_panic_mechanism("unwind")
            .run()
            .expect("panic=unwind should not cause an error");
        Ok(())
    }

    #[test]
    fn test_rustc_with_panic_abort() -> Result<()> {
        let _ = TestArgs::default_args()?
            .with_panic_mechanism("abort")
            .run()
            .expect("panic=abort should not cause an error");
        Ok(())
    }

    /// `test_invalid_h_out_path` tests not only the specific problem of an
    /// invalid `--h-out` argument, but also tests that errors from
    /// `run_with_tcx` are propagated.
    #[test]
    fn test_invalid_h_out_path() -> Result<()> {
        let err = TestArgs::default_args()?
            .with_h_path("../..")
            .run()
            .expect_err("Unwriteable --h-out should trigger an error");

        let msg = format!("{err:#}");
        assert_eq!("Error when writing to ../..: Is a directory (os error 21)", msg);
        Ok(())
    }

    fn assert_starts_with(data: &str, expected_prefix: &str) {
        assert!(
            data.starts_with(expected_prefix),
            "expected prefix:\n{expected_prefix}\n\nbut got:\n{data}"
        );
    }

    #[test]
    fn test_crate_features() -> Result<()> {
        let test_args = TestArgs::default_args()?.with_extra_crubit_args(&[
            "--default-features=supported",
            "--crate-feature=self=experimental",
        ]);
        let test_result = test_args.run()?;
        let cc_api = std::fs::read_to_string(&test_result.h_path)?;
        assert_starts_with(
            &cc_api,
            "// Automatically @generated C++ bindings for the following Rust crate:\n\
            // test_crate\n\
            // Features: experimental, supported",
        );
        Ok(())
    }

    #[test]
    fn test_crate_disabled_features() -> Result<()> {
        let test_args = TestArgs::default_args()?.with_extra_crubit_args(&[
            "--default-features=supported,experimental",
            "--crate-disabled-feature=self=experimental",
        ]);
        let test_result = test_args.run()?;
        let cc_api = std::fs::read_to_string(&test_result.h_path)?;
        assert_starts_with(
            &cc_api,
            "// Automatically @generated C++ bindings for the following Rust crate:\n\
            // test_crate\n\
            // Features: supported",
        );
        Ok(())
    }

    #[test]
    fn test_crate_default_features() -> Result<()> {
        let test_args =
            TestArgs::default_args()?.with_extra_crubit_args(&["--default-features=supported"]);
        let test_result = test_args.run()?;
        let cc_api = std::fs::read_to_string(&test_result.h_path)?;
        assert_starts_with(
            &cc_api,
            "// Automatically @generated C++ bindings for the following Rust crate:\n\
            // test_crate\n\
            // Features: supported",
        );
        Ok(())
    }
}
