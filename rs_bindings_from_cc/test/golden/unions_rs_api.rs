// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unions_cc

#![rustfmt::skip]
#![feature(
    allocator_api,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls
)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=EmptyUnion
pub union EmptyUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for EmptyUnion {}
impl !Sync for EmptyUnion {}
unsafe impl ::cxx::ExternType for EmptyUnion {
    type Id = ::cxx::type_id!("EmptyUnion");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for EmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'EmptyUnion::EmptyUnion':
// Can't generate bindings for EmptyUnion::EmptyUnion, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for EmptyUnion::EmptyUnion (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'EmptyUnion::EmptyUnion':
// Can't generate bindings for EmptyUnion::EmptyUnion, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for EmptyUnion::EmptyUnion (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'EmptyUnion::operator=':
// Can't generate bindings for EmptyUnion::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for EmptyUnion::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for EmptyUnion::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'EmptyUnion::operator=':
// Can't generate bindings for EmptyUnion::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for EmptyUnion::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for EmptyUnion::operator= (the type of __param_0 (parameter #1): references are not supported)

#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Nontrivial
pub struct Nontrivial {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub field: ::ffi_11::c_int,
}
impl !Send for Nontrivial {}
impl !Sync for Nontrivial {}
unsafe impl ::cxx::ExternType for Nontrivial {
    type Id = ::cxx::type_id!("Nontrivial");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for Nontrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ev(dest as *mut ::core::ffi::c_void);
            })
        }
    }
}

