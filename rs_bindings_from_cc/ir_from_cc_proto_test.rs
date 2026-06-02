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
fn test_func_proto() -> Result<()> {
    let ir = get_ir("int f(int a, int b);")?;
    let item = ir
        .items()
        .into_iter()
        .find(|i| i.has_func())
        .expect("should find func f from the source code");
    let func = item.func();
    assert_eq!(func.cc_name().ident().identifier(), "f");
    assert_eq!(func.params().len(), 2);
    assert_eq!(
        func.params().get(0).expect("should have parameter 'a'").identifier().identifier(),
        "a"
    );
    assert_eq!(
        func.params().get(1).expect("should have parameter 'b'").identifier().identifier(),
        "b"
    );
    Ok(())
}

#[gtest]
fn test_record_proto() -> Result<()> {
    let ir = get_ir("struct MyStruct { int a; };")?;
    let item = ir
        .items()
        .into_iter()
        .find(|i| i.has_record() && i.record().cc_name().identifier() == "MyStruct")
        .expect("should find struct MyStruct from the source code");
    let record = item.record();
    assert_eq!(record.cc_name().identifier(), "MyStruct");
    assert_eq!(record.fields().len(), 1);
    assert_eq!(
        record.fields().get(0).expect("should have field 'a'").cpp_identifier().identifier(),
        "a"
    );
    Ok(())
}

#[gtest]
fn test_function_with_asm_label_proto() -> Result<()> {
    let ir = get_ir("int f(int a, int b) asm(\"foo\");")?;
    let item = ir
        .items()
        .into_iter()
        .find(|i| i.has_func())
        .expect("should find func f from the source code");
    let func = item.func();
    assert_eq!(func.cc_name().ident().identifier(), "f");
    assert_eq!(func.rs_name().ident().identifier(), "f");

    match multiplatform_testing::test_platform() {
        multiplatform_testing::Platform::ArmMacOS | multiplatform_testing::Platform::X86MacOS => {
            assert_eq!(func.mangled_name(), "\u{1}foo");
        }
        _ => {
            assert_eq!(func.mangled_name(), "foo");
        }
    }
    Ok(())
}

#[gtest]
fn test_function_with_unnamed_parameters_proto() -> Result<()> {
    let ir = get_ir("int f(int, int);")?;
    let item = ir
        .items()
        .into_iter()
        .find(|i| i.has_func())
        .expect("should find func f from the source code");
    let func = item.func();
    assert_eq!(func.params().len(), 2);
    assert_eq!(
        func.params().get(0).expect("should have parameter 0").identifier().identifier(),
        "__param_0"
    );
    assert_eq!(
        func.params().get(1).expect("should have parameter 1").identifier().identifier(),
        "__param_1"
    );
    Ok(())
}

#[gtest]
fn test_unescapable_rust_keywords_in_function_parameters_proto() -> Result<()> {
    let ir = get_ir("int f(int self, int crate, int super);")?;
    let item = ir
        .items()
        .into_iter()
        .find(|i| i.has_func())
        .expect("should find func f from the source code");
    let func = item.func();
    assert_eq!(func.params().len(), 3);
    assert_eq!(
        func.params().get(0).expect("should have parameter 0").identifier().identifier(),
        "__param_0"
    );
    assert_eq!(
        func.params().get(1).expect("should have parameter 1").identifier().identifier(),
        "__param_1"
    );
    assert_eq!(
        func.params().get(2).expect("should have parameter 2").identifier().identifier(),
        "__param_2"
    );
    Ok(())
}

#[gtest]
fn test_unescapable_rust_keywords_in_struct_name_proto() -> Result<()> {
    let ir = get_ir("struct Self{ int field; };")?;
    let item =
        ir.items().into_iter().find(|i| i.has_unsupported_item()).expect(
            "should find an unsupported item due to keyword collision from the source code",
        );
    let unsupported = item.unsupported_item();
    assert_eq!(unsupported.name(), "Self");
    assert_eq!(unsupported.errors().len(), 1);
    assert!(unsupported
        .errors()
        .get(0)
        .expect("should have an error for unescapable identifier")
        .message()
        .contains("Record name is not supported: Unescapable identifier: Self"));
    Ok(())
}

