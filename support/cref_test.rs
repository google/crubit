// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![deny(improper_ctypes_definitions)]

use core::mem::{align_of, size_of, transmute};
use cref::{CMut, CMutTo, CRef, CRefTo};
use googletest::gtest;

#[gtest]
fn test_basic_usage() {
    fn accept_some_args<T>(_: CRef<'_, T>, _: CMut<'_, T>) {}
    let mut x = 0;
    let cmut = CMut::from_ref(&mut x);
    accept_some_args(CMut::into_const(cmut), cmut);
}

#[gtest]
fn test_macro_arguments() {
    fn accept_some_args<'a, 'b>(a: CRefTo!('a, i32), b: CMutTo!('b, i32)) {
        let _: CRef<'_, i32> = cref::CRefLike::into_cref(a);
        let _: CMut<'_, i32> = cref::CMutLike::into_cmut(b);
    }
    let x = 0;
    let mut y = 0;
    accept_some_args(&x, &mut y);
}

// `CRef` and `CMut` have the layout of a thin pointer, and so does `Option` of each.
const _: () = {
    assert!(size_of::<CRef<'static, i32>>() == size_of::<*const i32>());
    assert!(align_of::<CRef<'static, i32>>() == align_of::<*const i32>());
    assert!(size_of::<Option<CRef<'static, i32>>>() == size_of::<*const i32>());
    assert!(size_of::<CMut<'static, i32>>() == size_of::<*mut i32>());
    assert!(align_of::<CMut<'static, i32>>() == align_of::<*mut i32>());
    assert!(size_of::<Option<CMut<'static, i32>>>() == size_of::<*mut i32>());
};

/// Fails to compile under `improper_ctypes_definitions` if `CRef` or `CMut` stops
/// being FFI-safe as a nullable pointer.
extern "C" fn ffi_signature(p: Option<CMut<'_, i32>>) -> Option<CRef<'_, i32>> {
    p.map(CMut::into_const)
}

#[gtest]
fn test_option_none_is_all_zero_bytes() {
    const N: usize = size_of::<Option<CRef<'static, i32>>>();
    // SAFETY: The `std::option` docs guarantee that transmuting between
    // `[0u8; size_of::<T>()]` and `Option::<T>::None` is sound, in both directions,
    // for a `#[repr(transparent)]` struct around `NonNull<U>` with `U: Sized`.
    // `CRef<'static, i32>` is such a struct over `NonNull<i32>`, and `N` is its size.
    let none: Option<CRef<'static, i32>> = unsafe { transmute([0u8; N]) };
    assert!(none.is_none());
    // SAFETY: The same guarantee, for `CMut<'static, i32>`, reverse direction.
    let bytes: [u8; N] = unsafe { transmute(None::<CMut<'static, i32>>) };
    assert_eq!(bytes, [0u8; N]);
}

#[gtest]
fn test_ffi_signature_round_trip() {
    let mut x = 7;
    let px: *const i32 = &raw const x;
    let r = ffi_signature(Some(CMut::from_ref(&mut x)));
    assert_eq!(CRef::as_ptr(r.unwrap()), px);
    assert!(ffi_signature(None).is_none());
}
