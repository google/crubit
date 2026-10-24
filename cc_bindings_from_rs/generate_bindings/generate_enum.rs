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

#![feature(rustc_private)]
#![feature(stmt_expr_attributes)]

extern crate rustc_abi;
extern crate rustc_middle;
extern crate rustc_span;

use arc_anyhow::{Context, Result};
use code_gen_utils::CcInclude;
use crubit_feature::CrubitFeature;
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet, CcSnippets};
use database::{AdtCoreBindings, BindingsGenerator, TypeLocation};
use error_report::{anyhow, bail, ensure};
use format_type::{can_be_made_layout_compatible, format_cc_ident};
use generate_comment::{generate_deprecated_tag, generate_doc_comment};
use proc_macro2::{Ident, Literal, TokenStream};
use query_compiler::{field_def_is_pub_and_stable, get_layout, normalize_ty};
use quote::{format_ident, quote};
#[rustversion::since(2026-05-18)]
use rustc_abi::LayoutData;
use rustc_abi::{
    FieldIdx, FieldsShape, Integer, IntegerType, Layout, Primitive, TagEncoding, VariantIdx,
    Variants,
};
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::ty::layout::IntegerExt;
use rustc_middle::ty::{self, TyCtxt, TyKind};
use rustc_span::def_id::DefId;
use std::collections::HashSet;
use std::ops::RangeInclusive;

/// What `EnumApiV2` needs to know about an enum to generate its bindings.
pub struct EnumLayout<'db, 'tcx> {
    /// The enum's `Discriminant` type: the type of every variant's `kDiscriminant`, and of the
    /// value its tag encoding decodes. Is the declared discriminant type (`isize` by default) for
    /// `Single` enums, since this matches rustc.
    discriminant_ty: DiscriminantTy,
    tag_encoding: EnumTagEncoding,
    variants: Vec<EnumVariant<'db, 'tcx>>,
}

/// One of the integer types that `crubit::internal::RustcDiscriminant` allows, except the 128-bit
/// ones, which have no portable C++ type.
#[derive(Clone, Copy)]
struct DiscriminantTy {
    integer: Integer,
    signed: bool,
}

impl DiscriminantTy {
    fn new(integer: Integer, signed: bool) -> Result<Self> {
        ensure!(integer != Integer::I128, "128-bit discriminants are not supported (b/254094650)");
        Ok(Self { integer, signed })
    }

    /// The type of the tag `tag`. Pointer tags (as in `Option<&T>`) are mapped to the
    /// pointer-sized unsigned integer, matching rustc's `Primitive::to_int_ty`.
    fn of_tag(tcx: TyCtxt, tag: rustc_abi::Scalar) -> Result<Self> {
        match tag.primitive() {
            Primitive::Int(integer, signed) => Self::new(integer, signed),
            Primitive::Pointer(_) => {
                Self::new(tcx.data_layout.ptr_sized_integer(), /* signed= */ false)
            }
            primitive => bail!("Unsupported tag type: {primitive:?}"),
        }
    }

    /// The enum's declared discriminant type (`isize` unless `#[repr]` says otherwise).
    fn declared(tcx: TyCtxt, adt_def: ty::AdtDef) -> Result<Self> {
        match adt_def.repr().discr_type() {
            IntegerType::Fixed(integer, signed) => Self::new(integer, signed),
            IntegerType::Pointer(signed) => Self::new(tcx.data_layout.ptr_sized_integer(), signed),
        }
    }

    fn size(self) -> rustc_abi::Size {
        self.integer.size()
    }

    fn cc_type(self) -> TokenStream {
        match (self.integer, self.signed) {
            (Integer::I8, true) => quote! { ::std::int8_t },
            (Integer::I16, true) => quote! { ::std::int16_t },
            (Integer::I32, true) => quote! { ::std::int32_t },
            (Integer::I64, true) => quote! { ::std::int64_t },
            (Integer::I8, false) => quote! { ::std::uint8_t },
            (Integer::I16, false) => quote! { ::std::uint16_t },
            (Integer::I32, false) => quote! { ::std::uint32_t },
            (Integer::I64, false) => quote! { ::std::uint64_t },
            (Integer::I128, _) => unreachable!("Rejected by `DiscriminantTy::new`"),
        }
    }

    /// Formats `value` as a C++ literal of this type, truncating it to this type's width.
    fn literal_from_uint(self, tcx: TyCtxt, value: u128) -> Result<TokenStream> {
        let (scalar, _truncated) = ty::ScalarInt::truncate_from_uint(value, self.size());
        self.literal(tcx, scalar)
    }

