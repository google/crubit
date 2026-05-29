// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]
extern crate rustc_middle;

use generate_bindings::avoid_colliding_types::{AvoidCollidingTypes, TypeCollisionRisk};
use run_compiler_test_support::run_compiler_for_testing;
use rustc_middle::ty::{Ty, TyCtxt};

fn filter_colliding_types<'tcx>(
    tcx: TyCtxt<'tcx>,
    input: impl IntoIterator<Item = Ty<'tcx>>,
) -> Vec<Ty<'tcx>> {
    input
        .into_iter()
        .avoid_colliding_types(tcx, |ty| *ty)
        .into_iter()
        .filter_map(Result::ok)
        .collect()
}

#[test]
fn test_empty() {
    run_compiler_for_testing("", |tcx| {
        let input: [Ty<'_>; 0] = [];
        let expected_output = input.to_vec();
        let actual_output = filter_colliding_types(tcx, input);
        assert_eq!(actual_output, expected_output);
    });
}
#[test]
fn test_single_item() {
    run_compiler_for_testing("", |tcx| {
        let usize_ty = tcx.types.usize;
        let input = [usize_ty];
        let expected_output = input.to_vec();
        let actual_output = filter_colliding_types(tcx, input);
        assert_eq!(actual_output, expected_output);
    });
}

#[test]
fn test_bool_and_char() {
    run_compiler_for_testing("", |tcx| {
        let char_ty = tcx.types.char;
        let bool_ty = tcx.types.bool;
        let input = [char_ty, bool_ty];
        let expected_output = [char_ty, bool_ty].to_vec();
        let actual_output = filter_colliding_types(tcx, input);
        assert_eq!(actual_output, expected_output);
    });
}

#[test]
fn test_no_preferred_type_is_present() {
    run_compiler_for_testing("", |tcx| {
        let input = [tcx.types.i32, tcx.types.i64, tcx.types.u32, tcx.types.u64];
        let expected_output = [tcx.types.i32, tcx.types.i64, tcx.types.u32, tcx.types.u64].to_vec();
        let actual_output = filter_colliding_types(tcx, input);
        assert_eq!(actual_output, expected_output);
    });
}

#[test]
fn test_preferred_type_is_present() {
    run_compiler_for_testing("", |tcx| {
        let input = [
            tcx.types.u32,
            tcx.types.usize,
            tcx.types.u64,
            tcx.types.i32,
            tcx.types.isize,
            tcx.types.i64,
        ];
        let expected_output = [tcx.types.usize, tcx.types.isize].to_vec();
        let actual_output = filter_colliding_types(tcx, input);
        assert_eq!(actual_output, expected_output);
    });
}

#[test]
fn test_tuple_collision() {
    run_compiler_for_testing("", |tcx| {
        let bool_ty = tcx.types.bool;
        let u64_ty = tcx.types.u64;
        let usize_ty = tcx.types.usize;

        let tuple_u64 = Ty::new_tup(tcx, &[u64_ty, bool_ty]);
        let tuple_usize = Ty::new_tup(tcx, &[usize_ty, bool_ty]);

        let input = [tuple_u64, tuple_usize];
        // We expect `(usize, bool)` to win, so `(u64, bool)` is filtered out.
        let expected_output = [tuple_usize].to_vec();
        let actual_output = filter_colliding_types(tcx, input);
        assert_eq!(actual_output, expected_output);
    });
}

#[test]
fn test_slice_collision() {
    run_compiler_for_testing("", |tcx| {
        let u64_ty = tcx.types.u64;
        let usize_ty = tcx.types.usize;

        let slice_u64 = Ty::new_slice(tcx, u64_ty);
        let slice_usize = Ty::new_slice(tcx, usize_ty);

        let input = [slice_u64, slice_usize];
        // We expect `[usize]` to win, so `[u64]` is filtered out.
        let expected_output = [slice_usize].to_vec();
        let actual_output = filter_colliding_types(tcx, input);
        assert_eq!(actual_output, expected_output);
    });
}

#[test]
fn test_ref_collision() {
    run_compiler_for_testing("", |tcx| {
        let u64_ty = tcx.types.u64;
        let usize_ty = tcx.types.usize;
        let region = tcx.lifetimes.re_erased;

        let ref_u64 = Ty::new_ref(tcx, region, u64_ty, rustc_middle::mir::Mutability::Not);
        let ref_usize = Ty::new_ref(tcx, region, usize_ty, rustc_middle::mir::Mutability::Not);

        let input = [ref_u64, ref_usize];
        // We expect `&usize` to win, so `&u64` is filtered out.
        let expected_output = [ref_usize].to_vec();
        let actual_output = filter_colliding_types(tcx, input);
        assert_eq!(actual_output, expected_output);
    });
}

#[test]
fn test_type_collision_risk_details() {
    run_compiler_for_testing("", |tcx| {
        let u32_ty = tcx.types.u32;
        let usize_ty = tcx.types.usize;

        let input = [u32_ty, usize_ty];
        let actual_output = input.into_iter().avoid_colliding_types(tcx, |ty| *ty);

        let expected_output = [
            Err(TypeCollisionRisk { item: u32_ty, key_type: u32_ty, preferred_type: usize_ty }),
            Ok(usize_ty),
        ]
        .to_vec();

        assert_eq!(actual_output, expected_output);
    });
}
