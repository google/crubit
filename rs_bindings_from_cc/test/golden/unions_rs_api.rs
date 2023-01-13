// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unions_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub union EmptyUnion {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("EmptyUnion"), crate::EmptyUnion);

impl Default for EmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::EmptyUnion>> for EmptyUnion {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::EmptyUnion>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unions.h;l=10
// Error while generating bindings for item 'EmptyUnion::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/unions.h;l=10
// Error while generating bindings for item 'EmptyUnion::operator=':
// operator= for Unpin types is not yet supported.

#[::ctor::recursively_pinned]
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
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            })
        }
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::Nontrivial>> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::Nontrivial>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
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

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct TriviallyCopyableButNontriviallyDestructible {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TriviallyCopyableButNontriviallyDestructible"),
    crate::TriviallyCopyableButNontriviallyDestructible
);

impl<'b> ::ctor::Assign<&'b Self> for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::TriviallyCopyableButNontriviallyDestructible>
    for TriviallyCopyableButNontriviallyDestructible
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::TriviallyCopyableButNontriviallyDestructible) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_(::std::pin::Pin::into_inner_unchecked(dest),__param_0);
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::TriviallyCopyableButNontriviallyDestructible,)>
    for TriviallyCopyableButNontriviallyDestructible
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: (&'b crate::TriviallyCopyableButNontriviallyDestructible,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::TriviallyCopyableButNontriviallyDestructible>>::ctor_new(
            arg,
        )
    }
}

