// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use code_gen_utils::{expect_format_cc_ident, expect_format_cc_type_name, make_rs_ident};
use crubit_abi_type::CrubitAbiTypeToCppTokens;
use database::code_snippet::{Thunk, ThunkImpl};
use database::db::BindingsGenerator;
use database::rs_snippet::{
    format_generic_params, unique_lifetimes, BridgeRsTypeKind, Lifetime, Mutability, RsTypeKind,
};
use error_report::{anyhow, bail};
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::borrow::Cow;
use std::fmt::Write;
use std::rc::Rc;
use unicode_ident::is_xid_continue;

/// If we know the original C++ function is codegenned and already compatible
/// with `extern "C"` calling convention we skip creating/calling the C++ thunk
/// since we can call the original C++ directly.
pub fn can_skip_cc_thunk(db: &dyn BindingsGenerator, func: &Func) -> bool {
    // ## Inline functions
    //
    // Inline functions may not be codegenned in the C++ library since Clang doesn't
    // know if Rust calls the function or not. Therefore in order to make inline
    // functions callable from Rust we need to generate a C++ file that defines
    // a thunk that delegates to the original inline function. When compiled,
    // Clang will emit code for this thunk and Rust code will call the
    // thunk when the user wants to call the original inline function.
    //
    // This is not great runtime-performance-wise in regular builds (inline function
    // will not be inlined, there will always be a function call), but it is
    // correct. ThinLTO builds will be able to see through the thunk and inline
    // code across the language boundary. For non-ThinLTO builds we plan to
    // implement <internal link> which removes the runtime performance overhead.
    if func.is_inline {
        return false;
    }
    // ## Member functions (or descendants) of class templates
    //
    // A thunk is required to force/guarantee template instantiation.
    if func.is_member_or_descendant_of_class_template {
        return false;
    }
    // ## Virtual functions
    //
    // When calling virtual `A::Method()`, it's not necessarily the case that we'll
    // specifically call the concrete `A::Method` impl. For example, if this is
    // called on something whose dynamic type is some subclass `B` with an
    // overridden `B::Method`, then we'll call that.
    //
    // We must reuse the C++ dynamic dispatching system. In this case, the easiest
    // way to do it is by resorting to a C++ thunk, whose implementation will do
    // the lookup.
    //
    // In terms of runtime performance, since this only occurs for virtual function
    // calls, which are already slow, it may not be such a big deal. We can
    // benchmark it later. :)
    if let Some(meta) = &func.member_func_metadata {
        if let Some(inst_meta) = &meta.instance_method_metadata {
            if inst_meta.is_virtual {
                return false;
            }
        }
    }
    // ## Custom calling convention requires a thunk.
    //
    // The thunk has the "C" calling convention, and internally can call the
    // C++ function using any of the calling conventions supported by the C++
    // compiler (which might not always match the set supported by Rust - e.g.,
    // abi.rs doesn't contain "swiftcall" from
    // clang::FunctionType::getNameForCallConv)
    if !func.has_c_calling_convention {
        return false;
    }

    // ## Returning structs by value.
    //
    // Returning a struct by value requires an explicit thunk, because
    // `rs_bindings_from_cc` may not preserve the ABI of structs (e.g. when
    // replacing field types with an opaque blob of bytes - see b/270454629).
    //
    // Note: if the RsTypeKind cannot be parsed / rs_type_kind returns Err, then
    // bindings generation will fail for this function, so it doesn't really matter
    // what we do here.
    if let Ok(return_type) = db.rs_type_kind(func.return_type.clone()) {
        if !return_type.is_c_abi_compatible_by_value() {
            return false;
        }
    }
    // ## Nontrivial parameter types.
    //
    // If the function accepts a struct by value, then in the underlying ABI, it is
    // actually passed by pointer.
    //
    // Because there's no way to upgrade an lvalue (e.g. pointer) to a prvalue, we
    // cannot implement guaranteed copy/move elision for inline functions for
    // now: any thunk we generate would need to invoke the correct function as
    // if by magic.
    //
    // And so for now, we always use C++11 semantics, via an intermediate thunk.
    //
    // (As a side effect, this, like return values, means that support is
    // ABI-agnostic.)
    for param in &func.params {
        if let Ok(param_type) = db.rs_type_kind(param.type_.clone()) {
            if !param_type.is_c_abi_compatible_by_value() {
                return false;
            }
        }
    }

    true
}

