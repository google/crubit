// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;

use ffi_types::{FfiU8Slice, FfiU8SliceBox};
use ir::{self, make_ir_from_parts, Func, Identifier, Item, Record, IR};
use itertools::Itertools;

/// Generates `IR` from a header containing `header_source`.
pub fn ir_from_cc(header_source: &str) -> Result<IR> {
    ir_from_cc_dependency(header_source, "// empty header")
}

/// Prepends definitions for lifetime annotation macros to the code.
pub fn with_lifetime_macros(source: &str) -> String {
    let mut result = String::from(
        r#"
    #define $(l) [[clang::annotate_type("lifetime", #l)]]
    "#,
    );
    for l in 'a'..='z' {
        result.push_str(&format!("#define ${} $({})\n", l, l));
    }
    result.push_str("#define $static $(static)\n");
    result.push_str(source);
    result
}

/// Name of the current target used by `ir_from_cc` and `ir_from_cc_dependency`.
pub const TESTING_TARGET: &str = "//test:testing_target";

/// Create a testing `IR` instance from given items, using mock values for other
/// fields.
pub fn make_ir_from_items(items: impl IntoIterator<Item = Item>) -> Result<IR> {
    let target: ir::BazelLabel = TESTING_TARGET.into();
    make_ir_from_parts(
        items.into_iter().collect_vec(),
        /* public_headers= */ vec![],
        /* current_target= */ target.clone(),
        /* top_level_item_ids= */ vec![],
        /* crate_root_path= */ None,
        /* crubit_features= */ [(target, ir::CrubitFeature::Experimental)].into()
    )
}

/// Target of the dependency used by `ir_from_cc_dependency`.
/// Needs to be kept in sync with `kDependencyTarget` in `json_from_cc.cc`.
pub const DEPENDENCY_TARGET: &str = "//test:dependency";

/// Generates `IR` from a header that depends on another header.
///
/// `header_source` of the header will be updated to contain the `#include` line
/// for the header with `dependency_header_source`. The name of the dependency
/// target is exposed as `DEPENDENCY_TARGET`.
pub fn ir_from_cc_dependency(header_source: &str, dependency_header_source: &str) -> Result<IR> {
    const DEPENDENCY_HEADER_NAME: &str = "test/dependency_header.h";

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
    Identifier { identifier: name.into() }
}

/// Creates a simple `Item::Record` with a given name.
pub fn ir_record(name: &str) -> Record {
    let ir = ir_from_cc("struct REPLACEME final {};").unwrap();
    for item in ir.items() {
        if let Item::Record(record) = item {
            let mut record = (**record).clone();
            record.rs_name = name.into();
            record.cc_name = name.into();
            return record;
        }
    }
    panic!("Test IR doesn't contain a record");
}

/// Retrieves the function with the given name.
/// Panics if no such function could be found.
pub fn retrieve_func<'a>(ir: &'a IR, name: &str) -> &'a Func {
    for func in ir.functions() {
        if func.name == ir::UnqualifiedIdentifier::Identifier(ir_id(name)) {
            return func;
        }
    }
    panic!("Didn't find function with name {}", name);
}

/// Retrieves the `Record` with the given name.
/// Panics if no such record could be found.
pub fn retrieve_record<'a>(ir: &'a IR, cc_name: &str) -> &'a Record {
    for record in ir.records() {
        if record.cc_name.as_ref() == cc_name {
            return record;
        }
    }
    panic!("Didn't find record with cc_name {}", cc_name);
}
