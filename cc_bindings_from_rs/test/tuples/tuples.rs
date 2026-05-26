// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `tuples_test.cc`.

use crubit_annotate::must_bind;
use std::sync::atomic::{AtomicU8, Ordering};

#[must_bind]
#[allow(clippy::unused_unit)]
pub fn return_unit_is_not_tuple() -> () {}
#[must_bind]
pub fn return_c_abi_compatible_five_in_tuple() -> (i32,) {
    (5,)
}
#[must_bind]
pub fn param_c_abi_compatible_five_in_tuple(five: (i32,)) {
    assert_eq!(five.0, 5);
}

#[must_bind]
pub struct AdtHoldingFiveAndSix {
    five: i32,
    six: i32,
}
#[must_bind]
pub fn return_adt_in_tuple() -> (AdtHoldingFiveAndSix,) {
    (AdtHoldingFiveAndSix { five: 5, six: 6 },)
}
#[must_bind]
pub fn param_adt_in_tuple(adt: (AdtHoldingFiveAndSix,)) {
    assert_eq!(adt.0.five, 5);
    assert_eq!(adt.0.six, 6);
}

static DROP_COUNT: AtomicU8 = AtomicU8::new(0);

// Note: we need a `Default` impl in order for the value to be C++-movable.
#[must_bind]
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

#[must_bind]
pub fn return_new_nontrivial_drop_in_tuple() -> (NontrivialDrop,) {
    (NontrivialDrop(243),)
}
#[must_bind]
pub fn param_nontrivial_drop_in_tuple(nontrivial_drop: (NontrivialDrop,)) {
    assert_eq!(nontrivial_drop.0 .0, 243);
}
#[must_bind]
pub fn assert_nontrivial_drop_count(drop_count: u8) {
    assert_eq!(DROP_COUNT.load(Ordering::Relaxed), drop_count);
}

/// The same as NontrivialDrop, but without a C++ move operation. This can be returned by value,
/// even inside a tuple!
#[must_bind]
pub struct NonCppMovable {
    pub value: u8,
}

impl Drop for NonCppMovable {
    fn drop(&mut self) {}
}

pub fn return_new_non_cpp_movable_in_tuple() -> (NonCppMovable,) {
    (NonCppMovable { value: 42 },)
}
// pub fn return_new_non_cpp_movable_in_nested_tuple() -> ((NonCppMovable,),) {
//     ((NonCppMovable {value: 42},),)
// }

#[must_bind]
pub fn param_nested_tuples(v: ((i32, i32), i32)) {
    assert_eq!(v, ((1, 2), 3));
}
#[must_bind]
pub fn return_nested_tuples() -> ((i32, i32), i32) {
    ((1, 2), 3)
}

#[must_bind]
pub fn param_triply_nested_tuple(v: (((i32,),),)) {
    assert_eq!(v.0 .0 .0, 57);
}
#[must_bind]
pub fn return_triply_nested_tuple() -> (((i32,),),) {
    (((57,),),)
}

#[must_bind]
pub fn param_ffi_alias_in_tuple(five: (std::ffi::c_char,)) {
    assert_eq!(five.0, 5);
}
#[must_bind]
pub fn return_ffi_alias_in_tuple() -> (std::ffi::c_char,) {
    (5,)
}

#[must_bind]
pub struct TupleStruct {
    pub tuple_field: (i32,),
    pub empty_tuple_field: (),
}

impl TupleStruct {
    // making this a method so we can check for it in a requires block
    pub fn tuple_not_by_value(&self) -> *const () {
        std::ptr::null()
    }
}

pub const TUPLE_CONSTANT: (i32,) = (42,);

#[must_bind]
pub struct GetsTuple {
    pub value: (u32, u32),
}
impl GetsTuple {
    #[must_bind]
    pub fn new(val: u32) -> Self {
        GetsTuple { value: (val, val) }
    }
}

#[must_bind]
pub struct NestedTupleIntermediate1 {
    pub v1: ((u32, u32), u32),
    pub v2: (u32, (u32, u32)),
}

#[must_bind]
pub struct NestedTupleIntermediate2 {
    pub v1: (((u32, u32), u32), u32),
    pub v2: (u32, (u32, (u32, u32))),
}

#[must_bind]
pub struct NestedTupleStruct {
    pub in_tuple1: (((u32, u32), u32), u32),
    pub in_tuple2: (u32, (u32, (u32, u32))),
}

impl NestedTupleStruct {
    #[must_bind]
    pub fn new(val: u32) -> Self {
        NestedTupleStruct {
            in_tuple1: (((val, val), val), val),
            in_tuple2: (val, (val, (val, val))),
        }
    }
}

