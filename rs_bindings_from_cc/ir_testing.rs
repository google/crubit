// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::Result;

use ffi_types::{FfiU8Slice, FfiU8SliceBox};
use ir::{self, CcType, Func, FuncParam, Identifier, Item, ItemId, MappedType, Record, RsType, IR};

/// Generates `IR` from a header containing `header_source`.
pub fn ir_from_cc(header_source: &str) -> Result<IR> {
    ir_from_cc_dependency(header_source, "// empty header")
}

const DEPENDENCY_HEADER_NAME: &str = "test/dependency_header.h";

/// Generates `IR` from a header that depends on another header.
///
/// `header_source` of the header will be updated to contain the `#include` line
/// for the header with `dependency_header_source`. The name of the dependency
/// target is assumed to be `"//test:dependency"`.
pub fn ir_from_cc_dependency(header_source: &str, dependency_header_source: &str) -> Result<IR> {
    extern "C" {
        fn json_from_cc_dependency(
            header_source: FfiU8Slice,
            dependency_header_source: FfiU8Slice,
        ) -> FfiU8SliceBox;
    }

    let header_source_with_include =
        format!("#include \"{}\"\n\n{}", DEPENDENCY_HEADER_NAME, header_source);
    let header_source_with_include_u8 = header_source_with_include.as_bytes();
    let dependency_header_source_u8 = dependency_header_source.as_bytes();
    let json_utf8 = unsafe {
        json_from_cc_dependency(
            FfiU8Slice::from_slice(header_source_with_include_u8),
            FfiU8Slice::from_slice(dependency_header_source_u8),
        )
        .into_boxed_slice()
    };
    ir::deserialize_ir(&*json_utf8)
}

/// Creates an identifier
pub fn ir_id(name: &str) -> Identifier {
    Identifier { identifier: name.to_string() }
}

/// Creates a simple type instance for `int`/`i32`
pub fn ir_int() -> MappedType {
    MappedType {
        rs_type: RsType {
            name: "i32".to_string().into(),
            lifetime_args: vec![],
            type_args: vec![],
            decl_id: None,
        },
        cc_type: CcType {
            name: "int".to_string().into(),
            type_args: vec![],
            is_const: false,
            decl_id: None,
        },
    }
}

pub fn ir_type(decl_id: usize) -> MappedType {
    MappedType {
        rs_type: RsType {
            name: None,
            lifetime_args: vec![],
            type_args: vec![],
            decl_id: Some(ItemId(decl_id)),
        },
        cc_type: CcType {
            name: None,
            type_args: vec![],
            is_const: false,
            decl_id: Some(ItemId(decl_id)),
        },
    }
}

/// Creates a simple `FuncParam` with a given name and `int`/`i32` type
pub fn ir_int_param(name: &str) -> FuncParam {
    FuncParam { identifier: ir_id(name), type_: ir_int() }
}

pub fn ir_param(name: &str, decl_id: usize) -> FuncParam {
    FuncParam { identifier: ir_id(name), type_: ir_type(decl_id) }
}

/// Creates a simple `Func` with a given name
pub fn ir_func(name: &str) -> Func {
    let ir =
        ir_from_cc(&str::replace("inline int REPLACEME() {return 0;}", "REPLACEME", name)).unwrap();
    for item in ir.take_items() {
        if let Item::Func(func) = item {
            return func;
        }
    }
    panic!("Test IR doesn't contain a function");
}

/// Creates a simple `Item::Record` with a given name.
pub fn ir_record(name: &str) -> Record {
    let ir = ir_from_cc("struct REPLACEME final {};").unwrap();
    for item in ir.take_items() {
        if let Item::Record(mut record) = item {
            record.rs_name = name.to_string();
            record.cc_name = name.to_string();
            return record;
        }
    }
    panic!("Test IR doesn't contain a record");
}

/// Retrieves the function with the given name.
/// Panics if no such function could be found.
pub fn retrieve_func<'a>(ir: &'a IR, name: &str) -> &'a Func {
    for item in ir.items() {
        if let Item::Func(func) = item {
            if func.name == ir::UnqualifiedIdentifier::Identifier(ir_id(name)) {
                return func;
            }
        }
    }
    panic!("Didn't find function with name {}", name);
}
