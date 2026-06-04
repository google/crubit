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

    /// This enum is **not** a "ZST" (Zero-Sized Type), because of the C representation
    /// (even though it has only a single variant with no payload).
    #[repr(C)]
    pub enum ReprCWithSingleNoPayloadVariant {
        SingleVariant,
    }

    impl ReprCWithSingleNoPayloadVariant {
        pub fn is_single_variant(&self) -> bool {
            matches!(self, Self::SingleVariant)
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
    /// Doc comment of RustReprEnum.
    pub enum RustReprEnum {
        /// Doc comment of Variant1.
        Variant1,
        Variant2,
        Variant3,
        TuplePayloadVariant(i32, i32),
        StructPayloadVariant {
            x: i32,
            y: i32,
        },
    }

    impl RustReprEnum {
        pub fn get_variant_number(&self) -> i32 {
            match self {
                Self::Variant1 => 1,
                Self::Variant2 => 2,
                Self::Variant3 => 3,
                Self::TuplePayloadVariant(_, _) => 4,
                Self::StructPayloadVariant { .. } => 5,
            }
        }

        pub fn is_tuple_payload_variant(&self) -> bool {
            matches!(self, Self::TuplePayloadVariant(_, _))
        }

        pub fn get_first_item_from_tuple_payload(&self) -> i32 {
            match self {
                Self::TuplePayloadVariant(i, _) => *i,
                _ => panic!("Not a tuple payload"),
            }
        }
    }

    /// This enum is a "ZST" (Zero-Sized Type).
    /// Currently ZST types get no bindings (see b/258259459).
    pub enum RustReprWithSingleNoPayloadVariant {
        SingleVariant,
    }

    /// This enum is not a "ZST" (Zero-Sized Type), because of the payload.
    /// There is no tag / discriminant field, because there is only one variant.
    pub enum RustReprWithSingleTuplePayloadVariant {
        SingleVariant(i32),
    }

    impl RustReprWithSingleTuplePayloadVariant {
        pub fn get_single_item_from_tuple_payload(&self) -> i32 {
            match self {
                Self::SingleVariant(i) => *i,
            }
        }
    }

    pub enum RustReprWithNamingConflictBetweenCtorsAndMethods {
        NoPayloadVariant,
        TuplePayloadVariant(i32),
        StructPayloadVariant { x: i32 },
    }

    #[allow(non_snake_case)] // Need to replicate C++ names of variant constructors.
    impl RustReprWithNamingConflictBetweenCtorsAndMethods {
        /// Presence of this function tests the scenario where `MakeNoPayloadVariant` is a name of:
        /// 1. A static method (here/below).
        /// 2. An auto-generated factory/constructor static method
        pub fn MakeNoPayloadVariant() -> Self {
            Self::NoPayloadVariant
        }
        pub fn MakeTuplePayloadVariant(i: i32) -> Self {
            Self::TuplePayloadVariant(i)
        }
        pub fn MakeStructPayloadVariant(x: i32) -> Self {
            Self::StructPayloadVariant { x }
        }
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

    /// This enum is **not** a "ZST" (Zero-Sized Type), because of `#[repr(u32)]`
    /// (even though it has only a single variant with no payload).
    #[repr(u32)]
    pub enum IntReprWithSingleNoPayloadVariant {
        SingleVariant,
    }

    impl IntReprWithSingleNoPayloadVariant {
        pub fn is_single_variant(&self) -> bool {
            matches!(self, Self::SingleVariant)
        }
    }
}

/// This is a regression test for b/519192678.
///
/// This test mimics
/// [the `QrError` enum](https://docs.rs/qr_code/latest/qr_code/types/enum.QrError.html)
/// in an attempt to reproduce and prevent regressions where the `cc_bindings_from_rs`
/// bindings for `QrError::MakeDataTooLong()` created a wrong variant of the enum
/// (because it was not taking niche optimization into account):
///
/// ```
/// inline constexpr QrError QrError::MakeDataTooLong() {
///   return QrError(PrivateBytesTag{}, {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
///                                      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
/// }
/// ```
///
/// The object created above was interpreted on Rust side as
/// `Structured(AtLeast2Pieces)` instead of `DataTooLong`.  The correct bindings should
/// look like:
///
/// ```
/// inline constexpr QrError QrError::MakeDataTooLong() {
///   return QrError(PrivateBytesTag{}, {11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
///                                      0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
/// }
/// ```
pub mod qr_error {
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum QrError {
        DataTooLong,
        InvalidVersion,
        UnsupportedCharacterSet,
        InvalidEciDesignator,
        InvalidCharacter,
        Structured(StructuredQrError),
    }

    impl QrError {
        pub fn is_data_too_long(&self) -> bool {
            matches!(self, QrError::DataTooLong)
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum StructuredQrError {
        AtLeast2Pieces,
        TotalMismatch(usize),
        MissingParts,
        Parity,
        TooShort,
        StructuredWrongMode,
        StructuredWrongEnc,
        SeqGreaterThanTotal(u8, u8),
        LengthMismatch(usize, usize),
        UnsupportedVersion(i16),
        SplitMax16(usize),
    }
}
