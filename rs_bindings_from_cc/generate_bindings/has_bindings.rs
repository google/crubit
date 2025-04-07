// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, Error};
use database::code_snippet::{required_crubit_features, RequiredCrubitFeature};
use database::BindingsGenerator;
use ir::{GenericItem, Item};
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
pub enum HasBindings {
    /// This item is guaranteed to have bindings. If the translation unit
    /// defining the item fails to generate bindings for it, it will not
    /// compile.
    Yes,

    /// This item is not guaranteed to have bindings. There is no way to tell if
    /// bindings were generated unless the item is defined in the current
    /// translation unit.
    Maybe,

    /// These bindings are guaranteed not to exist.
    No(NoBindingsReason),
}

#[derive(Clone, PartialEq, Eq)]
pub enum NoBindingsReason {
    MissingRequiredFeatures {
        context: Rc<str>,
        missing_features: Vec<RequiredCrubitFeature>,
    },
    DependencyFailed {
        context: Rc<str>,
        error: Error,
    },
    /// This is directly unsupported.
    Unsupported {
        context: Rc<str>,
        error: Error,
    },
}

#[must_use]
pub fn has_bindings(db: &dyn BindingsGenerator, item: &Item) -> HasBindings {
    let ir = db.ir();

    match required_crubit_features(db, item) {
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

        match has_bindings(db, parent) {
            HasBindings::No(no_parent_bindings) => {
                return HasBindings::No(NoBindingsReason::DependencyFailed {
                    context: item.debug_name(&ir),
                    error: no_parent_bindings.into(),
                });
            }
            HasBindings::Maybe => {
                // This shouldn't happen, Maybe is meant for Func items.
                return HasBindings::No(NoBindingsReason::DependencyFailed {
                    context: item.debug_name(&ir),
                    error: anyhow!("parent item might not be defined"),
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
        // Function bindings aren't guaranteed, because they don't _need_ to be guaranteed. We
        // choose not to generate code which relies on functions existing in other TUs.
        Item::Func(..) => HasBindings::Maybe,
        Item::TypeAlias(alias) => match db.rs_type_kind(alias.underlying_type.cpp_type.clone()) {
            Ok(_) => HasBindings::Yes,
            Err(error) => HasBindings::No(NoBindingsReason::DependencyFailed {
                context: alias.debug_name(&ir),
                error,
            }),
        },
        Item::Enum(enum_) => match db.generate_enum(Rc::clone(enum_)) {
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

impl From<NoBindingsReason> for Error {
    fn from(reason: NoBindingsReason) -> Error {
        match reason {
            NoBindingsReason::MissingRequiredFeatures { context, missing_features } => {
                // This maybe could use .context(), but the ordering is backward.
                let mut all_missing = vec![];
                for missing in missing_features {
                    all_missing.push(missing.to_string());
                }
                anyhow!(
                    "Can't generate bindings for {context}, because of missing required features (<internal link>):\n{}",
                    all_missing.join("\n")
                )
            }
            NoBindingsReason::DependencyFailed { context, error } => error.context(format!(
                "Can't generate bindings for {context} due to missing bindings for its dependency"
            )),
            NoBindingsReason::Unsupported { context, error } => error.context(format!(
                "Can't generate bindings for {context}, because it is unsupported"
            )),
        }
    }
}
