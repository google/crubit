// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_support::{global_cpp, inline_cpp};

global_cpp! {
    #include <algorithm>
}

/// Calls an instantiated C++ standard library template.
pub fn clamp_value(val: f32, min: f32, max: f32) -> f32 {
    let clamp = inline_cpp! {
        (float val, float min, float max) -> float {
            return std::clamp<float>(val, min, max);
        }
    };
    clamp(val, min, max)
}
