// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::shared_ptr;
use googletest::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[gtest]
fn test_layout() {
    // Testing that the layout matches C++ shared_ptr<int32_t>
    expect_eq!(
        core::mem::size_of::<shared_ptr<i32>>(),
        test_helpers::shared_ptr_test::get_shared_ptr_size() as usize
    );
    expect_eq!(
        core::mem::align_of::<shared_ptr<i32>>(),
        test_helpers::shared_ptr_test::get_shared_ptr_alignment() as usize
    );
}

#[gtest]
fn test_polymorphic_destructor() {
    let mut count = 0;
    {
        let _shared_base =
            unsafe { test_helpers::shared_ptr_test::create_virtual_base(&mut count) };
        expect_eq!(core::mem::size_of_val(&_shared_base), 16);
    }
    // After dropping shared_ptr<Base> which points to Derived, count should be 1.
    expect_eq!(count, 1);
}

#[gtest]
fn test_deref() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr();
    expect_eq!(*shared_ptr::try_as_ref(&shared).unwrap(), 1);
}

#[gtest]
fn test_as_ptr_and_is_null() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr();
    expect_false!(shared_ptr::is_null(&shared));
    expect_eq!(unsafe { *shared_ptr::as_ptr(&shared) }, 1);

    let null_shared = test_helpers::shared_ptr_test::create_shared_ptr_void_ptr();
    expect_true!(shared_ptr::is_null(&null_shared));
}

#[gtest]
fn test_release() {
    let mut shared = test_helpers::shared_ptr_test::create_shared_ptr();
    expect_false!(shared_ptr::is_null(&shared));
    let (ptr, cntrl) = shared_ptr::release(&mut shared);
    expect_true!(shared_ptr::is_null(&shared));
    expect_eq!(unsafe { *ptr }, 1);
    // Re-wrap and drop to clean up reference count.
    let _rewrapped = unsafe { shared_ptr::from_raw_parts(ptr, cntrl) };
}

#[gtest]
fn test_into_raw_parts() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr();
    let (ptr, cntrl) = shared_ptr::into_raw_parts(shared);
    expect_eq!(unsafe { *ptr }, 1);
    // Re-wrap and drop to clean up reference count.
    let _rewrapped = unsafe { shared_ptr::from_raw_parts(ptr, cntrl) };
}

#[gtest]
fn test_const_shared_ptr_created_in_cpp() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr_const();
    expect_eq!(*shared_ptr::try_as_ref(&shared).unwrap(), 1);
    // Rust ignores const on T, so a shared_ptr<const int> from C++ can be passed
    // to a C++ function taking shared_ptr<int>.
    test_helpers::shared_ptr_test::destroy_shared_ptr(shared);
}

#[gtest]
fn test_non_const_shared_ptr_passed_to_const_param() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr();
    expect_eq!(*shared_ptr::try_as_ref(&shared).unwrap(), 1);
    // Rust ignores const on T, so a shared_ptr<int> from C++ can be passed
    // to a C++ function taking shared_ptr<const int>.
    test_helpers::shared_ptr_test::destroy_shared_ptr_const(shared);
}

#[gtest]
fn test_const_shared_ptr_passed_to_const_param() {
    let shared = test_helpers::shared_ptr_test::create_shared_ptr_const();
    test_helpers::shared_ptr_test::destroy_shared_ptr_const(shared);
}

#[gtest]
fn test_use_count_null() {
    let null_shared = test_helpers::shared_ptr_test::create_shared_ptr_void_ptr();
    expect_eq!(shared_ptr::use_count(&null_shared), 0);
}

#[gtest]
fn test_use_count() {
    let sp = test_helpers::shared_ptr_test::create_shared_ptr();
    expect_eq!(shared_ptr::use_count(&sp), 1);
    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp), 2);
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);
    drop(sp_clone);
    expect_eq!(shared_ptr::use_count(&sp), 1);
}

#[gtest]
fn test_new() {
    let sp = shared_ptr::new(42);
    expect_eq!(*shared_ptr::try_as_ref(&sp).unwrap(), 42);
    expect_eq!(shared_ptr::use_count(&sp), 1);

    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp), 2);
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);

    drop(sp);
    expect_eq!(shared_ptr::use_count(&sp_clone), 1);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);
}

