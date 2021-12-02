// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Types and deserialization logic for IR. See docs in
// `rs_bindings_from_cc/ir.h` for more information.
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;

/// Deserialize `IR` from JSON given as a reader.
pub fn deserialize_ir<R: Read>(reader: R) -> Result<IR> {
    let flat_ir = serde_json::from_reader(reader)?;
    make_ir(flat_ir)
}

/// Create a testing `IR` instance from given items, using mock values for other
/// fields.
pub fn make_ir_from_items(items: impl IntoIterator<Item = Item>) -> Result<IR> {
    make_ir_from_parts(
        items.into_iter().collect_vec(),
        /* used_headers= */ vec![],
        /* current_target= */ "//test:testing_target".into(),
    )
}

/// Create a testing `IR` instance from given parts. This function does not use
/// any mock values.
pub fn make_ir_from_parts(
    items: Vec<Item>,
    used_headers: Vec<HeaderName>,
    current_target: Label,
) -> Result<IR> {
    make_ir(FlatIR { used_headers, current_target, items })
}

fn make_ir(flat_ir: FlatIR) -> Result<IR> {
    let mut used_decl_ids = HashMap::new();
    for item in &flat_ir.items {
        if let Some(decl_id) = item.decl_id() {
            if let Some(existing_decl) = used_decl_ids.insert(decl_id, item) {
                bail!("Duplicate decl_id found in {:?} and {:?}", existing_decl, item);
            }
        }
    }
    let decl_id_to_item_idx = flat_ir
        .items
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| item.decl_id().map(|decl_id| (decl_id, idx)))
        .collect::<HashMap<_, _>>();
    Ok(IR { flat_ir, decl_id_to_item_idx })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct HeaderName {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct RsType {
    pub name: Option<String>,
    pub type_args: Vec<RsType>,
    pub decl_id: Option<DeclId>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct CcType {
    pub name: Option<String>,
    pub is_const: bool,
    pub type_args: Vec<CcType>,
    pub decl_id: Option<DeclId>,
}

pub trait OwningDeclId {
    fn owning_decl_id(&self) -> Option<DeclId>;
}

impl OwningDeclId for RsType {
    fn owning_decl_id(&self) -> Option<DeclId> {
        self.decl_id
    }
}

impl OwningDeclId for CcType {
    fn owning_decl_id(&self) -> Option<DeclId> {
        self.decl_id
    }
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
#[serde(transparent)]
pub struct DeclId(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(transparent)]
pub struct Label(pub String);

impl<T: Into<String>> From<T> for Label {
    fn from(label: T) -> Self {
        Self(label.into())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum UnqualifiedIdentifier {
    Identifier(Identifier),
    Constructor,
    Destructor,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum ReferenceQualification {
    LValue,
    RValue,
    Unqualified,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct InstanceMethodMetadata {
    pub reference: ReferenceQualification,
    pub is_const: bool,
    pub is_virtual: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct MemberFuncMetadata {
    pub for_type: Identifier,
    pub instance_method_metadata: Option<InstanceMethodMetadata>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct FuncParam {
    #[serde(rename(deserialize = "type"))]
    pub type_: MappedType,
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Func {
    pub name: UnqualifiedIdentifier,
    pub decl_id: DeclId,
    pub owning_target: Label,
    pub mangled_name: String,
    pub doc_comment: Option<String>,
    pub return_type: MappedType,
    pub params: Vec<FuncParam>,
    pub is_inline: bool,
    pub member_func_metadata: Option<MemberFuncMetadata>,
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
    NontrivialMembers,
    NontrivialSelf,
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
    pub decl_id: DeclId,
    pub owning_target: Label,
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
pub struct SourceLoc {
    pub filename: String,
    pub line: u64,
    pub column: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct UnsupportedItem {
    pub name: String,
    pub message: String,
    pub source_loc: SourceLoc,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Comment {
    pub text: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum Item {
    Func(Func),
    Record(Record),
    UnsupportedItem(UnsupportedItem),
    Comment(Comment),
}

impl Item {
    fn decl_id(&self) -> Option<DeclId> {
        match self {
            Item::Record(Record { decl_id, .. }) | Item::Func(Func { decl_id, .. }) => {
                Some(*decl_id)
            }
            _ => None,
        }
    }
}

impl From<Func> for Item {
    fn from(func: Func) -> Item {
        Item::Func(func)
    }
}

impl From<Record> for Item {
    fn from(record: Record) -> Item {
        Item::Record(record)
    }
}

impl From<UnsupportedItem> for Item {
    fn from(unsupported: UnsupportedItem) -> Item {
        Item::UnsupportedItem(unsupported)
    }
}

impl From<Comment> for Item {
    fn from(comment: Comment) -> Item {
        Item::Comment(comment)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(rename(deserialize = "IR"))]
struct FlatIR {
    #[serde(default)]
    used_headers: Vec<HeaderName>,
    current_target: Label,
    #[serde(default)]
    items: Vec<Item>,
}

/// Struct providing the necessary information about the API of a C++ target to
/// enable generation of Rust bindings source code (both `rs_api.rs` and
/// `rs_api_impl.cc` files).
#[derive(PartialEq, Debug)]
pub struct IR {
    flat_ir: FlatIR,
    // A map from a `decl_id` to an index of an `Item` in the `flat_ir.items` vec.
    decl_id_to_item_idx: HashMap<DeclId, usize>,
}

impl IR {
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.flat_ir.items.iter()
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut Item> {
        self.flat_ir.items.iter_mut()
    }

    pub fn take_items(self) -> Vec<Item> {
        self.flat_ir.items
    }

    pub fn used_headers(&self) -> impl Iterator<Item = &HeaderName> {
        self.flat_ir.used_headers.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &Func> {
        self.items().filter_map(|item| match item {
            Item::Func(func) => Some(func),
            _ => None,
        })
    }

    pub fn records(&self) -> impl Iterator<Item = &Record> {
        self.items().filter_map(|item| match item {
            Item::Record(func) => Some(func),
            _ => None,
        })
    }

    pub fn record_for_type<T>(&self, ty: &T) -> Result<&Record>
    where
        T: OwningDeclId + std::fmt::Debug,
    {
        if let Some(decl_id) = ty.owning_decl_id() {
            let idx = *self
                .decl_id_to_item_idx
                .get(&decl_id)
                .with_context(|| format!("Couldn't find decl_id {:?} in the IR.", decl_id))?;
            let item = self
                .flat_ir
                .items
                .get(idx)
                .with_context(|| format!("Couldn't find an item at idx {}", idx))?;
            match item {
                Item::Record(ref record) => Ok(record),
                _ => bail!("Unexpected item type {:?}", item),
            }
        } else {
            bail!("Type {:?} does not have an associated record.", ty)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_used_headers() {
        let input = r#"
        {
            "used_headers": [{ "name": "foo/bar.h" }],
            "current_target": "//foo:bar"
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = FlatIR {
            used_headers: vec![HeaderName { name: "foo/bar.h".to_string() }],
            current_target: "//foo:bar".into(),
            items: vec![],
        };
        assert_eq!(ir.flat_ir, expected);
    }

    #[test]
    fn test_member_access_specifiers() {
        let input = r#"
        {
            "current_target": "//foo:bar",
            "items": [
                { "Record" : {
                    "identifier": {"identifier": "SomeStruct" },
                    "decl_id": 42,
                    "owning_target": "//foo:bar",
                    "fields": [
                        {
                            "identifier": {"identifier": "public_int" },
                            "type": {
                                "rs_type": {"name": "i32", "type_args": []},
                                "cc_type": {"name": "int", "is_const": false, "type_args": []}
                            },
                            "access": "Public",
                            "offset": 0
                        },
                        {
                            "identifier": {"identifier": "protected_int" },
                            "type": {
                                "rs_type": {"name": "i32", "type_args": []},
                                "cc_type": {"name": "int", "is_const": false, "type_args": []}
                            },
                            "access": "Protected",
                            "offset": 32
                        },
                        {
                            "identifier": {"identifier": "private_int" },
                            "type": {
                                "rs_type": {"name": "i32", "type_args": []},
                                "cc_type": {"name": "int", "is_const": false, "type_args": []}
                            },
                            "access": "Private",
                            "offset": 64
                        }
                    ],
                    "size": 12,
                    "alignment": 4,
                    "copy_constructor": {
                        "definition": "NontrivialSelf",
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
        let expected = FlatIR {
            used_headers: vec![],
            current_target: "//foo:bar".into(),
            items: vec![Item::Record(Record {
                identifier: Identifier { identifier: "SomeStruct".to_string() },
                decl_id: DeclId(42),
                owning_target: "//foo:bar".into(),
                doc_comment: None,
                fields: vec![
                    Field {
                        identifier: Identifier { identifier: "public_int".to_string() },
                        doc_comment: None,
                        type_: MappedType {
                            rs_type: RsType {
                                name: "i32".to_string().into(),
                                type_args: vec![],
                                decl_id: None,
                            },
                            cc_type: CcType {
                                name: "int".to_string().into(),
                                is_const: false,
                                type_args: vec![],
                                decl_id: None,
                            },
                        },
                        access: AccessSpecifier::Public,
                        offset: 0,
                    },
                    Field {
                        identifier: Identifier { identifier: "protected_int".to_string() },
                        doc_comment: None,
                        type_: MappedType {
                            rs_type: RsType {
                                name: "i32".to_string().into(),
                                type_args: vec![],
                                decl_id: None,
                            },
                            cc_type: CcType {
                                name: "int".to_string().into(),
                                is_const: false,
                                type_args: vec![],
                                decl_id: None,
                            },
                        },
                        access: AccessSpecifier::Protected,
                        offset: 32,
                    },
                    Field {
                        identifier: Identifier { identifier: "private_int".to_string() },
                        doc_comment: None,
                        type_: MappedType {
                            rs_type: RsType {
                                name: "i32".to_string().into(),
                                type_args: vec![],
                                decl_id: None,
                            },
                            cc_type: CcType {
                                name: "int".to_string().into(),
                                is_const: false,
                                type_args: vec![],
                                decl_id: None,
                            },
                        },
                        access: AccessSpecifier::Private,
                        offset: 64,
                    },
                ],
                size: 12,
                alignment: 4,
                copy_constructor: SpecialMemberFunc {
                    definition: SpecialMemberDefinition::NontrivialSelf,
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
        };
        assert_eq!(ir.flat_ir, expected);
    }

    #[test]
    fn test_pointer_member_variable() {
        let input = r#"
        {
            "current_target": "//foo:bar",
            "items": [
                { "Record": {
                    "identifier": {"identifier": "SomeStruct" },
                    "decl_id": 42,
                    "owning_target": "//foo:bar",
                    "fields": [
                        {
                            "identifier": {"identifier": "ptr" },
                            "type": {
                                "rs_type": {"name": "*mut", "type_args": [
                                    {"name": "SomeStruct", "type_args": [], "decl_id": 42}
                                ]},
                                "cc_type": { "name": "*", "is_const": false, "type_args": [
                                    {
                                        "name": "SomeStruct",
                                        "is_const": false,
                                        "type_args": [],
                                        "decl_id": 42
                                    }
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
        let expected = FlatIR {
            used_headers: vec![],
            current_target: "//foo:bar".into(),
            items: vec![Item::Record(Record {
                identifier: Identifier { identifier: "SomeStruct".to_string() },
                decl_id: DeclId(42),
                owning_target: "//foo:bar".into(),
                doc_comment: None,
                fields: vec![Field {
                    identifier: Identifier { identifier: "ptr".to_string() },
                    doc_comment: None,
                    type_: MappedType {
                        rs_type: RsType {
                            name: "*mut".to_string().into(),
                            decl_id: None,
                            type_args: vec![RsType {
                                name: "SomeStruct".to_string().into(),
                                type_args: vec![],
                                decl_id: Some(DeclId(42)),
                            }],
                        },
                        cc_type: CcType {
                            name: "*".to_string().into(),
                            is_const: false,
                            decl_id: None,
                            type_args: vec![CcType {
                                name: "SomeStruct".to_string().into(),
                                is_const: false,
                                type_args: vec![],
                                decl_id: Some(DeclId(42)),
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
        };
        assert_eq!(ir.flat_ir, expected);
    }
}