    /// Formats `value` as a C++ literal of this type.
    fn literal(self, tcx: TyCtxt, value: ty::ScalarInt) -> Result<TokenStream> {
        let ty = self.integer.to_ty(tcx, self.signed);
        scalar_value_to_string(tcx, Scalar::Int(value), *ty.kind())?
            .parse::<TokenStream>()
            .map_err(|err| anyhow!("Invalid discriminant literal: {err}"))
    }
}

/// Mirrors the policies in `crubit::internal::tag_encoding` (support/rs_std/internal/enum.h).
enum EnumTagEncoding {
    /// `rustc_abi::Variants::Single`: there is no tag, and the discriminant is always that of the
    /// one variant.
    Single,
    /// `rustc_abi::TagEncoding::Direct`: every variant stores its discriminant in a tag at
    /// `offset` bytes from the start of the enum.
    Direct { offset: u64 },
    /// `rustc_abi::TagEncoding::Niche`: the niche variants store consecutive tag values starting
    /// at `niche_start` at `offset` bytes from the start of the enum; `untagged_variant` has
    /// no tag, and its field at `offset` never holds any of those values. Every variant's
    /// discriminant is its variant index.
    Niche {
        offset: u64,
        untagged_variant: VariantIdx,
        niche_variants: RangeInclusive<VariantIdx>,
        niche_start: TokenStream,
    },
}

struct EnumVariant<'db, 'tcx> {
    def_id: DefId,
    cc_name: Ident,
    /// The variant's `kDiscriminant`, as a literal of the enum's `discriminant_ty`.
    discriminant: TokenStream,
    /// The tag this variant stores, if any.
    tag: Option<EnumTag>,
    /// In declaration order, which is the order of the variant constructor's parameters. (The
    /// members are declared in memory order instead.)
    fields: Vec<EnumField<'db, 'tcx>>,
}

struct EnumTag {
    offset: u64,
    size: u64,
    /// A literal of the enum's `discriminant_ty`.
    value: TokenStream,
}

struct EnumField<'db, 'tcx> {
    /// The field's name, or `__field{index}` for tuple variants' fields, like tuple structs'.
    cc_name: Ident,
    cpp_type: &'db CcSnippet<'tcx>,
    offset: u64,
    size: u64,
    /// Whether the field's C++ type is a literal type with a `constexpr` move constructor, so
    /// a variant constructor taking it by value and move-initializing the member can be
    /// `constexpr`.
    is_constexpr_constructible: bool,
    doc_comment: TokenStream,
    attributes: Vec<TokenStream>,
}

