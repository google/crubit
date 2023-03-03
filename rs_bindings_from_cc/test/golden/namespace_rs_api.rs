// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:namespace_cc
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

pub mod test_namespace_bindings {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct S {
        pub i: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("S"),
        crate::test_namespace_bindings::S
    );

    impl Default for S {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings1SC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    impl<'b> From<::ctor::RvalueReference<'b, Self>> for S {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings1SC1EOS0_(
                    &mut tmp, __param_0,
                );
                tmp.assume_init()
            }
        }
    }

    impl<'b> ::ctor::UnpinAssign<&'b Self> for S {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings1SaSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for S {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings1SaSEOS0_(self, __param_0);
            }
        }
    }

    /// Free comment inside namespace
    #[inline(always)]
    pub fn f(s: crate::test_namespace_bindings::S) -> i32 {
        unsafe { crate::detail::__rust_thunk___ZN23test_namespace_bindings1fENS_1SE(s) }
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
pub fn identity(s: crate::test_namespace_bindings::S) -> crate::test_namespace_bindings::S {
    unsafe { crate::detail::__rust_thunk___Z8identityN23test_namespace_bindings1SE(s) }
}

pub mod test_namespace_bindings_reopened_0 {
    #[inline(always)]
    pub fn x() {
        unsafe { crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened1xEv() }
    }

    pub mod inner_0 {
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct S {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("S"),
            crate::test_namespace_bindings_reopened::inner::S
        );

        impl Default for S {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1Ev(
                        &mut tmp,
                    );
                    tmp.assume_init()
                }
            }
        }

        impl<'b> From<::ctor::RvalueReference<'b, Self>> for S {
            #[inline(always)]
            fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1EOS1_(&mut tmp,__param_0);
                    tmp.assume_init()
                }
            }
        }

        impl<'b> ::ctor::UnpinAssign<&'b Self> for S {
            #[inline(always)]
            fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SaSERKS1_(self,__param_0);
                }
            }
        }

        impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for S {
            #[inline(always)]
            fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SaSEOS1_(self,__param_0);
                }
            }
        }
    }

    // namespace inner
}

// namespace test_namespace_bindings_reopened

pub mod test_namespace_bindings_reopened {
    pub use super::test_namespace_bindings_reopened_0::*;

    #[inline(always)]
    pub fn y() {
        unsafe { crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened1yEv() }
    }

    pub mod inner {
        pub use super::inner_0::*;

        #[inline(always)]
        pub fn z(s: crate::test_namespace_bindings_reopened::inner::S) {
            unsafe {
                crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened5inner1zENS0_1SE(
                    s,
                )
            }
        }
    }

    // namespace inner
}

// namespace test_namespace_bindings_reopened

pub mod test_namespace_bindings_inline {
    pub mod inner {
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct StructInInlineNamespace {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("StructInInlineNamespace"),
            crate::test_namespace_bindings_inline::inner::StructInInlineNamespace
        );

        impl Default for StructInInlineNamespace {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceC1Ev(&mut tmp);
                    tmp.assume_init()
                }
            }
        }

        impl<'b> From<::ctor::RvalueReference<'b, Self>> for StructInInlineNamespace {
            #[inline(always)]
            fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceC1EOS1_(&mut tmp,__param_0);
                    tmp.assume_init()
                }
            }
        }

        impl<'b> ::ctor::UnpinAssign<&'b Self> for StructInInlineNamespace {
            #[inline(always)]
            fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceaSERKS1_(self,__param_0);
                }
            }
        }

        impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for StructInInlineNamespace {
            #[inline(always)]
            fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceaSEOS1_(self,__param_0);
                }
            }
        }
    }
    pub use inner::*;

    // namespace inner
}

// namespace test_namespace_bindings_inline

#[inline(always)]
pub fn useStructInInlineNamespaceWithFullQualifier(
    s: crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
) {
    unsafe {
        crate::detail::__rust_thunk___Z43useStructInInlineNamespaceWithFullQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(s)
    }
}

