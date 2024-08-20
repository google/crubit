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
