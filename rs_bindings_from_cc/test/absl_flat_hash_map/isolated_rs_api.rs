// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/absl_flat_hash_map:isolated

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

pub mod absl { // error: class `absl::flat_hash_map` could not be bound
               //   Class templates are not yet supported
}

// namespace absl

pub mod crubit {
    pub mod test {
        pub type MyMap = crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE;
    }
}

// namespace crubit::test

/// A stub implementation of absl::flat_hash_map to test code generation without
/// the absl dependency.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: absl :: flat_hash_map < int , unsigned long , 42 >
pub struct __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {}
impl !Sync for __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: absl :: flat_hash_map < int , unsigned long , 42 >"),
    crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE
);
impl __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {
    #[inline(always)]
    pub fn FunctionRemovedByOverride<'__this>(&'__this self) {
        unsafe {
            self::cc_template_inst_n4absl13flat_hash_map_iim_li42_eee::FunctionRemovedByOverride(
                self,
            )
        }
    }
}

impl Default for __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4absl13flat_hash_mapIimLi42EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

// error: function `absl::flat_hash_map<int, unsigned long, 42>::HarmlessTemplateFunction` could not be bound
//   Function templates are not yet supported

pub mod cc_template_inst_n4absl13flat_hash_map_iim_li42_eee {
    #[inline(always)]
    pub(crate) fn FunctionRemovedByOverride<'__this>(
        __this: &'__this crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZNK4absl13flat_hash_mapIimLi42EE25FunctionRemovedByOverrideEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(__this)
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN4absl13flat_hash_mapIimLi42EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK4absl13flat_hash_mapIimLi42EE25FunctionRemovedByOverrideEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE: Drop);
};
