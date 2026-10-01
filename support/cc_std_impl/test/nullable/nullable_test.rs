// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::{
    shared_ptr, unique_ptr, virtual_unique_ptr, Delete, Nullable, OptionLike, SupportsNullable,
};
use googletest::{expect_eq, expect_true, gtest};
use std::sync::Arc;

#[derive(Debug)]
struct VirtualFoo {
    val: i32,
}

// SAFETY: `VirtualFoo`s are only ever allocated by `Box::new` in this test.
unsafe impl Delete for VirtualFoo {
    unsafe fn delete(p: *mut Self) {
        // SAFETY: `p` was allocated by `Box::new`.
        unsafe { drop(Box::from_raw(p)) }
    }
}

fn new_virtual_foo(val: i32) -> virtual_unique_ptr<VirtualFoo> {
    // SAFETY: `VirtualFoo::delete` frees with `Box`.
    unsafe { virtual_unique_ptr::from_raw(Box::into_raw(Box::new(VirtualFoo { val }))) }
}

#[gtest]
fn test_layout_matches_pointer() {
    expect_eq!(size_of::<Nullable<unique_ptr<i32>>>(), size_of::<unique_ptr<i32>>());
    expect_eq!(align_of::<Nullable<unique_ptr<i32>>>(), align_of::<unique_ptr<i32>>());
    expect_eq!(size_of::<Nullable<shared_ptr<i32>>>(), size_of::<shared_ptr<i32>>());
    expect_eq!(align_of::<Nullable<shared_ptr<i32>>>(), align_of::<shared_ptr<i32>>());
    expect_eq!(
        size_of::<Nullable<virtual_unique_ptr<VirtualFoo>>>(),
        size_of::<virtual_unique_ptr<VirtualFoo>>()
    );
}

#[gtest]
fn test_null() {
    expect_true!(<unique_ptr<i32> as SupportsNullable>::is_null(&SupportsNullable::null()));
    expect_true!(<shared_ptr<i32> as SupportsNullable>::is_null(&SupportsNullable::null()));
    expect_true!(<virtual_unique_ptr<VirtualFoo> as SupportsNullable>::is_null(
        &SupportsNullable::null()
    ));
}

#[gtest]
fn test_default_is_null() {
    expect_true!(Nullable::<unique_ptr<i32>>::default().into_option().is_none());
    expect_true!(Nullable::<shared_ptr<i32>>::default().into_option().is_none());
    expect_true!(Nullable::<virtual_unique_ptr<VirtualFoo>>::default().into_option().is_none());
}

#[gtest]
fn test_into_option() {
    let p: Nullable<unique_ptr<i32>> = unique_ptr::new(1).into();
    let p: unique_ptr<i32> = p.into_option().unwrap();
    expect_eq!(unsafe { *unique_ptr::as_ptr(&p) }, 1);

    let p: Nullable<shared_ptr<i32>> = shared_ptr::new(2).into();
    expect_eq!(shared_ptr::try_as_ref(&p.into_option().unwrap()), Some(&2));

    let p: Nullable<virtual_unique_ptr<VirtualFoo>> = new_virtual_foo(3).into();
    let p = p.into_option().unwrap();
    expect_eq!(unsafe { (*virtual_unique_ptr::as_ptr(&p)).val }, 3);
}

#[gtest]
fn test_as_option() {
    let mut p: Nullable<shared_ptr<i32>> = shared_ptr::new(1).into();
    expect_eq!(p.as_option().and_then(shared_ptr::try_as_ref), Some(&1));
    expect_true!(p.as_option_mut().is_some());

    let mut null = Nullable::<shared_ptr<i32>>::default();
    expect_true!(null.as_option().is_none());
    expect_true!(null.as_option_mut().is_none());
}

/// `OptionLike` is implemented by the bare pointers too, so that code converting a `Nullable<Ptr>`
/// to an `Option` keeps compiling when C++ adds `absl_nonnull` and the type becomes `Ptr`.
#[gtest]
fn test_option_like_on_bare_pointers() {
    let mut p = unique_ptr::new(1);
    expect_true!(p.as_option().is_some());
    expect_true!(p.as_option_mut().is_some());
    expect_true!(p.into_option().is_some());

    let p = shared_ptr::new(2);
    expect_eq!(p.as_option().and_then(shared_ptr::try_as_ref), Some(&2));
    expect_true!(p.into_option().is_some());

    let p = new_virtual_foo(3);
    expect_true!(p.into_option().is_some());

    // A bare pointer can still be null, e.g. when moved from in C++, or constructed by unsafe code.
    // SAFETY: `from_raw` accepts a null pointer.
    let mut null: unique_ptr<i32> = unsafe { unique_ptr::from_raw(std::ptr::null_mut()) };
    expect_true!(null.as_option().is_none());
    expect_true!(null.as_option_mut().is_none());
    expect_true!(null.into_option().is_none());
}

#[gtest]
fn test_from_ref() {
    let p = shared_ptr::new(1);
    let nullable: &Nullable<shared_ptr<i32>> = (&p).into();
    expect_eq!(nullable.as_option().and_then(shared_ptr::try_as_ref), Some(&1));
}

#[gtest]
fn test_debug() {
    let p: Nullable<unique_ptr<i32>> = unique_ptr::new(42).into();
    expect_eq!(format!("{p:?}"), "42");

    let null = Nullable::<unique_ptr<i32>>::default();
    expect_eq!(format!("{null:?}"), "null");
    expect_eq!(format!("{null:>8?}"), "    null");
}

#[gtest]
fn test_drop() {
    let rc = Arc::new(());
    let p: Nullable<unique_ptr<Arc<()>>> = unique_ptr::new(rc.clone()).into();
    expect_eq!(Arc::strong_count(&rc), 2);
    drop(p);
    expect_eq!(Arc::strong_count(&rc), 1);

    let p: Nullable<shared_ptr<Arc<()>>> = shared_ptr::new(rc.clone()).into();
    expect_eq!(Arc::strong_count(&rc), 2);
    drop(p);
    expect_eq!(Arc::strong_count(&rc), 1);
}
