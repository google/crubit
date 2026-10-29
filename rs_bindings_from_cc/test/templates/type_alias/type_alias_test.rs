// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::CRef;
use forward_declare::CppCast;
use googletest::gtest;

#[gtest]
fn test_alias_to_template_instantiation() {
    let s = type_alias::MyTypeAlias::Create(123);
    // SAFETY: s is alive while s.value() is alive; no other references to s exist.
    assert_eq!(123, *unsafe { CRef::unchanging(s.value()) });
}

#[gtest]
fn test_aliases_in_same_target_are_compatible() {
    let s: type_alias::MyTypeAlias = type_alias::MyTypeAlias::Create(456);
    let s2: type_alias::OtherTypeAliasInSameTarget = s;
    // SAFETY: s2 is alive while s2.value() is alive; no other references to s2 exist.
    assert_eq!(456, *unsafe { CRef::unchanging(s2.value()) });
}

#[gtest]
fn test_alias_in_different_target_than_template() {
    let s = type_alias_in_different_target::TypeAliasInDifferentTarget::Create(789);
    // SAFETY: s is alive while s.value() is alive; no other references to s exist.
    assert_eq!(789, *unsafe { CRef::unchanging(s.value()) });

    // Template instantiation from `type_alias_in_different_target` can be cast
    // (i.e. transmuted) into identical instantiation from `type_alias` crate.
    let s2: type_alias::MyTypeAlias = s.cpp_cast();
    // SAFETY: s2 is alive while s2.value() is alive; no other references to s2 exist.
    assert_eq!(789, *unsafe { CRef::unchanging(s2.value()) });
}

#[gtest]
fn test_instantiation_is_named_after_first_alias() {
    // `MyTemplate<int>` is generated as the struct `MyTypeAlias`, and the old mangled name is a
    // (hidden) alias of it.
    let s: type_alias::__CcTemplateInst10MyTemplateIiE = type_alias::MyTypeAlias::Create(1);
    let s2: type_alias::MyTypeAlias = s;
    assert_eq!(1, type_alias::GetValue(s2));
}

#[gtest]
fn test_pass_named_instantiation_by_value_and_by_reference() {
    let s = type_alias::OtherTypeAliasInSameTarget::Create(42);
    assert_eq!(42, type_alias::GetValue(s));
    assert_eq!(42, type_alias::GetValueByRef(&s));
}

#[gtest]
fn test_instantiation_named_after_alias_in_nested_namespace() {
    let s = type_alias::nested::NestedAlias::Create(1.5);
    // SAFETY: s is alive while s.value() is alive; no other references to s exist.
    assert_eq!(1.5, *unsafe { CRef::unchanging(s.value()) });
}

#[gtest]
fn test_unaliased_instantiation_keeps_mangled_name() {
    let s: type_alias::__CcTemplateInst10MyTemplateIdE = type_alias::MakeUnaliased(7.0);
    // SAFETY: s is alive while s.value() is alive; no other references to s exist.
    assert_eq!(7.0, *unsafe { CRef::unchanging(s.value()) });
}
