// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/semantic_import:semantic_import_upcast

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "1S"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=:: S
pub struct S {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) x_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for S {}
impl !Sync for S {}
unsafe impl ::cxx::ExternType for S {
    type Id = ::cxx::type_id!(":: S");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for S {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("S").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: S"), crate::S);
impl S {
    #[inline(always)]
    pub fn x<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe { self::s::x(self) }
    }
    #[inline(always)]
    pub fn get_x<'__this>(&'__this mut self) -> ::ffi_11::c_int {
        unsafe { self::s::get_x(self) }
    }
    #[inline(always)]
    pub fn set_x<'__this>(&'__this mut self, x: ::ffi_11::c_int) {
        unsafe { self::s::set_x(self, x) }
    }
}

impl From<::ffi_11::c_int> for S {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1SC1Ei(&raw mut tmp as *mut _, x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for S {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

pub mod s {
    #[inline(always)]
    pub(crate) fn x<'__this>(__this: &'__this crate::S) -> ::ffi_11::c_int {
        unsafe {
            (*((&*__this as *const _ as *const u8).add(0) as *const ::ffi_11::c_int))
                as ::ffi_11::c_int
        }
    }
    #[inline(always)]
    pub(crate) fn get_x<'__this>(__this: &'__this mut crate::S) -> ::ffi_11::c_int {
        unsafe {
            (*((&*__this as *const _ as *const u8).add(0) as *const ::ffi_11::c_int))
                as ::ffi_11::c_int
        }
    }
    #[inline(always)]
    pub(crate) fn set_x<'__this>(__this: &'__this mut crate::S, x: ::ffi_11::c_int) {
        unsafe {
            *((__this as *mut _ as *mut u8).add(0) as *mut ::ffi_11::c_int) = (x as ::ffi_11::c_int)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "1T"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=:: T
pub struct T {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) y_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for T {}
impl !Sync for T {}
unsafe impl ::cxx::ExternType for T {
    type Id = ::cxx::type_id!(":: T");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for T {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("T")
            .field("", ::oops::Upcast::<&crate::S>::upcast(self))
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: T"), crate::T);
impl T {
    #[inline(always)]
    pub fn y<'__this>(&'__this self) -> f32 {
        unsafe { self::t::y(self) }
    }
    #[inline(always)]
    pub fn get_x<'__this>(&'__this mut self) -> ::ffi_11::c_int {
        unsafe { self::t::get_x(oops::Upcast::<_>::upcast(self)) }
    }
    #[inline(always)]
    pub fn set_x<'__this>(&'__this mut self, x: ::ffi_11::c_int) {
        unsafe { self::t::set_x(oops::Upcast::<_>::upcast(self), x) }
    }
    #[inline(always)]
    pub fn x<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe { self::t::x(oops::Upcast::<_>::upcast(self)) }
    }
}

impl From<(::ffi_11::c_int, f32)> for T {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, f32)) -> Self {
        let (mut x, mut y) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1TC1Eif(&raw mut tmp as *mut _, x, y);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, f32)> for T {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, f32)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, f32)>>::from(args)
    }
}

unsafe impl oops::Inherits<crate::S> for crate::T {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::S {
        unsafe { (derived as *const _ as *const u8).offset(0) as *const crate::S }
    }
}

pub mod t {
    #[inline(always)]
    pub(crate) fn y<'__this>(__this: &'__this crate::T) -> f32 {
        unsafe { (*((&*__this as *const _ as *const u8).add(4) as *const f32)) as f32 }
    }
    #[inline(always)]
    pub(crate) fn get_x<'__this>(__this: &'__this mut crate::S) -> ::ffi_11::c_int {
        unsafe {
            (*((&*__this as *const _ as *const u8).add(0) as *const ::ffi_11::c_int))
                as ::ffi_11::c_int
        }
    }
    #[inline(always)]
    pub(crate) fn set_x<'__this>(__this: &'__this mut crate::S, x: ::ffi_11::c_int) {
        unsafe {
            *((__this as *mut _ as *mut u8).add(0) as *mut ::ffi_11::c_int) = (x as ::ffi_11::c_int)
        }
    }
    #[inline(always)]
    pub(crate) fn x<'__this>(__this: &'__this crate::S) -> ::ffi_11::c_int {
        unsafe {
            (*((&*__this as *const _ as *const u8).add(0) as *const ::ffi_11::c_int))
                as ::ffi_11::c_int
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Chars"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: Chars
pub struct Chars {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) c_: [::core::mem::MaybeUninit<u8>; 1],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) sc_: [::core::mem::MaybeUninit<u8>; 1],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) uc_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Chars {}
impl !Sync for Chars {}
unsafe impl ::cxx::ExternType for Chars {
    type Id = ::cxx::type_id!(":: Chars");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for Chars {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("Chars").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: Chars"), crate::Chars);
impl Chars {
    #[inline(always)]
    pub fn c<'__this>(&'__this mut self) -> ::ffi_11::c_char {
        unsafe { self::chars::c(self) }
    }
    #[inline(always)]
    pub fn sc<'__this>(&'__this mut self) -> ::ffi_11::c_schar {
        unsafe { self::chars::sc(self) }
    }
    #[inline(always)]
    pub fn uc<'__this>(&'__this mut self) -> ::ffi_11::c_uchar {
        unsafe { self::chars::uc(self) }
    }
    #[inline(always)]
    pub fn set_c<'__this>(&'__this mut self, c: ::ffi_11::c_char) {
        unsafe { self::chars::set_c(self, c) }
    }
    #[inline(always)]
    pub fn set_sc<'__this>(&'__this mut self, sc: ::ffi_11::c_schar) {
        unsafe { self::chars::set_sc(self, sc) }
    }
    #[inline(always)]
    pub fn set_uc<'__this>(&'__this mut self, uc: ::ffi_11::c_uchar) {
        unsafe { self::chars::set_uc(self, uc) }
    }
}

