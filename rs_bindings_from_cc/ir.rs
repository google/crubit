// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::Result;
use serde::Deserialize;
use std::io::Read;

pub fn deserialize_ir<R: Read>(reader: R) -> Result<IR> {
    Ok(serde_json::from_reader(reader)?)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct IRType {
    pub rs_name: String,
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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct IR {
    pub functions: Vec<Func>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializing() {
        let input = r#"
        {
            "functions": [
                {
                    "identifier": { "identifier": "hello_world" },
                    "mangled_name": "$$mangled_name$$",
                    "params": [
                        {
                            "identifier": { "identifier": "arg" },
                            "type": { "rs_name":"i32" }
                        }
                    ],
                    "return_type": { "rs_name": "i32" }
                }
            ]
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = IR {
            functions: vec![Func {
                identifier: Identifier { identifier: "hello_world".to_string() },
                mangled_name: "$$mangled_name$$".to_string(),
                return_type: IRType { rs_name: "i32".to_string() },
                params: vec![FuncParam {
                    type_: IRType { rs_name: "i32".to_string() },
                    identifier: Identifier { identifier: "arg".to_string() },
                }],
            }],
        };
        assert_eq!(ir, expected);
    }
}
