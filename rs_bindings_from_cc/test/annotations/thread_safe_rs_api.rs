// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:thread_safe
// Features: fmt, supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

pub mod crubit {
    pub mod test {
        /// A simple thread-safe struct.
        ///
        /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=13
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ThreadSafeStruct
        pub struct ThreadSafeStruct {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) x_: [::core::mem::MaybeUninit<u8>; 4],
        }
        unsafe impl Send for ThreadSafeStruct {}
        unsafe impl Sync for ThreadSafeStruct {}
        unsafe impl ::cxx::ExternType for ThreadSafeStruct {
            type Id = ::cxx::type_id!("crubit :: test :: ThreadSafeStruct");
            type Kind = ::cxx::kind::Trivial;
        }
        impl ThreadSafeStruct {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            ///
            /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=15
            #[inline(always)]
            pub unsafe fn ConstGet(__this: *const Self) -> ::ffi_11::c_int {
                unsafe { self::thread_safe_struct::ConstGet(__this) }
            }
            /// A non-const method for testing the generation behavior.
            /// The implementation doesn't actually do anything non-const, but it doesn't
            /// matter for what we are testing, here.
            ///
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            ///
            /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=19
            #[inline(always)]
            pub unsafe fn NonConstGet(__this: *mut Self) -> ::ffi_11::c_int {
                unsafe { self::thread_safe_struct::NonConstGet(__this) }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=13
        impl Default for ThreadSafeStruct {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafeStructC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        pub mod thread_safe_struct {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            ///
            /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=15
            #[inline(always)]
            pub(crate) unsafe fn ConstGet(
                __this: *const crate::crubit::test::ThreadSafeStruct,
            ) -> ::ffi_11::c_int {
                unsafe {
                    crate::detail::__rust_thunk___ZNK6crubit4test16ThreadSafeStruct8ConstGetEv(
                        __this,
                    )
                }
            }
            /// A non-const method for testing the generation behavior.
            /// The implementation doesn't actually do anything non-const, but it doesn't
            /// matter for what we are testing, here.
            ///
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            ///
            /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=19
            #[inline(always)]
            pub(crate) unsafe fn NonConstGet(
                __this: *mut crate::crubit::test::ThreadSafeStruct,
            ) -> ::ffi_11::c_int {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test16ThreadSafeStruct11NonConstGetEv(
                        __this,
                    )
                }
            }
        }

        /// A regular (non-thread-safe) struct for comparison.
        ///
        /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=26
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: RegularStruct
        pub struct RegularStruct {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) x_: [::core::mem::MaybeUninit<u8>; 4],
        }
        impl !Send for RegularStruct {}
        impl !Sync for RegularStruct {}
        unsafe impl ::cxx::ExternType for RegularStruct {
            type Id = ::cxx::type_id!("crubit :: test :: RegularStruct");
            type Kind = ::cxx::kind::Trivial;
        }
        impl RegularStruct {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            ///
            /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=28
            #[inline(always)]
            pub unsafe fn ConstGet(__this: *const Self) -> ::ffi_11::c_int {
                unsafe { self::regular_struct::ConstGet(__this) }
            }
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            ///
            /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=29
            #[inline(always)]
            pub unsafe fn NonConstGet(__this: *mut Self) -> ::ffi_11::c_int {
                unsafe { self::regular_struct::NonConstGet(__this) }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=26
        impl Default for RegularStruct {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test13RegularStructC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        pub mod regular_struct {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            ///
            /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=28
            #[inline(always)]
            pub(crate) unsafe fn ConstGet(
                __this: *const crate::crubit::test::RegularStruct,
            ) -> ::ffi_11::c_int {
                unsafe {
                    crate::detail::__rust_thunk___ZNK6crubit4test13RegularStruct8ConstGetEv(__this)
                }
            }
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            ///
            /// Generated from: rs_bindings_from_cc/test/annotations/thread_safe.h;l=29
            #[inline(always)]
            pub(crate) unsafe fn NonConstGet(
                __this: *mut crate::crubit::test::RegularStruct,
            ) -> ::ffi_11::c_int {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test13RegularStruct11NonConstGetEv(
                        __this,
                    )
                }
            }
        }
    }
}

// namespace crubit::test

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafeStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test16ThreadSafeStruct8ConstGetEv(
            __this: *const crate::crubit::test::ThreadSafeStruct,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ThreadSafeStruct11NonConstGetEv(
            __this: *mut crate::crubit::test::ThreadSafeStruct,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test13RegularStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test13RegularStruct8ConstGetEv(
            __this: *const crate::crubit::test::RegularStruct,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test13RegularStruct11NonConstGetEv(
            __this: *mut crate::crubit::test::RegularStruct,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::ThreadSafeStruct>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::ThreadSafeStruct>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ThreadSafeStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ThreadSafeStruct: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ThreadSafeStruct, x_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::RegularStruct>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::RegularStruct>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::RegularStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::RegularStruct: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::RegularStruct, x_) == 0);
};