#[inline(always)]
pub fn useStructInInlineNamespaceSkipInlineQualifier(
    s: crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
) {
    unsafe {
        crate::detail::__rust_thunk___Z45useStructInInlineNamespaceSkipInlineQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(s)
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

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings1SC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::test_namespace_bindings::S>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings1SC1EOS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::test_namespace_bindings::S>,
            __param_0: ::ctor::RvalueReference<'b, crate::test_namespace_bindings::S>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings1SaSERKS0_<'a, 'b>(
            __this: &'a mut crate::test_namespace_bindings::S,
            __param_0: &'b crate::test_namespace_bindings::S,
        ) -> &'a mut crate::test_namespace_bindings::S;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings1SaSEOS0_<'a, 'b>(
            __this: &'a mut crate::test_namespace_bindings::S,
            __param_0: ::ctor::RvalueReference<'b, crate::test_namespace_bindings::S>,
        ) -> &'a mut crate::test_namespace_bindings::S;
        #[link_name = "_ZN23test_namespace_bindings1fENS_1SE"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings1fENS_1SE(
            s: crate::test_namespace_bindings::S,
        ) -> i32;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings15inline_functionEv();
        #[link_name = "_ZN23test_namespace_bindings5inner1iEv"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings5inner1iEv();
        #[link_name = "_Z8identityN23test_namespace_bindings1SE"]
        pub(crate) fn __rust_thunk___Z8identityN23test_namespace_bindings1SE(
            s: crate::test_namespace_bindings::S,
        ) -> crate::test_namespace_bindings::S;
        #[link_name = "_ZN32test_namespace_bindings_reopened1xEv"]
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened1xEv();
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<
                crate::test_namespace_bindings_reopened::inner::S,
            >,
        );
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1EOS1_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<
                crate::test_namespace_bindings_reopened::inner::S,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::test_namespace_bindings_reopened::inner::S,
            >,
        );
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SaSERKS1_<'a, 'b>(
            __this: &'a mut crate::test_namespace_bindings_reopened::inner::S,
            __param_0: &'b crate::test_namespace_bindings_reopened::inner::S,
        ) -> &'a mut crate::test_namespace_bindings_reopened::inner::S;
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SaSEOS1_<'a, 'b>(
            __this: &'a mut crate::test_namespace_bindings_reopened::inner::S,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::test_namespace_bindings_reopened::inner::S,
            >,
        ) -> &'a mut crate::test_namespace_bindings_reopened::inner::S;
        #[link_name = "_ZN32test_namespace_bindings_reopened1yEv"]
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened1yEv();
        #[link_name = "_ZN32test_namespace_bindings_reopened5inner1zENS0_1SE"]
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1zENS0_1SE(
            s: crate::test_namespace_bindings_reopened::inner::S,
        );
        pub(crate) fn __rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceC1Ev<
            'a,
        >(
            __this: &'a mut ::core::mem::MaybeUninit<
                crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
            >,
        );
        pub(crate) fn __rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceC1EOS1_<
            'a,
            'b,
        >(
            __this: &'a mut ::core::mem::MaybeUninit<
                crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
            >,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
            >,
        );
        pub(crate) fn __rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceaSERKS1_<
            'a,
            'b,
        >(
            __this: &'a mut crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
            __param_0: &'b crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
        ) -> &'a mut crate::test_namespace_bindings_inline::inner::StructInInlineNamespace;
        pub(crate) fn __rust_thunk___ZN30test_namespace_bindings_inline5inner23StructInInlineNamespaceaSEOS1_<
            'a,
            'b,
        >(
            __this: &'a mut crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
            __param_0: ::ctor::RvalueReference<
                'b,
                crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
            >,
        ) -> &'a mut crate::test_namespace_bindings_inline::inner::StructInInlineNamespace;
        #[link_name = "_Z43useStructInInlineNamespaceWithFullQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE"]
        pub(crate) fn __rust_thunk___Z43useStructInInlineNamespaceWithFullQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(
            s: crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
        );
        #[link_name = "_Z45useStructInInlineNamespaceSkipInlineQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE"]
        pub(crate) fn __rust_thunk___Z45useStructInInlineNamespaceSkipInlineQualifierN30test_namespace_bindings_inline5inner23StructInInlineNamespaceE(
            s: crate::test_namespace_bindings_inline::inner::StructInInlineNamespace,
        );
        pub(crate) fn __rust_thunk___ZN4impl3fooEv();
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::test_namespace_bindings::S>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::test_namespace_bindings::S>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::S: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::S: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::S: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::test_namespace_bindings::S, i) == 0);

const _: () =
    assert!(::core::mem::size_of::<crate::test_namespace_bindings_reopened::inner::S>() == 1);
const _: () =
    assert!(::core::mem::align_of::<crate::test_namespace_bindings_reopened::inner::S>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings_reopened::inner::S: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings_reopened::inner::S: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::test_namespace_bindings_reopened::inner::S: Drop
    );
};

const _: () = assert!(
    ::core::mem::size_of::<crate::test_namespace_bindings_inline::inner::StructInInlineNamespace>()
        == 1
);
const _: () = assert!(
    ::core::mem::align_of::<crate::test_namespace_bindings_inline::inner::StructInInlineNamespace>(
    ) == 1
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::test_namespace_bindings_inline::inner::StructInInlineNamespace: Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::test_namespace_bindings_inline::inner::StructInInlineNamespace: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::test_namespace_bindings_inline::inner::StructInInlineNamespace: Drop
    );
};
