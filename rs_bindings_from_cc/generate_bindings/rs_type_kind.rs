// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, ensure, Result};
use database::code_snippet::{NoBindingsReason, Visibility};
use database::rs_snippet::{ElisionOptions, Lifetime, Mutability, RsTypeKind, RustPtrKind};
use database::BindingsGenerator;
use ir::{CcType, CcTypeVariant, PointerTypeKind};
use std::rc::Rc;

/// Implementation of `BindingsGenerator::rs_type_kind`.
pub fn rs_type_kind_with_lifetime_elision(
    db: &dyn BindingsGenerator,
    ty: CcType,
    elision_options: ElisionOptions,
) -> Result<RsTypeKind> {
    ensure!(ty.unknown_attr.is_empty(), "unknown attribute(s): {}", ty.unknown_attr);
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

            // TODO(b/351976044): Support bridge types by pointer/reference.
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

            let lifetime = match pointer.lifetime {
                Some(lifetime_id) => db
                    .ir()
                    .get_lifetime(lifetime_id)
                    .map(Lifetime::from)
                    .ok_or_else(|| anyhow!("no known lifetime with id {lifetime_id:?}"))?,
                None if elision_options.elide_references => Lifetime::elided(),
                None => {
                    return Ok(RsTypeKind::Pointer {
                        pointee,
                        kind: RustPtrKind::CcPtr(pointer.kind),
                        mutability,
                    })
                }
            };
            if let PointerTypeKind::RValueRef = pointer.kind {
                Ok(RsTypeKind::RvalueReference { referent: pointee, mutability, lifetime })
            } else {
                Ok(RsTypeKind::Reference {
                    option: matches!(pointer.kind, PointerTypeKind::Nullable),
                    referent: pointee,
                    mutability,
                    lifetime,
                })
            }
        }
        CcTypeVariant::FuncPointer { non_null, call_conv, param_and_return_types } => {
            let (return_type, param_types) = param_and_return_types
                .split_last()
                .expect("In well-formed IR function pointers include at least the return type");
            let return_type = Rc::new(db.rs_type_kind_with_lifetime_elision(
                return_type.clone(),
                ElisionOptions::default(),
            )?);
            let param_types = param_types
                .iter()
                .map(|param_type| {
                    db.rs_type_kind_with_lifetime_elision(
                        param_type.clone(),
                        ElisionOptions::default(),
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

            if let Err(error) = db.has_bindings(item.clone()) {
                // Alias fallbacks: type aliases are unique among items, in that if the item
                // defining the alias fails to receive bindings, we can still use the aliased type.
                if let ir::Item::TypeAlias(alias) = item {
                    // Additionally, we should not "see through" type aliases that are specifically
                    // not on targets that intend to support Rust users of those type aliases.
                    // (If we did, then a C++ library owner could break Rust callers, which is a
                    // maintenance responsibility that they did not sign up for!)
                    if !matches!(error, NoBindingsReason::MissingRequiredFeatures { .. }) {
                        return db.rs_type_kind_with_lifetime_elision(
                            alias.underlying_type.clone(),
                            elision_options,
                        );
                    }
                }
                // Comprehensive fallbacks: if we can delay reifying the error, delay it.
                if let Ok(symbol) = cpp_type_name::tagless_cpp_type_name_for_item(item, db.ir()) {
                    return Ok(RsTypeKind::Error {
                        symbol: symbol.to_string().into(),
                        error: error.into(),
                        visibility_override: None,
                    });
                }
                return Err(error.into());
            }

            // This is the implementation of `BindingsGenerator::rs_type_kind()`, so of
            // course we can't call `rs_type_kind` here, and instead reuse the raw construction
            // logic.
            RsTypeKind::from_item_raw(
                db,
                item.clone(),
                elision_options.have_reference_param,
                elision_options.is_return_type,
            )
        }
    }
}
