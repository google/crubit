// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use calls_back_to_rust::{
    bridge_to_and_from_cpp, invoke, invoke_const, invoke_once, map_abi_compatible, map_int,
    map_layout_compatible, map_optional_int, ABICompatible, LayoutCompatible,
};
use googletest::{expect_eq, gtest};
use std::sync::{Arc, Mutex};

#[gtest]
fn test_invoke_once() {
    let state = Arc::new(Mutex::new(0));
    {
        let state = Arc::clone(&state);
        invoke_once(Box::new(move || {
            *state.lock().unwrap() = 42;
        }));
    }

    expect_eq!(*state.lock().unwrap(), 42);
    expect_eq!(Arc::strong_count(&state), 1, "invoke_once should have dropped the cloned state");
}

#[gtest]
fn test_invoke() {
    let state = Arc::new(Mutex::new(0));
    {
        let state = Arc::clone(&state);
        invoke(Box::new(move || {
            *state.lock().unwrap() = 42;
        }));
    }

    expect_eq!(*state.lock().unwrap(), 42);
    expect_eq!(Arc::strong_count(&state), 1, "invoke should have dropped the cloned state");
}

#[gtest]
fn test_invoke_const() {
    let state = Arc::new(Mutex::new(0));
    {
        let state = Arc::clone(&state);
        invoke_const(Box::new(move || {
            *state.lock().unwrap() = 42;
        }));
    }

    expect_eq!(*state.lock().unwrap(), 42);
    expect_eq!(Arc::strong_count(&state), 1, "invoke should have dropped the cloned state");
}

#[gtest]
fn test_map_int() {
    let result = map_int(Box::new(|x| x * 2), 10);
    expect_eq!(result, 20);
}

#[gtest]
fn test_map_optional_int() {
    let result = map_optional_int(Box::new(|x| x.map(|x| x * 2)), Some(10));
    expect_eq!(result, Some(20));
}

#[gtest]
fn test_map_abi_compatible() {
    let result = map_abi_compatible(
        Box::new(|abi_compatible| ABICompatible { x: abi_compatible.x * 2 }),
        ABICompatible { x: 10 },
    );
    expect_eq!(result.x, 20);
}

#[gtest]
fn test_map_layout_compatible() {
    let result = map_layout_compatible(
        Box::new(|layout_compatible| {
            LayoutCompatible::Create(2 * LayoutCompatible::get(&layout_compatible))
        }),
        LayoutCompatible::Create(10),
    );
    expect_eq!(LayoutCompatible::get(&result), 20);
}

#[gtest]
fn test_bridge_to_and_from_cpp() {
    let state = Arc::new(Mutex::new(0));
    let f = {
        let state = Arc::clone(&state);
        bridge_to_and_from_cpp(Box::new(move || {
            *state.lock().unwrap() = 42;
        }))
    };

    expect_eq!(*state.lock().unwrap(), 0);
    expect_eq!(Arc::strong_count(&state), 2, "The function hasn't been invoked yet");

    f();

    expect_eq!(*state.lock().unwrap(), 42);
    expect_eq!(Arc::strong_count(&state), 1, "The function has been invoked");
}
