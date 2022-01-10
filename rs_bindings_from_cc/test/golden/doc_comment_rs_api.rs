#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes)]

use memoffset_unstable_const::offset_of;

// <unknown location>
// Error while generating bindings for item '__builtin_ms_va_list':
// Cannot generate bindings for type aliases

/// Doc comment
///
///  * with three slashes
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DocCommentSlashes {
    /// A field.
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=7
// Error while generating bindings for item 'DocCommentSlashes::DocCommentSlashes':
// Nested classes are not supported yet

/// The default constructor which will get translated into
/// `impl Default for DocCommentSlashes`.
impl Default for DocCommentSlashes {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashesC1Ev(tmp.as_mut_ptr());
            tmp.assume_init()
        }
    }
}

/// A conversion constructor which will get translated into
/// `impl From<int> for DocCommentSlashes`.
impl From<i32> for DocCommentSlashes {
    #[inline(always)]
    fn from(__param_0: i32) -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashesC1Ei(tmp.as_mut_ptr(), __param_0);
            tmp.assume_init()
        }
    }
}

impl DocCommentSlashes {
    /// A static method.
    #[inline(always)]
    pub fn static_method() -> i32 {
        unsafe { crate::detail::__rust_thunk___ZN17DocCommentSlashes13static_methodEv() }
    }
}

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

// rs_bindings_from_cc/test/golden/doc_comment.h;l=26
// Error while generating bindings for item 'DocCommentBang::DocCommentBang':
// Nested classes are not supported yet

impl Default for DocCommentBang {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DocCommentBangC1Ev(tmp.as_mut_ptr());
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=26
// Error while generating bindings for item 'DocCommentBang::DocCommentBang':
// Parameter type 'struct DocCommentBang &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=26
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

// rs_bindings_from_cc/test/golden/doc_comment.h;l=34
// Error while generating bindings for item 'MultilineCommentTwoStars::MultilineCommentTwoStars':
// Nested classes are not supported yet

impl Default for MultilineCommentTwoStars {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(tmp.as_mut_ptr());
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=34
// Error while generating bindings for item 'MultilineCommentTwoStars::MultilineCommentTwoStars':
// Parameter type 'struct MultilineCommentTwoStars &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=34
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

// rs_bindings_from_cc/test/golden/doc_comment.h;l=42
// Error while generating bindings for item 'LineComment::LineComment':
// Nested classes are not supported yet

impl Default for LineComment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11LineCommentC1Ev(tmp.as_mut_ptr());
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=42
// Error while generating bindings for item 'LineComment::LineComment':
// Parameter type 'struct LineComment &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=42
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

// rs_bindings_from_cc/test/golden/doc_comment.h;l=50
// Error while generating bindings for item 'MultilineOneStar::MultilineOneStar':
// Nested classes are not supported yet

impl Default for MultilineOneStar {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16MultilineOneStarC1Ev(tmp.as_mut_ptr());
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=50
// Error while generating bindings for item 'MultilineOneStar::MultilineOneStar':
// Parameter type 'struct MultilineOneStar &&' is not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=50
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
        #[link_name = "_ZN17DocCommentSlashesC1Ev"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ev(__this: *mut DocCommentSlashes);
        #[link_name = "_ZN17DocCommentSlashesC1Ei"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ei(
            __this: *mut DocCommentSlashes,
            __param_0: i32,
        );
        #[link_name = "_ZN17DocCommentSlashes13static_methodEv"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashes13static_methodEv() -> i32;
        pub(crate) fn __rust_thunk___ZN14DocCommentBangC1Ev(__this: *mut DocCommentBang);
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(
            __this: *mut MultilineCommentTwoStars,
        );
        pub(crate) fn __rust_thunk___ZN11LineCommentC1Ev(__this: *mut LineComment);
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1Ev(__this: *mut MultilineOneStar);
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
