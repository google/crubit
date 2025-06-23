// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Note: a real example would require that Crubit implements CrubitAbiTrait in
// order for the generated code to properly compile. This example just serves to
// illustrate what the generated code will look like.

#[inline(always)]
pub fn ReturnCppStruct() -> crate::RustStruct {
    unsafe {
        ::bridge_rust::unstable_return!(crate::RustStructAbi, |__return_abi_buffer| {
            crate::detail::__rust_thunk___Z15ReturnCppStructv(__return_abi_buffer);
        })
    }
}

#[inline(always)]
pub fn TakeCppStruct(__param_0: crate::RustStruct) {
    unsafe {
        crate::detail::__rust_thunk___Z13TakeCppStruct9CppStruct(
            ::bridge_rust::unstable_encode!(crate::RustStructAbi, __param_0).as_ptr() as *const u8,
        )
    }
}

// Error while generating bindings for item 'MyOption':
// Class templates are not supported yet

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Vec3
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl !Send for Vec3 {}
impl !Sync for Vec3 {}
forward_declare::unsafe_define!(forward_declare::symbol!("Vec3"), crate::Vec3);

impl Default for Vec3 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4Vec3C1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for Vec3 {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4Vec3C1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Vec3 {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for Vec3 {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN4Vec3aSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Vec3 {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN4Vec3aSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn MakeOptionalVec3(x: f32, y: f32, z: f32, is_present: bool) -> crate::MyOption<crate::Vec3> {
    unsafe {
        ::bridge_rust::unstable_return!(
            crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,
            |__return_abi_buffer| {
                crate::detail::__rust_thunk___Z16MakeOptionalVec3fffb(
                    __return_abi_buffer,
                    x,
                    y,
                    z,
                    is_present,
                );
            }
        )
    }
}

#[inline(always)]
pub fn MapMultiply(v: crate::MyOption<crate::Vec3>, factor: f32) -> crate::MyOption<crate::Vec3> {
    unsafe {
        ::bridge_rust::unstable_return!(
            crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,
            |__return_abi_buffer| {
                crate::detail::__rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(
                    __return_abi_buffer,
                    ::bridge_rust::unstable_encode!(
                        crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,
                        v
                    )
                    .as_ptr() as *const u8,
                    factor,
                );
            }
        )
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z15ReturnCppStructv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z13TakeCppStruct9CppStruct(
            __param_0: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4Vec3C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN4Vec3C1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::Vec3>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4Vec3aSERKS_<'__return_lifetime>(
            __this: &mut crate::Vec3,
            __param_0: &crate::Vec3,
        ) -> &'__return_lifetime mut crate::Vec3;
        pub(crate) unsafe fn __rust_thunk___ZN4Vec3aSEOS_<'__return_lifetime>(
            __this: &mut crate::Vec3,
            __param_0: ::ctor::RvalueReference<'_, crate::Vec3>,
        ) -> &'__return_lifetime mut crate::Vec3;
        pub(crate) unsafe fn __rust_thunk___Z16MakeOptionalVec3fffb(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            x: f32,
            y: f32,
            z: f32,
            is_present: bool,
        );
        pub(crate) unsafe fn __rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            v: *const ::core::ffi::c_uchar,
            factor: f32,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Vec3>() == 12);
    assert!(::core::mem::align_of::<crate::Vec3>() == 4);
    static_assertions::assert_impl_all!(crate::Vec3: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Vec3: Drop);
    assert!(::core::mem::offset_of!(crate::Vec3, x) == 0);
    assert!(::core::mem::offset_of!(crate::Vec3, y) == 4);
    assert!(::core::mem::offset_of!(crate::Vec3, z) == 8);
};
