// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::{
    does_type_implement_trait, ensure_ty_is_pointer_like, format_cc_ident,
    format_param_types_for_cc, format_ret_ty_for_cc, is_bridged_type, is_c_abi_compatible_by_value,
    liberate_and_deanonymize_late_bound_regions, BridgedType, BridgedTypeConversionInfo, RsSnippet,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::escape_non_identifier_chars;
use code_gen_utils::make_rs_ident;
use code_gen_utils::CcConstQualifier;
use crubit_abi_type::CrubitAbiTypeToRustExprTokens;
use database::code_snippet::{CcPrerequisites, CcSnippet, ExternCDecl};
use database::{AdtCoreBindings, BindingsGenerator, SugaredTy};
use error_report::{anyhow, bail, ensure};
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use query_compiler::post_analysis_typing_env;
use quote::format_ident;
use quote::quote;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::DefId;
use rustc_span::symbol::{kw, sym, Symbol};
use rustc_type_ir::inherent::Region;
use std::collections::{BTreeSet, HashMap};
use std::ops::AddAssign;

/// Returns a C ABI-compatible C type to pass a tuple, or `None` if `possibly_tuple_ty` is not a
/// tuple.
///
/// Tuples are passed via a pointer to an array of `void*` where
/// each pointer points to the corresponding element of the tuple.
fn tuple_c_abi_c_type(possibly_tuple_ty: ty::Ty) -> Option<TokenStream> {
    let ty::TyKind::Tuple(_) = possibly_tuple_ty.kind() else { return None };
    // Sized array types are sadly not usable by-pointer in C++.
    Some(quote! { void** })
}

