// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;

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
forward_declare::unsafe_define!(
    forward_declare::symbol!("UserOfImportedType"),
    crate::UserOfImportedType
);

impl Default for UserOfImportedType {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::UserOfImportedType>> for UserOfImportedType {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::UserOfImportedType>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::UserOfImportedType>,
        );
        pub(crate) fn __rust_thunk___ZN18UserOfImportedTypeC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::UserOfImportedType>,
            __param_0: ctor::RvalueReference<'b, crate::UserOfImportedType>,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::UserOfImportedType>() == 8usize);
const _: () = assert!(rust_std::mem::align_of::<crate::UserOfImportedType>() == 8usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::UserOfImportedType: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::UserOfImportedType: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::UserOfImportedType: Drop);
};
const _: () = assert!(offset_of!(crate::UserOfImportedType, trivial) * 8 == 0usize);
