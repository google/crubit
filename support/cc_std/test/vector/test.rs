// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use std::cell::RefCell;
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
#[should_panic]
fn test_vector_mut_index_out_of_bounds() {
    let mut v = vector::Vector::new();
    v.push(1);
    v[1] = 10;
}

/// Use a variable in a way that can't be optimized out.
fn use_variable<T>(v: T) {
    std::hint::black_box(v);
}
