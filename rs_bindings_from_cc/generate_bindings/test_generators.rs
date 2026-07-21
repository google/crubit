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

pub fn generate_bindings_tokens_for_test(ir: IR<'_>) -> Result<BindingsTokens> {
    let fatal_errors = FatalErrors::new();
    let tokens = generate_bindings_tokens(
        &ir,
        dyn_format::Format::parse_with_metavars("crubit/rs_bindings_support", &["unused"]).unwrap(),
        dyn_format::Format::parse_with_metavars("crubit/rs_bindings_support/internal", &["unused"])
            .unwrap(),
        &error_report::IgnoreErrors,
        &fatal_errors,
        false,
        /*kythe_annotations=*/ false,
    )?;
    let fatal = fatal_errors.take_string();
    if !fatal.is_empty() {
        bail!("Fatal errors:{}", fatal)
    }
    Ok(tokens)
}

pub fn generate_bindings_tokens_for_test_with_annotations(ir: IR<'_>) -> Result<BindingsTokens> {
    let fatal_errors = FatalErrors::new();
    let tokens = generate_bindings_tokens(
        &ir,
        dyn_format::Format::parse_with_metavars("crubit/rs_bindings_support", &["unused"]).unwrap(),
        dyn_format::Format::parse_with_metavars("crubit/rs_bindings_support/internal", &["unused"])
            .unwrap(),
        &error_report::IgnoreErrors,
        &fatal_errors,
        false,
        /*kythe_annotations=*/ true,
    )?;
    let fatal = fatal_errors.take_string();
    if !fatal.is_empty() {
        bail!("Fatal errors:{}", fatal)
    }
    Ok(tokens)
}

pub struct TestDbFactory<'pb> {
    ir: IR<'pb>,
    errors: ErrorReport,
    fatal_errors: FatalErrors,
    interner: Interner,
}

impl<'pb> TestDbFactory<'pb> {
    pub fn new(ir: IR<'pb>) -> Self {
        Self {
            ir,
            errors: ErrorReport::new(SourceLanguage::Cpp),
            fatal_errors: FatalErrors::new(),
            interner: Interner::default(),
        }
    }
    pub fn make_db(&self) -> BindingsGenerator {
        new_database(
            &self.ir,
            &self.errors,
            &self.fatal_errors,
            false,
            /*kythe_annotations=*/ false,
            &self.interner,
        )
    }
}
