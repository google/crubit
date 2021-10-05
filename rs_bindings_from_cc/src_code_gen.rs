// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::bail;
use anyhow::{anyhow, Result};
use ffi_types::*;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::format_ident;
use quote::quote;
use std::io::Write;
use std::iter::Iterator;
use std::panic::catch_unwind;
use std::process;
use std::process::{Command, Stdio};

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

/// If we know the original C++ function is codegenned and already compatible with `extern "C"`
/// calling convention we skip creating/calling the C++ thunk since we can call the original C++
/// directly.
fn can_skip_cc_thunk(func: &Func) -> bool {
    // Inline functions may not be codegenned in the C++ library since Clang doesn't know if Rust
    // calls the function or not. Therefore in order to make inline functions callable from Rust we
    // need to generate a C++ file that defines a thunk that delegates to the original inline
    // function. When compiled, Clang will emit code for this thunk and Rust code will call the
    // thunk when the user wants to call the original inline function.
    //
    // This is not great runtime-performance-wise in regular builds (inline function will not be
    // inlined, there will always be a function call), but it is correct. ThinLTO builds will be
    // able to see through the thunk and inline code across the language boundary. For non-ThinLTO
    // builds we plan to implement <internal link> which removes the runtime performance overhead.
    !func.is_inline
}

