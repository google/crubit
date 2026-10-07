// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Bindings for non-`repr(C)` Rust enums under `CrubitFeature::EnumApiV2`.
//!
//! Each variant becomes a struct nested in the enum, deriving from
//! `::crubit::internal::Variant`, and the enum holds them in a
//! `::crubit::internal::VariadicUnionStorage`. See `support/rs_std/enum.h` for the C++ API this
//! produces, and `support/rs_std/enum_test_option_i32.h` for a hand-written example of the
//! output.

extern crate rustc_abi;
extern crate rustc_middle;
extern crate rustc_span;

use crate::generate_struct_and_union::{anonymous_field_ident, scalar_value_to_string};
use crate::{
    can_be_made_layout_compatible, format_cc_ident, generate_deprecated_tag, generate_doc_comment,
    get_layout, get_scalar_int_type, is_copy,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::CcInclude;
use crubit_feature::CrubitFeature;
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet};
use database::{AdtCoreBindings, BindingsGenerator, TypeLocation};
use error_report::{anyhow, bail, ensure};
use proc_macro2::{Ident, Literal, TokenStream};
use query_compiler::post_analysis_typing_env;
use quote::{format_ident, quote};
#[rustversion::since(2026-05-18)]
use rustc_abi::LayoutData;
use rustc_abi::{FieldIdx, FieldsShape, Layout, Size, TagEncoding, VariantIdx, Variants};
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::ty::{self, Ty, TyCtxt, TyKind};
use rustc_span::def_id::DefId;
use std::collections::HashSet;

/// What `EnumApiV2` needs to know about an enum to generate its bindings.
pub(crate) struct EnumLayout<'tcx> {
    /// The enum's `Discriminant` type: the type of every variant's `kDiscriminant`, and of the
    /// value its tag encoding decodes.
    discriminant_ty: Ty<'tcx>,
    tag_encoding: EnumTagEncoding,
    variants: Vec<EnumVariant<'tcx>>,
}

/// Mirrors the policies in `crubit::internal::tag_encoding` (support/rs_std/internal/enum.h).
enum EnumTagEncoding {
    /// `rustc_abi::Variants::Single`: there is no tag, and the discriminant is always that of the
    /// one variant.
    Single,
    /// `rustc_abi::TagEncoding::Direct`: every variant stores its discriminant in a tag at
    /// `offset` bytes from the start of the enum.
    Direct { offset: u64 },
}

struct EnumVariant<'tcx> {
    def_id: DefId,
    cc_name: Ident,
    /// The variant's `kDiscriminant`, as a literal of the enum's `discriminant_ty`.
    discriminant: TokenStream,
    /// The tag this variant stores, if any.
    tag: Option<EnumTag>,
    /// In declaration order, which is the order of the variant constructor's parameters. (The
    /// members are declared in memory order instead.)
    fields: Vec<EnumField<'tcx>>,
}

struct EnumTag {
    offset: u64,
    size: u64,
    /// A literal of the enum's `discriminant_ty`.
    value: TokenStream,
}

struct EnumField<'tcx> {
    cc_name: Ident,
    cpp_type: CcSnippet<'tcx>,
    offset: u64,
    size: u64,
    doc_comment: TokenStream,
    attributes: Vec<TokenStream>,
}

/// Returns `None` if `EnumApiV2` doesn't apply to `core`: the feature is off, `core` is not an
/// enum, or it is a `#[repr(C)]` enum, which keeps its own bindings.
///
/// Otherwise, returns the enum's layout, or the reason it can't use `EnumApiV2` and falls back
/// to an opaque blob of bytes.
///
/// `member_function_names` are the names of the enum's members so far, which variant names must
/// not collide with.
pub(crate) fn analyze_enum<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
    member_function_names: &HashSet<String>,
) -> Option<Result<EnumLayout<'tcx>>> {
    let TyKind::Adt(adt_def, generic_args) = core.common.self_ty.kind() else {
        return None;
    };
    if !adt_def.is_enum()
        || adt_def.repr().c()
        || !db.crate_features(db.source_crate_num()).contains(CrubitFeature::EnumApiV2)
    {
        return None;
    }
    Some(analyze_enum_layout(db, core, *adt_def, generic_args, member_function_names))
}

