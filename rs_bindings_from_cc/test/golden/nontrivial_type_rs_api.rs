// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "10Nontrivial"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Nontrivial
pub struct Nontrivial {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub field: ::ffi_11::c_int,
}
impl !Send for Nontrivial {}
impl !Sync for Nontrivial {}
unsafe impl ::cxx::ExternType for Nontrivial {
    type Id = ::cxx::type_id!("Nontrivial");
    type Kind = ::cxx::kind::Opaque;
}
impl Nontrivial {
    #[inline(always)]
    pub fn Unqualified<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { self::nontrivial::Unqualified(self) }
    }
    #[inline(always)]
    pub fn ConstQualified<'__this>(&'__this self) {
        unsafe { self::nontrivial::ConstQualified(self) }
    }
    #[inline(always)]
    pub fn LvalueRefQualified<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { self::nontrivial::LvalueRefQualified(self) }
    }
    #[inline(always)]
    pub fn ConstLvalueRefQualified<'__this>(&'__this self) {
        unsafe { self::nontrivial::ConstLvalueRefQualified(self) }
    }
    #[inline(always)]
    pub fn RvalueRefQualified<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { self::nontrivial::RvalueRefQualified(self) }
    }
    #[inline(always)]
    pub fn ConstRvalueRefQualified<'__this>(&'__this self) {
        unsafe { self::nontrivial::ConstRvalueRefQualified(self) }
    }
}

impl ::ctor::CtorNew<()> for Nontrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_int> for Nontrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut field = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ei(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    field,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for Nontrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_int)> for Nontrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self::CtorType {
        let (mut field, mut unused) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Eii(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    field,
                    unused,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for Nontrivial {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for Nontrivial {
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __param_0: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ffi_11::c_int> for Nontrivial {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: ::ffi_11::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEi(self, __param_0);
        }
    }
}

impl ::ctor::Assign<f32> for Nontrivial {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: f32) {
        unsafe {
            let _ = ::ctor::emplace!(::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialaSEf(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    self,
                    __param_0,
                );
            }));
        }
    }
}

impl ::ctor::PinnedDrop for Nontrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self) }
    }
}

impl PartialEq for Nontrivial {
    #[inline(always)]
    fn eq<'__this, 'rhs>(&'__this self, rhs: &'rhs Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZNK10NontrivialeqERKS_(self, rhs) }
    }
}

impl PartialOrd for Nontrivial {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self == other {
            return Some(core::cmp::Ordering::Equal);
        }
        if self < other {
            return Some(core::cmp::Ordering::Less);
        }
        if other < self {
            return Some(core::cmp::Ordering::Greater);
        }
        None
    }
    #[inline(always)]
    fn lt<'__this, 'rhs>(&'__this self, rhs: &'rhs Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZNK10NontrivialltERKS_(self, rhs) }
    }
}

impl<'__this, 'rhs> ::core::ops::Add<&'rhs crate::Nontrivial> for &'__this crate::Nontrivial {
    type Output = impl ::ctor::Ctor<Output = crate::Nontrivial, Error = ::ctor::Infallible>
        + use<'__this, 'rhs>;
    #[inline(always)]
    fn add(self, rhs: &'rhs crate::Nontrivial) -> Self::Output {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut crate::Nontrivial| {
                crate::detail::__rust_thunk___ZNK10NontrivialplERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    self,
                    rhs,
                );
            })
        }
    }
}

// error: function `Nontrivial::operator+=` could not be bound
//   Compound assignment operators are not supported for non-Unpin types, found ::core::pin::Pin<&'__this mut crate::Nontrivial>

