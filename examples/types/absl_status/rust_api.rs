// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// This function is only callable from Rust (for now).
pub fn returns_status(ok: bool) -> status::Status {
    if ok {
        status::OkStatus()
    } else {
        Err(status::internal("Something went wrong, oh no!"))
    }
}

/// This function is callable from C++.
#[allow(non_snake_case)]
pub fn ReturnsStatus(ok: bool) -> status_wrapper::StatusWrapper {
    returns_status(ok).into()
}
