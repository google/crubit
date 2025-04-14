// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use forward_declare::CppCast;
use googletest::prelude::*;
use static_assertions::{assert_impl_all, assert_not_impl_any};
use std::pin::Pin;

/// Given a complete UnpinStruct, all APIs accepting a (possibly incomplete)
/// UnpinStruct work (with an cpp_cast()).
#[gtest]
fn test_read_complete_unpin() {
    let s = definition::ns::UnpinStruct { field: 42 };
    let s = &s;

    // The normal way to call it, if you have a complete type (and know it).
    assert_eq!(definition::ns::ReadUnpinStruct(s), 42);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    assert_eq!(definition::ns::ReadUnpinStruct(s.cpp_cast()), 42);

    // Actual conversion.
    assert_eq!(declaration_1::ns::ReadUnpinStruct(s.cpp_cast()), 42);
    assert_eq!(declaration_2::ns::ReadUnpinStruct(s.cpp_cast()), 42);
}

/// Given a complete UnpinStruct, all APIs accepting a (possibly incomplete)
/// mut UnpinStruct work (with an cpp_cast()).
#[gtest]
fn test_write_complete_unpin() {
    let mut s = definition::ns::UnpinStruct { field: 42 };
    let s = &mut s;

    // The normal way to call it, if you have a complete type (and know it).
    definition::ns::WriteUnpinStruct(s, 0);
    assert_eq!(definition::ns::ReadUnpinStruct(s), 0);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    definition::ns::WriteUnpinStruct(s.cpp_cast(), 1);
    assert_eq!(definition::ns::ReadUnpinStruct(s), 1);

    // Actual conversions.
    declaration_1::ns::WriteUnpinStruct(s.cpp_cast(), 2);
    assert_eq!(definition::ns::ReadUnpinStruct(s), 2);
    declaration_2::ns::WriteUnpinStruct(s.cpp_cast(), 2);
    assert_eq!(definition::ns::ReadUnpinStruct(s), 2);
}

/// Given an incomplete UnpinStruct, all APIs accepting a (possibly
/// incomplete) UnpinStruct work (with an cpp_cast()).
#[gtest]
fn test_read_incomplete_unpin() {
    let s = definition::ns::UnpinStruct { field: 42 };
    let decl1_s: &declaration_1::ns::UnpinStruct = (&s).cpp_cast();

    // Cast from incomplete to complete:
    assert_eq!(definition::ns::ReadUnpinStruct(decl1_s.cpp_cast()), 42);

    // No cast necessary if it's the same forward declaration.
    assert_eq!(declaration_1::ns::ReadUnpinStruct(&*decl1_s), 42);
    // Buit a self-cast also works:
    assert_eq!(declaration_1::ns::ReadUnpinStruct(decl1_s.cpp_cast()), 42);

    // Cast from incomplete to different-incomplete:
    assert_eq!(declaration_2::ns::ReadUnpinStruct(decl1_s.cpp_cast()), 42);
}

/// Given an incomplete UnpinStruct, all APIs accepting a (possibly
/// incomplete) mut UnpinStruct work (with an cpp_cast()).
#[gtest]
fn test_write_incomplete_unpin() {
    let mut s = definition::ns::UnpinStruct { field: 42 };
    let mut decl1_s: Pin<&mut declaration_1::ns::UnpinStruct> = (&mut s).cpp_cast();

    // Cast from incomplete to complete:
    definition::ns::WriteUnpinStruct(decl1_s.as_mut().cpp_cast(), 0);
    assert_eq!(declaration_1::ns::ReadUnpinStruct(&*decl1_s), 0);

    // No cast necessary if it's the same forward declaration.
    declaration_1::ns::WriteUnpinStruct(decl1_s.as_mut(), 1);
    assert_eq!(declaration_1::ns::ReadUnpinStruct(&*decl1_s), 1);
    // But a self-cast also works.
    declaration_1::ns::WriteUnpinStruct(decl1_s.as_mut(), 2);
    assert_eq!(declaration_1::ns::ReadUnpinStruct(&*decl1_s), 2);

    // Cast from incomplete to different-incomplete:
    declaration_2::ns::WriteUnpinStruct(decl1_s.as_mut().cpp_cast(), 3);
    assert_eq!(declaration_1::ns::ReadUnpinStruct(&*decl1_s), 3);
}

/// Given a complete NonunpinStruct, all APIs accepting a (possibly incomplete)
/// NonunpinStruct work (with an cpp_cast()).
#[gtest]
fn test_read_complete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::ns::NonunpinStruct {field: 42});
    }

    // The normal way to call it, if you have a complete type (and know it).
    assert_eq!(definition::ns::ReadNonunpinStruct(&*s), 42);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    assert_eq!(definition::ns::ReadNonunpinStruct(s.as_ref().cpp_cast()), 42);

    // Actual conversion.
    assert_eq!(declaration_1::ns::ReadNonunpinStruct(s.as_ref().cpp_cast()), 42);
    assert_eq!(declaration_2::ns::ReadNonunpinStruct(s.as_ref().cpp_cast()), 42);
}

