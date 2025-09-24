// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:doc_comment_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Doc comment
///
///  * with three slashes
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DocCommentSlashes
pub struct DocCommentSlashes {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// A field.
    pub i: ::core::ffi::c_int,
}
impl !Send for DocCommentSlashes {}
impl !Sync for DocCommentSlashes {}
unsafe impl ::cxx::ExternType for DocCommentSlashes {
    type Id = ::cxx::type_id!("DocCommentSlashes");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'DocCommentSlashes::DocCommentSlashes':
// Can't generate bindings for DocCommentSlashes::DocCommentSlashes, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentSlashes::DocCommentSlashes (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'DocCommentSlashes::DocCommentSlashes':
// Can't generate bindings for DocCommentSlashes::DocCommentSlashes, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentSlashes::DocCommentSlashes (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'DocCommentSlashes::operator=':
// Can't generate bindings for DocCommentSlashes::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentSlashes::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentSlashes::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'DocCommentSlashes::operator=':
// Can't generate bindings for DocCommentSlashes::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentSlashes::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentSlashes::operator= (the type of __param_0 (parameter #1): references are not supported)

/// The default constructor which will get translated into
/// `impl Default for DocCommentSlashes`.
impl Default for DocCommentSlashes {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashesC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// An implicit conversion constructor which will get translated into `impl
/// From<int> for DocCommentSlashes`.
impl From<::core::ffi::c_int> for DocCommentSlashes {
    #[inline(always)]
    fn from(__param_0: ::core::ffi::c_int) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashesC1Ei(
                &raw mut tmp as *mut _,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::core::ffi::c_int> for DocCommentSlashes {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::core::ffi::c_int) -> Self::CtorType {
        <Self as From<::core::ffi::c_int>>::from(args)
    }
}

impl DocCommentSlashes {
    /// A non-static member function (`const` flavor).
    #[inline(always)]
    pub fn get_field_value<'a>(&'a self) -> ::core::ffi::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK17DocCommentSlashes15get_field_valueEv(self) }
    }
}

impl DocCommentSlashes {
    /// A non-static member function (non-`const` flavor).
    #[inline(always)]
    pub fn set_field_value<'a>(&'a mut self, new_value: ::core::ffi::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashes15set_field_valueEi(self, new_value)
        }
    }
}

impl DocCommentSlashes {
    /// A static method.
    #[inline(always)]
    pub fn static_method() -> ::core::ffi::c_int {
        unsafe { crate::detail::__rust_thunk___ZN17DocCommentSlashes13static_methodEv() }
    }
}

