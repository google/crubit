// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::{expect_eq, gtest};

#[gtest]
fn test_return_value() {
    use simple::return_value;
    expect_eq!(return_value(), 42);
}

#[gtest]
fn test_return_pointer() {
    use simple::return_pointer;
    unsafe {
        expect_eq!(*return_pointer(), 42);
    }
}

#[gtest]
fn test_return_reference() {
    use simple::return_reference;
    unsafe {
        expect_eq!(*return_reference(), 42);
    }
}

#[gtest]
fn test_take_pointer() {
    use simple::take_pointer;
    unsafe { take_pointer(std::ptr::null_mut()) };
    let mut i: i32 = 0;
    unsafe { take_pointer(&raw mut i) };
    expect_eq!(i, 42);
}

#[gtest]
fn test_take_reference() {
    use simple::take_reference;
    let mut i: i32 = 0;
    unsafe { take_reference(&raw mut i) };
    expect_eq!(i, 42);
}

#[gtest]
fn test_forward_pointer() {
    use simple::forward_pointer;
    expect_eq!(forward_pointer(std::ptr::null()), std::ptr::null());
    let i: i32 = 42;
    expect_eq!(unsafe { *forward_pointer(&raw const i) }, 42);
}

#[gtest]
fn test_forward_reference() {
    use simple::forward_reference;
    let i: i32 = 42;
    expect_eq!(unsafe { *forward_reference(&raw const i) }, 42);
}

#[gtest]
fn test_multiply() {
    expect_eq!(simple::multiply(42, 123), 42 * 123);
}

#[gtest]
fn test_multiply_with_unnamed_parameters() {
    expect_eq!(simple::multiply_with_unnamed_parameters(42, 456), 42 * 456);
}

#[gtest]
fn test_multiply_with_keyword_named_parameters() {
    expect_eq!(42 * 123 * 456, simple::multiply_with_keyword_named_parameters(42, 123, 456));
}

#[gtest]
fn test_function_pointer() {
    let maybe_mul_fn = simple::get_pointer_to_multiply_function();
    let mul_fn = maybe_mul_fn.expect("Expecting non-null / non-None function pointer");
    expect_eq!(mul_fn(123, 456), 123 * 456);
}

#[gtest]
fn test_function_reference() {
    // TODO(b/217419782): Replicate `test_function_pointer`, but for C++
    // references. (e.g. no `expect` / `Option` unwrapping should be
    // needed).
}

#[gtest]
fn test_function_pointer_returned_from_inline_function() {
    let maybe_mul_fn = simple::inline_get_pointer_to_multiply_function();
    let mul_fn = maybe_mul_fn.expect("Expecting non-null / non-None function pointer");
    expect_eq!(mul_fn(123, 456), 123 * 456);
}

/// Test that function pointers can be accepted as function parameters.
#[gtest]
fn test_apply_binary_op() {
    extern "C" fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }
    extern "C" fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    expect_eq!(100 * 200, simple::apply_binary_op(100, 200, Some(multiply)),);
    expect_eq!(300 + 400, simple::apply_binary_op(300, 400, Some(add)),);
}

#[gtest]
fn test_llvm_no_mangle_marker() {
    use simple::llvm_no_mangle_marker;
    expect_eq!(llvm_no_mangle_marker(), 42);
}

#[gtest]
fn asm_name_with_dollar_sign() {
    use simple::asm_name_with_dollar_sign;
    expect_eq!(asm_name_with_dollar_sign(), 42);
}
