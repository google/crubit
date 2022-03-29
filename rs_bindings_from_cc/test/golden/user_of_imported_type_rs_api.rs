// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc
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

#[inline(always)]
pub fn UsesImportedType(t: trivial_type_cc::Trivial) -> trivial_type_cc::Trivial {
    unsafe { crate::detail::__rust_thunk___Z16UsesImportedType7Trivial(t) }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct UserOfImportedType {
    pub trivial: *mut trivial_type_cc::Trivial,
}

impl Default for UserOfImportedType {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, UserOfImportedType>> for UserOfImportedType {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, UserOfImportedType>) -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=14
// Error while generating bindings for item 'UserOfImportedType::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=14
// Error while generating bindings for item 'UserOfImportedType::operator=':
// Bindings for this kind of operator are not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_IMPORTED_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_Z16UsesImportedType7Trivial"]
        pub(crate) fn __rust_thunk___Z16UsesImportedType7Trivial(
            t: trivial_type_cc::Trivial,
        ) -> trivial_type_cc::Trivial;
        pub(crate) fn __rust_thunk___ZN18UserOfImportedTypeC1Ev<'a>(
            __this: &'a mut std::mem::MaybeUninit<UserOfImportedType>,
        );
        pub(crate) fn __rust_thunk___ZN18UserOfImportedTypeC1EOS_<'a, 'b>(
            __this: &'a mut std::mem::MaybeUninit<UserOfImportedType>,
            __param_0: ctor::RvalueReference<'b, UserOfImportedType>,
        );
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<UserOfImportedType>() == 8usize);
const _: () = assert!(std::mem::align_of::<UserOfImportedType>() == 8usize);
const _: () = {
    assert_impl_all!(UserOfImportedType: Clone);
};
const _: () = {
    assert_impl_all!(UserOfImportedType: Copy);
};
const _: () = {
    assert_not_impl_all!(UserOfImportedType: Drop);
};
const _: () = assert!(offset_of!(UserOfImportedType, trivial) * 8 == 0usize);
