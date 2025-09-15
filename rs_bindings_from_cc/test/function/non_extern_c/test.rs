// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn test_return_value() {
    use simple_functions::return_value;
    assert_eq!(return_value(), 42);
}

#[gtest]
fn test_return_pointer() {
    use simple_functions::return_pointer;
    unsafe {
        assert_eq!(*return_pointer(), 42);
    }
}

#[gtest]
fn test_return_reference() {
    use simple_functions::return_reference;
    unsafe {
        assert_eq!(*return_reference(), 42);
    }
}

#[gtest]
fn test_take_pointer() {
    use simple_functions::take_pointer;
    unsafe {
        take_pointer(std::ptr::null_mut());
    }
    let mut i: i32 = 0;
    unsafe {
        take_pointer(&mut i);
    }
    assert_eq!(i, 42);
}

#[gtest]
fn test_take_reference() {
    use simple_functions::take_reference;
    let mut i: i32 = 0;
    unsafe {
        take_reference(&mut i);
    }
    assert_eq!(i, 42);
}

#[gtest]
fn test_forward_pointer() {
    use simple_functions::forward_pointer;
    assert_eq!(unsafe { forward_pointer(std::ptr::null()) }, std::ptr::null());
    let i: i32 = 42;
    assert_eq!(unsafe { *forward_pointer(&i) }, 42);
}

#[gtest]
fn test_forward_reference() {
    use simple_functions::forward_reference;
    let i: i32 = 42;
    assert_eq!(unsafe { *forward_reference(&i) }, 42);
}

#[gtest]
fn test_multiply() {
    assert_eq!(simple_functions::multiply(42, 123), 42 * 123);
}

#[gtest]
fn test_multiply_with_unnamed_parameters() {
    assert_eq!(simple_functions::multiply_with_unnamed_parameters(42, 456), 42 * 456);
}

#[gtest]
fn test_multiply_with_keyword_named_parameters() {
    assert_eq!(
        42 * 123 * 456,
        simple_functions::multiply_with_keyword_named_parameters(42, 123, 456)
    );
}

#[gtest]
fn test_function_pointer() {
    let maybe_mul_fn = simple_functions::get_pointer_to_multiply_function();
    let mul_fn = maybe_mul_fn.expect("Expecting non-null / non-None function pointer");
    assert_eq!(mul_fn(123, 456), 123 * 456);
}

#[gtest]
fn test_function_reference() {
    // TODO(b/217419782): Replicate `test_function_pointer`, but for C++
    // references. (e.g. no `expect` / `Option` unwrapping should be
    // needed).
}

#[gtest]
fn test_function_pointer_returned_from_inline_function() {
    let maybe_mul_fn = simple_functions::inline_get_pointer_to_multiply_function();
    let mul_fn = maybe_mul_fn.expect("Expecting non-null / non-None function pointer");
    assert_eq!(mul_fn(123, 456), 123 * 456);
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

    assert_eq!(100 * 200, simple_functions::apply_binary_op(100, 200, Some(multiply)),);
    assert_eq!(300 + 400, simple_functions::apply_binary_op(300, 400, Some(add)),);
}
