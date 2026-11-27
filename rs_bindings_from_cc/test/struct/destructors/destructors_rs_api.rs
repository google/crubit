// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/destructors:destructors

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// When destroyed, appends `int_field` to `*record` as a decimal digit, so that
/// a test can check the order in which objects were destroyed. Each test passes
/// in its own `record`.
///
/// `std::shared_ptr` isn't trivial-ABI on every platform, and where it isn't,
/// Clang ignores `[[clang::trivial_abi]]` on this class. Moving it with `memcpy`
/// is still sound, so `CRUBIT_UNSAFE_BITWISE_RELOCATABLE` keeps it Rust-movable
/// there too.
#[cfi_encoding = "24DestructionOrderRecorder"]
#[repr(C, align(8))]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=DestructionOrderRecorder")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct DestructionOrderRecorder {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) int_field_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
    /// Null if this object has been moved from.
    ///
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) record_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 16],
}
/// Cannot infer `Send` because:
/// * fields `int_field_`, `record_` are represented as opaque bytes
impl !Send for DestructionOrderRecorder {}
/// Cannot infer `Sync` because:
/// * fields `int_field_`, `record_` are represented as opaque bytes
impl !Sync for DestructionOrderRecorder {}
unsafe impl ::cxx::ExternType for DestructionOrderRecorder {
    type Id = ::cxx::type_id!("DestructionOrderRecorder");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("DestructionOrderRecorder"),
    crate::DestructionOrderRecorder
);

impl From<(::ffi_11::c_int, ::cc_std::std::shared_ptr<::ffi_11::c_int>)>
    for DestructionOrderRecorder
{
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, ::cc_std::std::shared_ptr<::ffi_11::c_int>)) -> Self {
        let (mut int_field, mut record) = args;
        let mut record = ::core::mem::MaybeUninit::new(record);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24DestructionOrderRecorderC1EiNSt3__u10shared_ptrIiEE(
                &raw mut tmp as *mut _,
                int_field,
                record.as_mut_ptr(),
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, ::cc_std::std::shared_ptr<::ffi_11::c_int>)>
    for DestructionOrderRecorder
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (::ffi_11::c_int, ::cc_std::std::shared_ptr<::ffi_11::c_int>),
    ) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, ::cc_std::std::shared_ptr<::ffi_11::c_int>)>>::from(args)
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for DestructionOrderRecorder {
    #[inline(always)]
    fn from(args: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut other = args;
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

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for DestructionOrderRecorder {
    #[inline(always)]
    fn unpin_assign<'__this>(&'__this mut self, other: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN24DestructionOrderRecorderaSEOS_(self, other);
        }
    }
}

impl Drop for DestructionOrderRecorder {
    #[inline(always)]
    fn drop<'__this>(&'__this mut self) {
        unsafe { crate::detail::__rust_thunk___ZN24DestructionOrderRecorderD1Ev(self) }
    }
}

