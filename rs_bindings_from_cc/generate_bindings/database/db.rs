// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::code_snippet::{ApiSnippets, BindingsInfo, NoBindingsReason};
use crate::function_types::{FunctionId, GeneratedFunction, ImplKind};
use crate::rs_snippet::RsTypeKind;
use arc_anyhow::Result;
use crubit_abi_type::CrubitAbiType;
use error_report::{ErrorReporting, ReportFatalError};
use ffi_types::Environment;
use ir::{CcType, Enum, Func, Record, UnqualifiedIdentifier, IR};
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

        #[input]
        /// A collection of errors that should cause bindings generation to fail.
        ///
        /// These errors should be issued only in response to misusage of Crubit itself, such as
        /// incorrect use of Crubit-specific annotations.
        fn fatal_errors(&self) -> &'db dyn ReportFatalError;

        #[input]
        fn environment(&self) -> Environment;

        #[break_cycles_with = false]
        /// Returns true if the given Rust type is an unsafe type, such as a raw pointer.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/lib.rs?q=function:is_rs_type_kind_unsafe
        fn is_rs_type_kind_unsafe(&self, rs_type_kind: RsTypeKind) -> bool;

        /// Returns the bindings info for the given item, or an error if the item is not supported.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/has_bindings.rs?q=function:has_bindings
        fn has_bindings(&self, item: ir::Item) -> Result<BindingsInfo, NoBindingsReason>;

        /// Returns the generated bindings for the given enum.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_enum.rs?q=function:generate_enum
        fn generate_enum(&self, enum_: Rc<Enum>) -> Result<ApiSnippets>;

        /// Returns the generated bindings for an item, or `Err` if bindings generation
        /// failed in such a way as to make the generated bindings as a whole invalid.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/lib.rs?q=function:generate_item
        fn generate_item(&self, item: ir::Item) -> Result<ApiSnippets>;

        /// Returns the generated bindings for the given record, along with associated safety
        /// assertions.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_struct_and_union.rs?q=function:generate_record
        fn generate_record(&self, record: Rc<Record>) -> Result<ApiSnippets>;

        /// Returns the Rust type kind of the given C++ type.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/rs_type_kind.rs?q=function:rs_type_kind
        fn rs_type_kind(&self, cc_type: CcType) -> Result<RsTypeKind>;

        /// Returns the generated bindings for the given function.
        ///
        /// `derived_record` is a derived class type which re-exports `func` as a
        /// method on this record. `func` must be a method on a base class of
        /// `derived_record`, if present.
        ///
        /// Returns:
        ///
        ///  * `Err(_)`: couldn't import the function, emit an `UnsupportedItem`.
        ///  * `Ok(None)`: the function imported as "nothing". (For example, a defaulted
        ///    destructor might be mapped to no `Drop` impl at all.)
        ///  * `Ok(GeneratedFunction)`: The Rust function definition,
        ///    thunk FFI definition, and function ID.
        ///
        /// Note that unlike other `generate_*` functions, this function may return `Ok()` but still
        /// fail to generate bindings (if `GeneratedFunction.status` is `Err`), and may fail
        /// to generate bindings even if `has_bindings` would return `Ok`. In general, one cannot
        /// rely on the bindings of a function being generated correctly, except for `Drop`.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_function.rs?q=function:generate_function
        fn generate_function(&self, func: Rc<Func>, derived_record: Option<Rc<Record>>) -> Result<Option<GeneratedFunction>>;

        /// Identifies all functions having overloads that we can't import (yet).
        ///
        /// TODO(b/213280424): Implement support for overloaded functions.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_function.rs?q=function:overloaded_funcs
        fn overloaded_funcs(&self) -> Rc<HashSet<Rc<FunctionId>>>;

        /// Returns whether the given record either implements or derives the Clone
        /// trait.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_function.rs?q=function:is_record_clonable
        fn is_record_clonable(&self, record: Rc<Record>) -> bool;

        /// Returns the generated bindings for a function with the given name and param
        /// types. If none exists, returns None.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_function.rs?q=function:get_binding
        fn get_binding(
            &self,
            expected_function_name: UnqualifiedIdentifier,
            expected_param_types: Vec<RsTypeKind>,
        ) -> Option<(Ident, ImplKind)>;

        /// Returns a collection of unqualified member functions of the given record.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_struct_and_union.rs?q=function:collect_unqualified_member_functions
        fn collect_unqualified_member_functions(
            &self,
            record: Rc<Record>,
        ) -> Rc<[Rc<Func>]>;

        /// Returns the `CrubitAbiType` for the given `RsTypeKind`.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/lib.rs?q=function:crubit_abi_type
        fn crubit_abi_type(&self, rs_type_kind: RsTypeKind) -> Result<CrubitAbiType>;
    }
    pub struct Database;
}
