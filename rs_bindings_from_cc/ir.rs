// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Types and deserialization logic for IR. See docs in
//! `rs_bindings_from_cc/ir.h` for more
//! information.
use arc_anyhow::{bail, ensure, Context, Error, Result};
use code_gen_utils::{make_rs_ident, try_make_rs_ident};
use crubit_feature::CrubitFeature;
pub mod ir_ffi_support;
pub mod ir_generated;
pub use ir_generated as ir_rust_proto;

use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::hash_map::{Entry, HashMap};
use std::collections::{BTreeMap, HashSet};
use std::fmt::{self, Debug, Display};

pub use ir_ffi_support::{
    BazelLabel, FfiMap, FfiRc, FfiRc as Rc, FfiString, FfiVec, ItemId, LifetimeId,
};
pub use ir_generated::array_value::{ArrayElements, IntegerElements, Kind as ArrayValueKind};
pub use ir_generated::bridge_type::callable::{BackingType, FnTrait};
pub use ir_generated::cc_type::{
    ConstantArrayType, FuncPointer, PointerType, Variant as CcTypeVariant,
};
pub use ir_generated::constant::Kind as ConstantValue;
pub use ir_generated::instance_method_metadata::ReferenceQualification;
pub use ir_generated::member_func_semantic::{Getter, Setter};
pub use ir_generated::template_specialization::Kind as TemplateSpecializationKind;
pub use ir_generated::unsupported_item::{
    Kind as UnsupportedItemKind, Path as UnsupportedItemPath,
};
pub use ir_generated::{
    AccessSpecifier, ArrayValue, BaseClass, BridgeType, CallingConv as CcCallingConv, CcType,
    Comment, Constant, DeclLocation, Enum, Enumerator, ExistingRustType, Field, FormattedError,
    Func, FuncParam, GlobalVar, HeaderName, Identifier, IncompleteRecord, InstanceMethodMetadata,
    IntegerConstant, Item, LifetimeName, MemberFuncSemantic, Namespace, Operator, OwnedPtrConfig,
    PointerTypeKind, Primitive, Record, RecordType, SafetyAnnotation, SizeAlign, SourceLocation,
    SpecialMemberFunc, StatusOrOptionalString, TemplateArg, TemplateSpecialization, TraitDerives,
    TraitImplPolarity, TypeAlias, UnqualifiedIdentifier, UnsupportedItem, UseMod,
};

/// Common data about all items.
pub trait GenericItem {
    fn id(&self) -> ItemId;

    /// The unique name (probably the USR) of the item for log aggregation purposes.
    fn unique_name(&self) -> Option<&str>;

    /// The Bazel target which owns the bindings for this item.
    fn owning_target(&self) -> Option<BazelLabel>;

    /// If this item is unsupported by Crubit, we may generate
    /// markers in the Rust bindings to indicate that the item is not
    /// supported. This function returns the kind of unsupported item
    /// in order to generate such markers in the proper namespace
    /// (type, function, module).
    fn unsupported_kind(&self) -> UnsupportedItemKind;

    /// Where the item is declared, or None if unknown.
    fn decl_loc(&self) -> Option<&DeclLocation> {
        None
    }

    /// A human-readable list of unknown attributes, or None if all attributes
    /// were understood.
    fn unknown_attr(&self) -> Option<&str>;

    /// Whether failure to generate binding should be treated as a hard error (`CRUBIT_MUST_BIND`).
    fn must_bind(&self) -> bool;

    /// Returns the C++ name of this item, if applicable.
    fn cc_name_as_str(&self) -> Option<&str> {
        None
    }
}

impl<T> GenericItem for FfiRc<T>
where
    T: GenericItem + ir_ffi_support::FfiDefault,
{
    fn id(&self) -> ItemId {
        (**self).id()
    }
    fn unique_name(&self) -> Option<&str> {
        (**self).unique_name()
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        (**self).owning_target()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        (**self).unsupported_kind()
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        (**self).decl_loc()
    }
    fn unknown_attr(&self) -> Option<&str> {
        (**self).unknown_attr()
    }
    fn must_bind(&self) -> bool {
        (**self).must_bind()
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        (**self).cc_name_as_str()
    }
}

impl<T> GenericItem for std::rc::Rc<T>
where
    T: GenericItem + ?Sized,
{
    fn id(&self) -> ItemId {
        (**self).id()
    }
    fn unique_name(&self) -> Option<&str> {
        (**self).unique_name()
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        (**self).owning_target()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        (**self).unsupported_kind()
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        (**self).decl_loc()
    }
    fn unknown_attr(&self) -> Option<&str> {
        (**self).unknown_attr()
    }
    fn must_bind(&self) -> bool {
        (**self).must_bind()
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        (**self).cc_name_as_str()
    }
}

/// A pre-order depth-first search iterator over the nested IR tree.
/// Preserves C++ declaration order and allows bindings generator components (e.g., `has_bindings`)
/// that expect a flat list of items to work in-place on the nested IR without a full rewrite.
pub struct ItemsIterator<'iter> {
    stack: Vec<&'iter Item>,
    visited: HashSet<ItemId>,
}

impl<'iter> ItemsIterator<'iter> {
    fn new(mut stack: Vec<&'iter Item>) -> Self {
        // Reverse so we pop roots in their original order.
        stack.reverse();
        Self { stack, visited: HashSet::new() }
    }
}

impl<'iter> Iterator for ItemsIterator<'iter> {
    type Item = &'iter Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            let id = item.id();
            if !self.visited.insert(id) {
                continue;
            }
            match item {
                Item::Record(record) => {
                    self.stack.extend(record.children.iter().rev());
                }
                Item::Namespace(ns) => {
                    self.stack.extend(ns.children.iter().rev());
                }
                _ => {}
            }
            return Some(item);
        }
        None
    }
}

// Read-only traversal to populate the lookup cache with deserialized items.
fn populate_item_id_to_item(item: &Item, item_id_to_item: &mut HashMap<ItemId, Item>) {
    match item_id_to_item.entry(item.id()) {
        Entry::Vacant(vacant) => {
            vacant.insert(item.clone());
        }
        Entry::Occupied(occupied) => {
            if occupied.get() != item {
                panic!("Duplicate decl_id found in {:?} and {:?}", occupied.get(), item);
            }
        }
    }
    match item {
        Item::Record(record) => {
            for child in &record.children {
                populate_item_id_to_item(child, item_id_to_item);
            }
        }
        Item::Namespace(ns) => {
            for child in &ns.children {
                populate_item_id_to_item(child, item_id_to_item);
            }
        }
        _ => {}
    }
}

#[cfg_attr(enable_heap_profiling, inline(never))]
pub fn make_ir(tree_ir: TreeIR) -> IR {
    let mut item_id_to_item = HashMap::new();

    for items in tree_ir.top_level_items.values() {
        for item in items {
            populate_item_id_to_item(item, &mut item_id_to_item);
        }
    }

    let ordered_items =
        ItemsIterator::new(tree_ir.top_level_items.values().flat_map(|v| v.iter()).collect());

    let mut lifetimes: HashMap<LifetimeId, LifetimeName> = HashMap::new();
    let mut namespace_id_to_number_of_reopened_namespaces = HashMap::new();
    let mut reopened_namespace_id_to_idx = HashMap::new();
    let mut function_name_to_functions: HashMap<UnqualifiedIdentifier, Vec<Rc<Func>>> =
        HashMap::new();

    for item in ordered_items {
        let lifetime_params = match item {
            Item::Record(record) => &record.lifetime_params[..],
            Item::Func(func) => &func.lifetime_params[..],
            _ => &[],
        };
        for lifetime in lifetime_params {
            match lifetimes.entry(lifetime.id) {
                Entry::Occupied(occupied) => {
                    panic!(
                        "Duplicate use of lifetime ID {:?} in item {item:?} for names: '{}, '{}",
                        lifetime.id,
                        occupied.get().name,
                        lifetime.name
                    )
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(lifetime.clone());
                }
            }
        }

        if let Item::Namespace(ns) = item
            && Namespace::owning_target(ns) == &tree_ir.current_target
        {
            let canonical_id = ns.canonical_namespace_id();
            let current_count =
                *namespace_id_to_number_of_reopened_namespaces.entry(canonical_id).or_insert(0);
            reopened_namespace_id_to_idx.insert(ns.id(), current_count);
            namespace_id_to_number_of_reopened_namespaces.insert(canonical_id, current_count + 1);
        }

        if let Item::Func(func) = item {
            function_name_to_functions.entry(func.rs_name.clone()).or_default().push(func.clone());
        }
    }

    IR {
        tree_ir,
        item_id_to_item,
        lifetimes,
        namespace_id_to_number_of_reopened_namespaces,
        reopened_namespace_id_to_idx,
        function_name_to_functions,
    }
}

