// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::generate_function_thunk::replace_all_regions_with_static;
use crate::generate_struct_and_union::{
    generate_associated_item, generate_relocating_ctor, has_type_or_const_vars,
    scalar_value_to_string,
};
use crate::generate_unsupported_def;
use arc_anyhow::{bail, Error, Result};
use code_gen_utils::{escape_non_identifier_chars, CcInclude};
use database::code_snippet::{
    ApiSnippets, CcPrerequisites, CcSnippet, EnumSpecializationKind, FormattedTy,
    RsStdEnumSpecialization, RsStdSpecializationArgs, RsStdTemplateSpecialization,
    TemplateSpecialization, TraitImplTemplateSpecialization,
};
use database::{BindingsGenerator, StaticMethodMode, TypeLocation};
use error_report::anyhow;
use itertools::Itertools;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use query_compiler::{get_layout, post_analysis_typing_env};
use quote::{format_ident, quote};
#[rustversion::nightly]
use rustc_abi::LayoutData;
use rustc_abi::{Layout, VariantIdx};
use rustc_middle::ty::layout::PrimitiveExt;
#[rustversion::since(2026-04-22)]
use rustc_middle::ty::Flags;
#[rustversion::since(2026-04-20)]
use rustc_middle::ty::Unnormalized;
use rustc_middle::ty::{self, AdtDef, Ty, TyCtxt, TypingEnv};
use rustc_span::def_id::DefId;
use std::collections::HashSet;
use std::rc::Rc;

fn is_cpp_movable<'tcx>(db: &BindingsGenerator<'tcx>, ty: Ty<'tcx>) -> bool {
    ty.ty_adt_def()
        .map(|adt| db.has_move_ctor_and_assignment_operator(Some(adt.did()), ty).is_some())
        // Primitive types bind to C++ primitives that support move construction and assignment.
        .unwrap_or_else(|| ty.is_primitive_ty())
}

pub(crate) fn parse_rs_std_template_specialization<'tcx>(
    db: &BindingsGenerator<'tcx>,
    self_ty: Ty<'tcx>,
) -> Option<Result<RsStdTemplateSpecialization<'tcx>>> {
    let tcx = db.tcx();
    #[rustversion::before(2026-04-20)]
    let unnorm_ty = self_ty;
    #[rustversion::since(2026-04-20)]
    let unnorm_ty = Unnormalized::new(self_ty);
    let self_ty = replace_all_regions_with_static(
        tcx,
        tcx.normalize_erasing_regions(ty::TypingEnv::fully_monomorphized(), unnorm_ty),
    );
    // If our specialization contains a status type from additonal srcs, we should not generate a
    // specialization for it.
    if self_ty.walk().any(|arg| {
        arg.as_type()
            .and_then(|ty| ty.ty_adt_def())
            .is_some_and(|adt| !crate::should_receive_bindings(db, adt.did()))
    }) {
        return None;
    }

    match self_ty.kind() {
        ty::TyKind::Adt(adt, substs) => {
            parse_adt_template_specialization(db, self_ty, *adt, substs)
        }
        ty::TyKind::Tuple(types) if !types.is_empty() => {
            parse_tuple_template_specialization(db, self_ty, types)
        }
        _ => None,
    }
}

