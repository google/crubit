// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::code_snippet::ApiSnippets;
use crate::function_types::{FunctionId, GeneratedFunction, ImplKind};
use crate::rs_snippet::RsTypeKind;
use arc_anyhow::Result;
use error_report::{ErrorReporting, ReportFatalError};
use ffi_types::Environment;
use ir::{Enum, Func, Record, RsType, UnqualifiedIdentifier, IR};
use proc_macro2::Ident;
use std::collections::HashSet;
use std::rc::Rc;

#[unsafe(no_mangle)]
pub fn test_again() {}

memoized::query_group! {
    pub trait BindingsGenerator<'db> {
        #[input]
        fn ir(&self) -> &'db IR;

        #[input]
        fn errors(&self) -> &'db dyn ErrorReporting;

        /// A collection of errors that should cause bindings generation to fail.
        ///
        /// These errors should be issued only in response to misusage of Crubit itself, such as
        /// incorrect use of Crubit-specific annotations.
        #[input]
        fn fatal_errors(&self) -> &'db dyn ReportFatalError;

        #[input]
        fn environment(&self) -> Environment;

        #[break_cycles_with = Ok(false)]
        fn is_rs_type_kind_unsafe(&self, rs_type_kind: RsTypeKind) -> Result<bool>;

        fn generate_enum(&self, enum_: Rc<Enum>) -> Result<ApiSnippets>;

        fn generate_item(&self, item: ir::Item) -> Result<ApiSnippets>;

        fn generate_record(&self, record: Rc<Record>) -> Result<ApiSnippets>;

        fn rs_type_kind(&self, rs_type: RsType) -> Result<RsTypeKind>;

        fn generate_function(&self, func: Rc<Func>, record_overwrite: Option<Rc<Record>>) -> Result<Option<GeneratedFunction>>;

        fn overloaded_funcs(&self) -> Rc<HashSet<Rc<FunctionId>>>;

        fn is_record_clonable(&self, record: Rc<Record>) -> bool;

        fn get_binding(
            &self,
            expected_function_name: UnqualifiedIdentifier,
            expected_param_types: Vec<RsTypeKind>,
        ) -> Option<(Ident, ImplKind)>;

        fn collect_unqualified_member_functions(
            &self,
            record: Rc<Record>,
        ) -> Rc<[Rc<Func>]>;
    }
    pub struct Database;
}
