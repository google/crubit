// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{transmute_abi, CrubitAbi, Decoder, Encoder};
use core::{mem, ptr};

#[track_caller]
pub fn moved_from_panic() -> ! {
    core::panic!("moved-from value");
}

/// An optional `Box<dyn Trait>` where the null variant can safetly be transmuted to from zeroed
/// memory, allowing it to be safely constructed from other languages.
///
/// # Why not `Option<Box<dyn Trait>>`?
///
/// Today, `Option<Box<dyn Trait>>` can be transmuted to from zeroed memory, but this behavior is
/// not guaranteed by the language, though it is unlikely to change. Interestingly, the `bytemuck`
/// crate relies on this behavior to implement `ZeroableInOption` for `Box<T>` for all `T: ?Sized`.
///
///   * https://docs.rs/bytemuck/latest/bytemuck/trait.ZeroableInOption.html#impl-ZeroableInOption-for-Box%3CT%3E
///
/// One of the reasons it could change is if rustc_abi decided to support composite data types
/// (e.g. `Box<dyn Trait>`) having more than one niche. If this were the case, then `Box<dyn Trait>`
/// would have two niches, and `Option<Box<dyn Trait>>` would only use up one, leaving a final niche
/// for the outer `Option` to use. If this were the case, it would be invalid to clobber the niche
/// with a zero.
///
/// # Guaranteeing a single niche.
///
/// By adding extra state to the null variant of an enum, we guarantee that the space cannot ever be
/// reused as a second niche, as we are allowed to write anything in that space, including zero.
/// This, along with some const assertions below to verify the size, ensures that the in memory
/// representation and niches are exactly what we expect.
pub enum ZeroableCallable<F: ?Sized> {
    Callable(Box<F>),
    #[doc(hidden)]
    Zeros {
        _deny_plausible_second_niche: usize,
    },
}

impl<F: ?Sized> Default for ZeroableCallable<F> {
    fn default() -> Self {
        ZeroableCallable::Zeros { _deny_plausible_second_niche: 0 }
    }
}

impl<F: ?Sized> ZeroableCallable<F> {
    const _ASSERTIONS: () = {
        assert!(mem::size_of::<Box<F>>() == 16, "Box<F> should be a wide pointer.");
        assert!(mem::size_of::<Self>() <= 16, "ZeroableCallable<F> should be at most 16 bytes, otherwise it means the Box nullptr niche is not being used, breaking a safety invariant.");
        assert!(
            mem::size_of::<Option<Self>>() > 16,
            "ZeroableCallable<F> should not have any niches."
        );
    };

    #[track_caller]
    pub fn unwrap_take(&mut self) -> Box<F> {
        match mem::take(self) {
            ZeroableCallable::Callable(boxed) => boxed,
            ZeroableCallable::Zeros { .. } => moved_from_panic(),
        }
    }

    #[track_caller]
    pub fn unwrap_ref(&self) -> &F {
        match self {
            ZeroableCallable::Callable(boxed) => boxed,
            ZeroableCallable::Zeros { .. } => moved_from_panic(),
        }
    }

    #[track_caller]
    pub fn unwrap_mut(&mut self) -> &mut F {
        match self {
            ZeroableCallable::Callable(boxed) => boxed,
            ZeroableCallable::Zeros { .. } => moved_from_panic(),
        }
    }
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

/// SAFETY: The ABI contract for `DynCallableAbi<F>` is that the value is encoded as follows:
/// the `Option<Box<dyn Trait>` (size 16, align 8 blob), followed by 8 more bytes containing the
/// manager function pointer.
unsafe impl<F: ?Sized> CrubitAbi for DynCallableAbi<F> {
    type Value = Box<F>;

    const SIZE: usize = 24;

    fn encode(self, value: Self::Value, encoder: &mut Encoder) {
        /// A Rust equivalent of `FunctionToCall` used internally in absl::AnyInvocable.
        // The variants are never constructed in Rust, they are only constructed in C++.
        #[allow(dead_code)]
        #[repr(u8)]
        enum FunctionToCall {
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
        /// `from` must be valid for reads. If `operation` is RelocateFromTo or
        /// RelocateFromToAndQueryRust, then `to` must be valid for writes. If `operation` is
        /// RelocateFromToAndQueryRust, then `to` must also point to a `*mut bool` which can be
        /// written to.
        unsafe extern "C" fn manager<T>(operation: FunctionToCall, from: *mut T, to: *mut T) {
            // SAFETY: The caller guarantees that `from` is valid for reads.
            let from = unsafe { ptr::read(from) };
            match operation {
                FunctionToCall::Dispose => drop(from),
                FunctionToCall::RelocateFromTo => {
                    // SAFETY: The caller guarantees that `to` is valid for writes.
                    unsafe { ptr::write(to, from) };
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
                    // SAFETY: The caller guarantees that `to` is valid for writes.
                    unsafe { ptr::write(to, from) };
                }
            }
        }

        transmute_abi().encode(Some(value), encoder);
        transmute_abi().encode(
            manager::<ZeroableCallable<F>>
                as unsafe extern "C" fn(
                    FunctionToCall,
                    *mut ZeroableCallable<F>,
                    *mut ZeroableCallable<F>,
                ),
            encoder,
        );
    }

    unsafe fn decode(self, decoder: &mut Decoder) -> Self::Value {
        // SAFETY: The Crubit ABI contract ensures that the ZeroableCallable<dyn Trait> is encoded
        // first.
        match unsafe { transmute_abi().decode(decoder) } {
            ZeroableCallable::Callable(boxed) => boxed,
            ZeroableCallable::Zeros { .. } => self.fallback,
        }
    }
}