pub mod nontrivial {
    #[inline(always)]
    pub(crate) fn Unqualified<'__this>(__this: ::core::pin::Pin<&'__this mut crate::Nontrivial>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nontrivial11UnqualifiedEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn ConstQualified<'__this>(__this: &'__this crate::Nontrivial) {
        unsafe { crate::detail::__rust_thunk___ZNK10Nontrivial14ConstQualifiedEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn LvalueRefQualified<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
    ) {
        unsafe { crate::detail::__rust_thunk___ZNR10Nontrivial18LvalueRefQualifiedEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn ConstLvalueRefQualified<'__this>(__this: &'__this crate::Nontrivial) {
        unsafe { crate::detail::__rust_thunk___ZNKR10Nontrivial23ConstLvalueRefQualifiedEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn RvalueRefQualified<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
    ) {
        unsafe { crate::detail::__rust_thunk___ZNO10Nontrivial18RvalueRefQualifiedEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn ConstRvalueRefQualified<'__this>(__this: &'__this crate::Nontrivial) {
        unsafe { crate::detail::__rust_thunk___ZNKO10Nontrivial23ConstRvalueRefQualifiedEv(__this) }
    }
}

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "16NontrivialInline"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialInline
pub struct NontrivialInline {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub field: ::ffi_11::c_int,
}
impl !Send for NontrivialInline {}
impl !Sync for NontrivialInline {}
unsafe impl ::cxx::ExternType for NontrivialInline {
    type Id = ::cxx::type_id!("NontrivialInline");
    type Kind = ::cxx::kind::Opaque;
}
impl NontrivialInline {
    #[inline(always)]
    pub fn MemberFunction<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { self::nontrivial_inline::MemberFunction(self) }
    }
}

impl ::ctor::CtorNew<()> for NontrivialInline {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_int> for NontrivialInline {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut field = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ei(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    field,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for NontrivialInline {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_int)> for NontrivialInline {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self::CtorType {
        let (mut field, mut unused) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Eii(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    field,
                    unused,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
    for NontrivialInline
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for NontrivialInline {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for NontrivialInline {
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __param_0: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ffi_11::c_int> for NontrivialInline {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: ::ffi_11::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEi(self, __param_0);
        }
    }
}

impl ::ctor::PinnedDrop for NontrivialInline {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self) }
    }
}

pub mod nontrivial_inline {
    #[inline(always)]
    pub(crate) fn MemberFunction<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::NontrivialInline>,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInline14MemberFunctionEv(__this) }
    }
}

/// Nontrivial due to member variables.
///
/// This changes how the destructor / drop impl work -- instead of calling
/// the destructor for NontrivialMembers, it just calls the destructors for
/// each field.
#[::ctor::recursively_pinned]
#[cfi_encoding = "17NontrivialMembers"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialMembers
pub struct NontrivialMembers {
    pub nontrivial_member: crate::Nontrivial,
}
impl !Send for NontrivialMembers {}
impl !Sync for NontrivialMembers {}
unsafe impl ::cxx::ExternType for NontrivialMembers {
    type Id = ::cxx::type_id!("NontrivialMembers");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for NontrivialMembers {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>
    for NontrivialMembers
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
    for NontrivialMembers
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for NontrivialMembers {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for NontrivialMembers {
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __param_0: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSEOS_(self, __param_0);
        }
    }
}

/// Nontrivial, but trivially relocatable and final (and therefore Unpin).
#[cfi_encoding = "15NontrivialUnpin"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialUnpin
pub struct NontrivialUnpin {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub field: ::ffi_11::c_int,
}
impl !Send for NontrivialUnpin {}
impl !Sync for NontrivialUnpin {}
unsafe impl ::cxx::ExternType for NontrivialUnpin {
    type Id = ::cxx::type_id!("NontrivialUnpin");
    type Kind = ::cxx::kind::Trivial;
}
impl NontrivialUnpin {
    #[inline(always)]
    pub fn MemberFunction<'__this>(&'__this mut self) {
        unsafe { self::nontrivial_unpin::MemberFunction(self) }
    }
}

impl Default for NontrivialUnpin {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl From<::ffi_11::c_int> for NontrivialUnpin {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut field = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ei(&raw mut tmp as *mut _, field);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for NontrivialUnpin {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

impl From<(::ffi_11::c_int, ::ffi_11::c_int)> for NontrivialUnpin {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self {
        let (mut field, mut unused) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Eii(
                &raw mut tmp as *mut _,
                field,
                unused,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_int)> for NontrivialUnpin {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, ::ffi_11::c_int)>>::from(args)
    }
}

impl Clone for NontrivialUnpin {
    #[inline(always)]
    fn clone<'__param_0>(&'__param_0 self) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1ERKS_(&raw mut tmp as *mut _, self);
            tmp.assume_init()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        use ::ctor::UnpinAssign;
        self.unpin_assign(other);
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for NontrivialUnpin {
    #[inline(always)]
    fn from(args: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut __param_0 = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EOS_(
                &raw mut tmp as *mut _,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for NontrivialUnpin {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl From<::ctor::RvalueReference<'_, crate::Nontrivial>> for NontrivialUnpin {
    #[inline(always)]
    fn from(args: ::ctor::RvalueReference<'_, crate::Nontrivial>) -> Self {
        let mut __param_0 = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial(
                &raw mut tmp as *mut _,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, crate::Nontrivial>> for NontrivialUnpin {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, crate::Nontrivial>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, crate::Nontrivial>>>::from(args)
    }
}

impl<'__param_0> ::ctor::UnpinAssign<&'__param_0 Self> for NontrivialUnpin {
    #[inline(always)]
    fn unpin_assign<'__this>(&'__this mut self, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for NontrivialUnpin {
    #[inline(always)]
    fn unpin_assign<'__this>(&'__this mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ffi_11::c_int> for NontrivialUnpin {
    #[inline(always)]
    fn unpin_assign<'__this>(&'__this mut self, __param_0: ::ffi_11::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinaSEi(self, __param_0);
        }
    }
}

impl Drop for NontrivialUnpin {
    #[inline(always)]
    fn drop<'__this>(&'__this mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpinD1Ev(self) }
    }
}

pub mod nontrivial_unpin {
    #[inline(always)]
    pub(crate) fn MemberFunction<'__this>(__this: &'__this mut crate::NontrivialUnpin) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv(__this) }
    }
}

#[inline(always)]
pub fn TakesByValue(
    nontrivial: ::ctor::Ctor![crate::Nontrivial],
) -> ::ctor::Ctor![crate::Nontrivial] {
    unsafe {
        ::ctor::FnCtor::new(move |__crubit_dest: *mut crate::Nontrivial| {
            crate::detail::__rust_thunk___Z12TakesByValue10Nontrivial(
                __crubit_dest as *mut ::core::ffi::c_void,
                ::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
            );
        })
    }
}

#[inline(always)]
pub fn TakesByValueInline(
    nontrivial: ::ctor::Ctor![crate::NontrivialInline],
) -> ::ctor::Ctor![crate::NontrivialInline] {
    unsafe {
        ::ctor::FnCtor::new(move |__crubit_dest: *mut crate::NontrivialInline| {
            crate::detail::__rust_thunk___Z18TakesByValueInline16NontrivialInline(
                __crubit_dest as *mut ::core::ffi::c_void,
                ::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
            );
        })
    }
}

#[inline(always)]
pub fn TakesByValueUnpin(mut nontrivial: crate::NontrivialUnpin) -> crate::NontrivialUnpin {
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<crate::NontrivialUnpin>::uninit();
        crate::detail::__rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            &mut nontrivial,
        );
        __crubit_return.assume_init()
    }
}

#[inline(always)]
pub fn TakesByReference<'nontrivial>(
    nontrivial: ::core::pin::Pin<&'nontrivial mut crate::Nontrivial>,
) -> ::cref::CMut<'nontrivial, crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesUnpinByReference<'nontrivial>(
    nontrivial: &'nontrivial mut crate::NontrivialUnpin,
) -> ::cref::CMut<'nontrivial, crate::NontrivialUnpin> {
    unsafe { crate::detail::__rust_thunk___Z21TakesUnpinByReferenceR15NontrivialUnpin(nontrivial) }
}

#[inline(always)]
pub fn TakesByConstReference<'nontrivial>(
    nontrivial: &'nontrivial crate::Nontrivial,
) -> ::cref::CRef<'nontrivial, crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z21TakesByConstReferenceRK10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesUnpinByConstReference<'nontrivial>(
    nontrivial: &'nontrivial crate::NontrivialUnpin,
) -> ::cref::CRef<'nontrivial, crate::NontrivialUnpin> {
    unsafe {
        crate::detail::__rust_thunk___Z26TakesUnpinByConstReferenceRK15NontrivialUnpin(nontrivial)
    }
}

