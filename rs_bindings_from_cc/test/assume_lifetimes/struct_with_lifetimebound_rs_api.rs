// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:struct_with_lifetimebound

#![rustfmt::skip]
#![feature(custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=PlainStruct
pub struct PlainStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for PlainStruct {}
impl !Sync for PlainStruct {}
unsafe impl ::cxx::ExternType for PlainStruct {
    type Id = ::cxx::type_id!("PlainStruct");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for PlainStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11PlainStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithLifetimeboundMemberFunction
pub struct StructWithLifetimeboundMemberFunction {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for StructWithLifetimeboundMemberFunction {}
impl !Sync for StructWithLifetimeboundMemberFunction {}
unsafe impl ::cxx::ExternType for StructWithLifetimeboundMemberFunction {
    type Id = ::cxx::type_id!("StructWithLifetimeboundMemberFunction");
    type Kind = ::cxx::kind::Trivial;
}
impl StructWithLifetimeboundMemberFunction {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> crate::PlainStruct {
        unsafe { self::struct_with_lifetimebound_member_function::f(self) }
    }
}

impl Default for StructWithLifetimeboundMemberFunction {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN37StructWithLifetimeboundMemberFunctionC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod struct_with_lifetimebound_member_function {
    #[inline(always)]
    pub(crate) fn f<'__this>(
        __this: &'__this crate::StructWithLifetimeboundMemberFunction,
    ) -> crate::PlainStruct {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::PlainStruct>::uninit();
            crate::detail::__rust_thunk___ZNK37StructWithLifetimeboundMemberFunction1fEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithLifetimeboundRefMemberFunction
pub struct StructWithLifetimeboundRefMemberFunction<'__implicit> {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    __marker___implicit: ::core::marker::PhantomData<&'__implicit ()>,
}
impl<'__implicit> !Send for StructWithLifetimeboundRefMemberFunction<'__implicit> {}
impl<'__implicit> !Sync for StructWithLifetimeboundRefMemberFunction<'__implicit> {}
unsafe impl<'__implicit> ::cxx::ExternType
    for StructWithLifetimeboundRefMemberFunction<'__implicit>
{
    type Id = ::cxx::type_id!("StructWithLifetimeboundRefMemberFunction");
    type Kind = ::cxx::kind::Trivial;
}
impl<'__implicit> StructWithLifetimeboundRefMemberFunction<'__implicit> {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> ::cref::CRef<'__implicit, crate::PlainStruct> {
        unsafe { self::struct_with_lifetimebound_ref_member_function::f(self) }
    }
}

impl<'__implicit> Default for StructWithLifetimeboundRefMemberFunction<'__implicit> {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN40StructWithLifetimeboundRefMemberFunctionC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod struct_with_lifetimebound_ref_member_function {
    #[inline(always)]
    pub(crate) fn f<'__implicit, '__this>(
        __this: &'__this crate::StructWithLifetimeboundRefMemberFunction<'__implicit>,
    ) -> ::cref::CRef<'__implicit, crate::PlainStruct> {
        unsafe {
            crate::detail::__rust_thunk___ZNK40StructWithLifetimeboundRefMemberFunction1fEv(__this)
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropClassWithLifetimeboundMemberFunction
pub struct DropClassWithLifetimeboundMemberFunction {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for DropClassWithLifetimeboundMemberFunction {}
impl !Sync for DropClassWithLifetimeboundMemberFunction {}
unsafe impl ::cxx::ExternType for DropClassWithLifetimeboundMemberFunction {
    type Id = ::cxx::type_id!("DropClassWithLifetimeboundMemberFunction");
    type Kind = ::cxx::kind::Opaque;
}
impl DropClassWithLifetimeboundMemberFunction {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> crate::PlainStruct {
        unsafe { self::drop_class_with_lifetimebound_member_function::f(self) }
    }
}

impl ::ctor::CtorNew<()> for DropClassWithLifetimeboundMemberFunction {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for DropClassWithLifetimeboundMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for DropClassWithLifetimeboundMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for DropClassWithLifetimeboundMemberFunction {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::PinnedDrop for DropClassWithLifetimeboundMemberFunction {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionD1Ev(self)
        }
    }
}

pub mod drop_class_with_lifetimebound_member_function {
    #[inline(always)]
    pub(crate) fn f<'__this>(
        __this: &'__this crate::DropClassWithLifetimeboundMemberFunction,
    ) -> crate::PlainStruct {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::PlainStruct>::uninit();
            crate::detail::__rust_thunk___ZNK40DropClassWithLifetimeboundMemberFunction1fEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropClassWithLifetimeboundRefMemberFunction
pub struct DropClassWithLifetimeboundRefMemberFunction<'__implicit> {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
    __marker___implicit: ::core::marker::PhantomData<&'__implicit ()>,
}
impl<'__implicit> !Send for DropClassWithLifetimeboundRefMemberFunction<'__implicit> {}
impl<'__implicit> !Sync for DropClassWithLifetimeboundRefMemberFunction<'__implicit> {}
unsafe impl<'__implicit> ::cxx::ExternType
    for DropClassWithLifetimeboundRefMemberFunction<'__implicit>
{
    type Id = ::cxx::type_id!("DropClassWithLifetimeboundRefMemberFunction");
    type Kind = ::cxx::kind::Opaque;
}
impl<'__implicit> DropClassWithLifetimeboundRefMemberFunction<'__implicit> {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> ::cref::CRef<'__implicit, crate::PlainStruct> {
        unsafe { self::drop_class_with_lifetimebound_ref_member_function::f(self) }
    }
}

impl<'__implicit> ::ctor::CtorNew<()> for DropClassWithLifetimeboundRefMemberFunction<'__implicit> {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__implicit>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::CtorNew<&'__param_0_0 crate::DropClassWithLifetimeboundRefMemberFunction<'__param_0>>
    for DropClassWithLifetimeboundRefMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>
        + use<'__implicit, '__param_0, '__param_0_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: &'__param_0_0 crate::DropClassWithLifetimeboundRefMemberFunction<'__param_0>,
    ) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionC1ERKS_(__crubit_dest as*mut::core::ffi::c_void,__param_0);
            })
        }
    }
}
impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::CtorNew<(&'__param_0_0 crate::DropClassWithLifetimeboundRefMemberFunction<'__param_0>,)>
    for DropClassWithLifetimeboundRefMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>
        + use<'__implicit, '__param_0, '__param_0_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (&'__param_0_0 crate::DropClassWithLifetimeboundRefMemberFunction<'__param_0>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            &'__param_0_0 crate::DropClassWithLifetimeboundRefMemberFunction<'__param_0>,
        >>::ctor_new(arg)
    }
}

impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::Assign<&'__param_0_0 crate::DropClassWithLifetimeboundRefMemberFunction<'__param_0>>
    for DropClassWithLifetimeboundRefMemberFunction<'__implicit>
{
    #[inline(always)]
    fn assign<'__this, '__this_0>(
        self: ::core::pin::Pin<&'__this_0 mut Self>,
        __param_0: &'__param_0_0 crate::DropClassWithLifetimeboundRefMemberFunction<'__param_0>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'__implicit> ::ctor::PinnedDrop for DropClassWithLifetimeboundRefMemberFunction<'__implicit> {
    #[inline(always)]
    unsafe fn pinned_drop<'__this, '__this_0>(self: ::core::pin::Pin<&'__this_0 mut Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionD1Ev(self)
        }
    }
}

pub mod drop_class_with_lifetimebound_ref_member_function {
    #[inline(always)]
    pub(crate) fn f<'__implicit, '__this>(
        __this: &'__this crate::DropClassWithLifetimeboundRefMemberFunction<'__implicit>,
    ) -> ::cref::CRef<'__implicit, crate::PlainStruct> {
        unsafe {
            crate::detail::__rust_thunk___ZNK43DropClassWithLifetimeboundRefMemberFunction1fEv(
                __this,
            )
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithLifetimeboundCtor
pub struct StructWithLifetimeboundCtor {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for StructWithLifetimeboundCtor {}
impl !Sync for StructWithLifetimeboundCtor {}
unsafe impl ::cxx::ExternType for StructWithLifetimeboundCtor {
    type Id = ::cxx::type_id!("StructWithLifetimeboundCtor");
    type Kind = ::cxx::kind::Trivial;
}

impl From<crate::PlainStruct> for StructWithLifetimeboundCtor {
    #[inline(always)]
    fn from(args: crate::PlainStruct) -> Self {
        let mut s = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN27StructWithLifetimeboundCtorC1E11PlainStruct(
                &raw mut tmp as *mut _,
                &mut s,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::PlainStruct> for StructWithLifetimeboundCtor {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::PlainStruct) -> Self::CtorType {
        <Self as From<crate::PlainStruct>>::from(args)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithLifetimeboundRefCtor
pub struct StructWithLifetimeboundRefCtor<'__implicit> {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    __marker___implicit: ::core::marker::PhantomData<&'__implicit ()>,
}
impl<'__implicit> !Send for StructWithLifetimeboundRefCtor<'__implicit> {}
impl<'__implicit> !Sync for StructWithLifetimeboundRefCtor<'__implicit> {}
unsafe impl<'__implicit> ::cxx::ExternType for StructWithLifetimeboundRefCtor<'__implicit> {
    type Id = ::cxx::type_id!("StructWithLifetimeboundRefCtor");
    type Kind = ::cxx::kind::Trivial;
}

impl<'__implicit> From<&'__implicit crate::PlainStruct>
    for StructWithLifetimeboundRefCtor<'__implicit>
{
    #[inline(always)]
    fn from(args: &'__implicit crate::PlainStruct) -> Self {
        let mut s = args;
        let mut tmp =
            ::core::mem::MaybeUninit::<crate::StructWithLifetimeboundRefCtor<'__implicit>>::zeroed(
            );
        unsafe {
            crate::detail::__rust_thunk___ZN30StructWithLifetimeboundRefCtorC1ERK11PlainStruct(
                &raw mut tmp as *mut _,
                s,
            );
            tmp.assume_init()
        }
    }
}
impl<'__implicit> ::ctor::CtorNew<&'__implicit crate::PlainStruct>
    for StructWithLifetimeboundRefCtor<'__implicit>
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__implicit crate::PlainStruct) -> Self::CtorType {
        <Self as From<&'__implicit crate::PlainStruct>>::from(args)
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropStructWithLifetimeboundCtor
pub struct DropStructWithLifetimeboundCtor {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for DropStructWithLifetimeboundCtor {}
impl !Sync for DropStructWithLifetimeboundCtor {}
unsafe impl ::cxx::ExternType for DropStructWithLifetimeboundCtor {
    type Id = ::cxx::type_id!("DropStructWithLifetimeboundCtor");
    type Kind = ::cxx::kind::Opaque;
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for DropStructWithLifetimeboundCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN31DropStructWithLifetimeboundCtorC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for DropStructWithLifetimeboundCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for DropStructWithLifetimeboundCtor {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN31DropStructWithLifetimeboundCtoraSERKS_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<crate::PlainStruct> for DropStructWithLifetimeboundCtor {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::PlainStruct) -> Self::CtorType {
        let mut s = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN31DropStructWithLifetimeboundCtorC1E11PlainStruct(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    &mut s,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(crate::PlainStruct,)> for DropStructWithLifetimeboundCtor {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (crate::PlainStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<crate::PlainStruct>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for DropStructWithLifetimeboundCtor {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN31DropStructWithLifetimeboundCtorD1Ev(self) }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropStructWithLifetimeboundRefCtor
pub struct DropStructWithLifetimeboundRefCtor<'__implicit> {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
    __marker___implicit: ::core::marker::PhantomData<&'__implicit ()>,
}
impl<'__implicit> !Send for DropStructWithLifetimeboundRefCtor<'__implicit> {}
impl<'__implicit> !Sync for DropStructWithLifetimeboundRefCtor<'__implicit> {}
unsafe impl<'__implicit> ::cxx::ExternType for DropStructWithLifetimeboundRefCtor<'__implicit> {
    type Id = ::cxx::type_id!("DropStructWithLifetimeboundRefCtor");
    type Kind = ::cxx::kind::Opaque;
}

impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::CtorNew<&'__param_0_0 crate::DropStructWithLifetimeboundRefCtor<'__param_0>>
    for DropStructWithLifetimeboundRefCtor<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>
        + use<'__implicit, '__param_0, '__param_0_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: &'__param_0_0 crate::DropStructWithLifetimeboundRefCtor<'__param_0>,
    ) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN34DropStructWithLifetimeboundRefCtorC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::CtorNew<(&'__param_0_0 crate::DropStructWithLifetimeboundRefCtor<'__param_0>,)>
    for DropStructWithLifetimeboundRefCtor<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>
        + use<'__implicit, '__param_0, '__param_0_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (&'__param_0_0 crate::DropStructWithLifetimeboundRefCtor<'__param_0>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            &'__param_0_0 crate::DropStructWithLifetimeboundRefCtor<'__param_0>,
        >>::ctor_new(arg)
    }
}

impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::Assign<&'__param_0_0 crate::DropStructWithLifetimeboundRefCtor<'__param_0>>
    for DropStructWithLifetimeboundRefCtor<'__implicit>
{
    #[inline(always)]
    fn assign<'__this, '__this_0>(
        self: ::core::pin::Pin<&'__this_0 mut Self>,
        __param_0: &'__param_0_0 crate::DropStructWithLifetimeboundRefCtor<'__param_0>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN34DropStructWithLifetimeboundRefCtoraSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'__implicit> ::ctor::CtorNew<&'__implicit crate::PlainStruct>
    for DropStructWithLifetimeboundRefCtor<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__implicit>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__implicit crate::PlainStruct) -> Self::CtorType {
        let mut s = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |__crubit_dest: *mut crate::DropStructWithLifetimeboundRefCtor<
                    '__implicit,
                >| {
                    crate::detail::__rust_thunk___ZN34DropStructWithLifetimeboundRefCtorC1ERK11PlainStruct(__crubit_dest as*mut::core::ffi::c_void,s);
                },
            )
        }
    }
}
impl<'__implicit> ::ctor::CtorNew<(&'__implicit crate::PlainStruct,)>
    for DropStructWithLifetimeboundRefCtor<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__implicit>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__implicit crate::PlainStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__implicit crate::PlainStruct>>::ctor_new(arg)
    }
}

impl<'__implicit> ::ctor::PinnedDrop for DropStructWithLifetimeboundRefCtor<'__implicit> {
    #[inline(always)]
    unsafe fn pinned_drop<'__this, '__this_0>(self: ::core::pin::Pin<&'__this_0 mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN34DropStructWithLifetimeboundRefCtorD1Ev(self) }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropStructWithRefCtorAndRefMemberFunction
pub struct DropStructWithRefCtorAndRefMemberFunction<'__implicit> {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
    __marker___implicit: ::core::marker::PhantomData<&'__implicit ()>,
}
impl<'__implicit> !Send for DropStructWithRefCtorAndRefMemberFunction<'__implicit> {}
impl<'__implicit> !Sync for DropStructWithRefCtorAndRefMemberFunction<'__implicit> {}
unsafe impl<'__implicit> ::cxx::ExternType
    for DropStructWithRefCtorAndRefMemberFunction<'__implicit>
{
    type Id = ::cxx::type_id!("DropStructWithRefCtorAndRefMemberFunction");
    type Kind = ::cxx::kind::Opaque;
}
impl<'__implicit> DropStructWithRefCtorAndRefMemberFunction<'__implicit> {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> ::cref::CRef<'__implicit, crate::PlainStruct> {
        unsafe { self::drop_struct_with_ref_ctor_and_ref_member_function::f(self) }
    }
}

impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::CtorNew<&'__param_0_0 crate::DropStructWithRefCtorAndRefMemberFunction<'__param_0>>
    for DropStructWithRefCtorAndRefMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>
        + use<'__implicit, '__param_0, '__param_0_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: &'__param_0_0 crate::DropStructWithRefCtorAndRefMemberFunction<'__param_0>,
    ) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::CtorNew<(&'__param_0_0 crate::DropStructWithRefCtorAndRefMemberFunction<'__param_0>,)>
    for DropStructWithRefCtorAndRefMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>
        + use<'__implicit, '__param_0, '__param_0_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (&'__param_0_0 crate::DropStructWithRefCtorAndRefMemberFunction<'__param_0>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            &'__param_0_0 crate::DropStructWithRefCtorAndRefMemberFunction<'__param_0>,
        >>::ctor_new(arg)
    }
}

impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::Assign<&'__param_0_0 crate::DropStructWithRefCtorAndRefMemberFunction<'__param_0>>
    for DropStructWithRefCtorAndRefMemberFunction<'__implicit>
{
    #[inline(always)]
    fn assign<'__this, '__this_0>(
        self: ::core::pin::Pin<&'__this_0 mut Self>,
        __param_0: &'__param_0_0 crate::DropStructWithRefCtorAndRefMemberFunction<'__param_0>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'__implicit> ::ctor::CtorNew<&'__implicit crate::PlainStruct>
    for DropStructWithRefCtorAndRefMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__implicit>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__implicit crate::PlainStruct) -> Self::CtorType {
        let mut s = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |__crubit_dest: *mut crate::DropStructWithRefCtorAndRefMemberFunction<
                    '__implicit,
                >| {
                    crate::detail::__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionC1ERK11PlainStruct(__crubit_dest as*mut::core::ffi::c_void,s);
                },
            )
        }
    }
}
impl<'__implicit> ::ctor::CtorNew<(&'__implicit crate::PlainStruct,)>
    for DropStructWithRefCtorAndRefMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__implicit>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__implicit crate::PlainStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__implicit crate::PlainStruct>>::ctor_new(arg)
    }
}

