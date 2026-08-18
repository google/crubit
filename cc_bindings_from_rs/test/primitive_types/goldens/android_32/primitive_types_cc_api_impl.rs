// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// primitive_types_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uchar_umut_uptr_uarg(__param_0: *mut ::ffi_11::c_char) -> () {
    unsafe { ::primitive_types_golden::argument_types::c_char_mut_ptr_arg(__param_0) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uchar_uptr_uarg(__param_0: *const ::ffi_11::c_char) -> () {
    unsafe { ::primitive_types_golden::argument_types::c_char_ptr_arg(__param_0) }
}
const _: () = assert!(::std::mem::size_of::<::primitive_types_golden::field_types::Types>() == 120);
const _: () = assert!(::std::mem::align_of::<::primitive_types_golden::field_types::Types>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, i8_func) == 0);
const _: () = assert!(
    ::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_char_func) == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_void_mut_ptr) == 8
);
const _: () = assert!(
    ::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_void_const_ptr) == 12
);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_int) == 16);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_uint) == 20);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_long) == 24);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_ulong) == 28);
const _: () = assert!(
    ::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_longlong) == 32
);
const _: () = assert!(
    ::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_ulonglong) == 40
);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_float) == 48);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_double) == 52);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, i32) == 60);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, u32) == 64);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, i64) == 68);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, u64) == 76);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, isize) == 84);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, usize) == 88);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, f32) == 92);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, f64) == 96);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_short) == 104);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_ushort) == 106);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, i16) == 108);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, u16) == 110);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_char) == 112);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_schar) == 113);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, c_uchar) == 114);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, i8) == 115);
const _: () =
    assert!(::core::mem::offset_of!(::primitive_types_golden::field_types::Types, u8) == 116);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uchar() -> ::ffi_11::c_char {
    unsafe { ::primitive_types_golden::return_types::c_char() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uchar_uconst_uptr() -> *const ::ffi_11::c_char {
    unsafe { ::primitive_types_golden::return_types::c_char_const_ptr() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uchar_ufunc() -> extern "C" fn(::ffi_11::c_char) {
    unsafe { ::primitive_types_golden::return_types::c_char_func() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uchar_umut_uptr() -> *mut ::ffi_11::c_char {
    unsafe { ::primitive_types_golden::return_types::c_char_mut_ptr() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_udouble() -> f64 {
    unsafe { ::primitive_types_golden::return_types::c_double() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_ufloat() -> f32 {
    unsafe { ::primitive_types_golden::return_types::c_float() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uint() -> i32 {
    unsafe { ::primitive_types_golden::return_types::c_int() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_ulong() -> ::ffi_11::c_long {
    unsafe { ::primitive_types_golden::return_types::c_long() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_ulonglong() -> i64 {
    unsafe { ::primitive_types_golden::return_types::c_longlong() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uschar() -> i8 {
    unsafe { ::primitive_types_golden::return_types::c_schar() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_ushort() -> i16 {
    unsafe { ::primitive_types_golden::return_types::c_short() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uuchar() -> u8 {
    unsafe { ::primitive_types_golden::return_types::c_uchar() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uuint() -> u32 {
    unsafe { ::primitive_types_golden::return_types::c_uint() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uulong() -> ::ffi_11::c_ulong {
    unsafe { ::primitive_types_golden::return_types::c_ulong() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uulonglong() -> u64 {
    unsafe { ::primitive_types_golden::return_types::c_ulonglong() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uushort() -> u16 {
    unsafe { ::primitive_types_golden::return_types::c_ushort() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uvoid() -> () {
    unsafe { ::primitive_types_golden::return_types::c_void() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uvoid_uconst_uptr() -> *const ::core::ffi::c_void {
    unsafe { ::primitive_types_golden::return_types::c_void_const_ptr() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_c_uvoid_umut_uptr() -> *mut ::core::ffi::c_void {
    unsafe { ::primitive_types_golden::return_types::c_void_mut_ptr() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_f32() -> f32 {
    unsafe { ::primitive_types_golden::return_types::f32() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_f64() -> f64 {
    unsafe { ::primitive_types_golden::return_types::f64() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_i16() -> i16 {
    unsafe { ::primitive_types_golden::return_types::i16() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_i32() -> i32 {
    unsafe { ::primitive_types_golden::return_types::i32() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_i64() -> i64 {
    unsafe { ::primitive_types_golden::return_types::i64() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_i8() -> i8 {
    unsafe { ::primitive_types_golden::return_types::i8() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_i8_ufunc() -> extern "C" fn(i8) {
    unsafe { ::primitive_types_golden::return_types::i8_func() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_isize() -> isize {
    unsafe { ::primitive_types_golden::return_types::isize() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_u16() -> u16 {
    unsafe { ::primitive_types_golden::return_types::u16() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_u32() -> u32 {
    unsafe { ::primitive_types_golden::return_types::u32() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_u64() -> u64 {
    unsafe { ::primitive_types_golden::return_types::u64() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_u8() -> u8 {
    unsafe { ::primitive_types_golden::return_types::u8() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_usize() -> usize {
    unsafe { ::primitive_types_golden::return_types::usize() }
}
const _: () = assert!(
    ::std::mem::size_of::<::primitive_types_golden::test_c_void_ptr::StructWithCVoidPointerMember>(
    ) == 8
);
const _: () = assert!(
    ::std::mem::align_of::<::primitive_types_golden::test_c_void_ptr::StructWithCVoidPointerMember>(
    ) == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(
        ::primitive_types_golden::test_c_void_ptr::StructWithCVoidPointerMember,
        ptr_const
    ) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(
        ::primitive_types_golden::test_c_void_ptr::StructWithCVoidPointerMember,
        ptr_mut
    ) == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_identity_uconst_uc_uvoid_uptr(
    ptr: *const ::core::ffi::c_void,
) -> *const ::core::ffi::c_void {
    unsafe { ::primitive_types_golden::test_c_void_ptr::identity_const_c_void_ptr(ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_identity_umut_uc_uvoid_uptr(
    ptr: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe { ::primitive_types_golden::test_c_void_ptr::identity_mut_c_void_ptr(ptr) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new_ustruct_uwith_uc_uvoid_upointer_umember(
    ptr_const: *const ::core::ffi::c_void,
    ptr_mut: *mut ::core::ffi::c_void,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::primitive_types_golden::test_c_void_ptr::new_struct_with_c_void_pointer_member(
                ptr_const, ptr_mut,
            );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_maybe_uuninit_uptr(
    maybe_uninit: *const std::mem::MaybeUninit<i32>,
) -> *const std::mem::MaybeUninit<i32> {
    unsafe { ::primitive_types_golden::test_maybe_uninit::maybe_uninit_ptr(maybe_uninit) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_maybe_uuninit_uptr_umut(
    maybe_uninit: *mut std::mem::MaybeUninit<i32>,
) -> *mut std::mem::MaybeUninit<i32> {
    unsafe { ::primitive_types_golden::test_maybe_uninit::maybe_uninit_ptr_mut(maybe_uninit) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_maybe_uuninit_uref(
    maybe_uninit: &'static std::mem::MaybeUninit<i32>,
) -> &'static std::mem::MaybeUninit<i32> {
    unsafe { ::primitive_types_golden::test_maybe_uninit::maybe_uninit_ref(maybe_uninit) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_maybe_uuninit_uref_umut(
    maybe_uninit: &'static mut std::mem::MaybeUninit<i32>,
) -> &'static mut std::mem::MaybeUninit<i32> {
    unsafe { ::primitive_types_golden::test_maybe_uninit::maybe_uninit_ref_mut(maybe_uninit) }
}
