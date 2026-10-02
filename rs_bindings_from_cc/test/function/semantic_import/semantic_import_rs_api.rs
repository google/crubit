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
        unsafe { (*((&*__this as *const _ as *const u8).add(4) as *const f32)) as f32 }
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
        unsafe { (*((&*__this as *const _ as *const u8).add(0) as *const bool)) as bool }
    }
    #[inline(always)]
    pub(crate) fn set_b<'__this>(__this: &'__this mut crate::Bools, b: bool) {
        unsafe { *((__this as *mut _ as *mut u8).add(0) as *mut bool) = (b as bool) }
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
        unsafe { (*((&*__this as *const _ as *const u8).add(0) as *const *const ::ffi_11::c_int)) }
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
        unsafe { *((__this as *mut _ as *mut u8).add(0) as *mut *const ::ffi_11::c_int) = p }
    }
    #[inline(always)]
    pub(crate) fn mut_p<'__this>(__this: &'__this crate::Pointers) -> *mut ::ffi_11::c_int {
        unsafe { (*((&*__this as *const _ as *const u8).add(8) as *const *mut ::ffi_11::c_int)) }
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
        unsafe { *((__this as *mut _ as *mut u8).add(8) as *mut *mut ::ffi_11::c_int) = mut_p }
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
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NonTrivialC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for NonTrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
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
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
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
        unsafe {
            (*((&*__this as *const _ as *const u8).add(0) as *const *const crate::NonTrivial))
        }
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
            *((::core::pin::Pin::into_inner_unchecked(__this) as *mut _ as *mut u8).add(0)
                as *mut *const crate::NonTrivial) = p
        }
    }
    #[inline(always)]
    pub(crate) fn mut_p<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
    ) -> *mut crate::NonTrivial {
        unsafe { (*((&*__this as *const _ as *const u8).add(8) as *const *mut crate::NonTrivial)) }
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
            *((::core::pin::Pin::into_inner_unchecked(__this) as *mut _ as *mut u8).add(8)
                as *mut *mut crate::NonTrivial) = mut_p
        }
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
        unsafe { (*((&*__this as *const _ as *const u8).add(0) as *const *mut ::ffi_11::c_void)) }
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
        unsafe { *((__this as *mut _ as *mut u8).add(0) as *mut *mut ::ffi_11::c_void) = v }
    }
    #[inline(always)]
    pub(crate) fn i<'__this>(__this: &'__this crate::MorePointers) -> *mut crate::Incomplete {
        unsafe { (*((&*__this as *const _ as *const u8).add(8) as *const *mut crate::Incomplete)) }
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
        unsafe { *((__this as *mut _ as *mut u8).add(8) as *mut *mut crate::Incomplete) = i }
    }
}

