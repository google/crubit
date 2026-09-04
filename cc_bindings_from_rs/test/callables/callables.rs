// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Test crate for Rust functions taking callable parameters (`Fn`, `FnMut`, `FnOnce`).

use crubit_annotate::must_bind;

#[must_bind]
pub fn call_dyn_fn(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

#[must_bind]
pub fn call_dyn_fn_mut(f: &mut dyn FnMut(i32) -> i32, x: i32) -> i32 {
    f(x)
}

#[must_bind]
pub fn call_box_dyn_fn(f: Box<dyn Fn(i32) -> i32>, x: i32) -> i32 {
    f(x)
}

#[must_bind]
pub fn call_box_dyn_fn_mut(mut f: Box<dyn FnMut(i32) -> i32>, x: i32) -> i32 {
    f(x)
}

#[must_bind]
pub fn call_box_dyn_fn_once(f: Box<dyn FnOnce(i32) -> i32>, x: i32) -> i32 {
    f(x)
}

#[must_bind]
pub fn call_impl_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

#[must_bind]
pub fn call_impl_fn_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 {
    f(x)
}

#[must_bind]
pub fn call_impl_fn_once(f: impl FnOnce(i32) -> i32, x: i32) -> i32 {
    f(x)
}

#[must_bind]
pub fn call_impl_fn_once_static(f: impl FnOnce(i32) -> i32 + 'static, x: i32) -> i32 {
    f(x)
}

#[must_bind]
pub fn call_two_args(f: impl Fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}

#[must_bind]
pub fn call_void(f: impl Fn(i32), x: i32) {
    f(x);
}

#[must_bind]
pub fn call_void_mut(mut f: impl FnMut(i32), x: i32) {
    f(x);
}

#[must_bind]
pub fn call_void_once(f: impl FnOnce(i32), x: i32) {
    f(x);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[must_bind]
pub fn call_with_point(f: impl Fn(Point) -> Point, pt: Point) -> Point {
    f(pt)
}

#[must_bind]
pub fn call_point_to_int(f: impl Fn(Point) -> i32, pt: Point) -> i32 {
    f(pt)
}

#[must_bind]
pub fn call_int_to_point(f: impl Fn(i32) -> Point, x: i32) -> Point {
    f(x)
}

#[must_bind]
pub fn call_point_mut(mut f: impl FnMut(Point) -> Point, pt: Point) -> Point {
    f(pt)
}

#[must_bind]
pub fn call_point_once_static(f: impl FnOnce(Point) -> Point + 'static, pt: Point) -> Point {
    f(pt)
}

#[must_bind]
pub fn call_two_points(f: impl Fn(Point, Point) -> Point, a: Point, b: Point) -> Point {
    f(a, b)
}

#[must_bind]
pub fn call_point_void(f: impl Fn(Point), pt: Point) {
    f(pt);
}

#[must_bind]
pub fn call_with_str<'a>(f: impl Fn(&'a str) -> i32, s: &'a str) -> i32 {
    f(s)
}

#[must_bind]
#[allow(clippy::type_complexity)]
pub fn call_with_tuple_option(f: Box<dyn FnOnce((i32, Option<i32>)) -> (i32, Option<i32>)>) -> i32 {
    let (a, b) = f((42, Some(100)));
    a + b.unwrap_or(0)
}

#[must_bind]
#[allow(clippy::type_complexity)]
pub fn call_impl_with_tuple_option(
    f: impl FnOnce((i32, Option<i32>)) -> (i32, Option<i32>) + 'static,
) -> i32 {
    let (a, b) = f((10, Some(20)));
    a + b.unwrap_or(0)
}

#[must_bind]
pub fn call_with_hrtb_str(f: impl Fn(&str) -> i32, s: &str) -> i32 {
    f(s)
}

#[must_bind]
pub fn call_with_str_to_str<'a>(f: impl Fn(&'a str) -> &'a str, s: &'a str) -> &'a str {
    f(s)
}

#[must_bind]
pub fn call_with_hrtb_str_to_str(f: impl Fn(&str) -> &str, s: &str) -> usize {
    f(s).len()
}

#[must_bind]
#[derive(Default)]
pub struct CallbackHolder {
    cb: Option<Box<dyn Fn()>>,
}

impl CallbackHolder {
    #[must_bind]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_bind]
    pub fn set_callback(&mut self, f: Box<dyn Fn()>) {
        self.cb = Some(f);
    }

    #[must_bind]
    pub fn call(&self) {
        if let Some(ref f) = self.cb {
            f();
        }
    }

    #[must_bind]
    pub fn drop_callback(&mut self) {
        self.cb = None;
    }
}

#[must_bind]
pub struct NonCppMovable(pub i32);

impl Drop for NonCppMovable {
    fn drop(&mut self) {}
}

#[must_bind]
pub fn call_and_return_non_movable(f: impl Fn() -> NonCppMovable) -> NonCppMovable {
    f()
}

#[must_bind]
pub fn call_and_return_non_movable_boxed(f: Box<dyn FnOnce() -> NonCppMovable>) -> NonCppMovable {
    f()
}

#[must_bind]
pub fn call_and_return_non_movable_box_fn(f: Box<dyn Fn() -> NonCppMovable>) -> NonCppMovable {
    f()
}

#[must_bind]
pub fn call_with_non_movable_ref(f: impl Fn(&NonCppMovable), x: &NonCppMovable) -> i32 {
    f(x);
    x.0
}

#[must_bind]
#[derive(Default)]
pub struct CppMovableDrop(pub i32);

impl Drop for CppMovableDrop {
    fn drop(&mut self) {}
}

#[must_bind]
pub fn call_with_movable_drop(f: impl Fn(CppMovableDrop), x: i32) {
    f(CppMovableDrop(x));
}
