// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::{liberate_and_deanonymize_late_bound_regions, matches_qualified_name};
use arc_anyhow::{anyhow, bail, ensure, Result};
use database::BindingsGenerator;
use rustc_infer::infer::{InferCtxt, RegionVariableOrigin};
use rustc_infer::traits::{Obligation, ObligationCause};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::DefId;
use rustc_span::symbol::{sym, Symbol};
use rustc_trait_selection::infer::canonical::ir::TypingMode;
use rustc_trait_selection::infer::TyCtxtInferExt;
use rustc_trait_selection::traits::ObligationCtxt;
use std::collections::{HashMap, HashSet};

/// Implementation of `BindingsGenerator::get_generic_args`.
pub fn get_generic_args<'tcx>(
    db: &BindingsGenerator<'tcx>,
    fn_def_id: DefId,
) -> Result<ty::GenericArgsRef<'tcx>> {
    let tcx = db.tcx();
    let generics = tcx.generics_of(fn_def_id);
    let predicates = tcx.predicates_of(fn_def_id);

    // See the doc comment for `unused_generic_param` in
    // `test/functions/functions.rs` for an explanation why we currently don't
    // support unused generic params.
    let indices_of_actually_used_generic_params = {
        let mut finder = GenericParamsFinder::default();
        let fn_sig = tcx.fn_sig(fn_def_id).instantiate_identity();
        let fn_sig = liberate_and_deanonymize_late_bound_regions(tcx, fn_sig, fn_def_id);
        use rustc_type_ir::TypeVisitable;
        fn_sig.visit_with(&mut finder);
        finder.generic_param_indices
    };

    let params_used_in_return_type = {
        let mut finder = GenericParamsFinder::default();
        let fn_sig = tcx.fn_sig(fn_def_id).instantiate_identity();
        let fn_sig = liberate_and_deanonymize_late_bound_regions(tcx, fn_sig, fn_def_id);
        use rustc_type_ir::TypeVisitable;
        fn_sig.output().visit_with(&mut finder);
        finder.generic_param_indices
    };

    let replacements: HashMap<usize, ty::GenericArg<'tcx>> = (0..generics.count())
        .map(|idx| {
            let param_def = generics.param_at(idx, tcx);
            let replacement = match param_def.kind {
                ty::GenericParamDefKind::Const { .. } => {
                    bail!("`const`-generic functions are not supported (b/259749023)");
                }
                ty::GenericParamDefKind::Lifetime => tcx.mk_param_from_def(param_def),
                ty::GenericParamDefKind::Type { .. } => {
                    ensure!(
                        indices_of_actually_used_generic_params.contains(&param_def.index),
                        "No support for replacing an _unused_ generic type param: `{}`",
                        param_def.name,
                    );
                    get_replacement_for_generic_type_param(
                        db,
                        fn_def_id,
                        predicates,
                        param_def,
                        params_used_in_return_type.contains(&param_def.index),
                    )
                    .map(|ty| ty.into())
                    .ok_or_else(|| {
                        anyhow!(
                            "No valid non-generic replacement for generic type param `{}`",
                            param_def.name,
                        )
                    })?
                }
            };
            Ok((idx, replacement))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .collect();

    Ok(ty::GenericArgs::for_item(tcx, fn_def_id, |param_def, _old_generic_args| {
        *replacements
            .get(&(param_def.index as usize))
            .expect("All errors should have been handled above")
    }))
}

/// Given a generic constraint of the form `T: Trait`, returns the type that can potentially
/// replace `T` in the generated bindings.
///
/// If the returned type needs to use a new anonymous lifetime, then it will be generated
/// using the given `def_id` as its scope.
fn get_replacement_for_trait_predicate<'tcx>(
    db: &BindingsGenerator<'tcx>,
    trait_predicate: ty::TraitPredicate<'tcx>,
    predicates: ty::GenericPredicates<'tcx>,
    new_anon_lifetime: impl Fn() -> ty::Region<'tcx>,
    is_used_in_return_type: bool,
) -> Option<Ty<'tcx>> {
    let tcx = db.tcx();
    if trait_predicate.polarity != ty::PredicatePolarity::Positive {
        return None;
    }
    let trait_ref = trait_predicate.trait_ref;

    // `args[0]` is `Self` / `T`.  And when working with `Into<U>`, `AsRef<U>`, etc.
    // we typically want the first and only other generic argument - `U`.
    let ty1 = trait_ref.args.get(1).and_then(|generic_arg| generic_arg.as_type());

    // `T: Into<U>` => `U`
    if tcx.is_diagnostic_item(sym::Into, trait_ref.def_id) {
        return ty1;
    }

    // `T: AsRef<U>` => `&U`
    if tcx.is_diagnostic_item(sym::AsRef, trait_ref.def_id) {
        return Some(Ty::new_imm_ref(tcx, new_anon_lifetime(), ty1?));
    }

    // `T: AsMut<U>` => `&mut U`
    if tcx.is_diagnostic_item(sym::AsMut, trait_ref.def_id) {
        return Some(Ty::new_mut_ref(tcx, new_anon_lifetime(), ty1?));
    }

    // Support for Ctor trait (b/489315162)
    if matches_qualified_name(db, trait_ref.def_id, &["ctor", "Ctor"]) {
        return get_replacement_for_ctor_trait(
            db,
            trait_ref,
            predicates,
            &new_anon_lifetime,
            is_used_in_return_type,
        );
    }

    // TODO(b/281542952): Implement other replacements as needed.
    None
}