impl<'__implicit> ::ctor::PinnedDrop for DropStructWithRefCtorAndRefMemberFunction<'__implicit> {
    #[inline(always)]
    unsafe fn pinned_drop<'__this, '__this_0>(self: ::core::pin::Pin<&'__this_0 mut Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionD1Ev(self)
        }
    }
}

pub mod drop_struct_with_ref_ctor_and_ref_member_function {
    #[inline(always)]
    pub(crate) fn f<'__implicit, '__this>(
        __this: &'__this crate::DropStructWithRefCtorAndRefMemberFunction<'__implicit>,
    ) -> ::cref::CRef<'__implicit, crate::PlainStruct> {
        unsafe {
            crate::detail::__rust_thunk___ZNK41DropStructWithRefCtorAndRefMemberFunction1fEv(__this)
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropStructWithCtorAndMemberFunction
pub struct DropStructWithCtorAndMemberFunction {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for DropStructWithCtorAndMemberFunction {}
impl !Sync for DropStructWithCtorAndMemberFunction {}
unsafe impl ::cxx::ExternType for DropStructWithCtorAndMemberFunction {
    type Id = ::cxx::type_id!("DropStructWithCtorAndMemberFunction");
    type Kind = ::cxx::kind::Opaque;
}
impl DropStructWithCtorAndMemberFunction {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> crate::PlainStruct {
        unsafe { self::drop_struct_with_ctor_and_member_function::f(self) }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for DropStructWithCtorAndMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN35DropStructWithCtorAndMemberFunctionC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for DropStructWithCtorAndMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for DropStructWithCtorAndMemberFunction {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN35DropStructWithCtorAndMemberFunctionaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<crate::PlainStruct> for DropStructWithCtorAndMemberFunction {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::PlainStruct) -> Self::CtorType {
        let mut s = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN35DropStructWithCtorAndMemberFunctionC1E11PlainStruct(__crubit_dest as*mut::core::ffi::c_void,&mut s);
            })
        }
    }
}
impl ::ctor::CtorNew<(crate::PlainStruct,)> for DropStructWithCtorAndMemberFunction {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (crate::PlainStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<crate::PlainStruct>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for DropStructWithCtorAndMemberFunction {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN35DropStructWithCtorAndMemberFunctionD1Ev(self) }
    }
}

pub mod drop_struct_with_ctor_and_member_function {
    #[inline(always)]
    pub(crate) fn f<'__this>(
        __this: &'__this crate::DropStructWithCtorAndMemberFunction,
    ) -> crate::PlainStruct {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::PlainStruct>::uninit();
            crate::detail::__rust_thunk___ZNK35DropStructWithCtorAndMemberFunction1fEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropStructWithCtorAndRefMemberFunction
pub struct DropStructWithCtorAndRefMemberFunction {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for DropStructWithCtorAndRefMemberFunction {}
impl !Sync for DropStructWithCtorAndRefMemberFunction {}
unsafe impl ::cxx::ExternType for DropStructWithCtorAndRefMemberFunction {
    type Id = ::cxx::type_id!("DropStructWithCtorAndRefMemberFunction");
    type Kind = ::cxx::kind::Opaque;
}
impl DropStructWithCtorAndRefMemberFunction {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> ::cref::CRef<'__this, crate::PlainStruct> {
        unsafe { self::drop_struct_with_ctor_and_ref_member_function::f(self) }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for DropStructWithCtorAndRefMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for DropStructWithCtorAndRefMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for DropStructWithCtorAndRefMemberFunction {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<crate::PlainStruct> for DropStructWithCtorAndRefMemberFunction {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::PlainStruct) -> Self::CtorType {
        let mut s = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionC1E11PlainStruct(__crubit_dest as*mut::core::ffi::c_void,&mut s);
            })
        }
    }
}
impl ::ctor::CtorNew<(crate::PlainStruct,)> for DropStructWithCtorAndRefMemberFunction {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (crate::PlainStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<crate::PlainStruct>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for DropStructWithCtorAndRefMemberFunction {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionD1Ev(self)
        }
    }
}

pub mod drop_struct_with_ctor_and_ref_member_function {
    #[inline(always)]
    pub(crate) fn f<'__this>(
        __this: &'__this crate::DropStructWithCtorAndRefMemberFunction,
    ) -> ::cref::CRef<'__this, crate::PlainStruct> {
        unsafe {
            crate::detail::__rust_thunk___ZNK38DropStructWithCtorAndRefMemberFunction1fEv(__this)
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropStructWithRefCtorAndMemberFunction
pub struct DropStructWithRefCtorAndMemberFunction<'__implicit> {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
    __marker___implicit: ::core::marker::PhantomData<&'__implicit ()>,
}
impl<'__implicit> !Send for DropStructWithRefCtorAndMemberFunction<'__implicit> {}
impl<'__implicit> !Sync for DropStructWithRefCtorAndMemberFunction<'__implicit> {}
unsafe impl<'__implicit> ::cxx::ExternType for DropStructWithRefCtorAndMemberFunction<'__implicit> {
    type Id = ::cxx::type_id!("DropStructWithRefCtorAndMemberFunction");
    type Kind = ::cxx::kind::Opaque;
}
impl<'__implicit> DropStructWithRefCtorAndMemberFunction<'__implicit> {
    /// This is a degenerate case, since `PlainStruct` binds no lifetimes.
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> crate::PlainStruct {
        unsafe { self::drop_struct_with_ref_ctor_and_member_function::f(self) }
    }
}

impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::CtorNew<&'__param_0_0 crate::DropStructWithRefCtorAndMemberFunction<'__param_0>>
    for DropStructWithRefCtorAndMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>
        + use<'__implicit, '__param_0, '__param_0_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: &'__param_0_0 crate::DropStructWithRefCtorAndMemberFunction<'__param_0>,
    ) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::CtorNew<(&'__param_0_0 crate::DropStructWithRefCtorAndMemberFunction<'__param_0>,)>
    for DropStructWithRefCtorAndMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>
        + use<'__implicit, '__param_0, '__param_0_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (&'__param_0_0 crate::DropStructWithRefCtorAndMemberFunction<'__param_0>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<
            &'__param_0_0 crate::DropStructWithRefCtorAndMemberFunction<'__param_0>,
        >>::ctor_new(arg)
    }
}

