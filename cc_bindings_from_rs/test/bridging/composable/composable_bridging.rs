// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{transmute_abi, CrubitAbi, Decoder, Encoder};
use cc_std::std::raw_string_view;
use std::mem;

pub fn maybe_int() -> MyOptionRust<i32> {
    MyOptionRust(Some(4))
}

pub fn maybe_string_view() -> MyOptionRust<raw_string_view> {
    MyOptionRust(None)
}

pub fn maybe_int_slice() -> MyOptionRust<*const [i32]> {
    MyOptionRust(Some(&[1, 2, 3][..] as *const [_]))
}

pub fn assert_none(x: MyOptionRust<i32>) {
    assert_eq!(x.0, None);
}

pub fn assert_some_5(x: MyOptionRust<i32>) {
    assert_eq!(x.0, Some(5));
}

pub fn assert_some_some_5(x: MyOptionRust<MyOptionRust<i32>>) {
    assert_eq!(x.0.and_then(|x| x.0), Some(5));
}

pub fn option_slice_without_first(x: MyOptionRust<&[i32]>) -> MyOptionRust<&[i32]> {
    let Some(slice) = x.0 else {
        return MyOptionRust(None);
    };
    let Some((_first, rest)) = slice.split_first() else {
        return MyOptionRust(None);
    };
    MyOptionRust(Some(rest))
}

pub fn option_adds_one_to_ref(x: MyOptionRust<&mut i32>) -> MyOptionRust<&mut i32> {
    MyOptionRust(x.0.map(|x| {
        *x += 1;
        x
    }))
}

#[crubit_annotate::cpp_bridge(
    cpp_type = "std::optional",
    bridge_abi_cpp = "crubit::OptionAbi",
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
