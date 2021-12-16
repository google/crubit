// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use methods::*;

    #[test]
    fn test_static_factory_method() {
        let s: SomeClass = SomeClass::static_factory_method(123);
        assert_eq!(123, s.int_var);
    }

    #[test]
    fn test_static_method_that_multiplies_its_args() {
        assert_eq!(42 * 789, SomeClass::static_method_that_multiplies_its_args(42, 789));
    }
}
