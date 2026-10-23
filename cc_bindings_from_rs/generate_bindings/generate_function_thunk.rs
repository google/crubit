// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]
#![feature(stmt_expr_attributes)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
use arc_anyhow::{Context, Result};
use code_gen_utils::escape_non_identifier_chars;
use code_gen_utils::make_rs_ident;
use crubit_abi_type::CrubitAbiTypeToRustExprTokens;
use database::code_snippet::{CcPrerequisites, CcSnippet, ExternCDecl, RsSnippet};
use database::{AdtCoreBindings, BindingsGenerator, TypeLocation};
use error_report::{anyhow, bail, ensure};
use format_type::CcType;
use format_type::{
    alias_ty_as_opaque_def_id, format_cc_ident, format_param_types_for_cc_thunk,
    format_ty_for_closure_param_rs, is_bridged_type, replace_all_regions_with_static,
    ty_as_alias_ty, BridgedType, CallableInfo, CallableKind,
};
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use query_compiler::{
    does_type_implement_trait, is_c_abi_compatible_by_value,
    liberate_and_deanonymize_late_bound_regions, normalize_ty, try_normalize,
};
use quote::format_ident;
use quote::quote;
use rustc_hir::def::DefKind;
#[rustversion::since(2026-04-22)]
use rustc_middle::ty::Flags;
use rustc_middle::ty::{self, Mutability, Ty, TyCtxt, TypingEnv};
use rustc_span::def_id::DefId;
use rustc_span::symbol::{kw, sym, Symbol};
use std::collections::{BTreeSet, HashMap};

/// How a function parameter is passed across the `extern "C"` thunk ABI.
pub enum ParamAbi<'tcx> {
    Callable(CallableInfo<'tcx>),
    Value(ValueAbi<'tcx>),
}

pub fn classify_param_abi<'tcx>(
    db: &BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
) -> Result<ParamAbi<'tcx>> {
    if let Some(info) = format_type::get_callable_info(db.tcx(), ty)? {
        return Ok(ParamAbi::Callable(info));
    }
    Ok(ParamAbi::Value(classify_value_abi(db, ty, /* is_constructor= */ false)?))
}

/// How a non-callable value (parameter or return value) is passed across the `extern "C"` thunk
/// ABI.
pub enum ValueAbi<'tcx> {
    Bridged(BridgedType),
    ByValue,
    LegacyTuple(&'tcx ty::List<Ty<'tcx>>),
    Array,
    ByPointer,
}

pub fn classify_value_abi<'tcx>(
    db: &BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
    is_constructor: bool,
) -> Result<ValueAbi<'tcx>> {
    if is_constructor {
        return Ok(if is_c_abi_compatible_by_value(db, ty) {
            ValueAbi::ByValue
        } else {
            ValueAbi::ByPointer
        });
    }
    if let Some(bridged_type) = is_bridged_type(db, ty)? {
        return Ok(ValueAbi::Bridged(bridged_type));
    }
    if is_c_abi_compatible_by_value(db, ty) {
        return Ok(ValueAbi::ByValue);
    }
    if let ty::TyKind::Tuple(tuple_tys) = ty.kind()
        && !db
            .crate_features(db.source_crate_num())
            .contains(crubit_feature::CrubitFeature::LayoutCompatTuple)
    {
        return Ok(ValueAbi::LegacyTuple(tuple_tys));
    }
    if let ty::TyKind::Array(inner_ty, _) = ty.kind() {
        // TODO: b/451981992 - Nested arrays containing types that are Drop but not Default do not
        // behave well in std::arrays.
        if let ty::TyKind::Array(nested_ty, _) = inner_ty.kind()
            && is_drop_not_default(db.tcx(), *nested_ty)
        {
            bail!(
                "b/260128806 - nested array {inner_ty} is not supported because it contains a type that implements Drop but not Default"
            );
        }
        return Ok(ValueAbi::Array);
    }
    Ok(ValueAbi::ByPointer)
}

/// Generates the C++ statement calling a constructor thunk to initialize `*this`.
pub fn generate_ctor_thunk_call<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
    thunk_name: &Ident,
    args: &[TokenStream],
) -> Result<TokenStream> {
    let thunk_qualifier = thunk_qualifier(core);
    Ok(
        if matches!(
            classify_value_abi(db, core.common.self_ty, /* is_constructor= */ true)?,
            ValueAbi::ByValue
        ) {
            quote! {
                *this = #thunk_qualifier::#thunk_name(#( #args ),*);
            }
        } else {
            let mut all_args = args.to_vec();
            all_args.push(quote! { this });
            quote! {
                #thunk_qualifier::#thunk_name(#( #all_args ),*);
            }
        },
    )
}

fn is_drop_not_default<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    if !ty.needs_drop(tcx, TypingEnv::fully_monomorphized()) {
        return false;
    }
    let trait_id =
        tcx.get_diagnostic_item(sym::Default).expect("Couldn't find `core::default::Default`");
    !does_type_implement_trait(tcx, ty, trait_id, [])
}

