// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use item_exists::{type_exists, value_exists};

mod definitions {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn disabled_struct_has_no_bindings() {
        // Disabled crate shouldn't get bindings at all. If it does, the type should not exist.
        #[cfg(definition_disabled)]
        assert!(
            !type_exists!(definition_disabled::DisabledStruct),
            "definition_disabled::DisabledStruct was exposed through bindings."
        );
    }
    #[gtest]
    fn enabled_struct_has_bindings() {
        assert!(
            type_exists!(definition_enabled::EnabledStruct),
            "definition_enabled::EnabledStruct was not found in bindings."
        );
    }
}

mod aliases {
    use super::*;
    use googletest::prelude::*;

    /// This test will fail if aliases expose a struct whose bindings were
    /// disabled.
    #[gtest]
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
    #[gtest]
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
    #[gtest]
    fn disabled_struct_aliases_arent_exposed() {
        // Disabled crate shouldn't get bindings at all. If it does, the type should not exist.
        #[cfg(alias_disabled)]
        assert!(
            !type_exists!(alias_disabled::AliasedEnabledStruct),
            "AliasedEnabledStruct was exported by `alias_disabled`, even though that build target disabled bindings."
        );
    }

    #[gtest]
    fn disabled_template_aliases_arent_exposed() {
        // Disabled crate shouldn't get bindings at all. If it does, the type should not exist.
        #[cfg(alias_disabled)]
        assert!(
            !type_exists!(alias_disabled::AliasedEnabledTemplate),
            "AliasedEnabledTemplate was exported by `alias_disabled`, even though that build target disabled bindings."
        );
    }
}

mod functions {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_functions_disabled_when_parameter_types_are() {
        assert!(!value_exists!(func_enabled::FuncTakesDisabledStruct));
        assert!(!value_exists!(func_enabled::FuncTakesDisabledTemplate));
    }

    #[gtest]
    fn test_functions_disabled_when_return_type_is() {
        assert!(!value_exists!(func_enabled::FuncReturnsDisabledStruct));
        assert!(!value_exists!(func_enabled::FuncReturnsDisabledTemplate));
    }
}

mod structs {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_struct_enabled() {
        // The generated field assertions will handle the rest.
        assert!(type_exists!(wrapper_struct_enabled::EnabledStructWithDisabledField));
    }
}
