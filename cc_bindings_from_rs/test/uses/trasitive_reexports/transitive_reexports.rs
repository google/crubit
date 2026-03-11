// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
extern crate direct;

pub fn direct_to_transitive(direct: &direct::Direct) -> direct::Transitive {
    direct::Transitive { value: direct.value }
}