fn parse_adt_template_specialization<'tcx>(
    db: &BindingsGenerator<'tcx>,
    self_ty: Ty<'tcx>,
    adt: ty::AdtDef<'tcx>,
    substs: ty::GenericArgsRef<'tcx>,
) -> Option<Result<RsStdTemplateSpecialization<'tcx>>> {
    use crate::BridgedBuiltin;
    use database::code_snippet::EnumSpecializationKind;
    let tcx = db.tcx();
    BridgedBuiltin::new(db, adt).map(|bridged_builtin| {
        if self_ty.walk().any(|arg| arg.as_type().is_some_and(|ty| ty.is_ptr_sized_integral())) {
            bail!("b/491106325 - isize and usize types are not yet supported as generic type arguments.")
        }
        match bridged_builtin {
            BridgedBuiltin::Option => {
                let some_ty = FormattedTy::try_from_ty(
                    substs.type_at(0),
                    TypeLocation::Other,
                    db,
                )?;
                let layout = get_layout(tcx, self_ty)?;

                let tag = match layout.variants() {
                    rustc_abi::Variants::Empty => {
                        unreachable!("Option is never uninhabited because of the None variant.")
                    }
                    rustc_abi::Variants::Single { .. } => {
                        unreachable!(
                            "This case only occurs when our Some variant contains an uninhabited \
                        type. This is unsupported today and we call format_ty_for_cc on our \
                        argument type before queueing a specialization, so this case should not \
                        occur in practice."
                        )
                    }
                    rustc_abi::Variants::Multiple { tag, .. } => tag,
                };
                let tag_type_rs = tag.primitive().to_int_ty(tcx);
                let tag_type_cc = db.format_ty_for_cc(tag_type_rs, TypeLocation::Other)?;
                let self_ty_cc = {
                    let mut prereqs = CcPrerequisites::default();
                    let some_ty_cc = some_ty.for_cc.clone().into_tokens(&mut prereqs);
                    prereqs.forward_declare_type(substs.type_at(0));
                    CcSnippet { tokens: quote! { rs_std::Option<#some_ty_cc> }, prereqs }
                };
                Ok(RsStdTemplateSpecialization {
                    layout,
                    self_ty_rs: self_ty,
                    self_ty_cc,
                    args: RsStdSpecializationArgs::Enum(RsStdEnumSpecialization {
                        tag_type_rs,
                        tag_type_cc: tag_type_cc.clone(),
                        kind: EnumSpecializationKind::Option { some_ty },
                    }),
                })
            }
            BridgedBuiltin::Result => {
                let ok_ty = FormattedTy::try_from_ty(
                    substs.type_at(0),
                    TypeLocation::Other,
                    db,
                )?;
                let err_ty = FormattedTy::try_from_ty(
                    substs.type_at(1),
                    TypeLocation::Other,
                    db,
                )?;

                let layout = get_layout(tcx, self_ty)?;
                let tag = match layout.variants() {
                    rustc_abi::Variants::Empty => {
                        unreachable!(
                            "Result is only uninhabited when both Ok and Err are uninhabited. We do \
                    not support uninhabited types today, and we format our Ok and Err type before queueing \
                    this specialization, so this case should not occur in practice."
                        )
                    }
                    rustc_abi::Variants::Single { .. } => {
                        unreachable!(
                            "Result only has a single variant when either Ok or Err is uninhabited. We do \
                    not support uninhabited types today, and we format our Ok and Err type before queueing \
                    this specialization, so this case should not occur in practice."
                        )
                    }
                    rustc_abi::Variants::Multiple { tag, .. } => {
                        // We only need to check the tag now. We'll use the rest of the fields
                        // in `generate_template_specialization` below.
                        tag
                    }
                };

                let tag_type_rs = tag.primitive().to_int_ty(tcx);
                let tag_type_cc = db.format_ty_for_cc(tag_type_rs, TypeLocation::Other)?;
                let self_ty_cc = {
                    let mut prereqs = CcPrerequisites::default();
                    let ok_ty_cc = ok_ty.for_cc.clone().into_tokens(&mut prereqs);
                    let err_ty_cc = err_ty.for_cc.clone().into_tokens(&mut prereqs);
                    prereqs.forward_declare_type(substs.type_at(0));
                    prereqs.forward_declare_type(substs.type_at(1));
                    CcSnippet {
                        tokens: quote! { rs_std::Result<#ok_ty_cc, #err_ty_cc> },
                        prereqs,
                    }
                };
                Ok(RsStdTemplateSpecialization {
                    layout,
                    self_ty_rs: self_ty,
                    self_ty_cc,
                    args: RsStdSpecializationArgs::Enum(RsStdEnumSpecialization {
                        tag_type_rs,
                        tag_type_cc: tag_type_cc.clone(),
                        kind: EnumSpecializationKind::Result { ok_ty, err_ty },
                    }),
                })
            }
            BridgedBuiltin::Vec => {
                let inner_ty = FormattedTy::try_from_ty(
                    substs.type_at(0),
                    TypeLocation::Other,
                    db,
                )?;
                let layout = get_layout(tcx, self_ty)?;
                let self_ty_cc = {
                    let mut prereqs = CcPrerequisites::default();
                    let inner_ty_cc = inner_ty.for_cc.clone().into_tokens(&mut prereqs);
                    prereqs.forward_declare_type(substs.type_at(0));
                    CcSnippet { tokens: quote! { rs_std::Vec<#inner_ty_cc> }, prereqs }
                };
                Ok(RsStdTemplateSpecialization {
                    layout,
                    self_ty_rs: self_ty,
                    self_ty_cc,
                    args: RsStdSpecializationArgs::Vec(inner_ty),
                })
            }
        }
    })
}

fn parse_tuple_template_specialization<'tcx>(
    db: &BindingsGenerator<'tcx>,
    self_ty: Ty<'tcx>,
    types: &'tcx ty::List<Ty<'tcx>>,
) -> Option<Result<RsStdTemplateSpecialization<'tcx>>> {
    let tcx = db.tcx();
    let element_tys = types
        .iter()
        .map(|ty| {
            FormattedTy::try_from_ty(
                replace_all_regions_with_static(tcx, ty),
                TypeLocation::Other,
                db,
            )
        })
        .collect::<Result<Vec<_>>>()
        .ok()?;

    let layout = get_layout(tcx, self_ty).ok()?;
    let self_ty_cc = {
        let mut prereqs = CcPrerequisites::default();
        let element_tys_cc = element_tys
            .iter()
            .map(|ty| ty.for_cc.clone().into_tokens(&mut prereqs))
            .collect::<Vec<_>>();
        for ty in types.iter() {
            prereqs.forward_declare_type(ty);
        }
        CcSnippet { tokens: quote! { rs_std::Tuple<#(#element_tys_cc),*> }, prereqs }
    };
    Some(Ok(RsStdTemplateSpecialization {
        layout,
        self_ty_rs: self_ty,
        self_ty_cc,
        args: RsStdSpecializationArgs::Tuple(element_tys),
    }))
}

struct OptionApiGenerator<'tcx> {
    arg_ty: TokenStream,
    needs_drop: bool,
    // Reads our tag out of our Option<T> and defines a variable `tag` pointing at it's value.
    // This is complicated by the niche optimization, so we abstract it as this reference so it's
    // easy to reference.
    tag_method: ApiSnippets<'tcx>,
    none_val: TokenStream,
    write_some_to_tag: TokenStream,
    some_ptr_val: TokenStream,
    some_const_ptr_val: TokenStream,
    tag_type_cc: TokenStream,
}

impl<'tcx> OptionApiGenerator<'tcx> {
    fn api_snippets(self) -> ApiSnippets<'tcx> {
        let Self {
            arg_ty,
            needs_drop,
            tag_method,
            none_val,
            write_some_to_tag,
            some_ptr_val,
            some_const_ptr_val,
            tag_type_cc,
            ..
        } = self;
        let mut prereqs = CcPrerequisites::default();

        let (drop, drop_details) = if needs_drop {
            (
                quote! {
                    constexpr ~Option() noexcept;
                },
                quote! {
                    inline constexpr rs_std::Option<#arg_ty>::~Option() noexcept {
                        this->reset();
                    }
                },
            )
        } else {
            prereqs.includes.insert(CcInclude::type_traits());
            (
                quote! {
                    ~Option() noexcept = default;
                },
                quote! { static_assert(::std::is_trivially_destructible_v<rs_std::Option<#arg_ty>>); },
            )
        };

        let tag_method_main_api = tag_method.main_api.into_tokens(&mut prereqs);
        let main_api = CcSnippet {
            tokens: quote! {
                using base_type = rs_std::OptionBase<rs_std::Option<#arg_ty>, #arg_ty>;
                constexpr Option() = default;
                constexpr Option(::std::nullopt_t) noexcept;
                constexpr Option& operator=(::std::nullopt_t) noexcept;

                template <typename U>
                  requires(!std::is_base_of_v<Option, std::decay_t<U>> &&
                           !std::is_same_v<std::decay_t<U>, ::std::nullopt_t> &&
                           !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
                           std::is_constructible_v<#arg_ty, U>)
                Option(U&& value) noexcept : base_type(::std::forward<U>(value)) {}

                template <typename U>
                  requires(!std::is_base_of_v<Option, std::decay_t<U>> &&
                           !std::is_same_v<std::decay_t<U>, ::std::nullopt_t> &&
                           !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
                           std::is_constructible_v<#arg_ty, U>)
                Option& operator=(U&& value) noexcept {
                    base_type::operator=(::std::forward<U>(value));
                    return *this;
                }

                template <typename Opt>
                  requires(std::is_same_v<std::decay_t<Opt>, ::std::optional<#arg_ty>> &&
                           !std::is_lvalue_reference_v<Opt>)
                Option(Opt&& value) noexcept : base_type(::std::forward<Opt>(value)) {}

                template <typename Opt>
                  requires(std::is_same_v<std::decay_t<Opt>, ::std::optional<#arg_ty>> &&
                           !std::is_lvalue_reference_v<Opt>)
                Option& operator=(Opt&& value) noexcept {
                    base_type::operator=(::std::forward<Opt>(value));
                    return *this;
                }

                template <typename... Args>
                explicit Option(::std::in_place_t ip, Args&&... args) noexcept
                    : base_type(ip, ::std::forward<Args>(args)...) {}

                #drop

            private:
                friend base_type;
                using tag_type = #tag_type_cc;
                static constexpr tag_type kNoneVal = #none_val;

                #arg_ty* some_ptr() noexcept {
                    return #some_ptr_val;
                }
                #arg_ty const* some_const_ptr() const noexcept {
                    return #some_const_ptr_val;
                }
                void set_some_tag() noexcept {
                    #write_some_to_tag
                }
                constexpr void set_none_tag() noexcept {
                    set_tag(kNoneVal);
                }
                constexpr bool is_none() const noexcept {
                    return tag() == kNoneVal;
                }

                #tag_method_main_api
            },
            prereqs,
        };

        let mut prereqs = CcPrerequisites::default();
        prereqs.includes.insert(CcInclude::utility());
        let tag_method_cc_details = tag_method.cc_details.into_tokens(&mut prereqs);
        let cc_details = CcSnippet {
            tokens: quote! {
                #drop_details __NEWLINE__
                #tag_method_cc_details __NEWLINE__

                inline constexpr rs_std::Option<#arg_ty>::Option(::std::nullopt_t) noexcept : base_type(::std::nullopt) {} __NEWLINE__
                inline constexpr rs_std::Option<#arg_ty>& rs_std::Option<#arg_ty>::operator=(::std::nullopt_t) noexcept {
                    base_type::operator=(::std::nullopt);
                    return *this;
                } __NEWLINE__
            },
            prereqs,
        };
        ApiSnippets { main_api, cc_details, ..Default::default() }
    }
}

struct OptionVariantIndices {
    some_idx: VariantIdx,
    none_idx: VariantIdx,
}

fn get_option_variant_indices<'tcx>(tcx: TyCtxt<'tcx>, adt: AdtDef<'tcx>) -> OptionVariantIndices {
    let (mut some_idx, mut none_idx) = (None, None);
    for (idx, variant) in adt.variants().iter_enumerated() {
        use rustc_hir::LangItem;
        match tcx.lang_items().from_def_id(variant.def_id) {
            Some(LangItem::OptionSome) => some_idx = Some(idx),
            Some(LangItem::OptionNone) => none_idx = Some(idx),
            _ => unreachable!("Option<T> must only have a Some and None variant"),
        }
    }
    OptionVariantIndices {
        some_idx: some_idx.expect("Option<T> must have a Some variant"),
        none_idx: none_idx.expect("Option<T> must have a None variant"),
    }
}

fn literal_of_tag_ty<'tcx>(tcx: TyCtxt<'tcx>, val: u128, ty: Ty<'tcx>) -> TokenStream {
    use rustc_middle::mir::interpret::Scalar;
    use rustc_middle::ty::ScalarInt;
    let (size, _) = ty.int_size_and_signed(tcx);
    let (scalar_int, _) = ScalarInt::truncate_from_uint(val, size);
    scalar_value_to_string(tcx, Scalar::Int(scalar_int), *ty.kind())
        .and_then(|s| {
            s.parse::<TokenStream>()
                .map_err(|_| anyhow!("scalar_value_to_string produced invalid tokens"))
        })
        .expect("tag to be valid scalar tokens")
}

struct ResultVariantIndices {
    ok_idx: VariantIdx,
    err_idx: VariantIdx,
}

fn get_result_variant_indices<'tcx>(tcx: TyCtxt<'tcx>, adt: AdtDef<'tcx>) -> ResultVariantIndices {
    let (mut ok_idx, mut err_idx) = (None, None);
    for (idx, variant) in adt.variants().iter_enumerated() {
        use rustc_hir::LangItem;
        match tcx.lang_items().from_def_id(variant.def_id) {
            Some(LangItem::ResultOk) => ok_idx = Some(idx),
            Some(LangItem::ResultErr) => err_idx = Some(idx),
            _ => unreachable!("Result<T, E> must only have an Ok and Err variant"),
        }
    }
    ResultVariantIndices {
        ok_idx: ok_idx.expect("Result<T, E> must have an Ok variant"),
        err_idx: err_idx.expect("Result<T, E> must have an Err variant"),
    }
}

struct ResultApiGenerator<'a, 'tcx> {
    db: &'a BindingsGenerator<'tcx>,
    ok_ty_rs: Ty<'tcx>,
    ok_ty_cpp: TokenStream,
    err_ty_rs: Ty<'tcx>,
    err_ty_cpp: TokenStream,
    needs_drop: bool,
    tag_method: ApiSnippets<'tcx>,
    has_value_impl: TokenStream,
    write_ok_to_tag: TokenStream,
    write_err_to_tag: TokenStream,
    ok_ptr_val: TokenStream,
    err_ptr_val: TokenStream,
}

