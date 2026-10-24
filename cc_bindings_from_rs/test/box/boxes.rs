// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(clippy::borrowed_box, clippy::boxed_local)]

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;

pub fn make_box(x: i32) -> Box<i32> {
    Box::new(x)
}

pub fn read_box(b: &Box<i32>) -> i32 {
    **b
}

pub fn mutate_box(b: &mut Box<i32>) {
    **b += 1;
}

pub fn consume_box(b: Box<i32>) -> i32 {
    *b
}

pub fn roundtrip_box(b: Box<i32>) -> Box<i32> {
    b
}

#[derive(Clone, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn make_boxed_point() -> Box<Point> {
    Box::new(Point { x: 10, y: 20 })
}

pub fn point_x(b: Box<Point>) -> i32 {
    b.x
}

pub struct Holder {
    pub b: Box<i32>,
}

impl Default for Holder {
    fn default() -> Self {
        Holder { b: Box::new(42) }
    }
}

pub fn make_holder() -> Holder {
    Holder::default()
}

pub fn holder_value(h: &Holder) -> i32 {
    *h.b
}

pub struct UnmovableHolder {
    pub b: Box<i32>,
}

pub fn make_unmovable_holder() -> UnmovableHolder {
    UnmovableHolder { b: Box::new(43) }
}

pub fn make_pin_box() -> Pin<Box<i32>> {
    Box::pin(100)
}

pub fn read_pin_box(b: &Pin<Box<i32>>) -> i32 {
    **b
}

pub struct NotUnpin {
    _pin: PhantomPinned,
    pub val: i32,
}

pub fn make_pin_box_not_unpin() -> Pin<Box<NotUnpin>> {
    Box::pin(NotUnpin { _pin: PhantomPinned, val: 200 })
}

pub fn make_box_not_unpin() -> Box<NotUnpin> {
    Box::new(NotUnpin { _pin: PhantomPinned, val: 300 })
}

pub fn make_slice_box(n: usize) -> Box<[i32]> {
    vec![1; n].into_boxed_slice()
}

pub fn sum_slice_box(b: &Box<[i32]>) -> i32 {
    b.iter().sum()
}

pub fn consume_slice_box(b: Box<[i32]>) -> usize {
    b.len()
}

pub fn make_str_box() -> Box<str> {
    "hello world".to_string().into_boxed_str()
}

pub fn str_box_len(s: &Box<str>) -> usize {
    s.len()
}

pub fn consume_str_box(s: Box<str>) -> usize {
    s.len()
}

#[derive(Clone)]
pub struct Tracked(Arc<()>);

pub fn make_tracked_box() -> Box<Tracked> {
    Box::new(Tracked(Arc::new(())))
}

pub fn tracked_strong_count(t: &Tracked) -> usize {
    Arc::strong_count(&t.0)
}

pub fn clone_handle(t: &Tracked) -> Tracked {
    Tracked(t.0.clone())
}

pub fn consume_tracked(_b: Box<Tracked>) {}
