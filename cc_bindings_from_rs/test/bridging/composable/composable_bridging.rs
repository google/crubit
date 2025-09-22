// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{CrubitAbi, Decoder, Encoder};
use cc_std::std::raw_string_view;
use std::marker::PhantomData;
use std::mem;

pub fn maybe_int() -> Option<i32> {
    Some(4)
}

pub fn maybe_string_view() -> Option<raw_string_view> {
    None
}

pub fn maybe_int_slice() -> Option<*const [i32]> {
    Some(&[1, 2, 3][..] as *const [_])
}

#[crubit_annotate::cpp_bridge(
    cpp_type = "std::optional",
    bridge_abi_cpp = "crubit::OptionalAbi",
    bridge_abi_rust = "MyOptionRustAbi"
)]
pub struct MyOptionRust<T>(Option<T>);

pub struct MyOptionRustAbi<T>(PhantomData<T>);

unsafe impl<A: CrubitAbi> CrubitAbi for MyOptionRustAbi<A> {
    type Value = MyOptionRust<A::Value>;

    const SIZE: usize = mem::size_of::<bool>() + A::SIZE;

    fn encode(value: Self::Value, encoder: &mut Encoder) {
        if let Some(inner) = value.0 {
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
                MyOptionRust(Some(decoder.decode::<A>()))
            } else {
                MyOptionRust(None)
            }
        }
    }
}
