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
forward_declare::unsafe_define!(
    forward_declare::symbol!("DocCommentSlashes"),
    crate::DocCommentSlashes
);

impl<'b> From<ctor::RvalueReference<'b, crate::DocCommentSlashes>> for DocCommentSlashes {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::DocCommentSlashes>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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
forward_declare::unsafe_define!(forward_declare::symbol!("DocCommentBang"), crate::DocCommentBang);

impl Default for DocCommentBang {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DocCommentBangC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::DocCommentBang>> for DocCommentBang {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::DocCommentBang>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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
forward_declare::unsafe_define!(
    forward_declare::symbol!("MultilineCommentTwoStars"),
    crate::MultilineCommentTwoStars
);

impl Default for MultilineCommentTwoStars {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::MultilineCommentTwoStars>>
    for MultilineCommentTwoStars
{
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::MultilineCommentTwoStars>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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
forward_declare::unsafe_define!(forward_declare::symbol!("LineComment"), crate::LineComment);

impl Default for LineComment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11LineCommentC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::LineComment>> for LineComment {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::LineComment>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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
forward_declare::unsafe_define!(
    forward_declare::symbol!("MultilineOneStar"),
    crate::MultilineOneStar
);

impl Default for MultilineOneStar {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16MultilineOneStarC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::MultilineOneStar>> for MultilineOneStar {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::MultilineOneStar>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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
pub type MyTypeAlias = crate::DocCommentSlashes;

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::DocCommentSlashes>,
            __param_0: ctor::RvalueReference<'b, crate::DocCommentSlashes>,
        );
        #[link_name = "_ZN17DocCommentSlashesC1Ev"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::DocCommentSlashes>,
        );
        #[link_name = "_ZN17DocCommentSlashesC1Ei"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ei<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::DocCommentSlashes>,
            __param_0: i32,
        );
        #[link_name = "_ZNK17DocCommentSlashes15get_field_valueEv"]
        pub(crate) fn __rust_thunk___ZNK17DocCommentSlashes15get_field_valueEv<'a>(
            __this: &'a crate::DocCommentSlashes,
        ) -> i32;
        #[link_name = "_ZN17DocCommentSlashes15set_field_valueEi"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashes15set_field_valueEi<'a>(
            __this: &'a mut crate::DocCommentSlashes,
            new_value: i32,
        );
        #[link_name = "_ZN17DocCommentSlashes13static_methodEv"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashes13static_methodEv() -> i32;
        pub(crate) fn __rust_thunk___ZN14DocCommentBangC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::DocCommentBang>,
        );
        pub(crate) fn __rust_thunk___ZN14DocCommentBangC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::DocCommentBang>,
            __param_0: ctor::RvalueReference<'b, crate::DocCommentBang>,
        );
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::MultilineCommentTwoStars>,
        );
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::MultilineCommentTwoStars>,
            __param_0: ctor::RvalueReference<'b, crate::MultilineCommentTwoStars>,
        );
        pub(crate) fn __rust_thunk___ZN11LineCommentC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::LineComment>,
        );
        pub(crate) fn __rust_thunk___ZN11LineCommentC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::LineComment>,
            __param_0: ctor::RvalueReference<'b, crate::LineComment>,
        );
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::MultilineOneStar>,
        );
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::MultilineOneStar>,
            __param_0: ctor::RvalueReference<'b, crate::MultilineOneStar>,
        );
        pub(crate) fn __rust_thunk___Z3foov() -> i32;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::DocCommentSlashes>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::DocCommentSlashes>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::DocCommentSlashes: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::DocCommentSlashes: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::DocCommentSlashes: Drop);
};
const _: () = assert!(offset_of!(crate::DocCommentSlashes, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<crate::DocCommentBang>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::DocCommentBang>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::DocCommentBang: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::DocCommentBang: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::DocCommentBang: Drop);
};
const _: () = assert!(offset_of!(crate::DocCommentBang, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<crate::MultilineCommentTwoStars>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::MultilineCommentTwoStars>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::MultilineCommentTwoStars: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::MultilineCommentTwoStars: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::MultilineCommentTwoStars: Drop);
};
const _: () = assert!(offset_of!(crate::MultilineCommentTwoStars, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<crate::LineComment>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::LineComment>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::LineComment: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::LineComment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::LineComment: Drop);
};
const _: () = assert!(offset_of!(crate::LineComment, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<crate::MultilineOneStar>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::MultilineOneStar>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::MultilineOneStar: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::MultilineOneStar: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::MultilineOneStar: Drop);
};
const _: () = assert!(offset_of!(crate::MultilineOneStar, i) * 8 == 0usize);
