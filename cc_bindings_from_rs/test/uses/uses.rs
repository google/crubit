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

mod gg {
    pub use extern_crate::X;
}

// TODO(b/350772554): `use extern_crate::*`.
pub use gg::X;
pub fn return_x() -> X {
    X { field: 42 }
}

#[crubit_annotate::must_bind]
pub fn return_y() -> ::extern_crate::Y {
    ::extern_crate::Y { field: 42 }
}

pub struct Original {
    pub field: i32,
}

pub type Alias = Original;

pub use Alias as Alias2;

pub mod doc_hidden_test {
    mod private_mod {
        pub fn private_fn() -> i32 {
            14568
        }
    }

    #[doc(hidden)]
    pub mod hidden {
        pub use super::private_mod::private_fn;
    }

    pub mod visible {
        pub use super::private_mod::private_fn;
    }
}

pub mod a {
    mod b {
        pub mod c {
            pub fn private_middle_path() -> i32 {
                742
            }
        }
    }

    pub use b::*;
}
