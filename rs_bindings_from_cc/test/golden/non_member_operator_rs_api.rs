// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:non_member_operator_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod ns {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=ns :: X
    pub struct X {
        pub f: ::core::ffi::c_int,
    }
    impl !Send for X {}
    impl !Sync for X {}
    unsafe impl ::cxx::ExternType for X {
        type Id = ::cxx::type_id!("ns :: X");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("ns :: X"), crate::ns::X);

    impl Default for X {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns1XC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
                tmp.assume_init()
            }
        }
    }
}

// namespace ns

impl PartialEq for crate::ns::X {
    #[inline(always)]
    fn eq(&self, b: &Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZeqN2ns1XES0_(&mut self.clone(), &mut b.clone()) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN2ns1XC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZeqN2ns1XES0_(
            a: &mut crate::ns::X,
            b: &mut crate::ns::X,
        ) -> bool;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ns::X>() == 4);
    assert!(::core::mem::align_of::<crate::ns::X>() == 4);
    static_assertions::assert_impl_all!(crate::ns::X: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ns::X: Drop);
    assert!(::core::mem::offset_of!(crate::ns::X, f) == 0);
};
