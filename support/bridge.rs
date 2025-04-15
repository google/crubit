// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::marker::PhantomData;
use core::mem::{self, MaybeUninit};
use core::ptr;

/// A mutually understood ABI for sending bridge types between Rust and C++.
///
/// Bridging values between Rust and C++ is typically done by breaking down values into their
/// primitive, ABI-compatible parts like integers and pointers in the native language. Then, these
/// primitive parts are sent across the language boundary, where the target language can reconstruct
/// the semantically equivalent value. This typically happens by sending the parts as function
/// arguments on an extern C function, but this doesn't work for generic/templated types without
/// monomorphizing each instantiation since C doesn't have templates.
///
/// The solution to this is to transform the value into an ABI-compatible layout that both languages
/// understand, allowing for passing arbitrarily complex types through C as a char pointer to a
/// stack allocated buffer. The [`CrubitAbi`] trait is used to describe this mutually understood
/// ABI.
///
/// # Creating a bridge type
///
/// Let's walk through the example of how `Option<T>` is bridged. The first step is to define an ABI
/// for how it should be bridged, which is represented by a type that implements [`CrubitAbi`]:
///
/// ```rust
/// pub struct OptionAbi<A>(PhantomData<A>);
///
/// unsafe impl<A: CrubitAbi> CrubitAbi for OptionAbi<A> {
///     type Value = Option<A::Value>;
///
///     // todo
/// }
/// ```
///
/// This is saying "`OptionAbi<A>` is a description of how to bridge an `Option<A::Value>`." We'll
/// get back to the unsafe part later. But before we proceed, we need to decide: what will the
/// Option ABI be? Rust allows for niche optimizations, but to keep things general we'll choose to
/// bridge `Option<T>` as a bool, followed by the value if the bool is true. To express this, we
/// need to implement the other items in the trait:
///
/// ```rust
/// unsafe impl<A: CrubitAbi> CrubitAbi for OptionAbi<A> {
///     type Value = Option<A::Value>;
///
///     const SIZE: usize = mem::size_of::<bool>() + A::SIZE;
///
///     fn encode(value: Self::Value, encoder: &mut Encoder) {
///         if let Some(inner) = value {
///             encoder.encode_transmute(true);
///             encoder.encode::<A>(inner);
///         } else {
///             encoder.encode_transmute(false);
///         }
///     }
///
///     unsafe fn decode(decoder: &mut Decoder) -> Self::Value {
///         // SAFETY: the caller guarantees that the buffer contains a bool, and if the bool is true,
///         // that the buffer also contains the value.
///         unsafe {
///             if decoder.decode_transmute() {
///                 Some(decoder.decode::<A>())
///             } else {
///                 None
///             }
///         }
///     }
/// }
/// ```
///
/// There are several things going on here. First, we need to define the `SIZE` constant. This
/// information is used to statically compute the size of the buffer required to encode/decode an
/// `Option<T>` with this ABI, allowing us to stack allocate the buffer. Importantly, the current
/// implementation packs all the data with unaligned writes/reads, so alignment information is not
/// needed. Second, we need to define the `encode` and `decode` functions. These functions implement
/// the agreed-upon ABI: bool, optionally followed by the value if the bool is true.
///
/// # Safety
///
/// It's safety critical that the C++ implementation matches the Rust implementation exactly, since
/// the ABI is supposed to be mutually understood.
pub unsafe trait CrubitAbi {
    /// The type that this CrubitAbi is encoding and decoding.
    type Value;

    /// The size in bytes of a `Value` when encoded with this ABI. This is used to statically
    /// compute the size of the buffer required to encode/decode a `Value` with this ABI.
    const SIZE: usize;

    /// Encodes a `Value`, advancing the encoders's position by `SIZE` bytes.
    ///
    /// Aside from implementations for primitives, most implementations of this function will be
    /// composed of other calls to [`Encoder::encode::<A>`], for some `A: CrubitAbi`,
    /// each one advancing the encoder's position by `A::SIZE` bytes. The
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
    /// like [`Box::leak`], or defer to [`core::mem::ManuallyDrop`] and [`core::mem::forget`] if
    /// leaking APIs are unavailable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// unsafe impl<A1: CrubitAbi, A2: CrubitAbi> CrubitAbi for TupleAbi<(A1, A2)> {
    ///     fn encode((a, b): Self::Value, encoder: &mut Encoder) {
    ///         encoder.encode::<A1>(a);
    ///         encoder.encode::<A2>(b);
    ///     }
    ///     // other items omitted...
    /// }
    /// ```
    fn encode(value: Self::Value, encoder: &mut Encoder);

