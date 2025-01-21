// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{ensure, Result};

flagset::flags! {
    /// An "expanded" version of CrubitFeature that includes specific cc_bindings_from_rs features.
    /// This allows them to be converted into more readable error messages: rather than simply
    /// stating "<big thing> requires experimental", we can say it requires experimental because
    /// it needs e.g. "references".
    pub enum FineGrainedFeature : u8 {
        References,
        LifetimeReuse,
        PossibleMutableAliasing,
        NonFreeReferenceParams,
        EscapeCppReservedKeyword,
        RustChar,
    }
}

impl FineGrainedFeature {
    pub fn ensure_crubit_feature(
        self,
        crubit_features: flagset::FlagSet<crubit_feature::CrubitFeature>,
    ) -> Result<()> {
        use crubit_feature::CrubitFeature::*;
        match self {
            Self::References => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for references of non-function-param types requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::LifetimeReuse => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for multiple uses of a lifetime parameter requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::NonFreeReferenceParams => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for bound reference lifetimes (such as 'static) requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::PossibleMutableAliasing => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for functions taking a mutable reference, and which may alias in C++, requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::EscapeCppReservedKeyword => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for escaping C++ reserved keywords requires {}",
                    Experimental.aspect_hint()
                )
            }
            Self::RustChar => {
                ensure!(
                    crubit_features.contains(Experimental),
                    "support for the Rust `char` type requires {}",
                    Experimental.aspect_hint()
                )
            }
        }
        Ok(())
    }
}
