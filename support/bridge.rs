// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::mem::{self, MaybeUninit};
use core::ptr;

// TODO(b/403623652): Document these schemas better.

/// Schema for encoding a value by calling [`core::ptr::write_aligned`] into the buffer.
pub struct ByTransmute;

/// Schema for encoding a value by bridging it.
pub struct ByBridge<S>(core::marker::PhantomData<S>);

/// A wrapper around a buffer that tracks which parts of a buffer have already been written to.
pub struct Encoder {
    // The number of bytes remaining in the buffer. We write backwards (counting down) so that
    // subtracting too much leads to underflow, which is checked in debug builds.
    remaining_bytes: usize,
    buf: *mut u8,
}

impl Encoder {
    /// Encodes a value by a provided schema.
    pub fn encode<T: CrubitAbi<S>, S>(&mut self, value: T) {
        CrubitAbi::<S>::encode(value, self)
    }

    /// Encodes a value via [`core::ptr::copy_nonoverlapping`].
    pub fn encode_transmute<T: CrubitAbi<ByTransmute>>(&mut self, value: T) {
        self.encode::<T, ByTransmute>(value);
    }
}

/// A wrapper around a buffer that tracks which parts of a buffer have already been read from.
pub struct Decoder {
    // The number of bytes remaining in the buffer. We read backwards (counting down) so that
    // subtracting too much leads to underflow, which is checked in debug builds.
    remaining_bytes: usize,
    buf: *const u8,
}

impl Decoder {
    /// Decodes a value by a provided schema.
    ///
    /// # Safety
    ///
    /// See [`CrubitAbi::<S>::decode`].
    pub unsafe fn decode<T: CrubitAbi<S>, S>(&mut self) -> T {
        // SAFETY: The caller guarantees the same invariants as [`CrubitAbi::<S>::decode`].
        unsafe { CrubitAbi::<S>::decode(self) }
    }

    /// Decodes a value via [`core::ptr::copy_nonoverlapping`].
    ///
    /// # Safety
    ///
    /// See [`CrubitAbi::<ByTransmute>::decode`].
    pub unsafe fn decode_transmute<T: CrubitAbi<ByTransmute>>(&mut self) -> T {
        // SAFETY: The caller guarantees the same invariants as [`CrubitAbi::<ByTransmute>::decode`].
        unsafe { self.decode::<T, ByTransmute>() }
    }
}

/// A type that can be bridged between Rust and C++ according to a provided schema `S`.
///
/// If you have a concrete type `T` that should be bridged, simply implement
/// `CrubitAbi<ByBridge<()>>`. If you have a generic type `T<T1, T2, ...Tn>` that should be bridged,
/// it's a bit more involved since the schema for each generic argument must be generically
/// provided. To do this, you should instead write:
///
/// ```rust
/// unsafe impl<T1, T2, ..., Tn, S1, S2, ..., Sn>
///     CrubitAbi<ByBridge<(S1, S2, ..., Sn)>> for T<T1, T2, ..., Tn>
/// where
///   T1: CrubitAbi<S1>,
///   T2: CrubitAbi<S2>,
///   ...,
///   Tn: CrubitAbi<Sn> { ... }
/// ```
///
/// As a side note, if there's exactly one generic argument, you should implement
/// `CrubitAbi<ByBridge<S>>`, not `CrubitAbi<ByBridge<(S,)>>`.
///
/// The implementation of `CrubitAbi` should ensure that the `SIZE` constant is greater than or
/// equal to the number of bytes required to encode/decode `T` for the given schema `S`. If this
/// invariant is not upheld, the encode and decode functions may panic in debug builds, or cause
/// undefined behavior in release builds.
///
/// # Safety
///
/// The implementation of `CrubitAbi<S>` must encode and decode the type exactly the same way as the
/// associated implementation of the `CrubitAbiTrait` in C++.
pub unsafe trait CrubitAbi<S> {
    /// The `SIZE` of a type `T` implementing `CrubitAbi<S>` defines the upper bound for how many
    /// bytes `T` could take up when encoded with `<T as CrubitAbi<S>>::encode`.
    const SIZE: usize;