/// Generates Rust source code for a given `Func`.
///
/// Returns the generated function and the thunk as a tuple.
fn generate_func(func: &Func) -> Result<(TokenStream, TokenStream)> {
    let mangled_name = &func.mangled_name;
    let ident = make_ident(&func.identifier.identifier);
    let thunk_ident = format_ident!("__rust_thunk__{}", &func.identifier.identifier);
    // TODO(hlopko): do not emit `-> ()` when return type is void, it's implicit.
    let return_type_name = format_rs_type(&func.return_type.rs_type)?;

    let param_idents =
        func.params.iter().map(|p| make_ident(&p.identifier.identifier)).collect_vec();

    let param_types =
        func.params.iter().map(|p| format_rs_type(&p.type_.rs_type)).collect::<Result<Vec<_>>>()?;

    let api_func = quote! {
        #[inline(always)]
        pub fn #ident( #( #param_idents: #param_types ),* ) -> #return_type_name {
            unsafe { crate::detail::#thunk_ident( #( #param_idents ),* ) }
        }
    };

    let thunk_attr = if can_skip_cc_thunk(func) {
        quote! {#[link_name = #mangled_name]}
    } else {
        quote! {}
    };

    let thunk = quote! {
        #thunk_attr
        pub(crate) fn #thunk_ident( #( #param_idents: #param_types ),* ) -> #return_type_name ;
    };

    Ok((api_func, thunk))
}

/// Generates Rust source code for a given `Record`.
fn generate_record(record: &Record) -> Result<TokenStream> {
    let ident = make_ident(&record.identifier.identifier);
    let doc_comment = match &record.doc_comment {
        Some(text) => {
            let doc = format!(" {}", text.replace("\n", "\n "));
            quote! {#[doc=#doc]}
        }
        None => quote! {},
    };
    let field_idents =
        record.fields.iter().map(|f| make_ident(&f.identifier.identifier)).collect_vec();
    let field_types = record
        .fields
        .iter()
        .map(|f| format_rs_type(&f.type_.rs_type))
        .collect::<Result<Vec<_>>>()?;
    let field_accesses = record
        .fields
        .iter()
        .map(|f| {
            if f.access == AccessSpecifier::Public {
                quote! { pub }
            } else {
                quote! {}
            }
        })
        .collect_vec();
    let size = record.size;
    let alignment = record.alignment;
    let field_assertions =
        record.fields.iter().zip(field_idents.iter()).map(|(field, field_ident)| {
            let offset = field.offset;
            quote! {
                // The IR contains the offset in bits, while offset_of!()
                // returns the offset in bytes, so we need to convert.
                const_assert_eq!(offset_of!(#ident, #field_ident) * 8, #offset);
            }
        });
    Ok(quote! {
        #doc_comment
        #[repr(C)]
        pub struct #ident {
            #( #field_accesses #field_idents: #field_types, )*
        }

        const_assert_eq!(std::mem::size_of::<#ident>(), #size);
        const_assert_eq!(std::mem::align_of::<#ident>(), #alignment);
        #( #field_assertions )*
    })
}

fn generate_rs_api(ir: &IR) -> Result<String> {
    let mut items = vec![];
    let mut thunks = vec![];

    let mut generate_imports = false;

    for item in &ir.items {
        match item {
            Item::Func(func) => {
                let (api_func, thunk) = generate_func(func)?;
                items.push(api_func);
                thunks.push(thunk);
            }
            Item::Record(record) => {
                items.push(generate_record(record)?);
                generate_imports = true;
            }
        }
    }

    let mod_detail = if thunks.is_empty() {
        quote! {}
    } else {
        quote! {
            mod detail {
                extern "C" {
                    #( #thunks )*
                }
            }
        }
    };

    let imports = if generate_imports {
        // TODO(mboehme): For the time being, we're using unstable features to
        // be able to use offset_of!() in static assertions. This is fine for a
        // prototype, but longer-term we want to either get those features
        // stabilized or find an alternative. For more details, see
        // b/200120034#comment15
        quote! {
            #![feature(const_ptr_offset_from, const_maybe_uninit_as_ptr, const_raw_ptr_deref)]
            use memoffset_unstable_const::offset_of;
            use static_assertions::const_assert_eq;
        }
    } else {
        quote! {}
    };

    let result = quote! {
        #imports

        #( #items )*

        #mod_detail
    };

    Ok(rustfmt(result.to_string())?)
}

fn rustfmt(input: String) -> Result<String> {
    // TODO(forster): This should use rustfmt as a library as soon as b/200503084 is fixed.

    let rustfmt = "third_party/unsupported_toolchains/rust/toolchains/nightly/bin/rustfmt";

    let mut child = Command::new(rustfmt)
        // TODO(forster): Add a way to specify this as a command line parameter.
        .args(&[
            "--config-path=external/rustfmt/rustfmt.toml",
            "--config=normalize_doc_attributes=true",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to spawn rustfmt at '{}'", rustfmt));

    let mut stdin = child.stdin.take().expect("Failed to open rustfmt stdin");
    std::thread::spawn(move || {
        stdin.write_all(input.as_bytes()).expect("Failed to write to rustfmt stdin");
    });
    let output = child.wait_with_output().expect("Failed to read rustfmt stdout");

    if !output.status.success() {
        // The rustfmt error message has already been printed to stderr.
        bail!("Unable to format output with rustfmt");
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn make_ident(ident: &str) -> Ident {
    format_ident!("{}", ident)
}

fn format_rs_type(ty: &ir::RsType) -> Result<TokenStream> {
    let ptr_fragment = match ty.name.as_str() {
        "*mut" => Some(quote! {*mut}),
        "*const" => Some(quote! {*const}),
        _ => None,
    };
    match ptr_fragment {
        Some(ptr_fragment) => {
            if ty.type_params.len() != 1 {
                return Err(anyhow!(
                    "Invalid pointer type (need exactly 1 type parameter): {:?}",
                    ty
                ));
            }
            let nested_type = format_rs_type(&ty.type_params[0])?;
            Ok(quote! {#ptr_fragment #nested_type})
        }
        None => {
            if ty.type_params.len() > 0 {
                return Err(anyhow!("Type not yet supported: {:?}", ty));
            }
            let ident = make_ident(&ty.name);
            Ok(quote! {#ident})
        }
    }
}

fn format_cc_type(ty: &ir::CcType) -> Result<TokenStream> {
    let const_fragment = if ty.is_const {
        quote! {const}
    } else {
        quote! {}
    };
    match ty.name.as_str() {
        "*" => {
            if ty.type_params.len() != 1 {
                return Err(anyhow!(
                    "Invalid pointer type (need exactly 1 type parameter): {:?}",
                    ty
                ));
            }
            assert_eq!(ty.type_params.len(), 1);
            let nested_type = format_cc_type(&ty.type_params[0])?;
            Ok(quote! {#nested_type * #const_fragment})
        }
        ident => {
            if ty.type_params.len() > 0 {
                return Err(anyhow!("Type not yet supported: {:?}", ty));
            }
            let ident = make_ident(ident);
            Ok(quote! {#ident #const_fragment})
        }
    }
}

fn cc_struct_layout_assertion(record: &Record) -> TokenStream {
    let record_ident = make_ident(&record.identifier.identifier);
    let size = Literal::usize_unsuffixed(record.size);
    let alignment = Literal::usize_unsuffixed(record.alignment);
    let field_assertions = record.fields.iter().map(|field| {
        let field_ident = make_ident(&field.identifier.identifier);
        let offset = Literal::usize_unsuffixed(field.offset);
        // The IR contains the offset in bits, while C++'s offsetof()
        // returns the offset in bytes, so we need to convert.
        quote! {
            static_assert(offsetof(#record_ident, #field_ident) * 8 == #offset);
        }
    });
    quote! {
        static_assert(sizeof(#record_ident) == #size);
        static_assert(alignof(#record_ident) == #alignment);
        #( #field_assertions )*
    }
}

fn generate_rs_api_impl(ir: &IR) -> Result<String> {
    // This function uses quote! to generate C++ source code out of convenience. This is a bold idea
    // so we have to continously evaluate if it still makes sense or the cost of working around
    // differences in Rust and C++ tokens is greather than the value added.
    //
    // See rs_bindings_from_cc/token_stream_printer.rs for a list
    // of supported placeholders.
    let mut thunks = vec![];
    for func in ir.functions() {
        if can_skip_cc_thunk(&func) {
            continue;
        }

        let thunk_ident = format_ident!("__rust_thunk__{}", &func.identifier.identifier);
        let ident = make_ident(&func.identifier.identifier);
        let return_type_name = format_cc_type(&func.return_type.cc_type)?;

        let param_idents =
            func.params.iter().map(|p| make_ident(&p.identifier.identifier)).collect_vec();

        let param_types = func
            .params
            .iter()
            .map(|p| format_cc_type(&p.type_.cc_type))
            .collect::<Result<Vec<_>>>()?;

        thunks.push(quote! {
            extern "C" #return_type_name #thunk_ident( #( #param_types #param_idents ),* ) {
                return #ident( #( #param_idents ),* );
            }
        });
    }

    let layout_assertions = ir.records().map(cc_struct_layout_assertion);

    let standard_headers =
        if ir.records().next().is_none() { vec![] } else { vec![make_ident("cstddef")] };

    // In order to generate C++ thunk in all the cases Clang needs to be able to access declarations
    // from public headers of the C++ library.
    let includes = ir.used_headers.iter().map(|i| &i.name);

    let result = quote! {
        #( __HASH_TOKEN__ include <#standard_headers> __NEWLINE__)*
        #( __HASH_TOKEN__ include #includes __NEWLINE__)*

        #( #thunks )*

        #( #layout_assertions )*
    };

    token_stream_printer::cc_tokens_to_string(result)
}

#[cfg(test)]
mod tests {
    use crate::rustfmt;

    use super::Result;
    use super::{generate_rs_api, generate_rs_api_impl};
    use anyhow::anyhow;
    use ir::*;
    use ir_testing::{
        ir_func, ir_id, ir_int, ir_int_param, ir_items, ir_public_trivial_special, ir_record,
    };
    use quote::quote;
    use token_stream_printer::cc_tokens_to_string;

    #[test]
    fn test_simple_function() -> Result<()> {
        let ir = ir_items(vec![Item::Func(Func {
            identifier: ir_id("add"),
            mangled_name: "_Z3Addii".to_string(),
            return_type: ir_int(),
            params: vec![ir_int_param("a"), ir_int_param("b")],
            is_inline: false,
        })]);
        assert_eq!(
            generate_rs_api(&ir)?,
            rustfmt(
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
            )?
        );
        assert_eq!(generate_rs_api_impl(&ir)?, "");
        Ok(())
    }

    #[test]
    fn test_inline_function() -> Result<()> {
        let ir = IR {
            used_headers: vec![
                HeaderName { name: "foo/bar.h".to_string() },
                HeaderName { name: "foo/baz.h".to_string() },
            ],
            items: vec![Item::Func(Func {
                identifier: Identifier { identifier: "add".to_string() },
                mangled_name: "_Z3Addii".to_string(),
                return_type: ir_int(),
                params: vec![ir_int_param("a"), ir_int_param("b")],
                is_inline: true,
            })],
        };

        assert_eq!(
            generate_rs_api(&ir)?,
            rustfmt(
                quote! {#[inline(always)]
                    pub fn add(a: i32, b: i32) -> i32 {
                        unsafe { crate::detail::__rust_thunk__add(a, b) }
                    }

                    mod detail {
                        extern "C" {
                            pub(crate) fn __rust_thunk__add(a: i32, b: i32) -> i32;
                        } // extern
                    } // mod detail
                }
                .to_string()
            )?
        );

        assert_eq!(
            generate_rs_api_impl(&ir)?,
            cc_tokens_to_string(quote! {
                __HASH_TOKEN__ include "foo/bar.h" __NEWLINE__
                __HASH_TOKEN__ include "foo/baz.h" __NEWLINE__

                extern "C" int __rust_thunk__add(int a, int b) {
                    return add(a, b);
                }
            })?
        );
        Ok(())
    }

    #[test]
    fn test_simple_struct() -> Result<()> {
        let ir = ir_items(vec![Item::Record(Record {
            identifier: ir_id("SomeStruct"),
            doc_comment: None,
            fields: vec![
                Field {
                    identifier: ir_id("public_int"),
                    type_: ir_int(),
                    access: AccessSpecifier::Public,
                    offset: 0,
                },
                Field {
                    identifier: ir_id("protected_int"),
                    type_: ir_int(),
                    access: AccessSpecifier::Protected,
                    offset: 32,
                },
                Field {
                    identifier: ir_id("private_int"),
                    type_: ir_int(),
                    access: AccessSpecifier::Private,
                    offset: 64,
                },
            ],
            size: 12,
            alignment: 4,
            copy_constructor: ir_public_trivial_special(),
            move_constructor: ir_public_trivial_special(),
            destructor: ir_public_trivial_special(),
            is_trivial_abi: true,
        })]);

        assert_eq!(
            generate_rs_api(&ir)?,
            rustfmt(
                quote! {
                    #![feature(const_ptr_offset_from, const_maybe_uninit_as_ptr, const_raw_ptr_deref)]
                    use memoffset_unstable_const::offset_of;
                    use static_assertions::const_assert_eq;

                    #[repr(C)]
                    pub struct SomeStruct {
                        pub public_int: i32,
                        protected_int: i32,
                        private_int: i32,
                    }

                    const_assert_eq!(std::mem::size_of::<SomeStruct>(), 12usize);
                    const_assert_eq!(std::mem::align_of::<SomeStruct>(), 4usize);
                    const_assert_eq!(offset_of!(SomeStruct, public_int) * 8, 0usize);
                    const_assert_eq!(offset_of!(SomeStruct, protected_int) * 8, 32usize);
                    const_assert_eq!(offset_of!(SomeStruct, private_int) * 8, 64usize);
                }
                .to_string()
            )?
        );
        assert_eq!(
            generate_rs_api_impl(&ir)?,
            cc_tokens_to_string(quote! {
                __HASH_TOKEN__ include <cstddef> __NEWLINE__
                static_assert(sizeof(SomeStruct) == 12);
                static_assert(alignof(SomeStruct) == 4);
                static_assert(offsetof(SomeStruct, public_int) * 8 == 0);
                static_assert(offsetof(SomeStruct, protected_int) * 8 == 32);
                static_assert(offsetof(SomeStruct, private_int) * 8 == 64);
            })?
        );
        Ok(())
    }

    #[test]
    fn test_ptr_func() -> Result<()> {
        let ir = ir_items(vec![Item::Func(Func {
            identifier: Identifier { identifier: "Deref".to_string() },
            mangled_name: "_Z5DerefPKPi".to_string(),
            return_type: MappedType {
                rs_type: RsType {
                    name: "*mut".to_string(),
                    type_params: vec![RsType { name: "i32".to_string(), type_params: vec![] }],
                },
                cc_type: CcType {
                    name: "*".to_string(),
                    is_const: false,
                    type_params: vec![CcType {
                        name: "int".to_string(),
                        is_const: false,
                        type_params: vec![],
                    }],
                },
            },
            params: vec![FuncParam {
                identifier: Identifier { identifier: "p".to_string() },
                type_: MappedType {
                    rs_type: RsType {
                        name: "*const".to_string(),
                        type_params: vec![RsType {
                            name: "*mut".to_string(),
                            type_params: vec![RsType {
                                name: "i32".to_string(),
                                type_params: vec![],
                            }],
                        }],
                    },
                    cc_type: CcType {
                        name: "*".to_string(),
                        is_const: false,
                        type_params: vec![CcType {
                            name: "*".to_string(),
                            is_const: true,
                            type_params: vec![CcType {
                                name: "int".to_string(),
                                is_const: false,
                                type_params: vec![],
                            }],
                        }],
                    },
                },
            }],
            is_inline: true,
        })]);
        assert_eq!(
            generate_rs_api(&ir)?,
            rustfmt(
                quote! {
                    #[inline(always)]
                    pub fn Deref(p: *const *mut i32) -> *mut i32 {
                        unsafe { crate::detail::__rust_thunk__Deref(p) }
                    }

                    mod detail {
                        extern "C" {
                            pub(crate) fn __rust_thunk__Deref(p: *const *mut i32) -> *mut i32;
                        } // extern
                    } // mod detail
                }
                .to_string()
            )?
        );

        assert_eq!(
            generate_rs_api_impl(&ir)?,
            cc_tokens_to_string(quote! {
                extern "C" int* __rust_thunk__Deref(int* const * p) {
                    return Deref(p);
                }
            })?
        );
        Ok(())
    }

    #[test]
    fn test_item_order() -> Result<()> {
        let ir = ir_items(vec![
            ir_func("first_func"),
            ir_record("FirstStruct"),
            ir_func("second_func"),
            ir_record("SecondStruct"),
        ]);

        let rs_api = generate_rs_api(&ir)?;

        let idx = |s: &str| rs_api.find(s).ok_or(anyhow!("'{}' missing", s));

        let f1 = idx("fn first_func")?;
        let f2 = idx("fn second_func")?;
        let s1 = idx("struct FirstStruct")?;
        let s2 = idx("struct SecondStruct")?;
        let t1 = idx("fn __rust_thunk__first_func")?;
        let t2 = idx("fn __rust_thunk__second_func")?;

        assert!(f1 < s1);
        assert!(s1 < f2);
        assert!(f2 < s2);
        assert!(s2 < t1);
        assert!(t1 < t2);

        Ok(())
    }

    #[test]
    fn test_doc_comment() -> Result<()> {
        let ir = IR {
            used_headers: vec![],
            items: vec![Item::Record(Record {
                identifier: ir_id("SomeStruct"),
                doc_comment: Some("Doc Comment\n\n * with bullet".to_string()),
                alignment: 0,
                size: 0,
                fields: vec![],
                copy_constructor: ir_public_trivial_special(),
                move_constructor: ir_public_trivial_special(),
                destructor: ir_public_trivial_special(),
                is_trivial_abi: true,
            })],
        };

        generate_rs_api(&ir)?
            .find("/// Doc Comment\n///\n///  * with bullet\n")
            .expect("doc comment missing");

        Ok(())
    }
}
