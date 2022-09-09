// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use inline::*;

    #[test]
    fn test_inline_namespaces() {
        let s = foo::inline1::MyStruct { value: 123 };
        assert_eq!(123, foo::inline1::GetStructValue1(&s));
        assert_eq!(123, foo::inline1::GetStructValue2(&s));
        assert_eq!(123, foo::inline1::GetStructValue3(&s));
        assert_eq!(123, foo::inline1::GetStructValue4(&s));

        // Notably, the C++ standard library uses `inline` namespaces, but we
        // still want to be able to refer to `std::string`, rather than
        // `std::__u::string`. Therefore the test verifies that the
        // inner types and functions are also available in the parent
        // namespace.
        //
        // TODO(b/244601795): Add test coverage below.
        // > // `foo::MyStruct` should be a type alias for
        // > // `foo::inline1::MyStruct`.
        // > let mut s2 = foo::MyStruct { value: 456 };
        // > s2 = s;
        // > // The functions should be available as `foo::GetStructValue...`
        // > // as well.
        // > assert_eq!(123, foo::GetStructValue1(&s));
        // > assert_eq!(123, foo::GetStructValue2(&s));
    }
}
