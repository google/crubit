// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
pub struct Transitive {
    pub value: i32,
}

impl Transitive {
    pub fn new(value: i32) -> Self {
        Transitive { value }
    }
}

pub mod public {
    pub struct TransitiveGlobA {
        pub value: i32,
    }

    pub struct TransitiveGlobB {
        pub value: i32,
    }
}

pub struct TransitiveReexportAndDirectReexport {
    pub value: i32,
}

/// Re-exported by `direct`, and re-exported again by `transitive_reexports`, which can't name
/// `transitive` directly. Its return type needs a template specialization, which only
/// `transitive`'s bindings must define: a re-export must not define it again (under a different
/// spelling of `Transitive`).
pub fn maybe_transitive(value: i32) -> Option<Transitive> {
    (value > 0).then_some(Transitive { value })
}