impl<'tcx> ResultApiGenerator<'_, 'tcx> {
    fn api_snippets(self) -> ApiSnippets<'tcx> {
        let Self {
            ok_ty_cpp,
            err_ty_cpp,
            needs_drop,
            tag_method,
            has_value_impl,
            ok_ptr_val,
            err_ptr_val,
            write_ok_to_tag,
            write_err_to_tag,
            ..
        } = self;
        let mut prereqs = CcPrerequisites::default();
        let full_self_ty = quote! { rs_std::Result<#ok_ty_cpp, #err_ty_cpp> };

        let (drop, drop_details) = if needs_drop {
            (
                quote! { ~Result() noexcept; },
                quote! {
                    inline #full_self_ty::~Result() noexcept {
                        this->Reset();
                    }
                },
            )
        } else {
            (
                quote! { ~Result() noexcept = default; },
                quote! { static_assert(::std::is_trivially_destructible_v<#full_self_ty>); },
            )
        };

        let tag_method_main_api = tag_method.main_api.into_tokens(&mut prereqs);
        let main_api = CcSnippet {
            tokens: quote! {
            public:
                using base_type = rs_std::ResultBase<rs_std::Result<#ok_ty_cpp, #err_ty_cpp>, #ok_ty_cpp, #err_ty_cpp>;

                template <typename U>
                  requires(!std::is_base_of_v<Result, std::decay_t<U>> &&
                           !rs_std::is_unexpected_v<std::decay_t<U>> &&
                           !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
                           !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
                           std::is_constructible_v<#ok_ty_cpp, U>)
                explicit constexpr Result(U&& ok) noexcept : base_type(::std::forward<U>(ok)) {}

                template <typename U>
                  requires(!std::is_base_of_v<Result, std::decay_t<U>> &&
                           !rs_std::is_unexpected_v<std::decay_t<U>> &&
                           !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
                           std::is_constructible_v<#ok_ty_cpp, U>)
                constexpr Result& operator=(U&& ok) noexcept {
                    base_type::operator=(::std::forward<U>(ok));
                    return *this;
                }

                template <typename F>
                  requires(std::is_constructible_v<#err_ty_cpp, F>)
                explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept : base_type(::std::move(err)) {}

                template <typename F>
                  requires(std::is_constructible_v<#err_ty_cpp, F>)
                constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept {
                    base_type::operator=(::std::move(err));
                    return *this;
                }

                template <typename... Args>
                explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept
                    : base_type(ip, ::std::forward<Args>(args)...) {}

                template <typename... Args>
                explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept
                    : base_type(u, ::std::forward<Args>(args)...) {}

                #drop

            private:
                friend base_type;

                bool has_value_impl() const noexcept {
                    return #has_value_impl;
                }
                #ok_ty_cpp* ok_ptr() noexcept {
                    return reinterpret_cast<#ok_ty_cpp*>(#ok_ptr_val);
                }
                #ok_ty_cpp const* ok_const_ptr() const noexcept {
                    return reinterpret_cast<#ok_ty_cpp const*>(#ok_ptr_val);
                }
                #err_ty_cpp* err_ptr() noexcept {
                    return reinterpret_cast<#err_ty_cpp*>(#err_ptr_val);
                }
                #err_ty_cpp const* err_const_ptr() const noexcept {
                    return reinterpret_cast<#err_ty_cpp const*>(#err_ptr_val);
                }
                void set_ok_tag() noexcept {
                    #write_ok_to_tag
                }
                void set_err_tag() noexcept {
                    #write_err_to_tag
                }

                #tag_method_main_api
            },
            prereqs,
        };

        let mut prereqs = CcPrerequisites::default();
        prereqs.includes.insert(CcInclude::utility());
        let tag_method_cc_details = tag_method.cc_details.into_tokens(&mut prereqs);
        let cc_details = CcSnippet {
            tokens: quote! {
                #drop_details __NEWLINE__
                #tag_method_cc_details
            },
            prereqs,
        };

        ApiSnippets { main_api, cc_details, ..Default::default() }
    }
}

struct TupleApiGenerator<'a, 'tcx> {
    db: &'a BindingsGenerator<'tcx>,
    element_tys: Vec<FormattedTy<'tcx>>,
    self_ty: Ty<'tcx>,
    layout: rustc_abi::Layout<'tcx>,
}

