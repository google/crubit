// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:trivial_type_cc
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
    /// Implicitly defined special member functions are trivial on a struct with
    /// only trivial members.
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct Trivial {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("Trivial"),
        crate::test_namespace_bindings::Trivial
    );

    impl Default for Trivial {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings7TrivialC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    impl<'b> From<ctor::RvalueReference<'b, crate::test_namespace_bindings::Trivial>> for Trivial {
        #[inline(always)]
        fn from(
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings::Trivial>,
        ) -> Self {
            let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings7TrivialC1EOS0_(
                    &mut tmp, __param_0,
                );
                tmp.assume_init()
            }
        }
    }

    // rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    // Error while generating bindings for item 'Trivial::operator=':
    // operator= for Unpin types is not yet supported.

    // rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    // Error while generating bindings for item 'Trivial::operator=':
    // operator= for Unpin types is not yet supported.

    /// Defaulted special member functions are trivial on a struct with only trivial
    /// members.
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct TrivialWithDefaulted {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TrivialWithDefaulted"),
        crate::test_namespace_bindings::TrivialWithDefaulted
    );

    impl Default for TrivialWithDefaulted {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedC1Ev(
                    &mut tmp,
                );
                tmp.assume_init()
            }
        }
    }

    // rs_bindings_from_cc/test/golden/trivial_type.h;l=23
    // Error while generating bindings for item 'TrivialWithDefaulted::operator=':
    // operator= for Unpin types is not yet supported.

    impl<'b> From<ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialWithDefaulted>>
        for TrivialWithDefaulted
    {
        #[inline(always)]
        fn from(
            __param_0: ctor::RvalueReference<
                'b,
                crate::test_namespace_bindings::TrivialWithDefaulted,
            >,
        ) -> Self {
            let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedC1EOS0_(&mut tmp,__param_0);
                tmp.assume_init()
            }
        }
    }

    // rs_bindings_from_cc/test/golden/trivial_type.h;l=25
    // Error while generating bindings for item 'TrivialWithDefaulted::operator=':
    // operator= for Unpin types is not yet supported.

    /// This struct is trivial, and therefore trivially relocatable etc., but still
    /// not safe to pass by reference as it is not final.
    #[ctor::recursively_pinned]
    #[repr(C)]
    pub struct TrivialNonfinal {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TrivialNonfinal"),
        crate::test_namespace_bindings::TrivialNonfinal
    );

    impl ctor::CtorNew<()> for TrivialNonfinal {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: ()) -> Self::CtorType {
            let () = args;
            ctor::FnCtor::new(
                move |dest: crate::rust_std::pin::Pin<
                    &mut crate::rust_std::mem::MaybeUninit<Self>,
                >| {
                    unsafe {
                        crate::detail::__rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalC1Ev(crate::rust_std::pin::Pin::into_inner_unchecked(dest));
                    }
                },
            )
        }
    }

    impl<'b> ctor::CtorNew<&'b crate::test_namespace_bindings::TrivialNonfinal> for TrivialNonfinal {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: &'b crate::test_namespace_bindings::TrivialNonfinal) -> Self::CtorType {
            let __param_0 = args;
            ctor::FnCtor::new(
                move |dest: crate::rust_std::pin::Pin<
                    &mut crate::rust_std::mem::MaybeUninit<Self>,
                >| {
                    unsafe {
                        crate::detail::__rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalC1ERKS0_(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
                    }
                },
            )
        }
    }
    impl<'b> ctor::CtorNew<(&'b crate::test_namespace_bindings::TrivialNonfinal,)> for TrivialNonfinal {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(
            args: (&'b crate::test_namespace_bindings::TrivialNonfinal,),
        ) -> Self::CtorType {
            let (arg,) = args;
            <Self as ctor::CtorNew<&'b crate::test_namespace_bindings::TrivialNonfinal>>::ctor_new(
                arg,
            )
        }
    }

    impl<'b>
        ctor::CtorNew<ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialNonfinal>>
        for TrivialNonfinal
    {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(
            args: ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialNonfinal>,
        ) -> Self::CtorType {
            let __param_0 = args;
            ctor::FnCtor::new(
                move |dest: crate::rust_std::pin::Pin<
                    &mut crate::rust_std::mem::MaybeUninit<Self>,
                >| {
                    unsafe {
                        crate::detail::__rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalC1EOS0_(crate::rust_std::pin::Pin::into_inner_unchecked(dest),__param_0);
                    }
                },
            )
        }
    }
    impl<'b>
        ctor::CtorNew<(ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialNonfinal>,)>
        for TrivialNonfinal
    {
        type CtorType = impl ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(
            args: (ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialNonfinal>,),
        ) -> Self::CtorType {
            let (arg,) = args;
            <Self as ctor::CtorNew<
                ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialNonfinal>,
            >>::ctor_new(arg)
        }
    }

    impl<'b> ::ctor::Assign<&'b crate::test_namespace_bindings::TrivialNonfinal> for TrivialNonfinal {
        #[inline(always)]
        fn assign<'a>(
            self: crate::rust_std::pin::Pin<&'a mut Self>,
            __param_0: &'b crate::test_namespace_bindings::TrivialNonfinal,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalaSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl<'b>
        ::ctor::Assign<ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialNonfinal>>
        for TrivialNonfinal
    {
        #[inline(always)]
        fn assign<'a>(
            self: crate::rust_std::pin::Pin<&'a mut Self>,
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialNonfinal>,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalaSEOS0_(
                    self, __param_0,
                );
            }
        }
    }

    #[inline(always)]
    pub fn TakesByValue(trivial: crate::test_namespace_bindings::Trivial) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings12TakesByValueENS_7TrivialE(
                trivial,
            )
        }
    }

    #[inline(always)]
    pub fn TakesWithDefaultedByValue(
        trivial: crate::test_namespace_bindings::TrivialWithDefaulted,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByValue(trivial: crate::test_namespace_bindings::TrivialNonfinal) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesByReference<'a>(trivial: &'a mut crate::test_namespace_bindings::Trivial) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings16TakesByReferenceERNS_7TrivialE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesWithDefaultedByReference<'a>(
        trivial: &'a mut crate::test_namespace_bindings::TrivialWithDefaulted,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByReference<'a>(
        trivial: crate::rust_std::pin::Pin<&'a mut crate::test_namespace_bindings::TrivialNonfinal>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN23test_namespace_bindings31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE(trivial)
        }
    }
}

