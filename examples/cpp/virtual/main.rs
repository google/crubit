// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::pin::Pin;

fn main() {
    use ctor::CtorNew;
    let derived = ctor::emplace!(example_lib::RustDerived::ctor_new(definition::RustDerived::new(
        definition::SomeRustSubclass
    )));
    let result = unsafe {
        base::GetMethod1(example_lib::RustDerived::Upcast(
            Pin::into_inner_unchecked(derived) as *mut _
        ))
    };
    assert_eq!(result, 42);
}
