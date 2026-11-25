// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::pin::Pin;
use ctor::{emplace, CtorNew as _};
use googletest::gtest;
use oops::Upcast as _;
use oops::VirtualUpcast;
use upcast::*;

#[gtest]
fn test_upcast() {
    let derived = Derived::default();
    let derived = &derived;
    let base0: &Base0 = unsafe { derived.upcast() };
    assert_eq!(base0 as *const _ as usize, derived.base0_address());
    assert_eq!(base0 as *const _ as usize, derived as *const _ as usize);
    let base1: &Base1 = unsafe { derived.upcast() };
    assert_eq!(base1 as *const _ as usize, derived.base1_address());
    let base2: &Base2 = unsafe { derived.upcast() };
    assert_eq!(base2 as *const _ as usize, derived.base2_address());
    let base3: &Base3 = unsafe { derived.upcast() };
    assert_eq!(base3 as *const _ as usize, derived.base3_address());
    let base4: &Base4 = unsafe { derived.upcast() };
    assert_eq!(base4 as *const _ as usize, derived.base4_address());
}

#[gtest]
fn test_virtual_upcast() {
    use upcast::virtual_inheritance::*;
    let derived = emplace!(VirtualDerived::ctor_new(()));
    let derived = &*derived;

    let base1: *const Base1 = unsafe { (derived as *const VirtualDerived).virtual_upcast() };
    let base1_address = base1 as usize;
    assert_eq!(base1_address, derived.base1_address());

    let base2: *const VirtualBase2 = unsafe { (derived as *const VirtualDerived).virtual_upcast() };
    assert_eq!(base2 as usize, derived.base2_address());

    let base3: *const VirtualBase3 = unsafe { (derived as *const VirtualDerived).virtual_upcast() };
    assert_eq!(base3 as *const _ as usize, derived.base3_address());

    let base1: *const Base1 = unsafe { base2.virtual_upcast() };
    assert_eq!(base1 as usize, base1_address);
    let base1: *const Base1 = unsafe { base3.virtual_upcast() };
    assert_eq!(base1 as usize, base1_address);
}

#[gtest]
fn test_virtual_base_with_derived_fields_layout_and_upcast() {
    use upcast::virtual_inheritance::*;

    let canary_before: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let derived = emplace!(DerivedWithFieldAndVirtualBase::ctor_new((7i8, 999i64)));
    let canary_after: u64 = 0x0123_4567_89AB_CDEF;

    assert_eq!(derived.derived_field, 7i8);
    assert_eq!(derived.read_base_value(), 999);

    derived.increment_counter();
    assert_eq!(derived.get_counter(), 1);

    let vbase_ptr: *const VirtualBaseWithData =
        unsafe { (&*derived as *const DerivedWithFieldAndVirtualBase).virtual_upcast() };
    assert_eq!(vbase_ptr as usize, derived.base_address());
    assert_eq!(unsafe { (*vbase_ptr).base_value }, 999);
    unsafe { (*vbase_ptr).increment_counter() };
    assert_eq!(derived.get_counter(), 2);

    assert_eq!(core::hint::black_box(canary_before), 0xDEAD_BEEF_CAFE_BABE);
    assert_eq!(core::hint::black_box(canary_after), 0x0123_4567_89AB_CDEF);
}

#[gtest]
fn test_diamond_virtual_inheritance_with_fields() {
    use upcast::virtual_inheritance::*;

    let canary_before: u64 = 0x1111_2222_3333_4444;
    let diamond = emplace!(DiamondFinal::ctor_new((111, 222, 33i8, 4444i64)));
    let canary_after: u64 = 0x5555_6666_7777_8888;

    assert_eq!(diamond.final_field, 33i8);

    let left: &DiamondLeft = unsafe { (&*diamond).upcast() };
    assert_eq!(left as *const _ as usize, diamond.left_address());
    assert_eq!(left.left_field, 111);

    let right: &DiamondRight = unsafe { (&*diamond).upcast() };
    assert_eq!(right as *const _ as usize, diamond.right_address());
    assert_eq!(right.right_field, 222);

    let vbase_from_final: *const VirtualBaseWithData =
        unsafe { (&*diamond as *const DiamondFinal).virtual_upcast() };
    let vbase_from_left: *const VirtualBaseWithData =
        unsafe { (left as *const DiamondLeft).virtual_upcast() };
    let vbase_from_right: *const VirtualBaseWithData =
        unsafe { (right as *const DiamondRight).virtual_upcast() };
    assert_eq!(vbase_from_final as usize, diamond.vbase_address());
    assert_eq!(vbase_from_left as usize, diamond.vbase_address());
    assert_eq!(vbase_from_right as usize, diamond.vbase_address());
    assert_eq!(unsafe { (*vbase_from_final).base_value }, 4444);

    assert_eq!(core::hint::black_box(canary_before), 0x1111_2222_3333_4444);
    assert_eq!(core::hint::black_box(canary_after), 0x5555_6666_7777_8888);
}

#[gtest]
fn test_overaligned_virtual_base_with_derived_field() {
    use upcast::virtual_inheritance::*;

    assert_eq!(core::mem::align_of::<DerivedFromOveralignedVirtualBase>(), 32);
    let derived = emplace!(DerivedFromOveralignedVirtualBase::ctor_new(()));
    assert_eq!((&*derived as *const _ as usize) % 32, 0);
    assert_eq!(derived.derived_field, 5i8);

    let vbase_ptr: *const OveralignedVirtualBase =
        unsafe { (&*derived as *const DerivedFromOveralignedVirtualBase).virtual_upcast() };
    assert_eq!(vbase_ptr as usize, derived.base_address());
    assert_eq!((vbase_ptr as usize) % 32, 0);
    assert_eq!(unsafe { (*vbase_ptr).data }, 77);
}

#[gtest]
fn test_upcast_thunk_name_uniqueness() {
    let derived = emplace!(another_namespace::VirtualBase2::ctor_new(()));
    let derived = &*derived;

    let base1: *const Base1 =
        unsafe { (derived as *const another_namespace::VirtualBase2).virtual_upcast() };
    let base1_address = base1 as usize;
    assert_eq!(base1_address, derived.base1_address());
}

#[gtest]
fn test_upcast_pin_mut() {
    use upcast::nonunpin_upcast::{NonUnpinBase, NonUnpinDerived};
    let mut derived = emplace!(NonUnpinDerived::ctor_new(()));
    // Call the non-const method via upcast:
    let base: Pin<&mut NonUnpinBase> = unsafe { derived.as_mut().upcast() };
    base.non_const_method();
}

// `Pin<&mut Derived>` can only be upcast if the base is `!Unpin`: if it were `Unpin`, safe code
// could turn the `Pin<&mut Base>` into a `&mut Base` and overwrite the base subobject.
static_assertions::assert_impl_all!(
    nonunpin_upcast::NonUnpinDerived: oops::InheritsPinned<nonunpin_upcast::NonUnpinBase>
);
static_assertions::assert_impl_all!(Derived: oops::Inherits<Base1>);
static_assertions::assert_not_impl_any!(Derived: oops::InheritsPinned<Base1>);
static_assertions::assert_not_impl_any!(
    Pin<&'static mut Derived>: oops::Upcast<Pin<&'static mut Base1>>
);
