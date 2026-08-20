// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use status::{err, ok, NewStatus as Status, NewStatusOr as StatusOr};

pub fn returns_status(is_ok: bool) -> Status {
    if is_ok {
        ok(())
    } else {
        err(status::internal("Something went wrong, oh no!"))
    }
}

pub fn returns_status_or_int(is_ok: bool) -> StatusOr<i32> {
    if is_ok {
        ok(42)
    } else {
        err(status::internal("Something went wrong, oh no!"))
    }
}
