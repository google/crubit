#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, const_raw_ptr_deref)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

/// Doc comment
///
///  * with three slashes
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DocCommentSlashes {
    /// A field
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=7
// Error while generating bindings for item 'DocCommentSlashes::DocCommentSlashes':
// Nested classes are not supported yet

/// Doc comment
///
///  * with slashes and bang
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DocCommentBang {
    /// A field
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=15
// Error while generating bindings for item 'DocCommentBang::DocCommentBang':
// Nested classes are not supported yet

/// Multiline comment
///
///  with two stars
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultilineCommentTwoStars {
    /// A field
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=23
// Error while generating bindings for item 'MultilineCommentTwoStars::MultilineCommentTwoStars':
// Nested classes are not supported yet

/// Line comment
///
///  * with two slashes
#[derive(Clone, Copy)]
#[repr(C)]
pub struct LineComment {
    /// A field
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=31
// Error while generating bindings for item 'LineComment::LineComment':
// Nested classes are not supported yet

/// Multiline comment
///
///  with one star
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultilineOneStar {
    /// A field
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'MultilineOneStar::MultilineOneStar':
// Nested classes are not supported yet

/// A function
#[inline(always)]
pub fn foo() -> i32 {
    unsafe { crate::detail::__rust_thunk__foo() }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk__foo() -> i32;
    }
}

const_assert_eq!(std::mem::size_of::<DocCommentSlashes>(), 4usize);
const_assert_eq!(std::mem::align_of::<DocCommentSlashes>(), 4usize);
const_assert_eq!(offset_of!(DocCommentSlashes, i) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<DocCommentBang>(), 4usize);
const_assert_eq!(std::mem::align_of::<DocCommentBang>(), 4usize);
const_assert_eq!(offset_of!(DocCommentBang, i) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<MultilineCommentTwoStars>(), 4usize);
const_assert_eq!(std::mem::align_of::<MultilineCommentTwoStars>(), 4usize);
const_assert_eq!(offset_of!(MultilineCommentTwoStars, i) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<LineComment>(), 4usize);
const_assert_eq!(std::mem::align_of::<LineComment>(), 4usize);
const_assert_eq!(offset_of!(LineComment, i) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<MultilineOneStar>(), 4usize);
const_assert_eq!(std::mem::align_of::<MultilineOneStar>(), 4usize);
const_assert_eq!(offset_of!(MultilineOneStar, i) * 8, 0usize);
