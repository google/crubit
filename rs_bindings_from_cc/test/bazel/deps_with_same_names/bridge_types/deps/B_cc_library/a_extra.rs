// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
pub struct TypeB {
    pub y: i32,
}

#[derive(Clone, Default)]
pub struct TypeBAbi;

// SAFETY: TypeB has a stable layout and its fields are trivially serializable.
// TypeBAbi correctly encodes and decodes TypeB by serializing its fields.
unsafe impl bridge_rust::CrubitAbi for TypeBAbi {
    type Value = TypeB;

    const SIZE: usize = 4;

    fn encode(self, value: Self::Value, encoder: &mut bridge_rust::Encoder) {
        bridge_rust::transmute_abi::<i32>().encode(value.y, encoder);
    }

    unsafe fn decode(self, decoder: &mut bridge_rust::Decoder) -> Self::Value {
        // SAFETY: Safe because decoding a primitive i32 is safe, assuming the decoder is valid.
        unsafe {
            let y = bridge_rust::transmute_abi::<i32>().decode(decoder);
            TypeB { y }
        }
    }
}
