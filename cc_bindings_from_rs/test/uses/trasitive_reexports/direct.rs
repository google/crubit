// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
extern crate transitive;

pub use transitive::Transitive;
pub use transitive::Transitive as TransitiveUseAlias;

pub use transitive::public as transitive_public;
pub use transitive::public::*;

pub type TransitiveTypeAlias = transitive::Transitive;

mod private {
    use transitive::Transitive;
    pub type PrivateTransitiveTypeAlias = Transitive;
}

pub use private::PrivateTransitiveTypeAlias as PrivateTransitiveTypeAliasUseAlias;

pub struct Direct {
    pub value: i32,
}

impl Direct {
    pub fn new(train: Transitive) -> Self {
        Direct { value: train.value }
    }
}

pub use transitive::TransitiveReexportAndDirectReexport;
