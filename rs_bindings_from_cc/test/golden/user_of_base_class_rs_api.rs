// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// The same as Derived from inheritance.h, but in a different build target.
///
/// This tests inheritance across library boundaries.
///
/// TODO(b/216195042): Correctly namespace base classes in generated Rust code.
#[::ctor::recursively_pinned]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Derived2
pub struct Derived2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 20],
    pub derived_1: ::core::ffi::c_char,
}
impl !Send for Derived2 {}
impl !Sync for Derived2 {}
unsafe impl ::cxx::ExternType for Derived2 {
    type Id = ::cxx::type_id!("Derived2");
    type Kind = ::cxx::kind::Opaque;
}

// Error while generating bindings for constructor 'Derived2::Derived2':
// Can't generate bindings for Derived2::Derived2, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::Derived2 (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for constructor 'Derived2::Derived2':
// Can't generate bindings for Derived2::Derived2, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::Derived2 (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::Derived2 (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'Derived2::Derived2':
// Can't generate bindings for Derived2::Derived2, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::Derived2 (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::Derived2 (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Derived2::operator=':
// Can't generate bindings for Derived2::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Derived2::operator=':
// Can't generate bindings for Derived2::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for Derived2::operator= (the type of __param_0 (parameter #1): references are not supported)

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=VirtualDerived2
pub struct VirtualDerived2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 32],
}
impl !Send for VirtualDerived2 {}
impl !Sync for VirtualDerived2 {}
unsafe impl ::cxx::ExternType for VirtualDerived2 {
    type Id = ::cxx::type_id!("VirtualDerived2");
    type Kind = ::cxx::kind::Opaque;
}

// Error while generating bindings for constructor 'VirtualDerived2::VirtualDerived2':
// Can't generate bindings for VirtualDerived2::VirtualDerived2, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::VirtualDerived2 (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for constructor 'VirtualDerived2::VirtualDerived2':
// Can't generate bindings for VirtualDerived2::VirtualDerived2, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::VirtualDerived2 (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::VirtualDerived2 (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'VirtualDerived2::VirtualDerived2':
// Can't generate bindings for VirtualDerived2::VirtualDerived2, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::VirtualDerived2 (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::VirtualDerived2 (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'VirtualDerived2::operator=':
// Can't generate bindings for VirtualDerived2::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'VirtualDerived2::operator=':
// Can't generate bindings for VirtualDerived2::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc needs [//features:experimental] for VirtualDerived2::operator= (the type of __param_0 (parameter #1): references are not supported)

const _: () = {
    assert!(::core::mem::size_of::<crate::Derived2>() == 24);
    assert!(::core::mem::align_of::<crate::Derived2>() == 8);
    static_assertions::assert_not_impl_any!(crate::Derived2: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::Derived2, derived_1) == 20);
    assert!(::core::mem::size_of::<crate::VirtualDerived2>() == 32);
    assert!(::core::mem::align_of::<crate::VirtualDerived2>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualDerived2: Copy,Drop);
};
