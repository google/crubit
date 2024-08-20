// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

// shadow core and std to try to break the proc macro :)
mod std {}
mod core {}

mod type_exists {
    #[allow(unused_imports)]
    use super::*;

    #[gtest]
    fn type_doesnt_exist() {
        mod m {}
        assert!(!item_exists::type_exists!(m::DoesNotExist));
    }

    #[gtest]
    fn struct_does_exist() {
        mod m {
            pub struct S {}
        }
        assert!(item_exists::type_exists!(m::S));
    }

    #[gtest]
    fn unit_struct_does_exist() {
        mod m {
            pub struct S;
        }
        assert!(item_exists::type_exists!(m::S));
    }

    #[gtest]
    fn tuple_struct_does_exist() {
        mod m {
            pub struct S();
        }
        assert!(item_exists::type_exists!(m::S));
    }

    #[gtest]
    fn enum_does_exist() {
        mod m {
            pub enum E {}
        }
        assert!(item_exists::type_exists!(m::E));
    }

    #[gtest]
    fn union_does_exist() {
        mod m {
            pub union U {
                _x: i32,
            }
        }
        assert!(item_exists::type_exists!(m::U));
    }

    /// When we use the same name in a type context, we find no such type
    /// exists.
    #[gtest]
    fn function_doesnt_exist() {
        mod m {
            #[allow(unused)]
            pub fn foo() {}
        }
        assert!(!item_exists::type_exists!(m::foo));
    }

    #[gtest]
    fn constant_doesnt_exist() {
        mod m {
            #[allow(unused)]
            pub const X: () = ();
        }
        assert!(!item_exists::type_exists!(m::X));
    }

    #[gtest]
    fn static_doesnt_exist() {
        mod m {
            #[allow(unused)]
            pub static X: () = ();
        }
        assert!(!item_exists::type_exists!(m::X));
    }

    #[gtest]
    fn mod_doesnt_exist() {
        mod m {
            mod m2 {}
        }
        assert!(!item_exists::type_exists!(m::m2));
    }

    #[gtest]
    fn nested_module() {
        mod m {
            pub mod m2 {
                pub struct S;
            }
        }
        assert!(item_exists::type_exists!(m::m2::S));
        assert!(!item_exists::type_exists!(m::S));
        assert!(!item_exists::type_exists!(m::m2::DoesNotExist));
    }

    #[gtest]
    fn std_type() {
        assert!(item_exists::type_exists!(::std::num::NonZeroU8));
    }

    #[gtest]
    fn function_type() {
        assert!(item_exists::type_exists!(::std::num::NonZeroU8));
    }

    #[gtest]
    fn alias() {
        mod m {
            pub type X = ::std::num::NonZeroU8;
        }
        assert!(item_exists::type_exists!(m::X));
    }

    #[gtest]
    fn use_alias() {
        mod m {
            pub use ::std::num::NonZeroU8 as X;
        }
        assert!(item_exists::type_exists!(m::X));
    }

    /// Invoke the proc-macro twice in the same scope. This can expose some
    /// implementation errors.
    #[gtest]
    fn type_exists_twice() {
        mod m {
            pub struct A;
        }
        _ = item_exists::type_exists!(m::A);
        _ = item_exists::type_exists!(m::A);
    }
}

mod value_exists {
    #[allow(unused_imports)]
    use super::*;

    #[gtest]
    fn value_doesnt_exist() {
        mod m {}
        assert!(!item_exists::value_exists!(m::does_not_exist));
    }

    #[gtest]
    fn struct_doesnt_exist() {
        mod m {
            #[allow(dead_code)]
            pub struct S {}
        }
        assert!(!item_exists::value_exists!(m::S));
    }

    /// In type_exists!, we find the type. In value_exists!, we find the
    /// _constant_.
    #[gtest]
    fn unit_struct_does_exist() {
        mod m {
            pub struct S;
        }
        assert!(item_exists::value_exists!(m::S));
    }

    /// This one may be surprising, but it's ultimately similar to unit structs.
    /// In type_exists!, we find the type. In value_exists!, we find the
    /// _constructor_, a function.
    #[gtest]
    fn tuple_struct_does_exist() {
        mod m {
            pub struct S();
        }
        assert!(item_exists::value_exists!(m::S));
    }

    #[gtest]
    fn enum_doesnt_exist() {
        mod m {
            #[allow(dead_code)]
            pub enum E {}
        }
        assert!(!item_exists::value_exists!(m::E));
    }

    #[gtest]
    fn union_doesnt_exist() {
        mod m {
            #[allow(dead_code)]
            pub union U {
                _x: i32,
            }
        }
        assert!(!item_exists::value_exists!(m::U));
    }

    #[gtest]
    fn function_does_exist() {
        mod m {
            pub fn foo() {}
        }
        assert!(item_exists::value_exists!(m::foo));
    }

    #[gtest]
    fn constant_does_exist() {
        mod m {
            pub const X: () = ();
        }
        assert!(item_exists::value_exists!(m::X));
    }

    #[gtest]
    fn static_does_exist() {
        mod m {
            pub static X: () = ();
        }
        assert!(item_exists::value_exists!(m::X));
    }

    #[gtest]
    fn mod_doesnt_exist() {
        mod m {
            mod m2 {}
        }
        assert!(!item_exists::value_exists!(m::m2));
    }

    #[gtest]
    fn nested_module() {
        mod m {
            pub mod m2 {
                pub const X: () = ();
            }
        }
        assert!(item_exists::value_exists!(m::m2::X));
        assert!(!item_exists::value_exists!(m::X));
        assert!(!item_exists::value_exists!(m::m2::DoesNotExist));
    }

    #[gtest]
    fn std_value() {
        assert!(item_exists::value_exists!(::std::f32::consts::E));
    }

    #[gtest]
    fn alias_doesnt_exist() {
        mod m {
            #[allow(dead_code)]
            pub type X = ::std::num::NonZeroU8;
        }
        assert!(!item_exists::value_exists!(m::X));
    }

    #[gtest]
    fn use_alias_doesnt_exist() {
        mod m {
            #[allow(unused_imports)]
            pub use ::std::num::NonZeroU8 as X;
        }
        assert!(!item_exists::value_exists!(m::X));
    }

    #[gtest]
    fn use_const_does_exist() {
        mod m {
            pub use ::std::f32::consts::E as X;
        }
        assert!(item_exists::value_exists!(m::X));
    }
}
