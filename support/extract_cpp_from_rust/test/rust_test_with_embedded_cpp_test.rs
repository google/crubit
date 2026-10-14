// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

macro_rules! global_cpp {
    ($($t:tt)*) => {};
}

use googletest::{expect_eq, gtest};
use inline_cpp_generated_bindings as rust_test_with_embedded_cpp_test_extracted_cc;
use inline_cpp_macro::inline_cpp;

global_cpp! {
    #include "third_party/absl/strings/string_view.h"

    int add_two_ints(int a, int b) { return a + b; }

    namespace math_utils {
    struct Point {
      int x;
      int y;
    };

    Point make_point(int x, int y) { return Point{x, y}; }
    }  // namespace math_utils

    struct NonPodStruct {
      int val;
      NonPodStruct(int v) : val(v) {}
      ~NonPodStruct() {}
      NonPodStruct(NonPodStruct&&) = default;
    };

    NonPodStruct make_non_pod(int v) { return NonPodStruct{v}; }
}

#[gtest]
fn test_global_cpp_in_test() {
    expect_eq!(rust_test_with_embedded_cpp_test_extracted_cc::add_two_ints(5, 7), 12);
}

#[gtest]
fn test_inline_cpp_in_test() {
    let add = inline_cpp! {
        (int a, int b) -> int { return a + b; }
    };
    expect_eq!(add(10, 20), 30);
}

#[gtest]
fn test_inline_string_length_in_test() {
    let c_str = std::ffi::CString::new("Hello from rust_test_with_embedded_cpp!").unwrap();
    let get_length = inline_cpp! {
        (const char* s) -> int {
          return absl::string_view(s).length();
        }
    };
    // SAFETY: `c_str` is a valid null-terminated C string that outlives the call.
    expect_eq!(unsafe { get_length(c_str.as_ptr() as *const _) }, 39);
}

#[gtest]
fn test_inline_point_sum_in_test() {
    let p = rust_test_with_embedded_cpp_test_extracted_cc::math_utils::make_point(40, 2);
    let sum = inline_cpp! {
        (math_utils::Point p) -> int {
          return p.x + p.y;
        }
    };
    expect_eq!(sum(p), 42);
}

#[gtest]
fn test_inline_non_pod_return_transform_in_test() {
    let non_pod = rust_test_with_embedded_cpp_test_extracted_cc::make_non_pod(42);
    let transform = inline_cpp! {
        (NonPodStruct s) -> NonPodStruct {
          return s;
        }
    };
    let result = transform(non_pod);
    let non_pod_result = ctor::emplace!(result);
    expect_eq!(non_pod_result.val, 42);
}
