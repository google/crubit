// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use std::cell::RefCell;
use std::ffi::c_void;
use std::hash::BuildHasher;
use std::rc::Rc;

#[gtest]
fn test_vector_new() {
    let v = vector::Vector::<i64>::new();
    expect_eq!(v.len(), 0);
    expect_eq!(v.capacity(), 0);
    expect_eq!(v.is_empty(), true);
}

#[gtest]
fn test_set_len() {
    let mut v = vector::Vector::<isize>::with_capacity(10);
    unsafe {
        let p = v.as_mut_ptr();
        v.prepare_to_write_into_tail();
        for i in 0..3 {
            std::ptr::write(p.offset(i), i);
        }
        v.set_len(3);
    }
    expect_eq!(v, [0, 1, 2]);
    expect_that!(v.capacity(), ge(10));

    unsafe {
        v.set_len(0);
    }
    expect_eq!(v, []);
    expect_that!(v.capacity(), ge(10));
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
    let v = vector::Vector::from(vec![1, 2, 3, 4]);
    expect_eq!(v[0], 1);
    expect_eq!(v[1], 2);
    expect_eq!(v[..], [1, 2, 3, 4]);
    expect_eq!(v[1..], [2, 3, 4]);
    expect_eq!(v[1..2], [2]);
}

#[gtest]
#[should_panic]
fn test_vector_index_out_of_bounds() {
    let v = vector::Vector::<i64>::new();
    use_variable(v[0]);
}

#[gtest]
fn test_vector_mut_index() {
    let mut v = vector::Vector::from(vec![1, 2, 3, 4]);
    v[0] = 5;
    expect_eq!(v, vec![5, 2, 3, 4]);
    v[1..3][0] = 6;
    expect_eq!(v, vec![5, 6, 3, 4]);
}

#[gtest]
fn test_vector_to_vec() {
    let v = vector::Vector::from(vec![4, 5, 6]);
    let v2 = v.to_vec();
    expect_eq!(v2, vec![4, 5, 6]);
}

#[gtest]
fn test_clone() {
    let v = vector::Vector::from(vec![4, 5, 6]);
    let v2 = v.clone();
    expect_eq!(v2, vec![4, 5, 6]);
}

struct CloneCounter {
    counter: Rc<RefCell<i32>>,
}

impl Clone for CloneCounter {
    fn clone(&self) -> Self {
        *self.counter.borrow_mut() += 1;
        CloneCounter { counter: self.counter.clone() }
    }
}

#[gtest]
fn test_clone_once_clone() {
    let counter = Rc::new(RefCell::new(0i32));
    let v = vector::Vector::from(vec![CloneCounter { counter: counter.clone() }]);
    expect_eq!(*counter.borrow(), 0);
    let _ = v.clone();
    expect_eq!(*counter.borrow(), 1); // the element was cloned once.
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
    let current_capacity = v.capacity();
    v.reserve(5); // 5 < 10, so it should do nothing.
    expect_eq!(v.capacity(), current_capacity);
}

#[gtest]
fn test_vector_reserve_exact() {
    let mut v = vector::Vector::<usize>::new();
    v.reserve_exact(10);
    expect_that!(v.capacity(), ge(10));
    let current_capacity = v.capacity();
    // Add elements to the current capacity.
    for i in v.len()..current_capacity {
        v.push(i);
    }
    v.reserve_exact(5); // it ensures that the capacity for additional 5 elements.
    expect_that!(v.capacity(), ge(current_capacity + 5));
}

#[gtest]
fn test_vector_try_reserve() {
    let mut v = vector::Vector::<i32>::new();
    v.try_reserve(100).unwrap();
    expect_that!(v.capacity(), ge(100));
}

#[gtest]
fn test_vector_try_reserve_exact() {
    let mut v = vector::Vector::<usize>::new();
    v.try_reserve_exact(10).unwrap();
    expect_that!(v.capacity(), ge(10));
    let current_capacity = v.capacity();
    // Add elements to the current capacity.
    for i in v.len()..current_capacity {
        v.push(i);
    }
    v.try_reserve_exact(5).unwrap(); // it ensures that the capacity for additional 5 elements.
    expect_that!(v.capacity(), ge(current_capacity + 5));
}

#[gtest]
fn test_shrink_to_fit() {
    let mut v = vector::Vector::<i32>::new();
    for i in 0..100 {
        v.push(i);
    }
    v.clear(); // doesn't shrink the capacity.
    v.shrink_to_fit();
    expect_eq!(v.capacity(), 0);
}

#[gtest]
fn test_shrink_to() {
    let mut v = vector::Vector::<i32>::new();
    for i in 0..100 {
        v.push(i);
    }
    v.clear(); // doesn't shrink the capacity.
    v.shrink_to(10);
    expect_that!(v.capacity(), le(99));
    expect_that!(v.capacity(), ge(10));
}

