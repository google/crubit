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
#![allow(dead_code)]
#![deny(warnings)]

// Note: a real example would require that Crubit implements CrubitAbiTrait in
// order for the generated code to properly compile. This example just serves to
// illustrate what the generated code will look like.

#[inline(always)]
pub fn ReturnCppStruct() -> crate::RustStruct {
    unsafe {
        let mut __return_abi_buffer = ::bridge_rust::internal::empty_buffer::<
            { <crate::RustStructAbi as ::bridge_rust::CrubitAbi>::SIZE },
        >();
        crate::detail::__rust_thunk___Z15ReturnCppStructv(
            __return_abi_buffer.as_mut_ptr() as *mut u8
        );
        ::bridge_rust::internal::decode::<crate::RustStructAbi>(
            __return_abi_buffer.as_ptr() as *const u8
        )
    }
}

#[inline(always)]
pub fn TakeCppStruct(__param_0: crate::RustStruct) {
    let mut __crubit_abi_arg_buffer = ::bridge_rust::internal::empty_buffer::<
        { <crate::RustStructAbi as ::bridge_rust::CrubitAbi>::SIZE },
    >();
    #[allow(unused_unsafe)]
    unsafe {
        ::bridge_rust::internal::encode::<crate::RustStructAbi>(
            __crubit_abi_arg_buffer.as_mut_ptr() as *mut u8,
            __param_0,
        );
    }
    let __param_0 = __crubit_abi_arg_buffer;
    unsafe {
        crate::detail::__rust_thunk___Z13TakeCppStruct9CppStruct(__param_0.as_ptr() as *const u8)
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

// Error while generating bindings for item 'Vec3::Vec3':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Vec3
// Missing lifetime for `__this` parameter type: *mut crate::Vec3

// Error while generating bindings for item 'Vec3::Vec3':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Vec3
// Missing lifetime for `__this` parameter type: *mut crate::Vec3

// Error while generating bindings for item 'Vec3::Vec3':
// Parameter #0 is not supported: Unsupported type 'Vec3 &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'Vec3::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'Vec3::operator=':
// Parameter #0 is not supported: Unsupported type 'Vec3 &&': Unsupported type: && without lifetime

#[inline(always)]
pub fn MakeOptionalVec3(x: f32, y: f32, z: f32, is_present: bool) -> crate::MyOption<crate::Vec3> {
    unsafe {
        let mut __return_abi_buffer = ::bridge_rust::internal::empty_buffer::<
            {
                <crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>as::bridge_rust::CrubitAbi>::SIZE
            },
        >();
        crate::detail::__rust_thunk___Z16MakeOptionalVec3fffb(
            __return_abi_buffer.as_mut_ptr() as *mut u8,
            x,
            y,
            z,
            is_present,
        );
        ::bridge_rust::internal::decode::<
            crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,
        >(__return_abi_buffer.as_ptr() as *const u8)
    }
}

#[inline(always)]
pub fn MapMultiply(v: crate::MyOption<crate::Vec3>, factor: f32) -> crate::MyOption<crate::Vec3> {
    let mut __crubit_abi_arg_buffer = ::bridge_rust::internal::empty_buffer::<
        {
            <crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>as::bridge_rust::CrubitAbi>::SIZE
        },
    >();
    #[allow(unused_unsafe)]
    unsafe {
        ::bridge_rust::internal::encode::<
            crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,
        >(__crubit_abi_arg_buffer.as_mut_ptr() as *mut u8, v);
    }
    let v = __crubit_abi_arg_buffer;
    unsafe {
        let mut __return_abi_buffer = ::bridge_rust::internal::empty_buffer::<
            {
                <crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>as::bridge_rust::CrubitAbi>::SIZE
            },
        >();
        crate::detail::__rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(
            __return_abi_buffer.as_mut_ptr() as *mut u8,
            v.as_ptr() as *const u8,
            factor,
        );
        ::bridge_rust::internal::decode::<
            crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,
        >(__return_abi_buffer.as_ptr() as *const u8)
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
    static_assertions::assert_impl_all!(crate::Vec3: Clone);
    static_assertions::assert_impl_all!(crate::Vec3: Copy);
    static_assertions::assert_not_impl_any!(crate::Vec3: Drop);
    assert!(::core::mem::offset_of!(crate::Vec3, x) == 0);
    assert!(::core::mem::offset_of!(crate::Vec3, y) == 4);
    assert!(::core::mem::offset_of!(crate::Vec3, z) == 8);
};
