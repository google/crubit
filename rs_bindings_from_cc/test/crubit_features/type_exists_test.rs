// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// shadow core and std to try to break the proc macro :)
mod std {}
mod core {}

#[test]
fn type_doesnt_exist() {
    mod m {}
    assert!(!type_exists::type_exists!(m::DoesNotExist));
}

#[test]
fn struct_does_exist() {
    mod m {
        pub struct S {}
    }
    assert!(type_exists::type_exists!(m::S));
}

#[test]
fn unit_struct_does_exist() {
    mod m {
        pub struct S;
    }
    assert!(type_exists::type_exists!(m::S));
}

#[test]
fn tuple_struct_does_exist() {
    mod m {
        pub struct S();
    }
    assert!(type_exists::type_exists!(m::S));
}

#[test]
fn enum_does_exist() {
    mod m {
        pub enum E {}
    }
    assert!(type_exists::type_exists!(m::E));
}

#[test]
fn union_does_exist() {
    mod m {
        pub union U {
            _x: i32,
        }
    }
    assert!(type_exists::type_exists!(m::U));
}

/// When we use the same name in a type context, we find no such type exists.
#[test]
fn function_doesnt_exist() {
    mod m {
        #[allow(unused)]
        pub fn foo() {}
    }
    assert!(!type_exists::type_exists!(m::foo));
}

#[test]
fn constant_doesnt_exist() {
    mod m {
        #[allow(unused)]
        const X: () = ();
    }
    assert!(!type_exists::type_exists!(m::X));
}

#[test]
fn static_doesnt_exist() {
    mod m {
        #[allow(unused)]
        static X: () = ();
    }
    assert!(!type_exists::type_exists!(m::X));
}

#[test]
fn mod_doesnt_exist() {
    mod m {
        mod m2 {}
    }
    assert!(!type_exists::type_exists!(m::m2));
}

#[test]
fn nested_module() {
    mod m {
        pub mod m2 {
            pub struct S;
        }
    }
    assert!(type_exists::type_exists!(m::m2::S));
    assert!(!type_exists::type_exists!(m::S));
    assert!(!type_exists::type_exists!(m::m2::DoesNotExist));
}

#[test]
fn std_type() {
    assert!(type_exists::type_exists!(::std::num::NonZeroU8));
}

#[test]
fn function_type() {
    assert!(type_exists::type_exists!(::std::num::NonZeroU8));
}

#[test]
fn alias() {
    mod m {
        pub type X = ::std::num::NonZeroU8;
    }
    assert!(type_exists::type_exists!(m::X));
}

#[test]
fn use_alias() {
    mod m {
        pub use ::std::num::NonZeroU8 as X;
    }
    assert!(type_exists::type_exists!(m::X));
}

/// Invoke the proc-macro twice in the same scope. This can expose some
/// implementation errors.
#[test]
fn type_exists_twice() {
    mod m {
        pub struct A;
    }
    type_exists::type_exists!(m::A);
    type_exists::type_exists!(m::A);
}
