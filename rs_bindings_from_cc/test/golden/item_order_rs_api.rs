// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:item_order_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FirstStruct {
    pub field: ::core::ffi::c_int,
}
forward_declare::unsafe_define!(forward_declare::symbol!("FirstStruct"), crate::FirstStruct);

impl Default for FirstStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for FirstStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for FirstStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for FirstStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructaSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn first_func() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z10first_funcv() }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SecondStruct {
    pub field: ::core::ffi::c_int,
}
forward_declare::unsafe_define!(forward_declare::symbol!("SecondStruct"), crate::SecondStruct);

impl Default for SecondStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SecondStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SecondStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SecondStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructaSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn second_func() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z11second_funcv() }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ITEM_ORDER_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN11FirstStructC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::FirstStruct>,
        );
        pub(crate) fn __rust_thunk___ZN11FirstStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::FirstStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::FirstStruct>,
        );
        pub(crate) fn __rust_thunk___ZN11FirstStructaSERKS_<'a, 'b>(
            __this: &'a mut crate::FirstStruct,
            __param_0: &'b crate::FirstStruct,
        ) -> &'a mut crate::FirstStruct;
        pub(crate) fn __rust_thunk___ZN11FirstStructaSEOS_<'a, 'b>(
            __this: &'a mut crate::FirstStruct,
            __param_0: ::ctor::RvalueReference<'b, crate::FirstStruct>,
        ) -> &'a mut crate::FirstStruct;
        pub(crate) fn __rust_thunk___Z10first_funcv() -> ::core::ffi::c_int;
        pub(crate) fn __rust_thunk___ZN12SecondStructC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SecondStruct>,
        );
        pub(crate) fn __rust_thunk___ZN12SecondStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SecondStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SecondStruct>,
        );
        pub(crate) fn __rust_thunk___ZN12SecondStructaSERKS_<'a, 'b>(
            __this: &'a mut crate::SecondStruct,
            __param_0: &'b crate::SecondStruct,
        ) -> &'a mut crate::SecondStruct;
        pub(crate) fn __rust_thunk___ZN12SecondStructaSEOS_<'a, 'b>(
            __this: &'a mut crate::SecondStruct,
            __param_0: ::ctor::RvalueReference<'b, crate::SecondStruct>,
        ) -> &'a mut crate::SecondStruct;
        pub(crate) fn __rust_thunk___Z11second_funcv() -> ::core::ffi::c_int;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::FirstStruct>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::FirstStruct>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::FirstStruct:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::FirstStruct:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::FirstStruct:Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::FirstStruct, field) == 0);

const _: () = assert!(::core::mem::size_of::<crate::SecondStruct>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::SecondStruct>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::SecondStruct:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SecondStruct:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SecondStruct:Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::SecondStruct, field) == 0);
