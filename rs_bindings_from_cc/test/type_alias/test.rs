// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#[test]
fn test_return_value() {
    use type_alias::return_underlying;
    use type_alias::Int;
    let i: Int = 42;
    assert_eq!(return_underlying(i), 42);
}

/// Vector aliases are not supported (yet???).
#[test]
fn test_vector_alias() {
    assert!(!item_exists::type_exists!(type_alias::MyVector));
    assert!(!item_exists::value_exists!(type_alias::VectorFunction));
}
