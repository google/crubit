// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Compile-time tests of the API generated for the `:isolated` target.
//!
//! `:isolated` instantiates a non-functional stub of `absl::flat_hash_map` (see `flat_hash_map.h`)
//! with a variety of element types, so calling the generated methods would be meaningless. Instead,
//! these tests check, in the type system, which instantiations get the `map::Map` API, with which
//! element types, and that the API is also reachable as inherent methods without importing `Map`.
//! The runtime behavior of the generated methods is tested against a working fake in
//! `fake_test.rs`.

use googletest::gtest;
use isolated::crubit::test::{
    CopyOnly, MapWithBridgedKey, MapWithBridgedValue, MapWithCopyOnlyKey, MapWithCopyOnlyValue,
    MapWithNoMoveNoCopyKey, MapWithNoMoveNoCopyValue, MapWithSliceRefValue, MapWithSpanValue,
    MapWithStrRefValue, MapWithStringViewKey, MapWithStringViewValue, MapWithThreeParams,
    MapWithTwoParams,
};
use map::Map;
use static_assertions::{assert_impl_all, assert_not_impl_any, assert_type_eq_all};

/// Fails to compile unless `M` implements `Map` with key type `K` and value type `V`, and gives
/// out `&mut V` (rather than `Pin<&mut V>`) for mutable access to values.
fn assert_unpin_value_map<M, K, V>()
where
    M: for<'a> Map<Key = K, Value = V, ValueMut<'a> = &'a mut V>,
{
}

#[gtest]
fn third_template_parameter_is_accepted() {
    assert_type_eq_all!(MapWithTwoParams, MapWithThreeParams);
}

#[gtest]
fn primitive_elements() {
    assert_unpin_value_map::<MapWithTwoParams, i32, u64>();
}

/// Elements that only have a copy constructor are moved into the map by copying.
#[gtest]
fn copy_only_elements() {
    assert_unpin_value_map::<MapWithCopyOnlyKey, CopyOnly, i32>();
    assert_unpin_value_map::<MapWithCopyOnlyValue, i32, CopyOnly>();
}

/// `std::string_view` and `rs_std::SliceRef` are accessed through their lifetime-less
/// layout-compatible Rust types.
#[gtest]
fn lifetime_less_view_elements() {
    assert_impl_all!(MapWithStringViewKey: Map);
    assert_impl_all!(MapWithStringViewValue: Map);
    assert_unpin_value_map::<MapWithSliceRefValue, i32, *const [i32]>();
}

/// Maps whose elements can't be accessed in place or moved into the map are still bound, but
/// without the `Map` API.
#[gtest]
fn unsupported_elements() {
    // Non-layout-compatible bridged types.
    assert_not_impl_any!(MapWithBridgedKey: Map);
    assert_not_impl_any!(MapWithBridgedValue: Map);
    // View types whose Rust types have lifetimes (b/478868411).
    assert_not_impl_any!(MapWithSpanValue: Map);
    assert_not_impl_any!(MapWithStrRefValue: Map);
    // Types with neither a move nor a copy constructor.
    assert_not_impl_any!(MapWithNoMoveNoCopyKey: Map);
    assert_not_impl_any!(MapWithNoMoveNoCopyValue: Map);
}

/// The `Map` API is also exposed as inherent methods, which can be called without importing
/// `Map`. This module deliberately doesn't import `Map`, so these only compile if the inherent
/// methods exist and have the expected signatures.
mod inherent_methods {
    use core::pin::Pin;
    use googletest::gtest;
    use isolated::crubit::test::{CopyOnly, MapWithCopyOnlyValue, MapWithTwoParams as M};

    #[gtest]
    fn available_without_importing_map() {
        let _: fn(&M) -> usize = M::len;
        let _: fn(&M) -> bool = M::is_empty;
        let _: fn(&M) -> usize = M::capacity;
        let _: fn(Pin<&mut M>) = M::clear;
        let _: fn(Pin<&mut M>, usize) = M::reserve;
        let _: fn(&M, &i32) -> bool = M::contains_key;
        let _: for<'a> fn(&'a M, &i32) -> Option<&'a u64> = M::get;
        let _: for<'a> fn(&'a M, &i32) -> Option<(&'a i32, &'a u64)> = M::get_key_value;
        let _: for<'a> fn(Pin<&'a mut M>, &i32) -> Option<&'a mut u64> = M::get_mut;
        let _: fn(Pin<&mut M>, &i32) -> bool = M::remove;
    }

    /// `insert` and `try_insert` take `impl Ctor` arguments, so they can't be coerced to function
    /// pointers. This is never called; it only needs to compile.
    fn _insert_and_try_insert_compile(mut map: Pin<&mut MapWithCopyOnlyValue>) {
        let _: (&i32, &mut CopyOnly) = map.as_mut().insert(1, CopyOnly::default());
        let _ = map.as_mut().try_insert(1, CopyOnly::default());
    }
}
