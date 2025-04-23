// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::anyhow;
use database::code_snippet::{
    required_crubit_features, BindingsInfo, NoBindingsReason, RequiredCrubitFeature, Visibility,
};
use database::rs_snippet::RsTypeKind;
use database::BindingsGenerator;
use ir::{Func, GenericItem, Item};

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

    match item {
        Item::Func(func) => func_has_bindings(db, &func),
        Item::TypeAlias(alias) => match db.rs_type_kind(alias.underlying_type.clone()) {
            Ok(_) => Ok(BindingsInfo { visibility: Visibility::Public }),
            Err(error) => {
                Err(NoBindingsReason::DependencyFailed { context: alias.debug_name(ir), error })
            }
        },
        Item::Enum(enum_) => match db.generate_enum(enum_.clone()) {
            Ok(_) => Ok(BindingsInfo { visibility: Visibility::Public }),
            Err(error) => {
                Err(NoBindingsReason::DependencyFailed { context: enum_.debug_name(ir), error })
            }
        },
        // TODO(b/392882224): Records might not generated if an error occurs in generation.
        _ => Ok(BindingsInfo { visibility: Visibility::Public }),
    }
}

/// Returns function-specific `has_bindings` information.
fn func_has_bindings(
    db: &dyn BindingsGenerator,
    func: &Func,
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

    require_nonunpin(
        &mut missing_features,
        db.rs_type_kind(func.return_type.clone()).unwrap(),
        &|| "the return type".into(),
    );
    for (i, param) in func.params.iter().enumerate() {
        require_nonunpin(
            &mut missing_features,
            db.rs_type_kind(param.type_.clone()).unwrap(),
            &|| format!("{} (parameter #{i})", &param.identifier).into(),
        );
    }

    if !missing_features.is_empty() {
        return Err(NoBindingsReason::MissingRequiredFeatures {
            context: func.debug_name(db.ir()),
            missing_features,
        });
    }

    if has_nonunpin && !enabled_features.contains(crubit_feature::CrubitFeature::Experimental) {
        return Ok(BindingsInfo { visibility: Visibility::PubCrate });
    }
    Ok(BindingsInfo { visibility: Visibility::Public })
}