fn analyze_enum_layout<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
    adt_def: ty::AdtDef<'tcx>,
    generic_args: ty::GenericArgsRef<'tcx>,
    member_function_names: &HashSet<String>,
) -> Result<EnumLayout<'tcx>> {
    let tcx = db.tcx();
    let self_ty = core.common.self_ty;
    let def_id = adt_def.did();

    ensure!(
        !adt_def.is_variant_list_non_exhaustive(),
        "`#[non_exhaustive]` enums are not supported, because C++ code could not handle the \
         variants they may gain"
    );
    // TODO(b/489126000): Support enums with drop glue and non-`Copy` enums.
    ensure!(is_copy(tcx, def_id, self_ty), "Only `Copy` enums are supported so far");

    let layout = get_layout(tcx, self_ty)?;
    let (discriminant_ty, tag_encoding) = match layout.variants() {
        Variants::Empty => bail!("Uninhabited enums have no variants to bind"),
        Variants::Single { index } => {
            ensure!(
                adt_def.variants().len() == 1,
                "Enums with uninhabited variants are not supported"
            );
            let discr = self_ty.discriminant_for_variant(tcx, *index).expect("Invalid VariantIdx");
            let (size, signed) = discr.ty.int_size_and_signed(tcx);
            (int_ty_of_size(tcx, size, signed)?, EnumTagEncoding::Single)
        }
        Variants::Multiple { tag, tag_encoding: TagEncoding::Direct, tag_field, .. } => {
            let offset = layout.fields().offset(tag_field.as_usize()).bytes();
            (get_scalar_int_type(tcx, *tag), EnumTagEncoding::Direct { offset })
        }
        Variants::Multiple { tag_encoding: TagEncoding::Niche { .. }, .. } => {
            // TODO(b/489126000): Support niche-encoded enums.
            bail!("Niche-encoded enums are not supported yet")
        }
    };

    // Names a variant can't have, because the variant's struct would collide with another member
    // of the enum (or with the enum itself, which a nested class can't be named after).
    let mut reserved_names = member_function_names.clone();
    reserved_names.extend(
        ["Discriminant", "storage_", "__crubit_field_offset_assertions", "std"].map(String::from),
    );
    reserved_names.insert(tcx.item_name(def_id).to_string());
    reserved_names.insert(core.common.cc_short_name.to_string());

    let typing_env = post_analysis_typing_env(tcx, def_id);
    let variants = adt_def
        .variants()
        .iter_enumerated()
        .map(|(variant_index, variant_def)| {
            let name = variant_def.name.as_str();
            ensure!(
                !variant_def.is_field_list_non_exhaustive(),
                "Variant `{name}` is `#[non_exhaustive]`"
            );
            ensure!(
                !reserved_names.contains(name),
                "Variant `{name}` would collide with another member of the enum's C++ class"
            );
            let cc_name = format_cc_ident(db, name)?;

            let (discriminant, tag) = match tag_encoding {
                EnumTagEncoding::Single => {
                    let discr = self_ty
                        .discriminant_for_variant(tcx, variant_index)
                        .expect("Invalid VariantIdx");
                    let size = discriminant_ty.int_size_and_signed(tcx).0;
                    let (value, _) = ty::ScalarInt::truncate_from_uint(discr.val, size);
                    (int_literal(tcx, value, discriminant_ty)?, None)
                }
                EnumTagEncoding::Direct { offset } => {
                    let value = tcx
                        .tag_for_variant(typing_env.as_query_input((
                            tcx.erase_and_anonymize_regions(self_ty),
                            variant_index,
                        )))
                        .expect("Every variant of a directly-tagged enum has a tag");
                    let literal = int_literal(tcx, value, discriminant_ty)?;
                    let tag =
                        EnumTag { offset, size: value.size().bytes(), value: literal.clone() };
                    (literal, Some(tag))
                }
            };

            let field_shape = variant_field_shape(layout, variant_index);
            let fields = variant_def
                .fields
                .iter_enumerated()
                .map(|(field_index, field_def)| {
                    analyze_field(db, generic_args, field_def, field_index, &field_shape)
                        .with_context(|| format!("Variant `{name}`"))
                })
                .collect::<Result<Vec<_>>>()?;
            for field in &fields {
                let field_name = field.cc_name.to_string();
                ensure!(
                    field_name != "Make" && field_name != "kDiscriminant" && field_name != name,
                    "Field `{field_name}` of variant `{name}` would collide with a member of the \
                     variant's C++ struct"
                );
            }
            check_no_overlap(name, tag.as_ref(), &fields)?;

            Ok(EnumVariant { def_id: variant_def.def_id, cc_name, discriminant, tag, fields })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(EnumLayout { discriminant_ty, tag_encoding, variants })
}

fn analyze_field<'tcx>(
    db: &BindingsGenerator<'tcx>,
    generic_args: ty::GenericArgsRef<'tcx>,
    field_def: &ty::FieldDef,
    field_index: FieldIdx,
    field_shape: &FieldsShape<FieldIdx>,
) -> Result<EnumField<'tcx>> {
    let tcx = db.tcx();
    let name = field_def.name.as_str();
    crate::field_def_is_pub_and_stable(tcx, field_def)
        .map_err(|private_or_unstable| anyhow!("Field `{name}` is {private_or_unstable}"))?;

    let ty = field_def.ty(tcx, generic_args);
    let ty = crate::normalize_ty(tcx, tcx.param_env(field_def.did), ty);
    let size = get_layout(tcx, ty)?.size().bytes();
    ensure!(size != 0, "Field `{name}` is zero-sized, and C++ does not support zero-sized types");
    ensure!(
        can_be_made_layout_compatible(db, ty).unwrap_or(true),
        "Field `{name}` is a bridged type and might not be layout-compatible with the C++ type \
         (b/400633609)"
    );
    let cpp_type = db
        .format_ty_for_cc(ty, TypeLocation::Field)
        .and_then(|cpp_type| {
            cpp_type.resolve_feature_requirements(db.crate_features(db.source_crate_num()))
        })
        .with_context(|| format!("Field `{name}`"))?;

    // Tuple variants' fields are named `0`, `1`, ..., which aren't C++ identifiers.
    let cc_name = if name.starts_with(|c: char| c.is_ascii_digit()) {
        anonymous_field_ident(field_index.as_usize())
    } else {
        format_cc_ident(db, name)?
    };

    let mut attributes = vec![];
    if let Some(deprecated) = generate_deprecated_tag(tcx, field_def.did) {
        attributes.push(deprecated);
    }

    Ok(EnumField {
        cc_name,
        cpp_type,
        offset: field_shape.offset(field_index.as_usize()).bytes(),
        size,
        doc_comment: generate_doc_comment(db, field_def.did),
        attributes,
    })
}

