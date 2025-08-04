// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::code_snippet::{
    ApiSnippets, BindingsInfo, NoBindingsReason, ResolvedTypeName, Visibility,
};
use crate::function_types::{FunctionId, GeneratedFunction, ImplKind};
use crate::rs_snippet::{ElisionOptions, RsTypeKind};
use arc_anyhow::{anyhow, Result};
use crubit_abi_type::CrubitAbiType;
use error_report::{ErrorReporting, ReportFatalError};
use ffi_types::Environment;
use ir::{BazelLabel, CcType, Enum, Func, Record, UnqualifiedIdentifier, IR};
use proc_macro2::Ident;
use std::collections::HashMap;
use std::rc::Rc;

#[unsafe(no_mangle)]
pub fn test_again() {}

#[derive(Clone)]
pub struct CodegenFunctions {
    pub generate_enum: fn(&dyn BindingsGenerator, Rc<Enum>) -> Result<ApiSnippets>,
    pub generate_item: fn(&dyn BindingsGenerator, ir::Item) -> Result<ApiSnippets>,
    pub generate_record: fn(&dyn BindingsGenerator, Rc<Record>) -> Result<ApiSnippets>,
}

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

        #[input]
        fn codegen_functions(&self) -> CodegenFunctions;

        #[break_cycles_with = false]
        /// Returns true if the given Rust type is an unsafe type, such as a raw pointer.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/lib.rs?q=function:is_rs_type_kind_unsafe
        fn is_rs_type_kind_unsafe(&self, rs_type_kind: RsTypeKind) -> bool;

        /// Returns the bindings info for the given item, or an error if the item is not supported.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/has_bindings.rs?q=function:has_bindings
        fn has_bindings(&self, item: ir::Item) -> Result<BindingsInfo, NoBindingsReason>;

        /// Returns the Rust type kind of the given C++ type, optionally filling in missing
        /// reference lifetimes with the elided lifetime (`'_`).
        ///
        /// An `Ok()` return value does not necessarily imply that the resulting `RsTypeKind` is
        /// usable in APIs: callers must also check the result of `db::type_visibility()` for
        /// the type, to see if it is usable within a specific crate. Eventually, all types will
        /// have a successful non-error return value, even if the type is not generally usable.
        /// Instead, restrictions will always be done via `type_visibility`.
        ///
        /// TODO(b/409128537): never return `Err` here, instead check `type_visibility`
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/rs_type_kind.rs?q=function:rs_type_kind_with_lifetime_elision
        fn rs_type_kind_with_lifetime_elision(&self, cc_type: CcType, elision_options: ElisionOptions) -> Result<RsTypeKind>;

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

        /// You should call is_function_ambiguous() instead.
        ///
        /// Identifies all functions having overloads that we can't import (yet).
        ///
        /// TODO(b/213280424): Implement support for overloaded functions.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_function.rs?q=function:overload_sets
        fn overload_sets(&self) -> Rc<HashMap<Rc<FunctionId>, Option<ir::ItemId>>>;

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

        // You should probably use db::type_visibility instead of this function.
        fn type_target_restriction(&self, rs_type_kind: RsTypeKind) -> Result<Option<BazelLabel>>;

        /// Resolves type names to a map from name to ResolvedTypeName.
        ///
        /// This only checks the type namespace, as described here:
        /// https://doc.rust-lang.org/reference/names/namespaces.html.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/has_bindings.rs?q=function:resolve_type_names
        fn resolve_type_names(&self, parent: Rc<Record>) -> Result<Rc<HashMap<Rc<str>, ResolvedTypeName>>>;

        #[provided]
        /// Returns the generated bindings for the given enum.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_enum.rs?q=function:generate_enum
        fn generate_enum(&self, enum_: Rc<Enum>) -> Result<ApiSnippets> {
            (self.codegen_functions().generate_enum)(self, enum_)
        }

        #[provided]
        /// Returns the generated bindings for an item, or `Err` if bindings generation
        /// failed in such a way as to make the generated bindings as a whole invalid.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/lib.rs?q=function:generate_item
        fn generate_item(&self, item: ir::Item) -> Result<ApiSnippets> {
            (self.codegen_functions().generate_item)(self, item)
        }

        #[provided]
        /// Returns the generated bindings for the given record, along with associated safety
        /// assertions.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/generate_struct_and_union.rs?q=function:generate_record
        fn generate_record(&self, record: Rc<Record>) -> Result<ApiSnippets> {
            (self.codegen_functions().generate_record)(self, record)
        }

        #[provided]
        /// Returns the Rust type kind of the given C++ type.
        ///
        /// This differs from `rs_type_kind_with_lifetime_elision` in that it replaces references
        /// with missing lifetimes with pointer types.
        fn rs_type_kind(&self, cc_type: CcType) -> Result<RsTypeKind> {
            self.rs_type_kind_with_lifetime_elision(cc_type, ElisionOptions::default())
        }

        #[provided]
        /// Returns true if an ItemId refers to a function that cannot receive bindings, because
        /// it is overloaded and ambiguous.
        ///
        /// This does not include functions that are overloaded, where all but one overload is
        /// deprecated.
        fn is_ambiguous_function(&self, function_id: &FunctionId, item_id: ir::ItemId) -> bool {
            match self.overload_sets().get(function_id) {
                None => false,
                Some(id) => *id != Some(item_id),
            }
        }
    }
    pub struct Database;
}

/// Returns the `Visibility` of the `rs_type_kind` in the given `library`.
// TODO(jeanpierreda): it would be nice if this was a `#[provided]` function,
// but because it calls `display`, it would need to convert to a
// `dyn BindingsGenerator`, which is not reasonably possible.
//
// See e.g. https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=10f937bb0f13d2ea05f20f676c37439a
pub fn type_visibility(
    db: &dyn BindingsGenerator,
    library: &BazelLabel,
    rs_type_kind: RsTypeKind,
) -> Result<Visibility> {
    match db.type_target_restriction(rs_type_kind.clone())? {
        Some(label) if &label != library => {
            let rs_type_kind = rs_type_kind.display(db);
            Err(anyhow!("{rs_type_kind} is `pub(crate)` in {label}"))
        }
        Some(_) => Ok(Visibility::PubCrate),
        None => {
            for subtype in rs_type_kind.dfs_iter() {
                if let RsTypeKind::Error { visibility_override, .. } = subtype {
                    return Ok(visibility_override.unwrap_or(Visibility::PubCrate));
                }
            }
            Ok(Visibility::Public)
        }
    }
}