impl<'__implicit, '__param_0, '__param_0_0>
    ::ctor::Assign<&'__param_0_0 crate::DropStructWithRefCtorAndMemberFunction<'__param_0>>
    for DropStructWithRefCtorAndMemberFunction<'__implicit>
{
    #[inline(always)]
    fn assign<'__this, '__this_0>(
        self: ::core::pin::Pin<&'__this_0 mut Self>,
        __param_0: &'__param_0_0 crate::DropStructWithRefCtorAndMemberFunction<'__param_0>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'__implicit> ::ctor::CtorNew<&'__implicit crate::PlainStruct>
    for DropStructWithRefCtorAndMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__implicit>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__implicit crate::PlainStruct) -> Self::CtorType {
        let mut s = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |__crubit_dest: *mut crate::DropStructWithRefCtorAndMemberFunction<
                    '__implicit,
                >| {
                    crate::detail::__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionC1ERK11PlainStruct(__crubit_dest as*mut::core::ffi::c_void,s);
                },
            )
        }
    }
}
impl<'__implicit> ::ctor::CtorNew<(&'__implicit crate::PlainStruct,)>
    for DropStructWithRefCtorAndMemberFunction<'__implicit>
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__implicit>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__implicit crate::PlainStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__implicit crate::PlainStruct>>::ctor_new(arg)
    }
}

