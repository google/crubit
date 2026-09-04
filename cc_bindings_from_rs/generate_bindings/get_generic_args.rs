// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::{liberate_and_deanonymize_late_bound_regions, matches_qualified_name};
use arc_anyhow::{anyhow, bail, ensure, Result};
use database::BindingsGenerator;
use rustc_infer::infer::{InferCtxt, RegionVariableOrigin};
use rustc_infer::traits::{Obligation, ObligationCause};
#[cfg_accessible(rustc_middle::ty::TraitClause)]
use rustc_middle::ty::TraitClause;
#[cfg_accessible(rustc_middle::ty::TraitPredicate)]
use rustc_middle::ty::TraitPredicate as TraitClause;
use rustc_middle::ty::{self, Ty, TyCtxt, TypeVisitableExt};

#[cfg_accessible(rustc_middle::ty::ClausePolarity)]
use rustc_middle::ty::ClausePolarity;
#[cfg_accessible(rustc_middle::ty::PredicatePolarity)]
use rustc_middle::ty::PredicatePolarity as ClausePolarity;

use rustc_span::def_id::DefId;
use rustc_span::symbol::{sym, Symbol};
use rustc_trait_selection::infer::canonical::ir::TypingMode;
use rustc_trait_selection::infer::TyCtxtInferExt;
use rustc_trait_selection::traits::ObligationCtxt;
use std::collections::{HashMap, HashSet};

#[cfg_accessible(rustc_middle::ty::GenericPredicates)]
type GenericClauses<'tcx> = ty::GenericPredicates<'tcx>;

#[cfg_accessible(rustc_middle::ty::GenericClauses)]
type GenericClauses<'tcx> = ty::GenericClauses<'tcx>;

trait GenericClausesExt<'tcx> {
    fn clauses(&self) -> &[(ty::Clause<'tcx>, rustc_span::Span)];
}

#[cfg_accessible(rustc_middle::ty::GenericPredicates)]
impl<'tcx> GenericClausesExt<'tcx> for ty::GenericPredicates<'tcx> {
    fn clauses(&self) -> &[(ty::Clause<'tcx>, rustc_span::Span)] {
        self.predicates
    }
}

#[cfg_accessible(rustc_middle::ty::GenericClauses)]
impl<'tcx> GenericClausesExt<'tcx> for ty::GenericClauses<'tcx> {
    fn clauses(&self) -> &[(ty::Clause<'tcx>, rustc_span::Span)] {
        self.clauses
    }
}

/// Implementation of `BindingsGenerator::get_generic_args`.
pub fn get_generic_args<'tcx>(
    db: &BindingsGenerator<'tcx>,
    fn_def_id: DefId,
) -> Result<ty::GenericArgsRef<'tcx>> {
    let tcx = db.tcx();
    let generics = tcx.generics_of(fn_def_id);
    #[cfg_accessible(rustc_middle::ty::GenericPredicates)]
    let predicates = tcx.predicates_of(fn_def_id);
    #[cfg_accessible(rustc_middle::ty::GenericClauses)]
    let predicates = tcx.clauses_of(fn_def_id);

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
                    bail!("crubit.rs/errors/unsupported_type: `const`-generic functions are not supported (b/259749023)");
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

fn make_dyn_fn_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    closure_kind: ty::ClosureKind,
    bound_vars: &'tcx ty::List<ty::BoundVariableKind<'tcx>>,
    args_tuple: Ty<'tcx>,
    ret_ty: Ty<'tcx>,
    region: ty::Region<'tcx>,
) -> Ty<'tcx> {
    let lang_items = tcx.lang_items();
    let trait_def_id = match closure_kind {
        ty::ClosureKind::Fn => lang_items.fn_trait().expect("Fn trait missing"),
        ty::ClosureKind::FnMut => lang_items.fn_mut_trait().expect("FnMut trait missing"),
        ty::ClosureKind::FnOnce => lang_items.fn_once_trait().expect("FnOnce trait missing"),
    };
    let trait_ref = ty::TraitRef::new(tcx, trait_def_id, [Ty::new_tup(tcx, &[]), args_tuple]);
    let existential_trait_ref = ty::ExistentialTraitRef::erase_self_ty(tcx, trait_ref);
    let existential_trait_pred = ty::ExistentialPredicate::Trait(existential_trait_ref);

    let fn_once_def_id = lang_items.fn_once_trait().expect("FnOnce trait missing");
    let output_def_id = tcx
        .associated_items(fn_once_def_id)
        .in_definition_order()
        .find(|item| item.name() == sym::Output && matches!(item.kind, ty::AssocKind::Type { .. }))
        .expect("Output missing")
        .def_id;
    let projection = ty::ExistentialProjection::new_from_args(
        tcx,
        output_def_id,
        tcx.mk_args(&[args_tuple.into()]),
        ret_ty.into(),
    );
    let existential_proj_pred = ty::ExistentialPredicate::Projection(projection);
    let preds = tcx.mk_poly_existential_predicates(&[
        ty::Binder::bind_with_vars(existential_trait_pred, bound_vars),
        ty::Binder::bind_with_vars(existential_proj_pred, bound_vars),
    ]);
    Ty::new_dynamic(tcx, preds, region)
}

