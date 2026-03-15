// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:simple_string_view
// Features: assume_lifetimes, assume_this_lifetimes, callables, check_default_initialized, experimental, fmt, supported, types, unsafe_view, wrapper

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/simple_string_view.h;l=10
#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SV
pub struct SV<'a> {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    __marker_a: ::core::marker::PhantomData<&'a ()>,
}
impl<'a> !Send for SV<'a> {}
impl<'a> !Sync for SV<'a> {}
unsafe impl<'a> ::cxx::ExternType for SV<'a> {
    type Id = ::cxx::type_id!("SV");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("SV"), crate::SV<'_>);

/// Generated from: rs_bindings_from_cc/test/assume_lifetimes/simple_string_view.h;l=10
impl<'a> Default for SV<'a> {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN2SVC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN2SVC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SV>() == 1);
    assert!(::core::mem::align_of::<crate::SV>() == 1);
    static_assertions::assert_impl_all!(crate::SV: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SV: Drop);
};
