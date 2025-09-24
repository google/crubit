// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:static_methods_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=SomeClass
pub struct SomeClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for SomeClass {}
impl !Sync for SomeClass {}
unsafe impl ::cxx::ExternType for SomeClass {
    type Id = ::cxx::type_id!("SomeClass");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'SomeClass::SomeClass':
// Can't generate bindings for SomeClass::SomeClass, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:static_methods_cc needs [//features:experimental] for SomeClass::SomeClass (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'SomeClass::SomeClass':
// Can't generate bindings for SomeClass::SomeClass, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:static_methods_cc needs [//features:experimental] for SomeClass::SomeClass (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'SomeClass::operator=':
// Can't generate bindings for SomeClass::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:static_methods_cc needs [//features:experimental] for SomeClass::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:static_methods_cc needs [//features:experimental] for SomeClass::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'SomeClass::operator=':
// Can't generate bindings for SomeClass::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:static_methods_cc needs [//features:experimental] for SomeClass::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:static_methods_cc needs [//features:experimental] for SomeClass::operator= (the type of __param_0 (parameter #1): references are not supported)

impl SomeClass {
    /// Example of a factory method.
    #[inline(always)]
    pub fn static_factory_method(initial_value_of_field: ::core::ffi::c_int) -> crate::SomeClass {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN9SomeClass21static_factory_methodEi(
                &raw mut __return as *mut ::core::ffi::c_void,
                initial_value_of_field,
            );
            __return.assume_init()
        }
    }
}

impl SomeClass {
    /// Static method working on primitive types (and unrelated to the struct).
    #[inline(always)]
    pub fn static_method_that_multiplies_its_args(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClass38static_method_that_multiplies_its_argsEii(
                x, y,
            )
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClassC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass21static_factory_methodEi(
            __return: *mut ::core::ffi::c_void,
            initial_value_of_field: ::core::ffi::c_int,
        );
        #[link_name = "_ZN9SomeClass38static_method_that_multiplies_its_argsEii"]
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClass38static_method_that_multiplies_its_argsEii(
            x: ::core::ffi::c_int,
            y: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeClass>() == 4);
    assert!(::core::mem::align_of::<crate::SomeClass>() == 4);
    static_assertions::assert_impl_all!(crate::SomeClass: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
    assert!(::core::mem::offset_of!(crate::SomeClass, field_) == 0);
};