fn validate_array_value(value: &ArrayValue) -> Result<()> {
    let length = value.length();
    ensure!(length > 0, "ArrayValue has non-positive length {length}");
    match value.kind() {
        ArrayValueKind::Integers(elements) => {
            validate_num_elements(elements.elements().len(), elements.has_filler(), length)?;
        }
        ArrayValueKind::Arrays(elements) => {
            validate_num_elements(elements.elements().len(), elements.has_filler(), length)?;
            for elem in elements.elements() {
                validate_array_value(elem)?;
            }
            if let Some(filler) = elements.filler() {
                validate_array_value(filler)?;
            }
            let mut first = None;
            for element in elements.elements().iter().chain(elements.filler()) {
                let first = *first.get_or_insert(element);
                ensure!(
                    array_dimensions(element).eq(array_dimensions(first)),
                    "ArrayValue elements have different shapes: [{}] and [{}]",
                    array_dimensions(first).format(", "),
                    array_dimensions(element).format(", "),
                );
            }
        }
        ArrayValueKind::StringLiteral(bytes) => {
            let len = bytes.len() as u64;
            ensure!(
                len < length,
                "string literal of length {len} (plus NUL terminator) does not fit in an \
                 array of length {length}"
            );
        }
    }
    Ok(())
}

fn validate_cc_type(ty: &CcType) -> Result<()> {
    match ty.variant() {
        CcTypeVariant::Pointer(p) => validate_cc_type(p.pointee_type())?,
        CcTypeVariant::FuncPointer(fp) => {
            for param_or_ret in fp.param_and_return_types() {
                validate_cc_type(param_or_ret)?;
            }
        }
        CcTypeVariant::ConstantArray(arr) => {
            ensure!(
                arr.num_elements() > 0,
                "ConstantArrayType has non-positive num_elements: {}",
                arr.num_elements()
            );
            validate_cc_type(arr.element_type())?;
        }
        CcTypeVariant::Primitive(_) | CcTypeVariant::Decl(_) | CcTypeVariant::Error(_) => {}
    }
    for arg in ty.template_args() {
        validate_cc_type(arg)?;
    }
    Ok(())
}

fn validate_item(item: &Item) -> Result<()> {
    match item {
        Item::Func(func) => {
            validate_cc_type(func.return_type())?;
            for param in func.params() {
                validate_cc_type(param.type_())?;
            }
        }
        Item::Constant(constant) => {
            if let Some(ty) = constant.type_() {
                validate_cc_type(ty)?;
            }
            match constant.kind() {
                ConstantValue::Integer(_) => {}
                ConstantValue::Array(array_value) => validate_array_value(array_value)?,
                ConstantValue::StringView(_) => {
                    ensure!(
                        !constant.has_type(),
                        "StringViewValue Constant should not have a CcType"
                    );
                }
            }
        }
        Item::GlobalVar(gv) => {
            validate_cc_type(gv.type_())?;
        }
        Item::TypeAlias(ta) => {
            validate_cc_type(ta.underlying_type())?;
        }
        Item::Record(record) => {
            for field in record.fields() {
                validate_cc_type(field.type_())?;
            }
            for child in record.children() {
                validate_item(child)?;
            }
        }
        Item::Namespace(ns) => {
            for child in ns.children() {
                validate_item(child)?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn proto_to_ir(proto: &ir_rust_proto::IRProto) -> Result<IR> {
    let public_headers = proto.public_headers().to_vec();

    let crate_root_path = proto.crate_root_path().map(FfiString::from);

    let crubit_features = proto
        .crubit_features()
        .iter()
        .map(|(target, feature_set)| {
            let mut features = flagset::FlagSet::<CrubitFeature>::default();
            for feature_name in feature_set.iter() {
                let feature_flags =
                    crubit_feature::named_features(feature_name.as_str().as_bytes()).with_context(
                        || format!("Invalid Crubit feature name: {:?}", feature_name.as_str()),
                    )?;
                features |= feature_flags;
            }
            Ok::<_, Error>((
                BazelLabel::from(target.clone()),
                crubit_feature::SerializedCrubitFeatures::resolved(features),
            ))
        })
        .try_collect()?;

    let unstable_rust_features = proto.unstable_rust_features().iter().map(String::from).collect();

    let reexported_namespaces = proto.reexported_namespaces().to_vec();

    let crate_names = proto
        .crate_names()
        .iter()
        .map(|(target, name_str)| {
            let ident = try_make_rs_ident(name_str.as_str()).with_context(|| {
                format!("Invalid crate name identifier: {:?}", name_str.as_str())
            })?;
            Ok::<_, Error>((BazelLabel::from(target.clone()), ident))
        })
        .try_collect()?;

    let top_level_items = proto
        .top_level_items()
        .iter()
        .map(|(target, item_list)| {
            for item in item_list.iter() {
                validate_item(item)?;
            }
            Ok::<_, Error>((BazelLabel::from(target.clone()), item_list.to_vec()))
        })
        .try_collect()?;

    let tree_ir = TreeIR {
        public_headers,
        current_target: BazelLabel::from(proto.current_target.clone()),
        crate_root_path,
        crubit_features,
        crate_names,
        unstable_rust_features,
        reexported_namespaces,
        top_level_items,
    };
    Ok(make_ir(tree_ir))
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};l={}", self.path(), self.line())
    }
}

impl Display for DeclLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /// Writes one line, e.g. `Generated from: path/to/header.h;l=12`.
        fn write_line(
            f: &mut fmt::Formatter<'_>,
            label: &str,
            location: Option<&SourceLocation>,
            name_range: Option<(u32, u32)>,
        ) -> fmt::Result {
            let Some(location) = location else {
                return f.write_str("<unknown location>");
            };
            let _ = name_range;
            write!(f, "{label}: {location}")
        }

        write_line(f, "Generated from", self.spelling(), self.name_range())?;
        if let Some(expansion) = self.expansion() {
            f.write_str("\n")?;
            // The name isn't at the expansion location, so its range is written as `[0,0]` when
            // generating Kythe annotations.
            let expansion_name_range = self.name_range().map(|_| (0, 0));
            write_line(f, "Expanded at", Some(expansion), expansion_name_range)?;
        }
        Ok(())
    }
}

/// Displays an `Option<&DeclLocation>`, falling back to `<unknown location>` when `None`.
#[derive(Clone, Copy)]
pub struct DeclLocationOrUnknown<'a>(pub Option<&'a DeclLocation>);

impl Display for DeclLocationOrUnknown<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(decl_loc) => write!(f, "{decl_loc}"),
            None => f.write_str("<unknown location>"),
        }
    }
}

impl HeaderName {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(name: &str) -> Self {
        Self { name: FfiString::from(name) }
    }
}

impl CcType {
    pub fn new(
        variant: CcTypeVariant,
        is_const: bool,
        unknown_attr: impl Into<FfiString>,
        explicit_lifetimes: impl Into<FfiVec<FfiString>>,
    ) -> Self {
        Self {
            variant,
            is_const,
            is_nonnull: false,
            unknown_attr: unknown_attr.into(),
            explicit_lifetimes: explicit_lifetimes.into(),
            template_args: FfiVec::default(),
        }
    }

    pub fn variant_mut(&mut self) -> &mut CcTypeVariant {
        &mut self.variant
    }

    pub fn explicit_lifetimes_mut(&mut self) -> &mut FfiVec<FfiString> {
        &mut self.explicit_lifetimes
    }

