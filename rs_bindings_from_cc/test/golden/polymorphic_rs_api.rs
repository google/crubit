// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:polymorphic_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C, align(8))]
pub struct PolymorphicBase {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 8],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("PolymorphicBase"),
    crate::PolymorphicBase
);

impl !Unpin for PolymorphicBase {}

impl ctor::CtorNew<()> for PolymorphicBase {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15PolymorphicBaseC1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::PolymorphicBase> for PolymorphicBase {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::PolymorphicBase) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15PolymorphicBaseC1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::PolymorphicBase,)> for PolymorphicBase {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::PolymorphicBase,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::PolymorphicBase>>::ctor_new(arg)
    }
}

// rs_bindings_from_cc/test/golden/polymorphic.h;l=10
// Error while generating bindings for item 'PolymorphicBase::operator=':
// Bindings for this kind of operator are not supported

impl Drop for PolymorphicBase {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15PolymorphicBaseD1Ev(self) }
    }
}

#[repr(C, align(8))]
pub struct PolymorphicBase2 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 8],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("PolymorphicBase2"),
    crate::PolymorphicBase2
);

impl !Unpin for PolymorphicBase2 {}

impl ctor::CtorNew<()> for PolymorphicBase2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN16PolymorphicBase2C1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::PolymorphicBase2> for PolymorphicBase2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::PolymorphicBase2) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN16PolymorphicBase2C1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::PolymorphicBase2,)> for PolymorphicBase2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::PolymorphicBase2,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::PolymorphicBase2>>::ctor_new(arg)
    }
}

// rs_bindings_from_cc/test/golden/polymorphic.h;l=14
// Error while generating bindings for item 'PolymorphicBase2::operator=':
// Bindings for this kind of operator are not supported

impl PolymorphicBase2 {
    #[inline(always)]
    pub fn Foo<'a>(self: crate::rust_std::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16PolymorphicBase23FooEv(self) }
    }
}

impl Drop for PolymorphicBase2 {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN16PolymorphicBase2D1Ev(self) }
    }
}

#[repr(C, align(8))]
pub struct PolymorphicDerived {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 16],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("PolymorphicDerived"),
    crate::PolymorphicDerived
);

impl !Unpin for PolymorphicDerived {}

impl ctor::CtorNew<()> for PolymorphicDerived {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::PolymorphicDerived> for PolymorphicDerived {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::PolymorphicDerived) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::PolymorphicDerived,)> for PolymorphicDerived {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::PolymorphicDerived,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::PolymorphicDerived>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::PolymorphicDerived>>
    for PolymorphicDerived
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::PolymorphicDerived>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::PolymorphicDerived>,)>
    for PolymorphicDerived
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::PolymorphicDerived>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::PolymorphicDerived>>>::ctor_new(arg)
    }
}

impl Drop for PolymorphicDerived {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN18PolymorphicDerivedD1Ev(self) }
    }
}

// rs_bindings_from_cc/test/golden/polymorphic.h;l=20
// Error while generating bindings for item 'PolymorphicDerived::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/polymorphic.h;l=20
// Error while generating bindings for item 'PolymorphicDerived::operator=':
// Bindings for this kind of operator are not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_POLYMORPHIC_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN15PolymorphicBaseC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::PolymorphicBase>,
        );
        pub(crate) fn __rust_thunk___ZN15PolymorphicBaseC1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::PolymorphicBase>,
            __param_0: &'b crate::PolymorphicBase,
        );
        pub(crate) fn __rust_thunk___ZN15PolymorphicBaseD1Ev<'a>(
            __this: *mut crate::PolymorphicBase,
        );
        pub(crate) fn __rust_thunk___ZN16PolymorphicBase2C1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::PolymorphicBase2>,
        );
        pub(crate) fn __rust_thunk___ZN16PolymorphicBase2C1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::PolymorphicBase2>,
            __param_0: &'b crate::PolymorphicBase2,
        );
        pub(crate) fn __rust_thunk___ZN16PolymorphicBase23FooEv<'a>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::PolymorphicBase2>,
        );
        pub(crate) fn __rust_thunk___ZN16PolymorphicBase2D1Ev<'a>(
            __this: *mut crate::PolymorphicBase2,
        );
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::PolymorphicDerived>,
        );
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedC1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::PolymorphicDerived>,
            __param_0: &'b crate::PolymorphicDerived,
        );
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::PolymorphicDerived>,
            __param_0: ctor::RvalueReference<'b, crate::PolymorphicDerived>,
        );
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedD1Ev<'a>(
            __this: *mut crate::PolymorphicDerived,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::PolymorphicBase>() == 8usize);
const _: () = assert!(rust_std::mem::align_of::<crate::PolymorphicBase>() == 8usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::PolymorphicBase: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::PolymorphicBase: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::PolymorphicBase2>() == 8usize);
const _: () = assert!(rust_std::mem::align_of::<crate::PolymorphicBase2>() == 8usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::PolymorphicBase2: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::PolymorphicBase2: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::PolymorphicDerived>() == 16usize);
const _: () = assert!(rust_std::mem::align_of::<crate::PolymorphicDerived>() == 8usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::PolymorphicDerived: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::PolymorphicDerived: Drop);
};
