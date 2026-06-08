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
