// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{CrubitAbi, Decoder, Encoder};

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Default)]
pub struct Vec3Abi<A>(pub A);

unsafe impl<A: CrubitAbi + Clone> CrubitAbi for Vec3Abi<A> {
    type Value = Vec3<A::Value>;

    const SIZE: usize = A::SIZE * 3;

    fn encode(self, value: Self::Value, encoder: &mut Encoder) {
        self.0.clone().encode(value.x, encoder);
        self.0.clone().encode(value.y, encoder);
        self.0.encode(value.z, encoder);
    }

    unsafe fn decode(self, decoder: &mut Decoder) -> Self::Value {
        unsafe {
            let x = self.0.clone().decode(decoder);
            let y = self.0.clone().decode(decoder);
            let z = self.0.decode(decoder);
            Vec3 { x, y, z }
        }
    }
}
