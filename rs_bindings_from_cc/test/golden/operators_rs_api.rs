// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:operators_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "18AddableConstMember"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableConstMember
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddableConstMember {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableConstMember {}
impl !Sync for AddableConstMember {}
unsafe impl ::cxx::ExternType for AddableConstMember {
    type Id = ::cxx::type_id!("AddableConstMember");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Add<&'rhs crate::AddableConstMember>
    for &'__this crate::AddableConstMember
{
    type Output = crate::AddableConstMember;
    #[inline(always)]
    fn add(self, rhs: &'rhs crate::AddableConstMember) -> Self::Output {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<crate::AddableConstMember>::uninit();
            crate::detail::__rust_thunk___ZNK18AddableConstMemberplERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "21AddableNonConstMember"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableNonConstMember
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddableNonConstMember {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableNonConstMember {}
impl !Sync for AddableNonConstMember {}
unsafe impl ::cxx::ExternType for AddableNonConstMember {
    type Id = ::cxx::type_id!("AddableNonConstMember");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableNonConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Add<&'rhs crate::AddableNonConstMember>
    for &'__this mut crate::AddableNonConstMember
{
    type Output = crate::AddableNonConstMember;
    #[inline(always)]
    fn add(self, rhs: &'rhs crate::AddableNonConstMember) -> Self::Output {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<crate::AddableNonConstMember>::uninit();
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberplERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "13AddableFriend"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableFriend
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddableFriend {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableFriend {}
impl !Sync for AddableFriend {}
unsafe impl ::cxx::ExternType for AddableFriend {
    type Id = ::cxx::type_id!("AddableFriend");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFriend {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'lhs, 'rhs> ::core::ops::Add<&'rhs crate::AddableFriend> for &'lhs crate::AddableFriend {
    type Output = crate::AddableFriend;
    #[inline(always)]
    fn add(self, rhs: &'rhs crate::AddableFriend) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::AddableFriend>::uninit();
            crate::detail::__rust_thunk___ZplRK13AddableFriendS1_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "21AddableFreeByConstRef"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByConstRef
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddableFreeByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByConstRef {}
impl !Sync for AddableFreeByConstRef {}
unsafe impl ::cxx::ExternType for AddableFreeByConstRef {
    type Id = ::cxx::type_id!("AddableFreeByConstRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFreeByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableFreeByConstRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "19AddableFreeByMutRef"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByMutRef
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddableFreeByMutRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByMutRef {}
impl !Sync for AddableFreeByMutRef {}
unsafe impl ::cxx::ExternType for AddableFreeByMutRef {
    type Id = ::cxx::type_id!("AddableFreeByMutRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFreeByMutRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19AddableFreeByMutRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "18AddableFreeByValue"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByValue
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddableFreeByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByValue {}
impl !Sync for AddableFreeByValue {}
unsafe impl ::cxx::ExternType for AddableFreeByValue {
    type Id = ::cxx::type_id!("AddableFreeByValue");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFreeByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableFreeByValueC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "22AddableFreeByRValueRef"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByRValueRef
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddableFreeByRValueRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByRValueRef {}
impl !Sync for AddableFreeByRValueRef {}
unsafe impl ::cxx::ExternType for AddableFreeByRValueRef {
    type Id = ::cxx::type_id!("AddableFreeByRValueRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFreeByRValueRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddableFreeByRValueRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'lhs, 'rhs> ::core::ops::Add<&'rhs crate::AddableFreeByConstRef>
    for &'lhs crate::AddableFreeByConstRef
{
    type Output = crate::AddableFreeByConstRef;
    #[inline(always)]
    fn add(self, rhs: &'rhs crate::AddableFreeByConstRef) -> Self::Output {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<crate::AddableFreeByConstRef>::uninit();
            crate::detail::__rust_thunk___ZplRK21AddableFreeByConstRefS1_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'lhs, 'rhs> ::core::ops::Add<&'rhs mut crate::AddableFreeByMutRef>
    for &'lhs mut crate::AddableFreeByMutRef
{
    type Output = crate::AddableFreeByMutRef;
    #[inline(always)]
    fn add(self, rhs: &'rhs mut crate::AddableFreeByMutRef) -> Self::Output {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<crate::AddableFreeByMutRef>::uninit();
            crate::detail::__rust_thunk___ZplR19AddableFreeByMutRefS0_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::Add<Self> for crate::AddableFreeByValue {
    type Output = crate::AddableFreeByValue;
    #[inline(always)]
    fn add(mut self, mut rhs: Self) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___Zpl18AddableFreeByValueS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                &mut self,
                &mut rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

// error: function `operator+` could not be bound
//   Rvalue reference types are not yet supported as first parameter of operators (b/219826128)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "10Overloaded"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Overloaded
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Overloaded {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Overloaded {}
impl !Sync for Overloaded {}
unsafe impl ::cxx::ExternType for Overloaded {
    type Id = ::cxx::type_id!("Overloaded");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Overloaded {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'lhs> ::core::ops::Add<::ffi_11::c_int> for &'lhs crate::Overloaded {
    type Output = ::ffi_11::c_int;
    #[inline(always)]
    fn add(self, rhs: ::ffi_11::c_int) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK10Overloadedi(self, rhs) }
    }
}

impl<'lhs> ::core::ops::Add<::ffi_11::c_uint> for &'lhs crate::Overloaded {
    type Output = ::ffi_11::c_int;
    #[inline(always)]
    fn add(self, rhs: ::ffi_11::c_uint) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK10Overloadedj(self, rhs) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "15IncompatibleLHS"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=IncompatibleLHS
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct IncompatibleLHS {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for IncompatibleLHS {}
impl !Sync for IncompatibleLHS {}
unsafe impl ::cxx::ExternType for IncompatibleLHS {
    type Id = ::cxx::type_id!("IncompatibleLHS");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for IncompatibleLHS {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: function `operator+` could not be bound
//   Non-record-nor-reference operator parameters are not yet supported, found ::ffi_11::c_int

// error: function `operator+` could not be bound
//   Expected first operator parameter to be a record or incomplete record, found ::ffi_11::c_int

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "18AddableReturnsVoid"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableReturnsVoid
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddableReturnsVoid {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableReturnsVoid {}
impl !Sync for AddableReturnsVoid {}
unsafe impl ::cxx::ExternType for AddableReturnsVoid {
    type Id = ::cxx::type_id!("AddableReturnsVoid");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableReturnsVoid {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Add<&'rhs crate::AddableReturnsVoid>
    for &'__this crate::AddableReturnsVoid
{
    type Output = ();
    #[inline(always)]
    fn add(self, rhs: &'rhs crate::AddableReturnsVoid) {
        unsafe { crate::detail::__rust_thunk___ZNK18AddableReturnsVoidplERKS_(self, rhs) }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "26AddableConstMemberNonunpin"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableConstMemberNonunpin
pub struct AddableConstMemberNonunpin {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for AddableConstMemberNonunpin {}
impl !Sync for AddableConstMemberNonunpin {}
unsafe impl ::cxx::ExternType for AddableConstMemberNonunpin {
    type Id = ::cxx::type_id!("AddableConstMemberNonunpin");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for AddableConstMemberNonunpin {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for AddableConstMemberNonunpin {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_(self, __param_0);
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Add<&'rhs crate::AddableConstMemberNonunpin>
    for &'__this crate::AddableConstMemberNonunpin
{
    type Output = impl ::ctor::Ctor<Output = crate::AddableConstMemberNonunpin, Error = ::ctor::Infallible>
        + use<'__this, 'rhs>;
    #[inline(always)]
    fn add(self, rhs: &'rhs crate::AddableConstMemberNonunpin) -> Self::Output {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut crate::AddableConstMemberNonunpin| {
                crate::detail::__rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    self,
                    rhs,
                );
            })
        }
    }
}

impl ::ctor::PinnedDrop for AddableConstMemberNonunpin {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinD1Ev(self) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "18AddAssignMemberInt"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignMemberInt
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddAssignMemberInt {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignMemberInt {}
impl !Sync for AddAssignMemberInt {}
unsafe impl ::cxx::ExternType for AddAssignMemberInt {
    type Id = ::cxx::type_id!("AddAssignMemberInt");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignMemberInt {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<::ffi_11::c_int> for AddAssignMemberInt {
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, rhs: ::ffi_11::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntpLEi(self, rhs);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "25AddAssignMemberByConstRef"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignMemberByConstRef
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddAssignMemberByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignMemberByConstRef {}
impl !Sync for AddAssignMemberByConstRef {}
unsafe impl ::cxx::ExternType for AddAssignMemberByConstRef {
    type Id = ::cxx::type_id!("AddAssignMemberByConstRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignMemberByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'rhs> ::core::ops::AddAssign<&'rhs Self> for AddAssignMemberByConstRef {
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefpLERKS_(self, rhs);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "23AddAssignFreeByConstRef"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFreeByConstRef
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddAssignFreeByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFreeByConstRef {}
impl !Sync for AddAssignFreeByConstRef {}
unsafe impl ::cxx::ExternType for AddAssignFreeByConstRef {
    type Id = ::cxx::type_id!("AddAssignFreeByConstRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignFreeByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23AddAssignFreeByConstRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'rhs> ::core::ops::AddAssign<&'rhs Self> for crate::AddAssignFreeByConstRef {
    #[inline(always)]
    fn add_assign<'lhs>(&'lhs mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR23AddAssignFreeByConstRefRKS_(self, rhs);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "20AddAssignFreeByValue"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFreeByValue
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddAssignFreeByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFreeByValue {}
impl !Sync for AddAssignFreeByValue {}
unsafe impl ::cxx::ExternType for AddAssignFreeByValue {
    type Id = ::cxx::type_id!("AddAssignFreeByValue");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignFreeByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20AddAssignFreeByValueC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<Self> for crate::AddAssignFreeByValue {
    #[inline(always)]
    fn add_assign<'lhs>(&'lhs mut self, mut rhs: Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR20AddAssignFreeByValueS_(self, &mut rhs);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "25AddAssignFriendByConstRef"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFriendByConstRef
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddAssignFriendByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFriendByConstRef {}
impl !Sync for AddAssignFriendByConstRef {}
unsafe impl ::cxx::ExternType for AddAssignFriendByConstRef {
    type Id = ::cxx::type_id!("AddAssignFriendByConstRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignFriendByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignFriendByConstRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'rhs> ::core::ops::AddAssign<&'rhs Self> for crate::AddAssignFriendByConstRef {
    #[inline(always)]
    fn add_assign<'lhs>(&'lhs mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR25AddAssignFriendByConstRefRKS_(self, rhs);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "22AddAssignFriendByValue"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFriendByValue
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddAssignFriendByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFriendByValue {}
impl !Sync for AddAssignFriendByValue {}
unsafe impl ::cxx::ExternType for AddAssignFriendByValue {
    type Id = ::cxx::type_id!("AddAssignFriendByValue");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignFriendByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddAssignFriendByValueC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<Self> for crate::AddAssignFriendByValue {
    #[inline(always)]
    fn add_assign<'lhs>(&'lhs mut self, mut rhs: Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR22AddAssignFriendByValueS_(self, &mut rhs);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "30AddAssignProhibitedConstMember"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignProhibitedConstMember
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddAssignProhibitedConstMember {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignProhibitedConstMember {}
impl !Sync for AddAssignProhibitedConstMember {}
unsafe impl ::cxx::ExternType for AddAssignProhibitedConstMember {
    type Id = ::cxx::type_id!("AddAssignProhibitedConstMember");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignProhibitedConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nCompound assignment with const left-hand side is not supported, found &'__this crate::AddAssignProhibitedConstMember"
)]
pub trait BindingFailedFor_ZNK30AddAssignProhibitedConstMemberpLEi {}
impl ::core::ops::AddAssign<::ffi_11::c_int> for AddAssignProhibitedConstMember
where
    for<'error> &'error (): BindingFailedFor_ZNK30AddAssignProhibitedConstMemberpLEi,
{
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, rhs: ::ffi_11::c_int) {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "33AddAssignProhibitedFriendConstLhs"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignProhibitedFriendConstLhs
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct AddAssignProhibitedFriendConstLhs {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignProhibitedFriendConstLhs {}
impl !Sync for AddAssignProhibitedFriendConstLhs {}
unsafe impl ::cxx::ExternType for AddAssignProhibitedFriendConstLhs {
    type Id = ::cxx::type_id!("AddAssignProhibitedFriendConstLhs");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignProhibitedFriendConstLhs {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nCompound assignment with const left-hand side is not supported, found &'lhs crate::AddAssignProhibitedFriendConstLhs"
)]
pub trait BindingFailedFor_ZpLRK33AddAssignProhibitedFriendConstLhsi {}
impl ::core::ops::AddAssign<::ffi_11::c_int> for crate::AddAssignProhibitedFriendConstLhs
where
    for<'error> &'error (): BindingFailedFor_ZpLRK33AddAssignProhibitedFriendConstLhsi,
{
    #[inline(always)]
    fn add_assign<'lhs>(&'lhs mut self, rhs: ::ffi_11::c_int) {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "13ManyOperators"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ManyOperators
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct ManyOperators {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for ManyOperators {}
impl !Sync for ManyOperators {}
unsafe impl ::cxx::ExternType for ManyOperators {
    type Id = ::cxx::type_id!("ManyOperators");
    type Kind = ::cxx::kind::Trivial;
}
impl ManyOperators {
    #[inline(always)]
    pub fn unary_plus<'__this>(&'__this self) -> crate::ManyOperators {
        unsafe { self::many_operators::unary_plus(self) }
    }
}

impl Default for ManyOperators {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Neg for &'__this crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsngEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Not for &'__this crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn not(self) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsntEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
            );
            __crubit_return.assume_init()
        }
    }
}

// error: function `ManyOperators::operator~` could not be bound
//   Bindings for this kind of operator (operator ~ with 1 parameter(s)) are not supported

impl<'__this, 'rhs> ::core::ops::Add<&'rhs crate::ManyOperators> for &'__this crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn add(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsplERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Sub<&'rhs crate::ManyOperators> for &'__this crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn sub(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsmiERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Mul<&'rhs crate::ManyOperators> for &'__this crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn mul(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsmlERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Div<&'rhs crate::ManyOperators> for &'__this crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn div(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsdvERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Rem<&'rhs crate::ManyOperators> for &'__this crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn rem(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsrmERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::BitAnd<&'rhs crate::ManyOperators>
    for &'__this crate::ManyOperators
{
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn bitand(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsanERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::BitOr<&'rhs crate::ManyOperators>
    for &'__this crate::ManyOperators
{
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn bitor(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsorERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::BitXor<&'rhs crate::ManyOperators>
    for &'__this crate::ManyOperators
{
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn bitxor(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorseoERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Shl<&'rhs crate::ManyOperators> for &'__this crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn shl(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorslsERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'__this, 'rhs> ::core::ops::Shr<&'rhs crate::ManyOperators> for &'__this crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn shr(self, rhs: &'rhs crate::ManyOperators) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsrsERKS_(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __crubit_return.assume_init()
        }
    }
}

impl<'rhs> ::core::ops::AddAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorspLERKS_(self, rhs);
        }
    }
}

impl<'rhs> ::core::ops::SubAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn sub_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsmIERKS_(self, rhs);
        }
    }
}

impl<'rhs> ::core::ops::MulAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn mul_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsmLERKS_(self, rhs);
        }
    }
}

impl<'rhs> ::core::ops::DivAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn div_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsdVERKS_(self, rhs);
        }
    }
}

impl<'rhs> ::core::ops::RemAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn rem_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsrMERKS_(self, rhs);
        }
    }
}

impl<'rhs> ::core::ops::BitAndAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn bitand_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsaNERKS_(self, rhs);
        }
    }
}

impl<'rhs> ::core::ops::BitOrAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn bitor_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsoRERKS_(self, rhs);
        }
    }
}

impl<'rhs> ::core::ops::BitXorAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn bitxor_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorseOERKS_(self, rhs);
        }
    }
}

impl<'rhs> ::core::ops::ShlAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn shl_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorslSERKS_(self, rhs);
        }
    }
}

impl<'rhs> ::core::ops::ShrAssign<&'rhs Self> for ManyOperators {
    #[inline(always)]
    fn shr_assign<'__this>(&'__this mut self, rhs: &'rhs Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsrSERKS_(self, rhs);
        }
    }
}

pub mod many_operators {
    #[inline(always)]
    pub(crate) fn unary_plus<'__this>(
        __this: &'__this crate::ManyOperators,
    ) -> crate::ManyOperators {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorspsEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
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
        pub(crate) unsafe fn __rust_thunk___ZNK18AddableConstMemberplERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::AddableConstMember,
            rhs: &'rhs crate::AddableConstMember,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableNonConstMemberC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableNonConstMemberplERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this mut crate::AddableNonConstMember,
            rhs: &'rhs crate::AddableNonConstMember,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13AddableFriendC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZplRK13AddableFriendS1_<'lhs, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            lhs: &'lhs crate::AddableFriend,
            rhs: &'rhs crate::AddableFriend,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableFreeByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19AddableFreeByMutRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableFreeByValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22AddableFreeByRValueRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZplRK21AddableFreeByConstRefS1_<'lhs, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            lhs: &'lhs crate::AddableFreeByConstRef,
            rhs: &'rhs crate::AddableFreeByConstRef,
        );
        pub(crate) unsafe fn __rust_thunk___ZplR19AddableFreeByMutRefS0_<'lhs, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            lhs: &'lhs mut crate::AddableFreeByMutRef,
            rhs: &'rhs mut crate::AddableFreeByMutRef,
        );
        pub(crate) unsafe fn __rust_thunk___Zpl18AddableFreeByValueS_(
            __return: *mut ::core::ffi::c_void,
            lhs: &mut crate::AddableFreeByValue,
            rhs: &mut crate::AddableFreeByValue,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10OverloadedC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZplRK10Overloadedi"]
        pub(crate) unsafe fn __rust_thunk___ZplRK10Overloadedi<'lhs>(
            lhs: &'lhs crate::Overloaded,
            rhs: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZplRK10Overloadedj"]
        pub(crate) unsafe fn __rust_thunk___ZplRK10Overloadedj<'lhs>(
            lhs: &'lhs crate::Overloaded,
            rhs: ::ffi_11::c_uint,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN15IncompatibleLHSC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableReturnsVoidC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZNK18AddableReturnsVoidplERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZNK18AddableReturnsVoidplERKS_<'__this, 'rhs>(
            __this: &'__this crate::AddableReturnsVoid,
            rhs: &'rhs crate::AddableReturnsVoid,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::AddableConstMemberNonunpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::AddableConstMemberNonunpin>,
            __param_0: &'__param_0 crate::AddableConstMemberNonunpin,
        ) -> ::core::pin::Pin<&'__this mut crate::AddableConstMemberNonunpin>;
        pub(crate) unsafe fn __rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::AddableConstMemberNonunpin,
            rhs: &'rhs crate::AddableConstMemberNonunpin,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::AddableConstMemberNonunpin>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddAssignMemberIntC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN18AddAssignMemberIntpLEi"]
        pub(crate) unsafe fn __rust_thunk___ZN18AddAssignMemberIntpLEi<'__this>(
            __this: &'__this mut crate::AddAssignMemberInt,
            rhs: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignMemberByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN25AddAssignMemberByConstRefpLERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignMemberByConstRefpLERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::AddAssignMemberByConstRef,
            rhs: &'rhs crate::AddAssignMemberByConstRef,
        ) -> &'__this mut crate::AddAssignMemberByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN23AddAssignFreeByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZpLR23AddAssignFreeByConstRefRKS_"]
        pub(crate) unsafe fn __rust_thunk___ZpLR23AddAssignFreeByConstRefRKS_<'lhs, 'rhs>(
            lhs: &'lhs mut crate::AddAssignFreeByConstRef,
            rhs: &'rhs crate::AddAssignFreeByConstRef,
        ) -> *mut crate::AddAssignFreeByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN20AddAssignFreeByValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZpLR20AddAssignFreeByValueS_<'lhs>(
            lhs: &'lhs mut crate::AddAssignFreeByValue,
            rhs: &mut crate::AddAssignFreeByValue,
        ) -> &'lhs mut crate::AddAssignFreeByValue;
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignFriendByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZpLR25AddAssignFriendByConstRefRKS_"]
        pub(crate) unsafe fn __rust_thunk___ZpLR25AddAssignFriendByConstRefRKS_<'lhs, 'rhs>(
            lhs: &'lhs mut crate::AddAssignFriendByConstRef,
            rhs: &'rhs crate::AddAssignFriendByConstRef,
        ) -> *mut crate::AddAssignFriendByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN22AddAssignFriendByValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZpLR22AddAssignFriendByValueS_<'lhs>(
            lhs: &'lhs mut crate::AddAssignFriendByValue,
            rhs: &mut crate::AddAssignFriendByValue,
        ) -> &'lhs mut crate::AddAssignFriendByValue;
        pub(crate) unsafe fn __rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorspsEv<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsngEv<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsntEv<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsplERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsmiERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsmlERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsdvERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsrmERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsanERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsorERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorseoERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorslsERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsrsERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        );
        #[link_name = "_ZN13ManyOperatorspLERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorspLERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsmIERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsmIERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsmLERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsmLERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsdVERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsdVERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsrMERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsrMERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsaNERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsaNERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsoRERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsoRERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorseOERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorseOERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorslSERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorslSERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsrSERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsrSERKS_<'__this, 'rhs>(
            __this: &'__this mut crate::ManyOperators,
            rhs: &'rhs crate::ManyOperators,
        ) -> &'__this mut crate::ManyOperators;
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
