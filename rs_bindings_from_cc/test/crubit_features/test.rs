// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use type_exists::type_exists;

mod definitions {
    use super::*;
    #[test]
    fn disabled_struct_has_no_bindings() {
        assert!(
            !type_exists!(definition_disabled::DisabledStruct),
            "definition_disabled::DisabledStruct was exposed through bindings."
        );
    }
    #[test]
    fn enabled_struct_has_bindings() {
        assert!(
            type_exists!(definition_enabled::EnabledStruct),
            "definition_enabled::EnabledStruct was not found in bindings."
        );
    }
}
