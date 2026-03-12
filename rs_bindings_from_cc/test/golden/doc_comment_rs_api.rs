// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:doc_comment_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
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
    pub i: ::ffi_11::c_int,
}
impl !Send for DocCommentSlashes {}
impl !Sync for DocCommentSlashes {}
unsafe impl ::cxx::ExternType for DocCommentSlashes {
    type Id = ::cxx::type_id!("DocCommentSlashes");
    type Kind = ::cxx::kind::Trivial;
}
impl DocCommentSlashes {
    /// A non-static member function (`const` flavor).
    #[inline(always)]
    pub fn get_field_value<'a>(&'a self) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK17DocCommentSlashes15get_field_valueEv(self) }
    }
    /// A non-static member function (non-`const` flavor).
    #[inline(always)]
    pub fn set_field_value<'a>(&'a mut self, new_value: ::ffi_11::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashes15set_field_valueEi(self, new_value)
        }
    }
    /// A static method.
    #[inline(always)]
    pub fn static_method() -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN17DocCommentSlashes13static_methodEv() }
    }
}

// error: constructor `DocCommentSlashes::DocCommentSlashes` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `DocCommentSlashes::DocCommentSlashes` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `DocCommentSlashes::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `DocCommentSlashes::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

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
impl From<::ffi_11::c_int> for DocCommentSlashes {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut __param_0 = args;
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
impl ::ctor::CtorNew<::ffi_11::c_int> for DocCommentSlashes {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
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
    pub i: ::ffi_11::c_int,
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

// error: constructor `DocCommentBang::DocCommentBang` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `DocCommentBang::DocCommentBang` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `DocCommentBang::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `DocCommentBang::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

/// Multiline comment
///
///  with two stars
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MultilineCommentTwoStars
pub struct MultilineCommentTwoStars {
    /// A field
    pub i: ::ffi_11::c_int,
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

// error: constructor `MultilineCommentTwoStars::MultilineCommentTwoStars` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `MultilineCommentTwoStars::MultilineCommentTwoStars` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MultilineCommentTwoStars::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MultilineCommentTwoStars::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

/// Line comment
///
///  * with two slashes
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=LineComment
pub struct LineComment {
    /// A field
    pub i: ::ffi_11::c_int,
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

// error: constructor `LineComment::LineComment` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `LineComment::LineComment` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `LineComment::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `LineComment::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

/// Multiline comment
///
///  with one star
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MultilineOneStar
pub struct MultilineOneStar {
    /// A field
    pub i: ::ffi_11::c_int,
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

// error: constructor `MultilineOneStar::MultilineOneStar` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `MultilineOneStar::MultilineOneStar` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MultilineOneStar::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MultilineOneStar::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

/// A function
#[inline(always)]
pub fn foo() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z3foov() }
}

/// A type alias
pub type MyTypeAlias = crate::DocCommentSlashes;

// error: class `MyTemplate` could not be bound
//   Class templates are not yet supported

// Class template specialization.

// A non-static member function in a specialization.

// Data member in a specialization.

// error: type alias `MyInstantiation` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: type alias `MySpecializedInstantiation` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: class `OuterTemplate` could not be bound
//   Class templates are not yet supported

// error: type alias `ConcreteNestedStruct` could not be bound
//   incomplete type

// error: struct `MyTemplate<int>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `MyTemplate<float>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

// error: struct `OuterTemplate<int>` could not be bound
//   template instantiation is not yet supported
//   template instantiation is not yet supported

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
            __param_0: ::ffi_11::c_int,
        );
        #[link_name = "_ZNK17DocCommentSlashes15get_field_valueEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK17DocCommentSlashes15get_field_valueEv<'a>(
            __this: &'a crate::DocCommentSlashes,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZN17DocCommentSlashes15set_field_valueEi"]
        pub(crate) unsafe fn __rust_thunk___ZN17DocCommentSlashes15set_field_valueEi<'a>(
            __this: &'a mut crate::DocCommentSlashes,
            new_value: ::ffi_11::c_int,
        );
        #[link_name = "_ZN17DocCommentSlashes13static_methodEv"]
        pub(crate) unsafe fn __rust_thunk___ZN17DocCommentSlashes13static_methodEv(
        ) -> ::ffi_11::c_int;
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
        pub(crate) unsafe fn __rust_thunk___Z3foov() -> ::ffi_11::c_int;
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
