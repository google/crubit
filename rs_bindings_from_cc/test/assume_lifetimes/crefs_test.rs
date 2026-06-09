// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::{CMut, CRef};
use googletest::gtest;

#[gtest]
fn int_test() {
    let x = 1;
    let _xref: CRef<'_, i32> = crefs::id_cref(&x);

    let mut y = 2;
    let _ymut: CMut<'_, i32> = crefs::id_cmut(&mut y);
}
