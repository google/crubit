// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ir::{
    self, CcType, Func, FuncParam, Identifier, Item, MappedType, Record, RsType, SpecialMemberFunc,
    IR,
};

/// Creates an identifier
pub fn ir_id(name: &str) -> Identifier {
    Identifier { identifier: name.to_string() }
}

/// Creates a simple type instance for `int`/`i32`
pub fn ir_int() -> MappedType {
    MappedType {
        rs_type: RsType { name: "i32".to_string(), type_params: vec![] },
        cc_type: CcType { name: "int".to_string(), type_params: vec![], is_const: false },
    }
}

/// Creates a simple `FuncParam` with a given name and `int`/`i32` type
pub fn ir_int_param(name: &str) -> FuncParam {
    FuncParam { identifier: ir_id(name), type_: ir_int() }
}

/// Creates a simple `Item::Func` with a given name
pub fn ir_func(name: &str) -> Item {
    Item::Func(Func {
        identifier: ir_id(name),
        doc_comment: None,
        is_inline: true,
        mangled_name: name.to_string(),
        return_type: ir_int(),
        params: vec![],
    })
}

pub fn ir_public_trivial_special() -> SpecialMemberFunc {
    SpecialMemberFunc {
        definition: ir::SpecialMemberDefinition::Trivial,
        access: ir::AccessSpecifier::Public,
    }
}

/// Creates a simple `Item::Record` with a given name.
pub fn ir_record(name: &str) -> Item {
    Item::Record(Record {
        identifier: ir_id(name),
        doc_comment: None,
        alignment: 0,
        size: 0,
        fields: vec![],
        copy_constructor: ir_public_trivial_special(),
        move_constructor: ir_public_trivial_special(),
        destructor: ir_public_trivial_special(),
        is_trivial_abi: true,
    })
}

// Creates a full `IR` data structure from a list of items
pub fn ir_items(items: Vec<Item>) -> IR {
    IR { used_headers: vec![], items }
}