#[gtest]
fn test_new_destroyed_in_cpp() {
    let sp = shared_ptr::new(42);
    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);

    test_helpers::shared_ptr_test::destroy_shared_ptr(sp);

    expect_eq!(shared_ptr::use_count(&sp_clone), 1);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);
}

struct DropDetector(Arc<AtomicUsize>);

impl Drop for DropDetector {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[gtest]
fn test_new_custom_drop() {
    let counter = Arc::new(AtomicUsize::new(0));
    {
        let _sp = shared_ptr::new(DropDetector(Arc::clone(&counter)));
        expect_eq!(counter.load(Ordering::SeqCst), 0);
    }
    expect_eq!(counter.load(Ordering::SeqCst), 1);
}

#[gtest]
fn test_new_sole_owner_destroyed_in_cpp() {
    let sp = shared_ptr::new(42);
    test_helpers::shared_ptr_test::destroy_shared_ptr(sp);
}

#[gtest]
fn test_from_box() {
    let b = Box::new(42);
    let sp = shared_ptr::from_box(b);
    expect_eq!(*shared_ptr::try_as_ref(&sp).unwrap(), 42);
    expect_eq!(shared_ptr::use_count(&sp), 1);

    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp), 2);
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);

    drop(sp);
    expect_eq!(shared_ptr::use_count(&sp_clone), 1);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);
}

#[gtest]
fn test_from_box_destroyed_in_cpp() {
    let b = Box::new(42);
    let sp = shared_ptr::from_box(b);
    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);

    test_helpers::shared_ptr_test::destroy_shared_ptr(sp);

    expect_eq!(shared_ptr::use_count(&sp_clone), 1);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);
}

#[gtest]
fn test_from_box_custom_drop() {
    let counter = Arc::new(AtomicUsize::new(0));
    {
        let b = Box::new(DropDetector(Arc::clone(&counter)));
        let _sp = shared_ptr::from_box(b);
        expect_eq!(counter.load(Ordering::SeqCst), 0);
    }
    expect_eq!(counter.load(Ordering::SeqCst), 1);
}

#[gtest]
fn test_from_box_sole_owner_destroyed_in_cpp() {
    let b = Box::new(42);
    let sp = shared_ptr::from_box(b);
    test_helpers::shared_ptr_test::destroy_shared_ptr(sp);
}

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[gtest]
fn test_project() {
    let sp = shared_ptr::new(Point { x: 10, y: 20 });
    let sp_x: shared_ptr<i32> = shared_ptr::project(sp, |p| &p.x);
    expect_eq!(*shared_ptr::try_as_ref(&sp_x).unwrap(), 10);
    expect_eq!(shared_ptr::use_count(&sp_x), 1);
}

#[gtest]
fn test_from_arc() {
    let a = Arc::new(42);
    let sp = shared_ptr::from_arc(a);
    expect_eq!(*shared_ptr::try_as_ref(&sp).unwrap(), 42);
    expect_eq!(shared_ptr::use_count(&sp), 1);

    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp), 2);
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);

    drop(sp);
    expect_eq!(shared_ptr::use_count(&sp_clone), 1);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);
}

#[gtest]
fn test_from_pinned_box() {
    let b = Box::pin(42);
    let sp = shared_ptr::from_pinned_box(b);
    expect_eq!(*shared_ptr::try_as_ref(&sp).unwrap(), 42);
    expect_eq!(shared_ptr::use_count(&sp), 1);

    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp), 2);
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);

    drop(sp);
    expect_eq!(shared_ptr::use_count(&sp_clone), 1);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);
}

#[gtest]
fn test_from_pinned_box_destroyed_in_cpp() {
    let b = Box::pin(42);
    let sp = shared_ptr::from_pinned_box(b);
    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);

    test_helpers::shared_ptr_test::destroy_shared_ptr(sp);

    expect_eq!(shared_ptr::use_count(&sp_clone), 1);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);
}

#[gtest]
fn test_from_pinned_box_custom_drop() {
    let counter = Arc::new(AtomicUsize::new(0));
    {
        let b = Box::pin(DropDetector(Arc::clone(&counter)));
        let _sp = shared_ptr::from_pinned_box(b);
        expect_eq!(counter.load(Ordering::SeqCst), 0);
    }
    expect_eq!(counter.load(Ordering::SeqCst), 1);
}

