// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:doc_comment_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// Doc comment
///
///  * with three slashes
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "17DocCommentSlashes"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DocCommentSlashes
///CRUBIT_ANNOTATE: cpp_move_constructible=
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
    pub fn get_field_value<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe { self::doc_comment_slashes::get_field_value(self) }
    }
    /// A non-static member function (non-`const` flavor).
    #[inline(always)]
    pub fn set_field_value<'__this>(&'__this mut self, new_value: ::ffi_11::c_int) {
        unsafe { self::doc_comment_slashes::set_field_value(self, new_value) }
    }
    /// A static method.
    #[inline(always)]
    pub fn static_method() -> ::ffi_11::c_int {
        unsafe { self::doc_comment_slashes::static_method() }
    }
}

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

pub mod doc_comment_slashes {
    /// A non-static member function (`const` flavor).
    #[inline(always)]
    pub(crate) fn get_field_value<'__this>(
        __this: &'__this crate::DocCommentSlashes,
    ) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK17DocCommentSlashes15get_field_valueEv(__this) }
    }
    /// A non-static member function (non-`const` flavor).
    #[inline(always)]
    pub(crate) fn set_field_value<'__this>(
        __this: &'__this mut crate::DocCommentSlashes,
        new_value: ::ffi_11::c_int,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashes15set_field_valueEi(
                __this, new_value,
            )
        }
    }
    /// A static method.
    #[inline(always)]
    pub(crate) fn static_method() -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN17DocCommentSlashes13static_methodEv() }
    }
}

/// Doc comment
///
///  * with slashes and bang
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "14DocCommentBang"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DocCommentBang
///CRUBIT_ANNOTATE: cpp_move_constructible=
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

/// Multiline comment
///
///  with two stars
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "24MultilineCommentTwoStars"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MultilineCommentTwoStars
///CRUBIT_ANNOTATE: cpp_move_constructible=
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

/// Line comment
///
///  * with two slashes
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "11LineComment"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=LineComment
///CRUBIT_ANNOTATE: cpp_move_constructible=
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

/// Multiline comment
///
///  with one star
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "16MultilineOneStar"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MultilineOneStar
///CRUBIT_ANNOTATE: cpp_move_constructible=
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

/// Type alias to template instantiation.
pub type MyInstantiation = crate::__CcTemplateInst10MyTemplateIiE;

/// Type alias to instantiation of a template specialization.
pub type MySpecializedInstantiation = crate::__CcTemplateInst10MyTemplateIfE;

// error: class `OuterTemplate` could not be bound
//   Class templates are not yet supported

// error: type alias `ConcreteNestedStruct` could not be bound
//   incomplete type

/// Doc comment for an enum.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[cfi_encoding = "14DocCommentEnum"]
///CRUBIT_ANNOTATE: cpp_type=DocCommentEnum
pub struct DocCommentEnum(::ffi_11::c_uint);
impl DocCommentEnum {
    /// Red color variant.
    pub const kDocCommentRed: DocCommentEnum = DocCommentEnum(::ffi_11::new_c_uint(0));
    /// Blue color variant.
    pub const kDocCommentBlue: DocCommentEnum = DocCommentEnum(::ffi_11::new_c_uint(1));
    /// Green color variant.
    pub const kDocCommentGreen: DocCommentEnum = DocCommentEnum(::ffi_11::new_c_uint(2));
}
impl From<::ffi_11::c_uint> for DocCommentEnum {
    fn from(value: ::ffi_11::c_uint) -> DocCommentEnum {
        DocCommentEnum(value)
    }
}
impl From<DocCommentEnum> for ::ffi_11::c_uint {
    fn from(value: DocCommentEnum) -> ::ffi_11::c_uint {
        value.0
    }
}

/// Class template.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIiE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst10MyTemplateIiE {
    /// Data member.
    pub value: ::ffi_11::c_int,
}
impl !Send for __CcTemplateInst10MyTemplateIiE {}
impl !Sync for __CcTemplateInst10MyTemplateIiE {}

impl Default for __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__c114dea5__ZN10MyTemplateIiEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// A non-static member function.

/// Class template specialization.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIfE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < float >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst10MyTemplateIfE {
    /// Data member in a specialization.
    pub value: f32,
}
impl !Send for __CcTemplateInst10MyTemplateIfE {}
impl !Sync for __CcTemplateInst10MyTemplateIfE {}

impl Default for __CcTemplateInst10MyTemplateIfE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__2a6ef96a__ZN10MyTemplateIfEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// A non-static member function in a specialization.

/// Class template with nested struct inside.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst13OuterTemplateIiE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=OuterTemplate < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst13OuterTemplateIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst13OuterTemplateIiE {}
impl !Sync for __CcTemplateInst13OuterTemplateIiE {}

impl Default for __CcTemplateInst13OuterTemplateIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__2cd151a5__ZN13OuterTemplateIiEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Doc comment for the nested struct.

// Data member in a nested struct.

// error: struct `OuterTemplate<int>::NestedStruct` could not be bound
//   incomplete type

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
        pub(crate) unsafe fn __rust_thunk___ZNK17DocCommentSlashes15get_field_valueEv<'__this>(
            __this: &'__this crate::DocCommentSlashes,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZN17DocCommentSlashes15set_field_valueEi"]
        pub(crate) unsafe fn __rust_thunk___ZN17DocCommentSlashes15set_field_valueEi<'__this>(
            __this: &'__this mut crate::DocCommentSlashes,
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
        pub(crate) unsafe fn __rust_thunk__c114dea5__ZN10MyTemplateIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__2a6ef96a__ZN10MyTemplateIfEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__2cd151a5__ZN13OuterTemplateIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
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
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIiE, value) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIfE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIfE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIfE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIfE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIfE, value) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInst13OuterTemplateIiE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst13OuterTemplateIiE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst13OuterTemplateIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst13OuterTemplateIiE: Drop);
};
