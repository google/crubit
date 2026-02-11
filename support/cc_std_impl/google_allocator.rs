// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[global_allocator]
static ALLOCATOR: crate::crubit_cc_std_internal::std_allocator::StdAllocator =
    crate::crubit_cc_std_internal::std_allocator::StdAllocator {};
