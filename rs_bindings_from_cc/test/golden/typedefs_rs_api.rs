// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:typedefs_cc

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
///CRUBIT_ANNOTATE: cpp_type=SomeStruct
pub struct SomeStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeStruct {}
impl !Sync for SomeStruct {}
unsafe impl ::cxx::ExternType for SomeStruct {
    type Id = ::cxx::type_id!("SomeStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

impl Default for SomeStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod some_struct {
    #[allow(unused_imports)]
    use super::*;

    pub type nested_type = ::core::ffi::c_int;
}

// Error while generating bindings for type alias 'SomeStruct':
// Typedef only used to introduce a name in C. Not importing.

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeOtherStruct
pub struct SomeOtherStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeOtherStruct {}
impl !Sync for SomeOtherStruct {}
unsafe impl ::cxx::ExternType for SomeOtherStruct {
    type Id = ::cxx::type_id!("SomeOtherStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("SomeOtherStruct"),
    crate::SomeOtherStruct
);

impl Default for SomeOtherStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15SomeOtherStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeUnion
pub union SomeUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeUnion {}
impl !Sync for SomeUnion {}
unsafe impl ::cxx::ExternType for SomeUnion {
    type Id = ::cxx::type_id!("SomeUnion");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeUnion"), crate::SomeUnion);

impl Default for SomeUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeUnionC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for type alias 'SomeUnion':
// Typedef only used to introduce a name in C. Not importing.

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeOtherUnion
pub union SomeOtherUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeOtherUnion {}
impl !Sync for SomeOtherUnion {}
unsafe impl ::cxx::ExternType for SomeOtherUnion {
    type Id = ::cxx::type_id!("SomeOtherUnion");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeOtherUnion"), crate::SomeOtherUnion);

impl Default for SomeOtherUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14SomeOtherUnionC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn FunctionUsingNestedType() -> crate::some_struct::nested_type {
    unsafe { crate::detail::__rust_thunk___Z23FunctionUsingNestedTypev() }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10SomeStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN15SomeOtherStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN9SomeUnionC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN14SomeOtherUnionC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_Z23FunctionUsingNestedTypev"]
        pub(crate) unsafe fn __rust_thunk___Z23FunctionUsingNestedTypev(
        ) -> crate::some_struct::nested_type;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeStruct>() == 1);
    assert!(::core::mem::align_of::<crate::SomeStruct>() == 1);
    static_assertions::assert_impl_all!(crate::SomeStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);

    assert!(::core::mem::size_of::<crate::SomeOtherStruct>() == 1);
    assert!(::core::mem::align_of::<crate::SomeOtherStruct>() == 1);
    static_assertions::assert_impl_all!(crate::SomeOtherStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeOtherStruct: Drop);

    assert!(::core::mem::size_of::<crate::SomeUnion>() == 1);
    assert!(::core::mem::align_of::<crate::SomeUnion>() == 1);
    static_assertions::assert_impl_all!(crate::SomeUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeUnion: Drop);

    assert!(::core::mem::size_of::<crate::SomeOtherUnion>() == 1);
    assert!(::core::mem::align_of::<crate::SomeOtherUnion>() == 1);
    static_assertions::assert_impl_all!(crate::SomeOtherUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeOtherUnion: Drop);
};
