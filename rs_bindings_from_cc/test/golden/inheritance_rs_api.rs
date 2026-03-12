// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:inheritance_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// Using classes to force these to be non-POD.
/// In the Itanium ABI, the tail padding of POD types cannot be reused by other
/// objects, even if the POD type is potentially-overlapping.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Base0
pub struct Base0 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Base0 {}
impl !Sync for Base0 {}
unsafe impl ::cxx::ExternType for Base0 {
    type Id = ::cxx::type_id!("Base0");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Base0 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base0C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: constructor `Base0::Base0` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `Base0::Base0` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `Base0::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `Base0::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Base1
pub struct Base1 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_1_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_2_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for Base1 {}
impl !Sync for Base1 {}
unsafe impl ::cxx::ExternType for Base1 {
    type Id = ::cxx::type_id!("Base1");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Base1 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base1C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: constructor `Base1::Base1` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `Base1::Base1` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `Base1::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `Base1::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(2))]
///CRUBIT_ANNOTATE: cpp_type=Base2
pub struct Base2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b2_1_: [::core::mem::MaybeUninit<u8>; 2],
}
impl !Send for Base2 {}
impl !Sync for Base2 {}
unsafe impl ::cxx::ExternType for Base2 {
    type Id = ::cxx::type_id!("Base2");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Base2 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base2C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: constructor `Base2::Base2` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `Base2::Base2` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `Base2::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `Base2::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Derived
pub struct Derived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 12],
    pub derived_1: ::ffi_11::c_char,
}
impl !Send for Derived {}
impl !Sync for Derived {}
unsafe impl ::cxx::ExternType for Derived {
    type Id = ::cxx::type_id!("Derived");
    type Kind = ::cxx::kind::Trivial;
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

// error: constructor `Derived::Derived` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `Derived::Derived` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `Derived::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `Derived::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=VirtualBase1
pub struct VirtualBase1 {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
}
impl !Send for VirtualBase1 {}
impl !Sync for VirtualBase1 {}
unsafe impl ::cxx::ExternType for VirtualBase1 {
    type Id = ::cxx::type_id!("VirtualBase1");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for VirtualBase1 {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN12VirtualBase1C1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// error: constructor `VirtualBase1::VirtualBase1` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `VirtualBase1::VirtualBase1` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `VirtualBase1::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `VirtualBase1::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=VirtualBase2
pub struct VirtualBase2 {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
}
impl !Send for VirtualBase2 {}
impl !Sync for VirtualBase2 {}
unsafe impl ::cxx::ExternType for VirtualBase2 {
    type Id = ::cxx::type_id!("VirtualBase2");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for VirtualBase2 {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN12VirtualBase2C1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// error: constructor `VirtualBase2::VirtualBase2` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `VirtualBase2::VirtualBase2` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `VirtualBase2::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `VirtualBase2::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=VirtualDerived
pub struct VirtualDerived {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 32],
}
impl !Send for VirtualDerived {}
impl !Sync for VirtualDerived {}
unsafe impl ::cxx::ExternType for VirtualDerived {
    type Id = ::cxx::type_id!("VirtualDerived");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for VirtualDerived {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN14VirtualDerivedC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// error: constructor `VirtualDerived::VirtualDerived` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `VirtualDerived::VirtualDerived` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `VirtualDerived::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `VirtualDerived::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=MyAbstractClass
pub struct MyAbstractClass {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for MyAbstractClass {}
impl !Sync for MyAbstractClass {}
unsafe impl ::cxx::ExternType for MyAbstractClass {
    type Id = ::cxx::type_id!("MyAbstractClass");
    type Kind = ::cxx::kind::Opaque;
}

// error: constructor `MyAbstractClass::MyAbstractClass` could not be bound
//   Can't directly construct values of type `MyAbstractClass` as it has a non-public or deleted destructor

// error: constructor `MyAbstractClass::MyAbstractClass` could not be bound
//   Can't directly construct values of type `MyAbstractClass` as it has a non-public or deleted destructor

// error: function `MyAbstractClass::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

/// Method inheritance
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MethodBase1
pub struct MethodBase1 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for MethodBase1 {}
impl !Sync for MethodBase1 {}
unsafe impl ::cxx::ExternType for MethodBase1 {
    type Id = ::cxx::type_id!("MethodBase1");
    type Kind = ::cxx::kind::Trivial;
}
impl MethodBase1 {
    #[inline(always)]
    pub fn Public<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase16PublicEv(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__param_0`: raw pointer
    #[inline(always)]
    pub unsafe fn Equals<'a>(&'a mut self, __param_0: *const Self) {
        crate::detail::__rust_thunk___ZN11MethodBase16EqualsEPKS_(self, __param_0)
    }
    #[inline(always)]
    pub fn Colliding1<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase110Colliding1Ev(self) }
    }
    #[inline(always)]
    pub fn Colliding2<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase110Colliding2Ev(self) }
    }
}

impl Default for MethodBase1 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase1C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: constructor `MethodBase1::MethodBase1` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `MethodBase1::MethodBase1` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MethodBase1::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MethodBase1::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MethodBase2
pub struct MethodBase2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for MethodBase2 {}
impl !Sync for MethodBase2 {}
unsafe impl ::cxx::ExternType for MethodBase2 {
    type Id = ::cxx::type_id!("MethodBase2");
    type Kind = ::cxx::kind::Trivial;
}
impl MethodBase2 {
    #[inline(always)]
    pub fn Colliding1<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase210Colliding1Ev(self) }
    }
    #[inline(always)]
    pub fn Colliding2<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase210Colliding2Ev(self) }
    }
}

impl Default for MethodBase2 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase2C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: constructor `MethodBase2::MethodBase2` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `MethodBase2::MethodBase2` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MethodBase2::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MethodBase2::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MethodDerived
pub struct MethodDerived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for MethodDerived {}
impl !Sync for MethodDerived {}
unsafe impl ::cxx::ExternType for MethodDerived {
    type Id = ::cxx::type_id!("MethodDerived");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for MethodDerived {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13MethodDerivedC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: constructor `MethodDerived::MethodDerived` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: constructor `MethodDerived::MethodDerived` could not be bound
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MethodDerived::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MethodDerived::operator=` could not be bound
//   Unsupported return type: references are not yet supported
//   Unsupported parameter #1 (__param_0): references are not yet supported

// error: function `MethodDerived::Colliding1` could not be bound
//   Function aliases are not yet supported.

// error: function `MethodDerived::Protected1` could not be bound
//   Function aliases are not yet supported.

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN5Base0C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5Base1C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5Base2C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN7DerivedC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase1C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN12VirtualBase2C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN14VirtualDerivedC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase1C1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN11MethodBase16PublicEv"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase16PublicEv<'a>(
            __this: &'a mut crate::MethodBase1,
        );
        #[link_name = "_ZN11MethodBase16EqualsEPKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase16EqualsEPKS_<'a>(
            __this: &'a mut crate::MethodBase1,
            __param_0: *const crate::MethodBase1,
        );
        #[link_name = "_ZN11MethodBase110Colliding1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase110Colliding1Ev<'a>(
            __this: &'a mut crate::MethodBase1,
        );
        #[link_name = "_ZN11MethodBase110Colliding2Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase110Colliding2Ev<'a>(
            __this: &'a mut crate::MethodBase1,
        );
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase2C1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN11MethodBase210Colliding1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase210Colliding1Ev<'a>(
            __this: &'a mut crate::MethodBase2,
        );
        #[link_name = "_ZN11MethodBase210Colliding2Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN11MethodBase210Colliding2Ev<'a>(
            __this: &'a mut crate::MethodBase2,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13MethodDerivedC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Base0>() == 1);
    assert!(::core::mem::align_of::<crate::Base0>() == 1);
    static_assertions::assert_impl_all!(crate::Base0: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Base0: Drop);

    assert!(::core::mem::size_of::<crate::Base1>() == 16);
    assert!(::core::mem::align_of::<crate::Base1>() == 8);
    static_assertions::assert_impl_all!(crate::Base1: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Base1: Drop);
    assert!(::core::mem::offset_of!(crate::Base1, b1_1_) == 0);
    assert!(::core::mem::offset_of!(crate::Base1, b1_2_) == 8);
    assert!(::core::mem::size_of::<crate::Base2>() == 2);
    assert!(::core::mem::align_of::<crate::Base2>() == 2);
    static_assertions::assert_impl_all!(crate::Base2: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Base2: Drop);
    assert!(::core::mem::offset_of!(crate::Base2, b2_1_) == 0);
    assert!(::core::mem::size_of::<crate::Derived>() == 16);
    assert!(::core::mem::align_of::<crate::Derived>() == 8);
    static_assertions::assert_impl_all!(crate::Derived: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Derived: Drop);
    assert!(::core::mem::offset_of!(crate::Derived, derived_1) == 12);
    assert!(::core::mem::size_of::<crate::VirtualBase1>() == 24);
    assert!(::core::mem::align_of::<crate::VirtualBase1>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualBase1: Copy,Drop);

    assert!(::core::mem::size_of::<crate::VirtualBase2>() == 24);
    assert!(::core::mem::align_of::<crate::VirtualBase2>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualBase2: Copy,Drop);

    assert!(::core::mem::size_of::<crate::VirtualDerived>() == 32);
    assert!(::core::mem::align_of::<crate::VirtualDerived>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualDerived: Copy,Drop);

    assert!(::core::mem::size_of::<crate::MyAbstractClass>() == 8);
    assert!(::core::mem::align_of::<crate::MyAbstractClass>() == 8);
    static_assertions::assert_not_impl_any!(crate::MyAbstractClass: Copy,Drop);

    assert!(::core::mem::size_of::<crate::MethodBase1>() == 1);
    assert!(::core::mem::align_of::<crate::MethodBase1>() == 1);
    static_assertions::assert_impl_all!(crate::MethodBase1: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MethodBase1: Drop);

    assert!(::core::mem::size_of::<crate::MethodBase2>() == 1);
    assert!(::core::mem::align_of::<crate::MethodBase2>() == 1);
    static_assertions::assert_impl_all!(crate::MethodBase2: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MethodBase2: Drop);

    assert!(::core::mem::size_of::<crate::MethodDerived>() == 1);
    assert!(::core::mem::align_of::<crate::MethodDerived>() == 1);
    static_assertions::assert_impl_all!(crate::MethodDerived: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MethodDerived: Drop);
};
