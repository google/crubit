// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Helpers for C++ bindings generation tests.

#![feature(rustc_private)]
extern crate rustc_middle;

use arc_anyhow::Result;
use code_gen_utils::CcInclude;
use database::code_snippet::ApiSnippets;
use database::{BindingsGenerator, IncludeGuard};
use error_report::{FatalErrors, IgnoreErrors};
use generate_bindings::{generate_bindings, new_database, BindingsTokens};
use run_compiler_test_support::{find_def_id_by_name, run_compiler_for_testing};
use rustc_middle::ty::TyCtxt;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use dyn_format::Format;

/// Tests invoking `generate_item` on the item with the specified `name`
/// from the given Rust `source`.  Returns the result of calling
/// `test_function` with `generate_item`'s result as an argument.
/// (`test_function` should typically `assert!` that it got the expected
/// result from `generate_item`.)
pub fn test_format_item<F, T>(source: &str, name: &str, test_function: F) -> T
where
    F: FnOnce(Result<Option<ApiSnippets>, String>) -> T + Send,
    T: Send,
{
    test_format_item_with_features(
        source,
        name,
        crubit_feature::CrubitFeature::Experimental | crubit_feature::CrubitFeature::Supported,
        /* with_kythe_annotations= */ false,
        test_function,
    )
}

/// Tests invoking `generate_item` on the item with the specified `name`
/// from the given Rust `source`, with the specified features. Returns
/// the result of calling `test_function` with `generate_item`'s result
/// as an argument. (`test_function` should typically `assert!` that it
/// got the expected result from `generate_item`.)
pub fn test_format_item_with_features<F, T>(
    source: &str,
    name: &str,
    features: impl Into<flagset::FlagSet<crubit_feature::CrubitFeature>>,
    with_kythe_annotations: bool,
    test_function: F,
) -> T
where
    F: FnOnce(Result<Option<ApiSnippets>, String>) -> T + Send,
    T: Send,
{
    let mut output: Option<T> = None;
    let output_ref = &mut output;
    test_format_item_with_features_impl(
        source,
        name,
        features.into(),
        with_kythe_annotations,
        Box::new(move |result| {
            *output_ref = Some(test_function(result));
        }),
    );
    output.unwrap()
}

type FormatItemTestFn<'a> =
    Box<dyn for<'tcx> FnOnce(Result<Option<ApiSnippets<'tcx>>, String>) + Send + 'a>;

/// Non-generic implementation of `test_format_item_with_features` so that compiler invocation,
/// database setup, item lookup, and error formatting are compiled once in `test_helpers` rather
/// than monomorphized at every test call site.
fn test_format_item_with_features_impl(
    source: &str,
    name: &str,
    features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    with_kythe_annotations: bool,
    test_function: FormatItemTestFn<'_>,
) {
    run_compiler_for_testing(source, |tcx| {
        let local_def_id = find_def_id_by_name(tcx, name);
        let result =
            bindings_db_for_tests_with_features(tcx, features, with_kythe_annotations, None)
                .generate_item(local_def_id.to_def_id())
                .cloned();

        // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations says:
        // To print causes as well [...], use the alternate selector “{:#}”.
        let result = result.map_err(|anyhow_err| format!("{anyhow_err:#}"));

        test_function(result);
    });
}

const DEFAULT_KNOWN_CRATE_NAMES: &[&str] = &["alloc", "core", "std"];

fn default_test_features() -> flagset::FlagSet<crubit_feature::CrubitFeature> {
    crubit_feature::CrubitFeature::Experimental | crubit_feature::CrubitFeature::Supported
}

struct TestDbOptions<'a> {
    known_crate_names: &'a [&'a str],
    features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    with_kythe_annotations: bool,
    self_namespace: Option<&'a str>,
    errors: Rc<dyn error_report::ErrorReporting>,
    fatal_errors: Rc<dyn error_report::ReportFatalError>,
    ignore_symbols_from_files: HashSet<PathBuf>,
}

impl Default for TestDbOptions<'_> {
    fn default() -> Self {
        Self {
            known_crate_names: DEFAULT_KNOWN_CRATE_NAMES,
            features: default_test_features(),
            with_kythe_annotations: false,
            self_namespace: None,
            errors: Rc::new(IgnoreErrors),
            fatal_errors: Rc::new(FatalErrors::new()),
            ignore_symbols_from_files: HashSet::default(),
        }
    }
}

