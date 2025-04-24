// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Generate the final bindings, including structures for code snippet, feature
/// gating, etc.
use crate::db::BindingsGenerator;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::rs_snippet::RsTypeKind;
use arc_anyhow::{anyhow, Error, Result};
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
pub struct ApiSnippets {
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
pub struct Bindings {
    // Rust source code.
    pub rs_api: String,
    // C++ source code.
    pub rs_api_impl: String,
}

/// Source code for generated bindings, as tokens.
///
/// This is public within the crate for testing purposes.
pub struct BindingsTokens {
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
            for target in item.defining_target().into_iter().chain(item.owning_target().as_ref()) {
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
                                context: &dyn Fn() -> Rc<str>| {
        for target in item.defining_target().into_iter().chain(item.owning_target().as_ref()) {
            let (missing, desc) =
                rs_type_kind.required_crubit_features(db, ir.target_crubit_features(target));
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
            if func.rs_name == UnqualifiedIdentifier::Destructor {
                // We support destructors in supported even though they use some features we
                // don't generally support with that feature set, because in this
                // particular case, it's safe.
                require_any_feature(
                    &mut missing_features,
                    crubit_feature::CrubitFeature::Supported.into(),
                    &|| "destructors".into(),
                );
            } else {
                let return_type = db.rs_type_kind(func.return_type.clone())?;
                require_rs_type_kind(&mut missing_features, &return_type, &|| "return type".into());
                for (i, param) in func.params.iter().enumerate() {
                    let param_type = db.rs_type_kind(param.type_.clone())?;
                    require_rs_type_kind(&mut missing_features, &param_type, &|| {
                        format!("the type of {} (parameter #{i})", &param.identifier).into()
                    });
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
        Item::Record(_) | Item::TypeAlias(_) | Item::Enum(_) => {
            require_rs_type_kind(
                &mut missing_features,
                // We use from_item_raw here because required_crubit_features is itself called
                // by `BindingsGenerator::rs_type_kind()` in order to decide if it should return
                // an error.
                &RsTypeKind::from_item_raw(db, item.clone())?,
                &|| "".into(),
            );
        }
        Item::GlobalVar(_) => {}
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

/// Visibility of an item.
///
/// Generally speaking, if an error occurs (e.g. a bindings doesn't exist), then
/// the way to "keep going" to catch more errors is to pretend that the missing
/// item is `Public`.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum Visibility {
    /// The item has `pub` visibility.
    #[default]
    Public,
    /// The item has `pub(crate)` visibility.
    PubCrate,
}

impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Visibility::Public => quote! {pub}.to_tokens(tokens),
            Visibility::PubCrate => quote! {pub(crate)}.to_tokens(tokens),
        }
    }
}

/// Information about the bindings that this item will have.
///
/// When this is returned, and the item is not a `Func`, then the bindings are
/// guaranteed to exist.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct BindingsInfo {
    pub visibility: Visibility,
}

/// Information about why bindings do not exist.
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

impl Display for NoBindingsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Error as Display>::fmt(&Error::from(self.clone()), f)
    }
}

impl std::fmt::Debug for NoBindingsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Error as std::fmt::Debug>::fmt(&Error::from(self.clone()), f)
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
