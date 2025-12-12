// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{transmute_abi, CrubitAbi, Decoder, Encoder};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Clone, Default)]
pub struct EitherAbi<LeftAbi, RightAbi>(pub LeftAbi, pub RightAbi);

unsafe impl<LeftAbi, RightAbi> CrubitAbi for EitherAbi<LeftAbi, RightAbi>
where
    LeftAbi: CrubitAbi,
    RightAbi: CrubitAbi,
{
    type Value = Either<LeftAbi::Value, RightAbi::Value>;

    const SIZE: usize = core::mem::size_of::<bool>()
        + if LeftAbi::SIZE > RightAbi::SIZE { LeftAbi::SIZE } else { RightAbi::SIZE };

    fn encode(self, value: Self::Value, encoder: &mut Encoder) {
        match value {
            Either::Left(inner) => {
                transmute_abi().encode(true, encoder);
                self.0.encode(inner, encoder);
            }
            Either::Right(inner) => {
                transmute_abi().encode(false, encoder);
                self.1.encode(inner, encoder);
            }
        }
    }

    unsafe fn decode(self, decoder: &mut Decoder) -> Self::Value {
        unsafe {
            if transmute_abi().decode(decoder) {
                Either::Left(self.0.decode(decoder))
            } else {
                Either::Right(self.1.decode(decoder))
            }
        }
    }
}
