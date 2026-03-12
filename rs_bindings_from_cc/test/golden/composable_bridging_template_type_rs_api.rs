// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_template_type_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

// error: class `MyOption` could not be bound
//   Class templates are not yet supported

// error: class `Value` could not be bound
//   Class templates are not yet supported

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

// error: constructor `Value<int>::Value<int>` could not be bound
//   b/248542210: template instantiation of member function cannot reliably get bindings

// error: constructor `Value<int>::Value<int>` could not be bound
//   b/248542210: template instantiation of member function cannot reliably get bindings

// error: constructor `Value<int>::Value<int>` could not be bound
//   b/248542210: template instantiation of member function cannot reliably get bindings

// error: function `Value<int>::operator=` could not be bound
//   b/248542210: template instantiation of member function cannot reliably get bindings

// error: function `Value<int>::operator=` could not be bound
//   b/248542210: template instantiation of member function cannot reliably get bindings

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
