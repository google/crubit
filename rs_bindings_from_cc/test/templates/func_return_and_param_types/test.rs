// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use func_return_and_param_types::*;

    // This tests whether Crubit supports template specialization/instantiation in a
    // function return type, or in a function parameter type - see b/228868369.
    #[test]
    fn test_template_instantiation_in_return_value_and_parameter_type() {
        // Note that the Rust code below never needs to refer to the
        // mangled name of the Rust struct that the class template
        // specialization/instantiation gets translated to.

        // Class template instantiation used as a function return type.
        let s = CreateInstanceOfMyTemplate(123);
        assert_eq!(123, *s.value());

        // Const-ref to class template instantiation used as a function parameter type.
        let d = DoubleInstanceOfMyTemplate(&s);
        assert_eq!(123 * 2, d);
    }
}
