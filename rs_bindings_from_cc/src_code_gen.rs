// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{anyhow, bail, Context, Result};
use ffi_types::*;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::format_ident;
use quote::quote;
use std::collections::{BTreeSet, HashMap};
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
    let rs_api =
        format!("#![rustfmt::skip]\n{}", rs_tokens_to_formatted_string(generate_rs_api(&ir)?)?);
    let rs_api_impl = tokens_to_string(generate_rs_api_impl(&ir)?)?;

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

    true
}

/// Generates Rust source code for a given `Func`.
///
/// Returns the generated function or trait impl, and the thunk, as a tuple.
fn generate_func(func: &Func, ir: &IR) -> Result<(RsSnippet, RsSnippet)> {
    let empty_result = Ok((quote! {}.into(), quote! {}.into()));
    let mangled_name = &func.mangled_name;
    let thunk_ident = thunk_ident(func);
    let doc_comment = generate_doc_comment(&func.doc_comment);
    let lifetime_to_name = HashMap::<LifetimeId, String>::from_iter(
        func.lifetime_params.iter().map(|l| (l.id, l.name.clone())),
    );
    let return_type_fragment = if func.return_type.rs_type.is_unit_type() {
        quote! {}
    } else {
        let return_type_name = format_rs_type(&func.return_type.rs_type, ir, &lifetime_to_name)
            .with_context(|| format!("Failed to format return type for {:?}", func))?;
        quote! { -> #return_type_name }
    };

    let param_idents =
        func.params.iter().map(|p| make_ident(&p.identifier.identifier)).collect_vec();

    let param_types = func
        .params
        .iter()
        .map(|p| {
            format_rs_type(&p.type_.rs_type, ir, &lifetime_to_name).with_context(|| {
                format!("Failed to format type for parameter {:?} on {:?}", p, func)
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let lifetimes = func
        .lifetime_params
        .iter()
        .map(|l| syn::Lifetime::new(&format!("'{}", l.name), proc_macro2::Span::call_site()));
    let generic_params = format_generic_params(lifetimes);

    let record: Option<&Record> =
        func.member_func_metadata.as_ref().map(|meta| meta.find_record(ir)).transpose()?;

    let mut calls_thunk = true;
    let api_func = match &func.name {
        UnqualifiedIdentifier::Identifier(id) => {
            let ident = make_ident(&id.identifier);
            let fn_def = quote! {
                #doc_comment
                #[inline(always)]
                pub fn #ident #generic_params( #( #param_idents: #param_types ),*
                ) #return_type_fragment {
                    unsafe { crate::detail::#thunk_ident( #( #param_idents ),* ) }
                }
            };
            match &func.member_func_metadata {
                None => fn_def,
                Some(meta) => {
                    let type_name = make_ident(&meta.find_record(ir)?.identifier.identifier);
                    quote! { impl #type_name { #fn_def } }
                }
            }
        }

        UnqualifiedIdentifier::Destructor => {
            let record = record.ok_or_else(|| anyhow!("Destructors must be member functions."))?;
            let type_name = make_ident(&record.identifier.identifier);
            match record.destructor.definition {
                // TODO(b/202258760): Only omit destructor if `Copy` is specified.
                SpecialMemberDefinition::Trivial => {
                    return empty_result;
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
                    // Note: to avoid double-destruction of the fields, they are all wrapped in
                    // ManuallyDrop in this case. See `generate_record`.
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

        UnqualifiedIdentifier::Constructor => {
            let record = record.ok_or_else(|| anyhow!("Constructors must be member functions."))?;
            if !record.is_trivial_abi {
                return empty_result;
            }
            let type_name = make_ident(&record.identifier.identifier);
            match func.params.len() {
                0 => bail!("Constructor should have at least 1 parameter (__this)"),
                1 => quote! {
                    #doc_comment
                    impl Default for #type_name {
                        #[inline(always)]
                        fn default() -> Self {
                            let mut tmp = std::mem::MaybeUninit::<Self>::uninit();
                            unsafe {
                                crate::detail::#thunk_ident(tmp.as_mut_ptr());
                                tmp.assume_init()
                            }
                        }
                    }
                },
                _ => {
                    // TODO(b/208946210): Map some of these constructors to the From trait.
                    // TODO(b/200066396): Map other constructors (to the Clone trait?).
                    return empty_result;
                }
            }
        }
    };

    let thunk = if calls_thunk {
        let thunk_attr = if can_skip_cc_thunk(func) {
            quote! {#[link_name = #mangled_name]}
        } else {
            quote! {}
        };

        quote! {
            #thunk_attr
            pub(crate) fn #thunk_ident #generic_params( #( #param_idents: #param_types ),*
            ) #return_type_fragment ;
        }
    } else {
        quote! {}
    };

    Ok((api_func.into(), thunk.into()))
}

fn generate_doc_comment(comment: &Option<String>) -> TokenStream {
    match comment {
        Some(text) => {
            // token_stream_printer (and rustfmt) don't put a space between /// and the doc
            // comment, let's add it here so our comments are pretty.
            let doc = format!(" {}", text.replace("\n", "\n "));
            quote! {#[doc=#doc]}
        }
        None => quote! {},
    }
}

fn format_generic_params<T: quote::ToTokens>(params: impl IntoIterator<Item = T>) -> TokenStream {
    let mut params = params.into_iter().peekable();
    if params.peek().is_none() {
        quote! {}
    } else {
        quote! { < #( #params ),* > }
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
            let mut formatted = format_rs_type(&f.type_.rs_type, ir, &HashMap::new())
                .with_context(|| {
                    format!("Failed to format type for field {:?} on record {:?}", f, record)
                })?;
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
    if record.is_unpin() {
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
          placeholder: std::mem::MaybeUninit<u8>,
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

fn generate_rs_api(ir: &IR) -> Result<TokenStream> {
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

    Ok(quote! {
        #features __NEWLINE__ __NEWLINE__
        #imports __NEWLINE__ __NEWLINE__

        #( #items __NEWLINE__ __NEWLINE__ )*

        #mod_detail __NEWLINE__ __NEWLINE__

         #( #assertions __NEWLINE__ __NEWLINE__ )*
    })
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
        let record = ir
            .record_for_type(ty)
            .with_context(|| format!("Failed to format Rust type {:?}", ty))?;
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
            let type_args = format_generic_params(
                ty.type_args
                    .iter()
                    .map(|type_arg| format_rs_type(type_arg, ir, lifetime_to_name))
                    .collect::<Result<Vec<_>>>()?,
            );
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
        let ident = make_ident(
            ir.record_for_type(ty)
                .with_context(|| format!("Failed to format C++ type {:?}", ty))?
                .identifier
                .identifier
                .as_str(),
        );
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
    let field_assertions =
        record.fields.iter().filter(|f| f.access == AccessSpecifier::Public).map(|field| {
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

fn generate_rs_api_impl(ir: &IR) -> Result<TokenStream> {
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
                let fn_ident = make_ident(&id.identifier);
                let static_method_metadata = func
                    .member_func_metadata
                    .as_ref()
                    .filter(|meta| meta.instance_method_metadata.is_none());
                match static_method_metadata {
                    None => quote! {#fn_ident},
                    Some(meta) => {
                        let record_ident = make_ident(&meta.find_record(ir)?.identifier.identifier);
                        quote! { #record_ident :: #fn_ident }
                    }
                }
            }
            // Use destroy_at to avoid needing to spell out the class name. Destructor identiifers
            // use the name of the type itself, without namespace qualification, template
            // parameters, or aliases. We do not need to use that naming scheme anywhere else in
            // the bindings, and it can be difficult (impossible?) to spell in the general case. By
            // using destroy_at, we avoid needing to determine or remember what the correct spelling
            // is.
            UnqualifiedIdentifier::Constructor => {
                if func.params.len() == 1 {
                    quote! { rs_api_impl_support::construct_at }
                } else {
                    // TODO(b/208946210): Map some of these constructors to the From trait.
                    // TODO(b/200066396): Map other constructors (to the Clone trait?).
                    continue;
                }
            }
            UnqualifiedIdentifier::Destructor => quote! {std::destroy_at},
        };
        let return_type_name = format_cc_type(&func.return_type.cc_type, ir)?;
        let return_stmt = if func.return_type.cc_type.is_void() {
            quote! {}
        } else {
            quote! { return }
        };

        let param_idents =
            func.params.iter().map(|p| make_ident(&p.identifier.identifier)).collect_vec();

        let param_types = func
            .params
            .iter()
            .map(|p| format_cc_type(&p.type_.cc_type, ir))
            .collect::<Result<Vec<_>>>()?;

        thunks.push(quote! {
            extern "C" #return_type_name #thunk_ident( #( #param_types #param_idents ),* ) {
                #return_stmt #implementation_function( #( #param_idents ),* );
            }
        });
    }

    let layout_assertions = ir.records().map(|record| cc_struct_layout_assertion(record, ir));

    let mut standard_headers = <BTreeSet<Ident>>::new();
    standard_headers.insert(make_ident("memory")); // ubiquitous.
    if ir.records().next().is_some() {
        standard_headers.insert(make_ident("cstddef"));
    };

    let mut includes =
        vec!["rs_bindings_from_cc/support/cxx20_backports.h"];

    // In order to generate C++ thunk in all the cases Clang needs to be able to
    // access declarations from public headers of the C++ library.
    includes.extend(ir.used_headers().map(|i| &i.name as &str));

    Ok(quote! {
        #( __HASH_TOKEN__ include <#standard_headers> __NEWLINE__)*
        #( __HASH_TOKEN__ include #includes __NEWLINE__)* __NEWLINE__

        #( #thunks )* __NEWLINE__ __NEWLINE__

        #( #layout_assertions __NEWLINE__ __NEWLINE__ )*

        // To satisfy http://cs/symbol:devtools.metadata.Presubmit.CheckTerminatingNewline check.
        __NEWLINE__
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use ir_testing::{ir_from_cc, ir_from_cc_dependency, ir_func, ir_record};
    use token_stream_matchers::{
        assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
    };
    use token_stream_printer::tokens_to_string;

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
        let ir = ir_from_cc("int Add(int a, int b);")?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn Add(a: i32, b: i32) -> i32 {
                    unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z3Addii"]
                        pub(crate) fn __rust_thunk___Z3Addii(a: i32, b: i32) -> i32;
                    }
                }
            }
        );

        assert_cc_not_matches!(generate_rs_api_impl(&ir)?, quote! {__rust_thunk___Z3Addii});

        Ok(())
    }

    #[test]
    fn test_inline_function() -> Result<()> {
        let ir = ir_from_cc("inline int Add(int a, int b);")?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn Add(a: i32, b: i32) -> i32 {
                    unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk___Z3Addii(a: i32, b: i32) -> i32;
                    }
                }
            }
        );

        assert_cc_matches!(
            generate_rs_api_impl(&ir)?,
            quote! {
                extern "C" int __rust_thunk___Z3Addii(int a, int b) {
                    return Add(a, b);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_simple_function_with_types_from_other_target() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "inline ReturnStruct DoSomething(ParamStruct param);",
            "struct ReturnStruct {}; struct ParamStruct {};",
        )?;

        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn DoSomething(param: dependency::ParamStruct)
                    -> dependency::ReturnStruct {
                    unsafe { crate::detail::__rust_thunk___Z11DoSomething11ParamStruct(param) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
            mod detail {
                use super::*;
                extern "C" {
                    pub(crate) fn __rust_thunk___Z11DoSomething11ParamStruct(param: dependency::ParamStruct)
                        -> dependency::ReturnStruct;
                }
            }}
        );

        assert_cc_matches!(
            generate_rs_api_impl(&ir)?,
            quote! {
                extern "C" ReturnStruct __rust_thunk___Z11DoSomething11ParamStruct(ParamStruct param) {
                    return DoSomething(param);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_simple_struct() -> Result<()> {
        let ir = ir_from_cc(&tokens_to_string(quote! {
            struct SomeStruct {
                int public_int;
              protected:
                int protected_int;
              private:
               int private_int;
            };
        })?)?;

        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C)]
                pub struct SomeStruct {
                    pub public_int: i32,
                    protected_int: i32,
                    private_int: i32,
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
                const _: () = assert!(std::mem::size_of::<SomeStruct>() == 12usize);
                const _: () = assert!(std::mem::align_of::<SomeStruct>() == 4usize);
                const _: () = assert!(offset_of!(SomeStruct, public_int) * 8 == 0usize);
                const _: () = assert!(offset_of!(SomeStruct, protected_int) * 8 == 32usize);
                const _: () = assert!(offset_of!(SomeStruct, private_int) * 8 == 64usize);
            }
        );
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN10SomeStructD1Ev(SomeStruct * __this) {
                    std :: destroy_at (__this) ;
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                static_assert(sizeof(SomeStruct) == 12);
                static_assert(alignof(SomeStruct) == 4);
                static_assert(offsetof(SomeStruct, public_int) * 8 == 0);
            }
        );
        Ok(())
    }

    #[test]
    fn test_record_static_methods_qualify_call_in_thunk() -> Result<()> {
        let ir = ir_from_cc(&tokens_to_string(quote! {
            struct SomeStruct {
                static inline int some_func() { return 42; }
            };
        })?)?;

        assert_cc_matches!(
            generate_rs_api_impl(&ir)?,
            quote! {
                extern "C" int __rust_thunk___ZN10SomeStruct9some_funcEv() {
                    return SomeStruct::some_func();
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_struct_from_other_target() -> Result<()> {
        let ir = ir_from_cc_dependency("// intentionally empty", "struct SomeStruct {};")?;
        assert_rs_not_matches!(generate_rs_api(&ir)?, quote! { SomeStruct });
        assert_cc_not_matches!(generate_rs_api_impl(&ir)?, quote! { SomeStruct });
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

    /// A type can be unsafe to pass in mut references from C++, but still
    /// Clone+Copy when handled by value.
    #[test]
    fn test_copy_derives_not_is_mut_reference_safe() {
        let mut record = ir_record("S");
        record.is_final = false;
        assert_eq!(generate_copy_derives(&record), &["Clone", "Copy"]);
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
        let ir = ir_from_cc(&tokens_to_string(quote! {
            inline int* Deref(int*const* p);
        })?)?;

        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn Deref(p: *const *mut i32) -> *mut i32 {
                    unsafe { crate::detail::__rust_thunk___Z5DerefPKPi(p) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk___Z5DerefPKPi(p: *const *mut i32) -> *mut i32;
                    }
                }
            }
        );

        assert_cc_matches!(
            generate_rs_api_impl(&ir)?,
            quote! {
                extern "C" int* __rust_thunk___Z5DerefPKPi(int* const * p) {
                    return Deref(p);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_item_order() -> Result<()> {
        let ir = ir_from_cc(
            "int first_func();
             struct FirstStruct {};
             int second_func();
             struct SecondStruct {};",
        )?;

        let rs_api = rs_tokens_to_formatted_string(generate_rs_api(&ir)?)?;

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
        let ir = ir_from_cc(
            "
        // Doc Comment
        // with two lines
        int func();",
        )?;

        assert_rs_matches!(
            generate_rs_api(&ir)?,
            // leading space is intentional so there is a space between /// and the text of the
            // comment
            quote! {
                #[doc = " Doc Comment\n with two lines"]
                #[inline(always)]
                pub fn func
            }
        );

        Ok(())
    }

    #[test]
    fn test_doc_comment_record() -> Result<()> {
        let ir = ir_from_cc(
            "// Doc Comment\n\
            //\n\
            //  * with bullet\n\
            struct SomeStruct {\n\
                // Field doc\n\
                int field;\
            };",
        )?;

        assert_rs_matches!(
            generate_rs_api(&ir)?,
            quote! {
                #[doc = " Doc Comment\n \n  * with bullet"]
                #[derive(Clone, Copy)]
                #[repr(C)]
                pub struct SomeStruct {
                    # [doc = " Field doc"]
                    pub field: i32,
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_virtual_thunk() -> Result<()> {
        let ir = ir_from_cc("struct Polymorphic { virtual void Foo(); };")?;

        assert_cc_matches!(
            generate_rs_api_impl(&ir)?,
            quote! {
                extern "C" void __rust_thunk___ZN11Polymorphic3FooEv(Polymorphic * __this)
            }
        );
        Ok(())
    }

    /// A trivially relocatable final struct is safe to use in Rust as normal,
    /// and is Unpin.
    #[test]
    fn test_no_negative_impl_unpin() -> Result<()> {
        let ir = ir_from_cc("struct Trivial final {};")?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_not_matches!(rs_api, quote! {impl !Unpin});
        Ok(())
    }

    /// A non-final struct, even if it's trivial, is not usable by mut
    /// reference, and so is !Unpin.
    #[test]
    fn test_negative_impl_unpin_nonfinal() -> Result<()> {
        let ir = ir_from_cc("struct Nonfinal {};")?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(rs_api, quote! {impl !Unpin for Nonfinal {}});
        Ok(())
    }

    /// At the least, a trivial type should have no drop impl if or until we add
    /// empty drop impls.
    #[test]
    fn test_no_impl_drop() -> Result<()> {
        let ir = ir_from_cc("struct Trivial {};")?;
        let rs_api = rs_tokens_to_formatted_string(generate_rs_api(&ir)?)?;
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
        println!("{}", rs_api);
        assert_rs_matches!(rs_api, quote! {impl Drop});
        assert_rs_not_matches!(rs_api, quote! {fn drop(&mut self) {}});
        assert_rs_matches!(rs_api, quote! {pub x: std::mem::ManuallyDrop<i32>,});
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
        assert_rs_matches!(rs_api, quote! {fn drop(&mut self) {}});
        assert_rs_matches!(rs_api, quote! {pub x: i32,});
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
        assert_rs_not_matches!(rs_api, quote! {impl Drop});
        assert_rs_matches!(rs_api, quote! {pub x: i32});
        Ok(())
    }

    #[test]
    fn test_impl_default_explicitly_defaulted_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"struct DefaultedConstructor {
                DefaultedConstructor() = default;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl Default for DefaultedConstructor {
                    #[inline(always)]
                    fn default() -> Self {
                        let mut tmp = std::mem::MaybeUninit::<Self>::uninit();
                        unsafe {
                            crate::detail::__rust_thunk___ZN20DefaultedConstructorC1Ev(
                                tmp.as_mut_ptr());
                            tmp.assume_init()
                        }
                    }
                }
            }
        );
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN20DefaultedConstructorC1Ev(
                        DefaultedConstructor* __this) {
                    rs_api_impl_support::construct_at (__this) ;
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_impl_default_non_trivial_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"struct NonTrivialStructWithConstructors {
                NonTrivialStructWithConstructors();
                ~NonTrivialStructWithConstructors();  // Non-trivial
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_not_matches!(rs_api, quote! {impl Default});
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
    fn test_elided_lifetimes() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
          struct S {
            int& f(int& i);
          };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn f<'a, 'b>(__this: &'b mut S, i: &'a mut i32) -> &'b mut i32 { ... }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) fn __rust_thunk___ZN1S1fERi<'a, 'b>(__this: &'b mut S, i: &'a mut i32)
                    -> &'b mut i32;
            }
        );
        Ok(())
    }

    #[test]
    fn test_format_generic_params() -> Result<()> {
        assert_rs_matches!(format_generic_params(std::iter::empty::<syn::Ident>()), quote! {});

        let idents = ["T1", "T2"].iter().map(|s| make_ident(s));
        assert_rs_matches!(format_generic_params(idents), quote! { < T1, T2 > });

        let lifetimes = ["a", "b"]
            .iter()
            .map(|s| syn::Lifetime::new(&format!("'{}", s), proc_macro2::Span::call_site()));
        assert_rs_matches!(format_generic_params(lifetimes), quote! { < 'a, 'b > });

        Ok(())
    }
}