// Error while generating bindings for constructor 'Nontrivial::Nontrivial':
// Can't generate bindings for Nontrivial::Nontrivial, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for Nontrivial::Nontrivial (the type of __param_0 (parameter #1): references are not supported)

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=UnionToRename
pub union RenamedUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for RenamedUnion {}
impl !Sync for RenamedUnion {}
unsafe impl ::cxx::ExternType for RenamedUnion {
    type Id = ::cxx::type_id!("UnionToRename");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for RenamedUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13UnionToRenameC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'UnionToRename::UnionToRename':
// Can't generate bindings for UnionToRename::UnionToRename, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionToRename::UnionToRename (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'UnionToRename::UnionToRename':
// Can't generate bindings for UnionToRename::UnionToRename, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionToRename::UnionToRename (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'UnionToRename::operator=':
// Can't generate bindings for UnionToRename::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionToRename::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionToRename::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'UnionToRename::operator=':
// Can't generate bindings for UnionToRename::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionToRename::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionToRename::operator= (the type of __param_0 (parameter #1): references are not supported)

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TriviallyCopyableButNontriviallyDestructible
pub struct TriviallyCopyableButNontriviallyDestructible {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for TriviallyCopyableButNontriviallyDestructible {}
impl !Sync for TriviallyCopyableButNontriviallyDestructible {}
unsafe impl ::cxx::ExternType for TriviallyCopyableButNontriviallyDestructible {
    type Id = ::cxx::type_id!("TriviallyCopyableButNontriviallyDestructible");
    type Kind = ::cxx::kind::Opaque;
}

// Error while generating bindings for function 'TriviallyCopyableButNontriviallyDestructible::operator=':
// Can't generate bindings for TriviallyCopyableButNontriviallyDestructible::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TriviallyCopyableButNontriviallyDestructible::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TriviallyCopyableButNontriviallyDestructible::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'TriviallyCopyableButNontriviallyDestructible::TriviallyCopyableButNontriviallyDestructible':
// Can't generate bindings for TriviallyCopyableButNontriviallyDestructible::TriviallyCopyableButNontriviallyDestructible, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TriviallyCopyableButNontriviallyDestructible::TriviallyCopyableButNontriviallyDestructible (the type of __param_0 (parameter #1): references are not supported)

impl ::ctor::PinnedDrop for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(self)
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NonEmptyUnion
pub union NonEmptyUnion {
    pub bool_field: bool,
    pub char_field: ::ffi_11::c_char,
    pub int_field: ::ffi_11::c_int,
    pub long_long_field: ::ffi_11::c_longlong,
}
impl !Send for NonEmptyUnion {}
impl !Sync for NonEmptyUnion {}
unsafe impl ::cxx::ExternType for NonEmptyUnion {
    type Id = ::cxx::type_id!("NonEmptyUnion");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for NonEmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'NonEmptyUnion::NonEmptyUnion':
// Can't generate bindings for NonEmptyUnion::NonEmptyUnion, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonEmptyUnion::NonEmptyUnion (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'NonEmptyUnion::NonEmptyUnion':
// Can't generate bindings for NonEmptyUnion::NonEmptyUnion, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonEmptyUnion::NonEmptyUnion (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NonEmptyUnion::operator=':
// Can't generate bindings for NonEmptyUnion::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonEmptyUnion::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonEmptyUnion::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NonEmptyUnion::operator=':
// Can't generate bindings for NonEmptyUnion::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonEmptyUnion::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonEmptyUnion::operator= (the type of __param_0 (parameter #1): references are not supported)

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NonCopyUnion
pub union NonCopyUnion {
    pub trivial_member: bool,
    pub nontrivial_member: ::core::mem::ManuallyDrop<crate::Nontrivial>,
}
impl !Send for NonCopyUnion {}
impl !Sync for NonCopyUnion {}
unsafe impl ::cxx::ExternType for NonCopyUnion {
    type Id = ::cxx::type_id!("NonCopyUnion");
    type Kind = ::cxx::kind::Opaque;
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NonCopyUnion2
pub union NonCopyUnion2 {
    pub trivial_member: bool,
    pub nontrivial_member:
        ::core::mem::ManuallyDrop<crate::TriviallyCopyableButNontriviallyDestructible>,
}
impl !Send for NonCopyUnion2 {}
impl !Sync for NonCopyUnion2 {}
unsafe impl ::cxx::ExternType for NonCopyUnion2 {
    type Id = ::cxx::type_id!("NonCopyUnion2");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'NonCopyUnion2::NonCopyUnion2':
// Can't generate bindings for NonCopyUnion2::NonCopyUnion2, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonCopyUnion2::NonCopyUnion2 (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'NonCopyUnion2::NonCopyUnion2':
// Can't generate bindings for NonCopyUnion2::NonCopyUnion2, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonCopyUnion2::NonCopyUnion2 (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NonCopyUnion2::operator=':
// Can't generate bindings for NonCopyUnion2::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonCopyUnion2::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonCopyUnion2::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'NonCopyUnion2::operator=':
// Can't generate bindings for NonCopyUnion2::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonCopyUnion2::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for NonCopyUnion2::operator= (the type of __param_0 (parameter #1): references are not supported)

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `constant_array_field_not_yet_supported`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'char[42]': Unsupported clang::Type class 'ConstantArray'
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=UnionWithOpaqueField
pub union UnionWithOpaqueField {
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'char[42]': Unsupported clang::Type class 'ConstantArray'
    pub(crate) constant_array_field_not_yet_supported: [::core::mem::MaybeUninit<u8>; 42],
}
impl !Send for UnionWithOpaqueField {}
impl !Sync for UnionWithOpaqueField {}
unsafe impl ::cxx::ExternType for UnionWithOpaqueField {
    type Id = ::cxx::type_id!("UnionWithOpaqueField");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for UnionWithOpaqueField {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithOpaqueFieldC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'UnionWithOpaqueField::UnionWithOpaqueField':
// Can't generate bindings for UnionWithOpaqueField::UnionWithOpaqueField, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithOpaqueField::UnionWithOpaqueField (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'UnionWithOpaqueField::UnionWithOpaqueField':
// Can't generate bindings for UnionWithOpaqueField::UnionWithOpaqueField, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithOpaqueField::UnionWithOpaqueField (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'UnionWithOpaqueField::operator=':
// Can't generate bindings for UnionWithOpaqueField::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithOpaqueField::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithOpaqueField::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'UnionWithOpaqueField::operator=':
// Can't generate bindings for UnionWithOpaqueField::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithOpaqueField::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithOpaqueField::operator= (the type of __param_0 (parameter #1): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TrivialButInheritable
pub struct TrivialButInheritable {
    pub x: ::ffi_11::c_int,
}
impl !Send for TrivialButInheritable {}
impl !Sync for TrivialButInheritable {}
unsafe impl ::cxx::ExternType for TrivialButInheritable {
    type Id = ::cxx::type_id!("TrivialButInheritable");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for TrivialButInheritable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21TrivialButInheritableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'TrivialButInheritable::TrivialButInheritable':
// Can't generate bindings for TrivialButInheritable::TrivialButInheritable, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TrivialButInheritable::TrivialButInheritable (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'TrivialButInheritable::TrivialButInheritable':
// Can't generate bindings for TrivialButInheritable::TrivialButInheritable, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TrivialButInheritable::TrivialButInheritable (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TrivialButInheritable::operator=':
// Can't generate bindings for TrivialButInheritable::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TrivialButInheritable::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TrivialButInheritable::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TrivialButInheritable::operator=':
// Can't generate bindings for TrivialButInheritable::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TrivialButInheritable::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TrivialButInheritable::operator= (the type of __param_0 (parameter #1): references are not supported)

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=UnionWithInheritable
pub union UnionWithInheritable {
    pub t: crate::TrivialButInheritable,
}
impl !Send for UnionWithInheritable {}
impl !Sync for UnionWithInheritable {}
unsafe impl ::cxx::ExternType for UnionWithInheritable {
    type Id = ::cxx::type_id!("UnionWithInheritable");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for UnionWithInheritable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20UnionWithInheritableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'UnionWithInheritable::UnionWithInheritable':
// Can't generate bindings for UnionWithInheritable::UnionWithInheritable, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithInheritable::UnionWithInheritable (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'UnionWithInheritable::UnionWithInheritable':
// Can't generate bindings for UnionWithInheritable::UnionWithInheritable, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithInheritable::UnionWithInheritable (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'UnionWithInheritable::operator=':
// Can't generate bindings for UnionWithInheritable::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithInheritable::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithInheritable::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'UnionWithInheritable::operator=':
// Can't generate bindings for UnionWithInheritable::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithInheritable::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for UnionWithInheritable::operator= (the type of __param_0 (parameter #1): references are not supported)

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypedefUnion
pub union TypedefUnion {
    pub trivial_member: bool,
}
impl !Send for TypedefUnion {}
impl !Sync for TypedefUnion {}
unsafe impl ::cxx::ExternType for TypedefUnion {
    type Id = ::cxx::type_id!("TypedefUnion");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for TypedefUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12TypedefUnionC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'TypedefUnion::TypedefUnion':
// Can't generate bindings for TypedefUnion::TypedefUnion, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnion::TypedefUnion (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'TypedefUnion::TypedefUnion':
// Can't generate bindings for TypedefUnion::TypedefUnion, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnion::TypedefUnion (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TypedefUnion::operator=':
// Can't generate bindings for TypedefUnion::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnion::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnion::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TypedefUnion::operator=':
// Can't generate bindings for TypedefUnion::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnion::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnion::operator= (the type of __param_0 (parameter #1): references are not supported)

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypedefUnionWithInheritable
pub union TypedefUnionWithInheritable {
    pub t: crate::TrivialButInheritable,
}
impl !Send for TypedefUnionWithInheritable {}
impl !Sync for TypedefUnionWithInheritable {}
unsafe impl ::cxx::ExternType for TypedefUnionWithInheritable {
    type Id = ::cxx::type_id!("TypedefUnionWithInheritable");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for TypedefUnionWithInheritable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN27TypedefUnionWithInheritableC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'TypedefUnionWithInheritable::TypedefUnionWithInheritable':
// Can't generate bindings for TypedefUnionWithInheritable::TypedefUnionWithInheritable, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnionWithInheritable::TypedefUnionWithInheritable (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'TypedefUnionWithInheritable::TypedefUnionWithInheritable':
// Can't generate bindings for TypedefUnionWithInheritable::TypedefUnionWithInheritable, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnionWithInheritable::TypedefUnionWithInheritable (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TypedefUnionWithInheritable::operator=':
// Can't generate bindings for TypedefUnionWithInheritable::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnionWithInheritable::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnionWithInheritable::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TypedefUnionWithInheritable::operator=':
// Can't generate bindings for TypedefUnionWithInheritable::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnionWithInheritable::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:unions_cc needs [//features:experimental] for TypedefUnionWithInheritable::operator= (the type of __param_0 (parameter #1): references are not supported)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10EmptyUnionC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN13UnionToRenameC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev<
            'a,
        >(
            __this: ::core::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13NonEmptyUnionC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN20UnionWithOpaqueFieldC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21TrivialButInheritableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20UnionWithInheritableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN12TypedefUnionC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN27TypedefUnionWithInheritableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::EmptyUnion>() == 1);
    assert!(::core::mem::align_of::<crate::EmptyUnion>() == 1);
    static_assertions::assert_impl_all!(crate::EmptyUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::EmptyUnion: Drop);

    assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
    assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::Nontrivial, field) == 0);
    assert!(::core::mem::size_of::<crate::RenamedUnion>() == 1);
    assert!(::core::mem::align_of::<crate::RenamedUnion>() == 1);
    static_assertions::assert_impl_all!(crate::RenamedUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::RenamedUnion: Drop);

    assert!(::core::mem::size_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
    assert!(::core::mem::align_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
    static_assertions::assert_impl_all!(crate::TriviallyCopyableButNontriviallyDestructible: Drop);
    static_assertions::assert_not_impl_any!(crate::TriviallyCopyableButNontriviallyDestructible: Copy);

    assert!(::core::mem::size_of::<crate::NonEmptyUnion>() == 8);
    assert!(::core::mem::align_of::<crate::NonEmptyUnion>() == 8);
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NonEmptyUnion: Drop);
    assert!(::core::mem::offset_of!(crate::NonEmptyUnion, bool_field) == 0);
    assert!(::core::mem::offset_of!(crate::NonEmptyUnion, char_field) == 0);
    assert!(::core::mem::offset_of!(crate::NonEmptyUnion, int_field) == 0);
    assert!(::core::mem::offset_of!(crate::NonEmptyUnion, long_long_field) == 0);
    static_assertions::assert_impl_all!(bool: Copy);
    static_assertions::assert_impl_all!(::ffi_11::c_char: Copy);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    static_assertions::assert_impl_all!(::ffi_11::c_longlong: Copy);
    assert!(::core::mem::size_of::<crate::NonCopyUnion>() == 4);
    assert!(::core::mem::align_of::<crate::NonCopyUnion>() == 4);
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::NonCopyUnion, trivial_member) == 0);
    assert!(::core::mem::offset_of!(crate::NonCopyUnion, nontrivial_member) == 0);
    static_assertions::assert_impl_all!(bool: Copy);
    assert!(::core::mem::size_of::<crate::NonCopyUnion2>() == 1);
    assert!(::core::mem::align_of::<crate::NonCopyUnion2>() == 1);
    static_assertions::assert_not_impl_any!(crate::NonCopyUnion2: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::NonCopyUnion2, trivial_member) == 0);
    assert!(::core::mem::offset_of!(crate::NonCopyUnion2, nontrivial_member) == 0);
    static_assertions::assert_impl_all!(bool: Copy);
    assert!(::core::mem::size_of::<crate::UnionWithOpaqueField>() == 42);
    assert!(::core::mem::align_of::<crate::UnionWithOpaqueField>() == 1);
    static_assertions::assert_impl_all!(crate::UnionWithOpaqueField: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::UnionWithOpaqueField: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::UnionWithOpaqueField,
            constant_array_field_not_yet_supported
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::TrivialButInheritable>() == 4);
    assert!(::core::mem::align_of::<crate::TrivialButInheritable>() == 4);
    static_assertions::assert_impl_all!(crate::TrivialButInheritable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TrivialButInheritable: Drop);
    assert!(::core::mem::offset_of!(crate::TrivialButInheritable, x) == 0);
    assert!(::core::mem::size_of::<crate::UnionWithInheritable>() == 4);
    assert!(::core::mem::align_of::<crate::UnionWithInheritable>() == 4);
    static_assertions::assert_impl_all!(crate::UnionWithInheritable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::UnionWithInheritable: Drop);
    assert!(::core::mem::offset_of!(crate::UnionWithInheritable, t) == 0);
    static_assertions::assert_impl_all!(crate::TrivialButInheritable: Copy);
    assert!(::core::mem::size_of::<crate::TypedefUnion>() == 1);
    assert!(::core::mem::align_of::<crate::TypedefUnion>() == 1);
    static_assertions::assert_impl_all!(crate::TypedefUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TypedefUnion: Drop);
    assert!(::core::mem::offset_of!(crate::TypedefUnion, trivial_member) == 0);
    static_assertions::assert_impl_all!(bool: Copy);
    assert!(::core::mem::size_of::<crate::TypedefUnionWithInheritable>() == 4);
    assert!(::core::mem::align_of::<crate::TypedefUnionWithInheritable>() == 4);
    static_assertions::assert_impl_all!(crate::TypedefUnionWithInheritable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TypedefUnionWithInheritable: Drop);
    assert!(::core::mem::offset_of!(crate::TypedefUnionWithInheritable, t) == 0);
    static_assertions::assert_impl_all!(crate::TrivialButInheritable: Copy);
};
