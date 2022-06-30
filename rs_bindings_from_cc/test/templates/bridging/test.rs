// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    // TODO(b/236037915): Rename `IncompleteCast` into something less focused on
    // forward declarations (maybe `IncompleteCast` => `CcCast` and
    // `forward_declare.rs` => `cc_cast.rs`?).
    use forward_declare::IncompleteCast;

    #[test]
    fn test_bridging() {
        let x = class_template_instantiation1::Create(123);

        // GetValue below expects a reference to template instantiation from the
        // `class_template_instantiation2` create, but `x` is from the
        // `class_template_instantiation1` crate instead.  Because of that an
        // explicit cast is required.
        let v = class_template_instantiation2::GetValue((&x).incomplete_cast());

        assert_eq!(123, v);
    }
}
