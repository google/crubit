// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![allow(private_interfaces)]
#![allow(dead_code)]
pub mod test_use_glob {
    pub fn f1() -> i32 {
        42
    }

    pub fn f2() -> i32 {
        43
    }

    fn f3() -> i32 {
        44
    }

    pub struct X1 {
        x: i32,
    }

    struct X2 {
        x: i32,
    }
}

pub use test_use_glob::*;

mod private_module {
    pub struct Bar {
        i: i32,
    }
    pub struct Foo {
        i: i32,
        pub bar: Bar,
    }

    impl Foo {
        pub fn create() -> Foo {
            Foo { i: 0, bar: Bar { i: 0 } }
        }

        pub fn bar() -> Bar {
            Bar { i: 0 }
        }
    }

    fn private_fn() -> i32 {
        42
    }

    pub fn g1() -> i32 {
        private_fn()
    }

    pub fn g2() -> i32 {
        private_fn()
    }
}

pub use private_module::*;

// Unsupported case: `use` mod.
/*
mod another_private_mod {
    pub mod another_private_mod_2 {
        pub struct X {
            pub field: i32,
        }
    }
}

pub use another_private_mod::another_private_mod_2;

pub fn f3() -> another_private_mod_2::X {
    another_private_mod_2::X { field: 42 }
}
*/

// Unsupported case: export ambiguous name in glob.
/*
pub mod mod1 {
    pub struct X {
        pub field: i32,
    }
}

pub mod mod2 {
    pub struct X {
        pub field: i32,
    }
}

pub use mod1::*;
pub use mod2::*;

*/
