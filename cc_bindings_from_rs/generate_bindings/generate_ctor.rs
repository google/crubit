// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! C++ constructors and assignment operators for Rust ADTs: the default, copy and move
//! constructors (and the matching assignment operators) that let a Rust type be used as a C++
//! value, together with the `BindingsGenerator` queries that decide which of them an ADT gets.

use crate::generate_function_thunk::{
    generate_ctor_thunk_call, generate_trait_thunks, TraitThunks,
};
use crate::thunk_qualifier;
use arc_anyhow::Result;
use code_gen_utils::CcInclude;
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet, CcSnippets, RsSnippet};
use database::{AdtCoreBindings, BindingsGenerator, CopyCodegenStyle, MoveCodegenStyle};
use error_report::{anyhow, bail};
use itertools::Itertools;
use proc_macro2::TokenStream;
use query_compiler::{does_type_implement_trait, is_copy, is_std_ptr_non_null};
use quote::quote;
use rustc_middle::ty::{self, Ty};
use rustc_span::symbol::sym;

pub(crate) fn has_default_ctor<'tcx>(db: &BindingsGenerator<'tcx>, self_ty: Ty<'tcx>) -> bool {
    let tcx = db.tcx();
    let trait_id = tcx
        .get_diagnostic_item(sym::Default)
        .expect("Couldn't find lang item `core::default::Default`");
    does_type_implement_trait(tcx, self_ty, trait_id, [])
}

/// Generates a default constructor for an ADT if possible (i.e. if the `Default`
/// trait is implemented for the ADT), or a `= delete`d default constructor otherwise.
pub(crate) fn generate_default_ctor<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
) -> ApiSnippets<'tcx> {
    fn fallible_format_default_ctor<'tcx>(
        db: &BindingsGenerator<'tcx>,
        core: &AdtCoreBindings<'tcx>,
    ) -> Result<ApiSnippets<'tcx>> {
        let tcx = db.tcx();
        let trait_id = tcx
            .get_diagnostic_item(sym::Default)
            .ok_or(anyhow!("Couldn't find `core::default::Default`"))?;
        let TraitThunks {
            method_name_to_cc_thunk_name,
            cc_thunk_decls,
            rs_thunk_impls: rs_details,
        } = generate_trait_thunks(
            db,
            trait_id,
            &[],
            core.common.self_ty,
            core.def_id,
            core.rs_fully_qualified_name.clone(),
            /* is_constructor= */ true,
            /* within_template= */ false,
        )?;

        let cc_struct_name = &core.common.cc_short_name;
        let main_api = CcSnippet::new(quote! {
            __NEWLINE__ __COMMENT__ "Default::default"
            #cc_struct_name(); __NEWLINE__ __NEWLINE__
        });
        let cc_details = {
            let thunk_name = method_name_to_cc_thunk_name
                .into_values()
                .exactly_one()
                .expect("Expecting a single `default` method");

            let fully_qualified_name = &core.common.cc_fully_qualified_name;
            let ctor_body = generate_ctor_thunk_call(db, core, &thunk_name, &[])?;
            let ctor_impl = quote! {
                inline #fully_qualified_name::#cc_struct_name() {
                    #ctor_body
                }
            };
            cc_thunk_decls.map_snippets(|cc_thunk_decls| {
                quote! {
                    #cc_thunk_decls
                    #ctor_impl
                }
            })
        };
        Ok(ApiSnippets { main_api, cc_details, rs_details })
    }
    fallible_format_default_ctor(db, core).unwrap_or_else(|err| {
        generate_deleted_default_ctor(&core.common.cc_short_name, &format!("{err:#}"))
    })
}

fn generate_deleted_default_ctor<'tcx>(
    adt_cc_name: &TokenStream,
    doc_comment: &str,
) -> ApiSnippets<'tcx> {
    ApiSnippets {
        main_api: CcSnippet::new(quote! {
            __NEWLINE__ __COMMENT__ #doc_comment
            #adt_cc_name() = delete; __NEWLINE__
        }),
        ..Default::default()
    }
}

