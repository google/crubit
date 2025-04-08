// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Types and deserialization logic for IR. See docs in
//! `rs_bindings_from_cc/ir.h` for more
//! information.

use arc_anyhow::{anyhow, bail, Context, Error, Result};
use code_gen_utils::{make_rs_ident, NamespaceQualifier};
use crubit_feature::CrubitFeature;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use serde::Deserialize;
use std::cell::OnceCell;
use std::collections::hash_map::{Entry, HashMap};
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Common data about all items.
pub trait GenericItem {
    fn id(&self) -> ItemId;

    /// The name of the item, readable by programmers.
    ///
    /// For example, `void Foo();` should have name `Foo`.
    fn debug_name(&self, ir: &IR) -> Rc<str>;

    /// If this item is unsupported by Crubit, we may generate
    /// markers in the Rust bindings to indicate that the item is not
    /// supported. This function returns the kind of unsupported item
    /// in order to generate such markers in the proper namespace
    /// (type, function, module).
    fn unsupported_kind(&self) -> UnsupportedItemKind;

    /// The recorded source location, or None if none is present.
    fn source_loc(&self) -> Option<Rc<str>>;

    /// A human-readable list of unknown attributes, or None if all attributes
    /// were understood.
    fn unknown_attr(&self) -> Option<Rc<str>>;

    /// Whether failure to generate binding should be treated as a hard error (`CRUBIT_MUST_BIND`).
    fn must_bind(&self) -> bool;
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
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        (**self).unsupported_kind()
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        (**self).source_loc()
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        (**self).unknown_attr()
    }
    fn must_bind(&self) -> bool {
        (**self).must_bind()
    }
}

/// Deserialize `IR` from JSON bytes.
pub fn deserialize_ir(bytes: &[u8]) -> Result<IR> {
    let flat_ir = serde_json::from_slice(bytes)?;
    Ok(make_ir(flat_ir))
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
) -> IR
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
            .map(|(label, features)| {
                (label, crubit_feature::SerializedCrubitFeatures(features.into()))
            })
            .collect(),
    })
}

fn make_ir(flat_ir: FlatIR) -> IR {
    let mut used_decl_ids = HashMap::new();
    for item in &flat_ir.items {
        if let Some(existing_decl) = used_decl_ids.insert(item.id(), item) {
            panic!("Duplicate decl_id found in {:?} and {:?}", existing_decl, item);
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
                    panic!(
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
            function_name_to_functions.entry(f.rs_name.clone()).or_default().push(f.clone());
        });

    IR {
        flat_ir,
        item_id_to_item_idx,
        lifetimes,
        namespace_id_to_number_of_reopened_namespaces,
        reopened_namespace_id_to_idx,
        function_name_to_functions,
    }
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
pub struct CcType {
    pub variant: CcTypeVariant,
    pub is_const: bool,
    pub unknown_attr: Rc<str>,
}

impl CcType {
    pub fn is_unit_type(&self) -> bool {
        matches!(&self.variant, CcTypeVariant::Primitive(Primitive::Void))
    }
}

#[derive(Copy, Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PointerTypeKind {
    LValueRef,
    RValueRef,
    Nullable,
    NonNull,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PointerType {
    pub kind: PointerTypeKind,
    pub lifetime: Option<LifetimeId>,
    pub pointee_type: Rc<CcType>,
}

/// Generates an enum type that implements `Deserialize`, which parses the stringified contents of
/// the braces, and `ToTokens`, which quotes the contents of the braces.
macro_rules! define_typed_tokens_enum {
    {$(#[$ty_attr:meta])* $vis:vis enum $Type:ident {$($(#[$variant_attr:meta])* $Variant:ident = {$($cpp_spelling:tt)+},)+}} => {
        $(#[$ty_attr])*
        $vis enum $Type {
            $(
                $(#[$variant_attr])*
                $Variant,
            )+
        }

        // #[serde(rename(deserialize = stringify!(...)))] doesn't work, so manual impl.
        impl<'de> serde::Deserialize<'de> for $Type {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct Visitor;
                impl serde::de::Visitor<'_> for Visitor {
                    type Value = $Type;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        const S: &str = concat!(
                            "one of: ",
                            $(
                                "\"",
                                stringify!($($cpp_spelling)+),
                                "\", "
                            ),+
                        );
                        formatter.write_str(&S[..S.len() - ", ".len()])
                    }

                    fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                        match s {
                            $(
                                stringify!($($cpp_spelling)+) => Ok($Type::$Variant),
                            )+
                            _ => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(s), &self))
                        }
                    }
                }
                deserializer.deserialize_str(Visitor)
            }
        }

        impl ToTokens for $Type {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                match self {
                    $(
                        Self::$Variant => quote! { $($cpp_spelling)+ }.to_tokens(tokens),
                    )+
                }
            }
        }
    }
}

