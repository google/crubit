// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_support::global_cpp;

// Declaring C++ headers and types:
global_cpp! {
    #include <algorithm>
    #include <cmath>
    #include "support/rs_std/str_ref.h"
    #include "third_party/absl/strings/str_cat.h"
    #include "third_party/absl/strings/string_view.h"

    struct Point {
        double x;
        double y;
    };

    template <typename T>
    struct MathHelper {
        static T Multiply(T a, T b) { return a * b; }
    };
}
