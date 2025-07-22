// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[inline(always)]
pub fn UsesImportedType(mut t: trivial_type_cc::ns::Trivial) -> trivial_type_cc::ns::Trivial {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<trivial_type_cc::ns::Trivial>::uninit();
        crate::detail::__rust_thunk___Z16UsesImportedTypeN2ns7TrivialE(
            &raw mut __return as *mut ::core::ffi::c_void,
            &mut t,
        );
        __return.assume_init()
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=UserOfImportedType
pub struct UserOfImportedType {
    pub trivial: *mut trivial_type_cc::ns::Trivial,
}
impl !Send for UserOfImportedType {}
impl !Sync for UserOfImportedType {}
unsafe impl ::cxx::ExternType for UserOfImportedType {
    type Id = ::cxx::type_id!("UserOfImportedType");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'UserOfImportedType::UserOfImportedType':
// Can't generate bindings for UserOfImportedType::UserOfImportedType, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::UserOfImportedType (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for constructor 'UserOfImportedType::UserOfImportedType':
// Can't generate bindings for UserOfImportedType::UserOfImportedType, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::UserOfImportedType (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::UserOfImportedType (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'UserOfImportedType::UserOfImportedType':
// Can't generate bindings for UserOfImportedType::UserOfImportedType, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::UserOfImportedType (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::UserOfImportedType (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'UserOfImportedType::operator=':
// Can't generate bindings for UserOfImportedType::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'UserOfImportedType::operator=':
// Can't generate bindings for UserOfImportedType::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc needs [//features:experimental] for UserOfImportedType::operator= (the type of __param_0 (parameter #1): references are not supported)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z16UsesImportedTypeN2ns7TrivialE(
            __return: *mut ::core::ffi::c_void,
            t: &mut trivial_type_cc::ns::Trivial,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::UserOfImportedType>() == 8);
    assert!(::core::mem::align_of::<crate::UserOfImportedType>() == 8);
    static_assertions::assert_impl_all!(crate::UserOfImportedType: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::UserOfImportedType: Drop);
    assert!(::core::mem::offset_of!(crate::UserOfImportedType, trivial) == 0);
};
