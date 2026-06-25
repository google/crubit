// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::code_snippet::{ApiSnippets, BindingsInfo, NoBindingsReason, ResolvedName, Visibility};
use crate::function_types::{FunctionId, GeneratedFunction, ImplKind};
use crate::rs_snippet::{LifetimeOptions, RsTypeKind, UnsafeReason};
use arc_anyhow::{anyhow, Error, Result};
use code_gen_utils::make_rs_ident;
use crubit_abi_type::CrubitAbiType;
use error_report::{ErrorReporting, ReportFatalError};
use heck::ToSnakeCase;
use ir::{BazelLabel, CcType, Enum, Field, Func, GenericItem, Record, UnqualifiedIdentifier, IR};
use proc_macro2::Ident;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct CodegenFunctions {
    pub generate_enum: fn(&BindingsGenerator, Rc<Enum>) -> Result<ApiSnippets>,
    pub generate_item: fn(&BindingsGenerator, ir::Item) -> Result<ApiSnippets>,
    pub generate_record: fn(&BindingsGenerator, Rc<Record>) -> Result<ApiSnippets>,
    pub decl_lifetime_arity: fn(&BindingsGenerator, ir::ItemId) -> Result<usize>,
}

memoized::query_group! {
    pub struct BindingsGenerator<'db> {
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
        fn is_golden_test(&self) -> bool;

        #[input]
        /// Feature flag enabling Kythe annotations
        fn kythe_annotations(&self) -> bool;

        #[input]
        fn codegen_functions(&self) -> CodegenFunctions;

        #[break_cycles_with = None]
        /// Returns whether the given Rust type is an unsafe type, such as a raw pointer.
        ///
        /// Returns `None` if the type is safe.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/lib.rs?q=function:rs_type_kind_safety
        fn rs_type_kind_safety(&self, rs_type_kind: RsTypeKind) -> Option<UnsafeReason>;

        #[break_cycles_with = None]
        /// Returns whether the given field is unsafe to access.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/lib.rs?q=function:record_field_safety
        fn record_field_safety(&self, field: Field) -> Option<UnsafeReason>;

        #[break_cycles_with = None]
        /// Returns whether the given record is unsafe.
        ///
        /// A record may be unsafe due to an explicitly annotation, or by being a union,
        /// or by having an unsafe public field (see `record_field_safety`).
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/lib.rs?q=function:record_safety
        fn record_safety(&self, record: Rc<Record>) -> Option<UnsafeReason>;

        /// Returns the bindings info for the given item, or an error if the item is not supported.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/has_bindings.rs?q=function:has_bindings
        fn has_bindings(&self, item: ir::Item) -> Result<BindingsInfo, NoBindingsReason>;

        /// Returns the Rust type kind of the given C++ type, optionally filling in missing
        /// reference lifetimes with the elided lifetime (`'_`).
        ///
        /// An `Ok()` return value does not necessarily imply that the resulting `RsTypeKind` is
        /// usable in APIs: callers must also check the result of `type_visibility()` for
        /// the type, to see if it is usable within a specific crate. Eventually, all types will
        /// have a successful non-error return value, even if the type is not generally usable.
        /// Instead, restrictions will always be done via `type_visibility`.
        ///
        /// TODO(b/409128537): never return `Err` here, instead check `type_visibility`
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/rs_type_kind.rs?q=function:rs_type_kind_with_lifetime_elision
        fn rs_type_kind_with_lifetime_elision(&self, cc_type: CcType, lifetime_options: LifetimeOptions) -> Result<RsTypeKind>;

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

        // You should probably use `type_visibility()` instead of this function.
        fn type_target_restriction(&self, rs_type_kind: RsTypeKind) -> Result<Option<BazelLabel>>;

        /// Resolves names to a map from name to ResolvedName.
        ///
        /// This checks both type and value namespaces.
        ///
        /// Implementation: rs_bindings_from_cc/generate_bindings/has_bindings.rs?q=function:resolve_names
        fn resolve_names(&self, parent: Rc<Record>) -> Result<Rc<HashMap<Rc<str>, ResolvedName>>>;
    }
}

