// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::db::BindingsGenerator;

use crate::code_snippet::{CcPrerequisites, CcSnippet, ExternCDecl};
use crate::generate_struct_and_union::AdtCoreBindings;
use crate::{
    does_type_implement_trait, ensure_ty_is_pointer_like, format_cc_ident,
    format_param_types_for_cc, format_region_as_rs_lifetime, format_ret_ty_for_cc,
    format_ty_for_rs, is_bridged_type, is_c_abi_compatible_by_value,
    liberate_and_deanonymize_late_bound_regions, post_analysis_typing_env, AllowReferences,
    BridgedType, BridgedTypeConversionInfo, CcType, FullyQualifiedName, RsSnippet,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::escape_non_identifier_chars;
use code_gen_utils::make_rs_ident;
use code_gen_utils::CcConstQualifier;
use error_report::{anyhow, bail, ensure};
use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use rustc_hir::Safety;
use rustc_middle::ty::{self, Ty};
use rustc_span::def_id::{DefId, LOCAL_CRATE};
use rustc_span::symbol::{kw, sym, Symbol};
use rustc_type_ir::RegionKind;
use std::collections::{BTreeSet, HashMap};
use std::ops::AddAssign;

/// Formats a C++ function declaration of a thunk that wraps a Rust function.
pub fn generate_thunk_decl<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    sig_mid: &ty::FnSig<'tcx>,
    sig_hir: Option<&rustc_hir::FnDecl<'tcx>>,
    thunk_name: &TokenStream,
    allow_references: AllowReferences,
) -> Result<CcSnippet> {
    let mut prereqs = CcPrerequisites::default();
    let main_api_ret_type = format_ret_ty_for_cc(db, sig_mid, sig_hir)?.into_tokens(&mut prereqs);

    let mut thunk_params = {
        let cpp_types = format_param_types_for_cc(db, sig_mid, sig_hir, allow_references)?;
        sig_mid
            .inputs()
            .iter()
            .zip(cpp_types.into_iter())
            .map(|(&ty, cpp_type)| -> Result<TokenStream> {
                let cpp_type = cpp_type.into_tokens(&mut prereqs);
                if is_bridged_type(db, ty)?.is_some() {
                    match code_gen_utils::is_cpp_pointer_type(cpp_type.clone()) {
                        Some(CcConstQualifier::Mut) | Some(CcConstQualifier::Const) => {
                            Ok(quote! { #cpp_type })
                        }
                        None => Ok(quote! { #cpp_type* }),
                    }
                } else if is_c_abi_compatible_by_value(ty) {
                    Ok(quote! { #cpp_type })
                } else if let Some(adt_def) = ty.ty_adt_def() {
                    let core = db.generate_adt_core(adt_def.did())?;
                    db.generate_move_ctor_and_assignment_operator(core).map_err(|_| {
                        anyhow!("Can't pass a type by value without a move constructor")
                    })?;
                    Ok(quote! { #cpp_type* })
                } else {
                    bail!("Unknown type")
                }
            })
            .collect::<Result<Vec<_>>>()?
    };

    let thunk_ret_type: TokenStream;
    if is_c_abi_compatible_by_value(sig_mid.output()) {
        thunk_ret_type = main_api_ret_type;
    } else {
        thunk_ret_type = quote! { void };
        thunk_params.push(quote! { #main_api_ret_type* __ret_ptr });
    };
    Ok(CcSnippet {
        prereqs,
        tokens: quote! {
            namespace __crubit_internal {
                extern "C" #thunk_ret_type #thunk_name ( #( #thunk_params ),* );
            }
        },
    })
}

/// Formats a thunk implementation in Rust that provides an `extern "C"` ABI for
/// calling a Rust function identified by `fn_def_id`.  `generate_thunk_impl`
/// may panic if `fn_def_id` doesn't identify a function.
///
/// `fully_qualified_fn_name` specifies how the thunk can identify the function
/// to call. Examples of valid arguments:
/// - `::crate_name::some_module::free_function`
/// - `::crate_name::some_module::SomeStruct::method`
/// - `<::crate_name::some_module::SomeStruct as
///   ::core::default::Default>::default`
pub fn generate_thunk_impl<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    fn_def_id: DefId,
    sig: &ty::FnSig<'tcx>,
    thunk_name: &str,
    fully_qualified_fn_name: TokenStream,
) -> Result<RsSnippet> {
    let tcx = db.tcx();
    let param_names_and_types: Vec<(Ident, Ty)> = {
        let param_names = tcx.fn_arg_names(fn_def_id).iter().enumerate().map(|(i, ident)| {
            if ident.as_str().is_empty() {
                format_ident!("__param_{i}")
            } else if ident.name == kw::SelfLower {
                format_ident!("__self")
            } else {
                make_rs_ident(ident.as_str())
            }
        });
        let param_types = sig.inputs().iter().copied();
        param_names.zip(param_types).collect_vec()
    };

    let mut thunk_params = param_names_and_types
        .iter()
        .map(|(param_name, ty)| {
            let rs_type = format_ty_for_rs(db, *ty)
                .with_context(|| format!("Error handling parameter `{param_name}`"))?;

            if let Some(BridgedType { cpp_type: CcType::Other(_), .. }) = is_bridged_type(db, *ty)?
            {
                Ok(quote! { #param_name: *const core::ffi::c_void })
            } else if let Some(BridgedType { cpp_type: CcType::Pointer { cv, .. }, .. }) =
                is_bridged_type(db, *ty)?
            {
                match cv {
                    CcConstQualifier::Mut => Ok(quote! { #param_name: *mut core::ffi::c_void }),
                    CcConstQualifier::Const => Ok(quote! { #param_name: *const core::ffi::c_void }),
                }
            } else if is_c_abi_compatible_by_value(*ty) {
                Ok(quote! { #param_name: #rs_type })
            } else {
                Ok(quote! { #param_name: &mut ::core::mem::MaybeUninit<#rs_type> })
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let mut extern_c_decls = BTreeSet::new();

    let fn_args_conversions = param_names_and_types
        .iter()
        .map(|(param_name, ty)| match is_bridged_type(db, *ty)? {
            None => Ok(quote! {}),
            Some(BridgedType { cpp_type, conversion_info, .. }) => {
                let rs_type = format_ty_for_rs(db, *ty)
                    .with_context(|| format!("Error handling parameter `{param_name}`"))?;

                let rs_out_varname = format_ident!("__crubit_{}_uninit", param_name);
                let rs_out_decl = quote! {
                    let mut #rs_out_varname =
                        ::core::mem::MaybeUninit::<#rs_type>::uninit();
                };

                match conversion_info {
                    BridgedTypeConversionInfo::PointerLikeTransmute => Ok(quote! {
                        #rs_out_decl

                        unsafe { #rs_out_varname.write(::core::mem::transmute(#param_name)); }
                    }),
                    BridgedTypeConversionInfo::ExternCFuncConverters {
                        cpp_to_rust_converter,
                        ..
                    } => {
                        let cpp_to_rust_converter_ident =
                            make_rs_ident(cpp_to_rust_converter.as_str());
                        let cpp_in_ty =
                            if let CcType::Pointer { cv: CcConstQualifier::Mut, .. } = cpp_type {
                                quote! { *mut core::ffi::c_void }
                            } else {
                                quote! { *const core::ffi::c_void }
                            };

                        extern_c_decls.insert(ExternCDecl {
                            symbol: cpp_to_rust_converter,
                            decl: quote! {
                                fn #cpp_to_rust_converter_ident(cpp_in: #cpp_in_ty,
                                    rs_out: *mut core::ffi::c_void);
                            },
                        });

                        Ok(quote! {
                            #rs_out_decl

                            unsafe { #cpp_to_rust_converter_ident(#param_name,
                                #rs_out_varname.as_mut_ptr() as *mut core::ffi::c_void); }
                        })
                    }
                }
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let mut thunk_ret_type = format_ty_for_rs(db, sig.output())?;

    let fn_args = param_names_and_types
        .iter()
        .map(|(rs_name, ty)| {
            if is_bridged_type(db, *ty)?.is_some() {
                let rs_out_varname = format_ident!("__crubit_{}_uninit", rs_name);
                Ok(quote! { unsafe { #rs_out_varname.assume_init() } })
            } else if is_c_abi_compatible_by_value(*ty) {
                Ok(quote! { #rs_name })
            } else if let Safety::Unsafe = sig.safety {
                // The whole call will be wrapped in `unsafe` below.
                Ok(quote! { #rs_name.assume_init_read() })
            } else {
                Ok(quote! { unsafe { #rs_name.assume_init_read() } })
            }
        })
        .collect::<Result<Vec<_>>>()?;
    let mut thunk_body;

    match is_bridged_type(db, sig.output())? {
        None => {
            thunk_body = quote! {
                #( #fn_args_conversions )*

                #fully_qualified_fn_name( #( #fn_args ),* )
            };

            if !is_c_abi_compatible_by_value(sig.output()) {
                thunk_params.push(quote! {
                    __ret_slot: &mut ::core::mem::MaybeUninit<#thunk_ret_type>
                });
                thunk_ret_type = quote! { () };
                thunk_body = quote! { __ret_slot.write({ #thunk_body }); };
            };
        }
        Some(BridgedType { cpp_type, conversion_info, .. }) => {
            let cpp_out_ty = match cpp_type {
                CcType::Pointer { cv: CcConstQualifier::Mut, .. } => {
                    quote! { *mut *mut core::ffi::c_void }
                }
                CcType::Pointer { cv: CcConstQualifier::Const, .. } => {
                    quote! { *mut *const core::ffi::c_void }
                }
                CcType::Other(_) => quote! { *mut core::ffi::c_void },
            };

            thunk_params.push(quote! {
                __ret_ptr: #cpp_out_ty
            });
            thunk_ret_type = quote! { () };

            let thunk_body_common = quote! {
                #( #fn_args_conversions )*

                let rs_val = #fully_qualified_fn_name( #( #fn_args ),* );
            };

            thunk_body = match conversion_info {
                BridgedTypeConversionInfo::PointerLikeTransmute => {
                    ensure_ty_is_pointer_like(db, sig.output())?;

                    quote! {
                        #thunk_body_common

                        unsafe { __ret_ptr.write(::core::mem::transmute(rs_val)); }
                    }
                }
                BridgedTypeConversionInfo::ExternCFuncConverters {
                    rust_to_cpp_converter, ..
                } => {
                    let rust_to_cpp_converter_ident = make_rs_ident(rust_to_cpp_converter.as_str());

                    extern_c_decls.insert(ExternCDecl {
                        symbol: rust_to_cpp_converter,
                        decl: quote! {
                        fn #rust_to_cpp_converter_ident(rs_in: *const core::ffi::c_void,
                            cpp_out: #cpp_out_ty);
                        },
                    });

                    quote! {
                        #thunk_body_common

                        unsafe {
                            #rust_to_cpp_converter_ident(
                                std::ptr::from_ref(&rs_val) as *const core::ffi::c_void,
                                __ret_ptr);
                        }
                    }
                }
            };
        }
    };

    // Wrap the call in an unsafe block, for the sake of RFC #2585
    // `unsafe_block_in_unsafe_fn`.
    if let Safety::Unsafe = sig.safety {
        thunk_body = quote! {unsafe {#thunk_body}};
    }

    let generic_params = {
        let regions = sig
            .inputs()
            .iter()
            .copied()
            .chain(std::iter::once(sig.output()))
            .flat_map(|ty| {
                ty.walk().filter_map(|generic_arg| match generic_arg.unpack() {
                    ty::GenericArgKind::Const(_) | ty::GenericArgKind::Type(_) => None,
                    ty::GenericArgKind::Lifetime(region) => Some(region),
                })
            })
            .filter(|region| match region.kind() {
                RegionKind::ReStatic => false,
                RegionKind::ReLateParam(_) => true,
                _ => panic!("Unexpected region kind: {region}"),
            })
            .sorted_by_key(|region| {
                region
                    .get_name()
                    .expect("Caller should use `liberate_and_deanonymize_late_bound_regions`")
            })
            .dedup()
            .collect_vec();
        if regions.is_empty() {
            quote! {}
        } else {
            let lifetimes = regions.into_iter().map(|region| format_region_as_rs_lifetime(&region));
            quote! { < #( #lifetimes ),* > }
        }
    };

    let thunk_name = make_rs_ident(thunk_name);
    let unsafe_qualifier = if let Safety::Unsafe = sig.safety {
        quote! {unsafe}
    } else {
        quote! {}
    };

    Ok(RsSnippet {
        tokens: quote! {
            #[unsafe(no_mangle)]
            #unsafe_qualifier extern "C" fn #thunk_name #generic_params (
                #( #thunk_params ),*
            ) -> #thunk_ret_type {
                #thunk_body
            }
        },
        extern_c_decls,
    })
}

/// Returns `Ok(())` if no thunk is required.
/// Otherwise returns an error the describes why the thunk is needed.
pub fn is_thunk_required(sig: &ty::FnSig) -> Result<()> {
    match sig.abi {
        // "C" ABI is okay: since https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html has been
        // accepted, a Rust panic that "escapes" a "C" ABI function is a defined crash. See
        // https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-unwinding.
        rustc_target::spec::abi::Abi::C { unwind: false } => (),

        // This requires a thunk if the calling C++ frames use `-fno-exceptions`, as it is
        // UB. However, we leave this to the caller: if you use `extern "C-unwind"`, we assume you
        // know what you are doing and do not block you from integrating with exception-enabled C++.
        rustc_target::spec::abi::Abi::C { unwind: true } => (),

        // All other ABIs trigger thunk generation.  This covers Rust ABI functions, but also
        // ABIs that theoretically are understood both by C++ and Rust (e.g. see
        // `format_cc_call_conv_as_clang_attribute` in `rs_bindings_from_cc/src_code_gen.rs`).
        _ => bail!("Any calling convention other than `extern \"C\"` requires a thunk"),
    };

    ensure!(is_c_abi_compatible_by_value(sig.output()), "Return type requires a thunk");
    for (i, param_ty) in sig.inputs().iter().enumerate() {
        ensure!(is_c_abi_compatible_by_value(*param_ty), "Type of parameter #{i} requires a thunk");
    }

    Ok(())
}

pub struct TraitThunks {
    pub method_name_to_cc_thunk_name: HashMap<Symbol, TokenStream>,
    pub cc_thunk_decls: CcSnippet,
    pub rs_thunk_impls: RsSnippet,
}

pub fn generate_trait_thunks<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    trait_id: DefId,
    adt: &AdtCoreBindings<'tcx>,
) -> Result<TraitThunks> {
    let tcx = db.tcx();
    assert!(tcx.is_trait(trait_id));

    let self_ty = adt.self_ty;
    let is_drop_trait = Some(trait_id) == tcx.lang_items().drop_trait();
    if is_drop_trait {
        // To support "drop glue" we don't require that `self_ty` directly implements
        // the `Drop` trait.  Instead we require the caller to check
        // `needs_drop`.
        assert!(self_ty.needs_drop(tcx, post_analysis_typing_env(tcx, adt.def_id)));
    } else if !does_type_implement_trait(tcx, self_ty, trait_id) {
        let trait_name = tcx.item_name(trait_id);
        bail!("`{self_ty}` doesn't implement the `{trait_name}` trait");
    }

    let mut method_name_to_cc_thunk_name = HashMap::new();
    let mut cc_thunk_decls = CcSnippet::default();
    let mut rs_thunk_impls = RsSnippet::default();
    let methods = tcx
        .associated_items(trait_id)
        .in_definition_order()
        .filter(|item| item.kind == ty::AssocKind::Fn);
    for method in methods {
        let substs = {
            let generics = tcx.generics_of(method.def_id);
            if generics.own_params.iter().any(|p| p.kind.is_ty_or_const()) {
                // Note that lifetime-generic methods are ok:
                // * they are handled by `generate_thunk_decl` and `generate_thunk_impl`
                // * the lifetimes are erased by `ty::Instance::mono` and *seem* to be erased by
                //   `ty::Instance::new`
                panic!(
                    "So far callers of `generate_trait_thunks` didn't need traits with \
                      methods that are type-generic or const-generic"
                );
            }
            assert!(generics.has_self);
            tcx.mk_args_trait(self_ty, std::iter::empty())
        };

        let thunk_name = {
            if db.no_thunk_name_mangling() {
                format!("__crubit_thunk_{}", &escape_non_identifier_chars(method.name.as_str()))
            } else {
                let instance = ty::Instance::new(method.def_id, substs);
                let symbol = tcx.symbol_name(instance);
                format!(
                    "__crubit_thunk_{}_{}",
                    tcx.crate_hash(LOCAL_CRATE).to_hex(),
                    &escape_non_identifier_chars(symbol.name)
                )
            }
        };
        method_name_to_cc_thunk_name.insert(method.name, format_cc_ident(db, &thunk_name)?);

        let sig_mid = liberate_and_deanonymize_late_bound_regions(
            tcx,
            tcx.fn_sig(method.def_id).instantiate(tcx, substs),
            method.def_id,
        );
        // TODO(b/254096006): Preserve the HIR here, if possible?
        // Cannot in general (e.g. blanket impl from another crate), but should be able
        // to for traits defined or implemented in the current crate.
        let sig_hir = None;

        let allow_references =
            if method.name == sym::clone_from && Some(trait_id) == tcx.lang_items().clone_trait() {
                // We specially handle aliases in `operator=` so that clone_from cannot be
                // called with an alias. (`if (this != &other) {...}`)
                AllowReferences::UnsafeAll
            } else {
                AllowReferences::Safe
            };

        cc_thunk_decls.add_assign({
            let thunk_name = format_cc_ident(db, &thunk_name)?;
            generate_thunk_decl(db, &sig_mid, sig_hir, &thunk_name, allow_references)?
        });

        rs_thunk_impls += {
            let struct_name = &adt.rs_fully_qualified_name;
            if is_drop_trait {
                // Manually formatting (instead of depending on `generate_thunk_impl`)
                // to avoid https://doc.rust-lang.org/error_codes/E0040.html
                let thunk_name = make_rs_ident(&thunk_name);
                RsSnippet::new(quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn #thunk_name(
                        __self: &mut ::core::mem::MaybeUninit<#struct_name>
                    ) {
                        unsafe { __self.assume_init_drop() };
                    }
                })
            } else {
                let fully_qualified_fn_name = {
                    let fully_qualified_trait_name =
                        FullyQualifiedName::new(db, trait_id).format_for_rs();
                    let method_name = make_rs_ident(method.name.as_str());
                    quote! { <#struct_name as #fully_qualified_trait_name>::#method_name }
                };
                generate_thunk_impl(
                    db,
                    method.def_id,
                    &sig_mid,
                    &thunk_name,
                    fully_qualified_fn_name,
                )?
            }
        };
    }

    Ok(TraitThunks { method_name_to_cc_thunk_name, cc_thunk_decls, rs_thunk_impls })
}
