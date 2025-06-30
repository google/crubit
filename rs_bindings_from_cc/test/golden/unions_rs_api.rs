// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unions_cc

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

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=EmptyUnion
pub union EmptyUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for EmptyUnion {}
impl !Sync for EmptyUnion {}
unsafe impl ::cxx::ExternType for EmptyUnion {
    type Id = ::cxx::type_id!("EmptyUnion");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("EmptyUnion"), crate::EmptyUnion);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN10EmptyUnionC1Ev {}
impl<'error> Default for EmptyUnion
where
    &'error (): BindingFailedFor_ZN10EmptyUnionC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for item 'EmptyUnion::EmptyUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN10EmptyUnionC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for EmptyUnion
where
    &'error (): BindingFailedFor_ZN10EmptyUnionC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for EmptyUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for EmptyUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionaSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Nontrivial
pub struct Nontrivial {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::core::ffi::c_int,
}
impl !Send for Nontrivial {}
impl !Sync for Nontrivial {}
unsafe impl ::cxx::ExternType for Nontrivial {
    type Id = ::cxx::type_id!("Nontrivial");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

impl ::ctor::CtorNew<()> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ev(dest as *mut ::core::ffi::c_void);
            })
        }
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=UnionToRename
pub union RenamedUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for RenamedUnion {}
impl !Sync for RenamedUnion {}
unsafe impl ::cxx::ExternType for RenamedUnion {
    type Id = ::cxx::type_id!("UnionToRename");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("UnionToRename"), crate::RenamedUnion);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN13UnionToRenameC1Ev {}
impl<'error> Default for RenamedUnion
where
    &'error (): BindingFailedFor_ZN13UnionToRenameC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for item 'UnionToRename::UnionToRename':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN13UnionToRenameC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for RenamedUnion
where
    &'error (): BindingFailedFor_ZN13UnionToRenameC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for RenamedUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13UnionToRenameaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for RenamedUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13UnionToRenameaSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TriviallyCopyableButNontriviallyDestructible
pub struct TriviallyCopyableButNontriviallyDestructible {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TriviallyCopyableButNontriviallyDestructible {}
impl !Sync for TriviallyCopyableButNontriviallyDestructible {}
unsafe impl ::cxx::ExternType for TriviallyCopyableButNontriviallyDestructible {
    type Id = ::cxx::type_id!("TriviallyCopyableButNontriviallyDestructible");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TriviallyCopyableButNontriviallyDestructible"),
    crate::TriviallyCopyableButNontriviallyDestructible
);

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

impl<'b> ::ctor::CtorNew<&'b Self> for TriviallyCopyableButNontriviallyDestructible {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_(dest as*mut::core::ffi::c_void,__param_0);
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for TriviallyCopyableButNontriviallyDestructible {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'b>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(self)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NonEmptyUnion
pub union NonEmptyUnion {
    pub bool_field: bool,
    pub char_field: ::core::ffi::c_char,
    pub int_field: ::core::ffi::c_int,
    pub long_long_field: ::core::ffi::c_longlong,
}
impl !Send for NonEmptyUnion {}
impl !Sync for NonEmptyUnion {}
unsafe impl ::cxx::ExternType for NonEmptyUnion {
    type Id = ::cxx::type_id!("NonEmptyUnion");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonEmptyUnion"), crate::NonEmptyUnion);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN13NonEmptyUnionC1Ev {}
impl<'error> Default for NonEmptyUnion
where
    &'error (): BindingFailedFor_ZN13NonEmptyUnionC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for item 'NonEmptyUnion::NonEmptyUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN13NonEmptyUnionC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for NonEmptyUnion
where
    &'error (): BindingFailedFor_ZN13NonEmptyUnionC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for NonEmptyUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for NonEmptyUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionaSEOS_(self, __param_0);
        }
    }
}

#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NonCopyUnion
pub union NonCopyUnion {
    pub trivial_member: bool,
    pub nontrivial_member: ::core::mem::ManuallyDrop<crate::Nontrivial>,
}
impl !Send for NonCopyUnion {}
impl !Sync for NonCopyUnion {}
unsafe impl ::cxx::ExternType for NonCopyUnion {
    type Id = ::cxx::type_id!("NonCopyUnion");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopyUnion"), crate::NonCopyUnion);

#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NonCopyUnion2
pub union NonCopyUnion2 {
    pub trivial_member: bool,
    pub nontrivial_member:
        ::core::mem::ManuallyDrop<crate::TriviallyCopyableButNontriviallyDestructible>,
}
impl !Send for NonCopyUnion2 {}
impl !Sync for NonCopyUnion2 {}
unsafe impl ::cxx::ExternType for NonCopyUnion2 {
    type Id = ::cxx::type_id!("NonCopyUnion2");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopyUnion2"), crate::NonCopyUnion2);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.\nCan't directly construct values of type `NonCopyUnion2` as it has a non-public or deleted destructor"
)]
pub trait BindingFailedFor_ZN13NonCopyUnion2C1ERKS_ {}
impl<'error> Clone for NonCopyUnion2
where
    &'error (): BindingFailedFor_ZN13NonCopyUnion2C1ERKS_,
{
    #[inline(always)]
    fn clone<'b>(&'b self) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.\nCan't directly construct values of type `NonCopyUnion2` as it has a non-public or deleted destructor"
)]
pub trait BindingFailedFor_ZN13NonCopyUnion2C1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for NonCopyUnion2
where
    &'error (): BindingFailedFor_ZN13NonCopyUnion2C1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for NonCopyUnion2 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonCopyUnion2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for NonCopyUnion2 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13NonCopyUnion2aSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=UnionWithOpaqueField