/// `rs_std::SliceRef` is not a raw pointer, so these accessors keep using a
/// thunk.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "6Slices"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Slices
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Slices {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) s_: [::core::mem::MaybeUninit<u8>; 16],
}
impl !Send for Slices {}
impl !Sync for Slices {}
unsafe impl ::cxx::ExternType for Slices {
    type Id = ::cxx::type_id!("Slices");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Slices"), crate::Slices);
impl Slices {
    #[inline(always)]
    pub fn s<'__this>(&'__this self) -> *const [::ffi_11::c_int] {
        unsafe { self::slices::s(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `s`: raw pointer
    #[inline(always)]
    pub unsafe fn set_s<'__this>(&'__this mut self, s: *const [::ffi_11::c_int]) {
        unsafe { self::slices::set_s(self, s) }
    }
}

impl Default for Slices {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6SlicesC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod slices {
    #[inline(always)]
    pub(crate) fn s<'__this>(__this: &'__this crate::Slices) -> *const [::ffi_11::c_int] {
        unsafe { crate::detail::__rust_thunk___ZNK6Slices1sEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `s`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_s<'__this>(
        __this: &'__this mut crate::Slices,
        s: *const [::ffi_11::c_int],
    ) {
        unsafe { crate::detail::__rust_thunk___ZN6Slices5set_sEN6rs_std8SliceRefIKiEE(__this, s) }
    }
}

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE = forward_declare::symbol!("std :: basic_string < char32_t , std :: char_traits < char32_t >, std :: pmr :: polymorphic_allocator < char32_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE = forward_declare::symbol!("std :: basic_string < char32_t , std :: char_traits < char32_t >, std :: allocator < char32_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE = forward_declare::symbol!("std :: basic_string < char16_t , std :: char_traits < char16_t >, std :: pmr :: polymorphic_allocator < char16_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE = forward_declare::symbol!("std :: basic_string < char16_t , std :: char_traits < char16_t >, std :: allocator < char16_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE = forward_declare::symbol!("std :: basic_string < char , std :: char_traits < char >, std :: pmr :: polymorphic_allocator < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("std :: basic_string < char , std :: char_traits < char >, std :: allocator < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_istreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("std :: basic_istream < char , std :: char_traits < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_istreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("std :: basic_istream < wchar_t , std :: char_traits < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_iostreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("std :: basic_iostream < char , std :: char_traits < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_iostreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("std :: basic_iostream < wchar_t , std :: char_traits < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u15basic_streambufIcNS_11char_traitsIcEEEE = forward_declare::symbol!("std :: basic_streambuf < char , std :: char_traits < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u15basic_streambufIwNS_11char_traitsIwEEEE = forward_declare::symbol!("std :: basic_streambuf < wchar_t , std :: char_traits < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_filebufIcNS_11char_traitsIcEEEE = forward_declare::symbol!("std :: basic_filebuf < char , std :: char_traits < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_filebufIwNS_11char_traitsIwEEEE = forward_declare::symbol!("std :: basic_filebuf < wchar_t , std :: char_traits < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_ifstreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("std :: basic_ifstream < char , std :: char_traits < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_ifstreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("std :: basic_ifstream < wchar_t , std :: char_traits < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_ofstreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("std :: basic_ofstream < char , std :: char_traits < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_ofstreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("std :: basic_ofstream < wchar_t , std :: char_traits < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_fstreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("std :: basic_fstream < char , std :: char_traits < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_fstreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("std :: basic_fstream < wchar_t , std :: char_traits < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u9basic_iosIcNS_11char_traitsIcEEEE = forward_declare::symbol!("std :: basic_ios < char , std :: char_traits < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u9basic_iosIwNS_11char_traitsIwEEEE = forward_declare::symbol!("std :: basic_ios < wchar_t , std :: char_traits < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("std :: basic_ostream < char , std :: char_traits < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_ostreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("std :: basic_ostream < wchar_t , std :: char_traits < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("std :: basic_stringbuf < char , std :: char_traits < char >, std :: allocator < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEE = forward_declare::symbol!("std :: basic_stringbuf < wchar_t , std :: char_traits < wchar_t >, std :: allocator < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("std :: basic_istringstream < char , std :: char_traits < char >, std :: allocator < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE = forward_declare::symbol!("std :: basic_istringstream < wchar_t , std :: char_traits < wchar_t >, std :: allocator < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("std :: basic_ostringstream < char , std :: char_traits < char >, std :: allocator < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE = forward_declare::symbol!("std :: basic_ostringstream < wchar_t , std :: char_traits < wchar_t >, std :: allocator < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("std :: basic_stringstream < char , std :: char_traits < char >, std :: allocator < char >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE = forward_declare::symbol!("std :: basic_stringstream < wchar_t , std :: char_traits < wchar_t >, std :: allocator < wchar_t >>"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u4fposI11__mbstate_tEE = forward_declare::symbol!("std :: fpos < __mbstate_t >"));

// Type bindings for rs_std::SliceRef<const int> suppressed due to being mapped to an existing Rust type (*const[::ffi_11::c_int])

// Type bindings for rs_std::SliceRef<int> suppressed due to being mapped to an existing Rust type (*mut[::ffi_11::c_int])

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
        pub(crate) unsafe fn __rust_thunk___ZN12MorePointersC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN6SlicesC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZNK6Slices1sEv<'__this>(
            __this: &'__this crate::Slices,
        ) -> *const [::ffi_11::c_int];
        pub(crate) unsafe fn __rust_thunk___ZN6Slices5set_sEN6rs_std8SliceRefIKiEE<'__this>(
            __this: &'__this mut crate::Slices,
            s: *const [::ffi_11::c_int],
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
    assert!(::core::mem::size_of::<crate::MorePointers>() == 16);
    assert!(::core::mem::align_of::<crate::MorePointers>() == 8);
    static_assertions::assert_impl_all!(crate::MorePointers: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MorePointers: Drop);
    assert!(::core::mem::offset_of!(crate::MorePointers, v_) == 0);
    assert!(::core::mem::offset_of!(crate::MorePointers, i_) == 8);
    assert!(::core::mem::size_of::<crate::Slices>() == 16);
    assert!(::core::mem::align_of::<crate::Slices>() == 8);
    static_assertions::assert_impl_all!(crate::Slices: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Slices: Drop);
    assert!(::core::mem::offset_of!(crate::Slices, s_) == 0);
    assert!(::core::mem::size_of::<*const [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*const [::ffi_11::c_int]>() == 8);
    assert!(::core::mem::size_of::<*mut [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*mut [::ffi_11::c_int]>() == 8);
};
