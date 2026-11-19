// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Flat projection enums for `std::variant`.

/// Submodule containing flat projection enums for variant types.
pub mod variant_enums {
    use crate::std::trivial_variant;
    use crate::std::variant;
    use crate::std::variant_internal::{TrivialVariantTypes, VariantTypes};

    macro_rules! define_variant_enums {
        ($($VariantN:ident, $len:expr; $($T:ident, $V:ident),*);* $(;)?) => {
            $(
                #[doc = concat!("Projection enum for a variant with ", stringify!($len), " alternative(s).")]
                #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub enum $VariantN<$($T),*> {
                    $($V($T)),*
                }
                impl<$($T),*> From<variant<($($T,)*)>> for $VariantN<$($T),*>
                where ($($T,)*): VariantTypes<Owned = $VariantN<$($T),*>> {
                    fn from(v: variant<($($T,)*)>) -> Self {
                        v.into_enum()
                    }
                }
                impl<$($T),*> From<trivial_variant<($($T,)*)>> for $VariantN<$($T),*>
                where ($($T,)*): TrivialVariantTypes<Owned = $VariantN<$($T),*>> {
                    fn from(v: trivial_variant<($($T,)*)>) -> Self {
                        v.into_enum()
                    }
                }
            )*
        }
    }

    define_variant_enums! {
        Variant1, 1; T0, V0;
        Variant2, 2; T0, V0, T1, V1;
        Variant3, 3; T0, V0, T1, V1, T2, V2;
        Variant4, 4; T0, V0, T1, V1, T2, V2, T3, V3;
        Variant5, 5; T0, V0, T1, V1, T2, V2, T3, V3, T4, V4;
        Variant6, 6; T0, V0, T1, V1, T2, V2, T3, V3, T4, V4, T5, V5;
        Variant7, 7; T0, V0, T1, V1, T2, V2, T3, V3, T4, V4, T5, V5, T6, V6;
        Variant8, 8; T0, V0, T1, V1, T2, V2, T3, V3, T4, V4, T5, V5, T6, V6, T7, V7;
    }
}
