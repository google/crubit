// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{CrubitAbi, Decoder, Encoder};

pub struct Bridge<T> {
    pub value: T,
}

#[derive(Clone, Default)]
pub struct BridgeAbi<A>(pub A);

unsafe impl<A: CrubitAbi> CrubitAbi for BridgeAbi<A> {
    type Value = Bridge<A::Value>;

    const SIZE: usize = A::SIZE;

    fn encode(self, value: Self::Value, encoder: &mut Encoder) {
        self.0.encode(value.value, encoder);
    }

    unsafe fn decode(self, decoder: &mut Decoder) -> Self::Value {
        unsafe { Bridge { value: self.0.decode(decoder) } }
    }
}
