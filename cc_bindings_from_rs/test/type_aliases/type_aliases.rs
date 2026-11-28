// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod test_type_aliases {
    pub type TypeAlias2 = TypeAlias;
    pub type TypeAlias = i32;

    pub fn func_using_alias() -> TypeAlias {
        0
    }
}

pub mod test_deprecated_type_alias {
    #[deprecated = "Use `OtherTypeAlias` instead"]
    pub type TypeAlias = i32;
}

pub mod test_generics_matching {
    pub type MatchingAlias<T, E> = Result<T, E>;
    pub type FlippedAlias<E, T> = Result<T, E>;
    pub type SpecializedAlias = Result<i32, i32>;

    pub fn returns_matching_alias() -> MatchingAlias<i32, i32> {
        Ok(0)
    }

    pub fn returns_flipped_alias() -> FlippedAlias<i8, u32> {
        Ok(0)
    }

    pub fn returns_specialized() -> SpecializedAlias {
        Ok(0)
    }
}

pub mod test_same_name_type_alias_and_function {
    pub type SameName = i32;

    #[allow(non_snake_case)]
    pub fn SameName(x: SameName) -> SameName {
        x
    }

    // Trailing underscore avoids forming a C++ reserved `__` identifier when renamed.
    #[allow(non_camel_case_types)]
    pub type Trailing_ = i64;

    #[allow(non_snake_case)]
    pub fn Trailing_(x: Trailing_) -> Trailing_ {
        x
    }

    // Secondary collision when `Secondary_` is already claimed by another function.
    pub type Secondary = i16;

    #[allow(non_snake_case)]
    pub fn Secondary() -> i16 {
        1
    }

    #[allow(non_snake_case)]
    pub fn Secondary_() -> i16 {
        2
    }

    // Sibling `Sibling_` keeps its own name when `Sibling` collides with `fn Sibling`.
    pub type Sibling = i32;
    pub type Sibling_ = u32;

    #[allow(non_snake_case)]
    pub fn Sibling() -> i32 {
        3
    }

    // Collisions with constants and child modules.
    #[allow(non_camel_case_types)]
    pub type SAME_CONST = i32;
    pub const SAME_CONST: SAME_CONST = 99;

    pub type WithSubmod = i32;

    #[allow(non_snake_case)]
    pub fn WithSubmod() -> i32 {
        7
    }

    #[allow(non_snake_case)]
    pub mod WithSubmod_ {
        pub fn inside() -> i32 {
            8
        }
    }

    // ADT type alias and re-exported type aliases colliding with functions.
    #[derive(Clone, Copy)]
    pub struct Wrapped {
        pub value: i32,
    }

    pub type SameAdt = Wrapped;

    #[allow(non_snake_case)]
    pub fn SameAdt(value: i32) -> SameAdt {
        Wrapped { value }
    }

    mod inner {
        pub type Reexported = i32;
    }
    pub use inner::Reexported;

    #[allow(non_snake_case)]
    pub fn Reexported(x: Reexported) -> Reexported {
        x + 1
    }

    pub mod other {
        pub type Shared = u16;
    }
    pub use other::Shared;

    #[allow(non_snake_case)]
    pub fn Shared() -> Shared {
        12
    }
}
