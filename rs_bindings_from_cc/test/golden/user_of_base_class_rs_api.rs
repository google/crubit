// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc

#![rustfmt::skip]
#![feature(
    allocator_api,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls
)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// The same as Derived from inheritance.h, but in a different build target.
///
/// This tests inheritance across library boundaries.
///
/// TODO(b/216195042): Correctly namespace base classes in generated Rust code.
#[::ctor::recursively_pinned]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Derived2
pub struct Derived2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 20],
    pub derived_1: ::core::ffi::c_char,
}
impl !Send for Derived2 {}
impl !Sync for Derived2 {}
forward_declare::unsafe_define!(forward_declare::symbol!("Derived2"), crate::Derived2);

impl ::ctor::CtorNew<()> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN8Derived2C1Ev(dest as *mut ::core::ffi::c_void);
            })
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN8Derived2C1ERKS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN8Derived2C1EOS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for Derived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for Derived2 {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN8Derived2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for Derived2 {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN8Derived2aSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<inheritance_cc::Base0> for crate::Derived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base0 {
        crate::detail::__crubit_dynamic_upcast__8Derived2__to__5Base0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3auser_5fof_5fbase_5fclass_5fcc(derived)
    }
}

unsafe impl oops::Inherits<inheritance_cc::Base1> for crate::Derived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base1 {
        (derived as *const _ as *const u8).offset(8) as *const inheritance_cc::Base1
    }
}

unsafe impl oops::Inherits<inheritance_cc::Base2> for crate::Derived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base2 {
        (derived as *const _ as *const u8).offset(18) as *const inheritance_cc::Base2
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=VirtualDerived2
pub struct VirtualDerived2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 32],
}
impl !Send for VirtualDerived2 {}
impl !Sync for VirtualDerived2 {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("VirtualDerived2"),
    crate::VirtualDerived2
);

impl ::ctor::CtorNew<()> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN15VirtualDerived2C1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN15VirtualDerived2C1ERKS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN15VirtualDerived2C1EOS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for VirtualDerived2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for VirtualDerived2 {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15VirtualDerived2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for VirtualDerived2 {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN15VirtualDerived2aSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<inheritance_cc::VirtualBase1> for crate::VirtualDerived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::VirtualBase1 {
        crate::detail::__crubit_dynamic_upcast__15VirtualDerived2__to__12VirtualBase1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3auser_5fof_5fbase_5fclass_5fcc(derived)
    }
}

unsafe impl oops::Inherits<inheritance_cc::Base1> for crate::VirtualDerived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base1 {
        crate::detail::__crubit_dynamic_upcast__15VirtualDerived2__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3auser_5fof_5fbase_5fclass_5fcc(derived)
    }
}

unsafe impl oops::Inherits<inheritance_cc::VirtualBase2> for crate::VirtualDerived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::VirtualBase2 {
        crate::detail::__crubit_dynamic_upcast__15VirtualDerived2__to__12VirtualBase2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3auser_5fof_5fbase_5fclass_5fcc(derived)
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2C1ERKS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'b crate::Derived2,
        );
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2C1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::Derived2>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2aSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::Derived2>,
            __param_0: &'b crate::Derived2,
        ) -> ::core::pin::Pin<&'a mut crate::Derived2>;
        pub(crate) unsafe fn __rust_thunk___ZN8Derived2aSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::Derived2>,
            __param_0: ::ctor::RvalueReference<'b, crate::Derived2>,
        ) -> ::core::pin::Pin<&'a mut crate::Derived2>;
        pub fn __crubit_dynamic_upcast__8Derived2__to__5Base0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3auser_5fof_5fbase_5fclass_5fcc(
            from: *const crate::Derived2,
        ) -> *const inheritance_cc::Base0;
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2C1ERKS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'b crate::VirtualDerived2,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2C1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualDerived2>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2aSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::VirtualDerived2>,
            __param_0: &'b crate::VirtualDerived2,
        ) -> ::core::pin::Pin<&'a mut crate::VirtualDerived2>;
        pub(crate) unsafe fn __rust_thunk___ZN15VirtualDerived2aSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::VirtualDerived2>,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualDerived2>,
        ) -> ::core::pin::Pin<&'a mut crate::VirtualDerived2>;
        pub fn __crubit_dynamic_upcast__15VirtualDerived2__to__12VirtualBase1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3auser_5fof_5fbase_5fclass_5fcc(
            from: *const crate::VirtualDerived2,
        ) -> *const inheritance_cc::VirtualBase1;
        pub fn __crubit_dynamic_upcast__15VirtualDerived2__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3auser_5fof_5fbase_5fclass_5fcc(
            from: *const crate::VirtualDerived2,
        ) -> *const inheritance_cc::Base1;
        pub fn __crubit_dynamic_upcast__15VirtualDerived2__to__12VirtualBase2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3auser_5fof_5fbase_5fclass_5fcc(
            from: *const crate::VirtualDerived2,
        ) -> *const inheritance_cc::VirtualBase2;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Derived2>() == 24);
    assert!(::core::mem::align_of::<crate::Derived2>() == 8);
    static_assertions::assert_not_impl_any!(crate::Derived2: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::Derived2, derived_1) == 20);
    assert!(::core::mem::size_of::<crate::VirtualDerived2>() == 32);
    assert!(::core::mem::align_of::<crate::VirtualDerived2>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualDerived2: Copy,Drop);
};
