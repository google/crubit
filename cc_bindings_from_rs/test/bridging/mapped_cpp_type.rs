// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_annotate::cpp_layout_equivalent;

#[cpp_layout_equivalent(
    cpp_type = "mapped_cpp_type::MappedCppType",
    include_path = "cc_bindings_from_rs/test/bridging/mapped_cpp_type_def.h"
)]
#[repr(C)]
pub struct MappedCppType(pub i32);

impl core::cmp::PartialEq for MappedCppType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl core::cmp::Eq for MappedCppType {}

impl core::fmt::Debug for MappedCppType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MappedCppType({})", self.0)
    }
}
