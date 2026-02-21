// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:owned_ptr
// Features: supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// An example of a C++ struct that supports ownership via the raw pointer.
///
/// The CRUBIT_OWNED_PTR_TYPE annotation is used to specify the Rust type that
/// will be used to represent the C++ struct when it is used in a position
/// annotated with CRUBIT_OWNED_PTR.
///
/// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=17
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Thing
pub struct RawThing {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub value: i32,
}
impl !Send for RawThing {}
impl !Sync for RawThing {}
unsafe impl ::cxx::ExternType for RawThing {
    type Id = ::cxx::type_id!("Thing");
    type Kind = ::cxx::kind::Trivial;
}
impl RawThing {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    ///
    /// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=21
    #[inline(always)]
    pub unsafe fn Close(__this: *mut Self) {
        crate::detail::__rust_thunk___ZN5Thing5CloseEv(__this)
    }
}

// Generated due to CRUBIT_OWNED_POINTEE annotation.
///Wrapper for a C++ RawThing owned by Rust.
///
/// Style guide: The C++ type to which this refers should be wrapped in an `Arc` or `Mutex` if it is not already thread-safe.
///
/// THIS TYPE REQUIRES A MANUAL DROP IMPLEMENTATION.
/// You MUST provide an `impl OwnedThing { pub fn DropImpl(&mut self) { /*...*/ } }` block in a separate Rust file (e.g., via `additional_rust_srcs`). Failure to do so will result in a compile-time error: `method not found in `OwnedThing``.
#[repr(transparent)]
pub struct OwnedThing(::core::ptr::NonNull<RawThing>);
impl Drop for OwnedThing {
    fn drop(&mut self) {
        // IMPORTANT: The DropImpl method for `{}` MUST be implemented in a user-written .rs file (e.g., using `additional_rust_srcs`).
        // Crubit cannot automatically generate the destruction logic for this type.
        // See the struct documentation for more details.
        self.DropImpl();
    }
}

/// Generated from: rs_bindings_from_cc/test/annotations/owned_ptr.h;l=18
impl From<i32> for RawThing {
    #[inline(always)]
    fn from(args: i32) -> Self {
        let mut value = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5ThingC1Ei(&raw mut tmp as *mut _, value);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<i32> for RawThing {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        <Self as From<i32>>::from(args)
    }
}

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// error: struct `std::integral_constant<bool, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// error: struct `std::integral_constant<bool, true>` could not be bound
//   template instantiation is not yet supported

#[path = "rs_bindings_from_cc/test/annotations/owned_ptr_rust_thing.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN5ThingC1Ei(
            __this: *mut ::core::ffi::c_void,
            value: i32,
        );
        pub(crate) unsafe fn __rust_thunk___ZN5Thing5CloseEv(__this: *mut crate::RawThing);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::RawThing>() == 4);
    assert!(::core::mem::align_of::<crate::RawThing>() == 4);
    static_assertions::assert_impl_all!(crate::RawThing: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::RawThing: Drop);
    assert!(::core::mem::offset_of!(crate::RawThing, value) == 0);
};