impl<'db> BindingsGenerator<'db> {
    /// Returns the generated bindings for the given enum.
    ///
    /// Implementation: rs_bindings_from_cc/generate_bindings/generate_enum.rs?q=function:generate_enum
    pub fn generate_enum(&self, enum_: Rc<Enum>) -> Result<ApiSnippets> {
        (self.codegen_functions().generate_enum)(self, enum_)
    }

    /// Returns the generated bindings for an item, or `Err` if bindings generation
    /// failed in such a way as to make the generated bindings as a whole invalid.
    ///
    /// Implementation: rs_bindings_from_cc/generate_bindings/lib.rs?q=function:generate_item
    pub fn generate_item(&self, item: ir::Item) -> Result<ApiSnippets> {
        (self.codegen_functions().generate_item)(self, item)
    }

    /// Returns the generated bindings for the given record, along with associated safety
    /// assertions.
    ///
    /// Implementation: rs_bindings_from_cc/generate_bindings/generate_struct_and_union.rs?q=function:generate_record
    pub fn generate_record(&self, record: Rc<Record>) -> Result<ApiSnippets> {
        (self.codegen_functions().generate_record)(self, record)
    }

    /// Returns the Rust type kind of the given C++ type.
    ///
    /// This differs from `rs_type_kind_with_lifetime_elision` in that it replaces references
    /// with missing lifetimes with pointer types.
    pub fn rs_type_kind(&self, cc_type: CcType) -> Result<RsTypeKind> {
        self.rs_type_kind_with_lifetime_elision(cc_type, LifetimeOptions::default())
    }

    /// Returns true if an ItemId refers to a function that cannot receive bindings, because
    /// it is overloaded and ambiguous.
    ///
    /// This does not include functions that are overloaded, where all but one overload is
    /// deprecated.
    pub fn is_ambiguous_function(&self, function_id: &FunctionId, item_id: ir::ItemId) -> bool {
        match self.overload_sets().get(function_id) {
            None => false,
            Some(id) => *id != Some(item_id),
        }
    }

