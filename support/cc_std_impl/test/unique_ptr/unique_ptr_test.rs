// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(allocator_api)]

use cc_std::std::{unique_ptr, virtual_unique_ptr, Allocator};
use googletest::gtest;
use std::sync::atomic::{AtomicI32, Ordering};

static INSTANCE_COUNTER: AtomicI32 = AtomicI32::new(0);

/// A struct with the same size and alignment as `int`.
/// The number of alive instances is tracked by `INSTANCE_COUNTER`.
#[repr(transparent)]
#[derive(Debug, PartialEq)]
struct InstanceCounted(std::ffi::c_int);

impl Drop for InstanceCounted {
    fn drop(&mut self) {
        INSTANCE_COUNTER.fetch_sub(1, Ordering::Release);
    }
}

impl InstanceCounted {
    fn new() -> InstanceCounted {
        INSTANCE_COUNTER.fetch_add(1, Ordering::Acquire);
        InstanceCounted(123456)
    }

    fn new_unique_ptr() -> unique_ptr<InstanceCounted> {
        let mut int_sized_unique_ptr = test_helpers::unique_ptr_test::create_unique_ptr();
        let p = unique_ptr::release(&mut int_sized_unique_ptr) as *mut InstanceCounted;
        assert!(!p.is_null());
        unsafe {
            // SAFETY: The pointer is non-null and points to a valid initialized object.
            p.write(InstanceCounted::new());

            // SAFETY: The pointer is allocated by `new`, and the pointee is initialized.
            unique_ptr::from_raw(p)
        }
    }
}

#[track_caller]
fn assert_drop_decrements_counter<T>(up: T) {
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Acquire), 1);
    drop(up);
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Acquire), 0);
}

#[gtest]
fn test_unique_ptr_can_be_dropped() {
    let up = InstanceCounted::new_unique_ptr();
    assert_drop_decrements_counter(up);
}

#[gtest]
fn test_unique_ptr_can_be_dropped_as_box() {
    let b: Box<InstanceCounted, Allocator> =
        unique_ptr::into_box(InstanceCounted::new_unique_ptr());
    assert_drop_decrements_counter(b);
}

#[gtest]
fn test_unique_ptr_new_can_be_dropped() {
    let up = unique_ptr::new(InstanceCounted::new());
    assert_drop_decrements_counter(up);
}

#[gtest]
fn test_value_into_unique_ptr_can_be_dropped() {
    let up: unique_ptr<InstanceCounted> = InstanceCounted::new().into();
    assert_drop_decrements_counter(up);
}

#[gtest]
fn test_box_can_be_dropped_as_unique_ptr() {
    let up: unique_ptr<InstanceCounted> = Box::new_in(InstanceCounted::new(), Allocator).into();
    assert_drop_decrements_counter(up);
}

#[gtest]
fn test_unique_ptr_into_inner_can_be_dropped() {
    let up = unique_ptr::new(InstanceCounted::new());
    let v = unique_ptr::into_inner(up);
    assert_drop_decrements_counter(v);
}

#[gtest]
fn test_unique_ptr_get_returns_non_owned_pointer() {
    let up = InstanceCounted::new_unique_ptr();
    assert_eq!(unique_ptr::as_ptr(&up), unique_ptr::as_ptr(&up));
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Acquire), 1);
}

#[gtest]
fn test_unique_ptr_as_mut_null() {
    let mut up = unsafe { unique_ptr::<InstanceCounted>::from_raw(std::ptr::null_mut()) };
    assert_eq!(unique_ptr::as_pin(&mut up), None);
}

#[gtest]
fn test_unique_ptr_as_mut_non_null() {
    let mut up = InstanceCounted::new_unique_ptr();
    assert_eq!(unique_ptr::as_pin(&mut up).unwrap().0, 123456);
}

#[gtest]
fn test_unique_ptr_release_returns_owned_pointer() {
    let mut up = InstanceCounted::new_unique_ptr();
    let pointer = unique_ptr::as_mut_ptr(&mut up);
    let owned_pointer = unique_ptr::release(&mut up);
    assert_eq!(owned_pointer, pointer);
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Acquire), 1);

    // Consume the pointer.
    let up = unsafe { unique_ptr::from_raw(owned_pointer) };
    drop(up);
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Acquire), 0);
}

/// Tests the behavior when a unique_ptr created in C++ is destroyed in Rust.
///
/// For example, ASan can flag any poor behavior here.
#[gtest]
fn test_unique_ptr_destroyed_in_rust() {
    let up = test_helpers::unique_ptr_test::create_unique_ptr();
    drop(up);
}

#[gtest]
fn test_unique_ptr_void_ptr_destroyed_in_rust() {
    drop(test_helpers::unique_ptr_test::create_unique_ptr_void_ptr());
}

#[gtest]
fn test_unique_ptr_short_destroyed_in_rust() {
    drop(test_helpers::unique_ptr_test::create_unique_ptr_short());
}

#[gtest]
fn test_unique_ptr_two_words_destroyed_in_rust() {
    drop(test_helpers::unique_ptr_test::create_unique_ptr_two_words());
}

#[gtest]
fn test_unique_ptr_char_destroyed_in_rust() {
    drop(test_helpers::unique_ptr_test::create_unique_ptr_char());
}

/// Tests the behavior when a unique_ptr created in Rust is destroyed in C++.
///
/// For example, ASan can flag any poor behavior here.
#[gtest]
fn test_unique_ptr_destroyed_in_cpp() {
    let mut up = test_helpers::unique_ptr_test::create_unique_ptr();
    let up = unsafe { unique_ptr::from_raw(unique_ptr::release(&mut up)) };
    test_helpers::unique_ptr_test::destroy_unique_ptr(up);
}

#[gtest]
fn test_unique_ptr_with_virtual_destructor() {
    let mut p = test_helpers::unique_ptr_test::create_virtual_base();
    assert_eq!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<virtual_unique_ptr<test_helpers::unique_ptr_test::Base>>()
    );
    unsafe {
        assert!(test_helpers::unique_ptr_test::Base::is_derived(
            <std::pin::Pin<&mut _>>::into_inner_unchecked(
                virtual_unique_ptr::as_pin(&mut p).unwrap()
            )
        ));
    }
    drop(p);
    assert_eq!(test_helpers::unique_ptr_test::get_derived_destructor_count(), 1);
}

#[gtest]
fn test_unique_ptr_with_custom_delete() {
    let p = test_helpers::unique_ptr_test::create_custom_delete();
    assert_eq!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<
            cc_std::std::virtual_unique_ptr<test_helpers::unique_ptr_test::CustomDelete>,
        >()
    );
    drop(p);
    assert_eq!(test_helpers::unique_ptr_test::get_custom_delete_count(), 1);
}

#[gtest]
fn test_covariance() {
    fn _assert_unique_ptr_covariance<'a: 'b, 'b>(x: unique_ptr<&'a i32>) -> unique_ptr<&'b i32> {
        x
    }
}
