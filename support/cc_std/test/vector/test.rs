// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

#[gtest]
fn test_vector_new() {
    let v = vector::Vector::<i64>::new();
    expect_eq!(v.len(), 0);
    expect_eq!(v.capacity(), 0);
    expect_eq!(v.is_empty(), true);
}

#[gtest]
fn test_vector_push() {
    let mut v = vector::Vector::new();
    v.push(1);
    expect_eq!(v.len(), 1);
    expect_that!(v.capacity(), ge(1));
    expect_eq!(v.get(0), Some(&1));
    v.push(5);
    expect_eq!(v.len(), 2);
    expect_that!(v.capacity(), ge(2));
    expect_eq!(v.get(0), Some(&1));
    expect_eq!(v.get(1), Some(&5));
}

#[gtest]
fn test_vector_deref() {
    let mut v = vector::Vector::new();
    v.push(1);
    v.push(5);
    expect_eq!((*v)[0], 1);
    expect_eq!((*v)[1], 5);
}

#[gtest]
fn test_vector_get_range() {
    let mut v = vector::Vector::new();
    v.push(1);
    v.push(-2);
    expect_eq!(v.get_mut(0), Some(&mut 1));
    expect_eq!(v.get(0..2), Some(&[1, -2][..]));
}

#[gtest]
fn test_vector_deref_mut() {
    let mut v = vector::Vector::new();
    v.push(1);
    v.push(5);
    (*v)[0] = 2;
    expect_eq!((*v)[0], 2);
    (*v)[1] = 6;
    expect_eq!((*v)[1], 6);
}

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
}

#[gtest]
fn test_vector_drop() {
    let mut v = vector::Vector::new();
    let counter = Rc::new(RefCell::new(0i32));
    v.push(InstanceCounted::new(counter.clone()));
    v.push(InstanceCounted::new(counter.clone()));
    v.push(InstanceCounted::new(counter.clone()));
    expect_eq!(*counter.borrow(), 3);
    drop(v);
    expect_eq!(*counter.borrow(), 0);
}

#[gtest]
fn test_vector_index() {
    let mut v = vector::Vector::new();
    v.push(1);
    v.push(-3);
    expect_eq!(v[0], 1);
    expect_eq!(v[1], -3);
}

#[gtest]
#[should_panic]
fn test_vector_index_out_of_bounds() {
    let v = vector::Vector::<i64>::new();
    use_variable(v[0]);
}

#[gtest]
fn test_vector_mut_index() {
    let mut v = vector::Vector::new();
    v.push(1);
    v[0] = 5;
    expect_eq!(v[0], 5);
}

#[gtest]
fn test_vector_to_vec() {
    let mut v = vector::Vector::from(vec![4, 5, 6]);
    let v2 = v.to_vec();
    expect_eq!(v2, vec![4, 5, 6]);
}

#[gtest]
#[should_panic]
fn test_vector_mut_index_out_of_bounds() {
    let mut v = vector::Vector::new();
    v.push(1);
    v[1] = 10;
}

#[gtest]
fn test_vector_from_iter() {
    let v = vector::Vector::from_iter(std::iter::repeat(1).take(3));
    expect_eq!(v, [1, 1, 1]);
}

#[gtest]
fn test_emtpy_vector_into_iter() {
    let v = vector::Vector::<i64>::new();
    let mut sum = 0;
    for x in v {
        sum += x;
    }
    expect_eq!(sum, 0);
}

#[gtest]
fn test_vector_into_iter() {
    let mut v = vector::Vector::new();
    v.push(1);
    v.push(2);
    v.push(5);
    let mut sum = 0;
    for x in v {
        sum += x;
    }
    expect_eq!(sum, 8);
}

#[gtest]
fn test_vector_into_iter_size() {
    let mut v = vector::Vector::new();
    for i in 0..15 {
        v.push(i);
    }
    let mut iter = v.into_iter();
    expect_eq!(iter.size_hint(), (15, Option::Some(15)));
    iter.next();
    expect_eq!(iter.size_hint(), (14, Option::Some(14)));
    expect_eq!(iter.count(), 14);
}

#[gtest]
fn test_vector_into_iter_as_slice() {
    let mut v = vector::Vector::new();
    for i in 0..5 {
        v.push(i);
    }
    let mut iter = v.into_iter();
    expect_eq!(iter.as_slice(), &[0, 1, 2, 3, 4]);
    iter.next();
    expect_eq!(iter.as_slice(), &[1, 2, 3, 4]);
}

