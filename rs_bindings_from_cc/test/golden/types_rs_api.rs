// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:types_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for type alias 'PtrDiff':
// Can't generate bindings for PtrDiff due to missing bindings for its dependency: Unsupported type 'decltype(static_cast<int *>(nullptr) - static_cast<int *>(nullptr))': Unsupported clang::Type class 'Decltype'

// Error while generating bindings for type alias 'Size':
// Can't generate bindings for Size due to missing bindings for its dependency: Unsupported type 'decltype(sizeof (0))': Unsupported clang::Type class 'Decltype'

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeStruct
pub struct SomeStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeStruct {}
impl !Sync for SomeStruct {}
unsafe impl ::cxx::ExternType for SomeStruct {
    type Id = ::cxx::type_id!("SomeStruct");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for SomeStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'SomeStruct::SomeStruct':
// Can't generate bindings for SomeStruct::SomeStruct, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for SomeStruct::SomeStruct (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'SomeStruct::SomeStruct':
// Can't generate bindings for SomeStruct::SomeStruct, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for SomeStruct::SomeStruct (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'SomeStruct::operator=':
// Can't generate bindings for SomeStruct::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for SomeStruct::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for SomeStruct::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'SomeStruct::operator=':
// Can't generate bindings for SomeStruct::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for SomeStruct::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for SomeStruct::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for struct 'ForwardDeclaredStruct':
// Can't generate bindings for ForwardDeclaredStruct, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:wrapper] for ForwardDeclaredStruct (incomplete type)

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `wchar_t_field`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'wchar_t': Unsupported builtin type
///   * `ptrdiff_t_field`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type alias PtrDiff
///   * `size_t_field`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type alias Size
///   * `ptr_field`: raw pointer
///   * `void_ptr_field`: raw pointer
///   * `const_void_ptr_field`: raw pointer
///   * `void_double_ptr_field`: raw pointer
///   * `struct_ptr_field`: raw pointer
///   * `const_struct_ptr_field`: raw pointer
///   * `struct_ref_field`: raw pointer
///   * `const_struct_ref_field`: raw pointer
///   * `forward_declared_ptr_field`: raw pointer
///   * `cyclic_ptr_field`: raw pointer
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=FieldTypeTestStruct
pub struct FieldTypeTestStruct {
    pub bool_field: bool,
    pub char_field: ::ffi_11::c_char,
    pub unsigned_char_field: ::ffi_11::c_uchar,
    pub signed_char_field: ::ffi_11::c_schar,
    pub char16_t_field: u16,
    pub char32_t_field: u32,
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'wchar_t': Unsupported builtin type
    pub(crate) wchar_t_field: [::core::mem::MaybeUninit<u8>; 4],
    pub short_field: ::ffi_11::c_short,
    pub int_field: ::ffi_11::c_int,
    pub long_field: ::ffi_11::c_long,
    pub long_long_field: ::ffi_11::c_longlong,
    pub unsigned_short_field: ::ffi_11::c_ushort,
    pub unsigned_int_field: ::ffi_11::c_uint,
    pub unsigned_long_field: ::ffi_11::c_ulong,
    pub unsigned_long_long_field: ::ffi_11::c_ulonglong,
    pub signed_short_field: ::ffi_11::c_short,
    pub signed_int_field: ::ffi_11::c_int,
    pub signed_long_field: ::ffi_11::c_long,
    pub signed_long_long_field: ::ffi_11::c_longlong,
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type alias PtrDiff
    pub(crate) ptrdiff_t_field: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type alias Size
    pub(crate) size_t_field: [::core::mem::MaybeUninit<u8>; 8],
    pub float_field: f32,
    pub double_field: f64,
    pub ptr_field: *mut ::ffi_11::c_int,
    pub void_ptr_field: *mut ::ffi_11::c_void,
    pub const_void_ptr_field: *const ::ffi_11::c_void,
    pub void_double_ptr_field: *mut *mut ::ffi_11::c_void,
    pub struct_field: crate::SomeStruct,
    pub struct_ptr_field: *mut crate::SomeStruct,
    pub const_struct_ptr_field: *const crate::SomeStruct,
    pub struct_ref_field: *mut crate::SomeStruct,
    pub const_struct_ref_field: *const crate::SomeStruct,
    /// TODO(b/226580208): Uncomment when these don't cause struct import to fail.
    /// SomeStruct&& struct_rvalue_ref_field;
    /// const SomeStruct&& const_struct_rvalue_ref_field;
    ///
    /// Reason for representing this field as a blob of bytes:
    /// missing features: [//features:wrapper]: error: Can't generate bindings for ForwardDeclaredStruct, because of missing required features (crubit.rs-features):
    /// //rs_bindings_from_cc/test/golden:types_cc needs [//features:wrapper] for ForwardDeclaredStruct (incomplete type)
    pub(crate) forward_declared_ptr_field: [::core::mem::MaybeUninit<u8>; 8],
    pub cyclic_ptr_field: *mut crate::FieldTypeTestStruct,
}
impl !Send for FieldTypeTestStruct {}
impl !Sync for FieldTypeTestStruct {}
unsafe impl ::cxx::ExternType for FieldTypeTestStruct {
    type Id = ::cxx::type_id!("FieldTypeTestStruct");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'FieldTypeTestStruct::FieldTypeTestStruct':
// Can't generate bindings for FieldTypeTestStruct::FieldTypeTestStruct, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for FieldTypeTestStruct::FieldTypeTestStruct (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'FieldTypeTestStruct::FieldTypeTestStruct':
// Can't generate bindings for FieldTypeTestStruct::FieldTypeTestStruct, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for FieldTypeTestStruct::FieldTypeTestStruct (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'FunctionTakingPointersAndReferences':
// Can't generate bindings for FunctionTakingPointersAndReferences, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for FunctionTakingPointersAndReferences (the type of const_ref_param (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:types_cc needs [//features:experimental] for FunctionTakingPointersAndReferences (the type of mut_ref_param (parameter #1): references are not supported)

#[inline(always)]
pub fn VoidReturningFunction() {
    unsafe { crate::detail::__rust_thunk___Z21VoidReturningFunctionv() }
}

/// Note especially the use of references. If we convert those to pointers,
/// this becomes un-compilable. The syntax here is awful, but this is a function
/// returning a function. In ML-like syntax:
/// FunctionPointerReturningFunction : () -> (const int&, int*) -> int&
#[inline(always)]
pub fn FunctionPointerReturningFunction() -> Option<
    unsafe extern "C" fn(*const ::ffi_11::c_int, *mut ::ffi_11::c_int) -> *mut ::ffi_11::c_int,
> {
    unsafe { crate::detail::__rust_thunk___Z32FunctionPointerReturningFunctionv() }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `__param_0`: raw pointer
/// * `__param_1`: raw pointer
#[inline(always)]
pub unsafe fn FunctionWithVoidPointers(
    __param_0: *mut ::ffi_11::c_void,
    __param_1: *const ::ffi_11::c_void,
) -> *mut ::ffi_11::c_void {
    crate::detail::__rust_thunk___Z24FunctionWithVoidPointersPvPKv(__param_0, __param_1)
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10SomeStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z21VoidReturningFunctionv();
        pub(crate) unsafe fn __rust_thunk___Z32FunctionPointerReturningFunctionv() -> Option<
            unsafe extern "C" fn(
                *const ::ffi_11::c_int,
                *mut ::ffi_11::c_int,
            ) -> *mut ::ffi_11::c_int,
        >;
        pub(crate) unsafe fn __rust_thunk___Z24FunctionWithVoidPointersPvPKv(
            __param_0: *mut ::ffi_11::c_void,
            __param_1: *const ::ffi_11::c_void,
        ) -> *mut ::ffi_11::c_void;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeStruct>() == 1);
    assert!(::core::mem::align_of::<crate::SomeStruct>() == 1);
    static_assertions::assert_impl_all!(crate::SomeStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);

    assert!(::core::mem::size_of::<crate::FieldTypeTestStruct>() == 208);
    assert!(::core::mem::align_of::<crate::FieldTypeTestStruct>() == 8);
    static_assertions::assert_impl_all!(crate::FieldTypeTestStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::FieldTypeTestStruct: Drop);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, bool_field) == 0);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, char_field) == 1);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_char_field) == 2);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_char_field) == 3);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, char16_t_field) == 4);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, char32_t_field) == 8);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, wchar_t_field) == 12);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, short_field) == 16);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, int_field) == 20);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, long_field) == 24);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, long_long_field) == 32);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_short_field) == 40);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_int_field) == 44);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_long_field) == 48);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_long_long_field) == 56);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_short_field) == 64);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_int_field) == 68);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_long_field) == 72);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_long_long_field) == 80);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, ptrdiff_t_field) == 88);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, size_t_field) == 96);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, float_field) == 104);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, double_field) == 112);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, ptr_field) == 120);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, void_ptr_field) == 128);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, const_void_ptr_field) == 136);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, void_double_ptr_field) == 144);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, struct_field) == 152);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, struct_ptr_field) == 160);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, const_struct_ptr_field) == 168);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, struct_ref_field) == 176);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, const_struct_ref_field) == 184);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, forward_declared_ptr_field) == 192);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, cyclic_ptr_field) == 200);
};