/// Returns `Ok(None)` if `EnumApiV2` doesn't apply to `core`: the feature is off, `core` is not
/// an enum, or it is a `#[repr(C)]` enum, which keeps its own bindings.
///
/// Otherwise, returns the enum's layout, or the reason it can't use `EnumApiV2` and falls back
/// to an opaque blob of bytes.
///
/// `member_function_names` are the names of the enum's members so far, which variant names must
/// not collide with.
pub fn analyze_enum<'db, 'tcx>(
    db: &'db BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
    member_function_names: &HashSet<String>,
) -> Result<Option<EnumLayout<'db, 'tcx>>> {
    let TyKind::Adt(adt_def, generic_args) = core.common.self_ty.kind() else {
        return Ok(None);
    };
    if !adt_def.is_enum()
        || adt_def.repr().c()
        || !db.crate_features(db.source_crate_num()).contains(CrubitFeature::EnumApiV2)
    {
        return Ok(None);
    }
    let (adt_def, generic_args) = (*adt_def, *generic_args);
    let tcx = db.tcx();
    let self_ty = core.common.self_ty;
    let def_id = adt_def.did();

    let layout = get_layout(tcx, self_ty)?;
    let (discriminant_ty, tag_encoding) = match layout.variants() {
        Variants::Empty => bail!("Uninhabited enums have no variants to bind"),
        Variants::Single { .. } => {
            (DiscriminantTy::declared(tcx, adt_def)?, EnumTagEncoding::Single)
        }
        Variants::Multiple { tag, tag_encoding: TagEncoding::Direct, tag_field, .. } => {
            let offset = layout.fields().offset(tag_field.as_usize()).bytes();
            (DiscriminantTy::of_tag(tcx, *tag)?, EnumTagEncoding::Direct { offset })
        }
        Variants::Multiple {
            tag,
            tag_encoding: TagEncoding::Niche { untagged_variant, niche_variants, niche_start },
            tag_field,
            ..
        } => {
            let offset = layout.fields().offset(tag_field.as_usize()).bytes();
            let discriminant_ty = DiscriminantTy::of_tag(tcx, *tag)?;
            let niche_start = discriminant_ty.literal_from_uint(tcx, *niche_start)?;
            (
                discriminant_ty,
                EnumTagEncoding::Niche {
                    offset,
                    untagged_variant: *untagged_variant,
                    niche_variants: (*niche_variants).into(),
                    niche_start,
                },
            )
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

    let typing_env = ty::TypingEnv::fully_monomorphized();
    // Uninhabited variants can never exist, so they get no bindings. (rustc may lay out an enum
    // with several variants as `Single` if all but one of them are uninhabited.)
    let variants = adt_def
        .variants()
        .iter_enumerated()
        .filter_map(|(variant_index, variant_def)| {
            let field_shape = inhabited_variant_field_shape(layout, variant_index)?;
            Some((variant_index, variant_def, field_shape))
        })
        .map(|(variant_index, variant_def, field_shape)| {
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

            let tag_at = |offset| -> Result<Option<EnumTag>> {
                let Some(value) = tcx.tag_for_variant(
                    typing_env
                        .as_query_input((tcx.erase_and_anonymize_regions(self_ty), variant_index)),
                ) else {
                    return Ok(None);
                };
                Ok(Some(EnumTag {
                    offset,
                    size: value.size().bytes(),
                    value: discriminant_ty.literal(tcx, value)?,
                }))
            };
            let (discriminant, tag) = match tag_encoding {
                EnumTagEncoding::Single => {
                    let discr = self_ty
                        .discriminant_for_variant(tcx, variant_index)
                        .expect("Invalid VariantIdx");
                    (discriminant_ty.literal_from_uint(tcx, discr.val)?, None)
                }
                EnumTagEncoding::Direct { offset } => {
                    let tag =
                        tag_at(offset)?.expect("Every variant of a directly-tagged enum has a tag");
                    (tag.value.clone(), Some(tag))
                }
                EnumTagEncoding::Niche { offset, .. } => {
                    let discriminant = discriminant_ty
                        .literal_from_uint(tcx, u128::from(variant_index.as_u32()))?;
                    (discriminant, tag_at(offset)?)
                }
            };

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
                    field_name != "Make",
                    "Field `Make` of variant `{name}` would collide with the variant's `Make` \
                     function, which constructs the enum; consider renaming the field"
                );
                ensure!(
                    field_name != "kDiscriminant",
                    "Field `kDiscriminant` of variant `{name}` would collide with the variant's \
                     `kDiscriminant` constant; consider renaming the field"
                );
                ensure!(
                    field_name != name,
                    "Field `{field_name}` of variant `{name}` has the same name as the variant, \
                     which C++ doesn't allow for a class with a constructor; consider renaming \
                     the field or the variant"
                );
            }
            check_no_overlap(name, tag.as_ref(), &fields)?;

            Ok(EnumVariant { def_id: variant_def.def_id, cc_name, discriminant, tag, fields })
        })
        .collect::<Result<Vec<_>>>()?;
    ensure!(!variants.is_empty(), "Uninhabited enums have no variants to bind");

    Ok(Some(EnumLayout { discriminant_ty, tag_encoding, variants }))
}

fn analyze_field<'db, 'tcx>(
    db: &'db BindingsGenerator<'tcx>,
    generic_args: ty::GenericArgsRef<'tcx>,
    field_def: &ty::FieldDef,
    field_index: FieldIdx,
    field_shape: &FieldsShape<FieldIdx>,
) -> Result<EnumField<'db, 'tcx>> {
    let tcx = db.tcx();
    let name = field_def.name.as_str();
    field_def_is_pub_and_stable(tcx, field_def)
        .map_err(|private_or_unstable| anyhow!("Field `{name}` is {private_or_unstable}"))?;

    let ty = field_def.ty(tcx, generic_args);
    let ty = normalize_ty(tcx, tcx.param_env(field_def.did), ty);
    let size = get_layout(tcx, ty)?.size().bytes();
    ensure!(size != 0, "Field `{name}` is zero-sized, and C++ does not support zero-sized types");
    ensure!(
        can_be_made_layout_compatible(db, ty).unwrap_or(true),
        "Field `{name}` is a bridged type and might not be layout-compatible with the C++ type \
         (b/400633609)"
    );
    let cpp_type =
        db.format_ty_for_cc(ty, TypeLocation::Field).with_context(|| format!("Field `{name}`"))?;
    ensure!(db.is_cpp_move_constructible(ty), "Field `{name}` is not movable in C++");

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
        // Stricter than the `is_cpp_move_constructible` check above: a field can be movable in
        // C++ (e.g. via a non-`constexpr` `MemSwap` move constructor, or as a non-literal type)
        // without being usable in a `constexpr` constructor.
        is_constexpr_constructible: db.is_cpp_constexpr_move_constructible(ty),
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
    for [(offset, size), (next_offset, _)] in ranges.array_windows() {
        ensure!(
            offset + size <= *next_offset,
            "Internal error: the members of variant `{variant_name}` overlap"
        );
    }
    Ok(())
}

