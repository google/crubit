// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for function 'apply':
// while generating bridge param 'callback': DynCallable is not supported yet

// Error while generating bindings for function 'apply_mut':
// while generating bridge param 'callback': DynCallable is not supported yet

// Error while generating bindings for function 'apply_once':
// while generating bridge param 'callback': DynCallable is not supported yet

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=NotCABICompatible
pub struct NotCABICompatible {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) private_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for NotCABICompatible {}
impl !Sync for NotCABICompatible {}
unsafe impl ::cxx::ExternType for NotCABICompatible {
    type Id = ::cxx::type_id!("NotCABICompatible");
    type Kind = ::cxx::kind::Trivial;
}
impl NotCABICompatible {
    #[inline(always)]
    pub unsafe fn get(__this: *const Self) -> ::ffi_11::c_int {
        crate::detail::__rust_thunk___ZNK17NotCABICompatible3getEv(__this)
    }
}

impl From<::ffi_11::c_int> for NotCABICompatible {
    #[inline(always)]
    fn from(x: ::ffi_11::c_int) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17NotCABICompatibleC1Ei(&raw mut tmp as *mut _, x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for NotCABICompatible {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

// Error while generating bindings for function 'rust_inspect_non_c_abi_compatible_struct':
// while generating bridge param 'cb': DynCallable is not supported yet

// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_cc needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_cc needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN17NotCABICompatibleC1Ei(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK17NotCABICompatible3getEv(
            __this: *const crate::NotCABICompatible,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::NotCABICompatible>() == 4);
    assert!(::core::mem::align_of::<crate::NotCABICompatible>() == 4);
    static_assertions::assert_impl_all!(crate::NotCABICompatible: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NotCABICompatible: Drop);
    assert!(::core::mem::offset_of!(crate::NotCABICompatible, private_) == 0);
};