#[cfi_encoding = "27FieldDestructionOrderTester"]
#[repr(C, align(8))]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=FieldDestructionOrderTester")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct FieldDestructionOrderTester {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field1_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field2_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field3_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
}
/// Cannot infer `Send` because:
/// * fields `field1_`, `field2_`, `field3_` are represented as opaque bytes
impl !Send for FieldDestructionOrderTester {}
/// Cannot infer `Sync` because:
/// * fields `field1_`, `field2_`, `field3_` are represented as opaque bytes
impl !Sync for FieldDestructionOrderTester {}
unsafe impl ::cxx::ExternType for FieldDestructionOrderTester {
    type Id = ::cxx::type_id!("FieldDestructionOrderTester");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("FieldDestructionOrderTester"),
    crate::FieldDestructionOrderTester
);
impl FieldDestructionOrderTester {
    #[inline(always)]
    pub fn DestructFromCpp(
        field1: ::ffi_11::c_int,
        field2: ::ffi_11::c_int,
        field3: ::ffi_11::c_int,
        record: ::cc_std::std::shared_ptr<::ffi_11::c_int>,
    ) {
        unsafe {
            self::field_destruction_order_tester::DestructFromCpp(field1, field2, field3, record)
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for FieldDestructionOrderTester {
    #[inline(always)]
    fn from(args: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut __param_0 = args;
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

impl Drop for FieldDestructionOrderTester {
    #[inline(always)]
    fn drop<'__this>(&'__this mut self) {
        unsafe { crate::detail::__rust_thunk___ZN27FieldDestructionOrderTesterD1Ev(self) }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for FieldDestructionOrderTester {
    #[inline(always)]
    fn unpin_assign<'__this>(&'__this mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN27FieldDestructionOrderTesteraSEOS_(self, __param_0);
        }
    }
}

impl
    From<(
        crate::DestructionOrderRecorder,
        crate::DestructionOrderRecorder,
        crate::DestructionOrderRecorder,
    )> for FieldDestructionOrderTester
{
    #[inline(always)]
    fn from(
        args: (
            crate::DestructionOrderRecorder,
            crate::DestructionOrderRecorder,
            crate::DestructionOrderRecorder,
        ),
    ) -> Self {
        let (mut field1, mut field2, mut field3) = args;
        let mut field1 = ::core::mem::MaybeUninit::new(field1);
        let mut field2 = ::core::mem::MaybeUninit::new(field2);
        let mut field3 = ::core::mem::MaybeUninit::new(field3);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN27FieldDestructionOrderTesterC1E24DestructionOrderRecorderS0_S0_(&raw mut tmp as*mut _,field1.as_mut_ptr(),field2.as_mut_ptr(),field3.as_mut_ptr());
            tmp.assume_init()
        }
    }
}
impl
    ::ctor::CtorNew<(
        crate::DestructionOrderRecorder,
        crate::DestructionOrderRecorder,
        crate::DestructionOrderRecorder,
    )> for FieldDestructionOrderTester
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (
            crate::DestructionOrderRecorder,
            crate::DestructionOrderRecorder,
            crate::DestructionOrderRecorder,
        ),
    ) -> Self::CtorType {
        <Self as From<(
            crate::DestructionOrderRecorder,
            crate::DestructionOrderRecorder,
            crate::DestructionOrderRecorder,
        )>>::from(args)
    }
}

pub mod field_destruction_order_tester {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: do_not_bind=")]
    #[inline(always)]
    pub fn DestructFromCpp(
        field1: ::ffi_11::c_int,
        field2: ::ffi_11::c_int,
        field3: ::ffi_11::c_int,
        record: ::cc_std::std::shared_ptr<::ffi_11::c_int>,
    ) {
        let mut record = ::core::mem::MaybeUninit::new(record);
        unsafe {
            crate::detail::__rust_thunk___ZN27FieldDestructionOrderTester15DestructFromCppEiiiNSt3__u10shared_ptrIiEE(field1,field2,field3,record.as_mut_ptr())
        }
    }
}

/// Shares ownership of `token`, so that a test can tell from `token`'s use count
/// whether this object has been destroyed.
#[cfi_encoding = "21PrivateOperatorDelete"]
#[repr(C, align(8))]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=PrivateOperatorDelete")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct PrivateOperatorDelete {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) token_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 16],
}
/// Cannot infer `Send` because:
/// * field `token_` is represented as opaque bytes
impl !Send for PrivateOperatorDelete {}
/// Cannot infer `Sync` because:
/// * field `token_` is represented as opaque bytes
impl !Sync for PrivateOperatorDelete {}
unsafe impl ::cxx::ExternType for PrivateOperatorDelete {
    type Id = ::cxx::type_id!("PrivateOperatorDelete");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("PrivateOperatorDelete"),
    crate::PrivateOperatorDelete
);

