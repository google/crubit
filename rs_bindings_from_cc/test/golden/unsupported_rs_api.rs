// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsupported_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "17TrivialCustomType"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TrivialCustomType
///CRUBIT_ANNOTATE: cpp_move_constructible=
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

// error: function `TrivialCustomType::operator||` could not be bound
//   Bindings for this kind of operator (operator || with 2 parameter(s)) are not supported

#[::ctor::recursively_pinned]
#[cfi_encoding = "20NontrivialCustomType"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialCustomType
///CRUBIT_ANNOTATE: cpp_move_constructible=
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

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>
    for NontrivialCustomType
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN20NontrivialCustomTypeC1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
    for NontrivialCustomType
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

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
//   Unknown attribute: gnu::cold`

// error: function `ParamWithUnknownAttribute` could not be bound
//   crubit.rs/errors/unknown_attribute: param i has unknown attribute(s): gnu::abi_tag

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN17TrivialCustomTypeC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN20NontrivialCustomTypeC1EOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN20NontrivialCustomTypeC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::NontrivialCustomType>,
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
