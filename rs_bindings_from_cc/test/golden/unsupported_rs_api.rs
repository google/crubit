// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:unsupported_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate static_assertions;
use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

pub type __builtin_ms_va_list = *mut u8;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
pub struct NontrivialCustomType {
    pub i: i32,
}

impl !Unpin for NontrivialCustomType {}

// rs_bindings_from_cc/test/golden/unsupported.h;l=11
// Error while generating bindings for item 'NontrivialCustomType::NontrivialCustomType':
// Parameter #0 is not supported: Unsupported type 'struct NontrivialCustomType &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/unsupported.h;l=16
// Error while generating bindings for item 'UnsupportedParamType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as parameter #0

// rs_bindings_from_cc/test/golden/unsupported.h;l=17
// Error while generating bindings for item 'UnsupportedReturnType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a return type

// rs_bindings_from_cc/test/golden/unsupported.h;l=19
// Error while generating bindings for item 'MultipleReasons':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a return type
//
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as parameter #0
//
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as parameter #2

// rs_bindings_from_cc/test/golden/unsupported.h;l=22
// Error while generating bindings for item 'ns':
// Namespaces are not supported yet

// namespace ns

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContainingStruct {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: std::mem::MaybeUninit<u8>,
}

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

// rs_bindings_from_cc/test/golden/unsupported.h;l=30
// Error while generating bindings for item 'ContainingStruct::ContainingStruct':
// Parameter #0 is not supported: Unsupported type 'struct ContainingStruct &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/unsupported.h;l=30
// Error while generating bindings for item 'ContainingStruct::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=30
// Error while generating bindings for item 'ContainingStruct::operator=':
// Parameter #0 is not supported: Unsupported type 'struct ContainingStruct &&': Unsupported clang::Type class 'RValueReference'

// rs_bindings_from_cc/test/golden/unsupported.h;l=31
// Error while generating bindings for item 'ContainingStruct::NestedStruct':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/unsupported.h;l=32
// Error while generating bindings for item 'ContainingStruct::NestedStruct::NonStaticMemberFunction':
// Couldn't import the parent

// rs_bindings_from_cc/test/golden/unsupported.h;l=33
// Error while generating bindings for item 'ContainingStruct::NestedStruct::StaticMemberFunction':
// Couldn't import the parent

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<ContainingStruct>,
        );
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<NontrivialCustomType>() == 4usize);
const _: () = assert!(std::mem::align_of::<NontrivialCustomType>() == 4usize);
const _: () = assert!(offset_of!(NontrivialCustomType, i) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<ContainingStruct>() == 1usize);
const _: () = assert!(std::mem::align_of::<ContainingStruct>() == 1usize);
