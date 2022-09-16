// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use forward_declare::CcCast as _;
use std::pin::Pin;

/// Given a complete UnpinStruct, all APIs accepting a (possibly incomplete)
/// UnpinStruct work (with an cc_cast()).
#[test]
fn test_read_complete_unpin() {
    let s = definition::UnpinStruct { field: 42 };
    let s = &s;

    // The normal way to call it, if you have a complete type (and know it).
    assert_eq!(definition::ReadUnpinStruct(s), 42);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    assert_eq!(definition::ReadUnpinStruct(s.cc_cast()), 42);

    // Actual conversion.
    assert_eq!(declaration_1::ReadUnpinStruct(s.cc_cast()), 42);
    assert_eq!(declaration_2::ReadUnpinStruct(s.cc_cast()), 42);
}

/// Given a complete UnpinStruct, all APIs accepting a (possibly incomplete)
/// mut UnpinStruct work (with an cc_cast()).
#[test]
fn test_write_complete_unpin() {
    let mut s = definition::UnpinStruct { field: 42 };
    let s = &mut s;

    // The normal way to call it, if you have a complete type (and know it).
    definition::WriteUnpinStruct(s, 0);
    assert_eq!(definition::ReadUnpinStruct(s), 0);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    definition::WriteUnpinStruct(s.cc_cast(), 1);
    assert_eq!(definition::ReadUnpinStruct(s), 1);

    // Actual conversions.
    declaration_1::WriteUnpinStruct(s.cc_cast(), 2);
    assert_eq!(definition::ReadUnpinStruct(s), 2);
    declaration_2::WriteUnpinStruct(s.cc_cast(), 2);
    assert_eq!(definition::ReadUnpinStruct(s), 2);
}

/// Given an incomplete UnpinStruct, all APIs accepting a (possibly
/// incomplete) UnpinStruct work (with an cc_cast()).
#[test]
fn test_read_incomplete_unpin() {
    let s = definition::UnpinStruct { field: 42 };
    let decl1_s: &declaration_1::UnpinStruct = (&s).cc_cast();

    // Cast from incomplete to complete:
    assert_eq!(definition::ReadUnpinStruct(decl1_s.cc_cast()), 42);

    // No cast necessary if it's the same forward declaration.
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 42);
    // Buit a self-cast also works:
    assert_eq!(declaration_1::ReadUnpinStruct(decl1_s.cc_cast()), 42);

    // Cast from incomplete to different-incomplete:
    assert_eq!(declaration_2::ReadUnpinStruct(decl1_s.cc_cast()), 42);
}

/// Given an incomplete UnpinStruct, all APIs accepting a (possibly
/// incomplete) mut UnpinStruct work (with an cc_cast()).
#[test]
fn test_write_incomplete_unpin() {
    let mut s = definition::UnpinStruct { field: 42 };
    let mut decl1_s: Pin<&mut declaration_1::UnpinStruct> = (&mut s).cc_cast();

    // Cast from incomplete to complete:
    definition::WriteUnpinStruct(decl1_s.as_mut().cc_cast(), 0);
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 0);

    // No cast necessary if it's the same forward declaration.
    declaration_1::WriteUnpinStruct(decl1_s.as_mut(), 1);
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 1);
    // But a self-cast also works.
    declaration_1::WriteUnpinStruct(decl1_s.as_mut(), 2);
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 2);

    // Cast from incomplete to different-incomplete:
    declaration_2::WriteUnpinStruct(decl1_s.as_mut().cc_cast(), 3);
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 3);
}

/// Given a complete NonunpinStruct, all APIs accepting a (possibly incomplete)
/// NonunpinStruct work (with an cc_cast()).
#[test]
fn test_read_complete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::NonunpinStruct {field: 42});
    }

    // The normal way to call it, if you have a complete type (and know it).
    assert_eq!(definition::ReadNonunpinStruct(&*s), 42);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    assert_eq!(definition::ReadNonunpinStruct(s.as_ref().cc_cast()), 42);

    // Actual conversion.
    assert_eq!(declaration_1::ReadNonunpinStruct(s.as_ref().cc_cast()), 42);
    assert_eq!(declaration_2::ReadNonunpinStruct(s.as_ref().cc_cast()), 42);
}

