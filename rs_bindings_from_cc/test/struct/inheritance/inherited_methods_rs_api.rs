// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/inheritance:inherited_methods
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

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

/// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=10
#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Nonmovable
pub struct Nonmovable {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Nonmovable {}
impl !Sync for Nonmovable {}
unsafe impl ::cxx::ExternType for Nonmovable {
    type Id = ::cxx::type_id!("Nonmovable");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nonmovable"), crate::Nonmovable);

/// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=11
impl ::ctor::CtorNew<()> for Nonmovable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NonmovableC1Ev(dest as *mut ::core::ffi::c_void);
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=16
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Base
pub struct Base {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Base {}
impl !Sync for Base {}
unsafe impl ::cxx::ExternType for Base {
    type Id = ::cxx::type_id!("Base");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base"), crate::Base);
impl Base {
    /// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=17
    #[inline(always)]
    pub fn has_bindings<'a>(&'a self) -> bool {
        unsafe { crate::detail::__rust_thunk___ZNK4Base12has_bindingsEv(self) }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=19
    #[inline(always)]
    pub fn no_bindings<'error, 'a>(
        &'a self,
        __param_0: impl ::ctor::Ctor<Output = crate::Nonmovable, Error = ::ctor::Infallible>,
    ) where
        &'error (): BindingFailedFor_ZNK4Base11no_bindingsE10Nonmovable,
    {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=16
impl Default for Base {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4BaseC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nNon-movable, non-trivial_abi type 'crate::Nonmovable' is not supported by value as parameter #1"
)]
pub trait BindingFailedFor_ZNK4Base11no_bindingsE10Nonmovable {}

/// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=22
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Derived
pub struct Derived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Derived {}
impl !Sync for Derived {}
unsafe impl ::cxx::ExternType for Derived {
    type Id = ::cxx::type_id!("Derived");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Derived"), crate::Derived);
impl Derived {
    /// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=17
    #[inline(always)]
    pub fn has_bindings<'a>(&'a self) -> bool {
        unsafe {
            crate::detail::__rust_thunk___ZNK4Base12has_bindingsEv(oops::Upcast::<_>::upcast(self))
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=19
    #[inline(always)]
    pub fn no_bindings<'error, 'a>(
        &'a self,
        __param_0: impl ::ctor::Ctor<Output = crate::Nonmovable, Error = ::ctor::Infallible>,
    ) where
        &'error (): BindingFailedFor_7Derived__ZNK4Base11no_bindingsE10Nonmovable,
    {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/inheritance/inherited_methods.h;l=22
impl Default for Derived {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7DerivedC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nNon-movable, non-trivial_abi type 'crate::Nonmovable' is not supported by value as parameter #1"
)]
pub trait BindingFailedFor_7Derived__ZNK4Base11no_bindingsE10Nonmovable {}

unsafe impl oops::Inherits<crate::Base> for crate::Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base {
        (derived as *const _ as *const u8).offset(0) as *const crate::Base
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10NonmovableC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN4BaseC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZNK4Base12has_bindingsEv<'a>(
            __this: &'a crate::Base,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN7DerivedC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Nonmovable>() == 1);
    assert!(::core::mem::align_of::<crate::Nonmovable>() == 1);
    static_assertions::assert_not_impl_any!(crate::Nonmovable: Copy,Drop);

    assert!(::core::mem::size_of::<crate::Base>() == 1);
    assert!(::core::mem::align_of::<crate::Base>() == 1);
    static_assertions::assert_impl_all!(crate::Base: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Base: Drop);

    assert!(::core::mem::size_of::<crate::Derived>() == 1);
    assert!(::core::mem::align_of::<crate::Derived>() == 1);
    static_assertions::assert_impl_all!(crate::Derived: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Derived: Drop);
};
