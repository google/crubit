// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::string_view;
use span::absl::SpanMut;

use googletest::gtest;

#[gtest]
fn identity_span_i32() {
    let mut array: [i32; 2] = [42, 24];
    let array_address = &mut array as *const i32;
    // Check that we return the right lifetime from IdentitySpan.
    fn _identity_span_wrap<'a>(s: SpanMut<'a, i32>) -> SpanMut<'a, i32> {
        assumed_span::IdentitySpan(s)
    }
    let span: SpanMut<'_, i32> = SpanMut::from(&mut array[..]);
    let span_addr = span.as_raw().as_mut_slice() as *const i32;
    assert_eq!(span_addr, array_address);
    let span_out: SpanMut<'_, i32> = assumed_span::IdentitySpan(span);
    assert_eq!(span_out.len(), 2);
    let span_out_addr = span_out.as_raw().as_mut_slice() as *const i32;
    assert_eq!(span_out_addr, array_address);
}

#[gtest]
fn identity_span_string_view() {
    let mut array: [string_view; 2] = ["hello".into(), "world".into()];
    let span: SpanMut<'_, string_view> = SpanMut::from(&mut array[..]);
    let span_out: SpanMut<'_, string_view> = assumed_span::IdentitySpanWithRef(span);
    assert_eq!(span_out.len(), 2);
}

#[gtest]
fn nota_bene_spans_and_inner_lifetimes() {
    fn identity_span_wrap<'a, 'b>(s: SpanMut<'a, string_view<'b>>) -> SpanMut<'a, string_view<'a>> {
        assumed_span::IdentitySpanWithRef(s)
    }
    let mut array: [string_view; 2] = ["hello".into(), "world".into()];
    let span: SpanMut<'_, string_view> = SpanMut::from(&mut array[..]);
    let span_out: SpanMut<'_, string_view> = identity_span_wrap(span);
    assert_eq!(span_out.len(), 2);
}
