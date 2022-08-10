// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct Nontrivial {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

impl ::ctor::CtorNew<()> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Ev(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl ::ctor::CtorNew<i32> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        let field = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Ei(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        field,
                    );
                },
            )
        }
    }
}
impl ::ctor::CtorNew<(i32,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<i32>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(i32, i32)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32, i32)) -> Self::CtorType {
        let (field, unused) = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Eii(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        field,
                        unused,
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::Nontrivial> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::Nontrivial) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1ERKS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::Nontrivial,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::Nontrivial,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::Nontrivial>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::Nontrivial>> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::Nontrivial>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::Nontrivial>,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, crate::Nontrivial>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::Nontrivial>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::Nontrivial> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b crate::Nontrivial) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::Nontrivial>> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<i32> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: i32) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEi(self, __param_0);
        }
    }
}

impl ::ctor::Assign<f32> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: f32) {
        unsafe {
            let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialaSEf(
                        ::std::pin::Pin::into_inner_unchecked(dest),
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
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self)
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nontrivial14MemberFunctionEv(self) }
    }
}

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct NontrivialInline {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
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
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
                >| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ev(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl ::ctor::CtorNew<i32> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        let field = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
                >| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ei(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        field,
                    );
                },
            )
        }
    }
}
impl ::ctor::CtorNew<(i32,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<i32>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(i32, i32)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32, i32)) -> Self::CtorType {
        let (field, unused) = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
                >| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1Eii(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        field,
                        unused,
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::NontrivialInline> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::NontrivialInline) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
                >| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1ERKS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::NontrivialInline,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::NontrivialInline,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::NontrivialInline>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::NontrivialInline>>
    for NontrivialInline
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::NontrivialInline>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
                >| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1EOS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::NontrivialInline>,)>
    for NontrivialInline
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, crate::NontrivialInline>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::NontrivialInline>>>::ctor_new(
            arg,
        )
    }
}

impl<'b> ::ctor::Assign<&'b crate::NontrivialInline> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b crate::NontrivialInline) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::NontrivialInline>> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::NontrivialInline>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<i32> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: i32) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEi(self, __param_0);
        }
    }
}

impl ::ctor::PinnedDrop for NontrivialInline {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self)
    }
}

impl NontrivialInline {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::std::pin::Pin<&'a mut Self>) {
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
pub struct NontrivialMembers {
    pub nontrivial_member: ::std::mem::ManuallyDrop<crate::Nontrivial>,
}
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
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialMembers>,
                >| {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersC1Ev(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::NontrivialMembers> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::NontrivialMembers) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialMembers>,
                >| {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersC1ERKS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::NontrivialMembers,)> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::NontrivialMembers,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::NontrivialMembers>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::NontrivialMembers>>
    for NontrivialMembers
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::NontrivialMembers>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialMembers>,
                >| {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersC1EOS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::NontrivialMembers>,)>
    for NontrivialMembers
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, crate::NontrivialMembers>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::NontrivialMembers>>>::ctor_new(
            arg,
        )
    }
}

impl ::ctor::PinnedDrop for NontrivialMembers {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self)
    }
}

impl<'b> ::ctor::Assign<&'b crate::NontrivialMembers> for NontrivialMembers {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b crate::NontrivialMembers) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::NontrivialMembers>>
    for NontrivialMembers
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::NontrivialMembers>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSEOS_(self, __param_0);
        }
    }
}

/// Nontrivial, but trivially relocatable and final (and therefore Unpin).
#[repr(C)]
pub struct NontrivialUnpin {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialUnpin"),
    crate::NontrivialUnpin
);

