// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use code_gen_utils::{format_cc_ident, format_nonportable_cc_ident, make_rs_ident};
use crubit_abi_type::{CrubitAbiTypeToCppExprTokens, CrubitAbiTypeToCppTokens};
use database::code_snippet::{Thunk, ThunkImpl};
use database::db::BindingsGenerator;
use database::rs_snippet::{
    format_generic_params, unique_lifetimes, Lifetime, Mutability, PassingConvention, RsTypeKind,
};
use error_report::{anyhow, bail};
use ir::*;
use itertools::Itertools;
use lifetime_defaults_transform::lifetime_defaults_transform_func;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::borrow::Cow;
use std::fmt::Write;
use std::rc::Rc;
use unicode_ident::is_xid_continue;

/// If we know the original C++ function is codegenned and already compatible
/// with `extern "C"` calling convention we skip creating/calling the C++ thunk
/// since we can call the original C++ directly.
pub fn can_skip_cc_thunk<'a>(db: &BindingsGenerator<'a>, func: &Func<'a>) -> bool {
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
    if func.is_inline() {
        return false;
    }
    // ## Member functions (or descendants) of class templates
    //
    // A thunk is required to force/guarantee template instantiation.
    if func.is_member_or_descendant_of_class_template() {
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
    if let Some(inst_meta) = func.instance_method_metadata()
        && inst_meta.is_virtual()
    {
        return false;
    }
    // ## Custom calling convention requires a thunk.
    //
    // The thunk has the "C" calling convention, and internally can call the
    // C++ function using any of the calling conventions supported by the C++
    // compiler (which might not always match the set supported by Rust - e.g.,
    // abi.rs doesn't contain "swiftcall" from
    // clang::FunctionType::getNameForCallConv)
    if !func.has_c_calling_convention() {
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
    if let Ok(return_type) = db.rs_type_kind(func.return_type().clone())
        && !return_type.is_c_abi_compatible_by_value()
    {
        return false;
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
    for param in func.params() {
        if let Ok(param_type) = db.rs_type_kind(param.type_().clone())
            && !param_type.is_c_abi_compatible_by_value()
        {
            return false;
        }
    }

    // ## Conflicting mangled names.
    //
    // If there is another function that maps to the same mangled name (linker symbol),
    // we must generate a C++ thunk to avoid generating clashing Rust FFI declarations
    // that share the same `link_name` attribute.
    if db.has_conflicting_mangled_name(func) {
        return false;
    }

    true
}

#[cfg_attr(enable_heap_profiling, inline(never))]
pub fn generate_function_thunk<'a>(
    db: &BindingsGenerator<'a>,
    func: &Func<'a>,
    param_idents: &[Ident],
    param_types: &[RsTypeKind<'a>],
    return_type: &RsTypeKind<'a>,
) -> Result<Thunk> {
    let assume_lifetimes = db
        .ir()
        .target_crubit_features(func.owning_target())
        .contains(crubit_feature::CrubitFeature::AssumeLifetimes);

    // TODO(b/454627672): is it worth caching this?
    let func = if assume_lifetimes { &lifetime_defaults_transform_func(db, func)? } else { func };

    // The first parameter is the output parameter, if any.
    let mut param_types = param_types.iter();
    let mut param_idents = param_idents.iter();
    let mut out_param = None;
    let mut out_param_ident = None;

    // Elided lifetimes in return position are replaced with a named lifetime in order to avoid
    // errors in the case of multiple elided input lifetimes.
    //
    // Note: this transformation is not nested since thunk return types will only have lifetimes
    // in the case of references, as more complex lifetime types will be transformed to out-params.
    // This is somewhat fragile and could be corrected with a more complex `map` transformation
    // over `RsTypeKind`.
    let mut return_type = return_type.clone();
    let extra_return_lifetime = match &mut return_type {
        RsTypeKind::Reference { lifetime, .. } if lifetime.is_elided() => {
            *lifetime = Lifetime::new("__return_lifetime");
            Some(lifetime.clone())
        }
        _ => None,
    };

    let mut return_type_fragment = return_type.format_as_return_type_fragment(db, None);
    if *func.rs_name() == UnqualifiedIdentifier::Constructor {
        // For constructors, inject MaybeUninit into the type of `__this_` parameter.
        let Some(first_param) = param_types.next() else {
            bail!("Constructors should have at least one parameter (__this), but none were found.")
        };
        let RsTypeKind::Reference { mutability: Mutability::Mut, .. } = first_param else {
            bail!(
                "Expected first constructor parameter to be a mutable reference, got: {}",
                first_param.display(db)
            )
        };
        out_param = Some(quote! { *mut ::core::ffi::c_void });
        out_param_ident = Some(param_idents.next().unwrap().clone());
    } else {
        match return_type.passing_convention() {
            PassingConvention::ComposablyBridged => {
                out_param = Some(quote! { *mut ::core::ffi::c_uchar });
                out_param_ident = Some(make_rs_ident("__return_abi_buffer"));
                return_type_fragment = None;
            }
            // For return types that can't be passed by value, create a new out parameter.
            PassingConvention::LayoutCompatible | PassingConvention::Ctor => {
                out_param = Some(quote! { *mut ::core::ffi::c_void });
                out_param_ident = Some(make_rs_ident("__return"));
                return_type_fragment = None;
            }
            PassingConvention::AbiCompatible
            | PassingConvention::Void
            | PassingConvention::OwnedPtr => {}
        }
    }

    let mut all_types: Vec<RsTypeKind<'a>> = param_types.clone().cloned().collect();
    all_types.push(return_type.clone());

    // Of the remaining lifetimes, put them in the generic parameters.
    let mut lifetimes: Vec<_> = unique_lifetimes(&all_types, func.lifetime_inputs())
        .into_iter()
        .filter(|lifetime| !lifetime.is_elided())
        .collect();
    if let Some(extra) = extra_return_lifetime {
        if !lifetimes.contains(&extra) {
            lifetimes.push(extra);
        }
    }

    let thunk_ident = thunk_ident(db, func);

    let generic_params = format_generic_params(&lifetimes, std::iter::empty::<syn::Ident>());
    let param_idents =
        out_param_ident.as_ref().into_iter().chain(param_idents).cloned().collect_vec();
    let param_types = out_param
        .into_iter()
        .chain(param_types.map(|param_type| match param_type.passing_convention() {
            PassingConvention::ComposablyBridged => quote! { *const ::core::ffi::c_uchar },
            PassingConvention::LayoutCompatible | PassingConvention::Ctor => {
                let param_type_tokens = param_type.to_token_stream(db);
                quote! {&mut #param_type_tokens}
            }
            PassingConvention::AbiCompatible
            | PassingConvention::Void
            | PassingConvention::OwnedPtr => param_type.to_token_stream(db),
        }))
        .collect_vec();

    Ok(Thunk::Function {
        mangled_name: can_skip_cc_thunk(db, func).then(|| Rc::from(func.mangled_name())),
        thunk_ident,
        generic_params,
        param_idents,
        param_types,
        return_type_fragment,
    })
}

// Converts `mangled_name` into a string that can be used within an identifier.
// All characters in the result are guaranteed to be from the XID_Continue class (though not
// necessarily XID_Start, so the fragment can't be used at the start of an identifier).
//
// The escaping scheme is not collision-free, i.e. two different inputs may map to the same output.
// In practice, though, collisions are extremely unlikely, and other aspects of the way we create
// thunk names may also cause (very unlikely) collisions.
fn ident_fragment_from_mangled_name(mangled_name: &str) -> Cow<'_, str> {
    // LLVM identifiers use the `\01` prefix to suppress mangling:
    // https://llvm.org/docs/LangRef.html#identifiers
    // We won't be passing the name to LLVM anyway, so we simply strip the prefix if present.
    let mangled_name = mangled_name.strip_prefix('\u{1}').unwrap_or(mangled_name);

    if mangled_name.chars().all(is_xid_continue) {
        return mangled_name.into();
    }

    let mut ident_name = String::new();

    for c in mangled_name.chars() {
        if is_xid_continue(c) {
            ident_name.push(c);
        } else {
            let _ = write!(ident_name, "_u{}_", c as u32);
        }
    }

    ident_name.into()
}

/// Returns a string with a hash of the given function's mangled name and source location.
/// Using only 32 bits of the hash improves the readability of the resulting identifiers,
/// but should still keep collisions fairly unlikely.
fn compute_disambiguator_hash(func: &Func) -> String {
    use rustc_stable_hash::{FromStableHash, SipHasher128Hash, StableSipHasher128};
    use std::hash::Hasher;

    struct Hash64(u64);
    impl FromStableHash for Hash64 {
        type Hash = SipHasher128Hash;
        fn from(SipHasher128Hash([low, high]): Self::Hash) -> Self {
            Hash64(low ^ high)
        }
    }

    let mut hasher = StableSipHasher128::new();
    hasher.write(func.owning_target().as_str().as_bytes());
    hasher.write(func.source_loc().as_bytes());
    let hash: Hash64 = hasher.finish();
    format!("{:08x}_", hash.0 as u32)
}

pub fn thunk_ident<'a>(db: &BindingsGenerator<'a>, func: &Func<'a>) -> Ident {
    let disambiguator = {
        let need_disambiguation = db.has_conflicting_mangled_name(func)
            || func.is_member_or_descendant_of_class_template();
        if need_disambiguation {
            compute_disambiguator_hash(func)
        } else {
            "".to_string()
        }
    };

    format_ident!(
        "__rust_thunk__{disambiguator}{}",
        ident_fragment_from_mangled_name(func.mangled_name())
    )
}

fn generate_function_assertion_thunk_impl<'a>(
    db: &BindingsGenerator<'a>,
    func: &Func<'a>,
    unqualified_func_name: TokenStream,
) -> Result<ThunkImpl> {
    let features = db.ir().target_crubit_features(func.owning_target());
    let mut namespace_qualifier = db.namespace_qualifier(func);
    // Keep goldens the same.
    namespace_qualifier.use_leading_colons = true;
    let path_to_func = namespace_qualifier.format_for_cc(features)?;
    let implementation_function = quote! { #path_to_func #unqualified_func_name };
    let method_qualification;
    let member_function_prefix;
    let func_params;
    if let Some(instance_method_metadata) = func.instance_method_metadata() {
        let const_qualifier = if instance_method_metadata.is_const() {
            quote! {const}
        } else {
            quote! {}
        };

        method_qualification = match instance_method_metadata.reference() {
            ir::ReferenceQualification::Unqualified => const_qualifier,
            ir::ReferenceQualification::LValue => {
                quote! { #const_qualifier & }
            }
            ir::ReferenceQualification::RValue => {
                quote! { #const_qualifier && }
            }
        };
        member_function_prefix = path_to_func;
        // The first parameter of instance methods is `this`.
        func_params = &func.params()[1..];
    } else {
        method_qualification = quote! {};
        member_function_prefix = quote! {};
        func_params = func.params();
    }

    let mut cc_param_types = func_params
        .iter()
        .map(|p| {
            let mut tt = cpp_type_name::format_cpp_type_with_references(
                &db.rs_type_kind(p.type_().clone())?,
                db,
            )?;
            if p.type_().is_const() {
                tt = quote! { #tt const };
            }
            Ok(tt)
        })
        .collect::<Result<Vec<_>>>()?;
    if func.is_variadic() {
        cc_param_types.push(quote! { ... });
    }

    let mut return_type_name = cpp_type_name::format_cpp_type_with_references(
        &db.rs_type_kind(func.return_type().clone())?,
        db,
    )?;

    if func.return_type().is_const() {
        return_type_name = quote! { #return_type_name const };
    }

    let cc_calling_conv = {
        match func.call_conv() {
            Some(CcCallingConv::C) | None => quote! {},
            Some(call_conv) => quote! { __attribute__((#call_conv)) },
        }
    };

    let cc_function_type = quote! {
        #return_type_name
        ( #cc_calling_conv #member_function_prefix* )
        ( #( #cc_param_types ),* )
        #method_qualification
    };

    Ok(ThunkImpl::FunctionTypeAssertion { cc_function_type, implementation_function })
}

fn generate_function_assertion_for_identifier<'a>(
    db: &BindingsGenerator<'a>,
    func: &Func<'a>,
    id: &Identifier<'a>,
) -> Result<ThunkImpl> {
    let fn_ident = format_nonportable_cc_ident(id.as_str())?;
    generate_function_assertion_thunk_impl(db, func, quote! { #fn_ident })
}

fn generate_function_assertion_for_operator<'a>(
    db: &BindingsGenerator<'a>,
    func: &Func<'a>,
    op: &Operator<'a>,
) -> Result<ThunkImpl> {
    let op_cc_name = op.cc_name();
    let op_tokens = syn::parse_str::<TokenStream>(&op_cc_name)?;
    generate_function_assertion_thunk_impl(db, func, op_tokens)
}

pub fn generate_function_assertion<'a>(
    db: &BindingsGenerator<'a>,
    func: &Func<'a>,
) -> Result<Option<ThunkImpl>> {
    if func.adl_enclosing_record().is_some() {
        // This is a friend function that is only reachable with ADL. We can't take the address.
        return Ok(None);
    }

    if !func.has_c_calling_convention() && func.call_conv().is_none() {
        return Ok(None);
    }

    match func.cc_name() {
        UnqualifiedIdentifier::Identifier(id) => {
            Ok(Some(generate_function_assertion_for_identifier(db, func, id)?))
        }
        UnqualifiedIdentifier::Operator(op) => {
            Ok(Some(generate_function_assertion_for_operator(db, func, op)?))
        }
        UnqualifiedIdentifier::Constructor => Ok(None),
        UnqualifiedIdentifier::Destructor => Ok(None),
        UnqualifiedIdentifier::ConversionOperator => Ok(None),
    }
}

// Returns whether `func` is a copy constructor of `record_id`, assuming that `func` is a
// constructor member function of `record_id`.
// TODO(zarko): do we need to distinguish between non-const and const ctors? See b/436870965.
fn is_copy_constructor(func: &Func<'_>, record_id: ItemId) -> bool {
    let [_, other] = func.params() else {
        return false;
    };
    let CcTypeVariant::Pointer(ptr) = other.type_().variant() else {
        return false;
    };
    if ptr.kind() != PointerTypeKind::LValueRef {
        return false;
    }
    let CcTypeVariant::Decl { id, .. } = ptr.pointee_type().variant() else {
        return false;
    };
    *id == record_id
}

pub fn generate_function_thunk_impl<'a>(
    db: &BindingsGenerator<'a>,
    func: &Func<'a>,
) -> Result<Option<ThunkImpl>> {
    if can_skip_cc_thunk(db, func) {
        return Ok(None);
    }
    let thunk_ident = thunk_ident(db, func);
    let implementation_function = match func.cc_name() {
        UnqualifiedIdentifier::Operator(op) => {
            let name = syn::parse_str::<TokenStream>(op.name())?;
            quote! { operator #name }
        }
        UnqualifiedIdentifier::Identifier(id) => {
            let features = db.ir().target_crubit_features(func.owning_target());
            let fn_ident = format_nonportable_cc_ident(id.as_str())?;
            let namespace_qualifier = db.namespace_qualifier(func).format_for_cc(features)?;
            if func.instance_method_metadata().is_some() || func.adl_enclosing_record().is_some() {
                quote! {#fn_ident}
            } else {
                quote! { #namespace_qualifier #fn_ident }
            }
        }
        // Use `destroy_at` to avoid needing to spell out the class name. Destructor identiifers
        // use the name of the type itself, without namespace qualification, template
        // parameters, or aliases. We do not need to use that naming scheme anywhere else in
        // the bindings, and it can be difficult (impossible?) to spell in the general case. By
        // using destroy_at, we avoid needing to determine or remember what the correct spelling
        // is. Similar arguments apply to `construct_at`.
        UnqualifiedIdentifier::Constructor => {
            if let Some(parent_id) = func.enclosing_item_id() {
                let record: &Rc<Record> = db.find_decl(parent_id)?;
                if is_copy_constructor(func, record.id())
                    && record.copy_constructor() == SpecialMemberFunc::Unavailable
                {
                    bail!(
                        "Would use an unavailable copy constructor for {}",
                        record.cc_name().as_str()
                    );
                }
            }
            quote! { crubit::construct_at }
        }
        UnqualifiedIdentifier::Destructor => quote! {std::destroy_at},
        UnqualifiedIdentifier::ConversionOperator => {
            let target_type_cpp = cpp_type_name::format_cpp_type_with_references(
                &db.rs_type_kind(func.return_type().clone())?,
                db,
            )?;
            quote! { operator #target_type_cpp }
        }
    };

    let CcThunkParts { return_type_name, param_types, param_idents, conversion_stmts, return_stmt } =
        generate_cc_thunk_parts(db, func, ThunkCallKind::Normal(implementation_function))?;

    Ok(Some(ThunkImpl::Function {
        return_type_name,
        thunk_ident,
        param_types,
        param_idents,
        conversion_stmts,
        return_stmt,
    }))
}

/// The lowered C++ components of a function thunk, including parameter types,
/// identifiers, conversion statements, and the return expression.
pub struct CcThunkParts {
    pub return_type_name: TokenStream,
    pub param_types: Vec<TokenStream>,
    pub param_idents: Vec<Ident>,
    pub conversion_stmts: TokenStream,
    pub return_stmt: TokenStream,
}

/// Specifies the type of C++ thunk invocation being lowered: a normal named C++
/// thunk function or an inline C++ body (`inline_cpp!`).
pub enum ThunkCallKind {
    Normal(TokenStream),
    InlineCpp(TokenStream),
}

/// Lowers the parameters, return type, and C-ABI type conversions for a function
/// thunk, returning the structured C++ parts needed to emit a C++ thunk definition.
pub fn generate_cc_thunk_parts<'a>(
    db: &BindingsGenerator<'a>,
    func: &Func<'a>,
    kind: ThunkCallKind,
) -> Result<CcThunkParts> {
    let features = db.ir().target_crubit_features(func.owning_target());
    let mut param_idents = Vec::new();
    let mut param_types = Vec::new();
    let mut conversion_stmts = quote! {};
    for p in func.params().iter() {
        let ident = format_nonportable_cc_ident(p.identifier().as_str())?;
        let arg_type = db.rs_type_kind(p.type_().clone())?;
        let cpp_type = cpp_type_name::format_cpp_type(&arg_type, db)?;

        let passing_convention = arg_type.passing_convention();

        if matches!(kind, ThunkCallKind::InlineCpp(_)) {
            match passing_convention {
                PassingConvention::ComposablyBridged => {
                    let ffi_ident = format_ident!("__{ident}");
                    let crubit_abi_type = db.crubit_abi_type(arg_type.clone())?;
                    let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
                    let crubit_abi_type_expr_tokens =
                        CrubitAbiTypeToCppExprTokens(&crubit_abi_type);
                    let decoder = format_ident!("__{ident}_decoder");
                    conversion_stmts.extend(quote! {
                        ::crubit::Decoder #decoder(#crubit_abi_type_tokens::kSize, #ffi_ident);
                        auto #ident = #crubit_abi_type_expr_tokens.Decode(#decoder);
                    });
                    param_idents.push(ffi_ident);
                    param_types.push(quote! { const unsigned char* });
                }
                PassingConvention::LayoutCompatible | PassingConvention::Ctor => {
                    if arg_type.is_c_abi_compatible_by_value() {
                        param_idents.push(ident);
                        param_types.push(cpp_type);
                    } else {
                        let ffi_ident = format_ident!("__{ident}");
                        conversion_stmts.extend(quote! {
                            auto&& #ident = std::move(*#ffi_ident);
                        });
                        param_idents.push(ffi_ident);
                        param_types.push(quote! { #cpp_type * });
                    }
                }
                PassingConvention::AbiCompatible
                | PassingConvention::Void
                | PassingConvention::OwnedPtr => {
                    param_idents.push(ident);
                    param_types.push(cpp_type);
                }
            }
        } else {
            match passing_convention {
                PassingConvention::ComposablyBridged => {
                    let crubit_abi_type = db.crubit_abi_type(arg_type.clone())?;
                    let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
                    let decoder = format_ident!("__{ident}_decoder");
                    conversion_stmts.extend(quote! {
                        ::crubit::Decoder #decoder(#crubit_abi_type_tokens::kSize, #ident);
                    });
                    param_idents.push(ident);
                    param_types.push(quote! { const unsigned char* });
                }
                PassingConvention::LayoutCompatible | PassingConvention::Ctor => {
                    if arg_type.is_c_abi_compatible_by_value() {
                        param_idents.push(ident);
                        param_types.push(cpp_type);
                    } else {
                        param_idents.push(ident);
                        param_types.push(quote! { #cpp_type * });
                    }
                }
                PassingConvention::AbiCompatible
                | PassingConvention::Void
                | PassingConvention::OwnedPtr => {
                    param_idents.push(ident);
                    param_types.push(cpp_type);
                }
            }
        }
    }

    let arg_expressions = func
        .params()
        .iter()
        .map(|p| {
            let ident = format_nonportable_cc_ident(p.identifier().as_str())?;
            match p.type_().variant() {
                CcTypeVariant::Pointer(pointer) => match pointer.kind() {
                    PointerTypeKind::RValueRef => Ok(quote! { std::move(*#ident) }),
                    PointerTypeKind::LValueRef => Ok(quote! { *#ident }),
                    PointerTypeKind::Nullable
                    | PointerTypeKind::NonNull
                    | PointerTypeKind::Owned => Ok(quote! { #ident }),
                },
                CcTypeVariant::FuncPointer { non_null, .. } => {
                    if *non_null {
                        Ok(quote! { *#ident })
                    } else {
                        Ok(quote! { #ident })
                    }
                }
                _ => {
                    let rs_type_kind = db.rs_type_kind(p.type_().clone())?;
                    // non-Unpin types are wrapped by a pointer in the thunk.
                    match rs_type_kind.passing_convention() {
                        PassingConvention::ComposablyBridged => {
                            let crubit_abi_type = db.crubit_abi_type(rs_type_kind.clone())?;
                            let crubit_abi_type_expr_tokens =
                                CrubitAbiTypeToCppExprTokens(&crubit_abi_type);
                            let decoder = format_ident!("__{ident}_decoder");
                            Ok(quote! {
                                #crubit_abi_type_expr_tokens.Decode(#decoder)
                            })
                        }
                        PassingConvention::LayoutCompatible | PassingConvention::Ctor => {
                            if rs_type_kind.is_c_abi_compatible_by_value() {
                                Ok(quote! { std::move( #ident) })
                            } else {
                                Ok(quote! { std::move(* #ident) })
                            }
                        }
                        PassingConvention::AbiCompatible | PassingConvention::OwnedPtr => {
                            match rs_type_kind {
                                RsTypeKind::RvalueReference { .. } => {
                                    Ok(quote! { std::move(*#ident) })
                                }
                                RsTypeKind::Reference { .. } => Ok(quote! { *#ident }),
                                _ => Ok(quote! { #ident }),
                            }
                        }
                        PassingConvention::Void => unreachable!("parameter types cannot be void"),
                    }
                }
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let return_type_kind = db.rs_type_kind(func.return_type().clone())?;
    let return_type_cpp_spelling = cpp_type_name::format_cpp_type(&return_type_kind, db)?;

    let return_type_name = match return_type_kind.passing_convention() {
        PassingConvention::ComposablyBridged => {
            // Here, we add a `__return_abi_buffer` parameter if the return type can't be passed by
            // value across `extern "C"` ABI.
            param_idents.insert(0, format_cc_ident("__return_abi_buffer", features)?);
            param_types.insert(0, quote! {unsigned char *});
            quote! { void }
        }
        PassingConvention::LayoutCompatible | PassingConvention::Ctor => {
            // Here, we add a `__return` parameter if the return type can't be passed by
            // value across `extern "C"` ABI.
            param_idents.insert(0, format_cc_ident("__return", features)?);
            param_types.insert(0, quote! {#return_type_cpp_spelling *});
            quote! {void}
        }
        PassingConvention::AbiCompatible
        | PassingConvention::OwnedPtr
        | PassingConvention::Void => return_type_cpp_spelling.clone(),
    };

    let return_stmt = match kind {
        ThunkCallKind::InlineCpp(ref body_tokens) => match return_type_kind.passing_convention() {
            PassingConvention::ComposablyBridged => {
                let out_param = &param_idents[0];
                let crubit_abi_type = db.crubit_abi_type(return_type_kind)?;
                let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
                let crubit_abi_type_expr_tokens = CrubitAbiTypeToCppExprTokens(&crubit_abi_type);
                let return_expr = quote! { ([&]() #body_tokens)() };
                quote! {
                    ::crubit::Encoder __return_encoder(#crubit_abi_type_tokens::kSize, #out_param);
                    #crubit_abi_type_expr_tokens.Encode(
                        #return_expr,
                        __return_encoder
                    )
                }
            }
            PassingConvention::LayoutCompatible | PassingConvention::Ctor => {
                let out_param = &param_idents[0];
                let return_expr = quote! { ([&]() #body_tokens)() };
                quote! { new(#out_param) #return_type_cpp_spelling(#return_expr) }
            }
            PassingConvention::Void
            | PassingConvention::AbiCompatible
            | PassingConvention::OwnedPtr => quote! { #body_tokens },
        },
        ThunkCallKind::Normal(implementation_function) => {
            let (implementation_function, arg_expressions) = {
                let mut this_ref_qualification = match func.rs_name() {
                    UnqualifiedIdentifier::Constructor | UnqualifiedIdentifier::Destructor => None,
                    UnqualifiedIdentifier::Identifier(_)
                    | UnqualifiedIdentifier::Operator(_)
                    | UnqualifiedIdentifier::ConversionOperator => {
                        func.instance_method_metadata().as_ref().map(|meta| meta.reference())
                    }
                };
                if func.cc_name().is_constructor() {
                    this_ref_qualification = None;
                }
                if let Some(this_ref_qualification) = this_ref_qualification {
                    let this_param = func
                        .params()
                        .first()
                        .ok_or_else(|| anyhow!("Instance methods must have `__this` param."))?;

                    let this_arg = format_nonportable_cc_ident(this_param.identifier().as_str())?;
                    let this_dot = if this_ref_qualification == ir::ReferenceQualification::RValue {
                        quote! {std::move(*#this_arg).}
                    } else {
                        quote! {#this_arg->}
                    };
                    (quote! { #this_dot #implementation_function}, arg_expressions[1..].to_vec())
                } else {
                    (implementation_function, arg_expressions)
                }
            };
            let return_expr = quote! {#implementation_function( #( #arg_expressions ),* )};
            match return_type_kind.passing_convention() {
                PassingConvention::ComposablyBridged => {
                    let out_param = &param_idents[0];
                    let crubit_abi_type = db.crubit_abi_type(return_type_kind.clone())?;
                    let crubit_abi_type_tokens = CrubitAbiTypeToCppTokens(&crubit_abi_type);
                    let crubit_abi_type_expr_tokens =
                        CrubitAbiTypeToCppExprTokens(&crubit_abi_type);
                    quote! {
                        ::crubit::Encoder __return_encoder(#crubit_abi_type_tokens::kSize, #out_param);
                        #crubit_abi_type_expr_tokens.Encode(
                            #return_expr,
                            __return_encoder
                        )
                    }
                }
                PassingConvention::LayoutCompatible | PassingConvention::Ctor => {
                    let out_param = &param_idents[0];
                    quote! {new(#out_param) auto(#return_expr)}
                }
                PassingConvention::Void => return_expr,
                PassingConvention::AbiCompatible | PassingConvention::OwnedPtr => {
                    match func.return_type().variant() {
                        CcTypeVariant::Pointer(pointer)
                            if pointer.kind() == PointerTypeKind::LValueRef =>
                        {
                            quote! { return std::addressof( #return_expr ) }
                        }
                        CcTypeVariant::Pointer(pointer)
                            if pointer.kind() == PointerTypeKind::RValueRef =>
                        {
                            let nested_type = cpp_type_name::format_cpp_type_with_references(
                                &db.rs_type_kind(func.return_type().clone())?,
                                db,
                            )?;
                            quote! {
                                #nested_type lvalue = #return_expr;
                                return &lvalue
                            }
                        }
                        CcTypeVariant::FuncPointer { non_null: true, .. } => {
                            quote! { return & #return_expr }
                        }
                        _ => quote! { return #return_expr },
                    }
                }
            }
        }
    };

    Ok(CcThunkParts { return_type_name, param_types, param_idents, conversion_stmts, return_stmt })
}

/// Generates an `inline_cpp!` macro invocation expression in Rust for `func`,
/// reusing standard C-ABI parameter and return type lowering logic.
///
/// Returns `Ok(None)` if the function cannot be emitted as an `inline_cpp!` expression,
/// such as constructors and destructors which cannot be called directly as free expressions.
pub fn generate_inline_cpp_call<'a>(
    db: &BindingsGenerator<'a>,
    func: &Func<'a>,
    thunk_args: &[TokenStream],
    body_tokens: TokenStream,
) -> Result<Option<TokenStream>> {
    if func.cc_name().is_constructor() || func.cc_name().is_destructor() {
        return Ok(None);
    }

    let CcThunkParts { return_type_name, param_types, param_idents, conversion_stmts, return_stmt } =
        generate_cc_thunk_parts(db, func, ThunkCallKind::InlineCpp(body_tokens))?;

    let adjusted_thunk_args = thunk_args
        .iter()
        .zip(func.params().iter())
        .map(|(arg, param)| {
            let rs_type = db.rs_type_kind(param.type_().clone())?;
            Ok(match rs_type {
                RsTypeKind::Record { .. }
                | RsTypeKind::Reference { .. }
                | RsTypeKind::Pointer { .. } => quote! { (#arg as *const _) },
                _ => quote! { #arg },
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let return_type_kind = db.rs_type_kind(func.return_type().clone())?;
    let body_block = if conversion_stmts.is_empty() {
        match return_type_kind.passing_convention() {
            PassingConvention::ComposablyBridged
            | PassingConvention::LayoutCompatible
            | PassingConvention::Ctor => quote! { { #return_stmt; } },
            _ => quote! { #return_stmt },
        }
    } else {
        match return_type_kind.passing_convention() {
            PassingConvention::ComposablyBridged
            | PassingConvention::LayoutCompatible
            | PassingConvention::Ctor => quote! {
                {
                    #conversion_stmts
                    #return_stmt;
                }
            },
            _ => quote! {
                {
                    #conversion_stmts
                    #return_stmt
                }
            },
        }
    };

    Ok(Some(quote! {
        unsafe {
            (::crubit_support::inline_cpp! {
                (
                #(#param_types #param_idents), *)->#return_type_name
                #body_block
            })( #( #adjusted_thunk_args ),* )
        }
    }))
}
