// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:string_view

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
pub fn string_view_sink<'s>(s: ::cc_std::std::string_view<'s>) {
    let mut s = ::core::mem::MaybeUninit::new(s);
    unsafe {
        crate::detail::__rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(s.as_mut_ptr())
    }
}

#[inline(always)]
pub fn string_view_return<'s>(s: ::cc_std::std::string_view<'s>) -> ::cc_std::std::string_view<'s> {
    let mut s = ::core::mem::MaybeUninit::new(s);
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<::cc_std::std::string_view<'s>>::uninit();
        crate::detail::__rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(&raw mut __crubit_return as*mut::core::ffi::c_void,s.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn ambiguous_string_view_return<'a, 'b>(
    a: ::cc_std::std::string_view<'a>,
    b: ::cc_std::std::string_view<'b>,
) -> ::cc_std::std::__u::raw_string_view {
    let mut a = ::core::mem::MaybeUninit::new(a);
    let mut b = ::core::mem::MaybeUninit::new(b);
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<::cc_std::std::__u::raw_string_view>::uninit();
        crate::detail::__rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(&raw mut __crubit_return as*mut::core::ffi::c_void,a.as_mut_ptr(),b.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn explicit_lifetime_string_view<'a>(x: ::cc_std::std::string_view<'a>) {
    let mut x = ::core::mem::MaybeUninit::new(x);
    unsafe {
        crate::detail::__rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(x.as_mut_ptr())
    }
}

#[inline(always)]
pub fn unambiguous_string_view_return_annotated<'a>(
    x: ::cc_std::std::string_view<'a>,
    y: ::cc_std::std::string_view<'a>,
) -> ::cc_std::std::string_view<'a> {
    let mut x = ::core::mem::MaybeUninit::new(x);
    let mut y = ::core::mem::MaybeUninit::new(y);
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<::cc_std::std::string_view<'a>>::uninit();
        crate::detail::__rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(&raw mut __crubit_return as*mut::core::ffi::c_void,x.as_mut_ptr(),y.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> *const ::ffi_11::c_char {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ip_kc_ee::base(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__046f2d88__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*const ::ffi_11::c_char>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *const ::ffi_11::c_char) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__9d7a113a__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const ::ffi_11::c_char>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const ::ffi_11::c_char) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const ::ffi_11::c_char>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ip_kc_ee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
    ) -> *const ::ffi_11::c_char {
        unsafe {
            crate::detail::__rust_thunk__74bf6f6f__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const wchar_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__046f2d88__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            's,
        >(
            s: *mut ::cc_std::std::string_view<'s>,
        );
        pub(crate) unsafe fn __rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            's,
        >(
            __return: *mut ::core::ffi::c_void,
            s: *mut ::cc_std::std::string_view<'s>,
        );
        pub(crate) unsafe fn __rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_<
            'a,
            'b,
        >(
            __return: *mut ::core::ffi::c_void,
            a: *mut ::cc_std::std::string_view<'a>,
            b: *mut ::cc_std::std::string_view<'b>,
        );
        pub(crate) unsafe fn __rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE<
            'a,
        >(
            x: *mut ::cc_std::std::string_view<'a>,
        );
        pub(crate) unsafe fn __rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_<
            'a,
        >(
            __return: *mut ::core::ffi::c_void,
            x: *mut ::cc_std::std::string_view<'a>,
            y: *mut ::cc_std::std::string_view<'a>,
        );
        pub(crate) unsafe fn __rust_thunk__046f2d88__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9d7a113a__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const ::ffi_11::c_char,
        );
        pub(crate) unsafe fn __rust_thunk__74bf6f6f__ZNKSt3__u16reverse_iteratorIPKcE4baseEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
        ) -> *const ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__046f2d88__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE, current)
            == 0
    );
};