impl Default for NontrivialUnpin {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=65
// Error while generating bindings for item 'NontrivialUnpin::NontrivialUnpin':
// Not yet supported type of constructor parameter

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=66
// Error while generating bindings for item 'NontrivialUnpin::NontrivialUnpin':
// More than 1 constructor parameter is not supported yet

impl Clone for NontrivialUnpin {
    #[inline(always)]
    fn clone<'b>(&'b self) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1ERKS_(&mut tmp, self);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::NontrivialUnpin>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::NontrivialUnpin>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::Nontrivial>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=70
// Error while generating bindings for item 'NontrivialUnpin::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=71
// Error while generating bindings for item 'NontrivialUnpin::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=72
// Error while generating bindings for item 'NontrivialUnpin::operator=':
// operator= for Unpin types is not yet supported.

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
pub fn TakesByValue(nontrivial: impl ::ctor::Ctor<Output = crate::Nontrivial>) {
    unsafe {
        crate::detail::__rust_thunk___Z12TakesByValue10Nontrivial(
            ::std::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
        )
    }
}

#[inline(always)]
pub fn TakesByValueInline(nontrivial: impl ::ctor::Ctor<Output = crate::NontrivialInline>) {
    unsafe {
        crate::detail::__rust_thunk___Z18TakesByValueInline16NontrivialInline(
            ::std::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
        )
    }
}

#[inline(always)]
pub fn TakesByValueUnpin(nontrivial: crate::NontrivialUnpin) {
    unsafe { crate::detail::__rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(nontrivial) }
}

#[inline(always)]
pub fn ReturnsByValue() -> impl ::ctor::Ctor<Output = crate::Nontrivial> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                crate::detail::__rust_thunk___Z14ReturnsByValuev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

#[inline(always)]
pub fn ReturnsByValueUnpin() -> crate::NontrivialUnpin {
    unsafe { crate::detail::__rust_thunk___Z19ReturnsByValueUnpinv() }
}

#[inline(always)]
pub fn TakesByConstReference<'a>(nontrivial: &'a crate::Nontrivial) -> &'a crate::Nontrivial {
    unsafe { crate::detail::__rust_thunk___Z21TakesByConstReferenceRK10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesByReference<'a>(
    nontrivial: ::std::pin::Pin<&'a mut crate::Nontrivial>,
) -> ::std::pin::Pin<&'a mut crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesByConstReferenceUnpin<'a>(
    nontrivial: &'a crate::NontrivialUnpin,
) -> &'a crate::NontrivialUnpin {
    unsafe {
        crate::detail::__rust_thunk___Z26TakesByConstReferenceUnpinRK15NontrivialUnpin(nontrivial)
    }
}

#[inline(always)]
pub fn TakesByReferenceUnpin<'a>(
    nontrivial: &'a mut crate::NontrivialUnpin,
) -> &'a mut crate::NontrivialUnpin {
    unsafe { crate::detail::__rust_thunk___Z21TakesByReferenceUnpinR15NontrivialUnpin(nontrivial) }
}

/// Finally, testing for strange by-value APIs.
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct NontrivialByValue {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialByValue"),
    crate::NontrivialByValue
);

impl<'b> ::ctor::CtorNew<&'b crate::NontrivialByValue> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::NontrivialByValue) -> Self::CtorType {
        let other = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialByValue>,
                >| {
                    crate::detail::__rust_thunk___ZN17NontrivialByValueC1ERKS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        other,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::NontrivialByValue,)> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::NontrivialByValue,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::NontrivialByValue>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::NontrivialByValue>>
    for NontrivialByValue
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::NontrivialByValue>) -> Self::CtorType {
        let other = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialByValue>,
                >| {
                    crate::detail::__rust_thunk___ZN17NontrivialByValueC1EOS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        other,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::NontrivialByValue>,)>
    for NontrivialByValue
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, crate::NontrivialByValue>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::NontrivialByValue>>>::ctor_new(
            arg,
        )
    }
}

impl<'b> ::ctor::Assign<&'b crate::NontrivialByValue> for NontrivialByValue {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, other: &'b crate::NontrivialByValue) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSERKS_(self, other);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::NontrivialByValue>>
    for NontrivialByValue
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        other: ::ctor::RvalueReference<'b, crate::NontrivialByValue>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSEOS_(self, other);
        }
    }
}

impl<'other> ::ctor::Assign<::ctor::RvalueReference<'other, crate::Nontrivial>>
    for NontrivialByValue
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
    ) {
        unsafe {
            let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialByValue>,
                >| {
                    crate::detail::__rust_thunk___ZN17NontrivialByValueaSE10Nontrivial(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        self,
                        other,
                    );
                }
            ));
        }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=102