#[inline(always)]
pub fn TakesByRvalueReference(
    nontrivial: ::ctor::RvalueReference<'_, crate::Nontrivial>,
) -> ::ctor::RvalueReference<'_, crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z22TakesByRvalueReferenceO10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesUnpinByRvalueReference(
    nontrivial: ::ctor::RvalueReference<'_, crate::NontrivialUnpin>,
) -> ::ctor::RvalueReference<'_, crate::NontrivialUnpin> {
    unsafe {
        crate::detail::__rust_thunk___Z27TakesUnpinByRvalueReferenceO15NontrivialUnpin(nontrivial)
    }
}

#[inline(always)]
pub fn TakesByConstRvalueReference(
    nontrivial: ::ctor::ConstRvalueReference<'_, crate::Nontrivial>,
) -> ::ctor::ConstRvalueReference<'_, crate::Nontrivial> {
    unsafe {
        crate::detail::__rust_thunk___Z27TakesByConstRvalueReferenceOK10Nontrivial(nontrivial)
    }
}

#[inline(always)]
pub fn TakesUnpinByConstRvalueReference(
    nontrivial: ::ctor::ConstRvalueReference<'_, crate::NontrivialUnpin>,
) -> ::ctor::ConstRvalueReference<'_, crate::NontrivialUnpin> {
    unsafe {
        crate::detail::__rust_thunk___Z32TakesUnpinByConstRvalueReferenceOK15NontrivialUnpin(
            nontrivial,
        )
    }
}

