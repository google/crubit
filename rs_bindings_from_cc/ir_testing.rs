// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ir::{CcType, Func, Identifier, Item, MappedType, Record, RsType};

/// Creates an identifier
pub fn id(name: &str) -> Identifier {
    Identifier { identifier: name.to_string() }
}

/// Creates a simple type instance for `int`/`i32`
pub fn int() -> MappedType {
    MappedType {
        rs_type: RsType { name: "i32".to_string(), type_params: vec![] },
        cc_type: CcType { name: "int".to_string(), type_params: vec![], is_const: false },
    }
}

/// Creates a simple `Item::Func` with a given name
pub fn func(name: &str) -> Item {
    Item::Func(Func {
        identifier: id(name),
        is_inline: true,
        mangled_name: name.to_string(),
        return_type: int(),
        params: vec![],
    })
}

/// Creates a simple `Item::Record` with a given name
pub fn record(name: &str) -> Item {
    Item::Record(Record {
        identifier: id(name),
        alignment: 0,
        size: 0,
        fields: vec![],
        is_trivial_abi: true,
    })
}