#[gtest]
fn test_from_pinned_box_sole_owner_destroyed_in_cpp() {
    let b = Box::pin(42);
    let sp = shared_ptr::from_pinned_box(b);
    test_helpers::shared_ptr_test::destroy_shared_ptr(sp);
}

struct NonUnpinType {
    val: i32,
    _pin: core::marker::PhantomPinned,
}

#[gtest]
fn test_from_pinned_box_non_unpin() {
    let b = Box::pin(NonUnpinType { val: 123, _pin: core::marker::PhantomPinned });
    let sp = shared_ptr::from_pinned_box(b);
    expect_eq!(shared_ptr::try_as_ref(&sp).unwrap().val, 123);
    expect_eq!(shared_ptr::use_count(&sp), 1);
}

#[gtest]
fn test_ptr_eq() {
    let sp1 = shared_ptr::new(Point { x: 10, y: 20 });
    let sp1_clone = sp1.clone();
    let sp2 = shared_ptr::new(Point { x: 10, y: 20 });
    let sp1_x: shared_ptr<i32> = shared_ptr::project(sp1.clone(), |p| &p.x);

    // Same control block
    expect_true!(shared_ptr::owner_equal(&sp1, &sp1_clone));
    // Different types sharing the same control block via projection
    expect_true!(shared_ptr::owner_equal(&sp1, &sp1_x));
    expect_true!(shared_ptr::owner_equal(&sp1_clone, &sp1_x));

    // Different allocations
    expect_false!(shared_ptr::owner_equal(&sp1, &sp2));
    expect_false!(shared_ptr::owner_equal(&sp1_x, &sp2));
}

#[gtest]
fn test_from_unique_ptr() {
    let u = test_helpers::shared_ptr_test::create_unique_ptr();
    let sp = shared_ptr::from_unique_ptr(u);
    expect_false!(shared_ptr::is_null(&sp));
    expect_eq!(*shared_ptr::try_as_ref(&sp).unwrap(), 42);
    expect_eq!(shared_ptr::use_count(&sp), 1);

    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp), 2);
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);

    drop(sp);
    expect_eq!(shared_ptr::use_count(&sp_clone), 1);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);
}

#[gtest]
fn test_from_unique_ptr_null() {
    let u = test_helpers::shared_ptr_test::create_null_unique_ptr();
    let sp = shared_ptr::from_unique_ptr(u);
    expect_true!(shared_ptr::is_null(&sp));
    expect_eq!(shared_ptr::use_count(&sp), 0);
}

#[gtest]
fn test_from_unique_ptr_destroyed_in_cpp() {
    let u = test_helpers::shared_ptr_test::create_unique_ptr();
    let sp = shared_ptr::from_unique_ptr(u);
    let sp_clone = sp.clone();
    expect_eq!(shared_ptr::use_count(&sp_clone), 2);

    test_helpers::shared_ptr_test::destroy_shared_ptr(sp);

    expect_eq!(shared_ptr::use_count(&sp_clone), 1);
    expect_eq!(*shared_ptr::try_as_ref(&sp_clone).unwrap(), 42);
}

#[gtest]
fn test_from_virtual_unique_ptr() {
    let mut count = 0;
    {
        let u = unsafe { test_helpers::shared_ptr_test::create_virtual_unique_base(&mut count) };
        let sp = shared_ptr::from_virtual_unique_ptr(u);
        expect_false!(shared_ptr::is_null(&sp));
        expect_eq!(shared_ptr::use_count(&sp), 1);

        let sp_clone = sp.clone();
        expect_eq!(shared_ptr::use_count(&sp), 2);
        expect_eq!(shared_ptr::use_count(&sp_clone), 2);
        drop(sp);
        expect_eq!(shared_ptr::use_count(&sp_clone), 1);
        expect_eq!(count, 0);
    }
    expect_eq!(count, 1);
}

#[gtest]
fn test_from_virtual_unique_ptr_null() {
    let u = test_helpers::shared_ptr_test::create_null_virtual_unique_base();
    let sp = shared_ptr::from_virtual_unique_ptr(u);
    expect_true!(shared_ptr::is_null(&sp));
    expect_eq!(shared_ptr::use_count(&sp), 0);
}