/// Returns a C ABI-compatible Rust type to pass a tuple, or `None` if `possibly_tuple_ty` is not a
/// tuple.
///
/// Tuples are passed via a pointer to an array of `*const c_void` where
/// each pointer points to the corresponding element of the tuple.
fn tuple_c_abi_rs_type(possibly_tuple_ty: ty::Ty) -> Option<TokenStream> {
    let ty::TyKind::Tuple(tuple_tys) = possibly_tuple_ty.kind() else { return None };
    let num_elements = tuple_tys.len();
    Some(quote! { *const [*const core::ffi::c_void; #num_elements] })
}

fn is_drop_not_default<'tcx>(tcx: ty::TyCtxt<'tcx>, ty: ty::Ty<'tcx>) -> bool {
    if !ty.needs_drop(
        tcx,
        ty::TypingEnv {
            typing_mode: ty::TypingMode::PostAnalysis,
            param_env: ty::ParamEnv::empty(),
        },
    ) {
        return false;
    }
    let trait_id =
        tcx.get_diagnostic_item(sym::Default).expect("Couldn't find `core::default::Default`");
    !does_type_implement_trait(tcx, ty, trait_id, [])
}

/// Returns a C ABI-compatible C type to pass a [inner_ty; _].
///
/// Layout-compatible arrays are passed through memory.
fn array_c_abi_c_type<'tcx>(tcx: ty::TyCtxt<'tcx>, inner_ty: ty::Ty<'tcx>) -> Result<TokenStream> {
    // TODO: b/451981992 - Nested arrays containing types that are Drop but not Default do not
    // behave well in std::arrays.
    match inner_ty.kind() {
        ty::TyKind::Array(ty, _) if is_drop_not_default(tcx, *ty) => {
            bail!("b/260128806 - nested array {inner_ty} is not supported because it contains a type that implements Drop but not Default")
        }
        _ => Ok(quote! { void* }),
    }
}

/// Formats a C++ declaration of a C-ABI-compatible-function wrapper around a Rust function.
pub fn generate_thunk_decl<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    sig_mid: &ty::FnSig<'tcx>,
    sig_hir: Option<&rustc_hir::FnDecl<'tcx>>,
    thunk_name: &Ident,
    has_self_param: bool,
) -> Result<CcSnippet> {
    let tcx = db.tcx();
    let mut prereqs = CcPrerequisites::default();
    let main_api_ret_type = format_ret_ty_for_cc(db, sig_mid)?.into_tokens(&mut prereqs);

    let mut thunk_params = {
        let cpp_types = format_param_types_for_cc(db, sig_mid, has_self_param)?;
        sig_mid
            .inputs()
            .iter()
            .zip(cpp_types.into_iter())
            .map(|(&ty, cpp_type)| -> Result<TokenStream> {
                let cpp_type = cpp_type.snippet.into_tokens(&mut prereqs);
                let bridged_type_opt = is_bridged_type(db, ty)?;
                if let Some(bridged_type) = bridged_type_opt {
                    match bridged_type {
                        BridgedType::Legacy { .. } => {
                            match code_gen_utils::is_cpp_pointer_type(cpp_type.clone()) {
                                Some(CcConstQualifier::Mut) | Some(CcConstQualifier::Const) => {
                                    Ok(quote! { #cpp_type })
                                }
                                None => Ok(quote! { #cpp_type* }),
                            }
                        }
                        BridgedType::Composable(_) => Ok(quote! { unsigned char* }),
                    }
                } else if is_c_abi_compatible_by_value(tcx, ty) {
                    Ok(quote! { #cpp_type })
                } else if let Some(tuple_abi) = tuple_c_abi_c_type(ty) {
                    Ok(tuple_abi)
                } else if let ty::TyKind::Array(inner_ty, _) = ty.kind() {
                    array_c_abi_c_type(db.tcx(), *inner_ty)
                } else if let Some(adt_def) = ty.ty_adt_def() {
                    let core = db.generate_adt_core(adt_def.did())?;
                    db.generate_move_ctor_and_assignment_operator(core).map_err(|e| {
                        anyhow!("Can't pass a type by value without a move constructor: {}", e.err)
                    })?;
                    Ok(quote! { #cpp_type* })
                } else {
                    bail!("Unknown type")
                }
            })
            .collect::<Result<Vec<_>>>()?
    };

    // Types which are not C-ABI compatible by-value are returned via out-pointer parameters.
    // TODO: b/ 459482188 - The order of this check must align with the order in `cc_return_value_from_c_abi`.
    // We should centralize this logic so that the order exists in a singular location used by both
    // places.
    let thunk_ret_type = if let Some(briging) = is_bridged_type(db, sig_mid.output())? {
        match briging {
            BridgedType::Legacy { .. } => {
                thunk_params.push(quote! { #main_api_ret_type* __ret_ptr });
                quote! { void }
            }
            BridgedType::Composable(_) => {
                thunk_params.push(quote! { unsigned char * __ret_ptr });
                quote! { void }
            }
        }
    } else if is_c_abi_compatible_by_value(tcx, sig_mid.output()) {
        main_api_ret_type
    } else if let Some(tuple_abi) = tuple_c_abi_c_type(sig_mid.output()) {
        thunk_params.push(quote! { #tuple_abi __ret_ptr });
        quote! { void }
    } else if let ty::TyKind::Array(inner_ty, _) = sig_mid.output().kind() {
        let c_type = array_c_abi_c_type(db.tcx(), *inner_ty)?;
        thunk_params.push(quote! { #c_type __ret_ptr });
        quote! { void }
    } else {
        thunk_params.push(quote! { #main_api_ret_type* __ret_ptr });
        quote! { void }
    };

    let mut attributes = vec![];
    // Attribute: noreturn
    let rs_return_type = SugaredTy::fn_output(sig_mid, None);
    if *rs_return_type.mid().kind() == ty::TyKind::Never {
        attributes.push(quote! {[[noreturn]]});
    }

    Ok(CcSnippet {
        prereqs,
        tokens: quote! {
            namespace __crubit_internal {
                extern "C" #(#attributes)* #thunk_ret_type #thunk_name ( #( #thunk_params ),* );
            }
        },
    })
}

/// Creates Rust code to convert a bridged type from a C ABI type to a Rust type.
///
/// Expects an exising local of type `cpp_type` named `local_name` and shadows it
/// with a local of type `ty` named `local_name`.
fn convert_bridged_type_from_c_abi_to_rust<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: ty::Ty<'tcx>,
    bridged_type: &BridgedType,
    local_name: &Ident,
    extern_c_decls: &mut BTreeSet<ExternCDecl>,
) -> Result<TokenStream> {
    let rs_type = db
        .format_ty_for_rs(ty)
        .with_context(|| format!("Error handling parameter `{local_name}`"))?;

    let temp_name = format_ident!("__crubit_temp");

    match bridged_type {
        BridgedType::Legacy { conversion_info, .. } => {
            let convert = match conversion_info {
                BridgedTypeConversionInfo::PointerLikeTransmute => quote! {
                    #temp_name.write(::core::mem::transmute(#local_name));
                },
                BridgedTypeConversionInfo::ExternCFuncConverters {
                    cpp_to_rust_converter, ..
                } => {
                    let cpp_to_rust_converter_ident = add_extern_c_decl(
                        extern_c_decls,
                        ExternCDeclKind::CppToRustConverter,
                        *cpp_to_rust_converter,
                    );
                    quote! {
                        #cpp_to_rust_converter_ident(#local_name,#temp_name.as_mut_ptr() as *mut core::ffi::c_void);
                    }
                }
            };
            Ok(quote! {
                let #local_name = {
                    let mut #temp_name = ::core::mem::MaybeUninit::<#rs_type>::uninit();
                    #convert
                    #temp_name.assume_init()
                };
            })
        }
        BridgedType::Composable(composable) => {
            let crubit_abi_type_expr = CrubitAbiTypeToRustExprTokens(&composable.crubit_abi_type);
            // SAFETY: The buffer is the correct size, as determined by Crubit.
            Ok(quote! {
                let #local_name = unsafe {
                    ::bridge_rust::internal::decode(#crubit_abi_type_expr, #local_name)
                };
            })
        }
    }
}

/// Converts a local named `local_name` from its C ABI-compatible type
/// `*const [*const core::ffi::c_void; <tuple_tys.len()>]` to a tuple of Rust types.
fn convert_tuple_from_c_abi_to_rust<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    tuple_tys: &[ty::Ty<'tcx>],
    local_name: &Ident,
    extern_c_decls: &mut BTreeSet<ExternCDecl>,
) -> Result<TokenStream> {
    let mut read_elements = Vec::with_capacity(tuple_tys.len());
    for (i, element_type) in tuple_tys.iter().copied().enumerate() {
        let element_c_abi_type = c_abi_for_param_type(db, element_type)?;
        let element_local_name = format_ident!("{local_name}_{i}");
        let from_c_abi_to_rust = convert_value_from_c_abi_to_rust(
            db,
            element_type,
            &element_local_name,
            extern_c_decls,
        )?;
        read_elements.push(quote! { {
            let #element_local_name: #element_c_abi_type = ((*#local_name)[#i] as *const #element_c_abi_type).read();
            #from_c_abi_to_rust
            #element_local_name
        } });
    }
    Ok(quote! {
        let #local_name = (#(#read_elements,)*);
    })
}

/// Returns code to convert a local named `local_name` from its C ABI-compatible type to its Rust
/// type.
fn convert_value_from_c_abi_to_rust<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: ty::Ty<'tcx>,
    local_name: &Ident,
    extern_c_decls: &mut BTreeSet<ExternCDecl>,
) -> Result<TokenStream> {
    if let Some(bridged) = is_bridged_type(db, ty)? {
        return convert_bridged_type_from_c_abi_to_rust(
            db,
            ty,
            &bridged,
            local_name,
            extern_c_decls,
        );
    }
    let tcx = db.tcx();
    if is_c_abi_compatible_by_value(tcx, ty) {
        return Ok(quote! {});
    }
    if let ty::TyKind::Tuple(tuple_tys) = ty.kind() {
        return convert_tuple_from_c_abi_to_rust(db, tuple_tys, local_name, extern_c_decls);
    }
    // Non-C-ABI-compatible-by-value types are passed by
    // `&mut MaybeUninit<T>` reference, so we need to read out the value.
    Ok(quote! { let #local_name = #local_name.assume_init_read(); })
}

fn c_abi_for_param_type<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: ty::Ty<'tcx>,
) -> Result<TokenStream> {
    let tcx = db.tcx();
    if let Some(bridged) = is_bridged_type(db, ty)? {
        match bridged {
            BridgedType::Legacy { .. } => Ok(quote! { *const core::ffi::c_void }),
            BridgedType::Composable(_) => Ok(quote! { *const core::ffi::c_uchar }),
        }
    } else if is_c_abi_compatible_by_value(tcx, ty) {
        let rs_type = db.format_ty_for_rs(ty)?;
        Ok(quote! { #rs_type })
    } else if let Some(tuple_abi) = tuple_c_abi_rs_type(ty) {
        Ok(quote! { #tuple_abi })
    } else {
        let rs_type = db.format_ty_for_rs(ty)?;
        // `'static` is used to erase all lifetime parameters since C++ doesn't understand
        // lifetime constraints.
        Ok(quote! { &'static mut ::core::mem::MaybeUninit<#rs_type> })
    }
}

#[rustversion::before(2025-03-19)]
pub(crate) fn ident_or_opt_ident(i: &rustc_span::Ident) -> Option<&rustc_span::Ident> {
    Some(i)
}

#[rustversion::since(2025-03-19)]
pub(crate) fn ident_or_opt_ident(i: &Option<rustc_span::Ident>) -> Option<&rustc_span::Ident> {
    i.as_ref()
}

/// Returns an iterator which yields arbitrary unique names for the parameters
/// of the function identified by `fn_def_id`.
pub fn thunk_param_names(
    tcx: ty::TyCtxt<'_>,
    fn_def_id: DefId,
) -> impl Iterator<Item = Ident> + '_ {
    tcx.fn_arg_idents(fn_def_id).iter().enumerate().map(|(i, ident)| {
        let Some(ident) = ident_or_opt_ident(ident) else {
            return format_ident!("__param_{i}");
        };
        // TODO(jeanpierreda): Deduplicate the logic after the next rustc rollout.
        if ident.name == kw::Underscore || ident.name.is_empty() {
            format_ident!("__param_{i}")
        } else if ident.name == kw::SelfLower {
            format_ident!("__self")
        } else {
            make_rs_ident(ident.as_str())
        }
    })
}

enum ExternCDeclKind {
    /// The function is a Rust to C++ converter.
    RustToCppConverter,
    /// The function is a C++ to Rust converter.
    CppToRustConverter,
}

fn add_extern_c_decl(
    extern_c_decls: &mut BTreeSet<ExternCDecl>,
    kind: ExternCDeclKind,
    symbol: Symbol,
) -> Ident {
    let converter_ident = make_rs_ident(symbol.as_str());
    let decl = match kind {
        ExternCDeclKind::RustToCppConverter => {
            quote! {
                fn #converter_ident(
                    rs_in: *const core::ffi::c_void,
                    cpp_out: *mut core::ffi::c_void);
            }
        }
        ExternCDeclKind::CppToRustConverter => {
            quote! {
                fn #converter_ident(
                    cpp_in: *const core::ffi::c_void,
                    rs_out: *mut core::ffi::c_void);
            }
        }
    };
    extern_c_decls.insert(ExternCDecl { symbol, decl });
    converter_ident
}

/// Writes a Rust value out into the memory pointed to a `*mut c_void` pointed to by `c_ptr`.
fn write_rs_value_to_c_abi_ptr<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    rs_value: &Ident,
    c_ptr: &Ident,
    rs_type: ty::Ty<'tcx>,
    extern_c_decls: &mut BTreeSet<ExternCDecl>,
) -> Result<TokenStream> {
    let write_directly = || -> Result<TokenStream> {
        let rs_type_tokens = db.format_ty_for_rs(rs_type)?;
        Ok(quote! { (#c_ptr as *mut #rs_type_tokens).write(#rs_value); })
    };
    let tcx = db.tcx();
    Ok(if let Some(bridged_type) = is_bridged_type(db, rs_type)? {
        match bridged_type {
            BridgedType::Legacy { conversion_info, .. } => match conversion_info {
                BridgedTypeConversionInfo::PointerLikeTransmute => {
                    ensure_ty_is_pointer_like(db, rs_type)?;
                    write_directly()?
                }
                BridgedTypeConversionInfo::ExternCFuncConverters {
                    rust_to_cpp_converter, ..
                } => {
                    let rust_to_cpp_converter_ident = add_extern_c_decl(
                        extern_c_decls,
                        ExternCDeclKind::RustToCppConverter,
                        rust_to_cpp_converter,
                    );
                    quote! {
                        #rust_to_cpp_converter_ident(
                            std::ptr::from_ref(&#rs_value) as *const core::ffi::c_void,
                            #c_ptr);
                    }
                }
            },
            BridgedType::Composable(composable) => {
                let crubit_abi_type_expr =
                    CrubitAbiTypeToRustExprTokens(&composable.crubit_abi_type);
                quote! {
                    // SAFETY: TODO(okabayashi)
                    unsafe {
                        ::bridge_rust::internal::encode(
                            #crubit_abi_type_expr,
                            // TODO(okabayashi): This ptr case can be removed once tuple bridging is supported,
                            // as it only is required in the tuple recursive case.
                            #c_ptr as *mut core::ffi::c_uchar,
                            #rs_value
                        );
                    }
                }
            }
        }
    } else if is_c_abi_compatible_by_value(tcx, rs_type) {
        write_directly()?
    } else if let ty::TyKind::Tuple(tuple_tys) = rs_type.kind() {
        let num_elements = tuple_tys.len();
        let rs_element_names =
            (0..num_elements).map(|i| format_ident!("{rs_value}_{i}")).collect_vec();
        let ptr_member_names =
            (0..num_elements).map(|i| format_ident!("{c_ptr}_{i}")).collect_vec();
        let unpack = quote! {
            let (#(#rs_element_names,)*) = #rs_value;
            let [#(#ptr_member_names),*] = *(#c_ptr as *mut [*mut core::ffi::c_void; #num_elements]);
        };
        let write_elements = (0..num_elements)
            .map(|i| {
                write_rs_value_to_c_abi_ptr(
                    db,
                    &rs_element_names[i],
                    &ptr_member_names[i],
                    tuple_tys[i],
                    extern_c_decls,
                )
            })
            .collect::<Result<TokenStream>>()?;
        quote! {
            #unpack
            #write_elements
        }
    } else if let ty::TyKind::Array { .. } = rs_type.kind() {
        write_directly()?
    } else if rs_type.ty_adt_def().is_some() {
        write_directly()?
    } else {
        bail!("Attempted to write out unknown type from Rust to C")
    })
}

fn replace_all_regions_with_static<'tcx, T>(tcx: TyCtxt<'tcx>, value: T) -> T
where
    T: ty::TypeFoldable<TyCtxt<'tcx>>,
{
    struct Staticifier<'tcx> {
        tcx: TyCtxt<'tcx>,
        static_region: ty::Region<'tcx>,
    }

    impl<'tcx> ty::TypeFolder<TyCtxt<'tcx>> for Staticifier<'tcx> {
        fn cx(&self) -> TyCtxt<'tcx> {
            self.tcx
        }
        fn fold_region(&mut self, _: ty::Region<'tcx>) -> ty::Region<'tcx> {
            self.static_region
        }
    }

    value.fold_with(&mut Staticifier { tcx, static_region: ty::Region::new_static(tcx) })
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

    // We replace all regions with `'static`. C++ doesn't understand region constraints, so our FFI
    // thunk cannot be dependent upon a particular choice of lifetime parameters. Using `'static`
    // everywhere is the easiest way to allow the thunk to compile regardless of the specific
    // relationship between the lifetime parameters.
    let sig = replace_all_regions_with_static(tcx, *sig);

    let param_names_and_types: Vec<(Ident, Ty)> = {
        let param_names = thunk_param_names(tcx, fn_def_id);
        let param_types = sig.inputs().iter().copied();
        param_names.zip(param_types).collect_vec()
    };

    let mut thunk_params = param_names_and_types
        .iter()
        .map(|(param_name, ty)| {
            let c_abi_type = c_abi_for_param_type(db, *ty)
                .with_context(|| format!("Error handling parameter `{param_name}`"))?;
            Ok(quote! { #param_name: #c_abi_type })
        })
        .collect::<Result<Vec<TokenStream>>>()?;

    let mut extern_c_decls = BTreeSet::new();

    // Convert all parameters from their C ABI types to their Rust types.
    let fn_args_conversions = param_names_and_types
        .iter()
        .map(|(param_name, ty)| {
            convert_value_from_c_abi_to_rust(db, *ty, param_name, &mut extern_c_decls)
        })
        .collect::<Result<Vec<TokenStream>>>()?;

    let fn_args: Vec<Ident> =
        param_names_and_types.into_iter().map(|(rs_name, _ty)| rs_name).collect();
    let output_is_bridged = is_bridged_type(db, sig.output())?;

    let thunk_return_type;
    let thunk_return_expression;
    if output_is_bridged.is_none() && is_c_abi_compatible_by_value(tcx, sig.output()) {
        // The output is not bridged and is C ABI compatible by-value, so we can just return
        // the result directly, and no out-param is needed.
        thunk_return_type = db.format_ty_for_rs(sig.output())?;
        thunk_return_expression = quote! {
            #fully_qualified_fn_name( #( #fn_args ),* )
        };
    } else {
        let return_ptr_ident = format_ident!("__ret_ptr");
        let rs_return_value_ident = format_ident!("__rs_return_value");
        thunk_return_type = quote! { () };

        let return_ptr_type = if let Some(BridgedType::Composable(_)) = output_is_bridged {
            // Composable bridging writes its Crubit ABI form in an unsigned char array.
            quote! { *mut core::ffi::c_uchar }
        } else {
            quote! { *mut core::ffi::c_void }
        };
        thunk_params.push(quote! {
            #return_ptr_ident: #return_ptr_type
        });
        let write_return_value = write_rs_value_to_c_abi_ptr(
            db,
            &rs_return_value_ident,
            &return_ptr_ident,
            sig.output(),
            &mut extern_c_decls,
        )?;
        thunk_return_expression = quote! {
            let #rs_return_value_ident = #fully_qualified_fn_name( #( #fn_args ),* );
            #write_return_value
        };
    }

    let thunk_name = make_rs_ident(thunk_name);
    Ok(RsSnippet {
        tokens: quote! {
            #[unsafe(no_mangle)]
            unsafe extern "C" fn #thunk_name (
                #( #thunk_params ),*
            ) -> #thunk_return_type { unsafe {
                #(#fn_args_conversions)*
                #thunk_return_expression
            } }
        },
        extern_c_decls,
    })
}

/// Returns `Ok(())` if no thunk is required.
/// Otherwise returns an error the describes why the thunk is needed.
pub fn is_thunk_required(tcx: TyCtxt<'_>, sig: &ty::FnSig) -> Result<()> {
    match sig.abi {
        // "C" ABI is okay: since https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html has been
        // accepted, a Rust panic that "escapes" a "C" ABI function is a defined crash. See
        // https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-unwinding.
        rustc_abi::ExternAbi::C { unwind: false } => (),

        // This requires a thunk if the calling C++ frames use `-fno-exceptions`, as it is
        // UB. However, we leave this to the caller: if you use `extern "C-unwind"`, we assume you
        // know what you are doing and do not block you from integrating with exception-enabled C++.
        rustc_abi::ExternAbi::C { unwind: true } => (),

        // All other ABIs trigger thunk generation.  This covers Rust ABI functions, but also
        // ABIs that theoretically are understood both by C++ and Rust (e.g. see
        // `format_cc_call_conv_as_clang_attribute` in `rs_bindings_from_cc/src_code_gen.rs`).
        _ => bail!("Any calling convention other than `extern \"C\"` requires a thunk"),
    };

    ensure!(is_c_abi_compatible_by_value(tcx, sig.output()), "Return type requires a thunk");
    for (i, param_ty) in sig.inputs().iter().enumerate() {
        ensure!(
            is_c_abi_compatible_by_value(tcx, *param_ty),
            "Type of parameter #{i} requires a thunk"
        );
    }

    Ok(())
}

pub struct TraitThunks {
    pub method_name_to_cc_thunk_name: HashMap<Symbol, Ident>,
    pub cc_thunk_decls: CcSnippet,
    pub rs_thunk_impls: RsSnippet,
}

pub fn generate_trait_thunks<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    trait_id: DefId,
    // We do not support other generic args, yet.
    type_args: &[Ty<'tcx>],
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
    } else if !does_type_implement_trait(
        tcx,
        self_ty,
        trait_id,
        type_args.iter().copied().map(ty::GenericArg::from),
    ) {
        let display_name = db
            .symbol_canonical_name(adt.def_id)
            .map(|canon| {
                let parts = canon.rs_name_parts().map(|s| format!("{}", s)).collect::<Vec<_>>();
                parts.join("::")
            })
            .unwrap_or_else(|| format!("{self_ty}"));
        let trait_name = tcx.item_name(trait_id);
        bail!("`{display_name}` doesn't implement the `{trait_name}` trait");
    }

    let mut method_name_to_cc_thunk_name = HashMap::new();
    let mut cc_thunk_decls = CcSnippet::default();
    let mut rs_thunk_impls = RsSnippet::default();
    let methods = tcx
        .associated_items(trait_id)
        .in_definition_order()
        .filter(|item| matches!(item.kind, ty::AssocKind::Fn { .. }));
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
            tcx.mk_args_trait(self_ty, type_args.iter().copied().map(ty::GenericArg::from))
        };

        let thunk_name = {
            if db.no_thunk_name_mangling() {
                let print_types = type_args.iter().map(|ty| format!("{}", ty)).collect_vec();
                let method_name = if print_types.is_empty() {
                    escape_non_identifier_chars(method.name().as_str())
                } else {
                    escape_non_identifier_chars(&format!(
                        "{}_{}",
                        method.name().as_str(),
                        print_types.join("_")
                    ))
                };
                format!("__crubit_thunk_{}", method_name)
            } else {
                #[rustversion::since(2025-05-06)]
                let instance = ty::Instance::new_raw(method.def_id, substs);
                #[rustversion::before(2025-05-06)]
                let instance = ty::Instance::new(method.def_id, substs);

                let symbol = tcx.symbol_name(instance);
                format!(
                    "__crubit_thunk_{}_{}",
                    tcx.crate_hash(db.source_crate_num()).to_hex(),
                    &escape_non_identifier_chars(symbol.name)
                )
            }
        };

        let sig_mid = liberate_and_deanonymize_late_bound_regions(
            tcx,
            tcx.fn_sig(method.def_id).instantiate(tcx, substs),
            method.def_id,
        );
        // TODO(b/254096006): Preserve the HIR here, if possible?
        // Cannot in general (e.g. blanket impl from another crate), but should be able
        // to for traits defined or implemented in the current crate.
        let sig_hir = None;

        let thunk_name_cc_ident = format_cc_ident(db, &thunk_name)?;
        cc_thunk_decls.add_assign(generate_thunk_decl(
            db,
            &sig_mid,
            sig_hir,
            &thunk_name_cc_ident,
            /*has_self_param=*/ true,
        )?);
        method_name_to_cc_thunk_name.insert(method.name(), thunk_name_cc_ident);

        rs_thunk_impls += {
            let struct_name = &adt.rs_fully_qualified_name;
            if is_drop_trait {
                // Manually formatting (instead of depending on `generate_thunk_impl`)
                // to avoid https://doc.rust-lang.org/error_codes/E0040.html
                let thunk_name = make_rs_ident(&thunk_name);
                RsSnippet::new(quote! {
                    #[unsafe(no_mangle)]
                    extern "C" fn #thunk_name(
                        __self: &'static mut ::core::mem::MaybeUninit<#struct_name>
                    ) {
                        unsafe { __self.assume_init_drop() };
                    }
                })
            } else {
                let fully_qualified_fn_name = {
                    let fully_qualified_trait_name = db
                        .symbol_canonical_name(trait_id)
                        .ok_or_else(|| anyhow!("Failed to get canonical name for {trait_id:?}"))?
                        .format_for_rs();
                    let method_name = make_rs_ident(method.name().as_str());
                    let args = type_args
                        .iter()
                        .map(|ty| {
                            let static_ty = replace_all_regions_with_static(tcx, *ty);
                            // Check our type has no variables.
                            assert!(
                                !static_ty.flags().contains(
                                    ty::TypeFlags::HAS_PARAM
                                        | ty::TypeFlags::HAS_INFER
                                        | ty::TypeFlags::HAS_PLACEHOLDER
                                        | ty::TypeFlags::HAS_FREE_REGIONS
                                ),
                                "Generic types are not supported in trait impls yet."
                            );
                            db.format_ty_for_rs(static_ty)
                                .expect("We've replaced all types with static")
                        })
                        .collect_vec();
                    let generics = if args.is_empty() {
                        quote! {}
                    } else {
                        quote! { < #( #args ),* > }
                    };
                    quote! { <#struct_name as #fully_qualified_trait_name #generics >::#method_name }
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
