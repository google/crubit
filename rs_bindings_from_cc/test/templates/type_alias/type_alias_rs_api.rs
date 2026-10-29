// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/type_alias:type_alias

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// error: class `MyTemplate` could not be bound
//   Class templates are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIiE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct MyTypeAlias {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for MyTypeAlias {}
impl !Sync for MyTypeAlias {}
forward_declare::unsafe_define!(forward_declare::symbol!("MyTemplate < int >"), crate::MyTypeAlias);
impl MyTypeAlias {
    #[inline(always)]
    pub fn Create(value: ::ffi_11::c_int) -> crate::MyTypeAlias {
        unsafe { self::my_type_alias::Create(value) }
    }
    #[inline(always)]
    pub fn value<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { self::my_type_alias::value(self) }
    }
}

impl Default for MyTypeAlias {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__de4d35d3__ZN10MyTemplateIiEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod my_type_alias {
    #[inline(always)]
    pub(crate) fn Create(value: ::ffi_11::c_int) -> crate::MyTypeAlias {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::MyTypeAlias>::uninit();
            crate::detail::__rust_thunk__5a9d55c6__ZN10MyTemplateIiE6CreateEi(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                value,
            );
            __crubit_return.assume_init()
        }
    }
    #[inline(always)]
    pub(crate) fn value<'__this>(
        __this: &'__this crate::MyTypeAlias,
    ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { crate::detail::__rust_thunk__758630aa__ZNK10MyTemplateIiE5valueEv(__this) }
    }
}

pub type OtherTypeAliasInSameTarget = crate::MyTypeAlias;

pub mod nested {
    /// The struct for `MyTemplate<float>` is generated as `nested::NestedAlias`,
    /// with this comment as its documentation.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInst10MyTemplateIfE"]
    #[repr(C, align(4))]
    ///CRUBIT_ANNOTATE: cpp_type=MyTemplate < float >
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct NestedAlias {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
    }
    impl !Send for NestedAlias {}
    impl !Sync for NestedAlias {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("MyTemplate < float >"),
        crate::nested::NestedAlias
    );
    impl NestedAlias {
        #[inline(always)]
        pub fn Create(value: f32) -> crate::nested::NestedAlias {
            unsafe { self::nested_alias::Create(value) }
        }
        #[inline(always)]
        pub fn value<'__this>(&'__this self) -> ::cref::CRef<'__this, f32> {
            unsafe { self::nested_alias::value(self) }
        }
    }

    impl Default for NestedAlias {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__de4d35d3__ZN10MyTemplateIfEC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    pub mod nested_alias {
        #[inline(always)]
        pub(crate) fn Create(value: f32) -> crate::nested::NestedAlias {
            unsafe {
                let mut __crubit_return =
                    ::core::mem::MaybeUninit::<crate::nested::NestedAlias>::uninit();
                crate::detail::__rust_thunk__5a9d55c6__ZN10MyTemplateIfE6CreateEf(
                    &raw mut __crubit_return as *mut ::core::ffi::c_void,
                    value,
                );
                __crubit_return.assume_init()
            }
        }
        #[inline(always)]
        pub(crate) fn value<'__this>(
            __this: &'__this crate::nested::NestedAlias,
        ) -> ::cref::CRef<'__this, f32> {
            unsafe { crate::detail::__rust_thunk__758630aa__ZNK10MyTemplateIfE5valueEv(__this) }
        }
    }
}

// namespace nested

#[inline(always)]
pub fn GetValue(s: crate::MyTypeAlias) -> ::ffi_11::c_int {
    let mut s = ::core::mem::MaybeUninit::new(s);
    unsafe { crate::detail::__rust_thunk___Z8GetValue10MyTemplateIiE(s.as_mut_ptr()) }
}

#[inline(always)]
pub fn GetValueByRef<'s>(s: &'s crate::MyTypeAlias) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z13GetValueByRefRK10MyTemplateIiE(s) }
}

