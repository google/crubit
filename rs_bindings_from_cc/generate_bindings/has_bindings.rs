// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use crubit_feature::CrubitFeature;
use database::code_snippet::{missing_feature_descriptions, NoBindingsReason, ResolvedName};
use database::rs_snippet::{LifetimeOptions, RsTypeKind};
use database::{intern, BindingsGenerator};
use error_report::{anyhow, bail};
use heck::ToSnakeCase;
use ir::{CcType, CcTypeVariant, Func, GenericItem, Item, ItemId, Record};
use std::collections::HashMap;
use std::rc::Rc;

use generate_function_thunk;

/// Recursively checks whether all C++ declarations (`Decl`) directly or transitively
/// referenced by `cc_type` have bindings.
fn cc_type_has_bindings<'a>(
    db: &BindingsGenerator<'a>,
    cc_type: &CcType,
    alias_id: ItemId,
) -> Result<(), NoBindingsReason> {
    match cc_type.variant() {
        CcTypeVariant::Decl { id, template_args } => {
            let underlying_item = db.find_untyped_decl(*id);
            if let ir::Item::TypeAlias(inner_alias) = underlying_item {
                cc_type_has_bindings(db, inner_alias.underlying_type(), alias_id)?;
            } else if let Err(no_bindings) = db.has_bindings(underlying_item.clone()) {
                return Err(NoBindingsReason::DependencyFailed {
                    type_name: db.debug_name(alias_id),
                    source: no_bindings.into(),
                });
            }
            if let Some(args) = template_args {
                for arg in args.iter() {
                    cc_type_has_bindings(db, arg, alias_id)?;
                }
            }
        }
        CcTypeVariant::Pointer(ptr) => {
            cc_type_has_bindings(db, ptr.pointee_type(), alias_id)?;
        }
        CcTypeVariant::ConstantArray(arr) => {
            cc_type_has_bindings(db, arr.element_type(), alias_id)?;
        }
        CcTypeVariant::FuncPointer { param_and_return_types, .. } => {
            for t in param_and_return_types.iter() {
                cc_type_has_bindings(db, t, alias_id)?;
            }
        }
        CcTypeVariant::Primitive(_) | CcTypeVariant::Error(_) => {}
    }
    Ok(())
}

