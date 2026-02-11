// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{bail, Result};
use crubit_abi_type::{CrubitAbiType, CrubitAbiTypeToCppExprTokens, CrubitAbiTypeToCppTokens};
use database::db::BindingsGenerator;
use database::rs_snippet::{BackingType, Callable, FnTrait, PassingConvention, RsTypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generates the `CrubitAbiType` for callables.
pub fn dyn_callable_crubit_abi_type(
    db: &dyn BindingsGenerator,
    callable: &Callable,
) -> Result<CrubitAbiType> {
    let dyn_fn_spelling = callable.dyn_fn_spelling(db);

    let rust_type_tokens = match callable.backing_type {
        BackingType::DynCallable => quote! {
            ::dyn_callable_rs::DynCallableAbi<#dyn_fn_spelling>
        },
        BackingType::AnyInvocable => quote! {
            ::any_invocable::AnyInvocableAbi<#dyn_fn_spelling>
        },
    };

    let on_empty_tokens = {
        let rust_return_type_fragment = callable.rust_return_type_fragment(db);
        let param_type_tokens =
            callable.param_types.iter().map(|param_ty| param_ty.to_token_stream(db));

        quote! {
            ::alloc::boxed::Box::new(|#(_: #param_type_tokens),*| #rust_return_type_fragment {
                ::core::panic!("moved-from value")
            })
        }
    };

    let rust_expr_tokens = match callable.backing_type {
        BackingType::DynCallable => quote! {
            ::dyn_callable_rs::DynCallableAbi::<#dyn_fn_spelling>::new(
                #on_empty_tokens,
            )
        },
        BackingType::AnyInvocable => {
            let make_cpp_invoker_tokens = generate_make_cpp_invoker_tokens()?;
            quote! {
                ::any_invocable::AnyInvocableAbi::<#dyn_fn_spelling>::new(
                    #on_empty_tokens,
                    #make_cpp_invoker_tokens,
                )
            }
        }
    };

    let qualifier = match callable.fn_trait {
        FnTrait::Fn => quote! { const },
        FnTrait::FnMut => quote! {},
        FnTrait::FnOnce => quote! { && },
    };

    let cpp_return_type = cpp_type_name::format_cpp_type(&callable.return_type, db.ir())?;
    let cpp_param_types = callable
        .param_types
        .iter()
        .map(|param_ty| cpp_type_name::format_cpp_type(param_ty, db.ir()))
        .collect::<Result<Vec<_>>>()?;
    let cpp_fn_sig = quote! {
        #cpp_return_type(#(#cpp_param_types),*) #qualifier
    };
    let cpp_type_tokens = match callable.backing_type {
        BackingType::DynCallable => quote! {
            ::rs_std::internal_dyn_callable::DynCallableAbi<#cpp_fn_sig>
        },
        BackingType::AnyInvocable => quote! {
            ::crubit::AnyInvocableAbi<#cpp_fn_sig>
        },
    };

    let cpp_expr_tokens = {
        let invoker_function_pointer =
            generate_invoker_function_pointer(db, callable, &cpp_param_types, &cpp_return_type)?;
        let manager_ident = &callable.manager_ident;

        // Construct the DynCallableAbi value with a pointer to the invoker function.
        quote! {
            #cpp_type_tokens(
                [](
                    absl::internal_any_invocable::FunctionToCall operation,
                    absl::internal_any_invocable::TypeErasedState* from,
                    absl::internal_any_invocable::TypeErasedState* to
                ) noexcept {
                    #manager_ident(operation, from, to);
                },
                #invoker_function_pointer
            )
        }
    };

    Ok(CrubitAbiType::Callable {
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
    callable: &Callable,
    cpp_param_types: &[TokenStream],
    cpp_return_type: &TokenStream,
) -> Result<TokenStream> {
    let invoker_ident = &callable.invoker_ident;
    // Even if the callable has all C ABI compatible inputs and outputs, we cannot pass the function
    // pointer directly because cfi doesn't recognize Rust function pointers as safe.
    let param_idents =
        (0..callable.param_types.len()).map(|i| format_ident!("param_{i}")).collect::<Vec<_>>();

    let mut arg_transforms = quote! {};
    let mut arg_exprs = Vec::with_capacity(param_idents.len());
    for (i, param_ty) in callable.param_types.iter().enumerate() {
        let param_ident = &param_idents[i];

        match param_ty.passing_convention() {
            PassingConvention::AbiCompatible => {
                arg_exprs.push(quote! { #param_ident });
            }
            PassingConvention::LayoutCompatible => {
                let arg_ident = format_ident!("stack_param_{i}");
                let cpp_param_type = &cpp_param_types[i];
                arg_transforms.extend(quote! {
                    ::crubit::Slot<#cpp_param_type> #arg_ident(std::move(#param_ident));
                });
                arg_exprs.push(quote! { #arg_ident.Get() });
            }
            PassingConvention::ComposablyBridged => {
                let crubit_abi_type = db.crubit_abi_type(param_ty.clone())?;
                let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
                let crubit_abi_type_expr_tokens = CrubitAbiTypeToCppExprTokens(&crubit_abi_type);
                let arg_ident = format_ident!("bridge_param_{i}");
                arg_transforms.extend(quote! {
                unsigned char #arg_ident[#crubit_abi_type_tokens::kSize];
                ::crubit::internal::Encode(#crubit_abi_type_expr_tokens, #arg_ident, #param_ident);
            });
                arg_exprs.push(quote! { #arg_ident });
            }
            PassingConvention::Ctor => {
                bail!("Ctor not supported");
            }
            PassingConvention::OwnedPtr => {
                bail!("OwnedPtr not supported");
            }
            PassingConvention::Void => unreachable!("parameter types cannot be void"),
        }
    }

    let out_param_arg = match callable.return_type.passing_convention() {
        PassingConvention::AbiCompatible | PassingConvention::Void => None,
        PassingConvention::LayoutCompatible => {
            arg_transforms.extend(quote! {
                ::crubit::Slot<#cpp_return_type> out;
            });
            Some(quote! { , out.Get() })
        }
        PassingConvention::ComposablyBridged => {
            let crubit_abi_type = db.crubit_abi_type(RsTypeKind::clone(&callable.return_type))?;
            let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
            arg_transforms.extend(quote! {
                unsigned char out[#crubit_abi_type_tokens::kSize];
            });
            Some(quote! { , out })
        }
        PassingConvention::Ctor => {
            bail!("Ctor not supported");
        }
        PassingConvention::OwnedPtr => {
            bail!("OwnedPtr not supported");
        }
    };

    let mut invoke_ffi_and_transform_to_cpp = quote! {
        #invoker_ident(state #(, #arg_exprs)* #out_param_arg);
    };

    match callable.return_type.passing_convention() {
        PassingConvention::AbiCompatible => {
            // Return the result.
            invoke_ffi_and_transform_to_cpp = quote! {
                return #invoke_ffi_and_transform_to_cpp
            };
        }
        PassingConvention::LayoutCompatible => {
            // The caller has finished initializing the return value, so we just
            // need to take ownership of it.
            invoke_ffi_and_transform_to_cpp.extend(quote! {
                return std::move(out).AssumeInitAndTakeValue();
            });
        }
        PassingConvention::ComposablyBridged => {
            let crubit_abi_type = db.crubit_abi_type(RsTypeKind::clone(&callable.return_type))?;
            let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
            let crubit_abi_type_expr_tokens = CrubitAbiTypeToCppExprTokens(&crubit_abi_type);
            invoke_ffi_and_transform_to_cpp.extend(quote! {
            // Because our bridge buffer is named `out`
            return ::crubit::internal::Decode<#crubit_abi_type_tokens>(#crubit_abi_type_expr_tokens, out);
        });
        }
        PassingConvention::Ctor => {
            bail!("Ctor not supported");
        }
        PassingConvention::OwnedPtr => {
            bail!("OwnedPtr not supported");
        }
        PassingConvention::Void => {
            // No need to return anything.
        }
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

/// Generates the `make_cpp_invoker` function for AnyInvocable.
fn generate_make_cpp_invoker_tokens() -> Result<TokenStream> {
    bail!("AnyInvocable is not yet supported")
}