/// Returns the offsets of the fields of the variant `variant_index` of an enum with `layout`,
/// relative to the start of the enum, or `None` if the variant is uninhabited.
#[rustversion::before(2026-05-18)]
fn inhabited_variant_field_shape(
    layout: Layout<'_>,
    variant_index: VariantIdx,
) -> Option<FieldsShape<FieldIdx>> {
    match layout.variants() {
        Variants::Multiple { variants, .. } => {
            let variant = &variants[variant_index];
            (!variant.is_uninhabited()).then(|| variant.fields.clone())
        }
        Variants::Single { index } => {
            (*index == variant_index && !layout.is_uninhabited()).then(|| layout.fields().clone())
        }
        Variants::Empty => None,
    }
}

/// Returns the offsets of the fields of the variant `variant_index` of an enum with `layout`,
/// relative to the start of the enum, or `None` if the variant is uninhabited.
#[rustversion::since(2026-05-18)]
fn inhabited_variant_field_shape(
    layout: Layout<'_>,
    variant_index: VariantIdx,
) -> Option<FieldsShape<FieldIdx>> {
    // Note: `LayoutData::is_variant_uninhabited` would be simpler, but it was only added on
    // 2026-08-06 (rust-lang/rust#160398), so it is unavailable in Rust 1.98.
    match layout.variants() {
        Variants::Multiple { variants, .. } => (!variants[variant_index].is_uninhabited())
            .then(|| LayoutData::for_variant(&layout, variant_index).fields),
        Variants::Single { index } => {
            (*index == variant_index && !layout.is_uninhabited()).then(|| layout.fields().clone())
        }
        Variants::Empty => None,
    }
}