#[gtest]
fn test_record_member_variable_access_specifiers_proto() -> Result<()> {
    let ir = get_ir(
        "
        struct SomeStruct {
            int default_access_int;
          public:
            int public_int;
          protected:
            int protected_int;
          private:
            int private_int;
        };

        class SomeClass {
          int default_access_int;
        };
    ",
    )?;

    let items = ir.items();
    let some_struct_item = items
        .into_iter()
        .find(|i| i.has_record() && i.record().cc_name().identifier() == "SomeStruct")
        .expect("should find struct SomeStruct from the source code");
    let some_struct = some_struct_item.record();

    assert_eq!(some_struct.fields().len(), 4);

    let f0 = some_struct.fields().get(0).expect("should have field 'default_access_int'");
    assert_eq!(f0.rust_identifier().identifier(), "default_access_int");
    assert_eq!(f0.access(), ir_rust_proto::AccessSpecifier::Public);

    let f1 = some_struct.fields().get(1).expect("should have field 'public_int'");
    assert_eq!(f1.rust_identifier().identifier(), "public_int");
    assert_eq!(f1.access(), ir_rust_proto::AccessSpecifier::Public);

    let f2 = some_struct.fields().get(2).expect("should have field 'protected_int'");
    assert_eq!(f2.rust_identifier().identifier(), "protected_int");
    assert_eq!(f2.access(), ir_rust_proto::AccessSpecifier::Protected);

    let f3 = some_struct.fields().get(3).expect("should have field 'private_int'");
    assert_eq!(f3.rust_identifier().identifier(), "private_int");
    assert_eq!(f3.access(), ir_rust_proto::AccessSpecifier::Private);

    let some_class_item = items
        .into_iter()
        .find(|i| i.has_record() && i.record().cc_name().identifier() == "SomeClass")
        .expect("should find class SomeClass from the source code");
    let some_class = some_class_item.record();
    assert_eq!(some_class.fields().len(), 1);
    let cf0 = some_class.fields().get(0).expect("should have field 'default_access_int'");
    assert_eq!(cf0.rust_identifier().identifier(), "default_access_int");
    assert_eq!(cf0.access(), ir_rust_proto::AccessSpecifier::Private);
    Ok(())
}

#[gtest]
fn test_enum_proto() -> Result<()> {
    let ir = get_ir("enum MyEnum { kA = 42, kB = -1 };")?;
    let items = ir.items();
    let enum_item = items
        .into_iter()
        .find(|i| i.has_enum_decl() && i.enum_decl().cc_name().identifier() == "MyEnum")
        .expect("should find enum MyEnum from the source code");
    let enum_decl = enum_item.enum_decl();
    assert_eq!(enum_decl.cc_name().identifier(), "MyEnum");
    assert_eq!(enum_decl.rs_name().identifier(), "MyEnum");

    assert!(enum_decl.has_underlying_type());
    assert!(enum_decl.underlying_type().has_primitive());

    assert_eq!(enum_decl.enumerators().len(), 2);

    let k_a = enum_decl.enumerators().get(0).expect("should have enumerator 'kA'");
    assert_eq!(k_a.identifier().identifier(), "kA");
    assert_eq!(k_a.value().wrapped_value(), 42);
    assert!(!k_a.value().is_negative());

    let k_b = enum_decl.enumerators().get(1).expect("should have enumerator 'kB'");
    assert_eq!(k_b.identifier().identifier(), "kB");
    // In proto, wrapped_value is int64, so -1 cast to int64 is -1.
    assert_eq!(k_b.value().wrapped_value(), -1);
    assert!(k_b.value().is_negative());
    Ok(())
}

#[gtest]
fn test_type_alias_proto() -> Result<()> {
    let ir = get_ir("typedef int MyInt;")?;
    let items = ir.items();
    let alias_item = items
        .into_iter()
        .find(|i| i.has_type_alias() && i.type_alias().cc_name().identifier() == "MyInt")
        .expect("should find type alias MyInt from the source code");
    let type_alias = alias_item.type_alias();
    assert_eq!(type_alias.cc_name().identifier(), "MyInt");
    assert_eq!(type_alias.rs_name().identifier(), "MyInt");

    assert!(type_alias.has_underlying_type());
    assert!(type_alias.underlying_type().has_primitive());
    assert_eq!(type_alias.underlying_type().primitive().spelling(), "int");
    Ok(())
}