fn get_replacement_for_ctor_trait<'tcx>(
    db: &BindingsGenerator<'tcx>,
    trait_ref: ty::TraitRef<'tcx>,
    predicates: ty::GenericPredicates<'tcx>,
    new_anon_lifetime: &impl Fn() -> ty::Region<'tcx>,
    is_used_in_return_type: bool,
) -> Option<Ty<'tcx>> {
    let tcx = db.tcx();
    if is_used_in_return_type {
        // TODO(b/489315162): Support Ctor in return types. We should be able to turn `fn foo() -> impl Ctor<Output = T>` into `T foo();` by using C++ guaranteed copy elision.
        return None;
    }
    // 1. Find the `DefId` of the `Output` associated type in the `Ctor` trait.
    let ctor_output_def_id = tcx
        .associated_items(trait_ref.def_id)
        .in_definition_order()
        .find(|item| item.name() == sym::Output && matches!(item.kind, ty::AssocKind::Type { .. }))
        .map(|item| item.def_id)?;

    // 2. Iterate over the predicates and look for projections.
    let output_ty = predicates
        .predicates
        .iter()
        .filter_map(|(clause, _)| {
            if let ty::ClauseKind::Projection(projection_predicate) = clause.kind().skip_binder() {
                let is_match = projection_predicate.def_id() == ctor_output_def_id;
                if is_match && projection_predicate.self_ty() == trait_ref.self_ty() {
                    return projection_predicate.term.as_type();
                }
            }
            None
        })
        .next()?;

    // 3. Find DefId for ByValue
    let ctor_crate = trait_ref.def_id.krate;
    let by_value_def_id = db.def_id_by_symbol(ctor_crate, Symbol::intern("ByValue"))?;

    // 4. Construct ByValue<'a, Output>
    let by_value_ty = Ty::new_adt(
        tcx,
        tcx.adt_def(by_value_def_id),
        tcx.mk_args(&[new_anon_lifetime().into(), output_ty.into()]),
    );

    // Verification that `Output` can be constructed from `ByValue<'_, Output>`
    // (i.e., `Output: CtorNew<ByValue<'_, Output>>`) is deferred to the caller.
    // The caller (`get_replacement_for_generic_type_param`) will check if
    // `ByValue<'_, Output>: Ctor` holds using the Rust trait solver.
    // Due to how `Ctor` is implemented for `ByValue` in `ctor.rs`,
    // this implicitly verifies the `CtorNew` constraint.

    Some(by_value_ty)
}

