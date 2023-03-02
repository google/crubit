// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_imported_type_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Generated from: rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=12
#[inline(always)]
pub fn UsesImportedType(t: trivial_type_cc::ns::Trivial) -> trivial_type_cc::ns::Trivial {
    unsafe { crate::detail::__rust_thunk___Z16UsesImportedTypeN2ns7TrivialE(t) }
}

/// Generated from: rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=14
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UserOfImportedType {
    pub trivial: *mut trivial_type_cc::ns::Trivial,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UserOfImportedType"),
    crate::UserOfImportedType
);

/// Generated from: rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=14
impl Default for UserOfImportedType {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=14
impl<'b> From<::ctor::RvalueReference<'b, Self>> for UserOfImportedType {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=14
impl<'b> ::ctor::UnpinAssign<&'b Self> for UserOfImportedType {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/user_of_imported_type.h;l=14
impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for UserOfImportedType {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN18UserOfImportedTypeaSEOS_(self, __param_0);
        }
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_IMPORTED_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_Z16UsesImportedTypeN2ns7TrivialE"]
        pub(crate) fn __rust_thunk___Z16UsesImportedTypeN2ns7TrivialE(
            t: trivial_type_cc::ns::Trivial,
        ) -> trivial_type_cc::ns::Trivial;
        pub(crate) fn __rust_thunk___ZN18UserOfImportedTypeC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::UserOfImportedType>,
        );
        pub(crate) fn __rust_thunk___ZN18UserOfImportedTypeC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::UserOfImportedType>,
            __param_0: ::ctor::RvalueReference<'b, crate::UserOfImportedType>,
        );
        pub(crate) fn __rust_thunk___ZN18UserOfImportedTypeaSERKS_<'a, 'b>(
            __this: &'a mut crate::UserOfImportedType,
            __param_0: &'b crate::UserOfImportedType,
        ) -> &'a mut crate::UserOfImportedType;
        pub(crate) fn __rust_thunk___ZN18UserOfImportedTypeaSEOS_<'a, 'b>(
            __this: &'a mut crate::UserOfImportedType,
            __param_0: ::ctor::RvalueReference<'b, crate::UserOfImportedType>,
        ) -> &'a mut crate::UserOfImportedType;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::UserOfImportedType>() == 8);
const _: () = assert!(::core::mem::align_of::<crate::UserOfImportedType>() == 8);
const _: () = {
    static_assertions::assert_impl_all!(crate::UserOfImportedType: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::UserOfImportedType: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::UserOfImportedType: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::UserOfImportedType, trivial) == 0);