/// Given a complete NonunpinStruct, all APIs accepting a (possibly incomplete)
/// mut NonunpinStruct work (with an cc_cast()).
#[test]
fn test_write_complete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::NonunpinStruct {field: 42});
    }

    // The normal way to call it, if you have a complete type (and know it).
    definition::WriteNonunpinStruct(s.as_mut(), 0);
    assert_eq!(definition::ReadNonunpinStruct(&*s), 0);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    definition::WriteNonunpinStruct(s.as_mut().cc_cast(), 1);
    assert_eq!(definition::ReadNonunpinStruct(&*s), 1);

    // Actual conversions.
    declaration_1::WriteNonunpinStruct(s.as_mut().cc_cast(), 2);
    assert_eq!(definition::ReadNonunpinStruct(&*s), 2);
    declaration_2::WriteNonunpinStruct(s.as_mut().cc_cast(), 2);
    assert_eq!(definition::ReadNonunpinStruct(&*s), 2);
}

/// Given an incomplete NonunpinStruct, all APIs accepting a (possibly
/// incomplete) NonunpinStruct work (with an cc_cast()).
#[test]
fn test_read_incomplete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::NonunpinStruct {field: 42});
    }
    let decl1_s: Pin<&mut declaration_1::NonunpinStruct> = s.cc_cast();

    // Cast from incomplete to complete:
    assert_eq!(definition::ReadNonunpinStruct(decl1_s.as_ref().cc_cast()), 42);

    // No cast necessary if it's the same forward declaration.
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 42);
    // Buit a self-cast also works:
    assert_eq!(declaration_1::ReadNonunpinStruct(decl1_s.as_ref().cc_cast()), 42);

    // Cast from incomplete to different-incomplete:
    assert_eq!(declaration_2::ReadNonunpinStruct(decl1_s.as_ref().cc_cast()), 42);
}

/// Given an incomplete NonunpinStruct, all APIs accepting a (possibly
/// incomplete) mut NonunpinStruct work (with an cc_cast()).
#[test]
fn test_write_incomplete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::NonunpinStruct {field: 42});
    }
    let mut decl1_s: Pin<&mut declaration_1::NonunpinStruct> = s.cc_cast();

    // Cast from incomplete to complete:
    definition::WriteNonunpinStruct(decl1_s.as_mut().cc_cast(), 0);
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 0);

    // No cast necessary if it's the same forward declaration.
    declaration_1::WriteNonunpinStruct(decl1_s.as_mut(), 1);
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 1);
    // But a self-cast also works.
    declaration_1::WriteNonunpinStruct(decl1_s.as_mut(), 2);
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 2);

    // Cast from incomplete to different-incomplete:
    declaration_2::WriteNonunpinStruct(decl1_s.as_mut().cc_cast(), 3);
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 3);
}

#[test]
fn test_inline_functions_with_incomplete_parameters() {
    let unpin = definition::UnpinStruct { field: 42 };
    let unpin_ref = &unpin;
    assert_eq!(42, declaration_1::InlineFunctionTakingUnpinStruct(unpin_ref.cc_cast()));

    ctor::emplace! {
      let nonunpin = ctor::ctor!(definition::NonunpinStruct {field: 123});
    }
    let nonunpin_ref = &*nonunpin;
    assert_eq!(123, declaration_1::InlineFunctionTakingNonunpinStruct(nonunpin_ref.cc_cast()));
}

#[test]
fn test_forward_declared_used_as_field_type() {
    // This is a regression test for b/246962427.  This mostly verifies that the
    // generated bindings compile (and are usable at a very basic level).
    use no_definition_in_headers::no_definition_in_headers::*;
    let _s = Defined { field: std::ptr::null_mut() };
}
