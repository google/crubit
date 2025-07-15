// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/wrapper/pub_crate_types:pub_crate_types
// Features: supported, unsafe_types, wrapper

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=10
// Error while generating bindings for class 'Template':
// Class templates are not supported yet

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=21
pub(crate) type TemplateIntAlias = crate::__CcTemplateInst8TemplateIiE;

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=23
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CompoundDataType
pub struct CompoundDataType {
    pub(crate) template_int: crate::__CcTemplateInst8TemplateIiE,
}
impl !Send for CompoundDataType {}
impl !Sync for CompoundDataType {}
unsafe impl ::cxx::ExternType for CompoundDataType {
    type Id = ::cxx::type_id!("CompoundDataType");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("CompoundDataType"),
    crate::CompoundDataType
);

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=23
// Error while generating bindings for function 'CompoundDataType::CompoundDataType':
// Expected first constructor parameter to be a mutable reference, got: *mut crate::CompoundDataType
// Expected first parameter to be a `__this` reference, found *mut crate::CompoundDataType

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=23
// Error while generating bindings for function 'CompoundDataType::CompoundDataType':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::CompoundDataType
// Expected first parameter to be a `__this` reference, found *mut crate::CompoundDataType

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=23
// Error while generating bindings for function 'CompoundDataType::CompoundDataType':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::CompoundDataType
// Expected first parameter to be a `__this` reference, found *mut crate::CompoundDataType

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=23
// Error while generating bindings for function 'CompoundDataType::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=23
// Error while generating bindings for function 'CompoundDataType::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=27
#[inline(always)]
pub(crate) fn GetTemplateInt() -> crate::__CcTemplateInst8TemplateIiE {
    unsafe {
        let mut __return =
            ::core::mem::MaybeUninit::<crate::__CcTemplateInst8TemplateIiE>::uninit();
        crate::detail::__rust_thunk___Z14GetTemplateIntv(
            &raw mut __return as *mut ::core::ffi::c_void,
        );
        __return.assume_init()
    }
}

extern "C" {
    pub(crate) static mut TemplateConstant: crate::__CcTemplateInst8TemplateIiE;
}

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=31
#[inline(always)]
pub fn ConsumeCompoundDataType(mut container: crate::CompoundDataType) -> ::core::ffi::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z23ConsumeCompoundDataType16CompoundDataType(&mut container)
    }
}

forward_declare::forward_declare!(pub ForwardDeclared = forward_declare::symbol!("ForwardDeclared"));

/// Forward declared types are not pub(crate) so that they can work across
/// module boundaries like this.
///
/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=42
#[inline(always)]
pub unsafe fn OtherPubCrateTypes(__param_0: *mut other_pub_crate_types::ForwardDeclared2) {
    crate::detail::__rust_thunk___Z18OtherPubCrateTypesP16ForwardDeclared2(__param_0)
}

/// Templates, otoh, are pub(crate), but work because templates are already
/// instantiated once per crate.
///
/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=46
#[inline(always)]
pub(crate) fn GetOtherPubCrateTemplate2Int() -> crate::__CcTemplateInst9Template2IiE {
    unsafe {
        let mut __return =
            ::core::mem::MaybeUninit::<crate::__CcTemplateInst9Template2IiE>::uninit();
        crate::detail::__rust_thunk___Z28GetOtherPubCrateTemplate2Intv(
            &raw mut __return as *mut ::core::ffi::c_void,
        );
        __return.assume_init()
    }
}