/// If we were to generate a C++ copy constructor and assignment operator for a Rust-originating
/// type, how would we do it?
pub(crate) fn copy_ctor_and_assignment_operator_codegen_style<'tcx>(
    db: &BindingsGenerator<'tcx>,
    self_ty: Ty<'tcx>,
) -> Option<CopyCodegenStyle> {
    if db.is_proto_message(self_ty) {
        return None;
    }
    let tcx = db.tcx();
    let clone_trait_id = tcx.lang_items().clone_trait().expect("Can't find the `Clone` trait");
    CopyCodegenStyle::from_available_traits(
        is_copy(tcx, self_ty),
        does_type_implement_trait(tcx, self_ty, clone_trait_id, []),
    )
}

/// Generates the copy constructor and the copy-assignment operator for an ADT if
/// possible (i.e. if the `Clone` trait is implemented for the ADT), or `= delete`d
/// declarations otherwise.
pub(crate) fn generate_copy_ctor_and_assignment_operator<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
) -> ApiSnippets<'tcx> {
    fn fallible_format_copy_codegen_style<'tcx>(
        db: &BindingsGenerator<'tcx>,
        core: &AdtCoreBindings<'tcx>,
        copy_codegen_style: Option<CopyCodegenStyle>,
    ) -> Result<ApiSnippets<'tcx>> {
        let tcx = db.tcx();
        let cc_struct_name = &core.common.cc_short_name;
        let qualified_adt_name = &core.common.cc_fully_qualified_name;
        match copy_codegen_style {
            Some(CopyCodegenStyle::Copy) => {
                let msg = "Rust types that are `Copy` get trivial, `default` C++ copy constructor \
                        and assignment operator.";
                let main_api = CcSnippet::new(quote! {
                    __NEWLINE__ __COMMENT__ #msg
                    #cc_struct_name(const #cc_struct_name&) = default;  __NEWLINE__
                    #cc_struct_name& operator=(const #cc_struct_name&) = default;
                });
                let cc_details = CcSnippet::with_include(
                    quote! {
                        static_assert(::std::is_trivially_copy_constructible_v<#qualified_adt_name>);
                        static_assert(::std::is_trivially_copy_assignable_v<#qualified_adt_name>);
                    },
                    CcInclude::type_traits(),
                );

                Ok(ApiSnippets { main_api, cc_details, rs_details: RsSnippet::default() })
            }
            Some(CopyCodegenStyle::Clone) => {
                let trait_id = tcx
                    .lang_items()
                    .clone_trait()
                    .ok_or_else(|| anyhow!("Can't find the `Clone` trait"))?;
                let TraitThunks {
                    method_name_to_cc_thunk_name,
                    cc_thunk_decls,
                    rs_thunk_impls: rs_details,
                } = generate_trait_thunks(
                    db,
                    trait_id,
                    &[],
                    core.common.self_ty,
                    core.def_id,
                    core.rs_fully_qualified_name.clone(),
                    /* is_constructor= */ true,
                    /* within_template= */ false,
                )?;
                let main_api = CcSnippet::new(quote! {
                    __NEWLINE__ __COMMENT__ "Clone::clone"
                    #cc_struct_name(const #cc_struct_name&) noexcept; __NEWLINE__
                    __NEWLINE__ __COMMENT__ "Clone::clone_from"
                    #qualified_adt_name& operator=(const #cc_struct_name&) noexcept; __NEWLINE__ __NEWLINE__
                });
                let cc_details = {
                    // `unwrap` calls are okay because `Clone` trait always has these methods.
                    let clone_thunk_name = method_name_to_cc_thunk_name.get(&sym::clone).unwrap();
                    let clone_from_thunk_name =
                        method_name_to_cc_thunk_name.get(&sym::clone_from).unwrap();

                    let thunk_qualifier = thunk_qualifier(core);
                    let ctor_body =
                        generate_ctor_thunk_call(db, core, clone_thunk_name, &[quote! { other }])?;
                    cc_thunk_decls.map_snippets(|cc_thunk_decls| {
                        quote! {
                            #cc_thunk_decls
                            inline #qualified_adt_name::#cc_struct_name(const #cc_struct_name& other) noexcept {
                                #ctor_body
                            }
                            inline #qualified_adt_name& #qualified_adt_name::operator=(const #cc_struct_name& other) noexcept {
                                if (this != &other) {
                                    #thunk_qualifier::#clone_from_thunk_name(*this, other);
                                }
                                return *this;
                            }
                        }
                    })
                };
                Ok(ApiSnippets { main_api, cc_details, rs_details })
            }
            None => {
                let display_name = core
                    .def_id
                    .and_then(|id| db.symbol_canonical_name(id).ok())
                    .map(|canon| {
                        let parts =
                            canon.rs_name_parts().map(|s| format!("{}", s)).collect::<Vec<_>>();
                        parts.join("::")
                    })
                    .unwrap_or_else(|| format!("{}", core.common.self_ty));
                bail!("`{display_name}` doesn't implement the `Clone` trait");
            }
        }
    }
    let copy_codegen_style =
        db.copy_ctor_and_assignment_operator_codegen_style(core.common.self_ty);
    fallible_format_copy_codegen_style(db, core, copy_codegen_style).unwrap_or_else(|err| {
        let msg = format!("{err:#}");
        let adt_cc_name = &core.common.cc_short_name;
        ApiSnippets {
            main_api: CcSnippet::new(quote! {
                __NEWLINE__ __COMMENT__ #msg
                #adt_cc_name(const #adt_cc_name&) = delete;  __NEWLINE__
                #adt_cc_name& operator=(const #adt_cc_name&) = delete;
            }),
            ..Default::default()
        }
    })
}

