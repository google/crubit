// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:doc_comment_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

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

impl<'b> From<ctor::RvalueReference<'b, DocCommentSlashes>> for DocCommentSlashes {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, DocCommentSlashes>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashesC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=13
// Error while generating bindings for item 'DocCommentSlashes::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=13
// Error while generating bindings for item 'DocCommentSlashes::operator=':
// Bindings for this kind of operator are not supported

/// The default constructor which will get translated into
/// `impl Default for DocCommentSlashes`.
impl Default for DocCommentSlashes {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
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
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
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
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DocCommentBangC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, DocCommentBang>> for DocCommentBang {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, DocCommentBang>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DocCommentBangC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'DocCommentBang::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'DocCommentBang::operator=':
// Bindings for this kind of operator are not supported

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
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, MultilineCommentTwoStars>> for MultilineCommentTwoStars {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, MultilineCommentTwoStars>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24MultilineCommentTwoStarsC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=47
// Error while generating bindings for item 'MultilineCommentTwoStars::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=47
// Error while generating bindings for item 'MultilineCommentTwoStars::operator=':
// Bindings for this kind of operator are not supported

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
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11LineCommentC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, LineComment>> for LineComment {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, LineComment>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11LineCommentC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=55
// Error while generating bindings for item 'LineComment::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=55
// Error while generating bindings for item 'LineComment::operator=':
// Bindings for this kind of operator are not supported

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
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16MultilineOneStarC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, MultilineOneStar>> for MultilineOneStar {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, MultilineOneStar>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16MultilineOneStarC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=63
// Error while generating bindings for item 'MultilineOneStar::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/doc_comment.h;l=63
// Error while generating bindings for item 'MultilineOneStar::operator=':
// Bindings for this kind of operator are not supported

/// A function
#[inline(always)]
pub fn foo() -> i32 {
    unsafe { crate::detail::__rust_thunk___Z3foov() }
}

/// A type alias
pub type MyTypeAlias = DocCommentSlashes;

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<DocCommentSlashes>,
            __param_0: ctor::RvalueReference<'b, DocCommentSlashes>,
        );
        #[link_name = "_ZN17DocCommentSlashesC1Ev"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<DocCommentSlashes>,
        );
        #[link_name = "_ZN17DocCommentSlashesC1Ei"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ei<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<DocCommentSlashes>,
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
            __this: &'a mut rust_std::mem::MaybeUninit<DocCommentBang>,
        );
        pub(crate) fn __rust_thunk___ZN14DocCommentBangC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<DocCommentBang>,
            __param_0: ctor::RvalueReference<'b, DocCommentBang>,
        );
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<MultilineCommentTwoStars>,
        );
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<MultilineCommentTwoStars>,
            __param_0: ctor::RvalueReference<'b, MultilineCommentTwoStars>,
        );
        pub(crate) fn __rust_thunk___ZN11LineCommentC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<LineComment>,
        );
        pub(crate) fn __rust_thunk___ZN11LineCommentC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<LineComment>,
            __param_0: ctor::RvalueReference<'b, LineComment>,
        );
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<MultilineOneStar>,
        );
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<MultilineOneStar>,
            __param_0: ctor::RvalueReference<'b, MultilineOneStar>,
        );
        pub(crate) fn __rust_thunk___Z3foov() -> i32;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<DocCommentSlashes>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<DocCommentSlashes>() == 4usize);
const _: () = {
    assert_impl_all!(DocCommentSlashes: Clone);
};
const _: () = {
    assert_impl_all!(DocCommentSlashes: Copy);
};
const _: () = {
    assert_not_impl_all!(DocCommentSlashes: Drop);
};
const _: () = assert!(offset_of!(DocCommentSlashes, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<DocCommentBang>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<DocCommentBang>() == 4usize);
const _: () = {
    assert_impl_all!(DocCommentBang: Clone);
};
const _: () = {
    assert_impl_all!(DocCommentBang: Copy);
};
const _: () = {
    assert_not_impl_all!(DocCommentBang: Drop);
};
const _: () = assert!(offset_of!(DocCommentBang, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<MultilineCommentTwoStars>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<MultilineCommentTwoStars>() == 4usize);
const _: () = {
    assert_impl_all!(MultilineCommentTwoStars: Clone);
};
const _: () = {
    assert_impl_all!(MultilineCommentTwoStars: Copy);
};
const _: () = {
    assert_not_impl_all!(MultilineCommentTwoStars: Drop);
};
const _: () = assert!(offset_of!(MultilineCommentTwoStars, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<LineComment>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<LineComment>() == 4usize);
const _: () = {
    assert_impl_all!(LineComment: Clone);
};
const _: () = {
    assert_impl_all!(LineComment: Copy);
};
const _: () = {
    assert_not_impl_all!(LineComment: Drop);
};
const _: () = assert!(offset_of!(LineComment, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<MultilineOneStar>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<MultilineOneStar>() == 4usize);
const _: () = {
    assert_impl_all!(MultilineOneStar: Clone);
};
const _: () = {
    assert_impl_all!(MultilineOneStar: Copy);
};
const _: () = {
    assert_not_impl_all!(MultilineOneStar: Drop);
};
const _: () = assert!(offset_of!(MultilineOneStar, i) * 8 == 0usize);
