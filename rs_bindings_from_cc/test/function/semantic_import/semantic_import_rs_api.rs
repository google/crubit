// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/semantic_import:semantic_import

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
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
///CRUBIT_ANNOTATE: cpp_type=S
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct S {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) x_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for S {}
impl !Sync for S {}
unsafe impl ::cxx::ExternType for S {
    type Id = ::cxx::type_id!("S");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("S"), crate::S);
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
        unsafe { (&raw const *__this).cast::<::ffi_11::c_int>().read() }
    }
    #[inline(always)]
    pub(crate) fn get_x<'__this>(__this: &'__this mut crate::S) -> ::ffi_11::c_int {
        unsafe { (&raw const *__this).cast::<::ffi_11::c_int>().read() }
    }
    #[inline(always)]
    pub(crate) fn set_x<'__this>(__this: &'__this mut crate::S, x: ::ffi_11::c_int) {
        unsafe { (&raw mut *__this).cast::<::ffi_11::c_int>().write(x) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "1T"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=T
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct T {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) y_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for T {}
impl !Sync for T {}
unsafe impl ::cxx::ExternType for T {
    type Id = ::cxx::type_id!("T");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("T"), crate::T);
impl T {
    #[inline(always)]
    pub fn y<'__this>(&'__this self) -> f32 {
        unsafe { self::t::y(self) }
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

pub mod t {
    #[inline(always)]
    pub(crate) fn y<'__this>(__this: &'__this crate::T) -> f32 {
        unsafe { (&raw const *__this).byte_add(4).cast::<f32>().read() }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Chars"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Chars
///CRUBIT_ANNOTATE: cpp_move_constructible=
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
    type Id = ::cxx::type_id!("Chars");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Chars"), crate::Chars);
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
        unsafe { (&raw const *__this).cast::<::ffi_11::c_char>().read() }
    }
    #[inline(always)]
    pub(crate) fn sc<'__this>(__this: &'__this mut crate::Chars) -> ::ffi_11::c_schar {
        unsafe { (&raw const *__this).byte_add(1).cast::<::ffi_11::c_schar>().read() }
    }
    #[inline(always)]
    pub(crate) fn uc<'__this>(__this: &'__this mut crate::Chars) -> ::ffi_11::c_uchar {
        unsafe { (&raw const *__this).byte_add(2).cast::<::ffi_11::c_uchar>().read() }
    }
    #[inline(always)]
    pub(crate) fn set_c<'__this>(__this: &'__this mut crate::Chars, c: ::ffi_11::c_char) {
        unsafe { (&raw mut *__this).cast::<::ffi_11::c_char>().write(c) }
    }
    #[inline(always)]
    pub(crate) fn set_sc<'__this>(__this: &'__this mut crate::Chars, sc: ::ffi_11::c_schar) {
        unsafe { (&raw mut *__this).byte_add(1).cast::<::ffi_11::c_schar>().write(sc) }
    }
    #[inline(always)]
    pub(crate) fn set_uc<'__this>(__this: &'__this mut crate::Chars, uc: ::ffi_11::c_uchar) {
        unsafe { (&raw mut *__this).byte_add(2).cast::<::ffi_11::c_uchar>().write(uc) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Bools"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Bools
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Bools {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Bools {}
impl !Sync for Bools {}
unsafe impl ::cxx::ExternType for Bools {
    type Id = ::cxx::type_id!("Bools");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Bools"), crate::Bools);
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
        unsafe { (&raw const *__this).cast::<bool>().read() }
    }
    #[inline(always)]
    pub(crate) fn set_b<'__this>(__this: &'__this mut crate::Bools, b: bool) {
        unsafe { (&raw mut *__this).cast::<bool>().write(b) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "8Pointers"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Pointers
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Pointers {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) p_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) mut_p_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for Pointers {}
impl !Sync for Pointers {}
unsafe impl ::cxx::ExternType for Pointers {
    type Id = ::cxx::type_id!("Pointers");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Pointers"), crate::Pointers);
impl Pointers {
    #[inline(always)]
    pub fn p<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::pointers::p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_p<'__this>(&'__this mut self, p: *const ::ffi_11::c_int) {
        unsafe { self::pointers::set_p(self, p) }
    }
    #[inline(always)]
    pub fn mut_p<'__this>(&'__this self) -> *mut ::ffi_11::c_int {
        unsafe { self::pointers::mut_p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `mut_p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_mut_p<'__this>(&'__this mut self, mut_p: *mut ::ffi_11::c_int) {
        unsafe { self::pointers::set_mut_p(self, mut_p) }
    }
}

impl Default for Pointers {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8PointersC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod pointers {
    #[inline(always)]
    pub(crate) fn p<'__this>(__this: &'__this crate::Pointers) -> *const ::ffi_11::c_int {
        unsafe { (&raw const *__this).cast::<*const ::ffi_11::c_int>().read() }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_p<'__this>(
        __this: &'__this mut crate::Pointers,
        p: *const ::ffi_11::c_int,
    ) {
        unsafe { (&raw mut *__this).cast::<*const ::ffi_11::c_int>().write(p) }
    }
    #[inline(always)]
    pub(crate) fn mut_p<'__this>(__this: &'__this crate::Pointers) -> *mut ::ffi_11::c_int {
        unsafe { (&raw const *__this).byte_add(8).cast::<*mut ::ffi_11::c_int>().read() }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `mut_p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_mut_p<'__this>(
        __this: &'__this mut crate::Pointers,
        mut_p: *mut ::ffi_11::c_int,
    ) {
        unsafe { (&raw mut *__this).byte_add(8).cast::<*mut ::ffi_11::c_int>().write(mut_p) }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "10NonTrivial"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=NonTrivial
pub struct NonTrivial {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) p_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) mut_p_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for NonTrivial {}
impl !Sync for NonTrivial {}
unsafe impl ::cxx::ExternType for NonTrivial {
    type Id = ::cxx::type_id!("NonTrivial");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonTrivial"), crate::NonTrivial);
impl NonTrivial {
    #[inline(always)]
    pub fn p<'__this>(&'__this self) -> *const crate::NonTrivial {
        unsafe { self::non_trivial::p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_p<'__this>(self: ::core::pin::Pin<&'__this mut Self>, p: *const Self) {
        unsafe { self::non_trivial::set_p(self, p) }
    }
    #[inline(always)]
    pub fn mut_p<'__this>(self: ::core::pin::Pin<&'__this mut Self>) -> *mut crate::NonTrivial {
        unsafe { self::non_trivial::mut_p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `mut_p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_mut_p<'__this>(self: ::core::pin::Pin<&'__this mut Self>, mut_p: *mut Self) {
        unsafe { self::non_trivial::set_mut_p(self, mut_p) }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for NonTrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let mut __param_0 = args;
                crate::detail::__rust_thunk___ZN10NonTrivialC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for NonTrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for NonTrivial {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NonTrivialaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::CtorNew<()> for NonTrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let () = args;
                crate::detail::__rust_thunk___ZN10NonTrivialC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

/// A user-provided destructor is what makes this type non-trivial (and so
/// `!Unpin` in Rust).
/// `= default` would defeat the purpose.
impl ::ctor::PinnedDrop for NonTrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10NonTrivialD1Ev(self) }
    }
}

pub mod non_trivial {
    #[inline(always)]
    pub(crate) fn p<'__this>(__this: &'__this crate::NonTrivial) -> *const crate::NonTrivial {
        unsafe { (&raw const *__this).cast::<*const crate::NonTrivial>().read() }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_p<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
        p: *const crate::NonTrivial,
    ) {
        unsafe {
            (&raw mut *::core::pin::Pin::into_inner_unchecked(__this))
                .cast::<*const crate::NonTrivial>()
                .write(p)
        }
    }
    #[inline(always)]
    pub(crate) fn mut_p<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
    ) -> *mut crate::NonTrivial {
        unsafe { (&raw const *__this).byte_add(8).cast::<*mut crate::NonTrivial>().read() }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `mut_p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_mut_p<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
        mut_p: *mut crate::NonTrivial,
    ) {
        unsafe {
            (&raw mut *::core::pin::Pin::into_inner_unchecked(__this))
                .byte_add(8)
                .cast::<*mut crate::NonTrivial>()
                .write(mut_p)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "7Padding"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Padding
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Padding {
    pub pad_: ::ffi_11::c_long,
}
impl !Send for Padding {}
impl !Sync for Padding {}
unsafe impl ::cxx::ExternType for Padding {
    type Id = ::cxx::type_id!("Padding");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Padding"), crate::Padding);

impl Default for Padding {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7PaddingC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "11PointerBase"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PointerBase
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct PointerBase {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) base_p_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for PointerBase {}
impl !Sync for PointerBase {}
unsafe impl ::cxx::ExternType for PointerBase {
    type Id = ::cxx::type_id!("PointerBase");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("PointerBase"), crate::PointerBase);
impl PointerBase {
    #[inline(always)]
    pub fn base_p<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::pointer_base::base_p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `base_p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_base_p<'__this>(&'__this mut self, base_p: *const ::ffi_11::c_int) {
        unsafe { self::pointer_base::set_base_p(self, base_p) }
    }
}

impl Default for PointerBase {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11PointerBaseC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod pointer_base {
    #[inline(always)]
    pub(crate) fn base_p<'__this>(__this: &'__this crate::PointerBase) -> *const ::ffi_11::c_int {
        unsafe { (&raw const *__this).cast::<*const ::ffi_11::c_int>().read() }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `base_p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_base_p<'__this>(
        __this: &'__this mut crate::PointerBase,
        base_p: *const ::ffi_11::c_int,
    ) {
        unsafe { (&raw mut *__this).cast::<*const ::ffi_11::c_int>().write(base_p) }
    }
}

/// `PointerBase` is at a non-zero offset within `DerivedPointers`, so the
/// accessors declared here must not reuse `PointerBase`'s offset for `base_p_`.
/// They therefore keep using a thunk, unlike `PointerBase`'s own accessors.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "15DerivedPointers"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=DerivedPointers
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct DerivedPointers {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 16],
}
impl !Send for DerivedPointers {}
impl !Sync for DerivedPointers {}
unsafe impl ::cxx::ExternType for DerivedPointers {
    type Id = ::cxx::type_id!("DerivedPointers");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("DerivedPointers"),
    crate::DerivedPointers
);
impl DerivedPointers {
    #[inline(always)]
    pub fn p<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::derived_pointers::p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_p<'__this>(&'__this mut self, p: *const ::ffi_11::c_int) {
        unsafe { self::derived_pointers::set_p(self, p) }
    }
}

impl Default for DerivedPointers {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15DerivedPointersC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod derived_pointers {
    #[inline(always)]
    pub(crate) fn p<'__this>(__this: &'__this crate::DerivedPointers) -> *const ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK15DerivedPointers1pEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_p<'__this>(
        __this: &'__this mut crate::DerivedPointers,
        p: *const ::ffi_11::c_int,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN15DerivedPointers5set_pEPKi(__this, p) }
    }
}

/// `NonStandardLayoutPointers` is not standard-layout because its fields have
/// different access control. Under pointer field protection (PFP), its pointer
/// fields may be signed and authenticated by C++ loads and stores, so `p` and
/// `set_p` must keep using a thunk, while the non-pointer `x` and `set_x` remain
/// thunkless.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "25NonStandardLayoutPointers"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=NonStandardLayoutPointers
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct NonStandardLayoutPointers {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub public_field_: ::ffi_11::c_int,
    __padding1: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) p_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) x_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for NonStandardLayoutPointers {}
impl !Sync for NonStandardLayoutPointers {}
unsafe impl ::cxx::ExternType for NonStandardLayoutPointers {
    type Id = ::cxx::type_id!("NonStandardLayoutPointers");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NonStandardLayoutPointers"),
    crate::NonStandardLayoutPointers
);
impl NonStandardLayoutPointers {
    #[inline(always)]
    pub fn p<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::non_standard_layout_pointers::p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_p<'__this>(&'__this mut self, p: *const ::ffi_11::c_int) {
        unsafe { self::non_standard_layout_pointers::set_p(self, p) }
    }
    #[inline(always)]
    pub fn x<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe { self::non_standard_layout_pointers::x(self) }
    }
    #[inline(always)]
    pub fn set_x<'__this>(&'__this mut self, x: ::ffi_11::c_int) {
        unsafe { self::non_standard_layout_pointers::set_x(self, x) }
    }
}

impl Default for NonStandardLayoutPointers {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25NonStandardLayoutPointersC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod non_standard_layout_pointers {
    #[inline(always)]
    pub(crate) fn p<'__this>(
        __this: &'__this crate::NonStandardLayoutPointers,
    ) -> *const ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK25NonStandardLayoutPointers1pEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_p<'__this>(
        __this: &'__this mut crate::NonStandardLayoutPointers,
        p: *const ::ffi_11::c_int,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN25NonStandardLayoutPointers5set_pEPKi(__this, p) }
    }
    #[inline(always)]
    pub(crate) fn x<'__this>(__this: &'__this crate::NonStandardLayoutPointers) -> ::ffi_11::c_int {
        unsafe { (&raw const *__this).byte_add(16).cast::<::ffi_11::c_int>().read() }
    }
    #[inline(always)]
    pub(crate) fn set_x<'__this>(
        __this: &'__this mut crate::NonStandardLayoutPointers,
        x: ::ffi_11::c_int,
    ) {
        unsafe { (&raw mut *__this).byte_add(16).cast::<::ffi_11::c_int>().write(x) }
    }
}

