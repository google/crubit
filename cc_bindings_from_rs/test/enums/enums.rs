// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `enums_test.cc`.

/// Test for a `#[repr(C)] enum
pub mod repr_c {
    #[repr(C, i64)]
    pub enum MyEnum {
        E(String, i32),
        A(i32, i64),
        F,
        Z(()),
        G,
        B { h: bool, i: bool } = 10000,
        C { a: i32, b: i32, c: i32 },
        D,
    }

    impl Default for MyEnum {
        fn default() -> MyEnum {
            MyEnum::A(1, 2)
        }
    }
}

pub mod repr_c_drop {
    #[repr(C)]
    pub enum DropMe {
        A(i32),
        B(i64),
        Q,
        C { p: *mut i32 },
    }

    impl Default for DropMe {
        fn default() -> DropMe {
            DropMe::A(1)
        }
    }

    impl Drop for DropMe {
        fn drop(&mut self) {
            if let DropMe::C { p } = *self {
                unsafe { *p += 1 }
            }
        }
    }
}

pub mod repr_c_clone_counter {
    #[repr(C, i8)]
    pub enum CloneCount {
        A { p: *mut i32 },
    }

    impl Default for CloneCount {
        fn default() -> CloneCount {
            CloneCount::A { p: std::ptr::null_mut() }
        }
    }

    impl Clone for CloneCount {
        fn clone(&self) -> CloneCount {
            match *self {
                CloneCount::A { p } => {
                    unsafe { *p += 1 }
                    CloneCount::A { p }
                }
            }
        }
    }
}

pub mod repr_c_clone_active_variant {
    #[repr(C, i8)]
    pub enum CloneActiveVariant {
        A(i32),
        B(i32),
        C(i32),
    }

    impl Default for CloneActiveVariant {
        fn default() -> CloneActiveVariant {
            CloneActiveVariant::A(1)
        }
    }

    impl Clone for CloneActiveVariant {
        fn clone(&self) -> CloneActiveVariant {
            match *self {
                CloneActiveVariant::A(i) => CloneActiveVariant::B(i),
                CloneActiveVariant::B(i) => CloneActiveVariant::C(i),
                CloneActiveVariant::C(i) => CloneActiveVariant::A(i),
            }
        }
    }

    pub fn is_a(e: &CloneActiveVariant) -> bool {
        matches!(e, CloneActiveVariant::A(_))
    }

    pub fn is_b(e: &CloneActiveVariant) -> bool {
        matches!(e, CloneActiveVariant::B(_))
    }

    pub fn is_c(e: &CloneActiveVariant) -> bool {
        matches!(e, CloneActiveVariant::C(_))
    }
}

pub mod repr_rust {
    /// Doc comment of RustReprEnumWithNoPayload.
    pub enum RustReprEnumWithNoPayload {
        /// Doc comment of Variant1.
        Variant1,
        Variant2,
        Variant3,
    }

    impl RustReprEnumWithNoPayload {
        pub fn get_variant_number(&self) -> i32 {
            match self {
                Self::Variant1 => 1,
                Self::Variant2 => 2,
                Self::Variant3 => 3,
            }
        }
    }

    pub enum RustReprWithSingleNoPayloadVariant {
        SingleVariant,
    }

    pub enum RustReprWithSingleTuplePayloadVariant {
        SingleVariant(i32),
    }
}

pub mod repr_int {
    /// Two `NoPayloadX` variants to test that the tag is correctly set
    /// (`NoPayload1` should have a tag of 0 and therefore `NoPayload2` is a
    /// slightly better test for things like encoding the tag value with the
    /// proper endianness, especially given that the tag is 4 bytes wide).
    #[repr(u32)]
    pub enum IntReprEnumWithNoPayload {
        NoPayload1,
        NoPayload2 = 1234,
    }

    impl IntReprEnumWithNoPayload {
        pub fn is_no_payload1(&self) -> bool {
            matches!(self, Self::NoPayload1)
        }
        pub fn is_no_payload2(&self) -> bool {
            matches!(self, Self::NoPayload2)
        }
    }
}
