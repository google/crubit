// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::emplace;
use methods_qualifiers::*;
use std::pin::Pin;

#[test]
fn test_methods_on_mov_pinned_box_to_plain_struct() {
    let c = ClassWithQualifiedMethods { i: 0 };
    assert_eq!(ctor::mov!(Box::pin(c)).0.const_qualified_get_i(), 0);
    assert_eq!(ctor::mov!(Box::pin(c)).0.const_lvalue_ref_qualified_get_i(), 0);
    assert_eq!(ctor::mov!(Box::pin(c)).0.const_rvalue_ref_qualified_get_i(), 0);
}
#[test]
fn test_methods_on_mov_pinned_box_to_mut_struct() {
    let mut c_mut = ClassWithQualifiedMethods { i: 0 };
    c_mut.increment_i(); // Slience the warning on unused `mut`.
    assert_eq!(ctor::mov!(Box::pin(c_mut)).0.unqualified_get_i(), 1);
    assert_eq!(ctor::mov!(Box::pin(c_mut)).0.const_qualified_get_i(), 1);
    assert_eq!(ctor::mov!(Box::pin(c_mut)).0.lvalue_ref_qualified_get_i(), 1);
    assert_eq!(ctor::mov!(Box::pin(c_mut)).0.const_lvalue_ref_qualified_get_i(), 1);
    assert_eq!(ctor::mov!(Box::pin(c_mut)).0.rvalue_ref_qualified_get_i(), 1);
    assert_eq!(ctor::mov!(Box::pin(c_mut)).0.const_rvalue_ref_qualified_get_i(), 1);
}
#[test]
fn test_methods_on_mov_pinned_mut_struct_ref() {
    emplace! {let c : Pin<&mut _> = ClassWithQualifiedMethods{i:0};}
    assert_eq!(ctor::mov!(c).0.unqualified_get_i(), 0);
    emplace! {let c : Pin<&mut _> = ClassWithQualifiedMethods{i:0};}
    assert_eq!(ctor::mov!(c).0.const_qualified_get_i(), 0);
    emplace! {let c : Pin<&mut _> = ClassWithQualifiedMethods{i:0};}
    assert_eq!(ctor::mov!(c).0.lvalue_ref_qualified_get_i(), 0);
    emplace! {let c : Pin<&mut _> = ClassWithQualifiedMethods{i:0};}
    assert_eq!(ctor::mov!(c).0.const_lvalue_ref_qualified_get_i(), 0);
    emplace! {let c : Pin<&mut _> = ClassWithQualifiedMethods{i:0};}
    assert_eq!(ctor::mov!(c).0.rvalue_ref_qualified_get_i(), 0);
    emplace! {let c : Pin<&mut _> = ClassWithQualifiedMethods{i:0};}
    assert_eq!(ctor::mov!(c).0.const_rvalue_ref_qualified_get_i(), 0);
}