forward_declare::forward_declare!(pub Incomplete = forward_declare::symbol!("Incomplete"));

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "12MorePointers"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=MorePointers
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct MorePointers {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) v_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for MorePointers {}
impl !Sync for MorePointers {}
unsafe impl ::cxx::ExternType for MorePointers {
    type Id = ::cxx::type_id!("MorePointers");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("MorePointers"), crate::MorePointers);
impl MorePointers {
    #[inline(always)]
    pub fn v<'__this>(&'__this self) -> *mut ::ffi_11::c_void {
        unsafe { self::more_pointers::v(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `v`: raw pointer
    #[inline(always)]
    pub unsafe fn set_v<'__this>(&'__this mut self, v: *mut ::ffi_11::c_void) {
        unsafe { self::more_pointers::set_v(self, v) }
    }
    #[inline(always)]
    pub fn i<'__this>(&'__this self) -> *mut crate::Incomplete {
        unsafe { self::more_pointers::i(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `i`: raw pointer
    #[inline(always)]
    pub unsafe fn set_i<'__this>(&'__this mut self, i: *mut crate::Incomplete) {
        unsafe { self::more_pointers::set_i(self, i) }
    }
}

impl Default for MorePointers {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12MorePointersC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod more_pointers {
    #[inline(always)]
    pub(crate) fn v<'__this>(__this: &'__this crate::MorePointers) -> *mut ::ffi_11::c_void {
        unsafe { (&raw const *__this).cast::<*mut ::ffi_11::c_void>().read() }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `v`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_v<'__this>(
        __this: &'__this mut crate::MorePointers,
        v: *mut ::ffi_11::c_void,
    ) {
        unsafe { (&raw mut *__this).cast::<*mut ::ffi_11::c_void>().write(v) }
    }
    #[inline(always)]
    pub(crate) fn i<'__this>(__this: &'__this crate::MorePointers) -> *mut crate::Incomplete {
        unsafe { (&raw const *__this).byte_add(8).cast::<*mut crate::Incomplete>().read() }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `i`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_i<'__this>(
        __this: &'__this mut crate::MorePointers,
        i: *mut crate::Incomplete,
    ) {
        unsafe { (&raw mut *__this).byte_add(8).cast::<*mut crate::Incomplete>().write(i) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "29StructWithRecordAndFreeGetter"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=StructWithRecordAndFreeGetter
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct StructWithRecordAndFreeGetter {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) s_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for StructWithRecordAndFreeGetter {}
impl !Sync for StructWithRecordAndFreeGetter {}
unsafe impl ::cxx::ExternType for StructWithRecordAndFreeGetter {
    type Id = ::cxx::type_id!("StructWithRecordAndFreeGetter");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithRecordAndFreeGetter"),
    crate::StructWithRecordAndFreeGetter
);
impl StructWithRecordAndFreeGetter {
    #[inline(always)]
    pub fn s<'__this>(&'__this self) -> crate::S {
        unsafe { self::struct_with_record_and_free_getter::s(self) }
    }
}

impl From<::ffi_11::c_int> for StructWithRecordAndFreeGetter {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN29StructWithRecordAndFreeGetterC1Ei(
                &raw mut tmp as *mut _,
                x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for StructWithRecordAndFreeGetter {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

#[inline(always)]
pub fn free_get_s<'__param_0>(
    __param_0: &'__param_0 crate::StructWithRecordAndFreeGetter,
) -> crate::S {
    unsafe { (&raw const *__param_0).cast::<crate::S>().read() }
}

pub mod struct_with_record_and_free_getter {
    #[inline(always)]
    pub(crate) fn s<'__this>(__this: &'__this crate::StructWithRecordAndFreeGetter) -> crate::S {
        unsafe { (&raw const *__this).cast::<crate::S>().read() }
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
        pub(crate) unsafe fn __rust_thunk___ZN8PointersC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::NonTrivial,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
            __param_0: &'__param_0 crate::NonTrivial,
        ) -> ::core::pin::Pin<&'__this mut crate::NonTrivial>;
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN7PaddingC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN11PointerBaseC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN15DerivedPointersC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK15DerivedPointers1pEv<'__this>(
            __this: &'__this crate::DerivedPointers,
        ) -> *const ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN15DerivedPointers5set_pEPKi<'__this>(
            __this: &'__this mut crate::DerivedPointers,
            p: *const ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25NonStandardLayoutPointersC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK25NonStandardLayoutPointers1pEv<'__this>(
            __this: &'__this crate::NonStandardLayoutPointers,
        ) -> *const ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN25NonStandardLayoutPointers5set_pEPKi<'__this>(
            __this: &'__this mut crate::NonStandardLayoutPointers,
            p: *const ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN12MorePointersC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN29StructWithRecordAndFreeGetterC1Ei(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
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
    assert!(::core::mem::size_of::<crate::Pointers>() == 16);
    assert!(::core::mem::align_of::<crate::Pointers>() == 8);
    static_assertions::assert_impl_all!(crate::Pointers: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Pointers: Drop);
    assert!(::core::mem::offset_of!(crate::Pointers, p_) == 0);
    assert!(::core::mem::offset_of!(crate::Pointers, mut_p_) == 8);
    assert!(::core::mem::size_of::<crate::NonTrivial>() == 16);
    assert!(::core::mem::align_of::<crate::NonTrivial>() == 8);
    static_assertions::assert_impl_all!(crate::NonTrivial: Drop);
    static_assertions::assert_not_impl_any!(crate::NonTrivial: Copy);
    assert!(::core::mem::offset_of!(crate::NonTrivial, p_) == 0);
    assert!(::core::mem::offset_of!(crate::NonTrivial, mut_p_) == 8);
    assert!(::core::mem::size_of::<crate::Padding>() == 8);
    assert!(::core::mem::align_of::<crate::Padding>() == 8);
    static_assertions::assert_impl_all!(crate::Padding: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Padding: Drop);
    assert!(::core::mem::offset_of!(crate::Padding, pad_) == 0);
    assert!(::core::mem::size_of::<crate::PointerBase>() == 8);
    assert!(::core::mem::align_of::<crate::PointerBase>() == 8);
    static_assertions::assert_impl_all!(crate::PointerBase: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::PointerBase: Drop);
    assert!(::core::mem::offset_of!(crate::PointerBase, base_p_) == 0);
    assert!(::core::mem::size_of::<crate::DerivedPointers>() == 16);
    assert!(::core::mem::align_of::<crate::DerivedPointers>() == 8);
    static_assertions::assert_impl_all!(crate::DerivedPointers: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DerivedPointers: Drop);

    assert!(::core::mem::size_of::<crate::NonStandardLayoutPointers>() == 24);
    assert!(::core::mem::align_of::<crate::NonStandardLayoutPointers>() == 8);
    static_assertions::assert_impl_all!(crate::NonStandardLayoutPointers: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NonStandardLayoutPointers: Drop);
    assert!(::core::mem::offset_of!(crate::NonStandardLayoutPointers, public_field_) == 0);
    assert!(::core::mem::offset_of!(crate::NonStandardLayoutPointers, p_) == 8);
    assert!(::core::mem::offset_of!(crate::NonStandardLayoutPointers, x_) == 16);
    assert!(::core::mem::size_of::<crate::MorePointers>() == 16);
    assert!(::core::mem::align_of::<crate::MorePointers>() == 8);
    static_assertions::assert_impl_all!(crate::MorePointers: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MorePointers: Drop);
    assert!(::core::mem::offset_of!(crate::MorePointers, v_) == 0);
    assert!(::core::mem::offset_of!(crate::MorePointers, i_) == 8);
    assert!(::core::mem::size_of::<crate::StructWithRecordAndFreeGetter>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithRecordAndFreeGetter>() == 4);
    static_assertions::assert_impl_all!(crate::StructWithRecordAndFreeGetter: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithRecordAndFreeGetter: Drop);
    assert!(::core::mem::offset_of!(crate::StructWithRecordAndFreeGetter, s_) == 0);
};
