// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, ensure, Error, Result};
use database::code_snippet::{NoBindingsReason, Visibility};
use database::rs_snippet::{Lifetime, LifetimeOptions, Mutability, RsTypeKind, RustPtrKind};
use database::BindingsGenerator;
use ir::{CcType, CcTypeVariant, PointerTypeKind};
use std::rc::Rc;

/// Implementation of `BindingsGenerator::rs_type_kind`.
pub fn rs_type_kind_with_lifetime_elision(
    db: &dyn BindingsGenerator,
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
            let mut pointee = db.rs_type_kind(pointer.pointee_type.as_ref().clone())?;

            // TODO(b/464492052): Support bridge types by pointer/reference.
            if let RsTypeKind::BridgeType { original_type, .. } = pointee.unalias() {
                let visibility_override = if pointee.is_proto_message_bridge_type() {
                    Some(Visibility::Public)
                } else {
                    None
                };
                pointee = RsTypeKind::Error {
                    symbol: cpp_type_name::cpp_tagless_type_name_for_record(
                        original_type,
                        db.ir(),
                    )?
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
                    RsTypeKind::Reference { referent: pointee, mutability, lifetime }
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
        CcTypeVariant::Decl(id) => {
            let ir = db.ir();
            let item = ir.find_untyped_decl(*id);

            if let Err(no_bindings_reason) = db.has_bindings(item.clone()) {
                let error: Error;
                let unsupported_alias_error = || {
                    use ir::GenericItem;
                    anyhow!("Unsupported type alias {name}", name = item.debug_name(db.ir()))
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
                if let Ok(symbol) = cpp_type_name::tagless_cpp_type_name_for_item(item, db.ir()) {
                    return Ok(RsTypeKind::Error {
                        symbol: symbol.to_string().into(),
                        error,
                        visibility_override: None,
                    });
                }
                return Err(error);
            }

            // This is the implementation of `BindingsGenerator::rs_type_kind()`, so of
            // course we can't call `rs_type_kind` here, and instead reuse the raw construction
            // logic.
            RsTypeKind::from_item_raw(
                db,
                item.clone(),
                lifetime_options.have_reference_param,
                lifetime_options.is_return_type,
            )
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
