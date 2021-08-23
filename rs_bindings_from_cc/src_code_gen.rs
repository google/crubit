// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::Result;
use ffi_types::*;
use ir::*;
use itertools::Itertools;
use quote::format_ident;
use quote::quote;
use std::iter::Iterator;
use std::panic::catch_unwind;
use std::process;
use syn::*;

/// FFI equivalent of `Bindings`.
#[repr(C)]
pub struct FfiBindings {
    rs_api: FfiU8SliceBox,
    rs_api_impl: FfiU8SliceBox,
}

/// Deserializes IR from `json` and generates bindings source code.
///
/// This function panics on error.
///
/// Ownership:
///    * function doesn't take ownership of (in other words it borrows) the param `json`
///    * function passes ownership of the returned value to the caller
///
/// Safety:
///    * function expects that param `json` is a FfiU8Slice for a valid array of bytes with the
///      given size.
///    * function expects that param `json` doesn't change during the call.
#[no_mangle]
pub unsafe extern "C" fn GenerateBindingsImpl(json: FfiU8Slice) -> FfiBindings {
    catch_unwind(|| {
        // It is ok to abort here.
        let Bindings { rs_api, rs_api_impl } = generate_bindings(json.as_slice()).unwrap();

        FfiBindings {
            rs_api: FfiU8SliceBox::from_boxed_slice(rs_api.into_bytes().into_boxed_slice()),
            rs_api_impl: FfiU8SliceBox::from_boxed_slice(
                rs_api_impl.into_bytes().into_boxed_slice(),
            ),
        }
    })
    .unwrap_or_else(|_| process::abort())
}

/// Source code for generated bindings.
struct Bindings {
    // Rust source code.
    rs_api: String,
    // C++ source code.
    rs_api_impl: String,
}

fn generate_bindings(json: &[u8]) -> Result<Bindings> {
    let ir = deserialize_ir(json)?;
    let rs_api = generate_rs_api(&ir)?;
    let rs_api_impl = generate_rs_api_impl(&ir)?;
    Ok(Bindings { rs_api, rs_api_impl })
}

fn generate_rs_api(ir: &IR) -> Result<String> {
    let mut thunks = vec![];
    let mut api_funcs = vec![];
    for func in &ir.functions {
        let mangled_name = &func.mangled_name;
        let ident = make_ident(&func.identifier.identifier);
        let thunk_ident = format_ident!("__rust_thunk__{}", &func.identifier.identifier);
        // TODO(hlopko): do not emit `-> ()` when return type is void, it's implicit.
        let return_type_name = make_ident(&func.return_type.rs_name);

        let param_idents =
            func.params.iter().map(|p| make_ident(&p.identifier.identifier)).collect_vec();

        let param_types = func.params.iter().map(|p| make_ident(&p.type_.rs_name)).collect_vec();

        api_funcs.push(quote! {
            #[inline(always)]
            pub fn #ident( #( #param_idents: #param_types ),* ) -> #return_type_name {
                unsafe { crate::detail::#thunk_ident( #( #param_idents ),* ) }
            }
        });

        thunks.push(quote! {
            #[link_name = #mangled_name]
            pub(crate) fn #thunk_ident( #( #param_idents: #param_types ),* ) -> #return_type_name ;
        });
    }

    let result = quote! {
        #( #api_funcs )*

        mod detail {
            extern "C" {
                #( #thunks )*
            }
        }
    };

    Ok(result.to_string())
}

fn generate_rs_api_impl(_ir: &IR) -> Result<String> {
    // TODO(hlopko): Generate C++ code when needed.
    Ok("// No bindings implementation code was needed.".to_string())
}

fn make_ident(ident: &str) -> Ident {
    format_ident!("{}", ident)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::Result;
    use quote::quote;

    #[test]
    fn test_simple_function() -> Result<()> {
        let ir = IR {
            used_headers: vec![],
            functions: vec![Func {
                identifier: Identifier { identifier: "add".to_string() },
                mangled_name: "_Z3Addii".to_string(),
                return_type: IRType { rs_name: "i32".to_string() },
                params: vec![
                    FuncParam {
                        identifier: Identifier { identifier: "a".to_string() },
                        type_: IRType { rs_name: "i32".to_string() },
                    },
                    FuncParam {
                        identifier: Identifier { identifier: "b".to_string() },
                        type_: IRType { rs_name: "i32".to_string() },
                    },
                ],
            }],
        };
        assert_eq!(
            generate_rs_api(&ir)?,
            quote! {
                #[inline(always)]
                pub fn add(a: i32, b: i32) -> i32 {
                    unsafe { crate::detail::__rust_thunk__add(a, b) }
                }

                mod detail {
                    extern "C" {
                        #[link_name = "_Z3Addii"]
                        pub(crate) fn __rust_thunk__add(a: i32, b: i32) -> i32;
                    } // extern
                } // mod detail
            }
            .to_string()
        );
        assert_eq!(generate_rs_api_impl(&ir)?, "// No bindings implementation code was needed.");
        Ok(())
    }
}
