// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use common::*;
use googletest::prelude::*;
use vector_lib::*;

/// Implemented for every `(Self, T)` pair, but if `Self == T`, there are two applicable impls,
/// making the `Marker` parameter ambiguous and causing a compile error ("type annotations needed").
///
/// (Comparing `TypeId`s in a `const` context would be more direct, but `PartialEq` is not yet
/// stable as a const trait.)
trait NotSameType<T, Marker> {}
impl<A, B> NotSameType<B, ()> for A {}
impl<A> NotSameType<A, u8> for A {}

/// Statically asserts that the return type of `f` is not `T`.
const fn assert_return_type_is_not<T, R: NotSameType<T, Marker>, Marker>(_f: fn() -> R) {}

#[gtest]
fn test_vector_wrapped_by_value_as_function_arg_and_return_value() {
    let mut v: cc_std::std::vector<i32> = MakeVector(1);
    let r = unsafe { UseVectorByRef(&mut v) };
    let v = UseVectorByValue(v);
    assert_eq!(v, 1);
    assert_eq!(r, 1);
}

/// std::vector<std::string> is supported because std::string is layout-compatible with string.
#[gtest]
fn test_vector_string() {
    let v: cc_std::std::vector<cc_std::std::string> = vector_lib::MakeVectorString();
    let slice: &[cc_std::std::string] = &*v;
    assert_eq!(slice.len(), 1);
    assert_eq!(&*slice[0], &b"hello, world"[..]);
}

/// MakeVectorBool gets bindings via template instantiation, but it must not be the Rust vector
/// reimplementation, because of the vector<bool> specialization.
const _: () =
    assert_return_type_is_not::<cc_std::std::vector<bool>, _, _>(vector_lib::MakeVectorBool);

/// MakeVectorOverloadedDelete gets bindings via template instantiation, but it must not be the
/// Rust vector reimplementation, because of the overloaded operator delete.
/// TODO(b/571224171): This could be made clearer.
const _: () = assert_return_type_is_not::<cc_std::std::vector<OverloadedDelete>, _, _>(
    vector_lib::MakeVectorOverloadedDelete,
);

/// MakeVectorOverloadedDestroyingDelete gets bindings via template instantiation, but it must not
/// be the Rust vector reimplementation, because of the overloaded operator delete.
const _: () = assert_return_type_is_not::<cc_std::std::vector<OverloadedDestroyingDelete>, _, _>(
    vector_lib::MakeVectorOverloadedDestroyingDelete,
);

/// MakeVectorPolymorphicType gets bindings via template instantiation, but it must not be the
/// Rust vector reimplementation, because it can call a derived class's overloaded operator
/// delete, but the Rust reimplementation will not.
const _: () = assert_return_type_is_not::<cc_std::std::vector<PolymorphicType>, _, _>(
    vector_lib::MakeVectorPolymorphicType,
);

#[gtest]
fn test_vector_final_type() {
    let _: cc_std::std::vector<FinalType> = vector_lib::MakeVectorFinalType();
}

#[gtest]
fn test_vector_deleted_destructor() {
    assert!(!item_exists::value_exists!(vector_lib::MakeVectorDeletedDestructor))
}

#[gtest]
fn test_vector_no_bindings() {
    assert!(!item_exists::value_exists!(vector_lib::MakeUniquePtrNoBindings))
}