pub union UnionWithOpaqueField {
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'char[42]': Unsupported clang::Type class 'ConstantArray'
    pub(crate) constant_array_field_not_yet_supported: [::core::mem::MaybeUninit<u8>; 42],
}
impl !Send for UnionWithOpaqueField {}
impl !Sync for UnionWithOpaqueField {}
unsafe impl ::cxx::ExternType for UnionWithOpaqueField {
    type Id = ::cxx::type_id!("UnionWithOpaqueField");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UnionWithOpaqueField"),
    crate::UnionWithOpaqueField
);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN20UnionWithOpaqueFieldC1Ev {}
impl<'error> Default for UnionWithOpaqueField
where
    &'error (): BindingFailedFor_ZN20UnionWithOpaqueFieldC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for item 'UnionWithOpaqueField::UnionWithOpaqueField':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN20UnionWithOpaqueFieldC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for UnionWithOpaqueField
where
    &'error (): BindingFailedFor_ZN20UnionWithOpaqueFieldC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for UnionWithOpaqueField {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for UnionWithOpaqueField {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TrivialButInheritable
pub struct TrivialButInheritable {
    pub x: ::core::ffi::c_int,
}
impl !Send for TrivialButInheritable {}
impl !Sync for TrivialButInheritable {}
unsafe impl ::cxx::ExternType for TrivialButInheritable {
    type Id = ::cxx::type_id!("TrivialButInheritable");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TrivialButInheritable"),
    crate::TrivialButInheritable
);

