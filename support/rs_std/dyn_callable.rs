// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{transmute_abi, CrubitAbi, Decoder, Encoder};
use core::ptr;

#[track_caller]
pub fn moved_from_panic() -> ! {
    core::panic!("moved-from value");
}

/// The [`CrubitAbi`] for a type `F`, where `F` is one of `dyn Fn`, `dyn FnMut`, or `dyn FnOnce`.
pub struct DynCallableAbi<F: ?Sized> {
    fallback: Box<F>,
}

impl<F: ?Sized> DynCallableAbi<F> {
    /// Returns a new `DynCallableAbi` instance.
    ///
    /// `fallback` will be used in decoding if C++ returns a moved-from value.
    pub fn new(fallback: Box<F>) -> Self {
        DynCallableAbi { fallback }
    }
}

/// SAFETY: The ABI contract for `DynCallableAbi<F>` varies between Rust -> C++, and C++ -> Rust.
///
/// When sending from Rust to C++, the value is encoded as `Box<dyn F>`.
///
/// When sending from C++ to Rust, the value is encoded as a bool indicating whether the value is
/// present. If present, the bool is followed by the `Box<dyn F>`.
unsafe impl<F: ?Sized> CrubitAbi for DynCallableAbi<F> {
    type Value = Box<F>;

    const SIZE: usize = 24;

    fn encode(self, value: Self::Value, encoder: &mut Encoder) {
        transmute_abi().encode(value, encoder);
    }

    unsafe fn decode(self, decoder: &mut Decoder) -> Self::Value {
        // SAFETY: When receiving from C++, the first value is a bool indicating whether the value
        // is present.
        let present = unsafe { transmute_abi().decode(decoder) };
        if present {
            // SAFETY: present is true, so the value is present.
            unsafe { transmute_abi().decode(decoder) }
        } else {
            self.fallback
        }
    }
}

/// A Rust equivalent of `FunctionToCall` used internally in absl::AnyInvocable.
// The variants are never constructed in Rust, they are only constructed in C++.
#[allow(dead_code)]
#[repr(u8)]
pub enum FunctionToCall {
    Dispose = 0,
    RelocateFromTo = 1,
    /// This is used right before passing an AnyInvocable to Rust.
    /// See b/463647332 for more details.
    RelocateFromToAndQueryRust = 2,
}

/// Implementation of the manager function, which has the same signature of
/// `absl::internal_any_invocable::ManagerType`.
///
/// # Safety
///
/// `from` must be valid for reads. If `operation` is RelocateFromTo or RelocateFromToAndQueryRust,
/// then `to` must be valid for writes, and be distinct from `from`. Additionally, if `operation` is
/// RelocateFromToAndQueryRust, then `to` must also point to a `*mut bool` which can be written to.
pub unsafe fn manager<F: ?Sized>(operation: FunctionToCall, from: *mut Box<F>, to: *mut Box<F>) {
    match operation {
        FunctionToCall::Dispose => {
            // SAFETY: The caller guarantees that `from` is valid for reads.
            unsafe { ptr::drop_in_place(from) }
        }
        FunctionToCall::RelocateFromTo => {
            // SAFETY: The caller guarantees that `from` is valid for reads and `to` is
            // valid for writes.
            unsafe { ptr::copy_nonoverlapping(from, to, 1) };
        }
        FunctionToCall::RelocateFromToAndQueryRust => {
            // This case happens right before passing an AnyInvocable to Rust. When it
            // happens, the caller guarantees that `to` points to a location that is not
            // only valid for writes, but also holds a pointer to a bool. C++ will never
            // touch this pointer, but the Rust manager function (us!) is expected to write
            // true to it.
            unsafe {
                let is_rust = to as *mut *mut bool;
                **is_rust = true;
            }
            // SAFETY: The caller guarantees that `from` is valid for reads and `to` is
            // valid for writes.
            unsafe { ptr::copy_nonoverlapping(from, to, 1) };
        }
    }
}