impl ::ctor::PinnedDrop for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(self)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union NonEmptyUnion {
    pub bool_field: bool,
    pub char_field: u8,
    pub int_field: i32,
    pub long_long_field: i64,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonEmptyUnion"), crate::NonEmptyUnion);

impl Default for NonEmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::NonEmptyUnion>> for NonEmptyUnion {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::NonEmptyUnion>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unions.h;l=25
// Error while generating bindings for item 'NonEmptyUnion::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/unions.h;l=25
// Error while generating bindings for item 'NonEmptyUnion::operator=':
// operator= for Unpin types is not yet supported.

#[::ctor::recursively_pinned]
#[repr(C)]
pub union NonCopyUnion {
    pub trivial_member: bool,
    pub nontrivial_member: ::std::mem::ManuallyDrop<crate::Nontrivial>,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopyUnion"), crate::NonCopyUnion);

#[::ctor::recursively_pinned]
#[repr(C)]
pub union NonCopyUnion2 {
    pub trivial_member: bool,
    pub nontrivial_member:
        ::std::mem::ManuallyDrop<crate::TriviallyCopyableButNontriviallyDestructible>,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopyUnion2"), crate::NonCopyUnion2);

// rs_bindings_from_cc/test/golden/unions.h;l=37
// Error while generating bindings for item 'NonCopyUnion2::NonCopyUnion2':
// Can't directly construct values of type `NonCopyUnion2` as it has a non-public or deleted destructor

// rs_bindings_from_cc/test/golden/unions.h;l=37
// Error while generating bindings for item 'NonCopyUnion2::NonCopyUnion2':
// Can't directly construct values of type `NonCopyUnion2` as it has a non-public or deleted destructor

impl<'b> ::ctor::Assign<&'b Self> for NonCopyUnion2 {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonCopyUnion2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::NonCopyUnion2>> for NonCopyUnion2 {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::NonCopyUnion2>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonCopyUnion2aSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union UnionWithOpaqueField {
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'char[42]': Unsupported clang::Type class 'ConstantArray'
    pub(crate) constant_array_field_not_yet_supported: [::std::mem::MaybeUninit<u8>; 42],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UnionWithOpaqueField"),
    crate::UnionWithOpaqueField
);

impl Default for UnionWithOpaqueField {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::UnionWithOpaqueField>> for UnionWithOpaqueField {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::UnionWithOpaqueField>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unions.h;l=42
// Error while generating bindings for item 'UnionWithOpaqueField::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/unions.h;l=42
// Error while generating bindings for item 'UnionWithOpaqueField::operator=':
// operator= for Unpin types is not yet supported.

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct TrivialButInheritable {
    pub x: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TrivialButInheritable"),
    crate::TrivialButInheritable
);

impl ::ctor::CtorNew<()> for TrivialButInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN21TrivialButInheritableC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            })
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::TrivialButInheritable> for TrivialButInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::TrivialButInheritable) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN21TrivialButInheritableC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::TrivialButInheritable,)> for TrivialButInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::TrivialButInheritable,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::TrivialButInheritable>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::TrivialButInheritable>>
    for TrivialButInheritable
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::TrivialButInheritable>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN21TrivialButInheritableC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::TrivialButInheritable>,)>
    for TrivialButInheritable
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: (::ctor::RvalueReference<'b, crate::TrivialButInheritable>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as::ctor::CtorNew<::ctor::RvalueReference<'b,crate::TrivialButInheritable>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for TrivialButInheritable {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN21TrivialButInheritableaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::TrivialButInheritable>>
    for TrivialButInheritable
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::TrivialButInheritable>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN21TrivialButInheritableaSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[repr(C)]
pub union UnionWithInheritable {
    pub t: ::std::mem::ManuallyDrop<crate::TrivialButInheritable>,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UnionWithInheritable"),
    crate::UnionWithInheritable
);

impl ::ctor::CtorNew<()> for UnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN20UnionWithInheritableC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            })
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::UnionWithInheritable> for UnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::UnionWithInheritable) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN20UnionWithInheritableC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::UnionWithInheritable,)> for UnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::UnionWithInheritable,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::UnionWithInheritable>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::UnionWithInheritable>>
    for UnionWithInheritable
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::UnionWithInheritable>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN20UnionWithInheritableC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::UnionWithInheritable>,)>
    for UnionWithInheritable
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: (::ctor::RvalueReference<'b, crate::UnionWithInheritable>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as::ctor::CtorNew<::ctor::RvalueReference<'b,crate::UnionWithInheritable>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for UnionWithInheritable {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithInheritableaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::UnionWithInheritable>>
    for UnionWithInheritable
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::UnionWithInheritable>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithInheritableaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union TypedefUnion {
    pub trivial_member: bool,
}
forward_declare::unsafe_define!(forward_declare::symbol!("TypedefUnion"), crate::TypedefUnion);

impl Default for TypedefUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12TypedefUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::TypedefUnion>> for TypedefUnion {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::TypedefUnion>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12TypedefUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unions.h;l=54
// Error while generating bindings for item 'TypedefUnion::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/unions.h;l=54
// Error while generating bindings for item 'TypedefUnion::operator=':
// operator= for Unpin types is not yet supported.

#[::ctor::recursively_pinned]
#[repr(C)]
pub union TypedefUnionWithInheritable {
    pub t: ::std::mem::ManuallyDrop<crate::TrivialButInheritable>,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TypedefUnionWithInheritable"),
    crate::TypedefUnionWithInheritable
);

impl ::ctor::CtorNew<()> for TypedefUnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            })
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::TypedefUnionWithInheritable> for TypedefUnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::TypedefUnionWithInheritable) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::TypedefUnionWithInheritable,)>
    for TypedefUnionWithInheritable
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::TypedefUnionWithInheritable,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::TypedefUnionWithInheritable>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>>
    for TypedefUnionWithInheritable
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: ::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>,
    ) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>,)>
    for TypedefUnionWithInheritable
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: (::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as::ctor::CtorNew<::ctor::RvalueReference<'b,crate::TypedefUnionWithInheritable>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for TypedefUnionWithInheritable {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>>
    for TypedefUnionWithInheritable
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableaSEOS_(self, __param_0);
        }
    }
}

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNIONS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10EmptyUnionC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::EmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN10EmptyUnionC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::EmptyUnion>,
            __param_0: ::ctor::RvalueReference<'b, crate::EmptyUnion>,
        );
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_<
            'a,
            'b,
        >(
            __this: ::std::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
            __param_0: &'b crate::TriviallyCopyableButNontriviallyDestructible,
        ) -> ::std::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>;
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_<
            'a,
            'b,
        >(
            __this: &'a mut ::std::mem::MaybeUninit<
                crate::TriviallyCopyableButNontriviallyDestructible,
            >,
            __param_0: &'b crate::TriviallyCopyableButNontriviallyDestructible,
        );
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
        );
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NonEmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NonEmptyUnion>,
            __param_0: ::ctor::RvalueReference<'b, crate::NonEmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN13NonCopyUnion2aSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NonCopyUnion2>,
            __param_0: &'b crate::NonCopyUnion2,
        ) -> ::std::pin::Pin<&'a mut crate::NonCopyUnion2>;
        pub(crate) fn __rust_thunk___ZN13NonCopyUnion2aSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NonCopyUnion2>,
            __param_0: ::ctor::RvalueReference<'b, crate::NonCopyUnion2>,
        ) -> ::std::pin::Pin<&'a mut crate::NonCopyUnion2>;
        pub(crate) fn __rust_thunk___ZN20UnionWithOpaqueFieldC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::UnionWithOpaqueField>,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithOpaqueFieldC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::UnionWithOpaqueField>,
            __param_0: ::ctor::RvalueReference<'b, crate::UnionWithOpaqueField>,
        );
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TrivialButInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TrivialButInheritable>,
            __param_0: &'b crate::TrivialButInheritable,
        );
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TrivialButInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::TrivialButInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::TrivialButInheritable>,
            __param_0: &'b crate::TrivialButInheritable,
        ) -> ::std::pin::Pin<&'a mut crate::TrivialButInheritable>;
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::TrivialButInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::TrivialButInheritable>,
        ) -> ::std::pin::Pin<&'a mut crate::TrivialButInheritable>;
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::UnionWithInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::UnionWithInheritable>,
            __param_0: &'b crate::UnionWithInheritable,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::UnionWithInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::UnionWithInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::UnionWithInheritable>,
            __param_0: &'b crate::UnionWithInheritable,
        ) -> ::std::pin::Pin<&'a mut crate::UnionWithInheritable>;
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::UnionWithInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::UnionWithInheritable>,
        ) -> ::std::pin::Pin<&'a mut crate::UnionWithInheritable>;
        pub(crate) fn __rust_thunk___ZN12TypedefUnionC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TypedefUnion>,
        );
        pub(crate) fn __rust_thunk___ZN12TypedefUnionC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TypedefUnion>,
            __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnion>,
        );
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TypedefUnionWithInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TypedefUnionWithInheritable>,
            __param_0: &'b crate::TypedefUnionWithInheritable,
        );
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TypedefUnionWithInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::TypedefUnionWithInheritable>,
            __param_0: &'b crate::TypedefUnionWithInheritable,
        ) -> ::std::pin::Pin<&'a mut crate::TypedefUnionWithInheritable>;
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::TypedefUnionWithInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>,
        ) -> ::std::pin::Pin<&'a mut crate::TypedefUnionWithInheritable>;
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::EmptyUnion>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::EmptyUnion>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::EmptyUnion: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::Nontrivial>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::Nontrivial>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::Nontrivial, field) == 0);

