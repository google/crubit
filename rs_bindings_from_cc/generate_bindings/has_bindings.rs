// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use crubit_feature::CrubitFeature;
use database::code_snippet::{
    missing_feature_descriptions, BindingsInfo, NoBindingsReason, ResolvedName, Visibility,
};
use database::rs_snippet::{LifetimeOptions, RsTypeKind};
use database::BindingsGenerator;
use error_report::{anyhow, bail};
use heck::ToSnakeCase;
use ir::{BazelLabel, Func, GenericItem, Item, ItemId, Record};
use std::collections::HashMap;
use std::rc::Rc;

/// Implementation of `BindingsGenerator::has_bindings`.
pub fn has_bindings(db: &BindingsGenerator, item: Item) -> Result<BindingsInfo, NoBindingsReason> {
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
                type_name: db.debug_name(item.id()).to_string(),
                reason: no_parent_bindings.to_string(),
            });
        }

        if let Item::Record(parent_record) = parent {
            if item.is_type_definition()
                // If we have an ancestor that is a template specialization, we can't generate bindings.
                // The parent check ensures that all ancestors are checked as well.
                && parent_record.template_specialization.is_some()
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
                            && parent_records_that_map_to_this_name.contains(&parent_record.id)
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
                            parent_name = db.debug_name(parent_record.id),
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
                                    .rs_name
                                    .identifier
                                    .to_string()
                            })
                            .collect(),
                    });
                }
            }
        }
    }

    if let Item::Enum(enum_) = &item {
        if enum_.enumerators.is_none() {
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
        if let Err(error) = db.rs_type_kind(enum_.underlying_type.clone()) {
            return Err(NoBindingsReason::DependencyFailed {
                type_name: db.debug_name(enum_.id()).to_string(),
                reason: error.to_string(),
            });
        }
    }
    // Require that the underlying type exists. Otherwise, the constant can't.
    if let Item::Constant(constant) = &item
        && let Err(error) = db.rs_type_kind(constant.type_.clone())
    {
        return Err(NoBindingsReason::DependencyFailed {
            type_name: db.debug_name(constant.id()).to_string(),
            reason: error.to_string(),
        });
    }

    // TODO(b/392882224): Records might not generated if an error occurs in generation.
    match item {
        // Functions receive bindings based on their parameter and return types.
        Item::Func(func) => func_has_bindings(db, func),
        // Types receive bindings with the same visibility (and success) as the RsTypeKind that
        // they are the definition for.
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
            // has_bindings is called from `rs_type_kind()`, so we can't use
            // `BindingsGenerator::rs_type_kind()` here.
            match RsTypeKind::from_item_raw(
                db,
                item.clone(),
                &LifetimeOptions { is_return_type: true, ..Default::default() },
                /*template_args=*/ &None,
                /*lifetimes=*/ &[],
            ) {
                Ok(rs_type_kind) => {
                    if matches!(item, Item::TypeAlias(_)) && rs_type_kind.unalias().is_bridge_type()
                    {
                        return Err(NoBindingsReason::Unsupported(anyhow!(
                            "Type alias for {cpp_type} suppressed due to being a bridge type",
                            cpp_type = db.debug_name(item.id()),
                        )));
                    }
                    let visibility = type_visibility(db, &item, rs_type_kind)?;
                    Ok(BindingsInfo { visibility })
                }
                Err(error) => Err(NoBindingsReason::DependencyFailed {
                    type_name: db.debug_name(item.id()).to_string(),
                    reason: error.to_string(),
                }),
            }
        }
        // Global variables receive bindings if the underlying type is visible.
        Item::GlobalVar(ref global_var) => match db.rs_type_kind(global_var.type_.clone()) {
            Ok(rs_type_kind) => {
                let visibility = type_visibility(db, &item, rs_type_kind)?;
                Ok(BindingsInfo { visibility })
            }
            Err(error) => Err(NoBindingsReason::DependencyFailed {
                type_name: db.debug_name(item.id()).to_string(),
                reason: error.to_string(),
            }),
        },
        // Other items are public.
        Item::Comment(_)
        | Item::Constant(_)
        | Item::Namespace(_)
        | Item::UnsupportedItem(_)
        | Item::UseMod(_) => Ok(BindingsInfo { visibility: Visibility::Public }),
    }
}

