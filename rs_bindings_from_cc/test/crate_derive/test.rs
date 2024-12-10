// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate_derive::*;
use googletest::prelude::*;

static_assertions::assert_not_impl_all!(StructWithDerives: Clone, Copy);
static_assertions::assert_impl_all!(StructWithDerives: std::fmt::Debug, Unpin);

static_assertions::assert_impl_all!(SendSync: Send, Sync);

#[gtest]
fn please_compile() {}
