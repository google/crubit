// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nonstandard_calling_convention_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[inline(always)]
pub fn function_vectorcall(a: f32, b: f32) -> f32 {
    unsafe { crate::detail::__rust_thunk___Z19function_vectorcallff(a, b) }
}

#[inline(always)]
pub fn function_win64(a: ::ffi_11::c_int) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z14function_win64i(a) }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "9SomeClass"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: SomeClass
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct SomeClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeClass {}
impl !Sync for SomeClass {}
unsafe impl ::cxx::ExternType for SomeClass {
    type Id = ::cxx::type_id!(":: SomeClass");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for SomeClass {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("SomeClass").finish()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: SomeClass"), crate::SomeClass);
impl SomeClass {
    #[inline(always)]
    pub fn function_vectorcall<'__this>(&'__this mut self, a: f32, b: f32) -> f32 {
        unsafe { self::some_class::function_vectorcall(self, a, b) }
    }
    #[inline(always)]
    pub fn function_win64<'__this>(&'__this mut self, a: ::ffi_11::c_int) -> ::ffi_11::c_int {
        unsafe { self::some_class::function_win64(self, a) }
    }
}

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod some_class {
    #[inline(always)]
    pub(crate) fn function_vectorcall<'__this>(
        __this: &'__this mut crate::SomeClass,
        a: f32,
        b: f32,
    ) -> f32 {
        unsafe { crate::detail::__rust_thunk___ZN9SomeClass19function_vectorcallEff(__this, a, b) }
    }
    #[inline(always)]
    pub(crate) fn function_win64<'__this>(
        __this: &'__this mut crate::SomeClass,
        a: ::ffi_11::c_int,
    ) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN9SomeClass14function_win64Ei(__this, a) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z19function_vectorcallff(a: f32, b: f32) -> f32;
        pub(crate) unsafe fn __rust_thunk___Z14function_win64i(
            a: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClassC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass19function_vectorcallEff<'__this>(
            __this: &'__this mut crate::SomeClass,
            a: f32,
            b: f32,
        ) -> f32;
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass14function_win64Ei<'__this>(
            __this: &'__this mut crate::SomeClass,
            a: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeClass>() == 1);
    assert!(::core::mem::align_of::<crate::SomeClass>() == 1);
    static_assertions::assert_impl_all!(crate::SomeClass: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
};