impl Default for Chars {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5CharsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod chars {
    #[inline(always)]
    pub(crate) fn c<'__this>(__this: &'__this mut crate::Chars) -> ::ffi_11::c_char {
        unsafe {
            (*((&*__this as *const _ as *const u8).add(0) as *const ::ffi_11::c_char))
                as ::ffi_11::c_char
        }
    }
    #[inline(always)]
    pub(crate) fn sc<'__this>(__this: &'__this mut crate::Chars) -> ::ffi_11::c_schar {
        unsafe {
            (*((&*__this as *const _ as *const u8).add(1) as *const ::ffi_11::c_schar))
                as ::ffi_11::c_schar
        }
    }
    #[inline(always)]
    pub(crate) fn uc<'__this>(__this: &'__this mut crate::Chars) -> ::ffi_11::c_uchar {
        unsafe {
            (*((&*__this as *const _ as *const u8).add(2) as *const ::ffi_11::c_uchar))
                as ::ffi_11::c_uchar
        }
    }
    #[inline(always)]
    pub(crate) fn set_c<'__this>(__this: &'__this mut crate::Chars, c: ::ffi_11::c_char) {
        unsafe {
            *((__this as *mut _ as *mut u8).add(0) as *mut ::ffi_11::c_char) =
                (c as ::ffi_11::c_char)
        }
    }
    #[inline(always)]
    pub(crate) fn set_sc<'__this>(__this: &'__this mut crate::Chars, sc: ::ffi_11::c_schar) {
        unsafe {
            *((__this as *mut _ as *mut u8).add(1) as *mut ::ffi_11::c_schar) =
                (sc as ::ffi_11::c_schar)
        }
    }
    #[inline(always)]
    pub(crate) fn set_uc<'__this>(__this: &'__this mut crate::Chars, uc: ::ffi_11::c_uchar) {
        unsafe {
            *((__this as *mut _ as *mut u8).add(2) as *mut ::ffi_11::c_uchar) =
                (uc as ::ffi_11::c_uchar)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Bools"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: Bools
pub struct Bools {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Bools {}
impl !Sync for Bools {}
unsafe impl ::cxx::ExternType for Bools {
    type Id = ::cxx::type_id!(":: Bools");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for Bools {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("Bools").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: Bools"), crate::Bools);
impl Bools {
    #[inline(always)]
    pub fn b<'__this>(&'__this mut self) -> bool {
        unsafe { self::bools::b(self) }
    }
    #[inline(always)]
    pub fn set_b<'__this>(&'__this mut self, b: bool) {
        unsafe { self::bools::set_b(self, b) }
    }
}

impl Default for Bools {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5BoolsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod bools {
    #[inline(always)]
    pub(crate) fn b<'__this>(__this: &'__this mut crate::Bools) -> bool {
        unsafe { (*((&*__this as *const _ as *const u8).add(0) as *const bool)) as bool }
    }
    #[inline(always)]
    pub(crate) fn set_b<'__this>(__this: &'__this mut crate::Bools, b: bool) {
        unsafe { *((__this as *mut _ as *mut u8).add(0) as *mut bool) = (b as bool) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN1SC1Ei(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN1TC1Eif(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
            y: f32,
        );
        pub(crate) unsafe fn __rust_thunk___ZN5CharsC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5BoolsC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::S>() == 4);
    assert!(::core::mem::align_of::<crate::S>() == 4);
    static_assertions::assert_impl_all!(crate::S: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::S: Drop);
    assert!(::core::mem::offset_of!(crate::S, x_) == 0);
    assert!(::core::mem::size_of::<crate::T>() == 8);
    assert!(::core::mem::align_of::<crate::T>() == 4);
    static_assertions::assert_impl_all!(crate::T: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::T: Drop);
    assert!(::core::mem::offset_of!(crate::T, y_) == 4);
    assert!(::core::mem::size_of::<crate::Chars>() == 3);
    assert!(::core::mem::align_of::<crate::Chars>() == 1);
    static_assertions::assert_impl_all!(crate::Chars: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Chars: Drop);
    assert!(::core::mem::offset_of!(crate::Chars, c_) == 0);
    assert!(::core::mem::offset_of!(crate::Chars, sc_) == 1);
    assert!(::core::mem::offset_of!(crate::Chars, uc_) == 2);
    assert!(::core::mem::size_of::<crate::Bools>() == 1);
    assert!(::core::mem::align_of::<crate::Bools>() == 1);
    static_assertions::assert_impl_all!(crate::Bools: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Bools: Drop);
    assert!(::core::mem::offset_of!(crate::Bools, b_) == 0);
};