/// Implementation of `BindingsGenerator::has_bindings`.
pub fn has_bindings<'a>(
    db: &BindingsGenerator<'a>,
    item: Item<'a>,
) -> Result<(), NoBindingsReason> {
    if let Some(name) = item.cc_name_as_str() {
        // Dunder namespaces are allowed for now.
        if name.starts_with("__") && !matches!(item, Item::Namespace(_)) {
            return Err(NoBindingsReason::LeadingDunder { name: name.to_string() });
        }
    }

    match missing_feature_descriptions(db, &item) {
        Ok(missing_features) if missing_features.is_empty() => {}
        Ok(missing_features) => {
            return Err(NoBindingsReason::MissingRequiredFeatures { missing_features });
        }
        Err(error) => return Err(NoBindingsReason::Unsupported(error)),
    }

    if let Some(parent) = item.enclosing_item_id() {
        let parent = db.find_untyped_decl(parent);

        if let Err(no_parent_bindings) = db.has_bindings(parent.clone()) {
            return Err(NoBindingsReason::DependencyFailed {
                type_name: db.debug_name(item.id()),
                source: no_parent_bindings.into(),
            });
        }

        if let Item::Record(parent_record) = parent {
            if item.is_type_definition()
                // If we have an ancestor that is a template specialization, we can't generate bindings.
                // The parent check ensures that all ancestors are checked as well.
                && parent_record.template_specialization().is_some()
            {
                return Err(NoBindingsReason::Unsupported(anyhow!(
                    "b/485949049: type definitions nested inside templated records are not yet supported"
                )));
            }

            if item.place_in_nested_module_if_nested_in_record() {
                // Our parent will be the module generated to hold nested items of the parent
                // record. So we try to resolve all the names in the namespace of the parent record,
                // and then seeing what the parent module name resolves to. If it resolves to the
                // parent module, and it was unique, great! If it resolves to something else, that
                // means it got overwritten. That would mean this item's parent cannot be generated,
                // so we cannot generate this item.
                let resolved_names = db
                    .resolve_names(parent_record.clone())
                    .expect("enclosing_item_id should always be a record or a namespace");

                let (parent_module_name, parent_records_that_map_to_this_name) = resolved_names
                    .iter()
                    .find_map(|(name, resolved_name)| {
                        if let ResolvedName::RecordNestedItems {
                            parent_records_that_map_to_this_name,
                        } = resolved_name
                            && parent_records_that_map_to_this_name.contains(&parent_record.id())
                        {
                            return Some((
                                name.clone(),
                                parent_records_that_map_to_this_name.clone(),
                            ));
                        }
                        None
                    })
                    .ok_or_else(|| {
                        NoBindingsReason::Unsupported(anyhow!(
                            "crubit.rs/errors/nested_type: Could not find parent's module name.\
                        \n  This is a bug. The parent's module name should always be\
                        \n  in the list. More info:\
                        \n    for item: {item_name}\
                        \n    inside parent record {parent_name}",
                            item_name = db.debug_name(item.id()),
                            parent_name = db.debug_name(parent_record.id()),
                        ))
                    })?;
                if parent_records_that_map_to_this_name.len() > 1 {
                    return Err(NoBindingsReason::ParentModuleNameNotUnique {
                        conflicting_name: parent_module_name.to_string(),
                        parent_names_that_map_to_same_name: parent_records_that_map_to_this_name
                            .iter()
                            .map(|&parent_record_id| {
                                db.find_decl::<Rc<Record>>(parent_record_id)
                                    .unwrap()
                                    .rs_name()
                                    .as_str()
                                    .to_string()
                            })
                            .collect(),
                    });
                }
            }
        }
    }

    if let Item::Enum(enum_) = &item {
        if enum_.enumerators().is_none() {
            return Err(NoBindingsReason::Unsupported(anyhow!(
                "b/322391132: Forward-declared (opaque) enums are not implemented yet"
            )));
        }
        // Require that the underlying type exists. Otherwise, the enum can't.
        //
        // NOTE: this cannot form a cycle: while rs_type_kind may call has_bindings, the
        // underlying type is never going to be or refer to this type, because the current
        // enum is not defined at the time that the underlying type is evaluated.
        // Not even forward declarations help. You just can't do `enum Foo: Something<Foo>;`.
        if let Err(error) = db.rs_type_kind(enum_.underlying_type().clone()) {
            return Err(NoBindingsReason::DependencyFailed {
                type_name: db.debug_name(enum_.id()),
                source: error,
            });
        }
    }
    // Require that the underlying type exists. Otherwise, the constant can't.
    if let Item::Constant(constant) = &item
        && let Some(type_) = constant.type_()
        && let Err(error) = db.rs_type_kind(type_.clone())
    {
        return Err(NoBindingsReason::DependencyFailed {
            type_name: db.debug_name(constant.id()),
            source: error,
        });
    }

    // TODO(b/392882224): Records might not generated if an error occurs in generation.
    match item {
        // Functions receive bindings based on their parameter and return types.
        Item::Func(func) => func_has_bindings(db, func),
        // Types receive bindings if the RsTypeKind that they are the definition for succeeds.
        Item::IncompleteRecord(_)
        | Item::Record(_)
        | Item::Enum(_)
        | Item::TypeAlias(_)
        | Item::ExistingRustType(_) => {
            // decl_lifetime_arity is safe to call on non-AssumeLifetimes Items (inasmuch as it
            // won't panic), but doing so may change the behavior of programs that don't have the
            // feature turned on.
            if matches!(item, Item::Record(_)) {
                // Only check those item kinds that decl_lifetime_arity explicitly supports.
                if let Some(ot) = &item.owning_target()
                    && db.ir().target_crubit_features(ot).contains(CrubitFeature::AssumeLifetimes)
                    && let Err(error) = (db.codegen_functions().decl_lifetime_arity)(db, item.id())
                {
                    return Err(NoBindingsReason::Unsupported(error));
                }
            }
            if let Item::TypeAlias(alias) = &item {
                cc_type_has_bindings(db, alias.underlying_type(), alias.id())?;
            }
            // has_bindings is called from `rs_type_kind()`, so we can't use
            // `BindingsGenerator::rs_type_kind()` here.
            match RsTypeKind::from_item_raw(
                db,
                item.clone(),
                &LifetimeOptions { is_return_type: true, ..Default::default() },
                /* template_args= */ &None,
                /* lifetimes= */ &[],
            ) {
                Ok(rs_type_kind) => {
                    // A few bridge types cannot be aliased: see `BridgeRsTypeKind::is_aliasable`.
                    if matches!(item, Item::TypeAlias(_))
                        && let Some(bridge_type) = rs_type_kind.as_bridge_type()
                        && !bridge_type.is_aliasable()
                    {
                        return Err(NoBindingsReason::Unsupported(anyhow!(
                            "Type alias for {cpp_type} suppressed due to being a bridge type",
                            cpp_type = db.debug_name(item.id()),
                        )));
                    }
                    Ok(())
                }
                Err(error) => Err(NoBindingsReason::DependencyFailed {
                    type_name: db.debug_name(item.id()),
                    source: error,
                }),
            }
        }
        // Global variables receive bindings if the underlying type has bindings.
        Item::GlobalVar(ref global_var) => match db.rs_type_kind(global_var.type_().clone()) {
            Ok(_) => Ok(()),
            Err(error) => Err(NoBindingsReason::DependencyFailed {
                type_name: db.debug_name(item.id()),
                source: error,
            }),
        },
        // Other items are public.
        Item::Comment(_)
        | Item::Constant(_)
        | Item::Namespace(_)
        | Item::UnsupportedItem(_)
        | Item::UseMod(_) => Ok(()),
    }
}

