// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsupported_cc

#![rustfmt::skip]
#![feature(
    allocator_api,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls
)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TrivialCustomType
pub struct TrivialCustomType {
    pub i: ::core::ffi::c_int,
}
impl !Send for TrivialCustomType {}
impl !Sync for TrivialCustomType {}
unsafe impl ::cxx::ExternType for TrivialCustomType {
    type Id = ::cxx::type_id!("TrivialCustomType");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TrivialCustomType"),
    crate::TrivialCustomType
);

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

// Error while generating bindings for function 'TrivialCustomType::operator||':
// Bindings for this kind of operator (operator || with 2 parameter(s)) are not supported

// Error while generating bindings for function 'TrivialCustomType::operator int':
// Function name is not supported: Unsupported name: operator int

#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialCustomType
pub struct NontrivialCustomType {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub i: ::core::ffi::c_int,
}
impl !Send for NontrivialCustomType {}
impl !Sync for NontrivialCustomType {}
unsafe impl ::cxx::ExternType for NontrivialCustomType {
    type Id = ::cxx::type_id!("NontrivialCustomType");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialCustomType"),
    crate::NontrivialCustomType
);

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialCustomType {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN20NontrivialCustomTypeC1EOS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialCustomType {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

// Error while generating bindings for function 'NontrivialCustomType::operator||':
// Bindings for this kind of operator (operator || with 2 parameter(s)) are not supported

// Error while generating bindings for struct 'PackedLayout':
// Records with packed layout are not supported

// Error while generating bindings for function 'MultipleReasons':
// Parameter #0 is not supported: Unsupported type 'volatile int *': Unsupported `volatile` qualifier: volatile int
//
// Return type is not supported: Unsupported type 'volatile int *': Unsupported `volatile` qualifier: volatile int

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN17TrivialCustomTypeC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN20NontrivialCustomTypeC1EOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN20NontrivialCustomTypeC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialCustomType>,
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
