// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Types and deserialization logic for IR. See docs in
//! `rs_bindings_from_cc/ir.h` for more
//! information.

use arc_anyhow::{anyhow, bail, Context, Error, Result};
use once_cell::unsync::OnceCell;
use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use serde::Deserialize;
use std::collections::hash_map::{Entry, HashMap};
use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::rc::Rc;

/// Deserialize `IR` from JSON given as a reader.
pub fn deserialize_ir<R: Read>(reader: R) -> Result<IR> {
    let flat_ir = serde_json::from_reader(reader)?;
    make_ir(flat_ir)
}

/// Create a testing `IR` instance from given parts. This function does not use
/// any mock values.
pub fn make_ir_from_parts(
    items: Vec<Item>,
    public_headers: Vec<HeaderName>,
    current_target: BazelLabel,
    top_level_item_ids: Vec<ItemId>,
    crate_root_path: Option<String>,
) -> Result<IR> {
    make_ir(FlatIR { public_headers, current_target, items, top_level_item_ids, crate_root_path })
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
            Item::Record(record) => &record.lifetime_params,
            Item::Func(func) => &func.lifetime_params,
            _ => continue,
        };
        for lifetime in lifetime_params {
            match lifetimes.entry(lifetime.id) {
                Entry::Occupied(occupied) => {
                    bail!(
                        "Duplicate use of lifetime ID {:?} in item {item:?} for names: '{}, '{}",
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
    let mut namespace_id_to_number_of_reopened_namespaces = HashMap::new();
    let mut reopened_namespace_id_to_idx = HashMap::new();

    flat_ir
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Namespace(ns) if ns.owning_target == flat_ir.current_target => {
                Some((ns.canonical_namespace_id, ns.id))
            }
            _ => None,
        })
        .for_each(|(canonical_id, id)| {
            let current_count =
                *namespace_id_to_number_of_reopened_namespaces.entry(canonical_id).or_insert(0);
            reopened_namespace_id_to_idx.insert(id, current_count);
            namespace_id_to_number_of_reopened_namespaces.insert(canonical_id, current_count + 1);
        });

    Ok(IR {
        flat_ir,
        item_id_to_item_idx,
        lifetimes,
        namespace_id_to_number_of_reopened_namespaces,
        reopened_namespace_id_to_idx,
    })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct HeaderName {
    pub name: Rc<str>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
#[serde(transparent)]
pub struct LifetimeId(pub i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct LifetimeName {
    pub name: Rc<str>,
    pub id: LifetimeId,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct RsType {
    pub name: Option<String>,
    pub lifetime_args: Rc<[LifetimeId]>,
    pub type_args: Rc<[RsType]>,
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
    pub identifier: Rc<str>,
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
    pub name: Rc<str>,
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
pub struct ItemId(usize);

impl ItemId {
    pub fn new_for_testing(value: usize) -> Self {
        Self(value)
    }
}

impl ToTokens for ItemId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        proc_macro2::Literal::usize_unsuffixed(self.0).to_tokens(tokens)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(transparent)]
pub struct BazelLabel(pub Rc<str>);

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
        Self(label.into().into())
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
            UnqualifiedIdentifier::Identifier(identifier) => Some(identifier.identifier.as_ref()),
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
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
    pub mangled_name: Rc<str>,
    pub doc_comment: Option<String>,
    pub return_type: MappedType,
    pub params: Vec<FuncParam>,
    /// For tests and internal use only.
    ///
    /// Prefer to reconstruct the lifetime params from the parameter types, as
    /// needed. This allows new parameters and lifetimes to be added that were
    /// not originally part of the IR.
    pub lifetime_params: Vec<LifetimeName>,
    pub is_inline: bool,
    pub member_func_metadata: Option<MemberFuncMetadata>,
    pub has_c_calling_convention: bool,
    pub is_member_or_descendant_of_class_template: bool,
    pub source_loc: SourceLoc,
    pub id: ItemId,
    pub enclosing_namespace_id: Option<ItemId>,
    pub adl_enclosing_record: Option<ItemId>,
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
    pub identifier: Option<Identifier>,
    pub doc_comment: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub type_: Result<MappedType, String>,
    pub access: AccessSpecifier,
    pub offset: usize,
    pub size: usize,
    pub is_no_unique_address: bool,
    pub is_bitfield: bool,
    // TODO(kinuko): Consider removing this, it is a duplicate of the same information
    // in `Record`.
    pub is_inheritable: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum SpecialMemberFunc {
    Trivial,
    NontrivialMembers,
    NontrivialUserDefined,
    Unavailable,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct BaseClass {
    pub base_record_id: ItemId,
    pub offset: Option<i64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct IncompleteRecord {
    pub cc_name: Rc<str>,
    pub rs_name: Rc<str>,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub record_type: RecordType,
    pub enclosing_namespace_id: Option<ItemId>,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize)]
pub enum RecordType {
    Struct,
    Union,
    Class,
}

impl ToTokens for RecordType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tag = match self {
            RecordType::Struct => quote! { struct },
            RecordType::Union => quote! { union },
            RecordType::Class => quote! { class },
        };
        tag.to_tokens(tokens)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Record {
    pub rs_name: Rc<str>,
    pub cc_name: Rc<str>,
    pub mangled_cc_name: Rc<str>,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub doc_comment: Option<String>,
    pub unambiguous_public_bases: Vec<BaseClass>,
    pub fields: Vec<Field>,
    pub lifetime_params: Vec<LifetimeName>,
    pub size: usize,
    pub original_cc_size: usize,
    pub alignment: usize,
    pub is_derived_class: bool,
    pub override_alignment: bool,
    pub copy_constructor: SpecialMemberFunc,
    pub move_constructor: SpecialMemberFunc,
    pub destructor: SpecialMemberFunc,
    pub is_trivial_abi: bool,
    pub is_inheritable: bool,
    pub is_abstract: bool,
    pub record_type: RecordType,
    pub is_aggregate: bool,
    pub is_anon_record_with_typedef: bool,
    pub child_item_ids: Vec<ItemId>,
    pub enclosing_namespace_id: Option<ItemId>,
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
    pub fn is_unpin(&self) -> bool {
        self.is_trivial_abi && !self.is_inheritable && self.fields.iter().all(|f| !f.is_inheritable)
    }

    pub fn is_union(&self) -> bool {
        match self.record_type {
            RecordType::Union => true,
            RecordType::Struct | RecordType::Class => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Enum {
    pub identifier: Identifier,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub underlying_type: MappedType,
    pub enumerators: Vec<Enumerator>,
    pub enclosing_namespace_id: Option<ItemId>,
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
    pub source_loc: SourceLoc,
    pub enclosing_record_id: Option<ItemId>,
    pub enclosing_namespace_id: Option<ItemId>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct SourceLoc {
    pub filename: Rc<str>,
    pub line: u64,
    pub column: u64,
}

/// A wrapper type that does not contribute to equality or hashing. All
/// instances are equal.
#[derive(Clone, Copy, Default)]
struct IgnoredField<T>(T);

impl<T> Debug for IgnoredField<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "_")
    }
}

impl<T> PartialEq for IgnoredField<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for IgnoredField<T> {}

impl<T> Hash for IgnoredField<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct UnsupportedItem {
    pub name: Rc<str>,
    message: Rc<str>,
    pub source_loc: SourceLoc,
    pub id: ItemId,
    #[serde(skip)]
    cause: IgnoredField<OnceCell<Error>>,
}

impl UnsupportedItem {
    pub fn new_with_message(name: &str, message: &str, source_loc: SourceLoc, id: ItemId) -> Self {
        Self {
            name: name.into(),
            message: message.into(),
            source_loc,
            id,
            cause: Default::default(),
        }
    }
    pub fn new_with_cause(name: String, cause: Error, source_loc: SourceLoc, id: ItemId) -> Self {
        Self {
            name: name.into(),
            message: cause.to_string().into(),
            source_loc,
            id,
            cause: IgnoredField(cause.into()),
        }
    }
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn cause(&self) -> &Error {
        self.cause.0.get_or_init(|| anyhow!(self.message.as_ref().to_owned()))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Comment {
    pub text: Rc<str>,
    pub id: ItemId,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct Namespace {
    pub name: Identifier,
    pub id: ItemId,
    pub canonical_namespace_id: ItemId,
    pub owning_target: BazelLabel,
    #[serde(default)]
    pub child_item_ids: Vec<ItemId>,
    pub enclosing_namespace_id: Option<ItemId>,
    pub is_inline: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct UseMod {
    pub path: Rc<str>,
    pub mod_name: Identifier,
    pub id: ItemId,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum Item {
    Func(Rc<Func>),
    IncompleteRecord(Rc<IncompleteRecord>),
    Record(Rc<Record>),
    Enum(Rc<Enum>),
    TypeAlias(Rc<TypeAlias>),
    UnsupportedItem(Rc<UnsupportedItem>),
    Comment(Rc<Comment>),
    Namespace(Rc<Namespace>),
    UseMod(Rc<UseMod>),
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
            Item::UseMod(use_mod) => use_mod.id,
        }
    }
    pub fn enclosing_namespace_id(&self) -> Option<ItemId> {
        match self {
            Item::Record(record) => record.enclosing_namespace_id,
            Item::IncompleteRecord(record) => record.enclosing_namespace_id,
            Item::Enum(enum_) => enum_.enclosing_namespace_id,
            Item::Func(func) => func.enclosing_namespace_id,
            Item::Namespace(namespace) => namespace.enclosing_namespace_id,
            Item::TypeAlias(type_alias) => type_alias.enclosing_namespace_id,
            Item::Comment(_) => None,
            Item::UnsupportedItem(_) => None,
            Item::UseMod(_) => None,
        }
    }
}

impl From<Func> for Item {
    fn from(func: Func) -> Item {
        Item::Func(Rc::new(func))
    }
}

impl<'a> TryFrom<&'a Item> for &'a Rc<Func> {
    type Error = Error;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        if let Item::Func(f) = value { Ok(f) } else { bail!("Not a Func: {:#?}", value) }
    }
}

impl From<Record> for Item {
    fn from(record: Record) -> Item {
        Item::Record(Rc::new(record))
    }
}

impl<'a> TryFrom<&'a Item> for &'a Rc<Record> {
    type Error = Error;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        if let Item::Record(r) = value { Ok(r) } else { bail!("Not a Record: {:#?}", value) }
    }
}

impl From<UnsupportedItem> for Item {
    fn from(unsupported: UnsupportedItem) -> Item {
        Item::UnsupportedItem(Rc::new(unsupported))
    }
}

impl<'a> TryFrom<&'a Item> for &'a Rc<UnsupportedItem> {
    type Error = Error;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        if let Item::UnsupportedItem(u) = value {
            Ok(u)
        } else {
            bail!("Not an UnsupportedItem: {:#?}", value)
        }
    }
}

impl From<Comment> for Item {
    fn from(comment: Comment) -> Item {
        Item::Comment(Rc::new(comment))
    }
}

impl<'a> TryFrom<&'a Item> for &'a Rc<Comment> {
    type Error = Error;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        if let Item::Comment(c) = value { Ok(c) } else { bail!("Not a Comment: {:#?}", value) }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(rename(deserialize = "IR"))]
struct FlatIR {
    #[serde(default)]
    public_headers: Vec<HeaderName>,
    current_target: BazelLabel,
    #[serde(default)]
    items: Vec<Item>,
    #[serde(default)]
    top_level_item_ids: Vec<ItemId>,
    #[serde(default)]
    crate_root_path: Option<String>,
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
    namespace_id_to_number_of_reopened_namespaces: HashMap<ItemId, usize>,
    reopened_namespace_id_to_idx: HashMap<ItemId, usize>,
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

    pub fn public_headers(&self) -> impl Iterator<Item = &HeaderName> {
        self.flat_ir.public_headers.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &Rc<Func>> {
        self.items().filter_map(|item| match item {
            Item::Func(func) => Some(func),
            _ => None,
        })
    }

    pub fn records(&self) -> impl Iterator<Item = &Rc<Record>> {
        self.items().filter_map(|item| match item {
            Item::Record(func) => Some(func),
            _ => None,
        })
    }

    pub fn unsupported_items(&self) -> impl Iterator<Item = &Rc<UnsupportedItem>> {
        self.items().filter_map(|item| match item {
            Item::UnsupportedItem(unsupported_item) => Some(unsupported_item),
            _ => None,
        })
    }

    pub fn comments(&self) -> impl Iterator<Item = &Rc<Comment>> {
        self.items().filter_map(|item| match item {
            Item::Comment(comment) => Some(comment),
            _ => None,
        })
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &Rc<Namespace>> {
        self.items().filter_map(|item| match item {
            Item::Namespace(ns) => Some(ns),
            _ => None,
        })
    }

    pub fn item_for_type<T>(&self, ty: &T) -> Result<&Item>
    where
        T: TypeWithDeclId + Debug,
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
        target.0.as_ref() == "//:virtual_clang_resource_dir_target"
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

    pub fn get_reopened_namespace_idx(&self, id: ItemId) -> Result<usize> {
        Ok(*self.reopened_namespace_id_to_idx.get(&id).with_context(|| {
            format!("Could not find the reopened namespace index for namespace {:?}.", id)
        })?)
    }

    pub fn is_last_reopened_namespace(&self, id: ItemId, canonical_id: ItemId) -> Result<bool> {
        let idx = self.get_reopened_namespace_idx(id)?;
        let last_item_idx = self
            .namespace_id_to_number_of_reopened_namespaces
            .get(&canonical_id)
            .with_context(|| {
            format!(
                "Could not find number of reopened namespaces for namespace {:?}.",
                canonical_id
            )
        })? - 1;
        Ok(idx == last_item_idx)
    }

    /// Returns the `Record` defining `func`, or `None` if `func` is not a
    /// member function.
    ///
    /// If `Func` is a member function, but its `Record` is somehow not in
    /// `self`, returns an error.
    pub fn record_for_member_func<'a>(&self, func: &'a Func) -> Result<Option<&Rc<Record>>> {
        if let Some(meta) = func.member_func_metadata.as_ref() {
            Ok(Some(self.find_decl(meta.record_id).with_context(|| {
                format!("Failed to retrieve Record for MemberFuncMetadata of {:?}", func)
            })?))
        } else {
            Ok(None)
        }
    }

    pub fn crate_root_path(&self) -> Option<&str> {
        self.flat_ir.crate_root_path.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_debug_print() {
        assert_eq!(format!("{:?}", Identifier { identifier: "hello".into() }), "\"hello\"");
    }

    #[test]
    fn test_unqualified_identifier_debug_print() {
        assert_eq!(
            format!(
                "{:?}",
                UnqualifiedIdentifier::Identifier(Identifier { identifier: "hello".into() })
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
            "public_headers": [{ "name": "foo/bar.h" }],
            "current_target": "//foo:bar"
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        let expected = FlatIR {
            public_headers: vec![HeaderName { name: "foo/bar.h".into() }],
            current_target: "//foo:bar".into(),
            top_level_item_ids: vec![],
            items: vec![],
            crate_root_path: None,
        };
        assert_eq!(ir.flat_ir, expected);
    }

    #[test]
    fn test_empty_crate_root_path() {
        let input = "{ \"current_target\": \"//foo:bar\" }";
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        assert_eq!(ir.crate_root_path(), None);
    }

    #[test]
    fn test_crate_root_path() {
        let input = r#"
        {
            "crate_root_path": "__cc_template_instantiations_rs_api",
            "current_target": "//foo:bar"
        }
        "#;
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        assert_eq!(ir.crate_root_path(), Some("__cc_template_instantiations_rs_api"));
    }
}
