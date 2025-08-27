// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn public_extra_function() {
    crate::PublicFunction();
}

// srcs don't get bindings, so there's no public_src_extra_function.
