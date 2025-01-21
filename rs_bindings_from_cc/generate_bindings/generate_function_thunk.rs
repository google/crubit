// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use code_gen_utils::make_rs_ident;
use database::db::BindingsGenerator;
use database::rs_snippet::{format_generic_params, unique_lifetimes, Mutability, RsTypeKind};
use error_report::{anyhow, bail};
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::rc::Rc;

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
    if let Ok(return_type) = db.rs_type_kind(func.return_type.rs_type.clone()) {
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
        if let Ok(param_type) = db.rs_type_kind(param.type_.rs_type.clone()) {
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
    derived_record: Option<Rc<Record>>,
) -> Result<TokenStream> {
    let thunk_attr = if can_skip_cc_thunk(db, func) {
        let mangled_name = func.mangled_name.as_ref();
        quote! {#[link_name = #mangled_name]}
    } else {
        quote! {}
    };
    let lifetimes: Vec<_> = unique_lifetimes(param_types).collect();

    // The first parameter is the output parameter, if any.
    let mut param_types = param_types.iter();
    let mut param_idents = param_idents.iter();
    let mut out_param = None;
    let mut out_param_ident = None;
    let mut return_type_fragment = return_type.format_as_return_type_fragment(db, None);
    if func.name == UnqualifiedIdentifier::Constructor {
        // For constructors, inject MaybeUninit into the type of `__this_` parameter.
        let Some(first_param) = param_types.next() else {
            bail!("Constructors should have at least one parameter (__this), but none were found.")
        };
        let RsTypeKind::Reference { referent, lifetime, mutability: Mutability::Mut } = first_param
        else {
            bail!(
                "Expected first constructor parameter to be a mutable reference, got: {}",
                first_param.display(db)
            )
        };
        let lifetime = lifetime.format_for_reference();
        let referent_tokens = referent.to_token_stream(db);
        out_param = Some(quote! { & #lifetime mut ::core::mem::MaybeUninit< #referent_tokens > });
        out_param_ident = Some(param_idents.next().unwrap().clone());
    } else if !return_type.is_c_abi_compatible_by_value() {
        let return_type_tokens = return_type.to_token_stream(db);
        // For return types that can't be passed by value, create a new out parameter.
        // The lifetime doesn't matter, so we can insert a new anonymous lifetime here.
        // TODO(yongheng): Switch to `void*`.
        out_param = Some(quote! {
            &mut ::core::mem::MaybeUninit< #return_type_tokens >
        });
        out_param_ident = Some(make_rs_ident("__return"));
        return_type_fragment = quote! {};
    }

    let thunk_ident = if let Some(derived_record) = derived_record {
        thunk_ident_for_derived_member_function(func, derived_record)
    } else {
        thunk_ident(func)
    };

    let generic_params = format_generic_params(&lifetimes, std::iter::empty::<syn::Ident>());
    let param_idents = out_param_ident.as_ref().into_iter().chain(param_idents);
    let param_types = out_param.into_iter().chain(param_types.map(|param_type| {
        let param_type_tokens = param_type.to_token_stream(db);
        if !param_type.is_c_abi_compatible_by_value() {
            quote! {&mut #param_type_tokens}
        } else {
            param_type_tokens
        }
    }));

    // Note: some of these are `safe`, but _all_ of them are currently wrapped by a
    // (possibly safe) function, so we leave them all `unsafe` for convenience.

    Ok(quote! {
        #thunk_attr
        pub(crate) unsafe fn #thunk_ident #generic_params( #( #param_idents: #param_types ),*
        ) #return_type_fragment ;
    })
}

pub fn thunk_ident(func: &Func) -> Ident {
    let odr_suffix = if func.is_member_or_descendant_of_class_template {
        func.owning_target.convert_to_cc_identifier()
    } else {
        String::new()
    };
    format_ident!("__rust_thunk__{}{odr_suffix}", func.mangled_name.as_ref())
}

pub fn thunk_ident_for_derived_member_function(func: &Func, derived_record: Rc<Record>) -> Ident {
    let odr_suffix = if func.is_member_or_descendant_of_class_template {
        func.owning_target.convert_to_cc_identifier()
    } else {
        String::new()
    };
    format_ident!(
        "__rust_thunk__{}{odr_suffix}_{}",
        func.mangled_name.as_ref(),
        derived_record.rs_name.as_ref()
    )
}

pub fn generate_function_thunk_impl(
    db: &dyn BindingsGenerator,
    func: &Func,
) -> Result<TokenStream> {
    if can_skip_cc_thunk(db, func) {
        return Ok(quote! {});
    }
    let ir = db.ir();
    let thunk_ident = thunk_ident(func);
    let implementation_function = match &func.name {
        UnqualifiedIdentifier::Operator(op) => {
            let name = syn::parse_str::<TokenStream>(&op.name)?;
            quote! { operator #name }
        }
        UnqualifiedIdentifier::Identifier(id) => {
            let fn_ident = crate::format_cc_ident(&id.identifier);
            match func.member_func_metadata.as_ref() {
                Some(meta) => {
                    if meta.instance_method_metadata.is_some() {
                        quote! { #fn_ident }
                    } else {
                        let record: &Rc<Record> = ir.find_decl(meta.record_id)?;
                        let record_ident = crate::format_cc_ident(record.cc_name.as_ref());
                        let namespace_qualifier = ir.namespace_qualifier(record).format_for_cc()?;
                        quote! { #namespace_qualifier #record_ident :: #fn_ident }
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
            quote! { crubit::construct_at }
        }
        UnqualifiedIdentifier::Destructor => quote! {std::destroy_at},
    };

    let mut param_idents =
        func.params.iter().map(|p| crate::format_cc_ident(&p.identifier.identifier)).collect_vec();

    let mut conversion_externs = quote! {};
    let mut conversion_stmts = quote! {};
    let convert_ident = |ident: &TokenStream| -> TokenStream {
        let ident = format_ident!("__converted_{}", ident.to_string());
        quote! { #ident }
    };
    let mut param_types = func
        .params
        .iter()
        .map(|p| {
            let cpp_type = crate::format_cpp_type(&p.type_.cpp_type, &ir)?;
            let arg_type = db.rs_type_kind(p.type_.rs_type.clone())?;
            if arg_type.is_bridge_type() {
                match &arg_type {
                    RsTypeKind::BridgeType { rust_to_cpp_converter, .. } => {
                        let convert_function = crate::format_cc_ident(rust_to_cpp_converter);
                        let ident = crate::format_cc_ident(&p.identifier.identifier);
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
                    }
                    _ => {
                        bail!("Invalid bridge type: {:?}", arg_type);
                    }
                }
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
            let mut ident = crate::format_cc_ident(&p.identifier.identifier);
            if db.rs_type_kind(p.type_.rs_type.clone())?.is_bridge_type() {
                let formatted_ident = convert_ident(&ident);
                ident = quote! { &(#formatted_ident.val) };
            }
            match p.type_.cpp_type.name.as_deref() {
                Some("&") => Ok(quote! { * #ident }),
                Some("&&") => Ok(quote! { std::move(* #ident) }),
                _ => {
                    let rs_type_kind = db.rs_type_kind(p.type_.rs_type.clone())?;
                    // non-Unpin types are wrapped by a pointer in the thunk.
                    if !rs_type_kind.is_c_abi_compatible_by_value() {
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
    let return_type_kind = db.rs_type_kind(func.return_type.rs_type.clone())?;
    let is_return_value_c_abi_compatible = return_type_kind.is_c_abi_compatible_by_value();

    let return_type_name = if !is_return_value_c_abi_compatible {
        param_idents.insert(0, crate::format_cc_ident("__return"));
        // In order to be modified, the return type can't be const.
        let mut cc_return_type = func.return_type.cpp_type.clone();
        cc_return_type.is_const = false;
        let return_type_name = crate::format_cpp_type(&cc_return_type, &ir)?;
        match &return_type_kind {
            RsTypeKind::BridgeType { cpp_to_rust_converter, .. } => {
                let convert_function = crate::format_cc_ident(cpp_to_rust_converter);
                conversion_externs.extend(quote! {
                    extern "C" void #convert_function(void* cpp_struct, void* rust_struct);
                });
                param_types.insert(0, quote! {void *});
            }
            _ => {
                param_types.insert(0, quote! {#return_type_name *});
            }
        };
        quote! {void}
    } else {
        crate::format_cpp_type(&func.return_type.cpp_type, &ir)?
    };

    let this_ref_qualification =
        func.member_func_metadata.as_ref().and_then(|meta| match &func.name {
            UnqualifiedIdentifier::Constructor | UnqualifiedIdentifier::Destructor => None,
            UnqualifiedIdentifier::Identifier(_) | UnqualifiedIdentifier::Operator(_) => meta
                .instance_method_metadata
                .as_ref()
                .map(|instance_method| instance_method.reference),
        });
    let (implementation_function, arg_expressions) =
        if let Some(this_ref_qualification) = this_ref_qualification {
            let this_param = func
                .params
                .first()
                .ok_or_else(|| anyhow!("Instance methods must have `__this` param."))?;

            let this_arg = crate::format_cc_ident(&this_param.identifier.identifier);
            let this_dot = if this_ref_qualification == ir::ReferenceQualification::RValue {
                quote! {std::move(*#this_arg).}
            } else {
                quote! {#this_arg->}
            };
            (
                quote! { #this_dot #implementation_function},
                arg_expressions.iter().skip(1).cloned().collect_vec(),
            )
        } else {
            (implementation_function, arg_expressions)
        };

    let return_expr = quote! {#implementation_function( #( #arg_expressions ),* )};
    let return_stmt = if !is_return_value_c_abi_compatible {
        let out_param = &param_idents[0];
        match &return_type_kind {
            RsTypeKind::BridgeType { cpp_to_rust_converter, .. } => {
                let convert_function = crate::format_cc_ident(cpp_to_rust_converter);
                quote! {
                    auto __original_cpp_struct = #return_expr;
                    #convert_function(&__original_cpp_struct, #out_param)
                }
            }
            _ => {
                // Explicitly use placement `new` so that we get guaranteed copy elision in
                // C++17.
                quote! {new(#out_param) auto(#return_expr)}
            }
        }
    } else {
        match func.return_type.cpp_type.name.as_deref() {
            Some("void") => return_expr,
            Some("&") => quote! { return & #return_expr },
            Some("&&") => {
                // The code below replicates bits of `format_cpp_type`, but formats an rvalue
                // reference (which `format_cpp_type` would format as a pointer).
                // `const_fragment` from `format_cpp_type` is ignored - it is not applicable for
                // references.
                let ty = &func.return_type.cpp_type;
                if ty.type_args.len() != 1 {
                    bail!("Invalid reference type (need exactly 1 type argument): {:?}", ty);
                }
                let nested_type = crate::format_cpp_type(&ty.type_args[0], &ir)?;
                quote! {
                    #nested_type && lvalue = #return_expr;
                    return &lvalue
                }
            }
            _ => quote! { return #return_expr },
        }
    };

    Ok(quote! {
        #conversion_externs

        extern "C" #return_type_name #thunk_ident( #( #param_types #param_idents ),* ) {
            #conversion_stmts
            #return_stmt;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use crate::BindingsTokens;
    use arc_anyhow::Result;
    use googletest::prelude::*;
    use token_stream_matchers::assert_cc_matches;

    #[gtest]
    fn test_c_abi_compatible_type_by_value_with_move() -> Result<()> {
        let ir = ir_from_cc(
            r#"
                typedef int MyTypedefDecl;

                inline void f(MyTypedefDecl a, void* b, int c) {}
            "#,
        )?;
        let BindingsTokens { rs_api_impl, .. } = generate_bindings_tokens(ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z1fiPvi(MyTypedefDecl a, void* b, int c) {
                    f(std::move(a), b, c);
                }
            }
        );
        Ok(())
    }
}
