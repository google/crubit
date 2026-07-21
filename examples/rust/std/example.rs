// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn returns_result(is_ok: bool) -> Result<Option<&'static str>, String> {
    if is_ok {
        Ok(Some("hello world!"))
    } else {
        Err("goodbye, cruel world!".to_string())
    }
}
