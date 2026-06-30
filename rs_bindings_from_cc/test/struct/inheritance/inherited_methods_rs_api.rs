// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/inheritance:inherited_methods

#![rustfmt::skip]
#![feature(custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: Nonmovable
pub struct Nonmovable {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for Nonmovable {}
impl !Sync for Nonmovable {}
unsafe impl ::cxx::ExternType for Nonmovable {
    type Id = ::cxx::type_id!(":: Nonmovable");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: Nonmovable"), crate::Nonmovable);

impl ::ctor::CtorNew<()> for Nonmovable {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NonmovableC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: Base
pub struct Base {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Base {}
impl !Sync for Base {}
unsafe impl ::cxx::ExternType for Base {
    type Id = ::cxx::type_id!(":: Base");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: Base"), crate::Base);
impl Base {
    #[inline(always)]
    pub fn has_bindings<'__this>(&'__this self) -> bool {
        unsafe { self::base::has_bindings(self) }
    }
    #[inline(always)]
    pub fn no_bindings<'__this>(&'__this self, __param_0: ::ctor::Ctor![crate::Nonmovable])
    where
        for<'error> &'error (): BindingFailedFor_ZNK4Base11no_bindingsE10Nonmovable,
    {
        unsafe { self::base::no_bindings(self, __param_0) }
    }
}

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

pub mod base {
    #[inline(always)]
    pub(crate) fn has_bindings<'__this>(__this: &'__this crate::Base) -> bool {
        unsafe { crate::detail::__rust_thunk___ZNK4Base12has_bindingsEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn no_bindings<'__this>(
        __this: &'__this crate::Base,
        __param_0: ::ctor::Ctor![crate::Nonmovable],
    ) {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: Derived
pub struct Derived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Derived {}
impl !Sync for Derived {}
unsafe impl ::cxx::ExternType for Derived {
    type Id = ::cxx::type_id!(":: Derived");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: Derived"), crate::Derived);
impl Derived {
    #[inline(always)]
    pub fn has_bindings<'__this>(&'__this self) -> bool {
        unsafe { self::derived::has_bindings(oops::Upcast::<_>::upcast(self)) }
    }
    #[inline(always)]
    pub fn no_bindings<'__this>(&'__this self, __param_0: ::ctor::Ctor![crate::Nonmovable])
    where
        for<'error> &'error (): BindingFailedFor_7Derived__ZNK4Base11no_bindingsE10Nonmovable,
    {
        unsafe { self::derived::no_bindings(oops::Upcast::<_>::upcast(self), __param_0) }
    }
}

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
        unsafe { (derived as *const _ as *const u8).offset(0) as *const crate::Base }
    }
}

pub mod derived {
    #[inline(always)]
    pub(crate) fn has_bindings<'__this>(__this: &'__this crate::Base) -> bool {
        unsafe { crate::detail::__rust_thunk___ZNK4Base12has_bindingsEv(__this) }
    }
    #[inline(always)]
    pub(crate) fn no_bindings<'__this>(
        __this: &'__this crate::Base,
        __param_0: ::ctor::Ctor![crate::Nonmovable],
    ) {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10NonmovableC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN4BaseC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZNK4Base12has_bindingsEv<'__this>(
            __this: &'__this crate::Base,
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
