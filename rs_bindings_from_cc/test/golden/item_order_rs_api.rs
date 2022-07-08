// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:item_order_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
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
    pub field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("FirstStruct"), crate::FirstStruct);

impl Default for FirstStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::FirstStruct>> for FirstStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::FirstStruct>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11FirstStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'FirstStruct::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'FirstStruct::operator=':
// operator= for Unpin types is not yet supported.

#[inline(always)]
pub fn first_func() -> i32 {
    unsafe { crate::detail::__rust_thunk___Z10first_funcv() }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SecondStruct {
    pub field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("SecondStruct"), crate::SecondStruct);

impl Default for SecondStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::SecondStruct>> for SecondStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::SecondStruct>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12SecondStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/item_order.h;l=16
// Error while generating bindings for item 'SecondStruct::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/item_order.h;l=16
// Error while generating bindings for item 'SecondStruct::operator=':
// operator= for Unpin types is not yet supported.

#[inline(always)]
pub fn second_func() -> i32 {
    unsafe { crate::detail::__rust_thunk___Z11second_funcv() }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ITEM_ORDER_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN11FirstStructC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::FirstStruct>,
        );
        pub(crate) fn __rust_thunk___ZN11FirstStructC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::FirstStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::FirstStruct>,
        );
        pub(crate) fn __rust_thunk___Z10first_funcv() -> i32;
        pub(crate) fn __rust_thunk___ZN12SecondStructC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::SecondStruct>,
        );
        pub(crate) fn __rust_thunk___ZN12SecondStructC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::SecondStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SecondStruct>,
        );
        pub(crate) fn __rust_thunk___Z11second_funcv() -> i32;
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::FirstStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::FirstStruct>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::FirstStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::FirstStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::FirstStruct: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::FirstStruct, field) == 0);

const _: () = assert!(::std::mem::size_of::<crate::SecondStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::SecondStruct>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::SecondStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SecondStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SecondStruct: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::SecondStruct, field) == 0);
