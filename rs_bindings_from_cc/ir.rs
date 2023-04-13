// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Types and deserialization logic for IR. See docs in
//! `rs_bindings_from_cc/ir.h` for more
//! information.

use arc_anyhow::{anyhow, bail, Context, Error, Result};
use once_cell::unsync::OnceCell;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;
use std::collections::hash_map::{Entry, HashMap};
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::rc::Rc;

/// Common data about all items.
pub trait GenericItem {
    fn id(&self) -> ItemId;
    /// The name of the item, readable by programmers.
    ///
    /// For example, `void Foo();` should have name `Foo`.
    fn debug_name(&self, ir: &IR) -> Rc<str>;

    /// The recorded source location, or None if none is present.
    fn source_loc(&self) -> Option<Rc<str>>;
}

impl<T> GenericItem for Rc<T>
where
    T: GenericItem + ?Sized,
{
    fn id(&self) -> ItemId {
        (**self).id()
    }
    fn debug_name(&self, ir: &IR) -> Rc<str> {
        (**self).debug_name(ir)
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        (**self).source_loc()
    }
}

/// Deserialize `IR` from JSON given as a reader.
pub fn deserialize_ir<R: Read>(reader: R) -> Result<IR> {
    let flat_ir = serde_json::from_reader(reader)?;
    make_ir(flat_ir)
}

