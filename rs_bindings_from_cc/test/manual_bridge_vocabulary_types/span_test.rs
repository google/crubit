// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::cell::Cell;
use ctor::CtorNew;
use googletest::{expect_eq, gtest};
use span::absl::{RawSpan, RawSpanMut, Span, SpanMut};
use span_lib::{
    MakeNonTrivial, NonTrivial, StructWithLifetimeAnnotatedSpanField, StructWithSpanField,
    TruncateSpan, TruncateSpanMut, TruncateSpanNonTrivial,
};
use static_assertions::{assert_impl_all, assert_not_impl_any};

#[gtest]
fn test_truncate_span_mut() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    let truncated_span: SpanMut<i32> = TruncateSpanMut(SpanMut::from(&mut array[..]), 3);
    let truncated_span: &mut [i32] = unsafe { &mut *truncated_span.as_raw().as_mut_slice() };
    expect_eq!(truncated_span, &mut [1, 2, 3]);
    truncated_span[0] = 100;
    expect_eq!(array, [100, 2, 3, 4, 5]);
}

#[gtest]
fn test_truncate_span() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let truncated_span: Span<i32> = TruncateSpan(Span::from(&array[..]), 3);
    let truncated_span: &[i32] = unsafe { &*truncated_span.as_raw().as_slice() };
    expect_eq!(truncated_span, &[1, 2, 3]);
    expect_eq!(array, [1, 2, 3, 4, 5]);
}

#[gtest]
fn test_make_non_trivial() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let non_trivial = ctor::emplace!(MakeNonTrivial(Span::from(&array[..])));
    expect_eq!(non_trivial.num, 5);
}

#[gtest]
fn test_truncate_span_non_trivial() {
    let nontrivial = ctor::emplace!(NonTrivial::ctor_new(()));
    let array: &[NonTrivial] = std::slice::from_ref(&*nontrivial);
    let truncated_span: Span<NonTrivial> = TruncateSpanNonTrivial(Span::from(array), 0);
    let truncated_span: &[NonTrivial] = unsafe { &*truncated_span.as_raw().as_slice() };
    expect_eq!(truncated_span.len(), 0);
}

assert_impl_all!(Span<'static, i32>: Send, Sync);
assert_impl_all!(SpanMut<'static, i32>: Send, Sync);
assert_not_impl_any!(RawSpan<i32>: Send, Sync);
assert_not_impl_any!(RawSpanMut<i32>: Send, Sync);

// Cell<i32> is !Sync, so Span<Cell<i32>> is !Send and !Sync.
assert_not_impl_any!(Span<'static, Cell<i32>>: Send, Sync);

// SpanMut is Send when T: Send, but !Sync when T: !Sync.
assert_impl_all!(SpanMut<'static, Cell<i32>>: Send);
assert_not_impl_any!(SpanMut<'static, Cell<i32>>: Sync);

// *const i32 is !Send and !Sync.
assert_not_impl_any!(Span<'static, *const i32>: Send, Sync);
assert_not_impl_any!(SpanMut<'static, *const i32>: Send, Sync);

#[gtest]
fn test_struct_with_unannotated_span_field_compiles_as_opaque_blob() {
    let s = StructWithSpanField::default();
    expect_eq!(s.other, 42);
}

#[gtest]
fn test_struct_with_lifetime_annotated_span_field() {
    let array: [i32; 3] = [10, 20, 30];
    let mut s = StructWithLifetimeAnnotatedSpanField::default();
    s.span = Span::from(&array[..]);
    let slice: &[i32] = unsafe { &*s.span.as_raw().as_slice() };
    expect_eq!(slice, &[10, 20, 30]);
    expect_eq!(s.other, 42);
}
