// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use crubit_abi_type::{CrubitAbiType, CrubitAbiTypeToCppExprTokens, CrubitAbiTypeToCppTokens};
use database::db::BindingsGenerator;
use database::rs_snippet::{DynCallable, FnKind, RsTypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generates the `CrubitAbiType` for DynCallables.
pub fn dyn_callable_crubit_abi_type(
    db: &dyn BindingsGenerator,
    dyn_callable: &DynCallable,
) -> Result<CrubitAbiType> {
    let dyn_fn_spelling = dyn_callable.dyn_fn_spelling(db);

    let rust_type_tokens = quote! {
        ::dyn_callable_rs::DynCallableAbi<#dyn_fn_spelling>
    };

    let rust_expr_tokens = {
        let rust_return_type_fragment = dyn_callable.rust_return_type_fragment(db);
        let param_type_tokens =
            dyn_callable.param_types.iter().map(|param_ty| param_ty.to_token_stream(db));

        // DynCallableAbi's constructor takes a "default" value to use in the case that
        // C++ gives us a value in the moved-from state. The closure we pass in is a ZST
        // (it has no state), so boxing it is a no-op. This gives us reasonable
        // behavior: if C++ passes a Rust closure back to Rust, but it's in the
        // moved-from state and we call it, we'll get a moved-from panic.
        quote! {
            ::dyn_callable_rs::DynCallableAbi::<#dyn_fn_spelling>::new(
                ::alloc::boxed::Box::new(|#(_: #param_type_tokens),*| #rust_return_type_fragment {
                    ::dyn_callable_rs::moved_from_panic()
                })
            )
        }
    };

    let qualifier = match dyn_callable.fn_kind {
        FnKind::Fn => quote! { const },
        FnKind::FnMut => quote! {},
        FnKind::FnOnce => quote! { && },
    };

    let cpp_return_type = cpp_type_name::format_cpp_type(&dyn_callable.return_type, db.ir())?;
    let cpp_param_types = dyn_callable
        .param_types
        .iter()
        .map(|param_ty| cpp_type_name::format_cpp_type(param_ty, db.ir()))
        .collect::<Result<Vec<_>>>()?;
    let cpp_type_tokens = quote! {
        ::rs_std::internal_dyn_callable::DynCallableAbi<
            #cpp_return_type(#(#cpp_param_types),*) #qualifier
        >
    };

    let cpp_expr_tokens = {
        let invoker_function_pointer = generate_invoker_function_pointer(
            db,
            dyn_callable,
            &cpp_param_types,
            &cpp_return_type,
        )?;

        // Construct the DynCallableAbi value with a pointer to the invoker function.
        quote! {
            #cpp_type_tokens(#invoker_function_pointer)
        }
    };

    Ok(CrubitAbiType::DynCallable {
        rust_type_tokens,
        rust_expr_tokens,
        cpp_type_tokens,
        cpp_expr_tokens,
    })
}

