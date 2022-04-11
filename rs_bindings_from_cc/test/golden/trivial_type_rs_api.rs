// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:trivial_type_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

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
forward_declare::unsafe_define!(forward_declare::symbol!("Trivial"), Trivial);

impl Default for Trivial {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7TrivialC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, Trivial>> for Trivial {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, Trivial>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
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
    TrivialWithDefaulted
);

impl Default for TrivialWithDefaulted {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20TrivialWithDefaultedC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=22
// Error while generating bindings for item 'TrivialWithDefaulted::operator=':
// Bindings for this kind of operator are not supported

impl<'b> From<ctor::RvalueReference<'b, TrivialWithDefaulted>> for TrivialWithDefaulted {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, TrivialWithDefaulted>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
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
forward_declare::unsafe_define!(forward_declare::symbol!("TrivialNonfinal"), TrivialNonfinal);

impl !Unpin for TrivialNonfinal {}

impl ctor::CtorNew<()> for TrivialNonfinal {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15TrivialNonfinalC1Ev(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b TrivialNonfinal> for TrivialNonfinal {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b TrivialNonfinal) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15TrivialNonfinalC1ERKS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, TrivialNonfinal>> for TrivialNonfinal {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, TrivialNonfinal>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15TrivialNonfinalC1EOS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=33
// Error while generating bindings for item 'TrivialNonfinal::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=33
// Error while generating bindings for item 'TrivialNonfinal::operator=':
// Bindings for this kind of operator are not supported

#[inline(always)]
pub fn TakesByValue(trivial: Trivial) {
    unsafe { crate::detail::__rust_thunk___Z12TakesByValue7Trivial(trivial) }
}

#[inline(always)]
pub fn TakesWithDefaultedByValue(trivial: TrivialWithDefaulted) {
    unsafe {
        crate::detail::__rust_thunk___Z25TakesWithDefaultedByValue20TrivialWithDefaulted(trivial)
    }
}

#[inline(always)]
pub fn TakesTrivialNonfinalByValue(trivial: TrivialNonfinal) {
    unsafe {
        crate::detail::__rust_thunk___Z27TakesTrivialNonfinalByValue15TrivialNonfinal(trivial)
    }
}

#[inline(always)]
pub fn TakesByReference<'a>(trivial: &'a mut Trivial) {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR7Trivial(trivial) }
}

#[inline(always)]
pub fn TakesWithDefaultedByReference<'a>(trivial: &'a mut TrivialWithDefaulted) {
    unsafe {
        crate::detail::__rust_thunk___Z29TakesWithDefaultedByReferenceR20TrivialWithDefaulted(
            trivial,
        )
    }
}

#[inline(always)]
pub fn TakesTrivialNonfinalByReference<'a>(trivial: rust_std::pin::Pin<&'a mut TrivialNonfinal>) {
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
            __this: &'a mut rust_std::mem::MaybeUninit<Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN7TrivialC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<Trivial>,
            __param_0: ctor::RvalueReference<'b, Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN20TrivialWithDefaultedC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN20TrivialWithDefaultedC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<TrivialWithDefaulted>,
            __param_0: ctor::RvalueReference<'b, TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN15TrivialNonfinalC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN15TrivialNonfinalC1ERKS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<TrivialNonfinal>,
            __param_0: &'b TrivialNonfinal,
        );
        pub(crate) fn __rust_thunk___ZN15TrivialNonfinalC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<TrivialNonfinal>,
            __param_0: ctor::RvalueReference<'b, TrivialNonfinal>,
        );
        #[link_name = "_Z12TakesByValue7Trivial"]
        pub(crate) fn __rust_thunk___Z12TakesByValue7Trivial(trivial: Trivial);
        #[link_name = "_Z25TakesWithDefaultedByValue20TrivialWithDefaulted"]
        pub(crate) fn __rust_thunk___Z25TakesWithDefaultedByValue20TrivialWithDefaulted(
            trivial: TrivialWithDefaulted,
        );
        #[link_name = "_Z27TakesTrivialNonfinalByValue15TrivialNonfinal"]
        pub(crate) fn __rust_thunk___Z27TakesTrivialNonfinalByValue15TrivialNonfinal(
            trivial: TrivialNonfinal,
        );
        #[link_name = "_Z16TakesByReferenceR7Trivial"]
        pub(crate) fn __rust_thunk___Z16TakesByReferenceR7Trivial<'a>(trivial: &'a mut Trivial);
        #[link_name = "_Z29TakesWithDefaultedByReferenceR20TrivialWithDefaulted"]
        pub(crate) fn __rust_thunk___Z29TakesWithDefaultedByReferenceR20TrivialWithDefaulted<'a>(
            trivial: &'a mut TrivialWithDefaulted,
        );
        #[link_name = "_Z31TakesTrivialNonfinalByReferenceR15TrivialNonfinal"]
        pub(crate) fn __rust_thunk___Z31TakesTrivialNonfinalByReferenceR15TrivialNonfinal<'a>(
            trivial: rust_std::pin::Pin<&'a mut TrivialNonfinal>,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<Trivial>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<Trivial>() == 4usize);
const _: () = {
    assert_impl_all!(Trivial: Clone);
};
const _: () = {
    assert_impl_all!(Trivial: Copy);
};
const _: () = {
    assert_not_impl_all!(Trivial: Drop);
};
const _: () = assert!(offset_of!(Trivial, trivial_field) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<TrivialWithDefaulted>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<TrivialWithDefaulted>() == 4usize);
const _: () = {
    assert_impl_all!(TrivialWithDefaulted: Clone);
};
const _: () = {
    assert_impl_all!(TrivialWithDefaulted: Copy);
};
const _: () = {
    assert_not_impl_all!(TrivialWithDefaulted: Drop);
};
const _: () = assert!(offset_of!(TrivialWithDefaulted, trivial_field) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<TrivialNonfinal>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<TrivialNonfinal>() == 4usize);
const _: () = {
    assert_not_impl_all!(TrivialNonfinal: Copy);
};
const _: () = {
    assert_not_impl_all!(TrivialNonfinal: Drop);
};
const _: () = assert!(offset_of!(TrivialNonfinal, trivial_field) * 8 == 0usize);