define_typed_tokens_enum! {
    #[derive(Copy, Debug, PartialEq, Eq, Hash, Clone)]
    pub enum Primitive {
        Bool = {bool},
        Void = {void},
        Float = {float},
        Double = {double},
        Char = {char},
        SignedChar = {signed char},
        UnsignedChar = {unsigned char},
        Short = {short},
        Int = {int},
        Long = {long},
        LongLong = {long long},
        UnsignedShort = {unsigned short},
        UnsignedInt = {unsigned int},
        UnsignedLong = {unsigned long},
        UnsignedLongLong = {unsigned long long},
        Char16T = {char16_t},
        Char32T = {char32_t},
        PtrdiffT = {ptrdiff_t},
        IntptrT = {intptr_t},
        SizeT = {size_t},
        UintptrT = {uintptr_t},
        StdPtrdiffT = {std::ptrdiff_t},
        StdIntptrT = {std::intptr_t},
        StdSizeT = {std::size_t},
        StdUintptrT = {std::uintptr_t},
        Int8T = {int8_t},
        Int16T = {int16_t},
        Int32T = {int32_t},
        Int64T = {int64_t},
        StdInt8T = {std::int8_t},
        StdInt16T = {std::int16_t},
        StdInt32T = {std::int32_t},
        StdInt64T = {std::int64_t},
        Uint8T = {uint8_t},
        Uint16T = {uint16_t},
        Uint32T = {uint32_t},
        Uint64T = {uint64_t},
        StdUint8T = {std::uint8_t},
        StdUint16T = {std::uint16_t},
        StdUint32T = {std::uint32_t},
        StdUint64T = {std::uint64_t},
    }
}

