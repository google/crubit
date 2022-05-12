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

/// Implicitly defined special member functions are trivial on a struct with
/// only trivial members.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Trivial {
    pub trivial_field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Trivial"), crate::Trivial);

impl Default for Trivial {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7TrivialC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::Trivial>> for Trivial {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::Trivial>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7TrivialC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=12
// Error while generating bindings for item 'Trivial::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=12
// Error while generating bindings for item 'Trivial::operator=':
// Bindings for this kind of operator are not supported

/// Defaulted special member functions are trivial on a struct with only trivial
/// members.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TrivialWithDefaulted {
    pub trivial_field: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TrivialWithDefaulted"),
    crate::TrivialWithDefaulted
);

impl Default for TrivialWithDefaulted {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20TrivialWithDefaultedC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=22
// Error while generating bindings for item 'TrivialWithDefaulted::operator=':
// Bindings for this kind of operator are not supported

impl<'b> From<ctor::RvalueReference<'b, crate::TrivialWithDefaulted>> for TrivialWithDefaulted {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::TrivialWithDefaulted>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20TrivialWithDefaultedC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=24
// Error while generating bindings for item 'TrivialWithDefaulted::operator=':
// Bindings for this kind of operator are not supported

/// This struct is trivial, and therefore trivially relocatable etc., but still
/// not safe to pass by reference as it is not final.
#[repr(C)]
pub struct TrivialNonfinal {
    pub trivial_field: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TrivialNonfinal"),
    crate::TrivialNonfinal
);

impl !Unpin for TrivialNonfinal {}

impl ctor::CtorNew<()> for TrivialNonfinal {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15TrivialNonfinalC1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::TrivialNonfinal> for TrivialNonfinal {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::TrivialNonfinal) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15TrivialNonfinalC1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::TrivialNonfinal,)> for TrivialNonfinal {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::TrivialNonfinal,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::TrivialNonfinal>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::TrivialNonfinal>> for TrivialNonfinal {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::TrivialNonfinal>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15TrivialNonfinalC1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::TrivialNonfinal>,)> for TrivialNonfinal {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::TrivialNonfinal>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::TrivialNonfinal>>>::ctor_new(arg)
    }
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=33
// Error while generating bindings for item 'TrivialNonfinal::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=33
// Error while generating bindings for item 'TrivialNonfinal::operator=':
// Bindings for this kind of operator are not supported

#[inline(always)]
pub fn TakesByValue(trivial: crate::Trivial) {
    unsafe { crate::detail::__rust_thunk___Z12TakesByValue7Trivial(trivial) }
}

#[inline(always)]
pub fn TakesWithDefaultedByValue(trivial: crate::TrivialWithDefaulted) {
    unsafe {
        crate::detail::__rust_thunk___Z25TakesWithDefaultedByValue20TrivialWithDefaulted(trivial)
    }
}

#[inline(always)]
pub fn TakesTrivialNonfinalByValue(trivial: crate::TrivialNonfinal) {
    unsafe {
        crate::detail::__rust_thunk___Z27TakesTrivialNonfinalByValue15TrivialNonfinal(trivial)
    }
}

#[inline(always)]
pub fn TakesByReference<'a>(trivial: &'a mut crate::Trivial) {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR7Trivial(trivial) }
}

#[inline(always)]
pub fn TakesWithDefaultedByReference<'a>(trivial: &'a mut crate::TrivialWithDefaulted) {
    unsafe {
        crate::detail::__rust_thunk___Z29TakesWithDefaultedByReferenceR20TrivialWithDefaulted(
            trivial,
        )
    }
}

#[inline(always)]
pub fn TakesTrivialNonfinalByReference<'a>(
    trivial: crate::rust_std::pin::Pin<&'a mut crate::TrivialNonfinal>,
) {
    unsafe {
        crate::detail::__rust_thunk___Z31TakesTrivialNonfinalByReferenceR15TrivialNonfinal(trivial)
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN7TrivialC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN7TrivialC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Trivial>,
            __param_0: ctor::RvalueReference<'b, crate::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN20TrivialWithDefaultedC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN20TrivialWithDefaultedC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::TrivialWithDefaulted>,
            __param_0: ctor::RvalueReference<'b, crate::TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN15TrivialNonfinalC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN15TrivialNonfinalC1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::TrivialNonfinal>,
            __param_0: &'b crate::TrivialNonfinal,
        );
        pub(crate) fn __rust_thunk___ZN15TrivialNonfinalC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::TrivialNonfinal>,
            __param_0: ctor::RvalueReference<'b, crate::TrivialNonfinal>,
        );
        #[link_name = "_Z12TakesByValue7Trivial"]
        pub(crate) fn __rust_thunk___Z12TakesByValue7Trivial(trivial: crate::Trivial);
        #[link_name = "_Z25TakesWithDefaultedByValue20TrivialWithDefaulted"]
        pub(crate) fn __rust_thunk___Z25TakesWithDefaultedByValue20TrivialWithDefaulted(
            trivial: crate::TrivialWithDefaulted,
        );
        #[link_name = "_Z27TakesTrivialNonfinalByValue15TrivialNonfinal"]
        pub(crate) fn __rust_thunk___Z27TakesTrivialNonfinalByValue15TrivialNonfinal(
            trivial: crate::TrivialNonfinal,
        );
        #[link_name = "_Z16TakesByReferenceR7Trivial"]
        pub(crate) fn __rust_thunk___Z16TakesByReferenceR7Trivial<'a>(
            trivial: &'a mut crate::Trivial,
        );
        #[link_name = "_Z29TakesWithDefaultedByReferenceR20TrivialWithDefaulted"]
        pub(crate) fn __rust_thunk___Z29TakesWithDefaultedByReferenceR20TrivialWithDefaulted<'a>(
            trivial: &'a mut crate::TrivialWithDefaulted,
        );
        #[link_name = "_Z31TakesTrivialNonfinalByReferenceR15TrivialNonfinal"]
        pub(crate) fn __rust_thunk___Z31TakesTrivialNonfinalByReferenceR15TrivialNonfinal<'a>(
            trivial: crate::rust_std::pin::Pin<&'a mut crate::TrivialNonfinal>,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::Trivial>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::Trivial>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::Trivial: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Trivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Trivial: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::Trivial, trivial_field) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<crate::TrivialWithDefaulted>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::TrivialWithDefaulted>() == 4usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::TrivialWithDefaulted: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TrivialWithDefaulted: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::TrivialWithDefaulted: Drop);
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::TrivialWithDefaulted, trivial_field) * 8 == 0usize
);

const _: () = assert!(rust_std::mem::size_of::<crate::TrivialNonfinal>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::TrivialNonfinal>() == 4usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::TrivialNonfinal: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::TrivialNonfinal: Drop);
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::TrivialNonfinal, trivial_field) * 8 == 0usize
);