/// Checks that a variant's tag and fields don't overlap, which rustc guarantees. A violation
/// would mean we misread the layout, and would otherwise surface as a confusing C++ error.
fn check_no_overlap(variant_name: &str, tag: Option<&EnumTag>, fields: &[EnumField]) -> Result<()> {
    let mut ranges = tag
        .map(|tag| (tag.offset, tag.size))
        .into_iter()
        .chain(fields.iter().map(|field| (field.offset, field.size)))
        .collect::<Vec<_>>();
    ranges.sort();
    for window in ranges.windows(2) {
        let [(offset, size), (next_offset, _)] = window else { unreachable!() };
        ensure!(
            offset + size <= *next_offset,
            "Internal error: the members of variant `{variant_name}` overlap"
        );
    }
    Ok(())
}

/// Returns the offsets of the fields of the variant `variant_index` of an enum with `layout`.
/// They are relative to the start of the enum.
#[rustversion::before(2026-05-18)]
fn variant_field_shape(layout: Layout<'_>, variant_index: VariantIdx) -> FieldsShape<FieldIdx> {
    match layout.variants() {
        Variants::Multiple { variants, .. } => variants[variant_index].fields.clone(),
        Variants::Single { .. } | Variants::Empty => layout.fields().clone(),
    }
}

/// Returns the offsets of the fields of the variant `variant_index` of an enum with `layout`.
/// They are relative to the start of the enum.
#[rustversion::since(2026-05-18)]
fn variant_field_shape(layout: Layout<'_>, variant_index: VariantIdx) -> FieldsShape<FieldIdx> {
    match layout.variants() {
        Variants::Multiple { .. } => LayoutData::for_variant(&layout, variant_index).fields,
        Variants::Single { .. } | Variants::Empty => layout.fields().clone(),
    }
}

/// Returns the Rust fixed-width integer type of the given size and signedness, which formats as
/// the matching C++ `<cstdint>` type, as `crubit::internal::RustcDiscriminant` requires.
fn int_ty_of_size<'tcx>(tcx: TyCtxt<'tcx>, size: Size, signed: bool) -> Result<Ty<'tcx>> {
    Ok(match (size.bytes(), signed) {
        (1, true) => tcx.types.i8,
        (2, true) => tcx.types.i16,
        (4, true) => tcx.types.i32,
        (8, true) => tcx.types.i64,
        (1, false) => tcx.types.u8,
        (2, false) => tcx.types.u16,
        (4, false) => tcx.types.u32,
        (8, false) => tcx.types.u64,
        _ => bail!("Unsupported discriminant size: {} bytes", size.bytes()),
    })
}