/// Returns function-specific `has_bindings` information.
fn func_has_bindings<'a>(
    db: &BindingsGenerator<'a>,
    func: Rc<Func<'a>>,
) -> Result<(), NoBindingsReason> {
    if func.is_consteval() {
        return Err(NoBindingsReason::Unsupported(anyhow!(
            "consteval functions are not supported"
        )));
    }

    if func.is_variadic() && !generate_function_thunk::can_skip_cc_thunk(db, &func) {
        return Err(NoBindingsReason::Unsupported(anyhow!(
            "Variadic functions requiring a C++ thunk are not supported"
        )));
    }

    if let Some(unknown_attr) = func.unknown_attr() {
        for attr in unknown_attr.split(',') {
            let attr = attr.trim();
            if attr == "enable_if" || attr == "diagnose_if" {
                return Err(NoBindingsReason::Unsupported(anyhow!(
                    "Functions with enable_if or diagnose_if attributes are not supported"
                )));
            }
        }
    }

    let ir = db.ir();
    let target = func.as_ref().owning_target();
    let enabled_features = ir.target_crubit_features(target);

    if matches!(func.cc_name(), ir::UnqualifiedIdentifier::ConversionOperator)
        && !enabled_features.contains(CrubitFeature::AssumeThisLifetimes)
    {
        return Err(NoBindingsReason::Unsupported(anyhow!(
            "Conversion operators are only supported when AssumeThisLifetimes is enabled"
        )));
    }

    let mut missing_features = vec![];

    if func.is_member_or_descendant_of_class_template()
        && *func.rs_name() != ir::UnqualifiedIdentifier::Destructor
        && !enabled_features.contains(CrubitFeature::TemplateInstantiation)
    {
        missing_features.push(anyhow!(
            "b/248542210: template instantiation of member function cannot reliably get bindings"
        ));
    }

    let sig_types =
        func.params().iter().map(|p| p.type_()).chain(std::iter::once(func.return_type()));
    for sig_type in sig_types {
        if let Err(source) = db.rs_type_kind(sig_type.clone()) {
            return Err(NoBindingsReason::DependencyFailed {
                type_name: db.debug_name(func.id()),
                source,
            });
        }
    }

    if !missing_features.is_empty() {
        return Err(NoBindingsReason::MissingRequiredFeatures { missing_features });
    }

    Ok(())
}

enum NameConflictAction {
    DoNotUpdate,
    Overwrite,
    Coalesce,
}

fn determine_name_conflict_action<'a>(
    db: &BindingsGenerator<'a>,
    old_resolved_name: &ResolvedName,
    new_resolved_name: &ResolvedName,
) -> NameConflictAction {
    let ResolvedName::ExplicitItem(new_id) = new_resolved_name else {
        return NameConflictAction::Coalesce;
    };
    let ResolvedName::ExplicitItem(old_id) = old_resolved_name else {
        return NameConflictAction::Coalesce;
    };
    let old_item = db.find_untyped_decl(*old_id);
    let new_item = db.find_untyped_decl(*new_id);
    match (old_item, new_item) {
        (Item::ExistingRustType(old_ert), Item::ExistingRustType(new_ert)) => {
            if old_ert.rs_name() == new_ert.rs_name() {
                NameConflictAction::DoNotUpdate
            } else {
                NameConflictAction::Coalesce
            }
        }
        (Item::IncompleteRecord(_), Item::Record(_)) => NameConflictAction::Overwrite,
        (Item::Record(_), Item::IncompleteRecord(_)) => NameConflictAction::DoNotUpdate,
        _ => NameConflictAction::Coalesce,
    }
}

