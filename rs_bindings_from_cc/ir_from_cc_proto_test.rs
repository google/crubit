// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use googletest::prelude::*;
use ir_testing::ir_proto_from_cc;

fn get_ir(header: &str) -> Result<ir::IR> {
    ir_proto_from_cc(multiplatform_testing::test_platform(), header)
}

#[gtest]
fn test_func_proto() -> Result<()> {
    let ir = get_ir("int f(int a, int b);")?;
    let func =
        ir.functions().find(|f| f.rs_name == "f").expect("should find func f from the source code");
    assert_eq!(func.cc_name.as_identifier().unwrap().identifier.as_ref(), "f");
    assert_eq!(func.params.len(), 2);
    assert_eq!(
        func.params.first().expect("should have parameter 'a'").identifier.identifier.as_ref(),
        "a"
    );
    assert_eq!(
        func.params.get(1).expect("should have parameter 'b'").identifier.identifier.as_ref(),
        "b"
    );
    Ok(())
}

#[gtest]
fn test_record_proto() -> Result<()> {
    let ir = get_ir("struct MyStruct { int a; };")?;
    let record = ir
        .records()
        .find(|r| r.cc_name == "MyStruct")
        .expect("should find struct MyStruct from the source code");
    assert_eq!(record.cc_name.identifier.as_ref(), "MyStruct");
    assert_eq!(record.fields.len(), 1);
    assert_eq!(
        record
            .fields
            .first()
            .expect("should have field 'a'")
            .cpp_identifier
            .as_ref()
            .unwrap()
            .identifier
            .as_ref(),
        "a"
    );
    Ok(())
}

#[gtest]
fn test_function_with_asm_label_proto() -> Result<()> {
    let ir = get_ir("int f(int a, int b) asm(\"foo\");")?;
    let func =
        ir.functions().find(|f| f.rs_name == "f").expect("should find func f from the source code");
    assert_eq!(func.cc_name.as_identifier().unwrap().identifier.as_ref(), "f");
    assert_eq!(func.rs_name.as_identifier().unwrap().identifier.as_ref(), "f");

    match multiplatform_testing::test_platform() {
        multiplatform_testing::Platform::ArmMacOS | multiplatform_testing::Platform::X86MacOS => {
            assert_eq!(func.mangled_name.as_ref(), "\u{1}foo");
        }
        _ => {
            assert_eq!(func.mangled_name.as_ref(), "foo");
        }
    }
    Ok(())
}

#[gtest]
fn test_function_with_unnamed_parameters_proto() -> Result<()> {
    let ir = get_ir("int f(int, int);")?;
    let func =
        ir.functions().find(|f| f.rs_name == "f").expect("should find func f from the source code");
    assert_eq!(func.params.len(), 2);
    assert_eq!(
        func.params.first().expect("should have parameter 0").identifier.identifier.as_ref(),
        "__param_0"
    );
    assert_eq!(
        func.params.get(1).expect("should have parameter 1").identifier.identifier.as_ref(),
        "__param_1"
    );
    Ok(())
}

#[gtest]
fn test_unescapable_rust_keywords_in_function_parameters_proto() -> Result<()> {
    let ir = get_ir("int f(int self, int crate, int super);")?;
    let func =
        ir.functions().find(|f| f.rs_name == "f").expect("should find func f from the source code");
    assert_eq!(func.params.len(), 3);
    assert_eq!(
        func.params.first().expect("should have parameter 0").identifier.identifier.as_ref(),
        "__param_0"
    );
    assert_eq!(
        func.params.get(1).expect("should have parameter 1").identifier.identifier.as_ref(),
        "__param_1"
    );
    assert_eq!(
        func.params.get(2).expect("should have parameter 2").identifier.identifier.as_ref(),
        "__param_2"
    );
    Ok(())
}

