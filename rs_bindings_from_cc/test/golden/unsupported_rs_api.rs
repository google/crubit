// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsupported_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TrivialCustomType
pub struct TrivialCustomType {
    pub i: ::ffi_11::c_int,
}
impl !Send for TrivialCustomType {}
impl !Sync for TrivialCustomType {}
unsafe impl ::cxx::ExternType for TrivialCustomType {
    type Id = ::cxx::type_id!("TrivialCustomType");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for TrivialCustomType {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17TrivialCustomTypeC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: constructor `TrivialCustomType::TrivialCustomType` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `TrivialCustomType::TrivialCustomType` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `TrivialCustomType::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `TrivialCustomType::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `TrivialCustomType::operator||` could not be bound
//   Bindings for this kind of operator (operator || with 2 parameter(s)) are not supported

// error: function `TrivialCustomType::operator int` could not be bound
//   Function name is not supported: Unsupported name: operator int

#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialCustomType
pub struct NontrivialCustomType {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub i: ::ffi_11::c_int,
}
impl !Send for NontrivialCustomType {}
impl !Sync for NontrivialCustomType {}
unsafe impl ::cxx::ExternType for NontrivialCustomType {
    type Id = ::cxx::type_id!("NontrivialCustomType");
    type Kind = ::cxx::kind::Opaque;
}

// error: constructor `NontrivialCustomType::NontrivialCustomType` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `NontrivialCustomType::operator||` could not be bound
//   Bindings for this kind of operator (operator || with 2 parameter(s)) are not supported

// error: struct `PackedLayout` could not be bound
//   Records with packed layout are not supported

// error: function `MultipleReasons` could not be bound
//   Parameter #0 is not supported: Unsupported `volatile` qualifier: volatile int
//   Return type is not supported: Unsupported `volatile` qualifier: volatile int

// error: struct `TypeWithUnknownAttribute` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): gnu::abi_tag

// error: function `FuncWithUnknownAttribute` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown function attributes are only supported with experimental features enabled on //rs_bindings_from_cc/test/golden:unsupported_cc
//   Unknown attribute: gnu::abi_tag`

// error: function `ParamWithUnknownAttribute` could not be bound
//   crubit.rs/errors/unknown_attribute: param i has unknown attribute(s): gnu::abi_tag

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN17TrivialCustomTypeC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::TrivialCustomType>() == 4);
    assert!(::core::mem::align_of::<crate::TrivialCustomType>() == 4);
    static_assertions::assert_impl_all!(crate::TrivialCustomType: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TrivialCustomType: Drop);
    assert!(::core::mem::offset_of!(crate::TrivialCustomType, i) == 0);
    assert!(::core::mem::size_of::<crate::NontrivialCustomType>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialCustomType>() == 4);
    static_assertions::assert_not_impl_any!(crate::NontrivialCustomType: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::NontrivialCustomType, i) == 0);
};
