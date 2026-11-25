// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use calls_back_to_rust::{
    bridge_to_and_from_cpp, invoke, invoke_as_absl_anyinvocable, invoke_const, invoke_fn_ref,
    invoke_fn_ref_const, invoke_once, map_abi_compatible, map_abi_compatible_fn_ref, map_int,
    map_int_fn_ref, map_layout_compatible, map_layout_compatible_fn_ref, map_optional_int,
    map_optional_int_fn_ref, ABICompatible, HasFnRefMethod, LayoutCompatible,
};
use googletest::{expect_eq, gtest};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[gtest]
fn test_invoke_once() {
    let state = Arc::new(Mutex::new(0));
    {
        let state = Arc::clone(&state);
        // Capture a `!Sync` `Cell` to verify that `FnOnce` callbacks do not require `Sync`.
        let non_sync_cell = std::cell::Cell::new(42);
        invoke_once(Box::new(move || {
            *state.lock().unwrap() = non_sync_cell.get();
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
    fn double(x: Option<i32>) -> Option<i32> {
        x.map(|x| x * 2)
    }
    let result = map_optional_int(Box::new(|x| double(x.into()).into()), Some(10).into());
    expect_eq!(Option::<i32>::from(result), Some(20));
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

#[gtest]
fn test_invoke_as_absl_anyinvocable() {
    let state = Arc::new(Mutex::new(0));
    {
        let state = Arc::clone(&state);
        invoke_as_absl_anyinvocable(Box::new(move || {
            *state.lock().unwrap() = 42;
        }));
    }

    expect_eq!(*state.lock().unwrap(), 42);
    expect_eq!(
        Arc::strong_count(&state),
        1,
        "invoke_as_absl_anyinvocable should have dropped the cloned state"
    );
}

#[gtest]
fn test_invoke_fn_ref() {
    // `rs::FnRef` does not require `Sync` or `Send`, so closures capturing `Cell` and `Rc` work.
    let state = Rc::new(Cell::new(0));
    let state_clone = Rc::clone(&state);
    invoke_fn_ref(&|| {
        state_clone.set(42);
    });
    expect_eq!(state.get(), 42);
}

#[gtest]
fn test_invoke_fn_ref_const() {
    let state = Cell::new(0);
    invoke_fn_ref_const(&|| {
        state.set(42);
    });
    expect_eq!(state.get(), 42);
}

#[gtest]
fn test_map_int_fn_ref() {
    let result = map_int_fn_ref(&|x| x * 2, 10);
    expect_eq!(result, 20);

    // passing pre-existing fn item
    fn double_int(x: i32) -> i32 {
        x * 2
    }
    let result2 = map_int_fn_ref(&double_int, 10);
    expect_eq!(result2, 20);

    // passing pre-existing &dyn Fn (without Sync)
    let f: &dyn Fn(i32) -> i32 = &|x| x * 2;
    let result3 = map_int_fn_ref(f, 10);
    expect_eq!(result3, 20);
}

#[gtest]
fn test_map_optional_int_fn_ref() {
    fn double(x: Option<i32>) -> Option<i32> {
        x.map(|x| x * 2)
    }
    let result = map_optional_int_fn_ref(&|x| double(x.into()).into(), Some(10).into());
    expect_eq!(Option::<i32>::from(result), Some(20));
}

#[gtest]
fn test_map_abi_compatible_fn_ref() {
    let result = map_abi_compatible_fn_ref(
        &|abi_compatible| ABICompatible { x: abi_compatible.x * 2 },
        ABICompatible { x: 10 },
    );
    expect_eq!(result.x, 20);
}

#[gtest]
fn test_map_layout_compatible_fn_ref() {
    let result = map_layout_compatible_fn_ref(
        &|layout_compatible| {
            LayoutCompatible::Create(2 * LayoutCompatible::get(&layout_compatible))
        },
        LayoutCompatible::Create(10),
    );
    expect_eq!(LayoutCompatible::get(&result), 20);
}

#[gtest]
fn test_has_fn_ref_method() {
    let mut obj = HasFnRefMethod { unused: 0 };
    let state = Cell::new(0);
    obj.invoke(&|| {
        state.set(42);
    });
    expect_eq!(state.get(), 42);
}
