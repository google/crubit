// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:inheritance_cc
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

/// Using classes to force these to be non-POD.
/// In the Itanium ABI, the tail padding of POD types cannot be reused by other
/// objects, even if the POD type is potentially-overlapping.
#[ctor::recursively_pinned]
#[repr(C)]
pub struct Base0 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base0"), crate::Base0);

impl ctor::CtorNew<()> for Base0 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN5Base0C1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::Base0> for Base0 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::Base0) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN5Base0C1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::Base0,)> for Base0 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::Base0,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::Base0>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::Base0>> for Base0 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::Base0>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN5Base0C1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::Base0>,)> for Base0 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::Base0>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::Base0>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::Base0> for Base0 {
    #[inline(always)]
    fn assign<'a>(self: crate::rust_std::pin::Pin<&'a mut Self>, __param_0: &'b crate::Base0) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base0aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<ctor::RvalueReference<'b, crate::Base0>> for Base0 {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<'b, crate::Base0>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base0aSEOS_(self, __param_0);
        }
    }
}

#[ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct Base1 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_1_: [crate::rust_std::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_2_: [crate::rust_std::mem::MaybeUninit<u8>; 8],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base1"), crate::Base1);

impl ctor::CtorNew<()> for Base1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN5Base1C1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::Base1> for Base1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::Base1) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN5Base1C1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::Base1,)> for Base1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::Base1,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::Base1>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::Base1>> for Base1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::Base1>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN5Base1C1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::Base1>,)> for Base1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::Base1>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::Base1>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::Base1> for Base1 {
    #[inline(always)]
    fn assign<'a>(self: crate::rust_std::pin::Pin<&'a mut Self>, __param_0: &'b crate::Base1) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base1aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<ctor::RvalueReference<'b, crate::Base1>> for Base1 {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<'b, crate::Base1>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base1aSEOS_(self, __param_0);
        }
    }
}

#[ctor::recursively_pinned]
#[repr(C, align(2))]
pub struct Base2 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b2_1_: [crate::rust_std::mem::MaybeUninit<u8>; 2],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base2"), crate::Base2);

impl ctor::CtorNew<()> for Base2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN5Base2C1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::Base2> for Base2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::Base2) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN5Base2C1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::Base2,)> for Base2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::Base2,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::Base2>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::Base2>> for Base2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::Base2>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN5Base2C1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::Base2>,)> for Base2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::Base2>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::Base2>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::Base2> for Base2 {
    #[inline(always)]
    fn assign<'a>(self: crate::rust_std::pin::Pin<&'a mut Self>, __param_0: &'b crate::Base2) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<ctor::RvalueReference<'b, crate::Base2>> for Base2 {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<'b, crate::Base2>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base2aSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Derived {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 12],
    pub derived_1: u8,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Derived"), crate::Derived);

impl Default for Derived {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7DerivedC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::Derived>> for Derived {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::Derived>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7DerivedC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/inheritance.h;l=25
// Error while generating bindings for item 'Derived::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/inheritance.h;l=25
// Error while generating bindings for item 'Derived::operator=':
// operator= for Unpin types is not yet supported.

unsafe impl oops::Inherits<crate::Base0> for Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base0 {
        (derived as *const _ as *const u8).offset(0) as *const crate::Base0
    }
}
unsafe impl oops::Inherits<crate::Base1> for Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        (derived as *const _ as *const u8).offset(0) as *const crate::Base1
    }
}
unsafe impl oops::Inherits<crate::Base2> for Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base2 {
        (derived as *const _ as *const u8).offset(10) as *const crate::Base2
    }
}

#[ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct VirtualBase1 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 24],
}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualBase1"), crate::VirtualBase1);

impl ctor::CtorNew<()> for VirtualBase1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN12VirtualBase1C1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::VirtualBase1> for VirtualBase1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::VirtualBase1) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN12VirtualBase1C1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::VirtualBase1,)> for VirtualBase1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::VirtualBase1,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::VirtualBase1>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::VirtualBase1>> for VirtualBase1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::VirtualBase1>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN12VirtualBase1C1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::VirtualBase1>,)> for VirtualBase1 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::VirtualBase1>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::VirtualBase1>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::VirtualBase1> for VirtualBase1 {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: &'b crate::VirtualBase1,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase1aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<ctor::RvalueReference<'b, crate::VirtualBase1>> for VirtualBase1 {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<'b, crate::VirtualBase1>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase1aSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<crate::Base1> for VirtualBase1 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        crate::detail::__crubit_dynamic_upcast__VirtualBase1__to__Base1(derived)
    }
}

#[ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct VirtualBase2 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 24],
}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualBase2"), crate::VirtualBase2);

