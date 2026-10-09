// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::{
    unique_ptr, virtual_unique_ptr, Allocator, NonNull, TryDeref, TryDerefMut, TryDerefPin,
};
use googletest::{expect_eq, expect_true, gtest};
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
    let nn = NonNull::new(up).unwrap();
    let r: &i32 = &nn;
    expect_eq!(*r, 1);
    expect_eq!(*nn, 1);
}

#[gtest]
fn test_unique_ptr_deref_mut() {
    let up = test_helpers::unique_ptr_test::create_unique_ptr();
    let mut nn = NonNull::new(up).unwrap();
    *nn = 654321;
    expect_eq!(*nn, 654321);
}

#[gtest]
fn test_unique_ptr_non_null_ref_and_mut() {
    let mut up = test_helpers::unique_ptr_test::create_unique_ptr();
    expect_eq!(*up.try_deref().unwrap(), 1);

    let nn_ref = NonNull::from_ref(&up).unwrap();
    expect_eq!(**nn_ref, 1);

    *up.try_deref_mut().unwrap() = 42;
    expect_eq!(*up.try_deref().unwrap(), 42);

    let nn_mut = NonNull::from_mut(&mut up).unwrap();
    **nn_mut = 100;
    expect_eq!(**nn_mut, 100);
}

#[gtest]
fn test_unique_ptr_deref_pin() {
    let up = test_helpers::unique_ptr_test::create_unique_ptr();
    let mut nn = NonNull::new(up).unwrap();
    let mut pin = NonNull::deref_pin(&mut nn);
    *pin = 999;
    expect_eq!(*pin, 999);
}

#[gtest]
fn test_unique_ptr_null_returns_none() {
    let mut up = unsafe { unique_ptr::<i32>::from_raw(std::ptr::null_mut()) };
    expect_true!(up.try_deref().is_none());
    expect_true!(up.try_deref_mut().is_none());
    expect_true!(up.try_deref_pin().is_none());
    expect_true!(NonNull::from_ref(&up).is_none());
    expect_true!(NonNull::from_mut(&mut up).is_none());
    expect_true!(NonNull::new(up).is_none());
}

#[gtest]
fn test_virtual_unique_ptr_deref() {
    let p = test_helpers::unique_ptr_test::create_virtual_base();
    let nn = NonNull::new(p).unwrap();
    let r: &test_helpers::unique_ptr_test::Base = &nn;
    expect_true!(r.is_derived());
    expect_true!(nn.is_derived());
}

#[gtest]
fn test_unique_ptr_non_null_into() {
    let nn = NonNull::new(test_helpers::unique_ptr_test::create_unique_ptr()).unwrap();
    let r: &unique_ptr<i32> = (&nn).into();
    expect_eq!(*r.try_deref().unwrap(), 1);
    let up: unique_ptr<i32> = nn.into();
    expect_eq!(*up.try_deref().unwrap(), 1);
}

#[gtest]
fn test_virtual_unique_ptr_non_null_into() {
    let nn = NonNull::new(test_helpers::unique_ptr_test::create_virtual_base()).unwrap();
    let r: &virtual_unique_ptr<test_helpers::unique_ptr_test::Base> = (&nn).into();
    expect_true!(r.try_deref().unwrap().is_derived());
    let vp: virtual_unique_ptr<test_helpers::unique_ptr_test::Base> = nn.into();
    expect_true!(vp.try_deref().unwrap().is_derived());
}

#[gtest]
fn test_virtual_unique_ptr_null_returns_none() {
    let mut vp = unsafe {
        virtual_unique_ptr::<test_helpers::unique_ptr_test::CustomDelete>::from_raw(
            std::ptr::null_mut(),
        )
    };
    expect_true!(vp.try_deref().is_none());
    expect_true!(vp.try_deref_mut().is_none());
    expect_true!(vp.try_deref_pin().is_none());
    expect_true!(NonNull::from_ref(&vp).is_none());
    expect_true!(NonNull::from_mut(&mut vp).is_none());
    expect_true!(NonNull::new(vp).is_none());
}

