// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, ensure, Result};
use database::code_snippet::{NoBindingsReason, Visibility};
use database::rs_snippet::{Lifetime, Mutability, RsTypeKind};
use database::BindingsGenerator;
use ir::{CcCallingConv, CcType, CcTypeVariant, PointerTypeKind};
use std::rc::Rc;

/// Implementation of `BindingsGenerator::rs_type_kind`.
pub fn rs_type_kind_with_lifetime_elision(
    db: &dyn BindingsGenerator,
    ty: CcType,
    elide_missing_lifetimes: bool,
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
                None if elide_missing_lifetimes => Lifetime::elided(),
                None => return Ok(RsTypeKind::Pointer { pointee, mutability }),
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
                elide_missing_lifetimes,
            )?);
            let param_types = param_types
                .iter()
                .map(|param_type| {
                    db.rs_type_kind_with_lifetime_elision(
                        param_type.clone(),
                        elide_missing_lifetimes,
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
                abi: cc_calling_conv_to_rs_abi(*call_conv).into(),
                return_type,
                param_types,
            })
        }
        CcTypeVariant::Record(id) => {
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
                            elide_missing_lifetimes,
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
            RsTypeKind::from_item_raw(db, item.clone())
        }
    }
}

/// Converts clang::CallingConv enum [1] into an equivalent Rust Abi [2, 3, 4].
/// [1]
/// https://github.com/llvm/llvm-project/blob/c6a3225bb03b6afc2b63fbf13db3c100406b32ce/clang/include/clang/Basic/Specifiers.h#L262-L283
/// [2] https://doc.rust-lang.org/reference/types/function-pointer.html
/// [3]
/// https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier
/// [4]
/// https://github.com/rust-lang/rust/blob/b27ccbc7e1e6a04d749e244a3c13f72ca38e80e7/compiler/rustc_target/src/spec/abi.rs#L49
fn cc_calling_conv_to_rs_abi(call_conv: CcCallingConv) -> &'static str {
    match call_conv {
        CcCallingConv::C => {
            // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
            // that:
            // - `extern "C"` [...] whatever the default your C compiler supports.
            // - `extern "cdecl"` -- The default for x86_32 C code.
            //
            // We don't support C++ exceptions and therefore we use "C" (rather than
            // "C-unwind") - we have no need for unwinding across the FFI boundary -
            // e.g. from C++ into Rust frames (or vice versa).
            "C"
        }
        CcCallingConv::X86FastCall => {
            // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
            // that the fastcall ABI -- corresponds to MSVC's __fastcall and GCC and
            // clang's __attribute__((fastcall)).
            "fastcall"
        }
        CcCallingConv::X86VectorCall => {
            // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
            // that the vectorcall ABI -- corresponds to MSVC's __vectorcall and
            // clang's __attribute__((vectorcall)).
            "vectorcall"
        }
        CcCallingConv::X86ThisCall => {
            // We don't support C++ exceptions and therefore we use "thiscall" (rather
            // than "thiscall-unwind") - we have no need for unwinding across the FFI
            // boundary - e.g. from C++ into Rust frames (or vice versa).
            "thiscall"
        }
        CcCallingConv::X86StdCall => {
            // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
            // extern "stdcall" -- The default for the Win32 API on x86_32.
            //
            // We don't support C++ exceptions and therefore we use "stdcall" (rather
            // than "stdcall-unwind") - we have no need for unwinding across the FFI
            // boundary - e.g. from C++ into Rust frames (or vice versa).
            "stdcall"
        }
        CcCallingConv::Win64 => {
            // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
            // extern "win64" -- The default for C code on x86_64 Windows.
            "win64"
        }
    }
}