/// Generates the members of the enum's C++ class: its `Discriminant` type, a struct per variant,
/// the in-place constructor, and the storage. Replaces `generate_fields` for enums with an
/// `EnumLayout`.
pub fn generate_enum<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
    layout: EnumLayout<'_, 'tcx>,
) -> ApiSnippets<'tcx> {
    let cc_short_name = &core.common.cc_short_name;
    let cc_fully_qualified_name = &core.common.cc_fully_qualified_name;

    let mut prereqs = CcPrerequisites::default();
    prereqs.includes.insert(db.support_header("rs_std/enum.h"));
    prereqs.includes.insert(CcInclude::utility());
    prereqs.includes.insert(CcInclude::cstdint());
    let discriminant_ty = layout.discriminant_ty.cc_type();

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
        EnumTagEncoding::Niche { offset, untagged_variant, niche_variants, niche_start } => {
            let offset = Literal::u64_unsuffixed(offset);
            let untagged = Literal::u32_unsuffixed(untagged_variant.as_u32());
            let start = Literal::u32_unsuffixed(niche_variants.start().as_u32());
            let end = Literal::u32_unsuffixed(niche_variants.end().as_u32());
            quote! {
                ::crubit::internal::tag_encoding::Niche<
                    #discriminant_ty, #offset, #untagged, #start, #end, #niche_start>
            }
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
    let (assertions_decl, cc_details) = if assertions.is_empty() {
        Default::default()
    } else {
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
    variant: &EnumVariant<'_, 'tcx>,
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
            let cpp_type = field.cpp_type.into_tokens(prereqs);
            let field_name = &field.cc_name;
            quote! { #cpp_type #field_name }
        })
        .collect::<Vec<_>>();

    // Members are declared in memory order, at the offsets rustc chose, with explicit padding in
    // between. So is the member initializer list, to match.
    enum Member<'a, 'db, 'tcx> {
        Tag(&'a EnumTag),
        Field(&'a EnumField<'db, 'tcx>),
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
                quote! { public: __NEWLINE__ }
            } else {
                quote! { private: __NEWLINE__ }
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
                let cpp_type = field.cpp_type.into_tokens(prereqs);
                let field_name = &field.cc_name;
                let field_doc_comment = &field.doc_comment;
                let field_attributes = &field.attributes;
                member_decls.extend(quote! {
                    #visibility
                    #field_doc_comment
                    #(#field_attributes)*
                    #cpp_type #field_name;
                });
            }
        }
    }

    let constexpr = if variant.fields.iter().all(|f| f.is_constexpr_constructible) {
        quote! { constexpr }
    } else {
        quote! {}
    };

    quote! {
        __NEWLINE__ #doc_comment
        struct #attributes #name : public ::crubit::internal::Variant<
            #name, #enum_cc_fully_qualified_name, #discriminant> {
            public: __NEWLINE__
            #constexpr explicit #name(::crubit::internal::VariantKey #(, #params)*)
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

/// Returns a string representation of the value of a given numeric Scalar having a given TyKind.
pub fn scalar_value_to_string(tcx: TyCtxt, scalar: Scalar, kind: TyKind) -> Result<String> {
    let scalar = match scalar {
        Scalar::Int(i) => i,
        Scalar::Ptr(..) => bail!("Pointer values cannot be used as scalar constants."),
    };

    // Print positive integers directly if they fit in an i64, since `int` is guaranteed to be at
    // least 16 bits wide.
    if matches!(kind, TyKind::Uint(_)) {
        let value: u128 = scalar.to_bits_unchecked();
        if value < (i16::MAX as u128) {
            return Ok((value as i16).to_string());
        }
    }

    use ty::FloatTy::*;
    use ty::IntTy::*;
    use ty::TyKind;
    use ty::UintTy::*;

    Ok(match kind {
        TyKind::Bool => scalar.try_to_bool().unwrap().to_string(),
        TyKind::Int(I8) => scalar.to_i8().to_string(),
        TyKind::Int(I16) => scalar.to_i16().to_string(),
        TyKind::Int(I32) => format!("INT32_C({})", scalar.to_i32()),
        TyKind::Uint(U8) => scalar.to_u8().to_string(),
        TyKind::Uint(U16) => format!("UINT16_C({})", scalar.to_u16()),
        TyKind::Uint(U32) => format!("UINT32_C({})", scalar.to_u32()),
        TyKind::Uint(U64) => format!("UINT64_C({})", scalar.to_u64()),
        TyKind::Float(F32) => format!("{}f", scalar.to_f32()),
        TyKind::Float(F64) => format!("{}L", scalar.to_f64()),
        TyKind::Uint(Usize) => format!("UINT64_C({})", scalar.to_target_usize(tcx)),

        // Signed integer minimums cannot be expressed with literals, as `-<int>` parses as a unary
        // minus operator applied to an out-of-range (for signed types) integer literal.
        TyKind::Int(I64) => {
            let value = scalar.to_i64();
            if value == i64::MIN {
                "INT64_MIN".to_string()
            } else {
                format!("INT64_C({value})")
            }
        }
        TyKind::Int(ty::IntTy::Isize) => {
            let value = scalar.to_target_isize(tcx);
            if value == i64::MIN {
                "INT64_MIN".to_string()
            } else {
                format!("INT64_C({value})")
            }
        }
        // Handle ffi_11 wrapper types.
        TyKind::Adt(adt, _) if tcx.crate_name(adt.did().krate).as_str() == "ffi_11" => {
            let name = tcx.item_name(adt.did());
            match name.as_str() {
                "c_char" => scalar.to_u8().to_string(),
                // If ffi_11::c_long is a wrapper type (and not a type alias) it is 32 bit on
                // LLP64 and ILP32 platforms, but 64 bit on LP64 platforms where `int64_t` is
                // `long long` (e.g. iOS). Same for c_ulong.
                "c_long" if scalar.size().bytes() == 8 => format!("INT64_C({})", scalar.to_i64()),
                "c_long" => format!("INT32_C({})", scalar.to_i32()),
                "c_ulong" if scalar.size().bytes() == 8 => {
                    format!("UINT64_C({})", scalar.to_u64())
                }
                "c_ulong" => format!("UINT32_C({})", scalar.to_u32()),
                "c_longlong" => format!("INT64_C({})", scalar.to_i64()),
                "c_ulonglong" => format!("UINT64_C({})", scalar.to_u64()),
                _ => bail!("Unsupported ffi_11 type: {:?}", kind),
            }
        }
        _ => bail!("Unsupported constant type: {:?}", kind),
    })
}

pub fn anonymous_field_ident(index: usize) -> Ident {
    format_ident!("__field{index}")
}
