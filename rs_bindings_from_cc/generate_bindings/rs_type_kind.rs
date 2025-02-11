// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, bail, ensure, Result};
use database::rs_snippet::{CratePath, Lifetime, Mutability, PrimitiveType, RsTypeKind};
use database::BindingsGenerator;
use has_bindings::{has_bindings, HasBindings, NoBindingsReason};
use ir::{rs_imported_crate_name, GenericItem, Item};
use std::rc::Rc;

pub fn rs_type_kind(db: &dyn BindingsGenerator, ty: ir::RsType) -> Result<RsTypeKind> {
    match &ty {
        ir::RsType::UnknownAttr { unknown_attr } => {
            // In most places, we only bail for unknown attributes in supported. However,
            // it's difficult and expensive to generate an RsTypeKind differently
            // depending on the translation unit for the item that contains it.
            // Rather than trying to keep going in experimental, we bail
            // unconditionally.
            //
            // The correct fix for this error is to add support for the attributes which are
            // not yet understood, but need to be used in practice.
            bail!("unknown attribute(s): {unknown_attr}")
        }
        ir::RsType::ItemIdType { decl_id } => {
            let ir = db.ir();
            let item = ir.find_untyped_decl(*decl_id);
            let fallback_type = match item {
                // Type aliases are unique among items, in that if the item defining the alias fails
                // to receive bindings, we can still use the aliased type.
                ir::Item::TypeAlias(alias) => Some(&alias.underlying_type.rs_type),
                _ => None,
            };
            match (has_bindings(db, item), fallback_type) {
                (HasBindings::Yes, _) => {}
                // Additionally, we should not "see through" type aliases that are specifically not
                // on targets that intend to support Rust users of those type aliases.
                // (If we did, then a C++ library owner could break Rust callers, which is a
                // maintenance responsibility that they did not sign up for!)
                (has_bindings, Some(fallback_type))
                    if !matches!(
                        has_bindings,
                        HasBindings::No(NoBindingsReason::MissingRequiredFeatures { .. })
                    ) =>
                {
                    return db.rs_type_kind(fallback_type.clone());
                }
                (HasBindings::Maybe, _) => {
                    bail!(
                        "Type {} may or may not exist, and cannot be used.",
                        item.debug_name(&ir)
                    );
                }
                (HasBindings::No(reason), _) => {
                    return Err(reason.into());
                }
            }
            match item {
                Item::IncompleteRecord(incomplete_record) => Ok(RsTypeKind::IncompleteRecord {
                    incomplete_record: incomplete_record.clone(),
                    crate_path: Rc::new(CratePath::new(
                        &ir,
                        ir.namespace_qualifier(incomplete_record),
                        rs_imported_crate_name(&incomplete_record.owning_target, &ir),
                    )),
                }),
                Item::Record(record) => RsTypeKind::new_record(db, record.clone(), ir),
                Item::Enum(enum_) => RsTypeKind::new_enum(enum_.clone(), &ir),
                Item::TypeAlias(type_alias) => RsTypeKind::new_type_alias(db, type_alias.clone()),
                Item::TypeMapOverride(type_map_override) => {
                    RsTypeKind::new_type_map_override(db, type_map_override)
                }
                other_item => bail!("Item does not define a type: {other_item:?}"),
            }
        }
        ir::RsType::NamedType { name, lifetime_args, type_args } => {
            let ir = db.ir();
            // The lambdas deduplicate code needed by multiple `match` branches.
            let get_type_args = || -> Result<Vec<RsTypeKind>> {
                type_args.iter().map(|type_arg| db.rs_type_kind(type_arg.clone())).collect()
            };
            let get_pointee = || -> Result<Rc<RsTypeKind>> {
                if type_args.len() != 1 {
                    bail!("Missing pointee/referent type (need exactly 1 type argument): {:?}", ty);
                }
                // TODO(b/351976044): Support bridge types by pointer/reference.
                let pointee = get_type_args()?.pop().unwrap();
                if pointee.is_bridge_type() {
                    bail!("Bridging types are not supported as pointee/referent types.");
                }
                Ok(Rc::new(pointee))
            };
            let get_lifetime = || -> Result<Lifetime> {
                if lifetime_args.len() != 1 {
                    bail!(
                        "Missing reference lifetime (need exactly 1 lifetime argument): {:?}",
                        ty
                    );
                }
                let lifetime_id = lifetime_args[0];
                ir.get_lifetime(lifetime_id)
                    .ok_or_else(|| anyhow!("no known lifetime with id {lifetime_id:?}"))
                    .map(Lifetime::from)
            };

            let result = match name.as_ref() {
                "*mut" => {
                    RsTypeKind::Pointer { pointee: get_pointee()?, mutability: Mutability::Mut }
                }
                "*const" => {
                    RsTypeKind::Pointer { pointee: get_pointee()?, mutability: Mutability::Const }
                }
                "&mut" => RsTypeKind::Reference {
                    referent: get_pointee()?,
                    mutability: Mutability::Mut,
                    lifetime: get_lifetime()?,
                },
                "&" => RsTypeKind::Reference {
                    referent: get_pointee()?,
                    mutability: Mutability::Const,
                    lifetime: get_lifetime()?,
                },
                "#RvalueReference mut" => RsTypeKind::RvalueReference {
                    referent: get_pointee()?,
                    mutability: Mutability::Mut,
                    lifetime: get_lifetime()?,
                },
                "#RvalueReference const" => RsTypeKind::RvalueReference {
                    referent: get_pointee()?,
                    mutability: Mutability::Const,
                    lifetime: get_lifetime()?,
                },
                "Option" => {
                    let mut type_args = get_type_args()?;
                    ensure!(
                        type_args.len() == 1,
                        "Option should have exactly 1 type argument (got {})",
                        type_args.len()
                    );
                    RsTypeKind::Option(Rc::new(type_args.remove(0)))
                }
                name => {
                    let mut type_args = get_type_args()?;

                    if let Some(primitive) = PrimitiveType::from_str(name) {
                        if !type_args.is_empty() {
                            bail!("{name} type must not have type arguments: {:?}", ty);
                        }
                        RsTypeKind::Primitive(primitive)
                    } else if let Some(abi) = name.strip_prefix("#funcPtr ") {
                        // Assert that function pointers in the IR either have static lifetime or
                        // no lifetime.
                        if let Ok(lifetime) = get_lifetime() {
                            assert_eq!(lifetime.0.as_ref(), "static");
                        }

                        assert!(
                            !type_args.is_empty(),
                            "In well-formed IR function pointers include at least the return type",
                        );
                        ensure!(
                            type_args.iter().all(|t| t.is_c_abi_compatible_by_value()),
                            "Either the return type or some of the parameter types require \
                            an FFI thunk (and function pointers don't have a thunk)",
                        );
                        RsTypeKind::FuncPtr {
                            abi: abi.into(),
                            return_type: Rc::new(type_args.remove(type_args.len() - 1)),
                            param_types: Rc::from(type_args),
                        }
                    } else {
                        bail!("Unknown type: {name}")
                    }
                }
            };
            Ok(result)
        }
    }
}
