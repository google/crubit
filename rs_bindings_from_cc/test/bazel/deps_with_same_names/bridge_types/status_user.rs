// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn call_make_status_or(x: i32) -> Result<i32, String> {
    status_user_cc::make_status_or(x).map_err(|e| format!("{:?}", e))
}
