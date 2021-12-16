// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Types and deserialization logic for IR. See docs in
// `rs_bindings_from_cc/ir.h` for more information.
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::io::Read;

pub const TESTING_TARGET: &str = "//test:testing_target";

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
        /* current_target= */ TESTING_TARGET.into(),
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
        if let Some(Record { id, .. }) = item.as_record() {
            if let Some(existing_decl) = used_decl_ids.insert(id, item) {
                bail!("Duplicate decl_id found in {:?} and {:?}", existing_decl, item);
            }
        }
    }
    let decl_id_to_item_idx = flat_ir
        .items
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| item.as_record().map(|record| (record.id, idx)))
        .collect::<HashMap<_, _>>();
    Ok(IR { flat_ir, decl_id_to_item_idx })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct HeaderName {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
#[serde(transparent)]
pub struct LifetimeId(pub i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Lifetime {
    pub name: String,
    pub id: LifetimeId,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct RsType {
    pub name: Option<String>,
    pub lifetime_args: Vec<LifetimeId>,
    pub type_args: Vec<RsType>,
    pub decl_id: Option<DeclId>,
}

impl RsType {
    pub fn is_unit_type(&self) -> bool {
        self.name.as_deref() == Some("()")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct CcType {
    pub name: Option<String>,
    pub is_const: bool,
    pub type_args: Vec<CcType>,
    pub decl_id: Option<DeclId>,
}

pub trait TypeWithDeclId {
    fn decl_id(&self) -> Option<DeclId>;
}

impl TypeWithDeclId for RsType {
    fn decl_id(&self) -> Option<DeclId> {
        self.decl_id
    }
}

impl TypeWithDeclId for CcType {
    fn decl_id(&self) -> Option<DeclId> {
        self.decl_id
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct MappedType {
    pub rs_type: RsType,
    pub cc_type: CcType,
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Identifier {
    pub identifier: String,
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("\"{}\"", &self.identifier))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
#[serde(transparent)]
pub struct DeclId(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(transparent)]
pub struct Label(pub String);

impl Label {
    pub fn target_name(&self) -> Result<&str> {
        match self.0.split_once(":") {
            Some((_package, target_name)) => Ok(target_name),
            None => bail!("Unsupported label format {:?}", self.0),
        }
    }
}

impl<T: Into<String>> From<T> for Label {
    fn from(label: T) -> Self {
        Self(label.into())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum UnqualifiedIdentifier {
    Identifier(Identifier),
    Constructor,
    Destructor,
}

impl UnqualifiedIdentifier {
    pub fn identifier_as_str(&self) -> Option<&str> {
        match self {
            UnqualifiedIdentifier::Identifier(identifier) => Some(identifier.identifier.as_str()),
            _ => None,
        }
    }
}

impl fmt::Debug for UnqualifiedIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnqualifiedIdentifier::Identifier(identifier) => fmt::Debug::fmt(identifier, f),
            UnqualifiedIdentifier::Constructor => f.write_str("Constructor"),
            UnqualifiedIdentifier::Destructor => f.write_str("Destructor"),
        }
    }
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
    pub record_id: DeclId,
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
    pub owning_target: Label,
    pub mangled_name: String,
    pub doc_comment: Option<String>,
    pub return_type: MappedType,
    pub params: Vec<FuncParam>,
    pub lifetime_params: Vec<Lifetime>,
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
    NontrivialUserDefined,
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
    pub id: DeclId,
    pub owning_target: Label,
    pub doc_comment: Option<String>,
    pub fields: Vec<Field>,
    pub lifetime_params: Vec<Lifetime>,
    pub size: usize,
    pub alignment: usize,
    pub copy_constructor: SpecialMemberFunc,
    pub move_constructor: SpecialMemberFunc,
    pub destructor: SpecialMemberFunc,
    pub is_trivial_abi: bool,
}

impl Record {
    pub fn owning_crate_name(&self) -> Result<&str> {
        self.owning_target.target_name()
    }
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
    fn as_record(&self) -> Option<&Record> {
        match self {
            Item::Record(record) => Some(record),
            _ => None,
        }
    }
}

impl From<Func> for Item {
    fn from(func: Func) -> Item {
        Item::Func(func)
    }
}

impl<'a> TryFrom<&'a Item> for &'a Func {
    type Error = anyhow::Error;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        if let Item::Func(f) = value { Ok(f) } else { anyhow::bail!("Not a Func: {:#?}", value) }
    }
}

impl From<Record> for Item {
    fn from(record: Record) -> Item {
        Item::Record(record)
    }
}

impl<'a> TryFrom<&'a Item> for &'a Record {
    type Error = anyhow::Error;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        if let Item::Record(r) = value {
            Ok(r)
        } else {
            anyhow::bail!("Not a Record: {:#?}", value)
        }
    }
}

impl From<UnsupportedItem> for Item {
    fn from(unsupported: UnsupportedItem) -> Item {
        Item::UnsupportedItem(unsupported)
    }
}

impl<'a> TryFrom<&'a Item> for &'a UnsupportedItem {
    type Error = anyhow::Error;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        if let Item::UnsupportedItem(u) = value {
            Ok(u)
        } else {
            anyhow::bail!("Not an UnsupportedItem: {:#?}", value)
        }
    }
}

impl From<Comment> for Item {
    fn from(comment: Comment) -> Item {
        Item::Comment(comment)
    }
}

impl<'a> TryFrom<&'a Item> for &'a Comment {
    type Error = anyhow::Error;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        if let Item::Comment(c) = value {
            Ok(c)
        } else {
            anyhow::bail!("Not a Comment: {:#?}", value)
        }
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

    pub fn unsupported_items(&self) -> impl Iterator<Item = &UnsupportedItem> {
        self.items().filter_map(|item| match item {
            Item::UnsupportedItem(unsupported_item) => Some(unsupported_item),
            _ => None,
        })
    }

    pub fn record_for_type<T>(&self, ty: &T) -> Result<&Record>
    where
        T: TypeWithDeclId + std::fmt::Debug,
    {
        if let Some(decl_id) = ty.decl_id() {
            self.find_decl(decl_id)?.try_into()
        } else {
            bail!("Type {:?} does not have an associated record.", ty)
        }
    }

    pub fn find_decl(&self, decl_id: DeclId) -> Result<&Item> {
        let idx = *self
            .decl_id_to_item_idx
            .get(&decl_id)
            .with_context(|| format!("Couldn't find decl_id {:?} in the IR.", decl_id))?;
        self.flat_ir.items.get(idx).with_context(|| format!("Couldn't find an item at idx {}", idx))
    }

    pub fn is_in_current_target(&self, record: &Record) -> bool {
        // TODO(hlopko): Make this be a pointer comparison, now it's comparing string
        // values.
        record.owning_target == self.flat_ir.current_target
    }

    // Returns the standard Debug print string for the `flat_ir`. The reason why we
    // don't use the debug print of `Self` is that `Self` contains HashMaps, and
    // their debug print produces content that is not valid Rust code.
    // `token_stream_matchers` (hacky) implementation parses the debug print and
    // chokes on HashMaps. Therefore this method.
    //
    // Used for `token_stream_matchers`, do not use for anything else.
    pub fn flat_ir_debug_print(&self) -> String {
        format!("{:?}", self.flat_ir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_debug_print() {
        assert_eq!(format!("{:?}", Identifier { identifier: "hello".to_string() }), "\"hello\"");
    }

    #[test]
    fn test_unqualified_identifier_debug_print() {
        assert_eq!(
            format!(
                "{:?}",
                UnqualifiedIdentifier::Identifier(Identifier { identifier: "hello".to_string() })
            ),
            "\"hello\""
        );
        assert_eq!(format!("{:?}", UnqualifiedIdentifier::Constructor), "Constructor");
        assert_eq!(format!("{:?}", UnqualifiedIdentifier::Destructor), "Destructor");
    }

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
}
