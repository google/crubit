// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:ignore_attr

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod crubit {
    pub mod test {
        /// This struct would not have bindings generated without the ignore annotation.
        /// `gnu::abi_tag` is an arbitrarily selected attribute that Crubit doesn't
        /// handle.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test8MyStructB3fooE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: MyStruct
        pub struct MyStruct {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for MyStruct {}
        impl !Sync for MyStruct {}
        unsafe impl ::cxx::ExternType for MyStruct {
            type Id = ::cxx::type_id!("crubit :: test :: MyStruct");
            type Kind = ::cxx::kind::Trivial;
        }

        impl Default for MyStruct {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test8MyStructB3fooC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test12PackedStructE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: PackedStruct
        pub struct PackedStruct {
            pub x: ::ffi_11::c_char,
            pub y: ::ffi_11::c_char,
        }
        impl !Send for PackedStruct {}
        impl !Sync for PackedStruct {}
        unsafe impl ::cxx::ExternType for PackedStruct {
            type Id = ::cxx::type_id!("crubit :: test :: PackedStruct");
            type Kind = ::cxx::kind::Trivial;
        }

        impl Default for PackedStruct {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test12PackedStructC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test13PointerStructE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: PointerStruct
        pub struct PointerStruct {
            pub x: ::ffi_11::c_int,
        }
        impl !Send for PointerStruct {}
        impl !Sync for PointerStruct {}
        unsafe impl ::cxx::ExternType for PointerStruct {
            type Id = ::cxx::type_id!("crubit :: test :: PointerStruct");
            type Kind = ::cxx::kind::Trivial;
        }

        impl Default for PointerStruct {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test13PointerStructC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
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
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test8MyStructB3fooC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test12PackedStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test13PointerStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::MyStruct>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::MyStruct>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::MyStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::MyStruct: Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::PackedStruct>() == 2);
    assert!(::core::mem::align_of::<crate::crubit::test::PackedStruct>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::PackedStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::PackedStruct: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::PackedStruct, x) == 0);
    assert!(::core::mem::offset_of!(crate::crubit::test::PackedStruct, y) == 1);
    assert!(::core::mem::size_of::<crate::crubit::test::PointerStruct>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::PointerStruct>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::PointerStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::PointerStruct: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::PointerStruct, x) == 0);
};
