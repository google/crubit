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
forward_declare::unsafe_define!(
    forward_declare::symbol!("DocCommentSlashes"),
    crate::DocCommentSlashes
);

/// The default constructor which will get translated into
/// `impl Default for DocCommentSlashes`.
impl Default for DocCommentSlashes {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17DocCommentSlashesC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
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
                &raw mut tmp as *mut ::core::ffi::c_void,
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
forward_declare::unsafe_define!(forward_declare::symbol!("DocCommentBang"), crate::DocCommentBang);

impl Default for DocCommentBang {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14DocCommentBangC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

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
forward_declare::unsafe_define!(
    forward_declare::symbol!("MultilineCommentTwoStars"),
    crate::MultilineCommentTwoStars
);

impl Default for MultilineCommentTwoStars {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24MultilineCommentTwoStarsC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

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
forward_declare::unsafe_define!(forward_declare::symbol!("LineComment"), crate::LineComment);

impl Default for LineComment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11LineCommentC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

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
forward_declare::unsafe_define!(
    forward_declare::symbol!("MultilineOneStar"),
    crate::MultilineOneStar
);

impl Default for MultilineOneStar {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16MultilineOneStarC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

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

/// Type alias to template instantiation.
pub type MyInstantiation = crate::__CcTemplateInst10MyTemplateIiE;

/// Type alias to instantiation of a template specialization.
pub type MySpecializedInstantiation = crate::__CcTemplateInst10MyTemplateIfE;

// Error while generating bindings for class 'OuterTemplate':
// Class templates are not supported yet

/// Type alias to a struct nested in a template instantiation.
pub(crate) type ConcreteNestedStruct = ::forward_declare::Incomplete<
    ::forward_declare::symbol!("struct OuterTemplate < int > :: NestedStruct"),
    (),
>;

/// Class template.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < int >
pub struct __CcTemplateInst10MyTemplateIiE {
    /// Data member.
    pub value: ::core::ffi::c_int,
}
impl !Send for __CcTemplateInst10MyTemplateIiE {}
impl !Sync for __CcTemplateInst10MyTemplateIiE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < int >"),
    crate::__CcTemplateInst10MyTemplateIiE
);

impl Default for __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

// A non-static member function.

impl __CcTemplateInst10MyTemplateIiE {
    /// A non-static member function.
    #[inline(always)]
    pub fn get_field_value<'a>(&'a self) -> &'a ::core::ffi::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZNK10MyTemplateIiE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(self)
        }
    }
}

/// Class template specialization.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < float >
pub struct __CcTemplateInst10MyTemplateIfE {
    /// Data member in a specialization.
    pub value: f32,
}
impl !Send for __CcTemplateInst10MyTemplateIfE {}
impl !Sync for __CcTemplateInst10MyTemplateIfE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < float >"),
    crate::__CcTemplateInst10MyTemplateIfE
);

impl Default for __CcTemplateInst10MyTemplateIfE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl __CcTemplateInst10MyTemplateIfE {
    /// A non-static member function in a specialization.
    #[inline(always)]
    pub fn get_field_value<'a>(&'a self) -> &'a f32 {
        unsafe {
            crate::detail::__rust_thunk___ZNK10MyTemplateIfE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(self)
        }
    }
}

/// Class template with nested struct inside.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=OuterTemplate < int >
pub struct __CcTemplateInst13OuterTemplateIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst13OuterTemplateIiE {}
impl !Sync for __CcTemplateInst13OuterTemplateIiE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("OuterTemplate < int >"),
    crate::__CcTemplateInst13OuterTemplateIiE
);

impl Default for __CcTemplateInst13OuterTemplateIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13OuterTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(&raw mut tmp as*mut::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

// Doc comment for the nested struct.

// Error while generating bindings for struct 'NestedStruct':
// Can't generate bindings for NestedStruct, because it is unsupported: b/200067824: type definitions nested inside templated records are not yet supported

// Data member in a nested struct.

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
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK10MyTemplateIiE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc<
            'a,
        >(
            __this: &'a crate::__CcTemplateInst10MyTemplateIiE,
        ) -> &'a ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN10MyTemplateIfEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK10MyTemplateIfE15get_field_valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc<
            'a,
        >(
            __this: &'a crate::__CcTemplateInst10MyTemplateIfE,
        ) -> &'a f32;
        pub(crate) unsafe fn __rust_thunk___ZN13OuterTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adoc_5fcomment_5fcc(
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
