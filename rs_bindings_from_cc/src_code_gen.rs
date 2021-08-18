// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::Result;
use ir::*;
use itertools::Itertools;
use quote::format_ident;
use quote::quote;
use std::boxed::Box;
use std::iter::Iterator;
use std::panic::catch_unwind;
use std::process;
use std::slice;
use syn::*;

#[repr(C)]
pub struct FfiU8Slice {
    ptr: *const u8,
    size: usize,
}

impl FfiU8Slice {
    /// Borrows data pointed to by this `FfiU8Slice` as a slice.
    fn as_slice(&self) -> &[u8] {
        // Safety:
        // Instances of `FfiU8Slice` are only created by FFI functions, which are unsafe themselves
        // so it's their responsibility to maintain safety.
        unsafe { slice::from_raw_parts(self.ptr, self.size) }
    }
}

#[repr(C)]
pub struct FfiU8SliceBox {
    ptr: *const u8,
    size: usize,
}

impl FfiU8SliceBox {
    fn from_boxed_slice(bytes: Box<[u8]>) -> FfiU8SliceBox {
        let slice = Box::leak(bytes);
        FfiU8SliceBox { ptr: slice.as_mut_ptr(), size: slice.len() }
    }

    /// Consumes self and returns boxed slice.
    fn into_boxed_slice(self) -> Box<[u8]> {
        // Safety:
        // Instances of `FfiU8SliceBox` are either created by `from_boxed_slice`, which is safe,
        // or by FFI functions, which are unsafe themselves so it's their responsibility to maintain
        // safety.
        unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr as *mut u8, self.size)) }
    }
}

/// Deserializes IR from `json` and generates Rust bindings source code.
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
pub unsafe extern "C" fn GenerateRustApiImpl(json: FfiU8Slice) -> FfiU8SliceBox {
    catch_unwind(|| {
        let result = gen_rs_api(json.as_slice());
        // it is ok to abort with the error message here.
        FfiU8SliceBox::from_boxed_slice(result.unwrap().into_bytes().into_boxed_slice())
    })
    .unwrap_or_else(|_| process::abort())
}

/// Frees C-string allocated by Rust.
#[no_mangle]
pub unsafe extern "C" fn FreeFfiU8SliceBox(sb: FfiU8SliceBox) {
    catch_unwind(|| {
        let _ = sb.into_boxed_slice();
    })
    .unwrap_or_else(|_| process::abort())
}

fn gen_rs_api(json: &[u8]) -> Result<String> {
    let ir = deserialize_ir(json)?;
    Ok(gen_src_code(ir)?)
}

fn gen_src_code(ir: IR) -> Result<String> {
    let mut thunks = vec![];
    let mut api_funcs = vec![];
    for func in ir.functions {
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

fn make_ident(ident: &str) -> Ident {
    format_ident!("{}", ident)
}

#[cfg(test)]
mod tests {
    use super::gen_src_code;
    use super::Result;
    use ir::*;
    use quote::quote;

    #[test]
    fn test_gen_src_code() -> Result<()> {
        let ir = IR {
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
        let result = gen_src_code(ir)?;
        assert_eq!(
            result,
            quote! {#[inline(always)]
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
        Ok(())
    }
}
