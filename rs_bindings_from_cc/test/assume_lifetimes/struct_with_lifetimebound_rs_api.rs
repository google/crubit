// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:struct_with_lifetimebound

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
#[cfi_encoding = "11PlainStruct"]
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
#[cfi_encoding = "37StructWithLifetimeboundMemberFunction"]
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

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "40StructWithLifetimeboundRefMemberFunction"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithLifetimeboundRefMemberFunction
pub struct StructWithLifetimeboundRefMemberFunction {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for StructWithLifetimeboundRefMemberFunction {}
impl !Sync for StructWithLifetimeboundRefMemberFunction {}
unsafe impl ::cxx::ExternType for StructWithLifetimeboundRefMemberFunction {
    type Id = ::cxx::type_id!("StructWithLifetimeboundRefMemberFunction");
    type Kind = ::cxx::kind::Trivial;
}
impl StructWithLifetimeboundRefMemberFunction {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> ::cref::CRef<'__this, crate::PlainStruct> {
        unsafe { self::struct_with_lifetimebound_ref_member_function::f(self) }
    }
}

impl Default for StructWithLifetimeboundRefMemberFunction {
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
    pub(crate) fn f<'__this>(
        __this: &'__this crate::StructWithLifetimeboundRefMemberFunction,
    ) -> ::cref::CRef<'__this, crate::PlainStruct> {
        unsafe {
            crate::detail::__rust_thunk___ZNK40StructWithLifetimeboundRefMemberFunction1fEv(__this)
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "40DropClassWithLifetimeboundMemberFunction"]
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
#[cfi_encoding = "43DropClassWithLifetimeboundRefMemberFunction"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropClassWithLifetimeboundRefMemberFunction
pub struct DropClassWithLifetimeboundRefMemberFunction {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for DropClassWithLifetimeboundRefMemberFunction {}
impl !Sync for DropClassWithLifetimeboundRefMemberFunction {}
unsafe impl ::cxx::ExternType for DropClassWithLifetimeboundRefMemberFunction {
    type Id = ::cxx::type_id!("DropClassWithLifetimeboundRefMemberFunction");
    type Kind = ::cxx::kind::Opaque;
}
impl DropClassWithLifetimeboundRefMemberFunction {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> ::cref::CRef<'__this, crate::PlainStruct> {
        unsafe { self::drop_class_with_lifetimebound_ref_member_function::f(self) }
    }
}

impl ::ctor::CtorNew<()> for DropClassWithLifetimeboundRefMemberFunction {
    type CtorType = ::ctor::Ctor![Self];
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

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for DropClassWithLifetimeboundRefMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionC1ERKS_(__crubit_dest as*mut::core::ffi::c_void,__param_0);
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)>
    for DropClassWithLifetimeboundRefMemberFunction
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for DropClassWithLifetimeboundRefMemberFunction {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::PinnedDrop for DropClassWithLifetimeboundRefMemberFunction {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionD1Ev(self)
        }
    }
}

