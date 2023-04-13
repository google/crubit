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

mod aliases {
    use super::*;
    /// This test will fail if aliases expose a struct whose bindings were
    /// disabled.
    #[test]
    fn aliases_dont_expose_disabled_structs() {
        assert!(
            !type_exists!(alias_enabled::AliasedDisabledStruct),
            "AliasedDisabledStruct was exported by `alias_enabled`, even though `DisabledStruct` disabled bindings."
        );
    }

    /// This test will fail if aliases expose a template whose bindings were
    /// disabled.
    ///
    /// This is subtly different from the non-template case, because template
    /// _instantiation_ actually occurs in this crate. Template _instantiation_
    /// in other headers should respect the template _definition_ and its
    /// API promises.
    #[test]
    #[ignore] // TODO(b/266727458): implement this
    fn aliases_dont_expose_disabled_templates() {
        assert!(
            !type_exists!(alias_enabled::AliasedDisabledTemplate),
            "AliasedDisabledTemplate was exported by `alias_enabled`, even though `DisabledTemplate` disabled bindings."
        );
    }

    /// This test will fail if aliases produce bindings on targets whose
    /// bindings were disabled, where the alias was to an enabled target.
    ///
    /// While Crubit _was_ enabled for the definition, the usage site also needs
    /// to consent to people depending on the type _via_ the using library,
    /// since that implies a maintenance burden.
    #[test]
    fn disabled_struct_aliases_arent_exposed() {
        assert!(
            !type_exists!(alias_disabled::AliasedEnabledStruct),
            "AliasedEnabledStruct was exported by `alias_disabled`, even though that build target disabled bindings."
        );
    }

    #[test]
    fn disabled_template_aliases_arent_exposed() {
        assert!(
            !type_exists!(alias_disabled::AliasedEnabledTemplate),
            "AliasedEnabledTemplate was exported by `alias_disabled`, even though that build target disabled bindings."
        );
    }
}
