// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_template_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for class 'MyOption':
// Class templates are not supported yet

// Error while generating bindings for class 'Value':
// Class templates are not supported yet

#[inline(always)]
pub fn ReturnsValue() -> crate::MyOption<crate::__CcTemplateInst5ValueIiE> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::__CcTemplateInst5ValueIiE>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::__CcTemplateInst5ValueIiE>>,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z12ReturnsValuev(__return_abi_buffer,); })
    }
}

/// A basic templated type that does nothing fancy.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Value < int >
pub(crate) struct __CcTemplateInst5ValueIiE {
    pub value: ::ffi_11::c_int,
}
impl !Send for __CcTemplateInst5ValueIiE {}
impl !Sync for __CcTemplateInst5ValueIiE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("Value < int >"),
    crate::__CcTemplateInst5ValueIiE
);

// Error while generating bindings for constructor 'Value<int>::Value<int>':
// Can't generate bindings for Value<int>::Value<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_template_type_cc needs [//features:experimental] for Value<int>::Value<int> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for constructor 'Value<int>::Value<int>':
// Can't generate bindings for Value<int>::Value<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_template_type_cc needs [//features:experimental] for Value<int>::Value<int> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for constructor 'Value<int>::Value<int>':
// Can't generate bindings for Value<int>::Value<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_template_type_cc needs [//features:experimental] for Value<int>::Value<int> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for function 'Value<int>::operator=':
// Can't generate bindings for Value<int>::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_template_type_cc needs [//features:experimental] for Value<int>::operator= (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for function 'Value<int>::operator=':
// Can't generate bindings for Value<int>::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_template_type_cc needs [//features:experimental] for Value<int>::operator= (b/248542210: template instantiation of member function cannot reliably get bindings)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z12ReturnsValuev(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst5ValueIiE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst5ValueIiE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst5ValueIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst5ValueIiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst5ValueIiE, value) == 0);
};
