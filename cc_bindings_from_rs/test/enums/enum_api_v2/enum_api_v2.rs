// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` with the `enum_api_v2` feature,
//! and the generated C++ bindings are then tested via `enum_api_v2_test.cc`.

/// Unit, tuple, and struct-like variants, with a direct tag.
#[derive(Clone, Copy)]
pub enum Shape {
    Point,
    Circle(i32),
    Rect { w: i32, h: i32 },
}

pub fn make_circle(r: i32) -> Shape {
    Shape::Circle(r)
}

pub fn make_rect(w: i32, h: i32) -> Shape {
    Shape::Rect { w, h }
}

pub fn area(shape: Shape) -> i32 {
    match shape {
        Shape::Point => 0,
        Shape::Circle(r) => 3 * r * r,
        Shape::Rect { w, h } => w * h,
    }
}

/// Negative discriminants.
#[derive(Clone, Copy)]
#[repr(i8)]
pub enum Sign {
    Negative = -1,
    Zero = 0,
    Positive = 1,
}

pub fn sign_of(x: i32) -> Sign {
    match x {
        ..0 => Sign::Negative,
        0 => Sign::Zero,
        _ => Sign::Positive,
    }
}

pub fn sign_value(sign: Sign) -> i32 {
    sign as i32
}

/// A single variant, so no tag at all.
#[derive(Clone, Copy)]
pub enum Wrapper {
    Only(i64),
}

pub fn unwrap(wrapper: Wrapper) -> i64 {
    let Wrapper::Only(value) = wrapper;
    value
}
