// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(allocator_api)]

use cc_std::std::{unique_ptr, virtual_unique_ptr, Allocator};
use googletest::{expect_eq, expect_false, expect_true, gtest};
use std::sync::Arc;

#[track_caller]
fn assert_drop_decrements_counter<T>(rc: &Arc<()>, up: T) {
    assert_eq!(Arc::strong_count(rc), 2);
    drop(up);
    assert_eq!(Arc::strong_count(rc), 1);
}

#[gtest]
fn test_unique_ptr_can_be_dropped_as_box() {
    let rc = Arc::new(());
    let b: Box<Arc<()>, Allocator> = unique_ptr::into_box(unique_ptr::new(rc.clone()));
    assert_drop_decrements_counter(&rc, b);
}

#[gtest]
fn test_unique_ptr_new_can_be_dropped() {
    let rc = Arc::new(());
    let up = unique_ptr::new(rc.clone());
    assert_drop_decrements_counter(&rc, up);
}

#[gtest]
fn test_value_into_unique_ptr_can_be_dropped() {
    let rc = Arc::new(());
    let up: unique_ptr<Arc<()>> = rc.clone().into();
    assert_drop_decrements_counter(&rc, up);
}

#[gtest]
fn test_box_can_be_dropped_as_unique_ptr() {
    let rc = Arc::new(());
    let up: unique_ptr<Arc<()>> = Box::new_in(rc.clone(), Allocator).into();
    assert_drop_decrements_counter(&rc, up);
}

#[gtest]
fn test_unique_ptr_into_inner_can_be_dropped() {
    let rc = Arc::new(());
    let up = unique_ptr::new(rc.clone());
    let v = unique_ptr::into_inner(up);
    assert_drop_decrements_counter(&rc, v);
}

#[gtest]
fn test_unique_ptr_as_pin_null() {
    let mut up = unsafe { unique_ptr::<Arc<()>>::from_raw(std::ptr::null_mut()) };
    assert_eq!(unique_ptr::as_pin(&mut up), None);
}

#[gtest]
fn test_unique_ptr_as_pin_non_null() {
    let rc = Arc::new(());
    let mut up = unique_ptr::new(rc.clone());
    assert!(unique_ptr::as_pin(&mut up).is_some());
}

#[gtest]
fn test_unique_ptr_release_returns_owned_pointer() {
    let rc = Arc::new(());
    let mut up = unique_ptr::new(rc.clone());
    let pointer = unique_ptr::as_mut_ptr(&mut up);
    let owned_pointer = unique_ptr::release(&mut up);
    assert_eq!(owned_pointer, pointer);
    assert_eq!(Arc::strong_count(&rc), 2);

    // Consume the pointer.
    let up = unsafe { unique_ptr::from_raw(owned_pointer) };
    assert_drop_decrements_counter(&rc, up);
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

#[gtest]
fn test_unique_ptr_deref() {
    let up = test_helpers::unique_ptr_test::create_unique_ptr();
    let r: &i32 = &up;
    expect_eq!(*r, 1);
    expect_eq!(*up, 1);
}

#[gtest]
fn test_unique_ptr_deref_mut() {
    let mut up = test_helpers::unique_ptr_test::create_unique_ptr();
    *up = 654321;
    expect_eq!(*up, 654321);
}

#[gtest]
#[should_panic(expected = "dereferencing a null unique_ptr")]
fn test_unique_ptr_deref_null_panics() {
    let up = unsafe { unique_ptr::<i32>::from_raw(std::ptr::null_mut()) };
    let _ = *up;
}

#[gtest]
#[should_panic(expected = "dereferencing a null unique_ptr")]
fn test_unique_ptr_deref_mut_null_panics() {
    let mut up = unsafe { unique_ptr::<i32>::from_raw(std::ptr::null_mut()) };
    *up = 1;
}

#[gtest]
fn test_virtual_unique_ptr_deref() {
    let p = test_helpers::unique_ptr_test::create_virtual_base();
    let r: &test_helpers::unique_ptr_test::Base = &p;
    expect_true!(r.is_derived());
    expect_false!(virtual_unique_ptr::is_null(&p));
    expect_true!(p.is_derived());
}

#[gtest]
#[should_panic(expected = "dereferencing a null virtual_unique_ptr")]
fn test_virtual_unique_ptr_deref_null_panics() {
    let vp = unsafe {
        virtual_unique_ptr::<test_helpers::unique_ptr_test::CustomDelete>::from_raw(
            std::ptr::null_mut(),
        )
    };
    let _ = *vp;
}

#[gtest]
#[should_panic(expected = "dereferencing a null virtual_unique_ptr")]
fn test_virtual_unique_ptr_deref_mut_null_panics() {
    let mut vp = unsafe {
        virtual_unique_ptr::<test_helpers::unique_ptr_test::CustomDelete>::from_raw(
            std::ptr::null_mut(),
        )
    };
    let _mut_ref: &mut test_helpers::unique_ptr_test::CustomDelete = &mut *vp;
}

#[gtest]
fn test_unique_ptr_debug() {
    let up = unique_ptr::new(42);
    assert_eq!(format!("{up:?}"), "42");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Foo {
        a: i32,
    }
    let up_foo = unique_ptr::new(Foo { a: 123 });
    assert_eq!(format!("{up_foo:?}"), "Foo { a: 123 }");

    let null_up = unsafe { unique_ptr::<i32>::from_raw(std::ptr::null_mut()) };
    assert_eq!(format!("{null_up:?}"), "null");
    assert_eq!(format!("{null_up:>8?}"), "    null");
    assert_eq!(format!("{null_up:<8?}"), "null    ");
    assert_eq!(format!("{null_up:^8?}"), "  null  ");
    assert_eq!(format!("{null_up:_>8?}"), "____null");
}

#[gtest]
fn test_virtual_unique_ptr_debug() {
    #[derive(Debug)]
    struct VirtualFoo {
        #[allow(unused)]
        val: i32,
    }
    unsafe impl cc_std::std::Delete for VirtualFoo {
        unsafe fn delete(p: *mut Self) {
            unsafe { drop(Box::from_raw(p)) }
        }
    }

    let vp =
        unsafe { virtual_unique_ptr::from_raw(Box::into_raw(Box::new(VirtualFoo { val: 42 }))) };
    assert_eq!(format!("{vp:?}"), "VirtualFoo { val: 42 }");

    let null_vp = unsafe { virtual_unique_ptr::<VirtualFoo>::from_raw(std::ptr::null_mut()) };
    assert_eq!(format!("{null_vp:?}"), "null");
    assert_eq!(format!("{null_vp:>8?}"), "    null");
    assert_eq!(format!("{null_vp:<8?}"), "null    ");
    assert_eq!(format!("{null_vp:^8?}"), "  null  ");
    assert_eq!(format!("{null_vp:_>8?}"), "____null");
}
