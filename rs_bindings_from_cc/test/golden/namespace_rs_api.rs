// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:namespace_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod test_namespace_bindings {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: S
    pub struct S {
        pub i: ::core::ffi::c_int,
    }
    impl !Send for S {}
    impl !Sync for S {}
    unsafe impl ::cxx::ExternType for S {
        type Id = ::cxx::type_id!("test_namespace_bindings :: S");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for S {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings1SC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    // Error while generating bindings for constructor 'test_namespace_bindings::S::S':
    // Can't generate bindings for test_namespace_bindings::S::S, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings::S::S (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for constructor 'test_namespace_bindings::S::S':
    // Can't generate bindings for test_namespace_bindings::S::S, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings::S::S (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'test_namespace_bindings::S::operator=':
    // Can't generate bindings for test_namespace_bindings::S::operator=, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings::S::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings::S::operator= (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'test_namespace_bindings::S::operator=':
    // Can't generate bindings for test_namespace_bindings::S::operator=, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings::S::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings::S::operator= (the type of __param_0 (parameter #1): references are not supported)

    /// Free comment inside namespace
    #[inline(always)]
    pub fn f(mut s: crate::test_namespace_bindings::S) -> ::core::ffi::c_int {
        unsafe { crate::detail::__rust_thunk___ZN23test_namespace_bindings1fENS_1SE(&mut s) }
    }

    #[inline(always)]
    pub fn inline_function() {
        unsafe { crate::detail::__rust_thunk___ZN23test_namespace_bindings15inline_functionEv() }
    }

    pub mod inner {
        #[inline(always)]
        pub fn i() {
            unsafe { crate::detail::__rust_thunk___ZN23test_namespace_bindings5inner1iEv() }
        }
    }

    // namespace inner
}

// namespace test_namespace_bindings

#[inline(always)]
pub fn identity(mut s: crate::test_namespace_bindings::S) -> crate::test_namespace_bindings::S {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::test_namespace_bindings::S>::uninit();
        crate::detail::__rust_thunk___Z8identityN23test_namespace_bindings1SE(
            &raw mut __return as *mut ::core::ffi::c_void,
            &mut s,
        );
        __return.assume_init()
    }
}

// namespace test_namespace_bindings_reopened

pub mod test_namespace_bindings_reopened {
    #[inline(always)]
    pub fn x() {
        unsafe { crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened1xEv() }
    }

    // namespace inner

    #[inline(always)]
    pub fn y() {
        unsafe { crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened1yEv() }
    }

    pub mod inner {
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings_reopened :: inner :: S
        pub struct S {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for S {}
        impl !Sync for S {}
        unsafe impl ::cxx::ExternType for S {
            type Id = ::cxx::type_id!("test_namespace_bindings_reopened :: inner :: S");
            type Kind = ::cxx::kind::Trivial;
        }

