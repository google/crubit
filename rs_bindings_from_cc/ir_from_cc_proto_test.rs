// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use googletest::prelude::*;
use ir_testing::ir_proto_from_cc;

fn get_ir(header: &str) -> Result<ir_rust_proto::IRProto> {
    ir_proto_from_cc(multiplatform_testing::test_platform(), header)
}

#[gtest]
fn test_func_proto() {
    let ir = get_ir("int f(int a, int b);").unwrap();
    let item = ir.items().into_iter().find(|i| i.has_func()).expect("No func found");
    let func = item.func();
    assert_eq!(func.cc_name().ident().identifier(), "f");
    assert_eq!(func.params().len(), 2);
    assert_eq!(func.params().get(0).unwrap().identifier().identifier(), "a");
    assert_eq!(func.params().get(1).unwrap().identifier().identifier(), "b");
}

#[gtest]
fn test_record_proto() {
    let ir = get_ir("struct MyStruct { int a; };").unwrap();
    let item = ir
        .items()
        .into_iter()
        .find(|i| i.has_record() && i.record().cc_name().identifier() == "MyStruct")
        .expect("No MyStruct found");
    let record = item.record();
    assert_eq!(record.cc_name().identifier(), "MyStruct");
    assert_eq!(record.fields().len(), 1);
    assert_eq!(record.fields().get(0).unwrap().cpp_identifier().identifier(), "a");
}
