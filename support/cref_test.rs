// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::{CMut, CMutTo, CRef, CRefTo};
use googletest::gtest;

#[gtest]
fn test_basic_usage() {
    fn accept_some_args<T>(_: CRef<'_, T>, _: CMut<'_, T>) {}
    let mut x = 0;
    let cmut = CMut::from_ref(&mut x);
    accept_some_args(CMut::into_const(cmut), cmut);
}

#[gtest]
fn test_macro_arguments() {
    fn accept_some_args<'a, 'b>(a: CRefTo!('a, i32), b: CMutTo!('b, i32)) {
        let _: CRef<'_, i32> = cref::CRefLike::into_cref(a);
        let _: CMut<'_, i32> = cref::CMutLike::into_cmut(b);
    }
    let x = 0;
    let mut y = 0;
    accept_some_args(&x, &mut y);
}
