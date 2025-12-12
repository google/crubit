// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{transmute_abi, CrubitAbi, Decoder, Encoder};
use std::mem;

pub fn returns_some_int() -> Option<i32> {
    Some(42)
}

pub fn returns_no_int() -> Option<i32> {
    None
}

pub fn unwrap_or_zero(x: Option<i32>) -> i32 {
    x.unwrap_or(0)
}

pub fn option_increments(x: Option<i32>) -> Option<i32> {
    x.map(|x| x + 1)
}

///CRUBIT_ANNOTATE: cpp_type=std::optional
///CRUBIT_ANNOTATE: bridge_abi_cpp=crubit::OptionalAbi
///CRUBIT_ANNOTATE: bridge_abi_rust=MyOptionRustAbi
pub struct MyOptionRust<T>(Option<T>);

pub fn make_my_option_rust() -> MyOptionRust<i32> {
    MyOptionRust(Some(42))
}

pub fn maybe_int_slice() -> Option<*const [i32]> {
    Some(&[1, 2, 3][..] as *const [i32])
}

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
