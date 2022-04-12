// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:namespace_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod test_namespace_bindings {
    #[repr(C)]
    pub struct S {
        pub i: i32,
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("S"), S);

    impl !Unpin for S {}

    // rs_bindings_from_cc/test/golden/namespace.h;l=9
    // Error while generating bindings for item 'S::S':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

    // rs_bindings_from_cc/test/golden/namespace.h;l=9
    // Error while generating bindings for item 'S::S':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

    // rs_bindings_from_cc/test/golden/namespace.h;l=9
    // Error while generating bindings for item 'test_namespace_bindings::S::S':
    // Parameter #0 is not supported: Unsupported type 'struct test_namespace_bindings::S &&': Unsupported type: && without lifetime

    // rs_bindings_from_cc/test/golden/namespace.h;l=9
    // Error while generating bindings for item 'S::operator=':
    // Bindings for this kind of operator are not supported

    // rs_bindings_from_cc/test/golden/namespace.h;l=9
    // Error while generating bindings for item 'test_namespace_bindings::S::operator=':
    // Parameter #0 is not supported: Unsupported type 'struct test_namespace_bindings::S &&': Unsupported type: && without lifetime

    /// Free comment inside namespace
    #[inline(always)]
    pub fn f(s: S) -> i32 {
        unsafe { detail::__rust_thunk___ZN23test_namespace_bindings1fENS_1SE(s) }
    }

    pub mod inner {
        #[inline(always)]
        pub fn i() {
            unsafe { detail::__rust_thunk___ZN23test_namespace_bindings5inner1iEv() }
        }

        mod detail {
            #[allow(unused_imports)]
            use super::*;
            extern "C" {
                #[link_name = "_ZN23test_namespace_bindings5inner1iEv"]
                pub(crate) fn __rust_thunk___ZN23test_namespace_bindings5inner1iEv();
            }
        }
    }

    // namespace inner

    mod detail {
        #[allow(unused_imports)]
        use super::*;
        extern "C" {
            #[link_name = "_ZN23test_namespace_bindings1fENS_1SE"]
            pub(crate) fn __rust_thunk___ZN23test_namespace_bindings1fENS_1SE(s: S) -> i32;
        }
    }

    const _: () = assert!(rust_std::mem::size_of::<S>() == 4usize);
    const _: () = assert!(rust_std::mem::align_of::<S>() == 4usize);
    const _: () = {
        static_assertions::assert_not_impl_all!(S: Copy);
    };
    const _: () = {
        static_assertions::assert_not_impl_all!(S: Drop);
    };
    const _: () = assert!(offset_of!(S, i) * 8 == 0usize);
}

// namespace test_namespace_bindings

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());