pub mod drop_class_with_lifetimebound_ref_member_function {
    #[inline(always)]
    pub(crate) fn f<'__this>(
        __this: &'__this crate::DropClassWithLifetimeboundRefMemberFunction,
    ) -> ::cref::CRef<'__this, crate::PlainStruct> {
        unsafe {
            crate::detail::__rust_thunk___ZNK43DropClassWithLifetimeboundRefMemberFunction1fEv(
                __this,
            )
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "27StructWithLifetimeboundCtor"]
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

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "30StructWithLifetimeboundRefCtor"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithLifetimeboundRefCtor
pub struct StructWithLifetimeboundRefCtor {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for StructWithLifetimeboundRefCtor {}
impl !Sync for StructWithLifetimeboundRefCtor {}
unsafe impl ::cxx::ExternType for StructWithLifetimeboundRefCtor {
    type Id = ::cxx::type_id!("StructWithLifetimeboundRefCtor");
    type Kind = ::cxx::kind::Trivial;
}

impl<'s> From<&'s crate::PlainStruct> for StructWithLifetimeboundRefCtor {
    #[inline(always)]
    fn from(args: &'s crate::PlainStruct) -> Self {
        let mut s = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30StructWithLifetimeboundRefCtorC1ERK11PlainStruct(
                &raw mut tmp as *mut _,
                s,
            );
            tmp.assume_init()
        }
    }
}
impl<'s> ::ctor::CtorNew<&'s crate::PlainStruct> for StructWithLifetimeboundRefCtor {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'s crate::PlainStruct) -> Self::CtorType {
        <Self as From<&'s crate::PlainStruct>>::from(args)
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "31DropStructWithLifetimeboundCtor"]
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
#[cfi_encoding = "34DropStructWithLifetimeboundRefCtor"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropStructWithLifetimeboundRefCtor
pub struct DropStructWithLifetimeboundRefCtor {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for DropStructWithLifetimeboundRefCtor {}
impl !Sync for DropStructWithLifetimeboundRefCtor {}
unsafe impl ::cxx::ExternType for DropStructWithLifetimeboundRefCtor {
    type Id = ::cxx::type_id!("DropStructWithLifetimeboundRefCtor");
    type Kind = ::cxx::kind::Opaque;
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for DropStructWithLifetimeboundRefCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
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
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for DropStructWithLifetimeboundRefCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for DropStructWithLifetimeboundRefCtor {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN34DropStructWithLifetimeboundRefCtoraSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'s> ::ctor::CtorNew<&'s crate::PlainStruct> for DropStructWithLifetimeboundRefCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'s>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'s crate::PlainStruct) -> Self::CtorType {
        let mut s = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN34DropStructWithLifetimeboundRefCtorC1ERK11PlainStruct(__crubit_dest as*mut::core::ffi::c_void,s);
            })
        }
    }
}
impl<'s> ::ctor::CtorNew<(&'s crate::PlainStruct,)> for DropStructWithLifetimeboundRefCtor {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'s>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'s crate::PlainStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'s crate::PlainStruct>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for DropStructWithLifetimeboundRefCtor {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN34DropStructWithLifetimeboundRefCtorD1Ev(self) }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "41DropStructWithRefCtorAndRefMemberFunction"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropStructWithRefCtorAndRefMemberFunction
pub struct DropStructWithRefCtorAndRefMemberFunction {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for DropStructWithRefCtorAndRefMemberFunction {}
impl !Sync for DropStructWithRefCtorAndRefMemberFunction {}
unsafe impl ::cxx::ExternType for DropStructWithRefCtorAndRefMemberFunction {
    type Id = ::cxx::type_id!("DropStructWithRefCtorAndRefMemberFunction");
    type Kind = ::cxx::kind::Opaque;
}
impl DropStructWithRefCtorAndRefMemberFunction {
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> ::cref::CRef<'__this, crate::PlainStruct> {
        unsafe { self::drop_struct_with_ref_ctor_and_ref_member_function::f(self) }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for DropStructWithRefCtorAndRefMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
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
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)>
    for DropStructWithRefCtorAndRefMemberFunction
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for DropStructWithRefCtorAndRefMemberFunction {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'s> ::ctor::CtorNew<&'s crate::PlainStruct> for DropStructWithRefCtorAndRefMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'s>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'s crate::PlainStruct) -> Self::CtorType {
        let mut s = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionC1ERK11PlainStruct(__crubit_dest as*mut::core::ffi::c_void,s);
            })
        }
    }
}
impl<'s> ::ctor::CtorNew<(&'s crate::PlainStruct,)> for DropStructWithRefCtorAndRefMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'s>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'s crate::PlainStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'s crate::PlainStruct>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for DropStructWithRefCtorAndRefMemberFunction {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionD1Ev(self)
        }
    }
}

pub mod drop_struct_with_ref_ctor_and_ref_member_function {
    #[inline(always)]
    pub(crate) fn f<'__this>(
        __this: &'__this crate::DropStructWithRefCtorAndRefMemberFunction,
    ) -> ::cref::CRef<'__this, crate::PlainStruct> {
        unsafe {
            crate::detail::__rust_thunk___ZNK41DropStructWithRefCtorAndRefMemberFunction1fEv(__this)
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "35DropStructWithCtorAndMemberFunction"]
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
#[cfi_encoding = "38DropStructWithCtorAndRefMemberFunction"]
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
#[cfi_encoding = "38DropStructWithRefCtorAndMemberFunction"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DropStructWithRefCtorAndMemberFunction
pub struct DropStructWithRefCtorAndMemberFunction {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for DropStructWithRefCtorAndMemberFunction {}
impl !Sync for DropStructWithRefCtorAndMemberFunction {}
unsafe impl ::cxx::ExternType for DropStructWithRefCtorAndMemberFunction {
    type Id = ::cxx::type_id!("DropStructWithRefCtorAndMemberFunction");
    type Kind = ::cxx::kind::Opaque;
}
impl DropStructWithRefCtorAndMemberFunction {
    /// This is a degenerate case, since `PlainStruct` binds no lifetimes.
    #[inline(always)]
    pub fn f<'__this>(&'__this self) -> crate::PlainStruct {
        unsafe { self::drop_struct_with_ref_ctor_and_member_function::f(self) }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for DropStructWithRefCtorAndMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
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
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for DropStructWithRefCtorAndMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for DropStructWithRefCtorAndMemberFunction {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'s> ::ctor::CtorNew<&'s crate::PlainStruct> for DropStructWithRefCtorAndMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'s>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'s crate::PlainStruct) -> Self::CtorType {
        let mut s = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionC1ERK11PlainStruct(__crubit_dest as*mut::core::ffi::c_void,s);
            })
        }
    }
}
impl<'s> ::ctor::CtorNew<(&'s crate::PlainStruct,)> for DropStructWithRefCtorAndMemberFunction {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'s>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'s crate::PlainStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'s crate::PlainStruct>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for DropStructWithRefCtorAndMemberFunction {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionD1Ev(self)
        }
    }
}