define_typed_tokens_enum! {
    /// The C++ calling convention of a function.
    #[derive(Copy, Debug, PartialEq, Eq, Hash, Clone)]
    pub enum CcCallingConv {
        C = {cdecl},
        X86FastCall = {fastcall},
        X86VectorCall = {vectorcall},
        X86ThisCall = {thiscall},
        X86StdCall = {stdcall},
        Win64 = {ms_abi},
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CcTypeVariant {
    Primitive(Primitive),
    Pointer(PointerType),
    FuncPointer {
        non_null: bool,
        call_conv: CcCallingConv,

        /// The parameter types, followed by the return type.
        param_and_return_types: Rc<[CcType]>,
    },
    Record(ItemId),
}

impl CcTypeVariant {
    pub fn as_pointer(&self) -> Option<&PointerType> {
        match &self {
            CcTypeVariant::Pointer(pointer) => Some(pointer),
            _ => None,
        }
    }
}

impl CcTypeVariant {
    pub fn as_primitive(&self) -> Option<Primitive> {
        match &self {
            CcTypeVariant::Primitive(primitive) => Some(*primitive),
            _ => None,
        }
    }
}

pub trait TypeWithDeclId {
    fn decl_id(&self) -> Option<ItemId>;
}

impl TypeWithDeclId for CcType {
    fn decl_id(&self) -> Option<ItemId> {
        match &self.variant {
            CcTypeVariant::Record(id) => Some(*id),
            _ => None,
        }
    }
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

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.identifier.as_ref() == other
    }
}

impl PartialEq<&str> for Identifier {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
#[serde(transparent)]
pub struct ItemId(usize);

impl Debug for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ItemId({:#x})", self.0)
    }
}

impl ItemId {
    pub fn new_for_testing(value: usize) -> Self {
        Self(value)
    }
}

impl ToTokens for ItemId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use std::str::FromStr;
        proc_macro2::Literal::from_str(&format!("{:#x}", self.0)).unwrap().to_tokens(tokens)
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

    fn package_name(&self) -> &str {
        self.0.rsplit_once(':').unwrap_or((&self.0, "")).0
    }

    fn last_package_component(&self) -> &str {
        self.package_name().rsplit_once('/').unwrap_or(("", "")).1
    }
    // TODO(b/216587072): Remove this hacky escaping and use the import! macro once
    // available.
    // For now, use the simple escaping scheme of mapping all invalid characters
    // to underscore, instead of the one similar to `convert_to_cc_identifier`, so
    // that the escaped target name doesn't become longer (rustc currently produces
    // .o artifacts that repeat the target name twice, which can easily cause
    // the path length of artifacts to exceed the limit of the file system.)
    pub fn target_name_escaped(&self) -> String {
        let mut target_name = self.target_name().to_owned();
        if target_name == "core" {
            target_name = "core_".to_owned() + self.last_package_component();
        } else if target_name.starts_with(char::is_numeric) {
            target_name.insert(0, 'n');
        }
        target_name.replace(|c: char| !c.is_ascii_alphanumeric(), "_")
    }

    // Returns the bazel label as a valid C++ identifier, with a leading underscore.
    // Non-alphanumeric characters are escaped as `_xx`, where `xx` is the the byte
    // as hexadecimal.
    //
    // For instance, `//foo` becomes `__2f_2ffoo`.
    pub fn convert_to_cc_identifier(&self) -> String {
        use std::fmt::Write;
        let mut result = "_".to_string();
        result.reserve_exact(self.0.len().checked_mul(2).unwrap_or(self.0.len()));

        // This is yet another escaping scheme... :-/  Compare this with
        // https://github.com/bazelbuild/rules_rust/blob/1f2e6231de29d8fad8d21486f0d16403632700bf/rust/private/utils.bzl#L459-L586
        for b in self.0.bytes() {
            if (b as char).is_ascii_alphanumeric() {
                result.push(b as char);
            } else {
                write!(result, "_{b:02x}").unwrap();
            }
        }
        result.shrink_to_fit();

        #[cfg(debug_assertions)]
        for c in result.chars() {
            debug_assert!(
                c.is_ascii_alphanumeric() || c == '_',
                "invalid result identifier: {result:?}"
            );
        }

        result
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

impl PartialEq<str> for UnqualifiedIdentifier {
    fn eq(&self, other: &str) -> bool {
        if let UnqualifiedIdentifier::Identifier(identifier) = self {
            &*identifier.identifier == other
        } else {
            false
        }
    }
}

impl PartialEq<&str> for UnqualifiedIdentifier {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
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
    pub type_: CcType,
    pub identifier: Identifier,
    /// A human-readable list of attributes that Crubit doesn't understand.
    ///
    /// Because attributes can change the behavior or semantics of function
    /// parameters in ways that may affect interop, we default-closed and
    /// do not expose functions with unknown attributes.
    ///
    /// One notable example is `lifetimebound`, which we might expect to map
    /// to Rust lifetimes.
    pub unknown_attr: Option<Rc<str>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SafetyAnnotation {
    DisableUnsafe,
    Unsafe,
    Unannotated,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Func {
    pub cc_name: UnqualifiedIdentifier,
    pub rs_name: UnqualifiedIdentifier,
    pub owning_target: BazelLabel,
    pub mangled_name: Rc<str>,
    pub doc_comment: Option<Rc<str>>,
    pub return_type: CcType,
    pub params: Vec<FuncParam>,
    /// For tests and internal use only.
    ///
    /// Prefer to reconstruct the lifetime params from the parameter types, as
    /// needed. This allows new parameters and lifetimes to be added that were
    /// not originally part of the IR.
    pub lifetime_params: Vec<LifetimeName>,
    pub is_inline: bool,
    pub member_func_metadata: Option<MemberFuncMetadata>,
    pub is_extern_c: bool,
    pub is_noreturn: bool,
    /// The `[[nodiscard("...")]]` string. If `[[nodiscard]]`, then the empty
    /// string is used.
    pub nodiscard: Option<Rc<str>>,
    /// The `[[deprecated("...")]]` string. If `[[deprecated]]`, then the empty
    /// string is used.
    pub deprecated: Option<Rc<str>>,
    /// A human-readable list of attributes that Crubit doesn't understand.
    ///
    /// Because attributes can change the behavior or semantics of functions in
    /// fairly significant ways, and in ways that may affect interop, we
    /// default-closed and do not expose functions with unknown attributes.
    pub unknown_attr: Option<Rc<str>>,
    pub has_c_calling_convention: bool,
    pub is_member_or_descendant_of_class_template: bool,
    pub safety_annotation: SafetyAnnotation,
    pub source_loc: Rc<str>,
    pub id: ItemId,
    pub enclosing_item_id: Option<ItemId>,

    /// If this function was declared as a `friend` inside of a record
    /// definition, this ItemId refers to the record containing the `friend`
    /// function declaration.
    ///
    /// The record pointed to by `ItemId` must then be ADL-visible in order to
    /// invoke this function.
    pub adl_enclosing_record: Option<ItemId>,
    pub must_bind: bool,
}

impl GenericItem for Func {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, ir: &IR) -> Rc<str> {
        let record: Option<Rc<str>> = ir.record_for_member_func(self).map(|r| r.debug_name(ir));
        let record: Option<&str> = record.as_deref();

        let func_name = match &self.rs_name {
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
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Value
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        Some(self.source_loc.clone())
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        self.unknown_attr.clone()
    }
    fn must_bind(&self) -> bool {
        self.must_bind
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
    pub type_: Result<CcType, String>,
    pub access: AccessSpecifier,
    pub offset: usize,
    pub size: usize,

    /// A human-readable list of attributes that Crubit doesn't understand.
    pub unknown_attr: Option<Rc<str>>,

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
    pub cc_name: Rc<Identifier>,
    pub rs_name: Rc<Identifier>,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    /// A human-readable list of attributes that Crubit doesn't understand.
    ///
    /// Because attributes can change the behavior or semantics of types in
    /// fairly significant ways, and in ways that may affect interop, we
    /// default-closed and do not expose functions with unknown attributes.
    pub unknown_attr: Option<Rc<str>>,
    pub record_type: RecordType,
    pub enclosing_item_id: Option<ItemId>,
    pub must_bind: bool,
}

impl GenericItem for IncompleteRecord {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.cc_name.identifier.clone()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Type
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        None
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        self.unknown_attr.clone()
    }
    fn must_bind(&self) -> bool {
        self.must_bind
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
pub struct SizeAlign {
    pub size: usize,
    pub alignment: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum BridgeType {
    Annotation {
        rust_name: Rc<str>,
        rust_to_cpp_converter: Rc<str>,
        cpp_to_rust_converter: Rc<str>,
    },
    StdOptional(CcType),
    StdPair(CcType, CcType),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct TemplateArg {
    #[serde(rename(deserialize = "type"))]
    pub type_: Result<CcType, String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub struct TemplateSpecialization {
    pub template_name: Rc<str>,
    pub template_args: Vec<TemplateArg>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum TraitImplPolarity {
    Negative,
    None,
    Positive,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraitDerives {
    // <internal link> start
    pub clone: TraitImplPolarity,
    pub copy: TraitImplPolarity,
    pub debug: TraitImplPolarity,
    // <internal link> end
    pub send: bool,
    pub sync: bool,
    pub custom: Vec<Rc<str>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub rs_name: Rc<Identifier>,
    pub cc_name: Rc<Identifier>,
    pub cc_preferred_name: Rc<str>,
    pub mangled_cc_name: Rc<str>,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    /// The target containing the template definition, if this is a templated
    /// record type.
    pub defining_target: Option<BazelLabel>,
    pub template_specialization: Option<TemplateSpecialization>,
    /// A human-readable list of attributes that Crubit doesn't understand.
    ///
    /// Because attributes can change the behavior or semantics of types in
    /// fairly significant ways, and in ways that may affect interop, we
    /// default-closed and do not expose functions with unknown attributes.
    pub unknown_attr: Option<Rc<str>>,
    pub doc_comment: Option<Rc<str>>,
    pub bridge_type: Option<BridgeType>,
    pub source_loc: Rc<str>,
    pub unambiguous_public_bases: Vec<BaseClass>,
    pub fields: Vec<Field>,
    pub lifetime_params: Vec<LifetimeName>,
    pub size_align: SizeAlign,
    pub trait_derives: TraitDerives,
    pub is_derived_class: bool,
    pub override_alignment: bool,
    pub is_unsafe_type: bool,
    pub copy_constructor: SpecialMemberFunc,
    pub move_constructor: SpecialMemberFunc,
    pub destructor: SpecialMemberFunc,
    pub is_trivial_abi: bool,
    pub is_inheritable: bool,
    pub is_abstract: bool,
    /// The `[[nodiscard("...")]]` string. If `[[nodiscard]]`, then the empty
    /// string is used.
    pub nodiscard: Option<Rc<str>>,
    pub record_type: RecordType,
    pub is_aggregate: bool,
    pub is_anon_record_with_typedef: bool,
    pub child_item_ids: Vec<ItemId>,
    pub enclosing_item_id: Option<ItemId>,
    pub must_bind: bool,
}

impl GenericItem for Record {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.cc_name.identifier.clone()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Type
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        Some(self.source_loc.clone())
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        self.unknown_attr.clone()
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

impl Record {
    /// Whether this type has Rust-like object semantics for mutating
    /// assignment, and can be passed by mut reference as a result.
    ///
    /// If a type `T` is mut reference safe, it can be possed as a `&mut T`
    /// safely. Otherwise, mutable references must use `Pin<&mut T>`.
    ///
    /// In C++, this is called "trivially relocatable". Such types can be passed
    /// by value and have their memory directly mutated by Rust using
    /// memcpy-like assignment/swap.
    ///
    /// Described in more detail at: docs/unpin
    pub fn is_unpin(&self) -> bool {
        self.is_trivial_abi
    }

    pub fn is_union(&self) -> bool {
        match self.record_type {
            RecordType::Union => true,
            RecordType::Struct | RecordType::Class => false,
        }
    }

    /// Returns a `TokenStream` containing the C++ tag kind for this record.
    ///
    /// This is the `struct`, `union`, or `class` keyword, or nothing if this
    /// is an anonymous record with a typedef.
    pub fn cc_tag_kind(&self) -> TokenStream {
        if self.is_anon_record_with_typedef {
            quote! {}
        } else {
            self.record_type.into_token_stream()
        }
    }

    pub fn should_implement_drop(&self) -> bool {
        match self.destructor {
            // TODO(b/202258760): Only omit destructor if `Copy` is specified.
            SpecialMemberFunc::Trivial => false,

            // TODO(b/212690698): Avoid calling into the C++ destructor (e.g. let
            // Rust drive `drop`-ing) to avoid (somewhat unergonomic) ManuallyDrop
            // if we can ask Rust to preserve C++ field destruction order in
            // NontrivialMembers case.
            SpecialMemberFunc::NontrivialMembers => true,

            // The `impl Drop` for NontrivialUserDefined needs to call into the
            // user-defined destructor on C++ side.
            SpecialMemberFunc::NontrivialUserDefined => true,

            // TODO(b/213516512): Today the IR doesn't contain Func entries for
            // deleted functions/destructors/etc. But, maybe we should generate
            // `impl Drop` in this case? With `unreachable!`? With
            // `std::mem::forget`?
            SpecialMemberFunc::Unavailable => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalVar {
    pub cc_name: Identifier,
    pub rs_name: Identifier,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub source_loc: Rc<str>,
    /// A human-readable list of attributes that Crubit doesn't understand.
    pub unknown_attr: Option<Rc<str>>,
    pub enclosing_item_id: Option<ItemId>,
    pub mangled_name: Option<Rc<str>>,
    #[serde(rename(deserialize = "type"))]
    pub type_: CcType,
    pub must_bind: bool,
}

impl GenericItem for GlobalVar {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.rs_name.identifier.clone()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Value
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        Some(self.source_loc.clone())
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        self.unknown_attr.clone()
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Enum {
    pub cc_name: Identifier,
    pub rs_name: Identifier,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub source_loc: Rc<str>,
    pub underlying_type: CcType,
    /// The enumerators. If None, this is a forward-declared (opaque) enum.
    ///
    /// That is, the difference between `enum X : int {};` and `enum X : int;`
    /// is that the former has `Some(vec![])` for the enumerators, while the
    /// latter has `None`.
    pub enumerators: Option<Vec<Enumerator>>,
    /// A human-readable list of attributes that Crubit doesn't understand.
    pub unknown_attr: Option<Rc<str>>,
    pub enclosing_item_id: Option<ItemId>,
    pub must_bind: bool,
}

impl GenericItem for Enum {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.rs_name.identifier.clone()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Type
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        Some(self.source_loc.clone())
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        self.unknown_attr.clone()
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Enumerator {
    pub identifier: Identifier,
    pub value: IntegerConstant,
    /// A human-readable list of attributes that Crubit doesn't understand.
    pub unknown_attr: Option<Rc<str>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeAlias {
    pub cc_name: Identifier,
    pub rs_name: Identifier,
    pub id: ItemId,
    pub owning_target: BazelLabel,
    pub doc_comment: Option<Rc<str>>,
    /// A human-readable list of attributes that Crubit doesn't understand.
    pub unknown_attr: Option<Rc<str>>,
    pub underlying_type: CcType,
    pub source_loc: Rc<str>,
    pub enclosing_item_id: Option<ItemId>,
    pub must_bind: bool,
}

impl GenericItem for TypeAlias {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.rs_name.identifier.clone()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Type
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        Some(self.source_loc.clone())
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        self.unknown_attr.clone()
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

impl Display for TypeAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {})", self.rs_name, self.owning_target, self.source_loc)
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

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormattedError {
    pub fmt: Rc<str>,
    pub message: Rc<str>,
}

/// Which Rust namespace a particular unsupported item belongs to.
/// See https://doc.rust-lang.org/reference/names/namespaces.html
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum UnsupportedItemKind {
    Value,
    Type,
    Unnameable,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnsupportedItemPath {
    pub ident: UnqualifiedIdentifier,
    pub enclosing_item_id: Option<ItemId>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnsupportedItem {
    pub name: Rc<str>,
    pub kind: UnsupportedItemKind,
    pub path: Option<UnsupportedItemPath>,
    errors: Vec<Rc<FormattedError>>,
    pub source_loc: Option<Rc<str>>,
    pub id: ItemId,
    pub must_bind: bool,

    /// Stores either one natively generated [`arc_anyhow::Error`] or the
    /// memoized result of converting `errors`.
    #[serde(skip)]
    cause: IgnoredField<OnceCell<Vec<Error>>>,
}

impl GenericItem for UnsupportedItem {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.name.clone()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        self.kind
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        self.source_loc.clone()
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        None
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

impl UnsupportedItem {
    fn new(
        ir: &IR,
        item: &impl GenericItem,
        path: Option<UnsupportedItemPath>,
        error: Option<Rc<FormattedError>>,
        cause: Option<Error>,
        must_bind: bool,
    ) -> Self {
        Self {
            name: item.debug_name(ir),
            errors: error.into_iter().collect(),
            kind: item.unsupported_kind(),
            path,
            source_loc: item.source_loc(),
            id: item.id(),
            cause: IgnoredField(cause.map(|e| OnceCell::from(vec![e])).unwrap_or_default()),
            must_bind,
        }
    }

    pub fn new_with_static_message(
        ir: &IR,
        item: &impl GenericItem,
        path: Option<UnsupportedItemPath>,
        message: &'static str,
    ) -> Self {
        Self::new(
            ir,
            item,
            path,
            Some(Rc::new(FormattedError { fmt: message.into(), message: message.into() })),
            None,
            item.must_bind(),
        )
    }

    pub fn new_with_cause(
        ir: &IR,
        item: &impl GenericItem,
        path: Option<UnsupportedItemPath>,
        cause: Error,
    ) -> Self {
        Self::new(ir, item, path, None, Some(cause), item.must_bind())
    }

    pub fn errors(&self) -> &[Error] {
        self.cause.0.get_or_init(|| {
            self.errors
                .iter()
                .map(|e| {
                    error_report::FormattedError {
                        fmt: e.fmt.to_string().into(),
                        message: e.message.to_string().into(),
                    }
                    .into()
                })
                .collect()
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Comment {
    pub text: Rc<str>,
    pub id: ItemId,
    pub must_bind: bool,
}

impl GenericItem for Comment {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        "comment".into()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Unnameable
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        None
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        None
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Namespace {
    pub cc_name: Identifier,
    pub rs_name: Identifier,
    pub id: ItemId,
    pub canonical_namespace_id: ItemId,
    /// A human-readable list of attributes that Crubit doesn't understand.
    pub unknown_attr: Option<Rc<str>>,
    pub owning_target: BazelLabel,
    #[serde(default)]
    pub child_item_ids: Vec<ItemId>,
    pub enclosing_item_id: Option<ItemId>,
    pub is_inline: bool,
    pub must_bind: bool,
}

impl GenericItem for Namespace {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.rs_name.to_string().into()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Type
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        None
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        self.unknown_attr.clone()
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UseMod {
    pub path: Rc<str>,
    pub mod_name: Identifier,
    pub id: ItemId,
    pub must_bind: bool,
}

impl GenericItem for UseMod {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        format!("[internal] use mod {}::* = {}", self.mod_name, self.path).into()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Unnameable
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        None
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        None
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeMapOverride {
    pub rs_name: Rc<str>,
    pub cc_name: Rc<str>,
    pub type_parameters: Vec<CcType>,
    pub owning_target: BazelLabel,
    pub size_align: Option<SizeAlign>,
    pub is_same_abi: bool,
    pub id: ItemId,
    pub must_bind: bool,
}

impl GenericItem for TypeMapOverride {
    fn id(&self) -> ItemId {
        self.id
    }
    fn debug_name(&self, _: &IR) -> Rc<str> {
        self.cc_name.clone()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Type
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        None
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        None
    }
    fn must_bind(&self) -> bool {
        self.must_bind
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum Item {
    Func(Rc<Func>),
    IncompleteRecord(Rc<IncompleteRecord>),
    Record(Rc<Record>),
    Enum(Rc<Enum>),
    GlobalVar(Rc<GlobalVar>),
    TypeAlias(Rc<TypeAlias>),
    UnsupportedItem(Rc<UnsupportedItem>),
    Comment(Rc<Comment>),
    Namespace(Rc<Namespace>),
    UseMod(Rc<UseMod>),
    TypeMapOverride(Rc<TypeMapOverride>),
}

macro_rules! forward_item {
    (match $item:ident { _($item_name:ident) => $expr:expr $(,)? }) => {
        match $item {
            Item::Func($item_name) => $expr,
            Item::IncompleteRecord($item_name) => $expr,
            Item::Record($item_name) => $expr,
            Item::Enum($item_name) => $expr,
            Item::GlobalVar($item_name) => $expr,
            Item::TypeAlias($item_name) => $expr,
            Item::UnsupportedItem($item_name) => $expr,
            Item::Comment($item_name) => $expr,
            Item::Namespace($item_name) => $expr,
            Item::UseMod($item_name) => $expr,
            Item::TypeMapOverride($item_name) => $expr,
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
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        use Item::*;
        match self {
            Func(_) => UnsupportedItemKind::Value,
            IncompleteRecord(_) => UnsupportedItemKind::Type,
            Record(_) => UnsupportedItemKind::Type,
            Enum(_) => UnsupportedItemKind::Type,
            GlobalVar(_) => UnsupportedItemKind::Value,
            TypeAlias(_) => UnsupportedItemKind::Type,
            UnsupportedItem(unsupported) => unsupported.kind,
            Comment(_) => UnsupportedItemKind::Unnameable,
            Namespace(_) => UnsupportedItemKind::Type,
            UseMod(_) => UnsupportedItemKind::Unnameable,
            TypeMapOverride(_) => UnsupportedItemKind::Type,
        }
    }
    fn source_loc(&self) -> Option<Rc<str>> {
        forward_item! {
            match self {
                _(x) => x.source_loc()
            }
        }
    }
    fn unknown_attr(&self) -> Option<Rc<str>> {
        forward_item! {
            match self {
                _(x) => x.unknown_attr()
            }
        }
    }
    fn must_bind(&self) -> bool {
        forward_item! {
            match self {
                _(x) => x.must_bind()
            }
        }
    }
}

impl Item {
    pub fn enclosing_item_id(&self) -> Option<ItemId> {
        match self {
            Item::Record(record) => record.enclosing_item_id,
            Item::IncompleteRecord(record) => record.enclosing_item_id,
            Item::Enum(enum_) => enum_.enclosing_item_id,
            Item::GlobalVar(type_) => type_.enclosing_item_id,
            Item::Func(func) => func.enclosing_item_id,
            Item::Namespace(namespace) => namespace.enclosing_item_id,
            Item::TypeAlias(type_alias) => type_alias.enclosing_item_id,
            Item::Comment(..) => None,
            Item::UnsupportedItem(unsupported) => {
                unsupported.path.as_ref().and_then(|p| p.enclosing_item_id)
            }
            Item::UseMod(..) => None,
            Item::TypeMapOverride(..) => None,
        }
    }

    /// Returns the target that this was defined in, if it was defined somewhere
    /// other than `owning_target()`.
    pub fn defining_target(&self) -> Option<&BazelLabel> {
        match self {
            Item::Record(record) => record.defining_target.as_ref(),
            _ => None,
        }
    }

    /// Returns the target that this should generate source code in.
    pub fn owning_target(&self) -> Option<&BazelLabel> {
        match self {
            Item::Func(func) => Some(&func.owning_target),
            Item::IncompleteRecord(record) => Some(&record.owning_target),
            Item::Record(record) => Some(&record.owning_target),
            Item::Enum(e) => Some(&e.owning_target),
            Item::GlobalVar(b) => Some(&b.owning_target),
            Item::TypeAlias(type_alias) => Some(&type_alias.owning_target),
            Item::UnsupportedItem(..) => None,
            Item::Comment(..) => None,
            Item::Namespace(ns) => Some(&ns.owning_target),
            Item::UseMod(..) => None,
            Item::TypeMapOverride(type_override) => Some(&type_override.owning_target),
        }
    }

    /// Returns true if this corresponds to the definition of a new name for a
    /// type.
    pub fn is_type_definition(&self) -> bool {
        match self {
            Item::Func(_) => false,
            Item::IncompleteRecord(_) => true,
            Item::Record(_) => true,
            Item::Enum(_) => true,
            Item::GlobalVar(_) => false,
            Item::TypeAlias(_) => true,
            Item::UnsupportedItem(_) => false,
            Item::Comment(_) => false,
            Item::Namespace(_) => false,
            Item::UseMod(_) => false,
            Item::TypeMapOverride(_) => false,
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
        if let Item::Func(f) = value {
            Ok(f)
        } else {
            bail!("Not a Func: {:#?}", value)
        }
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
        if let Item::Record(r) = value {
            Ok(r)
        } else {
            bail!("Not a Record: {:#?}", value)
        }
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
        if let Item::Comment(c) = value {
            Ok(c)
        } else {
            bail!("Not a Comment: {:#?}", value)
        }
    }
}

impl From<Namespace> for Item {
    fn from(ns: Namespace) -> Item {
        Item::Namespace(Rc::new(ns))
    }
}

impl<'a> TryFrom<&'a Item> for &'a Rc<Namespace> {
    type Error = Error;
    fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
        if let Item::Namespace(c) = value {
            Ok(c)
        } else {
            bail!("Not a Namespace: {:#?}", value)
        }
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
    crubit_features: HashMap<BazelLabel, crubit_feature::SerializedCrubitFeatures>,
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

    pub fn type_aliases(&self) -> impl Iterator<Item = &Rc<TypeAlias>> {
        self.items().filter_map(|item| match item {
            Item::TypeAlias(type_alias) => Some(type_alias),
            _ => None,
        })
    }

    pub fn records(&self) -> impl Iterator<Item = &Rc<Record>> {
        self.items().filter_map(|item| match item {
            Item::Record(record) => Some(record),
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
            Ok(self.find_untyped_decl(decl_id))
        } else {
            bail!("Type {:?} does not have an associated item.", ty)
        }
    }

    #[track_caller]
    pub fn find_decl<'a, T>(&'a self, decl_id: ItemId) -> Result<&'a T>
    where
        &'a T: TryFrom<&'a Item>,
    {
        self.find_untyped_decl(decl_id).try_into().map_err(|_| {
            anyhow!("DeclId {:?} doesn't refer to a {}", decl_id, std::any::type_name::<T>())
        })
    }

    #[track_caller]
    pub fn find_untyped_decl(&self, decl_id: ItemId) -> &Item {
        let Some(idx) = self.item_id_to_item_idx.get(&decl_id) else {
            panic!("Couldn't find decl_id {:?} in the IR:\n{:#?}", decl_id, self.flat_ir)
        };
        let Some(item) = self.flat_ir.items.get(*idx) else {
            panic!("Couldn't find an item at idx {} in IR:\n{:#?}", idx, self.flat_ir)
        };
        item
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

    /// Returns the `Item` defining `func`, or `None` if `func` is not a
    /// member function.
    ///
    /// Note that even if `func` is a member function, the associated record
    /// might not be a Record IR Item (e.g. it has its type changed via
    /// crubit_internal_rust_type).
    pub fn record_for_member_func(&self, func: &Func) -> Option<&Item> {
        if let Some(meta) = func.member_func_metadata.as_ref() {
            Some(self.find_untyped_decl(meta.record_id))
        } else {
            None
        }
    }

    pub fn crate_root_path(&self) -> Option<Rc<str>> {
        self.flat_ir.crate_root_path.clone()
    }

    pub fn crate_root_path_tokens(&self) -> TokenStream {
        match self.crate_root_path().as_deref().map(make_rs_ident) {
            None => quote! { crate },
            Some(crate_root_path) => quote! { crate :: #crate_root_path },
        }
    }

    pub fn get_functions_by_name(
        &self,
        function_name: &UnqualifiedIdentifier,
    ) -> impl Iterator<Item = &Rc<Func>> {
        self.function_name_to_functions.get(function_name).map_or([].iter(), |v| v.iter())
    }

    pub fn namespace_qualifier(&self, item: &impl GenericItem) -> NamespaceQualifier {
        self.namespace_qualifier_from_id(item.id())
    }

    #[track_caller]
    fn namespace_qualifier_from_id(&self, item: ItemId) -> NamespaceQualifier {
        let mut namespaces = vec![];
        let item: &Item = self.find_decl(item).unwrap();
        let mut enclosing_item_id = item.enclosing_item_id();
        while let Some(parent_id) = enclosing_item_id {
            match self.find_decl(parent_id).expect("No item found for enclosing_item_id") {
                Item::Namespace(ns) => {
                    namespaces.push(ns.rs_name.identifier.clone());
                    enclosing_item_id = ns.enclosing_item_id;
                }
                // TODO(b/200067824): This can lead to bugs, if this is used without checking for a
                // parent struct. This function will likely need to be expanded to navigate into
                // records, as part of b/200067824.
                Item::Record { .. } => {
                    assert!(
                        namespaces.is_empty(),
                        "Record was listed as the enclosing item for a namespace."
                    );
                    break;
                }
                _ => panic!("Expected namespace, found enclosing item {item:?}"),
            }
        }
        NamespaceQualifier::new(namespaces.into_iter().rev())
    }
}

// TODO(jeanpierreda): This should probably be a method on IR accepting a GenericItem,
// and returning the crate name, or similar.

/// Returns Some(crate_ident) if this is an imported crate.
pub fn rs_imported_crate_name(owning_target: &BazelLabel, ir: &IR) -> Option<Ident> {
    if ir.is_current_target(owning_target) {
        None
    } else {
        let owning_crate = make_rs_ident(&owning_target.target_name_escaped());
        Some(owning_crate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_identifier_debug_print() {
        assert_eq!(format!("{:?}", Identifier { identifier: "hello".into() }), "\"hello\"");
    }

    #[gtest]
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

    #[gtest]
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

    #[gtest]
    fn test_empty_crate_root_path() {
        let input = "{ \"current_target\": \"//foo:bar\" }";
        let ir = deserialize_ir(input.as_bytes()).unwrap();
        assert_eq!(ir.crate_root_path(), None);
    }

    #[gtest]
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

    #[gtest]
    fn test_bazel_label_target() {
        let label: BazelLabel = "//foo:bar".into();
        assert_eq!(label.target_name(), "bar");
    }

    #[gtest]
    fn test_bazel_label_target_dotless() {
        let label: BazelLabel = "//foo".into();
        assert_eq!(label.target_name(), "foo");
    }

    #[gtest]
    fn test_bazel_label_dotless_slashless() {
        let label: BazelLabel = "foo".into();
        assert_eq!(label.target_name(), "foo");
    }

    /// These are not labels, but there is an unambiguous interpretation of
    /// what their target should be that lets us keep going.
    #[gtest]
    fn test_bazel_label_empty_target() {
        for s in ["foo:", "foo/", ""] {
            let label: BazelLabel = s.into();
            assert_eq!(label.target_name(), "", "label={s:?}");
        }
    }

    #[gtest]
    fn test_bazel_label_escape_target_name_with_relative_label() {
        let label: BazelLabel = "foo".into();
        assert_eq!(label.target_name_escaped(), "foo");
    }

    #[gtest]
    fn test_bazel_label_escape_target_name_with_invalid_characters() {
        let label: BazelLabel = "//:!./%-@^#$&()*-+,;<=>?[]{|}~".into();
        assert_eq!(label.target_name_escaped(), "___________________________");
    }

    #[gtest]
    fn test_bazel_label_escape_target_name_core() {
        let label: BazelLabel = "//foo~:core".into();
        assert_eq!(label.target_name_escaped(), "core_foo_");
    }

    #[gtest]
    fn test_bazel_label_escape_target_name_with_no_target_name() {
        let label: BazelLabel = "//foo/bar~".into();
        assert_eq!(label.target_name_escaped(), "bar_");
    }

    #[gtest]
    fn test_bazel_label_escape_target_name_with_no_package_name() {
        let label: BazelLabel = "//:foo~".into();
        assert_eq!(label.target_name_escaped(), "foo_");
    }

    #[gtest]
    fn test_bazel_label_escape_target_name_core_with_no_package_name_with_no_target_name() {
        let label: BazelLabel = "core".into();
        assert_eq!(label.target_name_escaped(), "core_");
    }

    #[gtest]
    fn test_bazel_label_escape_target_name_starting_with_digit() {
        let label: BazelLabel = "12345".into();
        assert_eq!(label.target_name_escaped(), "n12345");
    }

    #[gtest]
    fn test_bazel_to_cc_identifier_empty() {
        assert_eq!(BazelLabel::from("").convert_to_cc_identifier(), "_");
    }

    #[gtest]
    fn test_bazel_to_cc_identifier_alphanumeric_not_transformed() {
        assert_eq!(BazelLabel::from("abc").convert_to_cc_identifier(), "_abc");
        assert_eq!(BazelLabel::from("foo123").convert_to_cc_identifier(), "_foo123");
        assert_eq!(BazelLabel::from("123foo").convert_to_cc_identifier(), "_123foo");
    }

    #[gtest]
    fn test_bazel_to_cc_identifier_simple_targets() {
        assert_eq!(
            BazelLabel::from("//foo/bar:baz_abc").convert_to_cc_identifier(),
            "__2f_2ffoo_2fbar_3abaz_5fabc"
        );
    }

    #[gtest]
    fn test_bazel_to_cc_identifier_conflict() {
        assert_ne!(
            BazelLabel::from("//foo_bar:baz").convert_to_cc_identifier(),
            BazelLabel::from("//foo/bar:baz").convert_to_cc_identifier()
        );
    }
}