pub fn generate_function_thunk(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_idents: &[Ident],
    param_types: &[RsTypeKind],
    return_type: &RsTypeKind,
) -> Result<Thunk> {
    // The first parameter is the output parameter, if any.
    let mut param_types = param_types.iter();
    let mut param_idents = param_idents.iter();
    let mut out_param = None;
    let mut out_param_ident = None;

    // Elided lifetimes in return position are replaced with a named lifetime in order to avoid
    // errors in the case of multiple elided input lifetimes.
    //
    // Note: this transformation is not nested since thunk return types will only have lifetimes
    // in the case of references, as more complex lifetime types will be transformed to out-params.
    // This is somewhat fragile and could be corrected with a more complex `map` transformation
    // over `RsTypeKind`.
    let mut return_type = return_type.clone();
    let extra_return_lifetime = match &mut return_type {
        RsTypeKind::Reference { lifetime, .. } if lifetime.is_elided() => {
            *lifetime = Lifetime::new("__return_lifetime");
            Some(lifetime.clone())
        }
        _ => None,
    };

    let mut return_type_fragment = return_type.format_as_return_type_fragment(db, None);
    if func.rs_name == UnqualifiedIdentifier::Constructor {
        // For constructors, inject MaybeUninit into the type of `__this_` parameter.
        let Some(first_param) = param_types.next() else {
            bail!("Constructors should have at least one parameter (__this), but none were found.")
        };
        let RsTypeKind::Reference { mutability: Mutability::Mut, .. } = first_param else {
            bail!(
                "Expected first constructor parameter to be a mutable reference, got: {}",
                first_param.display(db)
            )
        };
        out_param = Some(quote! { *mut ::core::ffi::c_void });
        out_param_ident = Some(param_idents.next().unwrap().clone());
    } else if return_type.as_c9_co().is_some() {
        // Returning a Co involves passing a CoVtable out ptr.
        out_param = Some(quote! { *mut ::co_vtable::c9::internal::rust::CoVtable });
        out_param_ident = Some(make_rs_ident("__return_co_vtable"));
        return_type_fragment = None;
    } else if return_type.is_crubit_abi_bridge_type() {
        out_param = Some(quote! { *mut ::core::ffi::c_uchar });
        out_param_ident = Some(make_rs_ident("__return_abi_buffer"));
        return_type_fragment = None;
    } else if !return_type.is_c_abi_compatible_by_value() {
        // For return types that can't be passed by value, create a new out parameter.
        out_param = Some(quote! { *mut ::core::ffi::c_void });
        out_param_ident = Some(make_rs_ident("__return"));
        return_type_fragment = None;
    }

    // Of the remaining lifetimes, collect them.
    let lifetimes: Vec<_> =
        unique_lifetimes(param_types.clone()).chain(extra_return_lifetime).collect();

    let thunk_ident = thunk_ident(func);

    let generic_params = format_generic_params(&lifetimes, std::iter::empty::<syn::Ident>());
    let param_idents =
        out_param_ident.as_ref().into_iter().chain(param_idents).cloned().collect_vec();
    let param_types = out_param
        .into_iter()
        .chain(param_types.map(|param_type| {
            if param_type.is_crubit_abi_bridge_type() {
                quote! { *const ::core::ffi::c_uchar }
            } else if !param_type.is_c_abi_compatible_by_value() {
                let param_type_tokens = param_type.to_token_stream(db);
                quote! {&mut #param_type_tokens}
            } else {
                param_type.to_token_stream(db)
            }
        }))
        .collect_vec();

    Ok(Thunk::Function {
        mangled_name: can_skip_cc_thunk(db, func).then(|| func.mangled_name.clone()),
        thunk_ident,
        generic_params,
        param_idents,
        param_types,
        return_type_fragment,
    })
}

// Converts `mangled_name` into a string that can be used within an identifier.
// All characters in the result are guaranteed to be from the XID_Continue class (though not
// necessarily XID_Start, so the fragment can't be used at the start of an identifier).
//
// The escaping scheme is not collision-free, i.e. two different inputs may map to the same output.
// In practice, though, collisions are extremely unlikely, and other aspects of the way we create
// thunk names may also cause (very unlikely) collisions.
fn ident_fragment_from_mangled_name(mangled_name: &str) -> Cow<'_, str> {
    // LLVM identifiers use the `\01` prefix to suppress mangling:
    // https://llvm.org/docs/LangRef.html#identifiers
    // We won't be passing the name to LLVM anyway, so we simply strip the prefix if present.
    let mangled_name = mangled_name.strip_prefix('\u{1}').unwrap_or(mangled_name);

    if mangled_name.chars().all(is_xid_continue) {
        return mangled_name.into();
    }

    let mut ident_name = String::new();

    for c in mangled_name.chars() {
        if is_xid_continue(c) {
            ident_name.push(c);
        } else {
            let _ = write!(ident_name, "_u{}_", c as u32);
        }
    }

    ident_name.into()
}

