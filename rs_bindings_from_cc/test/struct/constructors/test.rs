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
        assert_impl_all!(StructWithUserProvidedConstructor: Default);

        let s: StructWithUserProvidedConstructor = Default::default();
        assert_eq!(42, s.int_field);
    }
}
