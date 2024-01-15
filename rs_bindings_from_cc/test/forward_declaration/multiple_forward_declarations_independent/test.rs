// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::any::TypeId;
use forward_declare::CcCast;

#[test]
fn test_complete_to_incomplete_ptr_conversion_crossing_crate_boundaries() {
    let mut a = definition::A::default();
    let a_ptr: *mut definition::A = &mut a;
    unsafe {
        let identity1 = forward_declaration1::IdentityPtr1(a_ptr.cc_cast());
        let identity2 = forward_declaration2::IdentityPtr2(a_ptr.cc_cast());

        let identity2_from_identity1 = forward_declaration2::IdentityPtr2(identity1.cc_cast());
        let identity1_from_identity2 = forward_declaration1::IdentityPtr1(identity2.cc_cast());

        let _ = forward_declaration1::IdentityPtr1(identity2_from_identity1.cc_cast());
        let _ = forward_declaration2::IdentityPtr2(identity1_from_identity2.cc_cast());
    }
}

#[test]
fn test_complete_to_incomplete_ref_conversion_crossing_crate_boundaries() {
    let mut a = definition::A::default();
    let a_ptr: *mut definition::A = &mut a;
    unsafe {
        let identity1 = forward_declaration1::IdentityPtr1(a_ptr.cc_cast());
        let identity2 = forward_declaration2::IdentityPtr2(a_ptr.cc_cast());

        let identity2_from_identity1 = forward_declaration2::IdentityPtr2(identity1.cc_cast());
        let identity1_from_identity2 = forward_declaration1::IdentityPtr1(identity2.cc_cast());

        let _ = forward_declaration1::IdentityPtr1(identity2_from_identity1.cc_cast());
        let _ = forward_declaration2::IdentityPtr2(identity1_from_identity2.cc_cast());
    }
}

#[test]
fn test_each_crate_has_distinct_type_for_a() {
    assert_ne!(TypeId::of::<forward_declaration1::A>(), TypeId::of::<forward_declaration2::A>());
    assert_ne!(TypeId::of::<forward_declaration1::A>(), TypeId::of::<definition::A>());
    assert_ne!(TypeId::of::<forward_declaration2::A>(), TypeId::of::<definition::A>());
}
