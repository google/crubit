// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use bridge_rust::{CrubitAbi, Decoder, Encoder};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub struct EitherAbi<LeftAbi, RightAbi>(core::marker::PhantomData<(LeftAbi, RightAbi)>);

unsafe impl<LeftAbi, RightAbi> CrubitAbi for EitherAbi<LeftAbi, RightAbi>
where
    LeftAbi: CrubitAbi,
    RightAbi: CrubitAbi,
{
    type Value = Either<LeftAbi::Value, RightAbi::Value>;

    const SIZE: usize = core::mem::size_of::<bool>()
        + if LeftAbi::SIZE > RightAbi::SIZE { LeftAbi::SIZE } else { RightAbi::SIZE };

    fn encode(value: Self::Value, encoder: &mut Encoder) {
        match value {
            Either::Left(inner) => {
                encoder.encode_transmute(true);
                encoder.encode::<LeftAbi>(inner);
            }
            Either::Right(inner) => {
                encoder.encode_transmute(false);
                encoder.encode::<RightAbi>(inner);
            }
        }
    }

    unsafe fn decode(decoder: &mut Decoder) -> Self::Value {
        unsafe {
            if decoder.decode_transmute() {
                Either::Left(decoder.decode::<LeftAbi>())
            } else {
                Either::Right(decoder.decode::<RightAbi>())
            }
        }
    }
}