// Don't uncomment this: a `pair` include starts polluting the golden test with
// a lot of implementation details.
// But this function would produce a different error from the first,
// because it sees the types earlier.
// inline void MixedPubCrateTypes(std::pair<Template<int>*,
// Template2<int>*>) {}

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/other_pub_crate_types.h;l=11
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Template2 < int >
pub(crate) struct __CcTemplateInst9Template2IiE {
    pub value: ::core::ffi::c_int,
}
impl !Send for __CcTemplateInst9Template2IiE {}
impl !Sync for __CcTemplateInst9Template2IiE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("Template2 < int >"),
    crate::__CcTemplateInst9Template2IiE
);

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/other_pub_crate_types.h;l=11
// Error while generating bindings for function 'Template2<int>::Template2<int>':
// Expected first constructor parameter to be a mutable reference, got: *mut crate::__CcTemplateInst9Template2IiE
// Expected first parameter to be a `__this` reference, found *mut crate::__CcTemplateInst9Template2IiE

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/other_pub_crate_types.h;l=11
// Error while generating bindings for function 'Template2<int>::Template2<int>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::__CcTemplateInst9Template2IiE
// Expected first parameter to be a `__this` reference, found *mut crate::__CcTemplateInst9Template2IiE

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/other_pub_crate_types.h;l=11
// Error while generating bindings for function 'Template2<int>::Template2<int>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::__CcTemplateInst9Template2IiE
// Expected first parameter to be a `__this` reference, found *mut crate::__CcTemplateInst9Template2IiE

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/other_pub_crate_types.h;l=11
// Error while generating bindings for function 'Template2<int>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/other_pub_crate_types.h;l=11
// Error while generating bindings for function 'Template2<int>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

/// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=11
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Template < int >
pub(crate) struct __CcTemplateInst8TemplateIiE {
    pub value: ::core::ffi::c_int,
}
impl !Send for __CcTemplateInst8TemplateIiE {}
impl !Sync for __CcTemplateInst8TemplateIiE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("Template < int >"),
    crate::__CcTemplateInst8TemplateIiE
);

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=11
// Error while generating bindings for function 'Template<int>::Template<int>':
// Expected first constructor parameter to be a mutable reference, got: *mut crate::__CcTemplateInst8TemplateIiE
// Expected first parameter to be a `__this` reference, found *mut crate::__CcTemplateInst8TemplateIiE

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=11
// Error while generating bindings for function 'Template<int>::Template<int>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::__CcTemplateInst8TemplateIiE
// Expected first parameter to be a `__this` reference, found *mut crate::__CcTemplateInst8TemplateIiE

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=11
// Error while generating bindings for function 'Template<int>::Template<int>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::__CcTemplateInst8TemplateIiE
// Expected first parameter to be a `__this` reference, found *mut crate::__CcTemplateInst8TemplateIiE

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=11
// Error while generating bindings for function 'Template<int>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=11
// Error while generating bindings for function 'Template<int>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=14
// Error while generating bindings for function 'Template<int>::IndirectCannotBeInstantiated':
// Can't generate bindings for Template<int>::IndirectCannotBeInstantiated, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/wrapper/pub_crate_types:pub_crate_types needs [//features:experimental] for Template<int>::IndirectCannotBeInstantiated (b/248542210: template instantiation of member function cannot reliably get bindings)

// Generated from: rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=18
// Error while generating bindings for function 'Template<int>::CannotBeInstantiated':
// Failed to instantiate the function/method template: Diagnostics emitted:
// rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=16:5: note: in instantiation of member function 'Template<int>::CannotBeInstantiated' requested here
// rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h;l=18:47: error: static assertion failed

#[path = "rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types_extra.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z14GetTemplateIntv(__return: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z23ConsumeCompoundDataType16CompoundDataType(
            container: &mut crate::CompoundDataType,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___Z18OtherPubCrateTypesP16ForwardDeclared2(
            __param_0: *mut other_pub_crate_types::ForwardDeclared2,
        );
        pub(crate) unsafe fn __rust_thunk___Z28GetOtherPubCrateTemplate2Intv(
            __return: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::CompoundDataType>() == 4);
    assert!(::core::mem::align_of::<crate::CompoundDataType>() == 4);
    static_assertions::assert_impl_all!(crate::CompoundDataType: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::CompoundDataType: Drop);
    assert!(::core::mem::offset_of!(crate::CompoundDataType, template_int) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInst9Template2IiE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst9Template2IiE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst9Template2IiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst9Template2IiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst9Template2IiE, value) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInst8TemplateIiE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst8TemplateIiE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst8TemplateIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst8TemplateIiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst8TemplateIiE, value) == 0);
};
