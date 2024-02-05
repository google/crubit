// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use has_bindings::crubit::has_bindings;
use static_assertions::assert_not_impl_any;

#[test]
fn test_void_function() {
    has_bindings::crubit_void_function();
}
#[test]
fn test_void_ptr_function() {
    let value = 1;
    let ptr = &value as *const _ as *const std::ffi::c_void;
    // Safety: the pointer is valid in both C++ and Rust, and is not dereferenced.
    let result_ptr = unsafe { has_bindings::crubit_void_ptr_identity(ptr) };
    assert_eq!(ptr, result_ptr);
}

#[test]
fn test_user_struct() {
    let mut i: core::ffi::c_int = 123;
    let s = has_bindings::Struct { x: &mut i, y: 3.4, z: 0 as *mut _ };
    let s2 = unsafe { has_bindings::crubit_anystruct(s, &s) };
    assert_eq!(s2.x, &mut i as *mut core::ffi::c_int);
}

#[test]
fn test_nontrivial_struct() {
    let mut i = 0;
    {
        let s = has_bindings::NontrivialStruct { x: &mut i };
        let _s2 = s; // can still treat it like a normal Rust value!
    }
    // and the destructor gets invoked!
    assert_eq!(i, 42);
}

#[test]
fn test_user_enum() {
    let _: has_bindings::Enum = has_bindings::Enum::kEnumerator;
    // Can't really assert this due to how value_exists works, sadly.
    // assert!(!item_exists::value_exists!
    // (has_bindings::Enum::kUnkownAttrEnumerator));
}

#[test]
fn test_user_union() {
    // as close as one gets to verifying that it's a union. It is indeed a union!
    let _: has_bindings::Union = has_bindings::Union { x: 1 };
    let _: has_bindings::Union = has_bindings::Union { y: 3 };
}

#[test]
fn test_alias() {
    assert_eq!(
        std::any::TypeId::of::<has_bindings::Struct>(),
        std::any::TypeId::of::<has_bindings::StructAlias>(),
    )
}

#[test]
fn test_crubit_add() {
    assert_eq!(has_bindings::crubit_add(1, 2), 3);
}

#[test]
fn test_crubit_enum_function() {
    assert_eq!(
        has_bindings::crubit_enum_function(has_bindings::Enum::kEnumerator),
        has_bindings::Enum::kEnumerator
    );
}

#[test]
fn test_crubit_union_function() {
    let u = has_bindings::crubit_union_function(has_bindings::Union { x: 42 });
    assert_eq!(
        // SAFETY: x is initialized, because crubit_union_function is the identity function.
        unsafe { u.x },
        42
    );
}

#[test]
fn test_function_pointer() {
    extern "C" fn my_callback(a: *mut std::ffi::c_int) {
        // SAFETY: we're going to pass it a valid pointer to an integer, it's OK!
        unsafe {
            *a = 42;
        }
    }
    let f: has_bindings::Callback = my_callback;
    let mut state = 0;

    // SAFETY: this is the other half of the promise above. `state` is valid, and so
    // this is safe.
    unsafe {
        has_bindings::crubit_invoke_callback(f, &mut state);
    }
    assert_eq!(state, 42);
}

#[test]
fn test_nullable_function_pointer() {
    extern "C" fn my_callback(a: *mut std::ffi::c_int) {
        // SAFETY: we're going to pass it a valid pointer to an integer, it's OK!
        unsafe {
            *a = 42;
        }
    }
    let f: has_bindings::NullableCallback = Some(my_callback);
    let mut state = 0;

    // SAFETY: this is the other half of the promise above. `state` is valid, and so
    // this is safe.
    unsafe {
        has_bindings::crubit_invoke_nullable_callback(f, &mut state);
    }
    assert_eq!(state, 42);
}

/// You can use a class that uses inheritance, but to Rust, it looks like
/// private inheritance: the struct is only available as an opaque thunk within
/// the derived class.
#[test]
fn test_oop() {
    assert_not_impl_any!(has_bindings::MyDerivedStruct : oops::Inherits<has_bindings::Struct>);
}
