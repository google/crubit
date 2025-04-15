// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::anyhow;
use database::code_snippet::{required_crubit_features, HasBindings, NoBindingsReason};
use database::BindingsGenerator;
use ir::{GenericItem, Item};

#[must_use]
pub fn has_bindings(db: &dyn BindingsGenerator, item: Item) -> HasBindings {
    let ir = db.ir();

    match required_crubit_features(db, &item) {
        Ok(missing_features) if missing_features.is_empty() => {}
        Ok(missing_features) => {
            return HasBindings::No(NoBindingsReason::MissingRequiredFeatures {
                context: item.debug_name(&db.ir()),
                missing_features,
            });
        }
        Err(error) => {
            return HasBindings::No(NoBindingsReason::DependencyFailed {
                context: item.debug_name(&db.ir()),
                error,
            });
        }
    }

    if let Some(parent) = item.enclosing_item_id() {
        let parent = ir.find_untyped_decl(parent);

        match db.has_bindings(parent.clone()) {
            HasBindings::No(no_parent_bindings) => {
                return HasBindings::No(NoBindingsReason::DependencyFailed {
                    context: item.debug_name(&ir),
                    error: no_parent_bindings.into(),
                });
            }
            HasBindings::Yes => {}
        }

        // TODO(b/200067824): Allow nested type items inside records.
        if item.is_type_definition() {
            if let ir::Item::Record(_) = parent {
                return HasBindings::No(NoBindingsReason::Unsupported {
                    context: item.debug_name(&ir),
                    error: anyhow!(
                        "b/200067824: type definitions nested inside records are not yet supported"
                    ),
                });
            }
        }
    }

    match item {
        Item::TypeAlias(alias) => match db.rs_type_kind(alias.underlying_type.clone()) {
            Ok(_) => HasBindings::Yes,
            Err(error) => HasBindings::No(NoBindingsReason::DependencyFailed {
                context: alias.debug_name(&ir),
                error,
            }),
        },
        Item::Enum(enum_) => match db.generate_enum(enum_.clone()) {
            Ok(_) => HasBindings::Yes,
            Err(error) => HasBindings::No(NoBindingsReason::DependencyFailed {
                context: enum_.debug_name(ir),
                error,
            }),
        },
        // TODO(b/392882224): Records might not generated if an error occurs in generation.
        _ => HasBindings::Yes,
    }
}