/// Given a complete NonunpinStruct, all APIs accepting a (possibly incomplete)
/// mut NonunpinStruct work (with an cpp_cast()).
#[gtest]
fn test_write_complete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::ns::NonunpinStruct {field: 42});
    }

    // The normal way to call it, if you have a complete type (and know it).
    definition::ns::WriteNonunpinStruct(s.as_mut(), 0);
    assert_eq!(definition::ns::ReadNonunpinStruct(&*s), 0);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    definition::ns::WriteNonunpinStruct(s.as_mut().cpp_cast(), 1);
    assert_eq!(definition::ns::ReadNonunpinStruct(&*s), 1);

    // Actual conversions.
    declaration_1::ns::WriteNonunpinStruct(s.as_mut().cpp_cast(), 2);
    assert_eq!(definition::ns::ReadNonunpinStruct(&*s), 2);
    declaration_2::ns::WriteNonunpinStruct(s.as_mut().cpp_cast(), 2);
    assert_eq!(definition::ns::ReadNonunpinStruct(&*s), 2);
}

/// Given an incomplete NonunpinStruct, all APIs accepting a (possibly
/// incomplete) NonunpinStruct work (with an cpp_cast()).
#[gtest]
fn test_read_incomplete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::ns::NonunpinStruct {field: 42});
    }
    let decl1_s: Pin<&mut declaration_1::ns::NonunpinStruct> = s.cpp_cast();

    // Cast from incomplete to complete:
    assert_eq!(definition::ns::ReadNonunpinStruct(decl1_s.as_ref().cpp_cast()), 42);

    // No cast necessary if it's the same forward declaration.
    assert_eq!(declaration_1::ns::ReadNonunpinStruct(&*decl1_s), 42);
    // Buit a self-cast also works:
    assert_eq!(declaration_1::ns::ReadNonunpinStruct(decl1_s.as_ref().cpp_cast()), 42);

    // Cast from incomplete to different-incomplete:
    assert_eq!(declaration_2::ns::ReadNonunpinStruct(decl1_s.as_ref().cpp_cast()), 42);
}

/// Given an incomplete NonunpinStruct, all APIs accepting a (possibly
/// incomplete) mut NonunpinStruct work (with an cpp_cast()).
#[gtest]
fn test_write_incomplete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::ns::NonunpinStruct {field: 42});
    }
    let mut decl1_s: Pin<&mut declaration_1::ns::NonunpinStruct> = s.cpp_cast();

    // Cast from incomplete to complete:
    definition::ns::WriteNonunpinStruct(decl1_s.as_mut().cpp_cast(), 0);
    assert_eq!(declaration_1::ns::ReadNonunpinStruct(&*decl1_s), 0);

    // No cast necessary if it's the same forward declaration.
    declaration_1::ns::WriteNonunpinStruct(decl1_s.as_mut(), 1);
    assert_eq!(declaration_1::ns::ReadNonunpinStruct(&*decl1_s), 1);
    // But a self-cast also works.
    declaration_1::ns::WriteNonunpinStruct(decl1_s.as_mut(), 2);
    assert_eq!(declaration_1::ns::ReadNonunpinStruct(&*decl1_s), 2);

    // Cast from incomplete to different-incomplete:
    declaration_2::ns::WriteNonunpinStruct(decl1_s.as_mut().cpp_cast(), 3);
    assert_eq!(declaration_1::ns::ReadNonunpinStruct(&*decl1_s), 3);
}

#[gtest]
fn test_inline_functions_with_incomplete_parameters() {
    let unpin = definition::ns::UnpinStruct { field: 42 };
    let unpin_ref = &unpin;
    assert_eq!(42, declaration_1::ns::InlineFunctionTakingUnpinStruct(unpin_ref.cpp_cast()));

    ctor::emplace! {
      let nonunpin = ctor::ctor!(definition::ns::NonunpinStruct {field: 123});
    }
    let nonunpin_ref = &*nonunpin;
    assert_eq!(123, declaration_1::ns::InlineFunctionTakingNonunpinStruct(nonunpin_ref.cpp_cast()));
}

/// Classes in different forward-declared namespaces should not be castable to
/// one another.
#[gtest]
fn test_namespaced_forward_declarations() {
    type Declaration = *const declaration_1::ns::UnpinStruct;
    type Definition = *const definition::ns::UnpinStruct;
    type Other = *const declaration_other::ns_other::UnpinStruct;
    assert_impl_all!(Declaration: CppCast<Definition>);
    assert_impl_all!(Definition: CppCast<Declaration>);

    assert_not_impl_any!(Definition: CppCast<Other>);
    assert_not_impl_any!(Declaration: CppCast<Other>);

    assert_not_impl_any!(Other: CppCast<Definition>);
    assert_not_impl_any!(Other: CppCast<Declaration>);
}

#[gtest]
fn test_forward_declared_used_as_field_type() {
    // This is a regression test for b/246962427.  This mostly verifies that the
    // generated bindings compile (and are usable at a very basic level).
    use no_definition_in_headers::no_definition_in_headers::*;
    let _s = Defined { field: std::ptr::null_mut() };
}
