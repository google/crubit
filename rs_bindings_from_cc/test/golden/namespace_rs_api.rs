// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:namespace_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

use ::std as rust_std;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod test_namespace_bindings {
    #[ctor::recursively_pinned]
    #[repr(C)]
    pub struct S {
        pub i: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("S"),
        crate::test_namespace_bindings::S
    );

    // rs_bindings_from_cc/test/golden/namespace.h;l=11
    // Error while generating bindings for item 'S::S':
    // Cannot generate bindings for overloaded function

    impl<'b> ctor::CtorNew<&'b crate::test_namespace_bindings::S> for S {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: &'b crate::test_namespace_bindings::S) -> Self::CtorType {
            let __param_0 = args;
            ctor::FnCtor::new(
                move |dest: crate::rust_std::pin::Pin<
                    &mut crate::rust_std::mem::MaybeUninit<Self>,
                >| {
                    unsafe {
                        crate::detail::__rust_thunk___ZN23test_namespace_bindings1SC1ERKS0_(
                            crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    }
                },
            )
        }
    }
    impl<'b> ctor::CtorNew<(&'b crate::test_namespace_bindings::S,)> for S {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: (&'b crate::test_namespace_bindings::S,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ctor::CtorNew<&'b crate::test_namespace_bindings::S>>::ctor_new(arg)
        }
    }

    impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::test_namespace_bindings::S>> for S {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(
            args: ctor::RvalueReference<'b, crate::test_namespace_bindings::S>,
        ) -> Self::CtorType {
            let __param_0 = args;
            ctor::FnCtor::new(
                move |dest: crate::rust_std::pin::Pin<
                    &mut crate::rust_std::mem::MaybeUninit<Self>,
                >| {
                    unsafe {
                        crate::detail::__rust_thunk___ZN23test_namespace_bindings1SC1EOS0_(
                            crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    }
                },
            )
        }
    }
    impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::test_namespace_bindings::S>,)> for S {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(
            args: (ctor::RvalueReference<'b, crate::test_namespace_bindings::S>,),
        ) -> Self::CtorType {
            let (arg,) = args;
            <Self as ctor::CtorNew<ctor::RvalueReference<'b,crate::test_namespace_bindings::S>>>::ctor_new(arg)
        }
    }

    // rs_bindings_from_cc/test/golden/namespace.h;l=11
    // Error while generating bindings for item 'S::operator=':
    // Bindings for this kind of operator are not supported

    // rs_bindings_from_cc/test/golden/namespace.h;l=11
    // Error while generating bindings for item 'S::operator=':
    // Bindings for this kind of operator are not supported

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
        #[ctor::recursively_pinned]
        #[repr(C)]
        pub struct S {
            __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("S"),
            crate::test_namespace_bindings_reopened::inner::S
        );

        // rs_bindings_from_cc/test/golden/namespace.h;l=31
        // Error while generating bindings for item 'S::S':
        // Cannot generate bindings for overloaded function

        impl<'b> ctor::CtorNew<&'b crate::test_namespace_bindings_reopened::inner::S> for S {
            type CtorType = impl ctor::Ctor<Output = Self>;
            #[inline(always)]
            fn ctor_new(
                args: &'b crate::test_namespace_bindings_reopened::inner::S,
            ) -> Self::CtorType {
                let __param_0 = args;
                ctor::FnCtor::new(
                    move |dest: crate::rust_std::pin::Pin<
                        &mut crate::rust_std::mem::MaybeUninit<Self>,
                    >| {
                        unsafe {
                            crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1ERKS1_(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
                        }
                    },
                )
            }
        }
        impl<'b> ctor::CtorNew<(&'b crate::test_namespace_bindings_reopened::inner::S,)> for S {
            type CtorType = impl ctor::Ctor<Output = Self>;
            #[inline(always)]
            fn ctor_new(
                args: (&'b crate::test_namespace_bindings_reopened::inner::S,),
            ) -> Self::CtorType {
                let (arg,) = args;
                <Self as ctor::CtorNew<&'b crate::test_namespace_bindings_reopened::inner::S>>::ctor_new(arg)
            }
        }

        impl<'b>
            ctor::CtorNew<
                ctor::RvalueReference<'b, crate::test_namespace_bindings_reopened::inner::S>,
            > for S
        {
            type CtorType = impl ctor::Ctor<Output = Self>;
            #[inline(always)]
            fn ctor_new(
                args: ctor::RvalueReference<'b, crate::test_namespace_bindings_reopened::inner::S>,
            ) -> Self::CtorType {
                let __param_0 = args;
                ctor::FnCtor::new(
                    move |dest: crate::rust_std::pin::Pin<
                        &mut crate::rust_std::mem::MaybeUninit<Self>,
                    >| {
                        unsafe {
                            crate::detail::__rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1EOS1_(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
                        }
                    },
                )
            }
        }
        impl<'b>
            ctor::CtorNew<(
                ctor::RvalueReference<'b, crate::test_namespace_bindings_reopened::inner::S>,
            )> for S
        {
            type CtorType = impl ctor::Ctor<Output = Self>;
            #[inline(always)]
            fn ctor_new(
                args: (
                    ctor::RvalueReference<'b, crate::test_namespace_bindings_reopened::inner::S>,
                ),
            ) -> Self::CtorType {
                let (arg,) = args;
                <Self as ctor::CtorNew<
                    ctor::RvalueReference<'b, crate::test_namespace_bindings_reopened::inner::S>,
                >>::ctor_new(arg)
            }
        }

        // rs_bindings_from_cc/test/golden/namespace.h;l=31
        // Error while generating bindings for item 'S::operator=':
        // Bindings for this kind of operator are not supported

        // rs_bindings_from_cc/test/golden/namespace.h;l=31
        // Error while generating bindings for item 'S::operator=':
        // Bindings for this kind of operator are not supported
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

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings1SC1ERKS0_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::test_namespace_bindings::S>,
            __param_0: &'b crate::test_namespace_bindings::S,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings1SC1EOS0_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::test_namespace_bindings::S>,
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings::S>,
        );
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
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1ERKS1_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings_reopened::inner::S,
            >,
            __param_0: &'b crate::test_namespace_bindings_reopened::inner::S,
        );
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1SC1EOS1_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings_reopened::inner::S,
            >,
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings_reopened::inner::S>,
        );
        #[link_name = "_ZN32test_namespace_bindings_reopened1yEv"]
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened1yEv();
        #[link_name = "_ZN32test_namespace_bindings_reopened5inner1zENS0_1SE"]
        pub(crate) fn __rust_thunk___ZN32test_namespace_bindings_reopened5inner1zENS0_1SE(
            s: crate::test_namespace_bindings_reopened::inner::S,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::test_namespace_bindings::S>() == 4);
const _: () = assert!(rust_std::mem::align_of::<crate::test_namespace_bindings::S>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::test_namespace_bindings::S: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::test_namespace_bindings::S: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::test_namespace_bindings::S, i) == 0);

const _: () =
    assert!(rust_std::mem::size_of::<crate::test_namespace_bindings_reopened::inner::S>() == 1);
const _: () =
    assert!(rust_std::mem::align_of::<crate::test_namespace_bindings_reopened::inner::S>() == 1);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::test_namespace_bindings_reopened::inner::S: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::test_namespace_bindings_reopened::inner::S: Drop
    );
};
