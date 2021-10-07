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
pub struct RsType {
    pub name: String,
    pub type_params: Vec<RsType>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct CcType {
    pub name: String,
    pub is_const: bool,
    pub type_params: Vec<CcType>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct MappedType {
    pub rs_type: RsType,
    pub cc_type: CcType,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Identifier {
    pub identifier: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct FuncParam {
    #[serde(rename(deserialize = "type"))]
    pub type_: MappedType,
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Func {
    pub identifier: Identifier,
    pub mangled_name: String,
    pub return_type: MappedType,
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
    pub doc_comment: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub type_: MappedType,
    pub access: AccessSpecifier,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum SpecialMemberDefinition {
    Trivial,
    Nontrivial,
    Deleted,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct SpecialMemberFunc {
    pub definition: SpecialMemberDefinition,
    pub access: AccessSpecifier,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Record {
    pub identifier: Identifier,
    pub doc_comment: Option<String>,
    pub fields: Vec<Field>,
    pub size: usize,
    pub alignment: usize,
    pub copy_constructor: SpecialMemberFunc,
    pub move_constructor: SpecialMemberFunc,
    pub destructor: SpecialMemberFunc,
    pub is_trivial_abi: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum Item {
    Func(Func),
    Record(Record),
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct IR {
    #[serde(default)]
    pub used_headers: Vec<HeaderName>,
    #[serde(default)]
    pub items: Vec<Item>,
}

impl IR {
    pub fn functions(&self) -> impl Iterator<Item = &Func> {
        self.items.iter().filter_map(|item| match item {
            Item::Func(func) => Some(func),
            _ => None,
        })
    }

    pub fn records(&self) -> impl Iterator<Item = &Record> {
        self.items.iter().filter_map(|item| match item {
            Item::Record(func) => Some(func),
            _ => None,
        })
    }
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
    fn test_member_access_specifiers() {
        let input = r#"
        {
            "items": [
                { "Record" : {
                    "identifier": {"identifier": "SomeStruct" },
                    "fields": [
                        {
                            "identifier": {"identifier": "public_int" },
                            "type": {
                                "rs_type": {"name": "i32", "type_params": []},
                                "cc_type": {"name": "int", "is_const": false, "type_params": []}
                            },
                            "access": "Public",
                            "offset": 0
                        },
                        {
                            "identifier": {"identifier": "protected_int" },
                            "type": {
                                "rs_type": {"name": "i32", "type_params": []},
                                "cc_type": {"name": "int", "is_const": false, "type_params": []}
                            },
                            "access": "Protected",
                            "offset": 32
                        },
                        {
                            "identifier": {"identifier": "private_int" },
                            "type": {
                                "rs_type": {"name": "i32", "type_params": []},
                                "cc_type": {"name": "int", "is_const": false, "type_params": []}
                            },
                            "access": "Private",
                            "offset": 64
                        }
                    ],
                    "size": 12,
                    "alignment": 4,
                    "copy_constructor": {
                        "definition": "Nontrivial",
                        "access": "Private"
                    },
                    "move_constructor": {
                        "definition": "Deleted",
                        "access": "Protected"
                    },
                    "destructor": {
                        "definition": "Trivial",
                        "access": "Public"
                    },
                    "is_trivial_abi": true
                }}
            ]
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = IR {
            items: vec![Item::Record(Record {
                identifier: Identifier { identifier: "SomeStruct".to_string() },
                doc_comment: None,
                fields: vec![
                    Field {
                        identifier: Identifier { identifier: "public_int".to_string() },
                        doc_comment: None,
                        type_: MappedType {
                            rs_type: RsType { name: "i32".to_string(), type_params: vec![] },
                            cc_type: CcType {
                                name: "int".to_string(),
                                is_const: false,
                                type_params: vec![],
                            },
                        },
                        access: AccessSpecifier::Public,
                        offset: 0,
                    },
                    Field {
                        identifier: Identifier { identifier: "protected_int".to_string() },
                        doc_comment: None,
                        type_: MappedType {
                            rs_type: RsType { name: "i32".to_string(), type_params: vec![] },
                            cc_type: CcType {
                                name: "int".to_string(),
                                is_const: false,
                                type_params: vec![],
                            },
                        },
                        access: AccessSpecifier::Protected,
                        offset: 32,
                    },
                    Field {
                        identifier: Identifier { identifier: "private_int".to_string() },
                        doc_comment: None,
                        type_: MappedType {
                            rs_type: RsType { name: "i32".to_string(), type_params: vec![] },
                            cc_type: CcType {
                                name: "int".to_string(),
                                is_const: false,
                                type_params: vec![],
                            },
                        },
                        access: AccessSpecifier::Private,
                        offset: 64,
                    },
                ],
                size: 12,
                alignment: 4,
                copy_constructor: SpecialMemberFunc {
                    definition: SpecialMemberDefinition::Nontrivial,
                    access: AccessSpecifier::Private,
                },
                move_constructor: SpecialMemberFunc {
                    definition: SpecialMemberDefinition::Deleted,
                    access: AccessSpecifier::Protected,
                },
                destructor: SpecialMemberFunc {
                    definition: SpecialMemberDefinition::Trivial,
                    access: AccessSpecifier::Public,
                },
                is_trivial_abi: true,
            })],
            ..Default::default()
        };
        assert_eq!(ir, expected);
    }

    #[test]
    fn test_pointer_member_variable() {
        let input = r#"
        {
            "items": [
                { "Record": {
                    "identifier": {"identifier": "SomeStruct" },
                    "fields": [
                        {
                            "identifier": {"identifier": "ptr" },
                            "type": {
                                "rs_type": {"name": "*mut", "type_params": [
                                    {"name": "SomeStruct", "type_params": []}
                                ]},
                                "cc_type": { "name": "*", "is_const": false, "type_params": [
                                    {"name": "SomeStruct", "is_const": false, "type_params": []}
                                ]}
                            },
                            "access": "Public",
                            "offset": 0
                        }
                    ],
                    "size": 8,
                    "alignment": 8,
                    "copy_constructor": {
                        "definition": "Trivial",
                        "access": "Public"
                    },
                    "move_constructor": {
                        "definition": "Trivial",
                        "access": "Public"
                    },
                    "destructor": {
                        "definition": "Trivial",
                        "access": "Public"
                    },
                    "is_trivial_abi": true
                }}
            ]
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = IR {
            items: vec![Item::Record(Record {
                identifier: Identifier { identifier: "SomeStruct".to_string() },
                doc_comment: None,
                fields: vec![Field {
                    identifier: Identifier { identifier: "ptr".to_string() },
                    doc_comment: None,
                    type_: MappedType {
                        rs_type: RsType {
                            name: "*mut".to_string(),
                            type_params: vec![RsType {
                                name: "SomeStruct".to_string(),
                                type_params: vec![],
                            }],
                        },
                        cc_type: CcType {
                            name: "*".to_string(),
                            is_const: false,
                            type_params: vec![CcType {
                                name: "SomeStruct".to_string(),
                                is_const: false,
                                type_params: vec![],
                            }],
                        },
                    },
                    access: AccessSpecifier::Public,
                    offset: 0,
                }],
                size: 8,
                alignment: 8,
                move_constructor: SpecialMemberFunc {
                    definition: SpecialMemberDefinition::Trivial,
                    access: AccessSpecifier::Public,
                },
                copy_constructor: SpecialMemberFunc {
                    definition: SpecialMemberDefinition::Trivial,
                    access: AccessSpecifier::Public,
                },
                destructor: SpecialMemberFunc {
                    definition: SpecialMemberDefinition::Trivial,
                    access: AccessSpecifier::Public,
                },
                is_trivial_abi: true,
            })],
            ..Default::default()
        };
        assert_eq!(ir, expected);
    }
}
