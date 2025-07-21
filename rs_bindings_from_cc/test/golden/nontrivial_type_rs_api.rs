// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Nontrivial
pub struct Nontrivial {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::core::ffi::c_int,
}
impl !Send for Nontrivial {}
impl !Sync for Nontrivial {}
unsafe impl ::cxx::ExternType for Nontrivial {
    type Id = ::cxx::type_id!("Nontrivial");
    type Kind = ::cxx::kind::Opaque;
}

// Error while generating bindings for function 'Nontrivial::Nontrivial':
// Can't generate bindings for Nontrivial::Nontrivial, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::Nontrivial':
// Can't generate bindings for Nontrivial::Nontrivial, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::Nontrivial':
// Can't generate bindings for Nontrivial::Nontrivial, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::Nontrivial':
// Can't generate bindings for Nontrivial::Nontrivial, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::Nontrivial':
// Can't generate bindings for Nontrivial::Nontrivial, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator=':
// Can't generate bindings for Nontrivial::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator=':
// Can't generate bindings for Nontrivial::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator=':
// Can't generate bindings for Nontrivial::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator=':
// Can't generate bindings for Nontrivial::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator= (the type of __this (parameter #0): references are not supported)

impl ::ctor::PinnedDrop for Nontrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self)
    }
}

// Error while generating bindings for function 'Nontrivial::Unqualified':
// Can't generate bindings for Nontrivial::Unqualified, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::Unqualified (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::ConstQualified':
// Can't generate bindings for Nontrivial::ConstQualified, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::ConstQualified (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::LvalueRefQualified':
// Can't generate bindings for Nontrivial::LvalueRefQualified, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::LvalueRefQualified (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::ConstLvalueRefQualified':
// Can't generate bindings for Nontrivial::ConstLvalueRefQualified, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::ConstLvalueRefQualified (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::RvalueRefQualified':
// Can't generate bindings for Nontrivial::RvalueRefQualified, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::RvalueRefQualified (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::ConstRvalueRefQualified':
// Can't generate bindings for Nontrivial::ConstRvalueRefQualified, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::ConstRvalueRefQualified (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator==':
// Can't generate bindings for Nontrivial::operator==, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator== (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator== (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator!=':
// Bindings for this kind of operator (operator != with 2 parameter(s)) are not supported

// Error while generating bindings for function 'Nontrivial::operator<':
// Can't generate bindings for Nontrivial::operator<, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator< (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator< (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator+':
// Can't generate bindings for Nontrivial::operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator+ (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nontrivial::operator+ (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'Nontrivial::operator+=':
// Compound assignment operators are not supported for non-Unpin types, found ::core::pin::Pin<&'a mut crate::Nontrivial>

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialInline
pub struct NontrivialInline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::core::ffi::c_int,
}
impl !Send for NontrivialInline {}
impl !Sync for NontrivialInline {}
unsafe impl ::cxx::ExternType for NontrivialInline {
    type Id = ::cxx::type_id!("NontrivialInline");
    type Kind = ::cxx::kind::Opaque;
}

// Error while generating bindings for function 'NontrivialInline::NontrivialInline':
// Can't generate bindings for NontrivialInline::NontrivialInline, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::NontrivialInline (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'NontrivialInline::NontrivialInline':
// Can't generate bindings for NontrivialInline::NontrivialInline, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::NontrivialInline (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'NontrivialInline::NontrivialInline':
// Can't generate bindings for NontrivialInline::NontrivialInline, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::NontrivialInline (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'NontrivialInline::NontrivialInline':
// Can't generate bindings for NontrivialInline::NontrivialInline, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::NontrivialInline (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::NontrivialInline (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialInline::NontrivialInline':
// Can't generate bindings for NontrivialInline::NontrivialInline, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::NontrivialInline (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::NontrivialInline (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialInline::operator=':
// Can't generate bindings for NontrivialInline::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialInline::operator=':
// Can't generate bindings for NontrivialInline::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialInline::operator=':
// Can't generate bindings for NontrivialInline::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::operator= (the type of __this (parameter #0): references are not supported)

impl ::ctor::PinnedDrop for NontrivialInline {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self)
    }
}

// Error while generating bindings for function 'NontrivialInline::MemberFunction':
// Can't generate bindings for NontrivialInline::MemberFunction, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialInline::MemberFunction (the type of __this (parameter #0): references are not supported)

