#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes)]

use memoffset_unstable_const::offset_of;

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

#[inline(always)]
pub fn UsesImportedType(t: trivial_type_cc::Trivial) -> trivial_type_cc::Trivial {
    unsafe { crate::detail::__rust_thunk___Z16UsesImportedType7Trivial(t) }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct UserOfImportedType {
    pub trivial: *mut trivial_type_cc::Trivial,
}

// rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=8
// Error while generating bindings for item 'UserOfImportedType::UserOfImportedType':
// Nested classes are not supported yet

impl Default for UserOfImportedType {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeC1Ev(tmp.as_mut_ptr());
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=8
// Error while generating bindings for item 'UserOfImportedType::UserOfImportedType':
// Parameter type 'struct UserOfImportedType &&' is not supported

// rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=8
// Error while generating bindings for item 'UserOfImportedType::operator=':
// Parameter type 'struct UserOfImportedType &&' is not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_IMPORTED_TYPE_H_

mod detail {
    use super::*;
    extern "C" {
        #[link_name = "_Z16UsesImportedType7Trivial"]
        pub(crate) fn __rust_thunk___Z16UsesImportedType7Trivial(
            t: trivial_type_cc::Trivial,
        ) -> trivial_type_cc::Trivial;
        pub(crate) fn __rust_thunk___ZN18UserOfImportedTypeC1Ev(__this: *mut UserOfImportedType);
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<UserOfImportedType>() == 8usize);
const _: () = assert!(std::mem::align_of::<UserOfImportedType>() == 8usize);
const _: () = assert!(offset_of!(UserOfImportedType, trivial) * 8 == 0usize);
