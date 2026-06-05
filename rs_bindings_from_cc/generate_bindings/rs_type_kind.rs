// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, ensure, Error, Result};
use database::code_snippet::{NoBindingsReason, Visibility};
use database::rs_snippet::{Lifetime, LifetimeOptions, Mutability, RsTypeKind, RustPtrKind};
use database::BindingsGenerator;
use ir::GenericItem;
use ir::{CcType, CcTypeVariant, PointerTypeKind};
use lifetime_defaults_transform::lifetime_defaults_transform_item;
use std::rc::Rc;

fn pointee_is_string_view(db: &BindingsGenerator, ty: &CcType) -> bool {
    match ty.variant {
        CcTypeVariant::Decl { id, .. } => {
            let item = db.find_untyped_decl(id);
            if let ir::Item::Record(record) = item {
                record.is_string_view()
            } else {
                false
            }
        }
        _ => false,
    }
}

fn item_is_or_aliases_string_view(db: &BindingsGenerator, item: &ir::Item) -> bool {
    match item {
        ir::Item::Record(record) => record.is_string_view(),
        ir::Item::TypeAlias(type_alias) => match &type_alias.underlying_type.variant {
            CcTypeVariant::Decl { id, .. } => {
                item_is_or_aliases_string_view(db, db.find_untyped_decl(*id))
            }
            _ => false,
        },
        _ => false,
    }
}

