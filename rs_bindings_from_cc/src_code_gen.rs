// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
#[macro_use]
extern crate static_assertions;

use anyhow::{anyhow, bail, ensure, Context, Result};
use ffi_types::*;
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::format_ident;
use quote::quote;
use std::collections::{BTreeSet, HashMap, HashSet};
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
/// # Safety
///
/// Expectations:
///    * function expects that param `json` is a FfiU8Slice for a valid array of
///      bytes with the given size.
///    * function expects that param `json` doesn't change during the call.
///
/// Ownership:
///    * function doesn't take ownership of (in other words it borrows) the
///      param `json`
///    * function passes ownership of the returned value to the caller
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
    // downstream workflows from reformatting with a different configuration by
    // marking the output with `@generated`. See also
    // https://rust-lang.github.io/rustfmt/?version=v1.4.38&search=#format_generated_files
    //
    // TODO(lukasza): It would be nice to include "by $argv[0]"" in the
    // @generated comment below.  OTOH, `std::env::current_exe()` in our
    // current build environment returns a guid-like path... :-/
    //
    // TODO(lukasza): Try to remove `#![rustfmt:skip]` - in theory it shouldn't
    // be needed when `@generated` comment/keyword is present...
    let rs_api = format!(
        "// Automatically @generated Rust bindings for C++ target\n\
        // {target}\n\
        #![rustfmt::skip]\n\
        {code}",
        target = ir.current_target().0,
        code = rs_tokens_to_formatted_string(generate_rs_api(&ir)?)?
    );
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
///   features: btree_set![make_rs_ident("vec_into_raw_parts")],
///   tokens: quote!{vec![].into_raw_parts()},
/// }
/// ```
#[derive(Clone, Debug)]
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

/// Uniquely identifies a generated Rust function.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FunctionId {
    // If the function is on a trait impl, contains the name of the Self type for
    // which the trait is being implemented.
    self_type: Option<syn::Path>,
    // Fully qualified path of the function. For functions in impl blocks, this
    // includes the name of the type or trait on which the function is being
    // implemented, e.g. `Default::default`.
    function_path: syn::Path,
}

/// Returns the name of `func` in C++ syntax.
fn cxx_function_name(func: &Func, ir: &IR) -> Result<String> {
    let record: Option<&str> = func
        .member_func_metadata
        .as_ref()
        .map(|meta| meta.find_record(ir))
        .transpose()?
        .map(|r| &*r.cc_name);

    let func_name = match &func.name {
        UnqualifiedIdentifier::Identifier(id) => id.identifier.clone(),
        UnqualifiedIdentifier::Operator(op) => op.cc_name(),
        UnqualifiedIdentifier::Destructor => {
            format!("~{}", record.expect("destructor must be associated with a record"))
        }
        UnqualifiedIdentifier::Constructor => {
            record.expect("constructor must be associated with a record").to_string()
        }
    };

    if let Some(record_name) = record {
        Ok(format!("{}::{}", record_name, func_name))
    } else {
        Ok(func_name)
    }
}

fn make_unsupported_fn(func: &Func, ir: &IR, message: impl ToString) -> Result<UnsupportedItem> {
    Ok(UnsupportedItem {
        name: cxx_function_name(func, ir)?,
        message: message.to_string(),
        source_loc: func.source_loc.clone(),
    })
}

#[derive(Clone, Debug)]
enum GeneratedFunc {
    None, // No explicit function needed (e.g. when deriving Drop).
    Unsupported(UnsupportedItem),
    Some { api_func: RsSnippet, thunk: RsSnippet, function_id: FunctionId },
}