/// Returns concrete type replacements for generic parameters bounded by `Fn`, `FnMut`, or `FnOnce`.
///
/// This function returns multiple replacement types to support different caller use cases in C++:
/// 1. A non-owning borrowed reference (`&dyn Fn` or `&mut dyn FnMut`), which maps to `rs::FnRef` in
///    C++. This is preferred for performance as it avoids heap allocations, and is included when
///    the type parameter does not have an explicit outlives bound (such as `'static`).
/// 2. An owning `Box<dyn Fn/FnMut/FnOnce>`, which maps to `rs::Fn` in C++. This satisfies `'static`
///    and outlives bounds, allowing callers to pass callables that outlive the call or are stored.
///
/// Generating both replacements allows Crubit to produce C++ overloads for both transient borrowed
/// callable references (`rs::FnRef`) and heap-allocated owning closures (`rs::Fn`).
fn get_replacements_for_fn_trait<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_ref: ty::TraitRef<'tcx>,
    closure_kind: ty::ClosureKind,
    bound_vars: &'tcx ty::List<ty::BoundVariableKind<'tcx>>,
    predicates: GenericClauses<'tcx>,
    new_anon_lifetime: &dyn Fn() -> ty::Region<'tcx>,
    is_used_in_return_type: bool,
) -> Vec<Ty<'tcx>> {
    if is_used_in_return_type {
        return vec![];
    }
    let poly_trait_ref = ty::Binder::bind_with_vars(trait_ref, bound_vars);
    if poly_trait_ref.has_escaping_bound_vars() {
        return vec![];
    }
    let Some(args_tuple) = trait_ref.args.get(1).and_then(|a| a.as_type()) else {
        return vec![];
    };
    if !matches!(args_tuple.kind(), ty::TyKind::Tuple(_)) {
        return vec![];
    }

    let Some(fn_once_trait_def_id) = tcx.lang_items().fn_once_trait() else {
        return vec![];
    };
    let Some(output_def_id) = tcx
        .associated_items(fn_once_trait_def_id)
        .in_definition_order()
        .find(|item| item.name() == sym::Output && matches!(item.kind, ty::AssocKind::Type { .. }))
        .map(|item| item.def_id)
    else {
        return vec![];
    };

    let output_ty = predicates
        .clauses()
        .iter()
        .filter_map(|(clause, _)| {
            if let ty::ClauseKind::Projection(projection_predicate) = clause.kind().skip_binder() {
                let is_match = projection_predicate.def_id() == output_def_id;
                if is_match && projection_predicate.self_ty() == trait_ref.self_ty() {
                    return projection_predicate.term.as_type();
                }
            }
            None
        })
        .next()
        .unwrap_or_else(|| Ty::new_tup(tcx, &[]));

    let poly_output_ty = ty::Binder::bind_with_vars(output_ty, bound_vars);
    if poly_output_ty.has_escaping_bound_vars() {
        return vec![];
    }

    let mut replacements = Vec::new();

    let has_outlives_bound = predicates.clauses().iter().any(|(clause, _)| {
        if let ty::ClauseKind::TypeOutlives(outlives) = clause.kind().skip_binder() {
            outlives.0 == trait_ref.self_ty()
        } else {
            false
        }
    });

    if !has_outlives_bound {
        // 1. Non-owning reference replacement (if valid)
        let anon_lifetime = new_anon_lifetime();
        match closure_kind {
            ty::ClosureKind::Fn => {
                let dyn_ty = make_dyn_fn_ty(
                    tcx,
                    ty::ClosureKind::Fn,
                    bound_vars,
                    args_tuple,
                    output_ty,
                    anon_lifetime,
                );
                replacements.push(Ty::new_imm_ref(tcx, anon_lifetime, dyn_ty));
            }
            ty::ClosureKind::FnMut | ty::ClosureKind::FnOnce => {
                // `&mut dyn FnMut` implements both `FnMut` and `FnOnce`
                let dyn_ty = make_dyn_fn_ty(
                    tcx,
                    ty::ClosureKind::FnMut,
                    bound_vars,
                    args_tuple,
                    output_ty,
                    anon_lifetime,
                );
                replacements.push(Ty::new_mut_ref(tcx, anon_lifetime, dyn_ty));
            }
        }
    }

    // 2. Owning Box replacement (satisfies 'static, requires `alloc::boxed::Box`)
    if tcx.lang_items().owned_box().is_some() {
        let static_region = tcx.lifetimes.re_static;
        let dyn_ty_static =
            make_dyn_fn_ty(tcx, closure_kind, bound_vars, args_tuple, output_ty, static_region);
        replacements.push(Ty::new_box(tcx, dyn_ty_static));
    }

    replacements
}