/// Returns function-specific `has_bindings` information.
fn func_has_bindings(
    db: &BindingsGenerator,
    func: Rc<Func>,
) -> Result<BindingsInfo, NoBindingsReason> {
    if func.is_consteval {
        return Err(NoBindingsReason::Unsupported(anyhow!(
            "consteval functions are not supported"
        )));
    }

    let ir = db.ir();
    let target = &func.owning_target;
    let enabled_features = ir.target_crubit_features(target);

    if matches!(func.cc_name, ir::UnqualifiedIdentifier::ConversionOperator)
        && !enabled_features.contains(CrubitFeature::AssumeThisLifetimes)
    {
        return Err(NoBindingsReason::Unsupported(anyhow!(
            "Conversion operators are only supported when AssumeThisLifetimes is enabled"
        )));
    }

    let mut missing_features = vec![];

    if func.is_member_or_descendant_of_class_template
        && func.rs_name != ir::UnqualifiedIdentifier::Destructor
        && !enabled_features.contains(CrubitFeature::TemplateInstantiation)
    {
        missing_features.push(
            "b/248542210: template instantiation of member function cannot reliably get bindings"
                .to_string(),
        );
    }

    let mut visibility = Visibility::Public;
    let sig_types = func.params.iter().map(|p| &p.type_).chain(std::iter::once(&func.return_type));
    for sig_type in sig_types {
        let rs_type_kind = db.rs_type_kind(sig_type.clone()).unwrap();
        match type_visibility(db, &func, rs_type_kind) {
            Ok(vis) => {
                visibility = visibility.or(vis);
            }
            Err(NoBindingsReason::MissingRequiredFeatures {
                missing_features: new_missing_features,
            }) => {
                missing_features.extend(new_missing_features);
                // Keep going using public for now, we're not going to generate bindings anyway.
            }
            Err(other_reason) => return Err(other_reason),
        };
    }

    if !missing_features.is_empty() {
        return Err(NoBindingsReason::MissingRequiredFeatures { missing_features });
    }

    Ok(BindingsInfo { visibility })
}

/// Returns the set of crates which can use the type due it depending on a `pub(crate)` item.
///
/// - If no subtype is `pub(crate)`, returns `None`.
/// - If more than one subtype is `pub(crate)`, for two or more crates, returns `Err`.
/// - Otherwise, returns the crate which owns the `pub(crate)` subtype.
///
/// For example, if two targets `//foo:crate1` and `//foo:crate2` independently define a type
/// `pub(crate) struct X;`, then `&crate1::X` has a restriction of Ok(Some("//foo:crate1")),
/// while `(crate1::X, crate2::X)` is `Err` (because no crate can use both).
//
// Implementation notes:
//
// This in some amount duplicates the logic in `RsTypeKind::required_crubit_features`, but
// is kept distinct. In particular, for example, some types are going to be restricted in
// which features they require (e.g. `Wrapper` and above), and _separately_, are going to
// be `pub(crate)` when enabled. That logic does not need to be separated -- in principle,
// we could produce both results at once -- but because `required_crubit_features` is used
// to collect useful error messages, while this is used to compute visibility, they end up
// structured a bit differently, and it's difficult to share the code.
//
// YMMV: feel free to unify the two functions later.
pub fn type_target_restriction(
    db: &BindingsGenerator,
    rs_type_kind: RsTypeKind,
) -> Result<Option<BazelLabel>> {
    let mut dfs_iter = rs_type_kind.dfs_iter();
    // `unwrap()` is safe because we know there is at least the `Self type.
    let mut most_restricted_subtype = type_target_restriction_shallow(db, dfs_iter.next().unwrap());
    for child_type in dfs_iter {
        intersect_target_restrictions(
            db,
            &rs_type_kind,
            &mut most_restricted_subtype,
            type_target_restriction_shallow(db, child_type),
        )?;
    }
    Ok(most_restricted_subtype.map(|r| r.target))
}

/// A visibility restriction indicating that a type is `pub(crate)` within a specific target.
struct TargetRestriction {
    /// The target which provides the `pub(crate)` type.
    target: BazelLabel,
    /// The type which is `pub(crate)`, used for error messages.
    exemplar_type: RsTypeKind,
}

/// Updates `old_restriction`: if `new_restriction` is `pub(crate)` while
/// `old_restriction` is not, then `old_restriction` is updated to be `new_restriction`.
///
/// Returns an error if both are `pub(crate)`, and the two types are owned by different crates.
/// The error contains just a list of the types it found that are incompatible.
fn intersect_target_restrictions(
    db: &BindingsGenerator,
    original_type: &RsTypeKind,
    old_restriction: &mut Option<TargetRestriction>,
    new_restriction: Option<TargetRestriction>,
) -> Result<()> {
    if let Some(old_restriction) = old_restriction.as_ref()
        && let Some(new_restriction) = new_restriction.as_ref()
        && let old_target = &old_restriction.target
        && let new_target = &new_restriction.target
        && old_target != new_target
    {
        let original_type = original_type.display(db);
        let old_type = old_restriction.exemplar_type.display(db);
        let new_type = new_restriction.exemplar_type.display(db);
        // The top-line error message is built in the caller, with these as
        // a list of causes.
        return Err(anyhow!(
            "`{original_type}` depends on `pub(crate)` types from other targets:\n\
              {old_type} is `pub(crate)` in {old_target}\n\
              {new_type} is `pub(crate)` in {new_target}\n\
              See http://crubit.rs/errors/visibility"
        ));
    }
    if old_restriction.is_none() {
        *old_restriction = new_restriction;
    }
    Ok(())
}

