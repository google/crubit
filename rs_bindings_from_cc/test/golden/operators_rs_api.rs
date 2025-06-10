// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:operators_cc

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
#![allow(dead_code)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableConstMember
pub struct AddableConstMember {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableConstMember {}
impl !Sync for AddableConstMember {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableConstMember"),
    crate::AddableConstMember
);

impl Default for AddableConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableConstMember {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddableConstMember {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddableConstMember {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddableConstMember {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberaSEOS_(self, __param_0);
        }
    }
}

impl<'a, 'b> ::core::ops::Add<&'b crate::AddableConstMember> for &'a crate::AddableConstMember {
    type Output = crate::AddableConstMember;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableConstMember) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::AddableConstMember>::uninit();
            crate::detail::__rust_thunk___ZNK18AddableConstMemberplERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableNonConstMember
pub struct AddableNonConstMember {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableNonConstMember {}
impl !Sync for AddableNonConstMember {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableNonConstMember"),
    crate::AddableNonConstMember
);

impl Default for AddableNonConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableNonConstMember {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddableNonConstMember {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddableNonConstMember {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddableNonConstMember {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberaSEOS_(self, __param_0);
        }
    }
}

impl<'a, 'b> ::core::ops::Add<&'b crate::AddableNonConstMember>
    for &'a mut crate::AddableNonConstMember
{
    type Output = crate::AddableNonConstMember;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableNonConstMember) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::AddableNonConstMember>::uninit();
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberplERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableFriend
pub struct AddableFriend {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableFriend {}
impl !Sync for AddableFriend {}
forward_declare::unsafe_define!(forward_declare::symbol!("AddableFriend"), crate::AddableFriend);

impl Default for AddableFriend {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableFriend {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddableFriend {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddableFriend {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddableFriend {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendaSEOS_(self, __param_0);
        }
    }
}

impl<'a, 'b> ::core::ops::Add<&'b crate::AddableFriend> for &'a crate::AddableFriend {
    type Output = crate::AddableFriend;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableFriend) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::AddableFriend>::uninit();
            crate::detail::__rust_thunk___ZplRK13AddableFriendS1_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByConstRef
pub struct AddableFreeByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByConstRef {}
impl !Sync for AddableFreeByConstRef {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableFreeByConstRef"),
    crate::AddableFreeByConstRef
);

impl Default for AddableFreeByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableFreeByConstRefC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableFreeByConstRef {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableFreeByConstRefC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddableFreeByConstRef {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddableFreeByConstRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableFreeByConstRefaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddableFreeByConstRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableFreeByConstRefaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByMutRef
pub struct AddableFreeByMutRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByMutRef {}
impl !Sync for AddableFreeByMutRef {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableFreeByMutRef"),
    crate::AddableFreeByMutRef
);

impl Default for AddableFreeByMutRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19AddableFreeByMutRefC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableFreeByMutRef {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19AddableFreeByMutRefC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddableFreeByMutRef {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddableFreeByMutRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN19AddableFreeByMutRefaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddableFreeByMutRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN19AddableFreeByMutRefaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByValue
pub struct AddableFreeByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByValue {}
impl !Sync for AddableFreeByValue {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableFreeByValue"),
    crate::AddableFreeByValue
);

impl Default for AddableFreeByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableFreeByValueC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableFreeByValue {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableFreeByValueC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddableFreeByValue {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddableFreeByValue {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableFreeByValueaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddableFreeByValue {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableFreeByValueaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByRValueRef
pub struct AddableFreeByRValueRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByRValueRef {}
impl !Sync for AddableFreeByRValueRef {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableFreeByRValueRef"),
    crate::AddableFreeByRValueRef
);

impl Default for AddableFreeByRValueRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddableFreeByRValueRefC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableFreeByRValueRef {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddableFreeByRValueRefC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddableFreeByRValueRef {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddableFreeByRValueRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN22AddableFreeByRValueRefaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddableFreeByRValueRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN22AddableFreeByRValueRefaSEOS_(self, __param_0);
        }
    }
}

impl<'a, 'b> ::core::ops::Add<&'b crate::AddableFreeByConstRef>
    for &'a crate::AddableFreeByConstRef
{
    type Output = crate::AddableFreeByConstRef;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableFreeByConstRef) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::AddableFreeByConstRef>::uninit();
            crate::detail::__rust_thunk___ZplRK21AddableFreeByConstRefS1_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::Add<&'b mut crate::AddableFreeByMutRef>
    for &'a mut crate::AddableFreeByMutRef
{
    type Output = crate::AddableFreeByMutRef;
    #[inline(always)]
    fn add(self, rhs: &'b mut crate::AddableFreeByMutRef) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::AddableFreeByMutRef>::uninit();
            crate::detail::__rust_thunk___ZplR19AddableFreeByMutRefS0_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl ::core::ops::Add<Self> for crate::AddableFreeByValue {
    type Output = crate::AddableFreeByValue;
    #[inline(always)]
    fn add(mut self, mut rhs: Self) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___Zpl18AddableFreeByValueS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                &mut self,
                &mut rhs,
            );
            __return.assume_init()
        }
    }
}

// Error while generating bindings for item 'operator+':
// Rvalue reference types are not yet supported as first parameter of operators (b/219826128)

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Overloaded
pub struct Overloaded {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Overloaded {}
impl !Sync for Overloaded {}
forward_declare::unsafe_define!(forward_declare::symbol!("Overloaded"), crate::Overloaded);

impl Default for Overloaded {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for Overloaded {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for Overloaded {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for Overloaded {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Overloaded {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedaSEOS_(self, __param_0);
        }
    }
}

impl<'a> ::core::ops::Add<::core::ffi::c_int> for &'a crate::Overloaded {
    type Output = ::core::ffi::c_int;
    #[inline(always)]
    fn add(self, rhs: ::core::ffi::c_int) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK10Overloadedi(self, rhs) }
    }
}

impl<'a> ::core::ops::Add<::core::ffi::c_uint> for &'a crate::Overloaded {
    type Output = ::core::ffi::c_int;
    #[inline(always)]
    fn add(self, rhs: ::core::ffi::c_uint) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK10Overloadedj(self, rhs) }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=IncompatibleLHS
pub struct IncompatibleLHS {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for IncompatibleLHS {}
impl !Sync for IncompatibleLHS {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("IncompatibleLHS"),
    crate::IncompatibleLHS
);

impl Default for IncompatibleLHS {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for IncompatibleLHS {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for IncompatibleLHS {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for IncompatibleLHS {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for IncompatibleLHS {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for item 'operator+':
// Non-record-nor-reference operator parameters are not yet supported, found ::core::ffi::c_int

// Error while generating bindings for item 'operator+':
// Expected first operator parameter to be a record or incomplete record, found ::core::ffi::c_int

#[derive(Clone, Copy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableReturnsVoid
pub struct AddableReturnsVoid {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableReturnsVoid {}
impl !Sync for AddableReturnsVoid {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableReturnsVoid"),
    crate::AddableReturnsVoid
);

impl Default for AddableReturnsVoid {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableReturnsVoid {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddableReturnsVoid {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddableReturnsVoid {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddableReturnsVoid {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidaSEOS_(self, __param_0);
        }
    }
}

impl<'a, 'b> ::core::ops::Add<&'b crate::AddableReturnsVoid> for &'a crate::AddableReturnsVoid {
    type Output = ();
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableReturnsVoid) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK18AddableReturnsVoidplERKS_(self, rhs) }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableConstMemberNonunpin
pub struct AddableConstMemberNonunpin {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableConstMemberNonunpin {}
impl !Sync for AddableConstMemberNonunpin {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableConstMemberNonunpin"),
    crate::AddableConstMemberNonunpin
);

impl ::ctor::CtorNew<()> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for AddableConstMemberNonunpin {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_(self, __param_0);
        }
    }
}

impl<'a, 'b> ::core::ops::Add<&'b crate::AddableConstMemberNonunpin>
    for &'a crate::AddableConstMemberNonunpin
{
    type Output = impl ::ctor::Ctor<Output = crate::AddableConstMemberNonunpin> + use<'a, 'b>;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableConstMemberNonunpin) -> Self::Output {
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut crate::AddableConstMemberNonunpin| {
                crate::detail::__rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_(
                    dest as *mut ::core::ffi::c_void,
                    self,
                    rhs,
                );
            })
        }
    }
}

impl ::ctor::PinnedDrop for AddableConstMemberNonunpin {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinD1Ev(self)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignMemberInt
pub struct AddAssignMemberInt {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignMemberInt {}
impl !Sync for AddAssignMemberInt {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignMemberInt"),
    crate::AddAssignMemberInt
);

impl Default for AddAssignMemberInt {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignMemberInt {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddAssignMemberInt {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddAssignMemberInt {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddAssignMemberInt {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntaSEOS_(self, __param_0);
        }
    }
}

impl ::core::ops::AddAssign<::core::ffi::c_int> for AddAssignMemberInt {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: ::core::ffi::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntpLEi(self, rhs);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignMemberByConstRef
pub struct AddAssignMemberByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignMemberByConstRef {}
impl !Sync for AddAssignMemberByConstRef {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignMemberByConstRef"),
    crate::AddAssignMemberByConstRef
);

impl Default for AddAssignMemberByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignMemberByConstRef {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddAssignMemberByConstRef {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddAssignMemberByConstRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddAssignMemberByConstRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefaSEOS_(self, __param_0);
        }
    }
}

impl<'b> ::core::ops::AddAssign<&'b Self> for AddAssignMemberByConstRef {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefpLERKS_(self, rhs);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFreeByConstRef
pub struct AddAssignFreeByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFreeByConstRef {}
impl !Sync for AddAssignFreeByConstRef {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignFreeByConstRef"),
    crate::AddAssignFreeByConstRef
);

impl Default for AddAssignFreeByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23AddAssignFreeByConstRefC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignFreeByConstRef {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23AddAssignFreeByConstRefC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddAssignFreeByConstRef {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddAssignFreeByConstRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN23AddAssignFreeByConstRefaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddAssignFreeByConstRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN23AddAssignFreeByConstRefaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for item 'operator+=':
// Compound assignment operators are not yet supported for pointers with unknown lifetime (b/219826128), found *mut crate::AddAssignFreeByConstRef
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFreeByValue
pub struct AddAssignFreeByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFreeByValue {}
impl !Sync for AddAssignFreeByValue {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignFreeByValue"),
    crate::AddAssignFreeByValue
);

impl Default for AddAssignFreeByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20AddAssignFreeByValueC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignFreeByValue {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20AddAssignFreeByValueC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddAssignFreeByValue {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddAssignFreeByValue {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN20AddAssignFreeByValueaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddAssignFreeByValue {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN20AddAssignFreeByValueaSEOS_(self, __param_0);
        }
    }
}

impl ::core::ops::AddAssign<Self> for crate::AddAssignFreeByValue {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, mut rhs: Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR20AddAssignFreeByValueS_(self, &mut rhs);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFriendByConstRef
pub struct AddAssignFriendByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFriendByConstRef {}
impl !Sync for AddAssignFriendByConstRef {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignFriendByConstRef"),
    crate::AddAssignFriendByConstRef
);

impl Default for AddAssignFriendByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignFriendByConstRefC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignFriendByConstRef {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignFriendByConstRefC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddAssignFriendByConstRef {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddAssignFriendByConstRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignFriendByConstRefaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddAssignFriendByConstRef {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignFriendByConstRefaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFriendByValue
pub struct AddAssignFriendByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFriendByValue {}
impl !Sync for AddAssignFriendByValue {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignFriendByValue"),
    crate::AddAssignFriendByValue
);

impl Default for AddAssignFriendByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddAssignFriendByValueC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignFriendByValue {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddAssignFriendByValueC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddAssignFriendByValue {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddAssignFriendByValue {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN22AddAssignFriendByValueaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddAssignFriendByValue {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN22AddAssignFriendByValueaSEOS_(self, __param_0);
        }
    }
}

impl ::core::ops::AddAssign<Self> for crate::AddAssignFriendByValue {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, mut rhs: Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR22AddAssignFriendByValueS_(self, &mut rhs);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignProhibitedConstMember
pub struct AddAssignProhibitedConstMember {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignProhibitedConstMember {}
impl !Sync for AddAssignProhibitedConstMember {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignProhibitedConstMember"),
    crate::AddAssignProhibitedConstMember
);

impl Default for AddAssignProhibitedConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignProhibitedConstMember {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30AddAssignProhibitedConstMemberC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddAssignProhibitedConstMember {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddAssignProhibitedConstMember {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN30AddAssignProhibitedConstMemberaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AddAssignProhibitedConstMember {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN30AddAssignProhibitedConstMemberaSEOS_(self, __param_0);
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nCompound assignment with const left-hand side is not supported, found &'a crate::AddAssignProhibitedConstMember"
)]
pub trait BindingFailedFor_ZNK30AddAssignProhibitedConstMemberpLEi {}
impl<'error> ::core::ops::AddAssign<::core::ffi::c_int> for AddAssignProhibitedConstMember
where
    &'error (): BindingFailedFor_ZNK30AddAssignProhibitedConstMemberpLEi,
{
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: ::core::ffi::c_int) {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignProhibitedFriendConstLhs
pub struct AddAssignProhibitedFriendConstLhs {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignProhibitedFriendConstLhs {}
impl !Sync for AddAssignProhibitedFriendConstLhs {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignProhibitedFriendConstLhs"),
    crate::AddAssignProhibitedFriendConstLhs
);

impl Default for AddAssignProhibitedFriendConstLhs {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignProhibitedFriendConstLhs {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for AddAssignProhibitedFriendConstLhs {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AddAssignProhibitedFriendConstLhs {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN33AddAssignProhibitedFriendConstLhsaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>>
    for AddAssignProhibitedFriendConstLhs
{
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN33AddAssignProhibitedFriendConstLhsaSEOS_(
                self, __param_0,
            );
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nCompound assignment with const left-hand side is not supported, found &'a crate::AddAssignProhibitedFriendConstLhs"
)]
pub trait BindingFailedFor_ZpLRK33AddAssignProhibitedFriendConstLhsi {}
impl<'error> ::core::ops::AddAssign<::core::ffi::c_int> for crate::AddAssignProhibitedFriendConstLhs
where
    &'error (): BindingFailedFor_ZpLRK33AddAssignProhibitedFriendConstLhsi,
{
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: ::core::ffi::c_int) {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ManyOperators
pub struct ManyOperators {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for ManyOperators {}
impl !Sync for ManyOperators {}
forward_declare::unsafe_define!(forward_declare::symbol!("ManyOperators"), crate::ManyOperators);

impl Default for ManyOperators {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for ManyOperators {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for ManyOperators {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for ManyOperators {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsaSEOS_(self, __param_0);
        }
    }
}

impl ManyOperators {
    #[inline(always)]
    pub fn unary_plus<'a>(&'a self) -> crate::ManyOperators {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorspsEv(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
            );
            __return.assume_init()
        }
    }
}

impl<'a> ::core::ops::Neg for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsngEv(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
            );
            __return.assume_init()
        }
    }
}

impl<'a> ::core::ops::Not for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn not(self) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsntEv(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
            );
            __return.assume_init()
        }
    }
}

// Error while generating bindings for item 'ManyOperators::operator~':
// Bindings for this kind of operator (operator ~ with 1 parameter(s)) are not supported

impl<'a, 'b> ::core::ops::Add<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn add(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsplERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::Sub<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn sub(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsmiERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::Mul<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn mul(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsmlERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::Div<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn div(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsdvERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::Rem<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn rem(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsrmERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::BitAnd<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn bitand(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsanERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::BitOr<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn bitor(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsorERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::BitXor<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn bitxor(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorseoERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::Shl<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn shl(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorslsERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'a, 'b> ::core::ops::Shr<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn shr(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsrsERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

impl<'b> ::core::ops::AddAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorspLERKS_(self, rhs);
        }
    }
}

impl<'b> ::core::ops::SubAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn sub_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsmIERKS_(self, rhs);
        }
    }
}

impl<'b> ::core::ops::MulAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn mul_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsmLERKS_(self, rhs);
        }
    }
}

impl<'b> ::core::ops::DivAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn div_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsdVERKS_(self, rhs);
        }
    }
}

impl<'b> ::core::ops::RemAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn rem_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsrMERKS_(self, rhs);
        }
    }
}

impl<'b> ::core::ops::BitAndAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn bitand_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsaNERKS_(self, rhs);
        }
    }
}

impl<'b> ::core::ops::BitOrAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn bitor_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsoRERKS_(self, rhs);
        }
    }
}

impl<'b> ::core::ops::BitXorAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn bitxor_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorseOERKS_(self, rhs);
        }
    }
}

impl<'b> ::core::ops::ShlAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn shl_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorslSERKS_(self, rhs);
        }
    }
}

impl<'b> ::core::ops::ShrAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn shr_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsrSERKS_(self, rhs);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN18AddableConstMemberC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableConstMemberC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableConstMember>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableConstMemberaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddableConstMember,
            __param_0: &'b crate::AddableConstMember,
        ) -> &'a mut crate::AddableConstMember;
        pub(crate) unsafe fn __rust_thunk___ZN18AddableConstMemberaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddableConstMember,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableConstMember>,
        ) -> &'a mut crate::AddableConstMember;
        pub(crate) unsafe fn __rust_thunk___ZNK18AddableConstMemberplERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::AddableConstMember,
            rhs: &'b crate::AddableConstMember,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableNonConstMemberC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableNonConstMemberC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableNonConstMember>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableNonConstMemberaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddableNonConstMember,
            __param_0: &'b crate::AddableNonConstMember,
        ) -> &'a mut crate::AddableNonConstMember;
        pub(crate) unsafe fn __rust_thunk___ZN21AddableNonConstMemberaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddableNonConstMember,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableNonConstMember>,
        ) -> &'a mut crate::AddableNonConstMember;
        pub(crate) unsafe fn __rust_thunk___ZN21AddableNonConstMemberplERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a mut crate::AddableNonConstMember,
            rhs: &'b crate::AddableNonConstMember,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13AddableFriendC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN13AddableFriendC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFriend>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13AddableFriendaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddableFriend,
            __param_0: &'b crate::AddableFriend,
        ) -> &'a mut crate::AddableFriend;
        pub(crate) unsafe fn __rust_thunk___ZN13AddableFriendaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddableFriend,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFriend>,
        ) -> &'a mut crate::AddableFriend;
        pub(crate) unsafe fn __rust_thunk___ZplRK13AddableFriendS1_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            lhs: &'a crate::AddableFriend,
            rhs: &'b crate::AddableFriend,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableFreeByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableFreeByConstRefC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFreeByConstRef>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableFreeByConstRefaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddableFreeByConstRef,
            __param_0: &'b crate::AddableFreeByConstRef,
        ) -> &'a mut crate::AddableFreeByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN21AddableFreeByConstRefaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddableFreeByConstRef,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFreeByConstRef>,
        ) -> &'a mut crate::AddableFreeByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN19AddableFreeByMutRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19AddableFreeByMutRefC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFreeByMutRef>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19AddableFreeByMutRefaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddableFreeByMutRef,
            __param_0: &'b crate::AddableFreeByMutRef,
        ) -> &'a mut crate::AddableFreeByMutRef;
        pub(crate) unsafe fn __rust_thunk___ZN19AddableFreeByMutRefaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddableFreeByMutRef,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFreeByMutRef>,
        ) -> &'a mut crate::AddableFreeByMutRef;
        pub(crate) unsafe fn __rust_thunk___ZN18AddableFreeByValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableFreeByValueC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFreeByValue>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableFreeByValueaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddableFreeByValue,
            __param_0: &'b crate::AddableFreeByValue,
        ) -> &'a mut crate::AddableFreeByValue;
        pub(crate) unsafe fn __rust_thunk___ZN18AddableFreeByValueaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddableFreeByValue,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFreeByValue>,
        ) -> &'a mut crate::AddableFreeByValue;
        pub(crate) unsafe fn __rust_thunk___ZN22AddableFreeByRValueRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22AddableFreeByRValueRefC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFreeByRValueRef>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22AddableFreeByRValueRefaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddableFreeByRValueRef,
            __param_0: &'b crate::AddableFreeByRValueRef,
        ) -> &'a mut crate::AddableFreeByRValueRef;
        pub(crate) unsafe fn __rust_thunk___ZN22AddableFreeByRValueRefaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddableFreeByRValueRef,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFreeByRValueRef>,
        ) -> &'a mut crate::AddableFreeByRValueRef;
        pub(crate) unsafe fn __rust_thunk___ZplRK21AddableFreeByConstRefS1_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            lhs: &'a crate::AddableFreeByConstRef,
            rhs: &'b crate::AddableFreeByConstRef,
        );
        pub(crate) unsafe fn __rust_thunk___ZplR19AddableFreeByMutRefS0_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            lhs: &'a mut crate::AddableFreeByMutRef,
            rhs: &'b mut crate::AddableFreeByMutRef,
        );
        pub(crate) unsafe fn __rust_thunk___Zpl18AddableFreeByValueS_(
            __return: *mut ::core::ffi::c_void,
            lhs: &mut crate::AddableFreeByValue,
            rhs: &mut crate::AddableFreeByValue,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10OverloadedC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN10OverloadedC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::Overloaded>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10OverloadedaSERKS_<'a, 'b>(
            __this: &'a mut crate::Overloaded,
            __param_0: &'b crate::Overloaded,
        ) -> &'a mut crate::Overloaded;
        pub(crate) unsafe fn __rust_thunk___ZN10OverloadedaSEOS_<'a, 'b>(
            __this: &'a mut crate::Overloaded,
            __param_0: ::ctor::RvalueReference<'b, crate::Overloaded>,
        ) -> &'a mut crate::Overloaded;
        #[link_name = "_ZplRK10Overloadedi"]
        pub(crate) unsafe fn __rust_thunk___ZplRK10Overloadedi<'a>(
            lhs: &'a crate::Overloaded,
            rhs: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[link_name = "_ZplRK10Overloadedj"]
        pub(crate) unsafe fn __rust_thunk___ZplRK10Overloadedj<'a>(
            lhs: &'a crate::Overloaded,
            rhs: ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN15IncompatibleLHSC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15IncompatibleLHSC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::IncompatibleLHS>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15IncompatibleLHSaSERKS_<'a, 'b>(
            __this: &'a mut crate::IncompatibleLHS,
            __param_0: &'b crate::IncompatibleLHS,
        ) -> &'a mut crate::IncompatibleLHS;
        pub(crate) unsafe fn __rust_thunk___ZN15IncompatibleLHSaSEOS_<'a, 'b>(
            __this: &'a mut crate::IncompatibleLHS,
            __param_0: ::ctor::RvalueReference<'b, crate::IncompatibleLHS>,
        ) -> &'a mut crate::IncompatibleLHS;
        pub(crate) unsafe fn __rust_thunk___ZN18AddableReturnsVoidC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableReturnsVoidC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableReturnsVoid>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableReturnsVoidaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddableReturnsVoid,
            __param_0: &'b crate::AddableReturnsVoid,
        ) -> &'a mut crate::AddableReturnsVoid;
        pub(crate) unsafe fn __rust_thunk___ZN18AddableReturnsVoidaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddableReturnsVoid,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableReturnsVoid>,
        ) -> &'a mut crate::AddableReturnsVoid;
        #[link_name = "_ZNK18AddableReturnsVoidplERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZNK18AddableReturnsVoidplERKS_<'a, 'b>(
            __this: &'a crate::AddableReturnsVoid,
            rhs: &'b crate::AddableReturnsVoid,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'b crate::AddableConstMemberNonunpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>,
            __param_0: &'b crate::AddableConstMemberNonunpin,
        ) -> ::core::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>;
        pub(crate) unsafe fn __rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::AddableConstMemberNonunpin,
            rhs: &'b crate::AddableConstMemberNonunpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddAssignMemberIntC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddAssignMemberIntC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignMemberInt>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddAssignMemberIntaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignMemberInt,
            __param_0: &'b crate::AddAssignMemberInt,
        ) -> &'a mut crate::AddAssignMemberInt;
        pub(crate) unsafe fn __rust_thunk___ZN18AddAssignMemberIntaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddAssignMemberInt,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignMemberInt>,
        ) -> &'a mut crate::AddAssignMemberInt;
        #[link_name = "_ZN18AddAssignMemberIntpLEi"]
        pub(crate) unsafe fn __rust_thunk___ZN18AddAssignMemberIntpLEi<'a>(
            __this: &'a mut crate::AddAssignMemberInt,
            rhs: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignMemberByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignMemberByConstRefC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignMemberByConstRef>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignMemberByConstRefaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignMemberByConstRef,
            __param_0: &'b crate::AddAssignMemberByConstRef,
        ) -> &'a mut crate::AddAssignMemberByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignMemberByConstRefaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddAssignMemberByConstRef,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignMemberByConstRef>,
        ) -> &'a mut crate::AddAssignMemberByConstRef;
        #[link_name = "_ZN25AddAssignMemberByConstRefpLERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignMemberByConstRefpLERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignMemberByConstRef,
            rhs: &'b crate::AddAssignMemberByConstRef,
        ) -> &'a mut crate::AddAssignMemberByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN23AddAssignFreeByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23AddAssignFreeByConstRefC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFreeByConstRef>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23AddAssignFreeByConstRefaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignFreeByConstRef,
            __param_0: &'b crate::AddAssignFreeByConstRef,
        ) -> &'a mut crate::AddAssignFreeByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN23AddAssignFreeByConstRefaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddAssignFreeByConstRef,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFreeByConstRef>,
        ) -> &'a mut crate::AddAssignFreeByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN20AddAssignFreeByValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20AddAssignFreeByValueC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFreeByValue>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20AddAssignFreeByValueaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignFreeByValue,
            __param_0: &'b crate::AddAssignFreeByValue,
        ) -> &'a mut crate::AddAssignFreeByValue;
        pub(crate) unsafe fn __rust_thunk___ZN20AddAssignFreeByValueaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddAssignFreeByValue,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFreeByValue>,
        ) -> &'a mut crate::AddAssignFreeByValue;
        pub(crate) unsafe fn __rust_thunk___ZpLR20AddAssignFreeByValueS_<'a>(
            lhs: &'a mut crate::AddAssignFreeByValue,
            rhs: &mut crate::AddAssignFreeByValue,
        ) -> &'a mut crate::AddAssignFreeByValue;
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignFriendByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignFriendByConstRefC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFriendByConstRef>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignFriendByConstRefaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignFriendByConstRef,
            __param_0: &'b crate::AddAssignFriendByConstRef,
        ) -> &'a mut crate::AddAssignFriendByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignFriendByConstRefaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddAssignFriendByConstRef,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFriendByConstRef>,
        ) -> &'a mut crate::AddAssignFriendByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN22AddAssignFriendByValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22AddAssignFriendByValueC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFriendByValue>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22AddAssignFriendByValueaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignFriendByValue,
            __param_0: &'b crate::AddAssignFriendByValue,
        ) -> &'a mut crate::AddAssignFriendByValue;
        pub(crate) unsafe fn __rust_thunk___ZN22AddAssignFriendByValueaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddAssignFriendByValue,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFriendByValue>,
        ) -> &'a mut crate::AddAssignFriendByValue;
        pub(crate) unsafe fn __rust_thunk___ZpLR22AddAssignFriendByValueS_<'a>(
            lhs: &'a mut crate::AddAssignFriendByValue,
            rhs: &mut crate::AddAssignFriendByValue,
        ) -> &'a mut crate::AddAssignFriendByValue;
        pub(crate) unsafe fn __rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30AddAssignProhibitedConstMemberC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignProhibitedConstMember>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30AddAssignProhibitedConstMemberaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignProhibitedConstMember,
            __param_0: &'b crate::AddAssignProhibitedConstMember,
        ) -> &'a mut crate::AddAssignProhibitedConstMember;
        pub(crate) unsafe fn __rust_thunk___ZN30AddAssignProhibitedConstMemberaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddAssignProhibitedConstMember,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignProhibitedConstMember>,
        ) -> &'a mut crate::AddAssignProhibitedConstMember;
        pub(crate) unsafe fn __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignProhibitedFriendConstLhs>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsaSERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignProhibitedFriendConstLhs,
            __param_0: &'b crate::AddAssignProhibitedFriendConstLhs,
        ) -> &'a mut crate::AddAssignProhibitedFriendConstLhs;
        pub(crate) unsafe fn __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsaSEOS_<'a, 'b>(
            __this: &'a mut crate::AddAssignProhibitedFriendConstLhs,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignProhibitedFriendConstLhs>,
        ) -> &'a mut crate::AddAssignProhibitedFriendConstLhs;
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::ManyOperators>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsaSERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            __param_0: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsaSEOS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            __param_0: ::ctor::RvalueReference<'b, crate::ManyOperators>,
        ) -> &'a mut crate::ManyOperators;
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorspsEv<'a>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsngEv<'a>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsntEv<'a>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsplERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsmiERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsmlERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsdvERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsrmERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsanERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsorERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorseoERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorslsERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsrsERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        );
        #[link_name = "_ZN13ManyOperatorspLERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorspLERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsmIERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsmIERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsmLERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsmLERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsdVERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsdVERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsrMERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsrMERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsaNERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsaNERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsoRERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsoRERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorseOERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorseOERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorslSERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorslSERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsrSERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsrSERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::AddableConstMember>() == 4);
    assert!(::core::mem::align_of::<crate::AddableConstMember>() == 4);
    static_assertions::assert_impl_all!(crate::AddableConstMember: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableConstMember: Drop);
    assert!(::core::mem::offset_of!(crate::AddableConstMember, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddableNonConstMember>() == 4);
    assert!(::core::mem::align_of::<crate::AddableNonConstMember>() == 4);
    static_assertions::assert_impl_all!(crate::AddableNonConstMember: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableNonConstMember: Drop);
    assert!(::core::mem::offset_of!(crate::AddableNonConstMember, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddableFriend>() == 4);
    assert!(::core::mem::align_of::<crate::AddableFriend>() == 4);
    static_assertions::assert_impl_all!(crate::AddableFriend: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFriend: Drop);
    assert!(::core::mem::offset_of!(crate::AddableFriend, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddableFreeByConstRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddableFreeByConstRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddableFreeByConstRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFreeByConstRef: Drop);

    assert!(::core::mem::size_of::<crate::AddableFreeByMutRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddableFreeByMutRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddableFreeByMutRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFreeByMutRef: Drop);

    assert!(::core::mem::size_of::<crate::AddableFreeByValue>() == 1);
    assert!(::core::mem::align_of::<crate::AddableFreeByValue>() == 1);
    static_assertions::assert_impl_all!(crate::AddableFreeByValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFreeByValue: Drop);

    assert!(::core::mem::size_of::<crate::AddableFreeByRValueRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddableFreeByRValueRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddableFreeByRValueRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFreeByRValueRef: Drop);

    assert!(::core::mem::size_of::<crate::Overloaded>() == 1);
    assert!(::core::mem::align_of::<crate::Overloaded>() == 1);
    static_assertions::assert_impl_all!(crate::Overloaded: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Overloaded: Drop);

    assert!(::core::mem::size_of::<crate::IncompatibleLHS>() == 1);
    assert!(::core::mem::align_of::<crate::IncompatibleLHS>() == 1);
    static_assertions::assert_impl_all!(crate::IncompatibleLHS: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::IncompatibleLHS: Drop);

    assert!(::core::mem::size_of::<crate::AddableReturnsVoid>() == 4);
    assert!(::core::mem::align_of::<crate::AddableReturnsVoid>() == 4);
    static_assertions::assert_impl_all!(crate::AddableReturnsVoid: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableReturnsVoid: Drop);
    assert!(::core::mem::offset_of!(crate::AddableReturnsVoid, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddableConstMemberNonunpin>() == 4);
    assert!(::core::mem::align_of::<crate::AddableConstMemberNonunpin>() == 4);
    static_assertions::assert_impl_all!(crate::AddableConstMemberNonunpin: Drop);
    static_assertions::assert_not_impl_any!(crate::AddableConstMemberNonunpin: Copy);
    assert!(::core::mem::offset_of!(crate::AddableConstMemberNonunpin, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddAssignMemberInt>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignMemberInt>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignMemberInt: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignMemberInt: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignMemberByConstRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignMemberByConstRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignMemberByConstRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignMemberByConstRef: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignFreeByConstRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignFreeByConstRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignFreeByConstRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignFreeByConstRef: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignFreeByValue>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignFreeByValue>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignFreeByValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignFreeByValue: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignFriendByConstRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignFriendByConstRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignFriendByConstRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignFriendByConstRef: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignFriendByValue>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignFriendByValue>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignFriendByValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignFriendByValue: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignProhibitedConstMember>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignProhibitedConstMember>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignProhibitedConstMember: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignProhibitedConstMember: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignProhibitedFriendConstLhs>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignProhibitedFriendConstLhs>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignProhibitedFriendConstLhs: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignProhibitedFriendConstLhs: Drop);

    assert!(::core::mem::size_of::<crate::ManyOperators>() == 1);
    assert!(::core::mem::align_of::<crate::ManyOperators>() == 1);
    static_assertions::assert_impl_all!(crate::ManyOperators: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ManyOperators: Drop);
};
