// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc

#![rustfmt::skip]
#![feature(
    allocator_api,
    arbitrary_self_types,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls,
    register_tool
)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "Nontrivial")]
pub struct Nontrivial {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::core::ffi::c_int,
}
impl !Send for Nontrivial {}
impl !Sync for Nontrivial {}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

impl ::ctor::CtorNew<()> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                    );
                },
            )
        }
    }
}

impl ::ctor::CtorNew<::core::ffi::c_int> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ::core::ffi::c_int) -> Self::CtorType {
        let field = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Ei(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        field,
                    );
                },
            )
        }
    }
}
impl ::ctor::CtorNew<(::core::ffi::c_int,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (::core::ffi::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::core::ffi::c_int>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(::core::ffi::c_int, ::core::ffi::c_int)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (::core::ffi::c_int, ::core::ffi::c_int)) -> Self::CtorType {
        let (field, unused) = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Eii(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        field,
                        unused,
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::core::ffi::c_int> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: ::core::ffi::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEi(self, __param_0);
        }
    }
}

impl ::ctor::Assign<f32> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: f32) {
        unsafe {
            let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialaSEf(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        self,
                        __param_0,
                    );
                }
            ));
        }
    }
}

impl ::ctor::PinnedDrop for Nontrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self)
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn Unqualified<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nontrivial11UnqualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn ConstQualified<'a>(&'a self) {
        unsafe { crate::detail::__rust_thunk___ZNK10Nontrivial14ConstQualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn LvalueRefQualified<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZNR10Nontrivial18LvalueRefQualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn ConstLvalueRefQualified<'a>(&'a self) {
        unsafe { crate::detail::__rust_thunk___ZNKR10Nontrivial23ConstLvalueRefQualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn RvalueRefQualified<'a>(self: ::ctor::RvalueReference<'a, Self>) {
        unsafe { crate::detail::__rust_thunk___ZNO10Nontrivial18RvalueRefQualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn ConstRvalueRefQualified<'a>(self: ::ctor::ConstRvalueReference<'a, Self>) {
        unsafe { crate::detail::__rust_thunk___ZNKO10Nontrivial23ConstRvalueRefQualifiedEv(self) }
    }
}

impl PartialEq for Nontrivial {
    #[inline(always)]
    fn eq<'a, 'b>(&'a self, rhs: &'b Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZNK10NontrivialeqERKS_(self, rhs) }
    }
}

// Error while generating bindings for item 'Nontrivial::operator!=':
// Bindings for this kind of operator (operator != with 2 parameter(s)) are not supported

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
    fn lt<'a, 'b>(&'a self, rhs: &'b Self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZNK10NontrivialltERKS_(self, rhs) }
    }
}

impl<'a, 'b> ::core::ops::Add<&'b crate::Nontrivial> for &'a crate::Nontrivial {
    type Output = impl ::ctor::Ctor<Output = crate::Nontrivial> + use<'a, 'b>;
    #[inline(always)]
    fn add(self, rhs: &'b crate::Nontrivial) -> Self::Output {
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<crate::Nontrivial>>| {
                    crate::detail::__rust_thunk___ZNK10NontrivialplERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        self,
                        rhs,
                    );
                },
            )
        }
    }
}

// Error while generating bindings for item 'Nontrivial::operator+=':
// Compound assignment operators are not supported for non-Unpin types, found ::core::pin::Pin<&'a mut crate::Nontrivial>

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "NontrivialInline")]
pub struct NontrivialInline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::core::ffi::c_int,
}
impl !Send for NontrivialInline {}
impl !Sync for NontrivialInline {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialInline"),
    crate::NontrivialInline
);

impl ::ctor::CtorNew<()> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                    );
                },
            )
        }
    }
}

