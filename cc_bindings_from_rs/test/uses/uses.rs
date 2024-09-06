// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Put before the real definition to make sure that the generarated C++ bindings
// is not affected by the order of the imports.
pub use test_mod::f;

pub mod test_mod {
    pub fn f() -> i32 {
        42
    }
}

mod private_mod {
    pub struct ReexportedStruct {
        pub field: i32,
    }

    impl ReexportedStruct {
        pub fn create(field: i32) -> ReexportedStruct {
            ReexportedStruct { field }
        }
    }

    pub fn private_fn() -> i32 {
        42
    }
}

pub use private_mod::private_fn;
pub use private_mod::ReexportedStruct as ExportedStruct;
pub use private_mod::ReexportedStruct as AliasOfExportedStruct;