#[must_bind]
#[derive(Copy, Clone)]
pub struct CopyNoDefault {
    pub val: u8,
}
impl CopyNoDefault {
    #[must_bind]
    pub fn new(val: u8) -> Self {
        CopyNoDefault { val }
    }
}

#[must_bind]
pub struct CopyNoDefaultTuple {
    pub in_tuple1: (CopyNoDefault, u8),
    pub in_tuple2: (u8, CopyNoDefault),
}
impl CopyNoDefaultTuple {
    #[must_bind]
    pub fn new(val: u8) -> Self {
        CopyNoDefaultTuple {
            in_tuple1: (CopyNoDefault { val }, val),
            in_tuple2: (val, CopyNoDefault { val }),
        }
    }
}

#[must_bind]
#[derive(Clone)]
pub struct CloneNoDefault {
    pub val: u8,
}
impl CloneNoDefault {
    #[must_bind]
    pub fn new(val: u8) -> Self {
        CloneNoDefault { val }
    }
}
#[must_bind]
pub struct CloneNoDefaultTuple {
    pub in_tuple1: (CloneNoDefault, u8),
    pub in_tuple2: (u8, CloneNoDefault),
}
impl CloneNoDefaultTuple {
    #[must_bind]
    pub fn new(val: u8) -> Self {
        CloneNoDefaultTuple {
            in_tuple1: (CloneNoDefault { val }, val),
            in_tuple2: (val, CloneNoDefault { val }),
        }
    }
}

#[must_bind]
#[derive(Default)]
pub struct HasDefault {
    pub val: String,
}
impl HasDefault {
    #[must_bind]
    pub fn new(val: &str) -> Self {
        HasDefault { val: val.to_string() }
    }

    #[must_bind]
    pub fn val(&self) -> &str {
        &self.val
    }
}
#[must_bind]
pub struct HasDefaultTuple {
    pub in_tuple1: (HasDefault, u8),
    pub in_tuple2: (u8, HasDefault),
}
impl HasDefaultTuple {
    #[must_bind]
    pub fn new(val: &str) -> Self {
        HasDefaultTuple {
            in_tuple1: (HasDefault { val: val.to_string() }, b'a'),
            in_tuple2: (b'a', HasDefault { val: val.to_string() }),
        }
    }
}

#[must_bind]
pub struct HasNoDefault {
    pub val: String,
}
impl HasNoDefault {
    #[must_bind]
    pub fn val(&self) -> &str {
        &self.val
    }
}
#[must_bind]
pub struct HasNoDefaultTuple {
    pub in_tuple1: (HasNoDefault, u8),
    pub in_tuple2: (u8, HasNoDefault),
}
impl HasNoDefaultTuple {
    #[must_bind]
    pub fn new(val: &str) -> Self {
        HasNoDefaultTuple {
            in_tuple1: (HasNoDefault { val: val.to_string() }, b'a'),
            in_tuple2: (b'a', HasNoDefault { val: val.to_string() }),
        }
    }
}
#[must_bind]
pub fn take_tuple_copy_no_default_1(r: &(CopyNoDefault, u8)) -> u8 {
    r.0.val
}

#[must_bind]
pub fn take_tuple_clone_no_default_2(r: &(u8, CloneNoDefault)) -> u8 {
    r.1.val
}

#[must_bind]
pub fn take_tuple_has_default(r: &(HasDefault, u8)) -> &str {
    r.0.val()
}

#[must_bind]
pub fn return_option_in_tuple() -> (Option<i32>,) {
    (Some(42),)
}

#[must_bind]
pub fn param_option_in_tuple(opt: (Option<i32>,)) {
    assert_eq!(opt.0, Some(42));
}

#[must_bind]
pub fn return_option_in_tuple_ref(opt: &(Option<i32>,)) -> Option<i32> {
    opt.0
}

#[must_bind]
pub struct StructWithOptionTuple {
    pub opt_tuple: (Option<i32>, Result<i32, String>),
}

impl StructWithOptionTuple {
    #[must_bind]
    pub fn new(val: i32) -> Self {
        StructWithOptionTuple { opt_tuple: (Some(val), Ok(val)) }
    }
}

pub struct TupleWithSizeTypes {
    // b/491106325 - We expect these not to get bindings.
    pub uval_in_tuple1: (usize, u8),
    pub uval_in_tuple2: (u8, usize),
    pub ival_in_tuple1: (isize, i8),
    pub ival_in_tuple2: (i8, isize),
}