/// Resolves names to a map from name to ResolvedName.
///
/// This checks both type and value namespaces.
pub fn resolve_names<'a>(
    db: &BindingsGenerator<'a>,
    parent: Rc<Record<'a>>,
) -> Result<Rc<HashMap<Rc<str>, ResolvedName>>> {
    let child_items = match parent.enclosing_item_id().map(|id| db.find_untyped_decl(id)) {
        Some(Item::Namespace(ns)) => ns.children().iter(),
        Some(Item::Record(record)) => record.children().iter(),
        None => db.ir().top_level_items_in_target(parent.as_ref().owning_target()).iter(),
        _ => bail!("not a parent namespace or record"),
    };

    let mut names: HashMap<Rc<str>, ResolvedName> = HashMap::new();
    {
        let mut insert = |name: Rc<str>, resolved_type_name: ResolvedName| {
            use std::collections::hash_map::Entry::*;
            match names.entry(name) {
                Vacant(vacant) => {
                    vacant.insert(resolved_type_name);
                }
                Occupied(mut occupied) => {
                    let action =
                        determine_name_conflict_action(db, occupied.get(), &resolved_type_name);
                    match action {
                        NameConflictAction::DoNotUpdate => {}
                        NameConflictAction::Overwrite => {
                            occupied.insert(resolved_type_name);
                        }
                        NameConflictAction::Coalesce => {
                            let name = occupied.key().clone();
                            occupied.get_mut().coalesce(resolved_type_name).unwrap_or_else(|e| {
                                panic!(
                                    "name collision for '{}', this should never happen: {}",
                                    name, e
                                );
                            });
                        }
                    }
                }
            }
        };

        for item in child_items.clone() {
            let id = item.id();
            match item {
                Item::IncompleteRecord(incomplete_record) => {
                    insert(
                        Rc::from(incomplete_record.rs_name().as_str()),
                        ResolvedName::ExplicitItem(id),
                    );
                }
                Item::Record(record) => {
                    insert(Rc::from(record.rs_name().as_str()), ResolvedName::ExplicitItem(id));
                }
                Item::Enum(enum_) => {
                    insert(Rc::from(enum_.rs_name().as_str()), ResolvedName::ExplicitItem(id))
                }
                Item::TypeAlias(type_alias) => {
                    insert(Rc::from(type_alias.rs_name().as_str()), ResolvedName::ExplicitItem(id));
                }
                Item::Namespace(ns) => {
                    insert(
                        Rc::from(ns.rs_name().as_str()),
                        ResolvedName::Namespace {
                            canonical_namespace_id: ns.canonical_namespace_id(),
                        },
                    );
                }
                Item::UseMod(use_mod) => {
                    insert(Rc::from(use_mod.mod_name().as_str()), ResolvedName::ExplicitItem(id));
                }
                Item::ExistingRustType(existing_rust_type) => {
                    insert(Rc::from(existing_rust_type.rs_name()), ResolvedName::ExplicitItem(id));
                }
                Item::Func(func) => {
                    if let ir::UnqualifiedIdentifier::Identifier(ident) = func.rs_name() {
                        insert(Rc::from(ident.as_str()), ResolvedName::ValueItem(id));
                    }
                }
                Item::Constant(constant) => {
                    insert(Rc::from(constant.rs_name().as_str()), ResolvedName::ValueItem(id));
                }
                Item::GlobalVar(global_var) => {
                    insert(Rc::from(global_var.rs_name().as_str()), ResolvedName::ValueItem(id));
                }
                Item::Comment(_) | Item::UnsupportedItem(_) => {}
            }
        }
    }

    // Pass 2: Insert module names for records, checking for conflicts.
    for item in child_items {
        if let Item::Record(record) = item {
            let id = record.id();
            let make_module_for_nested_items = record
                .children()
                .iter()
                .any(|child| child.place_in_nested_module_if_nested_in_record());
            if make_module_for_nested_items {
                let mut name = record.rs_name().as_str().to_snake_case();

                // Disambiguation logic
                if name == record.rs_name().as_str() {
                    name = format!("{}_items", name);
                }

                let is_used = |n: &str| names.contains_key(n);

                if is_used(&name) {
                    if !name.ends_with("_items") {
                        name = format!("{}_items", name);
                    }
                    while is_used(&name) {
                        name.push('_');
                    }
                }

                match names.entry(intern!(db.interner(), "{name}")) {
                    std::collections::hash_map::Entry::Vacant(vacant) => {
                        vacant.insert(ResolvedName::RecordNestedItems {
                            parent_records_that_map_to_this_name: vec![id],
                        });
                    }
                    std::collections::hash_map::Entry::Occupied(_) => {
                        panic!("name collision after disambiguation");
                    }
                }
            }
        }
    }

    Ok(Rc::new(names))
}