        impl Default for S {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        // Error while generating bindings for constructor 'test_namespace_bindings_reopened::inner::S::S':
        // Can't generate bindings for test_namespace_bindings_reopened::inner::S::S, because of missing required features (crubit.rs-features):
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_reopened::inner::S::S (the type of __param_0 (parameter #1): references are not supported)

        // Error while generating bindings for constructor 'test_namespace_bindings_reopened::inner::S::S':
        // Can't generate bindings for test_namespace_bindings_reopened::inner::S::S, because of missing required features (crubit.rs-features):
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_reopened::inner::S::S (the type of __param_0 (parameter #1): references are not supported)

        // Error while generating bindings for function 'test_namespace_bindings_reopened::inner::S::operator=':
        // Can't generate bindings for test_namespace_bindings_reopened::inner::S::operator=, because of missing required features (crubit.rs-features):
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_reopened::inner::S::operator= (return type: references are not supported)
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_reopened::inner::S::operator= (the type of __param_0 (parameter #1): references are not supported)

        // Error while generating bindings for function 'test_namespace_bindings_reopened::inner::S::operator=':
        // Can't generate bindings for test_namespace_bindings_reopened::inner::S::operator=, because of missing required features (crubit.rs-features):
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_reopened::inner::S::operator= (return type: references are not supported)
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_reopened::inner::S::operator= (the type of __param_0 (parameter #1): references are not supported)

        #[inline(always)]
        pub fn z(mut s: crate::test_namespace_bindings_reopened::inner::S) {
            unsafe {
                crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened5inner1zENS0_1SE(
                    &mut s,
                )
            }
        }
    }

    // namespace inner
}

// namespace test_namespace_bindings_reopened

pub mod test_namespace_bindings_inline {
    pub mod inner {
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings_inline :: inner :: StructInInlineNamespace
        pub struct StructInInlineNamespace {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for StructInInlineNamespace {}
        impl !Sync for StructInInlineNamespace {}
        unsafe impl ::cxx::ExternType for StructInInlineNamespace {
            type Id = ::cxx::type_id!(
                "test_namespace_bindings_inline :: inner :: StructInInlineNamespace"
            );
            type Kind = ::cxx::kind::Trivial;
        }

        impl Default for StructInInlineNamespace {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceC1Ev(&raw mut tmp as*mut _);
                    tmp.assume_init()
                }
            }
        }

        // Error while generating bindings for constructor 'test_namespace_bindings_inline::inner::StructInInlineNamespace::StructInInlineNamespace':
        // Can't generate bindings for test_namespace_bindings_inline::inner::StructInInlineNamespace::StructInInlineNamespace, because of missing required features (crubit.rs-features):
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_inline::inner::StructInInlineNamespace::StructInInlineNamespace (the type of __param_0 (parameter #1): references are not supported)

        // Error while generating bindings for constructor 'test_namespace_bindings_inline::inner::StructInInlineNamespace::StructInInlineNamespace':
        // Can't generate bindings for test_namespace_bindings_inline::inner::StructInInlineNamespace::StructInInlineNamespace, because of missing required features (crubit.rs-features):
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_inline::inner::StructInInlineNamespace::StructInInlineNamespace (the type of __param_0 (parameter #1): references are not supported)

        // Error while generating bindings for function 'test_namespace_bindings_inline::inner::StructInInlineNamespace::operator=':
        // Can't generate bindings for test_namespace_bindings_inline::inner::StructInInlineNamespace::operator=, because of missing required features (crubit.rs-features):
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_inline::inner::StructInInlineNamespace::operator= (return type: references are not supported)
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_inline::inner::StructInInlineNamespace::operator= (the type of __param_0 (parameter #1): references are not supported)

        // Error while generating bindings for function 'test_namespace_bindings_inline::inner::StructInInlineNamespace::operator=':
        // Can't generate bindings for test_namespace_bindings_inline::inner::StructInInlineNamespace::operator=, because of missing required features (crubit.rs-features):
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_inline::inner::StructInInlineNamespace::operator= (return type: references are not supported)
        // //rs_bindings_from_cc/test/golden:namespace_cc needs [//features:experimental] for test_namespace_bindings_inline::inner::StructInInlineNamespace::operator= (the type of __param_0 (parameter #1): references are not supported)
    }
    #[allow(unused_imports)]
    pub use inner::*;

    // namespace inner
}

// namespace test_namespace_bindings_inline

#[inline(always)]
pub fn useStructInInlineNamespaceWithFullQualifier(
    mut s: crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
) {
    unsafe {
        crate::detail::__rust_thunk___Z43useStructInInlineNamespaceWithFullQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(&mut s)
    }
}

#[inline(always)]
pub fn useStructInInlineNamespaceSkipInlineQualifier(
    mut s: crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
) {
    unsafe {
        crate::detail::__rust_thunk___Z45useStructInInlineNamespaceSkipInlineQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(&mut s)
    }
}

pub mod r#impl {
    // `impl` is a reserved keyword in Rust

    #[inline(always)]
    pub fn foo() {
        unsafe { crate::detail::__rust_thunk___ZN4impl3fooEv() }
    }
}

// namespace impl

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings1SC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings1fENS_1SE(
            s: &mut crate::test_namespace_bindings::S,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings15inline_functionEv();
        #[link_name = "_ZN23test_namespace_bindings5inner1iEv"]
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings5inner1iEv();
        pub(crate) unsafe fn __rust_thunk___Z8identityN23test_namespace_bindings1SE(
            __return: *mut ::core::ffi::c_void,
            s: &mut crate::test_namespace_bindings::S,
        );
        #[link_name = "_ZN32test_namespace_bindings_reopened1xEv"]
        pub(crate) unsafe fn __rust_thunk___ZN32test_namespace_bindings_reopened1xEv();
        pub(crate) unsafe fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN32test_namespace_bindings_reopened1yEv"]
        pub(crate) unsafe fn __rust_thunk___ZN32test_namespace_bindings_reopened1yEv();
        pub(crate) unsafe fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1zENS0_1SE(
            s: &mut crate::test_namespace_bindings_reopened::inner::S,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___Z43useStructInInlineNamespaceWithFullQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(
            s: &mut crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
        );
        pub(crate) unsafe fn __rust_thunk___Z45useStructInInlineNamespaceSkipInlineQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(
            s: &mut crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4impl3fooEv();
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::test_namespace_bindings::S>() == 4);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::S>() == 4);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::S: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::S: Drop);
    assert!(::core::mem::offset_of!(crate::test_namespace_bindings::S, i) == 0);
    assert!(::core::mem::size_of::<crate::test_namespace_bindings_reopened::inner::S>() == 1);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings_reopened::inner::S>() == 1);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings_reopened::inner::S: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings_reopened::inner::S: Drop);

    assert!(
        ::core::mem::size_of::<crate::test_namespace_bindings_inline::inner::StructInInlineNamespace>(
        ) == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::test_namespace_bindings_inline::inner::StructInInlineNamespace: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings_inline::inner::StructInInlineNamespace: Drop);
};
