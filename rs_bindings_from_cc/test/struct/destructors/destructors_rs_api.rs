// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/destructors:destructors
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=10
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=DestructionOrderRecorder
pub struct DestructionOrderRecorder {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) int_field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for DestructionOrderRecorder {}
impl !Sync for DestructionOrderRecorder {}
unsafe impl ::cxx::ExternType for DestructionOrderRecorder {
    type Id = ::cxx::type_id!("DestructionOrderRecorder");
    type Kind = ::cxx::kind::Trivial;
}
impl DestructionOrderRecorder {
    /// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=34
    #[inline(always)]
    pub fn RecordDestruction(int_field: ::core::ffi::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN24DestructionOrderRecorder17RecordDestructionEi(
                int_field,
            )
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=35
    #[inline(always)]
    pub fn GetDestructionRecord() -> ::core::ffi::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZN24DestructionOrderRecorder20GetDestructionRecordEv()
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=36
    #[inline(always)]
    pub fn ClearDestructionRecord() {
        unsafe {
            crate::detail::__rust_thunk___ZN24DestructionOrderRecorder22ClearDestructionRecordEv()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=12
impl From<::core::ffi::c_int> for DestructionOrderRecorder {
    #[inline(always)]
    fn from(int_field: ::core::ffi::c_int) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24DestructionOrderRecorderC1Ei(
                &raw mut tmp as *mut _,
                int_field,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::core::ffi::c_int> for DestructionOrderRecorder {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::core::ffi::c_int) -> Self::CtorType {
        <Self as From<::core::ffi::c_int>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=16
impl From<::ctor::RvalueReference<'_, Self>> for DestructionOrderRecorder {
    #[inline(always)]
    fn from(other: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24DestructionOrderRecorderC1EOS_(
                &raw mut tmp as *mut _,
                other,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for DestructionOrderRecorder {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=20
impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for DestructionOrderRecorder {
    #[inline(always)]
    fn unpin_assign(&mut self, other: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN24DestructionOrderRecorderaSEOS_(self, other);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=28
impl Drop for DestructionOrderRecorder {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN24DestructionOrderRecorderD1Ev(self) }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=42
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=FieldDestructionOrderTester
pub struct FieldDestructionOrderTester {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field1_: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field2_: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field3_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for FieldDestructionOrderTester {}
impl !Sync for FieldDestructionOrderTester {}
unsafe impl ::cxx::ExternType for FieldDestructionOrderTester {
    type Id = ::cxx::type_id!("FieldDestructionOrderTester");
    type Kind = ::cxx::kind::Trivial;
}
impl FieldDestructionOrderTester {
    /// TODO: b/216648347 - Remove once multi-argument constructors are supported.
    ///
    /// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=52
    #[inline(always)]
    pub fn Create(
        mut field1: crate::DestructionOrderRecorder,
        mut field2: crate::DestructionOrderRecorder,
        mut field3: crate::DestructionOrderRecorder,
    ) -> crate::FieldDestructionOrderTester {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN27FieldDestructionOrderTester6CreateE24DestructionOrderRecorderS0_S0_(&raw mut __return as*mut::core::ffi::c_void,&mut field1,&mut field2,&mut field3);
            __return.assume_init()
        }
    }
    /// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=59
    #[inline(always)]
    pub fn DestructFromCpp(
        field1: ::core::ffi::c_int,
        field2: ::core::ffi::c_int,
        field3: ::core::ffi::c_int,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN27FieldDestructionOrderTester15DestructFromCppEiii(
                field1, field2, field3,
            )
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=42
impl From<::ctor::RvalueReference<'_, Self>> for FieldDestructionOrderTester {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN27FieldDestructionOrderTesterC1EOS_(
                &raw mut tmp as *mut _,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for FieldDestructionOrderTester {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=42
impl Drop for FieldDestructionOrderTester {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN27FieldDestructionOrderTesterD1Ev(self) }
    }
}

/// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=42
impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for FieldDestructionOrderTester {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN27FieldDestructionOrderTesteraSEOS_(self, __param_0);
        }
    }
}

// Generated from: rs_bindings_from_cc/test/struct/destructors/destructors.h;l=44
// Error while generating bindings for constructor 'FieldDestructionOrderTester::FieldDestructionOrderTester':
// Constructors with more than one parameter are not yet supported. See b/216648347.

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/destructors:destructors needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/struct/destructors:destructors needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorderC1Ei(
            __this: *mut ::core::ffi::c_void,
            int_field: ::core::ffi::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorderC1EOS_(
            __this: *mut ::core::ffi::c_void,
            other: ::ctor::RvalueReference<'_, crate::DestructionOrderRecorder>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorderaSEOS_<'__return_lifetime>(
            __this: &mut crate::DestructionOrderRecorder,
            other: ::ctor::RvalueReference<'_, crate::DestructionOrderRecorder>,
        ) -> &'__return_lifetime mut crate::DestructionOrderRecorder;
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorderD1Ev<'a>(
            __this: &'a mut crate::DestructionOrderRecorder,
        );
        #[link_name = "_ZN24DestructionOrderRecorder17RecordDestructionEi"]
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorder17RecordDestructionEi(
            int_field: ::core::ffi::c_int,
        );
        #[link_name = "_ZN24DestructionOrderRecorder20GetDestructionRecordEv"]
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorder20GetDestructionRecordEv(
        ) -> ::core::ffi::c_int;
        #[link_name = "_ZN24DestructionOrderRecorder22ClearDestructionRecordEv"]
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorder22ClearDestructionRecordEv();
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTesterC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::FieldDestructionOrderTester>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTesterD1Ev<'a>(
            __this: &'a mut crate::FieldDestructionOrderTester,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTesteraSEOS_<
            '__return_lifetime,
        >(
            __this: &mut crate::FieldDestructionOrderTester,
            __param_0: ::ctor::RvalueReference<'_, crate::FieldDestructionOrderTester>,
        ) -> &'__return_lifetime mut crate::FieldDestructionOrderTester;
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTester6CreateE24DestructionOrderRecorderS0_S0_(
            __return: *mut ::core::ffi::c_void,
            field1: &mut crate::DestructionOrderRecorder,
            field2: &mut crate::DestructionOrderRecorder,
            field3: &mut crate::DestructionOrderRecorder,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTester15DestructFromCppEiii(
            field1: ::core::ffi::c_int,
            field2: ::core::ffi::c_int,
            field3: ::core::ffi::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::DestructionOrderRecorder>() == 4);
    assert!(::core::mem::align_of::<crate::DestructionOrderRecorder>() == 4);
    static_assertions::assert_impl_all!(crate::DestructionOrderRecorder: Drop);
    static_assertions::assert_not_impl_any!(crate::DestructionOrderRecorder: Copy);
    assert!(::core::mem::offset_of!(crate::DestructionOrderRecorder, int_field_) == 0);
    assert!(::core::mem::size_of::<crate::FieldDestructionOrderTester>() == 12);
    assert!(::core::mem::align_of::<crate::FieldDestructionOrderTester>() == 4);
    static_assertions::assert_impl_all!(crate::FieldDestructionOrderTester: Drop);
    static_assertions::assert_not_impl_any!(crate::FieldDestructionOrderTester: Copy);
    assert!(::core::mem::offset_of!(crate::FieldDestructionOrderTester, field1_) == 0);
    assert!(::core::mem::offset_of!(crate::FieldDestructionOrderTester, field2_) == 4);
    assert!(::core::mem::offset_of!(crate::FieldDestructionOrderTester, field3_) == 8);
};
