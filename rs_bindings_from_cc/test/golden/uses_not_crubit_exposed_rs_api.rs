// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:uses_not_crubit_exposed_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

// error: function `UseNotCrubitExposed` could not be bound
//   Unsupported parameter type `NotCrubitExposed not_crubit_exposed`:
//     Crubit is not enabled on defining target:
//       rs_bindings_from_cc/test/golden/not_crubit_exposed.h

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=:: CannotUpcastInCrubit
pub struct CannotUpcastInCrubit {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for CannotUpcastInCrubit {}
impl !Sync for CannotUpcastInCrubit {}
unsafe impl ::cxx::ExternType for CannotUpcastInCrubit {
    type Id = ::cxx::type_id!(":: CannotUpcastInCrubit");
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

pub mod c9 { // error: class `c9::Co` could not be bound
             //   Class templates are not yet supported
}

// namespace c9

// error: function `ReturnsCo` could not be bound
//   Cannot use an error type `c9 :: Co < NotCrubitExposed >` by value:
//     `c9::Co<NotCrubitExposed>` is unsupported because `NotCrubitExposed` is unavailable:
//     Crubit is not enabled on defining target:
//         rs_bindings_from_cc/test/golden/not_crubit_exposed.h

// error: class `c9::Co<NotCrubitExposed>` could not be bound
//   `c9::Co<NotCrubitExposed>` is unsupported because `NotCrubitExposed` is unavailable:
//   Crubit is not enabled on defining target:
//       rs_bindings_from_cc/test/golden/not_crubit_exposed.h

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
