// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Types and deserialization logic for IR. See docs in
// `rs_bindings_from_cc/ir.h` for more information.
use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use serde::Deserialize;
use std::collections::hash_map::{Entry, HashMap};
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
        /* top_level_item_ids= */ vec![],
    )
}

/// Create a testing `IR` instance from given parts. This function does not use
/// any mock values.
pub fn make_ir_from_parts(
    items: Vec<Item>,
    used_headers: Vec<HeaderName>,
    current_target: BazelLabel,
    top_level_item_ids: Vec<ItemId>,
) -> Result<IR> {
    make_ir(FlatIR { used_headers, current_target, items, top_level_item_ids })
}

fn make_ir(flat_ir: FlatIR) -> Result<IR> {
    let mut used_decl_ids = HashMap::new();
    for item in &flat_ir.items {
        if let Some(existing_decl) = used_decl_ids.insert(item.id(), item) {
            bail!("Duplicate decl_id found in {:?} and {:?}", existing_decl, item);
        }
    }
    let item_id_to_item_idx = flat_ir
        .items
        .iter()
        .enumerate()
        .map(|(idx, item)| (item.id(), idx))
        .collect::<HashMap<_, _>>();

    let mut lifetimes: HashMap<LifetimeId, LifetimeName> = HashMap::new();
    for item in &flat_ir.items {
        let lifetime_params = match item {
            Item::Record(Record { lifetime_params, .. }) => lifetime_params,
            Item::Func(Func { lifetime_params, .. }) => lifetime_params,
            _ => continue,
        };
        for lifetime in lifetime_params {
            match lifetimes.entry(lifetime.id) {
                Entry::Occupied(occupied) => {
                    bail!(
                        "Duplicate use of lifetime ID {:?} for names: '{}, '{}",
                        lifetime.id,
                        &occupied.get().name,
                        &lifetime.name
                    )
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(lifetime.clone());
                }
            }
        }
    }
    Ok(IR { flat_ir, item_id_to_item_idx, lifetimes })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct HeaderName {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
#[serde(transparent)]
pub struct LifetimeId(pub i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct LifetimeName {
    pub name: String,
    pub id: LifetimeId,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct RsType {
    pub name: Option<String>,
    pub lifetime_args: Vec<LifetimeId>,
    pub type_args: Vec<RsType>,
    pub decl_id: Option<ItemId>,
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
    pub decl_id: Option<ItemId>,
}

impl CcType {
    pub fn is_void(&self) -> bool {
        self.name.as_deref() == Some("void")
    }
}

pub trait TypeWithDeclId {
    fn decl_id(&self) -> Option<ItemId>;
}

impl TypeWithDeclId for RsType {
    fn decl_id(&self) -> Option<ItemId> {
        self.decl_id
    }
}

impl TypeWithDeclId for CcType {
    fn decl_id(&self) -> Option<ItemId> {
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize)]
pub struct IntegerConstant {
    pub is_negative: bool,
    pub wrapped_value: u64,
}

impl ToTokens for IntegerConstant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(if self.is_negative {
            Literal::i64_unsuffixed(self.wrapped_value as i64)
        } else {
            Literal::u64_unsuffixed(self.wrapped_value)
        })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Operator {
    pub name: String,
}

impl Operator {
    pub fn cc_name(&self) -> String {
        let separator = match self.name.chars().next() {
            Some(c) if c.is_alphabetic() => " ",
            _ => "",
        };
        format!("operator{separator}{name}", separator = separator, name = self.name)
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("\"{}\"", &self.cc_name()))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
#[serde(transparent)]
pub struct ItemId(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(transparent)]
pub struct BazelLabel(pub String);

impl BazelLabel {
    pub fn target_name(&self) -> &str {
        match self.0.split_once(':') {
            Some((_package, target_name)) => target_name,
            None => panic!("Unsupported label format {:?}", self.0),
        }
    }
}

impl<T: Into<String>> From<T> for BazelLabel {
    fn from(label: T) -> Self {
        Self(label.into())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum UnqualifiedIdentifier {
    Identifier(Identifier),
    Operator(Operator),
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
            UnqualifiedIdentifier::Operator(op) => fmt::Debug::fmt(op, f),
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

    /// If the member function was a constructor with an `explicit` specifier.
    pub is_explicit_ctor: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct MemberFuncMetadata {
    pub record_id: ItemId,
    pub instance_method_metadata: Option<InstanceMethodMetadata>,
}

impl MemberFuncMetadata {
    pub fn find_record<'a>(&self, ir: &'a IR) -> Result<&'a Record> {
        ir.find_decl(self.record_id).context("Failed to retrieve Record for MemberFuncMetadata")
    }
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
    pub owning_target: BazelLabel,
    pub mangled_name: String,
    pub doc_comment: Option<String>,
    pub return_type: MappedType,
    pub params: Vec<FuncParam>,
    pub lifetime_params: Vec<LifetimeName>,
    pub is_inline: bool,
    pub member_func_metadata: Option<MemberFuncMetadata>,
    pub has_c_calling_convention: bool,
    pub source_loc: SourceLoc,
    pub id: ItemId,
}

impl Func {
    pub fn is_instance_method(&self) -> bool {
        self.member_func_metadata
            .as_ref()
            .filter(|meta| meta.instance_method_metadata.is_some())
            .is_some()
    }
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
    pub is_no_unique_address: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum SpecialMemberDefinition {
    Trivial,
    NontrivialMembers,
    NontrivialUserDefined,
    Deleted,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct BaseClass {
    pub base_record_id: ItemId,
    pub offset: Option<i64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct SpecialMemberFunc {
    pub definition: SpecialMemberDefinition,
    pub access: AccessSpecifier,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct IncompleteRecord {
    pub cc_name: String,
    pub id: ItemId,
    pub owning_target: BazelLabel,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Record {
    pub rs_name: String,
    pub cc_name: String,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub doc_comment: Option<String>,
    pub unambiguous_public_bases: Vec<BaseClass>,
    pub fields: Vec<Field>,
    pub lifetime_params: Vec<LifetimeName>,
    pub size: usize,
    pub alignment: usize,
    pub base_size: Option<usize>,
    pub override_alignment: bool,
    pub copy_constructor: SpecialMemberFunc,
    pub move_constructor: SpecialMemberFunc,
    pub destructor: SpecialMemberFunc,
    pub is_trivial_abi: bool,
    pub is_inheritable: bool,
    pub is_union: bool,
    pub child_item_ids: Vec<ItemId>,
}

impl Record {
    /// Whether this type has Rust-like object semantics for mutating
    /// assignment, and can be passed by mut reference as a result.
    ///
    /// If a type `T` is mut reference safe, it can be possed as a `&mut T`
    /// safely. Otherwise, mutable references must use `Pin<&mut T>`.
    ///
    /// Conditions:
    ///
    /// 1. It is trivially relocatable, and thus can be passed by value and have
    ///    its memory directly mutated by Rust using memcpy-like
    ///    assignment/swap.
    ///
    /// 2. It cannot overlap with any other objects. In particular, it cannot be
    ///    inherited from, as inheritance allows for the tail padding to be
    ///    reused by other objects.
    ///
    ///    (In future versions, we could also include types which are POD for
    ///    the purpose of layout, but this is less predictable to C++ users,
    ///    and ABI-specific.)
    ///
    ///    We are assuming, for the moment, that no object is stored in a
    ///    `[[no_unique_address]]` variable. Much like packed structs and
    ///    the like, users of `[[no_unique_address]]` must be very careful
    ///    when passing mutable references to Rust.
    ///
    /// Described in more detail at: docs/unpin
    ///
    /// TODO(b/200067242): Actually force mut references to !is_unpin to be
    /// Pin<&mut T>.
    pub fn is_unpin(&self) -> bool {
        self.is_trivial_abi && !self.is_inheritable
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Enum {
    pub identifier: Identifier,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub underlying_type: MappedType,
    pub enumerators: Vec<Enumerator>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Enumerator {
    pub identifier: Identifier,
    pub value: IntegerConstant,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct TypeAlias {
    pub identifier: Identifier,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub doc_comment: Option<String>,
    pub underlying_type: MappedType,
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
    pub id: ItemId,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Comment {
    pub text: String,
    pub id: ItemId,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Namespace {
    pub name: Identifier,
    pub id: ItemId,
    #[serde(default)]
    pub child_item_ids: Vec<ItemId>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum Item {
    Func(Func),
    IncompleteRecord(IncompleteRecord),
    Record(Record),
    Enum(Enum),
    TypeAlias(TypeAlias),
    UnsupportedItem(UnsupportedItem),
    Comment(Comment),
    Namespace(Namespace),
}

impl Item {
    fn id(&self) -> ItemId {
        match self {
            Item::Func(func) => func.id,
            Item::IncompleteRecord(record) => record.id,
            Item::Record(record) => record.id,
            Item::Enum(enum_) => enum_.id,
            Item::TypeAlias(type_alias) => type_alias.id,
            Item::UnsupportedItem(unsupported) => unsupported.id,
            Item::Comment(comment) => comment.id,
            Item::Namespace(namespace) => namespace.id,
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
    current_target: BazelLabel,
    #[serde(default)]
    items: Vec<Item>,
    #[serde(default)]
    top_level_item_ids: Vec<ItemId>,
}

/// Struct providing the necessary information about the API of a C++ target to
/// enable generation of Rust bindings source code (both `rs_api.rs` and
/// `rs_api_impl.cc` files).
#[derive(PartialEq, Debug)]
pub struct IR {
    flat_ir: FlatIR,
    // A map from a `decl_id` to an index of an `Item` in the `flat_ir.items` vec.
    item_id_to_item_idx: HashMap<ItemId, usize>,
    lifetimes: HashMap<LifetimeId, LifetimeName>,
}

impl IR {
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.flat_ir.items.iter()
    }

    pub fn top_level_item_ids(&self) -> impl Iterator<Item = &ItemId> {
        self.flat_ir.top_level_item_ids.iter()
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

    pub fn comments(&self) -> impl Iterator<Item = &Comment> {
        self.items().filter_map(|item| match item {
            Item::Comment(comment) => Some(comment),
            _ => None,
        })
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &Namespace> {
        self.items().filter_map(|item| match item {
            Item::Namespace(ns) => Some(ns),
            _ => None,
        })
    }

    pub fn item_for_type<T>(&self, ty: &T) -> Result<&Item>
    where
        T: TypeWithDeclId + std::fmt::Debug,
    {
        if let Some(decl_id) = ty.decl_id() {
            self.find_untyped_decl(decl_id)
                .with_context(|| format!("Failed to retrieve item for type {:?}", ty))
        } else {
            bail!("Type {:?} does not have an associated item.", ty)
        }
    }

    pub fn find_decl<'a, T>(&'a self, decl_id: ItemId) -> Result<&'a T>
    where
        &'a T: TryFrom<&'a Item>,
    {
        self.find_untyped_decl(decl_id).and_then(|decl| {
            decl.try_into().map_err(|_| {
                anyhow!("DeclId {:?} doesn't refer to a {}", decl_id, std::any::type_name::<T>())
            })
        })
    }

    fn find_untyped_decl(&self, decl_id: ItemId) -> Result<&Item> {
        let idx = *self
            .item_id_to_item_idx
            .get(&decl_id)
            .with_context(|| format!("Couldn't find decl_id {:?} in the IR.", decl_id))?;
        self.flat_ir.items.get(idx).with_context(|| format!("Couldn't find an item at idx {}", idx))
    }

    // Returns whether `target` is the current target.
    pub fn is_current_target(&self, target: &BazelLabel) -> bool {
        // TODO(hlopko): Make this be a pointer comparison, now it's comparing string
        // values.
        *target == *self.current_target()
    }

    pub fn current_target(&self) -> &BazelLabel {
        &self.flat_ir.current_target
    }

    // Returns whether `target` is the target that corresponds to the C++
    // standard library.
    pub fn is_stdlib_target(&self, target: &BazelLabel) -> bool {
        // TODO(hlopko): Make this be a pointer comparison, now it's comparing string
        // values.
        // TODO(b/208377928): We don't yet have an actual target for the standard
        // library, so instead we're just testing against the "virtual target" that
        // AstVisitor::GetOwningTarget() returns if it can't find the header in the
        // header-to-target map.
        // Once we do have an actual target for the standard library, we may need to
        // query `self` to find out what it is, so we have a `self` parameter on this
        // method even though we currently don't use it.
        target.0 == "//:virtual_clang_resource_dir_target"
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

    pub fn get_lifetime(&self, lifetime_id: LifetimeId) -> Option<&LifetimeName> {
        self.lifetimes.get(&lifetime_id)
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
            top_level_item_ids: vec![],
            items: vec![],
        };
        assert_eq!(ir.flat_ir, expected);
    }
}
