// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:rust_name
// Features: do_not_hardcode_status_bridge, experimental, infer_operator_lifetimes, supported, unsafe_types, wrapper

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod crubit {
    pub mod test {
        /// Generated from: rs_bindings_from_cc/test/annotations/rust_name.h;l=13
        #[inline(always)]
        pub fn free_fn_new_name() {
            unsafe { crate::detail::__rust_thunk___ZN6crubit4test13FreeFnOldNameEv() }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/rust_name.h;l=15
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: StructOldName
        pub struct StructNewName {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for StructNewName {}
        impl !Sync for StructNewName {}
        unsafe impl ::cxx::ExternType for StructNewName {
            type Id = ::cxx::type_id!("crubit :: test :: StructOldName");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: StructOldName"),
            crate::crubit::test::StructNewName
        );

        /// Generated from: rs_bindings_from_cc/test/annotations/rust_name.h;l=15
        impl Default for StructNewName {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test13StructOldNameC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/rust_name.h;l=17
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: SomeStruct
        pub struct SomeStruct {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            pub field_new_name: ::core::ffi::c_int,
        }
        impl !Send for SomeStruct {}
        impl !Sync for SomeStruct {}
        unsafe impl ::cxx::ExternType for SomeStruct {
            type Id = ::cxx::type_id!("crubit :: test :: SomeStruct");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: SomeStruct"),
            crate::crubit::test::SomeStruct
        );

        /// Generated from: rs_bindings_from_cc/test/annotations/rust_name.h;l=18
        impl Default for SomeStruct {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test10SomeStructC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        impl SomeStruct {
            /// Generated from: support/annotations_internal.h;l=13
            /// Expanded at: rs_bindings_from_cc/test/annotations/rust_name.h;l=19
            #[inline(always)]
            pub fn ConstructorNewName(
                a: ::core::ffi::c_int,
                b: ::core::ffi::c_int,
                c: ::core::ffi::c_int,
            ) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test10SomeStructC1Eiii(
                        &raw mut tmp as *mut _,
                        a,
                        b,
                        c,
                    );
                    tmp.assume_init()
                }
            }
        }

        impl SomeStruct {
            /// Generated from: support/annotations_internal.h;l=13
            /// Expanded at: rs_bindings_from_cc/test/annotations/rust_name.h;l=21
            #[inline(always)]
            pub unsafe fn MethodNewName(__this: *const Self) {
                crate::detail::__rust_thunk___ZNK6crubit4test10SomeStruct13MethodOldNameEv(__this)
            }
        }
    }
}

// namespace crubit::test

/// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: integral_constant < bool , false >
pub struct __CcTemplateInstNSt3__u17integral_constantIbLb0EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u17integral_constantIbLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u17integral_constantIbLb0EEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: integral_constant < bool , false >"),
    crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE
);

/// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
impl Default for __CcTemplateInstNSt3__u17integral_constantIbLb0EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZNSt3__u17integral_constantIbLb0EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fannotations_3arust_5fname(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=22
// Error while generating bindings for global variable 'std::integral_constant<bool, false>::value':
// static data members are not supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=23
// Error while generating bindings for type alias 'value_type':
// Can't generate bindings for value_type, because it is unsupported: b/200067824: type definitions nested inside templated records are not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=24
// Error while generating bindings for type alias 'type':
// Can't generate bindings for type, because it is unsupported: b/200067824: type definitions nested inside templated records are not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=413
// Expanded at: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=25
// Error while generating bindings for function 'std::integral_constant<bool, false>::operator bool':
// Function name is not supported: Unsupported name: operator bool

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=413
// Expanded at: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=27
// Error while generating bindings for function 'std::integral_constant<bool, false>::operator()':
// Bindings for this kind of operator (operator () with 1 parameter(s)) are not supported

/// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: integral_constant < bool , true >
pub struct __CcTemplateInstNSt3__u17integral_constantIbLb1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u17integral_constantIbLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u17integral_constantIbLb1EEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: integral_constant < bool , true >"),
    crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE
);

/// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
impl Default for __CcTemplateInstNSt3__u17integral_constantIbLb1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZNSt3__u17integral_constantIbLb1EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fannotations_3arust_5fname(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=22
// Error while generating bindings for global variable 'std::integral_constant<bool, true>::value':
// static data members are not supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=23
// Error while generating bindings for type alias 'value_type':
// Can't generate bindings for value_type, because it is unsupported: b/200067824: type definitions nested inside templated records are not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=24
// Error while generating bindings for type alias 'type':
// Can't generate bindings for type, because it is unsupported: b/200067824: type definitions nested inside templated records are not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=413
// Expanded at: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=25
// Error while generating bindings for function 'std::integral_constant<bool, true>::operator bool':
// Function name is not supported: Unsupported name: operator bool

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=413
// Expanded at: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=27
// Error while generating bindings for function 'std::integral_constant<bool, true>::operator()':
// Bindings for this kind of operator (operator () with 1 parameter(s)) are not supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test13FreeFnOldNameEv();
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test13StructOldNameC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test10SomeStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test10SomeStructC1Eiii(
            __this: *mut crate::crubit::test::SomeStruct,
            a: ::core::ffi::c_int,
            b: ::core::ffi::c_int,
            c: ::core::ffi::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test10SomeStruct13MethodOldNameEv(
            __this: *const crate::crubit::test::SomeStruct,
        );
        pub(crate) unsafe fn __rust_thunk___ZNSt3__u17integral_constantIbLb0EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fannotations_3arust_5fname(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNSt3__u17integral_constantIbLb1EEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fannotations_3arust_5fname(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::StructNewName>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::StructNewName>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::StructNewName: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::StructNewName: Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::SomeStruct>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::SomeStruct>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::SomeStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::SomeStruct: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::SomeStruct, field_new_name) == 0);
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE: Drop);
};
