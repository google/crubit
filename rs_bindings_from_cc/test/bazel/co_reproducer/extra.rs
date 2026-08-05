// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This imports the placeholder co crate, causing a collision if the generator
// uses the hardcoded `::co` name.
extern crate co;

pub use co::Placeholder;
