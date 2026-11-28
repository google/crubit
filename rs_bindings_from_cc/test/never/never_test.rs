// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use never::*;

#[googletest::test]
fn test_never_signatures() {
    let _f: fn() -> ! = MakeCustomNever;
    let _g: fn(!) = AcceptCustomNever;

    let _h: fn() -> ! = MakeNeverT;
    let _i: fn(!) = AcceptNeverT;

    let opt_custom: Option<!> = ReturnOptionalCustomNever();
    assert_that!(opt_custom, eq(None));

    let opt_never: Option<!> = ReturnOptionalNeverT();
    assert_that!(opt_never, eq(None));

    let p: *const ! = unsafe { PassPtrNeverT(std::ptr::null()) };
    assert!(p.is_null());
}
