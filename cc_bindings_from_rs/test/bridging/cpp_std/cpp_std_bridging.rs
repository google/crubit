// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cpp_std::{optional, raw_string_view, shared_ptr, string_view, string_wrapper, vector};

pub type CallbackFn = Box<dyn Fn(raw_string_view) -> shared_ptr<i32> + Send + Sync>;

pub fn take_callback(cb: CallbackFn, rsv: raw_string_view) -> shared_ptr<i32> {
    cb(rsv)
}

pub fn take_option_string_view(opt: Option<raw_string_view>) -> string_view<'static> {
    match opt {
        Some(_) => string_view::from("some"),
        None => string_view::from("none"),
    }
}

pub fn return_string_wrapper() -> string_wrapper {
    string_wrapper::from("hello")
}

pub fn roundtrip_vector(v: vector<i32>) -> vector<i32> {
    v
}

pub fn roundtrip_optional(v: optional<i32>) -> optional<i32> {
    v
}