impl ::ctor::CtorNew<::core::ffi::c_int> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ::core::ffi::c_int) -> Self::CtorType {
        let field = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ei(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        field,
                    );
                },
            )
        }
    }
}
impl ::ctor::CtorNew<(::core::ffi::c_int,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (::core::ffi::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::core::ffi::c_int>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(::core::ffi::c_int, ::core::ffi::c_int)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (::core::ffi::c_int, ::core::ffi::c_int)) -> Self::CtorType {
        let (field, unused) = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1Eii(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        field,
                        unused,
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::core::ffi::c_int> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: ::core::ffi::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEi(self, __param_0);
        }
    }
}

impl ::ctor::PinnedDrop for NontrivialInline {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self)
    }
}

impl NontrivialInline {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInline14MemberFunctionEv(self) }
    }
}

/// Nontrivial due to member variables.
///
/// This changes how the destructor / drop impl work -- instead of calling
/// the destructor for NontrivialMembers, it just calls the destructors for
/// each field.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "NontrivialMembers")]
pub struct NontrivialMembers {
    pub nontrivial_member: ::core::mem::ManuallyDrop<crate::Nontrivial>,
}
impl !Send for NontrivialMembers {}
impl !Sync for NontrivialMembers {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialMembers"),
    crate::NontrivialMembers
);

impl ::ctor::CtorNew<()> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for NontrivialMembers {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for NontrivialMembers {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for NontrivialMembers {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSEOS_(self, __param_0);
        }
    }
}

/// Nontrivial, but trivially relocatable and final (and therefore Unpin).
#[repr(C)]
#[__crubit::annotate(cpp_type = "NontrivialUnpin")]
pub struct NontrivialUnpin {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::core::ffi::c_int,
}
impl !Send for NontrivialUnpin {}
impl !Sync for NontrivialUnpin {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialUnpin"),
    crate::NontrivialUnpin
);

impl Default for NontrivialUnpin {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::core::ffi::c_int> for NontrivialUnpin {
    #[inline(always)]
    fn from(field: ::core::ffi::c_int) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ei(
                &raw mut tmp as *mut ::core::ffi::c_void,
                field,
            );
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for item 'NontrivialUnpin::NontrivialUnpin':
// Constructors with more than one parameter are not yet supported. See b/216648347.

impl Clone for NontrivialUnpin {
    #[inline(always)]
    fn clone<'b>(&'b self) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1ERKS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                self,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::Nontrivial>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for NontrivialUnpin {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for NontrivialUnpin {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::core::ffi::c_int> for NontrivialUnpin {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::core::ffi::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinaSEi(self, __param_0);
        }
    }
}

impl Drop for NontrivialUnpin {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpinD1Ev(self) }
    }
}

impl NontrivialUnpin {
    #[inline(always)]
    pub fn MemberFunction<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv(self) }
    }
}

#[inline(always)]
pub fn TakesByValue(
    nontrivial: impl ::ctor::Ctor<Output = crate::Nontrivial>,
) -> impl ::ctor::Ctor<Output = crate::Nontrivial> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<crate::Nontrivial>>| {
                crate::detail::__rust_thunk___Z12TakesByValue10Nontrivial(
                    ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                        as *mut ::core::ffi::c_void,
                    ::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
                );
            },
        )
    }
}

#[inline(always)]
pub fn TakesByValueInline(
    nontrivial: impl ::ctor::Ctor<Output = crate::NontrivialInline>,
) -> impl ::ctor::Ctor<Output = crate::NontrivialInline> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::core::pin::Pin<
                &mut ::core::mem::MaybeUninit<crate::NontrivialInline>,
            >| {
                crate::detail::__rust_thunk___Z18TakesByValueInline16NontrivialInline(
                    ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                        as *mut ::core::ffi::c_void,
                    ::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
                );
            },
        )
    }
}

#[inline(always)]
pub fn TakesByValueUnpin(mut nontrivial: crate::NontrivialUnpin) -> crate::NontrivialUnpin {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::NontrivialUnpin>::uninit();
        crate::detail::__rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            &raw mut __return as *mut ::core::ffi::c_void,
            &mut nontrivial,
        );
        __return.assume_init()
    }
}

