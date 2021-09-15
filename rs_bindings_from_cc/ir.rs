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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Field {
    pub identifier: Identifier,
    #[serde(rename(deserialize = "type"))]
    pub type_: IRType,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Record {
    pub identifier: Identifier,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct IR {
    pub used_headers: Vec<HeaderName>,
    pub functions: Vec<Func>,
    pub records: Vec<Record>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializing() {
        let input = r#"
        {
            "used_headers": [{ "name": "foo/bar.h" }],
            "functions": [
                {
                    "identifier": { "identifier": "hello_world" },
                    "mangled_name": "$$mangled_name$$",
                    "params": [
                        {
                            "identifier": { "identifier": "arg" },
                            "type": { "rs_name":"i32", "cc_name": "int" }
                        }
                    ],
                    "return_type": { "rs_name": "i32", "cc_name": "int" },
                    "is_inline": false
                }
            ],
            "records": [
                {
                    "identifier": {"identifier": "Cons" },
                    "fields": [
                        {
                            "identifier": {"identifier": "car" },
                            "type": {"rs_name": "i32", "cc_name": "int" }
                        },
                        {
                            "identifier": {"identifier": "cdr" },
                            "type": {"rs_name": "i32", "cc_name": "int" }
                        }
                    ]
                }
            ]
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = IR {
            used_headers: vec![HeaderName { name: "foo/bar.h".to_string() }],
            functions: vec![Func {
                identifier: Identifier { identifier: "hello_world".to_string() },
                mangled_name: "$$mangled_name$$".to_string(),
                return_type: IRType { rs_name: "i32".to_string(), cc_name: "int".to_string() },
                params: vec![FuncParam {
                    type_: IRType { rs_name: "i32".to_string(), cc_name: "int".to_string() },
                    identifier: Identifier { identifier: "arg".to_string() },
                }],
                is_inline: false,
            }],
            records: vec![Record {
                identifier: Identifier { identifier: "Cons".to_string() },
                fields: vec![
                    Field {
                        identifier: Identifier { identifier: "car".to_string() },
                        type_: IRType { rs_name: "i32".to_string(), cc_name: "int".to_string() },
                    },
                    Field {
                        identifier: Identifier { identifier: "cdr".to_string() },
                        type_: IRType { rs_name: "i32".to_string(), cc_name: "int".to_string() },
                    },
                ],
            }],
        };
        assert_eq!(ir, expected);
    }
}