/// Returns `true` if `new_ty` can be used as a replacement for `generic_param`
/// in a generic item identified by `def_id` and constrained by the given `predicates`.
fn is_valid_replacement_for_generic_type_param<'tcx>(
    infcx: &InferCtxt<'tcx>,
    def_id: DefId,
    predicates: ty::GenericPredicates<'tcx>,
    generic_param: &ty::GenericParamDef,
    new_ty: Ty<'tcx>,
) -> bool {
    let tcx = infcx.tcx;
    let generic_args = ty::GenericArgs::for_item(tcx, def_id, |param_def, _old_generic_args| {
        if param_def.index == generic_param.index {
            new_ty.into()
        } else {
            tcx.mk_param_from_def(param_def)
        }
    });

    let ocx = ObligationCtxt::new(&infcx);
    let param_env = tcx.param_env(def_id);
    for (predicate, _span) in predicates.instantiate(tcx, generic_args) {
        let cause = ObligationCause::dummy();
        let predicate = ocx.normalize(&cause, param_env, predicate);
        ocx.register_obligation(Obligation::new(tcx, cause, param_env, predicate));
    }
    let errors = ocx.evaluate_obligations_error_on_ambiguity();
    errors.is_empty()
}

/// Given a `generic_type_param` (e.g. `T` in `fn foo<T>(...)`) tries to find
/// a non-generic type which can be used instead.  For example, `T: Into<U>` may
/// be potentially replaced with `U`, if `U` meets all the other `predicates`
/// that may be constraining `T`.  When multiple answers are possible, returns
/// the first one.
fn get_replacement_for_generic_type_param<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
    predicates: ty::GenericPredicates<'tcx>,
    generic_type_param: &ty::GenericParamDef,
    is_used_in_return_type: bool,
) -> Option<Ty<'tcx>> {
    let tcx = db.tcx();
    // Look only at trait predicates involving this param (e.g. `T: SomeTrait`).
    let trait_predicates_for_this_generic_param = predicates
        .predicates
        .iter()
        .filter_map(|(clause, _)| match clause.kind().skip_binder() {
            ty::ClauseKind::Trait(trait_predicate) => Some(trait_predicate),
            _ => None,
        })
        .filter(|trait_predicate| match trait_predicate.trait_ref.self_ty().kind() {
            ty::Param(p) => p.index == generic_type_param.index,
            _ => false,
        });

    let infcx = tcx.infer_ctxt().build(TypingMode::non_body_analysis());
    let new_anon_lifetime =
        || infcx.next_region_var(RegionVariableOrigin::Coercion(tcx.def_span(def_id)));

    // Find the first replacement that fits all the constraints.
    trait_predicates_for_this_generic_param
        .filter_map(|trait_predicate| {
            get_replacement_for_trait_predicate(
                db,
                trait_predicate,
                predicates,
                new_anon_lifetime,
                is_used_in_return_type,
            )
        })
        .find(|new_ty| {
            // Verify that the candidate replacement satisfies all predicates.
            // For example, when replacing `T: Ctor` with `RvalueReference<'static, Output>`,
            // checking `RvalueReference<'static, Output>: Ctor` implicitly verifies that
            // `Output` can be constructed from it (see `get_replacement_for_ctor_trait`).
            is_valid_replacement_for_generic_type_param(
                &infcx,
                def_id,
                predicates,
                generic_type_param,
                *new_ty,
            )
        })
}

#[derive(Default)]
struct GenericParamsFinder {
    generic_param_indices: HashSet<u32>,
}

impl<'tcx> ty::TypeVisitor<TyCtxt<'tcx>> for GenericParamsFinder {
    fn visit_ty(&mut self, t: Ty<'tcx>) {
        if let ty::Param(p) = t.kind() {
            self.generic_param_indices.insert(p.index);
        }

        // Visit nested types (e.g., `&T` or `&[T]`)
        use ty::TypeSuperVisitable;
        t.super_visit_with(self)
    }
}
