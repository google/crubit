// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
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

    fn new_unique_ptr() -> cc_std::std::unique_ptr<InstanceCounted> {
        let mut int_sized_unique_ptr = test_helpers::unique_ptr_test::create_unique_ptr();
        let p = int_sized_unique_ptr.release() as *mut InstanceCounted;
        unsafe {
            assert!(!p.is_null());
            // SAFETY: The pointer is non-null and points to a valid initialized object.
            p.write(InstanceCounted::new());

            // SAFETY: The pointer is allocated by `new`, and the pointee is initialized.
            <cc_std::std::unique_ptr<InstanceCounted>>::new(p)
        }
    }
}

#[gtest]
fn test_unique_ptr_can_be_dropped() {
    let up = InstanceCounted::new_unique_ptr();
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Acquire), 1);
    drop(up);
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Acquire), 0);
}

#[gtest]
fn test_unique_ptr_get_returns_non_owned_pointer() {
    let up = InstanceCounted::new_unique_ptr();
    assert_eq!(up.get(), up.get());
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Acquire), 1);
}

#[gtest]
fn test_unique_ptr_as_mut_null() {
    let mut up = unsafe { cc_std::std::unique_ptr::<InstanceCounted>::new(std::ptr::null_mut()) };
    assert_eq!(up.as_mut(), None);
}

#[gtest]
fn test_unique_ptr_as_mut_non_null() {
    let mut up = InstanceCounted::new_unique_ptr();
    assert_eq!(up.as_mut().unwrap().0, 123456);
}

#[gtest]
fn test_unique_ptr_release_returns_owned_pointer() {
    let mut up = InstanceCounted::new_unique_ptr();
    let pointer = up.get();
    let owned_pointer = up.release();
    assert_eq!(owned_pointer, pointer);
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Acquire), 1);

    // Consume the pointer.
    let up = unsafe { cc_std::std::unique_ptr::new(owned_pointer) };
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

/// Tests the behavior when a unique_ptr created in Rust is destroyed in C++.
///
/// For example, ASan can flag any poor behavior here.
#[gtest]
fn test_unique_ptr_destroyed_in_cpp() {
    let mut up = test_helpers::unique_ptr_test::create_unique_ptr();
    let up = unsafe { cc_std::std::unique_ptr::new(up.release()) };
    test_helpers::unique_ptr_test::destroy_unique_ptr(up);
}

#[gtest]
fn test_unique_ptr_with_virtual_destructor() {
    let p = test_helpers::unique_ptr_test::create_virtual_base();
    assert_eq!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<cc_std::std::unique_ptr_dyn<test_helpers::unique_ptr_test::Base>>()
    );
    drop(p);
    assert_eq!(test_helpers::unique_ptr_test::get_derived_destructor_count(), 1);
}

#[gtest]
fn test_unique_ptr_with_custom_delete() {
    let p = test_helpers::unique_ptr_test::create_custom_delete();
    assert_eq!(
        std::any::Any::type_id(&p),
        std::any::TypeId::of::<
            cc_std::std::unique_ptr_dyn<test_helpers::unique_ptr_test::CustomDelete>,
        >()
    );
    drop(p);
    assert_eq!(test_helpers::unique_ptr_test::get_custom_delete_count(), 1);
}