impl ctor::CtorNew<()> for VirtualBase2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN12VirtualBase2C1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::VirtualBase2> for VirtualBase2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::VirtualBase2) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN12VirtualBase2C1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::VirtualBase2,)> for VirtualBase2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::VirtualBase2,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::VirtualBase2>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::VirtualBase2>> for VirtualBase2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::VirtualBase2>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN12VirtualBase2C1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::VirtualBase2>,)> for VirtualBase2 {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::VirtualBase2>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::VirtualBase2>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::VirtualBase2> for VirtualBase2 {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: &'b crate::VirtualBase2,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<ctor::RvalueReference<'b, crate::VirtualBase2>> for VirtualBase2 {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<'b, crate::VirtualBase2>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase2aSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<crate::Base1> for VirtualBase2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        crate::detail::__crubit_dynamic_upcast__VirtualBase2__to__Base1(derived)
    }
}

#[ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct VirtualDerived {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 32],
}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualDerived"), crate::VirtualDerived);

impl ctor::CtorNew<()> for VirtualDerived {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN14VirtualDerivedC1Ev(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ctor::CtorNew<&'b crate::VirtualDerived> for VirtualDerived {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::VirtualDerived) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN14VirtualDerivedC1ERKS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(&'b crate::VirtualDerived,)> for VirtualDerived {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::VirtualDerived,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<&'b crate::VirtualDerived>>::ctor_new(arg)
    }
}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::VirtualDerived>> for VirtualDerived {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::VirtualDerived>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN14VirtualDerivedC1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::VirtualDerived>,)> for VirtualDerived {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::VirtualDerived>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::VirtualDerived>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::VirtualDerived> for VirtualDerived {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: &'b crate::VirtualDerived,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN14VirtualDerivedaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<ctor::RvalueReference<'b, crate::VirtualDerived>> for VirtualDerived {
    #[inline(always)]
    fn assign<'a>(
        self: crate::rust_std::pin::Pin<&'a mut Self>,
        __param_0: ctor::RvalueReference<'b, crate::VirtualDerived>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN14VirtualDerivedaSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<crate::VirtualBase1> for VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::VirtualBase1 {
        crate::detail::__crubit_dynamic_upcast__VirtualDerived__to__VirtualBase1(derived)
    }
}
unsafe impl oops::Inherits<crate::Base1> for VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        crate::detail::__crubit_dynamic_upcast__VirtualDerived__to__Base1(derived)
    }
}
unsafe impl oops::Inherits<crate::VirtualBase2> for VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::VirtualBase2 {
        crate::detail::__crubit_dynamic_upcast__VirtualDerived__to__VirtualBase2(derived)
    }
}