/// If we were to generate a C++ move constructor and assignment operator for a Rust-originating
/// type, how would we do it?
pub(crate) fn move_ctor_and_assignment_operator_codegen_style<'tcx>(
    db: &BindingsGenerator<'tcx>,
    self_ty: Ty<'tcx>,
) -> Option<MoveCodegenStyle> {
    if db.is_proto_message(self_ty) {
        return None;
    }
    let tcx = db.tcx();
    let typing_env = ty::TypingEnv::fully_monomorphized();
    MoveCodegenStyle::from_available_traits(
        !self_ty.needs_drop(tcx, typing_env),
        db.has_default_ctor(self_ty),
        self_ty.is_unpin(tcx, typing_env),
        db.copy_ctor_and_assignment_operator_codegen_style(self_ty).is_some(),
    )
}

/// Returns true if the type is move-constructible in C++.
///
/// For imported C++ types (types annotated with `CRUBIT_ANNOTATE: cpp_type=...`),
/// this inspects `CRUBIT_ANNOTATE: cpp_move_constructible=` emitted by `rs_bindings_from_cc`.
///
/// For Rust types, this checks if Crubit can generate move operations for it via
/// `move_ctor_and_assignment_operator_codegen_style`.
pub(crate) fn is_cpp_move_constructible<'tcx>(db: &BindingsGenerator<'tcx>, ty: Ty<'tcx>) -> bool {
    if query_compiler::as_ref_or_pinned_ref(ty).is_some() {
        return true;
    }
    match *ty.kind() {
        // Primitives, references, and raw pointers are trivially movable in C++.
        ty::Bool
        | ty::Char
        | ty::Int(_)
        | ty::Uint(_)
        | ty::Float(_)
        | ty::Never
        | ty::RawPtr(..)
        | ty::Ref(..)
        | ty::FnPtr(..) => true,

        // ADT: check CrubitAttrs if C++-originated, else check
        // move_ctor_and_assignment_operator_codegen_style.
        ty::Adt(adt_def, substs) => {
            if is_std_ptr_non_null(db.tcx(), adt_def.did()) {
                return true;
            }
            if db.is_proto_message(ty) {
                return false;
            }
            if let Ok(attrs) = crubit_attr::get_attrs(db.tcx(), adt_def.did()) {
                if attrs.cpp_move_constructible {
                    return true;
                }
                if attrs.cpp_type.is_some() && substs.types().next().is_some() {
                    return substs.types().all(|t| db.is_cpp_move_constructible(t));
                }
            }

            db.move_ctor_and_assignment_operator_codegen_style(ty).is_some()
        }

        // Tuples: all elements must be move-constructible.
        ty::Tuple(fields) => fields.iter().all(|f| db.is_cpp_move_constructible(f)),

        // Arrays: element type must be move-constructible.
        ty::Array(elem_ty, _) => db.is_cpp_move_constructible(elem_ty),

        _ => false,
    }
}