    /// Decodes a [`Value`], advancing the decoder's position by `SIZE` bytes.
    ///
    /// Aside from implementations for primitives, most implementations of this function will be
    /// composed of other calls to [`Decoder::decode::<A>`], for some `A: CrubitAbi`,
    /// each one advancing the decoder's position by `A::SIZE` bytes. The
    /// implementation should ensure that the these calls do not advance the decoder's position by
    /// more than `SIZE` bytes. This is because the `SIZES` constant is used to compute the buffer
    /// size statically, and if the decoder's position is advanced by more than `SIZE`, the decoder
    /// may panic in debug builds, or cause undefined behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// unsafe impl<A1: CrubitAbi, A2: CrubitAbi> CrubitAbi for TupleAbi<(A1, A2)> {
    ///     unsafe fn decode(decoder: &mut Decoder) -> Self::Value {
    ///         // SAFETY: The caller guarantees that the buffer contains an `A1::Value`, followed
    ///         // by an `A2::Value`, which is the understood ABI for a `TupleAbi<(A1, A2)>`.
    ///         unsafe {
    ///             let a = decoder.decode::<A1>();
    ///             let b = decoder.decode::<A2>();
    ///             // At this point, it would be unsafe to call decoder.decode() with anything that
    ///             // would read from the underlying buffer, since we don't know what's there.
    ///             (a, b)
    ///         }
    ///     }
    ///     // other items omitted...
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// The caller guarantees that the buffer's current position contains a `Value` that was
    /// encoded with this ABI (either from Rust or C++).
    unsafe fn decode(decoder: &mut Decoder) -> Self::Value;
}

/// A wrapper around a buffer that tracks which parts of a buffer have already been written to.
pub struct Encoder {
    // The number of bytes remaining in the buffer. We write backwards (counting down) so that
    // subtracting too much leads to underflow, which is checked in debug builds.
    remaining_bytes: usize,
    buf: *mut u8,
}

impl Encoder {
    /// Encodes a value.
    pub fn encode<A: CrubitAbi>(&mut self, value: A::Value) {
        A::encode(value, self);
    }

    /// Encodes a value via [`core::ptr::copy_nonoverlapping`].
    pub fn encode_transmute<T>(&mut self, value: T) {
        self.encode::<TransmuteAbi<T>>(value);
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
    /// Decodes a value.
    ///
    /// # Safety
    ///
    /// See [`CrubitAbi::decode`].
    pub unsafe fn decode<A: CrubitAbi>(&mut self) -> A::Value {
        // SAFETY: The caller guarantees the same invariants as [`CrubitAbi::decode`].
        unsafe { A::decode(self) }
    }

    /// Decodes a value via [`core::ptr::copy_nonoverlapping`].
    ///
    /// # Safety
    ///
    /// See [`<TransmuteAbi<T> as CrubitAbi>::decode`].
    pub unsafe fn decode_transmute<T>(&mut self) -> T {
        // SAFETY: The caller guarantees the same invariants as
        // [`<TransmuteAbi<T> as CrubitAbi>::decode`].
        unsafe { self.decode::<TransmuteAbi<T>>() }
    }
}

/// A [`CrubitAbi`] for encoding a value by transmuting it into the buffer.
pub struct TransmuteAbi<T>(PhantomData<T>);

// Every T can be passed by value.
// SAFETY: The ABI contract for `TransmuteAbi<T>` is that the raw bytes of the value `T` are memcpyd
// into the buffer, padding and all. The idea is that this is only used on types that already have
// a shared ABI between Rust and C++, like primitives and opaque types.
unsafe impl<T> CrubitAbi for TransmuteAbi<T> {
    type Value = T;

    const SIZE: usize = mem::size_of::<Self::Value>();

