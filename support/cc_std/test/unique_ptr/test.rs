// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

struct InstanceCounted {
    counter: Rc<RefCell<i32>>,
}

impl Drop for InstanceCounted {
    fn drop(&mut self) {
        *self.counter.borrow_mut() -= 1;
    }
}

impl InstanceCounted {
    fn new(counter: Rc<RefCell<i32>>) -> InstanceCounted {
        *counter.borrow_mut() += 1;
        InstanceCounted { counter }
    }

    fn new_from_cpp_allocator(counter: Rc<RefCell<i32>>) -> *mut InstanceCounted {
        let buffer = cc_std::crubit_cc_std_internal::std_allocator::cpp_new_with_alignment(
            core::mem::size_of::<InstanceCounted>(),
            core::mem::align_of::<InstanceCounted>(),
        ) as *mut InstanceCounted;
        unsafe {
            buffer.write(InstanceCounted::new(counter));
        }
        buffer
    }
}

#[gtest]
fn test_unique_ptr_can_be_dropped() {
    let counter = Rc::new(RefCell::new(0i32));
    let up = unsafe {
        cc_std::std::unique_ptr::new(InstanceCounted::new_from_cpp_allocator(counter.clone()))
    };
    assert_eq!(*counter.borrow(), 1);
    drop(up);
    assert_eq!(*counter.borrow(), 0);
}

#[gtest]
fn test_unique_ptr_get_returns_non_owned_pointer() {
    let counter = Rc::new(RefCell::new(0i32));
    let pointer = InstanceCounted::new_from_cpp_allocator(counter.clone());
    let up = unsafe { cc_std::std::unique_ptr::new(pointer) };
    assert_eq!(up.get(), pointer);
    assert_eq!(*counter.borrow(), 1);
}

#[gtest]
fn test_unique_ptr_get_returns_owned_pointer() {
    let counter = Rc::new(RefCell::new(0i32));
    let pointer = InstanceCounted::new_from_cpp_allocator(counter.clone());
    let mut up = unsafe { cc_std::std::unique_ptr::new(pointer) };
    let owned_pointer = up.release();
    assert_eq!(owned_pointer, pointer);
    assert_eq!(*counter.borrow(), 1);

    // Consume the pointer.
    let up = unsafe { cc_std::std::unique_ptr::new(owned_pointer) };
    drop(up);
    assert_eq!(*counter.borrow(), 0);
}