/// Generates Rust source code for a given `Func`.
fn generate_func(func: &Func, ir: &IR) -> Result<GeneratedFunc> {
    let make_unsupported_result = |msg: &str| -> Result<GeneratedFunc> {
        Ok(GeneratedFunc::Unsupported(make_unsupported_fn(func, ir, msg)?))
    };

    let mangled_name = &func.mangled_name;
    let thunk_ident = thunk_ident(func);
    let doc_comment = generate_doc_comment(&func.doc_comment);
    let lifetime_to_name = HashMap::<LifetimeId, String>::from_iter(
        func.lifetime_params.iter().map(|l| (l.id, l.name.clone())),
    );
    let return_type_fragment = RsTypeKind::new(&func.return_type.rs_type, ir)
        .and_then(|t| t.format_as_return_type_fragment(ir, &lifetime_to_name))
        .with_context(|| format!("Failed to format return type for {:?}", func))?;

    let param_idents =
        func.params.iter().map(|p| make_rs_ident(&p.identifier.identifier)).collect_vec();

    let param_type_kinds = func
        .params
        .iter()
        .map(|p| {
            RsTypeKind::new(&p.type_.rs_type, ir).with_context(|| {
                format!("Failed to process type of parameter {:?} on {:?}", p, func)
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let param_types = param_type_kinds
        .iter()
        .map(|t| {
            t.format(ir, &lifetime_to_name)
                .with_context(|| format!("Failed to format parameter type {:?} on {:?}", t, func))
        })
        .collect::<Result<Vec<_>>>()?;
    let is_unsafe = param_type_kinds.iter().any(|p| matches!(p, RsTypeKind::Pointer { .. }))
        && func.name != UnqualifiedIdentifier::Destructor;

    let maybe_record: Option<&Record> =
        func.member_func_metadata.as_ref().map(|meta| meta.find_record(ir)).transpose()?;
    let maybe_record_name = maybe_record.map(|r| make_rs_ident(&r.rs_name));

    // Find 1) the `func_name` and `impl_kind` of the API function to generate
    // and 2) whether to `format_first_param_as_self` (`&self` or `&mut self`).
    enum TraitName {
        UnpinConstructor(TokenStream), // An Unpin constructor trait, e.g. From or Clone.
        Other(TokenStream),            // any other trait, e.g. Eq.
    }
    impl quote::ToTokens for TraitName {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                Self::UnpinConstructor(t) | Self::Other(t) => t.to_tokens(tokens),
            }
        }
    }
    enum ImplKind {
        None,   // No `impl` needed
        Struct, // e.g. `impl SomeStruct { ... }` (SomeStruct based on func.member_func_metadata)
        Trait {
            // Note that `record_name` might *not* be from
            // `func.member_func_metadata`.
            record_name: Ident,    // e.g. SomeStruct
            trait_name: TraitName, // e.g. quote!{ From<i32> }

            // Where to declare lifetimes: `impl<'b>` VS `fn foo<'b>`.
            declare_lifetimes: bool,
            // The generic params of trait `impl` (e.g. `<'b>`). These start
            // empty and only later are mutated into the correct value.
            trait_generic_params: TokenStream,
        },
    }
    impl ImplKind {
        fn new_trait(trait_name: TraitName, record_name: Ident) -> Self {
            ImplKind::Trait {
                trait_name,
                record_name,
                declare_lifetimes: false,
                trait_generic_params: quote! {},
            }
        }
        fn new_generic_trait(trait_name: TraitName, record_name: Ident) -> Self {
            ImplKind::Trait {
                trait_name,
                record_name,
                declare_lifetimes: true,
                trait_generic_params: quote! {},
            }
        }
    }
    let mut impl_kind: ImplKind;
    let func_name: syn::Ident;
    let format_first_param_as_self: bool;
    match &func.name {
        UnqualifiedIdentifier::Operator(op) if op.name == "==" => {
            if param_type_kinds.len() != 2 {
                bail!("Unexpected number of parameters in operator==: {:?}", func);
            }
            match (&param_type_kinds[0], &param_type_kinds[1]) {
                (
                    RsTypeKind::Reference { referent: lhs, mutability: Mutability::Const, .. },
                    RsTypeKind::Reference { referent: rhs, mutability: Mutability::Const, .. },
                ) => match **lhs {
                    RsTypeKind::Record(lhs_record) => {
                        let lhs: Ident = make_rs_ident(&lhs_record.rs_name);
                        let rhs: TokenStream = rhs.format(ir, &lifetime_to_name)?;
                        format_first_param_as_self = true;
                        func_name = make_rs_ident("eq");
                        // Not using `ImplKind::new_generic_trait`, because #rhs
                        // should be stripped of references + because `&'a self`
                        // needs to have its lifetime declared next to `fn`, not
                        // next to `impl`.
                        impl_kind =
                            ImplKind::new_trait(TraitName::Other(quote! {PartialEq<#rhs>}), lhs);
                    }
                    _ => {
                        return make_unsupported_result(
                            "operator== where lhs doesn't refer to a record",
                        );
                    }
                },
                _ => {
                    return make_unsupported_result(
                        "operator== where operands are not const references",
                    );
                }
            };
        }
        UnqualifiedIdentifier::Operator(_) => {
            return make_unsupported_result("Bindings for this kind of operator are not supported");
        }
        UnqualifiedIdentifier::Identifier(id) => {
            func_name = make_rs_ident(&id.identifier);
            match maybe_record {
                None => {
                    impl_kind = ImplKind::None;
                    format_first_param_as_self = false;
                }
                Some(record) => {
                    impl_kind = ImplKind::Struct;
                    if func.is_instance_method() {
                        let first_param = param_type_kinds.first().ok_or_else(|| {
                            anyhow!("Missing `__this` parameter in an instance method: {:?}", func)
                        })?;
                        format_first_param_as_self = first_param.is_ref_to(record)
                    } else {
                        format_first_param_as_self = false;
                    }
                }
            };
        }
        UnqualifiedIdentifier::Destructor => {
            // Note: to avoid double-destruction of the fields, they are all wrapped in
            // ManuallyDrop in this case. See `generate_record`.
            let record =
                maybe_record.ok_or_else(|| anyhow!("Destructors must be member functions."))?;
            if !should_implement_drop(record) {
                return Ok(GeneratedFunc::None);
            }
            let record_name = maybe_record_name
                .clone()
                .ok_or_else(|| anyhow!("Destructors must be member functions."))?;
            impl_kind = ImplKind::new_trait(TraitName::Other(quote! {Drop}), record_name);
            func_name = make_rs_ident("drop");
            format_first_param_as_self = true;
        }
        UnqualifiedIdentifier::Constructor => {
            let member_func_metadata = func
                .member_func_metadata
                .as_ref()
                .ok_or_else(|| anyhow!("Constructors must be member functions."))?;
            let record = maybe_record
                .ok_or_else(|| anyhow!("Constructors must be associated with a record."))?;
            let instance_method_metadata =
                member_func_metadata
                    .instance_method_metadata
                    .as_ref()
                    .ok_or_else(|| anyhow!("Constructors must be instance methods."))?;

            if !record.is_unpin() {
                // TODO: Handle <internal link>
                return make_unsupported_result(
                    "Bindings for constructors of non-trivial types are not supported yet",
                );
            }
            if is_unsafe {
                // TODO(b/216648347): Allow this outside of traits (e.g. after supporting
                // translating C++ constructors into static methods in Rust).
                return make_unsupported_result(
                    "Unsafe constructors (e.g. with no elided or explicit lifetimes) \
                    are intentionally not supported",
                );
            }

            let record_name = maybe_record_name
                .clone()
                .ok_or_else(|| anyhow!("Constructors must be member functions."))?;
            match func.params.len() {
                0 => bail!("Missing `__this` parameter in a constructor: {:?}", func),
                1 => {
                    impl_kind = ImplKind::new_trait(
                        TraitName::UnpinConstructor(quote! {Default}),
                        record_name,
                    );
                    func_name = make_rs_ident("default");
                    format_first_param_as_self = false;
                }
                2 => {
                    // TODO(lukasza): Do something smart with move constructor.
                    if param_type_kinds[1].is_shared_ref_to(record) {
                        // Copy constructor
                        if should_derive_clone(record) {
                            return Ok(GeneratedFunc::None);
                        } else {
                            impl_kind = ImplKind::new_trait(
                                TraitName::UnpinConstructor(quote! {Clone}),
                                record_name,
                            );
                            func_name = make_rs_ident("clone");
                            format_first_param_as_self = true;
                        }
                    } else if !instance_method_metadata.is_explicit_ctor {
                        let param_type = &param_types[1];
                        impl_kind = ImplKind::new_generic_trait(
                            TraitName::UnpinConstructor(quote! {From< #param_type >}),
                            record_name,
                        );
                        func_name = make_rs_ident("from");
                        format_first_param_as_self = false;
                    } else {
                        return make_unsupported_result(
                            "Not yet supported type of constructor parameter",
                        );
                    }
                }
                _ => {
                    // TODO(b/216648347): Support bindings for other constructors.
                    return make_unsupported_result(
                        "More than 1 constructor parameter is not supported yet",
                    );
                }
            }
        }
    }

    let api_func_def = {
        // Clone params, return type, etc - we may need to mutate them in the
        // API func, but we want to retain the originals for the thunk.
        let mut return_type_fragment = return_type_fragment.clone();
        let mut thunk_args = param_idents.iter().map(|id| quote! { #id}).collect_vec();
        let mut api_params = param_idents
            .iter()
            .zip(param_types.iter())
            .map(|(ident, type_)| quote! { #ident : #type_ })
            .collect_vec();
        let mut lifetimes = func.lifetime_params.iter().collect_vec();
        let mut maybe_first_api_param = param_type_kinds.get(0);

        if let ImplKind::Trait { trait_name: TraitName::UnpinConstructor(..), .. } = impl_kind {
            return_type_fragment = quote! { -> Self };

            // Drop `__this` parameter from the public Rust API. Presence of
            // element #0 is indirectly verified by a `Constructor`-related
            // `match` branch a little bit above.
            api_params.remove(0);
            thunk_args.remove(0);

            // Remove the lifetime associated with `__this`.
            ensure!(
                func.return_type.rs_type.is_unit_type(),
                "Unexpectedly non-void return type of a constructor: {:?}",
                func
            );
            let maybe_first_lifetime = func.params[0].type_.rs_type.lifetime_args.first();
            let no_longer_needed_lifetime_id = maybe_first_lifetime
                .ok_or_else(|| anyhow!("Missing lifetime on `__this` parameter: {:?}", func))?;
            lifetimes.retain(|l| l.id != *no_longer_needed_lifetime_id);
            if let Some(type_still_dependent_on_removed_lifetime) = param_type_kinds
                .iter()
                .skip(1) // Skipping `__this`
                .flat_map(|t| t.lifetimes())
                .find(|lifetime_id| *lifetime_id == *no_longer_needed_lifetime_id)
            {
                bail!(
                    "The lifetime of `__this` is unexpectedly also used by another \
                    parameter {:?} in function {:?}",
                    type_still_dependent_on_removed_lifetime,
                    func.name
                );
            }

            // Rebind `maybe_first_api_param` to the next param after `__this`.
            maybe_first_api_param = param_type_kinds.get(1);
        }

        // Change `__this: &'a SomeStruct` into `&'a self` if needed.
        if format_first_param_as_self {
            let first_api_param = maybe_first_api_param
                .ok_or_else(|| anyhow!("No parameter to format as 'self': {:?}", func))?;
            let self_decl =
                first_api_param.format_as_self_param(func, ir, &lifetime_to_name).with_context(
                    || format!("Failed to format as `self` param: {:?}", first_api_param),
                )?;
            // Presence of element #0 is verified by `ok_or_else` on
            // `maybe_first_api_param` above.
            api_params[0] = self_decl;
            thunk_args[0] = quote! { self };
        }

        let func_body = match &impl_kind {
            ImplKind::Trait { trait_name: TraitName::UnpinConstructor(..), .. } => {
                // SAFETY: A user-defined constructor is not guaranteed to
                // initialize all the fields. To make the `assume_init()` call
                // below safe, the memory is zero-initialized first. This is a
                // bit safer, because zero-initialized memory represents a valid
                // value for the currently supported field types (this may
                // change once the bindings generator starts supporting
                // reference fields). TODO(b/213243309): Double-check if
                // zero-initialization is desirable here.
                quote! {
                    let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
                    unsafe {
                        crate::detail::#thunk_ident( &mut tmp #( , #thunk_args )* );
                        tmp.assume_init()
                    }
                }
            }
            _ => {
                let mut body = quote! { crate::detail::#thunk_ident( #( #thunk_args ),* ) };
                // Only need to wrap everything in an `unsafe { ... }` block if
                // the *whole* api function is safe.
                if !is_unsafe {
                    body = quote! { unsafe { #body } };
                }
                body
            }
        };

        let (pub_, unsafe_) = match impl_kind {
            ImplKind::None | ImplKind::Struct => (
                quote! { pub },
                if is_unsafe {
                    quote! {unsafe}
                } else {
                    quote! {}
                },
            ),
            ImplKind::Trait { .. } => {
                // Currently supported bindings have no unsafe trait functions.
                assert!(!is_unsafe);
                (quote! {}, quote! {})
            }
        };

        let lifetimes = lifetimes.into_iter().map(|l| format_lifetime_name(&l.name));
        let fn_generic_params: TokenStream;
        if let ImplKind::Trait { declare_lifetimes: true, trait_generic_params, .. } =
            &mut impl_kind
        {
            *trait_generic_params = format_generic_params(lifetimes);
            fn_generic_params = quote! {}
        } else {
            fn_generic_params = format_generic_params(lifetimes);
        }

        quote! {
            #[inline(always)]
            #pub_ #unsafe_ fn #func_name #fn_generic_params(
                    #( #api_params ),* ) #return_type_fragment {
                #func_body
            }
        }
    };

    let api_func: TokenStream;
    let function_id: FunctionId;
    match impl_kind {
        ImplKind::None => {
            api_func = quote! { #doc_comment #api_func_def };
            function_id = FunctionId { self_type: None, function_path: func_name.into() };
        }
        ImplKind::Struct => {
            let record_name =
                maybe_record_name.ok_or_else(|| anyhow!("Struct methods must have records"))?;
            api_func = quote! { impl #record_name { #doc_comment #api_func_def } };
            function_id = FunctionId {
                self_type: None,
                function_path: syn::parse2(quote! { #record_name :: #func_name })?,
            };
        }
        ImplKind::Trait { trait_name, record_name, trait_generic_params, .. } => {
            api_func = quote! {
                #doc_comment
                impl #trait_generic_params #trait_name for #record_name {
                    #api_func_def
                }
            };
            function_id = FunctionId {
                self_type: Some(record_name.into()),
                function_path: syn::parse2(quote! { #trait_name :: #func_name })?,
            };
        }
    }

    let thunk = {
        let thunk_attr = if can_skip_cc_thunk(func) {
            quote! {#[link_name = #mangled_name]}
        } else {
            quote! {}
        };

        // For constructors inject MaybeUninit into the type of `__this_` parameter.
        let mut param_types = param_types;
        if func.name == UnqualifiedIdentifier::Constructor {
            if param_types.is_empty() || func.params.is_empty() {
                bail!("Constructors should have at least one parameter (__this)");
            }
            param_types[0] = param_type_kinds[0]
                .format_mut_ref_as_uninitialized(ir, &lifetime_to_name)
                .with_context(|| {
                    format!(
                        "Failed to format `__this` param for a constructor thunk: {:?}",
                        func.params[0]
                    )
                })?;
        } else if func.name == UnqualifiedIdentifier::Destructor {
            if param_types.is_empty() || func.params.is_empty() {
                bail!("Destructors should have at least one parameter (__this)");
            }
            param_types[0] = param_type_kinds[0]
                .format_ref_as_raw_ptr(ir, &lifetime_to_name)
                .with_context(|| {
                    format!(
                        "Failed to format `__this` param for a destructor thunk: {:?}",
                        func.params[0]
                    )
                })?;
        }

        let lifetimes = func.lifetime_params.iter().map(|l| format_lifetime_name(&l.name));
        let generic_params = format_generic_params(lifetimes);

        quote! {
            #thunk_attr
            pub(crate) fn #thunk_ident #generic_params( #( #param_idents: #param_types ),*
            ) #return_type_fragment ;
        }
    };

    Ok(GeneratedFunc::Some { api_func: api_func.into(), thunk: thunk.into(), function_id })
}

fn generate_doc_comment(comment: &Option<String>) -> TokenStream {
    match comment {
        Some(text) => {
            // token_stream_printer (and rustfmt) don't put a space between /// and the doc
            // comment, let's add it here so our comments are pretty.
            let doc = format!(" {}", text.replace('\n', "\n "));
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

fn should_implement_drop(record: &Record) -> bool {
    match record.destructor.definition {
        // TODO(b/202258760): Only omit destructor if `Copy` is specified.
        SpecialMemberDefinition::Trivial => false,

        // TODO(b/212690698): Avoid calling into the C++ destructor (e.g. let
        // Rust drive `drop`-ing) to avoid (somewhat unergonomic) ManuallyDrop
        // if we can ask Rust to preserve C++ field destruction order in
        // NontrivialMembers case.
        SpecialMemberDefinition::NontrivialMembers => true,

        // The `impl Drop` for NontrivialUserDefined needs to call into the
        // user-defined destructor on C++ side.
        SpecialMemberDefinition::NontrivialUserDefined => true,

        // TODO(b/213516512): Today the IR doesn't contain Func entries for
        // deleted functions/destructors/etc. But, maybe we should generate
        // `impl Drop` in this case? With `unreachable!`? With
        // `std::mem::forget`?
        SpecialMemberDefinition::Deleted => false,
    }
}

/// Returns whether fields of type `ty` need to be wrapped in `ManuallyDrop<T>`
/// to prevent the fields from being destructed twice (once by the C++
/// destructor calkled from the `impl Drop` of the struct and once by `drop` on
/// the Rust side).
///
/// A type is safe to destroy twice if it implements `Copy`. Fields of such
/// don't need to be wrapped in `ManuallyDrop<T>` even if the struct
/// containing the fields provides an `impl Drop` that calles into a C++
/// destructor (in addition to dropping the fields on the Rust side).
///
/// Note that it is not enough to just be `!needs_drop<T>()`: Rust only
/// guarantees that it is safe to use-after-destroy for `Copy` types. See
/// e.g. the documentation for
/// [`drop_in_place`](https://doc.rust-lang.org/std/ptr/fn.drop_in_place.html):
///
/// > if `T` is not `Copy`, using the pointed-to value after calling
/// > `drop_in_place` can cause undefined behavior
fn needs_manually_drop(ty: &ir::RsType, ir: &IR) -> Result<bool> {
    let ty_implements_copy = RsTypeKind::new(ty, ir)?.implements_copy();
    Ok(!ty_implements_copy)
}

/// Generates Rust source code for a given `Record` and associated assertions as
/// a tuple.
fn generate_record(record: &Record, ir: &IR) -> Result<(RsSnippet, RsSnippet)> {
    let ident = make_rs_ident(&record.rs_name);
    let doc_comment = generate_doc_comment(&record.doc_comment);
    let field_idents =
        record.fields.iter().map(|f| make_rs_ident(&f.identifier.identifier)).collect_vec();
    let field_doc_coments =
        record.fields.iter().map(|f| generate_doc_comment(&f.doc_comment)).collect_vec();
    let field_types = record
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            // [[no_unique_address]] fields are replaced by an unaligned block of memory
            // which fills space up to the next field.
            // See: docs/struct_layout
            if f.is_no_unique_address {
                let next_offset = if let Some(next) = record.fields.get(i + 1) {
                    next.offset
                } else {
                    record.size * 8
                };
                let width = Literal::usize_unsuffixed((next_offset - f.offset) / 8);
                return Ok(quote! {[std::mem::MaybeUninit<u8>; #width]});
            }
            let mut formatted = format_rs_type(&f.type_.rs_type, ir, &HashMap::new())
                .with_context(|| {
                    format!("Failed to format type for field {:?} on record {:?}", f, record)
                })?;
            // TODO(b/212696226): Verify cases where ManuallyDrop<T> is skipped
            // via static asserts in the generated code.
            if should_implement_drop(record) && needs_manually_drop(&f.type_.rs_type, ir)? {
                // TODO(b/212690698): Avoid (somewhat unergonomic) ManuallyDrop
                // if we can ask Rust to preserve field destruction order if the
                // destructor is the SpecialMemberDefinition::NontrivialMembers
                // case.
                formatted = quote! { std::mem::ManuallyDrop<#formatted> }
            };
            Ok(formatted)
        })
        .collect::<Result<Vec<_>>>()?;
    let field_accesses = record
        .fields
        .iter()
        .map(|f| {
            if f.access == AccessSpecifier::Public && !f.is_no_unique_address {
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
    assertion_features.insert(make_rs_ident("const_ptr_offset_from"));

    let derives = generate_derives(record);
    let derives = if derives.is_empty() {
        quote! {}
    } else {
        quote! {#[derive( #(#derives),* )]}
    };
    let unpin_impl = if record.is_unpin() {
        quote! {}
    } else {
        // negative_impls are necessary for universal initialization due to Rust's
        // coherence rules: PhantomPinned isn't enough to prove to Rust that a
        // blanket impl that requires Unpin doesn't apply. See http://<internal link>=h.f6jp8ifzgt3n
        record_features.insert(make_rs_ident("negative_impls"));
        quote! {
            __NEWLINE__  __NEWLINE__
            impl !Unpin for #ident {}
        }
    };

    let mut repr_attributes = vec![quote! {C}];
    if record.override_alignment && record.alignment > 1 {
        let alignment = Literal::usize_unsuffixed(record.alignment);
        repr_attributes.push(quote! {align(#alignment)});
    }

    // Adjust the struct to also include base class subobjects. We use an opaque
    // field because subobjects can live in the alignment of base class
    // subobjects.
    let base_subobjects_field = if let Some(base_size) = record.base_size {
        let n = proc_macro2::Literal::usize_unsuffixed(base_size);
        quote! {
            __base_class_subobjects: [std::mem::MaybeUninit<u8>; #n],
        }
    } else {
        quote! {}
    };

    let empty_struct_placeholder_field =
        if record.fields.is_empty() && record.base_size.unwrap_or(0) == 0 {
            quote! {
              /// Prevent empty C++ struct being zero-size in Rust.
              placeholder: std::mem::MaybeUninit<u8>,
            }
        } else {
            quote! {}
        };

    let no_unique_address_accessors = cc_struct_no_unique_address_impl(record, ir)?;
    let base_class_into = cc_struct_upcast_impl(record, ir)?;

    let record_tokens = quote! {
        #doc_comment
        #derives
        #[repr(#( #repr_attributes ),*)]
        pub struct #ident {
            #base_subobjects_field
            #( #field_doc_coments #field_accesses #field_idents: #field_types, )*
            #empty_struct_placeholder_field
        }

        #no_unique_address_accessors

        #base_class_into

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

fn should_derive_clone(record: &Record) -> bool {
    record.is_unpin()
        && record.copy_constructor.access == ir::AccessSpecifier::Public
        && record.copy_constructor.definition == SpecialMemberDefinition::Trivial
}

fn should_derive_copy(record: &Record) -> bool {
    // TODO(b/202258760): Make `Copy` inclusion configurable.
    should_derive_clone(record)
}

fn generate_derives(record: &Record) -> Vec<Ident> {
    let mut derives = vec![];
    if should_derive_clone(record) {
        derives.push(make_rs_ident("Clone"));
    }
    if should_derive_copy(record) {
        derives.push(make_rs_ident("Copy"));
    }
    derives
}

fn generate_enum(enum_: &Enum, ir: &IR) -> Result<TokenStream> {
    let name = make_rs_ident(&enum_.identifier.identifier);
    let underlying_type = format_rs_type(&enum_.underlying_type.rs_type, ir, &HashMap::new())?;
    let enumerator_names =
        enum_.enumerators.iter().map(|enumerator| make_rs_ident(&enumerator.identifier.identifier));
    let enumerator_values = enum_.enumerators.iter().map(|enumerator| enumerator.value);
    Ok(quote! {
        #[repr(transparent)]
        #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
        pub struct #name(#underlying_type);
        impl #name {
            #(pub const #enumerator_names: #name = #name(#enumerator_values);)*
        }
        impl From<#underlying_type> for #name {
            fn from(value: #underlying_type) -> #name {
                #name(v)
            }
        }
        impl From<#name> for #underlying_type {
            fn from(value: #name) -> #underlying_type {
                v.0
            }
        }
    })
}

fn generate_type_alias(type_alias: &TypeAlias, ir: &IR) -> Result<TokenStream> {
    let ident = make_rs_ident(&type_alias.identifier.identifier);
    let doc_comment = generate_doc_comment(&type_alias.doc_comment);
    let underlying_type = format_rs_type(&type_alias.underlying_type.rs_type, ir, &HashMap::new())
        .with_context(|| format!("Failed to format underlying type for {:?}", type_alias))?;
    Ok(quote! {
        #doc_comment
        pub type #ident = #underlying_type;
    })
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
    features.insert(make_rs_ident("custom_inner_attributes"));

    // Identify all functions having overloads that we can't import (yet).
    // TODO(b/213280424): Implement support for overloaded functions.
    let mut seen_funcs = HashSet::new();
    let mut overloaded_funcs = HashSet::new();
    for func in ir.functions() {
        if let GeneratedFunc::Some { function_id, .. } = generate_func(func, ir)? {
            if !seen_funcs.insert(function_id.clone()) {
                overloaded_funcs.insert(function_id);
            }
        }
    }

    for item in ir.items() {
        match item {
            Item::Func(func) => match generate_func(func, ir)? {
                GeneratedFunc::None => (),
                GeneratedFunc::Unsupported(unsupported) => {
                    items.push(generate_unsupported(&unsupported)?)
                }
                GeneratedFunc::Some { api_func, thunk, function_id } => {
                    if overloaded_funcs.contains(&function_id) {
                        items.push(generate_unsupported(&make_unsupported_fn(
                            func,
                            ir,
                            "Cannot generate bindings for overloaded function",
                        )?)?);
                        continue;
                    }
                    features.extend(api_func.features);
                    features.extend(thunk.features);
                    items.push(api_func.tokens);
                    thunks.push(thunk.tokens);
                }
            },
            Item::Record(record) => {
                if !ir.is_current_target(&record.owning_target)
                    && !ir.is_stdlib_target(&record.owning_target)
                {
                    continue;
                }
                let (snippet, assertions_snippet) = generate_record(record, ir)?;
                features.extend(snippet.features);
                features.extend(assertions_snippet.features);
                items.push(snippet.tokens);
                assertions.push(assertions_snippet.tokens);
                has_record = true;
            }
            Item::Enum(enum_) => {
                if !ir.is_current_target(&enum_.owning_target)
                    && !ir.is_stdlib_target(&enum_.owning_target)
                {
                    continue;
                }
                items.push(generate_enum(enum_, ir)?);
                continue;
            }
            Item::TypeAlias(type_alias) => {
                if !ir.is_current_target(&type_alias.owning_target)
                    && !ir.is_stdlib_target(&type_alias.owning_target)
                {
                    continue;
                }
                items.push(generate_type_alias(type_alias, ir)?);
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
                #[allow(unused_imports)]
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
        #features __NEWLINE__
        #![allow(non_camel_case_types)] __NEWLINE__
        #![allow(non_snake_case)] __NEWLINE__ __NEWLINE__

        #imports __NEWLINE__ __NEWLINE__

        #( #items __NEWLINE__ __NEWLINE__ )*

        #mod_detail __NEWLINE__ __NEWLINE__

         #( #assertions __NEWLINE__ __NEWLINE__ )*
    })
}

/// Makes an 'Ident' to be used in the Rust source code. Escapes Rust keywords.
fn make_rs_ident(ident: &str) -> Ident {
    // TODO(https://github.com/dtolnay/syn/pull/1098): Remove the hardcoded list once syn recognizes
    // 2018 and 2021 keywords.
    if ["async", "await", "try", "dyn"].contains(&ident) {
        return format_ident!("r#{}", ident);
    }
    match syn::parse_str::<syn::Ident>(ident) {
        Ok(_) => format_ident!("{}", ident),
        Err(_) => format_ident!("r#{}", ident),
    }
}

/// Formats a C++ identifier. Does not escape C++ keywords.
fn format_cc_ident(ident: &str) -> TokenStream {
    ident.parse().unwrap()
}

fn rs_type_name_for_target_and_identifier(
    owning_target: &BlazeLabel,
    identifier: &ir::Identifier,
    ir: &IR,
) -> Result<TokenStream> {
    let ident = make_rs_ident(identifier.identifier.as_str());

    if ir.is_current_target(owning_target) || ir.is_stdlib_target(owning_target) {
        Ok(quote! {#ident})
    } else {
        let owning_crate_name = owning_target.target_name()?;
        // TODO(b/216587072): Remove this hacky escaping and use the import! macro once
        // available
        let escaped_owning_crate_name = owning_crate_name.replace('-', "_");
        let owning_crate = make_rs_ident(&escaped_owning_crate_name);
        Ok(quote! {#owning_crate::#ident})
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Mutability {
    Const,
    Mut,
}

impl Mutability {
    fn format_for_pointer(&self) -> TokenStream {
        match self {
            Mutability::Mut => quote! {mut},
            Mutability::Const => quote! {const},
        }
    }

    fn format_for_reference(&self) -> TokenStream {
        match self {
            Mutability::Mut => quote! {mut},
            Mutability::Const => quote! {},
        }
    }
}

// TODO(b/213947473): Instead of having a separate RsTypeKind here, consider
// changing ir::RsType into a similar `enum`, with fields that contain
// references (e.g. &'ir Record`) instead of DeclIds.
#[derive(Debug)]
enum RsTypeKind<'ir> {
    Pointer { pointee: Box<RsTypeKind<'ir>>, mutability: Mutability },
    Reference { referent: Box<RsTypeKind<'ir>>, mutability: Mutability, lifetime_id: LifetimeId },
    FuncPtr { abi: &'ir str, return_type: Box<RsTypeKind<'ir>>, param_types: Vec<RsTypeKind<'ir>> },
    Record(&'ir Record),
    TypeAlias { type_alias: &'ir TypeAlias, underlying_type: Box<RsTypeKind<'ir>> },
    Unit,
    Other { name: &'ir str, type_args: Vec<RsTypeKind<'ir>> },
}

impl<'ir> RsTypeKind<'ir> {
    pub fn new(ty: &'ir ir::RsType, ir: &'ir IR) -> Result<Self> {
        // The lambdas deduplicate code needed by multiple `match` branches.
        let get_type_args = || -> Result<Vec<RsTypeKind<'ir>>> {
            ty.type_args.iter().map(|type_arg| RsTypeKind::<'ir>::new(type_arg, ir)).collect()
        };
        let get_pointee = || -> Result<Box<RsTypeKind<'ir>>> {
            if ty.type_args.len() != 1 {
                bail!("Missing pointee/referent type (need exactly 1 type argument): {:?}", ty);
            }
            Ok(Box::new(get_type_args()?.remove(0)))
        };
        let get_lifetime = || -> Result<LifetimeId> {
            if ty.lifetime_args.len() != 1 {
                bail!("Missing reference lifetime (need exactly 1 lifetime argument): {:?}", ty);
            }
            Ok(ty.lifetime_args[0])
        };

        let result = match ty.name.as_deref() {
            None => {
                ensure!(
                    ty.type_args.is_empty(),
                    "Type arguments on records nor type aliases are not yet supported: {:?}",
                    ty
                );
                match ir.item_for_type(ty)? {
                    Item::Record(record) => RsTypeKind::Record(record),
                    Item::TypeAlias(type_alias) => RsTypeKind::TypeAlias {
                        type_alias,
                        underlying_type: Box::new(RsTypeKind::new(
                            &type_alias.underlying_type.rs_type,
                            ir,
                        )?),
                    },
                    other_item => bail!("Item does not define a type: {:?}", other_item),
                }
            }
            Some(name) => match name {
                "()" => {
                    if !ty.type_args.is_empty() {
                        bail!("Unit type must not have type arguments: {:?}", ty);
                    }
                    RsTypeKind::Unit
                }
                "*mut" => {
                    RsTypeKind::Pointer { pointee: get_pointee()?, mutability: Mutability::Mut }
                }
                "*const" => {
                    RsTypeKind::Pointer { pointee: get_pointee()?, mutability: Mutability::Const }
                }
                "&mut" => RsTypeKind::Reference {
                    referent: get_pointee()?,
                    mutability: Mutability::Mut,
                    lifetime_id: get_lifetime()?,
                },
                "&" => RsTypeKind::Reference {
                    referent: get_pointee()?,
                    mutability: Mutability::Const,
                    lifetime_id: get_lifetime()?,
                },
                name => {
                    let mut type_args = get_type_args()?;
                    match name.strip_prefix("#funcPtr ") {
                        None => RsTypeKind::Other { name, type_args },
                        Some(abi) => {
                            // TODO(b/217419782): Consider enforcing `'static` lifetime.
                            ensure!(!type_args.is_empty(), "No return type in fn type: {:?}", ty);
                            RsTypeKind::FuncPtr {
                                abi,
                                return_type: Box::new(type_args.remove(type_args.len() - 1)),
                                param_types: type_args,
                            }
                        }
                    }
                }
            },
        };
        Ok(result)
    }

    /// Returns true if the type is known to be `Unpin`, false otherwise.
    pub fn is_unpin(&self, ir: &IR) -> bool {
        match self {
            RsTypeKind::Record(record) => record.is_unpin(),
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.is_unpin(ir),
            _ => true,
        }
    }

    pub fn format(
        &self,
        ir: &IR,
        lifetime_to_name: &HashMap<LifetimeId, String>,
    ) -> Result<TokenStream> {
        let result = match self {
            RsTypeKind::Pointer { pointee, mutability } => {
                let mutability = mutability.format_for_pointer();
                let nested_type = pointee.format(ir, lifetime_to_name)?;
                quote! {* #mutability #nested_type}
            }
            RsTypeKind::Reference { referent, mutability, lifetime_id } => {
                let mut_ = mutability.format_for_reference();
                let lifetime = Self::format_lifetime(lifetime_id, lifetime_to_name)?;
                let nested_type = referent.format(ir, lifetime_to_name)?;
                let reference = quote! {& #lifetime #mut_ #nested_type};
                if mutability == &Mutability::Mut && !referent.is_unpin(ir) {
                    // TODO(b/200067242): Add a `use std::pin::Pin` to the crate, and use `Pin`.
                    // Probably format needs to return an RsSnippet, and RsSnippet needs a `uses`
                    // field.
                    quote! {std::pin::Pin< #reference >}
                } else {
                    reference
                }
            }
            RsTypeKind::FuncPtr { abi, return_type, param_types } => {
                let return_frag =
                    return_type.format_as_return_type_fragment(ir, lifetime_to_name)?;
                let param_types = param_types
                    .iter()
                    .map(|t| t.format(ir, lifetime_to_name))
                    .collect::<Result<Vec<_>>>()?;
                quote! { extern #abi fn( #( #param_types ),* ) #return_frag }
            }
            RsTypeKind::Record(record) => rs_type_name_for_target_and_identifier(
                &record.owning_target,
                &Identifier { identifier: record.rs_name.clone() },
                ir,
            )?,
            RsTypeKind::TypeAlias { type_alias, .. } => rs_type_name_for_target_and_identifier(
                &type_alias.owning_target,
                &type_alias.identifier,
                ir,
            )?,
            RsTypeKind::Unit => quote! {()},
            RsTypeKind::Other { name, type_args } => {
                let ident = make_rs_ident(name);
                let generic_params = format_generic_params(
                    type_args
                        .iter()
                        .map(|type_arg| type_arg.format(ir, lifetime_to_name))
                        .collect::<Result<Vec<_>>>()?,
                );
                quote! {#ident #generic_params}
            }
        };
        Ok(result)
    }

    pub fn format_as_return_type_fragment(
        &self,
        ir: &IR,
        lifetime_to_name: &HashMap<LifetimeId, String>,
    ) -> Result<TokenStream> {
        match self {
            RsTypeKind::Unit => Ok(quote! {}),
            other_type => {
                let return_type = other_type.format(ir, lifetime_to_name)?;
                Ok(quote! { -> #return_type })
            }
        }
    }

    /// Formats this RsTypeKind as `&'a mut MaybeUninit<SomeStruct>`. This is
    /// used to format `__this` parameter in a constructor thunk.
    pub fn format_mut_ref_as_uninitialized(
        &self,
        ir: &IR,
        lifetime_to_name: &HashMap<LifetimeId, String>,
    ) -> Result<TokenStream> {
        match self {
            RsTypeKind::Reference { referent, lifetime_id, mutability: Mutability::Mut } => {
                let nested_type = referent.format(ir, lifetime_to_name)?;
                let lifetime = Self::format_lifetime(lifetime_id, lifetime_to_name)?;
                Ok(quote! { & #lifetime mut std::mem::MaybeUninit< #nested_type > })
            }
            _ => bail!("Expected reference to format as MaybeUninit, got: {:?}", self),
        }
    }

    /// Formats a reference or pointer as a raw pointer.
    pub fn format_ref_as_raw_ptr(
        &self,
        ir: &IR,
        lifetime_to_name: &HashMap<LifetimeId, String>,
    ) -> Result<TokenStream> {
        match self {
            RsTypeKind::Reference { referent: pointee, mutability, .. }
            | RsTypeKind::Pointer { pointee, mutability } => {
                let nested_type = pointee.format(ir, lifetime_to_name)?;
                let mut_ = mutability.format_for_pointer();
                Ok(quote! { * #mut_ #nested_type })
            }
            _ => bail!("Expected reference to format as raw ptr, got: {:?}", self),
        }
    }

    /// Formats this RsTypeKind as the `self` parameter: usually, `&'a self` or
    /// `&'a mut self`.
    ///
    /// If this is !Unpin, however, it uses `self: Pin<&mut Self>` instead.
    pub fn format_as_self_param(
        &self,
        func: &Func,
        ir: &IR,
        lifetime_to_name: &HashMap<LifetimeId, String>,
    ) -> Result<TokenStream> {
        if func.name == UnqualifiedIdentifier::Destructor {
            let record = func
                .member_func_metadata
                .as_ref()
                .ok_or_else(|| anyhow!("Destructors must be member functions: {:?}", func))?
                .find_record(ir)?;
            if self.is_mut_ptr_to(record) {
                // Even in C++ it is UB to retain `this` pointer and dereference it
                // after a destructor runs. Therefore it is safe to use `&self` or
                // `&mut self` in Rust even if IR represents `__this` as a Rust
                // pointer (e.g. when lifetime annotations are missing - lifetime
                // annotations are required to represent it as a Rust reference).
                return Ok(quote! { &mut self });
            }
        }

        match self {
            RsTypeKind::Reference { referent, lifetime_id, mutability } => {
                let mut_ = mutability.format_for_reference();
                let lifetime = Self::format_lifetime(lifetime_id, lifetime_to_name)?;
                if mutability == &Mutability::Mut
                    && !referent.is_unpin(ir)
                    && func.name != UnqualifiedIdentifier::Destructor
                {
                    // TODO(b/200067242): Add a `use std::pin::Pin` to the crate, and use `Pin`.
                    Ok(quote! {self: std::pin::Pin< & #lifetime #mut_ Self>})
                } else {
                    Ok(quote! { & #lifetime #mut_ self })
                }
            }
            _ => bail!("Unexpected type of `self` parameter: {:?}", self),
        }
    }

    fn format_lifetime(
        lifetime_id: &LifetimeId,
        lifetime_to_name: &HashMap<LifetimeId, String>,
    ) -> Result<TokenStream> {
        let lifetime_name = lifetime_to_name.get(lifetime_id).ok_or_else(|| {
            anyhow!("`lifetime_to_name` doesn't have an entry for {:?}", lifetime_id)
        })?;
        Ok(format_lifetime_name(lifetime_name))
    }

    /// Returns whether the type represented by `self` implements the `Copy`
    /// trait.
    pub fn implements_copy(&self) -> bool {
        // TODO(b/212696226): Verify results of `implements_copy` via static
        // assertions in the generated Rust code (because incorrect results
        // can silently lead to unsafe behavior).
        match self {
            RsTypeKind::Unit => true,
            RsTypeKind::Pointer { .. } => true,
            RsTypeKind::FuncPtr { .. } => true,
            RsTypeKind::Reference { mutability: Mutability::Const, .. } => true,
            RsTypeKind::Reference { mutability: Mutability::Mut, .. } => false,
            RsTypeKind::Record(record) => should_derive_copy(record),
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.implements_copy(),
            RsTypeKind::Other { type_args, .. } => {
                // All types that may appear here without `type_args` (e.g.
                // primitive types like `i32`) implement `Copy`. Generic types
                // that may be present here (e.g. Option<...>) are `Copy` if all
                // of their `type_args` are `Copy`.
                type_args.iter().all(|t| t.implements_copy())
            }
        }
    }

    pub fn is_mut_ptr_to(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Pointer { pointee, mutability: Mutability::Mut, .. } => {
                pointee.is_record(expected_record)
            }
            _ => false,
        }
    }

    pub fn is_ref_to(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Reference { referent, .. } => referent.is_record(expected_record),
            _ => false,
        }
    }

    pub fn is_shared_ref_to(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Reference { referent, mutability: Mutability::Const, .. } => {
                referent.is_record(expected_record)
            }
            _ => false,
        }
    }

    pub fn is_record(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Record(actual_record) => actual_record.id == expected_record.id,
            _ => false,
        }
    }

    /// Iterates over `self` and all the nested types (e.g. pointees, generic
    /// type args, etc.) in DFS order.
    pub fn dfs_iter<'ty>(&'ty self) -> impl Iterator<Item = &'ty RsTypeKind<'ir>> + '_ {
        RsTypeKindIter::new(self)
    }

    /// Iterates over all `LifetimeId`s in `self` and in all the nested types.
    /// Note that the results might contain duplicate LifetimeId values (e.g.
    /// if the same LifetimeId is used in two `type_args`).
    pub fn lifetimes(&self) -> impl Iterator<Item = LifetimeId> + '_ {
        self.dfs_iter().filter_map(|t| match t {
            RsTypeKind::Reference { lifetime_id, .. } => Some(*lifetime_id),
            _ => None,
        })
    }
}

struct RsTypeKindIter<'ty, 'ir> {
    todo: Vec<&'ty RsTypeKind<'ir>>,
}

impl<'ty, 'ir> RsTypeKindIter<'ty, 'ir> {
    pub fn new(ty: &'ty RsTypeKind<'ir>) -> Self {
        Self { todo: vec![ty] }
    }
}

impl<'ty, 'ir> Iterator for RsTypeKindIter<'ty, 'ir> {
    type Item = &'ty RsTypeKind<'ir>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.todo.pop() {
            None => None,
            Some(curr) => {
                match curr {
                    RsTypeKind::Unit | RsTypeKind::Record(_) => (),
                    RsTypeKind::Pointer { pointee, .. } => self.todo.push(pointee),
                    RsTypeKind::Reference { referent, .. } => self.todo.push(referent),
                    RsTypeKind::TypeAlias { underlying_type: t, .. } => self.todo.push(t),
                    RsTypeKind::FuncPtr { return_type, param_types, .. } => {
                        self.todo.push(return_type);
                        self.todo.extend(param_types.iter().rev());
                    }
                    RsTypeKind::Other { type_args, .. } => self.todo.extend(type_args.iter().rev()),
                };
                Some(curr)
            }
        }
    }
}

fn format_lifetime_name(lifetime_name: &str) -> TokenStream {
    let lifetime =
        syn::Lifetime::new(&format!("'{}", lifetime_name), proc_macro2::Span::call_site());
    quote! { #lifetime }
}

fn format_rs_type(
    ty: &ir::RsType,
    ir: &IR,
    lifetime_to_name: &HashMap<LifetimeId, String>,
) -> Result<TokenStream> {
    RsTypeKind::new(ty, ir)
        .and_then(|kind| kind.format(ir, lifetime_to_name))
        .with_context(|| format!("Failed to format Rust type {:?}", ty))
}

fn cc_type_name_for_item(item: &ir::Item) -> Result<TokenStream> {
    Ok(match item {
        Item::Record(record) => {
            let ident = format_cc_ident(&record.cc_name);
            quote! { class #ident }
        }
        Item::TypeAlias(type_alias) => {
            let ident = format_cc_ident(&type_alias.identifier.identifier);
            quote! { #ident }
        }
        _ => bail!("Item does not define a type: {:?}", item),
    })
}

// Maps a Rust ABI [1] into a Clang attribute. See also
// `ConvertCcCallConvIntoRsApi` in importer.cc.
// [1]
// https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier
fn format_cc_call_conv_as_clang_attribute(rs_abi: &str) -> Result<TokenStream> {
    match rs_abi {
        "cdecl" => Ok(quote! {}),
        "fastcall" => Ok(quote! { __attribute__((fastcall)) }),
        "stdcall" => Ok(quote! { __attribute__((stdcall)) }),
        "thiscall" => Ok(quote! { __attribute__((thiscall)) }),
        "vectorcall" => Ok(quote! { __attribute__((vectorcall)) }),
        _ => bail!("Unsupported ABI: {}", rs_abi),
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
            "&" => {
                if ty.type_args.len() != 1 {
                    bail!("Invalid reference type (need exactly 1 type argument): {:?}", ty);
                }
                let nested_type = format_cc_type(&ty.type_args[0], ir)?;
                Ok(quote! {#nested_type &})
            }
            cc_type_name => match cc_type_name.strip_prefix("#funcValue ") {
                None => {
                    if !ty.type_args.is_empty() {
                        bail!("Type not yet supported: {:?}", ty);
                    }
                    let idents = cc_type_name.split_whitespace().map(format_cc_ident);
                    Ok(quote! {#( #idents )* #const_fragment})
                }
                Some(abi) => match ty.type_args.split_last() {
                    None => bail!("funcValue type without a return type: {:?}", ty),
                    Some((ret_type, param_types)) => {
                        let ret_type = format_cc_type(ret_type, ir)?;
                        let param_types = param_types
                            .iter()
                            .map(|t| format_cc_type(t, ir))
                            .collect::<Result<Vec<_>>>()?;
                        let attr = format_cc_call_conv_as_clang_attribute(abi)?;
                        // `type_identity_t` is used below to avoid having to
                        // emit spiral-like syntax where some syntax elements of
                        // an inner type (e.g. function type as below) can
                        // surround syntax elements of an outer type (e.g. a
                        // pointer type). Compare: `int (*foo)(int, int)` VS
                        // `type_identity_t<int(int, int)>* foo`.
                        Ok(quote! { rs_api_impl_support::type_identity_t<
                            #ret_type ( #( #param_types ),* ) #attr
                        >  })
                    }
                },
            },
        }
    } else {
        let item = ir.item_for_type(ty)?;
        let type_name = cc_type_name_for_item(item)?;
        Ok(quote! {#const_fragment #type_name})
    }
}

fn cc_struct_layout_assertion(record: &Record, ir: &IR) -> TokenStream {
    if !ir.is_current_target(&record.owning_target) && !ir.is_stdlib_target(&record.owning_target) {
        return quote! {};
    }
    let record_ident = format_cc_ident(&record.cc_name);
    let size = Literal::usize_unsuffixed(record.size);
    let alignment = Literal::usize_unsuffixed(record.alignment);
    let field_assertions =
        record.fields.iter().filter(|f| f.access == AccessSpecifier::Public).map(|field| {
            let field_ident = format_cc_ident(&field.identifier.identifier);
            let offset = Literal::usize_unsuffixed(field.offset);
            // The IR contains the offset in bits, while C++'s offsetof()
            // returns the offset in bytes, so we need to convert.
            quote! {
                static_assert(offsetof(class #record_ident, #field_ident) * 8 == #offset);
            }
        });
    quote! {
        static_assert(sizeof(class #record_ident) == #size);
        static_assert(alignof(class #record_ident) == #alignment);
        #( #field_assertions )*
    }
}

// Returns the accessor functions for no_unique_address member variables.
fn cc_struct_no_unique_address_impl(record: &Record, ir: &IR) -> Result<TokenStream> {
    let mut fields = vec![];
    let mut types = vec![];
    for field in &record.fields {
        if field.access != AccessSpecifier::Public || !field.is_no_unique_address {
            continue;
        }
        fields.push(make_rs_ident(&field.identifier.identifier));
        types.push(format_rs_type(&field.type_.rs_type, ir, &HashMap::new()).with_context(
            || format!("Failed to format type for field {:?} on record {:?}", field, record),
        )?)
    }

    if fields.is_empty() {
        return Ok(quote! {});
    }

    let ident = make_rs_ident(&record.rs_name);
    Ok(quote! {
        impl #ident {
            #(
                pub fn #fields(&self) -> &#types {
                    unsafe {&* (&self.#fields as *const _ as *const #types)}
                }
            )*
        }
    })
}

/// Returns the implementation of base class conversions, for converting a type
/// to its unambiguous public base classes.
///
/// TODO(b/216195042): Implement this in terms of a supporting trait which casts
/// raw pointers. Then, we would have blanket impls for reference, pinned mut
/// reference, etc. conversion. The current version is just enough to test the
/// logic in importer.
//
// TODO(b/216195042): Should this use, like, AsRef/AsMut (and some equivalent
// for Pin)?
fn cc_struct_upcast_impl(record: &Record, ir: &IR) -> Result<TokenStream> {
    let mut impls = Vec::with_capacity(record.unambiguous_public_bases.len());
    for base in &record.unambiguous_public_bases {
        let base_record: &Record = ir.find_decl(base.base_record_id)?.try_into()?;
        if let Some(offset) = base.offset {
            let offset = Literal::i64_unsuffixed(offset);
            // TODO(b/216195042): Correctly handle imported records, lifetimes.
            let base_name = make_rs_ident(&base_record.rs_name);
            let derived_name = make_rs_ident(&record.rs_name);
            impls.push(quote! {
                impl<'a> From<&'a #derived_name> for &'a #base_name {
                    fn from(x: &'a #derived_name) -> Self {
                        unsafe {
                            &*((x as *const _ as *const u8).offset(#offset) as *const #base_name)
                        }
                    }
                }
            });
        } else {
            // TODO(b/216195042): determine offset dynamically / use a dynamic
            // cast. This requires a new C++ function to be
            // generated, so that we have something to call.
        }
    }

    Ok(quote! {
        #(#impls)*
    })
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
        if can_skip_cc_thunk(func) {
            continue;
        }

        let thunk_ident = thunk_ident(func);
        let implementation_function = match &func.name {
            UnqualifiedIdentifier::Operator(op) => {
                let name = syn::parse_str::<TokenStream>(&op.name)?;
                quote! { operator #name }
            }
            UnqualifiedIdentifier::Identifier(id) => {
                let fn_ident = format_cc_ident(&id.identifier);
                let static_method_metadata = func
                    .member_func_metadata
                    .as_ref()
                    .filter(|meta| meta.instance_method_metadata.is_none());
                match static_method_metadata {
                    None => quote! {#fn_ident},
                    Some(meta) => {
                        let record_ident = format_cc_ident(&meta.find_record(ir)?.cc_name);
                        quote! { #record_ident :: #fn_ident }
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
                quote! { rs_api_impl_support::construct_at }
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
            func.params.iter().map(|p| format_cc_ident(&p.identifier.identifier)).collect_vec();

        let param_types = func
            .params
            .iter()
            .map(|p| format_cc_type(&p.type_.cc_type, ir))
            .collect::<Result<Vec<_>>>()?;

        let needs_this_deref = match &func.member_func_metadata {
            None => false,
            Some(meta) => match &func.name {
                UnqualifiedIdentifier::Constructor | UnqualifiedIdentifier::Destructor => false,
                UnqualifiedIdentifier::Identifier(_) | UnqualifiedIdentifier::Operator(_) => {
                    meta.instance_method_metadata.is_some()
                }
            },
        };
        let (implementation_function, arg_expressions) = if !needs_this_deref {
            (implementation_function, param_idents.clone())
        } else {
            let this_param = func
                .params
                .first()
                .ok_or_else(|| anyhow!("Instance methods must have `__this` param."))?;
            let this_arg = format_cc_ident(&this_param.identifier.identifier);
            (
                quote! { #this_arg -> #implementation_function},
                param_idents.iter().skip(1).cloned().collect_vec(),
            )
        };

        thunks.push(quote! {
            extern "C" #return_type_name #thunk_ident( #( #param_types #param_idents ),* ) {
                #return_stmt #implementation_function( #( #arg_expressions ),* );
            }
        });
    }

    let layout_assertions = ir.records().map(|record| cc_struct_layout_assertion(record, ir));

    let mut standard_headers = <BTreeSet<Ident>>::new();
    standard_headers.insert(format_ident!("memory")); // ubiquitous.
    if ir.records().next().is_some() {
        standard_headers.insert(format_ident!("cstddef"));
    };

    let mut includes = vec!["rs_bindings_from_cc/support/cxx20_backports.h"];

    // In order to generate C++ thunk in all the cases Clang needs to be able to
    // access declarations from public headers of the C++ library.
    includes.extend(ir.used_headers().map(|i| &i.name as &str));

    Ok(quote! {
        #( __HASH_TOKEN__ include <#standard_headers> __NEWLINE__)*
        __NEWLINE__
        #( __HASH_TOKEN__ include #includes __NEWLINE__)* __NEWLINE__
        __HASH_TOKEN__ pragma clang diagnostic push __NEWLINE__
        // Disable Clang thread-safety-analysis warnings that would otherwise
        // complain about thunks that call mutex locking functions in an unpaired way.
        __HASH_TOKEN__ pragma clang diagnostic ignored "-Wthread-safety-analysis" __NEWLINE__

        #( #thunks )* __NEWLINE__ __NEWLINE__

        #( #layout_assertions __NEWLINE__ __NEWLINE__ )*

        __NEWLINE__
        __HASH_TOKEN__ pragma clang diagnostic pop __NEWLINE__
        // To satisfy http://cs/symbol:devtools.metadata.Presubmit.CheckTerminatingNewline check.
        __NEWLINE__
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use ir_testing::{ir_from_cc, ir_from_cc_dependency, ir_func, ir_record, retrieve_func};
    use token_stream_matchers::{
        assert_cc_matches, assert_cc_not_matches, assert_ir_matches, assert_rs_matches,
        assert_rs_not_matches,
    };
    use token_stream_printer::tokens_to_string;

    #[test]
    fn test_disable_thread_safety_warnings() -> Result<()> {
        let ir = ir_from_cc("inline void foo() {}")?;
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                ...
                __HASH_TOKEN__ pragma clang diagnostic push
                __HASH_TOKEN__ pragma clang diagnostic ignored "-Wthread-safety-analysis"
                ...

                __HASH_TOKEN__ pragma clang diagnostic pop
                ...
            }
        );
        Ok(())
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
                    #[allow(unused_imports)]
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
                    #[allow(unused_imports)]
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
                #[allow(unused_imports)]
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
                extern "C" class ReturnStruct __rust_thunk___Z11DoSomething11ParamStruct(class ParamStruct param) {
                    return DoSomething(param);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_simple_struct() -> Result<()> {
        let ir = ir_from_cc(&tokens_to_string(quote! {
            struct SomeStruct final {
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
                extern "C" void __rust_thunk___ZN10SomeStructD1Ev(class SomeStruct * __this) {
                    std :: destroy_at (__this) ;
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                static_assert(sizeof(class SomeStruct) == 12);
                static_assert(alignof(class SomeStruct) == 4);
                static_assert(offsetof(class SomeStruct, public_int) * 8 == 0);
            }
        );
        Ok(())
    }

    #[test]
    fn test_ref_to_struct_in_thunk_impls() -> Result<()> {
        let ir = ir_from_cc("struct S{}; inline void foo(class S& s) {} ")?;
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3fooR1S(class S& s) {
                    foo(s);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_const_ref_to_struct_in_thunk_impls() -> Result<()> {
        let ir = ir_from_cc("struct S{}; inline void foo(const class S& s) {} ")?;
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3fooRK1S(const class S& s) {
                    foo(s);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_unsigned_int_in_thunk_impls() -> Result<()> {
        let ir = ir_from_cc("inline void foo(unsigned int i) {} ")?;
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3fooj(unsigned int i) {
                    foo(i);
                }
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
    fn test_record_instance_methods_deref_this_in_thunk() -> Result<()> {
        let ir = ir_from_cc(&tokens_to_string(quote! {
            struct SomeStruct {
                inline int some_func(int arg) const { return 42 + arg; }
            };
        })?)?;

        assert_cc_matches!(
            generate_rs_api_impl(&ir)?,
            quote! {
                extern "C" int __rust_thunk___ZNK10SomeStruct9some_funcEi(
                        const class SomeStruct* __this, int arg) {
                    return __this->some_func(arg);
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
        assert_eq!(generate_derives(&record), &["Clone", "Copy"]);
    }

    #[test]
    fn test_copy_derives_not_is_trivial_abi() {
        let mut record = ir_record("S");
        record.is_trivial_abi = false;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    /// Even if it's trivially relocatable, !Unpin C++ type cannot be
    /// cloned/copied or otherwise used by value, because values would allow
    /// assignment into the Pin.
    ///
    /// All !Unpin C++ types, not just non trivially relocatable ones, are
    /// unsafe to assign in the Rust sense.
    #[test]
    fn test_copy_derives_not_final() {
        let mut record = ir_record("S");
        record.is_final = false;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_copy_derives_ctor_nonpublic() {
        let mut record = ir_record("S");
        for access in [ir::AccessSpecifier::Protected, ir::AccessSpecifier::Private] {
            record.copy_constructor.access = access;
            assert_eq!(generate_derives(&record), &[""; 0]);
        }
    }

    #[test]
    fn test_copy_derives_ctor_deleted() {
        let mut record = ir_record("S");
        record.copy_constructor.definition = ir::SpecialMemberDefinition::Deleted;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_copy_derives_ctor_nontrivial_members() {
        let mut record = ir_record("S");
        record.copy_constructor.definition = ir::SpecialMemberDefinition::NontrivialMembers;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_copy_derives_ctor_nontrivial_self() {
        let mut record = ir_record("S");
        record.copy_constructor.definition = ir::SpecialMemberDefinition::NontrivialUserDefined;
        assert_eq!(generate_derives(&record), &[""; 0]);
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
                pub unsafe fn Deref(p: *const *mut i32) -> *mut i32 {
                    crate::detail::__rust_thunk___Z5DerefPKPi(p)
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
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
    fn test_const_char_ptr_func() -> Result<()> {
        // This is a regression test: We used to include the "const" in the name
        // of the CcType, which caused a panic in the code generator
        // ('"const char" is not a valid Ident').
        // It's therefore important that f() is inline so that we need to
        // generate a thunk for it (where we then process the CcType).
        let ir = ir_from_cc(&tokens_to_string(quote! {
            inline void f(const char *str);
        })?)?;

        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub unsafe fn f(str: *const i8) {
                    crate::detail::__rust_thunk___Z1fPKc(str)
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                extern "C" {
                    pub(crate) fn __rust_thunk___Z1fPKc(str: *const i8);
                }
            }
        );

        assert_cc_matches!(
            generate_rs_api_impl(&ir)?,
            quote! {
                extern "C" void __rust_thunk___Z1fPKc(char const * str){ f(str) ; }
            }
        );
        Ok(())
    }

    #[test]
    fn test_func_ptr_where_params_are_primitive_types() -> Result<()> {
        let ir = ir_from_cc(r#" int (*get_ptr_to_func())(float, double); "#)?;
        let rs_api = generate_rs_api(&ir)?;
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ptr_to_func() -> Option<extern "C" fn (f32, f64) -> i32> {
                    unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z15get_ptr_to_funcv"]
                        pub(crate) fn __rust_thunk___Z15get_ptr_to_funcv()
                        -> Option<extern "C" fn(f32, f64) -> i32>;
                    }
                }
            }
        );
        // Verify that no C++ thunk got generated.
        assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });

        // TODO(b/217419782): Add another test for more exotic calling conventions /
        // abis.

        // TODO(b/217419782): Add another test for pointer to a function that
        // takes/returns non-trivially-movable types by value. See also
        // <internal link>

        Ok(())
    }

    #[test]
    fn test_func_ref() -> Result<()> {
        let ir = ir_from_cc(r#" int (&get_ref_to_func())(float, double); "#)?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ref_to_func() -> extern "C" fn (f32, f64) -> i32 {
                    unsafe { crate::detail::__rust_thunk___Z15get_ref_to_funcv() }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_func_ptr_with_non_static_lifetime() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            [[clang::annotate("lifetimes", "-> a")]]
            int (*get_ptr_to_func())(float, double); "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                // Error while generating bindings for item 'get_ptr_to_func':
                // Return type is not supported: Function pointers with non-'static lifetimes are not supported: int (*)(float, double)
            }
        );
        Ok(())
    }

    #[test]
    fn test_func_ptr_where_params_are_raw_ptrs() -> Result<()> {
        let ir = ir_from_cc(r#" const int* (*get_ptr_to_func())(const int*); "#)?;
        let rs_api = generate_rs_api(&ir)?;
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ptr_to_func() -> Option<extern "C" fn (*const i32) -> *const i32> {
                    unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z15get_ptr_to_funcv"]
                        pub(crate) fn __rust_thunk___Z15get_ptr_to_funcv()
                        -> Option<extern "C" fn(*const i32) -> *const i32>;
                    }
                }
            }
        );
        // Verify that no C++ thunk got generated.
        assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });

        // TODO(b/217419782): Add another test where params (and the return
        // type) are references with lifetimes.  Something like this:
        //     #pragma clang lifetime_elision
        //     const int& (*get_ptr_to_func())(const int&, const int&); "#)?;
        // 1) Need to investigate why this fails - seeing raw pointers in Rust
        //    seems to indicate that no lifetimes are present at the `importer.cc`
        //    level. Maybe lifetime elision doesn't support this scenario? Unclear
        //    how to explicitly apply [[clang::annotate("lifetimes", "a, b -> a")]]
        //    to the _inner_ function.
        // 2) It is important to have 2 reference parameters, so see if the problem
        //    of passing `lifetimes` by value would have been caught - see:
        //    cl/428079010/depot/rs_bindings_from_cc/
        // importer.cc?version=s6#823

        // TODO(b/217419782): Decide what to do if the C++ pointer is *not*
        // annotated with a lifetime - emit `unsafe fn(...) -> ...` in that
        // case?

        Ok(())
    }

    #[test]
    fn test_func_ptr_with_custom_abi() -> Result<()> {
        let ir = ir_from_cc(r#" int (*get_ptr_to_func())(float, double) [[clang::vectorcall]]; "#)?;

        // Verify that the test input correctly represents what we intend to
        // test - we want [[clang::vectorcall]] to apply to the returned
        // function pointer, but *not* apply to the `get_ptr_to_func` function.
        // TODO(lukasza): Check that latter part after adding `Func::rs_abi`
        // field to the IR in cl/428596835.
        assert_ir_matches!(
            ir,
            quote! {
                Func(Func {
                    name: "get_ptr_to_func", ...
                    return_type: MappedType {
                        rs_type: RsType {
                            name: Some("Option"), ...
                            type_args: [RsType { name: Some("#funcPtr vectorcall"), ... }], ...
                        },
                        cc_type: CcType {
                            name: Some("*"), ...
                            type_args: [CcType { name: Some("#funcValue vectorcall"), ... }], ...
                        },
                    },
                    ...
                }),
            }
        );

        // Check that the custom "vectorcall" ABI gets propagated into the
        // return type (i.e. into `extern "vectorcall" fn`).
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ptr_to_func() -> Option<extern "vectorcall" fn (f32, f64) -> i32> {
                    unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                }
            }
        );

        // The usual `extern "C"` ABI should be used for "get_ptr_to_func".
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z15get_ptr_to_funcv"]
                        pub(crate) fn __rust_thunk___Z15get_ptr_to_funcv()
                        -> Option<extern "vectorcall" fn(f32, f64) -> i32>;
                    }
                }
            }
        );

        // Verify that no C++ thunk got generated.
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });
        Ok(())
    }

    #[test]
    fn test_func_ptr_thunk() -> Result<()> {
        // Using an `inline` keyword forces generation of a C++ thunk in
        // `rs_api_impl` (i.e. exercises `format_cc_type` and similar code).
        let ir = ir_from_cc(
            r#"
            int multiply(int x, int y);
            inline int (*inline_get_pointer_to_function())(int, int) {
                return multiply;
            }
        "#,
        )?;
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" rs_api_impl_support::type_identity_t<int(int , int)>*
                __rust_thunk___Z30inline_get_pointer_to_functionv() {
                    return inline_get_pointer_to_function();
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

        let idx = |s: &str| rs_api.find(s).ok_or_else(|| anyhow!("'{}' missing", s));

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
    fn test_base_class_subobject_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            // We use a class here to force `Derived::z` to live inside the tail padding of `Base`.
            // On the Itanium ABI, this would not happen if `Base` were a POD type.
            class Base {__INT64_TYPE__ x; char y;};
            struct Derived final : Base {__INT16_TYPE__ z;};
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Derived {
                    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 10],
                    pub z: i16,
                }
            }
        );
        Ok(())
    }

    /// The same as test_base_class_subobject_layout, but with multiple
    /// inheritance.
    #[test]
    fn test_base_class_multiple_inheritance_subobject_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base1 {__INT64_TYPE__ x;};
            class Base2 {char y;};
            struct Derived final : Base1, Base2 {__INT16_TYPE__ z;};
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Derived {
                    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 10],
                    pub z: i16,
                }
            }
        );
        Ok(())
    }

    /// The same as test_base_class_subobject_layout, but with a chain of
    /// inheritance.
    #[test]
    fn test_base_class_deep_inheritance_subobject_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base1 {__INT64_TYPE__ x;};
            class Base2 : Base1 {char y;};
            struct Derived final : Base2 {__INT16_TYPE__ z;};
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Derived {
                    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 10],
                    pub z: i16,
                }
            }
        );
        Ok(())
    }

    /// For derived classes with no data members, we can't use the offset of the
    /// first member to determine the size of the base class subobjects.
    #[test]
    fn test_base_class_subobject_fieldless_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base {__INT64_TYPE__ x; char y;};
            struct Derived final : Base {};
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Derived {
                    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 9],
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_base_class_subobject_empty_fieldless() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base {};
            struct Derived final : Base {};
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                pub struct Derived {
                    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 0],
                    /// Prevent empty C++ struct being zero-size in Rust.
                    placeholder: std::mem::MaybeUninit<u8>,
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_base_class_subobject_empty() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base {};
            struct Derived final : Base {};
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                pub struct Derived {
                    __base_class_subobjects: [std::mem::MaybeUninit<u8>; 0],
                    /// Prevent empty C++ struct being zero-size in Rust.
                    placeholder: std::mem::MaybeUninit<u8>,
                }
            }
        );
        Ok(())
    }

    /// When a field is [[no_unique_address]], it occupies the space up to the
    /// next field.
    #[test]
    fn test_no_unique_address() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field1 {__INT64_TYPE__ x;};
            class Field2 {char y;};
            struct Struct final {
                [[no_unique_address]] Field1 field1;
                [[no_unique_address]] Field2 field2;
                __INT16_TYPE__ z;
            };
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C, align(8))]
                pub struct Struct {
                    field1: [std::mem::MaybeUninit<u8>; 8],
                    field2: [std::mem::MaybeUninit<u8>; 2],
                    pub z: i16,
                }

                impl Struct {
                    pub fn field1(&self) -> &Field1 {
                        unsafe {&* (&self.field1 as *const _ as *const Field1)}
                    }
                    pub fn field2(&self) -> &Field2 {
                        unsafe {&* (&self.field2 as *const _ as *const Field2)}
                    }
                }
            }
        );
        Ok(())
    }

    /// When a [[no_unique_address]] field is the last one, it occupies the rest
    /// of the object.
    #[test]
    fn test_no_unique_address_last_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field1 {__INT64_TYPE__ x;};
            class Field2 {char y;};
            struct Struct final {
                [[no_unique_address]] Field1 field1;
                [[no_unique_address]] Field2 field2;
            };
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C, align(8))]
                pub struct Struct {
                    field1: [std::mem::MaybeUninit<u8>; 8],
                    field2: [std::mem::MaybeUninit<u8>; 8],
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_no_unique_address_empty() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field {};
            struct Struct final {
                [[no_unique_address]] Field field;
                int x;
            };
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(4))]
                pub struct Struct {
                    field: [std::mem::MaybeUninit<u8>; 0],
                    pub x: i32,
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_base_class_subobject_empty_last_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field {};
            struct Struct final {
                [[no_unique_address]] Field field;
            };
        "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                pub struct Struct {
                    field: [std::mem::MaybeUninit<u8>; 1],
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_basic() -> Result<()> {
        let ir = ir_from_cc("enum Color { kRed = 5, kBlue };")?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(u32);
                impl Color {
                    pub const kRed: Color = Color(5);
                    pub const kBlue: Color = Color(6);
                }
                impl From<u32> for Color {
                    fn from(value: u32) -> Color {
                        Color(v)
                    }
                }
                impl From<Color> for u32 {
                    fn from(value: Color) -> u32 {
                        v.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_scoped_enum_basic() -> Result<()> {
        let ir = ir_from_cc("enum class Color { kRed = -5, kBlue };")?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(i32);
                impl Color {
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                }
                impl From<i32> for Color {
                    fn from(value: i32) -> Color {
                        Color(v)
                    }
                }
                impl From<Color> for i32 {
                    fn from(value: Color) -> i32 {
                        v.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_with_64_bit_signed_vals() -> Result<()> {
        let ir = ir_from_cc(
            "enum Color : long { kViolet = -9223372036854775807 - 1LL, kRed = -5, kBlue, kGreen = 3, kMagenta = 9223372036854775807 };",
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(i64);
                impl Color {
                    pub const kViolet: Color = Color(-9223372036854775808);
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                    pub const kGreen: Color = Color(3);
                    pub const kMagenta: Color = Color(9223372036854775807);
                }
                impl From<i64> for Color {
                    fn from(value: i64) -> Color {
                        Color(v)
                    }
                }
                impl From<Color> for i64 {
                    fn from(value: Color) -> i64 {
                        v.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_with_64_bit_unsigned_vals() -> Result<()> {
        let ir = ir_from_cc(
            "enum Color: unsigned long { kRed, kBlue, kLimeGreen = 18446744073709551615 };",
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(u64);
                impl Color {
                    pub const kRed: Color = Color(0);
                    pub const kBlue: Color = Color(1);
                    pub const kLimeGreen: Color = Color(18446744073709551615);
                }
                impl From<u64> for Color {
                    fn from(value: u64) -> Color {
                        Color(v)
                    }
                }
                impl From<Color> for u64 {
                    fn from(value: Color) -> u64 {
                        v.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_with_32_bit_signed_vals() -> Result<()> {
        let ir = ir_from_cc(
            "enum Color { kViolet = -2147483647 - 1, kRed = -5, kBlue, kGreen = 3, kMagenta = 2147483647 };",
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(i32);
                impl Color {
                    pub const kViolet: Color = Color(-2147483648);
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                    pub const kGreen: Color = Color(3);
                    pub const kMagenta: Color = Color(2147483647);
                }
                impl From<i32> for Color {
                    fn from(value: i32) -> Color {
                        Color(v)
                    }
                }
                impl From<Color> for i32 {
                    fn from(value: Color) -> i32 {
                        v.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_with_32_bit_unsigned_vals() -> Result<()> {
        let ir = ir_from_cc("enum Color: unsigned int { kRed, kBlue, kLimeGreen = 4294967295 };")?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(u32);
                impl Color {
                    pub const kRed: Color = Color(0);
                    pub const kBlue: Color = Color(1);
                    pub const kLimeGreen: Color = Color(4294967295);
                }
                impl From<u32> for Color {
                    fn from(value: u32) -> Color {
                        Color(v)
                    }
                }
                impl From<Color> for u32 {
                    fn from(value: Color) -> u32 {
                        v.0
                    }
                }
            }
        );
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
            struct SomeStruct final {\n\
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
    fn test_unambiguous_public_bases() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "
            struct VirtualBase {};
            struct PrivateBase {};
            struct ProtectedBase {};
            struct UnambiguousPublicBase {};
            struct AmbiguousPublicBase {};
            struct MultipleInheritance : UnambiguousPublicBase, AmbiguousPublicBase {};
            struct Derived : private PrivateBase, protected ProtectedBase, MultipleInheritance, AmbiguousPublicBase, virtual VirtualBase {};
        ",
            "",
        )?;
        let rs_api = generate_rs_api(&ir)?;
        // TODO(b/216195042): virtual bases.
        assert_rs_not_matches!(rs_api, quote! { From<&'a Derived> for &'a VirtualBase });
        assert_rs_matches!(rs_api, quote! { From<&'a Derived> for &'a UnambiguousPublicBase });
        assert_rs_matches!(rs_api, quote! { From<&'a Derived> for &'a MultipleInheritance });
        assert_rs_not_matches!(rs_api, quote! {From<&'a Derived> for &'a PrivateBase});
        assert_rs_not_matches!(rs_api, quote! {From<&'a Derived> for &'a ProtectedBase});
        assert_rs_not_matches!(rs_api, quote! {From<&'a Derived> for &'a AmbiguousPublicBase});
        Ok(())
    }

    /// Contrary to intuitions: a base class conversion is ambiguous even if the
    /// ambiguity is from a private base class cast that you can't even
    /// perform.
    ///
    /// Explanation (courtesy James Dennett):
    ///
    /// > Once upon a time, there was a rule in C++ that changing all access
    /// > specifiers to "public" would not change the meaning of code.
    /// > That's no longer true, but some of its effects can still be seen.
    ///
    /// So, we need to be sure to not allow casting to privately-ambiguous
    /// bases.
    #[test]
    fn test_unambiguous_public_bases_private_ambiguity() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "
            struct Base {};
            struct Intermediate : public Base {};
            struct Derived : Base, private Intermediate {};
        ",
            "",
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_not_matches!(rs_api, quote! { From<&'a Derived> for &'a Base });
        Ok(())
    }

    #[test]
    fn test_virtual_thunk() -> Result<()> {
        let ir = ir_from_cc("struct Polymorphic { virtual void Foo(); };")?;

        assert_cc_matches!(
            generate_rs_api_impl(&ir)?,
            quote! {
                extern "C" void __rust_thunk___ZN11Polymorphic3FooEv(class Polymorphic * __this)
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
            r#" struct NontrivialStruct { ~NontrivialStruct(); };
            struct UserDefinedDestructor {
                ~UserDefinedDestructor();
                int x;
                NontrivialStruct nts;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl Drop for UserDefinedDestructor {
                    #[inline(always)]
                    fn drop(&mut self) {
                        unsafe { crate::detail::__rust_thunk___ZN21UserDefinedDestructorD1Ev(self) }
                    }
                }
            }
        );
        assert_rs_matches!(rs_api, quote! {pub x: i32,});
        assert_rs_matches!(rs_api, quote! {pub nts: std::mem::ManuallyDrop<NontrivialStruct>,});
        Ok(())
    }

    /// nontrivial types without user-defined destructors should invoke
    /// the C++ destructor to preserve the order of field destructions.
    #[test]
    fn test_impl_drop_nontrivial_member_destructor() -> Result<()> {
        // TODO(jeanpierreda): This would be cleaner if the UserDefinedDestructor code were
        // omitted. For example, we simulate it so that UserDefinedDestructor
        // comes from another library.
        let ir = ir_from_cc(
            r#"struct UserDefinedDestructor final {
                ~UserDefinedDestructor();
            };
            struct TrivialStruct final { int i; };
            struct NontrivialMembers final {
                UserDefinedDestructor udd;
                TrivialStruct ts;
                int x;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl Drop for NontrivialMembers {
                    #[inline(always)]
                    fn drop(&mut self) {
                        unsafe { crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self) }
                    }
                }
            }
        );
        assert_rs_matches!(rs_api, quote! {pub x: i32,});
        assert_rs_matches!(rs_api, quote! {pub ts: TrivialStruct,});
        assert_rs_matches!(
            rs_api,
            quote! {pub udd: std::mem::ManuallyDrop<UserDefinedDestructor>,}
        );
        Ok(())
    }

    /// Trivial types (at least those that are mapped to Copy rust types) do not
    /// get a Drop impl.
    #[test]
    fn test_impl_drop_trivial() -> Result<()> {
        let ir = ir_from_cc(
            r#"struct Trivial final {
                ~Trivial() = default;
                int x;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_not_matches!(rs_api, quote! {impl Drop});
        assert_rs_matches!(rs_api, quote! {pub x: i32});
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        // TODO(b/213326125): Avoid generating thunk impls that are never called.
        // (The test assertion below should be reversed once this bug is fixed.)
        assert_cc_matches!(rs_api_impl, quote! { std::destroy_at });
        Ok(())
    }

    #[test]
    fn test_impl_default_explicitly_defaulted_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct DefaultedConstructor final {
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
                        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN20DefaultedConstructorC1Ev(&mut tmp);
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
                        class DefaultedConstructor* __this) {
                    rs_api_impl_support::construct_at (__this) ;
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_impl_clone_that_propagates_lifetime() -> Result<()> {
        // This test covers the case where a single lifetime applies to 1)
        // the `__this` parameter and 2) other constructor parameters. For
        // example, maybe the newly constructed object needs to have the
        // same lifetime as the constructor's parameter. (This might require
        // annotating the whole C++ struct with a lifetime, so maybe the
        // example below is not fully realistic/accurate...).
        let mut ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Foo final {
                [[clang::annotate("lifetimes", "a: a")]]
                Foo(const int& i);
            };"#,
        )?;
        let ctor: &mut Func = ir
            .items_mut()
            .filter_map(|item| match item {
                Item::Func(func) => Some(func),
                _ => None,
            })
            .find(|f| {
                matches!(&f.name, UnqualifiedIdentifier::Constructor)
                    && f.params.get(1).map(|p| p.identifier.identifier == "i").unwrap_or_default()
            })
            .unwrap();
        {
            // Double-check that the test scenario set up above uses the same lifetime
            // for both of the constructor's parameters: `__this` and `i`.
            assert_eq!(ctor.params.len(), 2);
            let this_lifetime: LifetimeId =
                *ctor.params[0].type_.rs_type.lifetime_args.first().unwrap();
            let i_lifetime: LifetimeId =
                *ctor.params[1].type_.rs_type.lifetime_args.first_mut().unwrap();
            assert_eq!(i_lifetime, this_lifetime);
        }

        // Before cl/423346348 the generated Rust code would incorrectly look
        // like this (note the mismatched 'a and 'b lifetimes):
        //     fn from<'b>(i: &'a i32) -> Self
        // After this CL, this scenario will result in an explicit error.
        let err = generate_rs_api(&ir).unwrap_err();
        let msg = format!("{}", err);
        assert!(
            msg.contains("The lifetime of `__this` is unexpectedly also used by another parameter")
        );
        Ok(())
    }

    #[test]
    fn test_impl_default_non_trivial_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct NonTrivialStructWithConstructors final {
                NonTrivialStructWithConstructors();
                ~NonTrivialStructWithConstructors();  // Non-trivial
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_not_matches!(rs_api, quote! {impl Default});
        Ok(())
    }

    #[test]
    fn test_impl_from_for_explicit_conversion_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                explicit SomeStruct(int i);
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        // As discussed in b/214020567 for now we only generate `From::from` bindings
        // for *implicit* C++ conversion constructors.
        assert_rs_not_matches!(rs_api, quote! {impl From});
        Ok(())
    }

    #[test]
    fn test_impl_from_for_implicit_conversion_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                SomeStruct(int i);  // implicit - no `explicit` keyword
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        // As discussed in b/214020567 we generate `From::from` bindings for
        // *implicit* C++ conversion constructors.
        assert_rs_matches!(
            rs_api,
            quote! {
                impl From<i32> for SomeStruct {
                    #[inline(always)]
                    fn from(i: i32) -> Self {
                        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStructC1Ei(&mut tmp, i);
                            tmp.assume_init()
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_for_implicit_conversion_from_reference() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeOtherStruct final { int i; };
            struct StructUnderTest final {
                StructUnderTest(const SomeOtherStruct& other);  // implicit - no `explicit` keyword
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        // This is a regression test for b/223800038: We want to ensure that the
        // code says `impl<'b>` (instead of incorrectly declaring that lifetime
        // in `fn from<'b>`).
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> From<&'b SomeOtherStruct> for StructUnderTest {
                    #[inline(always)]
                    fn from(other: &'b SomeOtherStruct) -> Self {
                        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN15StructUnderTestC1ERK15SomeOtherStruct(
                                &mut tmp, other);
                            tmp.assume_init()
                        }
                    }
                }
            },
        );
        Ok(())
    }

    #[test]
    fn test_impl_eq_for_member_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                inline bool operator==(const SomeStruct& other) const {
                    return i == other.i;
                }
                int i;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialEq<SomeStruct> for SomeStruct {
                    #[inline(always)]
                    fn eq<'a, 'b>(&'a self, other: &'b SomeStruct) -> bool {
                        unsafe { crate::detail::__rust_thunk___ZNK10SomeStructeqERKS_(self, other) }
                    }
                }
            }
        );
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" bool __rust_thunk___ZNK10SomeStructeqERKS_(
                        const class SomeStruct* __this, const class SomeStruct& other) {
                    return __this->operator==(other);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_impl_eq_for_free_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final { int i; };
            bool operator==(const SomeStruct& lhs, const SomeStruct& rhs) {
                return lhs.i == rhs.i;
            }"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialEq<SomeStruct> for SomeStruct {
                    #[inline(always)]
                    fn eq<'a, 'b>(&'a self, rhs: &'b SomeStruct) -> bool {
                        unsafe { crate::detail::__rust_thunk___ZeqRK10SomeStructS1_(self, rhs) }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_impl_eq_non_const_member_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                bool operator==(const SomeStruct& other) /* no `const` here */;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_not_matches!(rs_api, quote! {impl PartialEq});
        Ok(())
    }

    #[test]
    fn test_impl_eq_rhs_by_value() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                bool operator==(SomeStruct other) const;
            };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_not_matches!(rs_api, quote! {impl PartialEq});
        Ok(())
    }

    #[test]
    fn test_thunk_ident_function() {
        let func = ir_func("foo");
        assert_eq!(thunk_ident(&func), make_rs_ident("__rust_thunk___Z3foov"));
    }

    #[test]
    fn test_thunk_ident_special_names() {
        let ir = ir_from_cc("struct Class {};").unwrap();

        let destructor =
            ir.functions().find(|f| f.name == UnqualifiedIdentifier::Destructor).unwrap();
        assert_eq!(thunk_ident(destructor), make_rs_ident("__rust_thunk___ZN5ClassD1Ev"));

        let default_constructor = ir
            .functions()
            .find(|f| f.name == UnqualifiedIdentifier::Constructor && f.params.len() == 1)
            .unwrap();
        assert_eq!(thunk_ident(default_constructor), make_rs_ident("__rust_thunk___ZN5ClassC1Ev"));
    }

    #[test]
    fn test_elided_lifetimes() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
          struct S final {
            int& f(int& i);
          };"#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn f<'a, 'b>(&'a mut self, i: &'b mut i32) -> &'a mut i32 { ... }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) fn __rust_thunk___ZN1S1fERi<'a, 'b>(__this: &'a mut S, i: &'b mut i32)
                    -> &'a mut i32;
            }
        );
        Ok(())
    }

    #[test]
    fn test_annotated_lifetimes() -> Result<()> {
        let ir = ir_from_cc(
            r#"[[clang::annotate("lifetimes", "a, a -> a")]]
          int& f(int& i1, int& i2);
          "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn f<'a>(i1: &'a mut i32, i2: &'a mut i32) -> &'a mut i32 { ... }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) fn __rust_thunk___Z1fRiS_<'a>(i1: &'a mut i32, i2: &'a mut i32)
                    -> &'a mut i32;
            }
        );
        Ok(())
    }

    #[test]
    fn test_format_generic_params() -> Result<()> {
        assert_rs_matches!(format_generic_params(std::iter::empty::<syn::Ident>()), quote! {});

        let idents = ["T1", "T2"].iter().map(|s| make_rs_ident(s));
        assert_rs_matches!(format_generic_params(idents), quote! { < T1, T2 > });

        let lifetimes = ["a", "b"]
            .iter()
            .map(|s| syn::Lifetime::new(&format!("'{}", s), proc_macro2::Span::call_site()));
        assert_rs_matches!(format_generic_params(lifetimes), quote! { < 'a, 'b > });

        Ok(())
    }

    #[test]
    fn test_overloaded_functions() -> Result<()> {
        // TODO(b/213280424): We don't support creating bindings for overloaded
        // functions yet, except in the case of overloaded constructors with a
        // single parameter.
        let ir = ir_from_cc(
            r#" #pragma clang lifetime_elision
                void f();
                void f(int i);
                struct S1 final {
                  void f();
                  void f(int i);
                };
                struct S2 final {
                  void f();
                };
                struct S3 final {
                  S3(int i);
                  S3(double d);
                };
            "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        let rs_api_str = tokens_to_string(rs_api.clone())?;

        // Cannot overload free functions.
        assert!(rs_api_str.contains("Error while generating bindings for item 'f'"));
        assert_rs_not_matches!(rs_api, quote! {pub fn f()});
        assert_rs_not_matches!(rs_api, quote! {pub fn f(i: i32)});

        // Cannot overload member functions.
        assert!(rs_api_str.contains("Error while generating bindings for item 'S1::f'"));
        assert_rs_not_matches!(rs_api, quote! {pub fn f(... S1 ...)});

        // But we can import member functions that have the same name as a free
        // function.
        assert_rs_matches!(rs_api, quote! {pub fn f<'a>(&'a mut self)});

        // We can also import overloaded single-parameter constructors.
        assert_rs_matches!(rs_api, quote! {impl From<i32> for S3});
        assert_rs_matches!(rs_api, quote! {impl From<f64> for S3});
        Ok(())
    }

    #[test]
    fn test_type_alias() -> Result<()> {
        let ir = ir_from_cc(
            r#"
                // MyTypedefDecl doc comment
                typedef int MyTypedefDecl;

                using MyTypeAliasDecl = int;
                using MyTypeAliasDecl_Alias = MyTypeAliasDecl;

                struct S final {};
                using S_Alias = S;
                using S_Alias_Alias = S_Alias;

                inline void f(MyTypedefDecl t) {}
            "#,
        )?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[doc = " MyTypedefDecl doc comment"]
                pub type MyTypedefDecl = i32;
            }
        );
        assert_rs_matches!(rs_api, quote! { pub type MyTypeAliasDecl = i32; });
        assert_rs_matches!(rs_api, quote! { pub type MyTypeAliasDecl_Alias = MyTypeAliasDecl; });
        assert_rs_matches!(rs_api, quote! { pub type S_Alias = S; });
        assert_rs_matches!(rs_api, quote! { pub type S_Alias_Alias = S_Alias; });
        assert_rs_matches!(rs_api, quote! { pub fn f(t: MyTypedefDecl) });
        assert_cc_matches!(
            generate_rs_api_impl(&ir)?,
            quote! {
                extern "C" void __rust_thunk___Z1fi(MyTypedefDecl t){ f (t) ; }
            }
        );
        Ok(())
    }

    #[test]
    fn test_rs_type_kind_implements_copy() -> Result<()> {
        let template = r#" LIFETIMES
            struct [[clang::trivial_abi]] TrivialStruct final { int i; };
            struct [[clang::trivial_abi]] UserDefinedCopyConstructor final {
                UserDefinedCopyConstructor(const UserDefinedCopyConstructor&);
            };
            using IntAlias = int;
            using TrivialAlias = TrivialStruct;
            using NonTrivialAlias = UserDefinedCopyConstructor;
            void func(PARAM_TYPE some_param);
        "#;
        assert_impl_all!(i32: Copy);
        assert_impl_all!(&i32: Copy);
        assert_not_impl_all!(&mut i32: Copy);
        assert_impl_all!(Option<&i32>: Copy);
        assert_not_impl_all!(Option<&mut i32>: Copy);
        assert_impl_all!(*const i32: Copy);
        assert_impl_all!(*mut i32: Copy);
        struct Test {
            // Test inputs:
            cc: &'static str,
            lifetimes: bool,
            // Expected test outputs:
            rs: &'static str,
            is_copy: bool,
        }
        let tests = vec![
            // Validity of the next few tests is verified via
            // `assert_[not_]impl_all!` static assertions above.
            Test { cc: "int", lifetimes: true, rs: "i32", is_copy: true },
            Test { cc: "const int&", lifetimes: true, rs: "&'a i32", is_copy: true },
            Test { cc: "int&", lifetimes: true, rs: "&'a mut i32", is_copy: false },
            Test { cc: "const int*", lifetimes: true, rs: "Option<&'a i32>", is_copy: true },
            Test { cc: "int*", lifetimes: true, rs: "Option<&'a mut i32>", is_copy: false },
            Test { cc: "const int*", lifetimes: false, rs: "*const i32", is_copy: true },
            Test { cc: "int*", lifetimes: false, rs: "*mut i32", is_copy: true },
            // Tests below have been thought-through and verified "manually".
            // TrivialStruct is expected to derive Copy.
            Test { cc: "TrivialStruct", lifetimes: true, rs: "TrivialStruct", is_copy: true },
            Test {
                cc: "UserDefinedCopyConstructor",
                lifetimes: true,
                rs: "UserDefinedCopyConstructor",
                is_copy: false,
            },
            Test { cc: "IntAlias", lifetimes: true, rs: "IntAlias", is_copy: true },
            Test { cc: "TrivialAlias", lifetimes: true, rs: "TrivialAlias", is_copy: true },
            Test { cc: "NonTrivialAlias", lifetimes: true, rs: "NonTrivialAlias", is_copy: false },
        ];
        for test in tests.iter() {
            let test_name = format!("cc='{}', lifetimes={}", test.cc, test.lifetimes);
            let cc_input = template.replace("PARAM_TYPE", test.cc).replace(
                "LIFETIMES",
                if test.lifetimes { "#pragma clang lifetime_elision" } else { "" },
            );
            let ir = ir_from_cc(&cc_input)?;
            let f = retrieve_func(&ir, "func");
            let t = RsTypeKind::new(&f.params[0].type_.rs_type, &ir)?;

            let lifetime_to_name: HashMap<LifetimeId, String> =
                t.lifetimes().map(|lifetime_id| (lifetime_id, "a".to_string())).collect();

            let fmt = tokens_to_string(t.format(&ir, &lifetime_to_name)?)?;
            assert_eq!(test.rs, fmt, "Testing: {}", test_name);

            assert_eq!(test.is_copy, t.implements_copy(), "Testing: {}", test_name);
        }
        Ok(())
    }

    #[test]
    fn test_rs_type_kind_is_shared_ref_to_with_lifetimes() -> Result<()> {
        let ir = ir_from_cc(
            "#pragma clang lifetime_elision
             struct SomeStruct {};
             void foo(const SomeStruct& foo_param);
             void bar(SomeStruct& bar_param);",
        )?;
        let record = ir.records().next().unwrap();
        let foo_func = retrieve_func(&ir, "foo");
        let bar_func = retrieve_func(&ir, "bar");

        // const-ref + lifetimes in C++  ===>  shared-ref in Rust
        assert_eq!(foo_func.params.len(), 1);
        let foo_param = &foo_func.params[0];
        assert_eq!(&foo_param.identifier.identifier, "foo_param");
        let foo_type = RsTypeKind::new(&foo_param.type_.rs_type, &ir)?;
        assert!(foo_type.is_shared_ref_to(record));
        assert!(matches!(foo_type, RsTypeKind::Reference { mutability: Mutability::Const, .. }));

        // non-const-ref + lifetimes in C++  ===>  mutable-ref in Rust
        assert_eq!(bar_func.params.len(), 1);
        let bar_param = &bar_func.params[0];
        assert_eq!(&bar_param.identifier.identifier, "bar_param");
        let bar_type = RsTypeKind::new(&bar_param.type_.rs_type, &ir)?;
        assert!(!bar_type.is_shared_ref_to(record));
        assert!(matches!(bar_type, RsTypeKind::Reference { mutability: Mutability::Mut, .. }));

        Ok(())
    }

    #[test]
    fn test_rs_type_kind_is_shared_ref_to_without_lifetimes() -> Result<()> {
        let ir = ir_from_cc(
            "struct SomeStruct {};
             void foo(const SomeStruct& foo_param);",
        )?;
        let record = ir.records().next().unwrap();
        let foo_func = retrieve_func(&ir, "foo");

        // const-ref + *no* lifetimes in C++  ===>  const-pointer in Rust
        assert_eq!(foo_func.params.len(), 1);
        let foo_param = &foo_func.params[0];
        assert_eq!(&foo_param.identifier.identifier, "foo_param");
        let foo_type = RsTypeKind::new(&foo_param.type_.rs_type, &ir)?;
        assert!(!foo_type.is_shared_ref_to(record));
        assert!(matches!(foo_type, RsTypeKind::Pointer { mutability: Mutability::Const, .. }));

        Ok(())
    }

    #[test]
    fn test_rs_type_kind_dfs_iter_ordering() {
        // Set up a test input representing: A<B<C>, D<E>>.
        let a = {
            let b = {
                let c = RsTypeKind::Other { name: "C", type_args: vec![] };
                RsTypeKind::Other { name: "B", type_args: vec![c] }
            };
            let d = {
                let e = RsTypeKind::Other { name: "E", type_args: vec![] };
                RsTypeKind::Other { name: "D", type_args: vec![e] }
            };
            RsTypeKind::Other { name: "A", type_args: vec![b, d] }
        };
        let dfs_names = a
            .dfs_iter()
            .map(|t| match t {
                RsTypeKind::Other { name, .. } => *name,
                _ => unreachable!("Only 'other' types are used in this test"),
            })
            .collect_vec();
        assert_eq!(vec!["A", "B", "C", "D", "E"], dfs_names);
    }

    #[test]
    fn test_rs_type_kind_dfs_iter_ordering_for_func_ptr() {
        // Set up a test input representing: fn(A, B) -> C
        let f = {
            let a = RsTypeKind::Other { name: "A", type_args: vec![] };
            let b = RsTypeKind::Other { name: "B", type_args: vec![] };
            let c = RsTypeKind::Other { name: "C", type_args: vec![] };
            RsTypeKind::FuncPtr { abi: "blah", param_types: vec![a, b], return_type: Box::new(c) }
        };
        let dfs_names = f
            .dfs_iter()
            .map(|t| match t {
                RsTypeKind::FuncPtr { .. } => "fn",
                RsTypeKind::Other { name, .. } => *name,
                _ => unreachable!("Only FuncPtr and Other kinds are used in this test"),
            })
            .collect_vec();
        assert_eq!(vec!["fn", "A", "B", "C"], dfs_names);
    }

    #[test]
    fn test_rs_type_kind_lifetimes() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            using TypeAlias = int&;
            struct SomeStruct {};
            void foo(int a, int& b, int* c, int** d, TypeAlias e, SomeStruct f); "#,
        )?;
        let f = retrieve_func(&ir, "foo");
        let ret = RsTypeKind::new(&f.return_type.rs_type, &ir)?;
        let a = RsTypeKind::new(&f.params[0].type_.rs_type, &ir)?;
        let b = RsTypeKind::new(&f.params[1].type_.rs_type, &ir)?;
        let c = RsTypeKind::new(&f.params[2].type_.rs_type, &ir)?;
        let d = RsTypeKind::new(&f.params[3].type_.rs_type, &ir)?;
        let e = RsTypeKind::new(&f.params[4].type_.rs_type, &ir)?;
        let f = RsTypeKind::new(&f.params[5].type_.rs_type, &ir)?;

        assert_eq!(0, ret.lifetimes().count()); // No lifetimes on `void`.
        assert_eq!(0, a.lifetimes().count()); // No lifetimes on `int`.
        assert_eq!(1, b.lifetimes().count()); // `&'a i32` has a single lifetime.
        assert_eq!(1, c.lifetimes().count()); // `Option<&'b i32>` has a single lifetime.
        assert_eq!(2, d.lifetimes().count()); // `&'c Option<&'d i32>` has two lifetimes.
        assert_eq!(1, e.lifetimes().count()); // Lifetime of underlying type should show through.
        assert_eq!(0, f.lifetimes().count()); // No lifetimes on structs (yet).
        Ok(())
    }

    #[test]
    fn test_rs_type_kind_lifetimes_raw_ptr() -> Result<()> {
        let ir = ir_from_cc("void foo(int* a);")?;
        let f = retrieve_func(&ir, "foo");
        let a = RsTypeKind::new(&f.params[0].type_.rs_type, &ir)?;
        assert_eq!(0, a.lifetimes().count()); // No lifetimes on `int*`.
        Ok(())
    }

    #[test]
    fn test_rust_keywords_are_escaped_in_rs_api_file() -> Result<()> {
        let ir = ir_from_cc("struct type { int dyn; };")?;
        let rs_api = generate_rs_api(&ir)?;
        assert_rs_matches!(rs_api, quote! { struct r#type { ... r#dyn: i32 ... } });
        Ok(())
    }

    #[test]
    fn test_rust_keywords_are_not_escaped_in_rs_api_impl_file() -> Result<()> {
        let ir = ir_from_cc("struct type { int dyn; };")?;
        let rs_api_impl = generate_rs_api_impl(&ir)?;
        assert_cc_matches!(rs_api_impl, quote! { static_assert(offsetof(class type, dyn) ... ) });
        Ok(())
    }

    #[test]
    fn test_no_aligned_attr() {
        let ir = ir_from_cc("struct SomeStruct {};").unwrap();
        let rs_api = generate_rs_api(&ir).unwrap();

        assert_rs_matches! {rs_api, quote! {
         #[repr(C)]
         pub struct SomeStruct { ... }
        }};
    }

    #[test]
    fn test_aligned_attr() {
        let ir = ir_from_cc("struct SomeStruct {} __attribute__((aligned(64)));").unwrap();
        let rs_api = generate_rs_api(&ir).unwrap();

        assert_rs_matches! {rs_api, quote! {
           #[repr(C, align(64))]
           pub struct SomeStruct { ... }
          }
        };
    }

    /// !Unpin references should not be pinned.
    #[test]
    fn test_nonunpin_ref_param() -> Result<()> {
        let rs_api_impl = generate_rs_api(&ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {~S();};
            void Function(const S& s);
        "#,
        )?)?;
        assert_rs_matches!(
            rs_api_impl,
            quote! {
                fn Function<'a>(s: &'a S) { ... }
            }
        );
        Ok(())
    }

    /// !Unpin mut references must be pinned.
    #[test]
    fn test_nonunpin_mut_param() -> Result<()> {
        let rs_api_impl = generate_rs_api(&ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {~S();};
            void Function(S& s);
        "#,
        )?)?;
        assert_rs_matches!(
            rs_api_impl,
            quote! {
                fn Function<'a>(s: std::pin::Pin<&'a mut S>) { ... }
            }
        );
        Ok(())
    }

    /// !Unpin &self should not be pinned.
    #[test]
    fn test_nonunpin_ref_self() -> Result<()> {
        let rs_api_impl = generate_rs_api(&ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {
              ~S();
              void Function() const;
            };
        "#,
        )?)?;
        assert_rs_matches!(
            rs_api_impl,
            quote! {
                fn Function<'a>(&'a self) { ... }
            }
        );
        Ok(())
    }

    /// !Unpin &mut self must be pinned.
    #[test]
    fn test_nonunpin_mut_self() -> Result<()> {
        let rs_api_impl = generate_rs_api(&ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {
              ~S();
              void Function();
            };
        "#,
        )?)?;
        assert_rs_matches!(
            rs_api_impl,
            quote! {
                fn Function<'a>(self: std::pin::Pin<&'a mut Self>) { ... }
            }
        );
        Ok(())
    }

    /// Drop::drop must not use self : Pin<...>.
    #[test]
    fn test_nonunpin_drop() -> Result<()> {
        let rs_api_impl = generate_rs_api(&ir_from_cc(
            r#"
            struct S {~S();};
        "#,
        )?)?;
        assert_rs_matches!(
            rs_api_impl,
            quote! {
                fn drop(&mut self) { ... }
            }
        );
        Ok(())
    }
}
