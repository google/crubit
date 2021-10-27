// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{anyhow, bail, Result};
use ffi_types::*;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::format_ident;
use quote::quote;
use std::collections::BTreeSet;
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
///    * function doesn't take ownership of (in other words it borrows) the
///      param `json`
///    * function passes ownership of the returned value to the caller
///
/// Safety:
///    * function expects that param `json` is a FfiU8Slice for a valid array of
///      bytes with the given size.
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

/// Rust source code with attached information about how to modify the parent
/// crate.
///
/// For example, the snippet `vec![].into_raw_parts()` is not valid unless the
/// `vec_into_raw_parts` feature is enabled. So such a snippet should be
/// represented as:
///
/// ```
/// RsSnippet {
///   features: btree_set![make_ident("vec_into_raw_parts")],
///   tokens: quote!{vec![].into_raw_parts()},
/// }
/// ```
struct RsSnippet {
    /// Rust feature flags used by this snippet.
    features: BTreeSet<Ident>,
    /// The snippet itself, as a token stream.
    tokens: TokenStream,
}

impl From<TokenStream> for RsSnippet {
    fn from(tokens: TokenStream) -> Self {
        RsSnippet { features: BTreeSet::new(), tokens }
    }
}

/// If we know the original C++ function is codegenned and already compatible
/// with `extern "C"` calling convention we skip creating/calling the C++ thunk
/// since we can call the original C++ directly.
fn can_skip_cc_thunk(func: &Func) -> bool {
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
    !func.is_inline
}