// rs_bindings_from_cc/test/golden/inheritance.h;l=34
// Error while generating bindings for item 'MyAbstractClass':
// Abstract classes are not supported yet

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITANCE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN5Base0C1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Base0>,
        );
        pub(crate) fn __rust_thunk___ZN5Base0C1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Base0>,
            __param_0: &'b crate::Base0,
        );
        pub(crate) fn __rust_thunk___ZN5Base0C1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Base0>,
            __param_0: ctor::RvalueReference<'b, crate::Base0>,
        );
        pub(crate) fn __rust_thunk___ZN5Base0aSERKS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::Base0>,
            __param_0: &'b crate::Base0,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::Base0>;
        pub(crate) fn __rust_thunk___ZN5Base0aSEOS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::Base0>,
            __param_0: ctor::RvalueReference<'b, crate::Base0>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::Base0>;
        pub(crate) fn __rust_thunk___ZN5Base1C1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Base1>,
        );
        pub(crate) fn __rust_thunk___ZN5Base1C1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Base1>,
            __param_0: &'b crate::Base1,
        );
        pub(crate) fn __rust_thunk___ZN5Base1C1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Base1>,
            __param_0: ctor::RvalueReference<'b, crate::Base1>,
        );
        pub(crate) fn __rust_thunk___ZN5Base1aSERKS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::Base1>,
            __param_0: &'b crate::Base1,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::Base1>;
        pub(crate) fn __rust_thunk___ZN5Base1aSEOS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::Base1>,
            __param_0: ctor::RvalueReference<'b, crate::Base1>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::Base1>;
        pub(crate) fn __rust_thunk___ZN5Base2C1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Base2>,
        );
        pub(crate) fn __rust_thunk___ZN5Base2C1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Base2>,
            __param_0: &'b crate::Base2,
        );
        pub(crate) fn __rust_thunk___ZN5Base2C1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Base2>,
            __param_0: ctor::RvalueReference<'b, crate::Base2>,
        );
        pub(crate) fn __rust_thunk___ZN5Base2aSERKS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::Base2>,
            __param_0: &'b crate::Base2,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::Base2>;
        pub(crate) fn __rust_thunk___ZN5Base2aSEOS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::Base2>,
            __param_0: ctor::RvalueReference<'b, crate::Base2>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::Base2>;
        pub(crate) fn __rust_thunk___ZN7DerivedC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Derived>,
        );
        pub(crate) fn __rust_thunk___ZN7DerivedC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::Derived>,
            __param_0: ctor::RvalueReference<'b, crate::Derived>,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase1C1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::VirtualBase1>,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase1C1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::VirtualBase1>,
            __param_0: &'b crate::VirtualBase1,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase1C1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::VirtualBase1>,
            __param_0: ctor::RvalueReference<'b, crate::VirtualBase1>,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase1aSERKS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::VirtualBase1>,
            __param_0: &'b crate::VirtualBase1,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::VirtualBase1>;
        pub(crate) fn __rust_thunk___ZN12VirtualBase1aSEOS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::VirtualBase1>,
            __param_0: ctor::RvalueReference<'b, crate::VirtualBase1>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::VirtualBase1>;
        pub fn __crubit_dynamic_upcast__VirtualBase1__to__Base1(
            from: *const VirtualBase1,
        ) -> *const crate::Base1;
        pub(crate) fn __rust_thunk___ZN12VirtualBase2C1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::VirtualBase2>,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase2C1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::VirtualBase2>,
            __param_0: &'b crate::VirtualBase2,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase2C1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::VirtualBase2>,
            __param_0: ctor::RvalueReference<'b, crate::VirtualBase2>,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase2aSERKS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::VirtualBase2>,
            __param_0: &'b crate::VirtualBase2,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::VirtualBase2>;
        pub(crate) fn __rust_thunk___ZN12VirtualBase2aSEOS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::VirtualBase2>,
            __param_0: ctor::RvalueReference<'b, crate::VirtualBase2>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::VirtualBase2>;
        pub fn __crubit_dynamic_upcast__VirtualBase2__to__Base1(
            from: *const VirtualBase2,
        ) -> *const crate::Base1;
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::VirtualDerived>,
        );
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedC1ERKS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::VirtualDerived>,
            __param_0: &'b crate::VirtualDerived,
        );
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::VirtualDerived>,
            __param_0: ctor::RvalueReference<'b, crate::VirtualDerived>,
        );
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedaSERKS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::VirtualDerived>,
            __param_0: &'b crate::VirtualDerived,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::VirtualDerived>;
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedaSEOS_<'a, 'b>(
            __this: crate::rust_std::pin::Pin<&'a mut crate::VirtualDerived>,
            __param_0: ctor::RvalueReference<'b, crate::VirtualDerived>,
        ) -> crate::rust_std::pin::Pin<&'a mut crate::VirtualDerived>;
        pub fn __crubit_dynamic_upcast__VirtualDerived__to__VirtualBase1(
            from: *const VirtualDerived,
        ) -> *const crate::VirtualBase1;
        pub fn __crubit_dynamic_upcast__VirtualDerived__to__Base1(
            from: *const VirtualDerived,
        ) -> *const crate::Base1;
        pub fn __crubit_dynamic_upcast__VirtualDerived__to__VirtualBase2(
            from: *const VirtualDerived,
        ) -> *const crate::VirtualBase2;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::Base0>() == 1);
const _: () = assert!(rust_std::mem::align_of::<crate::Base0>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Base0: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Base0: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::Base1>() == 16);
const _: () = assert!(rust_std::mem::align_of::<crate::Base1>() == 8);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Base1: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Base1: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Base1, b1_1_) == 0);
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Base1, b1_2_) == 8);

const _: () = assert!(rust_std::mem::size_of::<crate::Base2>() == 2);
const _: () = assert!(rust_std::mem::align_of::<crate::Base2>() == 2);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Base2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Base2: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Base2, b2_1_) == 0);

const _: () = assert!(rust_std::mem::size_of::<crate::Derived>() == 16);
const _: () = assert!(rust_std::mem::align_of::<crate::Derived>() == 8);
const _: () = {
    static_assertions::assert_impl_all!(crate::Derived: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Derived: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Derived: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Derived, derived_1) == 12);

const _: () = assert!(rust_std::mem::size_of::<crate::VirtualBase1>() == 24);
const _: () = assert!(rust_std::mem::align_of::<crate::VirtualBase1>() == 8);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::VirtualBase1: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::VirtualBase1: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::VirtualBase2>() == 24);
const _: () = assert!(rust_std::mem::align_of::<crate::VirtualBase2>() == 8);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::VirtualBase2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::VirtualBase2: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::VirtualDerived>() == 32);
const _: () = assert!(rust_std::mem::align_of::<crate::VirtualDerived>() == 8);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::VirtualDerived: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::VirtualDerived: Drop);
};
