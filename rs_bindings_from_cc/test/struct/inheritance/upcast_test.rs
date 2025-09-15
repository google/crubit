// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::CtorNew as _;
use googletest::prelude::*;
use oops::Upcast as _;
use upcast::*;

#[gtest]
fn test_upcast() {
    let derived = Derived::default();
    let derived = &derived;
    let base0: &Base0 = derived.upcast();
    assert_eq!(base0 as *const _ as usize, derived.base0_address());
    assert_eq!(base0 as *const _ as usize, derived as *const _ as usize);
    let base1: &Base1 = derived.upcast();
    assert_eq!(base1 as *const _ as usize, derived.base1_address());
    let base2: &Base2 = derived.upcast();
    assert_eq!(base2 as *const _ as usize, derived.base2_address());
    let base3: &Base3 = derived.upcast();
    assert_eq!(base3 as *const _ as usize, derived.base3_address());
    let base4: &Base4 = derived.upcast();
    assert_eq!(base4 as *const _ as usize, derived.base4_address());
}

#[gtest]
fn test_virtual_upcast() {
    use upcast::virtual_inheritance::*;
    ctor::emplace! {
        let derived = VirtualDerived::ctor_new(());
    }
    let derived = &*derived;

    let base1: &Base1 = derived.upcast();
    let base1_address = base1 as *const _ as usize;
    assert_eq!(base1_address, derived.base1_address());
    let base2: &VirtualBase2 = derived.upcast();
    assert_eq!(base2 as *const _ as usize, derived.base2_address());
    let base3: &VirtualBase3 = derived.upcast();
    assert_eq!(base3 as *const _ as usize, derived.base3_address());

    let base1: &Base1 = base2.upcast();
    assert_eq!(base1 as *const _ as usize, base1_address);
    let base1: &Base1 = base3.upcast();
    assert_eq!(base1 as *const _ as usize, base1_address);
}

#[gtest]
fn test_upcast_thunk_name_uniqueness() {
    ctor::emplace! {
        let derived = another_namespace::VirtualBase2::ctor_new(());
    }
    let derived = &*derived;

    let base1: &Base1 = derived.upcast();
    let base1_address = base1 as *const _ as usize;
    assert_eq!(base1_address, derived.base1_address());
}
