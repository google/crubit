// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::*;

pub struct MyStringView {
    pub view: cc_std::std::string_view,
}

// SAFETY: The bridge schema for a MyStringView is the same in C++: just the string_view by value.
unsafe impl CrubitAbi<ByBridge<()>> for MyStringView {
    const SIZE: usize = core::mem::size_of::<cc_std::std::string_view>();

    fn encode(self, encoder: &mut Encoder) {
        encoder.encode_transmute(self.view);
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self {
        let view = unsafe { decoder.decode_transmute() };
        MyStringView { view }
    }
}

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

unsafe impl<T: CrubitAbi<S>, S> CrubitAbi<ByBridge<S>> for Vec3<T> {
    const SIZE: usize = <T as CrubitAbi<S>>::SIZE * 3;

    fn encode(self, encoder: &mut Encoder) {
        encoder.encode::<T, S>(self.x);
        encoder.encode::<T, S>(self.y);
        encoder.encode::<T, S>(self.z);
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self {
        unsafe {
            let x = decoder.decode::<T, S>();
            let y = decoder.decode::<T, S>();
            let z = decoder.decode::<T, S>();
            Vec3 { x, y, z }
        }
    }
}
