// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

macro_rules! global_cpp {
    ($($t:tt)*) => {};
}

use inline_cpp_macro::inline_cpp;

global_cpp! {
    #include "third_party/absl/strings/string_view.h"

    int test_global_val = 0;
    int get_test_global_val() {
        return test_global_val;
    }

    int add_two_ints(int a, int b) {
        return a + b;
    }

    int get_magic_number() {
        return 42;
    }

    namespace math_utils {
        int multiply_ints(int a, int b) {
            return a * b;
        }

        struct Point {
            int x;
            int y;
        };

        Point make_point(int x, int y) {
            return Point{x, y};
        }
    }

    int MyTestFunction(int x) {
        return x + 5;
    }

    namespace my_test_namespace {
        class TestClass {
         public:
          static int StaticMethod() {
            int result = 0;
            for (int i = 0; i < 3; ++i) {
                result += i;
            }
            return result;
          }
        };

        const char* GetStringWithBrace() {
            return "String with { and } in it!";
        }

        char GetCharWithBrace() {
           return '{';
        }
    }

    absl::string_view RetrieveStringView() {
        return "Hello absl!";
    }
}

pub fn call_add_two_ints(a: i32, b: i32) -> i32 {
    library_with_embedded_cpp_extracted_cc::add_two_ints(a, b)
}

pub fn call_get_magic_number() -> i32 {
    library_with_embedded_cpp_extracted_cc::get_magic_number()
}

pub fn call_multiply_ints(a: i32, b: i32) -> i32 {
    library_with_embedded_cpp_extracted_cc::math_utils::multiply_ints(a, b)
}

pub fn call_make_point(x: i32, y: i32) -> (i32, i32) {
    let p = library_with_embedded_cpp_extracted_cc::math_utils::make_point(x, y);
    (p.x, p.y)
}

pub fn call_my_test_function(x: i32) -> i32 {
    library_with_embedded_cpp_extracted_cc::MyTestFunction(x)
}

pub fn call_static_method() -> i32 {
    library_with_embedded_cpp_extracted_cc::my_test_namespace::TestClass::StaticMethod()
}

pub fn call_get_string_with_brace() -> String {
    unsafe {
        let ptr = library_with_embedded_cpp_extracted_cc::my_test_namespace::GetStringWithBrace();
        let c_str = std::ffi::CStr::from_ptr(ptr as *const std::os::raw::c_char);
        c_str.to_str().unwrap().to_owned()
    }
}

pub fn call_get_char_with_brace() -> u8 {
    let c = library_with_embedded_cpp_extracted_cc::my_test_namespace::GetCharWithBrace();
    u8::from(c)
}

pub fn call_get_test_global_val() -> i32 {
    library_with_embedded_cpp_extracted_cc::get_test_global_val()
}

pub fn set_global_val_to_99() {
    inline_cpp! {
        test_global_val = 99;
    }
}