// Error while generating bindings for item 'NontrivialByValue::operator==':
// operator== where operands are not const references

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct Nonmovable {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nonmovable"), crate::Nonmovable);

impl ::ctor::CtorNew<()> for Nonmovable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nonmovable>>| {
                    crate::detail::__rust_thunk___ZN10NonmovableC1Ev(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl ::ctor::PinnedDrop for Nonmovable {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NonmovableD1Ev(self)
    }
}

impl Nonmovable {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nonmovable14MemberFunctionEv(self) }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=114
// Error while generating bindings for item 'TakesNonmovableByValue':
// Non-movable, non-trivial_abi type 'crate :: Nonmovable' is not supported by value as parameter #0

#[inline(always)]
pub fn ReturnsNonmovableByValue() -> impl ::ctor::Ctor<Output = crate::Nonmovable> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nonmovable>>| {
                crate::detail::__rust_thunk___Z24ReturnsNonmovableByValuev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialC1Ei"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ei<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            field: i32,
        );
        #[link_name = "_ZN10NontrivialC1Eii"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Eii<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            field: i32,
            unused: i32,
        );
        #[link_name = "_ZN10NontrivialC1ERKS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: &'b crate::Nontrivial,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialaSERKS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: &'b crate::Nontrivial,
        ) -> ::std::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        ) -> ::std::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEi"]
        pub(crate) fn __rust_thunk___ZN10NontrivialaSEi<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: i32,
        ) -> ::std::pin::Pin<&'a mut crate::Nontrivial>;
        pub(crate) fn __rust_thunk___ZN10NontrivialaSEf<'a>(
            __return: &mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: f32,
        );
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZN10Nontrivial14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN10Nontrivial14MemberFunctionEv<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ei<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
            field: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Eii<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
            field: i32,
            unused: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
            __param_0: &'b crate::NontrivialInline,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: &'b crate::NontrivialInline,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialInline>,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineaSEi<'a>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: i32,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInline14MemberFunctionEv<'a>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialMembers>,
            __param_0: &'b crate::NontrivialMembers,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialMembers>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialMembers>,
            __param_0: &'b crate::NontrivialMembers,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialMembers>;
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialMembers>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialMembers>,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialMembers>;
        #[link_name = "_ZN15NontrivialUnpinC1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1ERKS_"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: &'b crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EOS_"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EO10Nontrivial"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN15NontrivialUnpinD1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinD1Ev<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpin14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        pub(crate) fn __rust_thunk___Z12TakesByValue10Nontrivial(
            nontrivial: &mut crate::Nontrivial,
        );
        pub(crate) fn __rust_thunk___Z18TakesByValueInline16NontrivialInline(
            nontrivial: &mut crate::NontrivialInline,
        );
        #[link_name = "_Z17TakesByValueUnpin15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            nontrivial: crate::NontrivialUnpin,
        );
        pub(crate) fn __rust_thunk___Z14ReturnsByValuev(
            __return: &mut ::std::mem::MaybeUninit<crate::Nontrivial>,
        );
        #[link_name = "_Z19ReturnsByValueUnpinv"]
        pub(crate) fn __rust_thunk___Z19ReturnsByValueUnpinv() -> crate::NontrivialUnpin;
        #[link_name = "_Z21TakesByConstReferenceRK10Nontrivial"]
        pub(crate) fn __rust_thunk___Z21TakesByConstReferenceRK10Nontrivial<'a>(
            nontrivial: &'a crate::Nontrivial,
        ) -> &'a crate::Nontrivial;
        #[link_name = "_Z16TakesByReferenceR10Nontrivial"]
        pub(crate) fn __rust_thunk___Z16TakesByReferenceR10Nontrivial<'a>(
            nontrivial: ::std::pin::Pin<&'a mut crate::Nontrivial>,
        ) -> ::std::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_Z26TakesByConstReferenceUnpinRK15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z26TakesByConstReferenceUnpinRK15NontrivialUnpin<'a>(
            nontrivial: &'a crate::NontrivialUnpin,
        ) -> &'a crate::NontrivialUnpin;
        #[link_name = "_Z21TakesByReferenceUnpinR15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z21TakesByReferenceUnpinR15NontrivialUnpin<'a>(
            nontrivial: &'a mut crate::NontrivialUnpin,
        ) -> &'a mut crate::NontrivialUnpin;
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialByValue>,
            other: &'b crate::NontrivialByValue,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialByValue>,
            other: ::ctor::RvalueReference<'b, crate::NontrivialByValue>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialByValue>,
            other: &'b crate::NontrivialByValue,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialByValue>;
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialByValue>,
            other: ::ctor::RvalueReference<'b, crate::NontrivialByValue>,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialByValue>;
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueaSE10Nontrivial<'a, 'other>(
            __return: &mut ::std::mem::MaybeUninit<crate::NontrivialByValue>,
            __this: ::std::pin::Pin<&'a mut crate::NontrivialByValue>,
            other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NonmovableC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NonmovableC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nonmovable>,
        );
        #[link_name = "_ZN10NonmovableD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NonmovableD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nonmovable>,
        );
        #[link_name = "_ZN10Nonmovable14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN10Nonmovable14MemberFunctionEv<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nonmovable>,
        );
        pub(crate) fn __rust_thunk___Z24ReturnsNonmovableByValuev(
            __return: &mut ::std::mem::MaybeUninit<crate::Nonmovable>,
        );
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::Nontrivial>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::Nontrivial>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Nontrivial, field) == 0);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::NontrivialInline>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::NontrivialInline>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialInline: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialInline: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::NontrivialInline, field) == 0);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::NontrivialMembers>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::NontrivialMembers>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialMembers: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialMembers: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::NontrivialMembers, nontrivial_member) == 0);

const _: () = assert!(::std::mem::size_of::<crate::NontrivialUnpin>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::NontrivialUnpin>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialUnpin: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialUnpin: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::NontrivialUnpin, field) == 0);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::NontrivialByValue>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::NontrivialByValue>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::Nonmovable>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::Nonmovable>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nonmovable: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Nonmovable: Drop);
};
