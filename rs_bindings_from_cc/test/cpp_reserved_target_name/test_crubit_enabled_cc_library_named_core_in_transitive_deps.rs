// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[test]
fn test_crubit_enabled_cc_library_named_core_in_transitive_deps() {
    let s = core_user::StructInHeaderThatIncludeCoreHeader::default();
    let _ = s.struct_in_core;
}