// namespace test_namespace_bindings

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings7TrivialC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::Trivial,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings7TrivialC1EOS0_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::Trivial,
            >,
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::TrivialWithDefaulted,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings20TrivialWithDefaultedC1EOS0_<
            'a,
            'b,
        >(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::TrivialWithDefaulted,
            >,
            __param_0: ctor::RvalueReference<
                'b,
                crate::test_namespace_bindings::TrivialWithDefaulted,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::TrivialNonfinal,
            >,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalC1ERKS0_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::TrivialNonfinal,
            >,
            __param_0: &'b crate::test_namespace_bindings::TrivialNonfinal,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalC1EOS0_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<
                crate::test_namespace_bindings::TrivialNonfinal,
            >,
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalaSERKS0_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<
                &'a mut crate::test_namespace_bindings::TrivialNonfinal,
            >,
            __param_0: &'b crate::test_namespace_bindings::TrivialNonfinal,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::test_namespace_bindings::TrivialNonfinal>;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings15TrivialNonfinalaSEOS0_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<
                &'a mut crate::test_namespace_bindings::TrivialNonfinal,
            >,
            __param_0: ctor::RvalueReference<'b, crate::test_namespace_bindings::TrivialNonfinal>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::test_namespace_bindings::TrivialNonfinal>;
        #[link_name = "_ZN23test_namespace_bindings12TakesByValueENS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings12TakesByValueENS_7TrivialE(
            trivial: crate::test_namespace_bindings::Trivial,
        );
        #[link_name = "_ZN23test_namespace_bindings25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE(
            trivial: crate::test_namespace_bindings::TrivialWithDefaulted,
        );
        #[link_name = "_ZN23test_namespace_bindings27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
            trivial: crate::test_namespace_bindings::TrivialNonfinal,
        );
        #[link_name = "_ZN23test_namespace_bindings16TakesByReferenceERNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings16TakesByReferenceERNS_7TrivialE<
            'a,
        >(
            trivial: &'a mut crate::test_namespace_bindings::Trivial,
        );
        #[link_name = "_ZN23test_namespace_bindings29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE<
            'a,
        >(
            trivial: &'a mut crate::test_namespace_bindings::TrivialWithDefaulted,
        );
        #[link_name = "_ZN23test_namespace_bindings31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: crate::rust_std::pin::Pin<
                &'a mut crate::test_namespace_bindings::TrivialNonfinal,
            >,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::test_namespace_bindings::Trivial>() == 4);
const _: () = assert!(rust_std::mem::align_of::<crate::test_namespace_bindings::Trivial>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Trivial: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Trivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::test_namespace_bindings::Trivial: Drop);
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::test_namespace_bindings::Trivial, trivial_field)
        == 0
);
const _: () =
    assert!(rust_std::mem::size_of::<crate::test_namespace_bindings::TrivialWithDefaulted>() == 4);
const _: () =
    assert!(rust_std::mem::align_of::<crate::test_namespace_bindings::TrivialWithDefaulted>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::test_namespace_bindings::TrivialWithDefaulted: Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::TrivialWithDefaulted: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::test_namespace_bindings::TrivialWithDefaulted: Drop
    );
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::test_namespace_bindings::TrivialWithDefaulted,
        trivial_field
    ) == 0
);
const _: () =
    assert!(rust_std::mem::size_of::<crate::test_namespace_bindings::TrivialNonfinal>() == 4);
const _: () =
    assert!(rust_std::mem::align_of::<crate::test_namespace_bindings::TrivialNonfinal>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::test_namespace_bindings::TrivialNonfinal: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::test_namespace_bindings::TrivialNonfinal: Drop);
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::test_namespace_bindings::TrivialNonfinal,
        trivial_field
    ) == 0
);
