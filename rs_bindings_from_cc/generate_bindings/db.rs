// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::generate_function::{
    generate_function, get_binding, is_record_clonable, overloaded_funcs, FunctionId, ImplKind,
};
use crate::generate_struct_and_union::collect_unqualified_member_functions;
use crate::rs_snippet::RsTypeKind;
use crate::rs_type_kind;
use crate::GeneratedItem;
use arc_anyhow::Result;
use error_report::ErrorReporting;
use ffi_types::SourceLocationDocComment;
use ir::{Func, Record, RsType, UnqualifiedIdentifier, IR};
use proc_macro2::Ident;
use std::collections::HashSet;
use std::rc::Rc;

memoized::query_group! {
    pub(crate) trait BindingsGenerator {
        #[input]
        fn ir(&self) -> Rc<IR>;
        #[input]
        fn errors(&self) -> Rc<dyn ErrorReporting>;
        #[input]
        fn generate_source_loc_doc_comment(&self) -> SourceLocationDocComment;

        fn rs_type_kind(&self, rs_type: RsType) -> Result<RsTypeKind>;

        fn generate_function(&self, func: Rc<Func>, record_overwrite: Option<Rc<Record>>) -> Result<Option<(Rc<GeneratedItem>, Rc<FunctionId>)>>;

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
    pub(crate) struct Database;
}