/// Finally, testing for strange by-value APIs.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "17NontrivialByValue"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialByValue
pub struct NontrivialByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NontrivialByValue {}
impl !Sync for NontrivialByValue {}
unsafe impl ::cxx::ExternType for NontrivialByValue {
    type Id = ::cxx::type_id!("NontrivialByValue");
    type Kind = ::cxx::kind::Trivial;
}

impl<'other> ::ctor::UnpinAssign<::ctor::RvalueReference<'other, crate::Nontrivial>>
    for NontrivialByValue
{
    #[inline(always)]
    fn unpin_assign<'__this>(
        &'__this mut self,
        other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
    ) {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSE10Nontrivial(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                other,
            );
            __crubit_return.assume_init();
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nExpected first operator== param reference to be immutable, but found mutable reference: &'__this mut crate::NontrivialByValue\ncomparison operator return type must be `bool`, found: crate::NontrivialByValue"
)]
pub trait BindingFailedFor_ZN17NontrivialByValueeqES_ {}
impl PartialEq for NontrivialByValue
where
    for<'error> &'error (): BindingFailedFor_ZN17NontrivialByValueeqES_,
{
    #[inline(always)]
    fn eq<'__this>(&'__this self, other: &Self) -> bool {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "10Nonmovable"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Nonmovable
pub struct Nonmovable {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for Nonmovable {}
impl !Sync for Nonmovable {}
unsafe impl ::cxx::ExternType for Nonmovable {
    type Id = ::cxx::type_id!("Nonmovable");
    type Kind = ::cxx::kind::Opaque;
}
impl Nonmovable {
    #[inline(always)]
    pub fn MemberFunction<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { self::nonmovable::MemberFunction(self) }
    }
}

impl ::ctor::CtorNew<()> for Nonmovable {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NonmovableC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::PinnedDrop for Nonmovable {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10NonmovableD1Ev(self) }
    }
}

