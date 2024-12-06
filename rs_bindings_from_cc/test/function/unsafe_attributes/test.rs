// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![deny(warnings)]

use unsafe_attributes::*;

#[googletest::gtest]
#[deny(unsafe_code)]
fn test_safe_signature_or_annotation_is_safe() {
    SafeSignatureWithoutAnnotation();
    SafeSignatureButAnnotatedSafe();
    UnsafeSignatureButAnnotatedSafe(core::ptr::null_mut());
}

#[googletest::gtest]
#[deny(unused_unsafe)]
fn test_unsafe_signature_or_annotation_is_unsafe() {
    unsafe { SafeSignatureButAnnotatedUnsafe() };
    unsafe { UnsafeSignatureButAnnotatedUnsafe(core::ptr::null_mut()) };
    unsafe { UnsafeSignatureWithoutAnnotation(core::ptr::null_mut()) };
}