impl Default for TrivialButInheritable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21TrivialButInheritableC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for TrivialButInheritable {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21TrivialButInheritableC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for TrivialButInheritable {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for TrivialButInheritable {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN21TrivialButInheritableaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for TrivialButInheritable {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN21TrivialButInheritableaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=UnionWithInheritable
pub union UnionWithInheritable {
    pub t: crate::TrivialButInheritable,
}
impl !Send for UnionWithInheritable {}
impl !Sync for UnionWithInheritable {}
unsafe impl ::cxx::ExternType for UnionWithInheritable {
    type Id = ::cxx::type_id!("UnionWithInheritable");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("UnionWithInheritable"),
    crate::UnionWithInheritable
);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN20UnionWithInheritableC1Ev {}
impl<'error> Default for UnionWithInheritable
where
    &'error (): BindingFailedFor_ZN20UnionWithInheritableC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for item 'UnionWithInheritable::UnionWithInheritable':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN20UnionWithInheritableC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for UnionWithInheritable
where
    &'error (): BindingFailedFor_ZN20UnionWithInheritableC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for UnionWithInheritable {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithInheritableaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for UnionWithInheritable {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithInheritableaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypedefUnion
pub union TypedefUnion {
    pub trivial_member: bool,
}
impl !Send for TypedefUnion {}
impl !Sync for TypedefUnion {}
unsafe impl ::cxx::ExternType for TypedefUnion {
    type Id = ::cxx::type_id!("TypedefUnion");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("TypedefUnion"), crate::TypedefUnion);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN12TypedefUnionC1Ev {}
impl<'error> Default for TypedefUnion
where
    &'error (): BindingFailedFor_ZN12TypedefUnionC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for item 'TypedefUnion::TypedefUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN12TypedefUnionC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for TypedefUnion
where
    &'error (): BindingFailedFor_ZN12TypedefUnionC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for TypedefUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN12TypedefUnionaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for TypedefUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN12TypedefUnionaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypedefUnionWithInheritable
pub union TypedefUnionWithInheritable {
    pub t: crate::TrivialButInheritable,
}
impl !Send for TypedefUnionWithInheritable {}
impl !Sync for TypedefUnionWithInheritable {}
unsafe impl ::cxx::ExternType for TypedefUnionWithInheritable {
    type Id = ::cxx::type_id!("TypedefUnionWithInheritable");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TypedefUnionWithInheritable"),
    crate::TypedefUnionWithInheritable
);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN27TypedefUnionWithInheritableC1Ev {}
impl<'error> Default for TypedefUnionWithInheritable
where
    &'error (): BindingFailedFor_ZN27TypedefUnionWithInheritableC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for item 'TypedefUnionWithInheritable::TypedefUnionWithInheritable':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN27TypedefUnionWithInheritableC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for TypedefUnionWithInheritable
where
    &'error (): BindingFailedFor_ZN27TypedefUnionWithInheritableC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for TypedefUnionWithInheritable {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for TypedefUnionWithInheritable {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableaSEOS_(self, __param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10EmptyUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::EmptyUnion,
            __param_0: &'b crate::EmptyUnion,
        ) -> &'a mut crate::EmptyUnion;
        pub(crate) unsafe fn __rust_thunk___ZN10EmptyUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::EmptyUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::EmptyUnion>,
        ) -> &'a mut crate::EmptyUnion;
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13UnionToRenameaSERKS_<'a, 'b>(
            __this: &'a mut crate::RenamedUnion,
            __param_0: &'b crate::RenamedUnion,
        ) -> &'a mut crate::RenamedUnion;
        pub(crate) unsafe fn __rust_thunk___ZN13UnionToRenameaSEOS_<'a, 'b>(
            __this: &'a mut crate::RenamedUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::RenamedUnion>,
        ) -> &'a mut crate::RenamedUnion;
        pub(crate) unsafe fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_<
            'a,
            'b,
        >(
            __this: ::core::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
            __param_0: &'b crate::TriviallyCopyableButNontriviallyDestructible,
        ) -> ::core::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>;
        pub(crate) unsafe fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_<
            'b,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'b crate::TriviallyCopyableButNontriviallyDestructible,
        );
        pub(crate) unsafe fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev<
            'a,
        >(
            __this: ::core::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13NonEmptyUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::NonEmptyUnion,
            __param_0: &'b crate::NonEmptyUnion,
        ) -> &'a mut crate::NonEmptyUnion;
        pub(crate) unsafe fn __rust_thunk___ZN13NonEmptyUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::NonEmptyUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::NonEmptyUnion>,
        ) -> &'a mut crate::NonEmptyUnion;
        pub(crate) unsafe fn __rust_thunk___ZN13NonCopyUnion2aSERKS_<'a, 'b>(
            __this: &'a mut crate::NonCopyUnion2,
            __param_0: &'b crate::NonCopyUnion2,
        ) -> &'a mut crate::NonCopyUnion2;
        pub(crate) unsafe fn __rust_thunk___ZN13NonCopyUnion2aSEOS_<'a, 'b>(
            __this: &'a mut crate::NonCopyUnion2,
            __param_0: ::ctor::RvalueReference<'b, crate::NonCopyUnion2>,
        ) -> &'a mut crate::NonCopyUnion2;
        pub(crate) unsafe fn __rust_thunk___ZN20UnionWithOpaqueFieldaSERKS_<'a, 'b>(
            __this: &'a mut crate::UnionWithOpaqueField,
            __param_0: &'b crate::UnionWithOpaqueField,
        ) -> &'a mut crate::UnionWithOpaqueField;
        pub(crate) unsafe fn __rust_thunk___ZN20UnionWithOpaqueFieldaSEOS_<'a, 'b>(
            __this: &'a mut crate::UnionWithOpaqueField,
            __param_0: ::ctor::RvalueReference<'b, crate::UnionWithOpaqueField>,
        ) -> &'a mut crate::UnionWithOpaqueField;
        pub(crate) unsafe fn __rust_thunk___ZN21TrivialButInheritableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21TrivialButInheritableC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::TrivialButInheritable>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21TrivialButInheritableaSERKS_<'a, 'b>(
            __this: &'a mut crate::TrivialButInheritable,
            __param_0: &'b crate::TrivialButInheritable,
        ) -> &'a mut crate::TrivialButInheritable;
        pub(crate) unsafe fn __rust_thunk___ZN21TrivialButInheritableaSEOS_<'a, 'b>(
            __this: &'a mut crate::TrivialButInheritable,
            __param_0: ::ctor::RvalueReference<'b, crate::TrivialButInheritable>,
        ) -> &'a mut crate::TrivialButInheritable;
        pub(crate) unsafe fn __rust_thunk___ZN20UnionWithInheritableaSERKS_<'a, 'b>(
            __this: &'a mut crate::UnionWithInheritable,
            __param_0: &'b crate::UnionWithInheritable,
        ) -> &'a mut crate::UnionWithInheritable;
        pub(crate) unsafe fn __rust_thunk___ZN20UnionWithInheritableaSEOS_<'a, 'b>(
            __this: &'a mut crate::UnionWithInheritable,
            __param_0: ::ctor::RvalueReference<'b, crate::UnionWithInheritable>,
        ) -> &'a mut crate::UnionWithInheritable;
        pub(crate) unsafe fn __rust_thunk___ZN12TypedefUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::TypedefUnion,
            __param_0: &'b crate::TypedefUnion,
        ) -> &'a mut crate::TypedefUnion;
        pub(crate) unsafe fn __rust_thunk___ZN12TypedefUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::TypedefUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnion>,
        ) -> &'a mut crate::TypedefUnion;
        pub(crate) unsafe fn __rust_thunk___ZN27TypedefUnionWithInheritableaSERKS_<'a, 'b>(
            __this: &'a mut crate::TypedefUnionWithInheritable,
            __param_0: &'b crate::TypedefUnionWithInheritable,
        ) -> &'a mut crate::TypedefUnionWithInheritable;
        pub(crate) unsafe fn __rust_thunk___ZN27TypedefUnionWithInheritableaSEOS_<'a, 'b>(
            __this: &'a mut crate::TypedefUnionWithInheritable,
            __param_0: ::ctor::RvalueReference<'b, crate::TypedefUnionWithInheritable>,
        ) -> &'a mut crate::TypedefUnionWithInheritable;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::EmptyUnion>() == 1);
    assert!(::core::mem::align_of::<crate::EmptyUnion>() == 1);
    static_assertions::assert_impl_all!(crate::EmptyUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::EmptyUnion: Drop);

    assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
    assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::Nontrivial, field) == 0);
    assert!(::core::mem::size_of::<crate::RenamedUnion>() == 1);
    assert!(::core::mem::align_of::<crate::RenamedUnion>() == 1);
    static_assertions::assert_impl_all!(crate::RenamedUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::RenamedUnion: Drop);

    assert!(::core::mem::size_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
    assert!(::core::mem::align_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
    static_assertions::assert_impl_all!(crate::TriviallyCopyableButNontriviallyDestructible: Drop);
    static_assertions::assert_not_impl_any!(crate::TriviallyCopyableButNontriviallyDestructible: Copy);

    assert!(::core::mem::size_of::<crate::NonEmptyUnion>() == 8);
    assert!(::core::mem::align_of::<crate::NonEmptyUnion>() == 8);
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NonEmptyUnion: Drop);
    assert!(::core::mem::offset_of!(crate::NonEmptyUnion, bool_field) == 0);
    assert!(::core::mem::offset_of!(crate::NonEmptyUnion, char_field) == 0);
    assert!(::core::mem::offset_of!(crate::NonEmptyUnion, int_field) == 0);
    assert!(::core::mem::offset_of!(crate::NonEmptyUnion, long_long_field) == 0);
    static_assertions::assert_impl_all!(bool: Copy);
    static_assertions::assert_impl_all!(::core::ffi::c_char: Copy);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);
    static_assertions::assert_impl_all!(::core::ffi::c_longlong: Copy);
    assert!(::core::mem::size_of::<crate::NonCopyUnion>() == 4);
    assert!(::core::mem::align_of::<crate::NonCopyUnion>() == 4);
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::NonCopyUnion, trivial_member) == 0);
    assert!(::core::mem::offset_of!(crate::NonCopyUnion, nontrivial_member) == 0);
    static_assertions::assert_impl_all!(bool: Copy);
    assert!(::core::mem::size_of::<crate::NonCopyUnion2>() == 1);
    assert!(::core::mem::align_of::<crate::NonCopyUnion2>() == 1);
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion2: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::NonCopyUnion2, trivial_member) == 0);
    assert!(::core::mem::offset_of!(crate::NonCopyUnion2, nontrivial_member) == 0);
    static_assertions::assert_impl_all!(bool: Copy);
    assert!(::core::mem::size_of::<crate::UnionWithOpaqueField>() == 42);
    assert!(::core::mem::align_of::<crate::UnionWithOpaqueField>() == 1);
    static_assertions::assert_impl_all!(crate::UnionWithOpaqueField: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::UnionWithOpaqueField: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::UnionWithOpaqueField,
            constant_array_field_not_yet_supported
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::TrivialButInheritable>() == 4);
    assert!(::core::mem::align_of::<crate::TrivialButInheritable>() == 4);
    static_assertions::assert_impl_all!(crate::TrivialButInheritable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TrivialButInheritable: Drop);
    assert!(::core::mem::offset_of!(crate::TrivialButInheritable, x) == 0);
    assert!(::core::mem::size_of::<crate::UnionWithInheritable>() == 4);
    assert!(::core::mem::align_of::<crate::UnionWithInheritable>() == 4);
    static_assertions::assert_impl_all!(crate::UnionWithInheritable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::UnionWithInheritable: Drop);
    assert!(::core::mem::offset_of!(crate::UnionWithInheritable, t) == 0);
    static_assertions::assert_impl_all!(crate::TrivialButInheritable: Copy);
    assert!(::core::mem::size_of::<crate::TypedefUnion>() == 1);
    assert!(::core::mem::align_of::<crate::TypedefUnion>() == 1);
    static_assertions::assert_impl_all!(crate::TypedefUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TypedefUnion: Drop);
    assert!(::core::mem::offset_of!(crate::TypedefUnion, trivial_member) == 0);
    static_assertions::assert_impl_all!(bool: Copy);
    assert!(::core::mem::size_of::<crate::TypedefUnionWithInheritable>() == 4);
    assert!(::core::mem::align_of::<crate::TypedefUnionWithInheritable>() == 4);
    static_assertions::assert_impl_all!(crate::TypedefUnionWithInheritable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TypedefUnionWithInheritable: Drop);
    assert!(::core::mem::offset_of!(crate::TypedefUnionWithInheritable, t) == 0);
    static_assertions::assert_impl_all!(crate::TrivialButInheritable: Copy);
};
