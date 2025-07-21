// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsupported_cc

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

// Error while generating bindings for function 'TrivialCustomType::TrivialCustomType':
// Can't generate bindings for TrivialCustomType::TrivialCustomType, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::TrivialCustomType (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'TrivialCustomType::TrivialCustomType':
// Can't generate bindings for TrivialCustomType::TrivialCustomType, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::TrivialCustomType (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::TrivialCustomType (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TrivialCustomType::TrivialCustomType':
// Can't generate bindings for TrivialCustomType::TrivialCustomType, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::TrivialCustomType (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::TrivialCustomType (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TrivialCustomType::operator=':
// Can't generate bindings for TrivialCustomType::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TrivialCustomType::operator=':
// Can't generate bindings for TrivialCustomType::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for TrivialCustomType::operator= (the type of __param_0 (parameter #1): references are not supported)

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

// Error while generating bindings for function 'NontrivialCustomType::NontrivialCustomType':
// Can't generate bindings for NontrivialCustomType::NontrivialCustomType, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for NontrivialCustomType::NontrivialCustomType (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:unsupported_cc needs [//features:experimental] for NontrivialCustomType::NontrivialCustomType (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialCustomType::operator||':
// Bindings for this kind of operator (operator || with 2 parameter(s)) are not supported

// Error while generating bindings for struct 'PackedLayout':
// Records with packed layout are not supported

// Error while generating bindings for function 'MultipleReasons':
// Parameter #0 is not supported: Unsupported type 'volatile int *': Unsupported `volatile` qualifier: volatile int
//
// Return type is not supported: Unsupported type 'volatile int *': Unsupported `volatile` qualifier: volatile int

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