    pub fn decl_template_args(&self) -> Option<&[CcType]> {
        (!self.template_args.is_empty()).then_some(self.template_args.as_slice())
    }

    pub fn template_args_mut(&mut self) -> &mut FfiVec<CcType> {
        &mut self.template_args
    }

    pub fn is_unit_type(&self) -> bool {
        matches!(&self.variant, CcTypeVariant::Primitive(Primitive::Void))
    }
}

impl From<&Record> for CcType {
    fn from(record: &Record) -> Self {
        CcType {
            variant: CcTypeVariant::Decl(record.id()),
            is_const: false,
            is_nonnull: false,
            unknown_attr: FfiString::default(),
            explicit_lifetimes: FfiVec::default(),
            template_args: FfiVec::default(),
        }
    }
}

impl From<&TypeAlias> for CcType {
    fn from(alias: &TypeAlias) -> Self {
        CcType {
            variant: CcTypeVariant::Decl(alias.id()),
            is_const: false,
            is_nonnull: false,
            unknown_attr: FfiString::default(),
            explicit_lifetimes: FfiVec::default(),
            template_args: FfiVec::default(),
        }
    }
}

impl From<&ExistingRustType> for CcType {
    fn from(existing_rust_type: &ExistingRustType) -> Self {
        CcType {
            variant: CcTypeVariant::Decl(existing_rust_type.id()),
            is_const: false,
            is_nonnull: false,
            unknown_attr: FfiString::default(),
            explicit_lifetimes: FfiVec::default(),
            template_args: FfiVec::default(),
        }
    }
}

impl PointerType {
    pub fn new(
        kind: PointerTypeKind,
        lifetime: Option<LifetimeId>,
        pointee_type: impl Into<FfiRc<CcType>>,
    ) -> Self {
        let mut pt = Self {
            has_bits: 0,
            kind,
            lifetime: LifetimeId::default(),
            pointee_type: pointee_type.into(),
        };
        if let Some(lifetime) = lifetime {
            pt.set_lifetime(lifetime);
        }
        pt
    }
}

impl ConstantArrayType {
    pub fn new(num_elements: usize, element_type: impl Into<FfiRc<CcType>>) -> Self {
        Self { num_elements, element_type: element_type.into() }
    }
}

macro_rules! impl_typed_tokens_for_enum {
    ($Type:ident { $($Variant:ident = { $($cpp_spelling:tt)+ },)+ }) => {
        impl $Type {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$Variant => stringify!($($cpp_spelling)+),
                    )+
                }
            }
        }

        impl std::fmt::Display for $Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl quote::ToTokens for $Type {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                match self {
                    $(
                        Self::$Variant => quote! { $($cpp_spelling)+ }.to_tokens(tokens),
                    )+
                }
            }
        }
    };
}

impl_typed_tokens_for_enum! {
    Primitive {
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
        Int128 = {__int128},
        UnsignedShort = {unsigned short},
        UnsignedInt = {unsigned int},
        UnsignedLong = {unsigned long},
        UnsignedLongLong = {unsigned long long},
        UnsignedInt128 = {unsigned __int128},

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

impl_typed_tokens_for_enum! {
    CcCallingConv {
        C = {cdecl},
        X86FastCall = {fastcall},
        X86VectorCall = {vectorcall},
        X86ThisCall = {thiscall},
        X86StdCall = {stdcall},
        Win64 = {ms_abi},
    }
}

impl CcCallingConv {
    /// Converts clang::CallingConv enum [1] into an equivalent Rust Abi [2, 3, 4].
    /// [1]
    /// https://github.com/llvm/llvm-project/blob/c6a3225bb03b6afc2b63fbf13db3c100406b32ce/clang/include/clang/Basic/Specifiers.h#L262-L283
    /// [2] https://doc.rust-lang.org/reference/types/function-pointer.html
    /// [3]
    /// https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier
    /// [4]
    /// https://github.com/rust-lang/rust/blob/b27ccbc7e1e6a04d749e244a3c13f72ca38e80e7/compiler/rustc_target/src/spec/abi.rs#L49
    pub fn rs_extern_abi(self) -> &'static str {
        match self {
            CcCallingConv::C => {
                // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
                // that:
                // - `extern "C"` [...] whatever the default your C compiler supports.
                // - `extern "cdecl"` -- The default for x86_32 C code.
                //
                // We don't support C++ exceptions and therefore we use "C" (rather than
                // "C-unwind") - we have no need for unwinding across the FFI boundary -
                // e.g. from C++ into Rust frames (or vice versa).
                "C"
            }
            CcCallingConv::X86FastCall => {
                // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
                // that the fastcall ABI -- corresponds to MSVC's __fastcall and GCC and
                // clang's __attribute__((fastcall)).
                "fastcall"
            }
            CcCallingConv::X86VectorCall => {
                // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
                // that the vectorcall ABI -- corresponds to MSVC's __vectorcall and
                // clang's __attribute__((vectorcall)).
                "vectorcall"
            }
            CcCallingConv::X86ThisCall => {
                // We don't support C++ exceptions and therefore we use "thiscall" (rather
                // than "thiscall-unwind") - we have no need for unwinding across the FFI
                // boundary - e.g. from C++ into Rust frames (or vice versa).
                "thiscall"
            }
            CcCallingConv::X86StdCall => {
                // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
                // extern "stdcall" -- The default for the Win32 API on x86_32.
                //
                // We don't support C++ exceptions and therefore we use "stdcall" (rather
                // than "stdcall-unwind") - we have no need for unwinding across the FFI
                // boundary - e.g. from C++ into Rust frames (or vice versa).
                "stdcall"
            }
            CcCallingConv::Win64 => {
                // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
                // extern "win64" -- The default for C code on x86_64 Windows.
                "win64"
            }
        }
    }
}

impl CcTypeVariant {
    pub fn as_pointer(&self) -> Option<&PointerType> {
        match &self {
            CcTypeVariant::Pointer(pointer) => Some(pointer),
            _ => None,
        }
    }

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
            CcTypeVariant::Decl(id) => Some(*id),
            _ => None,
        }
    }
}

impl Identifier {
    pub fn new(identifier: impl Into<FfiString>) -> Self {
        Self { identifier: identifier.into() }
    }

