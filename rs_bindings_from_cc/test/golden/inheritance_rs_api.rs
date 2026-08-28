// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:inheritance_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// Using classes to force these to be non-POD.
/// In the Itanium ABI, the tail padding of POD types cannot be reused by other
/// objects, even if the POD type is potentially-overlapping.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Base0"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Base0
pub struct Base0 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Base0 {}
impl !Sync for Base0 {}
unsafe impl ::cxx::ExternType for Base0 {
    type Id = ::cxx::type_id!("Base0");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Base0 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base0C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Base1"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Base1
pub struct Base1 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_1_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_2_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for Base1 {}
impl !Sync for Base1 {}
unsafe impl ::cxx::ExternType for Base1 {
    type Id = ::cxx::type_id!("Base1");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Base1 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base1C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Base2"]
#[repr(C, align(2))]
///CRUBIT_ANNOTATE: cpp_type=Base2
pub struct Base2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b2_1_: [::core::mem::MaybeUninit<u8>; 2],
}
impl !Send for Base2 {}
impl !Sync for Base2 {}
unsafe impl ::cxx::ExternType for Base2 {
    type Id = ::cxx::type_id!("Base2");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Base2 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base2C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "7Derived"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Derived
pub struct Derived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 12],
    pub derived_1: ::ffi_11::c_char,
}
impl !Send for Derived {}
impl !Sync for Derived {}
unsafe impl ::cxx::ExternType for Derived {
    type Id = ::cxx::type_id!("Derived");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Derived {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7DerivedC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "12VirtualBase1"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=VirtualBase1
pub struct VirtualBase1 {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
}
impl !Send for VirtualBase1 {}
impl !Sync for VirtualBase1 {}
unsafe impl ::cxx::ExternType for VirtualBase1 {
    type Id = ::cxx::type_id!("VirtualBase1");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for VirtualBase1 {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN12VirtualBase1C1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for VirtualBase1 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN12VirtualBase1C1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for VirtualBase1 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for VirtualBase1 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN12VirtualBase1C1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for VirtualBase1 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for VirtualBase1 {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase1aSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for VirtualBase1 {
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __param_0: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase1aSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "12VirtualBase2"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=VirtualBase2
pub struct VirtualBase2 {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
}
impl !Send for VirtualBase2 {}
impl !Sync for VirtualBase2 {}
unsafe impl ::cxx::ExternType for VirtualBase2 {
    type Id = ::cxx::type_id!("VirtualBase2");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for VirtualBase2 {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN12VirtualBase2C1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for VirtualBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN12VirtualBase2C1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for VirtualBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for VirtualBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN12VirtualBase2C1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for VirtualBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for VirtualBase2 {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase2aSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for VirtualBase2 {
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __param_0: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase2aSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "14VirtualDerived"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=VirtualDerived
pub struct VirtualDerived {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 32],
}
impl !Send for VirtualDerived {}
impl !Sync for VirtualDerived {}
unsafe impl ::cxx::ExternType for VirtualDerived {
    type Id = ::cxx::type_id!("VirtualDerived");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for VirtualDerived {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN14VirtualDerivedC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for VirtualDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN14VirtualDerivedC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for VirtualDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for VirtualDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN14VirtualDerivedC1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
    for VirtualDerived
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for VirtualDerived {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN14VirtualDerivedaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for VirtualDerived {
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __param_0: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN14VirtualDerivedaSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "15MyAbstractClass"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=MyAbstractClass
pub struct MyAbstractClass {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for MyAbstractClass {}
impl !Sync for MyAbstractClass {}
unsafe impl ::cxx::ExternType for MyAbstractClass {
    type Id = ::cxx::type_id!("MyAbstractClass");
    type Kind = ::cxx::kind::Opaque;
}

// error: constructor `MyAbstractClass::MyAbstractClass` could not be bound
//   `MyAbstractClass` can't be used by-value because it has a non-public or deleted destructor

// error: constructor `MyAbstractClass::MyAbstractClass` could not be bound
//   `MyAbstractClass` can't be used by-value because it has a non-public or deleted destructor

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for MyAbstractClass {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15MyAbstractClassaSERKS_(self, __param_0);
        }
    }
}

/// Method inheritance
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "11MethodBase1"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MethodBase1
pub struct MethodBase1 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for MethodBase1 {}
impl !Sync for MethodBase1 {}
unsafe impl ::cxx::ExternType for MethodBase1 {
    type Id = ::cxx::type_id!("MethodBase1");
    type Kind = ::cxx::kind::Trivial;
}
impl MethodBase1 {
    #[inline(always)]
    pub fn Public<'__this>(&'__this mut self) {
        unsafe { self::method_base1::Public(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__param_0`: raw pointer
    #[inline(always)]
    pub unsafe fn Equals<'__this>(&'__this mut self, __param_0: *const Self) {
        unsafe { self::method_base1::Equals(self, __param_0) }
    }
    #[inline(always)]
    pub fn Colliding1<'__this>(&'__this mut self) {
        unsafe { self::method_base1::Colliding1(self) }
    }
    #[inline(always)]
    pub fn Colliding2<'__this>(&'__this mut self) {
        unsafe { self::method_base1::Colliding2(self) }
    }
}

impl Default for MethodBase1 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase1C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod method_base1 {
    #[inline(always)]
    pub(crate) fn Public<'__this>(__this: &'__this mut crate::MethodBase1) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase16PublicEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__param_0`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn Equals<'__this>(
        __this: &'__this mut crate::MethodBase1,
        __param_0: *const crate::MethodBase1,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase16EqualsEPKS_(__this, __param_0) }
    }
    #[inline(always)]
    pub(crate) fn Colliding1<'__this>(__this: &'__this mut crate::MethodBase1) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase110Colliding1Ev(__this) }
    }
    #[inline(always)]
    pub(crate) fn Colliding2<'__this>(__this: &'__this mut crate::MethodBase1) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase110Colliding2Ev(__this) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "11MethodBase2"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MethodBase2
pub struct MethodBase2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for MethodBase2 {}
impl !Sync for MethodBase2 {}
unsafe impl ::cxx::ExternType for MethodBase2 {
    type Id = ::cxx::type_id!("MethodBase2");
    type Kind = ::cxx::kind::Trivial;
}
impl MethodBase2 {
    #[inline(always)]
    pub fn Colliding1<'__this>(&'__this mut self) {
        unsafe { self::method_base2::Colliding1(self) }
    }
    #[inline(always)]
    pub fn Colliding2<'__this>(&'__this mut self) {
        unsafe { self::method_base2::Colliding2(self) }
    }
}

impl Default for MethodBase2 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase2C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod method_base2 {
    #[inline(always)]
    pub(crate) fn Colliding1<'__this>(__this: &'__this mut crate::MethodBase2) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase210Colliding1Ev(__this) }
    }
    #[inline(always)]
    pub(crate) fn Colliding2<'__this>(__this: &'__this mut crate::MethodBase2) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase210Colliding2Ev(__this) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "13MethodDerived"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MethodDerived
pub struct MethodDerived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for MethodDerived {}
impl !Sync for MethodDerived {}
unsafe impl ::cxx::ExternType for MethodDerived {
    type Id = ::cxx::type_id!("MethodDerived");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for MethodDerived {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13MethodDerivedC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: function `MethodDerived::Colliding1` could not be bound
//   Function aliases are not yet supported.

// error: function `MethodDerived::Protected1` could not be bound
//   Function aliases are not yet supported.

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN5Base0C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5Base1C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5Base2C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN7DerivedC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase1C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase1C1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::VirtualBase1,
        );
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase1C1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::VirtualBase1>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase1aSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::VirtualBase1>,
            __param_0: &'__param_0 crate::VirtualBase1,
        ) -> ::core::pin::Pin<&'__this mut crate::VirtualBase1>;
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase1aSEOS_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::VirtualBase1>,
            __param_0: ::ctor::RvalueReference<'_, crate::VirtualBase1>,
        ) -> ::core::pin::Pin<&'__this mut crate::VirtualBase1>;
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase2C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase2C1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::VirtualBase2,
        );
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase2C1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::VirtualBase2>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase2aSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::VirtualBase2>,
            __param_0: &'__param_0 crate::VirtualBase2,
        ) -> ::core::pin::Pin<&'__this mut crate::VirtualBase2>;
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase2aSEOS_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::VirtualBase2>,
            __param_0: ::ctor::RvalueReference<'_, crate::VirtualBase2>,
        ) -> ::core::pin::Pin<&'__this mut crate::VirtualBase2>;
        pub(crate) unsafe fn __rust_thunk___ZN14VirtualDerivedC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN14VirtualDerivedC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::VirtualDerived,
        );
        pub(crate) unsafe fn __rust_thunk___ZN14VirtualDerivedC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::VirtualDerived>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN14VirtualDerivedaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::VirtualDerived>,
            __param_0: &'__param_0 crate::VirtualDerived,
        ) -> ::core::pin::Pin<&'__this mut crate::VirtualDerived>;
        pub(crate) unsafe fn __rust_thunk___ZN14VirtualDerivedaSEOS_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::VirtualDerived>,
            __param_0: ::ctor::RvalueReference<'_, crate::VirtualDerived>,
        ) -> ::core::pin::Pin<&'__this mut crate::VirtualDerived>;
        pub(crate) unsafe fn __rust_thunk___ZN15MyAbstractClassaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::MyAbstractClass>,
            __param_0: &'__param_0 crate::MyAbstractClass,
        ) -> ::core::pin::Pin<&'__this mut crate::MyAbstractClass>;
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase1C1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN11MethodBase16PublicEv"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase16PublicEv<'__this>(
            __this: &'__this mut crate::MethodBase1,
        );
        #[link_name = "_ZN11MethodBase16EqualsEPKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase16EqualsEPKS_<'__this>(
            __this: &'__this mut crate::MethodBase1,
            __param_0: *const crate::MethodBase1,
        );
        #[link_name = "_ZN11MethodBase110Colliding1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase110Colliding1Ev<'__this>(
            __this: &'__this mut crate::MethodBase1,
        );
        #[link_name = "_ZN11MethodBase110Colliding2Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase110Colliding2Ev<'__this>(
            __this: &'__this mut crate::MethodBase1,
        );
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase2C1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN11MethodBase210Colliding1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase210Colliding1Ev<'__this>(
            __this: &'__this mut crate::MethodBase2,
        );
        #[link_name = "_ZN11MethodBase210Colliding2Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase210Colliding2Ev<'__this>(
            __this: &'__this mut crate::MethodBase2,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13MethodDerivedC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Base0>() == 1);
    assert!(::core::mem::align_of::<crate::Base0>() == 1);
    static_assertions::assert_impl_all!(crate::Base0: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Base0: Drop);

    assert!(::core::mem::size_of::<crate::Base1>() == 16);
    assert!(::core::mem::align_of::<crate::Base1>() == 8);
    static_assertions::assert_impl_all!(crate::Base1: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Base1: Drop);
    assert!(::core::mem::offset_of!(crate::Base1, b1_1_) == 0);
    assert!(::core::mem::offset_of!(crate::Base1, b1_2_) == 8);
    assert!(::core::mem::size_of::<crate::Base2>() == 2);
    assert!(::core::mem::align_of::<crate::Base2>() == 2);
    static_assertions::assert_impl_all!(crate::Base2: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Base2: Drop);
    assert!(::core::mem::offset_of!(crate::Base2, b2_1_) == 0);
    assert!(::core::mem::size_of::<crate::Derived>() == 16);
    assert!(::core::mem::align_of::<crate::Derived>() == 8);
    static_assertions::assert_impl_all!(crate::Derived: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Derived: Drop);
    assert!(::core::mem::offset_of!(crate::Derived, derived_1) == 12);
    assert!(::core::mem::size_of::<crate::VirtualBase1>() == 24);
    assert!(::core::mem::align_of::<crate::VirtualBase1>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualBase1: Copy,Drop);

    assert!(::core::mem::size_of::<crate::VirtualBase2>() == 24);
    assert!(::core::mem::align_of::<crate::VirtualBase2>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualBase2: Copy,Drop);

    assert!(::core::mem::size_of::<crate::VirtualDerived>() == 32);
    assert!(::core::mem::align_of::<crate::VirtualDerived>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualDerived: Copy,Drop);

    assert!(::core::mem::size_of::<crate::MyAbstractClass>() == 8);
    assert!(::core::mem::align_of::<crate::MyAbstractClass>() == 8);
    static_assertions::assert_not_impl_any!(crate::MyAbstractClass: Copy,Drop);

    assert!(::core::mem::size_of::<crate::MethodBase1>() == 1);
    assert!(::core::mem::align_of::<crate::MethodBase1>() == 1);
    static_assertions::assert_impl_all!(crate::MethodBase1: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MethodBase1: Drop);

    assert!(::core::mem::size_of::<crate::MethodBase2>() == 1);
    assert!(::core::mem::align_of::<crate::MethodBase2>() == 1);
    static_assertions::assert_impl_all!(crate::MethodBase2: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MethodBase2: Drop);

    assert!(::core::mem::size_of::<crate::MethodDerived>() == 1);
    assert!(::core::mem::align_of::<crate::MethodDerived>() == 1);
    static_assertions::assert_impl_all!(crate::MethodDerived: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MethodDerived: Drop);
};
