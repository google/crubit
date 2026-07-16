// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::sync::Arc;

// 1. Struct wrapping `Arc<()>` to observably test C++ destructor / FFI drop execution.
#[derive(Clone, Default)]
pub struct ArcWrapper {
    arc: Arc<()>,
}

impl ArcWrapper {
    pub fn refcount(&self) -> usize {
        Arc::strong_count(&self.arc)
    }
}

impl Drop for ArcWrapper {
    fn drop(&mut self) {}
}

// 2. Function returning an unboxed `impl Drop` whose underlying type (`ArcWrapper`)
// implements `Drop`.
pub fn return_impl_drop(wrapper: &ArcWrapper) -> impl Drop + 'static {
    wrapper.clone()
}

// 3. Function returning `impl Future` whose underlying future captures an `ArcWrapper` (and thus implements `Drop`).
pub fn return_impl_future_with_drop(
    wrapper: &ArcWrapper,
) -> impl core::future::Future<Output = ()> + 'static {
    let wrapper = wrapper.clone();
    async move {
        let _ = wrapper;
    }
}

// 4. Function returning `impl Future` with trivial destructor.
pub fn return_impl_future_trivial(x: i32) -> impl core::future::Future<Output = i32> + 'static {
    core::future::ready(x)
}

// 5. Function returning `impl Iterator` whose underlying type (`ArcWrapper`)
// implements `Drop`.
pub fn return_impl_iterator_with_drop(wrapper: &ArcWrapper) -> impl Iterator<Item = ()> + 'static {
    let wrapper = wrapper.clone();
    std::iter::from_fn(move || {
        let _ = &wrapper;
        None
    })
}

// 6. Function returning `impl Iterator` with trivial destructor.
pub fn return_impl_iterator_trivial(x: i32) -> impl Iterator<Item = i32> + 'static {
    0..x
}