impl Clone for PrivateOperatorDelete {
    #[inline(always)]
    fn clone<'__param_0>(&'__param_0 self) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21PrivateOperatorDeleteC1ERKS_(
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

impl From<::ctor::RvalueReference<'_, Self>> for PrivateOperatorDelete {
    #[inline(always)]
    fn from(args: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut __param_0 = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21PrivateOperatorDeleteC1EOS_(
                &raw mut tmp as *mut _,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for PrivateOperatorDelete {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl Drop for PrivateOperatorDelete {
    #[inline(always)]
    fn drop<'__this>(&'__this mut self) {
        unsafe { crate::detail::__rust_thunk___ZN21PrivateOperatorDeleteD1Ev(self) }
    }
}

impl<'__param_0> ::ctor::UnpinAssign<&'__param_0 Self> for PrivateOperatorDelete {
    #[inline(always)]
    fn unpin_assign<'__this>(&'__this mut self, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN21PrivateOperatorDeleteaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for PrivateOperatorDelete {
    #[inline(always)]
    fn unpin_assign<'__this>(&'__this mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN21PrivateOperatorDeleteaSEOS_(self, __param_0);
        }
    }
}

impl From<::cc_std::std::shared_ptr<::ffi_11::c_int>> for PrivateOperatorDelete {
    #[inline(always)]
    fn from(args: ::cc_std::std::shared_ptr<::ffi_11::c_int>) -> Self {
        let mut token = args;
        let mut token = ::core::mem::MaybeUninit::new(token);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21PrivateOperatorDeleteC1ENSt3__u10shared_ptrIiEE(
                &raw mut tmp as *mut _,
                token.as_mut_ptr(),
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::cc_std::std::shared_ptr<::ffi_11::c_int>> for PrivateOperatorDelete {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::cc_std::std::shared_ptr<::ffi_11::c_int>) -> Self::CtorType {
        <Self as From<::cc_std::std::shared_ptr<::ffi_11::c_int>>>::from(args)
    }
}

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
    #[cfi_encoding = "__CcTemplateInstNSt3__u8weak_ptrIiEE"]
    #[repr(C, align(8))]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=std :: weak_ptr < int >")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstNSt3__u8weak_ptrIiEE {
        __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) __ptr_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) __cntrl_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
    }
    /// Cannot infer `Send` because:
    /// * fields `__ptr_`, `__cntrl_` are represented as opaque bytes
    impl !Send for __CcTemplateInstNSt3__u8weak_ptrIiEE {}
    /// Cannot infer `Sync` because:
    /// * fields `__ptr_`, `__cntrl_` are represented as opaque bytes
    impl !Sync for __CcTemplateInstNSt3__u8weak_ptrIiEE {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("std :: weak_ptr < int >"),
        crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE
    );
    impl __CcTemplateInstNSt3__u8weak_ptrIiEE {
        #[inline(always)]
        pub fn swap<'__r, '__this>(&'__this mut self, __r: &'__r mut Self) {
            unsafe {
                crate::detail::__rust_thunk__362c7055__ZNSt3__u8weak_ptrIiE4swapERS1_(self, __r)
            }
        }
        #[inline(always)]
        pub fn reset<'__this>(&'__this mut self) {
            unsafe { crate::detail::__rust_thunk__d20e9826__ZNSt3__u8weak_ptrIiE5resetEv(self) }
        }
        #[must_use]
        #[inline(always)]
        pub fn use_count<'__this>(&'__this self) -> ::ffi_11::c_long {
            unsafe {
                crate::detail::__rust_thunk__12ba3f91__ZNKSt3__u8weak_ptrIiE9use_countEv(self)
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn expired<'__this>(&'__this self) -> bool {
            unsafe { crate::detail::__rust_thunk__e00487c3__ZNKSt3__u8weak_ptrIiE7expiredEv(self) }
        }
        #[must_use]
        #[inline(always)]
        pub fn lock<'__this>(
            &'__this self,
        ) -> ::cc_std::std::Nullable<::cc_std::std::shared_ptr<::ffi_11::c_int>> {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<
                    ::cc_std::std::Nullable<::cc_std::std::shared_ptr<::ffi_11::c_int>>,
                >::uninit();
                crate::detail::__rust_thunk__710c15b7__ZNKSt3__u8weak_ptrIiE4lockEv(
                    &raw mut __crubit_return as *mut ::core::ffi::c_void,
                    self,
                );
                __crubit_return.assume_init()
            }
        }
    }

    impl Default for __CcTemplateInstNSt3__u8weak_ptrIiEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__aee881e6__ZNSt3__u8weak_ptrIiEC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    impl Clone for __CcTemplateInstNSt3__u8weak_ptrIiEE {
        #[inline(always)]
        fn clone<'__r>(&'__r self) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__603d2e7b__ZNSt3__u8weak_ptrIiEC1ERKS1_(
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

    impl From<::ctor::RvalueReference<'_, Self>> for __CcTemplateInstNSt3__u8weak_ptrIiEE {
        #[inline(always)]
        fn from(args: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut __r = args;
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__c4d4a469__ZNSt3__u8weak_ptrIiEC1EOS1_(
                    &raw mut tmp as *mut _,
                    __r,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for __CcTemplateInstNSt3__u8weak_ptrIiEE {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl Drop for __CcTemplateInstNSt3__u8weak_ptrIiEE {
        #[inline(always)]
        fn drop<'__this>(&'__this mut self) {
            unsafe { crate::detail::__rust_thunk__0fa7c98c__ZNSt3__u8weak_ptrIiED1Ev(self) }
        }
    }

    impl<'__r> ::ctor::UnpinAssign<&'__r Self> for __CcTemplateInstNSt3__u8weak_ptrIiEE {
        #[inline(always)]
        fn unpin_assign<'__this>(&'__this mut self, __r: &'__r Self) {
            unsafe {
                crate::detail::__rust_thunk__3056dfe4__ZNSt3__u8weak_ptrIiEaSERKS1_(self, __r);
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>>
        for __CcTemplateInstNSt3__u8weak_ptrIiEE
    {
        #[inline(always)]
        fn unpin_assign<'__this>(&'__this mut self, __r: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk__95fed9fb__ZNSt3__u8weak_ptrIiEaSEOS1_(self, __r);
            }
        }
    }
}
mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorderC1EiNSt3__u10shared_ptrIiEE(
            __this: *mut ::core::ffi::c_void,
            int_field: ::ffi_11::c_int,
            record: *mut ::cc_std::std::shared_ptr<::ffi_11::c_int>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorderC1EOS_(
            __this: *mut ::core::ffi::c_void,
            other: ::ctor::RvalueReference<'_, crate::DestructionOrderRecorder>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorderaSEOS_<'__this>(
            __this: &'__this mut crate::DestructionOrderRecorder,
            other: ::ctor::RvalueReference<'_, crate::DestructionOrderRecorder>,
        ) -> &'__this mut crate::DestructionOrderRecorder;
        pub(crate) unsafe fn __rust_thunk___ZN24DestructionOrderRecorderD1Ev<'__this>(
            __this: &'__this mut crate::DestructionOrderRecorder,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTesterC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::FieldDestructionOrderTester>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTesterD1Ev<'__this>(
            __this: &'__this mut crate::FieldDestructionOrderTester,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTesteraSEOS_<'__this>(
            __this: &'__this mut crate::FieldDestructionOrderTester,
            __param_0: ::ctor::RvalueReference<'_, crate::FieldDestructionOrderTester>,
        ) -> &'__this mut crate::FieldDestructionOrderTester;
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTesterC1E24DestructionOrderRecorderS0_S0_(
            __this: *mut ::core::ffi::c_void,
            field1: *mut crate::DestructionOrderRecorder,
            field2: *mut crate::DestructionOrderRecorder,
            field3: *mut crate::DestructionOrderRecorder,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27FieldDestructionOrderTester15DestructFromCppEiiiNSt3__u10shared_ptrIiEE(
            field1: ::ffi_11::c_int,
            field2: ::ffi_11::c_int,
            field3: ::ffi_11::c_int,
            record: *mut ::cc_std::std::shared_ptr<::ffi_11::c_int>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21PrivateOperatorDeleteC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::PrivateOperatorDelete,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21PrivateOperatorDeleteC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::PrivateOperatorDelete>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21PrivateOperatorDeleteD1Ev<'__this>(
            __this: &'__this mut crate::PrivateOperatorDelete,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21PrivateOperatorDeleteaSERKS_<'__param_0, '__this>(
            __this: &'__this mut crate::PrivateOperatorDelete,
            __param_0: &'__param_0 crate::PrivateOperatorDelete,
        ) -> &'__this mut crate::PrivateOperatorDelete;
        pub(crate) unsafe fn __rust_thunk___ZN21PrivateOperatorDeleteaSEOS_<'__this>(
            __this: &'__this mut crate::PrivateOperatorDelete,
            __param_0: ::ctor::RvalueReference<'_, crate::PrivateOperatorDelete>,
        ) -> &'__this mut crate::PrivateOperatorDelete;
        pub(crate) unsafe fn __rust_thunk___ZN21PrivateOperatorDeleteC1ENSt3__u10shared_ptrIiEE(
            __this: *mut ::core::ffi::c_void,
            token: *mut ::cc_std::std::shared_ptr<::ffi_11::c_int>,
        );
        pub(crate) unsafe fn __rust_thunk__aee881e6__ZNSt3__u8weak_ptrIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__603d2e7b__ZNSt3__u8weak_ptrIiEC1ERKS1_<'__r>(
            __this: *mut ::core::ffi::c_void,
            __r: &'__r crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        );
        pub(crate) unsafe fn __rust_thunk__c4d4a469__ZNSt3__u8weak_ptrIiEC1EOS1_(
            __this: *mut ::core::ffi::c_void,
            __r: ::ctor::RvalueReference<
                '_,
                crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
            >,
        );
        pub(crate) unsafe fn __rust_thunk__0fa7c98c__ZNSt3__u8weak_ptrIiED1Ev<'__this>(
            __this: &'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        );
        pub(crate) unsafe fn __rust_thunk__3056dfe4__ZNSt3__u8weak_ptrIiEaSERKS1_<'__r, '__this>(
            __this: &'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
            __r: &'__r crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        ) -> &'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE;
        pub(crate) unsafe fn __rust_thunk__95fed9fb__ZNSt3__u8weak_ptrIiEaSEOS1_<'__this>(
            __this: &'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
            __r: ::ctor::RvalueReference<
                '_,
                crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
            >,
        ) -> &'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE;
        pub(crate) unsafe fn __rust_thunk__362c7055__ZNSt3__u8weak_ptrIiE4swapERS1_<'__r, '__this>(
            __this: &'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
            __r: &'__r mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        );
        pub(crate) unsafe fn __rust_thunk__d20e9826__ZNSt3__u8weak_ptrIiE5resetEv<'__this>(
            __this: &'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        );
        pub(crate) unsafe fn __rust_thunk__12ba3f91__ZNKSt3__u8weak_ptrIiE9use_countEv<'__this>(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        ) -> ::ffi_11::c_long;
        pub(crate) unsafe fn __rust_thunk__e00487c3__ZNKSt3__u8weak_ptrIiE7expiredEv<'__this>(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__710c15b7__ZNKSt3__u8weak_ptrIiE4lockEv<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::DestructionOrderRecorder>() == 24);
    assert!(::core::mem::align_of::<crate::DestructionOrderRecorder>() == 8);
    static_assertions::assert_impl_all!(crate::DestructionOrderRecorder: Drop);
    static_assertions::assert_not_impl_any!(crate::DestructionOrderRecorder: Copy);
    assert!(::core::mem::offset_of!(crate::DestructionOrderRecorder, int_field_) == 0);
    assert!(::core::mem::offset_of!(crate::DestructionOrderRecorder, record_) == 8);
    assert!(::core::mem::size_of::<crate::FieldDestructionOrderTester>() == 72);
    assert!(::core::mem::align_of::<crate::FieldDestructionOrderTester>() == 8);
    static_assertions::assert_impl_all!(crate::FieldDestructionOrderTester: Drop);
    static_assertions::assert_not_impl_any!(crate::FieldDestructionOrderTester: Copy);
    assert!(::core::mem::offset_of!(crate::FieldDestructionOrderTester, field1_) == 0);
    assert!(::core::mem::offset_of!(crate::FieldDestructionOrderTester, field2_) == 24);
    assert!(::core::mem::offset_of!(crate::FieldDestructionOrderTester, field3_) == 48);
    assert!(::core::mem::size_of::<crate::PrivateOperatorDelete>() == 16);
    assert!(::core::mem::align_of::<crate::PrivateOperatorDelete>() == 8);
    static_assertions::assert_impl_all!(crate::PrivateOperatorDelete: Drop);
    static_assertions::assert_not_impl_any!(crate::PrivateOperatorDelete: Copy);
    assert!(::core::mem::offset_of!(crate::PrivateOperatorDelete, token_) == 0);
    assert!(
        ::core::mem::size_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE: Drop);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE: Copy);
    assert!(
        ::core::mem::offset_of!(
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
            __ptr_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u8weak_ptrIiEE,
            __cntrl_
        ) == 8
    );
};