#[gtest]
fn test_bare_unique_ptr_deref() {
    let up = test_helpers::unique_ptr_test::create_unique_ptr();
    let r: &i32 = &up;
    expect_eq!(*r, 1);
    expect_eq!(*up, 1);
}

#[gtest]
fn test_bare_unique_ptr_deref_mut() {
    let mut up = test_helpers::unique_ptr_test::create_unique_ptr();
    *up = 654321;
    expect_eq!(*up, 654321);
}

#[gtest]
#[should_panic(expected = "dereferencing a null unique_ptr")]
fn test_bare_unique_ptr_deref_null_panics() {
    // SAFETY: `from_raw` accepts a null pointer.
    let up = unsafe { unique_ptr::<i32>::from_raw(std::ptr::null_mut()) };
    let _value: i32 = *up;
}

#[gtest]
#[should_panic(expected = "dereferencing a null unique_ptr")]
fn test_bare_unique_ptr_deref_mut_null_panics() {
    // SAFETY: `from_raw` accepts a null pointer.
    let mut up = unsafe { unique_ptr::<i32>::from_raw(std::ptr::null_mut()) };
    *up = 1;
}

#[gtest]
fn test_unique_ptr_methods_do_not_shadow_pointee() {
    struct Pointee(i32);
    impl Pointee {
        fn get(&self) -> i32 {
            self.0
        }
        fn as_mut(&mut self) -> &mut i32 {
            &mut self.0
        }
    }
    let mut up = unique_ptr::new(Pointee(1));
    *up.as_mut() = 2;
    expect_eq!(up.get(), 2);
    expect_eq!(unique_ptr::get(&up), unique_ptr::as_ptr(&up).cast_mut());
    unique_ptr::as_mut(&mut up).0 = 3;
    expect_eq!(up.get(), 3);
}

/// A `virtual_unique_ptr` pointee defined in Rust, so tests using it don't touch the destructor
/// counters in `test_helpers`, which other tests running in parallel check.
#[derive(Debug, PartialEq)]
struct RustVirtual {
    val: i32,
}

// SAFETY: `delete` frees `p` with the C++ allocator after running `RustVirtual`'s destructor
// (which is trivial), which is exactly equivalent to C++ `delete p`.
unsafe impl cc_std::std::Delete for RustVirtual {
    /// # Safety
    ///
    /// `p` must point to a valid `RustVirtual` allocated with C++ `new` (e.g. by
    /// `unique_ptr::new`), and must not be used after this call.
    unsafe fn delete(p: *mut Self) {
        // SAFETY: by this function's precondition, `p` was allocated with the C++ allocator and
        // points to a valid `RustVirtual` that we now own.
        unsafe { drop(Box::from_raw_in(p, Allocator)) }
    }
}

fn new_rust_virtual(val: i32) -> virtual_unique_ptr<RustVirtual> {
    virtual_unique_ptr::from(RustVirtual { val })
}

#[gtest]
fn test_bare_virtual_unique_ptr_deref() {
    let mut vp = new_rust_virtual(1);
    let r: &RustVirtual = &vp;
    expect_eq!(r.val, 1);
    vp.val = 2;
    expect_eq!(vp.val, 2);
}

#[gtest]
#[should_panic(expected = "dereferencing a null virtual_unique_ptr")]
fn test_bare_virtual_unique_ptr_deref_null_panics() {
    // SAFETY: `from_raw` accepts a null pointer.
    let vp = unsafe {
        virtual_unique_ptr::<test_helpers::unique_ptr_test::CustomDelete>::from_raw(
            std::ptr::null_mut(),
        )
    };
    let _value: &test_helpers::unique_ptr_test::CustomDelete = &vp;
}

#[gtest]
#[should_panic(expected = "dereferencing a null virtual_unique_ptr")]
fn test_bare_virtual_unique_ptr_deref_mut_null_panics() {
    // SAFETY: `from_raw` accepts a null pointer.
    let mut vp = unsafe { virtual_unique_ptr::<RustVirtual>::from_raw(std::ptr::null_mut()) };
    vp.val = 1;
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
