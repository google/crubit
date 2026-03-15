// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/global:global
// Features: supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

extern "C" {
    pub static mut extern_int: ::ffi_11::c_int;
}

extern "C" {
    pub static kExternConstInt: ::ffi_11::c_int;
}

// Check that duplicate extern declarations are handled correctly.

// namespace foo

pub const kInlineConstInt: ::ffi_11::c_int = ::ffi_11::new_c_int(6);

pub const kConstexprInt: ::ffi_11::c_int = ::ffi_11::new_c_int(7);

pub const inline_int: ::ffi_11::c_int = ::ffi_11::new_c_int(5);

// namespace foo

// Generated from: rs_bindings_from_cc/test/global/global.h;l=30
// error: global variable `templated_variable` could not be bound
//   templated variables are not supported

/// instantiate templated_variable<int>
///
/// Generated from: rs_bindings_from_cc/test/global/global.h;l=33
#[inline(always)]
pub fn Unused(arg: ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z6Unusedi(arg) }
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=35
#[inline(always)]
pub fn GetIntVal() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z9GetIntValv() }
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=36
#[inline(always)]
pub fn GetNamespacedIntVal() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z19GetNamespacedIntValv() }
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=37
#[inline(always)]
pub fn GetCNamespacedIntVal() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z20GetCNamespacedIntValv() }
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=38
#[inline(always)]
pub fn GetInlineIntVal() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z15GetInlineIntValv() }
}

pub const kAnonEnumConst: ::ffi_11::c_uint = ::ffi_11::new_c_uint(123);

pub mod foo {
    extern "C" {
        #[link_name = "_ZN3foo21extern_int_namespacedE"]
        pub static mut extern_int_namespaced: ::ffi_11::c_int;
    }

    extern "C" {
        pub static mut extern_c_int_namespaced: ::ffi_11::c_int;
    }

    pub const inline_int_namespaced: ::ffi_11::c_int = ::ffi_11::new_c_int(5);

    pub const inline_long_long_namespaced: ::ffi_11::c_longlong = ::ffi_11::new_c_longlong(24);

    pub const inline_bool_namespaced: bool = true;

    pub const kAnonEnumNamespacedConst: ::ffi_11::c_uint = ::ffi_11::new_c_uint(456);
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=44
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithAnonEnum
pub struct StructWithAnonEnum {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for StructWithAnonEnum {}
impl !Sync for StructWithAnonEnum {}
unsafe impl ::cxx::ExternType for StructWithAnonEnum {
    type Id = ::cxx::type_id!("StructWithAnonEnum");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: rs_bindings_from_cc/test/global/global.h;l=44
impl Default for StructWithAnonEnum {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18StructWithAnonEnumC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod struct_with_anon_enum {
    #[allow(unused_imports)]
    use super::*;

    pub const kAnonEnumInStructConst: ::ffi_11::c_uint = ::ffi_11::new_c_uint(789);
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z6Unusedi(arg: ::ffi_11::c_int);
        #[link_name = "_Z9GetIntValv"]
        pub(crate) unsafe fn __rust_thunk___Z9GetIntValv() -> ::ffi_11::c_int;
        #[link_name = "_Z19GetNamespacedIntValv"]
        pub(crate) unsafe fn __rust_thunk___Z19GetNamespacedIntValv() -> ::ffi_11::c_int;
        #[link_name = "_Z20GetCNamespacedIntValv"]
        pub(crate) unsafe fn __rust_thunk___Z20GetCNamespacedIntValv() -> ::ffi_11::c_int;
        #[link_name = "_Z15GetInlineIntValv"]
        pub(crate) unsafe fn __rust_thunk___Z15GetInlineIntValv() -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN18StructWithAnonEnumC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::StructWithAnonEnum>() == 1);
    assert!(::core::mem::align_of::<crate::StructWithAnonEnum>() == 1);
    static_assertions::assert_impl_all!(crate::StructWithAnonEnum: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithAnonEnum: Drop);
};
