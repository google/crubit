// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, Context, Error, Result};
use database::code_snippet::{
    required_crubit_features, BindingsInfo, NoBindingsReason, RequiredCrubitFeature, Visibility,
};
use database::db;
use database::rs_snippet::RsTypeKind;
use database::BindingsGenerator;
use ir::{BazelLabel, Func, GenericItem, Item};
use std::rc::Rc;

/// Implementation of `BindingsGenerator::has_bindings`.
pub fn has_bindings(
    db: &dyn BindingsGenerator,
    item: Item,
) -> Result<BindingsInfo, NoBindingsReason> {
    let ir = db.ir();

    match required_crubit_features(db, &item) {
        Ok(missing_features) if missing_features.is_empty() => {}
        Ok(missing_features) => {
            return Err(NoBindingsReason::MissingRequiredFeatures {
                context: item.debug_name(&db.ir()),
                missing_features,
            });
        }
        Err(error) => {
            return Err(NoBindingsReason::DependencyFailed {
                context: item.debug_name(&db.ir()),
                error,
            });
        }
    }

    if let Some(parent) = item.enclosing_item_id() {
        let parent = ir.find_untyped_decl(parent);

        if let Err(no_parent_bindings) = db.has_bindings(parent.clone()) {
            return Err(NoBindingsReason::DependencyFailed {
                context: item.debug_name(ir),
                error: no_parent_bindings.into(),
            });
        }

        // TODO(b/200067824): Allow nested type items inside records.
        if item.is_type_definition() {
            if let ir::Item::Record(_) = parent {
                return Err(NoBindingsReason::Unsupported {
                    context: item.debug_name(ir),
                    error: anyhow!(
                        "b/200067824: type definitions nested inside records are not yet supported"
                    ),
                });
            }
        }
    }

    if let Item::Enum(enum_) = &item {
        if enum_.enumerators.is_none() {
            return Err(NoBindingsReason::Unsupported {
                context: enum_.debug_name(ir),
                error: anyhow!(
                    "b/322391132: Forward-declared (opaque) enums are not implemented yet"
                ),
            });
        }
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
        | Item::TypeMapOverride(_) => {
            // has_bindings is called from `rs_type_kind()`, so we can't use
            // `BindingsGenerator::rs_type_kind()` here.
            match RsTypeKind::from_item_raw(db, item.clone()) {
                Ok(rs_type_kind) => {
                    let visibility = type_visibility(db, &item, rs_type_kind)?;
                    Ok(BindingsInfo { visibility })
                }
                Err(error) => {
                    Err(NoBindingsReason::DependencyFailed { context: item.debug_name(ir), error })
                }
            }
        }
        // Global variables receive bindings if the underlying type is visible.
        Item::GlobalVar(ref global_var) => match db.rs_type_kind(global_var.type_.clone()) {
            Ok(rs_type_kind) => {
                let visibility = type_visibility(db, &item, rs_type_kind)?;
                Ok(BindingsInfo { visibility })
            }
            Err(error) => {
                Err(NoBindingsReason::DependencyFailed { context: item.debug_name(ir), error })
            }
        },
        // Other items are public.
        Item::UnsupportedItem(_) | Item::Comment(_) | Item::Namespace(_) | Item::UseMod(_) => {
            Ok(BindingsInfo { visibility: Visibility::Public })
        }
    }
}

/// Returns function-specific `has_bindings` information.
fn func_has_bindings(
    db: &dyn BindingsGenerator,
    func: Rc<Func>,
) -> Result<BindingsInfo, NoBindingsReason> {
    let ir = db.ir();
    let target = &func.owning_target;
    let enabled_features = ir.target_crubit_features(target);
    // Check for non-Unpin return/parameter types.
    // When we release non-Unpin types by value, this whole complicated check will go away.

    let mut missing_features = vec![];
    let mut has_nonunpin = false;

    let mut require_nonunpin =
        |missing_features: &mut Vec<RequiredCrubitFeature>,
         rs_type_kind: RsTypeKind,
         location: &dyn Fn() -> std::borrow::Cow<'static, str>| {
            if rs_type_kind.is_unpin() {
                return;
            }
            has_nonunpin = true;
            // TODO(b/409128537): On next binary release, add `"wrapper"` to `:experimental`,
            // and then change this to:
            //  `!enabled_features.contains(crubit_feature::CrubitFeature::Wrapper)`.
            if !enabled_features.is_disjoint(
                crubit_feature::CrubitFeature::Experimental
                    | crubit_feature::CrubitFeature::Wrapper,
            ) {
                return;
            }
            let location = location();
            missing_features.push(RequiredCrubitFeature {
                target: target.clone(),
                item: func.debug_name(ir),
                missing_features: crubit_feature::CrubitFeature::Wrapper.into(),
                capability_description: format!(
                    "<internal link>_relocatable_error: {location} is not rust-movable"
                )
                .into(),
            });
        };
    let require_visible = |old_visibility: &mut Visibility,
                           all_missing_features: &mut Vec<RequiredCrubitFeature>,
                           rs_type_kind: RsTypeKind| {
        let new_visibility = match type_visibility(db, &func, rs_type_kind) {
            Ok(vis) => vis,
            Err(NoBindingsReason::MissingRequiredFeatures { context: _, mut missing_features }) => {
                all_missing_features.append(&mut missing_features);
                // Keep going using public for now, we're not going to generate bindings anyway.
                Visibility::Public
            }
            Err(other_reason) => unreachable!("{:#?}", Error::from(other_reason)),
        };
        if *old_visibility == Visibility::Public {
            *old_visibility = new_visibility;
        }
    };

    let return_type = db.rs_type_kind(func.return_type.clone()).unwrap();
    require_nonunpin(&mut missing_features, return_type.clone(), &|| "the return type".into());
    let mut visibility = Visibility::Public;
    require_visible(&mut visibility, &mut missing_features, return_type);

    for (i, param) in func.params.iter().enumerate() {
        let param_type = db.rs_type_kind(param.type_.clone()).unwrap();
        require_nonunpin(&mut missing_features, param_type.clone(), &|| {
            format!("{} (parameter #{i})", &param.identifier).into()
        });

        require_visible(&mut visibility, &mut missing_features, param_type);
    }

    if !missing_features.is_empty() {
        return Err(NoBindingsReason::MissingRequiredFeatures {
            context: func.debug_name(db.ir()),
            missing_features,
        });
    }

    if has_nonunpin && !enabled_features.contains(crubit_feature::CrubitFeature::Experimental) {
        visibility = Visibility::PubCrate;
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
    db: &dyn BindingsGenerator,
    rs_type_kind: RsTypeKind,
) -> Result<Option<BazelLabel>> {
    // We visit `self` twice, but it doesn't matter, we just need a starting value.
    let mut most_restricted_subtype = type_target_restriction_shallow(db, rs_type_kind.clone());
    for child_type in rs_type_kind.dfs_iter() {
        intersect_target_restrictions(
            db,
            &mut most_restricted_subtype,
            type_target_restriction_shallow(db, child_type.clone()),
        )
        .with_context(|| {
            format!("<internal link>_visibility_error: {} has child types which are `pub(crate)` in two different crates, and cannot be used", rs_type_kind.display(db))
        })?;
    }
    Ok(most_restricted_subtype.target)
}

