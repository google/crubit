// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:static_methods_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

pub type __builtin_ms_va_list = *mut u8;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomeClass {
    field_: i32,
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

impl<'b> From<ctor::RvalueReference<'b, SomeClass>> for SomeClass {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, SomeClass>) -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/static_methods.h;l=10
// Error while generating bindings for item 'SomeClass::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/static_methods.h;l=10
// Error while generating bindings for item 'SomeClass::operator=':
// Bindings for this kind of operator are not supported

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

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_STATIC_METHODS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN9SomeClassC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<SomeClass>,
        );
        pub(crate) fn __rust_thunk___ZN9SomeClassC1EOS_<'a, 'b>(
            __this: &'a mut std::mem::MaybeUninit<SomeClass>,
            __param_0: ctor::RvalueReference<'b, SomeClass>,
        );
        #[link_name = "_ZN9SomeClass21static_factory_methodEi"]
        pub(crate) fn __rust_thunk___ZN9SomeClass21static_factory_methodEi(
            initial_value_of_field: i32,
        ) -> SomeClass;
        #[link_name = "_ZN9SomeClass38static_method_that_multiplies_its_argsEii"]
        pub(crate) fn __rust_thunk___ZN9SomeClass38static_method_that_multiplies_its_argsEii(
            x: i32,
            y: i32,
        ) -> i32;
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<SomeClass>() == 4usize);
const _: () = assert!(std::mem::align_of::<SomeClass>() == 4usize);
const _: () = {
    assert_impl_all!(SomeClass: Clone);
};
const _: () = {
    assert_impl_all!(SomeClass: Copy);
};
const _: () = {
    assert_not_impl_all!(SomeClass: Drop);
};
const _: () = assert!(offset_of!(SomeClass, field_) * 8 == 0usize);