pub mod nonmovable {
    #[inline(always)]
    pub(crate) fn MemberFunction<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::Nonmovable>,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN10Nonmovable14MemberFunctionEv(__this) }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nNon-movable, non-trivial_abi type 'crate::Nonmovable' is not supported by value as parameter #0"
)]
pub trait BindingFailedFor_Z22TakesNonmovableByValue10Nonmovable {}
#[inline(always)]
pub fn TakesNonmovableByValue(nonmovable: ::ctor::Ctor![crate::Nonmovable])
where
    for<'error> &'error (): BindingFailedFor_Z22TakesNonmovableByValue10Nonmovable,
{
    #![allow(unused_variables)]
    unreachable!(
        "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
    )
}

#[inline(always)]
pub fn ReturnsNonmovableByValue() -> ::ctor::Ctor![crate::Nonmovable] {
    unsafe {
        ::ctor::FnCtor::new(move |__crubit_dest: *mut crate::Nonmovable| {
            crate::detail::__rust_thunk___Z24ReturnsNonmovableByValuev(
                __crubit_dest as *mut ::core::ffi::c_void,
            );
        })
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN10NontrivialC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
        );
        #[link_name = "_ZN10NontrivialC1Eii"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Eii(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
            unused: ::ffi_11::c_int,
        );
        #[link_name = "_ZN10NontrivialC1ERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::Nontrivial,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialaSERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
            __param_0: &'__param_0 crate::Nontrivial,
        ) -> ::core::pin::Pin<&'__this mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSEOS_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'_, crate::Nontrivial>,
        ) -> ::core::pin::Pin<&'__this mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEi"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSEi<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
            __param_0: ::ffi_11::c_int,
        ) -> ::core::pin::Pin<&'__this mut crate::Nontrivial>;
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSEf<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
            __param_0: f32,
        );
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
        );
        #[link_name = "_ZN10Nontrivial11UnqualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZN10Nontrivial11UnqualifiedEv<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
        );
        #[link_name = "_ZNK10Nontrivial14ConstQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK10Nontrivial14ConstQualifiedEv<'__this>(
            __this: &'__this crate::Nontrivial,
        );
        #[link_name = "_ZNR10Nontrivial18LvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNR10Nontrivial18LvalueRefQualifiedEv<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
        );
        #[link_name = "_ZNKR10Nontrivial23ConstLvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKR10Nontrivial23ConstLvalueRefQualifiedEv<'__this>(
            __this: &'__this crate::Nontrivial,
        );
        #[link_name = "_ZNO10Nontrivial18RvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNO10Nontrivial18RvalueRefQualifiedEv<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Nontrivial>,
        );
        #[link_name = "_ZNKO10Nontrivial23ConstRvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKO10Nontrivial23ConstRvalueRefQualifiedEv<'__this>(
            __this: &'__this crate::Nontrivial,
        );
        #[link_name = "_ZNK10NontrivialeqERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZNK10NontrivialeqERKS_<'__this, 'rhs>(
            __this: &'__this crate::Nontrivial,
            rhs: &'rhs crate::Nontrivial,
        ) -> bool;
        #[link_name = "_ZNK10NontrivialltERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZNK10NontrivialltERKS_<'__this, 'rhs>(
            __this: &'__this crate::Nontrivial,
            rhs: &'rhs crate::Nontrivial,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZNK10NontrivialplERKS_<'__this, 'rhs>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::Nontrivial,
            rhs: &'rhs crate::Nontrivial,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1Eii(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
            unused: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::NontrivialInline,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::NontrivialInline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NontrivialInline>,
            __param_0: &'__param_0 crate::NontrivialInline,
        ) -> ::core::pin::Pin<&'__this mut crate::NontrivialInline>;
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineaSEOS_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NontrivialInline>,
            __param_0: ::ctor::RvalueReference<'_, crate::NontrivialInline>,
        ) -> ::core::pin::Pin<&'__this mut crate::NontrivialInline>;
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineaSEi<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NontrivialInline>,
            __param_0: ::ffi_11::c_int,
        ) -> ::core::pin::Pin<&'__this mut crate::NontrivialInline>;
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NontrivialInline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInline14MemberFunctionEv<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NontrivialInline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::NontrivialMembers,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::NontrivialMembers>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NontrivialMembers>,
            __param_0: &'__param_0 crate::NontrivialMembers,
        ) -> ::core::pin::Pin<&'__this mut crate::NontrivialMembers>;
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersaSEOS_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NontrivialMembers>,
            __param_0: ::ctor::RvalueReference<'_, crate::NontrivialMembers>,
        ) -> ::core::pin::Pin<&'__this mut crate::NontrivialMembers>;
        #[link_name = "_ZN15NontrivialUnpinC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN15NontrivialUnpinC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
        );
        #[link_name = "_ZN15NontrivialUnpinC1Eii"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1Eii(
            __this: *mut ::core::ffi::c_void,
            field: ::ffi_11::c_int,
            unused: ::ffi_11::c_int,
        );
        #[link_name = "_ZN15NontrivialUnpinC1ERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EO10Nontrivial"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::Nontrivial>,
        );
        #[link_name = "_ZN15NontrivialUnpinaSERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinaSERKS_<'__param_0, '__this>(
            __this: &'__this mut crate::NontrivialUnpin,
            __param_0: &'__param_0 crate::NontrivialUnpin,
        ) -> &'__this mut crate::NontrivialUnpin;
        #[link_name = "_ZN15NontrivialUnpinaSEOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinaSEOS_<'__this>(
            __this: &'__this mut crate::NontrivialUnpin,
            __param_0: ::ctor::RvalueReference<'_, crate::NontrivialUnpin>,
        ) -> &'__this mut crate::NontrivialUnpin;
        #[link_name = "_ZN15NontrivialUnpinaSEi"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinaSEi<'__this>(
            __this: &'__this mut crate::NontrivialUnpin,
            __param_0: ::ffi_11::c_int,
        ) -> &'__this mut crate::NontrivialUnpin;
        #[link_name = "_ZN15NontrivialUnpinD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinD1Ev<'__this>(
            __this: &'__this mut crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpin14MemberFunctionEv"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv<'__this>(
            __this: &'__this mut crate::NontrivialUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___Z12TakesByValue10Nontrivial(
            __return: *mut ::core::ffi::c_void,
            nontrivial: &mut crate::Nontrivial,
        );
        pub(crate) unsafe fn __rust_thunk___Z18TakesByValueInline16NontrivialInline(
            __return: *mut ::core::ffi::c_void,
            nontrivial: &mut crate::NontrivialInline,
        );
        pub(crate) unsafe fn __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            __return: *mut ::core::ffi::c_void,
            nontrivial: &mut crate::NontrivialUnpin,
        );
        #[link_name = "_Z16TakesByReferenceR10Nontrivial"]
        pub(crate) unsafe fn __rust_thunk___Z16TakesByReferenceR10Nontrivial<'nontrivial>(
            nontrivial: ::core::pin::Pin<&'nontrivial mut crate::Nontrivial>,
        ) -> ::cref::CMut<'nontrivial, crate::Nontrivial>;
        #[link_name = "_Z21TakesUnpinByReferenceR15NontrivialUnpin"]
        pub(crate) unsafe fn __rust_thunk___Z21TakesUnpinByReferenceR15NontrivialUnpin<
            'nontrivial,
        >(
            nontrivial: &'nontrivial mut crate::NontrivialUnpin,
        ) -> ::cref::CMut<'nontrivial, crate::NontrivialUnpin>;
        #[link_name = "_Z21TakesByConstReferenceRK10Nontrivial"]
        pub(crate) unsafe fn __rust_thunk___Z21TakesByConstReferenceRK10Nontrivial<'nontrivial>(
            nontrivial: &'nontrivial crate::Nontrivial,
        ) -> ::cref::CRef<'nontrivial, crate::Nontrivial>;
        #[link_name = "_Z26TakesUnpinByConstReferenceRK15NontrivialUnpin"]
        pub(crate) unsafe fn __rust_thunk___Z26TakesUnpinByConstReferenceRK15NontrivialUnpin<
            'nontrivial,
        >(
            nontrivial: &'nontrivial crate::NontrivialUnpin,
        ) -> ::cref::CRef<'nontrivial, crate::NontrivialUnpin>;
        #[link_name = "_Z22TakesByRvalueReferenceO10Nontrivial"]
        pub(crate) unsafe fn __rust_thunk___Z22TakesByRvalueReferenceO10Nontrivial(
            nontrivial: ::ctor::RvalueReference<'_, crate::Nontrivial>,
        ) -> ::ctor::RvalueReference<'_, crate::Nontrivial>;
        #[link_name = "_Z27TakesUnpinByRvalueReferenceO15NontrivialUnpin"]
        pub(crate) unsafe fn __rust_thunk___Z27TakesUnpinByRvalueReferenceO15NontrivialUnpin(
            nontrivial: ::ctor::RvalueReference<'_, crate::NontrivialUnpin>,
        ) -> ::ctor::RvalueReference<'_, crate::NontrivialUnpin>;
        #[link_name = "_Z27TakesByConstRvalueReferenceOK10Nontrivial"]
        pub(crate) unsafe fn __rust_thunk___Z27TakesByConstRvalueReferenceOK10Nontrivial(
            nontrivial: ::ctor::ConstRvalueReference<'_, crate::Nontrivial>,
        ) -> ::ctor::ConstRvalueReference<'_, crate::Nontrivial>;
        #[link_name = "_Z32TakesUnpinByConstRvalueReferenceOK15NontrivialUnpin"]
        pub(crate) unsafe fn __rust_thunk___Z32TakesUnpinByConstRvalueReferenceOK15NontrivialUnpin(
            nontrivial: ::ctor::ConstRvalueReference<'_, crate::NontrivialUnpin>,
        ) -> ::ctor::ConstRvalueReference<'_, crate::NontrivialUnpin>;
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialByValueaSE10Nontrivial<'__this, 'other>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this mut crate::NontrivialByValue,
            other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NonmovableC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NonmovableC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN10NonmovableD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NonmovableD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Nonmovable>,
        );
        #[link_name = "_ZN10Nonmovable14MemberFunctionEv"]
        pub(crate) unsafe fn __rust_thunk___ZN10Nonmovable14MemberFunctionEv<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Nonmovable>,
        );
        pub(crate) unsafe fn __rust_thunk___Z24ReturnsNonmovableByValuev(
            __return: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
    assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
    assert!(::core::mem::offset_of!(crate::Nontrivial, field) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NontrivialInline>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialInline>() == 4);
    static_assertions::assert_impl_all!(crate::NontrivialInline: Drop);
    static_assertions::assert_not_impl_any!(crate::NontrivialInline: Copy);
    assert!(::core::mem::offset_of!(crate::NontrivialInline, field) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NontrivialMembers>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialMembers>() == 4);
    static_assertions::assert_not_impl_any!(crate::NontrivialMembers: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::NontrivialMembers, nontrivial_member) == 0);
    assert!(::core::mem::size_of::<crate::NontrivialUnpin>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialUnpin>() == 4);
    static_assertions::assert_impl_all!(crate::NontrivialUnpin: Drop);
    static_assertions::assert_not_impl_any!(crate::NontrivialUnpin: Copy);
    assert!(::core::mem::offset_of!(crate::NontrivialUnpin, field) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NontrivialByValue>() == 1);
    assert!(::core::mem::align_of::<crate::NontrivialByValue>() == 1);
    static_assertions::assert_impl_all!(crate::NontrivialByValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Drop);

    assert!(::core::mem::size_of::<crate::Nonmovable>() == 1);
    assert!(::core::mem::align_of::<crate::Nonmovable>() == 1);
    static_assertions::assert_impl_all!(crate::Nonmovable: Drop);
    static_assertions::assert_not_impl_any!(crate::Nonmovable: Copy);
};
