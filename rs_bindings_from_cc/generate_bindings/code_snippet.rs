// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Generate the final bindings, including structures for code snippet, feature
/// gating, etc.
use crate::db::BindingsGenerator;
use proc_macro2::{Ident, TokenStream};

use crate::rs_snippet::{RsTypeKind, TypeLocation};
use arc_anyhow::Result;
use ffi_types::FfiU8SliceBox;
use ir::{BazelLabel, GenericItem, Item, UnqualifiedIdentifier};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

/// FFI equivalent of `Bindings`.
#[repr(C)]
pub struct FfiBindings {
    pub rs_api: FfiU8SliceBox,
    pub rs_api_impl: FfiU8SliceBox,
    pub error_report: FfiU8SliceBox,
    pub fatal_errors: FfiU8SliceBox,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ApiSnippets {
    /// Main API - for example:
    /// - A Rust definition of a function (with a doc comment),
    /// - A Rust definition of a struct (with a doc comment).
    pub main_api: TokenStream,

    /// Rust implementation details - for example:
    /// - A Rust declaration of an `extern "C"` thunk,
    /// - Rust static assertions about struct size, aligment, and field offsets.
    pub thunks: TokenStream,
    pub assertions: TokenStream,

    /// C++ implementation details - for example:
    /// - A C++ implementation of an `extern "C"` thunk,
    /// - C++ static assertions about struct size, aligment, and field offsets.
    pub cc_details: TokenStream,

    pub features: BTreeSet<Ident>,
}

impl From<TokenStream> for ApiSnippets {
    fn from(main_api: TokenStream) -> Self {
        ApiSnippets { main_api, ..Default::default() }
    }
}

/// Source code for generated bindings.
pub(crate) struct Bindings {
    // Rust source code.
    pub rs_api: String,
    // C++ source code.
    pub rs_api_impl: String,
}

/// Source code for generated bindings, as tokens.
///
/// This is public within the crate for testing purposes.
pub(crate) struct BindingsTokens {
    // Rust source code.
    pub rs_api: TokenStream,
    // C++ source code.
    pub rs_api_impl: TokenStream,
}

/// A missing set of crubit features caused by a capability that requires that
/// feature.
///
/// For example, if addition is not implemented due to missing the Experimental
/// feature on //foo, then we might have something like:
///
/// ```
/// RequiredCrubitFeature {
///   target: "//foo".into(),
///   item: "kFoo".into(),
///   missing_features: CrubitFeature::Experimental.into(),
///   capability_description: "int addition".into(),
/// }
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RequiredCrubitFeature {
    pub target: BazelLabel,
    pub item: Rc<str>,
    pub missing_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    pub capability_description: Rc<str>,
}

impl Display for RequiredCrubitFeature {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Self { target, item, missing_features, capability_description } = self;
        let feature_strings: Vec<&str> =
            missing_features.into_iter().map(|feature| feature.aspect_hint()).collect();
        write!(f, "{target} needs [{features}] for {item}", features = feature_strings.join(", "),)?;
        if !capability_description.is_empty() {
            write!(f, " ({capability_description})")?;
        }
        Ok(())
    }
}