impl<'tcx> TupleApiGenerator<'_, 'tcx> {
    fn api_snippets(self) -> ApiSnippets<'tcx> {
        let mut prereqs = CcPrerequisites::default();
        prereqs.includes.insert(CcInclude::tuple());
        prereqs.includes.insert(CcInclude::utility());

        let element_cc_tys: Vec<_> =
            self.element_tys.iter().map(|ty| ty.for_cc.clone().into_tokens(&mut prereqs)).collect();
        let full_self_ty = quote! { rs_std::Tuple<#(#element_cc_tys),*> };

        let mut construct_elements = quote! {};
        let mut convert_elements = Vec::new();
        for (i, element_cc_ty) in element_cc_tys.iter().enumerate() {
            let offset = Literal::u64_unsuffixed(self.layout.fields().offset(i).bytes());
            let element_ptr = quote! { reinterpret_cast<#element_cc_ty*>(storage_ + #offset) };
            let i_idx = Literal::usize_unsuffixed(i);

            construct_elements.extend(quote! {
                std::construct_at(#element_ptr, std::move(std::get<#i_idx>(tuple)));
            });
            convert_elements.push(quote! {
                    std::move(*#element_ptr)
            });
        }

        let needs_drop = self.self_ty.needs_drop(self.db.tcx(), TypingEnv::fully_monomorphized());

        let (drop_decl, drop_impl) = if needs_drop {
            let mut drop_elements = quote! {};
            for (i, element_cc_ty) in element_cc_tys.iter().enumerate() {
                let offset = Literal::u64_unsuffixed(self.layout.fields().offset(i).bytes());
                drop_elements.extend(quote! {
                    std::destroy_at(reinterpret_cast<#element_cc_ty*>(storage_ + #offset));
                });
            }
            (
                quote! { ~Tuple(); },
                quote! {
                    inline #full_self_ty::~Tuple() {
                        #drop_elements
                    }
                },
            )
        } else {
            (quote! { ~Tuple() = default; }, quote! {})
        };

        let all_elements_cpp_movable = self.element_tys.iter().all(|element| {
            self.db.has_move_ctor_and_assignment_operator(None, element.ty).is_some()
        });

        let (std_tuple_main_api_ctor, std_tuple_main_api_conv, std_tuple_cc_details) =
            if all_elements_cpp_movable {
                (
                    quote! { Tuple(std::tuple<#(#element_cc_tys),*>&& tuple) noexcept; },
                    quote! { operator std::tuple<#(#element_cc_tys),*>() && noexcept; },
                    quote! {
                        inline #full_self_ty::Tuple(std::tuple<#(#element_cc_tys),*>&& tuple) noexcept {
                            #construct_elements
                        } __NEWLINE__
                        inline #full_self_ty::operator std::tuple<#(#element_cc_tys),*>() && noexcept {
                            return std::tuple<#(#element_cc_tys),*>(#(#convert_elements),*);
                        }
                    },
                )
            } else {
                (
                    quote! { Tuple(std::tuple<#(#element_cc_tys),*>&& tuple) = delete; },
                    quote! { operator std::tuple<#(#element_cc_tys),*>() && = delete; },
                    quote! {},
                )
            };

        ApiSnippets {
            main_api: CcSnippet {
                tokens: quote! {
                    #std_tuple_main_api_ctor
                    #drop_decl
                    #std_tuple_main_api_conv
                },
                prereqs,
            },
            cc_details: CcSnippet::new(quote! {
                #std_tuple_cc_details __NEWLINE__
                #drop_impl __NEWLINE__
            }),
            ..Default::default()
        }
    }
}

fn specialize_tuple<'tcx>(
    db: &BindingsGenerator<'tcx>,
    rs_std: &RsStdTemplateSpecialization<'tcx>,
    element_tys: Vec<FormattedTy<'tcx>>,
) -> ApiSnippets<'tcx> {
    let layout = rs_std.layout;
    let mut prereqs = CcPrerequisites::default();
    let element_cc_tys = element_tys
        .iter()
        .map(|ty| {
            db.format_ty_for_cc(ty.ty, TypeLocation::Field).unwrap().into_tokens(&mut prereqs)
        })
        .collect_vec();

    let tuple_api = TupleApiGenerator {
        db,
        element_tys: element_tys.clone(),
        self_ty: rs_std.self_ty_rs,
        layout,
    };

    let rs_fully_qualified_name = {
        let element_rs_tys = element_tys.iter().map(|ty| &ty.for_rs);
        quote! { (#(#element_rs_tys,)*) }
    };
    let cc_fully_qualified_name = quote! { ::rs_std::Tuple<#(#element_cc_tys),*> };

    let core = Rc::new(database::AdtCoreBindings {
        def_id: None,
        keyword: quote! { struct },
        cc_short_name: format_ident!("Tuple"),
        rs_fully_qualified_name: rs_fully_qualified_name.clone(),
        cc_fully_qualified_name: cc_fully_qualified_name.clone(),
        lifetime_params: vec![],
        self_ty: rs_std.self_ty_rs,
        alignment_in_bytes: layout.align().abi.bytes(),
        size_in_bytes: layout.size().bytes(),
    });

    let copy_ctor_and_assignment_snippets =
        db.generate_copy_ctor_and_assignment_operator(core.clone()).unwrap_or_else(|err| err);
    let move_ctor_and_assignment_snippets = db
        .generate_move_ctor_and_assignment_operator(core.clone())
        .unwrap_or_else(|err| err.explicitly_deleted);
    let relocating_ctor_snippets = generate_relocating_ctor(db, &core.cc_short_name);
    let default_ctor_snippets = db.generate_default_ctor(core.clone()).unwrap_or_else(|err| err);

    let ApiSnippets { main_api, cc_details, rs_details } = [
        default_ctor_snippets,
        copy_ctor_and_assignment_snippets,
        move_ctor_and_assignment_snippets,
        relocating_ctor_snippets,
        tuple_api.api_snippets(),
    ]
    .into_iter()
    .collect();

    let main_api_tokens = main_api.into_tokens(&mut prereqs);
    let qualified_name = cc_fully_qualified_name.to_string();
    let name = escape_non_identifier_chars(&qualified_name);
    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_{}", name);
    let size_literal = Literal::u64_unsuffixed(layout.size().bytes());
    let align_literal = Literal::u64_unsuffixed(layout.align().abi.bytes());
    let internal_rust_type_string = rs_fully_qualified_name.to_string();

    let main_api_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        template<> __NEWLINE__
        struct alignas(#align_literal)
        CRUBIT_INTERNAL_RUST_TYPE(#internal_rust_type_string)
        rs_std::Tuple<#(#element_cc_tys),*> { __NEWLINE__
        public:
            #main_api_tokens __NEWLINE__
        private:
            unsigned char storage_[#size_literal]; __NEWLINE__
        }; __NEWLINE__
        __HASH_TOKEN__ endif __NEWLINE__
    };

    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_IMPL_{}", name);
    let cc_details_tokens = cc_details.into_tokens(&mut prereqs);
    let cc_details_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        #cc_details_tokens __NEWLINE__
        __HASH_TOKEN__ endif __NEWLINE__
    };

    ApiSnippets {
        main_api: CcSnippet { tokens: main_api_tokens, prereqs },
        cc_details: CcSnippet::new(cc_details_tokens),
        rs_details,
    }
}

fn find_pointer_field_offset<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<u64> {
    find_pointer_field_offset_impl(tcx, ty, 0)
}

fn find_pointer_field_offset_impl<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
    current_offset: u64,
) -> Option<u64> {
    let kind = ty.kind();

    if matches!(kind, ty::TyKind::RawPtr(_, _)) {
        return Some(current_offset);
    }

    if let ty::TyKind::Pat(underlying_ty, _) = kind {
        return find_pointer_field_offset_impl(tcx, *underlying_ty, current_offset);
    }

    if let ty::TyKind::Adt(adt_def, args) = kind {
        if adt_def.is_enum() {
            return None;
        }

        let layout = get_layout(tcx, ty).ok()?;
        let variant = adt_def.non_enum_variant();

        for (i, field) in variant.fields.iter().enumerate() {
            let field_offset = layout.fields().offset(i).bytes();
            let field_ty = field.ty(tcx, args);
            let field_ty = crate::normalize_ty(tcx, tcx.param_env(field.did), field_ty);

            if let Some(offset) =
                find_pointer_field_offset_impl(tcx, field_ty, current_offset + field_offset)
            {
                return Some(offset);
            }
        }
    }

    None
}

/// Computes the offsets of the pointer and length fields of `Vec`.
///
/// At the top level of `Vec<T>`, the layout consists of:
/// - `len: usize` (primitive)
/// - `buf: RawVec<T>` (ADT struct)
///
/// Because `capacity` is nested inside `buf` (at a deeper level), `len` is the
/// only top-level field of type `usize`. This function exploits this structure:
/// 1. It finds `len` by looking only at the top-level fields of `Vec` for a
///    `usize` type.
/// 2. It finds the data pointer by recursively searching the other top-level
///    ADT fields for a raw pointer type.
///
/// This allows us to find the offsets without relying on the names of `buf`,
/// `len`, `inner`, `ptr`, or `cap`.
struct VecLayoutOffsets {
    ptr_offset: u64,
    len_offset: u64,
}

fn compute_vec_layout_offsets<'tcx>(
    tcx: TyCtxt<'tcx>,
    self_ty: Ty<'tcx>,
    layout: Layout<'tcx>,
) -> VecLayoutOffsets {
    let (adt_def, adt_generic_args) = match self_ty.kind() {
        ty::TyKind::Adt(adt_def, args) => (adt_def, args),
        _ => panic!("Expected Adt type"),
    };
    let variant = adt_def.non_enum_variant();

    let mut len_offset_val = None;
    let mut ptr_offset_val = None;

    for (i, field) in variant.fields.iter().enumerate() {
        let field_ty = field.ty(tcx, adt_generic_args);
        let field_ty = crate::normalize_ty(tcx, tcx.param_env(field.did), field_ty);
        let field_offset = layout.fields().offset(i).bytes();

        if matches!(field_ty.kind(), ty::TyKind::Uint(ty::UintTy::Usize)) {
            len_offset_val = Some(field_offset);
        } else if let Some(nested_ptr_offset) = find_pointer_field_offset(tcx, field_ty) {
            ptr_offset_val = Some(field_offset + nested_ptr_offset);
        }
    }

    let len_offset = len_offset_val.expect("Failed to find len field in Vec");
    let ptr_offset = ptr_offset_val.expect("Failed to find ptr field in Vec");

    VecLayoutOffsets { ptr_offset, len_offset }
}

fn specialize_vec<'tcx>(
    db: &BindingsGenerator<'tcx>,
    rs_std: &RsStdTemplateSpecialization<'tcx>,
    inner_ty: FormattedTy<'tcx>,
) -> ApiSnippets<'tcx> {
    let tcx = db.tcx();
    let layout = rs_std.layout;
    let mut prereqs = CcPrerequisites::default();
    let inner_ty_cc =
        db.format_ty_for_cc(inner_ty.ty, TypeLocation::Field).unwrap().into_tokens(&mut prereqs);
    let inner_ty_rs = &inner_ty.for_rs;

    let rs_fully_qualified_name = quote! { ::alloc::vec::Vec<#inner_ty_rs> };
    let cc_fully_qualified_name = quote! { rs_std::Vec<#inner_ty_cc> };

    let adt_def = rs_std.self_ty_rs.ty_adt_def().expect("Vec should be an ADT");
    let def_id = Some(adt_def.did());

    let core = Rc::new(database::AdtCoreBindings {
        def_id,
        keyword: quote! { struct },
        cc_short_name: format_ident!("Vec"),
        rs_fully_qualified_name: rs_fully_qualified_name.clone(),
        cc_fully_qualified_name: cc_fully_qualified_name.clone(),
        self_ty: rs_std.self_ty_rs,
        alignment_in_bytes: layout.align().abi.bytes(),
        size_in_bytes: layout.size().bytes(),
    });

    let default_ctor_snippets = db.generate_default_ctor(core.clone()).unwrap_or_else(|err| err);
    let copy_ctor_and_assignment_snippets =
        db.generate_copy_ctor_and_assignment_operator(core.clone()).unwrap_or_else(|err| err);
    let move_ctor_and_assignment_snippets = db
        .generate_move_ctor_and_assignment_operator(core.clone())
        .unwrap_or_else(|err| err.explicitly_deleted);
    let relocating_ctor_snippets = generate_relocating_ctor(db, &core.cc_short_name);

    let target_path_mangled_hash = if db.is_golden_test() {
        "".to_string()
    } else {
        format!("{:x}_", tcx.stable_crate_id(db.source_crate_num()))
    };

    let qualified_name = cc_fully_qualified_name.to_string();
    let name = escape_non_identifier_chars(&qualified_name);
    let drop_thunk_name = format_ident!("__crubit_drop_{}{}", target_path_mangled_hash, name);

    let rs_drop = quote! {
        #[unsafe(no_mangle)]
        unsafe extern "C" fn #drop_thunk_name(vec: *mut #rs_fully_qualified_name) {
            // SAFETY: The caller guarantees `vec` is a valid pointer to an initialized Vec.
            unsafe { ::core::ptr::drop_in_place(vec) };
        }
    };

    let drop_decl = quote! {
        ~Vec() noexcept;
    };
    let drop_impl = quote! {
        extern "C" void #drop_thunk_name(void* vec) noexcept;
        inline rs_std::Vec<#inner_ty_cc>::~Vec() noexcept {
            #drop_thunk_name(this);
        }
    };

    let offsets = compute_vec_layout_offsets(tcx, rs_std.self_ty_rs, layout);

    let ptr_offset = Literal::u64_unsuffixed(offsets.ptr_offset);
    let len_offset = Literal::u64_unsuffixed(offsets.len_offset);

    prereqs.includes.insert(CcInclude::bit());
    prereqs.includes.insert(CcInclude::cstddef());
    prereqs.includes.insert(CcInclude::cstdint());
    prereqs.includes.insert(db.support_header("internal/check.h"));

    let accessors_decl = quote! {
        #inner_ty_cc* data() noexcept;
        const #inner_ty_cc* data() const noexcept;
        std::size_t size() const noexcept;
        #inner_ty_cc& operator[](std::size_t index) noexcept;
        const #inner_ty_cc& operator[](std::size_t index) const noexcept;
        #inner_ty_cc* begin() noexcept;
        const #inner_ty_cc* begin() const noexcept;
        #inner_ty_cc* end() noexcept;
        const #inner_ty_cc* end() const noexcept;
    };

    let full_self_ty = quote! { rs_std::Vec<#inner_ty_cc> };
    let accessors_impl = quote! {
        inline #inner_ty_cc* #full_self_ty::data() noexcept {
            return std::bit_cast<#inner_ty_cc*>(
                *reinterpret_cast<const std::uintptr_t*>(&storage_[#ptr_offset]));
        }
        inline const #inner_ty_cc* #full_self_ty::data() const noexcept {
            return std::bit_cast<#inner_ty_cc*>(
                *reinterpret_cast<const std::uintptr_t*>(&storage_[#ptr_offset]));
        }
        inline std::size_t #full_self_ty::size() const noexcept {
            return std::bit_cast<std::size_t>(
                *reinterpret_cast<const std::size_t*>(&storage_[#len_offset]));
        }
        inline #inner_ty_cc& #full_self_ty::operator[](std::size_t index) noexcept {
            CRUBIT_CHECK(index < size());
            return data()[index];
        }
        inline const #inner_ty_cc& #full_self_ty::operator[](std::size_t index) const noexcept {
            CRUBIT_CHECK(index < size());
            return data()[index];
        }
        inline #inner_ty_cc* #full_self_ty::begin() noexcept { return data(); }
        inline const #inner_ty_cc* #full_self_ty::begin() const noexcept { return data(); }
        inline #inner_ty_cc* #full_self_ty::end() noexcept { return data() + size(); }
        inline const #inner_ty_cc* #full_self_ty::end() const noexcept { return data() + size(); }
    };

    let ApiSnippets { main_api, cc_details, rs_details } = [
        default_ctor_snippets,
        copy_ctor_and_assignment_snippets,
        move_ctor_and_assignment_snippets,
        relocating_ctor_snippets,
    ]
    .into_iter()
    .collect();

    let mut rs_details = rs_details;
    rs_details.tokens.extend(rs_drop);

    let main_api_tokens = main_api.into_tokens(&mut prereqs);
    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_{}", name);
    let size_literal = Literal::u64_unsuffixed(layout.size().bytes());
    let align_literal = Literal::u64_unsuffixed(layout.align().abi.bytes());
    let internal_rust_type_string = rs_fully_qualified_name.to_string();

    let main_api_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        template<> __NEWLINE__
        struct alignas(#align_literal)
        CRUBIT_INTERNAL_RUST_TYPE(#internal_rust_type_string)
        rs_std::Vec<#inner_ty_cc> { __NEWLINE__
        public:
            #main_api_tokens __NEWLINE__
            #drop_decl __NEWLINE__
            #accessors_decl __NEWLINE__

        private:
            unsigned char storage_[#size_literal];
            __NEWLINE__
        }; __NEWLINE__

        __HASH_TOKEN__ endif __NEWLINE__
        __NEWLINE__
    };

    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_IMPL_{}", name);
    let cc_details_tokens = cc_details.into_tokens(&mut prereqs);
    let cc_details_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        #cc_details_tokens __NEWLINE__
        #drop_impl __NEWLINE__
        #accessors_impl __NEWLINE__
        __HASH_TOKEN__ endif __NEWLINE__
        __NEWLINE__
    };

    ApiSnippets {
        main_api: CcSnippet { tokens: main_api_tokens, prereqs },
        cc_details: CcSnippet::new(cc_details_tokens),
        rs_details,
    }
}

fn specialize_result<'tcx>(
    db: &BindingsGenerator<'tcx>,
    rs_std: &RsStdTemplateSpecialization<'tcx>,
    enum_spec: &RsStdEnumSpecialization<'tcx>,
    ok_ty: FormattedTy<'tcx>,
    err_ty: FormattedTy<'tcx>,
) -> ApiSnippets<'tcx> {
    let tcx = db.tcx();
    let mut prereqs = CcPrerequisites::default();
    let ok_ty_tokens =
        db.format_ty_for_cc(ok_ty.ty, TypeLocation::Field).unwrap().into_tokens(&mut prereqs);
    let err_ty_tokens =
        db.format_ty_for_cc(err_ty.ty, TypeLocation::Field).unwrap().into_tokens(&mut prereqs);
    let layout = rs_std.layout;
    let (tag_encoding, tag_field) = match layout.variants() {
        rustc_abi::Variants::Empty | rustc_abi::Variants::Single { .. } => {
            unreachable!("This should have been checked in parse_rs_std_template_specialization")
        }
        rustc_abi::Variants::Multiple { tag_encoding, tag_field, .. } => (tag_encoding, tag_field),
    };

    let tag_type = enum_spec.tag_type_rs;
    let tag_type_cc_tokens: TokenStream = enum_spec.tag_type_cc.clone().into_tokens(&mut prereqs);
    let ok_ty_for_rs = ok_ty.for_rs;
    let err_ty_for_rs = err_ty.for_rs;

    let ty::TyKind::Adt(adt, _) = rs_std.self_ty_rs.kind() else {
        unreachable!("Result<T, E> must be an ADT");
    };
    let ResultVariantIndices { ok_idx, err_idx } = get_result_variant_indices(tcx, *adt);
    let tag_offset = Literal::u64_unsuffixed(layout.fields().offset(tag_field.as_usize()).bytes());
    let endian = tcx.sess.target.options.endian;
    let byte_index_read = match endian {
        rustc_abi::Endian::Little => quote! { i },
        rustc_abi::Endian::Big => quote! { sizeof(#tag_type_cc_tokens) - 1 - i },
    };
    let byte_index_write = match endian {
        rustc_abi::Endian::Little => quote! { i },
        rustc_abi::Endian::Big => quote! { sizeof(#tag_type_cc_tokens) - 1 - i },
    };
    let tag_method = ApiSnippets {
        main_api: CcSnippet::new(quote! {
            constexpr #tag_type_cc_tokens tag() const& noexcept; __NEWLINE__
            constexpr void set_tag(#tag_type_cc_tokens tag) noexcept; __NEWLINE__
        }),
        cc_details: CcSnippet::with_include(
            quote! {
                inline constexpr #tag_type_cc_tokens rs_std::Result<#ok_ty_tokens, #err_ty_tokens>::tag() const& noexcept {
                    std::array<unsigned char, sizeof(#tag_type_cc_tokens)> __bytes = {};
                    for (std::size_t i = 0; i < sizeof(#tag_type_cc_tokens); ++i) {
                        __bytes[#byte_index_read] = __storage[#tag_offset + i];
                    }
                    return std::bit_cast<#tag_type_cc_tokens>(__bytes);
                }
                __NEWLINE__
                inline constexpr void rs_std::Result<#ok_ty_tokens, #err_ty_tokens>::set_tag(#tag_type_cc_tokens tag) noexcept {
                    auto __bytes = std::bit_cast<std::array<unsigned char, sizeof(#tag_type_cc_tokens)>>(tag);
                    for (std::size_t i = 0; i < sizeof(#tag_type_cc_tokens); ++i) {
                        __storage[#tag_offset + i] = __bytes[#byte_index_write];
                    }
                }
                __NEWLINE__
            },
            CcInclude::bit(),
        ),
        ..Default::default()
    };

    let needs_drop = rs_std.self_ty_rs.needs_drop(tcx, post_analysis_typing_env(tcx, adt.did()));
    let discr_for_ok = rs_std.self_ty_rs.discriminant_for_variant(tcx, ok_idx).expect(
        "We do not support zero sized types. Before generating a specialization, we\
            check that the type can be formatted as a C++ type. That should exclude this case \
            from occurring",
    );
    let ok_discr_val = literal_of_tag_ty(tcx, discr_for_ok.val, tag_type);
    let discr_for_err = rs_std.self_ty_rs.discriminant_for_variant(tcx, err_idx).expect(
        "We do not support zero sized types. Before generating a specialization, we\
            check that the type can be formatted as a C++ type. That should exclude this case \
            from occurring",
    );
    let err_discr_val = literal_of_tag_ty(tcx, discr_for_err.val, tag_type);

    #[rustversion::stable]
    let (ok_offset, err_offset) = {
        let variants = match layout.variants() {
            rustc_abi::Variants::Empty | rustc_abi::Variants::Single { .. } => {
                unreachable!(
                    "This should have been checked in parse_rs_std_template_specialization"
                )
            }
            rustc_abi::Variants::Multiple { variants, .. } => variants,
        };
        (
            Literal::u64_unsuffixed(variants[ok_idx].fields.offset(0).bytes()),
            Literal::u64_unsuffixed(variants[err_idx].fields.offset(0).bytes()),
        )
    };
    #[rustversion::nightly]
    let (ok_offset, err_offset) = (
        Literal::u64_unsuffixed(LayoutData::for_variant(&layout, ok_idx).fields.offset(0).bytes()),
        Literal::u64_unsuffixed(LayoutData::for_variant(&layout, err_idx).fields.offset(0).bytes()),
    );

    let result_api = match tag_encoding {
        rustc_abi::TagEncoding::Direct => ResultApiGenerator {
            db,
            ok_ty_rs: ok_ty.ty,
            ok_ty_cpp: ok_ty_tokens.clone(),
            err_ty_rs: err_ty.ty,
            err_ty_cpp: err_ty_tokens.clone(),
            needs_drop,
            tag_method,
            has_value_impl: quote! { tag() == #ok_discr_val },
            write_ok_to_tag: quote! { set_tag(#ok_discr_val); },
            write_err_to_tag: quote! { set_tag(#err_discr_val); },
            ok_ptr_val: quote! {
                __storage + #ok_offset
            },
            err_ptr_val: quote! {
                __storage + #err_offset
            },
        },
        rustc_abi::TagEncoding::Niche { niche_start, niche_variants, untagged_variant } => {
            let mut has_value_impl = quote! {};
            let (write_ok_to_tag, ok_ptr_val) = if *untagged_variant == ok_idx {
                // Untagged variant is Ok, we don't need to set the tag when we write Ok.
                // Our tag is implicitly ok when it is not the err discriminant value.
                (quote! {}, quote! { __storage })
            } else {
                #[rustversion::before(2026-05-30)]
                let ok_relative_idx =
                    ok_idx.as_u32().strict_sub(niche_variants.start().as_u32()) as u128;
                #[rustversion::since(2026-05-30)]
                let ok_relative_idx =
                    ok_idx.as_u32().strict_sub(niche_variants.start.as_u32()) as u128;
                let ok_relative_val =
                    literal_of_tag_ty(tcx, *niche_start + ok_relative_idx, tag_type);
                has_value_impl = quote! { tag() == #ok_relative_val };
                (
                    quote! { set_tag(#ok_relative_val); },
                    quote! {
                        __storage + #ok_offset
                    },
                )
            };
            let (write_err_to_tag, err_ptr_val) = if *untagged_variant == err_idx {
                (quote! {}, quote! { __storage })
            } else {
                #[rustversion::before(2026-05-30)]
                let err_relative_idx =
                    err_idx.as_u32().strict_sub(niche_variants.start().as_u32()) as u128;
                #[rustversion::since(2026-05-30)]
                let err_relative_idx =
                    err_idx.as_u32().strict_sub(niche_variants.start.as_u32()) as u128;
                let err_relative_val =
                    literal_of_tag_ty(tcx, *niche_start + err_relative_idx, tag_type);
                has_value_impl = quote! { tag() != #err_relative_val };
                (
                    quote! { set_tag(#err_relative_val); },
                    quote! {
                        __storage + #err_offset
                    },
                )
            };
            ResultApiGenerator {
                db,
                ok_ty_rs: ok_ty.ty,
                ok_ty_cpp: ok_ty_tokens.clone(),
                err_ty_rs: err_ty.ty,
                err_ty_cpp: err_ty_tokens.clone(),
                needs_drop,
                tag_method,
                has_value_impl,
                write_ok_to_tag,
                write_err_to_tag,
                ok_ptr_val,
                err_ptr_val,
            }
        }
    };

    let rs_fully_qualified_name = quote! { std::result::Result<#ok_ty_for_rs, #err_ty_for_rs> };
    let cc_fully_qualified_name = quote! { rs_std::Result<#ok_ty_tokens, #err_ty_tokens> };
    let core = Rc::new(database::AdtCoreBindings {
        def_id: Some(adt.did()),
        keyword: quote! { struct },
        cc_short_name: format_ident!("Result"),
        rs_fully_qualified_name: rs_fully_qualified_name.clone(),
        cc_fully_qualified_name: cc_fully_qualified_name.clone(),
        lifetime_params: vec![],
        self_ty: rs_std.self_ty_rs,
        alignment_in_bytes: layout.align().abi.bytes(),
        size_in_bytes: layout.size().bytes(),
    });

    let copy_ctor_and_assignment_snippets =
        db.generate_copy_ctor_and_assignment_operator(core.clone()).unwrap_or_else(|err| err);
    let move_ctor_and_assignment_snippets = db
        .generate_move_ctor_and_assignment_operator(core.clone())
        .unwrap_or_else(|err| err.explicitly_deleted);
    let relocating_ctor_snippets = generate_relocating_ctor(db, &core.cc_short_name);

    let ApiSnippets { main_api, cc_details, rs_details } = [
        copy_ctor_and_assignment_snippets,
        move_ctor_and_assignment_snippets,
        relocating_ctor_snippets,
        result_api.api_snippets(),
    ]
    .into_iter()
    .collect();
    let main_api_tokens = main_api.into_tokens(&mut prereqs);

    let qualified_name = cc_fully_qualified_name.to_string();
    let name = escape_non_identifier_chars(&qualified_name);
    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_{}", name);
    let size_literal = Literal::u64_unsuffixed(layout.size().bytes());
    let align_literal = Literal::u64_unsuffixed(layout.align().abi.bytes());
    let internal_rust_type_string = rs_fully_qualified_name.to_string();
    let main_api_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        template<> __NEWLINE__
        struct
        alignas(#align_literal) __NEWLINE__
        CRUBIT_INTERNAL_RUST_TYPE(#internal_rust_type_string)
        rs_std::Result<#ok_ty_tokens, #err_ty_tokens>
            : public rs_std::ResultBase<rs_std::Result<#ok_ty_tokens, #err_ty_tokens>, #ok_ty_tokens, #err_ty_tokens> { __NEWLINE__
        public:
            #main_api_tokens __NEWLINE__

            private:
               unsigned char __storage[#size_literal]; __NEWLINE__
        }; __NEWLINE__

        __HASH_TOKEN__ endif __NEWLINE__
        __NEWLINE__
    };

    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_IMPL_{}", name);
    let cc_details_tokens = cc_details.into_tokens(&mut prereqs);
    let cc_details_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        #cc_details_tokens __NEWLINE__
        __HASH_TOKEN__ endif __NEWLINE__
        __NEWLINE__
    };

    ApiSnippets {
        main_api: CcSnippet { tokens: main_api_tokens, prereqs },
        cc_details: CcSnippet::new(cc_details_tokens),
        rs_details,
    }
}

fn specialize_option<'tcx>(
    db: &BindingsGenerator<'tcx>,
    rs_std: &RsStdTemplateSpecialization<'tcx>,
    enum_spec: &RsStdEnumSpecialization<'tcx>,
    arg_ty: FormattedTy<'tcx>,
) -> ApiSnippets<'tcx> {
    let tcx = db.tcx();
    let mut prereqs = CcPrerequisites::default();
    let ty_tokens =
        db.format_ty_for_cc(arg_ty.ty, TypeLocation::Field).unwrap().into_tokens(&mut prereqs);
    let layout = rs_std.layout;

    let (tag_encoding, tag_field) = match layout.variants() {
        rustc_abi::Variants::Empty | rustc_abi::Variants::Single { .. } => {
            unreachable!("This should have been checked in parse_rs_std_template_specialization")
        }
        rustc_abi::Variants::Multiple { tag_encoding, tag_field, .. } => (tag_encoding, tag_field),
    };
    let tag_type = enum_spec.tag_type_rs;
    let tag_type_cc: TokenStream = enum_spec.tag_type_cc.clone().into_tokens(&mut prereqs);
    let arg_ty_for_rs = arg_ty.for_rs;

    let ty::TyKind::Adt(adt, _) = rs_std.self_ty_rs.kind() else {
        unreachable!("Option<T> must be an ADT");
    };
    let needs_drop = rs_std.self_ty_rs.needs_drop(tcx, post_analysis_typing_env(tcx, adt.did()));

    let OptionVariantIndices { some_idx, none_idx } = get_option_variant_indices(tcx, *adt);

    prereqs.includes.insert(CcInclude::optional());
    let tag_offset = Literal::u64_unsuffixed(layout.fields().offset(tag_field.as_usize()).bytes());

    let endian = tcx.sess.target.options.endian;
    let endian_index = match endian {
        rustc_abi::Endian::Little => quote! { i },
        rustc_abi::Endian::Big => quote! { sizeof(#tag_type_cc) - 1 - i },
    };
    let tag_method = ApiSnippets {
        main_api: CcSnippet::new(quote! {
            constexpr #tag_type_cc tag() const& noexcept; __NEWLINE__
            constexpr void set_tag(#tag_type_cc tag) noexcept; __NEWLINE__
        }),
        cc_details: CcSnippet::with_include(
            quote! {
                inline constexpr #tag_type_cc rs_std::Option<#ty_tokens>::tag() const& noexcept {
                    ::std::array<unsigned char, sizeof(#tag_type_cc)> __bytes = {};
                    for (::std::size_t i = 0; i < sizeof(#tag_type_cc); ++i) {
                        __bytes[#endian_index] = storage_[#tag_offset + i];
                    }
                    return ::std::bit_cast<#tag_type_cc>(__bytes);
                }
                __NEWLINE__
                inline constexpr void rs_std::Option<#ty_tokens>::set_tag(#tag_type_cc tag) noexcept {
                    auto __bytes = ::std::bit_cast<::std::array<unsigned char, sizeof(#tag_type_cc)>>(tag);
                    for (::std::size_t i = 0; i < sizeof(#tag_type_cc); ++i) {
                        storage_[#tag_offset + i] = __bytes[#endian_index];
                    }
                }
                __NEWLINE__
            },
            CcInclude::bit(),
        ),
        ..Default::default()
    };

    let expect_msg =
            "Please file a bug at crubit.rs-bug. We do not support zero sized types. Before generating \
            a specialization, we check that the type can be formatted as a C++ type. That should \
            exclude this case from occurring.";
    let discr_for_none =
        rs_std.self_ty_rs.discriminant_for_variant(tcx, none_idx).expect(expect_msg);
    let none_discr_val = literal_of_tag_ty(tcx, discr_for_none.val, tag_type);
    let option_api = match tag_encoding {
        rustc_abi::TagEncoding::Direct => {
            // Option::None is variant 0. Option::Some is variant 1.
            #[rustversion::stable]
            let payload_offset = {
                let variants = match layout.variants() {
                    rustc_abi::Variants::Empty | rustc_abi::Variants::Single { .. } => {
                        unreachable!(
                            "This should have been checked in parse_rs_std_template_specialization"
                        )
                    }
                    rustc_abi::Variants::Multiple { variants, .. } => variants,
                };
                Literal::u64_unsuffixed(variants[some_idx].fields.offset(0).bytes())
            };
            #[rustversion::nightly]
            let payload_offset = Literal::u64_unsuffixed(
                LayoutData::for_variant(&layout, some_idx).fields.offset(0).bytes(),
            );
            let discr_for_some =
                rs_std.self_ty_rs.discriminant_for_variant(tcx, some_idx).expect(expect_msg);
            let some_discr_val = literal_of_tag_ty(tcx, discr_for_some.val, tag_type);

            OptionApiGenerator {
                arg_ty: ty_tokens.clone(),
                needs_drop,
                tag_method,
                none_val: quote! { #none_discr_val },
                write_some_to_tag: quote! { set_tag(#some_discr_val); },
                some_ptr_val: quote! {
                    reinterpret_cast<#ty_tokens*>(storage_ + #payload_offset)
                },
                some_const_ptr_val: quote! {
                    reinterpret_cast<#ty_tokens const*>(storage_ + #payload_offset)
                },
                tag_type_cc: tag_type_cc.clone(),
            }
        }
        rustc_abi::TagEncoding::Niche { niche_start, niche_variants, .. } => {
            #[rustversion::before(2026-05-30)]
            let none_relative_idx =
                none_idx.as_u32().strict_sub(niche_variants.start().as_u32()) as u128;
            #[rustversion::since(2026-05-30)]
            let none_relative_idx =
                none_idx.as_u32().strict_sub(niche_variants.start.as_u32()) as u128;
            let none_relative_val =
                literal_of_tag_ty(tcx, niche_start + none_relative_idx, tag_type);
            OptionApiGenerator {
                arg_ty: ty_tokens.clone(),
                needs_drop,
                tag_method,
                none_val: quote! { #none_relative_val },
                some_ptr_val: quote! {
                    reinterpret_cast<#ty_tokens*>(storage_)
                },
                some_const_ptr_val: quote! {
                    reinterpret_cast<#ty_tokens const*>(storage_)
                },
                // With a niche, the Some variant is implicitly encoded. We don't need to write out
                // a discriminant value. It is accomplished by writing a value to the Some payload.
                write_some_to_tag: quote! {},
                tag_type_cc: tag_type_cc.clone(),
            }
        }
    };
    let rs_fully_qualified_name = quote! { std::option::Option<#arg_ty_for_rs> };
    let cc_fully_qualified_name = quote! { rs_std::Option<#ty_tokens> };
    let core = Rc::new(database::AdtCoreBindings {
        def_id: Some(adt.did()),
        keyword: quote! { struct },
        cc_short_name: format_ident!("Option"),
        rs_fully_qualified_name: rs_fully_qualified_name.clone(),
        cc_fully_qualified_name: cc_fully_qualified_name.clone(),
        lifetime_params: vec![],
        self_ty: rs_std.self_ty_rs,
        alignment_in_bytes: layout.align().abi.bytes(),
        size_in_bytes: layout.size().bytes(),
    });

    let copy_ctor_and_assignment_snippets =
        db.generate_copy_ctor_and_assignment_operator(core.clone()).unwrap_or_else(|err| err);

    let move_ctor_and_assignment_snippets = db
        .generate_move_ctor_and_assignment_operator(core.clone())
        .unwrap_or_else(|err| err.explicitly_deleted);

    let relocating_ctor_snippets = generate_relocating_ctor(db, &core.cc_short_name);

    let ApiSnippets { main_api, cc_details, rs_details } = [
        copy_ctor_and_assignment_snippets,
        move_ctor_and_assignment_snippets,
        relocating_ctor_snippets,
        option_api.api_snippets(),
    ]
    .into_iter()
    .collect();
    let main_api_tokens = main_api.into_tokens(&mut prereqs);

    let qualified_name = cc_fully_qualified_name.to_string();
    let name = escape_non_identifier_chars(&qualified_name);
    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_{}", name);
    let size_literal = Literal::u64_unsuffixed(layout.size().bytes());
    let align_literal = Literal::u64_unsuffixed(layout.align().abi.bytes());
    let internal_rust_type_string = rs_fully_qualified_name.to_string();
    // TODO(cramertj): Consider standardizing the `storage_` field with other representations in
    // `generate_adt`.
    let main_api_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        template<> __NEWLINE__
        struct alignas(#align_literal)
        CRUBIT_INTERNAL_RUST_TYPE(#internal_rust_type_string)
        rs_std::Option<#ty_tokens>
            : public rs_std::OptionBase<rs_std::Option<#ty_tokens>, #ty_tokens> { __NEWLINE__
        public:
            #main_api_tokens __NEWLINE__

        private:
            unsigned char storage_[#size_literal];
            __NEWLINE__
        }; __NEWLINE__

        __HASH_TOKEN__ endif __NEWLINE__
        __NEWLINE__
    };

    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_IMPL_{}", name);
    let cc_details_tokens = cc_details.into_tokens(&mut prereqs);
    let cc_details_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        #cc_details_tokens __NEWLINE__
        __HASH_TOKEN__ endif __NEWLINE__
        __NEWLINE__
    };
    ApiSnippets {
        main_api: CcSnippet { tokens: main_api_tokens, prereqs },
        cc_details: CcSnippet::new(cc_details_tokens),
        rs_details,
    }
}

trait TemplateSpecializationExt<'tcx> {
    fn api_snippets(self, db: &BindingsGenerator<'tcx>) -> ApiSnippets<'tcx>;
}

impl<'tcx> TemplateSpecializationExt<'tcx> for RsStdTemplateSpecialization<'tcx> {
    fn api_snippets(self, db: &BindingsGenerator<'tcx>) -> ApiSnippets<'tcx> {
        match &self.args {
            RsStdSpecializationArgs::Enum(enum_spec) => match &enum_spec.kind {
                EnumSpecializationKind::Option { some_ty } => {
                    let some_ty_ty = some_ty.ty;
                    let mut snippets = specialize_option(db, &self, enum_spec, some_ty.clone());
                    snippets.main_api.prereqs.forward_declare_type(some_ty_ty);
                    snippets
                }
                EnumSpecializationKind::Result { ok_ty, err_ty } => {
                    let ok_ty_ty = ok_ty.ty;
                    let err_ty_ty = err_ty.ty;
                    let mut snippets =
                        specialize_result(db, &self, enum_spec, ok_ty.clone(), err_ty.clone());
                    snippets.main_api.prereqs.forward_declare_type(ok_ty_ty);
                    snippets.main_api.prereqs.forward_declare_type(err_ty_ty);
                    snippets
                }
            },
            RsStdSpecializationArgs::Tuple(element_tys) => {
                let mut snippets = specialize_tuple(db, &self, element_tys.clone());
                for element_ty in element_tys {
                    snippets.main_api.prereqs.forward_declare_type(element_ty.ty);
                }
                snippets
            }
            RsStdSpecializationArgs::Vec(inner_ty) => {
                let inner_ty_ty = inner_ty.ty;
                let mut snippets = specialize_vec(db, &self, inner_ty.clone());
                snippets.main_api.prereqs.forward_declare_type(inner_ty_ty);
                snippets
            }
        }
    }
}

/// Collect trait implementations and map them to `TemplateSpecialization::TraitImpl`.
pub(crate) fn collect_trait_impls<'a, 'tcx>(
    db: &'a BindingsGenerator<'tcx>,
) -> impl Iterator<Item = TemplateSpecialization<'tcx>> + use<'a, 'tcx> {
    let tcx = db.tcx();
    let supported_traits: Vec<DefId> = db.supported_traits().iter().copied().collect();
    // TyCtxt makes it easy to get all the implementations of a trait, but there isn't an easy way
    // to say give me all the trait implementations for this type. This is by design. The compiler
    // lazily determines conformance to traits as needed for types and never computes every trait
    // for a type in a single data structure.
    //
    // We, however, want every implementation for a supported type, so we can emit bindings to them.
    // We achieve this by walking every supported trait, walking every implementation of that trait,
    // and paring down to the implementations that receive bindings.
    //
    // A serendipitous side effect of this approach is that our implementations are emitted as a
    // single list containing just implementations. We want to emit all of our implementations in a
    // separate portion of our header from the rest of our bindings. Our impls become template
    // specializaitons, which are required to be in an enclosing namespace of the template they
    // specialize. This prevents them from living in the same namespace as our other bindings, as
    // they may implement a trait that is not enclosed by that namespace.
    supported_traits.into_iter().flat_map(move |trait_def_id| {
        use rustc_middle::ty::fast_reject::SimplifiedType;
        tcx.trait_impls_of(trait_def_id)
            .non_blanket_impls()
            .into_iter()
            .filter_map(move |(simple_ty, impl_def_ids)| match simple_ty {
                SimplifiedType::Adt(did) => {
                    // Only bind implementations for supported ADTs.
                    let canonical_name = db.symbol_canonical_name(*did)?;
                    // We explicitly want to allow ADTs that specify cpp_type.
                    // These are typically C++ types that have generated Rust bindings.
                    if canonical_name.unqualified.cpp_type.is_none()
                        && db.adt_needs_bindings(*did).is_err()
                    {
                        return None;
                    }
                    let adt_cc_name = canonical_name.format_for_cc(db).ok()?;
                    Some((adt_cc_name, impl_def_ids))
                }
                // TODO: b/457803426 - Support trait implementations for non-adt types.
                _ => None,
            })
            .flat_map(move |(adt_cc_name, impl_def_ids)| {
                impl_def_ids
                    .iter()
                    .copied()
                    // TODO: b/458768435 - This is technically suboptimal. We could instead only
                    // query for the impls from this crate, but the logic is complicated by
                    // supporting LOCAL_CRATE. Revisit once we've migrated to rmetas.
                    .filter(|impl_def_id| impl_def_id.krate == db.source_crate_num())
                    .map(move |impl_def_id| {
                        TemplateSpecialization::TraitImpl(TraitImplTemplateSpecialization {
                            self_ty_cc_name: adt_cc_name.clone(),
                            trait_impl: impl_def_id,
                        })
                    })
            })
    })
}

fn generate_trait_impl_specialization<'tcx>(
    db: &BindingsGenerator<'tcx>,
    trait_impl: &TraitImplTemplateSpecialization,
) -> std::result::Result<ApiSnippets<'tcx>, (DefId, Error)> {
    let tcx = db.tcx();
    let impl_def_id = trait_impl.trait_impl;
    let trait_header = tcx.impl_trait_header(impl_def_id);
    #[rustversion::all(before(1.94), before(2025-10-17))]
    let trait_header = trait_header.expect("Trait impl should have a trait header");
    let trait_ref = crate::normalize_ty(
        tcx,
        tcx.param_env(impl_def_id),
        trait_header.trait_ref.instantiate_identity(),
    );
    let trait_def_id = trait_ref.def_id;

    let canonical_trait_name = db.symbol_canonical_name(trait_def_id).expect(
        "symbol_canonical_name was unexpectedly called on a trait without a canonical name",
    );
    let trait_name = canonical_trait_name.format_for_cc(db).map_err(|err| (impl_def_id, err))?;

    let mut prereqs = CcPrerequisites::default();
    let trait_args: Vec<_> = trait_ref
        .args
        .iter()
        // Skip self type.
        .skip(1)
        .filter_map(|arg| arg.as_type())
        .map(|arg| {
            if arg.flags().intersects(has_type_or_const_vars()) {
                return Err((impl_def_id, anyhow!("Implementation of traits must specify all types to receive bindings.")));
            }
            if arg.walk().any(|arg| arg.as_type().is_some_and(|ty| ty.is_ptr_sized_integral())) {
                return Err((
                    impl_def_id,
                    anyhow!(
                        "b/491106325 - isize and usize types are not yet supported as trait type arguments."
                    ),
                ));
            }
            db.format_ty_for_cc(arg, TypeLocation::Other)
                .map(|snippet| snippet.into_tokens(&mut prereqs))
                .map_err(|err| (impl_def_id, err))
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let type_args = if trait_args.is_empty() {
        quote! {}
    } else {
        quote! { <#(#trait_args),*> }
    };

    let trait_name_with_args = quote! { #trait_name #type_args };

    prereqs.depend_on_def(db, trait_def_id).map_err(|err| (impl_def_id, err))?;
    if let Some(adt) = trait_ref.self_ty().ty_adt_def() {
        let def_id = adt.did();
        let canonical_name = db.symbol_canonical_name(def_id).expect(
            "Self type should have a canonical name if we are generating a specialization for it",
        );
        // When `self_ty` belongs to the current crate (`source_crate_num()`), we must distinguish
        // between standard Rust structs (which get a C++ definition in `main_apis`) and mapped C++
        // types (`cpp_type.is_some()`, whose `main_api` generation is suppressed by Crubit).
        //
        // For standard Rust structs, we insert `def_id` into `fwd_decls` to avoid C++ dependency
        // cycles between a container and its iterator. For mapped C++ types, emitting a forward
        // declaration in the Rust crate's namespace is incorrect and fails to pull in the real C++
        // header. Instead, we must explicitly add the annotated `include_path` to prerequisites.
        if canonical_name.unqualified.cpp_type.is_some() {
            let attrs = crubit_attr::get_attrs(db.tcx(), def_id)
                .map_err(|err| (impl_def_id, arc_anyhow::Error::from(err)))?;
            if let Some(path) = attrs.include_path {
                prereqs.includes.insert(CcInclude::from_path(path.as_str()));
            } else {
                // The C++ type is already available (e.g., a fundamental type or from a header
                // injected globally via command-line flags). No additional #include, forward
                // declaration, or def dependency is needed.
            }
        } else if canonical_name.krate_num == db.source_crate_num() {
            prereqs.fwd_decls.insert(def_id);
        } else {
            prereqs.depend_on_def(db, def_id).map_err(|err| (impl_def_id, err))?;
        }
    }

    let mut member_function_names = HashSet::new();
    let assoc_items: ApiSnippets = tcx
        .associated_items(impl_def_id)
        .in_definition_order()
        .flat_map(|assoc_item| {
            generate_associated_item(
                db,
                assoc_item,
                &mut member_function_names,
                None,
                StaticMethodMode::ForceStaticMethod,
            )
        })
        .collect();

    let main_api = assoc_items.main_api.into_tokens(&mut prereqs);
    prereqs.includes.insert(db.support_header("rs_std/traits.h"));

    let self_ty_cc_name = &trait_impl.self_ty_cc_name;
    Ok(ApiSnippets {
        main_api: CcSnippet {
            tokens: quote! {
                __NEWLINE__
                template<>
                struct rs_std::impl<#self_ty_cc_name, #trait_name_with_args> {
                    static constexpr bool kIsImplemented = true;

                    #main_api
                };
                __NEWLINE__
            },
            prereqs,
        },
        cc_details: assoc_items.cc_details,
        rs_details: assoc_items.rs_details,
    })
}

/// Generate a template specialization.
pub fn generate_template_specialization<'tcx>(
    db: &BindingsGenerator<'tcx>,
    specialization: TemplateSpecialization<'tcx>,
) -> ApiSnippets<'tcx> {
    let mut snippets = match &specialization {
        TemplateSpecialization::RsStd(rs_std) => rs_std.clone().api_snippets(db),
        TemplateSpecialization::TraitImpl(trait_impl) => {
            generate_trait_impl_specialization(db, trait_impl).unwrap_or_else(|(def_id, err)| {
                generate_unsupported_def(db, def_id, err).into_main_api()
            })
        }
    };
    // Because we reuse logic from generate_struct_and_union here, we will add our `self_ty` as a template specialization of it's own specialization creating a depedency cycle.
    // We break that loop manually here to avoid that.
    snippets.main_api.prereqs.template_specializations.remove(&specialization);
    snippets
}
