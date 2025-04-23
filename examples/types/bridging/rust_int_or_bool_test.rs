// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use either::either::Either;
use googletest::prelude::*;
use int_or_bool::{MakeBool, MakeInt};

#[gtest]
fn test_either() {
    expect_that!(MakeInt(1), pat!(Either::Left(1)));
    expect_that!(MakeBool(true), pat!(Either::Right(true)));
}
