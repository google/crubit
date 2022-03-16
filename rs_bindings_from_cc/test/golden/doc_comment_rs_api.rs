// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:doc_comment_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Doc comment
///
///  * with three slashes
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DocCommentSlashes {
    /// A field.
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=13
// Error while generating bindings for item 'DocCommentSlashes::DocCommentSlashes':
// Parameter #0 is not supported: Unsupported type 'struct DocCommentSlashes &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/doc_comment.h;l=13
// Error while generating bindings for item 'DocCommentSlashes::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=13
// Error while generating bindings for item 'DocCommentSlashes::operator=':
// Parameter #0 is not supported: Unsupported type 'struct DocCommentSlashes &&': Unsupported clang::Type class 'RValueReference'

/// The default constructor which will get translated into
/// `impl Default for DocCommentSlashes`.
impl Default for DocCommentSlashes {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashesC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// An implicit conversion constructor which will get translated into `impl
/// From<int> for DocCommentSlashes`.
impl From<i32> for DocCommentSlashes {
    #[inline(always)]
    fn from(__param_0: i32) -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashesC1Ei(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl DocCommentSlashes {
    /// A non-static member function (`const` flavor).
    #[inline(always)]
    pub fn get_field_value<'a>(&'a self) -> i32 {
        unsafe { crate::detail::__rust_thunk___ZNK17DocCommentSlashes15get_field_valueEv(self) }
    }
}

impl DocCommentSlashes {
    /// A non-static member function (non-`const` flavor).
    #[inline(always)]
    pub fn set_field_value<'a>(&'a mut self, new_value: i32) {
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashes15set_field_valueEi(self, new_value)
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

/// Doc comment
///
///  * with slashes and bang
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DocCommentBang {
    /// A field
    pub i: i32,
}

impl Default for DocCommentBang {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DocCommentBangC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'DocCommentBang::DocCommentBang':
// Parameter #0 is not supported: Unsupported type 'struct DocCommentBang &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'DocCommentBang::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'DocCommentBang::operator=':
// Parameter #0 is not supported: Unsupported type 'struct DocCommentBang &&': Unsupported clang::Type class 'RValueReference'

/// Multiline comment
///
///  with two stars
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultilineCommentTwoStars {
    /// A field
    pub i: i32,
}

impl Default for MultilineCommentTwoStars {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=47
// Error while generating bindings for item 'MultilineCommentTwoStars::MultilineCommentTwoStars':
// Parameter #0 is not supported: Unsupported type 'struct MultilineCommentTwoStars &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/doc_comment.h;l=47
// Error while generating bindings for item 'MultilineCommentTwoStars::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=47
// Error while generating bindings for item 'MultilineCommentTwoStars::operator=':
// Parameter #0 is not supported: Unsupported type 'struct MultilineCommentTwoStars &&': Unsupported clang::Type class 'RValueReference'

/// Line comment
///
///  * with two slashes
#[derive(Clone, Copy)]
#[repr(C)]
pub struct LineComment {
    /// A field
    pub i: i32,
}

impl Default for LineComment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11LineCommentC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=55
// Error while generating bindings for item 'LineComment::LineComment':
// Parameter #0 is not supported: Unsupported type 'struct LineComment &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/doc_comment.h;l=55
// Error while generating bindings for item 'LineComment::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=55
// Error while generating bindings for item 'LineComment::operator=':
// Parameter #0 is not supported: Unsupported type 'struct LineComment &&': Unsupported clang::Type class 'RValueReference'

/// Multiline comment
///
///  with one star
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultilineOneStar {
    /// A field
    pub i: i32,
}

impl Default for MultilineOneStar {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16MultilineOneStarC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=63
// Error while generating bindings for item 'MultilineOneStar::MultilineOneStar':
// Parameter #0 is not supported: Unsupported type 'struct MultilineOneStar &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/doc_comment.h;l=63
// Error while generating bindings for item 'MultilineOneStar::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=63
// Error while generating bindings for item 'MultilineOneStar::operator=':
// Parameter #0 is not supported: Unsupported type 'struct MultilineOneStar &&': Unsupported clang::Type class 'RValueReference'

/// A function
#[inline(always)]
pub fn foo() -> i32 {
    unsafe { crate::detail::__rust_thunk___Z3foov() }
}

/// A type alias
pub type MyTypeAlias = DocCommentSlashes;

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN17DocCommentSlashesC1Ev"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<DocCommentSlashes>,
        );
        #[link_name = "_ZN17DocCommentSlashesC1Ei"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ei<'a>(
            __this: &'a mut std::mem::MaybeUninit<DocCommentSlashes>,
            __param_0: i32,
        );
        #[link_name = "_ZNK17DocCommentSlashes15get_field_valueEv"]
        pub(crate) fn __rust_thunk___ZNK17DocCommentSlashes15get_field_valueEv<'a>(
            __this: &'a DocCommentSlashes,
        ) -> i32;
        #[link_name = "_ZN17DocCommentSlashes15set_field_valueEi"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashes15set_field_valueEi<'a>(
            __this: &'a mut DocCommentSlashes,
            new_value: i32,
        );
        #[link_name = "_ZN17DocCommentSlashes13static_methodEv"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashes13static_methodEv() -> i32;
        pub(crate) fn __rust_thunk___ZN14DocCommentBangC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<DocCommentBang>,
        );
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<MultilineCommentTwoStars>,
        );
        pub(crate) fn __rust_thunk___ZN11LineCommentC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<LineComment>,
        );
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<MultilineOneStar>,
        );
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
