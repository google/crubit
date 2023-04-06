// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `structs_test.cc`.

/// Test for a `#[repr(C)` struct.
pub mod repr_c {

    #[repr(C)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub fn create(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_x(p: Point) -> i32 {
        p.x
    }
}

/// Test for a struct using default layout (i.e. one without an explicit
/// `#[repr(C)]` or similar attribute).  Among other things, it tests that
/// building generated `..._cc_api_impl.rs` will not warn about
/// `improper_ctypes_definitions` (search for this warning name in `bindings.rs`
/// for a longer explanation of why suppressing this warning is okay).
pub mod default_repr {

    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    pub fn create(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_x(p: Point) -> i32 {
        p.x
    }
}

/// Test for a struct containing zero-sized fields.
pub mod zst_fields {

    pub struct Zst1;
    pub struct Zst2();
    pub struct Zst3 {}

    pub struct ZstFields {
        pub zst1: Zst1,
        pub zst2: Zst2,
        pub zst3: Zst3,
        pub value: i32,
    }

    pub fn create(value: i32) -> ZstFields {
        ZstFields { zst1: Zst1, zst2: Zst2(), zst3: Zst3 {}, value }
    }

    pub fn get_value(x: ZstFields) -> i32 {
        x.value
    }
}

/// Test of ABI classification.
///
/// System V ABI can classify function parameter and return types into broad
/// categories like "integer", "sse2", or "memory".  Classification impacts how
/// a given value is passed (e.g. by value in `eax` or `xmm0` register, or by
/// pointer).  ABI classification of C++ structs generated
/// by `cc_bindings_from_rs` needs to match exactly the classification of the
/// Rust structs in the input crate (e.g. from this test).  Mismatched ABI
/// classification will lead to Undefined Behavior.
///
/// This is a regression test for b/270454629 - replacing fields with an opaque
/// blob of bytes (e.g. using `[u8; N]` instead of the actual field type) may
/// change the ABI classification of a struct.  The fields of structs below are
/// private (i.e. non-`pub`) to encourage `cc_bindings_from_rs` to treat them as
/// an opaque blob of bytes.
///
/// Optimizing compiler can make the disassembly of the `create` methods quite
/// empty (probably because the input argument uses the same register as the
/// return value.  To make the tests more sensitive to ABI choices, the
/// `multiply` method is used (to actually operate on the input arguments and to
/// have to calculate a *new* return value).
pub mod abi_classification {
    /// Expected ABI classification: integer.  (For indirect confirmation, see
    /// the disassembly at https://godbolt.org/z/b7eeGcrGn).
    pub struct StructInteger(i32);

    /// Expected ABI classification: SSE.  (For indirect confirmation, see the
    /// disassembly at https://godbolt.org/z/b7eeGcrGn).
    pub struct StructFloat(
        f64,
        f32,
        // In Q1 2023 the bindings include explicit padding here - the presence of the padding
        // changes the ABI classification of the struct.
    );

    /// Expected ABI classification: memory.  (For indirect confirmation, see
    /// the disassembly at https://godbolt.org/z/b7eeGcrGn).
    #[repr(packed(1))]
    pub struct StructMemory {
        _padding: u8,
        i: i32,
    }

    impl StructInteger {
        pub fn create(i: i32) -> Self {
            Self(i)
        }
        pub fn multiply(x: Self, y: Self) -> Self {
            Self(x.0 * y.0)
        }
        pub fn inspect(s: Self) -> i32 {
            s.0
        }
    }

    impl StructFloat {
        pub fn create(f: f32) -> Self {
            Self(12.34, f)
        }
        pub fn multiply(x: Self, y: Self) -> Self {
            assert_eq!(12.34, x.0);
            assert_eq!(12.34, y.0);
            Self::create(x.1 * y.1)
        }
        pub fn inspect(s: Self) -> f32 {
            assert_eq!(12.34, s.0);
            s.1
        }
    }

    impl StructMemory {
        pub fn create(i: i32) -> Self {
            Self { _padding: 0, i }
        }
        pub fn multiply(x: Self, y: Self) -> Self {
            Self::create(x.i * y.i)
        }
        pub fn inspect(s: Self) -> i32 {
            s.i
        }
    }
}

/// Test that definition-less, thunk-less functions can pass structs by value.
///
/// Some Rust functions can just be redeclared on C++ side (i.e. without
/// requiring a separate thunk implemented in `..._cc_api_impl.rs`.  The
/// redeclaration needs to always replicate the Rust-side function signature.
/// This means that special-handling of passing structs-by-value (e.g. injecting
/// an `__ret_slot` output pointer/parameter) should be disabled for such
/// thunk-less functions.
///
/// The structure of this test mimics to some extent a subset of the
/// `abi_classification` test above.
pub mod struct_by_float_passing_with_no_cc_definition {
    #[repr(C)]
    pub struct StructFloat(
        f64,
        f32,
        // In Q1 2023 the bindings include explicit padding here - the presence of the padding
        // changes the ABI classification of the struct.
    );

    #[no_mangle]
    pub extern "C" fn no_mangle_create(f: f32) -> StructFloat {
        StructFloat(12.34, f)
    }

    #[no_mangle]
    pub extern "C" fn no_mangle_multiply(x: StructFloat, y: StructFloat) -> StructFloat {
        assert_eq!(12.34, x.0);
        assert_eq!(12.34, y.0);
        no_mangle_create(x.1 * y.1)
    }

    #[no_mangle]
    pub extern "C" fn no_mangle_inspect(s: StructFloat) -> f32 {
        assert_eq!(12.34, s.0);
        s.1
    }
}

/// Test that thunk-less functions (that still have a C++-side definition due to
/// naming difference) can pass structs by value.
///
/// Some Rust functions can just be redeclared on C++ side (i.e. without
/// requiring a separate thunk implemented in `..._cc_api_impl.rs`.  The
/// redeclaration needs to always replicate the Rust-side function signature.
/// This means that special-handling of passing structs-by-value (e.g. injecting
/// an `__ret_slot` output pointer/parameter) should be disabled for such
/// thunk-less functions.
///
/// The structure of this test mimics to some extent a subset of the
/// `abi_classification` test above.
pub mod struct_by_float_passing_with_no_thunk {
    #[repr(C)]
    pub struct StructFloat(
        f64,
        f32,
        // In Q1 2023 the bindings include explicit padding here - the presence of the padding
        // changes the ABI classification of the struct.
    );

    #[export_name = "struct_by_float_passing_with_no_thunk__thunkless_create"]
    pub extern "C" fn thunkless_create(f: f32) -> StructFloat {
        StructFloat(12.34, f)
    }

    #[export_name = "struct_by_float_passing_with_no_thunk__thunkless_multiply"]
    pub extern "C" fn thunkless_multiply(x: StructFloat, y: StructFloat) -> StructFloat {
        assert_eq!(12.34, x.0);
        assert_eq!(12.34, y.0);
        thunkless_create(x.1 * y.1)
    }

    #[export_name = "struct_by_float_passing_with_no_thunk__thunkless_inspect"]
    pub extern "C" fn thunkless_inspect(s: StructFloat) -> f32 {
        assert_eq!(12.34, s.0);
        s.1
    }
}