pub fn thunk_ident(func: &Func) -> Ident {
    let odr_suffix = if func.is_member_or_descendant_of_class_template {
        func.owning_target.convert_to_cc_identifier()
    } else {
        String::new()
    };
    format_ident!(
        "__rust_thunk__{}{odr_suffix}",
        ident_fragment_from_mangled_name(func.mangled_name.as_ref())
    )
}

fn generate_function_assertation_for_identifier(
    db: &dyn BindingsGenerator,
    func: &Func,
    id: &Identifier,
) -> Result<ThunkImpl> {
    let ir = db.ir();

    let fn_ident = expect_format_cc_ident(&id.identifier);
    let method_qualification;
    let implementation_function;
    let member_function_prefix;
    let func_params;
    if let Some(meta) = func.member_func_metadata.as_ref() {
        let record: &Rc<Record> = ir.find_decl(meta.record_id)?;
        let record_ident = expect_format_cc_type_name(record.cc_name.identifier.as_ref());
        let namespace_qualifier = ir.namespace_qualifier(record).format_for_cc()?;
        if let Some(instance_method_metadata) = meta.instance_method_metadata.as_ref() {
            let const_qualifier = if instance_method_metadata.is_const {
                quote! {const}
            } else {
                quote! {}
            };

            method_qualification = match instance_method_metadata.reference {
                ir::ReferenceQualification::Unqualified => const_qualifier,
                ir::ReferenceQualification::LValue => {
                    quote! { #const_qualifier & }
                }
                ir::ReferenceQualification::RValue => {
                    quote! { #const_qualifier && }
                }
            };
            implementation_function = quote! { #namespace_qualifier #record_ident :: #fn_ident };
            member_function_prefix = quote! { :: #namespace_qualifier #record_ident :: };
            // The first parameter of instance methods is `this`.
            func_params = &func.params[1..];
        } else {
            method_qualification = quote! {};
            implementation_function = quote! { #namespace_qualifier #record_ident :: #fn_ident };
            member_function_prefix = quote! {};
            func_params = &func.params[..];
        }
    } else {
        let namespace_qualifier = ir.namespace_qualifier(func).format_for_cc()?;
        method_qualification = quote! {};
        implementation_function = quote! { #namespace_qualifier #fn_ident };
        member_function_prefix = quote! {};
        func_params = &func.params[..];
    }

    let mut cc_param_types = func_params
        .iter()
        .map(|p| {
            let mut tt = cpp_type_name::format_cpp_type_with_references(
                &db.rs_type_kind(p.type_.clone())?,
                ir,
            )?;
            if p.type_.is_const {
                tt = quote! { #tt const };
            }
            Ok(tt)
        })
        .collect::<Result<Vec<_>>>()?;
    if func.is_variadic {
        cc_param_types.push(quote! { ... });
    }

    let mut return_type_name = cpp_type_name::format_cpp_type_with_references(
        &db.rs_type_kind(func.return_type.clone())?,
        ir,
    )?;

    if func.return_type.is_const {
        return_type_name = quote! { #return_type_name const };
    }

    let cc_function_type = quote! {
        #return_type_name
        ( #member_function_prefix* )
        ( #( #cc_param_types ),* )
        #method_qualification
    };

    Ok(ThunkImpl::FuntionTypeAssertation { cc_function_type, implementation_function })
}

pub fn generate_function_assertation(
    db: &dyn BindingsGenerator,
    func: &Func,
) -> Result<Option<ThunkImpl>> {
    if func.adl_enclosing_record.is_some() {
        // This is a friend function that is only reachable with ADL. We can't take the address.
        return Ok(None);
    }

    // TODO: b/393169953 - support functions with non-standard calling conventions
    if !func.has_c_calling_convention {
        return Ok(None);
    }

    match &func.cc_name {
        UnqualifiedIdentifier::Identifier(id) => {
            Ok(Some(generate_function_assertation_for_identifier(db, func, id)?))
        }
        // TODO: b/393169953 - support operators
        UnqualifiedIdentifier::Operator(_op) => Ok(None),
        UnqualifiedIdentifier::Constructor => Ok(None),
        UnqualifiedIdentifier::Destructor => Ok(None),
    }
}

// Returns whether `func` is a copy constructor of `record_id`, assuming that `func` is a
// constructor member function of `record_id`.
// TODO: b/436870965 - do we need to distinguish between non-const and const ctors?
fn is_copy_constructor(func: &Func, record_id: ItemId) -> bool {
    match &func.params[..] {
    // We already know this is a constructor.
    [FuncParam { type_: CcType { variant: CcTypeVariant::Pointer(_), ..}, .. },
    // Match on any C([const] C&).
     FuncParam { type_: CcType { variant: CcTypeVariant::Pointer(
        PointerType {kind: PointerTypeKind::LValueRef, pointee_type: inner_type, ..}), ..}, .. }] =>
        matches!(&**inner_type, CcType { variant: CcTypeVariant::Decl(rid), ..} if *rid == record_id),
    _ => false
  }
}

pub fn generate_function_thunk_impl(
    db: &dyn BindingsGenerator,
    func: &Func,
) -> Result<Option<ThunkImpl>> {
    if can_skip_cc_thunk(db, func) {
        return Ok(None);
    }
    let ir = db.ir();
    let thunk_ident = thunk_ident(func);
    let implementation_function = match &func.cc_name {
        UnqualifiedIdentifier::Operator(op) => {
            let name = syn::parse_str::<TokenStream>(&op.name)?;
            quote! { operator #name }
        }
        UnqualifiedIdentifier::Identifier(id) => {
            let fn_ident = expect_format_cc_ident(&id.identifier);
            match func.member_func_metadata.as_ref() {
                Some(meta) => {
                    if meta.instance_method_metadata.is_some() {
                        quote! { #fn_ident }
                    } else {
                        let record: &Rc<Record> = ir.find_decl(meta.record_id)?;
                        let record_name =
                            expect_format_cc_type_name(record.cc_name.identifier.as_ref());
                        let namespace_qualifier = ir.namespace_qualifier(record).format_for_cc()?;
                        quote! { #namespace_qualifier #record_name :: #fn_ident }
                    }
                }
                None => {
                    let namespace_qualifier = ir.namespace_qualifier(func).format_for_cc()?;
                    quote! { #namespace_qualifier #fn_ident }
                }
            }
        }
        // Use `destroy_at` to avoid needing to spell out the class name. Destructor identiifers
        // use the name of the type itself, without namespace qualification, template
        // parameters, or aliases. We do not need to use that naming scheme anywhere else in
        // the bindings, and it can be difficult (impossible?) to spell in the general case. By
        // using destroy_at, we avoid needing to determine or remember what the correct spelling
        // is. Similar arguments apply to `construct_at`.
        UnqualifiedIdentifier::Constructor => {
            if let Some(meta) = func.member_func_metadata.as_ref() {
                let record: &Rc<Record> = ir.find_decl(meta.record_id)?;
                if is_copy_constructor(func, record.id)
                    && record.copy_constructor == SpecialMemberFunc::Unavailable
                {
                    bail!(
                        "Would use an unavailable copy constructor for {}",
                        record.cc_name.identifier.as_ref()
                    );
                }
            }
            quote! { crubit::construct_at }
        }
        UnqualifiedIdentifier::Destructor => quote! {std::destroy_at},
    };

    let mut param_idents =
        func.params.iter().map(|p| expect_format_cc_ident(&p.identifier.identifier)).collect_vec();

    let mut conversion_externs = quote! {};
    let mut conversion_stmts = quote! {};
    let convert_ident =
        |ident: &Ident| -> Ident { format_ident!("__converted_{}", ident.to_string()) };
    let mut param_types = func
        .params
        .iter()
        .map(|p| {
            let arg_type = db.rs_type_kind(p.type_.clone())?;
            let cpp_type = cpp_type_name::format_cpp_type(&arg_type, ir)?;
            if let RsTypeKind::BridgeType { bridge_type, .. } = arg_type.unalias() {
                let BridgeRsTypeKind::BridgeVoidConverters { rust_to_cpp_converter, .. } =
                    bridge_type
                else {
                    return Ok(quote! { const unsigned char* });
                };

                let convert_function = expect_format_cc_ident(rust_to_cpp_converter);
                let ident = expect_format_cc_ident(&p.identifier.identifier);
                let cpp_ident = convert_ident(&ident);
                conversion_externs.extend(quote! {
                    extern "C" void #convert_function(void* rust_struct, void* cpp_struct);
                });
                conversion_stmts.extend(quote! {
                    ::crubit::LazyInit<#cpp_type> #cpp_ident;
                });
                conversion_stmts.extend(quote! {
                    #convert_function(#ident, &#cpp_ident.val);
                });
                Ok(quote! { void* })
            } else if !arg_type.is_c_abi_compatible_by_value() {
                // non-Unpin types are wrapped by a pointer in the thunk.
                Ok(quote! {#cpp_type *})
            } else {
                Ok(cpp_type)
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let arg_expressions = func
        .params
        .iter()
        .map(|p| {
            let ident = expect_format_cc_ident(&p.identifier.identifier);
            let ident = if db.rs_type_kind(p.type_.clone())?.is_pointer_bridge_type() {
                let formatted_ident = convert_ident(&ident);
                quote! { &(#formatted_ident.val) }
            } else {
                ident.to_token_stream()
            };
            match &p.type_.variant {
                CcTypeVariant::Pointer(pointer) => match pointer.kind {
                    PointerTypeKind::RValueRef => Ok(quote! { std::move(*#ident) }),
                    PointerTypeKind::LValueRef => Ok(quote! { *#ident }),
                    PointerTypeKind::Nullable | PointerTypeKind::NonNull => Ok(quote! { #ident }),
                },
                CcTypeVariant::FuncPointer { non_null, .. } => {
                    if *non_null {
                        Ok(quote! { *#ident })
                    } else {
                        Ok(quote! { #ident })
                    }
                }
                _ => {
                    let rs_type_kind = db.rs_type_kind(p.type_.clone())?;
                    // non-Unpin types are wrapped by a pointer in the thunk.
                    if rs_type_kind.is_crubit_abi_bridge_type() {
                        let crubit_abi_type = db.crubit_abi_type(rs_type_kind)?;
                        let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
                        Ok(quote! { ::crubit::internal::Decode<#crubit_abi_type_tokens>(#ident) })
                    } else if !rs_type_kind.is_c_abi_compatible_by_value() {
                        Ok(quote! { std::move(* #ident) })
                    } else if rs_type_kind.is_primitive() || rs_type_kind.referent().is_some() {
                        Ok(quote! { #ident })
                    } else {
                        Ok(quote! { std::move( #ident) })
                    }
                }
            }
        })
        .collect::<Result<Vec<_>>>()?;

    // Here, we add a `__return` parameter if the return type can't be passed by
    // value across `extern "C"` ABI.  (We do this after the arg_expressions
    // computation, so that it's only in the parameter list, not the argument
    // list.)
    let return_type_kind = db.rs_type_kind(func.return_type.clone())?;
    let is_return_value_c_abi_compatible = return_type_kind.is_c_abi_compatible_by_value();
    let return_type_cpp_spelling = cpp_type_name::format_cpp_type(&return_type_kind, ir)?;

    let return_type_name = if return_type_kind.as_c9_co().is_some() {
        param_idents.insert(0, expect_format_cc_ident("__return_co_vtable"));
        param_types.insert(0, quote! { c9::internal::rust::CoVtable* });
        quote! {void}
    } else if return_type_kind.is_crubit_abi_bridge_type() {
        param_idents.insert(0, expect_format_cc_ident("__return_abi_buffer"));
        param_types.insert(0, quote! {unsigned char *});
        quote! { void }
    } else if !is_return_value_c_abi_compatible {
        param_idents.insert(0, expect_format_cc_ident("__return"));
        if let RsTypeKind::BridgeType {
            bridge_type: BridgeRsTypeKind::BridgeVoidConverters { cpp_to_rust_converter, .. },
            ..
        } = return_type_kind.unalias()
        {
            let convert_function = expect_format_cc_ident(cpp_to_rust_converter);
            conversion_externs.extend(quote! {
                extern "C" void #convert_function(void* cpp_struct, void* rust_struct);
            });
            param_types.insert(0, quote! {void *});
        } else {
            param_types.insert(0, quote! {#return_type_cpp_spelling *});
        }
        quote! {void}
    } else {
        return_type_cpp_spelling.clone()
    };

    let mut this_ref_qualification =
        func.member_func_metadata.as_ref().and_then(|meta| match &func.rs_name {
            UnqualifiedIdentifier::Constructor | UnqualifiedIdentifier::Destructor => None,
            UnqualifiedIdentifier::Identifier(_) | UnqualifiedIdentifier::Operator(_) => meta
                .instance_method_metadata
                .as_ref()
                .map(|instance_method| instance_method.reference),
        });
    if func.cc_name.is_constructor() {
        this_ref_qualification = None;
    }
    let (implementation_function, arg_expressions) =
        if let Some(this_ref_qualification) = this_ref_qualification {
            let this_param = func
                .params
                .first()
                .ok_or_else(|| anyhow!("Instance methods must have `__this` param."))?;

            let this_arg = expect_format_cc_ident(&this_param.identifier.identifier);
            let this_dot = if this_ref_qualification == ir::ReferenceQualification::RValue {
                quote! {std::move(*#this_arg).}
            } else {
                quote! {#this_arg->}
            };
            (quote! { #this_dot #implementation_function}, &arg_expressions[1..])
        } else {
            (implementation_function, &arg_expressions[..])
        };

    let return_expr = quote! {#implementation_function( #( #arg_expressions ),* )};
    let return_stmt = if let Some(result_type_kind) = return_type_kind.as_c9_co() {
        // The result_type_kind is the T in Co<T>
        let start_coroutine = if result_type_kind.is_void() {
            // For coroutines that return void, we use the non-templated version.
            quote! { &c9::internal::rust::StartCoroutineFromRust }
        } else {
            let result_type_crubit_abi_type = db.crubit_abi_type(result_type_kind.clone())?;
            let result_type_crubit_abi_type_tokens =
                CrubitAbiTypeToCppTokens(&result_type_crubit_abi_type);

            // For coroutines that return a non-void value, we use the templated version.
            quote! { &c9::internal::rust::StartCoroutineFromRust<#result_type_crubit_abi_type_tokens> }
        };
        let out_param = &param_idents[0];
        let result_type_cpp_spelling = cpp_type_name::format_cpp_type(result_type_kind, ir)?;
        quote! {
            #out_param->addr = #return_expr.release_handle(c9::internal::PassKey()).address();
            #out_param->destroy_at_initial_suspend = &c9::internal::rust::DestroyCoroutineFrameFromRust<#result_type_cpp_spelling>;
            #out_param->start_coroutine = #start_coroutine;
        }
    } else if return_type_kind.is_crubit_abi_bridge_type() {
        let out_param = &param_idents[0];
        let crubit_abi_type = db.crubit_abi_type(return_type_kind)?;
        let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
        quote! {
            ::crubit::internal::Encode<#crubit_abi_type_tokens>(#out_param, #return_expr)
        }
    } else if !is_return_value_c_abi_compatible {
        let out_param = &param_idents[0];
        if let RsTypeKind::BridgeType {
            bridge_type: BridgeRsTypeKind::BridgeVoidConverters { cpp_to_rust_converter, .. },
            ..
        } = return_type_kind.unalias()
        {
            let convert_function = expect_format_cc_ident(cpp_to_rust_converter);
            quote! {
                auto __original_cpp_struct = #return_expr;
                #convert_function(&__original_cpp_struct, #out_param)
            }
        } else {
            // Explicitly use placement `new` so that we get guaranteed copy elision in
            // C++17.
            quote! {new(#out_param) auto(#return_expr)}
        }
    } else {
        match &func.return_type.variant {
            CcTypeVariant::Primitive(Primitive::Void) => return_expr,
            CcTypeVariant::Pointer(PointerType { kind: PointerTypeKind::LValueRef, .. }) => {
                quote! { return std::addressof( #return_expr ) }
            }
            CcTypeVariant::Pointer(PointerType { kind: PointerTypeKind::RValueRef, .. }) => {
                let nested_type = cpp_type_name::format_cpp_type_with_references(
                    &db.rs_type_kind(func.return_type.clone())?,
                    ir,
                )?;
                quote! {
                    #nested_type lvalue = #return_expr;
                    return &lvalue
                }
            }
            CcTypeVariant::FuncPointer { non_null: true, .. } => quote! { return & #return_expr },
            _ => quote! { return #return_expr },
        }
    };

    Ok(Some(ThunkImpl::Function {
        conversion_externs,
        return_type_name,
        thunk_ident,
        param_types,
        param_idents,
        conversion_stmts,
        return_stmt,
    }))
}