    /// Encodes a value by a provided schema, advancing the encoders's position by `SIZE` bytes.
    ///
    /// Aside from implementations for primitives, most implementations of this function will be
    /// composed of other calls to [`Encoder::encode::<T, S>`], for some `T: CrubitAbi<S>` and `S`,
    /// each one advancing the encoder's position by `<T as CrubitAbi<S>>::SIZE` bytes. The
    /// implementation should ensure that the these calls do not advance the encoder's position by
    /// more than `SIZE` bytes. This is because the `SIZES` constant is used to compute the buffer
    /// size statically, and if the encoder's position is advanced by more than `SIZE`, the encoder
    /// may panic in debug builds, or cause undefined behavior in release builds.
    ///
    /// # Notes
    ///
    /// The value must be semantically moved into the encoder. This means that if you're
    /// transferring ownership of anything, you must ensure that the original owner leaks the
    /// resource so it can later be reclaimed by decoding. Prefer functions that explicitly leak,
    /// like [`Box::leak`], or defer to [`core::mem::ManuallyDrop`] if leaking APIs are unavailable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// unsafe impl<T1: CrubitAbi<S1>, T2: CrubitAbi<S2>, S1, S2>
    ///     CrubitAbi<ByBridge<(S1, S2)>> for (T1, T2)
    /// {
    ///     const SIZE: usize = <T1 as CrubitAbi<S1>>::SIZE + <T2 as CrubitAbi<S2>>::SIZE;
    ///
    ///     fn encode(self, encoder: &mut Encoder) {
    ///         let (a, b) = self;
    ///         encoder.encode::<T1, S1>(a);
    ///         encoder.encode::<T2, S2>(b);
    ///     }
    ///
    ///     unsafe fn decode(decoder: &mut Decoder) -> Self {
    ///         // SAFETY: The caller guarantees that the buffer contains both `a` and `b` with the
    ///         // correct schemas.
    ///         unsafe {
    ///             let a = decoder.decode::<T1, S1>();
    ///             let b = decoder.decode::<T2, S2>();
    ///             // At this point, it would be unsafe to call decoder.decode() with anything that
    ///             // would read from the underlying buffer, since we don't know what's there.
    ///             (a, b)
    ///         }
    ///     }
    /// }
    /// ```
    fn encode(self, encoder: &mut Encoder);

    /// Decodes a value by a provided schema, advancing the decoder's position by `SIZE` bytes.
    ///
    /// Aside from implementations for primitives, most implementations of this function will be
    /// composed of other calls to [`Decoder::decode::<T, S>`], for some `T: CrubitAbi<S>` and `S`,
    /// each one advancing the decoder's position by `<T as CrubitAbi<S>>::SIZE` bytes. The
    /// implementation should ensure that the these calls do not advance the decoder's position by
    /// more than `SIZE` bytes. This is because the `SIZES` constant is used to compute the buffer
    /// size statically, and if the decoder's position is advanced by more than `SIZE`, the decoder
    /// may panic in debug builds, or cause undefined behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// unsafe impl<T1: CrubitAbi<S1>, T2: CrubitAbi<S2>, S1, S2>
    ///     CrubitAbi<ByBridge<(S1, S2)>> for (T1, T2)
    /// {
    ///     const SIZE: usize = <T1 as CrubitAbi<S1>>::SIZE + <T2 as CrubitAbi<S2>>::SIZE;
    ///
    ///     fn encode(self, encoder: &mut Encoder) {
    ///         let (a, b) = self;
    ///         encoder.encode::<T1, S1>(a);
    ///         encoder.encode::<T2, S2>(b);
    ///     }
    ///
    ///     unsafe fn decode(decoder: &mut Decoder) -> Self {
    ///         // SAFETY: The caller guarantees that the buffer contains both `a` and `b` with the
    ///         // correct schemas.
    ///         unsafe {
    ///             let a = decoder.decode::<T1, S1>();
    ///             let b = decoder.decode::<T2, S2>();
    ///             // At this point, it would be unsafe to call decoder.decode() with anything that
    ///             // would read from the underlying buffer, since we don't know what's there.
    ///             (a, b)
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// The caller guarantees that the buffer contains a `T` that was encoded with the same schema
    /// `S`.
    unsafe fn decode(decoder: &mut Decoder) -> Self;
}

// Every T can be passed by value.
// SAFETY: The by-transmute schema is the same in C++, just a memcpy.
unsafe impl<T> CrubitAbi<ByTransmute> for T {
    const SIZE: usize = mem::size_of::<T>();

