// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Trait implemented by Rust callers that want to be a subclass.
pub trait BaseTrait {
    fn rust_method1(&self, this: &base::ExampleBase) -> ffi_11::c_int;
}

/// A dispatching type useed by the C++ derived class to dispatch to Rust "subclasses".
#[derive(Default)]
pub struct RustDerived {
    inner: Option<Box<dyn BaseTrait>>,
}

impl RustDerived {
    pub fn new(inner: impl BaseTrait + 'static) -> Self {
        Self { inner: Some(Box::new(inner)) }
    }

    pub fn rust_method1(&self, this: &base::ExampleBase) -> ffi_11::c_int {
        let Some(inner) = &self.inner else {
            panic!("Called rust_method1 on a moved-from RustImpl");
        };
        inner.rust_method1(this)
    }
}

/// Example Rust implementation.
pub struct SomeRustSubclass;

impl BaseTrait for SomeRustSubclass {
    fn rust_method1(&self, this: &base::ExampleBase) -> ffi_11::c_int {
        // SAFETY: `this` is a valid pointer to a `ExampleBase` object of the appropriate lifetime.
        unsafe { base::ExampleBase::Method2(this) * 42 }
    }
}
