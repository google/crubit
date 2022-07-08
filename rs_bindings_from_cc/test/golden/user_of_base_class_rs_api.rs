// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// The same as Derived from inheritance.h, but in a different build target.
///
/// This tests inheritance across library boundaries.
///
/// TODO(b/216195042): Correctly namespace base classes in generated Rust code.
#[::ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct Derived2 {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 20],
    pub derived_1: u8,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Derived2"), crate::Derived2);

impl ::ctor::CtorNew<()> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN8Derived2C1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::Derived2> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::Derived2) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN8Derived2C1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::Derived2,)> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::Derived2,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::Derived2>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::Derived2>> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::Derived2>) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN8Derived2C1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::Derived2>,)> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, crate::Derived2>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::Derived2>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::Derived2> for Derived2 {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b crate::Derived2) {
        unsafe {
            crate::detail::__rust_thunk___ZN8Derived2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::Derived2>> for Derived2 {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::Derived2>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN8Derived2aSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<inheritance_cc::Base0> for Derived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base0 {
        crate::detail::__crubit_dynamic_upcast__Derived2__to__Base0(derived)
    }
}
unsafe impl oops::Inherits<inheritance_cc::Base1> for Derived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base1 {
        (derived as *const _ as *const u8).offset(8) as *const inheritance_cc::Base1
    }
}
unsafe impl oops::Inherits<inheritance_cc::Base2> for Derived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base2 {
        (derived as *const _ as *const u8).offset(18) as *const inheritance_cc::Base2
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct VirtualDerived2 {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 32],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("VirtualDerived2"),
    crate::VirtualDerived2
);

impl ::ctor::CtorNew<()> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15VirtualDerived2C1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::VirtualDerived2> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::VirtualDerived2) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15VirtualDerived2C1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::VirtualDerived2,)> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::VirtualDerived2,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::VirtualDerived2>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::VirtualDerived2>> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::VirtualDerived2>) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN15VirtualDerived2C1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::VirtualDerived2>,)>
    for VirtualDerived2
{
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, crate::VirtualDerived2>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::VirtualDerived2>>>::ctor_new(
            arg,
        )
    }
}

impl<'b> ::ctor::Assign<&'b crate::VirtualDerived2> for VirtualDerived2 {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b crate::VirtualDerived2) {
        unsafe {
            crate::detail::__rust_thunk___ZN15VirtualDerived2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::VirtualDerived2>> for VirtualDerived2 {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::VirtualDerived2>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN15VirtualDerived2aSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<inheritance_cc::VirtualBase1> for VirtualDerived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::VirtualBase1 {
        crate::detail::__crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase1(derived)
    }
}
unsafe impl oops::Inherits<inheritance_cc::Base1> for VirtualDerived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base1 {
        crate::detail::__crubit_dynamic_upcast__VirtualDerived2__to__Base1(derived)
    }
}
unsafe impl oops::Inherits<inheritance_cc::VirtualBase2> for VirtualDerived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::VirtualBase2 {
        crate::detail::__crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase2(derived)
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_BASE_CLASS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN8Derived2C1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Derived2>,
        );
        pub(crate) fn __rust_thunk___ZN8Derived2C1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Derived2>,
            __param_0: &'b crate::Derived2,
        );
        pub(crate) fn __rust_thunk___ZN8Derived2C1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Derived2>,
            __param_0: ::ctor::RvalueReference<'b, crate::Derived2>,
        );
        pub(crate) fn __rust_thunk___ZN8Derived2aSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::Derived2>,
            __param_0: &'b crate::Derived2,
        ) -> ::std::pin::Pin<&'a mut crate::Derived2>;
        pub(crate) fn __rust_thunk___ZN8Derived2aSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::Derived2>,
            __param_0: ::ctor::RvalueReference<'b, crate::Derived2>,
        ) -> ::std::pin::Pin<&'a mut crate::Derived2>;
        pub fn __crubit_dynamic_upcast__Derived2__to__Base0(
            from: *const Derived2,
        ) -> *const inheritance_cc::Base0;
        pub(crate) fn __rust_thunk___ZN15VirtualDerived2C1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::VirtualDerived2>,
        );
        pub(crate) fn __rust_thunk___ZN15VirtualDerived2C1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::VirtualDerived2>,
            __param_0: &'b crate::VirtualDerived2,
        );
        pub(crate) fn __rust_thunk___ZN15VirtualDerived2C1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::VirtualDerived2>,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualDerived2>,
        );
        pub(crate) fn __rust_thunk___ZN15VirtualDerived2aSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::VirtualDerived2>,
            __param_0: &'b crate::VirtualDerived2,
        ) -> ::std::pin::Pin<&'a mut crate::VirtualDerived2>;
        pub(crate) fn __rust_thunk___ZN15VirtualDerived2aSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::VirtualDerived2>,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualDerived2>,
        ) -> ::std::pin::Pin<&'a mut crate::VirtualDerived2>;
        pub fn __crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase1(
            from: *const VirtualDerived2,
        ) -> *const inheritance_cc::VirtualBase1;
        pub fn __crubit_dynamic_upcast__VirtualDerived2__to__Base1(
            from: *const VirtualDerived2,
        ) -> *const inheritance_cc::Base1;
        pub fn __crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase2(
            from: *const VirtualDerived2,
        ) -> *const inheritance_cc::VirtualBase2;
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::Derived2>() == 24);
const _: () = assert!(::std::mem::align_of::<crate::Derived2>() == 8);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Derived2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Derived2: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Derived2, derived_1) == 20);

const _: () = assert!(::std::mem::size_of::<crate::VirtualDerived2>() == 32);
const _: () = assert!(::std::mem::align_of::<crate::VirtualDerived2>() == 8);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::VirtualDerived2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::VirtualDerived2: Drop);
};