/// Generates the move constructor and the move-assignment operator for an ADT if possible
/// (depending on `needs_drop`, `is_unpin`, and implementations of `Default` and/or `Clone` traits),
/// or `= delete`d declarations otherwise.
pub(crate) fn generate_move_ctor_and_assignment_operator<'tcx>(
    db: &BindingsGenerator<'tcx>,
    core: &AdtCoreBindings<'tcx>,
) -> ApiSnippets<'tcx> {
    let common = &core.common;
    let adt_cc_name = &common.cc_short_name;
    let qualified_adt_name = &common.cc_fully_qualified_name;
    match db.move_ctor_and_assignment_operator_codegen_style(common.self_ty) {
        // We rely on the copy constructor and assignment operator to handle the move
        // operations.
        Some(MoveCodegenStyle::Copy) => ApiSnippets::default(),
        None => {
            let msg = "C++ move operations are unavailable for this type. See \
                http://crubit.rs/rust/movable_types for an explanation of Rust types that are C++ \
                movable.";
            ApiSnippets {
                main_api: CcSnippet::new(quote! {
                    __NEWLINE__ __COMMENT__ #msg
                    #adt_cc_name(#adt_cc_name&&) = delete;  __NEWLINE__
                    #qualified_adt_name& operator=(#adt_cc_name&&) = delete;
                }),
                ..Default::default()
            }
        }
        Some(MoveCodegenStyle::MemSwap) => {
            let main_api = CcSnippet::new(quote! {
                #adt_cc_name(#adt_cc_name&&) noexcept; __NEWLINE__
                #qualified_adt_name& operator=(#adt_cc_name&&) noexcept; __NEWLINE__
            });
            let mut prereqs = CcPrerequisites::default();
            prereqs.includes.insert(db.support_header("internal/memswap.h"));
            prereqs.includes.insert(CcInclude::utility()); // for `std::move`
            let tokens = quote! {
                inline #qualified_adt_name::#adt_cc_name(#adt_cc_name&& other) noexcept
                        : #adt_cc_name() {
                    *this = ::std::move(other);
                }
                inline #qualified_adt_name& #qualified_adt_name::operator=(#adt_cc_name&& other) noexcept {
                    crubit::MemSwap(*this, other);
                    return *this;
                }
            };
            let cc_details = CcSnippet { tokens, prereqs };
            ApiSnippets { main_api, cc_details, ..Default::default() }
        }
        Some(MoveCodegenStyle::Default) => {
            let main_api = CcSnippet::new(quote! {
                // The generated bindings have to follow Rust move semantics:
                // * All Rust types are memcpy-movable (e.g. <internal link>/constructors.html says
                //   that "Every type must be ready for it to be blindly memcopied to somewhere
                //   else in memory")
                // * The only valid operation on a moved-from non-`Copy` Rust struct is to assign to
                //   it.
                //
                // The generated C++ bindings below match the required semantics because they:
                // * Generate trivial` C++ move constructor and move assignment operator. Per
                //   <internal link>/cpp/language/move_constructor#Trivial_move_constructor: "A trivial
                //   move constructor is a constructor that performs the same action as the trivial
                //   copy constructor, that is, makes a copy of the object representation as if by
                //   std::memmove."
                // * Generate trivial C++ destructor.
                //
                // In particular, note that the following C++ code and Rust code are exactly
                // equivalent (except that in Rust, reuse of `y` is forbidden at compile time,
                // whereas in C++, it's only prohibited by convention):
                // * C++, assumming trivial move constructor and trivial destructor:
                //   `auto x = std::move(y);`
                // * Rust, assumming non-`Copy`, no custom `Drop` or drop glue:
                //   `let x = y;`
                //
                // TODO(b/258251148): If the ADT provides a custom `Drop` impls or requires drop
                // glue, then extra care should be taken to ensure the C++ destructor can handle
                // the moved-from object in a way that meets Rust move semantics.  For example, the
                // generated C++ move constructor might need to assign `Default::default()` to the
                // moved-from object.
                #adt_cc_name(#adt_cc_name&&) = default; __NEWLINE__
                #adt_cc_name& operator=(#adt_cc_name&&) = default; __NEWLINE__
                __NEWLINE__
            });
            let cc_details = CcSnippet::with_include(
                quote! {
                    static_assert(::std::is_trivially_move_constructible_v<#qualified_adt_name>);
                    static_assert(::std::is_trivially_move_assignable_v<#qualified_adt_name>);
                },
                CcInclude::type_traits(),
            );
            ApiSnippets { main_api, cc_details, ..Default::default() }
        }
    }
}
