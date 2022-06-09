// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    #[test]
    fn test_const_ref_to_self() {
        // Among other things, this test provides coverage against infinite
        // recursion around Importer::ConvertTemplateSpecializationType which
        // in the past might have happened when a member function's parameters
        // would refer back to the same class template specialization.
        let s1 = method_params::MyTypeAlias::Create(1);
        let s2 = method_params::MyTypeAlias::Create(2);
        assert_eq!(1 + 2, s1.AddOneOtherItem(&s2));
    }

    fn test_repeating_parameter_type() {
        // Among other things, this test provides coverage for the (not currently
        // implemented, but still considered for the future) mangling algorithm
        // that is required as part of the "Function template" approach from
        // `thunks_for_class_template_member_functions.md`. In particular,
        // repeated parameter type should be subject to https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangling-compression
        // (which was missed in the initial prototype of this approach).
        let s1 = method_params::MyTypeAlias::Create(1);
        let s2 = method_params::MyTypeAlias::Create(2);
        let s3 = method_params::MyTypeAlias::Create(3);
        let s4 = method_params::MyTypeAlias::Create(4);
        assert_eq!(1 + 2 + 3 + 4, s1.AddThreeOtherItems(&s2, &s3, &s4));
    }
}
