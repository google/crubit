// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::CtorNew;
use googletest::{expect_eq, gtest};
use span::absl::{RawSpan, RawSpanMut, Span, SpanMut};
use span_lib::{MakeNonTrivial, NonTrivial, TruncateSpan, TruncateSpanMut, TruncateSpanNonTrivial};

#[gtest]
fn test_truncate_span_mut() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    let truncated_span: RawSpanMut<i32> = TruncateSpanMut(SpanMut::from(&mut array[..]), 3);
    let truncated_span: &mut [i32] = unsafe { &mut *truncated_span.as_mut_slice() };
    expect_eq!(truncated_span, &mut [1, 2, 3]);
    truncated_span[0] = 100;
    expect_eq!(array, [100, 2, 3, 4, 5]);
}

#[gtest]
fn test_truncate_span() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let truncated_span: RawSpan<i32> = TruncateSpan(Span::from(&array[..]), 3);
    let truncated_span: &[i32] = unsafe { &*truncated_span.as_slice() };
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
    let truncated_span: RawSpan<NonTrivial> = TruncateSpanNonTrivial(Span::from(array), 0);
    let truncated_span: &[NonTrivial] = unsafe { &*truncated_span.as_slice() };
    expect_eq!(truncated_span.len(), 0);
}
