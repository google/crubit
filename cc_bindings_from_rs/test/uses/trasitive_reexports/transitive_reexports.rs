// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
extern crate direct;

pub fn direct_to_transitive(direct: &direct::Direct) -> direct::Transitive {
    direct::Transitive { value: direct.value }
}

pub fn direct_to_transitive_glob_a(
    direct: &direct::Direct,
) -> direct::transitive_public::TransitiveGlobA {
    direct::transitive_public::TransitiveGlobA { value: direct.value }
}

pub fn direct_to_transittive_type_alias(direct: &direct::Direct) -> direct::TransitiveTypeAlias {
    direct::TransitiveTypeAlias { value: direct.value }
}

pub fn direct_to_transitive_private_type_alias(
    direct: &direct::Direct,
) -> direct::PrivateTransitiveTypeAliasUseAlias {
    direct::PrivateTransitiveTypeAliasUseAlias { value: direct.value }
}

pub fn direct_to_transitive_use_alias(direct: &direct::Direct) -> direct::TransitiveUseAlias {
    direct::TransitiveUseAlias { value: direct.value }
}

pub type DirectReexportOfTransitive = direct::TransitiveReexportAndDirectReexport;
