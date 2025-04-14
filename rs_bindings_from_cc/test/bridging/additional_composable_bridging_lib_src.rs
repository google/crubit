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

pub struct Vec3Abi<A>(core::marker::PhantomData<A>);

unsafe impl<A: CrubitAbi> CrubitAbi for Vec3Abi<A> {
    type Value = Vec3<A::Value>;

    const SIZE: usize = A::SIZE * 3;

    fn encode(value: Self::Value, encoder: &mut Encoder) {
        encoder.encode::<A>(value.x);
        encoder.encode::<A>(value.y);
        encoder.encode::<A>(value.z);
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self::Value {
        unsafe {
            let x = decoder.decode::<A>();
            let y = decoder.decode::<A>();
            let z = decoder.decode::<A>();
            Vec3 { x, y, z }
        }
    }
}
