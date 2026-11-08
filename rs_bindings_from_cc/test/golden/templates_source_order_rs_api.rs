// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_source_order_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// error: class `MyTemplate` could not be bound
//   Class templates are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "8TopLevel"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TopLevel
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct TopLevel {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TopLevel {}
impl !Sync for TopLevel {}
unsafe impl ::cxx::ExternType for TopLevel {
    type Id = ::cxx::type_id!("TopLevel");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("TopLevel"), crate::TopLevel);

impl Default for TopLevel {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8TopLevelC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIiE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Alias1 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for Alias1 {}
impl !Sync for Alias1 {}
forward_declare::unsafe_define!(forward_declare::symbol!("MyTemplate < int >"), crate::Alias1);

impl Default for Alias1 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__3c5bb598__ZN10MyTemplateIiEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIfE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < float >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Alias2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for Alias2 {}
impl !Sync for Alias2 {}
forward_declare::unsafe_define!(forward_declare::symbol!("MyTemplate < float >"), crate::Alias2);

impl Default for Alias2 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__3c5bb598__ZN10MyTemplateIfEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateI8TopLevelE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < struct TopLevel >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Alias3 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Alias3 {}
impl !Sync for Alias3 {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < struct TopLevel >"),
    crate::Alias3
);

impl Default for Alias3 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__3c5bb598__ZN10MyTemplateI8TopLevelEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIdE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < double >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Alias4 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for Alias4 {}
impl !Sync for Alias4 {}
forward_declare::unsafe_define!(forward_declare::symbol!("MyTemplate < double >"), crate::Alias4);

impl Default for Alias4 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__3c5bb598__ZN10MyTemplateIdEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIbE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < bool >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Alias5 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Alias5 {}
impl !Sync for Alias5 {}
forward_declare::unsafe_define!(forward_declare::symbol!("MyTemplate < bool >"), crate::Alias5);

impl Default for Alias5 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__3c5bb598__ZN10MyTemplateIbEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIS_I8TopLevelEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < class MyTemplate < struct TopLevel >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Alias6 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Alias6 {}
impl !Sync for Alias6 {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < class MyTemplate < struct TopLevel >>"),
    crate::Alias6
);

impl Default for Alias6 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__3c5bb598__ZN10MyTemplateIS_I8TopLevelEEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod test_namespace_bindings {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N23test_namespace_bindings5InnerE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: Inner
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Inner {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Inner {}
    impl !Sync for Inner {}
    unsafe impl ::cxx::ExternType for Inner {
        type Id = ::cxx::type_id!("test_namespace_bindings :: Inner");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("test_namespace_bindings :: Inner"),
        crate::test_namespace_bindings::Inner
    );

    impl Default for Inner {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings5InnerC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInst10MyTemplateIcE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=MyTemplate < char >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Alias7 {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Alias7 {}
    impl !Sync for Alias7 {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("MyTemplate < char >"),
        crate::test_namespace_bindings::Alias7
    );

    impl Default for Alias7 {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__3c5bb598__ZN10MyTemplateIcEC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=MyTemplate < struct test_namespace_bindings :: Inner >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Alias8 {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Alias8 {}
    impl !Sync for Alias8 {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("MyTemplate < struct test_namespace_bindings :: Inner >"),
        crate::test_namespace_bindings::Alias8
    );

    impl Default for Alias8 {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__3c5bb598__ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=MyTemplate < class MyTemplate < struct test_namespace_bindings :: Inner >>
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Alias9 {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) t: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Alias9 {}
    impl !Sync for Alias9 {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!(
            "MyTemplate < class MyTemplate < struct test_namespace_bindings :: Inner >>"
        ),
        crate::test_namespace_bindings::Alias9
    );

    impl Default for Alias9 {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__3c5bb598__ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }
}

// namespace test_namespace_bindings

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIiE = crate::Alias1;

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIfE = crate::Alias2;

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateI8TopLevelE = crate::Alias3;

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIdE = crate::Alias4;

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIbE = crate::Alias5;

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIS_I8TopLevelEE = crate::Alias6;

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIcE = crate::test_namespace_bindings::Alias7;

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE =
    crate::test_namespace_bindings::Alias8;

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE =
    crate::test_namespace_bindings::Alias9;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN8TopLevelC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk__3c5bb598__ZN10MyTemplateIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__3c5bb598__ZN10MyTemplateIfEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__3c5bb598__ZN10MyTemplateI8TopLevelEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__3c5bb598__ZN10MyTemplateIdEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__3c5bb598__ZN10MyTemplateIbEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__3c5bb598__ZN10MyTemplateIS_I8TopLevelEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings5InnerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__3c5bb598__ZN10MyTemplateIcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__3c5bb598__ZN10MyTemplateIN23test_namespace_bindings5InnerEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__3c5bb598__ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::TopLevel>() == 1);
    assert!(::core::mem::align_of::<crate::TopLevel>() == 1);
    static_assertions::assert_impl_all!(crate::TopLevel: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TopLevel: Drop);

    assert!(::core::mem::size_of::<crate::Alias1>() == 4);
    assert!(::core::mem::align_of::<crate::Alias1>() == 4);
    static_assertions::assert_impl_all!(crate::Alias1: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Alias1: Drop);
    assert!(::core::mem::offset_of!(crate::Alias1, t) == 0);
    assert!(::core::mem::size_of::<crate::Alias2>() == 4);
    assert!(::core::mem::align_of::<crate::Alias2>() == 4);
    static_assertions::assert_impl_all!(crate::Alias2: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Alias2: Drop);
    assert!(::core::mem::offset_of!(crate::Alias2, t) == 0);
    assert!(::core::mem::size_of::<crate::Alias3>() == 1);
    assert!(::core::mem::align_of::<crate::Alias3>() == 1);
    static_assertions::assert_impl_all!(crate::Alias3: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Alias3: Drop);
    assert!(::core::mem::offset_of!(crate::Alias3, t) == 0);
    assert!(::core::mem::size_of::<crate::Alias4>() == 8);
    assert!(::core::mem::align_of::<crate::Alias4>() == 8);
    static_assertions::assert_impl_all!(crate::Alias4: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Alias4: Drop);
    assert!(::core::mem::offset_of!(crate::Alias4, t) == 0);
    assert!(::core::mem::size_of::<crate::Alias5>() == 1);
    assert!(::core::mem::align_of::<crate::Alias5>() == 1);
    static_assertions::assert_impl_all!(crate::Alias5: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Alias5: Drop);
    assert!(::core::mem::offset_of!(crate::Alias5, t) == 0);
    assert!(::core::mem::size_of::<crate::Alias6>() == 1);
    assert!(::core::mem::align_of::<crate::Alias6>() == 1);
    static_assertions::assert_impl_all!(crate::Alias6: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Alias6: Drop);
    assert!(::core::mem::offset_of!(crate::Alias6, t) == 0);
    assert!(::core::mem::size_of::<crate::test_namespace_bindings::Inner>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::Inner>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Inner: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::Inner: Drop);

    assert!(::core::mem::size_of::<crate::test_namespace_bindings::Alias7>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::Alias7>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Alias7: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::Alias7: Drop);
    assert!(::core::mem::offset_of!(crate::test_namespace_bindings::Alias7, t) == 0);
    assert!(::core::mem::size_of::<crate::test_namespace_bindings::Alias8>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::Alias8>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Alias8: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::Alias8: Drop);
    assert!(::core::mem::offset_of!(crate::test_namespace_bindings::Alias8, t) == 0);
    assert!(::core::mem::size_of::<crate::test_namespace_bindings::Alias9>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::Alias9>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Alias9: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::Alias9: Drop);
    assert!(::core::mem::offset_of!(crate::test_namespace_bindings::Alias9, t) == 0);
};