#[inline(always)]
pub fn TakesByReference<'a>(
    nontrivial: ::core::pin::Pin<&'a mut crate::Nontrivial>,
) -> ::core::pin::Pin<&'a mut crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesUnpinByReference<'a>(
    nontrivial: &'a mut crate::NontrivialUnpin,
) -> &'a mut crate::NontrivialUnpin {
    unsafe { crate::detail::__rust_thunk___Z21TakesUnpinByReferenceR15NontrivialUnpin(nontrivial) }
}

#[inline(always)]
pub fn TakesByConstReference<'a>(nontrivial: &'a crate::Nontrivial) -> &'a crate::Nontrivial {
    unsafe { crate::detail::__rust_thunk___Z21TakesByConstReferenceRK10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesUnpinByConstReference<'a>(
    nontrivial: &'a crate::NontrivialUnpin,
) -> &'a crate::NontrivialUnpin {
    unsafe {
        crate::detail::__rust_thunk___Z26TakesUnpinByConstReferenceRK15NontrivialUnpin(nontrivial)
    }
}

#[inline(always)]
pub fn TakesByRvalueReference<'a>(
    nontrivial: ::ctor::RvalueReference<'a, crate::Nontrivial>,
) -> ::ctor::RvalueReference<'a, crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z22TakesByRvalueReferenceO10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesUnpinByRvalueReference<'a>(
    nontrivial: ::ctor::RvalueReference<'a, crate::NontrivialUnpin>,
) -> ::ctor::RvalueReference<'a, crate::NontrivialUnpin> {
    unsafe {
        crate::detail::__rust_thunk___Z27TakesUnpinByRvalueReferenceO15NontrivialUnpin(nontrivial)
    }
}

#[inline(always)]
pub fn TakesByConstRvalueReference<'a>(
    nontrivial: ::ctor::ConstRvalueReference<'a, crate::Nontrivial>,
) -> ::ctor::ConstRvalueReference<'a, crate::Nontrivial> {
    unsafe {
        crate::detail::__rust_thunk___Z27TakesByConstRvalueReferenceOK10Nontrivial(nontrivial)
    }
}

#[inline(always)]
pub fn TakesUnpinByConstRvalueReference<'a>(
    nontrivial: ::ctor::ConstRvalueReference<'a, crate::NontrivialUnpin>,
) -> ::ctor::ConstRvalueReference<'a, crate::NontrivialUnpin> {
    unsafe {
        crate::detail::__rust_thunk___Z32TakesUnpinByConstRvalueReferenceOK15NontrivialUnpin(
            nontrivial,
        )
    }
}

/// Finally, testing for strange by-value APIs.
#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "NontrivialByValue")]
pub struct NontrivialByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NontrivialByValue {}
impl !Sync for NontrivialByValue {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialByValue"),
    crate::NontrivialByValue
);

impl<'b> From<::ctor::RvalueReference<'b, Self>> for NontrivialByValue {
    #[inline(always)]
    fn from(other: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialByValueC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                other,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for NontrivialByValue {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, other: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSERKS_(self, other);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for NontrivialByValue {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, other: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSEOS_(self, other);
        }
    }
}

impl<'other> ::ctor::UnpinAssign<::ctor::RvalueReference<'other, crate::Nontrivial>>
    for NontrivialByValue
{
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, other: ::ctor::RvalueReference<'other, crate::Nontrivial>) {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSE10Nontrivial(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                other,
            );
            __return.assume_init();
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding genertion for function failed\nExpected first operator== param reference to be immutable, but found mutable reference: &'a mut crate::NontrivialByValue\ncomparison operator return type must be `bool`, found: crate::NontrivialByValue"
)]
pub trait BindingFailedFor_ZN17NontrivialByValueeqES_ {}
impl<'error> PartialEq for NontrivialByValue
where
    &'error (): BindingFailedFor_ZN17NontrivialByValueeqES_,
{
    #[inline(always)]
    fn eq<'a>(&'a self, other: &Self) -> bool {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "Nonmovable")]
