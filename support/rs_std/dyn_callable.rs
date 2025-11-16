// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{transmute_abi, CrubitAbi, Decoder, Encoder};
use core::marker::PhantomData;
use core::{mem, ptr};

/// The [`CrubitAbi`] for a `Box<dyn Trait>` type, where `Trait` is one of `Fn`, `FnMut`, or
/// `FnOnce`.
pub struct DynCallableAbi<F> {
    _marker: PhantomData<F>,
}

impl<F> DynCallableAbi<F> {
    /// Returns a new `DynCallableAbi` instance.
    pub fn new() -> Self {
        DynCallableAbi { _marker: PhantomData }
    }
}

impl<F> Default for DynCallableAbi<F> {
    fn default() -> Self {
        DynCallableAbi::new()
    }
}

/// SAFETY: The ABI contract for `DynCallableAbi<F>` is that the value is encoded as follows:
/// the `Option<Box<dyn Trait>` (size 16, align 8 blob), followed by 8 more bytes containing the
/// manager function pointer.
unsafe impl<F> CrubitAbi for DynCallableAbi<F> {
    type Value = F;

    const SIZE: usize = mem::size_of::<Option<F>>() + 8;

    fn encode(self, value: Self::Value, encoder: &mut Encoder) {
        /// Implementation of the manager function, which has the same signature of
        /// `absl::internal_any_invocable::ManagerType`. For ABI compatibility, we use a bool
        /// instead of a `FunctionToCall` type, and have renamed it to `do_dispose`. This aligns
        /// with the C++ `FunctionToCall` enum, where `relocate_from_to` is false, and `dispose` is
        /// true.
        ///
        /// # Safety
        ///
        /// `from` must be valid for reads. If `do_dispose` is false, then `to` must be valid for
        /// writes.
        unsafe extern "C" fn relocate_or_dispose<F>(
            do_dispose: bool,
            from: *mut Option<F>,
            to: *mut Option<F>,
        ) {
            // SAFETY: The caller guarantees that `from` is valid for reads.
            let from = unsafe { Option::take(&mut *from) };
            if !do_dispose {
                // SAFETY: The caller guarantees that `to` is valid for writes with `do_dispose` is
                // set to false.
                unsafe { ptr::write(to, from) };
            }
        }

        transmute_abi().encode(Some(value), encoder);
        transmute_abi().encode(
            relocate_or_dispose::<F> as unsafe extern "C" fn(bool, *mut Option<F>, *mut Option<F>),
            encoder,
        );
    }

    unsafe fn decode(self, decoder: &mut Decoder) -> Self::Value {
        // SAFETY: The Crubit ABI contract ensures that the Option<Box<dyn Trait>> is encoded first.
        unsafe {
            transmute_abi::<Option<F>>()
                .decode(decoder)
                .expect("DynCallable should not be in the moved-from state")
        }
    }
}
