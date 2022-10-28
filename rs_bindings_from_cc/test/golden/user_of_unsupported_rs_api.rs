// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_unsupported_cc

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[inline(always)]
pub fn UseNontrivialCustomType(
    non_trivial_custom_type: impl ::ctor::Ctor<Output = unsupported_cc::NontrivialCustomType>,
) {
    unsafe {
        crate::detail::__rust_thunk___Z23UseNontrivialCustomType20NontrivialCustomType(
            ::std::pin::Pin::into_inner_unchecked(::ctor::emplace!(non_trivial_custom_type)),
        )
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_UNSUPPORTED_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___Z23UseNontrivialCustomType20NontrivialCustomType(
            non_trivial_custom_type: &mut unsupported_cc::NontrivialCustomType,
        );
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());