#[gtest]
fn test_vector_into_iter_as_mut_slice() {
    let mut v = vector::Vector::new();
    for i in 0..5 {
        v.push(i);
    }
    let mut iter = v.into_iter();
    expect_eq!(iter.as_mut_slice(), &mut [0, 1, 2, 3, 4]);
    iter.next();
    expect_eq!(iter.as_mut_slice(), &mut [1, 2, 3, 4]);
}

#[gtest]
fn test_vector_ref_iter() {
    let mut v = vector::Vector::new();
    v.push(1);
    v.push(5);
    v.push(10);
    let mut sum = 0;
    for x in &v {
        sum += x;
    }
    // Sum the elements in the vector 2nd time to ensure that the vector is still
    // valid.
    for x in &v {
        sum += x;
    }
    expect_eq!(sum, 32);
}

#[gtest]
fn test_vector_mut_ref_iter() {
    let mut v = vector::Vector::new();
    v.push(1);
    v.push(2);
    // Increment the elements in the vector (mut access).
    for x in &mut v {
        *x += 1;
    }
    // Check that the elements were incremented.
    let mut sum = 0;
    for x in &v {
        sum += x;
    }
    expect_eq!(sum, 5);
}

#[gtest]
fn test_iterator_as_ref() {
    let mut v = vector::Vector::new();
    for i in 0..5 {
        v.push(i);
    }
    let mut iter = v.into_iter();
    expect_eq!(iter.as_ref(), &[0, 1, 2, 3, 4]);
    iter.next();
    expect_eq!(iter.as_ref(), &[1, 2, 3, 4]);
}

#[gtest]
fn test_iterator_debug() {
    let mut v = vector::Vector::new();
    v.push(1);
    v.push(2);
    v.push(3);
    expect_eq!(format!("{:?}", v.into_iter()), "IntoIter([1, 2, 3])");
}

#[gtest]
fn test_iterator_default() {
    expect_eq!(
        format!("{:?}", <vector::Vector<i32> as IntoIterator>::IntoIter::default()),
        "IntoIter([])"
    );
}

#[gtest]
fn test_iterator_next_back() {
    let mut v = vector::Vector::new();
    v.push(1);
    v.push(2);
    let mut iter = v.into_iter();
    expect_eq!(iter.next_back(), Some(2));
    expect_eq!(iter.next_back(), Some(1));
    expect_eq!(iter.next_back(), None);
}

#[gtest]
fn test_iterator_drop() {
    let mut v = vector::Vector::new();
    v.push(10);
    let it = v.into_iter();
    drop(it)
}

/// Use a variable in a way that can't be optimized out.
fn use_variable<T>(v: T) {
    std::hint::black_box(v);
}

#[gtest]
fn test_vector_reserve() {
    let mut v = vector::Vector::<i32>::new();
    v.reserve(10);
    expect_that!(v.capacity(), ge(10));
}

#[gtest]
fn test_vector_with_capacity() {
    let v = vector::Vector::<i32>::with_capacity(100);
    expect_that!(v.capacity(), ge(100));
}

#[gtest]
fn test_vector_into_vec() {
    let mut v = vector::Vector::<i32>::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v2 = v.into_vec();
    expect_eq!(v2, vec![1, 2, 3]);
}
#[gtest]
fn test_vector_into_dropped_counter() {
    let mut v = vector::Vector::new();
    let counter = Rc::new(RefCell::new(0i32));
    v.push(InstanceCounted::new(counter.clone()));
    v.push(InstanceCounted::new(counter.clone()));
    v.push(InstanceCounted::new(counter.clone()));
    expect_eq!(*counter.borrow(), 3);
    let v2 = v.into_vec();
    drop(v2);
    expect_eq!(*counter.borrow(), 0);
}

#[gtest]
fn test_vector_from_vec() {
    let v = vector::Vector::from(vec![0, 1, 2, 3, 4]);
    expect_eq!(v.len(), 5);
    for i in 0..5 {
        expect_eq!(v[i], i);
    }
}

#[gtest]
fn test_vector_from_vec_with_capacity() {
    let mut vector = vector::Vector::from(vec![0, 1, 2, 3, 4]);
    let mut vec = Vec::from(vector);
    expect_eq!(vec, [0, 1, 2, 3, 4]);
}

mod layout_tests {
    use crate::to_void_ptr;
    use googletest::prelude::*;

