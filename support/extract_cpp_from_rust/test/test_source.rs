// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This is a test rust file to test global_cpp! extraction.

macro_rules! global_cpp {
    ($($t:tt)*) => {};
}

global_cpp! {
    int MyTestFunction(int x) {
        return x + 5;
    }
}

pub fn do_nothing() {
    let _s = "{";
    let _t = "}";
}

global_cpp! {
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
}
