// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Context;
use code_gen_utils::CommonTokens;
use database::code_snippet::Bindings;
use database::db::{BindingsGenerator, CodegenFunctions, Interner};
use error_report::{ErrorReport, ErrorReporting, FatalErrors, ReportFatalError, SourceLanguage};
use generate_bindings::generate_bindings as inner_generate_bindings;
use ir::ir_generated::{GenerateBindingsRequest, GenerateBindingsResponse};
use std::ffi::OsString;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process;

/// Creates a new database. Public for testing.
pub fn new_database<'db>(
    ir: &'db ir::IR,
    errors: &'db dyn ErrorReporting,
    fatal_errors: &'db dyn ReportFatalError,
    is_golden_test: bool,
    kythe_annotations: bool,
    interner: &'db Interner,
) -> BindingsGenerator<'db> {
    BindingsGenerator::new(
        ir,
        errors,
        fatal_errors,
        is_golden_test,
        kythe_annotations,
        CodegenFunctions {
            generate_enum: generate_enum::generate_enum,
            generate_item: generate_bindings::generate_item,
            generate_record: generate_struct_and_union::generate_record,
        },
        interner,
        CommonTokens::new(),
        lifetime_defaults_transform::decl_lifetime_arity,
        generate_bindings::rs_type_kind_safety,
        generate_bindings::unknown_lifetime_safety,
        lifetime_defaults_transform::lifetime_defaults_transform_item,
        lifetime_defaults_transform::lifetime_defaults_transform_record,
        lifetime_defaults_transform::lifetime_defaults_transform_func,
        lifetime_defaults_transform::lifetime_defaults_transform_type_alias,
        generate_bindings::record_field_safety,
        generate_bindings::record_safety,
        has_bindings::has_bindings,
        rs_type_kind::rs_type_kind_with_lifetime_elision,
        generate_function::generate_function,
        generate_function::overload_sets,
        generate_function::is_record_clonable,
        generate_function::get_binding,
        generate_struct_and_union::collect_unqualified_member_functions,
        generate_bindings::crubit_abi_type,
        has_bindings::resolve_names,
        generate_function::mangled_name_counts,
        generate_bindings::unsupported_item_errors,
    )
}

pub fn generate_bindings(
    request: GenerateBindingsRequest,
    response: &mut GenerateBindingsResponse,
) {
    let ir = ir::proto_to_ir(request.ir())
        .with_context(|| "Failed to deserialize IR".to_string())
        .unwrap();
    let crubit_support_path_format: &str = request.crubit_support_path_format();
    let crubit_support_versioned_path_format: &str = request.crubit_support_versioned_path_format();
    let clang_format_exe_path: OsString = request.clang_format_exe_path().into();
    let rustfmt_exe_path: OsString = request.rustfmt_exe_path().into();
    let rustfmt_config_path: OsString = request.rustfmt_config_path().into();
    let kythe_default_corpus: &str = request.kythe_default_corpus();
    let generate_error_report = request.generate_error_report();
    let is_golden_test = request.is_golden_test();
    let kythe_annotations = request.kythe_annotations();

    // The `ir::IR` tree from the outer scope is not unwind safe. However, since we abort on
    // panic anyways, it is safe to bypass this check.
    catch_unwind(AssertUnwindSafe(|| {
        let mut error_report: Option<ErrorReport> = None;
        let errors: &dyn ErrorReporting = if generate_error_report {
            error_report.insert(ErrorReport::new(SourceLanguage::Cpp))
        } else {
            &error_report::IgnoreErrors
        };
        let fatal_errors = FatalErrors::new();
        let Bindings { rs_api, rs_api_impl } = {
            let interner = Interner::default();
            let db = new_database(
                &ir,
                errors,
                &fatal_errors,
                is_golden_test,
                kythe_annotations,
                &interner,
            );
            inner_generate_bindings(
                &db,
                crubit_support_path_format,
                crubit_support_versioned_path_format,
                &clang_format_exe_path,
                &rustfmt_exe_path,
                &rustfmt_config_path,
                kythe_default_corpus,
            )
            .unwrap()
        };

        response.set_rs_api(rs_api.into());
        response.set_rs_api_impl(rs_api_impl.into());
        if let Some(report) = error_report {
            response.set_error_report(report.to_json_string().into());
        }
        response.set_fatal_errors(fatal_errors.take_string().into());
    }))
    .unwrap_or_else(|_| process::abort())
}