/// Formats `value` as a C++ literal of the integer type `ty`.
fn int_literal<'tcx>(tcx: TyCtxt<'tcx>, value: ty::ScalarInt, ty: Ty<'tcx>) -> Result<TokenStream> {
    scalar_value_to_string(tcx, Scalar::Int(value), *ty.kind())?
        .parse::<TokenStream>()
        .map_err(|err| anyhow!("Invalid discriminant literal: {err}"))
}

/// Generates the members of the enum's C++ class: its `Discriminant` type, a struct per variant,
/// the in-place constructor, and the storage. Replaces `generate_fields` for enums with an
/// `EnumLayout`.
pub(crate) fn generate_enum<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
    layout: EnumLayout<'tcx>,
) -> ApiSnippets<'tcx> {
    let cc_short_name = &core.common.cc_short_name;
    let cc_fully_qualified_name = &core.common.cc_fully_qualified_name;

    let mut prereqs = CcPrerequisites::default();
    prereqs.includes.insert(db.support_header("rs_std/enum.h"));
    prereqs.includes.insert(CcInclude::utility());
    let discriminant_ty = db
        .format_ty_for_cc(layout.discriminant_ty, TypeLocation::Other)
        .expect("Fixed-width integer types should always have a C++ type")
        .into_tokens(&mut prereqs);

    let variant_structs: TokenStream = layout
        .variants
        .iter()
        .map(|variant| {
            generate_variant(db, cc_fully_qualified_name, &discriminant_ty, variant, &mut prereqs)
        })
        .collect();

    let tag_encoding = match layout.tag_encoding {
        EnumTagEncoding::Single => {
            let discriminant = &layout.variants[0].discriminant;
            quote! { ::crubit::internal::tag_encoding::Single<#discriminant_ty, #discriminant> }
        }
        EnumTagEncoding::Direct { offset } => {
            let offset = Literal::u64_unsuffixed(offset);
            quote! { ::crubit::internal::tag_encoding::Direct<#discriminant_ty, #offset> }
        }
    };
    let variant_names = layout.variants.iter().map(|variant| &variant.cc_name);

    let assertions: TokenStream = layout
        .variants
        .iter()
        .flat_map(|variant| {
            let variant_name = &variant.cc_name;
            variant.fields.iter().map(move |field| {
                let field_name = &field.cc_name;
                let offset = Literal::u64_unsuffixed(field.offset);
                quote! {
                    {
                        using __crubit_assert_type = #cc_fully_qualified_name::#variant_name;
                        static_assert(#offset == offsetof(__crubit_assert_type, #field_name));
                    }
                }
            })
        })
        .collect();
    let has_assertions = !assertions.is_empty();
    let (assertions_decl, cc_details) = if has_assertions {
        (
            quote! { private: static void __crubit_field_offset_assertions(); },
            CcSnippet::with_include(
                quote! {
                    inline void #cc_fully_qualified_name::__crubit_field_offset_assertions() {
                        #assertions
                    }
                },
                CcInclude::cstddef(),
            ),
        )
    } else {
        (quote! {}, CcSnippet::default())
    };

    let main_api = CcSnippet {
        prereqs,
        tokens: quote! {
            public: __NEWLINE__
            using Discriminant = #discriminant_ty; __NEWLINE__
            #variant_structs
            __NEWLINE__
            __COMMENT__ "Constructs the enum holding the variant `__Variant`, built in place from `__args`. `__Variant::Make(__args...)` is shorthand for this."
            template <typename __Variant, typename... __Args>
                requires ::crubit::internal::ConstructibleVariantOf<
                    __Variant, #cc_fully_qualified_name, __Args...>
            constexpr explicit #cc_short_name(
                ::std::in_place_type_t<__Variant> __variant, __Args&&... __args)
                : storage_(__variant, ::std::forward<__Args>(__args)...) {}
            __NEWLINE__
            private: __NEWLINE__
            friend struct ::crubit::internal::EnumAccess;
            ::crubit::internal::VariadicUnionStorage<#tag_encoding, #(#variant_names),*> storage_;
            __NEWLINE__
            #assertions_decl
        },
    };

    // Unlike for structs, there are no Rust-side offset assertions: `offset_of!` on enum
    // variants is unstable (`offset_of_enum`). The C++-side ones above are still checked against
    // rustc's layout, since that is where `field.offset` comes from.
    ApiSnippets { main_api, cc_details, ..Default::default() }
}

