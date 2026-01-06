// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/references:references
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/references/references.h;l=8
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypeWithPtrConstructor
pub struct TypeWithPtrConstructor {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TypeWithPtrConstructor {}
impl !Sync for TypeWithPtrConstructor {}
unsafe impl ::cxx::ExternType for TypeWithPtrConstructor {
    type Id = ::cxx::type_id!("TypeWithPtrConstructor");
    type Kind = ::cxx::kind::Trivial;
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nConstructors cannot be `unsafe`, but this constructor accepts:\n    `ptr` of unsafe type `*mut::ffi_11::c_int`"
)]
pub trait BindingFailedFor_ZN22TypeWithPtrConstructorC1EPi {}
/// Generated from: rs_bindings_from_cc/test/references/references.h;l=10
impl<'error> From<*mut ::ffi_11::c_int> for TypeWithPtrConstructor
where
    &'error (): BindingFailedFor_ZN22TypeWithPtrConstructorC1EPi,
{
    #[inline(always)]
    fn from(ptr: *mut ::ffi_11::c_int) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

/// Generated from: rs_bindings_from_cc/test/references/references.h;l=13
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypeWithNonNullPtrConstructor
pub struct TypeWithNonNullPtrConstructor {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TypeWithNonNullPtrConstructor {}
impl !Sync for TypeWithNonNullPtrConstructor {}
unsafe impl ::cxx::ExternType for TypeWithNonNullPtrConstructor {
    type Id = ::cxx::type_id!("TypeWithNonNullPtrConstructor");
    type Kind = ::cxx::kind::Trivial;
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nConstructors cannot be `unsafe`, but this constructor accepts:\n    `ptr` of unsafe type `*mut::ffi_11::c_int`"
)]
pub trait BindingFailedFor_ZN29TypeWithNonNullPtrConstructorC1EPi {}
/// Generated from: rs_bindings_from_cc/test/references/references.h;l=15
impl<'error> From<*mut ::ffi_11::c_int> for TypeWithNonNullPtrConstructor
where
    &'error (): BindingFailedFor_ZN29TypeWithNonNullPtrConstructorC1EPi,
{
    #[inline(always)]
    fn from(ptr: *mut ::ffi_11::c_int) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

/// Generated from: rs_bindings_from_cc/test/references/references.h;l=18
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypeWithReferenceConstructor
pub struct TypeWithReferenceConstructor {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TypeWithReferenceConstructor {}
impl !Sync for TypeWithReferenceConstructor {}
unsafe impl ::cxx::ExternType for TypeWithReferenceConstructor {
    type Id = ::cxx::type_id!("TypeWithReferenceConstructor");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: rs_bindings_from_cc/test/references/references.h;l=20
impl From<&mut ::ffi_11::c_int> for TypeWithReferenceConstructor {
    #[inline(always)]
    fn from(r#ref: &mut ::ffi_11::c_int) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN28TypeWithReferenceConstructorC1ERi(
                &raw mut tmp as *mut _,
                r#ref,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<&mut ::ffi_11::c_int> for TypeWithReferenceConstructor {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &mut ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<&mut ::ffi_11::c_int>>::from(args)
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN28TypeWithReferenceConstructorC1ERi(
            __this: *mut ::core::ffi::c_void,
            r#ref: &mut ::ffi_11::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::TypeWithPtrConstructor>() == 1);
    assert!(::core::mem::align_of::<crate::TypeWithPtrConstructor>() == 1);
    static_assertions::assert_impl_all!(crate::TypeWithPtrConstructor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TypeWithPtrConstructor: Drop);

    assert!(::core::mem::size_of::<crate::TypeWithNonNullPtrConstructor>() == 1);
    assert!(::core::mem::align_of::<crate::TypeWithNonNullPtrConstructor>() == 1);
    static_assertions::assert_impl_all!(crate::TypeWithNonNullPtrConstructor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TypeWithNonNullPtrConstructor: Drop);

    assert!(::core::mem::size_of::<crate::TypeWithReferenceConstructor>() == 1);
    assert!(::core::mem::align_of::<crate::TypeWithReferenceConstructor>() == 1);
    static_assertions::assert_impl_all!(crate::TypeWithReferenceConstructor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TypeWithReferenceConstructor: Drop);
};
