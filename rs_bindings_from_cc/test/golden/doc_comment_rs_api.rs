#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, custom_inner_attributes)]

use memoffset_unstable_const::offset_of;

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

// rs_bindings_from_cc/test/golden/doc_comment.h;l=7
// Error while generating bindings for item 'DocCommentSlashes::DocCommentSlashes':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=7
// Error while generating bindings for item 'DocCommentSlashes::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=7
// Error while generating bindings for item 'DocCommentSlashes::DocCommentSlashes':
// Parameter type 'struct DocCommentSlashes &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=7
// Error while generating bindings for item 'DocCommentSlashes::operator=':
// Parameter type 'struct DocCommentSlashes &&' is not supported

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

// rs_bindings_from_cc/test/golden/doc_comment.h;l=15
// Error while generating bindings for item 'DocCommentBang::DocCommentBang':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=15
// Error while generating bindings for item 'DocCommentBang::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=15
// Error while generating bindings for item 'DocCommentBang::DocCommentBang':
// Parameter type 'struct DocCommentBang &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=15
// Error while generating bindings for item 'DocCommentBang::operator=':
// Parameter type 'struct DocCommentBang &&' is not supported

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

// rs_bindings_from_cc/test/golden/doc_comment.h;l=23
// Error while generating bindings for item 'MultilineCommentTwoStars::MultilineCommentTwoStars':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=23
// Error while generating bindings for item 'MultilineCommentTwoStars::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=23
// Error while generating bindings for item 'MultilineCommentTwoStars::MultilineCommentTwoStars':
// Parameter type 'struct MultilineCommentTwoStars &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=23
// Error while generating bindings for item 'MultilineCommentTwoStars::operator=':
// Parameter type 'struct MultilineCommentTwoStars &&' is not supported

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

// rs_bindings_from_cc/test/golden/doc_comment.h;l=31
// Error while generating bindings for item 'LineComment::LineComment':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=31
// Error while generating bindings for item 'LineComment::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=31
// Error while generating bindings for item 'LineComment::LineComment':
// Parameter type 'struct LineComment &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=31
// Error while generating bindings for item 'LineComment::operator=':
// Parameter type 'struct LineComment &&' is not supported

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

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'MultilineOneStar::MultilineOneStar':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'MultilineOneStar::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'MultilineOneStar::MultilineOneStar':
// Parameter type 'struct MultilineOneStar &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'MultilineOneStar::operator=':
// Parameter type 'struct MultilineOneStar &&' is not supported

/// A function
#[inline(always)]
pub fn foo() -> i32 {
    unsafe { crate::detail::__rust_thunk__foo() }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_constructor_thunk__DocCommentSlashes(
            __this: *mut DocCommentSlashes,
        ) -> ();
        pub(crate) fn __rust_constructor_thunk__DocCommentBang(__this: *mut DocCommentBang) -> ();
        pub(crate) fn __rust_constructor_thunk__MultilineCommentTwoStars(
            __this: *mut MultilineCommentTwoStars,
        ) -> ();
        pub(crate) fn __rust_constructor_thunk__LineComment(__this: *mut LineComment) -> ();
        pub(crate) fn __rust_constructor_thunk__MultilineOneStar(
            __this: *mut MultilineOneStar,
        ) -> ();
        pub(crate) fn __rust_thunk__foo() -> i32;
    }
}

const _: () = assert!(std::mem::size_of::<DocCommentSlashes>() == 4usize);
const _: () = assert!(std::mem::align_of::<DocCommentSlashes>() == 4usize);
const _: () = assert!(offset_of!(DocCommentSlashes, i) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<DocCommentBang>() == 4usize);
const _: () = assert!(std::mem::align_of::<DocCommentBang>() == 4usize);
const _: () = assert!(offset_of!(DocCommentBang, i) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<MultilineCommentTwoStars>() == 4usize);
const _: () = assert!(std::mem::align_of::<MultilineCommentTwoStars>() == 4usize);
const _: () = assert!(offset_of!(MultilineCommentTwoStars, i) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<LineComment>() == 4usize);
const _: () = assert!(std::mem::align_of::<LineComment>() == 4usize);
const _: () = assert!(offset_of!(LineComment, i) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<MultilineOneStar>() == 4usize);
const _: () = assert!(std::mem::align_of::<MultilineOneStar>() == 4usize);
const _: () = assert!(offset_of!(MultilineOneStar, i) * 8 == 0usize);
