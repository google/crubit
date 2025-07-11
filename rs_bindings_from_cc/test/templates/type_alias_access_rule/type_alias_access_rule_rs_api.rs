// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/type_alias_access_rule:type_alias_access_rule
// Features: experimental, infer_operator_lifetimes, supported, unsafe_types, wrapper

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=10
// Error while generating bindings for item 'A':
// Class templates are not supported yet

/// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=13
#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=B
pub struct B {
    /// Reason for representing this field as a blob of bytes:
    /// Can't generate bindings for A<B::PrivateMember> due to missing bindings for its dependency: Unsupported type 'struct B::PrivateMember': No generated bindings found for 'PrivateMember'
    pub(crate) a_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for B {}
impl !Sync for B {}
unsafe impl ::cxx::ExternType for B {
    type Id = ::cxx::type_id!("B");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("B"), crate::B);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN1BC1Ev {}
/// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=13
impl<'error> Default for B
where
    &'error (): BindingFailedFor_ZN1BC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=13
// Error while generating bindings for item 'B::B':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN1BC1EOS_ {}
/// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=13
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for B
where
    &'error (): BindingFailedFor_ZN1BC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

/// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=13
impl<'b> ::ctor::UnpinAssign<&'b Self> for B {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN1BaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=13
impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for B {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN1BaSEOS_(self, __param_0);
        }
    }
}

// Generated from: rs_bindings_from_cc/test/templates/type_alias_access_rule/type_alias_access_rule.h;l=11
// Error while generating bindings for item 'A<B::PrivateMember>':
// Can't generate bindings for A<B::PrivateMember> due to missing bindings for its dependency: Unsupported type 'struct B::PrivateMember': No generated bindings found for 'PrivateMember'

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN1BaSERKS_<'a, 'b>(
            __this: &'a mut crate::B,
            __param_0: &'b crate::B,
        ) -> &'a mut crate::B;
        pub(crate) unsafe fn __rust_thunk___ZN1BaSEOS_<'a, 'b>(
            __this: &'a mut crate::B,
            __param_0: ::ctor::RvalueReference<'b, crate::B>,
        ) -> &'a mut crate::B;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::B>() == 1);
    assert!(::core::mem::align_of::<crate::B>() == 1);
    static_assertions::assert_impl_all!(crate::B: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::B: Drop);
    assert!(::core::mem::offset_of!(crate::B, a_) == 0);
};
