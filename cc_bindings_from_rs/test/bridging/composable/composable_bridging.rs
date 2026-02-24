// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{transmute_abi, CrubitAbi, Decoder, Encoder};
use cc_std::std::raw_string_view;
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

pub fn assert_none(x: Option<i32>) {
    assert_eq!(x, None);
}

pub fn assert_some_5(x: Option<i32>) {
    assert_eq!(x, Some(5));
}

pub fn assert_some_some_5(x: Option<Option<i32>>) {
    assert_eq!(x, Some(Some(5)));
}

pub fn option_slice_without_first(x: Option<&[i32]>) -> Option<&[i32]> {
    let (_first, rest) = x?.split_first()?;
    Some(rest)
}

pub fn option_adds_one_to_ref(x: Option<&mut i32>) -> Option<&mut i32> {
    x.map(|x| {
        *x += 1;
        x
    })
}

#[crubit_annotate::cpp_bridge(
    cpp_type = "std::optional",
    bridge_abi_cpp = "crubit::OptionalAbi",
    bridge_abi_rust = "MyOptionRustAbi"
)]
pub struct MyOptionRust<T>(Option<T>);

#[derive(Clone, Default)]
pub struct MyOptionRustAbi<A>(pub A);

unsafe impl<A: CrubitAbi> CrubitAbi for MyOptionRustAbi<A> {
    type Value = MyOptionRust<A::Value>;

    const SIZE: usize = mem::size_of::<bool>() + A::SIZE;

    fn encode(self, value: Self::Value, encoder: &mut Encoder) {
        if let Some(inner) = value.0 {
            transmute_abi().encode(true, encoder);
            self.0.encode(inner, encoder);
        } else {
            transmute_abi().encode(false, encoder);
        }
    }

    unsafe fn decode(self, decoder: &mut Decoder) -> Self::Value {
        // SAFETY: the caller guarantees that the buffer contains a bool, and if the bool is true,
        // that the buffer also contains the value.
        unsafe {
            if transmute_abi().decode(decoder) {
                MyOptionRust(Some(self.0.decode(decoder)))
            } else {
                MyOptionRust(None)
            }
        }
    }
}