impl<'__implicit> ::ctor::PinnedDrop for DropStructWithRefCtorAndMemberFunction<'__implicit> {
    #[inline(always)]
    unsafe fn pinned_drop<'__this, '__this_0>(self: ::core::pin::Pin<&'__this_0 mut Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionD1Ev(self)
        }
    }
}

pub mod drop_struct_with_ref_ctor_and_member_function {
    /// This is a degenerate case, since `PlainStruct` binds no lifetimes.
    #[inline(always)]
    pub(crate) fn f<'__implicit, '__this>(
        __this: &'__this crate::DropStructWithRefCtorAndMemberFunction<'__implicit>,
    ) -> crate::PlainStruct {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::PlainStruct>::uninit();
            crate::detail::__rust_thunk___ZNK38DropStructWithRefCtorAndMemberFunction1fEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

// error: struct `Impossible` could not be bound
//   Cycle detected: decl_lifetime_arity

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN11PlainStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN37StructWithLifetimeboundMemberFunctionC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK37StructWithLifetimeboundMemberFunction1fEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::StructWithLifetimeboundMemberFunction,
        );
        pub(crate) unsafe fn __rust_thunk___ZN40StructWithLifetimeboundRefMemberFunctionC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZNK40StructWithLifetimeboundRefMemberFunction1fEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK40StructWithLifetimeboundRefMemberFunction1fEv<
            '__implicit,
            '__this,
        >(
            __this: &'__this crate::StructWithLifetimeboundRefMemberFunction<'__implicit>,
        ) -> ::cref::CRef<'__implicit, crate::PlainStruct>;
        pub(crate) unsafe fn __rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionC1ERKS_<
            '__param_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::DropClassWithLifetimeboundMemberFunction,
        );
        pub(crate) unsafe fn __rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionaSERKS_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropClassWithLifetimeboundMemberFunction>,
            __param_0: &'__param_0 crate::DropClassWithLifetimeboundMemberFunction,
        ) -> ::core::pin::Pin<&'__this mut crate::DropClassWithLifetimeboundMemberFunction>;
        pub(crate) unsafe fn __rust_thunk___ZNK40DropClassWithLifetimeboundMemberFunction1fEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::DropClassWithLifetimeboundMemberFunction,
        );
        #[link_name = "_ZN40DropClassWithLifetimeboundMemberFunctionD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN40DropClassWithLifetimeboundMemberFunctionD1Ev<
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropClassWithLifetimeboundMemberFunction>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionC1ERKS_<
            '__param_0,
            '__param_0_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0_0 crate::DropClassWithLifetimeboundRefMemberFunction<'__param_0>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionaSERKS_<
            '__param_0,
            '__param_0_0,
            '__this,
            '__this_0,
        >(
            __this: ::core::pin::Pin<
                &'__this_0 mut crate::DropClassWithLifetimeboundRefMemberFunction<'__this>,
            >,
            __param_0: &'__param_0_0 crate::DropClassWithLifetimeboundRefMemberFunction<'__param_0>,
        ) -> ::core::pin::Pin<
            &'__this_0 mut crate::DropClassWithLifetimeboundRefMemberFunction<'__this_0>,
        >;
        #[link_name = "_ZNK43DropClassWithLifetimeboundRefMemberFunction1fEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK43DropClassWithLifetimeboundRefMemberFunction1fEv<
            '__implicit,
            '__this,
        >(
            __this: &'__this crate::DropClassWithLifetimeboundRefMemberFunction<'__implicit>,
        ) -> ::cref::CRef<'__implicit, crate::PlainStruct>;
        #[link_name = "_ZN43DropClassWithLifetimeboundRefMemberFunctionD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionD1Ev<
            '__this,
            '__this_0,
        >(
            __this: ::core::pin::Pin<
                &'__this_0 mut crate::DropClassWithLifetimeboundRefMemberFunction<'__this>,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27StructWithLifetimeboundCtorC1E11PlainStruct(
            __this: *mut ::core::ffi::c_void,
            s: &mut crate::PlainStruct,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30StructWithLifetimeboundRefCtorC1ERK11PlainStruct<
            '__implicit,
        >(
            __this: *mut ::core::ffi::c_void,
            s: &'__implicit crate::PlainStruct,
        );
        pub(crate) unsafe fn __rust_thunk___ZN31DropStructWithLifetimeboundCtorC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::DropStructWithLifetimeboundCtor,
        );
        pub(crate) unsafe fn __rust_thunk___ZN31DropStructWithLifetimeboundCtoraSERKS_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithLifetimeboundCtor>,
            __param_0: &'__param_0 crate::DropStructWithLifetimeboundCtor,
        ) -> ::core::pin::Pin<&'__this mut crate::DropStructWithLifetimeboundCtor>;
        pub(crate) unsafe fn __rust_thunk___ZN31DropStructWithLifetimeboundCtorC1E11PlainStruct(
            __this: *mut ::core::ffi::c_void,
            s: &mut crate::PlainStruct,
        );
        #[link_name = "_ZN31DropStructWithLifetimeboundCtorD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN31DropStructWithLifetimeboundCtorD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithLifetimeboundCtor>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN34DropStructWithLifetimeboundRefCtorC1ERKS_<
            '__param_0,
            '__param_0_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0_0 crate::DropStructWithLifetimeboundRefCtor<'__param_0>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN34DropStructWithLifetimeboundRefCtoraSERKS_<
            '__param_0,
            '__param_0_0,
            '__this,
            '__this_0,
        >(
            __this: ::core::pin::Pin<
                &'__this_0 mut crate::DropStructWithLifetimeboundRefCtor<'__this>,
            >,
            __param_0: &'__param_0_0 crate::DropStructWithLifetimeboundRefCtor<'__param_0>,
        ) -> ::core::pin::Pin<&'__this_0 mut crate::DropStructWithLifetimeboundRefCtor<'__this_0>>;
        pub(crate) unsafe fn __rust_thunk___ZN34DropStructWithLifetimeboundRefCtorC1ERK11PlainStruct<
            '__implicit,
        >(
            __this: *mut ::core::ffi::c_void,
            s: &'__implicit crate::PlainStruct,
        );
        #[link_name = "_ZN34DropStructWithLifetimeboundRefCtorD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN34DropStructWithLifetimeboundRefCtorD1Ev<
            '__this,
            '__this_0,
        >(
            __this: ::core::pin::Pin<
                &'__this_0 mut crate::DropStructWithLifetimeboundRefCtor<'__this>,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionC1ERKS_<
            '__param_0,
            '__param_0_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0_0 crate::DropStructWithRefCtorAndRefMemberFunction<'__param_0>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionaSERKS_<
            '__param_0,
            '__param_0_0,
            '__this,
            '__this_0,
        >(
            __this: ::core::pin::Pin<
                &'__this_0 mut crate::DropStructWithRefCtorAndRefMemberFunction<'__this>,
            >,
            __param_0: &'__param_0_0 crate::DropStructWithRefCtorAndRefMemberFunction<'__param_0>,
        ) -> ::core::pin::Pin<
            &'__this_0 mut crate::DropStructWithRefCtorAndRefMemberFunction<'__this_0>,
        >;
        pub(crate) unsafe fn __rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionC1ERK11PlainStruct<
            '__implicit,
        >(
            __this: *mut ::core::ffi::c_void,
            s: &'__implicit crate::PlainStruct,
        );
        #[link_name = "_ZNK41DropStructWithRefCtorAndRefMemberFunction1fEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK41DropStructWithRefCtorAndRefMemberFunction1fEv<
            '__implicit,
            '__this,
        >(
            __this: &'__this crate::DropStructWithRefCtorAndRefMemberFunction<'__implicit>,
        ) -> ::cref::CRef<'__implicit, crate::PlainStruct>;
        #[link_name = "_ZN41DropStructWithRefCtorAndRefMemberFunctionD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionD1Ev<
            '__this,
            '__this_0,
        >(
            __this: ::core::pin::Pin<
                &'__this_0 mut crate::DropStructWithRefCtorAndRefMemberFunction<'__this>,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN35DropStructWithCtorAndMemberFunctionC1ERKS_<
            '__param_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::DropStructWithCtorAndMemberFunction,
        );
        pub(crate) unsafe fn __rust_thunk___ZN35DropStructWithCtorAndMemberFunctionaSERKS_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithCtorAndMemberFunction>,
            __param_0: &'__param_0 crate::DropStructWithCtorAndMemberFunction,
        ) -> ::core::pin::Pin<&'__this mut crate::DropStructWithCtorAndMemberFunction>;
        pub(crate) unsafe fn __rust_thunk___ZN35DropStructWithCtorAndMemberFunctionC1E11PlainStruct(
            __this: *mut ::core::ffi::c_void,
            s: &mut crate::PlainStruct,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK35DropStructWithCtorAndMemberFunction1fEv<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::DropStructWithCtorAndMemberFunction,
        );
        #[link_name = "_ZN35DropStructWithCtorAndMemberFunctionD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN35DropStructWithCtorAndMemberFunctionD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithCtorAndMemberFunction>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionC1ERKS_<
            '__param_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::DropStructWithCtorAndRefMemberFunction,
        );
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionaSERKS_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithCtorAndRefMemberFunction>,
            __param_0: &'__param_0 crate::DropStructWithCtorAndRefMemberFunction,
        ) -> ::core::pin::Pin<&'__this mut crate::DropStructWithCtorAndRefMemberFunction>;
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionC1E11PlainStruct(
            __this: *mut ::core::ffi::c_void,
            s: &mut crate::PlainStruct,
        );
        #[link_name = "_ZNK38DropStructWithCtorAndRefMemberFunction1fEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK38DropStructWithCtorAndRefMemberFunction1fEv<
            '__this,
        >(
            __this: &'__this crate::DropStructWithCtorAndRefMemberFunction,
        ) -> ::cref::CRef<'__this, crate::PlainStruct>;
        #[link_name = "_ZN38DropStructWithCtorAndRefMemberFunctionD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithCtorAndRefMemberFunctionD1Ev<
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithCtorAndRefMemberFunction>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionC1ERKS_<
            '__param_0,
            '__param_0_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0_0 crate::DropStructWithRefCtorAndMemberFunction<'__param_0>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionaSERKS_<
            '__param_0,
            '__param_0_0,
            '__this,
            '__this_0,
        >(
            __this: ::core::pin::Pin<
                &'__this_0 mut crate::DropStructWithRefCtorAndMemberFunction<'__this>,
            >,
            __param_0: &'__param_0_0 crate::DropStructWithRefCtorAndMemberFunction<'__param_0>,
        ) -> ::core::pin::Pin<&'__this_0 mut crate::DropStructWithRefCtorAndMemberFunction<'__this_0>>;
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionC1ERK11PlainStruct<
            '__implicit,
        >(
            __this: *mut ::core::ffi::c_void,
            s: &'__implicit crate::PlainStruct,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK38DropStructWithRefCtorAndMemberFunction1fEv<
            '__implicit,
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::DropStructWithRefCtorAndMemberFunction<'__implicit>,
        );
        #[link_name = "_ZN38DropStructWithRefCtorAndMemberFunctionD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionD1Ev<
            '__this,
            '__this_0,
        >(
            __this: ::core::pin::Pin<
                &'__this_0 mut crate::DropStructWithRefCtorAndMemberFunction<'__this>,
            >,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::PlainStruct>() == 1);
    assert!(::core::mem::align_of::<crate::PlainStruct>() == 1);
    static_assertions::assert_impl_all!(crate::PlainStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::PlainStruct: Drop);

    assert!(::core::mem::size_of::<crate::StructWithLifetimeboundMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::StructWithLifetimeboundMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::StructWithLifetimeboundMemberFunction: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithLifetimeboundMemberFunction: Drop);

    assert!(::core::mem::size_of::<crate::StructWithLifetimeboundRefMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::StructWithLifetimeboundRefMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::StructWithLifetimeboundRefMemberFunction<'static>: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithLifetimeboundRefMemberFunction<'static>: Drop);

    assert!(::core::mem::size_of::<crate::DropClassWithLifetimeboundMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::DropClassWithLifetimeboundMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::DropClassWithLifetimeboundMemberFunction: Drop);
    static_assertions::assert_not_impl_any!(crate::DropClassWithLifetimeboundMemberFunction: Copy);

    assert!(::core::mem::size_of::<crate::DropClassWithLifetimeboundRefMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::DropClassWithLifetimeboundRefMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::DropClassWithLifetimeboundRefMemberFunction<'static>: Drop);
    static_assertions::assert_not_impl_any!(crate::DropClassWithLifetimeboundRefMemberFunction<'static>: Copy);

    assert!(::core::mem::size_of::<crate::StructWithLifetimeboundCtor>() == 1);
    assert!(::core::mem::align_of::<crate::StructWithLifetimeboundCtor>() == 1);
    static_assertions::assert_impl_all!(crate::StructWithLifetimeboundCtor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithLifetimeboundCtor: Drop);

    assert!(::core::mem::size_of::<crate::StructWithLifetimeboundRefCtor>() == 1);
    assert!(::core::mem::align_of::<crate::StructWithLifetimeboundRefCtor>() == 1);
    static_assertions::assert_impl_all!(crate::StructWithLifetimeboundRefCtor<'static>: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithLifetimeboundRefCtor<'static>: Drop);

    assert!(::core::mem::size_of::<crate::DropStructWithLifetimeboundCtor>() == 1);
    assert!(::core::mem::align_of::<crate::DropStructWithLifetimeboundCtor>() == 1);
    static_assertions::assert_impl_all!(crate::DropStructWithLifetimeboundCtor: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithLifetimeboundCtor: Copy);

    assert!(::core::mem::size_of::<crate::DropStructWithLifetimeboundRefCtor>() == 1);
    assert!(::core::mem::align_of::<crate::DropStructWithLifetimeboundRefCtor>() == 1);
    static_assertions::assert_impl_all!(crate::DropStructWithLifetimeboundRefCtor<'static>: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithLifetimeboundRefCtor<'static>: Copy);

    assert!(::core::mem::size_of::<crate::DropStructWithRefCtorAndRefMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::DropStructWithRefCtorAndRefMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::DropStructWithRefCtorAndRefMemberFunction<'static>: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithRefCtorAndRefMemberFunction<'static>: Copy);

    assert!(::core::mem::size_of::<crate::DropStructWithCtorAndMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::DropStructWithCtorAndMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::DropStructWithCtorAndMemberFunction: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithCtorAndMemberFunction: Copy);

    assert!(::core::mem::size_of::<crate::DropStructWithCtorAndRefMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::DropStructWithCtorAndRefMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::DropStructWithCtorAndRefMemberFunction: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithCtorAndRefMemberFunction: Copy);

    assert!(::core::mem::size_of::<crate::DropStructWithRefCtorAndMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::DropStructWithRefCtorAndMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::DropStructWithRefCtorAndMemberFunction<'static>: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithRefCtorAndMemberFunction<'static>: Copy);
};
