// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::Result;

use ffi_types::{FfiU8Slice, FfiU8SliceBox};
use ir::{
    self, CcType, Func, FuncParam, Identifier, Item, MappedType, Record, RsType, SpecialMemberFunc,
    IR,
};

pub fn ir_from_cc(src: &str) -> Result<IR> {
    extern "C" {
        fn json_from_cc(cc_source: FfiU8Slice) -> FfiU8SliceBox;
    }

    let src_u8 = src.as_bytes();
    let json_utf8 = unsafe { json_from_cc(FfiU8Slice::from_slice(src_u8)).into_boxed_slice() };
    ir::deserialize_ir(&*json_utf8)
}

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

/// Creates a simple `Func` with a given name
pub fn ir_func(name: &str) -> Func {
    let ir = ir_from_cc("inline int REPLACEME() {}").unwrap();
    for item in ir.items {
        if let Item::Func(mut func) = item {
            func.name = ir::UnqualifiedIdentifier::Identifier(ir_id(name));
            return func;
        }
    }
    panic!("Test IR doesn't contain a function");
}

pub fn ir_public_trivial_special() -> SpecialMemberFunc {
    SpecialMemberFunc {
        definition: ir::SpecialMemberDefinition::Trivial,
        access: ir::AccessSpecifier::Public,
    }
}

/// Creates a simple `Item::Record` with a given name.
pub fn ir_record(name: &str) -> Record {
    let ir = ir_from_cc("struct REPLACEME {};").unwrap();
    for item in ir.items {
        if let Item::Record(mut record) = item {
            record.identifier = ir_id(name);
            return record;
        }
    }
    panic!("Test IR doesn't contain a record");
}

// Creates a full `IR` data structure from a list of items
pub fn ir_items(items: Vec<Item>) -> IR {
    IR { used_headers: vec![], items }
}
