// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Types and deserialization logic for IR. See docs in
// `rs_bindings_from_cc/ir.h` for more information.
use anyhow::Result;
use serde::Deserialize;
use std::io::Read;

pub fn deserialize_ir<R: Read>(reader: R) -> Result<IR> {
    Ok(serde_json::from_reader(reader)?)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct HeaderName {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct IRType {
    pub rs_name: String,
    pub cc_name: String,
    pub cc_const: bool,
    pub type_params: Vec<IRType>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Identifier {
    pub identifier: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct FuncParam {
    #[serde(rename(deserialize = "type"))]
    pub type_: IRType,
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Func {
    pub identifier: Identifier,
    pub mangled_name: String,
    pub return_type: IRType,
    pub params: Vec<FuncParam>,
    pub is_inline: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize)]
pub enum AccessSpecifier {
    Public,
    Protected,
    Private,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Field {
    pub identifier: Identifier,
    #[serde(rename(deserialize = "type"))]
    pub type_: IRType,
    pub access: AccessSpecifier,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Record {
    pub identifier: Identifier,
    pub fields: Vec<Field>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct IR {
    #[serde(default)]
    pub used_headers: Vec<HeaderName>,
    #[serde(default)]
    pub functions: Vec<Func>,
    #[serde(default)]
    pub records: Vec<Record>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_used_headers() {
        let input = r#"
        {
            "used_headers": [{ "name": "foo/bar.h" }]
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = IR {
            used_headers: vec![HeaderName { name: "foo/bar.h".to_string() }],
            ..Default::default()
        };
        assert_eq!(ir, expected);
    }

    #[test]
    fn test_functions() {
        let input = r#"
        {
            "functions": [
                {
                    "identifier": { "identifier": "hello_world" },
                    "mangled_name": "$$mangled_name$$",
                    "params": [
                        {
                            "identifier": { "identifier": "arg" },
                            "type": { "rs_name":"i32", "cc_name": "int", "cc_const": false, "type_params": [] }
                        }
                    ],
                    "return_type": { "rs_name": "i32", "cc_name": "int", "cc_const": false, "type_params": [] },
                    "is_inline": false
                }
            ]
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = IR {
            functions: vec![Func {
                identifier: Identifier { identifier: "hello_world".to_string() },
                mangled_name: "$$mangled_name$$".to_string(),
                return_type: IRType {
                    rs_name: "i32".to_string(),
                    cc_name: "int".to_string(),
                    cc_const: false,
                    type_params: vec![],
                },
                params: vec![FuncParam {
                    type_: IRType {
                        rs_name: "i32".to_string(),
                        cc_name: "int".to_string(),
                        cc_const: false,
                        type_params: vec![],
                    },
                    identifier: Identifier { identifier: "arg".to_string() },
                }],
                is_inline: false,
            }],
            ..Default::default()
        };
        assert_eq!(ir, expected);
    }

    #[test]
    fn test_member_access_specifiers() {
        let input = r#"
        {
            "records": [
                {
                    "identifier": {"identifier": "SomeStruct" },
                    "fields": [
                        {
                            "identifier": {"identifier": "public_int" },
                            "type": {"rs_name": "i32", "cc_name": "int", "cc_const": false, "type_params": [] },
                            "access": "Public"
                        },
                        {
                            "identifier": {"identifier": "protected_int" },
                            "type": {"rs_name": "i32", "cc_name": "int", "cc_const": false, "type_params": [] },
                            "access": "Protected"
                        },
                        {
                            "identifier": {"identifier": "private_int" },
                            "type": {"rs_name": "i32", "cc_name": "int", "cc_const": false, "type_params": [] },
                            "access": "Private"
                        }
                    ]
                }
            ]
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = IR {
            records: vec![Record {
                identifier: Identifier { identifier: "SomeStruct".to_string() },
                fields: vec![
                    Field {
                        identifier: Identifier { identifier: "public_int".to_string() },
                        type_: IRType {
                            rs_name: "i32".to_string(),
                            cc_name: "int".to_string(),
                            cc_const: false,
                            type_params: vec![],
                        },
                        access: AccessSpecifier::Public,
                    },
                    Field {
                        identifier: Identifier { identifier: "protected_int".to_string() },
                        type_: IRType {
                            rs_name: "i32".to_string(),
                            cc_name: "int".to_string(),
                            cc_const: false,
                            type_params: vec![],
                        },
                        access: AccessSpecifier::Protected,
                    },
                    Field {
                        identifier: Identifier { identifier: "private_int".to_string() },
                        type_: IRType {
                            rs_name: "i32".to_string(),
                            cc_name: "int".to_string(),
                            cc_const: false,
                            type_params: vec![],
                        },
                        access: AccessSpecifier::Private,
                    },
                ],
            }],
            ..Default::default()
        };
        assert_eq!(ir, expected);
    }

    #[test]
    fn test_pointer_member_variable() {
        let input = r#"
        {
            "records": [
                {
                    "identifier": {"identifier": "SomeStruct" },
                    "fields": [
                        {
                            "identifier": {"identifier": "ptr" },
                            "type": {"rs_name": "*mut", "cc_name": "*", "cc_const": false, "type_params": [
                                {"rs_name": "SomeStruct", "cc_name": "SomeStruct", "cc_const": false, "type_params": []}
                            ] },
                            "access": "Public"
                        }
                    ]
                }
            ]
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = IR {
            records: vec![Record {
                identifier: Identifier { identifier: "SomeStruct".to_string() },
                fields: vec![Field {
                    identifier: Identifier { identifier: "ptr".to_string() },
                    type_: IRType {
                        rs_name: "*mut".to_string(),
                        cc_name: "*".to_string(),
                        cc_const: false,
                        type_params: vec![IRType {
                            rs_name: "SomeStruct".to_string(),
                            cc_name: "SomeStruct".to_string(),
                            cc_const: false,
                            type_params: vec![],
                        }],
                    },
                    access: AccessSpecifier::Public,
                }],
            }],
            ..Default::default()
        };
        assert_eq!(ir, expected);
    }
}
