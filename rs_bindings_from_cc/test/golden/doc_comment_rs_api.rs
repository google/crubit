// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:doc_comment_cc

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Doc comment
///
///  * with three slashes
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DocCommentSlashes {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// A field.
    pub i: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("DocCommentSlashes"),
    crate::DocCommentSlashes
);

impl<'b> From<::ctor::RvalueReference<'b, crate::DocCommentSlashes>> for DocCommentSlashes {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::DocCommentSlashes>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashesC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=13
// Error while generating bindings for item 'DocCommentSlashes::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/doc_comment.h;l=13
// Error while generating bindings for item 'DocCommentSlashes::operator=':
// operator= for Unpin types is not yet supported.

/// The default constructor which will get translated into
/// `impl Default for DocCommentSlashes`.
impl Default for DocCommentSlashes {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
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
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
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
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DocCommentBangC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::DocCommentBang>> for DocCommentBang {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::DocCommentBang>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DocCommentBangC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'DocCommentBang::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/doc_comment.h;l=39
// Error while generating bindings for item 'DocCommentBang::operator=':
// operator= for Unpin types is not yet supported.

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
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::MultilineCommentTwoStars>>
    for MultilineCommentTwoStars
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::MultilineCommentTwoStars>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24MultilineCommentTwoStarsC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=47
// Error while generating bindings for item 'MultilineCommentTwoStars::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/doc_comment.h;l=47
// Error while generating bindings for item 'MultilineCommentTwoStars::operator=':
// operator= for Unpin types is not yet supported.

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
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11LineCommentC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::LineComment>> for LineComment {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::LineComment>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11LineCommentC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=55
// Error while generating bindings for item 'LineComment::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/doc_comment.h;l=55
// Error while generating bindings for item 'LineComment::operator=':
// operator= for Unpin types is not yet supported.

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
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16MultilineOneStarC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::MultilineOneStar>> for MultilineOneStar {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::MultilineOneStar>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16MultilineOneStarC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=63
// Error while generating bindings for item 'MultilineOneStar::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/doc_comment.h;l=63
// Error while generating bindings for item 'MultilineOneStar::operator=':
// operator= for Unpin types is not yet supported.

/// A function
#[inline(always)]
pub fn foo() -> i32 {
    unsafe { crate::detail::__rust_thunk___Z3foov() }
}

/// A type alias
pub type MyTypeAlias = crate::DocCommentSlashes;

// rs_bindings_from_cc/test/golden/doc_comment.h;l=75
// Error while generating bindings for item 'MyTemplate':
// Class templates are not supported yet

// Class template specialization.

// A non-static member function in a specialization.

// Data member in a specialization.

/// Type alias to template instantiation.
pub type MyInstantiation = crate::__CcTemplateInst10MyTemplateIiE;

/// Type alias to instantiation of a template specialization.
pub type MySpecializedInstantiation = crate::__CcTemplateInst10MyTemplateIfE;

// rs_bindings_from_cc/test/golden/doc_comment.h;l=101
// Error while generating bindings for item 'OuterTemplate':
// Class templates are not supported yet

// rs_bindings_from_cc/test/golden/doc_comment.h;l=111
// Error while generating bindings for item 'ConcreteNestedStruct':
// Unsupported type 'struct OuterTemplate<int>::NestedStruct': No generated bindings found for 'NestedStruct'

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DOC_COMMENT_H_

/// Class template.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInst10MyTemplateIiE {
    /// Data member.
    pub value: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<int>"),
    crate::__CcTemplateInst10MyTemplateIiE
);

impl Default for __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIiE>>
    for __CcTemplateInst10MyTemplateIiE
{
    #[inline(always)]
    fn from(
        __param_0: ::ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIiE>,
    ) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=76
// Error while generating bindings for item 'MyTemplate<int>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/doc_comment.h;l=76
// Error while generating bindings for item 'MyTemplate<int>::operator=':
// operator= for Unpin types is not yet supported.

// A non-static member function.

impl __CcTemplateInst10MyTemplateIiE {
    /// A non-static member function.
    #[inline(always)]
    pub fn get_field_value<'a>(&'a self) -> &'a i32 {
        unsafe {
            crate::detail::__rust_thunk___ZNK10MyTemplateIiE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(self)
        }
    }
}

/// Class template specialization.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInst10MyTemplateIfE {
    /// Data member in a specialization.
    pub value: f32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<float>"),
    crate::__CcTemplateInst10MyTemplateIfE
);

impl Default for __CcTemplateInst10MyTemplateIfE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIfE>>
    for __CcTemplateInst10MyTemplateIfE
{
    #[inline(always)]
    fn from(
        __param_0: ::ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIfE>,
    ) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIfEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(&mut tmp,__param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/doc_comment.h;l=86
// Error while generating bindings for item 'MyTemplate<float>::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/doc_comment.h;l=86
// Error while generating bindings for item 'MyTemplate<float>::operator=':
// operator= for Unpin types is not yet supported.

impl __CcTemplateInst10MyTemplateIfE {
    /// A non-static member function in a specialization.
    #[inline(always)]
    pub fn get_field_value<'a>(&'a self) -> &'a f32 {
        unsafe {
            crate::detail::__rust_thunk___ZNK10MyTemplateIfE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(self)
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::DocCommentSlashes>,
            __param_0: ::ctor::RvalueReference<'b, crate::DocCommentSlashes>,
        );
        #[link_name = "_ZN17DocCommentSlashesC1Ev"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::DocCommentSlashes>,
        );
        #[link_name = "_ZN17DocCommentSlashesC1Ei"]
        pub(crate) fn __rust_thunk___ZN17DocCommentSlashesC1Ei<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::DocCommentSlashes>,
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
            __this: &'a mut ::std::mem::MaybeUninit<crate::DocCommentBang>,
        );
        pub(crate) fn __rust_thunk___ZN14DocCommentBangC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::DocCommentBang>,
            __param_0: ::ctor::RvalueReference<'b, crate::DocCommentBang>,
        );
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::MultilineCommentTwoStars>,
        );
        pub(crate) fn __rust_thunk___ZN24MultilineCommentTwoStarsC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::MultilineCommentTwoStars>,
            __param_0: ::ctor::RvalueReference<'b, crate::MultilineCommentTwoStars>,
        );
        pub(crate) fn __rust_thunk___ZN11LineCommentC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::LineComment>,
        );
        pub(crate) fn __rust_thunk___ZN11LineCommentC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::LineComment>,
            __param_0: ::ctor::RvalueReference<'b, crate::LineComment>,
        );
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::MultilineOneStar>,
        );
        pub(crate) fn __rust_thunk___ZN16MultilineOneStarC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::MultilineOneStar>,
            __param_0: ::ctor::RvalueReference<'b, crate::MultilineOneStar>,
        );
        pub(crate) fn __rust_thunk___Z3foov() -> i32;
        pub(crate) fn __rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc<
            'a,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<crate::__CcTemplateInst10MyTemplateIiE>,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIiEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<crate::__CcTemplateInst10MyTemplateIiE>,
            __param_0: ::ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIiE>,
        );
        pub(crate) fn __rust_thunk___ZNK10MyTemplateIiE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc<
            'a,
        >(
            __this: &'a crate::__CcTemplateInst10MyTemplateIiE,
        ) -> &'a i32;
        pub(crate) fn __rust_thunk___ZN10MyTemplateIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc<
            'a,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<crate::__CcTemplateInst10MyTemplateIfE>,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIfEC1EOS0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<crate::__CcTemplateInst10MyTemplateIfE>,
            __param_0: ::ctor::RvalueReference<'b, crate::__CcTemplateInst10MyTemplateIfE>,
        );
        pub(crate) fn __rust_thunk___ZNK10MyTemplateIfE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc<
            'a,
        >(
            __this: &'a crate::__CcTemplateInst10MyTemplateIfE,
        ) -> &'a f32;
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::DocCommentSlashes>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::DocCommentSlashes>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::DocCommentSlashes: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::DocCommentSlashes: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::DocCommentSlashes: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::DocCommentSlashes, i) == 0);

const _: () = assert!(::std::mem::size_of::<crate::DocCommentBang>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::DocCommentBang>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::DocCommentBang: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::DocCommentBang: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::DocCommentBang: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::DocCommentBang, i) == 0);

const _: () = assert!(::std::mem::size_of::<crate::MultilineCommentTwoStars>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::MultilineCommentTwoStars>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::MultilineCommentTwoStars: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::MultilineCommentTwoStars: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::MultilineCommentTwoStars: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::MultilineCommentTwoStars, i) == 0);

const _: () = assert!(::std::mem::size_of::<crate::LineComment>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::LineComment>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::LineComment: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::LineComment: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::LineComment: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::LineComment, i) == 0);

const _: () = assert!(::std::mem::size_of::<crate::MultilineOneStar>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::MultilineOneStar>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::MultilineOneStar: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::MultilineOneStar: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::MultilineOneStar: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::MultilineOneStar, i) == 0);

const _: () = assert!(::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIiE: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::__CcTemplateInst10MyTemplateIiE, value) == 0);

const _: () = assert!(::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIfE>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIfE>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIfE: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIfE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIfE: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::__CcTemplateInst10MyTemplateIfE, value) == 0);