    fn encode(self, encoder: &mut Encoder) {
        // We use the fact that underflow is checked in debug builds to ensure that callers
        // don't overwrite the buffer.
        encoder.remaining_bytes -= mem::size_of::<T>();

        // SAFETY: We have just "allocated" space to write the value.
        unsafe {
            ptr::write_unaligned(encoder.buf.add(encoder.remaining_bytes).cast::<T>(), self);
        }
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self {
        // We use the fact that underflow is checked in debug builds to ensure that callers
        // don't overwrite the buffer.
        decoder.remaining_bytes -= mem::size_of::<T>();
        // SAFETY: The caller guarantees that the buffer contains a T.
        unsafe { ptr::read_unaligned(decoder.buf.add(decoder.remaining_bytes).cast::<T>()) }
    }
}

// SAFETY: The bridge schema for a slice pointer is the same in C++: the pointer, followed by the
// length, both as pointer sized integers.
unsafe impl<T> CrubitAbi<ByBridge<()>> for *const [T] {
    const SIZE: usize = mem::size_of::<[usize; 2]>();

    fn encode(self, encoder: &mut Encoder) {
        encoder.encode_transmute([(self as *const T).expose_provenance(), self.len()]);
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self {
        // SAFETY: the caller guarantees that the buffer contains a *const T and a usize.
        unsafe {
            let [addr, len] = decoder.decode_transmute::<[usize; 2]>();
            // This function is safe, but actually using the return value is unsafe.
            core::ptr::slice_from_raw_parts(core::ptr::with_exposed_provenance(addr), len)
        }
    }
}

// SAFETY: The bridge schema for a slice pointer is the same in C++: the pointer, followed by the
// length, both as pointer sized integers.
unsafe impl<T> CrubitAbi<ByBridge<()>> for *mut [T] {
    const SIZE: usize = mem::size_of::<[usize; 2]>();

    fn encode(self, encoder: &mut Encoder) {
        encoder.encode_transmute([(self as *mut T).expose_provenance(), self.len()]);
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self {
        // SAFETY: the caller guarantees that the buffer contains a *mut T and a usize.
        unsafe {
            let [addr, len] = decoder.decode_transmute::<[usize; 2]>();
            // This function is safe, but actually using the return value is unsafe.
            core::ptr::slice_from_raw_parts_mut(core::ptr::with_exposed_provenance_mut(addr), len)
        }
    }
}

macro_rules! impl_crubit_abi_for_tuple {
    { $( impl CrubitAbi<ByBridge<( $($S:tt,)* )>> for ( $($T:tt,)*); )* } => {
        $(
            // SAFETY: The bridge schema for a tuple is the same in C++: each element of the tuple
            // is encoded in order with the corresponding schema.
            unsafe impl<$($T : CrubitAbi<$S>,)* $($S,)*> CrubitAbi<ByBridge<( $($S,)* )>> for ($($T,)*) {
                const SIZE: usize = 0 $( + <$T as CrubitAbi<$S>>::SIZE )*;

                 fn encode(self, encoder: &mut Encoder) {
                    #![allow(non_snake_case)]
                    #![allow(unused_variables)] // for () case
                    let ($($T,)*) = self;
                    $(
                        encoder.encode::<$T, $S>($T);
                    )*
                }

                unsafe fn decode(decoder: &mut Decoder) -> Self {
                    #![allow(non_snake_case)]
                    #![allow(unused_variables)] // for () case
                    #![allow(clippy::unused_unit)] // for () case
                    $(
                        // SAFETY: the caller guarantees that the buffer contains each element of
                        // the tuple with the correct schema.
                        let $T = unsafe { decoder.decode::<$T, $S>() };
                    )*
                    ($($T,)*)
                }
            }
        )*
    }
}

// Every tuple can be passed by bridge. Add more impls here if needed.
impl_crubit_abi_for_tuple! {
    impl CrubitAbi<ByBridge<()>> for ();
    impl CrubitAbi<ByBridge<(S1,)>> for (T1,);
    impl CrubitAbi<ByBridge<(S1, S2,)>> for (T1, T2,);
    impl CrubitAbi<ByBridge<(S1, S2, S3,)>> for (T1, T2, T3,);
    impl CrubitAbi<ByBridge<(S1, S2, S3, S4,)>> for (T1, T2, T3, T4,);
    impl CrubitAbi<ByBridge<(S1, S2, S3, S4, S5,)>> for (T1, T2, T3, T4, T5,);
    impl CrubitAbi<ByBridge<(S1, S2, S3, S4, S5, S6,)>> for (T1, T2, T3, T4, T5, T6,);
    impl CrubitAbi<ByBridge<(S1, S2, S3, S4, S5, S6, S7,)>> for (T1, T2, T3, T4, T5, T6, T7,);
    impl CrubitAbi<ByBridge<(S1, S2, S3, S4, S5, S6, S7, S8,)>> for (T1, T2, T3, T4, T5, T6, T7, T8,);
    impl CrubitAbi<ByBridge<(S1, S2, S3, S4, S5, S6, S7, S8, S9,)>> for (T1, T2, T3, T4, T5, T6, T7, T8, T9,);
    impl CrubitAbi<ByBridge<(S1, S2, S3, S4, S5, S6, S7, S8, S9, S10,)>> for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,);
    impl CrubitAbi<ByBridge<(S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11,)>> for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,);
    impl CrubitAbi<ByBridge<(S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12,)>> for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,);
}