/// Create a testing `IR` instance from given parts. This function does not use
/// any mock values.
pub fn make_ir_from_parts<CrubitFeatures>(
    items: Vec<Item>,
    public_headers: Vec<HeaderName>,
    current_target: BazelLabel,
    top_level_item_ids: Vec<ItemId>,
    crate_root_path: Option<Rc<str>>,
    crubit_features: HashMap<BazelLabel, CrubitFeatures>,
) -> Result<IR>
where
    CrubitFeatures: Into<flagset::FlagSet<CrubitFeature>>,
{
    make_ir(FlatIR {
        public_headers,
        current_target,
        items,
        top_level_item_ids,
        crate_root_path,
        crubit_features: crubit_features
            .into_iter()
            .map(|(label, features)| (label, CrubitFeaturesIR(features.into())))
            .collect(),
    })
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

    let mut function_name_to_functions = HashMap::<UnqualifiedIdentifier, Vec<Rc<Func>>>::new();
    flat_ir
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Func(func) => Some(func),
            _ => None,
        })
        .for_each(|f| {
            function_name_to_functions.entry(f.name.clone()).or_default().push(f.clone());
        });

    Ok(IR {
        flat_ir,
        item_id_to_item_idx,
        lifetimes,
        namespace_id_to_number_of_reopened_namespaces,
        reopened_namespace_id_to_idx,
        function_name_to_functions,
    })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeaderName {
    pub name: Rc<str>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct LifetimeId(pub i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LifetimeName {
    pub name: Rc<str>,
    pub id: LifetimeId,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RsType {
    pub name: Option<Rc<str>>,
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
#[serde(deny_unknown_fields)]
pub struct CcType {
    pub name: Option<Rc<str>>,
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
#[serde(deny_unknown_fields)]
pub struct MappedType {
    pub rs_type: RsType,
    pub cc_type: CcType,
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identifier {
    pub identifier: Rc<str>,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.identifier)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegerConstant {
    pub is_negative: bool,
    pub wrapped_value: u64,
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
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

impl Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.cc_name())
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

/// A Bazel label, e.g. `//foo:bar`.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(transparent)]
pub struct BazelLabel(pub Rc<str>);

impl BazelLabel {
    /// Returns the target name. E.g. `bar` for `//foo:bar`.
    pub fn target_name(&self) -> &str {
        if let Some((_package, target_name)) = self.0.split_once(':') {
            return target_name;
        }
        if let Some((_, last_package_component)) = self.0.rsplit_once('/') {
            return last_package_component;
        }
        &self.0
    }
}

impl<T: Into<String>> From<T> for BazelLabel {
    fn from(label: T) -> Self {
        Self(label.into().into())
    }
}

impl Display for BazelLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.0)
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

impl Debug for UnqualifiedIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnqualifiedIdentifier::Identifier(identifier) => Debug::fmt(identifier, f),
            UnqualifiedIdentifier::Operator(op) => Debug::fmt(op, f),
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
#[serde(deny_unknown_fields)]
pub struct InstanceMethodMetadata {
    pub reference: ReferenceQualification,
    pub is_const: bool,
    pub is_virtual: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemberFuncMetadata {
    pub record_id: ItemId,
    pub instance_method_metadata: Option<InstanceMethodMetadata>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FuncParam {
    #[serde(rename(deserialize = "type"))]
    pub type_: MappedType,
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Func {
    pub name: UnqualifiedIdentifier,
    pub owning_target: BazelLabel,
    pub mangled_name: Rc<str>,
    pub doc_comment: Option<Rc<str>>,
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
    pub source_loc: Rc<str>,
    pub id: ItemId,
    pub enclosing_namespace_id: Option<ItemId>,
    pub adl_enclosing_record: Option<ItemId>,
}

impl GenericItem for Func {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, ir: &IR) -> Rc<str> {
        let record: Option<Rc<str>> = ir.record_for_member_func(self).map(|r| r.debug_name(ir));
        let record: Option<&str> = record.as_deref();

        let func_name = match &self.name {
            UnqualifiedIdentifier::Identifier(id) => id.identifier.to_string(),
            UnqualifiedIdentifier::Operator(op) => op.cc_name(),
            UnqualifiedIdentifier::Destructor => {
                format!("~{}", record.expect("destructor must be associated with a record"))
            }
            UnqualifiedIdentifier::Constructor => {
                record.expect("constructor must be associated with a record").to_string()
            }
        };

        if let Some(record_name) = record {
            format!("{}::{}", record_name, func_name).into()
        } else {
            func_name.into()
        }
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        Some(self.source_loc.clone())
    }
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
#[serde(deny_unknown_fields)]
pub struct Field {
    pub identifier: Option<Identifier>,
    pub doc_comment: Option<Rc<str>>,
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
#[serde(deny_unknown_fields)]
pub struct BaseClass {
    pub base_record_id: ItemId,
    pub offset: Option<i64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IncompleteRecord {
    pub cc_name: Rc<str>,
    pub rs_name: Rc<str>,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub record_type: RecordType,
    pub enclosing_namespace_id: Option<ItemId>,
}

impl GenericItem for IncompleteRecord {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.cc_name.clone()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        None
    }
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
#[serde(deny_unknown_fields)]
pub struct Record {
    pub rs_name: Rc<str>,
    pub cc_name: Rc<str>,
    pub mangled_cc_name: Rc<str>,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub doc_comment: Option<Rc<str>>,
    pub source_loc: Rc<str>,
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

impl GenericItem for Record {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.cc_name.clone()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        Some(self.source_loc.clone())
    }
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
#[serde(deny_unknown_fields)]
pub struct Enum {
    pub identifier: Identifier,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub source_loc: Rc<str>,
    pub underlying_type: MappedType,
    pub enumerators: Vec<Enumerator>,
    pub enclosing_namespace_id: Option<ItemId>,
}

impl GenericItem for Enum {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.identifier.identifier.clone()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        Some(self.source_loc.clone())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Enumerator {
    pub identifier: Identifier,
    pub value: IntegerConstant,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeAlias {
    pub identifier: Identifier,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub doc_comment: Option<Rc<str>>,
    pub underlying_type: MappedType,
    pub source_loc: Rc<str>,
    pub enclosing_record_id: Option<ItemId>,
    pub enclosing_namespace_id: Option<ItemId>,
}

impl GenericItem for TypeAlias {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.identifier.identifier.clone()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        Some(self.source_loc.clone())
    }
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
#[serde(deny_unknown_fields)]
pub struct UnsupportedItem {
    pub name: Rc<str>,
    message: Rc<str>,
    pub source_loc: Option<Rc<str>>,
    pub id: ItemId,
    #[serde(skip)]
    cause: IgnoredField<OnceCell<Error>>,
}

impl GenericItem for UnsupportedItem {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.name.clone()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        self.source_loc.clone()
    }
}

impl UnsupportedItem {
    fn new(ir: &IR, item: &impl GenericItem, message: Rc<str>, cause: Option<Error>) -> Self {
        Self {
            name: item.debug_name(ir),
            message,
            source_loc: item.source_loc(),
            id: item.id(),
            cause: IgnoredField(cause.map(OnceCell::from).unwrap_or_default()),
        }
    }

    pub fn new_with_message(ir: &IR, item: &impl GenericItem, message: impl Into<Rc<str>>) -> Self {
        Self::new(ir, item, message.into(), None)
    }
    pub fn new_with_cause(ir: &IR, item: &impl GenericItem, cause: Error) -> Self {
        Self::new(ir, item, cause.to_string().into(), Some(cause))
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn cause(&self) -> &Error {
        self.cause.0.get_or_init(|| anyhow!(self.message.as_ref().to_owned()))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Comment {
    pub text: Rc<str>,
    pub id: ItemId,
}

impl GenericItem for Comment {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        "comment".into()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
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

impl GenericItem for Namespace {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.name.to_string().into()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UseMod {
    pub path: Rc<str>,
    pub mod_name: Identifier,
    pub id: ItemId,
}

impl GenericItem for UseMod {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        format!("[internal] use mod {}::* = {}", self.mod_name, self.path).into()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        None
    }
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

macro_rules! forward_item {
    (match $item:ident { _($item_name:ident) => $expr:expr $(,)? }) => {
        match $item {
                                                    Item::Func($item_name) => $expr,
                                                    Item::IncompleteRecord($item_name) => $expr,
                                                    Item::Record($item_name) => $expr,
                                                    Item::Enum($item_name) => $expr,
                                                    Item::TypeAlias($item_name) => $expr,
                                                    Item::UnsupportedItem($item_name) => $expr,
                                                    Item::Comment($item_name) => $expr,
                                                    Item::Namespace($item_name) => $expr,
                                                    Item::UseMod($item_name) => $expr,
                                                }
    };
}

impl GenericItem for Item {
    fn id(&self) -> ItemId {
        forward_item! {
            match self {
                _(x) => x.id()
            }
        }
    }
    fn debug_name(&self, ir: &IR) -> Rc<str> {
        forward_item! {
            match self {
                _(x) => x.debug_name(ir)
            }
        }
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        forward_item! {
            match self {
                _(x) => x.source_loc()
            }
        }
    }
}

impl Item {
    pub fn enclosing_namespace_id(&self) -> Option<ItemId> {
        match self {
            Item::Record(record) => record.enclosing_namespace_id,
            Item::IncompleteRecord(record) => record.enclosing_namespace_id,
            Item::Enum(enum_) => enum_.enclosing_namespace_id,
            Item::Func(func) => func.enclosing_namespace_id,
            Item::Namespace(namespace) => namespace.enclosing_namespace_id,
            Item::TypeAlias(type_alias) => type_alias.enclosing_namespace_id,
            Item::Comment(..) => None,
            Item::UnsupportedItem(..) => None,
            Item::UseMod(..) => None,
        }
    }

    /// Returns the target that this was defined in.
    pub fn defining_target(&self) -> Option<&BazelLabel> {
        // TODO(b/266727458): return the template target where applicable.
        self.owning_target()
    }

    /// Returns the target that this should generate source code in.
    pub fn owning_target(&self) -> Option<&BazelLabel> {
        match self {
            Item::Func(func) => Some(&func.owning_target),
            Item::IncompleteRecord(record) => Some(&record.owning_target),
            Item::Record(record) => Some(&record.owning_target),
            Item::Enum(e) => Some(&e.owning_target),
            Item::TypeAlias(type_alias) => Some(&type_alias.owning_target),
            Item::UnsupportedItem(..) => None,
            Item::Comment(..) => None,
            Item::Namespace(..) => None,
            Item::UseMod(..) => None,
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

flagset::flags! {
    pub enum CrubitFeature : u8 {
        Supported,
        /// Experimental is never *set* without also setting Supported, but we allow it to be
        /// *required* without also requiring Supported, so that error messages can be more direct.
        Experimental,
    }
}

impl CrubitFeature {
    /// The name of this feature.
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Experimental => "experimental",
        }
    }

    /// The aspect hint required to enable this feature.
    pub fn aspect_hint(&self) -> &'static str {
        match self {
            Self::Supported => "//:supported",
            Self::Experimental => "//:experimental",
        }
    }
}

/// A newtype around a flagset of features, so that it can be deserialized from
/// an array of strings instead of an integer.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct CrubitFeaturesIR(pub(crate) flagset::FlagSet<CrubitFeature>);

impl<'de> serde::Deserialize<'de> for CrubitFeaturesIR {
    fn deserialize<D>(deserializer: D) -> Result<CrubitFeaturesIR, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut features = flagset::FlagSet::<CrubitFeature>::default();
        for feature in <Vec<String> as serde::Deserialize<'de>>::deserialize(deserializer)? {
            features |= match &*feature {
                "experimental" => CrubitFeature::Experimental,
                "supported" => CrubitFeature::Supported,
                other => {
                    return Err(<D::Error as serde::de::Error>::custom(format!(
                        "Unexpected Crubit feature: {other}"
                    )));
                }
            };
        }
        Ok(CrubitFeaturesIR(features))
    }
}

#[derive(PartialEq, Eq, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename(deserialize = "IR"))]
struct FlatIR {
    #[serde(default)]
    public_headers: Vec<HeaderName>,
    current_target: BazelLabel,
    #[serde(default)]
    items: Vec<Item>,
    #[serde(default)]
    top_level_item_ids: Vec<ItemId>,
    #[serde(default)]
    crate_root_path: Option<Rc<str>>,
    #[serde(default)]
    crubit_features: HashMap<BazelLabel, CrubitFeaturesIR>,
}

/// A custom debug impl that wraps the HashMap in rustfmt-friendly notation.
///
/// See b/272530008.
impl Debug for FlatIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugHashMap<T: Debug>(pub T);
        impl<T: Debug> Debug for DebugHashMap<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // prefix the hash map with `hash_map!` so that the output can be fed to
                // rustfmt. The end result is something like `hash_map!{k:v,
                // k2:v2}`, which reads well.
                write!(f, "hash_map!")?;
                Debug::fmt(&self.0, f)
            }
        }
        // exhaustive-match so we don't forget to add fields to Debug when we add to
        // FlatIR.
        let FlatIR {
            public_headers,
            current_target,
            items,
            top_level_item_ids,
            crate_root_path,
            crubit_features,
        } = self;
        f.debug_struct("FlatIR")
            .field("public_headers", public_headers)
            .field("current_target", current_target)
            .field("items", items)
            .field("top_level_item_ids", top_level_item_ids)
            .field("crate_root_path", crate_root_path)
            .field("crubit_features", &DebugHashMap(crubit_features))
            .finish()
    }
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
    function_name_to_functions: HashMap<UnqualifiedIdentifier, Vec<Rc<Func>>>,
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

    /// Returns whether `target` is the current target.
    pub fn is_current_target(&self, target: &BazelLabel) -> bool {
        // TODO(hlopko): Make this be a pointer comparison, now it's comparing string
        // values.
        *target == *self.current_target()
    }

    /// Returns the Crubit features enabled for the given `target`.
    #[must_use]
    pub fn target_crubit_features(&self, target: &BazelLabel) -> flagset::FlagSet<CrubitFeature> {
        self.flat_ir.crubit_features.get(target).cloned().unwrap_or_default().0
    }

    /// Returns a mutable reference to the Crubit features enabled for the given
    /// `target`.
    ///
    /// Since IR is generally only held immutably, this is only useful for
    /// testing.
    #[must_use]
    pub fn target_crubit_features_mut(
        &mut self,
        target: &BazelLabel,
    ) -> &mut flagset::FlagSet<CrubitFeature> {
        // TODO(jeanpierreda): migrate to raw_entry_mut when stable.
        // (target is taken by reference exactly because ideally this function would use
        // the raw entry API.)
        &mut self.flat_ir.crubit_features.entry(target.clone()).or_default().0
    }

    pub fn current_target(&self) -> &BazelLabel {
        &self.flat_ir.current_target
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
    /// Panics if `Func` is a member function, but its `Record` is somehow not
    /// in `self`.
    pub fn record_for_member_func(&self, func: &Func) -> Option<&Rc<Record>> {
        if let Some(meta) = func.member_func_metadata.as_ref() {
            Some(
                self.find_decl(meta.record_id)
                    .with_context(|| {
                        format!("Failed to retrieve Record for MemberFuncMetadata of {:?}", func)
                    })
                    .unwrap(),
            )
        } else {
            None
        }
    }

    pub fn crate_root_path(&self) -> Option<Rc<str>> {
        self.flat_ir.crate_root_path.clone()
    }

    pub fn get_functions_by_name(
        &self,
        function_name: &UnqualifiedIdentifier,
    ) -> impl Iterator<Item = &Rc<Func>> {
        self.function_name_to_functions.get(function_name).map_or([].iter(), |v| v.iter())
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
            crubit_features: Default::default(),
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
        assert_eq!(ir.crate_root_path().as_deref(), Some("__cc_template_instantiations_rs_api"));
    }

    #[test]
    fn test_bazel_label_target() {
        let label: BazelLabel = "//foo:bar".into();
        assert_eq!(label.target_name(), "bar");
    }

    #[test]
    fn test_bazel_label_target_dotless() {
        let label: BazelLabel = "//foo".into();
        assert_eq!(label.target_name(), "foo");
    }

    #[test]
    fn test_bazel_label_dotless_slashless() {
        let label: BazelLabel = "foo".into();
        assert_eq!(label.target_name(), "foo");
    }

    /// These are not labels, but there is an unambiguous interpretation of
    /// what their target should be that lets us keep going.
    #[test]
    fn test_bazel_label_empty_target() {
        for s in ["foo:", "foo/", ""] {
            let label: BazelLabel = s.into();
            assert_eq!(label.target_name(), "", "label={s:?}");
        }
    }
}
