#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes)]

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
// Parameter type 'struct MultilineOneStar &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'MultilineOneStar::operator=':
// Parameter type 'struct MultilineOneStar &&' is not supported

/// A function
#[inline(always)]
pub fn foo() -> i32 {
    unsafe { crate::detail::__rust_thunk___Z3foov() }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ev(
            __this: *mut DocCommentSlashes,
        ) -> ();
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1ERKS_(
            __this: *mut DocCommentSlashes,
            __param_0: *const DocCommentSlashes,
        ) -> ();
        pub(crate) fn __rust_thunk___ZN14DocCommentBangC1Ev(__this: *mut DocCommentBang) -> ();
        pub(crate) fn __rust_thunk___ZN14DocCommentBangC1ERKS_(
            __this: *mut DocCommentBang,
            __param_0: *const DocCommentBang,
        ) -> ();
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(
            __this: *mut MultilineCommentTwoStars,
        ) -> ();
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1ERKS_(
            __this: *mut MultilineCommentTwoStars,
            __param_0: *const MultilineCommentTwoStars,
        ) -> ();
        pub(crate) fn __rust_thunk___ZN11LineCommentC1Ev(__this: *mut LineComment) -> ();
        pub(crate) fn __rust_thunk___ZN11LineCommentC1ERKS_(
            __this: *mut LineComment,
            __param_0: *const LineComment,
        ) -> ();
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1Ev(__this: *mut MultilineOneStar) -> ();
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1ERKS_(
            __this: *mut MultilineOneStar,
            __param_0: *const MultilineOneStar,
        ) -> ();
        pub(crate) fn __rust_thunk___Z3foov() -> i32;
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

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
