#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

#[repr(C)]
pub struct NontrivialCustomType {
    pub i: i32,
}

impl !Unpin for NontrivialCustomType {}

// rs_bindings_from_cc/test/golden/unsupported.h;l=4
// Error while generating bindings for item 'NontrivialCustomType::NontrivialCustomType':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/unsupported.h;l=5
// Error while generating bindings for item 'NontrivialCustomType::NontrivialCustomType':
// Parameter type 'struct NontrivialCustomType &&' is not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=10
// Error while generating bindings for item 'UnsupportedParamType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a parameter

// rs_bindings_from_cc/test/golden/unsupported.h;l=11
// Error while generating bindings for item 'UnsupportedReturnType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a return type

// rs_bindings_from_cc/test/golden/unsupported.h;l=13
// Error while generating bindings for item 'MultipleReasons':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a parameter

// rs_bindings_from_cc/test/golden/unsupported.h;l=13
// Error while generating bindings for item 'MultipleReasons':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a return type

// rs_bindings_from_cc/test/golden/unsupported.h;l=16
// Error while generating bindings for item 'ns::FunctionInNamespace':
// Items contained in namespaces are not supported yet

// rs_bindings_from_cc/test/golden/unsupported.h;l=17
// Error while generating bindings for item 'ns::StructInNamespace':
// Items contained in namespaces are not supported yet

// namespace ns

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContainingStruct {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}

// rs_bindings_from_cc/test/golden/unsupported.h;l=20
// Error while generating bindings for item 'ContainingStruct::ContainingStruct':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/unsupported.h;l=21
// Error while generating bindings for item 'ContainingStruct::NestedStruct':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/unsupported.h;l=21
// Error while generating bindings for item 'ContainingStruct::NestedStruct::NestedStruct':
// Nested classes are not supported yet

impl Default for ContainingStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl From<*const ContainingStruct> for ContainingStruct {
    #[inline(always)]
    fn from(__param_0: *const ContainingStruct) -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructC1ERKS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unsupported.h;l=20
// Error while generating bindings for item 'ContainingStruct::ContainingStruct':
// Parameter type 'struct ContainingStruct &&' is not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=20
// Error while generating bindings for item 'ContainingStruct::operator=':
// Parameter type 'struct ContainingStruct &&' is not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1Ev(
            __this: &mut std::mem::MaybeUninit<ContainingStruct>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1ERKS_(
            __this: &mut std::mem::MaybeUninit<ContainingStruct>,
            __param_0: *const ContainingStruct,
        );
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<NontrivialCustomType>() == 4usize);
const _: () = assert!(std::mem::align_of::<NontrivialCustomType>() == 4usize);
const _: () = assert!(offset_of!(NontrivialCustomType, i) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<ContainingStruct>() == 1usize);
const _: () = assert!(std::mem::align_of::<ContainingStruct>() == 1usize);
