// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=type
pub struct r#type {
    pub r#dyn: ::core::ffi::c_int,
}
impl !Send for r#type {}
impl !Sync for r#type {}
unsafe impl ::cxx::ExternType for r#type {
    type Id = ::cxx::type_id!("type");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for r#type {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4typeC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'type::type':
// Can't generate bindings for type::type, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc needs [//features:experimental] for type::type (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'type::type':
// Can't generate bindings for type::type, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc needs [//features:experimental] for type::type (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'type::operator=':
// Can't generate bindings for type::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc needs [//features:experimental] for type::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc needs [//features:experimental] for type::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'type::operator=':
// Can't generate bindings for type::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc needs [//features:experimental] for type::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc needs [//features:experimental] for type::operator= (the type of __param_0 (parameter #1): references are not supported)

#[inline(always)]
pub fn r#impl(r#match: ::core::ffi::c_int) {
    unsafe { crate::detail::__rust_thunk___Z4impli(r#match) }
}

// Error while generating bindings for class 'await':
// Class templates are not supported yet

// Error while generating bindings for function 'become':
// Function templates are not supported yet

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN4typeC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_Z4impli"]
        pub(crate) unsafe fn __rust_thunk___Z4impli(r#match: ::core::ffi::c_int);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::r#type>() == 4);
    assert!(::core::mem::align_of::<crate::r#type>() == 4);
    static_assertions::assert_impl_all!(crate::r#type: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::r#type: Drop);
    assert!(::core::mem::offset_of!(crate::r#type, r#dyn) == 0);
};
