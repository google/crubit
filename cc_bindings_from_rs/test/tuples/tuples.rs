// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `tuples_test.cc`.

use std::sync::atomic::{AtomicU8, Ordering};

#[allow(clippy::unused_unit)]
pub fn return_unit_is_not_tuple() -> () {}
pub fn return_c_abi_compatible_five_in_tuple() -> (i32,) {
    (5,)
}
pub fn param_c_abi_compatible_five_in_tuple(five: (i32,)) {
    assert_eq!(five.0, 5);
}

pub struct AdtHoldingFiveAndSix {
    five: i32,
    six: i32,
}
pub fn return_adt_in_tuple() -> (AdtHoldingFiveAndSix,) {
    (AdtHoldingFiveAndSix { five: 5, six: 6 },)
}
pub fn param_adt_in_tuple(adt: (AdtHoldingFiveAndSix,)) {
    assert_eq!(adt.0.five, 5);
    assert_eq!(adt.0.six, 6);
}

static DROP_COUNT: AtomicU8 = AtomicU8::new(0);

// Note: we need a `Default` impl in order for the value to be C++-movable.
#[derive(Default)]
pub struct NontrivialDrop(u8);

impl Drop for NontrivialDrop {
    fn drop(&mut self) {
        if self.0 != 0 {
            DROP_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        self.0 = 55;
    }
}
pub fn return_new_nontrivial_drop_in_tuple() -> (NontrivialDrop,) {
    (NontrivialDrop(243),)
}
pub fn param_nontrivial_drop_in_tuple(nontrivial_drop: (NontrivialDrop,)) {
    assert_eq!(nontrivial_drop.0 .0, 243);
}
pub fn assert_nontrivial_drop_count(drop_count: u8) {
    assert_eq!(DROP_COUNT.load(Ordering::Relaxed), drop_count);
}

pub fn param_nested_tuples(v: ((i32, i32), i32)) {
    assert_eq!(v, ((1, 2), 3));
}
pub fn return_nested_tuples() -> ((i32, i32), i32) {
    ((1, 2), 3)
}

pub fn param_triply_nested_tuple(v: (((i32,),),)) {
    assert_eq!(v.0 .0 .0, 57);
}
pub fn return_triply_nested_tuple() -> (((i32,),),) {
    (((57,),),)
}

pub fn param_ffi_alias_in_tuple(five: (std::ffi::c_char,)) {
    assert_eq!(five.0, 5);
}
pub fn return_ffi_alias_in_tuple() -> (std::ffi::c_char,) {
    (5,)
}
