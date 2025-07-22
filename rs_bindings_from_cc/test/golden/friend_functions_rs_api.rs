// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:friend_functions_cc

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
///CRUBIT_ANNOTATE: cpp_type=SomeClass
pub struct SomeClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeClass {}
impl !Sync for SomeClass {}
unsafe impl ::cxx::ExternType for SomeClass {
    type Id = ::cxx::type_id!("SomeClass");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'SomeClass::SomeClass':
// Can't generate bindings for SomeClass::SomeClass, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::SomeClass (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for constructor 'SomeClass::SomeClass':
// Can't generate bindings for SomeClass::SomeClass, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::SomeClass (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::SomeClass (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'SomeClass::SomeClass':
// Can't generate bindings for SomeClass::SomeClass, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::SomeClass (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::SomeClass (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'SomeClass::operator=':
// Can't generate bindings for SomeClass::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'SomeClass::operator=':
// Can't generate bindings for SomeClass::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for SomeClass::operator= (the type of __param_0 (parameter #1): references are not supported)

/// Friend functions that are visible via ADL.
#[inline(always)]
pub fn visible_val(mut __param_0: crate::SomeClass) {
    unsafe { crate::detail::__rust_thunk___Z11visible_val9SomeClass(&mut __param_0) }
}

// Error while generating bindings for function 'visible_ref':
// Can't generate bindings for visible_ref, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for visible_ref (the type of __param_0 (parameter #0): references are not supported)

// Error while generating bindings for function 'visible_cref':
// Can't generate bindings for visible_cref, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for visible_cref (the type of __param_0 (parameter #0): references are not supported)

// Error while generating bindings for function 'visible_rref':
// Can't generate bindings for visible_rref, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for visible_rref (the type of __param_0 (parameter #0): references are not supported)

// Error while generating bindings for function 'multiple_declarations':
// Can't generate bindings for multiple_declarations, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:friend_functions_cc needs [//features:experimental] for multiple_declarations (the type of __param_0 (parameter #0): references are not supported)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z11visible_val9SomeClass(
            __param_0: &mut crate::SomeClass,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeClass>() == 1);
    assert!(::core::mem::align_of::<crate::SomeClass>() == 1);
    static_assertions::assert_impl_all!(crate::SomeClass: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
};