// SAFETY: The bridge schema for an Option is the same in C++: a bool, followed by the value if the
// bool is true.
unsafe impl<T: CrubitAbi<S>, S> CrubitAbi<ByBridge<S>> for Option<T> {
    const SIZE: usize = mem::size_of::<bool>() + <T as CrubitAbi<S>>::SIZE;

    fn encode(self, encoder: &mut Encoder) {
        if let Some(value) = self {
            encoder.encode_transmute(true);
            encoder.encode(value);
        } else {
            encoder.encode_transmute(false);
        }
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self {
        // SAFETY: the caller guarantees that the buffer contains a bool, and if the bool is true,
        // that the buffer also contains the value.
        unsafe {
            if decoder.decode_transmute() {
                Some(decoder.decode())
            } else {
                None
            }
        }
    }
}

/// Internal functions and types for Crubit generated code.
#[doc(hidden)]
pub mod internal {
    use super::*;

    /// Encodes a value into a buffer.
    ///
    /// This function is only intended to be called by Crubit generated code.
    ///
    /// # Safety
    ///
    /// `buf` must point to a buffer that is large enough to hold the encoded value. The exact size
    /// written can be determined by `<T as CrubitAbi<S>>::SIZE`.
    pub unsafe fn encode<T: CrubitAbi<S>, S>(buf: *mut u8, value: T) {
        Encoder::encode::<T, S>(
            &mut Encoder { remaining_bytes: <T as CrubitAbi<S>>::SIZE, buf },
            value,
        );
    }

    /// Decodes a value from a buffer.
    ///
    /// This function is only intended to be called by Crubit generated code.
    ///
    /// # Safety
    ///
    /// `buf` must point to a buffer that is at least `<T as CrubitAbi<S>>::SIZE` bytes large, and must
    /// contain a `T` that was encoded with the same schema `S`.
    pub unsafe fn decode<T: CrubitAbi<S>, S>(buf: *const u8) -> T {
        // SAFETY: The caller guarantees that the buffer contains a `T` that was encoded with schema `S`.
        unsafe { Decoder { remaining_bytes: <T as CrubitAbi<S>>::SIZE, buf }.decode::<T, S>() }
    }

    /// Helper function that returns an empty buffer to reduce noise in the generated code.
    ///
    /// This function is intended to be used by Crubit generated code.
    pub const fn empty_buffer<const BYTES: usize>() -> [MaybeUninit<u8>; BYTES] {
        const UNINIT: MaybeUninit<u8> = MaybeUninit::uninit();
        [UNINIT; BYTES]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_encode_decode_u8_pair() {
        type T = (u8, u8);
        type S = ByBridge<(ByTransmute, ByTransmute)>;

        let original = (1, 2);

        let mut buf = internal::empty_buffer::<{ <T as CrubitAbi<S>>::SIZE }>();
        // SAFETY: the buffer is large enough to hold a T encoded as S.
        unsafe {
            internal::encode::<T, S>(buf.as_mut_ptr() as *mut u8, original);
        }
        // SAFETY: the buffer contains a T encoded as S.
        let value = unsafe { internal::decode::<T, S>(buf.as_ptr() as *const u8) };
        expect_eq!(value, original);
    }

    #[gtest]
    fn test_encode_decode_stuff() {
        type T = (Option<(i64, bool)>, (u8, f32));
        type S = ByBridge<(
            ByBridge<ByBridge<(ByTransmute, ByTransmute)>>,
            ByBridge<(ByTransmute, ByTransmute)>,
        )>;

        let original: T = (Some((-8, true)), (1, 2.0));

        let mut buf = internal::empty_buffer::<{ <T as CrubitAbi<S>>::SIZE }>();
        // SAFETY: the buffer is large enough to hold a T encoded as S.
        unsafe {
            internal::encode::<T, S>(buf.as_mut_ptr() as *mut u8, original);
        }
        // SAFETY: the buffer contains a T encoded as S.
        let value = unsafe { internal::decode::<T, S>(buf.as_ptr() as *const u8) };
        expect_eq!(value, original);
    }
}