#[gtest]
fn test_vector_with_capacity() {
    let v = vector::Vector::<i32>::with_capacity(100);
    expect_that!(v.capacity(), ge(100));
}

#[gtest]
fn test_vector_insert() {
    let mut v = vector::Vector::<i32>::new();
    v.insert(0, 1);
    v.insert(0, 2);
    expect_eq!(v, [2, 1]);
    v.insert(1, 3);
    expect_eq!(v, [2, 3, 1]);
}

#[gtest]
fn test_vector_append() {
    let mut v = vector::Vector::from(vec![1, 2]);
    let mut v2 = vector::Vector::from(vec![3, 4]);
    v.append(&mut v2);
    expect_eq!(v, [1, 2, 3, 4]);
    expect_eq!(v2, []);
    expect_eq!(v2.is_empty(), true);
}

#[gtest]
fn test_swap_remove() {
    let mut v = vector::Vector::from(vec![1, 2, 3, 4]);
    expect_eq!(v.swap_remove(1), 2);
    expect_eq!(v, [1, 4, 3]);
    expect_eq!(v.swap_remove(0), 1);
    expect_eq!(v, [3, 4]);
    expect_eq!(v.swap_remove(0), 3);
    expect_eq!(v, [4]);
    expect_eq!(v.swap_remove(0), 4);
    expect_eq!(v.is_empty(), true);
}

#[gtest]
#[should_panic]
fn test_swap_remove_out_of_bounds() {
    let mut v = vector::Vector::from(vec![1, 2, 3, 4]);
    v.swap_remove(4);
}

#[gtest]
fn test_remove() {
    let mut v = vector::Vector::from(vec![1, 2, 3, 4]);
    expect_eq!(v.remove(1), 2);
    expect_eq!(v, [1, 3, 4]);
    expect_eq!(v.remove(0), 1);
    expect_eq!(v, [3, 4]);
    expect_eq!(v.remove(1), 4);
    expect_eq!(v, [3]);
    expect_eq!(v.remove(0), 3);
    expect_eq!(v.is_empty(), true);
}

#[gtest]
#[should_panic]
fn test_remove_out_of_bounds() {
    let mut v = vector::Vector::from(vec![1, 2, 3]);
    v.remove(3);
}

#[gtest]
fn test_pop() {
    let mut v = vector::Vector::from(vec![1, 2, 3, 4]);
    expect_eq!(v.pop(), Some(4));
    expect_eq!(v, [1, 2, 3]);
    expect_eq!(v.pop(), Some(3));
    expect_eq!(v, [1, 2]);
    expect_eq!(v.pop(), Some(2));
    expect_eq!(v, [1]);
    expect_eq!(v.pop(), Some(1));
    expect_eq!(v.is_empty(), true);
    expect_eq!(v.pop(), None);
}

#[gtest]
fn test_vector_clear() {
    let mut v = vector::Vector::from(vec![1, 2, 3, 4]);
    v.clear();
    expect_eq!(v.is_empty(), true);
    expect_eq!(v.capacity(), 4);
}

#[gtest]
fn test_truncate() {
    let mut v = vector::Vector::from(vec![1, 2, 3, 4, 5]);
    let capacity = v.capacity();
    v.truncate(2);
    expect_eq!(v, [1, 2]);
    expect_eq!(v.capacity(), capacity);
    v.truncate(3); // no effect
    expect_eq!(v, [1, 2]);

    v.truncate(0);
    expect_eq!(v, []);
    expect_eq!(v.capacity(), capacity);
}

#[gtest]
fn test_vector_resize() {
    let mut v = vector::Vector::from(vec![1, 2]);
    v.resize(6, 5);
    expect_eq!(v, [1, 2, 5, 5, 5, 5]);
    v.resize(0, 5);
    expect_eq!(v, []);
    v.resize(1, 5);
    expect_eq!(v, [5]);
}

#[gtest]
fn test_vector_resize_with() {
    let mut v = vector::Vector::from(vec![1, 2]);
    v.resize_with(6, Default::default);
    expect_eq!(v, [1, 2, 0, 0, 0, 0]);
    v.resize_with(0, Default::default);
    expect_eq!(v, []);
}

#[gtest]
fn test_vector_dedup() {
    let mut v = vector::Vector::from(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4]);
    v.dedup();
    expect_eq!(v, [1, 2, 3, 4]);
}

#[gtest]
fn test_vector_dedup_by() {
    let mut v = vector::Vector::from(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4]);
    v.dedup_by(|a, b| a == b);
    expect_eq!(v, [1, 2, 3, 4]);
}

#[gtest]
fn test_vector_dedup_by_key() {
    let mut v = vector::Vector::from(vec![1, 2, 2, 6, 3, 3, 5, 4, 4, 4, 4]);
    v.dedup_by_key(|a| *a % 2);
    expect_eq!(v, [1, 2, 3, 4]);
}