const _: () =
    assert!(::std::mem::size_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () =
    assert!(::std::mem::align_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::TriviallyCopyableButNontriviallyDestructible: Copy
    );
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TriviallyCopyableButNontriviallyDestructible: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::NonEmptyUnion>() == 8);
const _: () = assert!(::std::mem::align_of::<crate::NonEmptyUnion>() == 8);
const _: () = {
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NonEmptyUnion: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(u8: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i64: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::NonCopyUnion>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::NonCopyUnion>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::NonCopyUnion2>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::NonCopyUnion2>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion2: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::UnionWithOpaqueField>() == 42);
const _: () = assert!(::std::mem::align_of::<crate::UnionWithOpaqueField>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::UnionWithOpaqueField: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::UnionWithOpaqueField: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::UnionWithOpaqueField: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::TrivialButInheritable>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::TrivialButInheritable>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TrivialButInheritable: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TrivialButInheritable: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::TrivialButInheritable, x) == 0);

const _: () = assert!(::std::mem::size_of::<crate::UnionWithInheritable>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::UnionWithInheritable>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::UnionWithInheritable: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::UnionWithInheritable: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::TypedefUnion>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::TypedefUnion>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::TypedefUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TypedefUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TypedefUnion: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::TypedefUnionWithInheritable>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::TypedefUnionWithInheritable>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TypedefUnionWithInheritable: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TypedefUnionWithInheritable: Drop);
};