    pub fn as_str(&self) -> &str {
        self.identifier.as_str()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Identifier {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<str> for &Identifier {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

/// Returns the length of each dimension of `value`, outermost first.
///
/// `value` must already have been validated, so that all elements of each outer dimension have
/// the same shape, and only the first element (or the filler) needs to be followed.
fn array_dimensions(value: &ArrayValue) -> impl Iterator<Item = u64> + '_ {
    std::iter::successors(Some(value), |value| match value.kind() {
        ArrayValueKind::Arrays(elements) => elements.elements().first().or(elements.filler()),
        _ => None,
    })
    .map(|value| value.length())
}

/// Validates that `num_elements` explicitly initialized elements fit in an array of length `size`,
/// and that there is a filler if and only if they don't fill it.
fn validate_num_elements(num_elements: usize, has_filler: bool, size: u64) -> Result<()> {
    let num_elements = num_elements as u64;
    ensure!(
        num_elements <= size,
        "{num_elements} elements do not fit in an array of length {size}"
    );
    ensure!(
        has_filler == (num_elements < size),
        "an array of length {size} with {num_elements} elements must {}have a filler",
        if has_filler { "not " } else { "" }
    );
    Ok(())
}

impl Operator {
    pub fn new(name: impl Into<FfiString>) -> Self {
        Self { name: name.into() }
    }

    pub fn cc_name(&self) -> String {
        let separator = match self.name.chars().next() {
            Some(c) if c.is_alphabetic() => " ",
            _ => "",
        };
        format!("operator{separator}{name}", separator = separator, name = self.name)
    }
}

impl UnqualifiedIdentifier {
    pub fn is_constructor(&self) -> bool {
        matches!(self, UnqualifiedIdentifier::Constructor)
    }
    pub fn is_destructor(&self) -> bool {
        matches!(self, UnqualifiedIdentifier::Destructor)
    }
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            UnqualifiedIdentifier::Identifier(identifier) => Some(identifier),
            _ => None,
        }
    }
    pub fn as_operator(&self) -> Option<&Operator> {
        match self {
            UnqualifiedIdentifier::Operator(op) => Some(op),
            _ => None,
        }
    }
    pub fn identifier_as_str(&self) -> Option<&str> {
        self.as_identifier().map(|id| id.as_str())
    }
}

impl PartialEq<str> for UnqualifiedIdentifier {
    fn eq(&self, other: &str) -> bool {
        if let UnqualifiedIdentifier::Identifier(identifier) = self {
            identifier.as_str() == other
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

impl PartialEq<str> for &UnqualifiedIdentifier {
    fn eq(&self, other: &str) -> bool {
        (*self).eq(other)
    }
}

impl InstanceMethodMetadata {
    pub fn new(reference: ReferenceQualification, is_const: bool, is_virtual: bool) -> Self {
        Self { reference, is_const, is_virtual }
    }
}

impl FuncParam {
    pub fn type_mut(&mut self) -> &mut CcType {
        &mut self.type_
    }
}

impl Func {
    pub fn has_c_calling_convention(&self) -> bool {
        self.call_conv() == Some(CcCallingConv::C)
    }

    pub fn is_instance_method(&self) -> bool {
        self.instance_method_metadata().is_some()
    }

    pub fn source_text_as_token_stream(&self) -> Option<proc_macro2::TokenStream> {
        self.inline_cpp_source_text()?.parse::<proc_macro2::TokenStream>().ok()
    }

    pub fn return_type_mut(&mut self) -> &mut CcType {
        &mut self.return_type
    }

    pub fn params_mut(&mut self) -> &mut FfiVec<FuncParam> {
        &mut self.params
    }

    pub fn lifetime_inputs_mut(&mut self) -> &mut FfiVec<FfiString> {
        &mut self.lifetime_inputs
    }
}

impl GenericItem for Func {
    fn id(&self) -> ItemId {
        self.id()
    }
    fn unique_name(&self) -> Option<&str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        if self.cc_name == UnqualifiedIdentifier::Constructor {
            UnsupportedItemKind::Constructor
        } else {
            UnsupportedItemKind::Func
        }
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        Self::decl_loc(self)
    }
    fn unknown_attr(&self) -> Option<&str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        self.must_bind()
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        self.cc_name.identifier_as_str()
    }
}

impl Field {
    pub fn type_mut(&mut self) -> &mut CcType {
        &mut self.type_
    }
}

impl GenericItem for IncompleteRecord {
    fn id(&self) -> ItemId {
        IncompleteRecord::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        self.record_type.unsupported_item_kind()
    }
    fn unknown_attr(&self) -> Option<&str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        IncompleteRecord::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        Some(self.cc_name().as_str())
    }
}

impl RecordType {
    fn unsupported_item_kind(&self) -> UnsupportedItemKind {
        match self {
            RecordType::Struct => UnsupportedItemKind::Struct,
            RecordType::Union => UnsupportedItemKind::Union,
            RecordType::Class => UnsupportedItemKind::Class,
        }
    }
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

impl TemplateSpecialization {
    pub fn new_for_testing(defining_target: BazelLabel, kind: TemplateSpecializationKind) -> Self {
        Self { defining_target, kind }
    }
}

impl Record {
    pub fn is_packed(&self) -> bool {
        self.pack().is_some()
    }

    pub fn fields_mut(&mut self) -> &mut FfiVec<Field> {
        &mut self.fields
    }

    pub fn lifetime_inputs_mut(&mut self) -> &mut FfiVec<FfiString> {
        &mut self.lifetime_inputs
    }

    /// Whether this type has Rust-like object semantics for mutating
    /// assignment, and can be passed by mut reference as a result.
    ///
    /// If a type `T` is mut reference safe, it can be possed as a `&mut T`
    /// safely. Otherwise, mutable references must use `Pin<&mut T>`.
    ///
    /// Such types are Rust-movable: Rust can move them with a `memcpy`,
    /// without running a C++ move constructor and without running the C++
    /// destructor at the old location. This is what lets Rust pass them by
    /// value and mutate their memory directly with memcpy-like
    /// assignment/swap.
    ///
    /// See the `is_rust_movable` field in `ir.proto`, and
    /// docs/design/unpin.md for more details.
    pub fn is_unpin(&self) -> bool {
        self.is_rust_movable()
    }

    // TODO(b/498977848): The record with cc_name
    // "std::basic_string_view<char8_t, std::char_traits<char8_t>>" with
    // rs_name "__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE" is given
    // TemplateSpecialization kind NonSpecial. This is unfortunate, since we want to exclude all
    // flavors of string_view because of our special-casing.
    pub fn is_string_view(&self) -> bool {
        match self.template_specialization() {
            Some(TemplateSpecialization { defining_target, kind })
                if defining_target.is_cc_std() =>
            {
                let is_string_view = *kind == TemplateSpecializationKind::StdStringView
                    || *kind == TemplateSpecializationKind::StdWStringView;
                if is_string_view {
                    return true;
                };
                self.cc_name().as_str().starts_with("std::basic_string_view<")
            }
            _ => false,
        }
    }

    /// Detect whether a `Record` is *specifically* the raw projection of `std::string_view`, as we
    /// special-case this.
    pub fn is_raw_string_view(&self) -> bool {
        matches!(
            self.template_specialization(),
            Some(TemplateSpecialization { kind: TemplateSpecializationKind::StdStringView, .. })
        ) && self.rs_name().as_str() == "raw_string_view"
    }

    pub fn is_union(&self) -> bool {
        match self.record_type() {
            RecordType::Union => true,
            RecordType::Struct | RecordType::Class => false,
        }
    }

    /// Returns a `TokenStream` containing the C++ tag kind for this record.
    ///
    /// This is the `struct`, `union`, or `class` keyword, or nothing if this is a canonical alias
    /// to a record type. (For example, typedefs to anonymous records, or template specializations
    /// with a `preferred_name`.)
    pub fn cc_tag_kind(&self) -> TokenStream {
        if self.is_canonical_alias() {
            quote! {}
        } else {
            self.record_type().into_token_stream()
        }
    }

    pub fn should_implement_drop(&self) -> bool {
        match self.destructor() {
            SpecialMemberFunc::Trivial => false,

            // TODO(jeanpierreda): b/212690698 - Avoid calling into the C++ destructor
            // (e.g. let Rust drive `drop`-ing) to avoid (somewhat unergonomic)
            // ManuallyDrop if we can ask Rust to preserve C++ field destruction
            // order in NontrivialMembers case.
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

    pub fn should_derive_copy(&self) -> bool {
        // Thread-safe types wrap their fields in UnsafeCell<[MaybeUninit<u8>; N]>,
        // which prevents them from deriving Copy.
        if self.is_thread_safe() {
            return false;
        }
        match self.trait_derives.copy {
            TraitImplPolarity::Positive => true,
            TraitImplPolarity::Negative => false,
            TraitImplPolarity::None => {
                self.is_unpin()
                    && self.copy_constructor() == SpecialMemberFunc::Trivial
                    && self.destructor() == SpecialMemberFunc::Trivial
                    && self.check_by_value().is_ok()
                    && self.trait_derives.clone != TraitImplPolarity::Negative
                    // Mutable fields become `Cell<T>` in Rust, which prevents
                    // the struct from deriving `Copy`.
                    && self.fields.iter().all(|f| !f.is_mutable())
            }
        }
    }

    /// Returns Ok if the type can exist by value.
    ///
    /// This does not necessarily imply that the type is Rust-movable, e.g. trivially relocatable.
    pub fn check_by_value(&self) -> Result<()> {
        ensure!(
            self.destructor() != SpecialMemberFunc::Unavailable,
            "`{}` can't be used by-value because it has a non-public or deleted destructor",
            self.cc_name()
        );
        ensure!(
            !self.is_abstract(),
            "`{}` can be used by-value because it has pure virtual functions that are not overridden",
            self.cc_name()
        );
        Ok(())
    }

    /// Whether this record has a unique owning target.
    ///
    /// Notably, all records that have a unique owning target are supported, e.g. `std::string`, but
    /// not all supported records have a unique owning target, e.g. `std::vector<int>`.
    pub fn has_unique_owning_target(&self) -> bool {
        self.template_specialization().is_none() || self.is_canonical_alias()
    }
}

impl GenericItem for Record {
    fn id(&self) -> ItemId {
        Record::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        self.record_type().unsupported_item_kind()
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        Self::decl_loc(self)
    }
    fn unknown_attr(&self) -> Option<&str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        Record::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        Some(self.cc_name().as_str())
    }
}

impl Constant {
    pub fn value(&self) -> &ConstantValue {
        self.kind()
    }
}

impl GenericItem for Constant {
    fn id(&self) -> ItemId {
        Constant::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::GlobalVar
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        Self::decl_loc(self)
    }
    fn unknown_attr(&self) -> Option<&str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        Constant::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        Some(self.cc_name().as_str())
    }
}

impl GenericItem for GlobalVar {
    fn id(&self) -> ItemId {
        GlobalVar::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::GlobalVar
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        Self::decl_loc(self)
    }
    fn unknown_attr(&self) -> Option<&str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        GlobalVar::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        Some(self.cc_name().as_str())
    }
}

impl GenericItem for Enum {
    fn id(&self) -> ItemId {
        Enum::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Enum
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        Self::decl_loc(self)
    }
    fn unknown_attr(&self) -> Option<&str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        Enum::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        Some(self.cc_name().as_str())
    }
}

