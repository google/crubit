// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unions_cc
// Features: experimental, supported

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

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=10
#[derive(Clone, Copy)]
#[repr(C)]
pub union EmptyUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("EmptyUnion"), crate::EmptyUnion);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=10
impl Default for EmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=10
impl<'b> From<::ctor::RvalueReference<'b, Self>> for EmptyUnion {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=10
impl<'b> ::ctor::UnpinAssign<&'b Self> for EmptyUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=10
impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for EmptyUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=12
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct Nontrivial {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=13
impl ::ctor::CtorNew<()> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=14
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=19
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct TriviallyCopyableButNontriviallyDestructible {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TriviallyCopyableButNontriviallyDestructible"),
    crate::TriviallyCopyableButNontriviallyDestructible
);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=19
impl<'b> ::ctor::Assign<&'b Self> for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_(
                self, __param_0,
            );
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=20
impl<'b> ::ctor::CtorNew<&'b Self> for TriviallyCopyableButNontriviallyDestructible {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_(::core::pin::Pin::into_inner_unchecked(dest),__param_0);
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for TriviallyCopyableButNontriviallyDestructible {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=22
impl ::ctor::PinnedDrop for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(self)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=25
#[derive(Clone, Copy)]
#[repr(C)]
pub union NonEmptyUnion {
    pub bool_field: bool,
    pub char_field: u8,
    pub int_field: i32,
    pub long_long_field: i64,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonEmptyUnion"), crate::NonEmptyUnion);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=25
impl Default for NonEmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=25
impl<'b> From<::ctor::RvalueReference<'b, Self>> for NonEmptyUnion {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=25
impl<'b> ::ctor::UnpinAssign<&'b Self> for NonEmptyUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=25
impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for NonEmptyUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=32
#[::ctor::recursively_pinned]
#[repr(C)]
pub union NonCopyUnion {
    pub trivial_member: bool,
    pub nontrivial_member: ::core::mem::ManuallyDrop<crate::Nontrivial>,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopyUnion"), crate::NonCopyUnion);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=37
#[::ctor::recursively_pinned]
#[repr(C)]
pub union NonCopyUnion2 {
    pub trivial_member: bool,
    pub nontrivial_member:
        ::core::mem::ManuallyDrop<crate::TriviallyCopyableButNontriviallyDestructible>,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopyUnion2"), crate::NonCopyUnion2);

// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=37
// Error while generating bindings for item 'NonCopyUnion2::NonCopyUnion2':
// Can't directly construct values of type `NonCopyUnion2` as it has a non-public or deleted destructor

// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=37
// Error while generating bindings for item 'NonCopyUnion2::NonCopyUnion2':
// Can't directly construct values of type `NonCopyUnion2` as it has a non-public or deleted destructor

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=37
impl<'b> ::ctor::Assign<&'b Self> for NonCopyUnion2 {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonCopyUnion2aSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=37
impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for NonCopyUnion2 {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonCopyUnion2aSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=42
#[derive(Clone, Copy)]
#[repr(C)]
pub union UnionWithOpaqueField {
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'char[42]': Unsupported clang::Type class 'ConstantArray'
    pub(crate) constant_array_field_not_yet_supported: [::core::mem::MaybeUninit<u8>; 42],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UnionWithOpaqueField"),
    crate::UnionWithOpaqueField
);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=42
impl Default for UnionWithOpaqueField {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=42
impl<'b> From<::ctor::RvalueReference<'b, Self>> for UnionWithOpaqueField {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=42
impl<'b> ::ctor::UnpinAssign<&'b Self> for UnionWithOpaqueField {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=42
impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for UnionWithOpaqueField {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=46
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct TrivialButInheritable {
    pub x: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TrivialButInheritable"),
    crate::TrivialButInheritable
);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=46
impl ::ctor::CtorNew<()> for TrivialButInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN21TrivialButInheritableC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=46
impl<'b> ::ctor::CtorNew<&'b Self> for TrivialButInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN21TrivialButInheritableC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for TrivialButInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=46
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for TrivialButInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN21TrivialButInheritableC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for TrivialButInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=46
impl<'b> ::ctor::Assign<&'b Self> for TrivialButInheritable {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN21TrivialButInheritableaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=46
impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for TrivialButInheritable {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN21TrivialButInheritableaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=50
#[::ctor::recursively_pinned]
#[repr(C)]
pub union UnionWithInheritable {
    pub t: ::core::mem::ManuallyDrop<crate::TrivialButInheritable>,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UnionWithInheritable"),
    crate::UnionWithInheritable
);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=50
impl ::ctor::CtorNew<()> for UnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN20UnionWithInheritableC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=50
impl<'b> ::ctor::CtorNew<&'b Self> for UnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN20UnionWithInheritableC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for UnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=50
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for UnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN20UnionWithInheritableC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for UnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=50
impl<'b> ::ctor::Assign<&'b Self> for UnionWithInheritable {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithInheritableaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=50
impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for UnionWithInheritable {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithInheritableaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=54
#[derive(Clone, Copy)]
#[repr(C)]
pub union TypedefUnion {
    pub trivial_member: bool,
}
forward_declare::unsafe_define!(forward_declare::symbol!("TypedefUnion"), crate::TypedefUnion);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=54
impl Default for TypedefUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12TypedefUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=54
impl<'b> From<::ctor::RvalueReference<'b, Self>> for TypedefUnion {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12TypedefUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=54
impl<'b> ::ctor::UnpinAssign<&'b Self> for TypedefUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN12TypedefUnionaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=54
impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for TypedefUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN12TypedefUnionaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=58
#[::ctor::recursively_pinned]
#[repr(C)]
pub union TypedefUnionWithInheritable {
    pub t: ::core::mem::ManuallyDrop<crate::TrivialButInheritable>,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TypedefUnionWithInheritable"),
    crate::TypedefUnionWithInheritable
);

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=58
impl ::ctor::CtorNew<()> for TypedefUnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=58
impl<'b> ::ctor::CtorNew<&'b Self> for TypedefUnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for TypedefUnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=58
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for TypedefUnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for TypedefUnionWithInheritable {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=58
impl<'b> ::ctor::Assign<&'b Self> for TypedefUnionWithInheritable {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/unions.h;l=58
impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for TypedefUnionWithInheritable {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
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
            __this: &'a mut ::core::mem::MaybeUninit<crate::EmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN10EmptyUnionC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::EmptyUnion>,
            __param_0: ::ctor::RvalueReference<'b, crate::EmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN10EmptyUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::EmptyUnion,
            __param_0: &'b crate::EmptyUnion,
        ) -> &'a mut crate::EmptyUnion;
        pub(crate) fn __rust_thunk___ZN10EmptyUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::EmptyUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::EmptyUnion>,
        ) -> &'a mut crate::EmptyUnion;
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_<
            'a,
            'b,
        >(
            __this: ::core::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
            __param_0: &'b crate::TriviallyCopyableButNontriviallyDestructible,
        ) -> ::core::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>;
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_<
            'a,
            'b,
        >(
            __this: &'a mut ::core::mem::MaybeUninit<
                crate::TriviallyCopyableButNontriviallyDestructible,
            >,
            __param_0: &'b crate::TriviallyCopyableButNontriviallyDestructible,
        );
        pub(crate) fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
        );
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NonEmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NonEmptyUnion>,
            __param_0: ::ctor::RvalueReference<'b, crate::NonEmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::NonEmptyUnion,
            __param_0: &'b crate::NonEmptyUnion,
        ) -> &'a mut crate::NonEmptyUnion;
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::NonEmptyUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::NonEmptyUnion>,
        ) -> &'a mut crate::NonEmptyUnion;
        pub(crate) fn __rust_thunk___ZN13NonCopyUnion2aSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NonCopyUnion2>,
            __param_0: &'b crate::NonCopyUnion2,
        ) -> ::core::pin::Pin<&'a mut crate::NonCopyUnion2>;
        pub(crate) fn __rust_thunk___ZN13NonCopyUnion2aSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NonCopyUnion2>,
            __param_0: ::ctor::RvalueReference<'b, crate::NonCopyUnion2>,
        ) -> ::core::pin::Pin<&'a mut crate::NonCopyUnion2>;
        pub(crate) fn __rust_thunk___ZN20UnionWithOpaqueFieldC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::UnionWithOpaqueField>,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithOpaqueFieldC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::UnionWithOpaqueField>,
            __param_0: ::ctor::RvalueReference<'b, crate::UnionWithOpaqueField>,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithOpaqueFieldaSERKS_<'a, 'b>(
            __this: &'a mut crate::UnionWithOpaqueField,
            __param_0: &'b crate::UnionWithOpaqueField,
        ) -> &'a mut crate::UnionWithOpaqueField;
        pub(crate) fn __rust_thunk___ZN20UnionWithOpaqueFieldaSEOS_<'a, 'b>(
            __this: &'a mut crate::UnionWithOpaqueField,
            __param_0: ::ctor::RvalueReference<'b, crate::UnionWithOpaqueField>,
        ) -> &'a mut crate::UnionWithOpaqueField;
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TrivialButInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TrivialButInheritable>,
            __param_0: &'b crate::TrivialButInheritable,
        );
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TrivialButInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::TrivialButInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::TrivialButInheritable>,
            __param_0: &'b crate::TrivialButInheritable,
        ) -> ::core::pin::Pin<&'a mut crate::TrivialButInheritable>;
        pub(crate) fn __rust_thunk___ZN21TrivialButInheritableaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::TrivialButInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::TrivialButInheritable>,
        ) -> ::core::pin::Pin<&'a mut crate::TrivialButInheritable>;
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::UnionWithInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::UnionWithInheritable>,
            __param_0: &'b crate::UnionWithInheritable,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::UnionWithInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::UnionWithInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::UnionWithInheritable>,
            __param_0: &'b crate::UnionWithInheritable,
        ) -> ::core::pin::Pin<&'a mut crate::UnionWithInheritable>;
        pub(crate) fn __rust_thunk___ZN20UnionWithInheritableaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::UnionWithInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::UnionWithInheritable>,
        ) -> ::core::pin::Pin<&'a mut crate::UnionWithInheritable>;
        pub(crate) fn __rust_thunk___ZN12TypedefUnionC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TypedefUnion>,
        );
        pub(crate) fn __rust_thunk___ZN12TypedefUnionC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TypedefUnion>,
            __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnion>,
        );
        pub(crate) fn __rust_thunk___ZN12TypedefUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::TypedefUnion,
            __param_0: &'b crate::TypedefUnion,
        ) -> &'a mut crate::TypedefUnion;
        pub(crate) fn __rust_thunk___ZN12TypedefUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::TypedefUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnion>,
        ) -> &'a mut crate::TypedefUnion;
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TypedefUnionWithInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TypedefUnionWithInheritable>,
            __param_0: &'b crate::TypedefUnionWithInheritable,
        );
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TypedefUnionWithInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>,
        );
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::TypedefUnionWithInheritable>,
            __param_0: &'b crate::TypedefUnionWithInheritable,
        ) -> ::core::pin::Pin<&'a mut crate::TypedefUnionWithInheritable>;
        pub(crate) fn __rust_thunk___ZN27TypedefUnionWithInheritableaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::TypedefUnionWithInheritable>,
            __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>,
        ) -> ::core::pin::Pin<&'a mut crate::TypedefUnionWithInheritable>;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::EmptyUnion>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::EmptyUnion>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::EmptyUnion: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::Nontrivial, field) == 0);

const _: () =
    assert!(::core::mem::size_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () =
    assert!(::core::mem::align_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::TriviallyCopyableButNontriviallyDestructible: Copy
    );
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TriviallyCopyableButNontriviallyDestructible: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::NonEmptyUnion>() == 8);
const _: () = assert!(::core::mem::align_of::<crate::NonEmptyUnion>() == 8);
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

const _: () = assert!(::core::mem::size_of::<crate::NonCopyUnion>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::NonCopyUnion>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};

const _: () = assert!(::core::mem::size_of::<crate::NonCopyUnion2>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::NonCopyUnion2>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion2: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};

const _: () = assert!(::core::mem::size_of::<crate::UnionWithOpaqueField>() == 42);
const _: () = assert!(::core::mem::align_of::<crate::UnionWithOpaqueField>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::UnionWithOpaqueField: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::UnionWithOpaqueField: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::UnionWithOpaqueField: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::TrivialButInheritable>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::TrivialButInheritable>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TrivialButInheritable: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TrivialButInheritable: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::TrivialButInheritable, x) == 0);

const _: () = assert!(::core::mem::size_of::<crate::UnionWithInheritable>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::UnionWithInheritable>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::UnionWithInheritable: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::UnionWithInheritable: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::TypedefUnion>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::TypedefUnion>() == 1);
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

const _: () = assert!(::core::mem::size_of::<crate::TypedefUnionWithInheritable>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::TypedefUnionWithInheritable>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TypedefUnionWithInheritable: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TypedefUnionWithInheritable: Drop);
};