/// Generates the struct for one variant of an enum.
fn generate_variant<'tcx>(
    db: &BindingsGenerator<'tcx>,
    enum_cc_fully_qualified_name: &TokenStream,
    discriminant_ty: &TokenStream,
    variant: &EnumVariant<'tcx>,
    prereqs: &mut CcPrerequisites<'tcx>,
) -> TokenStream {
    let name = &variant.cc_name;
    let discriminant = &variant.discriminant;
    let doc_comment = generate_doc_comment(db, variant.def_id);
    let attributes = generate_deprecated_tag(db.tcx(), variant.def_id);

    let params = variant
        .fields
        .iter()
        .map(|field| {
            let cpp_type = field.cpp_type.clone().into_tokens(prereqs);
            let field_name = &field.cc_name;
            quote! { #cpp_type #field_name }
        })
        .collect::<Vec<_>>();

    // Members are declared in memory order, at the offsets rustc chose, with explicit padding in
    // between. So is the member initializer list, to match.
    enum Member<'a, 'tcx> {
        Tag(&'a EnumTag),
        Field(&'a EnumField<'tcx>),
    }
    let mut members = variant
        .tag
        .iter()
        .map(|tag| (tag.offset, tag.size, Member::Tag(tag)))
        .chain(variant.fields.iter().map(|field| (field.offset, field.size, Member::Field(field))))
        .collect::<Vec<_>>();
    members.sort_by_key(|(offset, _, _)| *offset);

    let initializers = members.iter().filter_map(|(_, _, member)| match member {
        Member::Tag(_) => None,
        Member::Field(field) => {
            let field_name = &field.cc_name;
            Some(quote! { #field_name(::std::move(#field_name)) })
        }
    });
    let initializers = initializers.collect::<Vec<_>>();
    let initializer_list = if initializers.is_empty() {
        quote! {}
    } else {
        quote! { : #(#initializers),* }
    };

    let mut is_public = false;
    let mut set_visibility = |public: bool| {
        if is_public == public {
            quote! {}
        } else {
            is_public = public;
            if public {
                quote! { public: }
            } else {
                quote! { private: }
            }
        }
    };
    let mut member_decls = quote! {};
    let mut end = 0;
    let mut padding_index = 0;
    for (offset, size, member) in &members {
        if *offset > end {
            let visibility = set_visibility(false);
            let padding_name = format_ident!("__crubit_padding{padding_index}");
            let padding_size = Literal::u64_unsuffixed(offset - end);
            padding_index += 1;
            member_decls.extend(quote! {
                #visibility
                [[maybe_unused]] unsigned char #padding_name[#padding_size] = {};
            });
        }
        end = offset + size;
        match member {
            Member::Tag(tag) => {
                let visibility = set_visibility(false);
                let value = &tag.value;
                member_decls.extend(quote! {
                    #visibility
                    [[maybe_unused]] #discriminant_ty __crubit_tag = #value;
                });
            }
            Member::Field(field) => {
                let visibility = set_visibility(true);
                let cpp_type = field.cpp_type.clone().into_tokens(prereqs);
                let field_name = &field.cc_name;
                let field_doc_comment = &field.doc_comment;
                let field_attributes = &field.attributes;
                member_decls.extend(quote! {
                    #visibility __NEWLINE__
                    #field_doc_comment
                    #(#field_attributes)*
                    #cpp_type #field_name;
                });
            }
        }
    }

    quote! {
        __NEWLINE__ #doc_comment
        struct #attributes #name : public ::crubit::internal::Variant<
            #name, #enum_cc_fully_qualified_name, #discriminant> {
            public: __NEWLINE__
            constexpr explicit #name(::crubit::internal::VariantKey #(, #params)*)
                #initializer_list {}
            __NEWLINE__
            private: __NEWLINE__
            __COMMENT__ "Variants can't be copied or moved on their own, only as part of the enum. See `::crubit::internal::Variant`."
            template <typename...>
            friend union ::crubit::internal::VariadicUnion;
            #name(const #name&) = default;
            #name(#name&&) = default;
            #name& operator=(const #name&) = default;
            #name& operator=(#name&&) = default;
            __NEWLINE__
            #member_decls
        };
        __NEWLINE__
    }
}