impl TypeAlias {
    pub fn underlying_type_mut(&mut self) -> &mut CcType {
        &mut self.underlying_type
    }

    pub fn lifetime_inputs_mut(&mut self) -> &mut FfiVec<FfiString> {
        &mut self.lifetime_inputs
    }
}

impl GenericItem for TypeAlias {
    fn id(&self) -> ItemId {
        TypeAlias::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::TypeAlias
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        Self::decl_loc(self)
    }
    fn unknown_attr(&self) -> Option<&str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        TypeAlias::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        Some(self.cc_name().as_str())
    }
}

impl Display for TypeAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {})",
            self.rs_name(),
            self.owning_target(),
            DeclLocationOrUnknown(self.decl_loc())
        )
    }
}

impl From<std::rc::Rc<FormattedError>> for FormattedError {
    fn from(rc: std::rc::Rc<FormattedError>) -> Self {
        (*rc).clone()
    }
}

impl UnsupportedItemKind {
    fn str(&self) -> &'static str {
        match self {
            UnsupportedItemKind::Func => "function",
            UnsupportedItemKind::GlobalVar => "global variable",
            UnsupportedItemKind::Struct => "struct",
            UnsupportedItemKind::Union => "union",
            UnsupportedItemKind::Class => "class",
            UnsupportedItemKind::Enum => "enum",
            UnsupportedItemKind::TypeAlias => "type alias",
            UnsupportedItemKind::Namespace => "namespace",
            UnsupportedItemKind::Constructor => "constructor",
            UnsupportedItemKind::Other => "item",
        }
    }
}

impl Display for UnsupportedItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl UnsupportedItem {
    pub fn source_text_as_token_stream(&self) -> Option<proc_macro2::TokenStream> {
        self.inline_cpp_source_text()?.parse::<proc_macro2::TokenStream>().ok()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_raw(
        name: impl Into<FfiString>,
        unique_name: Option<impl Into<FfiString>>,
        kind: UnsupportedItemKind,
        id: ItemId,
        decl_loc: Option<DeclLocation>,
        defining_target: Option<BazelLabel>,
        must_bind: bool,
        path: Option<UnsupportedItemPath>,
        error: Option<impl Into<FormattedError>>,
    ) -> Self {
        let mut item = Self {
            has_bits: 0,
            name: name.into(),
            unique_name: FfiString::default(),
            kind,
            path: UnsupportedItemPath::default(),
            errors: error.into_iter().map(Into::into).collect(),
            decl_loc: DeclLocation::default(),
            inline_cpp_source_text: FfiString::default(),
            id,
            must_bind,
            defining_target: BazelLabel::default(),
        };
        if let Some(unique_name) = unique_name {
            item.set_unique_name(unique_name.into());
        }
        if let Some(path) = path {
            item.set_path(path);
        }
        if let Some(decl_loc) = decl_loc {
            item.set_decl_loc(decl_loc);
        }
        if let Some(defining_target) = defining_target {
            item.set_defining_target(defining_target);
        }
        item
    }
}

impl GenericItem for UnsupportedItem {
    fn id(&self) -> ItemId {
        UnsupportedItem::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        self.unique_name()
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        None
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        self.kind
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        Self::decl_loc(self)
    }
    fn unknown_attr(&self) -> Option<&str> {
        None
    }
    fn must_bind(&self) -> bool {
        UnsupportedItem::must_bind(self)
    }
}

impl GenericItem for Comment {
    fn id(&self) -> ItemId {
        Comment::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        None
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        None
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Other
    }
    fn unknown_attr(&self) -> Option<&str> {
        None
    }
    fn must_bind(&self) -> bool {
        Comment::must_bind(self)
    }
}

impl Namespace {
    pub fn children_mut(&mut self) -> &mut FfiVec<Item> {
        &mut self.children
    }
}

impl GenericItem for Namespace {
    fn id(&self) -> ItemId {
        Namespace::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target().clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Namespace
    }
    fn unknown_attr(&self) -> Option<&str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        Namespace::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&str> {
        Some(self.cc_name().as_str())
    }
}

impl GenericItem for UseMod {
    fn id(&self) -> ItemId {
        UseMod::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        None
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        None
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Other
    }
    fn unknown_attr(&self) -> Option<&str> {
        None
    }
    fn must_bind(&self) -> bool {
        UseMod::must_bind(self)
    }
}

impl GenericItem for ExistingRustType {
    fn id(&self) -> ItemId {
        ExistingRustType::id(self)
    }
    fn unique_name(&self) -> Option<&str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Other
    }
    fn unknown_attr(&self) -> Option<&str> {
        None
    }
    fn must_bind(&self) -> bool {
        ExistingRustType::must_bind(self)
    }
}

