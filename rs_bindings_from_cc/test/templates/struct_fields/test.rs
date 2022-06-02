// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use ctor::CtorNew as _;
    use struct_fields::*;

    // This tests whether Crubit supports template specialization/instantiation in a
    // struct field - see b/228868369.
    #[test]
    fn test_template_instantiation_in_return_value_and_parameter_type() {
        // Note that the Rust code below never needs to refer to the
        // mangled name of the Rust struct that the class template
        // specialization/instantiation gets translated to.

        // Class template instantiation used as a type of a public field.
        ctor::emplace! {
            let s = MyStruct::ctor_new(123);
        }
        assert_eq!(123, *s.public_field.value());
    }
}
