// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{bail, Result};
use ffi_types::*;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::format_ident;
use quote::quote;
use std::collections::{BTreeSet, HashMap};
use std::convert::TryInto;
use std::iter::Iterator;
use std::panic::catch_unwind;
use std::process;
use token_stream_printer::{rs_tokens_to_formatted_string, tokens_to_string};

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

    // The code is formatted with a non-default rustfmt configuration. Prevent
    // downstream workflows from reformatting with a different configuration.
    let rs_api = format!("#![rustfmt::skip]\n{}", generate_rs_api(&ir)?);
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
fn generate_func(func: &Func, ir: &IR) -> Result<(RsSnippet, RsSnippet)> {
    let mangled_name = &func.mangled_name;
    let thunk_ident = thunk_ident(func);
    let doc_comment = generate_doc_comment(&func.doc_comment);
    let lifetime_to_name = HashMap::<LifetimeId, String>::from_iter(
        func.lifetime_params.iter().map(|l| (l.id, l.name.clone())),
    );
    // TODO(hlopko): do not emit `-> ()` when return type is void, it's implicit.
    let return_type_name = format_rs_type(&func.return_type.rs_type, ir, &lifetime_to_name)?;

    let param_idents =
        func.params.iter().map(|p| make_ident(&p.identifier.identifier)).collect_vec();

    let param_types = func
        .params
        .iter()
        .map(|p| format_rs_type(&p.type_.rs_type, ir, &lifetime_to_name))
        .collect::<Result<Vec<_>>>()?;

    let lifetimes = func
        .lifetime_params
        .iter()
        .map(|l| syn::Lifetime::new(&format!("'{}", l.name), proc_macro2::Span::call_site()))
        .collect_vec();
    let generic_params = if lifetimes.is_empty() {
        quote! {}
    } else {
        quote! { < #( #lifetimes ),* > }
    };

    let mut calls_thunk = true;
    let api_func = match &func.name {
        UnqualifiedIdentifier::Identifier(id) => {
            let ident = make_ident(&id.identifier);
            quote! {
                #doc_comment
                #[inline(always)]
                pub fn #ident #generic_params( #( #param_idents: #param_types ),* ) -> #return_type_name {
                    unsafe { crate::detail::#thunk_ident( #( #param_idents ),* ) }
                }
            }
        }

        UnqualifiedIdentifier::Destructor => {
            let record: &Record = ir
                .find_decl(
                    func.member_func_metadata
                        .as_ref()
                        .expect("Destructors must be member functions.")
                        .record_id,
                )?
                .try_into()?;
            let type_name = make_ident(&record.identifier.identifier);
            match record.destructor.definition {
                // TODO(b/202258760): Only omit destructor if `Copy` is specified.
                SpecialMemberDefinition::Trivial => {
                    calls_thunk = false;
                    quote! {}
                }
                SpecialMemberDefinition::NontrivialMembers => {
                    calls_thunk = false;
                    quote! {
                        #doc_comment
                        impl Drop for #type_name {
                            #[inline(always)]
                            fn drop(&mut self) {
                                /* the destructors of the members can be invoked instead */
                            }
                        }
                    }
                }
                SpecialMemberDefinition::NontrivialUserDefined => {
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
                SpecialMemberDefinition::Deleted => {
                    bail!("Deleted destructors can't be called") // TODO(b/200066399): handle this?
                }
            }
        }

        _ => quote! {}, // TODO(b/200066396): define these.
    };

    let thunk = if calls_thunk {
        let thunk_attr = if can_skip_cc_thunk(func) {
            quote! {#[link_name = #mangled_name]}
        } else {
            quote! {}
        };

        quote! {
            #thunk_attr
            pub(crate) fn #thunk_ident #generic_params( #( #param_idents: #param_types ),* ) -> #return_type_name ;
        }
    } else {
        quote! {}
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
fn generate_record(record: &Record, ir: &IR) -> Result<(RsSnippet, RsSnippet)> {
    let ident = make_ident(&record.identifier.identifier);
    let doc_comment = generate_doc_comment(&record.doc_comment);
    let field_idents =
        record.fields.iter().map(|f| make_ident(&f.identifier.identifier)).collect_vec();
    let field_doc_coments =
        record.fields.iter().map(|f| generate_doc_comment(&f.doc_comment)).collect_vec();
    let field_types = record
        .fields
        .iter()
        .map(|f| {
            let mut formatted = format_rs_type(&f.type_.rs_type, ir, &HashMap::new())?;
            if record.destructor.definition == SpecialMemberDefinition::NontrivialUserDefined {
                formatted = quote! {
                    std::mem::ManuallyDrop<#formatted>
                };
            }
            Ok(formatted)
        })
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
                const _: () = assert!(offset_of!(#ident, #field_ident) * 8 == #offset);
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
        const _: () = assert!(std::mem::size_of::<#ident>() == #size);
        const _: () = assert!(std::mem::align_of::<#ident>() == #alignment);
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

    // We import nullable pointers as an Option<&T> and assume that at the ABI
    // level, None is represented as a zero pointer value whereas Some is
    // represented as as non-zero pointer value. This seems like a pretty safe
    // assumption to make, but to provide some safeguard, assert that
    // `Option<&i32>` and `&i32` have the same size.
    assertions.push(quote! {
        const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
    });

    // TODO(jeanpierreda): Delete has_record, either in favor of using RsSnippet, or not
    // having uses. See https://chat.google.com/room/AAAAnQmj8Qs/6QbkSvWcfhA
    let mut has_record = false;
    let mut features = BTreeSet::new();

    // For #![rustfmt::skip].
    features.insert(make_ident("custom_inner_attributes"));

    for item in ir.items() {
        match item {
            Item::Func(func) => {
                let (snippet, thunk) = generate_func(func, ir)?;
                features.extend(snippet.features);
                features.extend(thunk.features);
                items.push(snippet.tokens);
                thunks.push(thunk.tokens);
            }
            Item::Record(record) => {
                if !ir.is_in_current_target(record) {
                    continue;
                }
                let (snippet, assertions_snippet) = generate_record(record, ir)?;
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

    rs_tokens_to_formatted_string(result)
}

fn make_ident(ident: &str) -> Ident {
    format_ident!("{}", ident)
}

fn format_rs_type(
    ty: &ir::RsType,
    ir: &IR,
    lifetime_to_name: &HashMap<LifetimeId, String>,
) -> Result<TokenStream> {
    enum TypeKind<'a> {
        Pointer(TokenStream),
        Reference(TokenStream),
        Record(TokenStream),
        Unit,
        Other(&'a str),
    }
    let kind = if let Some(ref name) = ty.name {
        match name.as_str() {
            "*mut" => TypeKind::Pointer(quote! {mut}),
            "*const" => TypeKind::Pointer(quote! {const}),
            "&mut" => TypeKind::Reference(quote! {mut}),
            "&" => TypeKind::Reference(quote! {}),
            "()" => TypeKind::Unit,
            _ => TypeKind::Other(name),
        }
    } else {
        let record = ir.record_for_type(ty)?;
        let ident = make_ident(record.identifier.identifier.as_str());
        let path: TokenStream = if ir.is_in_current_target(record) {
            quote! {#ident}
        } else {
            let owning_crate = make_ident(record.owning_crate_name()?);
            quote! {#owning_crate::#ident}
        };
        TypeKind::Record(path)
    };
    match kind {
        TypeKind::Pointer(mutability) => {
            if ty.type_args.len() != 1 {
                bail!("Invalid pointer type (need exactly 1 type argument): {:?}", ty);
            }
            let nested_type = format_rs_type(&ty.type_args[0], ir, lifetime_to_name)?;
            Ok(quote! {* #mutability #nested_type})
        }
        TypeKind::Reference(mutability) => {
            if ty.lifetime_args.len() != 1 || ty.type_args.len() != 1 {
                bail!(
                    "Invalid reference type (need exactly 1 lifetime argument and 1 type argument): {:?}",
                    ty
                );
            }
            let nested_type = format_rs_type(&ty.type_args[0], ir, lifetime_to_name)?;
            let lifetime_id = &ty.lifetime_args[0];
            let lifetime = syn::Lifetime::new(
                &format!("'{}", lifetime_to_name.get(lifetime_id).unwrap()),
                proc_macro2::Span::call_site(),
            );
            Ok(quote! {& #lifetime #mutability #nested_type})
        }
        TypeKind::Record(path) => {
            if !ty.type_args.is_empty() {
                bail!("Type arguments on records are not yet supported: {:?}", ty);
            }
            Ok(path)
        }
        TypeKind::Unit => {
            if !ty.type_args.is_empty() {
                bail!("Unit type must not have type arguments: {:?}", ty);
            }
            Ok(quote! {()})
        }
        TypeKind::Other(name) => {
            let ident = make_ident(name);
            let type_args = if ty.type_args.is_empty() {
                quote! {}
            } else {
                let mut formatted_args = vec![];
                for type_arg in ty.type_args.iter() {
                    formatted_args.push(format_rs_type(type_arg, ir, lifetime_to_name)?);
                }
                quote! { < #( #formatted_args ),* > }
            };
            Ok(quote! {#ident #type_args})
        }
    }
}

fn format_cc_type(ty: &ir::CcType, ir: &IR) -> Result<TokenStream> {
    let const_fragment = if ty.is_const {
        quote! {const}
    } else {
        quote! {}
    };
    if let Some(ref name) = ty.name {
        match name.as_str() {
            "*" => {
                if ty.type_args.len() != 1 {
                    bail!("Invalid pointer type (need exactly 1 type argument): {:?}", ty);
                }
                assert_eq!(ty.type_args.len(), 1);
                let nested_type = format_cc_type(&ty.type_args[0], ir)?;
                Ok(quote! {#nested_type * #const_fragment})
            }
            ident => {
                if !ty.type_args.is_empty() {
                    bail!("Type not yet supported: {:?}", ty);
                }
                let ident = make_ident(ident);
                Ok(quote! {#ident #const_fragment})
            }
        }
    } else {
        let ident = make_ident(ir.record_for_type(ty)?.identifier.identifier.as_str());
        Ok(quote! {#ident})
    }
}

fn cc_struct_layout_assertion(record: &Record, ir: &IR) -> TokenStream {
    if !ir.is_in_current_target(record) {
        return quote! {};
    }
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

fn thunk_ident(func: &Func) -> Ident {
    format_ident!("__rust_thunk__{}", func.mangled_name)
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

        let thunk_ident = thunk_ident(func);
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
        let return_type_name = format_cc_type(&func.return_type.cc_type, ir)?;

        let param_idents =
            func.params.iter().map(|p| make_ident(&p.identifier.identifier)).collect_vec();

        let param_types = func
            .params
            .iter()
            .map(|p| format_cc_type(&p.type_.cc_type, ir))
            .collect::<Result<Vec<_>>>()?;

        thunks.push(quote! {
            extern "C" #return_type_name #thunk_ident( #( #param_types #param_idents ),* ) {
                return #implementation_function( #( #param_idents ),* );
            }
        });
    }

    let layout_assertions = ir.records().map(|record| cc_struct_layout_assertion(record, ir));

    let mut standard_headers = <BTreeSet<Ident>>::new();
    standard_headers.insert(make_ident("memory")); // ubiquitous.
    if ir.records().next().is_some() {
        standard_headers.insert(make_ident("cstddef"));
    };

    // In order to generate C++ thunk in all the cases Clang needs to be able to
    // access declarations from public headers of the C++ library.
    let includes = ir.used_headers().map(|i| &i.name);

    let result = quote! {
        #( __HASH_TOKEN__ include <#standard_headers> __NEWLINE__)*
        #( __HASH_TOKEN__ include #includes __NEWLINE__)* __NEWLINE__

        #( #thunks )* __NEWLINE__ __NEWLINE__

        #( #layout_assertions __NEWLINE__ __NEWLINE__ )*

        // To satisfy http://cs/symbol:devtools.metadata.Presubmit.CheckTerminatingNewline check.
        __NEWLINE__
    };

    tokens_to_string(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use ir_testing::{
        ir_from_cc, ir_from_cc_dependency, ir_func, ir_id, ir_int, ir_int_param,
        ir_public_trivial_special, ir_record, retrieve_func,
    };
    use quote::quote;
    use token_stream_printer::tokens_to_string;

    fn assert_code_contains(code: &TokenStream, snippet: &str) {
        let code_str = rs_tokens_to_formatted_string(code.clone()).unwrap();
        assert!(code_str.contains(snippet), "{}", code_str);
    }

    #[test]
    // TODO(hlopko): Move this test to a more principled place where it can access
    // `ir_testing`.
    fn test_duplicate_decl_ids_err() {
        let mut r1 = ir_record("R1");
        r1.id = DeclId(42);
        let mut r2 = ir_record("R2");
        r2.id = DeclId(42);
        let result = make_ir_from_items([r1.into(), r2.into()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate decl_id found in"));
    }

    #[test]
    fn test_simple_function() -> Result<()> {
        let ir = make_ir_from_items([Item::Func(Func {
            name: UnqualifiedIdentifier::Identifier(ir_id("add")),
            owning_target: ir::TESTING_TARGET.into(),
            mangled_name: "_Z3Addii".to_string(),
            doc_comment: None,
            return_type: ir_int(),
            params: vec![ir_int_param("a"), ir_int_param("b")],
            lifetime_params: vec![],
            is_inline: false,
            member_func_metadata: None,
        })])?;
        assert_eq!(
            generate_rs_api(&ir)?,
            rs_tokens_to_formatted_string(quote! {
                #![feature(custom_inner_attributes)] __NEWLINE__ __NEWLINE__

                #[inline(always)]
                pub fn add(a: i32, b: i32) -> i32 {
                    unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
                } __NEWLINE__ __NEWLINE__

                mod detail {
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z3Addii"]
                        pub(crate) fn __rust_thunk___Z3Addii(a: i32, b: i32) -> i32;
                    } // extern
                } // mod detail
                __NEWLINE__ __NEWLINE__
                const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
            })?
        );

        let rs_api = generate_rs_api_impl(&ir)?;
        assert_eq!(rs_api.trim(), "#include<memory>");
        assert!(rs_api.ends_with("\n"));

        Ok(())
    }

    #[test]
    fn test_inline_function() -> Result<()> {
        let ir = make_ir_from_parts(
            vec![Item::Func(Func {
                name: UnqualifiedIdentifier::Identifier(ir_id("add")),
                owning_target: ir::TESTING_TARGET.into(),
                mangled_name: "_Z3Addii".to_string(),
                doc_comment: None,
                return_type: ir_int(),
                params: vec![ir_int_param("a"), ir_int_param("b")],
                lifetime_params: vec![],
                is_inline: true,
                member_func_metadata: None,
            })],
            /* used_headers= */
            vec![
                HeaderName { name: "foo/bar.h".to_string() },
                HeaderName { name: "foo/baz.h".to_string() },
            ],
            /* current_target= */ "//foo:bar".into(),
        )?;

        assert_eq!(
            generate_rs_api(&ir)?,
            rs_tokens_to_formatted_string(quote! {
                #![feature(custom_inner_attributes)] __NEWLINE__ __NEWLINE__

                #[inline(always)]
                pub fn add(a: i32, b: i32) -> i32 {
                    unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
                } __NEWLINE__ __NEWLINE__

                mod detail {
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk___Z3Addii(a: i32, b: i32) -> i32;
                    } // extern
                } // mod detail
                __NEWLINE__ __NEWLINE__
                const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
            })?
        );

        assert_eq!(
            generate_rs_api_impl(&ir)?.trim(),
            tokens_to_string(quote! {
                __HASH_TOKEN__ include <memory> __NEWLINE__
                __HASH_TOKEN__ include "foo/bar.h" __NEWLINE__
                __HASH_TOKEN__ include "foo/baz.h" __NEWLINE__ __NEWLINE__

                extern "C" int __rust_thunk___Z3Addii(int a, int b) {
                    return add(a, b);
                }
            })?
        );
        Ok(())
    }

    #[test]
    fn test_simple_function_with_types_from_other_target() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "inline ReturnStruct DoSomething(ParamStruct param);",
            "struct ReturnStruct {}; struct ParamStruct {};",
        )?;

        assert_eq!(
            generate_rs_api(&ir)?,
            rs_tokens_to_formatted_string(quote! {
                #![feature(custom_inner_attributes)] __NEWLINE__ __NEWLINE__

                #[inline(always)]
                pub fn DoSomething(param: dependency::ParamStruct)
                  -> dependency::ReturnStruct {
                    unsafe { crate::detail::__rust_thunk___Z11DoSomething11ParamStruct(param) }
                } __NEWLINE__ __NEWLINE__

                mod detail {
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk___Z11DoSomething11ParamStruct(param: dependency::ParamStruct)
                          -> dependency::ReturnStruct;
                    } // extern
                } // mod detail
                __NEWLINE__ __NEWLINE__
                const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
            })?
        );

        assert_eq!(
            generate_rs_api_impl(&ir)?.trim(),
            tokens_to_string(quote! {
                __HASH_TOKEN__ include <cstddef> __NEWLINE__
                __HASH_TOKEN__ include <memory> __NEWLINE__
                __HASH_TOKEN__ include "ir_from_cc_virtual_header.h" __NEWLINE__ __NEWLINE__

                extern "C" ReturnStruct __rust_thunk___Z11DoSomething11ParamStruct(ParamStruct param) {
                    return DoSomething(param);
                }
            })?
        );
        Ok(())
    }

    #[test]
    fn test_simple_struct() -> Result<()> {
        let ir = make_ir_from_items([Item::Record(Record {
            identifier: ir_id("SomeStruct"),
            id: DeclId(42),
            owning_target: ir::TESTING_TARGET.into(),
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
            lifetime_params: vec![],
            size: 12,
            alignment: 4,
            copy_constructor: ir_public_trivial_special(),
            move_constructor: ir_public_trivial_special(),
            destructor: ir_public_trivial_special(),
            is_trivial_abi: true,
        })])?;

        assert_eq!(
            generate_rs_api(&ir)?,
            rs_tokens_to_formatted_string(quote! {
                #![feature(const_ptr_offset_from, custom_inner_attributes)] __NEWLINE__ __NEWLINE__

                use memoffset_unstable_const::offset_of; __NEWLINE__ __NEWLINE__

                #[derive(Clone, Copy)]
                #[repr(C)]
                pub struct SomeStruct {
                    pub public_int: i32,
                    protected_int: i32,
                    private_int: i32,
                } __NEWLINE__ __NEWLINE__

                const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
                __NEWLINE__ __NEWLINE__
                const _: () = assert!(std::mem::size_of::<SomeStruct>() == 12usize);
                const _: () = assert!(std::mem::align_of::<SomeStruct>() == 4usize);
                const _: () = assert!(offset_of!(SomeStruct, public_int) * 8 == 0usize);
                const _: () = assert!(offset_of!(SomeStruct, protected_int) * 8 == 32usize);
                const _: () = assert!(offset_of!(SomeStruct, private_int) * 8 == 64usize);
            })?
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
    fn test_struct_from_other_target() -> Result<()> {
        let ir = ir_from_cc_dependency("// intentionally empty", "struct SomeStruct {};")?;

        assert_eq!(
            generate_rs_api(&ir)?,
            rs_tokens_to_formatted_string(quote! {
              #![feature(custom_inner_attributes)]
              __NEWLINE__ __NEWLINE__
              const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
            })?
        );
        assert_eq!(
            generate_rs_api_impl(&ir)?.trim(),
            tokens_to_string(quote! {
                __HASH_TOKEN__ include <cstddef> __NEWLINE__
                __HASH_TOKEN__ include <memory> __NEWLINE__
                __HASH_TOKEN__ include "ir_from_cc_virtual_header.h"
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
        record.copy_constructor.definition = ir::SpecialMemberDefinition::NontrivialUserDefined;
        assert_eq!(generate_copy_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_ptr_func() -> Result<()> {
        let ir = make_ir_from_items([Item::Func(Func {
            name: UnqualifiedIdentifier::Identifier(Identifier { identifier: "Deref".to_string() }),
            owning_target: ir::TESTING_TARGET.into(),
            mangled_name: "_Z5DerefPKPi".to_string(),
            doc_comment: None,
            return_type: MappedType {
                rs_type: RsType {
                    name: "*mut".to_string().into(),
                    decl_id: None,
                    lifetime_args: vec![],
                    type_args: vec![RsType {
                        name: "i32".to_string().into(),
                        lifetime_args: vec![],
                        type_args: vec![],
                        decl_id: None,
                    }],
                },
                cc_type: CcType {
                    name: "*".to_string().into(),
                    is_const: false,
                    decl_id: None,
                    type_args: vec![CcType {
                        name: "int".to_string().into(),
                        is_const: false,
                        type_args: vec![],
                        decl_id: None,
                    }],
                },
            },
            params: vec![FuncParam {
                identifier: Identifier { identifier: "p".to_string() },
                type_: MappedType {
                    rs_type: RsType {
                        name: "*const".to_string().into(),
                        decl_id: None,
                        lifetime_args: vec![],
                        type_args: vec![RsType {
                            name: "*mut".to_string().into(),
                            decl_id: None,
                            lifetime_args: vec![],
                            type_args: vec![RsType {
                                name: "i32".to_string().into(),
                                lifetime_args: vec![],
                                type_args: vec![],
                                decl_id: None,
                            }],
                        }],
                    },
                    cc_type: CcType {
                        name: "*".to_string().into(),
                        is_const: false,
                        decl_id: None,
                        type_args: vec![CcType {
                            name: "*".to_string().into(),
                            is_const: true,
                            decl_id: None,
                            type_args: vec![CcType {
                                name: "int".to_string().into(),
                                is_const: false,
                                type_args: vec![],
                                decl_id: None,
                            }],
                        }],
                    },
                },
            }],
            lifetime_params: vec![],
            is_inline: true,
            member_func_metadata: None,
        })])?;
        assert_eq!(
            generate_rs_api(&ir)?,
            rs_tokens_to_formatted_string(quote! {
                #![feature(custom_inner_attributes)] __NEWLINE__ __NEWLINE__

                #[inline(always)]
                pub fn Deref(p: *const *mut i32) -> *mut i32 {
                    unsafe { crate::detail::__rust_thunk___Z5DerefPKPi(p) }
                } __NEWLINE__ __NEWLINE__

                mod detail {
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk___Z5DerefPKPi(p: *const *mut i32) -> *mut i32;
                    } // extern
                } // mod detail
                __NEWLINE__ __NEWLINE__
                const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
            })?
        );

        assert_eq!(
            generate_rs_api_impl(&ir)?.trim(),
            tokens_to_string(quote! {
                __HASH_TOKEN__ include <memory> __NEWLINE__
                __NEWLINE__
                extern "C" int* __rust_thunk___Z5DerefPKPi(int* const * p) {
                    return Deref(p);
                }
            })?
        );
        Ok(())
    }

    #[test]
    fn test_item_order() -> Result<()> {
        let ir = make_ir_from_items([
            ir_func("first_func").into(),
            ir_record("FirstStruct").into(),
            ir_func("second_func").into(),
            ir_record("SecondStruct").into(),
        ])?;

        let rs_api = generate_rs_api(&ir)?;
        let idx = |s: &str| rs_api.find(s).ok_or(anyhow!("'{}' missing", s));

        println!("{:?}", ir);

        let f1 = idx("fn first_func")?;
        let f2 = idx("fn second_func")?;
        let s1 = idx("struct FirstStruct")?;
        let s2 = idx("struct SecondStruct")?;
        let t1 = idx("fn __rust_thunk___Z10first_funcv")?;
        let t2 = idx("fn __rust_thunk___Z11second_funcv")?;

        assert!(f1 < s1);
        assert!(s1 < f2);
        assert!(f2 < s2);
        assert!(s2 < t1);
        assert!(t1 < t2);

        Ok(())
    }

    #[test]
    fn test_doc_comment_func() -> Result<()> {
        let ir = make_ir_from_items([Item::Func(Func {
            name: UnqualifiedIdentifier::Identifier(ir_id("func")),
            owning_target: ir::TESTING_TARGET.into(),
            mangled_name: "foo".to_string(),
            doc_comment: Some("Doc Comment".to_string()),
            return_type: ir_int(),
            params: vec![],
            lifetime_params: vec![],
            is_inline: true,
            member_func_metadata: None,
        })])?;

        assert!(
            generate_rs_api(&ir)?.contains("/// Doc Comment\n#[inline(always)]\npub fn func"),
            "func doc comment missing"
        );

        Ok(())
    }

    #[test]
    fn test_doc_comment_record() -> Result<()> {
        let ir = make_ir_from_items([Item::Record(Record {
            identifier: ir_id("SomeStruct"),
            id: DeclId(42),
            owning_target: ir::TESTING_TARGET.into(),
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
            lifetime_params: vec![],
            copy_constructor: ir_public_trivial_special(),
            move_constructor: ir_public_trivial_special(),
            destructor: ir_public_trivial_special(),
            is_trivial_abi: true,
        })])?;

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
        let ir = ir_from_cc("struct Trivial {};")?;
        let rs_api = generate_rs_api(&ir)?;
        assert!(!rs_api.contains("impl Drop"));
        Ok(())
    }

    /// User-defined destructors *must* become Drop impls with ManuallyDrop
    /// fields
    #[test]
    fn test_impl_drop_user_defined_destructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"struct UserDefinedDestructor {
                ~UserDefinedDestructor();
                int x;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert!(rs_api.contains("impl Drop"));
        assert!(!rs_api.contains("fn drop(&mut self) {}"));
        assert!(rs_api.contains("pub x: std::mem::ManuallyDrop<i32>,"));
        Ok(())
    }

    /// nontrivial types without user-defined destructors must have an empty
    /// drop definition.
    #[test]
    fn test_impl_drop_nontrivial_member_destructor() -> Result<()> {
        // TODO(jeanpierreda): This would be cleaner if the UserDefinedDestructor code were
        // omitted. For example, we simulate it so that UserDefinedDestructor
        // comes from another library.
        let ir = ir_from_cc(
            r#"struct UserDefinedDestructor {
                ~UserDefinedDestructor();
            };

            struct NontrivialMembers {
                UserDefinedDestructor udd;
                int x;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert!(rs_api.contains("fn drop(&mut self) {}"));
        assert!(rs_api.contains("pub x: i32,"));
        Ok(())
    }

    /// Trivial types (at least those that are mapped to Copy rust types) do not
    /// get a Drop impl.
    #[test]
    fn test_impl_drop_trivial() -> Result<()> {
        let ir = ir_from_cc(
            r#"struct Trivial {
                ~Trivial() = default;
                int x;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert!(!rs_api.contains("impl Drop"));
        assert!(rs_api.contains("pub x: i32,"));
        Ok(())
    }

    #[test]
    fn test_thunk_ident_function() {
        let func = ir_func("foo");
        assert_eq!(thunk_ident(&func), make_ident("__rust_thunk___Z3foov"));
    }

    #[test]
    fn test_thunk_ident_special_names() {
        let ir = ir_from_cc("struct Class {};").unwrap();

        let destructor =
            ir.functions().find(|f| f.name == UnqualifiedIdentifier::Destructor).unwrap();
        assert_eq!(thunk_ident(&destructor), make_ident("__rust_thunk___ZN5ClassD1Ev"));

        let constructor =
            ir.functions().find(|f| f.name == UnqualifiedIdentifier::Constructor).unwrap();
        assert_eq!(thunk_ident(&constructor), make_ident("__rust_thunk___ZN5ClassC1Ev"));
    }

    #[test]
    fn test_elided_lifetimes() {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
          struct S {
            int& f(int& i);
          };"#,
        )
        .unwrap();
        let func = retrieve_func(&ir, "f");
        let (api, thunk) = generate_func(&func, &ir).unwrap();
        assert_code_contains(
            &api.tokens,
            "pub fn f<'a, 'b>(__this: &'b mut S, i: &'a mut i32) -> &'b mut i32",
        );
        assert_code_contains(
            &thunk.tokens,
            "pub(crate) fn __rust_thunk___ZN1S1fERi<'a, 'b>(__this: &'b mut S, i: &'a mut i32) -> &'b mut i32",
        );
    }
}
