#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomeClass {
    field_: i32,
}

// rs_bindings_from_cc/test/golden/static_methods.h;l=4
// Error while generating bindings for item 'SomeClass::SomeClass':
// Nested classes are not supported yet

impl SomeClass {
    /// Example of a factory method.
    #[inline(always)]
    pub fn static_factory_method(initial_value_of_field: i32) -> SomeClass {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClass21static_factory_methodEi(
                initial_value_of_field,
            )
        }
    }
}

impl SomeClass {
    /// Static method working on primitive types (and unrelated to the struct).
    #[inline(always)]
    pub fn static_method_that_multiplies_its_args(x: i32, y: i32) -> i32 {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClass38static_method_that_multiplies_its_argsEii(
                x, y,
            )
        }
    }
}

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/static_methods.h;l=4
// Error while generating bindings for item 'SomeClass::SomeClass':
// Parameter type 'class SomeClass &&' is not supported

// rs_bindings_from_cc/test/golden/static_methods.h;l=4
// Error while generating bindings for item 'SomeClass::operator=':
// Parameter type 'class SomeClass &&' is not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_STATIC_METHODS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN9SomeClass21static_factory_methodEi"]
        pub(crate) fn __rust_thunk___ZN9SomeClass21static_factory_methodEi(
            initial_value_of_field: i32,
        ) -> SomeClass;
        #[link_name = "_ZN9SomeClass38static_method_that_multiplies_its_argsEii"]
        pub(crate) fn __rust_thunk___ZN9SomeClass38static_method_that_multiplies_its_argsEii(
            x: i32,
            y: i32,
        ) -> i32;
        pub(crate) fn __rust_thunk___ZN9SomeClassC1Ev(
            __this: &mut std::mem::MaybeUninit<SomeClass>,
        );
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<SomeClass>() == 4usize);
const _: () = assert!(std::mem::align_of::<SomeClass>() == 4usize);
const _: () = assert!(offset_of!(SomeClass, field_) * 8 == 0usize);
