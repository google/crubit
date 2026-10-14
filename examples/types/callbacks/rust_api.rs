// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Accepts a borrowed callback (non-owning) matching `rs::FnRef<int32_t(int32_t) const>` in C++.
pub fn apply_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

/// Accepts a mutable borrowed callback matching `rs::FnRef<int32_t(int32_t)>` in C++.
pub fn apply_fn_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 {
    f(x)
}

/// Accepts an owned callback invoked once matching `rs::Fn<int32_t(int32_t) &&>` in C++.
pub fn apply_fn_once(f: Box<dyn FnOnce(i32) -> i32>, x: i32) -> i32 {
    f(x)
}
