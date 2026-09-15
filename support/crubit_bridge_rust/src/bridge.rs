// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Forwarding module to `crubit_support::bridge`.

pub use crubit_support::bridge::*;
pub use crubit_support::{unstable_encode, unstable_return};

/// FFI payload for non-owning callable references (`rs::FnRef`).
///
/// This struct has a C-compatible representation matching the layout passed across
/// the C ABI boundary by Crubit's C++ `rs::FnRef`.
///
/// The fields are private to prevent unintended modification. The closure is invoked
/// across FFI by calling `invoker(data, args...)` where `data` is the raw pointer to the
/// C++ callable (or its state) and `invoker` is a trampoline function with the C ABI
/// matching the callable's signature.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FnRefPayload {
    data: *mut core::ffi::c_void,
    invoker: *mut core::ffi::c_void,
}

// SAFETY: `FnRefPayload` represents a borrowed C++ callable reference passed across
// the FFI boundary. The raw pointers (`data` and `invoker`) cannot implement `Send`
// automatically, but implementing `Send` is required to allow Rust closures wrapping
// `FnRefPayload` to satisfy trait bounds like `Send` when passed to APIs such as
// multi-threaded task runners or callbacks. The caller creating the `FnRefPayload`
// is responsible for ensuring the underlying C++ callable is safe to transfer across
// threads. [ai_genererated_and_unmodified]
unsafe impl Send for FnRefPayload {}

// SAFETY: `FnRefPayload` represents a borrowed C++ callable reference passed across
// the FFI boundary. The raw pointers (`data` and `invoker`) cannot implement `Sync`
// automatically, but implementing `Sync` is required to allow Rust closures wrapping
// `FnRefPayload` to satisfy trait bounds like `Sync` when shared across threads. The
// caller creating the `FnRefPayload` is responsible for ensuring the underlying C++
// callable is safe to access concurrently. [ai_genererated_and_unmodified]
unsafe impl Sync for FnRefPayload {}

impl FnRefPayload {
    /// Constructs a new `FnRefPayload`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `data` and `invoker` are valid for invocations
    /// throughout the lifetime of this reference payload. [ai_genererated_and_unmodified]
    pub const unsafe fn new(data: *mut core::ffi::c_void, invoker: *mut core::ffi::c_void) -> Self {
        Self { data, invoker }
    }

    /// Returns the raw user data pointer for the closure invoker.
    pub fn data(&self) -> *mut core::ffi::c_void {
        self.data
    }

    /// Returns the raw function pointer to invoke the closure.
    pub fn invoker(&self) -> *mut core::ffi::c_void {
        self.invoker
    }
}

/// FFI payload for owning callables (`rs::Fn`).
///
/// This struct has a C-compatible representation matching the layout passed across
/// the C ABI boundary by Crubit's C++ `rs::Fn`.
///
/// The fields are private to ensure safety: `FnPayload` implements [`Drop`], which calls
/// `destroyer(data)`. If `data` or `destroyer` could be modified by safe code, dropping
/// `FnPayload` would result in undefined behavior (e.g. calling arbitrary addresses or
/// freeing invalid memory).
///
/// # Invocation Protocol
///
/// 1. C++ passes `FnPayload` across the `extern "C"` thunk boundary by value.
/// 2. The generated Rust thunk wraps `FnPayload` in a Rust closure.
/// 3. When called, the closure invokes `invoker(data, args...)`, where `invoker` is a
///    trampoline with the C ABI matching the closure's signature.
/// 4. When the Rust closure is eventually dropped, `FnPayload::drop` is invoked, which calls
///    `destroyer(data)` to clean up the C++ callable stored on the C++ side.
#[repr(C)]
#[derive(Debug)]
pub struct FnPayload {
    data: *mut core::ffi::c_void,
    invoker: *mut core::ffi::c_void,
    destroyer: unsafe extern "C" fn(*mut core::ffi::c_void),
}

// SAFETY: `FnPayload` represents an owned C++ callable passed across the FFI boundary.
// The raw pointers (`data`, `invoker`, `destroyer`) cannot implement `Send` automatically,
// but implementing `Send` is required to allow Rust closures wrapping `FnPayload` to satisfy
// trait bounds like `Send` (e.g. in `std::panic::set_hook`). The caller creating the
// `FnPayload` is responsible for ensuring the underlying C++ callable is safe to transfer
// across threads. [ai_genererated_and_unmodified]
unsafe impl Send for FnPayload {}

// SAFETY: `FnPayload` represents an owned C++ callable passed across the FFI boundary.
// The raw pointers (`data`, `invoker`, `destroyer`) cannot implement `Sync` automatically,
// but implementing `Sync` is required to allow Rust closures wrapping `FnPayload` to satisfy
// trait bounds like `Sync` (e.g. in `std::panic::set_hook`). The caller creating the
// `FnPayload` is responsible for ensuring the underlying C++ callable is safe to access
// concurrently. [ai_genererated_and_unmodified]
unsafe impl Sync for FnPayload {}

impl FnPayload {
    /// Constructs a new `FnPayload`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `data` and `invoker` are valid, and that `destroyer`,
    /// when called with `data`, safely cleans up the underlying callable. [ai_genererated_and_unmodified]
    pub const unsafe fn new(
        data: *mut core::ffi::c_void,
        invoker: *mut core::ffi::c_void,
        destroyer: unsafe extern "C" fn(*mut core::ffi::c_void),
    ) -> Self {
        Self { data, invoker, destroyer }
    }

    /// Returns the raw user data pointer for the closure invoker.
    pub fn data(&self) -> *mut core::ffi::c_void {
        self.data
    }

    /// Returns the raw function pointer to invoke the closure.
    pub fn invoker(&self) -> *mut core::ffi::c_void {
        self.invoker
    }
}

impl Drop for FnPayload {
    fn drop(&mut self) {
        if !self.data.is_null() {
            // SAFETY: `data` was passed across FFI along with `destroyer` from C++
            // `rs::Fn`, which guarantees that `destroyer(data)` is valid to call once
            // upon destruction of the payload. [ai_genererated_and_unmodified]
            unsafe { (self.destroyer)(self.data) };
            self.data = core::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::expect_eq;
    use googletest::gtest;

    #[gtest]
    fn test_forwarding() {
        type Abi = (TransmuteAbi<u8>, TransmuteAbi<u8>);
        let original = (1, 2);
        let value = unsafe {
            internal::decode::<Abi>(
                Abi::default(),
                unstable_encode!(@ (transmute_abi(), transmute_abi()), Abi, original).as_ptr()
                    as *const u8,
            )
        };
        expect_eq!(value, original);
    }
}
