// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Utilities for generating bindings in tests.

use arc_anyhow::Result;
use database::code_snippet::BindingsTokens;
use database::db::BindingsGenerator;
use error_report::{bail, ErrorReport, FatalErrors, SourceLanguage};
use ffi_types::Environment;
use generate_bindings::{generate_bindings_tokens, new_database};
use ir::IR;
use multiplatform_ir_testing::ir_from_cc;

pub fn generate_bindings_tokens_for_test(ir: IR) -> Result<BindingsTokens> {
    let fatal_errors = FatalErrors::new();
    let tokens = generate_bindings_tokens(
        &ir,
        dyn_format::Format::parse_with_metavars("crubit/rs_bindings_support", &["unused"]).unwrap(),
        &error_report::IgnoreErrors,
        &fatal_errors,
        Environment::Production,
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
}

impl TestDbFactory {
    pub fn from_cc(cc_str: &str) -> Result<Self> {
        Ok(Self {
            ir: ir_from_cc(cc_str)?,
            errors: ErrorReport::new(SourceLanguage::Cpp),
            fatal_errors: FatalErrors::new(),
        })
    }
    pub fn make_db(&self) -> BindingsGenerator {
        new_database(&self.ir, &self.errors, &self.fatal_errors, Environment::Production)
    }
}