/// A type representing a visibility restriction: if `target == Some("//foo:bar")`, the type
/// is a `pub(crate)` type defined in `//foo:bar`. If `target == None`, the type is `pub` and usable
/// by any crate in any target.
struct TargetRestriction {
    /// If `None`, the type is `pub`. Otherwise, it's the target the type is pub(crate) for.
    target: Option<BazelLabel>,
    /// The type which is `pub(crate)`, used for error messages.
    exemplar_type: RsTypeKind,
}

/// Updates `old_restriction`: if `new_restriction` is `pub(crate)` while
/// `old_restriction` is not, then `old_restriction` is updated to be `new_restriction`.
///
/// Returns an error if both are `pub(crate)`, and the two types are owned by different crates.
/// The error contains just a list of the types it found that are incompatible.
fn intersect_target_restrictions(
    db: &dyn BindingsGenerator,
    old_restriction: &mut TargetRestriction,
    new_restriction: TargetRestriction,
) -> Result<()> {
    match (&old_restriction.target, &new_restriction.target) {
        (_, None) => {}
        (Some(old_label), Some(new_label)) => {
            if old_label != new_label {
                let old_type = old_restriction.exemplar_type.display(db);
                let new_type = new_restriction.exemplar_type.display(db);
                // The top-line error message is built in the caller, with these as
                // a list of causes.
                return Err(anyhow!("{old_type} is `pub(crate)` in {old_label}")
                    .context(format!("{new_type} is `pub(crate)` in {new_label}")));
            }
        }
        (_, _) => {
            *old_restriction = new_restriction;
        }
    }
    Ok(())
}

/// Without recursing, returns the visibility restriction of the top-level compound data type.
///
/// For example, the top level visibility restriction of `*mut T` is `None` for all `T`, because
/// pointers are never `pub(crate)`, only their pointees can be.
fn type_target_restriction_shallow(
    db: &dyn BindingsGenerator,
    rs_type_kind: RsTypeKind,
) -> TargetRestriction {
    let mut target = match &rs_type_kind {
        RsTypeKind::IncompleteRecord { incomplete_record, .. } => {
            Some(&incomplete_record.owning_target)
        }
        // TODO(b/410575605): check for the template instantiation allowlist (string_view etc.),
        // and do:
        //
        // RsTypeKind::Record { record, .. } if record.defining_target.is_some() && !allowlist => {
        //     Some(&record.owning_target)
        // }
        _ => None,
    };

    // Targets with experimental features generate `pub` bindings (for now?), no matter what.
    if let Some(some_target) = target {
        if db
            .ir()
            .target_crubit_features(some_target)
            .contains(crubit_feature::CrubitFeature::Experimental)
        {
            target = None;
        }
    }
    TargetRestriction { target: target.cloned(), exemplar_type: rs_type_kind }
}

fn type_visibility(
    db: &dyn BindingsGenerator,
    item: &dyn GenericItem,
    rs_type_kind: RsTypeKind,
) -> Result<Visibility, NoBindingsReason> {
    let Some(target) = item.owning_target() else {
        return Ok(Visibility::Public);
    };
    match db::type_visibility(db, &target, rs_type_kind.clone()) {
        Ok(vis) => Ok(vis),
        Err(error) => {
            let missing_features = vec![RequiredCrubitFeature {
                target: target.clone(),
                // slightly hacky: we didn't keep track of which item in the type in particular
                // is causing a visibility restriction, but we can stringify the whole type.
                item: rs_type_kind.display(db).to_string().into(),
                // All visibility restrictions are turned off in `:experimental`.
                missing_features: crubit_feature::CrubitFeature::Experimental.into(),
                // again a slight hack.
                capability_description: error.to_string().into(),
            }];
            Err(NoBindingsReason::MissingRequiredFeatures {
                context: item.debug_name(db.ir()),
                missing_features,
            })
        }
    }
}