/// Generates Rust source code for a given `Func`.
///
/// Returns the generated function or trait impl, and the thunk, as a tuple.
fn generate_func(func: &Func) -> Result<(RsSnippet, RsSnippet)> {
    let mangled_name = &func.mangled_name;
    let thunk_ident = thunk_ident(func)?;
    let doc_comment = generate_doc_comment(&func.doc_comment);
    // TODO(hlopko): do not emit `-> ()` when return type is void, it's implicit.
    let return_type_name = format_rs_type(&func.return_type.rs_type)?;

    let param_idents =
        func.params.iter().map(|p| make_ident(&p.identifier.identifier)).collect_vec();

    let param_types =
        func.params.iter().map(|p| format_rs_type(&p.type_.rs_type)).collect::<Result<Vec<_>>>()?;

    let api_func = match &func.name {
        UnqualifiedIdentifier::Identifier(id) => {
            let ident = make_ident(&id.identifier);
            quote! {
                #doc_comment
                #[inline(always)]
                pub fn #ident( #( #param_idents: #param_types ),* ) -> #return_type_name {
                    unsafe { crate::detail::#thunk_ident( #( #param_idents ),* ) }
                }
            }
        }

        UnqualifiedIdentifier::Destructor => {
            let type_name = make_ident(
                &func
                    .member_func_metadata
                    .as_ref()
                    .expect("Destructors must be member functions.")
                    .for_type
                    .identifier,
            );
            // TODO(b/200066399): do not implement Drop if the type is trivial.
            quote! {
                #doc_comment
                impl Drop for #type_name {
                    #[inline(always)]
                    fn drop(&mut self) {
                        unsafe { crate::detail::#thunk_ident(self) }
                    }
                }
            }
        }

        _ => quote! {}, // TODO(b/200066396): define these.
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

    Ok((api_func.into(), thunk.into()))
}

fn generate_doc_comment(comment: &Option<String>) -> TokenStream {
    match comment {
        Some(text) => {
            let doc = format!(" {}", text.replace("\n", "\n "));
            quote! {#[doc=#doc]}
        }
        None => quote! {},
    }
}
/// Generates Rust source code for a given `Record` and associated assertions as
/// a tuple.
fn generate_record(record: &Record) -> Result<(RsSnippet, RsSnippet)> {
    let ident = make_ident(&record.identifier.identifier);
    let doc_comment = generate_doc_comment(&record.doc_comment);
    let field_idents =
        record.fields.iter().map(|f| make_ident(&f.identifier.identifier)).collect_vec();
    let field_doc_coments =
        record.fields.iter().map(|f| generate_doc_comment(&f.doc_comment)).collect_vec();
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
    let mut record_features = BTreeSet::new();
    let mut assertion_features = BTreeSet::new();

    // TODO(mboehme): For the time being, we're using unstable features to
    // be able to use offset_of!() in static assertions. This is fine for a
    // prototype, but longer-term we want to either get those features
    // stabilized or find an alternative. For more details, see
    // b/200120034#comment15
    assertion_features.insert(make_ident("const_ptr_offset_from"));
    assertion_features.insert(make_ident("const_maybe_uninit_as_ptr"));
    assertion_features.insert(make_ident("const_raw_ptr_deref"));

    let derives = generate_copy_derives(record);
    let derives = if derives.is_empty() {
        quote! {}
    } else {
        quote! {#[derive( #(#derives),* )]}
    };
    let unpin_impl;
    if record.is_trivial_abi {
        unpin_impl = quote! {};
    } else {
        // negative_impls are necessary for universal initialization due to Rust's
        // coherence rules: PhantomPinned isn't enough to prove to Rust that a
        // blanket impl that requires Unpin doesn't apply. See http://<internal link>=h.f6jp8ifzgt3n
        record_features.insert(make_ident("negative_impls"));
        unpin_impl = quote! {
            __NEWLINE__  __NEWLINE__
            impl !Unpin for #ident {}
        };
    }

    let empty_struct_placeholder_field = if record.fields.is_empty() {
        quote! {
          /// Prevent empty C++ struct being zero-size in Rust.
          placeholder: core::mem::MaybeUninit<u8>,
        }
    } else {
        quote! {}
    };

    let record_tokens = quote! {
        #doc_comment
        #derives
        #[repr(C)]
        pub struct #ident {
            #( #field_doc_coments #field_accesses #field_idents: #field_types, )*
            #empty_struct_placeholder_field
        }

        #unpin_impl
    };

    let assertion_tokens = quote! {
        const_assert_eq!(std::mem::size_of::<#ident>(), #size);
        const_assert_eq!(std::mem::align_of::<#ident>(), #alignment);
        #( #field_assertions )*
    };

    Ok((
        RsSnippet { features: record_features, tokens: record_tokens },
        RsSnippet { features: assertion_features, tokens: assertion_tokens },
    ))
}

fn generate_copy_derives(record: &Record) -> Vec<Ident> {
    if record.is_trivial_abi
        && record.copy_constructor.access == ir::AccessSpecifier::Public
        && record.copy_constructor.definition == SpecialMemberDefinition::Trivial
    {
        // TODO(b/202258760): Make `Copy` inclusion configurable.
        vec![make_ident("Clone"), make_ident("Copy")]
    } else {
        vec![]
    }
}

/// Generates Rust source code for a given `UnsupportedItem`.
fn generate_unsupported(item: &UnsupportedItem) -> Result<TokenStream> {
    let location = if item.source_loc.filename.is_empty() {
        "<unknown location>".to_string()
    } else {
        // TODO(forster): The "google3" prefix should probably come from a command line
        // argument.
        // TODO(forster): Consider linking to the symbol instead of to the line number
        // to avoid wrong links while generated files have not caught up.
        format!("google3/{};l={}", &item.source_loc.filename, &item.source_loc.line)
    };
    let message = format!(
        "{}\nError while generating bindings for item '{}':\n{}",
        &location, &item.name, &item.message
    );
    Ok(quote! { __COMMENT__ #message })
}

/// Generates Rust source code for a given `Comment`.
fn generate_comment(comment: &Comment) -> Result<TokenStream> {
    let text = &comment.text;
    Ok(quote! { __COMMENT__ #text })
}

fn generate_rs_api(ir: &IR) -> Result<String> {
    let mut items = vec![];
    let mut thunks = vec![];
    let mut assertions = vec![];

    // TODO(jeanpierreda): Delete has_record, either in favor of using RsSnippet, or not
    // having uses. See https://chat.google.com/room/AAAAnQmj8Qs/6QbkSvWcfhA
    let mut has_record = false;
    let mut features = BTreeSet::new();

    // For #![rustfmt::skip].
    features.insert(make_ident("custom_inner_attributes"));

    for item in &ir.items {
        match item {
            Item::Func(func) => {
                let (snippet, thunk) = generate_func(func)?;
                features.extend(snippet.features);
                features.extend(thunk.features);
                items.push(snippet.tokens);
                thunks.push(thunk.tokens);
            }
            Item::Record(record) => {
                let (snippet, assertions_snippet) = generate_record(record)?;
                features.extend(snippet.features);
                features.extend(assertions_snippet.features);
                items.push(snippet.tokens);
                assertions.push(assertions_snippet.tokens);
                has_record = true;
            }
            Item::UnsupportedItem(unsupported) => items.push(generate_unsupported(unsupported)?),
            Item::Comment(comment) => items.push(generate_comment(comment)?),
        }
    }

    let mod_detail = if thunks.is_empty() {
        quote! {}
    } else {
        quote! {
            mod detail {
                use super::*;
                extern "C" {
                    #( #thunks )*
                }
            }
        }
    };

    let imports = if has_record {
        quote! {
            use memoffset_unstable_const::offset_of;
            use static_assertions::const_assert_eq;
        }
    } else {
        quote! {}
    };

    let features = if features.is_empty() {
        quote! {}
    } else {
        quote! {
            #![feature( #(#features),* )]
        }
    };

    let result = quote! {
        #features __NEWLINE__ __NEWLINE__
        #imports __NEWLINE__ __NEWLINE__

        #( #items __NEWLINE__ __NEWLINE__ )*

        #mod_detail __NEWLINE__ __NEWLINE__

         #( #assertions __NEWLINE__ __NEWLINE__ )*
    };

    rustfmt(token_stream_printer::tokens_to_string(result)?)
}

fn rustfmt(input: String) -> Result<String> {
    // TODO(forster): This should use rustfmt as a library as soon as b/200503084 is
    // fixed.

    let rustfmt = "third_party/unsupported_toolchains/rust/toolchains/nightly/bin/rustfmt";

    let mut child = Command::new(rustfmt)
        .args(&[
            // TODO(forster): Add a way to specify this as a command line parameter.
            "--config-path=external/rustfmt/rustfmt.toml",
            // We are representing doc comments as attributes in the token stream and use rustfmt
            // to unpack them again.
            "--config=normalize_doc_attributes=true",
            // We don't want rustfmt to reflow C++ doc comments, so we turn off wrapping globally
            // and reflow generated comments manually.
            "--config=wrap_comments=false",
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

    // The code is formatted with a non-default rustfmt configuration. Prevent
    // downstream workflows from reformatting with a different configuration.
    let mut result = "#![rustfmt::skip]\n".to_string();
    result += &*String::from_utf8_lossy(&output.stdout);
    Ok(result)
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
            match ty.name.as_str() {
                "()" => Ok(quote! {()}),
                name => {
                    let ident = make_ident(name);
                    Ok(quote! {#ident})
                }
            }
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

fn thunk_ident(func: &Func) -> Result<Ident> {
    let get_type_name = || {
        if let Some(meta) = &func.member_func_metadata {
            Ok(&meta.for_type.identifier)
        } else {
            bail!("Special member function must be a member function: {:?}", func.name)
        }
    };
    match &func.name {
        UnqualifiedIdentifier::Identifier(id) => {
            Ok(format_ident!("__rust_thunk__{}", &id.identifier))
        }
        UnqualifiedIdentifier::Destructor => {
            Ok(format_ident!("__rust_destructor_thunk__{}", get_type_name()?))
        }
        UnqualifiedIdentifier::Constructor => {
            Ok(format_ident!("__rust_constructor_thunk__{}", get_type_name()?))
        }
    }
}

fn generate_rs_api_impl(ir: &IR) -> Result<String> {
    // This function uses quote! to generate C++ source code out of convenience.
    // This is a bold idea so we have to continously evaluate if it still makes
    // sense or the cost of working around differences in Rust and C++ tokens is
    // greather than the value added.
    //
    // See rs_bindings_from_cc/
    // token_stream_printer.rs for a list of supported placeholders.
    let mut thunks = vec![];
    for func in ir.functions() {
        if can_skip_cc_thunk(&func) {
            continue;
        }

        let thunk_ident = thunk_ident(func)?;
        let implementation_function = match &func.name {
            UnqualifiedIdentifier::Identifier(id) => {
                let ident = make_ident(&id.identifier);
                quote! {#ident}
            }
            // Use destroy_at to avoid needing to spell out the type name. The type name can be
            // difficult (impossible?) to spell in the general case, but by using destroy_at, we
            // avoid needing to determine what the correct spelling is, or save that spelling within
            // the IR.
            UnqualifiedIdentifier::Destructor => quote! {std::destroy_at},
            _ => continue, // TODO(b/200066396): handle other cases
        };
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
                return #implementation_function( #( #param_idents ),* );
            }
        });
    }

    let layout_assertions = ir.records().map(cc_struct_layout_assertion);

    let mut standard_headers = <BTreeSet<Ident>>::new();
    standard_headers.insert(make_ident("memory")); // ubiquitous.
    if ir.records().next().is_some() {
        standard_headers.insert(make_ident("cstddef"));
    };

    // In order to generate C++ thunk in all the cases Clang needs to be able to
    // access declarations from public headers of the C++ library.
    let includes = ir.used_headers.iter().map(|i| &i.name);

    let result = quote! {
        #( __HASH_TOKEN__ include <#standard_headers> __NEWLINE__)*
        #( __HASH_TOKEN__ include #includes __NEWLINE__)* __NEWLINE__

        #( #thunks )* __NEWLINE__ __NEWLINE__

        #( #layout_assertions __NEWLINE__ __NEWLINE__ )*

        // To satisfy http://cs/symbol:devtools.metadata.Presubmit.CheckTerminatingNewline check.
        __NEWLINE__
    };

    token_stream_printer::tokens_to_string(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use ir_testing::{
        ir_func, ir_id, ir_int, ir_int_param, ir_items, ir_public_trivial_special, ir_record,
    };
    use quote::quote;
    use token_stream_printer::tokens_to_string;

    #[test]
    fn test_simple_function() -> Result<()> {
        let ir = ir_items(vec![Item::Func(Func {
            name: UnqualifiedIdentifier::Identifier(ir_id("add")),
            mangled_name: "_Z3Addii".to_string(),
            doc_comment: None,
            return_type: ir_int(),
            params: vec![ir_int_param("a"), ir_int_param("b")],
            is_inline: false,
            member_func_metadata: None,
        })]);
        assert_eq!(
            generate_rs_api(&ir)?,
            rustfmt(tokens_to_string(quote! {
                #![feature(custom_inner_attributes)] __NEWLINE__ __NEWLINE__

                #[inline(always)]
                pub fn add(a: i32, b: i32) -> i32 {
                    unsafe { crate::detail::__rust_thunk__add(a, b) }
                } __NEWLINE__ __NEWLINE__

                mod detail {
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z3Addii"]
                        pub(crate) fn __rust_thunk__add(a: i32, b: i32) -> i32;
                    } // extern
                } // mod detail
            })?)?
        );

        let rs_api = generate_rs_api_impl(&ir)?;
        assert_eq!(rs_api.trim(), "#include<memory>");
        assert!(rs_api.ends_with("\n"));

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
                name: UnqualifiedIdentifier::Identifier(ir_id("add")),
                mangled_name: "_Z3Addii".to_string(),
                doc_comment: None,
                return_type: ir_int(),
                params: vec![ir_int_param("a"), ir_int_param("b")],
                is_inline: true,
                member_func_metadata: None,
            })],
        };

        assert_eq!(
            generate_rs_api(&ir)?,
            rustfmt(tokens_to_string(quote! {
                #![feature(custom_inner_attributes)] __NEWLINE__ __NEWLINE__

                #[inline(always)]
                pub fn add(a: i32, b: i32) -> i32 {
                    unsafe { crate::detail::__rust_thunk__add(a, b) }
                } __NEWLINE__ __NEWLINE__

                mod detail {
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk__add(a: i32, b: i32) -> i32;
                    } // extern
                } // mod detail
            })?)?
        );

        assert_eq!(
            generate_rs_api_impl(&ir)?.trim(),
            tokens_to_string(quote! {
                __HASH_TOKEN__ include <memory> __NEWLINE__
                __HASH_TOKEN__ include "foo/bar.h" __NEWLINE__
                __HASH_TOKEN__ include "foo/baz.h" __NEWLINE__ __NEWLINE__

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
                    doc_comment: None,
                    type_: ir_int(),
                    access: AccessSpecifier::Public,
                    offset: 0,
                },
                Field {
                    identifier: ir_id("protected_int"),
                    doc_comment: None,
                    type_: ir_int(),
                    access: AccessSpecifier::Protected,
                    offset: 32,
                },
                Field {
                    identifier: ir_id("private_int"),
                    doc_comment: None,
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
            rustfmt(tokens_to_string(quote! {
                #![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, const_raw_ptr_deref, custom_inner_attributes)] __NEWLINE__ __NEWLINE__

                use memoffset_unstable_const::offset_of;
                use static_assertions::const_assert_eq; __NEWLINE__ __NEWLINE__

                #[derive(Clone, Copy)]
                #[repr(C)]
                pub struct SomeStruct {
                    pub public_int: i32,
                    protected_int: i32,
                    private_int: i32,
                } __NEWLINE__ __NEWLINE__

                const_assert_eq!(std::mem::size_of::<SomeStruct>(), 12usize);
                const_assert_eq!(std::mem::align_of::<SomeStruct>(), 4usize);
                const_assert_eq!(offset_of!(SomeStruct, public_int) * 8, 0usize);
                const_assert_eq!(offset_of!(SomeStruct, protected_int) * 8, 32usize);
                const_assert_eq!(offset_of!(SomeStruct, private_int) * 8, 64usize);
            })?)?
        );
        assert_eq!(
            generate_rs_api_impl(&ir)?.trim(),
            tokens_to_string(quote! {
                __HASH_TOKEN__ include <cstddef> __NEWLINE__
                __HASH_TOKEN__ include <memory> __NEWLINE__
                __NEWLINE__ __NEWLINE__ __NEWLINE__
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
    fn test_copy_derives() {
        let record = ir_record("S");
        assert_eq!(generate_copy_derives(&record), &["Clone", "Copy"]);
    }

    #[test]
    fn test_copy_derives_not_is_trivial_abi() {
        let mut record = ir_record("S");
        record.is_trivial_abi = false;
        assert_eq!(generate_copy_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_copy_derives_ctor_nonpublic() {
        let mut record = ir_record("S");
        for access in [ir::AccessSpecifier::Protected, ir::AccessSpecifier::Private] {
            record.copy_constructor.access = access;
            assert_eq!(generate_copy_derives(&record), &[""; 0]);
        }
    }

    #[test]
    fn test_copy_derives_ctor_deleted() {
        let mut record = ir_record("S");
        record.copy_constructor.definition = ir::SpecialMemberDefinition::Deleted;
        assert_eq!(generate_copy_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_copy_derives_ctor_nontrivial_members() {
        let mut record = ir_record("S");
        record.copy_constructor.definition = ir::SpecialMemberDefinition::NontrivialMembers;
        assert_eq!(generate_copy_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_copy_derives_ctor_nontrivial_self() {
        let mut record = ir_record("S");
        record.copy_constructor.definition = ir::SpecialMemberDefinition::NontrivialSelf;
        assert_eq!(generate_copy_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_ptr_func() -> Result<()> {
        let ir = ir_items(vec![Item::Func(Func {
            name: UnqualifiedIdentifier::Identifier(Identifier { identifier: "Deref".to_string() }),
            mangled_name: "_Z5DerefPKPi".to_string(),
            doc_comment: None,
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
            member_func_metadata: None,
        })]);
        assert_eq!(
            generate_rs_api(&ir)?,
            rustfmt(tokens_to_string(quote! {
                #![feature(custom_inner_attributes)] __NEWLINE__ __NEWLINE__

                #[inline(always)]
                pub fn Deref(p: *const *mut i32) -> *mut i32 {
                    unsafe { crate::detail::__rust_thunk__Deref(p) }
                } __NEWLINE__ __NEWLINE__

                mod detail {
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk__Deref(p: *const *mut i32) -> *mut i32;
                    } // extern
                } // mod detail
            })?)?
        );

        assert_eq!(
            generate_rs_api_impl(&ir)?.trim(),
            tokens_to_string(quote! {
                __HASH_TOKEN__ include <memory> __NEWLINE__
                __NEWLINE__
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
            ir_func("first_func").into(),
            ir_record("FirstStruct").into(),
            ir_func("second_func").into(),
            ir_record("SecondStruct").into(),
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
    fn test_doc_comment_func() -> Result<()> {
        let ir = IR {
            used_headers: vec![],
            items: vec![Item::Func(Func {
                name: UnqualifiedIdentifier::Identifier(ir_id("func")),
                mangled_name: "foo".to_string(),
                doc_comment: Some("Doc Comment".to_string()),
                return_type: ir_int(),
                params: vec![],
                is_inline: true,
                member_func_metadata: None,
            })],
        };

        assert!(
            generate_rs_api(&ir)?.contains("/// Doc Comment\n#[inline(always)]\npub fn func"),
            "func doc comment missing"
        );

        Ok(())
    }

    #[test]
    fn test_doc_comment_record() -> Result<()> {
        let ir = IR {
            used_headers: vec![],
            items: vec![Item::Record(Record {
                identifier: ir_id("SomeStruct"),
                doc_comment: Some("Doc Comment\n\n * with bullet".to_string()),
                alignment: 0,
                size: 0,
                fields: vec![Field {
                    identifier: ir_id("field"),
                    doc_comment: Some("Field doc".to_string()),
                    type_: ir_int(),
                    access: AccessSpecifier::Public,
                    offset: 0,
                }],
                copy_constructor: ir_public_trivial_special(),
                move_constructor: ir_public_trivial_special(),
                destructor: ir_public_trivial_special(),
                is_trivial_abi: true,
            })],
        };

        let rs_api = generate_rs_api(&ir)?;
        assert!(
            rs_api.contains("/// Doc Comment\n///\n///  * with bullet\n"),
            "struct doc comment missing"
        );
        assert!(rs_api.contains("/// Field doc\n"), "field doc comment missing");

        Ok(())
    }

    /// At the least, a trivial type should have no drop impl if or until we add
    /// empty drop impls.
    #[test]
    fn test_no_impl_drop() -> Result<()> {
        let ir = ir_testing::ir_from_cc("struct Trivial {};")?;
        let rs_api = generate_rs_api(&ir)?;
        assert!(!rs_api.contains("impl Drop"));
        Ok(())
    }

    /// User-defined destructors *must* become Drop impls.
    #[test]
    fn test_impl_drop() -> Result<()> {
        let ir = ir_testing::ir_from_cc(
            r#"struct UserDefinedDestructor {
                ~UserDefinedDestructor();
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert!(rs_api.contains("impl Drop"));
        Ok(())
    }

    #[test]
    fn test_thunk_ident_function() {
        let func = ir_func("foo");
        assert_eq!(thunk_ident(&func).unwrap(), make_ident("__rust_thunk__foo"));
    }

    #[test]
    fn test_thunk_ident_special_names() {
        let mut func = ir_func("unused");
        func.member_func_metadata = Some(ir::MemberFuncMetadata {
            for_type: ir_id("Class"),
            instance_method_metadata: None,
        });

        func.name = UnqualifiedIdentifier::Destructor;
        assert_eq!(thunk_ident(&func).unwrap(), make_ident("__rust_destructor_thunk__Class"));

        func.name = UnqualifiedIdentifier::Constructor;
        assert_eq!(thunk_ident(&func).unwrap(), make_ident("__rust_constructor_thunk__Class"));
    }

    #[test]
    fn test_thunk_ident_invalid() {
        let mut func = ir_func("unused");
        for name in [UnqualifiedIdentifier::Destructor, UnqualifiedIdentifier::Constructor] {
            func.name = name;
            assert!(
                thunk_ident(&func).unwrap_err().to_string().contains("must be a member function")
            );
        }
    }
}
