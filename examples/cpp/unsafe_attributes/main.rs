// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[deny(unused_unsafe)]
fn main() {
    // Can safely invoke functions that accept simple types or are marked safe.
    example_lib::SafeSignatureWithoutAnnotation();
    example_lib::UnsafeSignatureButAnnotatedSafe(core::ptr::null_mut());
    example_lib::SafeBasedOnBoolean();

    // `unsafe` is needed to invoke functions that accept raw pointers or are marked
    // unsafe.
    unsafe {
        example_lib::UnsafeSignatureWithoutAnnotation(core::ptr::null_mut());
        example_lib::SafeSignatureButAnnotatedUnsafe();
        example_lib::UnsafeBasedOnBoolean();
    }
}