/// Without recursing, returns the visibility restriction of the top-level compound data type.
///
/// For example, the top level visibility restriction of `*mut T` is `None` for all `T`, because
/// pointers are never `pub(crate)`, only their pointees can be.
fn type_target_restriction_shallow(
    db: &BindingsGenerator,
    rs_type_kind: &RsTypeKind,
) -> Option<TargetRestriction> {
    let rs_type_kind = rs_type_kind.unalias();
    let RsTypeKind::Record { record, .. } = rs_type_kind else {
        // All non-record types are `pub` if they receive bindings.
        return None;
    };
    let target = &record.owning_target;
    // Template types (except for the special-cased ones like `[w]string_view`)
    // are the only types whose bindings have restrictions, and they do not have
    // unique owning targets.
    if record.has_unique_owning_target()
        || db.ir().target_crubit_features(target).contains(CrubitFeature::TemplateInstantiation)
    {
        return None;
    }
    // Instantiations of UniformReprTemplateTypes are unrestricted.
    if let RsTypeKind::Record { uniform_repr_template_type: Some(_), .. } = rs_type_kind {
        return None;
    }
    // Targets with experimental features generate `pub` bindings (for now?), no matter what.
    if db.ir().target_crubit_features(target).contains(CrubitFeature::Experimental) {
        return None;
    }
    Some(TargetRestriction { target: target.clone(), exemplar_type: rs_type_kind.clone() })
}

fn type_visibility(
    db: &BindingsGenerator,
    item: &dyn GenericItem,
    rs_type_kind: RsTypeKind,
) -> Result<Visibility, NoBindingsReason> {
    let Some(target) = item.owning_target() else {
        return Ok(Visibility::Public);
    };
    db.type_visibility(&target, rs_type_kind.clone()).map_err(NoBindingsReason::Visibility)
}

enum NameConflictAction {
    DoNotUpdate,
    Overwrite,
    Coalesce,
}

fn determine_name_conflict_action(
    db: &BindingsGenerator,
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
            if old_ert.rs_name == new_ert.rs_name {
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
pub fn resolve_names(
    db: &BindingsGenerator,
    parent: Rc<Record>,
) -> Result<Rc<HashMap<Rc<str>, ResolvedName>>> {
    let child_items = match parent.enclosing_item_id.map(|id| db.find_untyped_decl(id)) {
        Some(Item::Namespace(ns)) => ns.children.iter(),
        Some(Item::Record(record)) => record.children.iter(),
        None => db.ir().top_level_items_in_target(&parent.owning_target).iter(),
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
                        incomplete_record.rs_name.identifier.clone(),
                        ResolvedName::ExplicitItem(id),
                    );
                }
                Item::Record(record) => {
                    insert(record.rs_name.identifier.clone(), ResolvedName::ExplicitItem(id));
                }
                Item::Enum(enum_) => {
                    insert(enum_.rs_name.identifier.clone(), ResolvedName::ExplicitItem(id))
                }
                Item::TypeAlias(type_alias) => {
                    insert(type_alias.rs_name.identifier.clone(), ResolvedName::ExplicitItem(id));
                }
                Item::Namespace(ns) => {
                    insert(
                        ns.rs_name.identifier.clone(),
                        ResolvedName::Namespace {
                            canonical_namespace_id: ns.canonical_namespace_id,
                        },
                    );
                }
                Item::UseMod(use_mod) => {
                    insert(use_mod.mod_name.identifier.clone(), ResolvedName::ExplicitItem(id));
                }
                Item::ExistingRustType(existing_rust_type) => {
                    insert(existing_rust_type.rs_name.clone(), ResolvedName::ExplicitItem(id));
                }
                Item::Func(func) => {
                    if let ir::UnqualifiedIdentifier::Identifier(ident) = &func.rs_name {
                        insert(ident.identifier.clone(), ResolvedName::ValueItem(id));
                    }
                }
                Item::Constant(constant) => {
                    insert(constant.rs_name.identifier.clone(), ResolvedName::ValueItem(id));
                }
                Item::GlobalVar(global_var) => {
                    insert(global_var.rs_name.identifier.clone(), ResolvedName::ValueItem(id));
                }
                Item::Comment(_) | Item::UnsupportedItem(_) => {}
            }
        }
    }

    // Pass 2: Insert module names for records, checking for conflicts.
    for item in child_items {
        if let Item::Record(record) = item {
            let id = record.id;
            let make_module_for_nested_items = record
                .children
                .iter()
                .any(|child| child.place_in_nested_module_if_nested_in_record());
            if make_module_for_nested_items {
                let mut name = record.rs_name.identifier.as_ref().to_snake_case();

                // Disambiguation logic
                if name == record.rs_name.identifier.as_ref() {
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

                match names.entry(name.into()) {
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
