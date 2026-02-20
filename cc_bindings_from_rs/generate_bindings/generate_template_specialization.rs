// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::generate_function_thunk::replace_all_regions_with_static;
use crate::generate_struct_and_union::{
    generate_fields, generate_relocating_ctor, scalar_value_to_string,
};
use arc_anyhow::Result;
use code_gen_utils::{escape_non_identifier_chars, CcInclude};
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet, TemplateSpecialization};
use database::{BindingsGenerator, TypeLocation};
use error_report::anyhow;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use query_compiler::{get_layout, post_analysis_typing_env};
use quote::{format_ident, quote};
use rustc_abi::VariantIdx;
use rustc_middle::ty::{self, AdtDef, Ty, TyCtxt};
use std::collections::HashSet;
use std::rc::Rc;

struct OptionApi<'a, 'tcx> {
    db: &'a BindingsGenerator<'tcx>,
    arg_ty_rs: Ty<'tcx>,
    arg_ty: TokenStream,
    needs_drop: bool,
    // Reads our tag out of our Option<T> and defines a variable `tag` pointing at it's value.
    // This is complicated by the niche optimization, so we abstract it as this reference so it's
    // easy to reference.
    tag_method: ApiSnippets<'tcx>,
    none_val: TokenStream,
    write_some_to_tag: TokenStream,
    some_ptr_val: TokenStream,
    tag_type_cc: TokenStream,
}

