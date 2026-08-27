// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub use dep_crate::ExpectedName as SAlias;

pub type Foo = dep_crate::ExpectedName;

pub fn use_s() -> Result<Foo, i32> {
    Err(22)
}

pub fn use_s_alias() -> Result<SAlias, i32> {
    Err(32)
}