/// `MyTemplate<double>` isn't named by any alias, so it keeps its mangled name.
#[inline(always)]
pub fn MakeUnaliased(value: f64) -> crate::__CcTemplateInst10MyTemplateIdE {
    unsafe {
        let mut __crubit_return =
            ::core::mem::MaybeUninit::<crate::__CcTemplateInst10MyTemplateIdE>::uninit();
        crate::detail::__rust_thunk___Z13MakeUnaliasedd(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
            value,
        );
        __crubit_return.assume_init()
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10MyTemplateIdE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < double >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst10MyTemplateIdE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInst10MyTemplateIdE {}
impl !Sync for __CcTemplateInst10MyTemplateIdE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < double >"),
    crate::__CcTemplateInst10MyTemplateIdE
);
impl __CcTemplateInst10MyTemplateIdE {
    #[inline(always)]
    pub fn Create(value: f64) -> crate::__CcTemplateInst10MyTemplateIdE {
        unsafe { self::cc_template_inst10_my_template_id_e::Create(value) }
    }
    #[inline(always)]
    pub fn value<'__this>(&'__this self) -> ::cref::CRef<'__this, f64> {
        unsafe { self::cc_template_inst10_my_template_id_e::value(self) }
    }
}

impl Default for __CcTemplateInst10MyTemplateIdE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__de4d35d3__ZN10MyTemplateIdEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst10_my_template_id_e {
    #[inline(always)]
    pub(crate) fn Create(value: f64) -> crate::__CcTemplateInst10MyTemplateIdE {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<crate::__CcTemplateInst10MyTemplateIdE>::uninit();
            crate::detail::__rust_thunk__5a9d55c6__ZN10MyTemplateIdE6CreateEd(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                value,
            );
            __crubit_return.assume_init()
        }
    }
    #[inline(always)]
    pub(crate) fn value<'__this>(
        __this: &'__this crate::__CcTemplateInst10MyTemplateIdE,
    ) -> ::cref::CRef<'__this, f64> {
        unsafe { crate::detail::__rust_thunk__758630aa__ZNK10MyTemplateIdE5valueEv(__this) }
    }
}

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIiE = crate::MyTypeAlias;

#[doc(hidden)]
pub type __CcTemplateInst10MyTemplateIfE = crate::nested::NestedAlias;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk__de4d35d3__ZN10MyTemplateIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__5a9d55c6__ZN10MyTemplateIiE6CreateEi(
            __return: *mut ::core::ffi::c_void,
            value: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk__758630aa__ZNK10MyTemplateIiE5valueEv<'__this>(
            __this: &'__this crate::MyTypeAlias,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__de4d35d3__ZN10MyTemplateIfEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__5a9d55c6__ZN10MyTemplateIfE6CreateEf(
            __return: *mut ::core::ffi::c_void,
            value: f32,
        );
        pub(crate) unsafe fn __rust_thunk__758630aa__ZNK10MyTemplateIfE5valueEv<'__this>(
            __this: &'__this crate::nested::NestedAlias,
        ) -> ::cref::CRef<'__this, f32>;
        pub(crate) unsafe fn __rust_thunk___Z8GetValue10MyTemplateIiE(
            s: *mut crate::MyTypeAlias,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z13GetValueByRefRK10MyTemplateIiE<'s>(
            s: &'s crate::MyTypeAlias,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z13MakeUnaliasedd(
            __return: *mut ::core::ffi::c_void,
            value: f64,
        );
        pub(crate) unsafe fn __rust_thunk__de4d35d3__ZN10MyTemplateIdEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__5a9d55c6__ZN10MyTemplateIdE6CreateEd(
            __return: *mut ::core::ffi::c_void,
            value: f64,
        );
        pub(crate) unsafe fn __rust_thunk__758630aa__ZNK10MyTemplateIdE5valueEv<'__this>(
            __this: &'__this crate::__CcTemplateInst10MyTemplateIdE,
        ) -> ::cref::CRef<'__this, f64>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::MyTypeAlias>() == 4);
    assert!(::core::mem::align_of::<crate::MyTypeAlias>() == 4);
    static_assertions::assert_impl_all!(crate::MyTypeAlias: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MyTypeAlias: Drop);
    assert!(::core::mem::offset_of!(crate::MyTypeAlias, value_) == 0);
    assert!(::core::mem::size_of::<crate::nested::NestedAlias>() == 4);
    assert!(::core::mem::align_of::<crate::nested::NestedAlias>() == 4);
    static_assertions::assert_impl_all!(crate::nested::NestedAlias: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::nested::NestedAlias: Drop);
    assert!(::core::mem::offset_of!(crate::nested::NestedAlias, value_) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIdE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIdE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIdE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIdE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIdE, value_) == 0);
};