/// Returns the list of features required to use the item which are not yet
/// enabled.
///
/// If the item doesn't have a defining target, the return value is meaningless,
/// and bindings will always be generated.
///
/// If the item does have a defining target, and it doesn't enable the specified
/// features, then bindings are suppressed for this item.
pub fn required_crubit_features(
    db: &dyn BindingsGenerator,
    item: &Item,
) -> Result<Vec<RequiredCrubitFeature>> {
    let mut missing_features = vec![];

    let ir = &db.ir();

    let require_any_feature =
        |missing_features: &mut Vec<RequiredCrubitFeature>,
         alternative_required_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
         capability_description: &dyn Fn() -> Rc<str>| {
            // We refuse to generate bindings if either the definition of an item, or
            // instantiation (if it is a template) of an item are in a translation unit
            // which doesn't have the required Crubit features.
            for target in item.defining_target().into_iter().chain(item.owning_target()) {
                let enabled_features = ir.target_crubit_features(target);
                if (alternative_required_features & enabled_features).is_empty() {
                    missing_features.push(RequiredCrubitFeature {
                        target: target.clone(),
                        item: item.debug_name(ir),
                        missing_features: alternative_required_features,
                        capability_description: capability_description(),
                    });
                }
            }
        };

    let require_rs_type_kind = |missing_features: &mut Vec<RequiredCrubitFeature>,
                                rs_type_kind: &RsTypeKind,
                                type_location: TypeLocation,
                                context: &dyn Fn() -> Rc<str>| {
        for target in item.defining_target().into_iter().chain(item.owning_target()) {
            let (missing, desc) = rs_type_kind.required_crubit_features(
                db,
                ir.target_crubit_features(target),
                type_location,
            );
            if !missing.is_empty() {
                let context = context();
                let capability_description = if desc.is_empty() {
                    context
                } else if context.is_empty() {
                    desc.into()
                } else {
                    format!("{context}: {desc}").into()
                };
                missing_features.push(RequiredCrubitFeature {
                    target: target.clone(),
                    item: item.debug_name(ir),
                    missing_features: missing,
                    capability_description,
                });
            }
        }
    };

    if let Some(unknown_attr) = item.unknown_attr() {
        require_any_feature(
            &mut missing_features,
            crubit_feature::CrubitFeature::Experimental.into(),
            &|| format!("unknown attribute(s): {unknown_attr}").into(),
        );
    }
    match item {
        Item::UnsupportedItem(..) => {}
        Item::Func(func) => {
            if func.name == UnqualifiedIdentifier::Destructor {
                // We support destructors in supported even though they use some features we
                // don't generally support with that feature set, because in this
                // particular case, it's safe.
                require_any_feature(
                    &mut missing_features,
                    crubit_feature::CrubitFeature::Supported.into(),
                    &|| "destructors".into(),
                );
            } else {
                let return_type = db.rs_type_kind(func.return_type.rs_type.clone())?;
                require_rs_type_kind(
                    &mut missing_features,
                    &return_type,
                    TypeLocation::FnReturn,
                    &|| "return type".into(),
                );
                for (i, param) in func.params.iter().enumerate() {
                    let param_type = db.rs_type_kind(param.type_.rs_type.clone())?;
                    require_rs_type_kind(
                        &mut missing_features,
                        &param_type,
                        TypeLocation::FnParam,
                        &|| format!("the type of {} (parameter #{i})", &param.identifier).into(),
                    );
                }
                if func.is_extern_c {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Supported.into(),
                        &|| "extern \"C\" function".into(),
                    );
                } else {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Supported.into(),
                        &|| "non-extern \"C\" function".into(),
                    );
                }
                if !func.has_c_calling_convention {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Experimental.into(),
                        &|| "non-C calling convention".into(),
                    );
                }
                if func.is_noreturn {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Experimental.into(),
                        &|| "[[noreturn]] attribute".into(),
                    );
                }
                if func.nodiscard.is_some() {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Experimental.into(),
                        &|| "[[nodiscard]] attribute".into(),
                    );
                }
                if func.deprecated.is_some() {
                    require_any_feature(
                        &mut missing_features,
                        crubit_feature::CrubitFeature::Experimental.into(),
                        &|| "[[deprecated]] attribute".into(),
                    );
                }
                for param in &func.params {
                    if let Some(unknown_attr) = &param.unknown_attr {
                        require_any_feature(
                            &mut missing_features,
                            crubit_feature::CrubitFeature::Experimental.into(),
                            &|| {
                                format!(
                                    "param {param} has unknown attribute(s): {unknown_attr}",
                                    param = &param.identifier.identifier
                                )
                                .into()
                            },
                        );
                    }
                }
            }
        }
        Item::Record(record) => {
            require_rs_type_kind(
                &mut missing_features,
                &RsTypeKind::new_record(db, record.clone(), &db.ir())?,
                TypeLocation::Other,
                &|| "".into(),
            );
        }
        Item::TypeAlias(alias) => {
            require_rs_type_kind(
                &mut missing_features,
                &RsTypeKind::new_type_alias(db, alias.clone())?,
                TypeLocation::Other,
                &|| "".into(),
            );
        }
        Item::Enum(e) => {
            require_rs_type_kind(
                &mut missing_features,
                &RsTypeKind::new_enum(e.clone(), &db.ir())?,
                TypeLocation::Other,
                &|| "".into(),
            );
        }
        Item::Namespace(_) => {
            require_any_feature(
                &mut missing_features,
                crubit_feature::CrubitFeature::Supported.into(),
                &|| "namespace".into(),
            );
        }
        Item::IncompleteRecord(_) => {
            require_any_feature(
                &mut missing_features,
                crubit_feature::CrubitFeature::Experimental.into(),
                &|| "incomplete type".into(),
            );
        }
        Item::Comment { .. } | Item::UseMod { .. } => {}
        Item::TypeMapOverride { .. } => {
            require_any_feature(
                &mut missing_features,
                crubit_feature::CrubitFeature::Experimental.into(),
                &|| "type map override".into(),
            );
        }
    }
    Ok(missing_features)
}
