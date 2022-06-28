// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use forward_declare::IncompleteCast as _;
use std::pin::Pin;

/// Given a complete UnpinStruct, all APIs accepting a (possibly incomplete)
/// UnpinStruct work (with an incomplete_cast()).
#[test]
fn test_read_complete_unpin() {
    let s = definition::UnpinStruct { field: 42 };
    let s = &s;

    // The normal way to call it, if you have a complete type (and know it).
    assert_eq!(definition::ReadUnpinStruct(s), 42);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    assert_eq!(definition::ReadUnpinStruct(s.incomplete_cast()), 42);

    // Actual conversion.
    assert_eq!(declaration_1::ReadUnpinStruct(s.incomplete_cast()), 42);
    assert_eq!(declaration_2::ReadUnpinStruct(s.incomplete_cast()), 42);
}

/// Given a complete UnpinStruct, all APIs accepting a (possibly incomplete)
/// mut UnpinStruct work (with an incomplete_cast()).
#[test]
fn test_write_complete_unpin() {
    let mut s = definition::UnpinStruct { field: 42 };
    let s = &mut s;

    // The normal way to call it, if you have a complete type (and know it).
    definition::WriteUnpinStruct(s, 0);
    assert_eq!(definition::ReadUnpinStruct(s), 0);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    definition::WriteUnpinStruct(s.incomplete_cast(), 1);
    assert_eq!(definition::ReadUnpinStruct(s), 1);

    // Actual conversions.
    declaration_1::WriteUnpinStruct(s.incomplete_cast(), 2);
    assert_eq!(definition::ReadUnpinStruct(s), 2);
    declaration_2::WriteUnpinStruct(s.incomplete_cast(), 2);
    assert_eq!(definition::ReadUnpinStruct(s), 2);
}

/// Given an incomplete UnpinStruct, all APIs accepting a (possibly
/// incomplete) UnpinStruct work (with an incomplete_cast()).
#[test]
fn test_read_incomplete_unpin() {
    let s = definition::UnpinStruct { field: 42 };
    let decl1_s: &declaration_1::UnpinStruct = (&s).incomplete_cast();

    // Cast from incomplete to complete:
    assert_eq!(definition::ReadUnpinStruct(decl1_s.incomplete_cast()), 42);

    // No cast necessary if it's the same forward declaration.
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 42);
    // Buit a self-cast also works:
    assert_eq!(declaration_1::ReadUnpinStruct(decl1_s.incomplete_cast()), 42);

    // Cast from incomplete to different-incomplete:
    assert_eq!(declaration_2::ReadUnpinStruct(decl1_s.incomplete_cast()), 42);
}

/// Given an incomplete UnpinStruct, all APIs accepting a (possibly
/// incomplete) mut UnpinStruct work (with an incomplete_cast()).
#[test]
fn test_write_incomplete_unpin() {
    let mut s = definition::UnpinStruct { field: 42 };
    let mut decl1_s: Pin<&mut declaration_1::UnpinStruct> = (&mut s).incomplete_cast();

    // Cast from incomplete to complete:
    definition::WriteUnpinStruct(decl1_s.as_mut().incomplete_cast(), 0);
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 0);

    // No cast necessary if it's the same forward declaration.
    declaration_1::WriteUnpinStruct(decl1_s.as_mut(), 1);
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 1);
    // But a self-cast also works.
    declaration_1::WriteUnpinStruct(decl1_s.as_mut(), 2);
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 2);

    // Cast from incomplete to different-incomplete:
    declaration_2::WriteUnpinStruct(decl1_s.as_mut().incomplete_cast(), 3);
    assert_eq!(declaration_1::ReadUnpinStruct(&*decl1_s), 3);
}

/// Given a complete NonunpinStruct, all APIs accepting a (possibly incomplete)
/// NonunpinStruct work (with an incomplete_cast()).
#[test]
fn test_read_complete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::NonunpinStruct {field: 42});
    }

    // The normal way to call it, if you have a complete type (and know it).
    assert_eq!(definition::ReadNonunpinStruct(&*s), 42);

    // Self-cast: if either the argument, or the parameter, was or will be
    // incomplete.
    assert_eq!(definition::ReadNonunpinStruct(s.as_ref().incomplete_cast()), 42);

    // Actual conversion.
    assert_eq!(declaration_1::ReadNonunpinStruct(s.as_ref().incomplete_cast()), 42);
    assert_eq!(declaration_2::ReadNonunpinStruct(s.as_ref().incomplete_cast()), 42);
}

/// Given a complete NonunpinStruct, all APIs accepting a (possibly incomplete)
/// mut NonunpinStruct work (with an incomplete_cast()).
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
    definition::WriteNonunpinStruct(s.as_mut().incomplete_cast(), 1);
    assert_eq!(definition::ReadNonunpinStruct(&*s), 1);

    // Actual conversions.
    declaration_1::WriteNonunpinStruct(s.as_mut().incomplete_cast(), 2);
    assert_eq!(definition::ReadNonunpinStruct(&*s), 2);
    declaration_2::WriteNonunpinStruct(s.as_mut().incomplete_cast(), 2);
    assert_eq!(definition::ReadNonunpinStruct(&*s), 2);
}

/// Given an incomplete NonunpinStruct, all APIs accepting a (possibly
/// incomplete) NonunpinStruct work (with an incomplete_cast()).
#[test]
fn test_read_incomplete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::NonunpinStruct {field: 42});
    }
    let mut decl1_s: Pin<&mut declaration_1::NonunpinStruct> = s.incomplete_cast();

    // Cast from incomplete to complete:
    assert_eq!(definition::ReadNonunpinStruct(decl1_s.as_ref().incomplete_cast()), 42);

    // No cast necessary if it's the same forward declaration.
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 42);
    // Buit a self-cast also works:
    assert_eq!(declaration_1::ReadNonunpinStruct(decl1_s.as_ref().incomplete_cast()), 42);

    // Cast from incomplete to different-incomplete:
    assert_eq!(declaration_2::ReadNonunpinStruct(decl1_s.as_ref().incomplete_cast()), 42);
}

/// Given an incomplete NonunpinStruct, all APIs accepting a (possibly
/// incomplete) mut NonunpinStruct work (with an incomplete_cast()).
#[test]
fn test_write_incomplete_nonunpin() {
    ctor::emplace! {
      let mut s = ctor::ctor!(definition::NonunpinStruct {field: 42});
    }
    let mut decl1_s: Pin<&mut declaration_1::NonunpinStruct> = s.incomplete_cast();

    // Cast from incomplete to complete:
    definition::WriteNonunpinStruct(decl1_s.as_mut().incomplete_cast(), 0);
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 0);

    // No cast necessary if it's the same forward declaration.
    declaration_1::WriteNonunpinStruct(decl1_s.as_mut(), 1);
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 1);
    // But a self-cast also works.
    declaration_1::WriteNonunpinStruct(decl1_s.as_mut(), 2);
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 2);

    // Cast from incomplete to different-incomplete:
    declaration_2::WriteNonunpinStruct(decl1_s.as_mut().incomplete_cast(), 3);
    assert_eq!(declaration_1::ReadNonunpinStruct(&*decl1_s), 3);
}