    fn encode(value: Self::Value, encoder: &mut Encoder) {
        // We use the fact that underflow is checked in debug builds to ensure that callers
        // don't overwrite the buffer.
        encoder.remaining_bytes -= Self::SIZE;

        // SAFETY: We have just allocated space to write the value.
        unsafe {
            ptr::write_unaligned(encoder.buf.add(encoder.remaining_bytes).cast::<T>(), value);
        }
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self::Value {
        // We use the fact that underflow is checked in debug builds to ensure that callers
        // don't overwrite the buffer.
        decoder.remaining_bytes -= Self::SIZE;

        // SAFETY: The caller guarantees that the buffer contains a T.
        unsafe { ptr::read_unaligned(decoder.buf.add(decoder.remaining_bytes).cast::<T>()) }
    }
}

/// A [`CrubitAbi`] for encoding a tuple by encoding each element in order with its corresponding
/// `CrubitAbi`.
pub struct TupleAbi<A>(PhantomData<A>);

macro_rules! unsafe_impl_crubit_abi_for_tuple_abi {
    { $( unsafe impl CrubitAbi for TupleAbi<( $($A:ident,)*)>; )* } => {
        $(
            // SAFETY: The bridge schema for a tuple is the same in C++: each element of the tuple
            // is encoded in order with the corresponding schema.
            unsafe impl<$($A: CrubitAbi),*> CrubitAbi for TupleAbi<($($A,)*)> {
                type Value = ( $($A::Value,)* );

                const SIZE: usize = 0 $( + $A::SIZE )*;

                 fn encode(( $($A,)* ): Self::Value, encoder: &mut Encoder) {
                    #![allow(non_snake_case)]
                    #![allow(unused_variables)] // for `encoder` in () case
                    $(
                        encoder.encode::<$A>($A);
                    )*
                }

                unsafe fn decode(decoder: &mut Decoder) -> Self::Value {
                    #![allow(clippy::unused_unit)] // for () case
                    #![allow(unused_variables)] // for `decoder` in () case

                    // SAFETY: the caller guarantees that the buffer contains each element of
                    // the tuple with the correct schema.
                    ( $( unsafe { decoder.decode::<$A>() },)* )
                }
            }
        )*
    }
}

// Every tuple can be passed by bridge. Add more impls here if needed.
// SAFETY: The ABI contract for `TupleAbi<T>` is that the elements of the tuple are encoded in order
// with the corresponding `CrubitAbi`s.
unsafe_impl_crubit_abi_for_tuple_abi! {
    unsafe impl CrubitAbi for TupleAbi<()>;
    unsafe impl CrubitAbi for TupleAbi<(A1,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3, A4,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3, A4, A5,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3, A4, A5, A6,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3, A4, A5, A6, A7,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3, A4, A5, A6, A7, A8,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3, A4, A5, A6, A7, A8, A9,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11,)>;
    unsafe impl CrubitAbi for TupleAbi<(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12,)>;
}

/// A [`CrubitAbi`] for encoding an `Option` by encoding a bool followed by the value if the bool
/// is true.
pub struct OptionAbi<A>(PhantomData<A>);

// SAFETY: The ABI contract for `OptionAbi<T>` is that the value is encoded as follows:
// bool, optionally followed by the value if the bool is true.
unsafe impl<A: CrubitAbi> CrubitAbi for OptionAbi<A> {
    type Value = Option<A::Value>;

    const SIZE: usize = mem::size_of::<bool>() + A::SIZE;

    fn encode(value: Self::Value, encoder: &mut Encoder) {
        if let Some(inner) = value {
            encoder.encode_transmute(true);
            encoder.encode::<A>(inner);
        } else {
            encoder.encode_transmute(false);
        }
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self::Value {
        // SAFETY: the caller guarantees that the buffer contains a bool, and if the bool is true,
        // that the buffer also contains the value.
        unsafe {
            if decoder.decode_transmute() {
                Some(decoder.decode::<A>())
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
    pub unsafe fn encode<A: CrubitAbi>(buf: *mut u8, value: A::Value) {
        A::encode(value, &mut Encoder { remaining_bytes: A::SIZE, buf });
    }

    /// Decodes a value from a buffer.
    ///
    /// This function is only intended to be called by Crubit generated code.
    ///
    /// # Safety
    ///
    /// `buf` must point to a buffer that is at least `<T as CrubitAbi<S>>::SIZE` bytes large, and must
    /// contain a `T` that was encoded with the same schema `S`.
    pub unsafe fn decode<A: CrubitAbi>(buf: *const u8) -> A::Value {
        // SAFETY: The caller guarantees that the buffer contains a `T` that was encoded with schema `S`.
        unsafe { A::decode(&mut Decoder { remaining_bytes: A::SIZE, buf }) }
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
        type Abi = TupleAbi<(TransmuteAbi<u8>, TransmuteAbi<u8>)>;

        let original = (1, 2);

        let mut buf = internal::empty_buffer::<{ Abi::SIZE }>();
        // SAFETY: the buffer is large enough to hold a T encoded as A.
        unsafe {
            internal::encode::<Abi>(buf.as_mut_ptr() as *mut u8, original);
        }
        // SAFETY: the buffer contains a T encoded as A.
        let value = unsafe { internal::decode::<Abi>(buf.as_ptr() as *const u8) };
        expect_eq!(value, original);
    }

    #[gtest]
    fn test_encode_decode_stuff() {
        type Abi = TupleAbi<(
            OptionAbi<TupleAbi<(TransmuteAbi<i64>, TransmuteAbi<bool>)>>,
            TupleAbi<(TransmuteAbi<u8>, TransmuteAbi<f32>)>,
        )>;

        let original = (Some((-8, true)), (1, 2.0));

        let mut buf = internal::empty_buffer::<{ Abi::SIZE }>();
        // SAFETY: the buffer is large enough to hold a T encoded as A.
        unsafe {
            internal::encode::<Abi>(buf.as_mut_ptr() as *mut u8, original);
        }
        // SAFETY: the buffer contains a T encoded as A.
        let value = unsafe { internal::decode::<Abi>(buf.as_ptr() as *const u8) };
        expect_eq!(value, original);
    }
}