#[gtest]
fn test_unescapable_rust_keywords_in_struct_name_proto() -> Result<()> {
    let ir = get_ir("struct Self{ int field; };")?;
    let unsupported = ir
        .unsupported_items()
        .find(|i| i.name.as_ref() == "Self")
        .expect("should find an unsupported item due to keyword collision from the source code");
    assert_eq!(unsupported.name.as_ref(), "Self");
    assert_eq!(unsupported.errors().len(), 1);
    assert!(unsupported
        .errors()
        .first()
        .expect("should have an error for unescapable identifier")
        .to_string()
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

    let some_struct = ir
        .records()
        .find(|r| r.cc_name == "SomeStruct")
        .expect("should find struct SomeStruct from the source code");

    assert_eq!(some_struct.fields.len(), 4);

    let f0 = some_struct.fields.first().expect("should have field 'default_access_int'");
    assert_eq!(f0.rust_identifier.as_ref().unwrap().identifier.as_ref(), "default_access_int");
    assert_eq!(f0.access, ir::AccessSpecifier::Public);

    let f1 = some_struct.fields.get(1).expect("should have field 'public_int'");
    assert_eq!(f1.rust_identifier.as_ref().unwrap().identifier.as_ref(), "public_int");
    assert_eq!(f1.access, ir::AccessSpecifier::Public);

    let f2 = some_struct.fields.get(2).expect("should have field 'protected_int'");
    assert_eq!(f2.rust_identifier.as_ref().unwrap().identifier.as_ref(), "protected_int");
    assert_eq!(f2.access, ir::AccessSpecifier::Protected);

    let f3 = some_struct.fields.get(3).expect("should have field 'private_int'");
    assert_eq!(f3.rust_identifier.as_ref().unwrap().identifier.as_ref(), "private_int");
    assert_eq!(f3.access, ir::AccessSpecifier::Private);

    let some_class = ir
        .records()
        .find(|r| r.cc_name == "SomeClass")
        .expect("should find class SomeClass from the source code");
    assert_eq!(some_class.fields.len(), 1);
    let cf0 = some_class.fields.first().expect("should have field 'default_access_int'");
    assert_eq!(cf0.rust_identifier.as_ref().unwrap().identifier.as_ref(), "default_access_int");
    assert_eq!(cf0.access, ir::AccessSpecifier::Private);
    Ok(())
}

#[gtest]
fn test_enum_proto() -> Result<()> {
    let ir = get_ir("enum MyEnum { kA = 42, kB = -1 };")?;
    let enum_decl = ir
        .enums()
        .find(|r| r.cc_name == "MyEnum")
        .expect("should find enum MyEnum from the source code");
    assert_eq!(enum_decl.cc_name.identifier.as_ref(), "MyEnum");
    assert_eq!(enum_decl.rs_name.identifier.as_ref(), "MyEnum");

    let k_a = enum_decl.enumerators.as_ref().unwrap().first().expect("should have enumerator 'kA'");
    assert_eq!(k_a.identifier.identifier.as_ref(), "kA");
    assert_eq!(k_a.value.wrapped_value, 42);
    assert!(!k_a.value.is_negative);

    let k_b = enum_decl.enumerators.as_ref().unwrap().get(1).expect("should have enumerator 'kB'");
    assert_eq!(k_b.identifier.identifier.as_ref(), "kB");
    // In proto, wrapped_value is int64, so -1 cast to int64 is -1.
    assert_eq!(k_b.value.wrapped_value as i64, -1);
    assert!(k_b.value.is_negative);
    Ok(())
}

#[gtest]
fn test_type_alias_proto() -> Result<()> {
    let ir = get_ir("typedef int MyInt;")?;
    let type_alias = ir
        .type_aliases()
        .find(|t| t.cc_name == "MyInt")
        .expect("should find type alias MyInt from the source code");
    assert_eq!(type_alias.cc_name.identifier.as_ref(), "MyInt");
    assert_eq!(type_alias.rs_name.identifier.as_ref(), "MyInt");

    Ok(())
}
