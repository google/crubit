// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:free_function

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
//
// int& increment_int_ref(int& a);
// void bad_lifetime_name(int& type);

#[derive(Clone, Copy)]
#[cfi_encoding = "1C"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=C
pub struct C<'a> {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    __marker_a: ::core::marker::PhantomData<&'a ()>,
}
impl<'a> !Send for C<'a> {}
impl<'a> !Sync for C<'a> {}
unsafe impl<'a> ::cxx::ExternType for C<'a> {
    type Id = ::cxx::type_id!("C");
    type Kind = ::cxx::kind::Trivial;
}
impl<'a> C<'a> {
    #[inline(always)]
    pub fn f<'__this>(&'__this mut self) -> ::cref::CMut<'a, ::ffi_11::c_int> {
        unsafe { self::c::f(self) }
    }
}

impl<'a> Default for C<'a> {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1CC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod c {
    #[inline(always)]
    pub(crate) fn f<'__this, 'a>(
        __this: &'__this mut crate::C<'a>,
    ) -> ::cref::CMut<'a, ::ffi_11::c_int> {
        unsafe { crate::detail::__rust_thunk___ZN1C1fEv(__this) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN1CC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN1C1fEv"]
        pub(crate) unsafe fn __rust_thunk___ZN1C1fEv<'__this, 'a>(
            __this: &'__this mut crate::C<'a>,
        ) -> ::cref::CMut<'a, ::ffi_11::c_int>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::C>() == 1);
    assert!(::core::mem::align_of::<crate::C>() == 1);
    static_assertions::assert_impl_all!(crate::C<'static>: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::C<'static>: Drop);
};