    /// Tests that `Vector` has the same memory layout as `std::vector` in C++.
    #[gtest]
    fn test_begin_end() {
        let mut v = vector::Vector::<i32>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        unsafe {
            expect_eq!(cc_helper_functions::crubit_test::vector_int32_sum(to_void_ptr(&v)), 6);
        }
    }

    /// Tests that `Vector` has the same memory layout as `std::vector` in C++.
    #[gtest]
    fn test_capacity() {
        let mut v = vector::Vector::<i32>::new();
        for i in 0..100 {
            v.push(i);
        }
        unsafe {
            expect_eq!(
                cc_helper_functions::crubit_test::vector_int32_capacity(to_void_ptr(&v)),
                v.capacity(),
            );
        }
    }
}

mod allocation_tests {
    use crate::to_void_ptr;
    use googletest::prelude::*;
    use std::mem::forget;
    use std::mem::MaybeUninit;

    // The following tests check that the memory allocated in one language is
    // correctly reallocated and dealocated in another language.
    #[gtest]
    fn test_allocate_in_rust_deallocate_in_cc() {
        let mut v = vector::Vector::<i32>::new();
        // Allocate heap memory in Rust (by adding elements)
        for i in 0..10000 {
            v.push(i);
        }
        // Deallocate the heap memory in C++.
        unsafe {
            cc_helper_functions::crubit_test::vector_int32_call_destructor(to_void_ptr(&v));
        }
        forget(v);
    }

    #[gtest]
    fn test_allocate_in_cc_delocate_in_rust() {
        unsafe {
            // Allocate heap memory in C++ (by adding many elements)
            let maybe_uninit_v = MaybeUninit::<vector::Vector<i32>>::uninit();
            cc_helper_functions::crubit_test::vector_int32_construct(to_void_ptr(&maybe_uninit_v));
            let v = maybe_uninit_v.assume_init();
            expect_eq!(v.is_empty(), true);

            for i in 0..10000 {
                cc_helper_functions::crubit_test::vector_int32_push_back(to_void_ptr(&v), i);
            }

            expect_eq!(v.len(), 10000);
            // Deallocate the heap memory in Rust.
            drop(v)
        }
    }

    #[gtest]
    fn test_allocate_in_cc_interchangeble_reallocate_in_different_languages() {
        unsafe {
            let maybe_uninit_v = MaybeUninit::<vector::Vector<i32>>::uninit();
            cc_helper_functions::crubit_test::vector_int32_construct(to_void_ptr(&maybe_uninit_v));
            let mut v = maybe_uninit_v.assume_init();
            expect_eq!(v.is_empty(), true);

            // Allocate heap memory in C++.
            for i in 0..10 {
                cc_helper_functions::crubit_test::vector_int32_push_back(to_void_ptr(&v), i);
            }

            // Reallocate heap memory in Rust by adding much more elements.
            for i in 10..1000 {
                v.push(i);
            }

            // Reallocate heap memory in C++ by adding much much more elements.
            for i in 1000..100000 {
                cc_helper_functions::crubit_test::vector_int32_push_back(to_void_ptr(&v), i);
            }

            expect_eq!(v.len(), 100000);
            // Deallocate the heap memory in Rust.
            drop(v)
        }
    }

    #[gtest]
    fn test_allocate_in_rust_interchangeble_reallocate_in_different_languages() {
        let mut v = vector::Vector::<i32>::new();
        // Allocate heap memory in Rust (by adding elements)
        for i in 0..10 {
            v.push(i);
        }

        unsafe {
            // Reallocate heap memory in C++ by adding much more elements.
            for i in 10..1000 {
                cc_helper_functions::crubit_test::vector_int32_push_back(to_void_ptr(&v), i);
            }
        }
        // Reallocate heap memory in Rust (by adding much much more elements)
        for i in 1000..10000 {
            v.push(i);
        }
    }
}

mod alignment_tests {
    use googletest::prelude::*;
    // Tests that Vector can hold a struct with correct alignment.
    #[repr(align(1024))]
    struct OveralignedStuct {
        x: i32,
    }

    #[gtest]
    fn test_vector_of_overaligned_struct() {
        let mut v = vector::Vector::<OveralignedStuct>::new();
        v.push(OveralignedStuct { x: 1 });
        v.push(OveralignedStuct { x: 2 });
        expect_eq!(v[0].x, 1);
        expect_eq!(v[1].x, 2);
        expect_eq!(&raw const v[0] as usize % 1024, 0);
    }
}

fn to_void_ptr<T>(t: &T) -> *mut c_void {
    t as *const _ as *mut c_void
}
