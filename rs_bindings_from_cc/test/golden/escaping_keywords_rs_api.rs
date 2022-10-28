// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:escaping_keywords_cc

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub struct r#type {
    pub r#dyn: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("type"), crate::r#type);

impl Default for r#type {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4typeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::r#type>> for r#type {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::r#type>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4typeC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/escaping_keywords.h;l=10
// Error while generating bindings for item 'type::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/escaping_keywords.h;l=10
// Error while generating bindings for item 'type::operator=':
// operator= for Unpin types is not yet supported.

#[inline(always)]
pub fn r#impl(r#match: i32) {
    unsafe { crate::detail::__rust_thunk___Z4impli(r#match) }
}

// rs_bindings_from_cc/test/golden/escaping_keywords.h;l=16
// Error while generating bindings for item 'await':
// Class templates are not supported yet

// rs_bindings_from_cc/test/golden/escaping_keywords.h;l=21
// Error while generating bindings for item 'become':
// Function templates are not supported yet

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ESCAPING_KEYWORDS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN4typeC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::r#type>,
        );
        pub(crate) fn __rust_thunk___ZN4typeC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::r#type>,
            __param_0: ::ctor::RvalueReference<'b, crate::r#type>,
        );
        #[link_name = "_Z4impli"]
        pub(crate) fn __rust_thunk___Z4impli(r#match: i32);
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::r#type>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::r#type>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::r#type: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::r#type: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::r#type: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::r#type, r#dyn) == 0);