pub mod drop_struct_with_ref_ctor_and_member_function {
    /// This is a degenerate case, since `PlainStruct` binds no lifetimes.
    #[inline(always)]
    pub(crate) fn f<'__this>(
        __this: &'__this crate::DropStructWithRefCtorAndMemberFunction,
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

/// We can't figure out the lifetime arity of this struct.
#[::ctor::recursively_pinned]
#[cfi_encoding = "10Impossible"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Impossible
pub struct Impossible {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for Impossible {}
impl !Sync for Impossible {}
unsafe impl ::cxx::ExternType for Impossible {
    type Id = ::cxx::type_id!("Impossible");
    type Kind = ::cxx::kind::Opaque;
}

// error: function `Impossible::f` could not be bound
//   `Impossible` can't be used by-value because it has a non-public or deleted destructor

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
            '__this,
        >(
            __this: &'__this crate::StructWithLifetimeboundRefMemberFunction,
        ) -> ::cref::CRef<'__this, crate::PlainStruct>;
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
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::DropClassWithLifetimeboundRefMemberFunction,
        );
        pub(crate) unsafe fn __rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionaSERKS_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<
                &'__this mut crate::DropClassWithLifetimeboundRefMemberFunction,
            >,
            __param_0: &'__param_0 crate::DropClassWithLifetimeboundRefMemberFunction,
        ) -> ::core::pin::Pin<&'__this mut crate::DropClassWithLifetimeboundRefMemberFunction>;
        #[link_name = "_ZNK43DropClassWithLifetimeboundRefMemberFunction1fEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK43DropClassWithLifetimeboundRefMemberFunction1fEv<
            '__this,
        >(
            __this: &'__this crate::DropClassWithLifetimeboundRefMemberFunction,
        ) -> ::cref::CRef<'__this, crate::PlainStruct>;
        #[link_name = "_ZN43DropClassWithLifetimeboundRefMemberFunctionD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN43DropClassWithLifetimeboundRefMemberFunctionD1Ev<
            '__this,
        >(
            __this: ::core::pin::Pin<
                &'__this mut crate::DropClassWithLifetimeboundRefMemberFunction,
            >,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27StructWithLifetimeboundCtorC1E11PlainStruct(
            __this: *mut ::core::ffi::c_void,
            s: &mut crate::PlainStruct,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30StructWithLifetimeboundRefCtorC1ERK11PlainStruct<
            's,
        >(
            __this: *mut ::core::ffi::c_void,
            s: &'s crate::PlainStruct,
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
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::DropStructWithLifetimeboundRefCtor,
        );
        pub(crate) unsafe fn __rust_thunk___ZN34DropStructWithLifetimeboundRefCtoraSERKS_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithLifetimeboundRefCtor>,
            __param_0: &'__param_0 crate::DropStructWithLifetimeboundRefCtor,
        ) -> ::core::pin::Pin<&'__this mut crate::DropStructWithLifetimeboundRefCtor>;
        pub(crate) unsafe fn __rust_thunk___ZN34DropStructWithLifetimeboundRefCtorC1ERK11PlainStruct<
            's,
        >(
            __this: *mut ::core::ffi::c_void,
            s: &'s crate::PlainStruct,
        );
        #[link_name = "_ZN34DropStructWithLifetimeboundRefCtorD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN34DropStructWithLifetimeboundRefCtorD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithLifetimeboundRefCtor>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionC1ERKS_<
            '__param_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::DropStructWithRefCtorAndRefMemberFunction,
        );
        pub(crate) unsafe fn __rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionaSERKS_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithRefCtorAndRefMemberFunction>,
            __param_0: &'__param_0 crate::DropStructWithRefCtorAndRefMemberFunction,
        ) -> ::core::pin::Pin<&'__this mut crate::DropStructWithRefCtorAndRefMemberFunction>;
        pub(crate) unsafe fn __rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionC1ERK11PlainStruct<
            's,
        >(
            __this: *mut ::core::ffi::c_void,
            s: &'s crate::PlainStruct,
        );
        #[link_name = "_ZNK41DropStructWithRefCtorAndRefMemberFunction1fEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK41DropStructWithRefCtorAndRefMemberFunction1fEv<
            '__this,
        >(
            __this: &'__this crate::DropStructWithRefCtorAndRefMemberFunction,
        ) -> ::cref::CRef<'__this, crate::PlainStruct>;
        #[link_name = "_ZN41DropStructWithRefCtorAndRefMemberFunctionD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN41DropStructWithRefCtorAndRefMemberFunctionD1Ev<
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithRefCtorAndRefMemberFunction>,
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
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::DropStructWithRefCtorAndMemberFunction,
        );
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionaSERKS_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithRefCtorAndMemberFunction>,
            __param_0: &'__param_0 crate::DropStructWithRefCtorAndMemberFunction,
        ) -> ::core::pin::Pin<&'__this mut crate::DropStructWithRefCtorAndMemberFunction>;
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionC1ERK11PlainStruct<
            's,
        >(
            __this: *mut ::core::ffi::c_void,
            s: &'s crate::PlainStruct,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK38DropStructWithRefCtorAndMemberFunction1fEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::DropStructWithRefCtorAndMemberFunction,
        );
        #[link_name = "_ZN38DropStructWithRefCtorAndMemberFunctionD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN38DropStructWithRefCtorAndMemberFunctionD1Ev<
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::DropStructWithRefCtorAndMemberFunction>,
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
    static_assertions::assert_impl_all!(crate::StructWithLifetimeboundRefMemberFunction: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithLifetimeboundRefMemberFunction: Drop);

    assert!(::core::mem::size_of::<crate::DropClassWithLifetimeboundMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::DropClassWithLifetimeboundMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::DropClassWithLifetimeboundMemberFunction: Drop);
    static_assertions::assert_not_impl_any!(crate::DropClassWithLifetimeboundMemberFunction: Copy);

    assert!(::core::mem::size_of::<crate::DropClassWithLifetimeboundRefMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::DropClassWithLifetimeboundRefMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::DropClassWithLifetimeboundRefMemberFunction: Drop);
    static_assertions::assert_not_impl_any!(crate::DropClassWithLifetimeboundRefMemberFunction: Copy);

    assert!(::core::mem::size_of::<crate::StructWithLifetimeboundCtor>() == 1);
    assert!(::core::mem::align_of::<crate::StructWithLifetimeboundCtor>() == 1);
    static_assertions::assert_impl_all!(crate::StructWithLifetimeboundCtor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithLifetimeboundCtor: Drop);

    assert!(::core::mem::size_of::<crate::StructWithLifetimeboundRefCtor>() == 1);
    assert!(::core::mem::align_of::<crate::StructWithLifetimeboundRefCtor>() == 1);
    static_assertions::assert_impl_all!(crate::StructWithLifetimeboundRefCtor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithLifetimeboundRefCtor: Drop);

    assert!(::core::mem::size_of::<crate::DropStructWithLifetimeboundCtor>() == 1);
    assert!(::core::mem::align_of::<crate::DropStructWithLifetimeboundCtor>() == 1);
    static_assertions::assert_impl_all!(crate::DropStructWithLifetimeboundCtor: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithLifetimeboundCtor: Copy);

    assert!(::core::mem::size_of::<crate::DropStructWithLifetimeboundRefCtor>() == 1);
    assert!(::core::mem::align_of::<crate::DropStructWithLifetimeboundRefCtor>() == 1);
    static_assertions::assert_impl_all!(crate::DropStructWithLifetimeboundRefCtor: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithLifetimeboundRefCtor: Copy);

    assert!(::core::mem::size_of::<crate::DropStructWithRefCtorAndRefMemberFunction>() == 1);
    assert!(::core::mem::align_of::<crate::DropStructWithRefCtorAndRefMemberFunction>() == 1);
    static_assertions::assert_impl_all!(crate::DropStructWithRefCtorAndRefMemberFunction: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithRefCtorAndRefMemberFunction: Copy);

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
    static_assertions::assert_impl_all!(crate::DropStructWithRefCtorAndMemberFunction: Drop);
    static_assertions::assert_not_impl_any!(crate::DropStructWithRefCtorAndMemberFunction: Copy);

    assert!(::core::mem::size_of::<crate::Impossible>() == 1);
    assert!(::core::mem::align_of::<crate::Impossible>() == 1);
    static_assertions::assert_not_impl_any!(crate::Impossible: Copy,Drop);
};
