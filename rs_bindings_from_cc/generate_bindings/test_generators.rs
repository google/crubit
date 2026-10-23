// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Utilities for generating bindings in tests.

use arc_anyhow::Result;
use database::code_snippet::BindingsTokens;
use database::db::{BindingsGenerator, Interner};
use error_report::{bail, ErrorReport, FatalErrors, SourceLanguage};
use generate_bindings::{generate_bindings_tokens, new_database};
use ir::IR;
use ir_testing::{make_test_ir, make_test_ir_dependency};
use multiplatform_ir_testing::{
    ir_proto_from_assumed_lifetimes_cc, ir_proto_from_assumed_lifetimes_cc_dependency,
    ir_proto_from_cc, ir_proto_from_cc_annotated, ir_proto_from_cc_dependency,
    ir_proto_from_cc_with_inline_cpp, ir_proto_from_record_impl_debug_cc,
};

pub fn generate_bindings_tokens_from_cc(header: &str) -> Result<BindingsTokens> {
    let proto = ir_proto_from_cc(header)?;
    let ir = make_test_ir(&proto)?;
    generate_bindings_tokens_for_test(ir)
}

pub fn generate_bindings_tokens_from_cc_annotated(header: &str) -> Result<BindingsTokens> {
    let proto = ir_proto_from_cc_annotated(header)?;
    let ir = make_test_ir(&proto)?;
    generate_bindings_tokens_for_test_with_annotations(ir)
}

pub fn generate_bindings_tokens_from_cc_dependency(
    header: &str,
    dep_header: &str,
) -> Result<BindingsTokens> {
    let proto = ir_proto_from_cc_dependency(header, dep_header)?;
    let ir = make_test_ir(&proto)?;
    generate_bindings_tokens_for_test(ir)
}

pub fn generate_bindings_tokens_from_cc_with_inline_cpp(header: &str) -> Result<BindingsTokens> {
    let proto = ir_proto_from_cc_with_inline_cpp(header)?;
    let ir = make_test_ir(&proto)?;
    generate_bindings_tokens_for_test(ir)
}

pub fn generate_bindings_tokens_from_record_impl_debug_cc(header: &str) -> Result<BindingsTokens> {
    let proto = ir_proto_from_record_impl_debug_cc(header)?;
    let ir = make_test_ir(&proto)?;
    generate_bindings_tokens_for_test(ir)
}

pub fn generate_bindings_tokens_from_assumed_lifetimes_cc(program: &str) -> Result<BindingsTokens> {
    let proto = ir_proto_from_assumed_lifetimes_cc(program)?;
    let ir = make_test_ir_dependency(&proto, Some("assume_lifetimes"))?;
    generate_bindings_tokens_for_test(ir)
}

pub fn generate_bindings_tokens_from_assumed_lifetimes_cc_dependency(
    header: &str,
    dep_header: &str,
) -> Result<BindingsTokens> {
    let proto = ir_proto_from_assumed_lifetimes_cc_dependency(header, dep_header)?;
    let ir = make_test_ir_dependency(&proto, Some("assume_lifetimes"))?;
    generate_bindings_tokens_for_test(ir)
}

pub fn generate_bindings_tokens_for_test(ir: IR) -> Result<BindingsTokens> {
    let fatal_errors = FatalErrors::new();
    let tokens = generate_bindings_tokens(
        &ir,
        dyn_format::Format::parse_with_metavars("crubit/rs_bindings_support", &["unused"]).unwrap(),
        dyn_format::Format::parse_with_metavars("crubit/rs_bindings_support/internal", &["unused"])
            .unwrap(),
        &error_report::IgnoreErrors,
        &fatal_errors,
        false,
        /* kythe_annotations= */ false,
    )?;
    let fatal = fatal_errors.take_string();
    if !fatal.is_empty() {
        bail!("Fatal errors:{}", fatal)
    }
    Ok(tokens)
}

pub fn generate_bindings_tokens_for_test_with_annotations(ir: IR) -> Result<BindingsTokens> {
    let fatal_errors = FatalErrors::new();
    let tokens = generate_bindings_tokens(
        &ir,
        dyn_format::Format::parse_with_metavars("crubit/rs_bindings_support", &["unused"]).unwrap(),
        dyn_format::Format::parse_with_metavars("crubit/rs_bindings_support/internal", &["unused"])
            .unwrap(),
        &error_report::IgnoreErrors,
        &fatal_errors,
        false,
        /* kythe_annotations= */ true,
    )?;
    let fatal = fatal_errors.take_string();
    if !fatal.is_empty() {
        bail!("Fatal errors:{}", fatal)
    }
    Ok(tokens)
}

pub struct TestDbFactory {
    ir: IR,
    errors: ErrorReport,
    fatal_errors: FatalErrors,
    interner: Interner,
}

impl TestDbFactory {
    pub fn new(ir: IR) -> Self {
        Self {
            ir,
            errors: ErrorReport::new(SourceLanguage::Cpp),
            fatal_errors: FatalErrors::new(),
            interner: Interner::default(),
        }
    }
    pub fn make_db(&self) -> BindingsGenerator<'_> {
        self.make_db_with_golden_test(false)
    }
    pub fn make_db_with_golden_test(&self, is_golden_test: bool) -> BindingsGenerator<'_> {
        new_database(
            &self.ir,
            &self.errors,
            &self.fatal_errors,
            is_golden_test,
            /* kythe_annotations= */ false,
            &self.interner,
        )
    }
    pub fn fatal_errors(&self) -> &FatalErrors {
        &self.fatal_errors
    }
    pub fn ir(&self) -> &IR {
        &self.ir
    }
}