/// Generates the function pointer object that DynCallable will use in operator()().
///
/// This will often produce tokens of the form:
///
/// ```cpp
/// [](
///     ::rs_std::internal_dyn_callable::TypeErasedState* state
///     P...,
/// ) -> R {
///     // impl
/// }
/// ```
///
/// This lambda accepts and returns idiomatic C++ values, and is responsible for preparing arguments
/// to be passed to Rust via the C ABI, invoking a statically-known forward-declared thunk which
/// is linked to a Rust definition, and then converting the return value back into an idiomatic C++
/// value. In the case that all inputs and outputs are C-compatible by value, this lambda is simply
/// a pointer to the thunk.
fn generate_invoker_function_pointer(
    db: &dyn BindingsGenerator,
    dyn_callable: &DynCallable,
    cpp_param_types: &[TokenStream],
    cpp_return_type: &TokenStream,
) -> Result<TokenStream> {
    let thunk_ident = &dyn_callable.thunk_ident;
    if dyn_callable.is_c_abi_compatible() {
        // The input and output types are all C-compatible, no wrapper lambda is
        // needed.
        return Ok(quote! { &#thunk_ident });
    }

    let param_idents =
        (0..dyn_callable.param_types.len()).map(|i| format_ident!("param_{i}")).collect::<Vec<_>>();

    let mut arg_transforms = quote! {};
    let mut arg_exprs = Vec::with_capacity(param_idents.len());
    for (i, param_ty) in dyn_callable.param_types.iter().enumerate() {
        let param_ident = &param_idents[i];

        if param_ty.is_c_abi_compatible_by_value() {
            arg_exprs.push(quote! { #param_ident });
        } else if param_ty.is_crubit_abi_bridge_type() {
            let crubit_abi_type = db.crubit_abi_type(param_ty.clone())?;
            let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
            let crubit_abi_type_expr_tokens = CrubitAbiTypeToCppExprTokens(&crubit_abi_type);
            let arg_ident = format_ident!("bridge_param_{i}");
            arg_transforms.extend(quote! {
                unsigned char #arg_ident[#crubit_abi_type_tokens::kSize];
                ::crubit::internal::Encode(#crubit_abi_type_expr_tokens, #arg_ident, #param_ident);
            });
            arg_exprs.push(quote! { #arg_ident });
        } else {
            let arg_ident = format_ident!("stack_param_{i}");
            let cpp_param_type = &cpp_param_types[i];
            arg_transforms.extend(quote! {
                ::crubit::Slot<#cpp_param_type> #arg_ident(std::move(#param_ident));
            });
            arg_exprs.push(quote! { #arg_ident.Get() });
        }
    }

    let out_param_arg = if dyn_callable.return_type.is_void()
        || dyn_callable.return_type.is_c_abi_compatible_by_value()
    {
        None
    } else if dyn_callable.return_type.is_crubit_abi_bridge_type() {
        let crubit_abi_type = db.crubit_abi_type(RsTypeKind::clone(&dyn_callable.return_type))?;
        let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
        arg_transforms.extend(quote! {
            unsigned char out[#crubit_abi_type_tokens::kSize];
        });
        Some(quote! { , out })
    } else {
        arg_transforms.extend(quote! {
            ::crubit::Slot<#cpp_return_type> out;
        });
        Some(quote! { , out.Get() })
    };

    let mut invoke_ffi_and_transform_to_cpp = quote! {
        #thunk_ident(state #(, #arg_exprs)* #out_param_arg);
    };

    if dyn_callable.return_type.is_void() {
        // No need to return anything.
    } else if dyn_callable.return_type.is_c_abi_compatible_by_value() {
        // Return the result.
        invoke_ffi_and_transform_to_cpp = quote! {
            return #invoke_ffi_and_transform_to_cpp
        };
    } else if dyn_callable.return_type.is_crubit_abi_bridge_type() {
        let crubit_abi_type = db.crubit_abi_type(RsTypeKind::clone(&dyn_callable.return_type))?;
        let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
        let crubit_abi_type_expr_tokens = CrubitAbiTypeToCppExprTokens(&crubit_abi_type);
        invoke_ffi_and_transform_to_cpp.extend(quote! {
            // Because our bridge buffer is named `out`
            return ::crubit::internal::Decode<#crubit_abi_type_tokens>(#crubit_abi_type_expr_tokens, out);
        });
    } else {
        // The caller has finished initializing the return value, so we just
        // need to take ownership of it.
        invoke_ffi_and_transform_to_cpp.extend(quote! {
            return std::move(out).AssumeInitAndTakeValue();
        });
    }
    Ok(quote! {
        [](
            ::rs_std::internal_dyn_callable::TypeErasedState* state
            #(
                , #cpp_param_types #param_idents
            )*
        ) -> #cpp_return_type {
            #arg_transforms

            #invoke_ffi_and_transform_to_cpp
        }
    })
}