/// Nontrivial due to member variables.
///
/// This changes how the destructor / drop impl work -- instead of calling
/// the destructor for NontrivialMembers, it just calls the destructors for
/// each field.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=NontrivialMembers
pub struct NontrivialMembers {
    /// Reason for representing this field as a blob of bytes:
    /// nontrivial fields would be destroyed in the wrong order
    pub(crate) nontrivial_member: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for NontrivialMembers {}
impl !Sync for NontrivialMembers {}
unsafe impl ::cxx::ExternType for NontrivialMembers {
    type Id = ::cxx::type_id!("NontrivialMembers");
    type Kind = ::cxx::kind::Opaque;
}

// Error while generating bindings for function 'NontrivialMembers::NontrivialMembers':
// Can't generate bindings for NontrivialMembers::NontrivialMembers, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::NontrivialMembers (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'NontrivialMembers::NontrivialMembers':
// Can't generate bindings for NontrivialMembers::NontrivialMembers, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::NontrivialMembers (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::NontrivialMembers (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialMembers::NontrivialMembers':
// Can't generate bindings for NontrivialMembers::NontrivialMembers, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::NontrivialMembers (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::NontrivialMembers (the type of __param_0 (parameter #1): references are not supported)

impl ::ctor::PinnedDrop for NontrivialMembers {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self)
    }
}

// Error while generating bindings for function 'NontrivialMembers::operator=':
// Can't generate bindings for NontrivialMembers::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialMembers::operator=':
// Can't generate bindings for NontrivialMembers::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialMembers::operator= (the type of __param_0 (parameter #1): references are not supported)

/// Nontrivial, but trivially relocatable and final (and therefore Unpin).
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialUnpin
pub struct NontrivialUnpin {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: ::core::ffi::c_int,
}
impl !Send for NontrivialUnpin {}
impl !Sync for NontrivialUnpin {}
unsafe impl ::cxx::ExternType for NontrivialUnpin {
    type Id = ::cxx::type_id!("NontrivialUnpin");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for function 'NontrivialUnpin::NontrivialUnpin':
// Can't generate bindings for NontrivialUnpin::NontrivialUnpin, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::NontrivialUnpin':
// Can't generate bindings for NontrivialUnpin::NontrivialUnpin, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::NontrivialUnpin':
// Constructors with more than one parameter are not yet supported. See b/216648347.

// Error while generating bindings for function 'NontrivialUnpin::NontrivialUnpin':
// Can't generate bindings for NontrivialUnpin::NontrivialUnpin, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::NontrivialUnpin':
// Can't generate bindings for NontrivialUnpin::NontrivialUnpin, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::NontrivialUnpin':
// Can't generate bindings for NontrivialUnpin::NontrivialUnpin, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::NontrivialUnpin (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::operator=':
// Can't generate bindings for NontrivialUnpin::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::operator=':
// Can't generate bindings for NontrivialUnpin::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialUnpin::operator=':
// Can't generate bindings for NontrivialUnpin::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::operator= (the type of __this (parameter #0): references are not supported)

impl Drop for NontrivialUnpin {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpinD1Ev(self) }
    }
}

// Error while generating bindings for function 'NontrivialUnpin::MemberFunction':
// Can't generate bindings for NontrivialUnpin::MemberFunction, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialUnpin::MemberFunction (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesByValue':
// Can't generate bindings for TakesByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:wrapper] for TakesByValue (<internal link>_relocatable_error: the return type is not rust-movable)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:wrapper] for TakesByValue (<internal link>_relocatable_error: nontrivial (parameter #0) is not rust-movable)

// Error while generating bindings for function 'TakesByValueInline':
// Can't generate bindings for TakesByValueInline, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:wrapper] for TakesByValueInline (<internal link>_relocatable_error: the return type is not rust-movable)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:wrapper] for TakesByValueInline (<internal link>_relocatable_error: nontrivial (parameter #0) is not rust-movable)

#[inline(always)]
pub fn TakesByValueUnpin(mut nontrivial: crate::NontrivialUnpin) -> crate::NontrivialUnpin {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::NontrivialUnpin>::uninit();
        crate::detail::__rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            &raw mut __return as *mut ::core::ffi::c_void,
            &mut nontrivial,
        );
        __return.assume_init()
    }
}

// Error while generating bindings for function 'TakesByReference':
// Can't generate bindings for TakesByReference, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesUnpinByReference':
// Can't generate bindings for TakesUnpinByReference, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesByConstReference':
// Can't generate bindings for TakesByConstReference, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByConstReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByConstReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesUnpinByConstReference':
// Can't generate bindings for TakesUnpinByConstReference, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByConstReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByConstReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesByRvalueReference':
// Can't generate bindings for TakesByRvalueReference, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByRvalueReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByRvalueReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesUnpinByRvalueReference':
// Can't generate bindings for TakesUnpinByRvalueReference, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByRvalueReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByRvalueReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesByConstRvalueReference':
// Can't generate bindings for TakesByConstRvalueReference, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByConstRvalueReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesByConstRvalueReference (the type of nontrivial (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesUnpinByConstRvalueReference':
// Can't generate bindings for TakesUnpinByConstRvalueReference, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByConstRvalueReference (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for TakesUnpinByConstRvalueReference (the type of nontrivial (parameter #0): references are not supported)