/// Formats a C++ declaration of a C-ABI-compatible-function wrapper around a Rust function.
pub fn generate_thunk_decl<'tcx>(
    db: &BindingsGenerator<'tcx>,
    sig_mid: &ty::FnSig<'tcx>,
    thunk_name: &Ident,
    has_self_param: bool,
    is_constructor: bool,
    within_template: bool,
    is_async: bool,
) -> Result<CcSnippet<'tcx>> {
    let tcx = db.tcx();
    let mut prereqs = CcPrerequisites::default();
    let actual_output_ty = if is_async {
        get_async_future_output_ty(tcx, sig_mid.output())?
    } else {
        sig_mid.output()
    };
    let main_api_ret_type = if is_async {
        let cc_ret_ty = db
            .format_ty_for_cc(actual_output_ty, TypeLocation::FnReturn { is_constructor })?
            .into_tokens(&mut prereqs);
        prereqs.includes.insert(db.support_header("rs_std/dyn_erased_future.h"));
        quote! { ::crubit::DynErasedFuture<#cc_ret_ty> }
    } else {
        db.format_ty_for_cc(actual_output_ty, TypeLocation::FnReturn { is_constructor })
            .with_context(|| format!("Error formatting function return type `{actual_output_ty}`"))?
            .into_tokens(&mut prereqs)
    };

    let mut thunk_params = {
        let cpp_types = format_param_types_for_cc_thunk(db, sig_mid, has_self_param)?;
        sig_mid
            .inputs()
            .iter()
            .zip(cpp_types)
            .map(|(&ty, cpp_type)| -> Result<TokenStream> {
                match classify_param_abi(db, ty)? {
                    ParamAbi::Callable(info) => Ok(if info.kind.is_owning() {
                        prereqs.includes.insert(db.support_header("rs_std/fn.h"));
                        quote! { ::rs::internal::FnPayload }
                    } else {
                        prereqs.includes.insert(db.support_header("rs_std/fn_ref.h"));
                        quote! { ::rs::internal::FnRefPayload }
                    }),
                    ParamAbi::Value(ValueAbi::Bridged(bridged_type)) => match bridged_type {
                        BridgedType::ExternCFuncConverters {
                            cpp_type: bridged_cpp_type, ..
                        } => {
                            let cpp_type = cpp_type.snippet.into_tokens(&mut prereqs);
                            if matches!(bridged_cpp_type, CcType::Pointer { .. }) {
                                Ok(quote! { #cpp_type })
                            } else {
                                Ok(quote! { #cpp_type* })
                            }
                        }
                        BridgedType::Option(_) => Ok(quote! { unsigned char* }),
                    },
                    ParamAbi::Value(ValueAbi::ByValue) => {
                        let cpp_type = cpp_type.snippet.into_tokens(&mut prereqs);
                        Ok(quote! { #cpp_type })
                    }
                    ParamAbi::Value(ValueAbi::LegacyTuple(_)) => Ok(quote! { void** }),
                    ParamAbi::Value(ValueAbi::Array) => Ok(quote! { void* }),
                    ParamAbi::Value(ValueAbi::ByPointer) => {
                        if ty.ty_adt_def().is_some() && !db.is_cpp_move_constructible(ty) {
                            let underlying_ty = db
                                .format_ty_for_cc(ty, TypeLocation::Other)?
                                .into_tokens(&mut prereqs);
                            Ok(quote! { #underlying_ty* })
                        } else {
                            let cpp_type = cpp_type.snippet.into_tokens(&mut prereqs);
                            Ok(quote! { #cpp_type* })
                        }
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?
    };

    // Types which are not C-ABI compatible by-value are returned via out-pointer parameters.
    let thunk_ret_type = if is_async {
        let cc_ret_ty = db
            .format_ty_for_cc(actual_output_ty, TypeLocation::FnReturn { is_constructor })?
            .tokens;
        thunk_params.push(quote! { ::crubit::DynErasedFuture<#cc_ret_ty>* __ret_ptr });
        quote! { void }
    } else {
        match classify_value_abi(db, sig_mid.output(), is_constructor)? {
            ValueAbi::Bridged(BridgedType::ExternCFuncConverters { .. }) | ValueAbi::ByPointer => {
                thunk_params.push(quote! { #main_api_ret_type* __ret_ptr });
                quote! { void }
            }
            ValueAbi::Bridged(BridgedType::Option(_)) => {
                thunk_params.push(quote! { unsigned char * __ret_ptr });
                quote! { void }
            }
            ValueAbi::ByValue => main_api_ret_type,
            ValueAbi::LegacyTuple(_) => {
                thunk_params.push(quote! { void** __ret_ptr });
                quote! { void }
            }
            ValueAbi::Array => {
                thunk_params.push(quote! { void* __ret_ptr });
                quote! { void }
            }
        }
    };

    let mut attributes = vec![];
    // Attribute: noreturn
    let rs_return_type = sig_mid.output();
    if *rs_return_type.kind() == ty::TyKind::Never {
        attributes.push(quote! {[[noreturn]]});
    }

    let mut tokens = quote! {
        extern "C" #(#attributes)* #thunk_ret_type #thunk_name ( #( #thunk_params ),* );
    };
    if !within_template {
        tokens = quote! {
            namespace __crubit_internal {
                #tokens
            }
        }
    }

    Ok(CcSnippet { prereqs, tokens })
}

/// Creates Rust code to convert a bridged C++ type from a C ABI type to a Rust type.
///
/// Expects an existing local of type `cpp_type` named `local_name` and shadows it
/// with a local of type `ty` named `local_name`.
fn convert_bridged_type_from_c_abi_to_rust<'tcx>(
    db: &BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
    bridged_type: &BridgedType,
    local_name: &Ident,
    extern_c_decls: &mut BTreeSet<ExternCDecl>,
) -> Result<TokenStream> {
    let rs_type = db
        .format_ty_for_rs(ty)
        .with_context(|| format!("Error handling parameter `{local_name}`"))?;

    match bridged_type {
        BridgedType::ExternCFuncConverters { cpp_to_rust_converter, .. } => {
            let cpp_to_rust_converter_ident = add_extern_c_decl(
                extern_c_decls,
                ExternCDeclKind::CppToRustConverter,
                *cpp_to_rust_converter,
            );
            // The generated block below declares a local named `__crubit_temp`, which would
            // shadow the parameter or the converter if either of them had the same name.
            // `thunk_param_names` never produces `__crubit_`-prefixed names, so this can
            // only fail for a converter function literally named `__crubit_temp`.
            ensure!(
                *local_name != "__crubit_temp" && cpp_to_rust_converter_ident != "__crubit_temp",
                "`__crubit_temp` is reserved and cannot be used as the name of a parameter \
                 or a `cpp_to_rust_converter`"
            );
            Ok(quote! {
                let #local_name = {
                    let mut __crubit_temp = ::core::mem::MaybeUninit::<#rs_type>::uninit();
                    #cpp_to_rust_converter_ident(#local_name, __crubit_temp.as_mut_ptr() as *mut core::ffi::c_void);
                    __crubit_temp.assume_init()
                };
            })
        }
        BridgedType::Option(crubit_abi_type) => {
            let crubit_abi_type_expr = CrubitAbiTypeToRustExprTokens(crubit_abi_type);
            // SAFETY: The buffer is the correct size, as determined by Crubit.
            Ok(quote! {
                let #local_name = unsafe {
                    ::crubit_support::bridge::internal::decode(#crubit_abi_type_expr, #local_name)
                };
            })
        }
    }
}

/// Converts a local named `local_name` from its C ABI-compatible type
/// `*const [*const core::ffi::c_void; <tuple_tys.len()>]` to a tuple of Rust types.
fn convert_tuple_from_c_abi_to_rust<'tcx>(
    db: &BindingsGenerator<'tcx>,
    tuple_tys: &[Ty<'tcx>],
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
    db: &BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
    local_name: &Ident,
    extern_c_decls: &mut BTreeSet<ExternCDecl>,
) -> Result<TokenStream> {
    match classify_param_abi(db, ty)? {
        ParamAbi::Callable(_) | ParamAbi::Value(ValueAbi::ByValue) => Ok(quote! {}),
        ParamAbi::Value(ValueAbi::Bridged(bridged_type)) => {
            convert_bridged_type_from_c_abi_to_rust(
                db,
                ty,
                &bridged_type,
                local_name,
                extern_c_decls,
            )
        }
        ParamAbi::Value(ValueAbi::LegacyTuple(tuple_tys)) => {
            convert_tuple_from_c_abi_to_rust(db, tuple_tys, local_name, extern_c_decls)
        }
        ParamAbi::Value(ValueAbi::Array | ValueAbi::ByPointer) => {
            // Non-C-ABI-compatible-by-value types are passed by
            // `*mut T`, so we need to read out the value.
            Ok(quote! { let #local_name = #local_name.read(); })
        }
    }
}

fn c_abi_for_param_type<'tcx>(db: &BindingsGenerator<'tcx>, ty: Ty<'tcx>) -> Result<TokenStream> {
    match classify_param_abi(db, ty)? {
        ParamAbi::Callable(info) => Ok(if info.kind.is_owning() {
            quote! { ::crubit_support::bridge::FnPayload }
        } else {
            quote! { ::crubit_support::bridge::FnRefPayload }
        }),
        ParamAbi::Value(ValueAbi::Bridged(bridged_type)) => match bridged_type {
            BridgedType::ExternCFuncConverters { .. } => Ok(quote! { *const core::ffi::c_void }),
            BridgedType::Option(_) => Ok(quote! { *const core::ffi::c_uchar }),
        },
        ParamAbi::Value(ValueAbi::ByValue) => {
            let rs_type = db.format_ty_for_rs(ty)?;
            Ok(quote! { #rs_type })
        }
        ParamAbi::Value(ValueAbi::LegacyTuple(tuple_tys)) => {
            let num_elements = tuple_tys.len();
            Ok(quote! { *const [*const core::ffi::c_void; #num_elements] })
        }
        ParamAbi::Value(ValueAbi::Array | ValueAbi::ByPointer) => {
            let rs_type = db.format_ty_for_rs(ty)?;
            Ok(quote! { *mut #rs_type })
        }
    }
}

/// Returns an iterator which yields arbitrary unique names for the parameters
/// of the function identified by `fn_def_id`.
///
/// The returned names never start with `__crubit_`, which is reserved for locals introduced by
/// the generated thunk body.
fn thunk_param_names(tcx: TyCtxt<'_>, fn_def_id: DefId) -> impl Iterator<Item = Ident> + '_ {
    fn_arg_idents(tcx, fn_def_id).into_iter().enumerate().map(|(i, ident)| {
        let Some(ident) = ident.as_ref() else {
            return format_ident!("__param_{i}");
        };
        // TODO(jeanpierreda): Deduplicate the logic after the next rustc rollout.
        if ident.name == kw::Underscore
            || ident.name.is_empty()
            || ident.as_str().starts_with("__crubit_")
        {
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
///
/// `local_name_prefix` is used to name any locals introduced to hold parts of `rs_value` (e.g.
/// the elements of a tuple). It must be unique within the scope of the generated code, so that
/// these locals do not shadow other locals (including those of enclosing tuples).
fn write_rs_value_to_c_abi_ptr<'tcx>(
    db: &BindingsGenerator<'tcx>,
    rs_value: &TokenStream,
    local_name_prefix: &Ident,
    c_ptr: &Ident,
    rs_type: Ty<'tcx>,
    extern_c_decls: &mut BTreeSet<ExternCDecl>,
    is_constructor: bool,
) -> Result<TokenStream> {
    let write_directly = || quote! { ::core::ptr::write(#c_ptr as *mut _, #rs_value); };
    Ok(match classify_value_abi(db, rs_type, is_constructor)? {
        ValueAbi::Bridged(bridged_type) => match bridged_type {
            BridgedType::ExternCFuncConverters { rust_to_cpp_converter, .. } => {
                let rust_to_cpp_converter_ident = add_extern_c_decl(
                    extern_c_decls,
                    ExternCDeclKind::RustToCppConverter,
                    rust_to_cpp_converter,
                );
                // The temporary holding `rs_value` lives until the end of the statement, so
                // it is dropped only after the converter has run.
                quote! {
                    #rust_to_cpp_converter_ident(
                        ::core::ptr::from_ref(&#rs_value) as *const ::core::ffi::c_void,
                        #c_ptr);
                }
            }
            BridgedType::Option(crubit_abi_type) => {
                let crubit_abi_type_expr = CrubitAbiTypeToRustExprTokens(&crubit_abi_type);
                quote! {
                    // SAFETY: TODO(okabayashi)
                    unsafe {
                        ::crubit_support::bridge::internal::encode(
                            #crubit_abi_type_expr,
                            // TODO(okabayashi): This ptr case can be removed once tuple bridging is supported,
                            // as it only is required in the tuple recursive case.
                            #c_ptr as *mut core::ffi::c_uchar,
                            #rs_value
                        );
                    }
                }
            }
        },
        ValueAbi::LegacyTuple(tuple_tys) => {
            let num_elements = tuple_tys.len();
            let rs_element_names =
                (0..num_elements).map(|i| format_ident!("{local_name_prefix}_{i}")).collect_vec();
            let ptr_member_names =
                (0..num_elements).map(|i| format_ident!("{c_ptr}_{i}")).collect_vec();
            let unpack = quote! {
                let (#(#rs_element_names,)*) = #rs_value;
                let [#(#ptr_member_names),*] = *(#c_ptr as *mut [*mut core::ffi::c_void; #num_elements]);
            };
            let write_elements = (0..num_elements)
                .map(|i| {
                    let elem_ident = &rs_element_names[i];
                    write_rs_value_to_c_abi_ptr(
                        db,
                        &quote! { #elem_ident },
                        elem_ident,
                        &ptr_member_names[i],
                        tuple_tys[i],
                        extern_c_decls,
                        /* is_constructor= */ false,
                    )
                })
                .collect::<Result<TokenStream>>()?;
            quote! {
                #unpack
                #write_elements
            }
        }
        ValueAbi::ByValue | ValueAbi::Array | ValueAbi::ByPointer => write_directly(),
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
/// - `<::crate_name::some_module::SomeStruct as ::core::default::Default>::default`
pub fn generate_thunk_impl<'tcx>(
    db: &BindingsGenerator<'tcx>,
    fn_def_id: DefId,
    sig: &ty::FnSig<'tcx>,
    thunk_name: &str,
    fully_qualified_fn_name: TokenStream,
    is_constructor: bool,
    is_async: bool,
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
        param_names
            .zip(param_types)
            .enumerate()
            .map(|(i, (name, ty))| {
                let has_collision = if let Some(adt_def) = ty.peel_refs().ty_adt_def()
                    && adt_def.is_enum()
                {
                    adt_def
                        .variants()
                        .iter()
                        .any(|v| v.name.as_str() == name.to_string().trim_start_matches("r#"))
                } else {
                    false
                };
                if has_collision {
                    (format_ident!("__param_{i}"), ty)
                } else {
                    (name, ty)
                }
            })
            .collect_vec()
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

    let uninstantiated_sig = liberate_and_deanonymize_late_bound_regions(
        tcx,
        tcx.fn_sig(fn_def_id).instantiate_identity(),
        fn_def_id,
    );
    let fn_args: Vec<TokenStream> = param_names_and_types
        .iter()
        .enumerate()
        .map(|(i, (rs_name, ty))| -> Result<TokenStream> {
            if let Some(info) = format_type::get_callable_info(db.tcx(), *ty)? {
                let is_generic_param = matches!(
                    uninstantiated_sig.inputs().get(i).map(|t| t.kind()),
                    Some(ty::TyKind::Param(_))
                );
                format_callable_thunk_arg(db, rs_name, is_generic_param, &info)
            } else {
                Ok(quote! { #rs_name })
            }
        })
        .collect::<Result<Vec<TokenStream>>>()?;
    let thunk_return_type;
    let thunk_return_expression;
    if is_async {
        let return_ptr_ident = format_ident!("__ret_ptr");
        thunk_return_type = quote! { () };
        thunk_params.push(quote! {
            #return_ptr_ident: *mut ::crubit_support::dyn_erased_future::DynErasedFuture<'_>
        });
        thunk_return_expression = quote! {
            // SAFETY: `__ret_ptr` points to a valid, uninitialized crubit::Slot.
            #return_ptr_ident.write(
                ::crubit_support::dyn_erased_future::DynErasedFuture::new(#fully_qualified_fn_name( #( #fn_args ),* ))
            );
        };
    } else {
        let return_abi = classify_value_abi(db, sig.output(), is_constructor)?;
        if matches!(return_abi, ValueAbi::ByValue) {
            // The output is not bridged and is C ABI compatible by-value, so we can just return
            // the result directly, and no out-param is needed.
            thunk_return_type = db.format_ty_for_rs(sig.output())?;
            thunk_return_expression = quote! {
                #fully_qualified_fn_name( #( #fn_args ),* )
            };
        } else {
            let return_ptr_ident = format_ident!("__ret_ptr");
            thunk_return_type = quote! { () };

            let fn_call_expr = quote! { #fully_qualified_fn_name( #( #fn_args ),* ) };
            match return_abi {
                ValueAbi::Array | ValueAbi::ByPointer => {
                    // Like by-pointer params (see `c_abi_for_param_type`), the out-param is
                    // typed as `*mut T`, so the call result can be written directly into it.
                    let rs_type = db.format_ty_for_rs(sig.output())?;
                    thunk_params.push(quote! { #return_ptr_ident: *mut #rs_type });
                    thunk_return_expression = quote! {
                        #return_ptr_ident.write(#fn_call_expr);
                    };
                }
                _ => {
                    let return_ptr_type = match return_abi {
                        ValueAbi::Bridged(BridgedType::Option(_)) => {
                            // Composable bridging writes its Crubit ABI form in an unsigned char
                            // array.
                            quote! { *mut core::ffi::c_uchar }
                        }
                        _ => quote! { *mut core::ffi::c_void },
                    };
                    thunk_params.push(quote! {
                        #return_ptr_ident: #return_ptr_type
                    });
                    thunk_return_expression = write_rs_value_to_c_abi_ptr(
                        db,
                        &fn_call_expr,
                        &format_ident!("__rs_return_value"),
                        &return_ptr_ident,
                        sig.output(),
                        &mut extern_c_decls,
                        is_constructor,
                    )?;
                }
            }
        }
    }

    let thunk_name = make_rs_ident(thunk_name);
    Ok(RsSnippet::with_extern_c_decls(
        quote! {
            #[unsafe(no_mangle)]
            unsafe extern "C" fn #thunk_name (
                #( #thunk_params ),*
            ) -> #thunk_return_type { unsafe {
                #(#fn_args_conversions)*
                #thunk_return_expression
            } }
        },
        extern_c_decls,
    ))
}

/// Returns the raw pointer spelling of `ty`, if `ty` is a thin Rust reference.
///
/// The `__invoker` of a callable is an `extern "C"` function pointer that is shared with
/// C++, so its signature has to be spelled in a way that both languages agree on for the
/// purposes of cross-language CFI. A Rust reference `&T` is ABI-identical to `*const T`,
/// but the two are *not* encoded identically in the CFI type id: rustc encodes references
/// using a Rust-specific vendor extension (`u3refI..E`), whereas Clang encodes the
/// corresponding C++ pointer as `P..`. Spelling references as raw pointers makes the two
/// sides agree, so that the CFI check on the indirect call through `__invoker` passes.
///
/// Returns `None` for non-references and for wide references (`&str`, `&[T]`,
/// `&dyn Trait`), which are not ABI-compatible with a thin pointer.
fn format_ref_as_raw_ptr_for_invoker<'tcx>(
    db: &BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
) -> Result<Option<(Mutability, TokenStream)>> {
    let ty::TyKind::Ref(_, referent_ty, mutability) = ty.kind() else {
        return Ok(None);
    };
    if matches!(
        referent_ty.kind(),
        ty::TyKind::Str | ty::TyKind::Slice(_) | ty::TyKind::Dynamic(..)
    ) {
        return Ok(None);
    }
    let referent = format_ty_for_closure_param_rs(db, *referent_ty, false)?;
    Ok(Some(match mutability {
        Mutability::Mut => (Mutability::Mut, quote! { *mut #referent }),
        Mutability::Not => (Mutability::Not, quote! { *const #referent }),
    }))
}

fn format_callable_thunk_arg<'tcx>(
    db: &BindingsGenerator<'tcx>,
    rs_name: &Ident,
    is_generic_param: bool,
    info: &CallableInfo<'tcx>,
) -> Result<TokenStream> {
    let rs_ret_ty = format_ty_for_closure_param_rs(db, info.return_ty, true)?;
    let is_ret_unit = info.return_ty.is_unit();
    let is_ret_c_abi_compat = is_c_abi_compatible_by_value(db, info.return_ty);

    let mut closure_params = Vec::new();
    let mut invoker_param_tys = Vec::new();
    let mut prep_stmts = Vec::new();
    let mut invoker_args = Vec::new();

    for (arg_idx, &param_ty) in info.param_tys.iter().enumerate() {
        let arg_name = format_ident!("__arg_{arg_idx}");
        let rs_param_ty = format_ty_for_closure_param_rs(db, param_ty, false)?;
        closure_params.push(quote! { #arg_name: #rs_param_ty });

        if is_c_abi_compatible_by_value(db, param_ty) {
            if let Some((mutability, raw_ptr_ty)) = format_ref_as_raw_ptr_for_invoker(db, param_ty)?
            {
                invoker_param_tys.push(raw_ptr_ty);
                invoker_args.push(match mutability {
                    Mutability::Mut => quote! { #arg_name as *mut _ },
                    Mutability::Not => quote! { #arg_name as *const _ },
                });
            } else {
                invoker_param_tys.push(quote! { #rs_param_ty });
                invoker_args.push(quote! { #arg_name });
            }
        } else {
            invoker_param_tys.push(quote! { *mut #rs_param_ty });
            prep_stmts.push(quote! {
                let mut #arg_name = ::core::mem::ManuallyDrop::new(#arg_name);
            });
            invoker_args.push(quote! { &mut *#arg_name as *mut _ });
        }
    }

    let mut full_invoker_param_tys = vec![quote! { *mut core::ffi::c_void }];
    full_invoker_param_tys.extend(invoker_param_tys);

    let mut full_invoker_call_args = vec![quote! { #rs_name.data() }];
    full_invoker_call_args.extend(invoker_args);

    let (invoker_ret_ty, call_body) = if is_ret_c_abi_compat {
        if is_ret_unit {
            (
                quote! { () },
                quote! {
                    unsafe {
                        __invoker(#( #full_invoker_call_args ),*);
                    }
                },
            )
        } else if let Some((mutability, raw_ptr_ty)) =
            format_ref_as_raw_ptr_for_invoker(db, info.return_ty)?
        {
            let call = quote! { __invoker(#( #full_invoker_call_args ),*) };
            (
                raw_ptr_ty,
                match mutability {
                    Mutability::Mut => quote! { unsafe { &mut *#call } },
                    Mutability::Not => quote! { unsafe { &*#call } },
                },
            )
        } else {
            (
                quote! { #rs_ret_ty },
                quote! {
                    unsafe {
                        __invoker(#( #full_invoker_call_args ),*)
                    }
                },
            )
        }
    } else {
        full_invoker_param_tys.push(quote! { *mut core::ffi::c_void });
        (
            quote! { () },
            quote! {
                let mut __ret_storage = ::core::mem::MaybeUninit::<#rs_ret_ty>::uninit();
                unsafe {
                    __invoker(#( #full_invoker_call_args, )* __ret_storage.as_mut_ptr() as *mut _);
                    __ret_storage.assume_init()
                }
            },
        )
    };

    let closure = quote! {
        move |#( #closure_params ),*| {
            #( #prep_stmts )*
            let __invoker: unsafe extern "C" fn(#( #full_invoker_param_tys ),*) -> #invoker_ret_ty =
                unsafe { ::core::mem::transmute(#rs_name.invoker()) };
            #call_body
        }
    };
    if is_generic_param {
        Ok(quote! { #closure })
    } else {
        Ok(match info.kind {
            CallableKind::FnRef => quote! { &#closure },
            CallableKind::FnMutRef => quote! { &mut #closure },
            CallableKind::FnBox | CallableKind::FnMutBox | CallableKind::FnOnceBox => {
                quote! { ::alloc::boxed::Box::new(#closure) }
            }
        })
    }
}

pub struct TraitThunks<'tcx> {
    pub method_name_to_cc_thunk_name: HashMap<Symbol, Ident>,
    pub cc_thunk_decls: CcSnippet<'tcx>,
    pub rs_thunk_impls: RsSnippet,
}

#[allow(clippy::too_many_arguments)]
pub fn generate_trait_thunks<'tcx>(
    db: &BindingsGenerator<'tcx>,
    trait_id: DefId,
    // We do not support other generic args, yet.
    type_args: &[Ty<'tcx>],
    self_ty: Ty<'tcx>,
    def_id: Option<DefId>,
    rs_fully_qualified_name: TokenStream,
    is_constructor: bool,
    within_template: bool,
) -> Result<TraitThunks<'tcx>> {
    let tcx = db.tcx();
    assert!(tcx.is_trait(trait_id));

    let is_drop_trait = Some(trait_id) == tcx.lang_items().drop_trait();
    if is_drop_trait {
        // To support "drop glue" we don't require that `self_ty` directly implements
        // the `Drop` trait.  Instead we require the caller to check
        // `needs_drop`.
        assert!(self_ty.needs_drop(tcx, TypingEnv::fully_monomorphized()));
    } else if !does_type_implement_trait(
        tcx,
        self_ty,
        trait_id,
        type_args.iter().copied().map(ty::GenericArg::from),
    ) {
        let display_name = def_id
            .and_then(|id| db.symbol_canonical_name(id).ok())
            .map(|canon| {
                let parts = canon.rs_name_parts().map(|s| format!("{}", s)).collect::<Vec<_>>();
                parts.join("::")
            })
            .unwrap_or_else(|| format!("{self_ty}"));
        let trait_name = tcx.item_name(trait_id);
        bail!("`{display_name}` doesn't implement the `{trait_name}` trait");
    }

    fn is_supported_trait_method<'tcx>(tcx: TyCtxt<'tcx>, method_def_id: DefId) -> bool {
        // We want to check the `self` type of the method (and not the enclosing impl) so that we
        // see Pin<Self>/Box<Self>.
        let fn_sig = normalize_ty(
            tcx,
            tcx.param_env(method_def_id),
            tcx.fn_sig(method_def_id).instantiate_identity(),
        )
        .skip_binder();
        // If our method has no parameters, we treat it as supported.
        let Some(self_ty) = fn_sig.inputs().first() else {
            return true;
        };
        // We don't support trait methods that use `Box<Self>` or `Pin<Self>` yet.
        self_ty.pinned_ty().is_none() && self_ty.boxed_ty().is_none()
    }
    let substs = tcx.mk_args_trait(self_ty, type_args.iter().copied().map(ty::GenericArg::from));

    let mut method_name_to_cc_thunk_name = HashMap::new();
    let mut cc_thunk_decls = CcSnippet::default();
    let mut rs_thunk_impls = RsSnippet::default();
    let methods = tcx
        .associated_items(trait_id)
        .in_definition_order()
        .filter(|item| matches!(item.kind, ty::AssocKind::Fn { .. }))
        .filter(|item| is_supported_trait_method(tcx, item.def_id));
    for method in methods {
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

        let thunk_name = make_thunk_name(db, ThunkKind::TraitMethod { method, substs });

        // We normalize here to expand associated types to their underlying type.
        let sig_mid = liberate_and_deanonymize_late_bound_regions(
            tcx,
            tcx.fn_sig(method.def_id).instantiate(tcx, substs),
            method.def_id,
        );
        let sig_mid = try_normalize(tcx, TypingEnv::non_body_analysis(tcx, method.def_id), sig_mid)
            .expect("Normalization should succeed since this code typechecked");

        let thunk_name_cc_ident = format_cc_ident(db, &thunk_name)?;
        let is_async = tcx.asyncness(method.def_id).is_async();
        cc_thunk_decls += generate_thunk_decl(
            db,
            &sig_mid,
            &thunk_name_cc_ident,
            /* has_self_param= */ method.is_method(),
            is_constructor,
            within_template,
            is_async,
        )?;
        method_name_to_cc_thunk_name.insert(method.name(), thunk_name_cc_ident);

        rs_thunk_impls += if is_drop_trait {
            // Manually formatting (instead of depending on `generate_thunk_impl`)
            // to avoid https://doc.rust-lang.org/error_codes/E0040.html
            let thunk_name = make_rs_ident(&thunk_name);
            let struct_name = &rs_fully_qualified_name;
            RsSnippet::new(quote! {
                #[unsafe(no_mangle)]
                extern "C" fn #thunk_name(__self: *mut #struct_name) {
                    unsafe { ::core::ptr::drop_in_place(__self) };
                }
            })
        } else {
            let fully_qualified_trait_name = db.symbol_canonical_name(trait_id)?.format_for_rs();
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
                })
                .collect::<Result<Vec<_>>>()?;
            let generics = if args.is_empty() {
                quote! {}
            } else {
                quote! { < #( #args ),* > }
            };

            let struct_name = &rs_fully_qualified_name;
            let fully_qualified_fn_name = quote! {
                <#struct_name as #fully_qualified_trait_name #generics >::#method_name
            };
            generate_thunk_impl(
                db,
                method.def_id,
                &sig_mid,
                &thunk_name,
                fully_qualified_fn_name,
                is_constructor,
                is_async,
            )?
            .into_deduplicated_thunk(Symbol::intern(&thunk_name))
        };
    }

    Ok(TraitThunks { method_name_to_cc_thunk_name, cc_thunk_decls, rs_thunk_impls })
}

pub enum ThunkKind<'tcx> {
    Function { def_id: DefId, export_name: Option<Symbol>, substs: ty::GenericArgsRef<'tcx> },
    TraitMethod { method: &'tcx ty::AssocItem, substs: ty::GenericArgsRef<'tcx> },
}

pub fn make_thunk_name<'tcx>(db: &BindingsGenerator<'tcx>, kind: ThunkKind<'tcx>) -> String {
    let tcx = db.tcx();
    let is_golden_test = db.is_golden_test();
    let target_path_mangled_hash = if is_golden_test {
        "".to_string()
    } else {
        format!("{:x}_", tcx.stable_crate_id(db.source_crate_num()))
    };

    let details = match kind {
        ThunkKind::Function { def_id, export_name, substs } => {
            function_symbol_name(db, def_id, export_name, substs)
        }
        ThunkKind::TraitMethod { method, substs } => {
            if is_golden_test {
                let trait_id = tcx.parent(method.def_id);
                let trait_name = db
                    .symbol_unqualified_name(trait_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "Traits are assumed to always have a name {}",
                            tcx.def_path_str(trait_id)
                        )
                    })
                    .rs_name
                    .to_string();
                let method_symbol = method.name();
                let method_name = method_symbol.as_str();
                let args = substs.iter().map(|arg| arg.to_string()).collect_vec();
                format!("{}_{}_{}", trait_name, method_name, args.join("_"))
            } else {
                let instance = ty::Instance::new_raw(method.def_id, substs);

                let symbol = tcx.symbol_name(instance);
                symbol.name.to_string()
            }
        }
    };

    format!("__crubit_thunk_{}{}", target_path_mangled_hash, escape_non_identifier_chars(&details))
}

pub fn trait_method_thunk_name<'tcx>(
    db: &BindingsGenerator<'tcx>,
    trait_id: DefId,
    method_name: &str,
    self_ty: Ty<'tcx>,
    type_args: &[Ty<'tcx>],
) -> Result<String> {
    let tcx = db.tcx();
    let method = tcx
        .associated_items(trait_id)
        .in_definition_order()
        .find(|item| {
            item.name().as_str() == method_name && matches!(item.kind, ty::AssocKind::Fn { .. })
        })
        .ok_or_else(|| {
            anyhow!("Trait `{}` missing method `{method_name}`", tcx.def_path_str(trait_id))
        })?;
    let substs = tcx.mk_args_trait(self_ty, type_args.iter().copied().map(ty::GenericArg::from));
    Ok(make_thunk_name(db, ThunkKind::TraitMethod { method, substs }))
}

pub fn function_symbol_name<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
    export_name: Option<Symbol>,
    generic_args: ty::GenericArgsRef<'tcx>,
) -> String {
    let tcx = db.tcx();
    if db.is_golden_test() {
        if let Some(export_name) = export_name {
            export_name.to_string()
        } else {
            let def_name = db
                .symbol_unqualified_name(def_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Functions are assumed to always have a name {}",
                        tcx.def_path_str(def_id)
                    )
                })
                .rs_name
                .to_string();
            let base_name = tcx
                .trait_impl_of_assoc(def_id)
                .map(|impl_id| {
                    let trait_ref = normalize_ty(
                        tcx,
                        tcx.param_env(impl_id),
                        tcx.impl_trait_ref(impl_id).instantiate_identity(),
                    );
                    let trait_id = trait_ref.def_id;
                    let trait_name = db
                        .symbol_unqualified_name(trait_id)
                        .unwrap_or_else(|| {
                            panic!(
                                "Traits are assumed to always have a name {}",
                                tcx.def_path_str(trait_id)
                            )
                        })
                        .rs_name
                        .to_string();
                    let trait_args =
                        trait_ref.args.iter().map(|arg| format!("{}", arg)).collect_vec();
                    if trait_args.is_empty() {
                        format!("{}_{}", trait_name, def_name)
                    } else {
                        format!("{}_{}_{}", trait_name, def_name, trait_args.join("_"))
                    }
                })
                .unwrap_or(def_name);

            let args = generic_args.iter().map(|arg| format!("{}", arg)).collect_vec();
            if args.is_empty() {
                base_name
            } else {
                format!("{}_{}", base_name, args.join("_"))
            }
        }
    } else {
        let typing_env = ty::TypingEnv::non_body_analysis(tcx, def_id);
        let span = tcx.def_span(def_id);
        let instance = ty::Instance::expect_resolve(tcx, typing_env, def_id, generic_args, span);
        tcx.symbol_name(instance).name.to_string()
    }
}

pub fn fn_arg_idents(tcx: TyCtxt, def_id: DefId) -> Vec<Option<rustc_span::Ident>> {
    match tcx.def_kind(def_id) {
        DefKind::Ctor { .. } => {
            // Documentation of `skip_binder` says that accessing generic args in the returned value
            // is generally incorrect, but specifically points out that it should be okay to use
            // it to get the number of arguments of an `FnSig`.
            let arg_count = tcx.fn_sig(def_id).skip_binder().inputs().skip_binder().len();
            vec![None; arg_count]
        }
        _ => tcx.fn_arg_idents(def_id).iter().cloned().collect_vec(),
    }
}

/// If `rs_return_type` represents an async future desugared type, extracts and returns its `Output`
/// type.
pub fn get_async_future_output_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    rs_return_type: Ty<'tcx>,
) -> Result<Ty<'tcx>> {
    let Some(alias_ty) = ty_as_alias_ty(rs_return_type) else {
        bail!(
            "async functions should always return a TyKind::Alias (`{rs_return_type}`), this should never happen."
        );
    };
    if alias_ty_as_opaque_def_id(tcx, alias_ty).is_none() {
        bail!(
            "crubit.rs-bug: Future::Output alias is not an opaque type (`{rs_return_type}`), this should never happen."
        );
    };
    format_type::get_associated_type(tcx, rs_return_type, sym::Output).ok_or_else(|| {
        anyhow!("Failed to find Future::Output associated type in bounds of {:?}", rs_return_type)
    })
}

pub fn thunk_qualifier(core: &AdtCoreBindings<'_>) -> TokenStream {
    if core.def_id.is_none() {
        quote! { ::__crubit_internal }
    } else {
        quote! { __crubit_internal }
    }
}