macro_rules! forward_item {
    (match $item:ident { _($item_name:ident) => $expr:expr $(,)? }) => {
        match $item {
            Item::Func($item_name) => $expr,
            Item::IncompleteRecord($item_name) => $expr,
            Item::Record($item_name) => $expr,
            Item::Enum($item_name) => $expr,
            Item::Constant($item_name) => $expr,
            Item::GlobalVar($item_name) => $expr,
            Item::TypeAlias($item_name) => $expr,
            Item::UnsupportedItem($item_name) => $expr,
            Item::Comment($item_name) => $expr,
            Item::Namespace($item_name) => $expr,
            Item::UseMod($item_name) => $expr,
            Item::ExistingRustType($item_name) => $expr,
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
    fn unique_name(&self) -> Option<&str> {
        forward_item! {
            match self {
                _(x) => x.unique_name()
            }
        }
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        forward_item! {
            match self {
                _(x) => x.owning_target()
            }
        }
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        forward_item! {
            match self {
                _(x) => x.unsupported_kind()
            }
        }
    }
    fn decl_loc(&self) -> Option<&DeclLocation> {
        forward_item! {
            match self {
                _(x) => x.decl_loc()
            }
        }
    }
    fn unknown_attr(&self) -> Option<&str> {
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

    /// Forwards C++ identifier extraction to the underlying AST item variant for compatibilty with
    /// golden tests/error messages. Returns None for non-named items like comments and imports.
    fn cc_name_as_str(&self) -> Option<&str> {
        forward_item! {
            match self {
                _(x) => x.cc_name_as_str()
            }
        }
    }
}

impl Item {
    /// Returns true if this is an aggregate type (struct, union, class, or
    /// enum).
    pub fn is_aggregate_type(&self) -> bool {
        matches!(
            self,
            Item::IncompleteRecord(_)
                | Item::Record(_)
                | Item::Enum(_)
                | Item::ExistingRustType(_)
                | Item::TypeAlias(_)
        )
    }

    /// Returns true if this item should generate an `f::UnsupportedItem` record
    /// instead of a regular Rust item.
    pub fn is_unsupported(&self) -> bool {
        matches!(self, Item::UnsupportedItem(_))
    }

    pub fn enclosing_item_id(&self) -> Option<ItemId> {
        match self {
            Item::Record(record) => record.enclosing_item_id(),
            Item::IncompleteRecord(record) => record.enclosing_item_id(),
            Item::Enum(enum_) => enum_.enclosing_item_id(),
            Item::Constant(constant) => constant.enclosing_item_id(),
            Item::GlobalVar(type_) => type_.enclosing_item_id(),
            Item::Func(func) => func.enclosing_item_id(),
            Item::Namespace(namespace) => namespace.enclosing_item_id(),
            Item::TypeAlias(type_alias) => type_alias.enclosing_item_id(),
            Item::Comment(..) => None,
            Item::UnsupportedItem(unsupported) => {
                unsupported.path().and_then(|p| p.enclosing_item_id())
            }
            Item::UseMod(..) => None,
            Item::ExistingRustType(..) => None,
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
            Item::Constant(_) => false,
            Item::GlobalVar(_) => false,
            Item::TypeAlias(_) => true,
            Item::UnsupportedItem(_) => false,
            Item::Comment(_) => false,
            Item::Namespace(_) => false,
            Item::UseMod(_) => false,
            Item::ExistingRustType(_) => false,
        }
    }

    /// Returns the C++ identifier for this item, if it has one.
    pub fn cc_name_as_str(&self) -> Option<&str> {
        match self {
            Item::Func(func) => match &func.cc_name {
                UnqualifiedIdentifier::Identifier(identifier) => Some(identifier.as_str()),
                _ => None,
            },
            Item::IncompleteRecord(incomplete_record) => Some(incomplete_record.cc_name().as_str()),
            Item::Record(record) => Some(record.cc_name().as_str()),
            Item::Enum(enum_) => Some(enum_.cc_name().as_str()),
            Item::Constant(constant) => Some(constant.cc_name().as_str()),
            Item::GlobalVar(global_var) => Some(global_var.cc_name().as_str()),
            Item::TypeAlias(type_alias) => Some(type_alias.cc_name().as_str()),
            Item::Namespace(namespace) => Some(namespace.cc_name().as_str()),
            Item::UnsupportedItem(_) => None,
            Item::Comment(_) => None,
            Item::UseMod(_) => None,
            Item::ExistingRustType(existing_rust_type) => Some(existing_rust_type.cc_name()),
        }
    }

    /// Returns the Rust identifier for this item, if it has one and the
    /// identifier is unqualified.
    pub fn rs_name_as_str(&self) -> Option<&str> {
        match self {
            Item::Func(func) => func.rs_name.identifier_as_str(),
            Item::IncompleteRecord(incomplete_record) => Some(incomplete_record.rs_name().as_str()),
            Item::Record(record) => Some(record.rs_name().as_str()),
            Item::Enum(enum_) => Some(enum_.rs_name().as_str()),
            Item::Constant(constant) => Some(constant.rs_name().as_str()),
            Item::GlobalVar(global_var) => Some(global_var.rs_name().as_str()),
            Item::TypeAlias(type_alias) => Some(type_alias.rs_name().as_str()),
            Item::UnsupportedItem(_) => None,
            Item::Comment(_) => None,
            Item::Namespace(namespace) => Some(namespace.rs_name().as_str()),
            Item::UseMod(use_mod) => Some(use_mod.mod_name().as_str()),
            Item::ExistingRustType(existing_rust_type) => Some(existing_rust_type.rs_name()),
        }
    }

    /// Returns whether this item is a namespace whose namespace_id matches
    /// `canonical_namespace_id`.
    pub fn is_canonical_namespace(&self, canonical_namespace_id: ItemId) -> bool {
        matches!(self, Item::Namespace(ns) if ns.id() == canonical_namespace_id)
    }

    /// If this item is a child item of a Record, returns true if it should be
    /// placed in a nested module.
    ///
    /// Functions become methods and constants become associated consts of the
    /// record, so they do not need the module.
    pub fn place_in_nested_module_if_nested_in_record(&self) -> bool {
        match self {
            Item::IncompleteRecord(_)
            | Item::Record(_)
            | Item::GlobalVar(_)
            | Item::TypeAlias(_)
            | Item::Enum(_)
            | Item::UseMod(_)
            | Item::ExistingRustType(_) => true,
            Item::Func(_) | Item::Constant(_) | Item::UnsupportedItem(_) | Item::Comment(_) => {
                false
            }
            Item::Namespace(_) => unreachable!("Found a namespace that's opened inside of a record. This is not valid C++, so this is a bug."),
        }
    }
}

macro_rules! impl_item_conversions {
    ($($variant:ident($ty:ty)),* $(,)?) => {
        $(
            impl From<$ty> for Item {
                fn from(val: $ty) -> Item {
                    Item::$variant(Rc::new(val))
                }
            }

            impl<'a> TryFrom<&'a Item> for &'a Rc<$ty> {
                type Error = Error;
                fn try_from(value: &'a Item) -> Result<Self, Self::Error> {
                    if let Item::$variant(x) = value {
                        Ok(x)
                    } else {
                        bail!("Not a {}: {:#?}", stringify!($ty), value)
                    }
                }
            }
        )*
    };
}

impl_item_conversions! {
    Func(Func),
    IncompleteRecord(IncompleteRecord),
    Record(Record),
    Enum(Enum),
    Constant(Constant),
    GlobalVar(GlobalVar),
    TypeAlias(TypeAlias),
    UnsupportedItem(UnsupportedItem),
    Comment(Comment),
    Namespace(Namespace),
    UseMod(UseMod),
    ExistingRustType(ExistingRustType),
}

#[derive(PartialEq, Eq, Clone)]
pub struct TreeIR {
    pub public_headers: Vec<HeaderName>,
    pub current_target: BazelLabel,
    pub crate_root_path: Option<FfiString>,
    pub crubit_features: BTreeMap<BazelLabel, crubit_feature::SerializedCrubitFeatures>,
    pub crate_names: BTreeMap<BazelLabel, Ident>,
    pub unstable_rust_features: Vec<String>,
    pub reexported_namespaces: Vec<FfiString>,
    pub top_level_items: BTreeMap<BazelLabel, Vec<Item>>,
}

/// A custom debug impl that wraps the HashMap in rustfmt-friendly notation.
///
/// See b/272530008.
impl Debug for TreeIR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // BTreeMap has consistent ordering, unlike HashMap, so it's reasonable to rely on a
        // consistent Debug output.

        // Format as `[ (k, v) ]` instead of `map! { k: v }` because rustfmt fails on macros
        // with complicated contents (our nested items) which it cannot verify is valid Rust. This
        // ensures the output is parse-able for rustfmt on test failures. See b/272530008.
        fn debug_btree_map<K: Debug, V: Debug>(map: &BTreeMap<K, V>) -> impl Debug + '_ {
            fmt::from_fn(move |f| f.debug_list().entries(map).finish())
        }

        // exhaustive-match so we don't forget to add fields to Debug when we add to TreeIR.
        let TreeIR {
            public_headers,
            current_target,
            crate_root_path,
            crubit_features,
            crate_names,
            unstable_rust_features,
            reexported_namespaces,
            top_level_items,
        } = self;
        f.debug_struct("TreeIR")
            .field("public_headers", public_headers)
            .field("current_target", current_target)
            .field("crate_root_path", crate_root_path)
            .field("crubit_features", &debug_btree_map(crubit_features))
            .field("crate_names", &debug_btree_map(crate_names))
            .field("unstable_rust_features", unstable_rust_features)
            .field("reexported_namespaces", reexported_namespaces)
            .field("top_level_items", &debug_btree_map(top_level_items))
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IR {
    tree_ir: TreeIR,
    item_id_to_item: HashMap<ItemId, Item>,
    lifetimes: HashMap<LifetimeId, LifetimeName>,
    namespace_id_to_number_of_reopened_namespaces: HashMap<ItemId, usize>,
    reopened_namespace_id_to_idx: HashMap<ItemId, usize>,
    function_name_to_functions: HashMap<UnqualifiedIdentifier, Vec<Rc<Func>>>,
}

impl IR {
    pub fn tree_ir(&self) -> &TreeIR {
        &self.tree_ir
    }

    pub fn unstable_rust_features(&self) -> &[String] {
        &self.tree_ir.unstable_rust_features
    }

    pub fn get_decl(&self, id: ItemId) -> Option<&Item> {
        self.item_id_to_item.get(&id)
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> + '_ {
        let roots = self.tree_ir.top_level_items.values().flat_map(|v| v.iter());
        ItemsIterator::new(roots.collect())
    }

    pub fn lifetimes(&self) -> impl Iterator<Item = (&LifetimeId, &LifetimeName)> {
        self.lifetimes.iter()
    }

    /// Returns the top-level items of a target.
    pub fn top_level_items_in_target(&self, target: &BazelLabel) -> &[Item] {
        self.tree_ir.top_level_items.get(target).map(|v| v.as_slice()).unwrap_or_default()
    }

    pub fn top_level_items(&self) -> &[Item] {
        self.top_level_items_in_target(self.current_target())
    }

    pub fn reexported_namespaces(&self) -> &[FfiString] {
        &self.tree_ir.reexported_namespaces
    }

    pub fn public_headers(&self) -> impl Iterator<Item = &HeaderName> {
        self.tree_ir.public_headers.iter()
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

    pub fn enums(&self) -> impl Iterator<Item = &Rc<Enum>> {
        self.items().filter_map(|item| match item {
            Item::Enum(enum_item) => Some(enum_item),
            _ => None,
        })
    }

    pub fn constants(&self) -> impl Iterator<Item = &Rc<Constant>> {
        self.items().filter_map(|item| match item {
            Item::Constant(constant) => Some(constant),
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

    pub fn existing_rust_types(&self) -> impl Iterator<Item = &Rc<ExistingRustType>> {
        self.items().filter_map(|item| match item {
            Item::ExistingRustType(existing_rust_type) => Some(existing_rust_type),
            _ => None,
        })
    }

    /// Returns whether `target` is the current target.
    pub fn is_current_target(&self, target: &BazelLabel) -> bool {
        // TODO(hlopko): Make this be a pointer comparison, now it's comparing string
        // values.
        *target == *self.current_target()
    }

    /// Returns the custom crate name for the given `target` if it was explicitly specified.
    pub fn crate_name(&self, target: &BazelLabel) -> Option<Ident> {
        self.tree_ir.crate_names.get(target).cloned()
    }

    /// Returns the Crubit features enabled for the given `target`.
    #[must_use]
    pub fn target_crubit_features(&self, target: &BazelLabel) -> flagset::FlagSet<CrubitFeature> {
        self.tree_ir.crubit_features.get(target).cloned().unwrap_or_default().0
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
        &mut self.tree_ir.crubit_features.entry(target.clone()).or_default().0
    }

    pub fn current_target(&self) -> &BazelLabel {
        &self.tree_ir.current_target
    }

    // Returns the standard Debug print string for the `flat_ir`. The reason why we
    // don't use the debug print of `Self` is that `Self` contains HashMaps, and
    // their debug print produces content that is not valid Rust code.
    // `token_stream_matchers` (hacky) implementation parses the debug print and
    // chokes on HashMaps. Therefore this method.
    //
    // Used for `token_stream_matchers`, do not use for anything else.
    pub fn tree_ir_debug_print(&self) -> String {
        format!("{:?}", self.tree_ir)
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

    pub fn crate_root_path(&self) -> Option<&str> {
        self.tree_ir.crate_root_path.as_deref()
    }

    pub fn crate_root_path_tokens(&self) -> TokenStream {
        match self.crate_root_path().map(make_rs_ident) {
            None => quote! { crate },
            Some(crate_root_path) => quote! { crate :: #crate_root_path },
        }
    }

    pub fn get_functions_by_name(
        &self,
        function_name: &UnqualifiedIdentifier,
    ) -> impl Iterator<Item = &Rc<Func>> + '_ {
        self.function_name_to_functions.get(function_name).map_or([].iter(), |v| v.iter())
    }
}

// TODO(jeanpierreda): This should probably be a method on IR accepting a GenericItem,
// and returning the crate name, or similar.

/// Returns Some(crate_ident) if this is an imported crate.
pub fn rs_imported_crate_name(owning_target: &BazelLabel, ir: &IR) -> Option<Ident> {
    if ir.is_current_target(owning_target) {
        None
    } else {
        let owning_crate = if let Some(custom_name) = ir.crate_name(owning_target) {
            custom_name
        } else {
            make_rs_ident(&owning_target.target_name_escaped())
        };
        Some(owning_crate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::gtest;
    use ir_rust_proto::IRProto;

    #[gtest]
    fn test_identifier_debug_print() {
        let identifier = ir_rust_proto::proto!(ir_rust_proto::Identifier { identifier: "hello" });
        assert_eq!(format!("{identifier:?}"), "\"hello\"");
    }

    #[gtest]
    fn test_unqualified_identifier_debug_print() {
        let identifier = ir_rust_proto::proto!(ir_rust_proto::Identifier { identifier: "hello" });
        assert_eq!(format!("{:?}", UnqualifiedIdentifier::Identifier(identifier)), "\"hello\"");
        assert_eq!(format!("{:?}", UnqualifiedIdentifier::Constructor), "Constructor");
        assert_eq!(format!("{:?}", UnqualifiedIdentifier::Destructor), "Destructor");
    }

    #[gtest]
    fn test_used_headers() {
        let proto = ir_rust_proto::proto!(IRProto {
            public_headers: [__ { name: "foo/bar.h" }],
            current_target: "//foo:bar",
        });
        let ir = proto_to_ir(&proto).unwrap();
        let expected = TreeIR {
            public_headers: vec![HeaderName::from_str("foo/bar.h")],
            current_target: "//foo:bar".into(),
            crate_root_path: None,
            crubit_features: Default::default(),
            crate_names: BTreeMap::new(),
            unstable_rust_features: vec![],
            reexported_namespaces: vec![],
            top_level_items: BTreeMap::new(),
        };
        assert_eq!(ir.tree_ir, expected);
    }

    #[gtest]
    fn test_empty_crate_root_path() {
        let proto = ir_rust_proto::proto!(IRProto { current_target: "//foo:bar" });
        let ir = proto_to_ir(&proto).unwrap();
        assert_eq!(ir.crate_root_path(), None);
    }

    #[gtest]
    fn test_crate_root_path() {
        let proto = ir_rust_proto::proto!(IRProto {
            crate_root_path: "__cc_template_instantiations_rs_api",
            current_target: "//foo:bar",
        });
        let ir = proto_to_ir(&proto).unwrap();
        assert_eq!(ir.crate_root_path(), Some("__cc_template_instantiations_rs_api"));
    }

    #[gtest]
    fn test_bazel_label_target() {
        let label: BazelLabel = "//foo:bar".into();
        assert_eq!(label.target_name(), "bar");
    }

    #[gtest]
    fn test_bazel_label_is_cc_std() {
        // Internal HEAD layout (and also the target label used in Chromium's repository
        // layout).
        assert!(BazelLabel::from("//support/cc_std:cc_std").is_cc_std());
        // Internal toolchain stable release snapshot.
        assert!(BazelLabel::from(
            "//third_party/crosstool/rust/stable/crubit/support/cc_std:cc_std"
        )
        .is_cc_std());
        // Internal toolchain stable_nonprod release snapshot.
        assert!(BazelLabel::from(
            "//third_party/crosstool/rust/stable_nonprod/crubit/support/cc_std:cc_std"
        )
        .is_cc_std());
        // Open-source Crubit repository layout (GitHub: google/crubit, rewritten by Copybara).
        assert!(BazelLabel::from("//support/cc_std:cc_std").is_cc_std());
        // Open-source Crubit repository layout using Bazel's shorthand package syntax.
        assert!(BazelLabel::from("//support/cc_std").is_cc_std());

        // Negative cases:
        // A different target within the cc_std package.
        assert!(!BazelLabel::from("//support/cc_std:other").is_cc_std());
        // An unrelated target.
        assert!(!BazelLabel::from("//rs_bindings_from_cc:ir").is_cc_std());
    }

    #[gtest]
    fn test_bazel_label_target_dotless() {
        let label: BazelLabel = "//foo".into();
        assert_eq!(label.target_name(), "foo");
    }

    #[gtest]
    fn test_bazel_label_implicit_target_equals_explicit_target() {
        let implicit: BazelLabel = "//foo".into();
        let explicit: BazelLabel = "//foo:foo".into();
        assert_eq!(implicit, explicit);
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

    #[gtest]
    fn test_make_ir_happy_path() {
        let proto = ir_rust_proto::proto!(IRProto {
            current_target: "//foo:bar",
            top_level_items: [(
                "//foo:bar",
                [__ {
                    namespace_decl: __ {
                        cc_name: __ { identifier: "nsA" },
                        rs_name: __ { identifier: "nsA" },
                        unique_name: "nsA",
                        id: 1u64,
                        canonical_namespace_id: 1u64,
                        owning_target: "//foo:bar",
                        is_inline: false,
                        must_bind: false,
                        children: [__ {
                            namespace_decl: __ {
                                cc_name: __ { identifier: "nsB" },
                                rs_name: __ { identifier: "nsB" },
                                unique_name: "nsB",
                                id: 2u64,
                                canonical_namespace_id: 2u64,
                                owning_target: "//foo:bar",
                                is_inline: false,
                                must_bind: false,
                                children: [__ {
                                    comment: __ { text: "hello", id: 3u64, must_bind: false }
                                }]
                            }
                        }]
                    }
                }]
            )]
        });
        let ir = proto_to_ir(&proto).unwrap();

        let ids: Vec<_> = ir.items().map(|item| item.id().as_u64()).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[gtest]
    fn test_make_ir_redeclarations() {
        let proto = ir_rust_proto::proto!(IRProto {
            current_target: "//foo:bar",
            top_level_items: [(
                "//foo:bar",
                [
                    __ {
                        namespace_decl: __ {
                            cc_name: __ { identifier: "ns1" },
                            rs_name: __ { identifier: "ns1" },
                            unique_name: "ns1",
                            id: 100u64,
                            canonical_namespace_id: 100u64,
                            owning_target: "//foo:bar",
                            is_inline: false,
                            must_bind: false,
                            children: [__ {
                                comment: __ { text: "hello", id: 200u64, must_bind: false }
                            }]
                        }
                    },
                    __ {
                        namespace_decl: __ {
                            cc_name: __ { identifier: "ns1" },
                            rs_name: __ { identifier: "ns1" },
                            unique_name: "ns1",
                            id: 101u64,
                            canonical_namespace_id: 100u64,
                            owning_target: "//foo:bar",
                            is_inline: false,
                            must_bind: false,
                            children: [__ {
                                comment: __ { text: "hello", id: 200u64, must_bind: false }
                            }]
                        }
                    }
                ]
            )]
        });
        let ir = proto_to_ir(&proto).unwrap();

        let comment_items: Vec<_> = ir.items().filter(|item| item.id() == ItemId(200)).collect();

        assert_eq!(comment_items.len(), 1);
    }

    #[gtest]
    fn test_proto_crate_names() {
        let proto = ir_rust_proto::proto!(IRProto {
            current_target: "//foo:bar",
            crate_names: [("//dep:target", "custom_crate")]
        });
        let ir = proto_to_ir(&proto).unwrap();
        assert_eq!(
            ir.crate_name(&"//dep:target".into()).map(|i| i.to_string()),
            Some("custom_crate".to_string())
        );
    }

    #[gtest]
    fn test_proto_crate_names_invalid_ident() {
        let proto = ir_rust_proto::proto!(IRProto {
            current_target: "//foo:bar",
            crate_names: [("//dep:target", "invalid*crate")]
        });
        let result = proto_to_ir(&proto);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Invalid crate name identifier: \"invalid*crate\""),
            "error: {}",
            err_msg
        );
    }

    #[gtest]
    fn test_rs_imported_crate_name_with_custom_name() {
        let proto = ir_rust_proto::proto!(IRProto {
            current_target: "//foo:bar",
            crate_names: [("//dep:target", "custom_crate")],
        });
        let ir = proto_to_ir(&proto).unwrap();
        let crate_ident = rs_imported_crate_name(&"//dep:target".into(), &ir).unwrap();
        assert_eq!(crate_ident.to_string(), "custom_crate");
    }

    #[gtest]
    fn test_rs_imported_crate_name_without_custom_name() {
        let proto = ir_rust_proto::proto!(IRProto { current_target: "//foo:bar" });
        let ir = proto_to_ir(&proto).unwrap();
        let crate_ident = rs_imported_crate_name(&"//dep:target".into(), &ir).unwrap();
        assert_eq!(crate_ident.to_string(), "target");
    }

    #[gtest]
    fn test_record_is_string_view() {
        for (defining_target, expected) in [
            // Internal HEAD layout (and also the target label used in Chromium's repository
            // layout).
            ("//support/cc_std:cc_std", true),
            // Internal toolchain stable release snapshot.
            ("//third_party/crosstool/rust/stable/crubit/support/cc_std:cc_std", true),
            // Internal toolchain stable_nonprod release snapshot.
            ("//third_party/crosstool/rust/stable_nonprod/crubit/support/cc_std:cc_std", true),
            // Open-source Crubit repository layout (GitHub: google/crubit, rewritten by Copybara).
            ("//support/cc_std:cc_std", true),
            // Open-source Crubit repository layout using Bazel's shorthand package syntax.
            ("//support/cc_std", true),
            // Negative case: a different target within the cc_std package.
            ("//support/cc_std:other", false),
            // Negative case: an unrelated target.
            ("//rs_bindings_from_cc:ir", false),
        ] {
            let record = ir_rust_proto::proto!(crate::ir_rust_proto::Record {
                rs_name: __ { identifier: "string_view" },
                trait_derives: __ {
                    clone: crate::ir_rust_proto::TraitImplPolarity::None,
                    copy: crate::ir_rust_proto::TraitImplPolarity::None,
                    debug: crate::ir_rust_proto::TraitImplPolarity::None,
                    send: crate::ir_rust_proto::TraitImplPolarity::None,
                    sync: crate::ir_rust_proto::TraitImplPolarity::None,
                },
                template_specialization: __ {
                    defining_target: defining_target,
                    kind: crate::TemplateSpecializationKind::StdStringView,
                },
            });
            assert_eq!(record.is_string_view(), expected, "target={defining_target}");
        }
    }

    #[gtest]
    fn test_constant_array_type_validate_num_elements() {
        let make_proto = |num_elements: usize| {
            ir_rust_proto::proto!(IRProto {
                current_target: "//foo:bar",
                top_level_items: [(
                    "//foo:bar",
                    [__ {
                        type_alias: __ {
                            cc_name: __ { identifier: "Arr" },
                            rs_name: __ { identifier: "Arr" },
                            owning_target: "//foo:bar",
                            id: 1u64,
                            underlying_type: __ {
                                variant: __ {
                                    constant_array: __ {
                                        num_elements: num_elements,
                                        element_type: __ {
                                            variant: __ { primitive: Primitive::Int },
                                        },
                                    },
                                },
                            },
                        },
                    }]
                )],
            })
        };

        let err = proto_to_ir(&make_proto(0)).unwrap_err();
        assert!(
            err.to_string().contains("ConstantArrayType has non-positive num_elements"),
            "unexpected error for num_elements=0: {err}"
        );

        let valid_ir = proto_to_ir(&make_proto(4)).unwrap();
        let ta = valid_ir.type_aliases().next().unwrap();
        let CcTypeVariant::ConstantArray(arr) = ta.underlying_type().variant() else {
            panic!("expected ConstantArray, got {:?}", ta.underlying_type());
        };
        assert_eq!(arr.num_elements(), 4);
    }
}