/// Finally, testing for strange by-value APIs.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NontrivialByValue
pub struct NontrivialByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NontrivialByValue {}
impl !Sync for NontrivialByValue {}
unsafe impl ::cxx::ExternType for NontrivialByValue {
    type Id = ::cxx::type_id!("NontrivialByValue");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for function 'NontrivialByValue::NontrivialByValue':
// Can't generate bindings for NontrivialByValue::NontrivialByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::NontrivialByValue (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::NontrivialByValue (the type of other (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialByValue::NontrivialByValue':
// Can't generate bindings for NontrivialByValue::NontrivialByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::NontrivialByValue (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::NontrivialByValue (the type of other (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialByValue::operator=':
// Can't generate bindings for NontrivialByValue::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (the type of other (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialByValue::operator=':
// Can't generate bindings for NontrivialByValue::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (the type of other (parameter #1): references are not supported)

// Error while generating bindings for function 'NontrivialByValue::operator=':
// Can't generate bindings for NontrivialByValue::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator= (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'NontrivialByValue::operator==':
// Can't generate bindings for NontrivialByValue::operator==, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for NontrivialByValue::operator== (the type of __this (parameter #0): references are not supported)

#[::ctor::recursively_pinned(PinnedDrop)]
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

// Error while generating bindings for function 'Nonmovable::Nonmovable':
// Can't generate bindings for Nonmovable::Nonmovable, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nonmovable::Nonmovable (the type of __this (parameter #0): references are not supported)

impl ::ctor::PinnedDrop for Nonmovable {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NonmovableD1Ev(self)
    }
}

// Error while generating bindings for function 'Nonmovable::MemberFunction':
// Can't generate bindings for Nonmovable::MemberFunction, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:experimental] for Nonmovable::MemberFunction (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for function 'TakesNonmovableByValue':
// Can't generate bindings for TakesNonmovableByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:wrapper] for TakesNonmovableByValue (<internal link>_relocatable_error: nonmovable (parameter #0) is not rust-movable)

// Error while generating bindings for function 'ReturnsNonmovableByValue':
// Can't generate bindings for ReturnsNonmovableByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc needs [//features:wrapper] for ReturnsNonmovableByValue (<internal link>_relocatable_error: the return type is not rust-movable)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialMembers>,
        );
        #[link_name = "_ZN15NontrivialUnpinD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN15NontrivialUnpinD1Ev<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        pub(crate) unsafe fn __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            __return: *mut ::core::ffi::c_void,
            nontrivial: &mut crate::NontrivialUnpin,
        );
        #[link_name = "_ZN10NonmovableD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NonmovableD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nonmovable>,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
    assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
    assert!(::core::mem::offset_of!(crate::Nontrivial, field) == 0);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NontrivialInline>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialInline>() == 4);
    static_assertions::assert_impl_all!(crate::NontrivialInline: Drop);
    static_assertions::assert_not_impl_any!(crate::NontrivialInline: Copy);
    assert!(::core::mem::offset_of!(crate::NontrivialInline, field) == 0);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NontrivialMembers>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialMembers>() == 4);
    static_assertions::assert_impl_all!(crate::NontrivialMembers: Drop);
    static_assertions::assert_not_impl_any!(crate::NontrivialMembers: Copy);
    assert!(::core::mem::offset_of!(crate::NontrivialMembers, nontrivial_member) == 0);
    assert!(::core::mem::size_of::<crate::NontrivialUnpin>() == 4);
    assert!(::core::mem::align_of::<crate::NontrivialUnpin>() == 4);
    static_assertions::assert_impl_all!(crate::NontrivialUnpin: Drop);
    static_assertions::assert_not_impl_any!(crate::NontrivialUnpin: Copy);
    assert!(::core::mem::offset_of!(crate::NontrivialUnpin, field) == 0);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);
    assert!(::core::mem::size_of::<crate::NontrivialByValue>() == 1);
    assert!(::core::mem::align_of::<crate::NontrivialByValue>() == 1);
    static_assertions::assert_impl_all!(crate::NontrivialByValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Drop);

    assert!(::core::mem::size_of::<crate::Nonmovable>() == 1);
    assert!(::core::mem::align_of::<crate::Nonmovable>() == 1);
    static_assertions::assert_impl_all!(crate::Nonmovable: Drop);
    static_assertions::assert_not_impl_any!(crate::Nonmovable: Copy);
};