/// Given a generic constraint of the form `T: Trait`, returns the types that can potentially
/// replace `T` in the generated bindings.
///
/// If the returned type needs to use a new anonymous lifetime, then it will be generated
/// using the given `def_id` as its scope.
fn get_replacements_for_trait_predicate<'tcx>(
    db: &BindingsGenerator<'tcx>,
    trait_predicate: TraitClause<'tcx>,
    bound_vars: &'tcx ty::List<ty::BoundVariableKind<'tcx>>,
    predicates: GenericClauses<'tcx>,
    new_anon_lifetime: impl Fn() -> ty::Region<'tcx>,
    is_used_in_return_type: bool,
) -> Vec<Ty<'tcx>> {
    let tcx = db.tcx();
    if trait_predicate.polarity != ClausePolarity::Positive {
        return vec![];
    }
    let trait_ref = trait_predicate.trait_ref;

    if let Some(closure_kind) = tcx.fn_trait_kind_from_def_id(trait_ref.def_id) {
        return get_replacements_for_fn_trait(
            tcx,
            trait_ref,
            closure_kind,
            bound_vars,
            predicates,
            &new_anon_lifetime,
            is_used_in_return_type,
        );
    }

    if !bound_vars.is_empty() {
        return vec![];
    }

    // `args[0]` is `Self` / `T`.  And when working with `Into<U>`, `AsRef<U>`, etc.
    // we typically want the first and only other generic argument - `U`.
    let ty1 = trait_ref.args.get(1).and_then(|generic_arg| generic_arg.as_type());

    // `T: Into<U>` => `U`
    if tcx.is_diagnostic_item(sym::Into, trait_ref.def_id) {
        return ty1.into_iter().collect();
    }

    // `T: AsRef<U>` => `&U`
    if tcx.is_diagnostic_item(sym::AsRef, trait_ref.def_id) {
        return ty1.map(|t| Ty::new_imm_ref(tcx, new_anon_lifetime(), t)).into_iter().collect();
    }

    // `T: AsMut<U>` => `&mut U`
    if tcx.is_diagnostic_item(sym::AsMut, trait_ref.def_id) {
        return ty1.map(|t| Ty::new_mut_ref(tcx, new_anon_lifetime(), t)).into_iter().collect();
    }

    // Support for Ctor trait (b/489315162)
    if matches_qualified_name(db, trait_ref.def_id, &["ctor", "Ctor"]) {
        return get_replacement_for_ctor_trait(
            db,
            trait_ref,
            predicates,
            &new_anon_lifetime,
            is_used_in_return_type,
        )
        .into_iter()
        .collect();
    }

    // TODO(b/281542952): Implement other replacements as needed.
    vec![]
}

fn get_replacement_for_ctor_trait<'tcx>(
    db: &BindingsGenerator<'tcx>,
    trait_ref: ty::TraitRef<'tcx>,
    predicates: GenericClauses<'tcx>,
    new_anon_lifetime: &dyn Fn() -> ty::Region<'tcx>,
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
        .clauses()
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
    predicates: GenericClauses<'tcx>,
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

    let ocx = ObligationCtxt::new(infcx);
    let param_env = tcx.param_env(def_id);
    for (predicate, _span) in predicates.instantiate(tcx, generic_args) {
        let cause = ObligationCause::dummy();
        let predicate = ocx.normalize(&cause, param_env, predicate);
        ocx.register_obligation(Obligation::new(tcx, cause, param_env, predicate));
    }
    let errors = ocx.evaluate_obligations_error_on_ambiguity();
    errors.into_iter().next().is_none()
}

/// Given a `generic_type_param` (e.g. `T` in `fn foo<T>(...)`) tries to find
/// a non-generic type which can be used instead.  For example, `T: Into<U>` may
/// be potentially replaced with `U`, if `U` meets all the other `predicates`
/// that may be constraining `T`.  When multiple answers are possible, returns
/// the first one.
fn get_replacement_for_generic_type_param<'tcx>(
    db: &BindingsGenerator<'tcx>,
    def_id: DefId,
    predicates: GenericClauses<'tcx>,
    generic_type_param: &ty::GenericParamDef,
    is_used_in_return_type: bool,
) -> Option<Ty<'tcx>> {
    let tcx = db.tcx();
    // Look only at trait predicates involving this param (e.g. `T: SomeTrait`).
    let trait_predicates_for_this_generic_param = predicates
        .clauses()
        .iter()
        .filter_map(|(clause, _)| match clause.kind().skip_binder() {
            ty::ClauseKind::Trait(trait_predicate) => {
                Some((trait_predicate, clause.kind().bound_vars()))
            }
            _ => None,
        })
        .filter(|(trait_predicate, _)| match trait_predicate.trait_ref.self_ty().kind() {
            ty::Param(p) => p.index == generic_type_param.index,
            _ => false,
        });

    let infcx = tcx.infer_ctxt().build(TypingMode::non_body_analysis());
    let new_anon_lifetime =
        || infcx.next_region_var(RegionVariableOrigin::Coercion(tcx.def_span(def_id)));

    // Find the first replacement that fits all the constraints.
    trait_predicates_for_this_generic_param
        .flat_map(|(trait_predicate, bound_vars)| {
            get_replacements_for_trait_predicate(
                db,
                trait_predicate,
                bound_vars,
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