impl<'tcx> OptionApi<'_, 'tcx> {
    // Remove wrapper Option<T> indirection.
    fn underlying_argument_ty(&self) -> Ty<'tcx> {
        fn underlying_argument_ty<'tcx>(db: &BindingsGenerator<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
            let ty::TyKind::Adt(adt, subst) = ty.kind() else {
                return ty;
            };
            let Some(variant) = adt.variants().iter().next() else {
                return ty;
            };
            use rustc_hir::LangItem;
            match db.tcx().lang_items().from_def_id(variant.def_id) {
                Some(LangItem::OptionSome | LangItem::OptionNone) => {
                    underlying_argument_ty(db, subst.type_at(0))
                }
                _ => ty,
            }
        }
        underlying_argument_ty(self.db, self.arg_ty_rs)
    }

    fn has_move_ctor(&self) -> bool {
        let underlying_argument_ty = self.underlying_argument_ty();

        underlying_argument_ty
            .ty_adt_def()
            .map(|adt| {
                self.db
                    .generate_adt_core(adt.did())
                    .and_then(|core| {
                        self.db.generate_move_ctor_and_assignment_operator(core).map_err(|e| e.err)
                    })
                    .is_ok()
            })
            .unwrap_or(false)
    }

    fn has_relocating_ctor(&self) -> bool {
        let underlying_argument_ty = self.underlying_argument_ty();
        underlying_argument_ty
            .ty_adt_def()
            .map(|adt| self.db.generate_adt_core(adt.did()).is_ok())
            .unwrap_or(false)
    }

    fn api_snippets(self) -> ApiSnippets<'tcx> {
        let has_move_ctor = self.has_move_ctor();
        let has_relocating_ctor = self.has_relocating_ctor();
        let Self {
            arg_ty,
            needs_drop,
            tag_method,
            none_val,
            write_some_to_tag,
            some_ptr_val,
            tag_type_cc,
            ..
        } = self;
        let mut prereqs = CcPrerequisites::default();
        if has_relocating_ctor {
            prereqs.includes.insert(self.db.support_header("internal/slot.h"));
        }
        let set_none = quote! {
            *this->tag() = #none_val;
        };
        let set_some_from_std_optional = {
            let write_some = if has_move_ctor {
                quote! { *some = std::move(value.value()); }
            } else if has_relocating_ctor {
                quote! { std::construct_at(some, crubit::UnsafeRelocateTag{}, std::move(*value)); }
            } else {
                quote! { *some = value.value(); }
            };
            quote! {
                #write_some_to_tag
                #arg_ty* some = #some_ptr_val;
                #write_some
                std::construct_at(&value, std::nullopt);
            }
        };
        let take_some = if has_relocating_ctor {
            quote! {
                struct DeferSetTagNone {
                    #tag_type_cc* _value;
                    DeferSetTagNone(#tag_type_cc* tag) : _value(tag) {}
                    ~DeferSetTagNone() {
                        #set_none
                    }
                    #tag_type_cc* tag() noexcept {
                        return _value;
                    }
                } defer(this->tag());
                return std::make_optional<#arg_ty>(crubit::UnsafeRelocateTag{}, std::move(*#some_ptr_val));
            }
        } else {
            quote! {
                #arg_ty& value = *#some_ptr_val;
                std::optional<#arg_ty> return_value(std::move(value));
                std::destroy_at(&value);
                #set_none
                return return_value;
            }
        };

        // Destruct a some value if present.
        let reset = quote! {
            if (*this->tag() != #none_val) {
                std::destroy_at(#some_ptr_val);
            }
        };

        let (drop, drop_details) = if needs_drop {
            (
                quote! {
                    ~Option() noexcept;
                },
                quote! {
                    inline rs_std::Option<#arg_ty>::~Option() noexcept {
                        #reset
                    }
                },
            )
        } else {
            prereqs.includes.insert(CcInclude::type_traits());
            (
                quote! {
                    ~Option() noexcept = default;
                },
                quote! { static_assert(std::is_trivially_destructible_v<rs_std::Option<#arg_ty>>); },
            )
        };

        // We can only move construct from an `Option<T>`'s `T` if it has a move constructor.
        let (value_move_ctor_and_assign, value_move_ctor_and_assign_details) = if !has_move_ctor {
            (quote! {}, quote! {})
        } else {
            (
                quote! {
                  Option(#arg_ty&& value) noexcept; __NEWLINE__
                  Option& operator=(#arg_ty&& value) noexcept; __NEWLINE__ __NEWLINE__
                },
                quote! {
                    inline rs_std::Option<#arg_ty>::Option(#arg_ty&& value) noexcept {
                        #write_some_to_tag
                        std::construct_at(#some_ptr_val, std::move(value));
                    } __NEWLINE__
                    inline rs_std::Option<#arg_ty>& rs_std::Option<#arg_ty>::operator=(#arg_ty&& value) noexcept {
                        if (*this->tag() != #none_val) {
                          *#some_ptr_val = std::move(value);
                        } else {
                          #write_some_to_tag
                          std::construct_at(#some_ptr_val, std::move(value));
                        }
                        return *this;
                    } __NEWLINE__
                },
            )
        };

        let tag_method_main_api = tag_method.main_api.into_tokens(&mut prereqs);
        let main_api = CcSnippet {
            tokens: quote! {
                Option();  __NEWLINE__ __NEWLINE__

                explicit Option(std::nullopt_t) noexcept; __NEWLINE__
                Option& operator=(std::nullopt_t) noexcept; __NEWLINE__ __NEWLINE__

                #value_move_ctor_and_assign

                explicit Option(std::optional<#arg_ty>&& value) noexcept; __NEWLINE__
                Option& operator=(std::optional<#arg_ty>&& value) noexcept; __NEWLINE__ __NEWLINE__

                template<typename... Args>
                Option(std::in_place_t, Args&&... args) noexcept;

                #drop

                operator std::optional<#arg_ty>() && noexcept;

                bool has_value() noexcept;
            private:
                #tag_method_main_api
            },
            prereqs,
        };
        let mut prereqs = CcPrerequisites::default();
        // For std::move.
        prereqs.includes.insert(CcInclude::utility());
        let tag_method_cc_details = tag_method.cc_details.into_tokens(&mut prereqs);
        let cc_details = CcSnippet {
            tokens: quote! {
                inline rs_std::Option<#arg_ty>::Option() {
                    #set_none
                } __NEWLINE__

                inline rs_std::Option<#arg_ty>::Option(std::nullopt_t) noexcept {
                    #set_none
                } __NEWLINE__
                inline rs_std::Option<#arg_ty>& rs_std::Option<#arg_ty>::operator=(std::nullopt_t) noexcept {
                    #reset
                    #set_none
                    return *this;
                } __NEWLINE__

                #value_move_ctor_and_assign_details

                inline rs_std::Option<#arg_ty>::Option(std::optional<#arg_ty>&& value) noexcept {
                    if (value.has_value()) {
                        #set_some_from_std_optional
                    } else {
                        #set_none
                    }
                } __NEWLINE__
                inline rs_std::Option<#arg_ty>& rs_std::Option<#arg_ty>::operator=(std::optional<#arg_ty>&& value) noexcept {
                    #reset
                    if (value.has_value()) {
                        #set_some_from_std_optional
                    } else {
                        #set_none
                    }
                    return *this;
                } __NEWLINE__

                template<typename... Args>
                inline rs_std::Option<#arg_ty>::Option(std::in_place_t, Args&&... args) noexcept {
                    #write_some_to_tag
                    std::construct_at(#some_ptr_val, std::forward<Args>(args)...);
                } __NEWLINE__

                #drop_details

                inline rs_std::Option<#arg_ty>::operator std::optional<#arg_ty>() && noexcept {
                    if (*this->tag() == #none_val) {
                        return std::nullopt;
                    } else {
                        #take_some
                    }
                } __NEWLINE__

                inline bool rs_std::Option<#arg_ty>::has_value() noexcept {
                    return *this->tag() != #none_val;
                } __NEWLINE__

                #tag_method_cc_details
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

/// Generate a template specialization for an instance of `Option<T>`.
///
/// `arg_ty` is used to determine the corresponding C++ type of the `T` in `Option<T>`.
/// `self_ty` is the type of the `Option<T>` and is used to determine the layout of the type,
/// especially the location of the tag accounting for any niches.
fn specialize_option<'tcx>(
    db: &BindingsGenerator<'tcx>,
    arg_ty: Ty<'tcx>,
    self_ty: Ty<'tcx>,
) -> Result<ApiSnippets<'tcx>> {
    let tcx = db.tcx();
    let mut prereqs = CcPrerequisites::default();
    let arg_ty = replace_all_regions_with_static(tcx, arg_ty);
    let ty_tokens = db
        .format_ty_for_cc(arg_ty, TypeLocation::Other)?
        .resolve_feature_requirements(crate::crate_features(db, db.source_crate_num()))?
        .into_tokens(&mut prereqs);
    let layout = get_layout(tcx, self_ty).expect("We've already checked this layout is valid");
    let size = layout.size().bytes();
    let member_fields_names = HashSet::new();
    let ty::TyKind::Adt(adt, _) = self_ty.kind() else {
        unreachable!("Option<T> must be an ADT");
    };
    let needs_drop = self_ty.needs_drop(tcx, post_analysis_typing_env(tcx, adt.did()));
    let fields = generate_fields(
        db,
        self_ty,
        &format_ident!("Option"),
        &quote! { Option<#ty_tokens> },
        &[],
        size,
        layout.align().abi.bytes(),
        &member_fields_names,
    );
    let fields_main_api = fields.main_api.into_tokens(&mut prereqs);
    let name = escape_non_identifier_chars(&format!("{}", self_ty));

    let OptionVariantIndices { some_idx, none_idx } = get_option_variant_indices(tcx, *adt);

    prereqs.includes.insert(CcInclude::optional());

    let (tag, tag_encoding, tag_field, variants) = match layout.variants() {
        rustc_abi::Variants::Empty => {
            unreachable!("Option is never uninhabited because of the None variant.")
        }
        rustc_abi::Variants::Single { .. } => {
            unreachable!(
                "This case only occurs when our Some variant contains a zero sized type \
             (aka an uninhabited type). This is unssupported today and we call format_ty_for_cc on \
             our argument type before queueing a specialization, so this case should not occur in \
             practice."
            )
        }
        rustc_abi::Variants::Multiple { tag, tag_encoding, tag_field, variants } => {
            (tag, tag_encoding, tag_field, variants)
        }
    };
    use rustc_middle::ty::layout::PrimitiveExt;
    let tag_type = tag.primitive().to_int_ty(tcx);
    let tag_type_cc: TokenStream =
        db.format_ty_for_cc(tag_type, TypeLocation::Other)?.into_tokens(&mut prereqs);
    let tag_offset = Literal::u64_unsuffixed(layout.fields().offset(tag_field.as_usize()).bytes());

    let literal_of_tag_ty = |val: u128, ty: Ty<'tcx>| {
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
    };
    let discr_for_none = self_ty.discriminant_for_variant(tcx, none_idx).expect(
        "We do not support zero sized types. Before generating a specialization, we check \
        that the type can be formatted as a C++ type. That should exclude this case from occurring",
    );
    let none_discr_val = literal_of_tag_ty(discr_for_none.val, tag_type);
    let tag_method = ApiSnippets {
        main_api: CcSnippet::new(quote! {
            #tag_type_cc* tag() noexcept;
        }),
        cc_details: CcSnippet::new(quote! {
            #tag_type_cc* rs_std::Option<#ty_tokens>::tag() noexcept {
                return reinterpret_cast<#tag_type_cc*>(
                    reinterpret_cast<char*>(this) + #tag_offset);
            }
        }),
        ..Default::default()
    };

    let option_api = match tag_encoding {
        rustc_abi::TagEncoding::Direct => {
            // Option::None is variant 0. Option::Some is variant 1.
            let some_variant = &variants[some_idx];
            let payload_offset = Literal::u64_unsuffixed(some_variant.fields.offset(0).bytes());
            let discr_for_some = self_ty.discriminant_for_variant(tcx, some_idx).expect(
                "We do not support zero sized types. Before generating a specialization, we\
                 check that the type can be formatted as a C++ type. That should exclude this case \
                 from occurring",
            );
            let some_discr_val = literal_of_tag_ty(discr_for_some.val, tag_type);

            OptionApi {
                db,
                arg_ty_rs: arg_ty,
                arg_ty: ty_tokens.clone(),
                needs_drop,
                tag_method,
                none_val: quote! { #none_discr_val },
                write_some_to_tag: quote! { *this->tag() = #some_discr_val; },
                some_ptr_val: quote! {
                    reinterpret_cast<#ty_tokens*>(
                        reinterpret_cast<char*>(this) + #payload_offset)
                },
                tag_type_cc: tag_type_cc.clone(),
            }
        }
        rustc_abi::TagEncoding::Niche { niche_start, ref niche_variants, .. } => {
            let none_relative_idx =
                none_idx.as_u32().strict_sub(niche_variants.start().as_u32()) as u128;
            let none_relative_val = literal_of_tag_ty(*niche_start + none_relative_idx, tag_type);
            OptionApi {
                db,
                arg_ty_rs: arg_ty,
                arg_ty: ty_tokens.clone(),
                needs_drop,
                tag_method,
                none_val: quote! { #none_relative_val },
                some_ptr_val: quote! {
                    reinterpret_cast<#ty_tokens*>(this)
                },
                // With a niche, the Some variant is implicitly encoded. We don't need to write out
                // a discriminant value. It is accomplished by writing a value to the Some payload.
                write_some_to_tag: quote! {},
                tag_type_cc: tag_type_cc.clone(),
            }
        }
    };
    let ty::TyKind::Adt(adt, _) = self_ty.kind() else {
        unreachable!("Option<T> must be an ADT");
    };
    let arg_ty_for_rs = db.format_ty_for_rs(arg_ty)?;
    let core = Rc::new(database::AdtCoreBindings {
        def_id: adt.did(),
        keyword: quote! { struct },
        cc_short_name: format_ident!("Option"),
        rs_fully_qualified_name: quote! { std::option::Option<#arg_ty_for_rs> },
        cc_fully_qualified_name: quote! { rs_std::Option<#ty_tokens> },
        self_ty,
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

    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_{}", name);
    let main_api_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        template<> __NEWLINE__
        struct rs_std::Option<#ty_tokens> { __NEWLINE__
        public:
            #main_api_tokens __NEWLINE__

            #fields_main_api __NEWLINE__
        }; __NEWLINE__

        __HASH_TOKEN__ endif __NEWLINE__
        __NEWLINE__
    };

    let guard_name = format_ident!("_CRUBIT_BINDINGS_FOR_IMPL_{}", name);
    let cc_details_tokens = cc_details.into_tokens(&mut prereqs);
    let cc_details_tokens = quote! {
        __HASH_TOKEN__ ifndef #guard_name __NEWLINE__
        __HASH_TOKEN__ define #guard_name __NEWLINE__
        #cc_details_tokens
        __HASH_TOKEN__ endif __NEWLINE__
        __NEWLINE__
    };
    Ok(ApiSnippets {
        main_api: CcSnippet { tokens: main_api_tokens, prereqs },
        cc_details: CcSnippet::new(cc_details_tokens),
        rs_details,
    })
}

/// Generate a template specialization.
pub fn generate_template_specialization<'tcx>(
    db: &BindingsGenerator<'tcx>,
    specialization: TemplateSpecialization<'tcx>,
) -> ApiSnippets<'tcx> {
    let snippet_res: Result<ApiSnippets<'tcx>, (Ty<'tcx>, arc_anyhow::Error)> =
        match &specialization {
            TemplateSpecialization::RsStdOption { arg_ty, self_ty } => {
                specialize_option(db, *arg_ty, *self_ty)
                    .map(|mut snippets| {
                        let ty::TyKind::Adt(adt, _) = arg_ty.kind() else {
                            return snippets;
                        };
                        let def_id = adt.did();
                        if !snippets.main_api.prereqs.defs.remove(&def_id) {
                            return snippets;
                        }
                        // If our specialization depends on it's argument type, we need to forward declare that type.
                        snippets.main_api.prereqs.fwd_decls.insert(def_id);
                        snippets
                    })
                    .map_err(|e| (*self_ty, e))
            }
        };

    let mut snippets = snippet_res.unwrap_or_else(|(ty, err)| {
        use rustc_hir::def::Namespace;
        use rustc_middle::ty::print::FmtPrinter;
        use rustc_middle::ty::print::PrettyPrinter;
        let tcx = db.tcx();
        let name =
            FmtPrinter::print_string(tcx, Namespace::TypeNS, |fmt| fmt.pretty_print_type(ty))
                .unwrap_or_else(|_| "<unknown type>".to_string());
        let msg = format!("Error generating specialization for `{name}`: {err:#}");
        CcSnippet::new(quote! { __NEWLINE__ __NEWLINE__ __COMMENT__ #msg __NEWLINE__ })
            .into_main_api()
    });
    // Because we reuse logic from generate_struct_and_union here, we will add our `self_ty` as a template specialization of it's own specialization creating a depedency cycle.
    // We break that loop manually here to avoid that.
    snippets.main_api.prereqs.template_specializations.remove(&specialization);
    snippets
}
