// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod coproduct {
    //! Coproduct types for arbitrary-arity `std::variant` (used when there are more than 8
    //! alternatives).

    use crate::std::variant_internal::{TrivialVariantTypes, VariantTypes};
    use crate::std::{trivial_variant, variant};

    /// An uninhabited type marking the end of a [`Coproduct`] type list.
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum CNil {}

    /// A recursive sum type holding either a head alternative `H` ([`Coproduct::Inl`]) or a tail
    /// coproduct `T` ([`Coproduct::Inr`]), terminating in [`CNil`].
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Coproduct<H, T> {
        /// The head alternative of this coproduct node.
        Inl(H),
        /// An alternative in the remaining tail of the coproduct list.
        Inr(T),
    }

    impl<H, T> From<variant<Coproduct<H, T>>> for Coproduct<H, T>
    where
        Coproduct<H, T>: VariantTypes<Owned = Coproduct<H, T>>,
    {
        fn from(v: variant<Coproduct<H, T>>) -> Self {
            v.into_enum()
        }
    }

    impl<H, T> From<trivial_variant<Coproduct<H, T>>> for Coproduct<H, T>
    where
        Coproduct<H, T>: TrivialVariantTypes<Owned = Coproduct<H, T>>,
    {
        fn from(v: trivial_variant<Coproduct<H, T>>) -> Self {
            v.into_enum()
        }
    }
}