pub struct Nonmovable {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Nonmovable {}
impl !Sync for Nonmovable {}
forward_declare::unsafe_define!(forward_declare::symbol!("Nonmovable"), crate::Nonmovable);

impl ::ctor::CtorNew<()> for Nonmovable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NonmovableC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                            as *mut ::core::ffi::c_void,
                    );
                },
            )
        }
    }
}

impl ::ctor::PinnedDrop for Nonmovable {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NonmovableD1Ev(self)
    }
}

impl Nonmovable {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nonmovable14MemberFunctionEv(self) }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding genertion for function failed\nNon-movable, non-trivial_abi type 'crate::Nonmovable' is not supported by value as parameter #0"
)]
pub trait BindingFailedFor_Z22TakesNonmovableByValue10Nonmovable {}
#[inline(always)]
pub fn TakesNonmovableByValue<'error>(nonmovable: impl ::ctor::Ctor<Output = crate::Nonmovable>)
where
    &'error (): BindingFailedFor_Z22TakesNonmovableByValue10Nonmovable,
{
    #![allow(unused_variables)]
    unreachable!(
        "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
    )
}

#[inline(always)]
pub fn ReturnsNonmovableByValue() -> impl ::ctor::Ctor<Output = crate::Nonmovable> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<crate::Nonmovable>>| {
                crate::detail::__rust_thunk___Z24ReturnsNonmovableByValuev(
                    ::core::pin::Pin::into_inner_unchecked(dest) as *mut _
                        as *mut ::core::ffi::c_void,
                );
            },
        )
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
            field: ::core::ffi::c_int,
        );
        #[link_name = "_ZN10NontrivialC1Eii"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Eii(
            __this: *mut ::core::ffi::c_void,
            field: ::core::ffi::c_int,
            unused: ::core::ffi::c_int,
        );
        #[link_name = "_ZN10NontrivialC1ERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1ERKS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'b crate::Nontrivial,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialaSERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: &'b crate::Nontrivial,
        ) -> ::core::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        ) -> ::core::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEi"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSEi<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: ::core::ffi::c_int,
        ) -> ::core::pin::Pin<&'a mut crate::Nontrivial>;
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSEf<'a>(
            __return: *mut ::core::ffi::c_void,
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: f32,
        );
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZN10Nontrivial11UnqualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZN10Nontrivial11UnqualifiedEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZNK10Nontrivial14ConstQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK10Nontrivial14ConstQualifiedEv<'a>(
            __this: &'a crate::Nontrivial,
        );
        #[link_name = "_ZNR10Nontrivial18LvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNR10Nontrivial18LvalueRefQualifiedEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZNKR10Nontrivial23ConstLvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKR10Nontrivial23ConstLvalueRefQualifiedEv<'a>(
            __this: &'a crate::Nontrivial,
        );
        #[link_name = "_ZNO10Nontrivial18RvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNO10Nontrivial18RvalueRefQualifiedEv<'a>(
            __this: ::ctor::RvalueReference<'a, crate::Nontrivial>,
        );
        #[link_name = "_ZNKO10Nontrivial23ConstRvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKO10Nontrivial23ConstRvalueRefQualifiedEv<'a>(
            __this: ::ctor::ConstRvalueReference<'a, crate::Nontrivial>,
        );
        #[link_name = "_ZNK10NontrivialeqERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZNK10NontrivialeqERKS_<'a, 'b>(
            __this: &'a crate::Nontrivial,
            rhs: &'b crate::Nontrivial,
        ) -> bool;
        #[link_name = "_ZNK10NontrivialltERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZNK10NontrivialltERKS_<'a, 'b>(
            __this: &'a crate::Nontrivial,
            rhs: &'b crate::Nontrivial,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZNK10NontrivialplERKS_<'a, 'b>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::Nontrivial,
            rhs: &'b crate::Nontrivial,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::core::ffi::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1Eii(
            __this: *mut ::core::ffi::c_void,
            field: ::core::ffi::c_int,
            unused: ::core::ffi::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1ERKS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'b crate::NontrivialInline,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialInline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: &'b crate::NontrivialInline,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialInline>,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineaSEi<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: ::core::ffi::c_int,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInline14MemberFunctionEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersC1ERKS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'b crate::NontrivialMembers,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialMembers>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialMembers>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialMembers>,
            __param_0: &'b crate::NontrivialMembers,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialMembers>;
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialMembers>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialMembers>,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialMembers>;
        #[link_name = "_ZN15NontrivialUnpinC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN15NontrivialUnpinC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1Ei(
            __this: *mut ::core::ffi::c_void,
            field: ::core::ffi::c_int,
        );
        #[link_name = "_ZN15NontrivialUnpinC1ERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1ERKS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'b crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EO10Nontrivial"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN15NontrivialUnpinaSERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinaSERKS_<'a, 'b>(
            __this: &'a mut crate::NontrivialUnpin,
            __param_0: &'b crate::NontrivialUnpin,
        ) -> &'a mut crate::NontrivialUnpin;
        #[link_name = "_ZN15NontrivialUnpinaSEOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinaSEOS_<'a, 'b>(
            __this: &'a mut crate::NontrivialUnpin,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialUnpin>,
        ) -> &'a mut crate::NontrivialUnpin;
        #[link_name = "_ZN15NontrivialUnpinaSEi"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinaSEi<'a>(
            __this: &'a mut crate::NontrivialUnpin,
            __param_0: ::core::ffi::c_int,
        ) -> &'a mut crate::NontrivialUnpin;
        #[link_name = "_ZN15NontrivialUnpinD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinD1Ev<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpin14MemberFunctionEv"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv<'a>(
            __this: &'a mut crate::NontrivialUnpin,
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
        pub(crate) unsafe fn __rust_thunk___Z16TakesByReferenceR10Nontrivial<'a>(
            nontrivial: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        ) -> ::core::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_Z21TakesUnpinByReferenceR15NontrivialUnpin"]
        pub(crate) unsafe fn __rust_thunk___Z21TakesUnpinByReferenceR15NontrivialUnpin<'a>(
            nontrivial: &'a mut crate::NontrivialUnpin,
        ) -> &'a mut crate::NontrivialUnpin;
        #[link_name = "_Z21TakesByConstReferenceRK10Nontrivial"]
        pub(crate) unsafe fn __rust_thunk___Z21TakesByConstReferenceRK10Nontrivial<'a>(
            nontrivial: &'a crate::Nontrivial,
        ) -> &'a crate::Nontrivial;
        #[link_name = "_Z26TakesUnpinByConstReferenceRK15NontrivialUnpin"]
        pub(crate) unsafe fn __rust_thunk___Z26TakesUnpinByConstReferenceRK15NontrivialUnpin<'a>(
            nontrivial: &'a crate::NontrivialUnpin,
        ) -> &'a crate::NontrivialUnpin;
        #[link_name = "_Z22TakesByRvalueReferenceO10Nontrivial"]
        pub(crate) unsafe fn __rust_thunk___Z22TakesByRvalueReferenceO10Nontrivial<'a>(
            nontrivial: ::ctor::RvalueReference<'a, crate::Nontrivial>,
        ) -> ::ctor::RvalueReference<'a, crate::Nontrivial>;
        #[link_name = "_Z27TakesUnpinByRvalueReferenceO15NontrivialUnpin"]
        pub(crate) unsafe fn __rust_thunk___Z27TakesUnpinByRvalueReferenceO15NontrivialUnpin<'a>(
            nontrivial: ::ctor::RvalueReference<'a, crate::NontrivialUnpin>,
        ) -> ::ctor::RvalueReference<'a, crate::NontrivialUnpin>;
        #[link_name = "_Z27TakesByConstRvalueReferenceOK10Nontrivial"]
        pub(crate) unsafe fn __rust_thunk___Z27TakesByConstRvalueReferenceOK10Nontrivial<'a>(
            nontrivial: ::ctor::ConstRvalueReference<'a, crate::Nontrivial>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::Nontrivial>;
        #[link_name = "_Z32TakesUnpinByConstRvalueReferenceOK15NontrivialUnpin"]
        pub(crate) unsafe fn __rust_thunk___Z32TakesUnpinByConstRvalueReferenceOK15NontrivialUnpin<
            'a,
        >(
            nontrivial: ::ctor::ConstRvalueReference<'a, crate::NontrivialUnpin>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::NontrivialUnpin>;
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialByValueC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            other: ::ctor::RvalueReference<'b, crate::NontrivialByValue>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialByValueaSERKS_<'a, 'b>(
            __this: &'a mut crate::NontrivialByValue,
            other: &'b crate::NontrivialByValue,
        ) -> &'a mut crate::NontrivialByValue;
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialByValueaSEOS_<'a, 'b>(
            __this: &'a mut crate::NontrivialByValue,
            other: ::ctor::RvalueReference<'b, crate::NontrivialByValue>,
        ) -> &'a mut crate::NontrivialByValue;
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialByValueaSE10Nontrivial<'a, 'other>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a mut crate::NontrivialByValue,
            other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NonmovableC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NonmovableC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN10NonmovableD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NonmovableD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nonmovable>,
        );
        #[link_name = "_ZN10Nonmovable14MemberFunctionEv"]
        pub(crate) unsafe fn __rust_thunk___ZN10Nonmovable14MemberFunctionEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nonmovable>,
        );
        pub(crate) unsafe fn __rust_thunk___Z24ReturnsNonmovableByValuev(
            __return: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
    assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
    assert!(::core::mem::offset_of!(crate::Nontrivial, field) == 0);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);

    assert!(::core::mem::size_of::<crate::NontrivialInline>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialInline>() == 4);
    static_assertions::assert_not_impl_any!(crate::NontrivialInline: Copy);
    static_assertions::assert_impl_all!(crate::NontrivialInline: Drop);
    assert!(::core::mem::offset_of!(crate::NontrivialInline, field) == 0);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);

    assert!(::core::mem::size_of::<crate::NontrivialMembers>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialMembers>() == 4);
    static_assertions::assert_not_impl_any!(crate::NontrivialMembers: Copy);
    static_assertions::assert_impl_all!(crate::NontrivialMembers: Drop);
    assert!(::core::mem::offset_of!(crate::NontrivialMembers, nontrivial_member) == 0);

    assert!(::core::mem::size_of::<crate::NontrivialUnpin>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialUnpin>() == 4);
    static_assertions::assert_not_impl_any!(crate::NontrivialUnpin: Copy);
    static_assertions::assert_impl_all!(crate::NontrivialUnpin: Drop);
    assert!(::core::mem::offset_of!(crate::NontrivialUnpin, field) == 0);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);

    assert!(::core::mem::size_of::<crate::NontrivialByValue>() == 1);
    assert!(::core::mem::align_of::<crate::NontrivialByValue>() == 1);
    static_assertions::assert_impl_all!(crate::NontrivialByValue: Clone);
    static_assertions::assert_impl_all!(crate::NontrivialByValue: Copy);
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Drop);

    assert!(::core::mem::size_of::<crate::Nonmovable>() == 1);
    assert!(::core::mem::align_of::<crate::Nonmovable>() == 1);
    static_assertions::assert_not_impl_any!(crate::Nonmovable: Copy);
    static_assertions::assert_impl_all!(crate::Nonmovable: Drop);
};