fn build_test_db<'tcx>(tcx: TyCtxt<'tcx>, options: TestDbOptions<'_>) -> BindingsGenerator<'tcx> {
    let TestDbOptions {
        known_crate_names,
        features,
        with_kythe_annotations,
        self_namespace,
        errors,
        fatal_errors,
        ignore_symbols_from_files,
    } = options;
    let crate_name_to_include_paths = known_crate_names
        .iter()
        .map(|&name| {
            (
                Rc::from(name),
                vec![CcInclude::user_header(Rc::from(
                    format!("fake_bindings_for_unittests/{name}_cc_api.h").as_str(),
                ))],
            )
        })
        .collect();
    let crate_name_to_features = known_crate_names
        .iter()
        .copied()
        .chain(std::iter::once("self"))
        .map(|name| (Rc::from(name), features))
        .collect();
    let mut crate_name_to_namespace: HashMap<Rc<str>, Rc<str>> = known_crate_names
        .iter()
        .map(|&name| (Rc::from(name), Rc::from(format!("rs::{name}").as_str())))
        .collect();
    if let Some(ns) = self_namespace {
        crate_name_to_namespace.insert(Rc::from("self"), Rc::from(ns));
    }
    new_database(
        tcx,
        /* source_crate_name= */ None,
        /* crubit_support_path_format= */
        Format::parse_with_metavars("<crubit/support/for/tests/{header}>", &["header"]).unwrap(),
        /* crubit_debug_path_format= */ None,
        /* default_features= */ Default::default(),
        /* kythe_annotations= */ with_kythe_annotations,
        /* portable_abi_compatible= */ true,
        /* enable_rmeta_interface= */ false,
        crate_name_to_include_paths,
        crate_name_to_features,
        crate_name_to_namespace,
        /* crate_renames= */ HashMap::default().into(),
        errors,
        fatal_errors,
        /* is_golden_test= */ true,
        /* include_guard */ IncludeGuard::PragmaOnce,
        /* ignore_symbols_from_files */ ignore_symbols_from_files.into(),
    )
}

pub fn bindings_db_for_tests_with_crate_headers<'tcx>(
    tcx: TyCtxt<'tcx>,
    known_crate_names: &[&str],
) -> BindingsGenerator<'tcx> {
    build_test_db(tcx, TestDbOptions { known_crate_names, ..Default::default() })
}

pub fn bindings_db_for_tests_with_features<'tcx>(
    tcx: TyCtxt<'tcx>,
    features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    with_kythe_annotations: bool,
    self_namespace: Option<&str>,
) -> BindingsGenerator<'tcx> {
    build_test_db(
        tcx,
        TestDbOptions { features, with_kythe_annotations, self_namespace, ..Default::default() },
    )
}

pub fn bindings_db_for_tests_with_error_report<'tcx>(
    tcx: TyCtxt<'tcx>,
    features: flagset::FlagSet<crubit_feature::CrubitFeature>,
) -> (BindingsGenerator<'tcx>, Rc<error_report::ErrorReport>, Rc<FatalErrors>) {
    let error_report = Rc::new(error_report::ErrorReport::new(error_report::SourceLanguage::Rust));
    let fatal_errors = Rc::new(FatalErrors::new());
    let db = build_test_db(
        tcx,
        TestDbOptions {
            features,
            errors: error_report.clone(),
            fatal_errors: fatal_errors.clone(),
            ..Default::default()
        },
    );
    (db, error_report, fatal_errors)
}

pub fn bindings_db_for_tests_with_ignore_symbols_from_files<'tcx>(
    tcx: TyCtxt<'tcx>,
    ignore_symbols_from_files: HashSet<PathBuf>,
) -> BindingsGenerator<'tcx> {
    build_test_db(tcx, TestDbOptions { ignore_symbols_from_files, ..Default::default() })
}

pub fn bindings_db_for_tests<'tcx>(tcx: TyCtxt<'tcx>) -> BindingsGenerator<'tcx> {
    bindings_db_for_tests_with_features(
        tcx,
        default_test_features(),
        /* with_kythe_annotations= */ false,
        None,
    )
}

pub fn bindings_db_for_tests_with_namespace<'tcx>(
    tcx: TyCtxt<'tcx>,
    namespace: &str,
) -> BindingsGenerator<'tcx> {
    bindings_db_for_tests_with_features(
        tcx,
        default_test_features(),
        /* with_kythe_annotations= */ false,
        Some(namespace),
    )
}

/// Tests invoking `generate_bindings` on the given Rust `source`.
/// Returns the result of calling `test_function` with the generated
/// bindings as an argument. (`test_function` should typically `assert!`
/// that it got the expected `GeneratedBindings`.)
pub fn test_generated_bindings<F, T>(source: &str, test_function: F) -> T
where
    F: FnOnce(Result<BindingsTokens>) -> T + Send,
    T: Send,
{
    test_function(generate_bindings_for_test_impl(source, None))
}

pub fn test_generated_bindings_with_namespace<F, T>(
    source: &str,
    namespace: &str,
    test_function: F,
) -> T
where
    F: FnOnce(Result<BindingsTokens>) -> T + Send,
    T: Send,
{
    test_function(generate_bindings_for_test_impl(source, Some(namespace)))
}

fn generate_bindings_for_test_impl(
    source: &str,
    namespace: Option<&str>,
) -> Result<BindingsTokens> {
    let (cc_api, cc_api_impl) = run_compiler_for_testing(source, |tcx| {
        let db = bindings_db_for_tests_with_features(
            tcx,
            default_test_features(),
            /* with_kythe_annotations= */ false,
            namespace,
        );
        generate_bindings(&db).map(|tokens| {
            // Convert `TokenStream` to `String` across the thread boundary since `TokenStream` is
            // `!Send`.
            (tokens.cc_api.to_string(), tokens.cc_api_impl.to_string())
        })
    })?;
    Ok(BindingsTokens {
        cc_api: cc_api.parse().unwrap(),
        cc_api_impl: cc_api_impl.parse().unwrap(),
    })
}