    /// Returns the `Visibility` of the `rs_type_kind` in the given `library`.
    pub fn type_visibility(
        &self,
        library: &BazelLabel,
        rs_type_kind: RsTypeKind,
    ) -> Result<Visibility> {
        match self.type_target_restriction(rs_type_kind.clone())? {
            Some(label) if &label != library => {
                let rs_type_kind = rs_type_kind.display(self);
                Err(anyhow!(
                    "crubit.rs/errors/visibility: Support for `{rs_type_kind}` is experimental.\n\
                      Its use is restricted to `pub(crate)` in defining target:\n\
                      `{label}`"
                ))
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

    /// Returns the target that this item was defined in, if it was defined somewhere other than
    /// `owning_target()`. This may be `Some` for class template specializations and their member
    /// functions and is `None` otherwise.
    pub fn defining_target(&self, item_id: ir::ItemId) -> Option<ir::BazelLabel> {
        let item = self.find_untyped_decl(item_id);
        match item {
            ir::Item::Func(f) => {
                if let Some(parent_id) = f.enclosing_item_id
                    && let Ok(record) = self.find_decl::<std::rc::Rc<ir::Record>>(parent_id)
                {
                    return self.defining_target(record.id);
                }
                None
            }
            ir::Item::Record(r) => {
                r.template_specialization.as_ref().map(|ts| ts.defining_target.clone())
            }
            ir::Item::UnsupportedItem(ui) => ui.defining_target.clone(),
            _ => None,
        }
    }

    /// The name of the item, readable by programmers.
    ///
    /// For example, `void Foo();` should have name `Foo`.
    pub fn debug_name(&self, item_id: ir::ItemId) -> std::rc::Rc<str> {
        let item = self.find_untyped_decl(item_id);
        let (id, name) = match item {
            ir::Item::Func(f) => {
                let mut name = self.namespace_qualifier_from_id(f.id).format_for_cc_debug();
                let record_name = || -> Option<std::rc::Rc<str>> {
                    if let Some(parent_id) = f.enclosing_item_id {
                        match self.find_untyped_decl(parent_id) {
                            ir::Item::ExistingRustType(existing_rust_type) => {
                                Some(existing_rust_type.cc_name.clone())
                            }
                            ir::Item::Record(record) => Some(record.cc_name.identifier.clone()),
                            ir::Item::IncompleteRecord(record) => {
                                Some(record.cc_name.identifier.clone())
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                };
                match &f.cc_name {
                    ir::UnqualifiedIdentifier::Identifier(id) => {
                        name.push_str(&id.identifier);
                    }
                    ir::UnqualifiedIdentifier::Operator(op) => {
                        name.push_str(&op.cc_name());
                    }
                    ir::UnqualifiedIdentifier::Destructor => {
                        name.push('~');
                        name.push_str(
                            &record_name().expect("destructor must be associated with a record"),
                        );
                    }
                    ir::UnqualifiedIdentifier::Constructor => {
                        name.push_str(
                            &record_name().expect("constructor must be associated with a record"),
                        );
                    }
                }
                return name.into();
            }
            ir::Item::Comment(c) => {
                return format!(
                    "<[internal] comment at {}>",
                    c.source_loc().as_deref().unwrap_or("<unknown loc>")
                )
                .into()
            }
            ir::Item::UseMod(u) => {
                return format!("<[internal] use mod {}::* = {}>", u.mod_name, u.path).into()
            }
            ir::Item::UnsupportedItem(ui) => return ui.name.clone(),
            ir::Item::ExistingRustType(e) => (e.id, e.cc_name.clone()),
            ir::Item::Namespace(n) => (n.id, n.cc_name.identifier.clone()),
            ir::Item::IncompleteRecord(r) => (r.id, r.cc_name.identifier.clone()),
            ir::Item::Record(r) => (r.id, r.cc_name.identifier.clone()),
            ir::Item::Enum(e) => (e.id, e.cc_name.identifier.clone()),
            ir::Item::Constant(c) => (c.id, c.cc_name.identifier.clone()),
            ir::Item::GlobalVar(g) => (g.id, g.cc_name.identifier.clone()),
            ir::Item::TypeAlias(t) => (t.id, t.cc_name.identifier.clone()),
        };
        let qualifier = self.namespace_qualifier_from_id(id).format_for_cc_debug();
        return format! {"{qualifier}{name}"}.into();
    }

    pub fn cc_type_debug_name(&self, cc_type: &CcType) -> String {
        let base_name = match &cc_type.variant {
            ir::CcTypeVariant::Primitive(p) => match p {
                ir::Primitive::Bool => "bool",
                ir::Primitive::Void => "void",
                ir::Primitive::Float => "float",
                ir::Primitive::Double => "double",
                ir::Primitive::Char => "char",
                ir::Primitive::SignedChar => "signed char",
                ir::Primitive::UnsignedChar => "unsigned char",
                ir::Primitive::Short => "short",
                ir::Primitive::Int => "int",
                ir::Primitive::Long => "long",
                ir::Primitive::LongLong => "long long",
                ir::Primitive::UnsignedShort => "unsigned short",
                ir::Primitive::UnsignedInt => "unsigned int",
                ir::Primitive::UnsignedLong => "unsigned long",
                ir::Primitive::UnsignedLongLong => "unsigned long long",
                ir::Primitive::Char16T => "char16_t",
                ir::Primitive::Char32T => "char32_t",
                ir::Primitive::PtrdiffT => "ptrdiff_t",
                ir::Primitive::IntptrT => "intptr_t",
                ir::Primitive::SizeT => "size_t",
                ir::Primitive::UintptrT => "uintptr_t",
                ir::Primitive::StdPtrdiffT => "std::ptrdiff_t",
                ir::Primitive::StdIntptrT => "std::intptr_t",
                ir::Primitive::StdSizeT => "std::size_t",
                ir::Primitive::StdUintptrT => "std::uintptr_t",
                ir::Primitive::Int8T => "int8_t",
                ir::Primitive::Int16T => "int16_t",
                ir::Primitive::Int32T => "int32_t",
                ir::Primitive::Int64T => "int64_t",
                ir::Primitive::StdInt8T => "std::int8_t",
                ir::Primitive::StdInt16T => "std::int16_t",
                ir::Primitive::StdInt32T => "std::int32_t",
                ir::Primitive::StdInt64T => "std::int64_t",
                ir::Primitive::Uint8T => "uint8_t",
                ir::Primitive::Uint16T => "uint16_t",
                ir::Primitive::Uint32T => "uint32_t",
                ir::Primitive::Uint64T => "uint64_t",
                ir::Primitive::StdUint8T => "std::uint8_t",
                ir::Primitive::StdUint16T => "std::uint16_t",
                ir::Primitive::StdUint32T => "std::uint32_t",
                ir::Primitive::StdUint64T => "std::uint64_t",
            }
            .to_string(),
            ir::CcTypeVariant::Pointer(ptr) => {
                let ptr_str = match ptr.kind {
                    ir::PointerTypeKind::LValueRef => "&",
                    ir::PointerTypeKind::RValueRef => "&&",
                    ir::PointerTypeKind::Nullable
                    | ir::PointerTypeKind::NonNull
                    | ir::PointerTypeKind::Owned => "*",
                };
                let pointee_name = self.cc_type_debug_name(&ptr.pointee_type);
                format!("{pointee_name}{ptr_str}")
            }
            ir::CcTypeVariant::FuncPointer { .. } => "function pointer".to_string(),
            ir::CcTypeVariant::Decl { id, .. } => self.debug_name(*id).to_string(),
            ir::CcTypeVariant::Error(err) => format!("<error: {}>", err.message),
        };

        if cc_type.is_const {
            if matches!(cc_type.variant, ir::CcTypeVariant::Pointer(_)) {
                format!("{} const", base_name)
            } else {
                format!("const {}", base_name)
            }
        } else {
            base_name
        }
    }

    pub fn new_unsupported_item(
        &self,
        item: &impl GenericItem,
        path: Option<ir::UnsupportedItemPath>,
        error: Option<Rc<ir::FormattedError>>,
        cause: Option<Error>,
        must_bind: bool,
    ) -> ir::UnsupportedItem {
        ir::UnsupportedItem::new_raw(
            self.debug_name(item.id()),
            item.unique_name(),
            item.unsupported_kind(),
            item.id(),
            item.source_loc(),
            self.defining_target(item.id()),
            must_bind,
            path,
            error,
            cause,
        )
    }

    pub fn new_unsupported_item_with_static_message(
        &self,
        item: &impl GenericItem,
        path: Option<ir::UnsupportedItemPath>,
        message: &'static str,
    ) -> ir::UnsupportedItem {
        self.new_unsupported_item(
            item,
            path,
            Some(Rc::new(ir::FormattedError { fmt: message.into(), message: message.into() })),
            None,
            item.must_bind(),
        )
    }

    pub fn new_unsupported_item_with_cause(
        &self,
        item: &impl GenericItem,
        path: Option<ir::UnsupportedItemPath>,
        cause: Error,
    ) -> ir::UnsupportedItem {
        self.new_unsupported_item(item, path, None, Some(cause), item.must_bind())
    }

    pub fn error_item_name(&self, item_id: ir::ItemId) -> error_report::ItemName {
        let name = self.debug_name(item_id);
        let item = self.find_untyped_decl(item_id);
        error_report::ItemName {
            name,
            id: item.id().as_u64(),
            unique_name: item.unique_name(),
            defining_target: self
                .defining_target(item.id())
                .map(|ir::BazelLabel(label)| std::rc::Rc::clone(&label)),
        }
    }

    pub fn error_scope<'a>(&'a self, item_id: ir::ItemId) -> Option<error_report::ItemScope<'a>> {
        let item = self.find_untyped_decl(item_id);
        if matches!(item, ir::Item::Comment(_) | ir::Item::UseMod(_)) {
            None
        } else {
            Some(error_report::ItemScope::new(self.errors(), self.error_item_name(item_id)))
        }
    }

    pub fn assert_in_error_scope(&self, item_id: ir::ItemId) {
        self.errors().assert_in_item(self.error_item_name(item_id));
    }

    pub fn item_for_type<T>(&self, ty: &T) -> arc_anyhow::Result<&'db ir::Item>
    where
        T: ir::TypeWithDeclId + std::fmt::Debug,
    {
        if let Some(decl_id) = ty.decl_id() {
            Ok(self.find_untyped_decl(decl_id))
        } else {
            arc_anyhow::bail!("Type {:?} does not have an associated item.", ty)
        }
    }

    #[track_caller]
    pub fn find_decl<T>(&self, decl_id: ir::ItemId) -> arc_anyhow::Result<&'db T>
    where
        &'db T: TryFrom<&'db ir::Item>,
    {
        self.find_untyped_decl(decl_id).try_into().map_err(|_| {
            arc_anyhow::anyhow!(
                "DeclId {:?} doesn't refer to a {}",
                decl_id,
                std::any::type_name::<T>()
            )
        })
    }

    #[track_caller]
    pub fn find_untyped_decl(&self, decl_id: ir::ItemId) -> &'db ir::Item {
        let Some(item) = self.ir().get_decl(decl_id) else {
            panic!("Couldn't find decl_id {:?} in the IR:\n{:#?}", decl_id, self.ir().tree_ir())
        };
        item
    }

    pub fn namespace_qualifier(
        &self,
        item: &impl ir::GenericItem,
    ) -> code_gen_utils::NamespaceQualifier {
        self.namespace_qualifier_from_id(item.id())
    }

    #[track_caller]
    pub fn namespace_qualifier_from_id(
        &self,
        item_id: ir::ItemId,
    ) -> code_gen_utils::NamespaceQualifier {
        let mut namespaces = vec![];
        let mut nested_records = vec![];
        let mut enclosing_item_id = self.find_untyped_decl(item_id).enclosing_item_id();
        while let Some(parent_id) = enclosing_item_id {
            match self.find_untyped_decl(parent_id) {
                ir::Item::Namespace(ns) => {
                    namespaces.push(ns.rs_name.identifier.clone());
                    enclosing_item_id = ns.enclosing_item_id;
                }
                ir::Item::Record(parent_record) => {
                    assert!(
                        namespaces.is_empty(),
                        "Record was listed as the enclosing item for a namespace, this is a bug."
                    );
                    let module_name =
                        self.record_to_associated_module_name(parent_record.clone()).unwrap();
                    nested_records.push((
                        module_name.to_string().into(),
                        parent_record.cc_name.identifier.clone(),
                    ));
                    enclosing_item_id = parent_record.enclosing_item_id;
                }
                ir::Item::ExistingRustType(rust_type) => {
                    assert!(
                        namespaces.is_empty(),
                        "An existing rust type was listed as the enclosing item for a namespace, this is a bug."
                    );
                    nested_records.push((rust_type.rs_name.clone(), rust_type.cc_name.clone()));
                    // The cc_name and rs_name are fully qualified already.
                    enclosing_item_id = None;
                }
                item => {
                    panic!("Expected namespace or parent record, found enclosing item: {item:#?}");
                }
            }
        }
        namespaces.reverse();
        nested_records.reverse();
        let use_leading_colons =
            if let Some(target) = self.find_untyped_decl(item_id).owning_target() {
                self.ir()
                    .target_crubit_features(&target)
                    .contains(crubit_feature::CrubitFeature::LeadingColonsForCppType)
            } else {
                // We default to true here because the final change will always be to add `::` to
                // the beginning of the type.
                true
            };
        code_gen_utils::NamespaceQualifier { namespaces, nested_records, use_leading_colons }
    }

    /// Returns the name of the snake-cased module that exposes the given record's nested items.
    pub fn record_to_associated_module_name(
        &self,
        record: Rc<Record>,
    ) -> Result<proc_macro2::Ident> {
        let record_name: &str = record.rs_name.as_str();
        let snake_case_name = record_name.to_snake_case();
        // Add an `_items` suffix to distinguish the module name if the record name is already snake-case,
        // then distinguish by adding `_` suffixes until we find a name that is not in use.
        let mut name = if snake_case_name == record_name {
            format!("{}_items", snake_case_name)
        } else {
            snake_case_name
        };

        let resolved_names = self.resolve_names(record.clone())?;
        let is_used = |n: &str| match resolved_names.get(n) {
            Some(ResolvedName::RecordNestedItems { parent_records_that_map_to_this_name }) => {
                !parent_records_that_map_to_this_name.contains(&record.id)
            }
            Some(_) => true,
            None => false,
        };
        if is_used(&name) {
            if !name.ends_with("_items") {
                name = format!("{}_items", name);
            }
            while is_used(&name) {
                name.push('_');
            }
        }
        Ok(make_rs_ident(&name))
    }
}
