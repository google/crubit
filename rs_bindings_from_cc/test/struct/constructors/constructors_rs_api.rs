// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/constructors:constructors
// Features: assume_lifetimes, callables, check_default_initialized, experimental, fmt, supported, unsafe_view, wrapper

#![rustfmt::skip]
#![feature(custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// `[[clang::trivial_abi]]` is used so that `is_trivial_abi` doesn't prevent
/// generating bindings for constructors, even though the presence of a
/// user-defined copy constructor technically means that the struct below
/// is non-trivial.
///
/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=12
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithUserProvidedConstructors
pub struct StructWithUserProvidedConstructors {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub int_field: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for StructWithUserProvidedConstructors {}
impl !Sync for StructWithUserProvidedConstructors {}
unsafe impl ::cxx::ExternType for StructWithUserProvidedConstructors {
    type Id = ::cxx::type_id!("StructWithUserProvidedConstructors");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithUserProvidedConstructors"),
    crate::StructWithUserProvidedConstructors
);

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=12
impl<'__param_0> ::ctor::UnpinAssign<&'__param_0 Self> for StructWithUserProvidedConstructors {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN34StructWithUserProvidedConstructorsaSERKS_(
                self, __param_0,
            );
        }
    }
}

/// `impl Default for StructWithUserProvidedConstructors { /*...*/ }`.
///
/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=14
impl Default for StructWithUserProvidedConstructors {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN34StructWithUserProvidedConstructorsC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

/// `impl Clone for StructWithUserProvidedConstructors { /*...*/ }`.
///
/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=17
impl Clone for StructWithUserProvidedConstructors {
    #[inline(always)]
    fn clone<'__param_0>(&'__param_0 self) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN34StructWithUserProvidedConstructorsC1ERKS_(
                &raw mut tmp as *mut _,
                self,
            );
            tmp.assume_init()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        use ::ctor::UnpinAssign;
        self.unpin_assign(other);
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=22
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithExplicitConversionConstructor
pub struct StructWithExplicitConversionConstructor {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub int_field: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for StructWithExplicitConversionConstructor {}
impl !Sync for StructWithExplicitConversionConstructor {}
unsafe impl ::cxx::ExternType for StructWithExplicitConversionConstructor {
    type Id = ::cxx::type_id!("StructWithExplicitConversionConstructor");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithExplicitConversionConstructor"),
    crate::StructWithExplicitConversionConstructor
);

/// Testing `impl From<int> for /*...*/` when the constructor is `explicit`.
///
/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=24
impl From<::ffi_11::c_int> for StructWithExplicitConversionConstructor {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut i = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN39StructWithExplicitConversionConstructorC1Ei(
                &raw mut tmp as *mut _,
                i,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for StructWithExplicitConversionConstructor {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=29
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithMultipleConstructors
pub struct StructWithMultipleConstructors {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub int_field: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for StructWithMultipleConstructors {}
impl !Sync for StructWithMultipleConstructors {}
unsafe impl ::cxx::ExternType for StructWithMultipleConstructors {
    type Id = ::cxx::type_id!("StructWithMultipleConstructors");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithMultipleConstructors"),
    crate::StructWithMultipleConstructors
);

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=30
impl From<::ffi_11::c_int> for StructWithMultipleConstructors {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut i = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30StructWithMultipleConstructorsC1Ei(
                &raw mut tmp as *mut _,
                i,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for StructWithMultipleConstructors {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=31
impl From<(::ffi_11::c_int, ::ffi_11::c_int)> for StructWithMultipleConstructors {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self {
        let (mut i, mut j) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30StructWithMultipleConstructorsC1Eii(
                &raw mut tmp as *mut _,
                i,
                j,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_int)> for StructWithMultipleConstructors {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, ::ffi_11::c_int)>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=32
impl From<(::ffi_11::c_int, ::ffi_11::c_int, ::ffi_11::c_int)> for StructWithMultipleConstructors {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, ::ffi_11::c_int, ::ffi_11::c_int)) -> Self {
        let (mut i, mut j, mut k) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30StructWithMultipleConstructorsC1Eiii(
                &raw mut tmp as *mut _,
                i,
                j,
                k,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_int, ::ffi_11::c_int)>
    for StructWithMultipleConstructors
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_int, ::ffi_11::c_int)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, ::ffi_11::c_int, ::ffi_11::c_int)>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=38
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithImplicitConversionConstructor
pub struct StructWithImplicitConversionConstructor {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub int_field: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for StructWithImplicitConversionConstructor {}
impl !Sync for StructWithImplicitConversionConstructor {}
unsafe impl ::cxx::ExternType for StructWithImplicitConversionConstructor {
    type Id = ::cxx::type_id!("StructWithImplicitConversionConstructor");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithImplicitConversionConstructor"),
    crate::StructWithImplicitConversionConstructor
);

/// Testing `impl From<int> for /*...*/` when the constructor is *not* `explicit`.
///
/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=41
impl From<::ffi_11::c_int> for StructWithImplicitConversionConstructor {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut i = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN39StructWithImplicitConversionConstructorC1Ei(
                &raw mut tmp as *mut _,
                i,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for StructWithImplicitConversionConstructor {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=46
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=OtherSimpleStruct
pub struct OtherSimpleStruct {
    pub int_field: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for OtherSimpleStruct {}
impl !Sync for OtherSimpleStruct {}
unsafe impl ::cxx::ExternType for OtherSimpleStruct {
    type Id = ::cxx::type_id!("OtherSimpleStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("OtherSimpleStruct"),
    crate::OtherSimpleStruct
);

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=46
impl Default for OtherSimpleStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17OtherSimpleStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=50
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithImplicitConversionFromReference
pub struct StructWithImplicitConversionFromReference {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub int_field: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for StructWithImplicitConversionFromReference {}
impl !Sync for StructWithImplicitConversionFromReference {}
unsafe impl ::cxx::ExternType for StructWithImplicitConversionFromReference {
    type Id = ::cxx::type_id!("StructWithImplicitConversionFromReference");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithImplicitConversionFromReference"),
    crate::StructWithImplicitConversionFromReference
);

/// Testing `impl<'b> From<&'b OtherSimpleStruct> for /*...*/`.
///
/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=53
impl<'other> From<&'other crate::OtherSimpleStruct> for StructWithImplicitConversionFromReference {
    #[inline(always)]
    fn from(args: &'other crate::OtherSimpleStruct) -> Self {
        let mut other = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN41StructWithImplicitConversionFromReferenceC1ERK17OtherSimpleStruct(&raw mut tmp as*mut _,other);
            tmp.assume_init()
        }
    }
}
impl<'other> ::ctor::CtorNew<&'other crate::OtherSimpleStruct>
    for StructWithImplicitConversionFromReference
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'other crate::OtherSimpleStruct) -> Self::CtorType {
        <Self as From<&'other crate::OtherSimpleStruct>>::from(args)
    }
}

/// Inline-defined constructors test that thunks are properly implemented by
/// `generate_rs_api_impl`.
///
/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=61
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithInlineConstructors
pub struct StructWithInlineConstructors {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub int_field: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for StructWithInlineConstructors {}
impl !Sync for StructWithInlineConstructors {}
unsafe impl ::cxx::ExternType for StructWithInlineConstructors {
    type Id = ::cxx::type_id!("StructWithInlineConstructors");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithInlineConstructors"),
    crate::StructWithInlineConstructors
);

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=61
impl<'__param_0> ::ctor::UnpinAssign<&'__param_0 Self> for StructWithInlineConstructors {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN28StructWithInlineConstructorsaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=62
impl Default for StructWithInlineConstructors {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN28StructWithInlineConstructorsC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=63
impl Clone for StructWithInlineConstructors {
    #[inline(always)]
    fn clone<'other>(&'other self) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN28StructWithInlineConstructorsC1ERKS_(
                &raw mut tmp as *mut _,
                self,
            );
            tmp.assume_init()
        }
    }
    fn clone_from(&mut self, other: &Self) {
        use ::ctor::UnpinAssign;
        self.unpin_assign(other);
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=66
impl From<::ffi_11::c_int> for StructWithInlineConstructors {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut i = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN28StructWithInlineConstructorsC1Ei(
                &raw mut tmp as *mut _,
                i,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for StructWithInlineConstructors {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=70
#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithDeletedConstructors
pub struct StructWithDeletedConstructors {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub int_field: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for StructWithDeletedConstructors {}
impl !Sync for StructWithDeletedConstructors {}
unsafe impl ::cxx::ExternType for StructWithDeletedConstructors {
    type Id = ::cxx::type_id!("StructWithDeletedConstructors");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithDeletedConstructors"),
    crate::StructWithDeletedConstructors
);

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=70
impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for StructWithDeletedConstructors {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN29StructWithDeletedConstructorsaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=78
#[::ctor::recursively_pinned]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=StructWithPrivateConstructors
pub struct StructWithPrivateConstructors {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) int_field: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for StructWithPrivateConstructors {}
impl !Sync for StructWithPrivateConstructors {}
unsafe impl ::cxx::ExternType for StructWithPrivateConstructors {
    type Id = ::cxx::type_id!("StructWithPrivateConstructors");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithPrivateConstructors"),
    crate::StructWithPrivateConstructors
);

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=78
impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for StructWithPrivateConstructors {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN29StructWithPrivateConstructorsaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=87
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithExplicitlyDefaultedConstructors
pub struct StructWithExplicitlyDefaultedConstructors {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field_with_explicit_initializer: ::ffi_11::c_int,
    pub field_with_no_initializer: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for StructWithExplicitlyDefaultedConstructors {}
impl !Sync for StructWithExplicitlyDefaultedConstructors {}
unsafe impl ::cxx::ExternType for StructWithExplicitlyDefaultedConstructors {
    type Id = ::cxx::type_id!("StructWithExplicitlyDefaultedConstructors");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithExplicitlyDefaultedConstructors"),
    crate::StructWithExplicitlyDefaultedConstructors
);

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=88
impl Default for StructWithExplicitlyDefaultedConstructors {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN41StructWithExplicitlyDefaultedConstructorsC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

/// TODO(lukasza): Add StructWithImplicitlyDefaultedConstructor test (or is
///                that just testing the compiler and therefore not useful?).
///
/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=99
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NonTrivialStructWithConstructors
pub struct NonTrivialStructWithConstructors {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub int_field: ::ffi_11::c_int,
    #[doc(hidden)]
    pub __crubit_hidden_field_to_force_struct_update_syntax: (),
}
impl !Send for NonTrivialStructWithConstructors {}
impl !Sync for NonTrivialStructWithConstructors {}
unsafe impl ::cxx::ExternType for NonTrivialStructWithConstructors {
    type Id = ::cxx::type_id!("NonTrivialStructWithConstructors");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NonTrivialStructWithConstructors"),
    crate::NonTrivialStructWithConstructors
);

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=99
impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for NonTrivialStructWithConstructors {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN32NonTrivialStructWithConstructorsC1ERKS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for NonTrivialStructWithConstructors {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=99
impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for NonTrivialStructWithConstructors {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN32NonTrivialStructWithConstructorsaSERKS_(
                self, __param_0,
            );
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=100
impl ::ctor::CtorNew<()> for NonTrivialStructWithConstructors {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN32NonTrivialStructWithConstructorsC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=101
impl ::ctor::CtorNew<::ffi_11::c_int> for NonTrivialStructWithConstructors {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN32NonTrivialStructWithConstructorsC1Ei(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for NonTrivialStructWithConstructors {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

/// Presence of a user-defined destructor makes this struct non-trivial.
///
/// Generated from: rs_bindings_from_cc/test/struct/constructors/constructors.h;l=104
impl ::ctor::PinnedDrop for NonTrivialStructWithConstructors {
    #[inline(always)]
    unsafe fn pinned_drop(self: ::core::pin::Pin<&mut Self>) {
        crate::detail::__rust_thunk___ZN32NonTrivialStructWithConstructorsD1Ev(self)
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN34StructWithUserProvidedConstructorsaSERKS_<
            '__param_0,
        >(
            __this: &mut crate::StructWithUserProvidedConstructors,
            __param_0: &'__param_0 crate::StructWithUserProvidedConstructors,
        ) -> &'__param_0 mut crate::StructWithUserProvidedConstructors;
        #[link_name = "_ZN34StructWithUserProvidedConstructorsC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN34StructWithUserProvidedConstructorsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN34StructWithUserProvidedConstructorsC1ERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN34StructWithUserProvidedConstructorsC1ERKS_<
            '__param_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::StructWithUserProvidedConstructors,
        );
        pub(crate) unsafe fn __rust_thunk___ZN39StructWithExplicitConversionConstructorC1Ei(
            __this: *mut ::core::ffi::c_void,
            i: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30StructWithMultipleConstructorsC1Ei(
            __this: *mut ::core::ffi::c_void,
            i: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30StructWithMultipleConstructorsC1Eii(
            __this: *mut ::core::ffi::c_void,
            i: ::ffi_11::c_int,
            j: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30StructWithMultipleConstructorsC1Eiii(
            __this: *mut ::core::ffi::c_void,
            i: ::ffi_11::c_int,
            j: ::ffi_11::c_int,
            k: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN39StructWithImplicitConversionConstructorC1Ei(
            __this: *mut ::core::ffi::c_void,
            i: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17OtherSimpleStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN41StructWithImplicitConversionFromReferenceC1ERK17OtherSimpleStruct<
            'other,
        >(
            __this: *mut ::core::ffi::c_void,
            other: &'other crate::OtherSimpleStruct,
        );
        pub(crate) unsafe fn __rust_thunk___ZN28StructWithInlineConstructorsaSERKS_<'__param_0>(
            __this: &mut crate::StructWithInlineConstructors,
            __param_0: &'__param_0 crate::StructWithInlineConstructors,
        ) -> &'__param_0 mut crate::StructWithInlineConstructors;
        pub(crate) unsafe fn __rust_thunk___ZN28StructWithInlineConstructorsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN28StructWithInlineConstructorsC1ERKS_<'other>(
            __this: *mut ::core::ffi::c_void,
            other: &'other crate::StructWithInlineConstructors,
        );
        pub(crate) unsafe fn __rust_thunk___ZN28StructWithInlineConstructorsC1Ei(
            __this: *mut ::core::ffi::c_void,
            i: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN29StructWithDeletedConstructorsaSERKS_<'__param_0>(
            __this: ::core::pin::Pin<&mut crate::StructWithDeletedConstructors>,
            __param_0: &'__param_0 crate::StructWithDeletedConstructors,
        ) -> ::core::pin::Pin<&'__param_0 mut crate::StructWithDeletedConstructors>;
        pub(crate) unsafe fn __rust_thunk___ZN29StructWithPrivateConstructorsaSERKS_<'__param_0>(
            __this: ::core::pin::Pin<&mut crate::StructWithPrivateConstructors>,
            __param_0: &'__param_0 crate::StructWithPrivateConstructors,
        ) -> ::core::pin::Pin<&'__param_0 mut crate::StructWithPrivateConstructors>;
        pub(crate) unsafe fn __rust_thunk___ZN41StructWithExplicitlyDefaultedConstructorsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN32NonTrivialStructWithConstructorsC1ERKS_<
            '__param_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::NonTrivialStructWithConstructors,
        );
        pub(crate) unsafe fn __rust_thunk___ZN32NonTrivialStructWithConstructorsaSERKS_<
            '__param_0,
        >(
            __this: ::core::pin::Pin<&mut crate::NonTrivialStructWithConstructors>,
            __param_0: &'__param_0 crate::NonTrivialStructWithConstructors,
        ) -> ::core::pin::Pin<&'__param_0 mut crate::NonTrivialStructWithConstructors>;
        #[link_name = "_ZN32NonTrivialStructWithConstructorsC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN32NonTrivialStructWithConstructorsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN32NonTrivialStructWithConstructorsC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN32NonTrivialStructWithConstructorsC1Ei(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ffi_11::c_int,
        );
        #[link_name = "_ZN32NonTrivialStructWithConstructorsD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN32NonTrivialStructWithConstructorsD1Ev(
            __this: ::core::pin::Pin<&mut crate::NonTrivialStructWithConstructors>,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::StructWithUserProvidedConstructors>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithUserProvidedConstructors>() == 4);
    static_assertions::assert_not_impl_any!(crate::StructWithUserProvidedConstructors: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::StructWithUserProvidedConstructors, int_field) == 0);
    assert!(::core::mem::size_of::<crate::StructWithExplicitConversionConstructor>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithExplicitConversionConstructor>() == 4);
    static_assertions::assert_impl_all!(crate::StructWithExplicitConversionConstructor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithExplicitConversionConstructor: Drop);
    assert!(
        ::core::mem::offset_of!(crate::StructWithExplicitConversionConstructor, int_field) == 0
    );
    assert!(::core::mem::size_of::<crate::StructWithMultipleConstructors>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithMultipleConstructors>() == 4);
    static_assertions::assert_impl_all!(crate::StructWithMultipleConstructors: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithMultipleConstructors: Drop);
    assert!(::core::mem::offset_of!(crate::StructWithMultipleConstructors, int_field) == 0);
    assert!(::core::mem::size_of::<crate::StructWithImplicitConversionConstructor>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithImplicitConversionConstructor>() == 4);
    static_assertions::assert_impl_all!(crate::StructWithImplicitConversionConstructor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithImplicitConversionConstructor: Drop);
    assert!(
        ::core::mem::offset_of!(crate::StructWithImplicitConversionConstructor, int_field) == 0
    );
    assert!(::core::mem::size_of::<crate::OtherSimpleStruct>() == 4);
    assert!(::core::mem::align_of::<crate::OtherSimpleStruct>() == 4);
    static_assertions::assert_impl_all!(crate::OtherSimpleStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::OtherSimpleStruct: Drop);
    assert!(::core::mem::offset_of!(crate::OtherSimpleStruct, int_field) == 0);
    assert!(::core::mem::size_of::<crate::StructWithImplicitConversionFromReference>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithImplicitConversionFromReference>() == 4);
    static_assertions::assert_impl_all!(crate::StructWithImplicitConversionFromReference: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithImplicitConversionFromReference: Drop);
    assert!(
        ::core::mem::offset_of!(crate::StructWithImplicitConversionFromReference, int_field) == 0
    );
    assert!(::core::mem::size_of::<crate::StructWithInlineConstructors>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithInlineConstructors>() == 4);
    static_assertions::assert_not_impl_any!(crate::StructWithInlineConstructors: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::StructWithInlineConstructors, int_field) == 0);
    assert!(::core::mem::size_of::<crate::StructWithDeletedConstructors>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithDeletedConstructors>() == 4);
    static_assertions::assert_not_impl_any!(crate::StructWithDeletedConstructors: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::StructWithDeletedConstructors, int_field) == 0);
    assert!(::core::mem::size_of::<crate::StructWithPrivateConstructors>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithPrivateConstructors>() == 4);
    static_assertions::assert_not_impl_any!(crate::StructWithPrivateConstructors: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::StructWithPrivateConstructors, int_field) == 0);
    assert!(::core::mem::size_of::<crate::StructWithExplicitlyDefaultedConstructors>() == 8);
    assert!(::core::mem::align_of::<crate::StructWithExplicitlyDefaultedConstructors>() == 4);
    static_assertions::assert_impl_all!(crate::StructWithExplicitlyDefaultedConstructors: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithExplicitlyDefaultedConstructors: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::StructWithExplicitlyDefaultedConstructors,
            field_with_explicit_initializer
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::StructWithExplicitlyDefaultedConstructors,
            field_with_no_initializer
        ) == 4
    );
    assert!(::core::mem::size_of::<crate::NonTrivialStructWithConstructors>() == 4);
    assert!(::core::mem::align_of::<crate::NonTrivialStructWithConstructors>() == 4);
    static_assertions::assert_impl_all!(crate::NonTrivialStructWithConstructors: Drop);
    static_assertions::assert_not_impl_any!(crate::NonTrivialStructWithConstructors: Copy);
    assert!(::core::mem::offset_of!(crate::NonTrivialStructWithConstructors, int_field) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
};