/// Doc comment
///
///  * with slashes and bang
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DocCommentBang
pub struct DocCommentBang {
    /// A field
    pub i: ::core::ffi::c_int,
}
impl !Send for DocCommentBang {}
impl !Sync for DocCommentBang {}
unsafe impl ::cxx::ExternType for DocCommentBang {
    type Id = ::cxx::type_id!("DocCommentBang");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for DocCommentBang {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DocCommentBangC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'DocCommentBang::DocCommentBang':
// Can't generate bindings for DocCommentBang::DocCommentBang, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentBang::DocCommentBang (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'DocCommentBang::DocCommentBang':
// Can't generate bindings for DocCommentBang::DocCommentBang, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentBang::DocCommentBang (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'DocCommentBang::operator=':
// Can't generate bindings for DocCommentBang::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentBang::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentBang::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'DocCommentBang::operator=':
// Can't generate bindings for DocCommentBang::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentBang::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for DocCommentBang::operator= (the type of __param_0 (parameter #1): references are not supported)

/// Multiline comment
///
///  with two stars
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MultilineCommentTwoStars
pub struct MultilineCommentTwoStars {
    /// A field
    pub i: ::core::ffi::c_int,
}
impl !Send for MultilineCommentTwoStars {}
impl !Sync for MultilineCommentTwoStars {}
unsafe impl ::cxx::ExternType for MultilineCommentTwoStars {
    type Id = ::cxx::type_id!("MultilineCommentTwoStars");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for MultilineCommentTwoStars {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'MultilineCommentTwoStars::MultilineCommentTwoStars':
// Can't generate bindings for MultilineCommentTwoStars::MultilineCommentTwoStars, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineCommentTwoStars::MultilineCommentTwoStars (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'MultilineCommentTwoStars::MultilineCommentTwoStars':
// Can't generate bindings for MultilineCommentTwoStars::MultilineCommentTwoStars, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineCommentTwoStars::MultilineCommentTwoStars (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'MultilineCommentTwoStars::operator=':
// Can't generate bindings for MultilineCommentTwoStars::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineCommentTwoStars::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineCommentTwoStars::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'MultilineCommentTwoStars::operator=':
// Can't generate bindings for MultilineCommentTwoStars::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineCommentTwoStars::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineCommentTwoStars::operator= (the type of __param_0 (parameter #1): references are not supported)

/// Line comment
///
///  * with two slashes
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=LineComment
pub struct LineComment {
    /// A field
    pub i: ::core::ffi::c_int,
}
impl !Send for LineComment {}
impl !Sync for LineComment {}
unsafe impl ::cxx::ExternType for LineComment {
    type Id = ::cxx::type_id!("LineComment");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for LineComment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11LineCommentC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'LineComment::LineComment':
// Can't generate bindings for LineComment::LineComment, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for LineComment::LineComment (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'LineComment::LineComment':
// Can't generate bindings for LineComment::LineComment, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for LineComment::LineComment (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'LineComment::operator=':
// Can't generate bindings for LineComment::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for LineComment::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for LineComment::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'LineComment::operator=':
// Can't generate bindings for LineComment::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for LineComment::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for LineComment::operator= (the type of __param_0 (parameter #1): references are not supported)

/// Multiline comment
///
///  with one star
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MultilineOneStar
pub struct MultilineOneStar {
    /// A field
    pub i: ::core::ffi::c_int,
}
impl !Send for MultilineOneStar {}
impl !Sync for MultilineOneStar {}
unsafe impl ::cxx::ExternType for MultilineOneStar {
    type Id = ::cxx::type_id!("MultilineOneStar");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for MultilineOneStar {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16MultilineOneStarC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'MultilineOneStar::MultilineOneStar':
// Can't generate bindings for MultilineOneStar::MultilineOneStar, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineOneStar::MultilineOneStar (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'MultilineOneStar::MultilineOneStar':
// Can't generate bindings for MultilineOneStar::MultilineOneStar, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineOneStar::MultilineOneStar (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'MultilineOneStar::operator=':
// Can't generate bindings for MultilineOneStar::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineOneStar::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineOneStar::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'MultilineOneStar::operator=':
// Can't generate bindings for MultilineOneStar::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineOneStar::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:experimental] for MultilineOneStar::operator= (the type of __param_0 (parameter #1): references are not supported)

/// A function
#[inline(always)]
pub fn foo() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z3foov() }
}

/// A type alias
pub type MyTypeAlias = crate::DocCommentSlashes;

// Error while generating bindings for class 'MyTemplate':
// Class templates are not supported yet

// Class template specialization.

// A non-static member function in a specialization.

// Data member in a specialization.

// Error while generating bindings for type alias 'MyInstantiation':
// Can't generate bindings for MyInstantiation, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MyInstantiation (error: Can't generate bindings for MyTemplate<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MyTemplate<int> (crate::__CcTemplateInst10MyTemplateIiE is a template instantiation)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MyTemplate<int> (crate::__CcTemplateInst10MyTemplateIiE is a template instantiation))

// Error while generating bindings for type alias 'MySpecializedInstantiation':
// Can't generate bindings for MySpecializedInstantiation, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MySpecializedInstantiation (error: Can't generate bindings for MyTemplate<float>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MyTemplate<float> (crate::__CcTemplateInst10MyTemplateIfE is a template instantiation)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MyTemplate<float> (crate::__CcTemplateInst10MyTemplateIfE is a template instantiation))

// Error while generating bindings for class 'OuterTemplate':
// Class templates are not supported yet

// Error while generating bindings for type alias 'ConcreteNestedStruct':
// Can't generate bindings for NestedStruct, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for NestedStruct (incomplete type)

// Error while generating bindings for struct 'MyTemplate<int>':
// Can't generate bindings for MyTemplate<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MyTemplate<int> (crate::__CcTemplateInst10MyTemplateIiE is a template instantiation)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MyTemplate<int> (crate::__CcTemplateInst10MyTemplateIiE is a template instantiation)

// Error while generating bindings for struct 'MyTemplate<float>':
// Can't generate bindings for MyTemplate<float>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MyTemplate<float> (crate::__CcTemplateInst10MyTemplateIfE is a template instantiation)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for MyTemplate<float> (crate::__CcTemplateInst10MyTemplateIfE is a template instantiation)

// Error while generating bindings for struct 'OuterTemplate<int>':
// Can't generate bindings for OuterTemplate<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for OuterTemplate<int> (crate::__CcTemplateInst13OuterTemplateIiE is a template instantiation)
// //rs_bindings_from_cc/test/golden:doc_comment_cc needs [//features:wrapper] for OuterTemplate<int> (crate::__CcTemplateInst13OuterTemplateIiE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_ZN17DocCommentSlashesC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN17DocCommentSlashesC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN17DocCommentSlashesC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN17DocCommentSlashesC1Ei(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::core::ffi::c_int,
        );
        #[link_name = "_ZNK17DocCommentSlashes15get_field_valueEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK17DocCommentSlashes15get_field_valueEv<'a>(
            __this: &'a crate::DocCommentSlashes,
        ) -> ::core::ffi::c_int;
        #[link_name = "_ZN17DocCommentSlashes15set_field_valueEi"]
        pub(crate) unsafe fn __rust_thunk___ZN17DocCommentSlashes15set_field_valueEi<'a>(
            __this: &'a mut crate::DocCommentSlashes,
            new_value: ::core::ffi::c_int,
        );
        #[link_name = "_ZN17DocCommentSlashes13static_methodEv"]
        pub(crate) unsafe fn __rust_thunk___ZN17DocCommentSlashes13static_methodEv(
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN14DocCommentBangC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN11LineCommentC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN16MultilineOneStarC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___Z3foov() -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::DocCommentSlashes>() == 4);
    assert!(::core::mem::align_of::<crate::DocCommentSlashes>() == 4);
    static_assertions::assert_impl_all!(crate::DocCommentSlashes: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DocCommentSlashes: Drop);
    assert!(::core::mem::offset_of!(crate::DocCommentSlashes, i) == 0);
    assert!(::core::mem::size_of::<crate::DocCommentBang>() == 4);
    assert!(::core::mem::align_of::<crate::DocCommentBang>() == 4);
    static_assertions::assert_impl_all!(crate::DocCommentBang: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DocCommentBang: Drop);
    assert!(::core::mem::offset_of!(crate::DocCommentBang, i) == 0);
    assert!(::core::mem::size_of::<crate::MultilineCommentTwoStars>() == 4);
    assert!(::core::mem::align_of::<crate::MultilineCommentTwoStars>() == 4);
    static_assertions::assert_impl_all!(crate::MultilineCommentTwoStars: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MultilineCommentTwoStars: Drop);
    assert!(::core::mem::offset_of!(crate::MultilineCommentTwoStars, i) == 0);
    assert!(::core::mem::size_of::<crate::LineComment>() == 4);
    assert!(::core::mem::align_of::<crate::LineComment>() == 4);
    static_assertions::assert_impl_all!(crate::LineComment: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::LineComment: Drop);
    assert!(::core::mem::offset_of!(crate::LineComment, i) == 0);
    assert!(::core::mem::size_of::<crate::MultilineOneStar>() == 4);
    assert!(::core::mem::align_of::<crate::MultilineOneStar>() == 4);
    static_assertions::assert_impl_all!(crate::MultilineOneStar: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MultilineOneStar: Drop);
    assert!(::core::mem::offset_of!(crate::MultilineOneStar, i) == 0);
};
