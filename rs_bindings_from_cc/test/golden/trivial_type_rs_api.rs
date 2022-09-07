// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:trivial_type_cc
#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod ns {
    /// Implicitly defined special member functions are trivial on a struct with
    /// only trivial members.
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct Trivial {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("Trivial"), crate::ns::Trivial);

    impl Default for Trivial {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    impl<'b> From<::ctor::RvalueReference<'b, crate::ns::Trivial>> for Trivial {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>) -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1EOS0_(&mut tmp, __param_0);
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
        crate::ns::TrivialWithDefaulted
    );

    impl Default for TrivialWithDefaulted {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    // rs_bindings_from_cc/test/golden/trivial_type.h;l=23
    // Error while generating bindings for item 'TrivialWithDefaulted::operator=':
    // operator= for Unpin types is not yet supported.

    impl<'b> From<::ctor::RvalueReference<'b, crate::ns::TrivialWithDefaulted>>
        for TrivialWithDefaulted
    {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialWithDefaulted>) -> Self {
            let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedC1EOS0_(
                    &mut tmp, __param_0,
                );
                tmp.assume_init()
            }
        }
    }

    // rs_bindings_from_cc/test/golden/trivial_type.h;l=25
    // Error while generating bindings for item 'TrivialWithDefaulted::operator=':
    // operator= for Unpin types is not yet supported.

    /// This struct is trivial, and therefore trivially relocatable etc., but still
    /// not safe to pass by reference as it is not final.
    #[::ctor::recursively_pinned]
    #[repr(C)]
    pub struct TrivialNonfinal {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TrivialNonfinal"),
        crate::ns::TrivialNonfinal
    );

    impl ::ctor::CtorNew<()> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: ()) -> Self::CtorType {
            let () = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
                    >| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1Ev(
                            ::std::pin::Pin::into_inner_unchecked(dest),
                        );
                    },
                )
            }
        }
    }

    impl<'b> ::ctor::CtorNew<&'b crate::ns::TrivialNonfinal> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: &'b crate::ns::TrivialNonfinal) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
                    >| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1ERKS0_(
                            ::std::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(&'b crate::ns::TrivialNonfinal,)> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: (&'b crate::ns::TrivialNonfinal,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<&'b crate::ns::TrivialNonfinal>>::ctor_new(arg)
        }
    }

    impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>>
        for TrivialNonfinal
    {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(
            args: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        ) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::std::pin::Pin<
                        &mut ::std::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
                    >| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_(
                            ::std::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,)>
        for TrivialNonfinal
    {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(
            args: (::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,),
        ) -> Self::CtorType {
            let (arg,) = args;
            <Self as::ctor::CtorNew<::ctor::RvalueReference<'b,crate::ns::TrivialNonfinal>>>::ctor_new(arg)
        }
    }

    impl<'b> ::ctor::Assign<&'b crate::ns::TrivialNonfinal> for TrivialNonfinal {
        #[inline(always)]
        fn assign<'a>(
            self: ::std::pin::Pin<&'a mut Self>,
            __param_0: &'b crate::ns::TrivialNonfinal,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_(self, __param_0);
            }
        }
    }

    impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>>
        for TrivialNonfinal
    {
        #[inline(always)]
        fn assign<'a>(
            self: ::std::pin::Pin<&'a mut Self>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_(self, __param_0);
            }
        }
    }

    #[inline(always)]
    pub fn TakesByValue(trivial: crate::ns::Trivial) {
        unsafe { crate::detail::__rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(trivial) }
    }

    #[inline(always)]
    pub fn TakesWithDefaultedByValue(trivial: crate::ns::TrivialWithDefaulted) {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByValue(
        trivial: impl ::ctor::Ctor<Output = crate::ns::TrivialNonfinal>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
                ::std::pin::Pin::into_inner_unchecked(::ctor::emplace!(trivial)),
            )
        }
    }

    #[inline(always)]
    pub fn TakesByReference<'a>(trivial: &'a mut crate::ns::Trivial) {
        unsafe { crate::detail::__rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE(trivial) }
    }

    #[inline(always)]
    pub fn TakesWithDefaultedByReference<'a>(trivial: &'a mut crate::ns::TrivialWithDefaulted) {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByReference<'a>(
        trivial: ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE(trivial)
        }
    }
}

// namespace ns

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN2ns7TrivialC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns7TrivialC1EOS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::Trivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedC1EOS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialWithDefaulted>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1ERKS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            __param_0: &'b crate::ns::TrivialNonfinal,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
            __param_0: &'b crate::ns::TrivialNonfinal,
        ) -> ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        ) -> ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        #[link_name = "_ZN2ns12TakesByValueENS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(trivial: crate::ns::Trivial);
        #[link_name = "_ZN2ns25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN2ns25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE(
            trivial: crate::ns::TrivialWithDefaulted,
        );
        pub(crate) fn __rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
            trivial: &mut crate::ns::TrivialNonfinal,
        );
        #[link_name = "_ZN2ns16TakesByReferenceERNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE<'a>(
            trivial: &'a mut crate::ns::Trivial,
        );
        #[link_name = "_ZN2ns29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN2ns29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE<
            'a,
        >(
            trivial: &'a mut crate::ns::TrivialWithDefaulted,
        );
        #[link_name = "_ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: ::std::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
        );
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::ns::Trivial>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::ns::Trivial>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::Trivial: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::Trivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::Trivial: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::Trivial, trivial_field) == 0);
const _: () = assert!(::std::mem::size_of::<crate::ns::TrivialWithDefaulted>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::ns::TrivialWithDefaulted>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::TrivialWithDefaulted: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::TrivialWithDefaulted: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialWithDefaulted: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::TrivialWithDefaulted, trivial_field) == 0);
const _: () = assert!(::std::mem::size_of::<crate::ns::TrivialNonfinal>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::ns::TrivialNonfinal>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialNonfinal: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialNonfinal: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::TrivialNonfinal, trivial_field) == 0);
