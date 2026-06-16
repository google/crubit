// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:template_instantiation_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(rust_2024_compatibility)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

// error: class `TS` could not be bound
//   Class templates are not yet supported

#[inline(always)]
pub fn RTS() -> crate::__CcTemplateInst2TSIiE {
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<crate::__CcTemplateInst2TSIiE>::uninit();
        crate::detail::__rust_thunk___Z3RTSv(&raw mut __crubit_return as *mut ::core::ffi::c_void);
        __crubit_return.assume_init()
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TS < int >
pub struct __CcTemplateInst2TSIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst2TSIiE {}
impl !Sync for __CcTemplateInst2TSIiE {}

impl Default for __CcTemplateInst2TSIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN2TSIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplate_5finstantiation_5fcc(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z3RTSv(__return: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN2TSIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplate_5finstantiation_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst2TSIiE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst2TSIiE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst2TSIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst2TSIiE: Drop);
};