#[gtest]
fn test_retain() {
    let mut v = vector::Vector::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    v.retain(|&x| x % 2 == 0);
    expect_eq!(v, [2, 4, 6, 8, 10]);
}

#[gtest]
fn test_retain_mut() {
    let mut v = vector::Vector::from(vec![1, 2, 3, 4]);
    v.retain_mut(|x| {
        if *x <= 3 {
            *x += 1;
            true
        } else {
            false
        }
    });
    expect_eq!(v, [2, 3, 4]);
}

#[gtest]
fn test_split_off() {
    let mut vec = vector::Vector::from(vec![1, 2, 3]);
    let vec2 = vec.split_off(1);
    expect_eq!(vec, [1]);
    expect_eq!(vec2, [2, 3]);
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
fn test_vector_extend_from_within() {
    let mut v = vector::Vector::from(vec![0, 1, 2, 3, 4]);

    v.extend_from_within(2..);
    expect_eq!(v, [0, 1, 2, 3, 4, 2, 3, 4]);

    v.extend_from_within(..2);
    expect_eq!(v, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1]);

    v.extend_from_within(4..8);
    expect_eq!(v, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1, 4, 2, 3, 4]);
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
fn test_vector_extend() {
    let mut v = vector::Vector::new();
    v.extend(vec![1, 2, 3]);
    expect_eq!(v, [1, 2, 3]);
    v.extend(vec![4, 5, 6]);
    expect_eq!(v, [1, 2, 3, 4, 5, 6]);
}

#[gtest]
fn test_hash() {
    let b = std::hash::RandomState::new();
    let v: vector::Vector<i64> = vector::Vector::from(vec![10, 12, -4]);
    let s: &[i64] = &[10, 12, -4];
    expect_eq!(b.hash_one(v), b.hash_one(s));
}

#[gtest]
fn test_vector_from_vec_with_capacity() {
    let vector = vector::Vector::from(vec![0, 1, 2, 3, 4]);
    let vec = Vec::from(vector);
    expect_eq!(vec, [0, 1, 2, 3, 4]);
}

#[gtest]
fn test_as_ptr() {
    let v = vector::Vector::from(vec!['a', 'b']);
    let ptr: *const char = v.as_ptr();
    unsafe {
        expect_eq!(*ptr, 'a');
    }
}

#[gtest]
fn test_as_mut_ptr() {
    let mut v = vector::Vector::from(vec!['a', 'b']);
    let ptr: *mut char = v.as_mut_ptr();
    unsafe {
        expect_eq!(*ptr, 'a');
        *ptr = 'c';
    }
    expect_eq!(v[0], 'c');
}

#[gtest]
fn test_as_slice() {
    let v = vector::Vector::from(vec![5, 6, 7]);
    let slice: &[i32] = v.as_slice();
    expect_eq!(slice, &[5, 6, 7]);
}

#[gtest]
fn test_as_mut_slice() {
    let mut v = vector::Vector::from(vec![5, 6, 7]);
    let slice: &mut [i32] = v.as_mut_slice();
    expect_eq!(slice, &[5, 6, 7]);
    slice[1] = 1;
    expect_eq!(v[1], 1);
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

mod partial_equal_tests {
    use googletest::prelude::*;

    #[gtest]
    fn test_vector_eq() {
        let v = vector::Vector::from(vec![1, 2, 3]);
        let u = vector::Vector::from(vec![1, 2]);
        expect_eq!(v == v, true);
        expect_eq!(v == u, false);
        expect_eq!(v != u, true);
        expect_eq!(v != v, false);
    }

    #[gtest]
    fn test_vec_partial_eq() {
        let v = vector::Vector::from(vec![1, 2, 3]);
        expect_eq!(v == vec![1, 2, 3], true);
        expect_eq!(vec![1, 2, 3] == v, true);
        expect_eq!(v == vec![1, 2], false);
        expect_eq!(v == vec![1i16, 2i16, 3i16], true); // i64 and i16 are
                                                       // comparable.
    }

    #[gtest]
    fn test_slice_partial_eq() {
        let v = vector::Vector::from(vec![1, 2, 3]);
        expect_eq!(v == [1, 2, 3].as_slice(), true);
        expect_eq!([1, 2, 3].as_slice() == v, true);
        expect_eq!(v == [1, 2, 3].as_mut_slice(), true);
        expect_eq!([1, 2, 3].as_mut_slice() == v, true);
    }

    #[gtest]
    fn test_array_partial_eq() {
        let v = vector::Vector::from(vec![1, 2, 3]);
        expect_eq!(v == [1, 2, 3], true);
        expect_eq!(v == &[1, 2, 3], true);
    }
}

fn to_void_ptr<T>(t: &T) -> *mut c_void {
    t as *const _ as *mut c_void
}