/// Implementation of `BindingsGenerator::rs_type_kind`.
pub fn rs_type_kind_with_lifetime_elision(
    db: &BindingsGenerator,
    ty: CcType,
    lifetime_options: LifetimeOptions,
) -> Result<RsTypeKind> {
    ensure!(
        ty.unknown_attr.is_empty(),
        "crubit.rs/errors/unknown_attribute: unknown attribute(s): {}",
        ty.unknown_attr
    );
    match &ty.variant {
        CcTypeVariant::Primitive(primitive) => Ok(RsTypeKind::Primitive(*primitive)),
        CcTypeVariant::Pointer(pointer) => {
            // In Rust, we have no such concept of a "const" type. All types can be either
            // mutable or immutable depending on the context. However, we do have mutable and
            // immutable references/pointers, where the mutability determines whether the pointee
            // can be mutated.
            // This is not the case in C++, where whether or not the pointee can be mutated is
            // a property of the mutability of the pointee, not the pointer. To map this logic into
            // Rust, we use the mutability of the C++ pointee to determine the mutability of the
            // Rust pointer (as opposed to the mutability of the C++ pointer to determine the
            // mutability of the Rust pointer, e.g. ty.is_const).
            let mutability =
                if pointer.pointee_type.is_const { Mutability::Const } else { Mutability::Mut };
            let mut pointee = db.rs_type_kind_with_lifetime_elision(
                pointer.pointee_type.as_ref().clone(),
                LifetimeOptions {
                    assume_lifetimes: lifetime_options.assume_lifetimes
                        && !pointee_is_string_view(db, &pointer.pointee_type),
                    // is_return_type is used in !assume_lifetimes contexts for absl::span
                    // to determine whether lifetimes should be provided.
                    is_return_type: lifetime_options.is_return_type
                        && lifetime_options.assume_lifetimes,
                    is_operator: lifetime_options.is_operator,
                    ..LifetimeOptions::default()
                },
            )?;

            // TODO(b/464492052): Support bridge types by pointer/reference.
            if let RsTypeKind::BridgeType { original_type, .. } = pointee.unalias() {
                let visibility_override = if pointee.is_proto_message_bridge_type() {
                    Some(Visibility::Public)
                } else {
                    None
                };
                pointee = RsTypeKind::Error {
                    symbol: cpp_type_name::cpp_tagless_type_name_for_record(original_type, db)?
                        .to_string()
                        .into(),
                    error: anyhow!("Bridging types are not supported as pointee/referent types."),
                    visibility_override,
                };
            }
            let pointee = Rc::new(pointee);

            let lifetime = if lifetime_options.assume_lifetimes {
                match &ty.explicit_lifetimes[..] {
                    [] => Lifetime::elided(),
                    [name] => Lifetime::new(name),
                    _ => return Err(anyhow!("pointers may only have one lifetime")),
                }
            } else {
                match pointer.lifetime {
                    Some(lifetime_id) => db
                        .ir()
                        .get_lifetime(lifetime_id)
                        .map(Lifetime::from)
                        .ok_or_else(|| anyhow!("no known lifetime with id {lifetime_id:?}"))?,
                    None if lifetime_options.infer_lifetimes => Lifetime::elided(),
                    None => {
                        return Ok(RsTypeKind::Pointer {
                            pointee,
                            kind: RustPtrKind::CcPtr(pointer.kind),
                            mutability,
                        })
                    }
                }
            };
            Ok(match pointer.kind {
                PointerTypeKind::LValueRef => {
                    let is_cref = lifetime_options.assume_lifetimes
                        && (!pointee.is_complete()
                            || (lifetime_options.is_return_type && !lifetime_options.is_operator));
                    RsTypeKind::Reference {
                        referent: pointee,
                        mutability,
                        lifetime,
                        // lifetime_defaults_transform should never give us an LValueRef without
                        // a lifetime assignment.
                        is_cref,
                    }
                }
                PointerTypeKind::RValueRef => {
                    RsTypeKind::RvalueReference { referent: pointee, mutability, lifetime }
                }
                // Note: this conversion discards information about the nullability and lifetime
                // of the pointer. In the future, we may wish to consume this information along
                // with a user-provided annotation in order to convert some pointers into either
                // references or optional references.
                PointerTypeKind::NonNull | PointerTypeKind::Nullable | PointerTypeKind::Owned => {
                    RsTypeKind::Pointer {
                        pointee,
                        kind: RustPtrKind::CcPtr(pointer.kind),
                        mutability,
                    }
                }
            })
        }
        CcTypeVariant::FuncPointer {
            non_null,
            call_conv,
            param_and_return_types,
            lifetime_inputs: _,
        } => {
            let (return_type, param_types) = param_and_return_types
                .split_last()
                .expect("In well-formed IR function pointers include at least the return type");
            let return_type = Rc::new(db.rs_type_kind_with_lifetime_elision(
                return_type.clone(),
                LifetimeOptions {
                    assume_lifetimes: lifetime_options.assume_lifetimes,
                    ..LifetimeOptions::default()
                },
            )?);
            let param_types = param_types
                .iter()
                .map(|param_type| {
                    db.rs_type_kind_with_lifetime_elision(
                        param_type.clone(),
                        LifetimeOptions {
                            assume_lifetimes: lifetime_options.assume_lifetimes,
                            ..LifetimeOptions::default()
                        },
                    )
                })
                .collect::<Result<Rc<[_]>>>()?;
            ensure!(
                param_types
                    .iter()
                    .chain(Some(return_type.as_ref()))
                    .all(|t| t.is_c_abi_compatible_by_value()),
                "Either the return type or some of the parameter types require an FFI thunk (and function pointers don't have a thunk)",
            );
            Ok(RsTypeKind::FuncPtr {
                option: !*non_null,
                cc_calling_conv: *call_conv,
                return_type,
                param_types,
            })
        }
        CcTypeVariant::Decl { id, template_args } => {
            let item = db.find_untyped_decl(*id);

            if let Err(no_bindings_reason) = db.has_bindings(item.clone()) {
                let error: Error;
                let unsupported_alias_error = || {
                    use ir::GenericItem;
                    anyhow!("Unsupported type alias {name}", name = db.debug_name(item.id()))
                };
                // Alias fallbacks: type aliases are unique among items, in that if the item
                // defining the alias fails to receive bindings, we can still use the aliased type.
                if let ir::Item::TypeAlias(alias) = item {
                    // Additionally, we should not "see through" type aliases that are specifically
                    // not on targets that intend to support Rust users of those type aliases.
                    // (If we did, then a C++ library owner could break Rust callers, which is a
                    // maintenance responsibility that they did not sign up for!)
                    if !matches!(
                        no_bindings_reason,
                        NoBindingsReason::MissingRequiredFeatures { .. }
                    ) {
                        if let Ok(ty) = db.rs_type_kind_with_lifetime_elision(
                            alias.underlying_type.clone(),
                            lifetime_options,
                        ) {
                            return Ok(ty);
                        } else {
                            // TODO(b/481368622): this fails if we fall through to comprehensive fallbacks.
                            return Err(unsupported_alias_error());
                        }
                    }
                    // If it still fails, hide the error. Most users only need to know
                    // "Alias does not exist", not "decltype(declval<T>().x()) does not exist".
                    error = unsupported_alias_error();
                } else {
                    error = no_bindings_reason.into();
                }
                // Comprehensive fallbacks: if we can delay reifying the error, delay it.
                if let Ok(symbol) = cpp_type_name::tagless_cpp_type_name_for_item(item, db) {
                    return Ok(RsTypeKind::Error {
                        symbol: symbol.to_string().into(),
                        error,
                        visibility_override: None,
                    });
                }
                return Err(error);
            }

            let (decl_assumes_lifetimes, item) = {
                if let Some(owning_target) = item.owning_target() {
                    let decl_assumes_lifetimes = db
                        .ir()
                        .target_crubit_features(&owning_target)
                        .contains(crubit_feature::CrubitFeature::AssumeLifetimes)
                        && !item_is_or_aliases_string_view(db, item);
                    (
                        decl_assumes_lifetimes,
                        if decl_assumes_lifetimes {
                            lifetime_defaults_transform_item(db, item)?
                        } else {
                            item.clone()
                        },
                    )
                } else {
                    (false, item.clone())
                }
            };

            let lifetimes: Vec<Lifetime> =
                if decl_assumes_lifetimes || lifetime_options.assume_lifetimes {
                    ty.explicit_lifetimes.iter().map(|lt| Lifetime::new(lt)).collect()
                } else {
                    vec![]
                };

            // This is the implementation of `BindingsGenerator::rs_type_kind()`, so of
            // course we can't call `rs_type_kind` here, and instead reuse the raw construction
            // logic.
            RsTypeKind::from_item_raw(db, item, &lifetime_options, template_args, &lifetimes)
        }
        CcTypeVariant::Error(e) => {
            let e = error_report::FormattedError::new(
                e.fmt.to_string().into(),
                e.message.to_string().into(),
            );
            Err(e.into())
        }
    }
}
