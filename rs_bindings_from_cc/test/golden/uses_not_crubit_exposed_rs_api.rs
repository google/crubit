// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:uses_not_crubit_exposed_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for function 'UseNotCrubitExposed':
// Can't generate bindings for UseNotCrubitExposed, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:uses_not_crubit_exposed_cc needs [//features:wrapper] for UseNotCrubitExposed (the type of not_crubit_exposed (parameter #0): error: Can't generate bindings for NotCrubitExposed, because of missing required features (<internal link>):
// rs_bindings_from_cc/test/golden/not_crubit_exposed.h needs [//features:supported] for NotCrubitExposed)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=CannotUpcastInCrubit
pub struct CannotUpcastInCrubit {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for CannotUpcastInCrubit {}
impl !Sync for CannotUpcastInCrubit {}
unsafe impl ::cxx::ExternType for CannotUpcastInCrubit {
    type Id = ::cxx::type_id!("CannotUpcastInCrubit");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for CannotUpcastInCrubit {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20CannotUpcastInCrubitC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN20CannotUpcastInCrubitC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::CannotUpcastInCrubit>() == 4);
    assert!(::core::mem::align_of::<crate::CannotUpcastInCrubit>() == 4);
    static_assertions::assert_impl_all!(crate::CannotUpcastInCrubit: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::CannotUpcastInCrubit: Drop);
};
