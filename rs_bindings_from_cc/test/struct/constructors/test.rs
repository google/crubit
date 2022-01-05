// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[macro_use]
extern crate static_assertions;

#[cfg(test)]
mod tests {
    use constructors::*;

    #[test]
    fn test_user_provided_constructors() {
        assert_impl_all!(StructWithUserProvidedConstructors: From<i32>);
        assert_impl_all!(StructWithUserProvidedConstructors: Default);

        let s: StructWithUserProvidedConstructors = Default::default();
        assert_eq!(42, s.int_field);

        let i: StructWithUserProvidedConstructors = 123.into();
        assert_eq!(123, i.int_field);
    }

    #[test]
    fn test_deleted_constructors() {
        assert_not_impl_all!(StructWithDeletedConstructor: Default);
    }

    #[test]
    fn test_private_constructors() {
        assert_not_impl_all!(StructWithPrivateConstructor: Default);
    }

    #[test]
    fn test_explicitly_defaulted_constructor() {
        assert_impl_all!(StructWithExplicitlyDefaultedConstructor: Default);

        // Default constructor should set uninitialized fields to 0 (real testing here
        // depends somewhat on MSan).
        let s: StructWithExplicitlyDefaultedConstructor = Default::default();
        assert_eq!(0, s.field_with_no_initializer);
        assert_eq!(123, s.field_with_explicit_initializer);
    }

    #[test]
    fn test_nontrivial_struct() {
        assert_not_impl_all!(NonTrivialStructWithConstructors: Default);
    }
}
