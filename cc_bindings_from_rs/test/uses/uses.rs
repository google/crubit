// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crubit_annotate::must_bind;

// Used before the real definition to make sure that the generarated C++ bindings
// are not affected by the order of the imports.
#[must_bind]
pub use test_mod::f;

pub mod test_mod {
    #[crubit_annotate::must_bind]
    pub fn f() -> i32 {
        42
    }
}

mod private_mod {
    #[crubit_annotate::must_bind]
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

#[must_bind]
pub use private_mod::private_fn;
#[must_bind]
pub use private_mod::ReexportedStruct as ExportedStruct;
#[must_bind]
pub use private_mod::ReexportedStruct as AliasOfExportedStruct;

mod gg {
    #[crubit_annotate::must_bind]
    pub use extern_crate::X;
}

pub use extern_crate::X as XFromExternCrate;
#[must_bind]
pub use gg::X;

#[must_bind]
pub fn return_x() -> X {
    X { field: 42 }
}

#[must_bind]
pub fn return_y() -> ::extern_crate::Y {
    ::extern_crate::Y { field: 42 }
}

#[must_bind]
pub struct Original {
    pub field: i32,
}

#[must_bind]
pub type Alias = Original;

#[must_bind]
pub use Alias as Alias2;

mod private_for_renaming {
    pub struct NonPublicName(pub i32);
    pub fn non_public_name() {}
}
#[must_bind]
pub use private_for_renaming::non_public_name as public_name;
#[must_bind]
pub use private_for_renaming::NonPublicName as PublicName;

#[must_bind]
pub use private_for_renaming::non_public_name as other_public_name;
#[must_bind]
pub use private_for_renaming::NonPublicName as OtherPublicName;

pub mod doc_hidden_test {
    mod private_mod {
        #[crubit_annotate::must_bind]
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
            #[crubit_annotate::must_bind]
            pub fn private_middle_path() -> i32 {
                742
            }
        }
    }

    pub use b::*;
}
