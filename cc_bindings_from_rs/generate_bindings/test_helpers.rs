// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Helpers for C++ bindings generation tests.

#![feature(rustc_private)]

use arc_anyhow::Result;
use database::code_snippet::ApiSnippets;
use database::{BindingsGenerator as _, Database, IncludeGuard};
use error_report::{FatalErrors, IgnoreErrors};
use generate_bindings::{generate_bindings, new_database, BindingsTokens};
use run_compiler_test_support::{find_def_id_by_name, run_compiler_for_testing};
use rustc_middle::ty::TyCtxt;
use std::collections::HashMap;
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
    test_function: F,
) -> T
where
    F: FnOnce(Result<Option<ApiSnippets>, String>) -> T + Send,
    T: Send,
{
    let features = features.into();
    run_compiler_for_testing(source, |tcx| {
        let local_def_id = find_def_id_by_name(tcx, name);
        let result = bindings_db_for_tests_with_features(tcx, features)
            .generate_item(local_def_id.to_def_id());

        // https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations says:
        // To print causes as well [...], use the alternate selector “{:#}”.
        let result = result.map_err(|anyhow_err| format!("{anyhow_err:#}"));

        test_function(result)
    })
}

fn bindings_db_for_tests_with_features(
    tcx: TyCtxt,
    features: flagset::FlagSet<crubit_feature::CrubitFeature>,
) -> Database {
    new_database(
        tcx,
        /* source_crate_name= */ None,
        /* crubit_support_path_format= */
        Format::parse_with_metavars("<crubit/support/for/tests/{header}>".into(), &["header"])
            .unwrap(),
        /* default_features= */ Default::default(),
        /* crate_name_to_include_paths= */ Default::default(),
        /* crate_name_to_features= */
        Rc::new(HashMap::from([(Rc::from("self"), features)])),
        /* crate_name_to_namespace= */ HashMap::default().into(),
        /* crate_renames= */ HashMap::default().into(),
        /* errors = */ Rc::new(IgnoreErrors),
        /* fatal_errors= */ Rc::new(FatalErrors::new()),
        /* no_thunk_name_mangling= */ true,
        /* include_guard */ IncludeGuard::PragmaOnce,
    )
}

pub fn bindings_db_for_tests(tcx: TyCtxt) -> Database {
    bindings_db_for_tests_with_features(
        tcx,
        crubit_feature::CrubitFeature::Experimental | crubit_feature::CrubitFeature::Supported,
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
    run_compiler_for_testing(source, |tcx| {
        test_function(generate_bindings(&bindings_db_for_tests(tcx)))
    })
}
