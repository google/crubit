// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Types and deserialization logic for IR. See docs in
//! `rs_bindings_from_cc/ir.h` for more
//! information.
use arc_anyhow::{bail, ensure, Context, Error, Result};
use code_gen_utils::{make_rs_ident, try_make_rs_ident};
use crubit_feature::CrubitFeature;
use ir_rust_proto::{
    CommentView, ConstantView, EnumView, EnumeratorView, ExistingRustTypeView, FuncView,
    GlobalVarView, IdentifierView, IncompleteRecordView, IntegerConstantView, NamespaceView,
    RecordView, SizeAlignView, TypeAliasView, UseModView,
};
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::hash_map::{Entry, HashMap};
use std::collections::{BTreeMap, HashSet};
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

mod proto_to_ir;
pub use proto_to_ir::proto_to_ir;

pub use ir_rust_proto::bridge_type::callable::{BackingType, FnTrait};

/// Trait for validating a proto before converting it to an IR item.
///
/// Separates eager validation from lazy, infallible construction later.
trait ProtoWrapper<'pb>: Sized {
    type ProtoView;

    /// Validates the proto, returning an error if it is invalid.
    fn validate(proto: Self::ProtoView) -> Result<()>;

    /// Converts the proto to the IR item.
    ///
    /// This function should only be called after `validate` has returned `Ok`. If the proto isn't
    /// validated, methods on the resulting IR item may panic.
    fn from_proto(proto: Self::ProtoView) -> Self;
}

/// Common data about all items.
pub trait GenericItem<'pb> {
    fn id(&self) -> ItemId;

    /// The unique name (probably the USR) of the item for log aggregation purposes.
    fn unique_name(&self) -> Option<&'pb str>;

    /// The Bazel target which owns the bindings for this item.
    fn owning_target(&self) -> Option<BazelLabel>;

    /// If this item is unsupported by Crubit, we may generate
    /// markers in the Rust bindings to indicate that the item is not
    /// supported. This function returns the kind of unsupported item
    /// in order to generate such markers in the proper namespace
    /// (type, function, module).
    fn unsupported_kind(&self) -> UnsupportedItemKind;

    /// The recorded source location, or None if none is present.
    fn source_loc(&self) -> Option<&'pb str>;

    /// A human-readable list of unknown attributes, or None if all attributes
    /// were understood.
    fn unknown_attr(&self) -> Option<&'pb str>;

    /// Whether failure to generate binding should be treated as a hard error (`CRUBIT_MUST_BIND`).
    fn must_bind(&self) -> bool;

    /// Returns the C++ name of this item, if applicable.
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        None
    }
}

impl<'pb, T> GenericItem<'pb> for Rc<T>
where
    T: GenericItem<'pb> + ?Sized,
{
    fn id(&self) -> ItemId {
        (**self).id()
    }
    fn unique_name(&self) -> Option<&'pb str> {
        (**self).unique_name()
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        (**self).owning_target()
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        (**self).unsupported_kind()
    }
    fn source_loc(&self) -> Option<&'pb str> {
        (**self).source_loc()
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        (**self).unknown_attr()
    }
    fn must_bind(&self) -> bool {
        (**self).must_bind()
    }
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        (**self).cc_name_as_str()
    }
}

/// A pre-order depth-first search iterator over the nested IR tree.
/// Preserves C++ declaration order and allows bindings generator components (e.g., `has_bindings`)
/// that expect a flat list of items to work in-place on the nested IR without a full rewrite.
pub struct ItemsIterator<'pb, 'iter> {
    stack: Vec<&'iter Item<'pb>>,
    visited: HashSet<ItemId>,
}

impl<'pb, 'iter> ItemsIterator<'pb, 'iter> {
    fn new(mut stack: Vec<&'iter Item<'pb>>) -> Self {
        // Reverse so we pop roots in their original order.
        stack.reverse();
        Self { stack, visited: HashSet::new() }
    }
}

impl<'pb, 'iter> Iterator for ItemsIterator<'pb, 'iter> {
    type Item = &'iter Item<'pb>;

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
fn populate_item_id_to_item<'pb>(
    item: &Item<'pb>,
    item_id_to_item: &mut HashMap<ItemId, Item<'pb>>,
) {
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
pub fn make_ir<'pb>(tree_ir: TreeIR<'pb>) -> IR<'pb> {
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
    let mut function_name_to_functions: HashMap<UnqualifiedIdentifier<'pb>, Vec<Rc<Func<'pb>>>> =
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

macro_rules! derive_debug_partialeq_eq_hash {
    {
        $(#[$impl_metas:meta]),*
        impl<'pb> $name:ident<'pb> {
            $(
                $(#[$fn_metas:meta])*
                $fn_vis:vis fn $fn_ident:ident(&$self_:ident) -> $ret:ty { $body:expr }
            )*
        }
    } => {
        $(#[$impl_metas])*
        impl<'pb> $name<'pb> {
            $(
                $(#[$fn_metas])*
                $fn_vis fn $fn_ident(&$self_) -> $ret { $body }
            )*
        }

        impl std::fmt::Debug for $name<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    $(
                        .field(stringify!($fn_ident), &self.$fn_ident())
                    )*
                    .finish()
            }
        }

        impl std::cmp::PartialEq for $name<'_> {
            fn eq(&self, other: &Self) -> bool {
                true
                $(
                    && self.$fn_ident() == other.$fn_ident()
                )*
            }
        }

        impl std::cmp::Eq for $name<'_> {}

        impl std::hash::Hash for $name<'_> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                $(
                    self.$fn_ident().hash(state);
                )*
            }
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct HeaderName<'pb> {
    pub(crate) name: &'pb str,
}

impl<'pb> HeaderName<'pb> {
    pub fn name(&self) -> &'pb str {
        self.name
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LifetimeId(pub i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LifetimeName {
    pub(crate) name: Rc<str>,
    pub(crate) id: LifetimeId,
}

impl LifetimeName {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> LifetimeId {
        self.id
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct CcType {
    pub(crate) variant: CcTypeVariant,
    pub(crate) is_const: bool,
    pub(crate) unknown_attr: Rc<str>,
    // An ordered list of lifetime variable names applied to this type. It is valid for the same
    // name to appear multiple times.
    pub(crate) explicit_lifetimes: Vec<Rc<str>>,
}

impl CcType {
    pub fn new(
        variant: CcTypeVariant,
        is_const: bool,
        unknown_attr: impl Into<Rc<str>>,
        explicit_lifetimes: Vec<Rc<str>>,
    ) -> Self {
        Self { variant, is_const, unknown_attr: unknown_attr.into(), explicit_lifetimes }
    }

    pub fn variant(&self) -> &CcTypeVariant {
        &self.variant
    }

    pub fn variant_mut(&mut self) -> &mut CcTypeVariant {
        &mut self.variant
    }

    pub fn is_const(&self) -> bool {
        self.is_const
    }

    pub fn set_is_const(&mut self, is_const: bool) {
        self.is_const = is_const;
    }

    pub fn unknown_attr(&self) -> &str {
        &self.unknown_attr
    }

    pub fn explicit_lifetimes(&self) -> &[Rc<str>] {
        &self.explicit_lifetimes
    }

    pub fn explicit_lifetimes_mut(&mut self) -> &mut Vec<Rc<str>> {
        &mut self.explicit_lifetimes
    }

    pub fn is_unit_type(&self) -> bool {
        matches!(&self.variant, CcTypeVariant::Primitive(Primitive::Void))
    }
}

impl From<&Record<'_>> for CcType {
    fn from(record: &Record) -> Self {
        CcType {
            variant: CcTypeVariant::Decl { id: record.id(), template_args: None },
            is_const: false,
            unknown_attr: Rc::default(),
            explicit_lifetimes: Vec::default(),
        }
    }
}

impl From<&TypeAlias<'_>> for CcType {
    fn from(alias: &TypeAlias) -> Self {
        CcType {
            variant: CcTypeVariant::Decl { id: alias.id(), template_args: None },
            is_const: false,
            unknown_attr: Rc::default(),
            explicit_lifetimes: Vec::default(),
        }
    }
}

impl From<&ExistingRustType<'_>> for CcType {
    fn from(existing_rust_type: &ExistingRustType) -> Self {
        CcType {
            variant: CcTypeVariant::Decl { id: existing_rust_type.id(), template_args: None },
            is_const: false,
            unknown_attr: Rc::default(),
            explicit_lifetimes: Vec::default(),
        }
    }
}

#[derive(Copy, Debug, PartialEq, Eq, Hash, Clone)]
pub enum PointerTypeKind {
    LValueRef,
    RValueRef,
    Nullable,
    NonNull,
    Owned,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PointerType {
    pub(crate) kind: PointerTypeKind,
    pub(crate) lifetime: Option<LifetimeId>,
    pub(crate) pointee_type: Rc<CcType>,
}

impl PointerType {
    pub fn new(
        kind: PointerTypeKind,
        lifetime: Option<LifetimeId>,
        pointee_type: Rc<CcType>,
    ) -> Self {
        Self { kind, lifetime, pointee_type }
    }

    pub fn kind(&self) -> PointerTypeKind {
        self.kind
    }

    pub fn pointee_type(&self) -> &CcType {
        &self.pointee_type
    }

    pub fn lifetime(&self) -> Option<LifetimeId> {
        self.lifetime
    }

    pub fn set_kind(&mut self, kind: PointerTypeKind) {
        self.kind = kind;
    }

    pub fn set_pointee_type(&mut self, pointee_type: Rc<CcType>) {
        self.pointee_type = pointee_type;
    }

    pub fn set_lifetime(&mut self, lifetime: Option<LifetimeId>) {
        self.lifetime = lifetime;
    }
}

///// Generates an enum type that implements `ToTokens`, which quotes the contents of the braces.
macro_rules! define_typed_tokens_enum {
    {$(#[$ty_attr:meta])* $vis:vis enum $Type:ident {$($(#[$variant_attr:meta])* $Variant:ident = {$($cpp_spelling:tt)+},)+}} => {
        $(#[$ty_attr])*
        $vis enum $Type {
            $(
                $(#[$variant_attr])*
                $Variant,
            )+
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

        impl std::str::FromStr for $Type {
            type Err = ::arc_anyhow::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($($cpp_spelling)+) => Ok($Type::$Variant),
                    )+
                    _ => ::arc_anyhow::bail!("Unknown {} spelling: {:?}", stringify!($Type), s),
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CcTypeVariant {
    Primitive(Primitive),
    Pointer(PointerType),
    FuncPointer {
        non_null: bool,
        call_conv: CcCallingConv,

        /// The parameter types, followed by the return type.
        param_and_return_types: Rc<[CcType]>,

        // Lifetime variable names bound by this function pointer.
        lifetime_inputs: Vec<Rc<str>>,
    },
    Decl {
        id: ItemId,
        /// The type arguments to the type. These override any type arguments attached to the item.
        template_args: Option<Rc<[CcType]>>,
    },
    /// This type could not be translated to Rust.
    ///
    /// It's preferable to forward on a failed type conversion,
    /// to defer errors as late as possible. For instance, struct
    /// fields should become blobs of bytes, instead of failing
    /// the whole struct.
    Error(FormattedError),
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
            CcTypeVariant::Decl { id, .. } => Some(*id),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Identifier<'pb> {
    proto: IdentifierView<'pb>,
}

impl<'pb> ProtoWrapper<'pb> for Identifier<'pb> {
    type ProtoView = IdentifierView<'pb>;

    fn validate(proto: IdentifierView<'pb>) -> Result<()> {
        let _ = proto.identifier().to_str()?;
        Ok(())
    }

    fn from_proto(proto: IdentifierView<'pb>) -> Self {
        Identifier { proto }
    }
}

impl<'pb> Identifier<'pb> {
    pub fn as_str(&self) -> &'pb str {
        self.proto
            .identifier()
            .to_str()
            .expect("`identifier` should have been validated by `Identifier::validate`")
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.as_str())
    }
}

impl PartialEq for Identifier<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Identifier<'_> {}

impl Hash for Identifier<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl PartialEq<str> for Identifier<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Identifier<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

#[derive(Copy, Clone)]
pub struct IntegerConstant<'pb>(pub(crate) IntegerConstantView<'pb>);

derive_debug_partialeq_eq_hash! {
    impl<'pb> IntegerConstant<'pb> {
        pub fn is_negative(&self) -> bool {
            self.0.is_negative()
        }

        pub fn wrapped_value(&self) -> u64 {
            self.0.wrapped_value() as u64
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Operator<'pb> {
    pub(crate) name: &'pb str,
}

impl<'pb> Operator<'pb> {
    pub fn new(name: &'pb str) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &'pb str {
        self.name
    }

    pub fn cc_name(&self) -> String {
        let separator = match self.name.chars().next() {
            Some(c) if c.is_alphabetic() => " ",
            _ => "",
        };
        format!("operator{separator}{name}", separator = separator, name = self.name)
    }
}

impl Debug for Operator<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.cc_name())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ItemId(usize);

impl Debug for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ItemId({:#x})", self.0)
    }
}

impl ItemId {
    pub fn as_u64(self) -> u64 {
        self.0 as u64
    }
    pub const fn new_for_testing(value: usize) -> Self {
        Self(value)
    }
}

impl ToTokens for ItemId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use std::str::FromStr;
        proc_macro2::Literal::from_str(&format!("{:#x}", self.0)).unwrap().to_tokens(tokens)
    }
}

impl ToTokens for LifetimeId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use std::str::FromStr;
        proc_macro2::Literal::from_str(&format!("{:#}", self.0)).unwrap().to_tokens(tokens)
    }
}
pub use bazel_label::BazelLabel;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum UnqualifiedIdentifier<'pb> {
    Identifier(Identifier<'pb>),
    Operator(Operator<'pb>),
    Constructor,
    Destructor,
    ConversionOperator,
}

impl<'pb> UnqualifiedIdentifier<'pb> {
    pub fn is_constructor(&self) -> bool {
        matches!(self, UnqualifiedIdentifier::Constructor)
    }
    pub fn is_destructor(&self) -> bool {
        matches!(self, UnqualifiedIdentifier::Destructor)
    }
    pub fn as_identifier(&self) -> Option<&Identifier<'pb>> {
        match self {
            UnqualifiedIdentifier::Identifier(identifier) => Some(identifier),
            _ => None,
        }
    }
    pub fn as_operator(&self) -> Option<&Operator<'pb>> {
        match self {
            UnqualifiedIdentifier::Operator(op) => Some(op),
            _ => None,
        }
    }
    pub fn identifier_as_str(&self) -> Option<&'pb str> {
        self.as_identifier().map(|id| id.as_str())
    }
}

impl Debug for UnqualifiedIdentifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnqualifiedIdentifier::Identifier(identifier) => Debug::fmt(identifier, f),
            UnqualifiedIdentifier::Operator(op) => Debug::fmt(op, f),
            UnqualifiedIdentifier::Constructor => f.write_str("Constructor"),
            UnqualifiedIdentifier::Destructor => f.write_str("Destructor"),
            UnqualifiedIdentifier::ConversionOperator => f.write_str("ConversionOperator"),
        }
    }
}

impl PartialEq<str> for UnqualifiedIdentifier<'_> {
    fn eq(&self, other: &str) -> bool {
        if let UnqualifiedIdentifier::Identifier(identifier) = self {
            identifier.as_str() == other
        } else {
            false
        }
    }
}

impl PartialEq<&str> for UnqualifiedIdentifier<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ReferenceQualification {
    LValue,
    RValue,
    Unqualified,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct InstanceMethodMetadata {
    pub(crate) reference: ReferenceQualification,
    pub(crate) is_const: bool,
    pub(crate) is_virtual: bool,
}

impl InstanceMethodMetadata {
    pub fn new(reference: ReferenceQualification, is_const: bool, is_virtual: bool) -> Self {
        Self { reference, is_const, is_virtual }
    }

    pub fn reference(&self) -> ReferenceQualification {
        self.reference
    }

    pub fn is_const(&self) -> bool {
        self.is_const
    }

    pub fn is_virtual(&self) -> bool {
        self.is_virtual
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Setter {
    pub type_: CcType,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Getter {
    pub type_: CcType,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MemberFuncSemantic {
    Setter(Setter),
    Getter(Getter),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FuncParam<'pb> {
    pub(crate) type_: CcType,
    pub(crate) identifier: Identifier<'pb>,
    pub(crate) clang_lifetime_capture_by: Vec<i32>,
    pub(crate) clang_lifetimebound: bool,
    pub(crate) unknown_attr: Option<&'pb str>,
}

impl<'pb> FuncParam<'pb> {
    pub fn new(
        type_: CcType,
        identifier: Identifier<'pb>,
        clang_lifetime_capture_by: Vec<i32>,
        clang_lifetimebound: bool,
        unknown_attr: Option<&'pb str>,
    ) -> Self {
        Self { type_, identifier, clang_lifetime_capture_by, clang_lifetimebound, unknown_attr }
    }

    pub fn type_(&self) -> &CcType {
        &self.type_
    }

    pub fn type_mut(&mut self) -> &mut CcType {
        &mut self.type_
    }

    pub fn set_type(&mut self, type_: CcType) {
        self.type_ = type_;
    }

    pub fn identifier(&self) -> &Identifier<'pb> {
        &self.identifier
    }

    /// A list of parameter indices attached to this parameter by Clang's lifetime_capture_by.
    /// In `f(x, y)`, `x` is parameter 0 and y is parameter 1. In the member function
    /// `S::f(x, y)`, `this` is parameter 0, `x` is 1, and `y` is 2.
    pub fn clang_lifetime_capture_by(&self) -> &[i32] {
        &self.clang_lifetime_capture_by
    }

    /// True if this parameter was annotated with Clang's lifetimebound.
    pub fn clang_lifetimebound(&self) -> bool {
        self.clang_lifetimebound
    }

    /// A human-readable list of attributes that Crubit doesn't understand.
    ///
    /// Because attributes can change the behavior or semantics of function
    /// parameters in ways that may affect interop, we default-closed and
    /// do not expose functions with unknown attributes.
    ///
    /// One notable example is `lifetimebound`, which we might expect to map
    /// to Rust lifetimes.
    pub fn unknown_attr(&self) -> Option<&'pb str> {
        self.unknown_attr
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SafetyAnnotation {
    DisableUnsafe,
    Unsafe,
    Unannotated,
}

#[derive(Clone)]
pub struct Func<'pb> {
    pub(crate) proto: FuncView<'pb>,
    pub(crate) cc_name: UnqualifiedIdentifier<'pb>,
    pub(crate) rs_name: UnqualifiedIdentifier<'pb>,
    pub(crate) unique_name: &'pb str,
    pub(crate) owning_target: BazelLabel,
    pub(crate) mangled_name: &'pb str,
    pub(crate) doc_comment: Option<&'pb str>,
    pub(crate) return_type: CcType,
    pub(crate) params: Vec<FuncParam<'pb>>,
    pub(crate) lifetime_params: Vec<LifetimeName>,
    pub(crate) instance_method_metadata: Option<InstanceMethodMetadata>,
    pub(crate) nodiscard: Option<&'pb str>,
    pub(crate) deprecated: Option<&'pb str>,
    pub(crate) unknown_attr: Option<&'pb str>,
    pub(crate) safety_annotation: SafetyAnnotation,
    pub(crate) source_loc: &'pb str,

    pub(crate) inline_cpp_source_text: Option<Rc<str>>,

    pub(crate) lifetime_inputs: Vec<Rc<str>>,

    pub(crate) semantic: Option<MemberFuncSemantic>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> Func<'pb> {
        pub fn cc_name(&self) -> &UnqualifiedIdentifier<'pb> {
            &self.cc_name
        }

        pub fn rs_name(&self) -> &UnqualifiedIdentifier<'pb> {
            &self.rs_name
        }

        pub fn unique_name(&self) -> &'pb str {
            self.unique_name
        }

        pub fn owning_target(&self) -> &BazelLabel {
            &self.owning_target
        }

        pub fn mangled_name(&self) -> &'pb str {
            self.mangled_name
        }

        pub fn doc_comment(&self) -> Option<&'pb str> {
            self.doc_comment
        }

        pub fn return_type(&self) -> &CcType {
            &self.return_type
        }

        pub fn params(&self) -> &[FuncParam<'pb>] {
            &self.params
        }

        /// For tests and internal use only.
        ///
        /// Prefer to reconstruct the lifetime params from the parameter types, as
        /// needed. This allows new parameters and lifetimes to be added that were
        /// not originally part of the IR.
        pub fn lifetime_params(&self) -> &[LifetimeName] {
            &self.lifetime_params
        }

        pub fn is_inline(&self) -> bool {
            self.proto.is_inline()
        }

        pub fn instance_method_metadata(&self) -> Option<&InstanceMethodMetadata> {
            self.instance_method_metadata.as_ref()
        }

        pub fn is_extern_c(&self) -> bool {
            self.proto.is_extern_c()
        }

        pub fn is_noreturn(&self) -> bool {
            self.proto.is_noreturn()
        }

        pub fn is_variadic(&self) -> bool {
            self.proto.is_variadic()
        }

        pub fn is_consteval(&self) -> bool {
            self.proto.is_consteval()
        }

        /// The `[[nodiscard("...")]]` string. If `[[nodiscard]]`, then the empty
        /// string is used.
        pub fn nodiscard(&self) -> Option<&'pb str> {
            self.nodiscard
        }

        /// The `[[deprecated("...")]]` string. If `[[deprecated]]`, then the empty
        /// string is used.
        pub fn deprecated(&self) -> Option<&'pb str> {
            self.deprecated
        }

        /// A human-readable list of attributes that Crubit doesn't understand.
        ///
        /// Because attributes can change the behavior or semantics of functions in
        /// fairly significant ways, and in ways that may affect interop, we
        /// default-closed and do not expose functions with unknown attributes.
        pub fn unknown_attr(&self) -> Option<&'pb str> {
            self.unknown_attr
        }

        pub fn has_c_calling_convention(&self) -> bool {
            self.proto.has_c_calling_convention()
        }

        pub fn is_member_or_descendant_of_class_template(&self) -> bool {
            self.proto.is_member_or_descendant_of_class_template()
        }

        pub fn safety_annotation(&self) -> SafetyAnnotation {
            self.safety_annotation
        }

        pub fn source_loc(&self) -> &'pb str {
            self.source_loc
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        /// The enclosing item ID.
        ///
        /// If this is a free function, then this will be None or a namespace. If this is
        /// a member function, it will be a record type in C++, but might be an
        /// `ExistingRustType` if it was renamed.
        pub fn enclosing_item_id(&self) -> Option<ItemId> {
            Into::<Option<i64>>::into(self.proto.enclosing_item_id_opt()).map(|id| ItemId(id as usize))
        }

        /// If this function was declared as a `friend` inside of a record
        /// definition, this ItemId refers to the record containing the `friend`
        /// function declaration.
        ///
        /// The record pointed to by `ItemId` must then be ADL-visible in order to
        /// invoke this function.
        pub fn adl_enclosing_record(&self) -> Option<ItemId> {
            Into::<Option<i64>>::into(self.proto.adl_enclosing_record_opt()).map(|id| ItemId(id as usize))
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }

        pub fn inline_cpp_source_text(&self) -> Option<&str> {
            self.inline_cpp_source_text.as_deref()
        }

        /// Lifetime variable names bound by this function.
        pub fn lifetime_inputs(&self) -> &[Rc<str>] {
            &self.lifetime_inputs
        }

        pub fn semantic(&self) -> Option<&MemberFuncSemantic> {
            self.semantic.as_ref()
        }
    }
}

impl<'pb> GenericItem<'pb> for Func<'pb> {
    fn id(&self) -> ItemId {
        self.id()
    }
    fn unique_name(&self) -> Option<&'pb str> {
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
    fn source_loc(&self) -> Option<&'pb str> {
        Some(self.source_loc())
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        self.must_bind()
    }
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        self.cc_name.identifier_as_str()
    }
}

impl<'pb> Func<'pb> {
    pub fn is_instance_method(&self) -> bool {
        self.instance_method_metadata.is_some()
    }

    pub fn source_text_as_token_stream(&self) -> Option<proc_macro2::TokenStream> {
        self.inline_cpp_source_text()?.parse::<proc_macro2::TokenStream>().ok()
    }

    pub fn set_inline_cpp_source_text(&mut self, text: Option<Rc<str>>) {
        self.inline_cpp_source_text = text;
    }

    pub fn return_type_mut(&mut self) -> &mut CcType {
        &mut self.return_type
    }

    pub fn set_return_type(&mut self, return_type: CcType) {
        self.return_type = return_type;
    }

    pub fn params_mut(&mut self) -> &mut Vec<FuncParam<'pb>> {
        &mut self.params
    }

    pub fn lifetime_inputs_mut(&mut self) -> &mut Vec<Rc<str>> {
        &mut self.lifetime_inputs
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum AccessSpecifier {
    Public,
    Protected,
    Private,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Field<'pb> {
    pub(crate) rust_identifier: Option<Identifier<'pb>>,
    pub(crate) cpp_identifier: Option<Identifier<'pb>>,
    pub(crate) doc_comment: Option<&'pb str>,
    pub(crate) type_: CcType,
    pub(crate) access: AccessSpecifier,
    pub(crate) offset: usize,
    pub(crate) size: usize,

    pub(crate) unknown_attr: Result<Option<&'pb str>, String>,

    pub(crate) is_no_unique_address: bool,
    pub(crate) is_bitfield: bool,

    // TODO(kinuko): Consider removing this, it is a duplicate of the same information
    // in `Record`.
    pub(crate) is_inheritable: bool,
    pub(crate) is_mutable: bool,

    pub(crate) deprecated: Option<&'pb str>,
}

impl<'pb> Field<'pb> {
    pub fn rust_identifier(&self) -> Option<&Identifier<'pb>> {
        self.rust_identifier.as_ref()
    }

    pub fn cpp_identifier(&self) -> Option<&Identifier<'pb>> {
        self.cpp_identifier.as_ref()
    }

    pub fn doc_comment(&self) -> Option<&'pb str> {
        self.doc_comment
    }

    pub fn type_(&self) -> &CcType {
        &self.type_
    }

    pub fn type_mut(&mut self) -> &mut CcType {
        &mut self.type_
    }

    pub fn set_type(&mut self, type_: CcType) {
        self.type_ = type_;
    }

    pub fn access(&self) -> AccessSpecifier {
        self.access
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// A human-readable list of attributes that Crubit doesn't understand.
    pub fn unknown_attr(&self) -> &Result<Option<&'pb str>, String> {
        &self.unknown_attr
    }

    pub fn is_no_unique_address(&self) -> bool {
        self.is_no_unique_address
    }

    pub fn is_bitfield(&self) -> bool {
        self.is_bitfield
    }

    pub fn is_inheritable(&self) -> bool {
        self.is_inheritable
    }

    pub fn is_mutable(&self) -> bool {
        self.is_mutable
    }

    /// The `[[deprecated("...")]]` string. If `[[deprecated]]`, then the empty
    /// string is used.
    pub fn deprecated(&self) -> Option<&'pb str> {
        self.deprecated
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpecialMemberFunc {
    Trivial,
    NontrivialMembers,
    NontrivialUserDefined,
    Unavailable,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BaseClass {
    pub(crate) base_record_id: ItemId,
    pub(crate) offset: Option<i64>,
}

impl BaseClass {
    pub fn base_record_id(&self) -> ItemId {
        self.base_record_id
    }

    pub fn offset(&self) -> Option<i64> {
        self.offset
    }
}

#[derive(Clone)]
pub struct IncompleteRecord<'pb> {
    pub(crate) proto: IncompleteRecordView<'pb>,
    pub(crate) cc_name: Identifier<'pb>,
    pub(crate) rs_name: Identifier<'pb>,
    pub(crate) unique_name: &'pb str,
    pub(crate) owning_target: BazelLabel,
    pub(crate) unknown_attr: Option<&'pb str>,
    pub(crate) record_type: RecordType,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> IncompleteRecord<'pb> {
        pub fn cc_name(&self) -> &Identifier<'pb> {
            &self.cc_name
        }

        pub fn rs_name(&self) -> &Identifier<'pb> {
            &self.rs_name
        }

        pub fn unique_name(&self) -> &'pb str {
            self.unique_name
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn owning_target(&self) -> &BazelLabel {
            &self.owning_target
        }

        /// A human-readable list of attributes that Crubit doesn't understand.
        ///
        /// Because attributes can change the behavior or semantics of types in
        /// fairly significant ways, and in ways that may affect interop, we
        /// default-closed and do not expose functions with unknown attributes.
        pub fn unknown_attr(&self) -> Option<&'pb str> {
            self.unknown_attr
        }

        pub fn record_type(&self) -> RecordType {
            self.record_type
        }

        pub fn enclosing_item_id(&self) -> Option<ItemId> {
            Into::<Option<i64>>::into(self.proto.enclosing_item_id_opt()).map(|id| ItemId(id as usize))
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }
    }
}

impl<'pb> GenericItem<'pb> for IncompleteRecord<'pb> {
    fn id(&self) -> ItemId {
        IncompleteRecord::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        self.record_type.unsupported_item_kind()
    }
    fn source_loc(&self) -> Option<&'pb str> {
        None
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        IncompleteRecord::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        Some(self.cc_name.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum RecordType {
    Struct,
    Union,
    Class,
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

#[derive(Copy, Clone)]
pub struct SizeAlign<'pb>(pub(crate) SizeAlignView<'pb>);

derive_debug_partialeq_eq_hash! {
    impl<'pb> SizeAlign<'pb> {
        pub fn size(&self) -> usize {
            self.0.size() as usize
        }

        pub fn alignment(&self) -> usize {
            self.0.alignment() as usize
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BridgeType<'pb> {
    ProtoMessageBridge {
        rust_name: &'pb str,
    },
    Bridge {
        rust_name: &'pb str,
        abi_rust: &'pb str,
        abi_cpp: &'pb str,
        template_args: Rc<[CcType]>,
        label_hint: Option<&'pb str>,
    },
    StdOptional(CcType),
    StdPair(CcType, CcType),
    StdString,
    Callable {
        backing_type: BackingType,
        fn_trait: FnTrait,
        return_type: CcType,
        param_types: Vec<CcType>,
    },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TemplateArg {
    Type(CcType),
    Bool(bool),
    Int(i64),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TemplateSpecialization {
    pub(crate) defining_target: BazelLabel,

    pub(crate) kind: TemplateSpecializationKind,
}

impl TemplateSpecialization {
    /// The target containing the template definition
    pub fn defining_target(&self) -> &BazelLabel {
        &self.defining_target
    }

    /// The kind of template specialization.
    pub fn kind(&self) -> &TemplateSpecializationKind {
        &self.kind
    }

    pub fn new_for_testing(defining_target: BazelLabel, kind: TemplateSpecializationKind) -> Self {
        Self { defining_target, kind }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TemplateSpecializationKind {
    /// std::basic_string_view<char, std::char_traits<char>>
    StdStringView,
    /// std::basic_string_view<wchar_t, std::char_traits<wchar_t>>
    StdWStringView,
    /// std::vector<T, std::allocator<T>>
    StdVector { raw_element_type: CcType },
    /// std::shared_ptr<T>
    StdSharedPtr { raw_element_type: CcType },
    /// std::unique_ptr<T, std::default_delete<T>>
    StdUniquePtr { raw_element_type: CcType },
    /// c9::Co<T>
    C9Co { raw_element_type: CcType },
    /// absl::Span<T>
    AbslSpan { raw_element_type: CcType },
    /// absl::flat_hash_map<K, V, ...>
    AbslFlatHashMap { raw_key_type: CcType, raw_value_type: CcType },
    /// absl::flat_hash_set<T, ...>
    AbslFlatHashSet { raw_element_type: CcType },
    /// Some other template specialization.
    NonSpecial,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TraitImplPolarity {
    Negative,
    None,
    Positive,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TraitDerives<'pb> {
    // <internal link> start
    pub clone: TraitImplPolarity,
    pub copy: TraitImplPolarity,
    pub debug: TraitImplPolarity,
    // <internal link> end
    pub send: bool,
    pub sync: bool,
    pub custom: Vec<&'pb str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OwnedPtrConfig<'pb> {
    pub owned_ptr_type: &'pb str,
    pub drop_impl: &'pb str,
}

#[derive(Clone)]
pub struct Record<'pb> {
    pub(crate) proto: RecordView<'pb>,
    pub(crate) rs_name: Identifier<'pb>,
    pub(crate) cc_name: Identifier<'pb>,
    pub(crate) unique_name: &'pb str,
    pub(crate) mangled_cc_name: &'pb str,
    pub(crate) owning_target: BazelLabel,
    pub(crate) template_specialization: Option<TemplateSpecialization>,
    pub(crate) unknown_attr: Option<&'pb str>,
    pub(crate) doc_comment: Option<&'pb str>,
    pub(crate) bridge_type: Option<BridgeType<'pb>>,
    pub(crate) owned_ptr_config: Option<OwnedPtrConfig<'pb>>,
    pub(crate) source_loc: &'pb str,
    pub(crate) unambiguous_public_bases: Vec<BaseClass>,
    pub(crate) fields: Vec<Field<'pb>>,
    pub(crate) lifetime_params: Vec<LifetimeName>,
    pub(crate) trait_derives: TraitDerives<'pb>,
    pub(crate) safety_annotation: SafetyAnnotation,
    pub(crate) copy_constructor: SpecialMemberFunc,
    pub(crate) move_constructor: SpecialMemberFunc,
    pub(crate) destructor: SpecialMemberFunc,
    pub(crate) nodiscard: Option<&'pb str>,
    pub(crate) record_type: RecordType,
    pub(crate) lifetime_inputs: Vec<Rc<str>>,
    pub(crate) deprecated: Option<&'pb str>,
    pub(crate) children: Vec<Item<'pb>>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> Record<'pb> {
        pub fn rs_name(&self) -> &Identifier<'pb> {
            &self.rs_name
        }

        /// The C++ name of the record. If the record is a template specialization, the fully qualified
        /// name is used. Otherwise, the only the name of the record is used.
        /// Today, cc_name is only used for debugging, checking for names starting in __, and generating
        /// parent modules for nested items which are disallowed for template specializations in Crubit.
        pub fn cc_name(&self) -> &Identifier<'pb> {
            &self.cc_name
        }

        pub fn unique_name(&self) -> &'pb str {
            self.unique_name
        }

        /// Mangled record names are used to 1) provide valid Rust identifiers for
        /// C++ template specializations, and 2) help build unique names for virtual
        /// upcast thunks.
        pub fn mangled_cc_name(&self) -> &'pb str {
            self.mangled_cc_name
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn owning_target(&self) -> &BazelLabel {
            &self.owning_target
        }

        pub fn template_specialization(&self) -> Option<&TemplateSpecialization> {
            self.template_specialization.as_ref()
        }

        /// A human-readable list of attributes that Crubit doesn't understand.
        ///
        /// Because attributes can change the behavior or semantics of types in
        /// fairly significant ways, and in ways that may affect interop, we
        /// default-closed and do not expose functions with unknown attributes.
        pub fn unknown_attr(&self) -> Option<&'pb str> {
            self.unknown_attr
        }

        pub fn doc_comment(&self) -> Option<&'pb str> {
            self.doc_comment
        }

        pub fn bridge_type(&self) -> Option<&BridgeType<'pb>> {
            self.bridge_type.as_ref()
        }

        pub fn owned_ptr_config(&self) -> Option<&OwnedPtrConfig<'pb>> {
            self.owned_ptr_config.as_ref()
        }

        pub fn source_loc(&self) -> &'pb str {
            self.source_loc
        }

        pub fn unambiguous_public_bases(&self) -> &[BaseClass] {
            &self.unambiguous_public_bases
        }

        pub fn fields(&self) -> &[Field<'pb>] {
            &self.fields
        }

        pub fn lifetime_params(&self) -> &[LifetimeName] {
            &self.lifetime_params
        }

        pub fn size_align(&self) -> SizeAlign<'pb> {
            SizeAlign(self.proto.size_align())
        }

        pub fn trait_derives(&self) -> &TraitDerives<'pb> {
            &self.trait_derives
        }

        pub fn is_derived_class(&self) -> bool {
            self.proto.is_derived_class()
        }

        pub fn override_alignment(&self) -> bool {
            self.proto.override_alignment()
        }

        pub fn safety_annotation(&self) -> SafetyAnnotation {
            self.safety_annotation
        }

        pub fn copy_constructor(&self) -> SpecialMemberFunc {
            self.copy_constructor
        }

        pub fn move_constructor(&self) -> SpecialMemberFunc {
            self.move_constructor
        }

        pub fn destructor(&self) -> SpecialMemberFunc {
            self.destructor
        }

        pub fn is_trivial_abi(&self) -> bool {
            self.proto.is_trivial_abi()
        }

        pub fn is_inheritable(&self) -> bool {
            self.proto.is_inheritable()
        }

        pub fn is_abstract(&self) -> bool {
            self.proto.is_abstract()
        }

        /// The `[[nodiscard("...")]]` string. If `[[nodiscard]]`, then the empty
        /// string is used.
        pub fn nodiscard(&self) -> Option<&'pb str> {
            self.nodiscard
        }

        pub fn record_type(&self) -> RecordType {
            self.record_type
        }

        pub fn is_aggregate(&self) -> bool {
            self.proto.is_aggregate()
        }

        pub fn is_canonical_alias(&self) -> bool {
            self.proto.is_canonical_alias()
        }

        pub fn enclosing_item_id(&self) -> Option<ItemId> {
            Into::<Option<i64>>::into(self.proto.enclosing_item_id_opt()).map(|id| ItemId(id as usize))
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }

        /// Whether this type has an overload of `operator delete`.
        pub fn overloads_operator_delete(&self) -> bool {
            self.proto.overloads_operator_delete()
        }

        pub fn has_private_or_deleted_operator_delete(&self) -> bool {
            self.proto.has_private_or_deleted_operator_delete()
        }

        /// Lifetime variable names bound by this record.
        pub fn lifetime_inputs(&self) -> &[Rc<str>] {
            &self.lifetime_inputs
        }

        pub fn impl_debug(&self) -> bool {
            self.proto.impl_debug()
        }

        pub fn has_private_pointer_or_reference_fields(&self) -> bool {
            self.proto.has_private_pointer_or_reference_fields()
        }

        pub fn detected_formatter(&self) -> bool {
            self.proto.detected_formatter()
        }

        /// The `[[deprecated("...")]]` string. If `[[deprecated]]`, then the empty
        /// string is used.
        pub fn deprecated(&self) -> Option<&'pb str> {
            self.deprecated
        }

        /// Whether this type is annotated as thread-safe (CRUBIT_THREAD_SAFE).
        pub fn is_thread_safe(&self) -> bool {
            self.proto.is_thread_safe()
        }

        pub fn is_explicit_class_template_instantiation_definition(&self) -> bool {
            self.proto.is_explicit_class_template_instantiation_definition()
        }

        pub fn children(&self) -> &[Item<'pb>] {
            &self.children
        }
    }
}

impl<'pb> GenericItem<'pb> for Record<'pb> {
    fn id(&self) -> ItemId {
        Record::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        self.record_type.unsupported_item_kind()
    }
    fn source_loc(&self) -> Option<&'pb str> {
        Some(self.source_loc())
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        Record::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        Some(self.cc_name.as_str())
    }
}

impl<'pb> Record<'pb> {
    pub fn set_rs_name(&mut self, rs_name: Identifier<'pb>) {
        self.rs_name = rs_name;
    }

    pub fn set_cc_name(&mut self, cc_name: Identifier<'pb>) {
        self.cc_name = cc_name;
    }

    pub fn fields_mut(&mut self) -> &mut Vec<Field<'pb>> {
        &mut self.fields
    }

    pub fn lifetime_inputs_mut(&mut self) -> &mut Vec<Rc<str>> {
        &mut self.lifetime_inputs
    }

    pub fn set_children(&mut self, children: Vec<Item<'pb>>) {
        self.children = children;
    }

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
    /// Described in more detail at: docs/design/unpin.md
    pub fn is_unpin(&self) -> bool {
        self.is_trivial_abi()
    }

    // TODO(b/498977848): The record with cc_name
    // "std::basic_string_view<char8_t, std::char_traits<char8_t>>" with
    // rs_name "__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE" is given
    // TemplateSpecialization kind NonSpecial. This is unfortunate, since we want to exclude all
    // flavors of string_view because of our special-casing.
    pub fn is_string_view(&self) -> bool {
        match &self.template_specialization {
            Some(TemplateSpecialization { defining_target, kind, .. }) => {
                let is_in_cc_std = *defining_target
                    == BazelLabel::from("//support/cc_std:cc_std")
                    || *defining_target
                        == BazelLabel::from(
                            "//third_party/crosstool/rust/stable/crubit/support/cc_std:cc_std",
                        );
                if is_in_cc_std {
                    let is_string_view = *kind == TemplateSpecializationKind::StdStringView
                        || *kind == TemplateSpecializationKind::StdWStringView;
                    if is_string_view {
                        return true;
                    };
                    self.cc_name.as_str().starts_with("std::basic_string_view<")
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Detect whether a `Record` is *specifically* the raw projection of `std::string_view`, as we
    /// special-case this.
    pub fn is_raw_string_view(&self) -> bool {
        matches!(
            self.template_specialization,
            Some(TemplateSpecialization { kind: TemplateSpecializationKind::StdStringView, .. })
        ) && self.rs_name.as_str() == "raw_string_view"
    }

    pub fn is_union(&self) -> bool {
        match self.record_type {
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
            self.record_type.into_token_stream()
        }
    }

    pub fn should_implement_drop(&self) -> bool {
        match self.destructor {
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
                    && self.copy_constructor == SpecialMemberFunc::Trivial
                    && self.destructor == SpecialMemberFunc::Trivial
                    && self.check_by_value().is_ok()
                    && self.trait_derives.clone != TraitImplPolarity::Negative
                    // Mutable fields become `Cell<T>` in Rust, which prevents
                    // the struct from deriving `Copy`.
                    && self.fields.iter().all(|f| !f.is_mutable)
            }
        }
    }

    /// Returns Ok if the type can exist by value.
    ///
    /// This does not necessarily imply that the type is Rust-movable, e.g. trivially relocatable.
    pub fn check_by_value(&self) -> Result<()> {
        ensure!(
            self.destructor != SpecialMemberFunc::Unavailable,
            "`{}` can't be used by-value because it has a non-public or deleted destructor",
            self.cc_name
        );
        ensure!(
            !self.is_abstract(),
            "`{}` can be used by-value because it has pure virtual functions that are not overridden",
            self.cc_name
        );
        Ok(())
    }

    /// Whether this record has a unique owning target.
    ///
    /// Notably, all records that have a unique owning target are supported, e.g. `std::string`, but
    /// not all supported records have a unique owning target, e.g. `std::vector<int>`.
    pub fn has_unique_owning_target(&self) -> bool {
        self.template_specialization.is_none() || self.is_canonical_alias()
    }
}

#[derive(Clone)]
pub struct Constant<'pb> {
    pub(crate) proto: ConstantView<'pb>,
    pub(crate) cc_name: Identifier<'pb>,
    pub(crate) rs_name: Identifier<'pb>,
    pub(crate) unique_name: &'pb str,
    pub(crate) owning_target: BazelLabel,
    pub(crate) source_loc: &'pb str,
    pub(crate) unknown_attr: Option<&'pb str>,
    pub(crate) type_: CcType,
    pub(crate) deprecated: Option<&'pb str>,
    pub(crate) doc_comment: Option<&'pb str>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> Constant<'pb> {
        pub fn value(&self) -> IntegerConstant<'pb> {
            IntegerConstant(self.proto.value())
        }

        pub fn cc_name(&self) -> &Identifier<'pb> {
            &self.cc_name
        }

        pub fn rs_name(&self) -> &Identifier<'pb> {
            &self.rs_name
        }

        pub fn unique_name(&self) -> &'pb str {
            self.unique_name
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn owning_target(&self) -> &BazelLabel {
            &self.owning_target
        }

        pub fn source_loc(&self) -> &'pb str {
            self.source_loc
        }

        pub fn unknown_attr(&self) -> Option<&'pb str> {
            self.unknown_attr
        }

        pub fn enclosing_item_id(&self) -> Option<ItemId> {
            Into::<Option<i64>>::into(self.proto.enclosing_item_id_opt()).map(|id| ItemId(id as usize))
        }

        pub fn type_(&self) -> &CcType {
            &self.type_
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }

        /// The `[[deprecated("...")]]` string. If `[[deprecated]]`, then the empty
        /// string is used.
        pub fn deprecated(&self) -> Option<&'pb str> {
            self.deprecated
        }

        pub fn doc_comment(&self) -> Option<&'pb str> {
            self.doc_comment
        }
    }
}

impl<'pb> GenericItem<'pb> for Constant<'pb> {
    fn id(&self) -> ItemId {
        Constant::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::GlobalVar
    }
    fn source_loc(&self) -> Option<&'pb str> {
        Some(self.source_loc())
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        Constant::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        Some(self.cc_name.as_str())
    }
}

#[derive(Clone)]
pub struct GlobalVar<'pb> {
    pub(crate) proto: GlobalVarView<'pb>,
    pub(crate) cc_name: Identifier<'pb>,
    pub(crate) rs_name: Identifier<'pb>,
    pub(crate) unique_name: &'pb str,
    pub(crate) owning_target: BazelLabel,
    pub(crate) source_loc: &'pb str,
    pub(crate) unknown_attr: Option<&'pb str>,
    pub(crate) mangled_name: Option<&'pb str>,
    pub(crate) type_: CcType,
    pub(crate) deprecated: Option<&'pb str>,
    pub(crate) doc_comment: Option<&'pb str>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> GlobalVar<'pb> {
        pub fn cc_name(&self) -> &Identifier<'pb> {
            &self.cc_name
        }

        pub fn rs_name(&self) -> &Identifier<'pb> {
            &self.rs_name
        }

        pub fn unique_name(&self) -> &'pb str {
            self.unique_name
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn owning_target(&self) -> &BazelLabel {
            &self.owning_target
        }

        pub fn source_loc(&self) -> &'pb str {
            self.source_loc
        }

        /// A human-readable list of attributes that Crubit doesn't understand.
        pub fn unknown_attr(&self) -> Option<&'pb str> {
            self.unknown_attr
        }

        pub fn enclosing_item_id(&self) -> Option<ItemId> {
            Into::<Option<i64>>::into(self.proto.enclosing_item_id_opt()).map(|id| ItemId(id as usize))
        }

        pub fn mangled_name(&self) -> Option<&'pb str> {
            self.mangled_name
        }

        pub fn type_(&self) -> &CcType {
            &self.type_
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }

        /// The `[[deprecated("...")]]` string. If `[[deprecated]]`, then the empty
        /// string is used.
        pub fn deprecated(&self) -> Option<&'pb str> {
            self.deprecated
        }

        pub fn doc_comment(&self) -> Option<&'pb str> {
            self.doc_comment
        }
    }
}

impl<'pb> GenericItem<'pb> for GlobalVar<'pb> {
    fn id(&self) -> ItemId {
        GlobalVar::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::GlobalVar
    }
    fn source_loc(&self) -> Option<&'pb str> {
        Some(self.source_loc())
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        GlobalVar::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        Some(self.cc_name.as_str())
    }
}

#[derive(Clone)]
pub struct Enum<'pb> {
    pub(crate) proto: EnumView<'pb>,
    pub(crate) cc_name: Identifier<'pb>,
    pub(crate) rs_name: Identifier<'pb>,
    pub(crate) unique_name: &'pb str,
    pub(crate) mangled_cc_name: &'pb str,
    pub(crate) owning_target: BazelLabel,
    pub(crate) source_loc: &'pb str,
    pub(crate) underlying_type: CcType,
    pub(crate) enumerators: Option<Vec<Enumerator<'pb>>>,
    pub(crate) unknown_attr: Option<&'pb str>,
    pub(crate) nodiscard: Option<&'pb str>,
    pub(crate) deprecated: Option<&'pb str>,
    pub(crate) doc_comment: Option<&'pb str>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> Enum<'pb> {
        pub fn cc_name(&self) -> &Identifier<'pb> {
            &self.cc_name
        }

        pub fn rs_name(&self) -> &Identifier<'pb> {
            &self.rs_name
        }

        pub fn unique_name(&self) -> &'pb str {
            self.unique_name
        }

        pub fn mangled_cc_name(&self) -> &'pb str {
            self.mangled_cc_name
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn owning_target(&self) -> &BazelLabel {
            &self.owning_target
        }

        pub fn source_loc(&self) -> &'pb str {
            self.source_loc
        }

        pub fn underlying_type(&self) -> &CcType {
            &self.underlying_type
        }

        /// The enumerators. If None, this is a forward-declared (opaque) enum.
        ///
        /// That is, the difference between `enum X : int {};` and `enum X : int;`
        /// is that the former has `Some(vec![])` for the enumerators, while the
        /// latter has `None`.
        pub fn enumerators(&self) -> Option<&[Enumerator<'pb>]> {
            self.enumerators.as_deref()
        }

        /// A human-readable list of attributes that Crubit doesn't understand.
        pub fn unknown_attr(&self) -> Option<&'pb str> {
            self.unknown_attr
        }

        pub fn enclosing_item_id(&self) -> Option<ItemId> {
            Into::<Option<i64>>::into(self.proto.enclosing_item_id_opt()).map(|id| ItemId(id as usize))
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }

        pub fn detected_formatter(&self) -> bool {
            self.proto.detected_formatter()
        }

        /// The `[[nodiscard("...")]]` string. If `[[nodiscard]]`, then the empty
        /// string is used.
        pub fn nodiscard(&self) -> Option<&'pb str> {
            self.nodiscard
        }

        /// The `[[deprecated("...")]]` string. If `[[deprecated]]`, then the empty
        /// string is used.
        pub fn deprecated(&self) -> Option<&'pb str> {
            self.deprecated
        }

        pub fn doc_comment(&self) -> Option<&'pb str> {
            self.doc_comment
        }
    }
}

impl<'pb> GenericItem<'pb> for Enum<'pb> {
    fn id(&self) -> ItemId {
        Enum::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Enum
    }
    fn source_loc(&self) -> Option<&'pb str> {
        Some(self.source_loc())
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        Enum::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        Some(self.cc_name.as_str())
    }
}

#[derive(Clone)]
pub struct Enumerator<'pb> {
    pub(crate) proto: EnumeratorView<'pb>,
    pub(crate) identifier: Identifier<'pb>,
    pub(crate) unknown_attr: Option<&'pb str>,
    pub(crate) deprecated: Option<&'pb str>,
    pub(crate) doc_comment: Option<&'pb str>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> Enumerator<'pb> {
        pub fn identifier(&self) -> &Identifier<'pb> {
            &self.identifier
        }

        pub fn value(&self) -> IntegerConstant<'pb> {
            IntegerConstant(self.proto.value())
        }

        /// A human-readable list of attributes that Crubit doesn't understand.
        pub fn unknown_attr(&self) -> Option<&'pb str> {
            self.unknown_attr
        }

        /// The `[[deprecated("...")]]` string. If `[[deprecated]]`, then the empty
        /// string is used.
        pub fn deprecated(&self) -> Option<&'pb str> {
            self.deprecated
        }

        pub fn doc_comment(&self) -> Option<&'pb str> {
            self.doc_comment
        }
    }
}

#[derive(Clone)]
pub struct TypeAlias<'pb> {
    pub(crate) proto: TypeAliasView<'pb>,
    pub(crate) cc_name: Identifier<'pb>,
    pub(crate) rs_name: Identifier<'pb>,
    pub(crate) unique_name: &'pb str,
    pub(crate) owning_target: BazelLabel,
    pub(crate) doc_comment: Option<&'pb str>,
    pub(crate) unknown_attr: Option<&'pb str>,
    pub(crate) underlying_type: CcType,
    pub(crate) source_loc: &'pb str,
    pub(crate) deprecated: Option<&'pb str>,
    // Lifetime variable names bound by this type alias.
    pub(crate) lifetime_inputs: Vec<Rc<str>>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> TypeAlias<'pb> {
        pub fn cc_name(&self) -> &Identifier<'pb> {
            &self.cc_name
        }

        pub fn rs_name(&self) -> &Identifier<'pb> {
            &self.rs_name
        }

        pub fn unique_name(&self) -> &'pb str {
            self.unique_name
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn owning_target(&self) -> &BazelLabel {
            &self.owning_target
        }

        pub fn doc_comment(&self) -> Option<&'pb str> {
            self.doc_comment
        }

        /// A human-readable list of attributes that Crubit doesn't understand.
        pub fn unknown_attr(&self) -> Option<&'pb str> {
            self.unknown_attr
        }

        pub fn underlying_type(&self) -> &CcType {
            &self.underlying_type
        }

        pub fn source_loc(&self) -> &'pb str {
            self.source_loc
        }

        pub fn enclosing_item_id(&self) -> Option<ItemId> {
            Into::<Option<i64>>::into(self.proto.enclosing_item_id_opt()).map(|id| ItemId(id as usize))
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }

        /// The `[[deprecated("...")]]` string. If `[[deprecated]]`, then the empty
        /// string is used.
        pub fn deprecated(&self) -> Option<&'pb str> {
            self.deprecated
        }

        pub fn lifetime_inputs(&self) -> &[Rc<str>] {
            &self.lifetime_inputs
        }
    }
}

impl<'pb> TypeAlias<'pb> {
    pub fn underlying_type_mut(&mut self) -> &mut CcType {
        &mut self.underlying_type
    }

    pub fn lifetime_inputs_mut(&mut self) -> &mut Vec<Rc<str>> {
        &mut self.lifetime_inputs
    }
}

impl<'pb> GenericItem<'pb> for TypeAlias<'pb> {
    fn id(&self) -> ItemId {
        TypeAlias::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::TypeAlias
    }
    fn source_loc(&self) -> Option<&'pb str> {
        Some(self.source_loc())
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        TypeAlias::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        Some(self.cc_name.as_str())
    }
}

impl Display for TypeAlias<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {})", self.rs_name, self.owning_target, self.source_loc)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FormattedError {
    pub fmt: Rc<str>,
    pub message: Rc<str>,
}

/// Kind is used to indicate which item would cannot be wrapped.
/// Need to be synced with UnsupportedItem::Kind in ir.h.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum UnsupportedItemKind {
    Func,
    GlobalVar,
    Struct,
    Union,
    Class,
    Enum,
    TypeAlias,
    Namespace,
    Constructor,
    // Represents: Comment, Type Map (crubit_internal_rust_type),
    // Use Mod, Hard Error in c++.
    Other,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UnsupportedItemPath<'pb> {
    pub(crate) ident: UnqualifiedIdentifier<'pb>,
    pub(crate) enclosing_item_id: Option<ItemId>,
}

impl<'pb> UnsupportedItemPath<'pb> {
    pub fn ident(&self) -> &UnqualifiedIdentifier<'pb> {
        &self.ident
    }

    pub fn enclosing_item_id(&self) -> Option<ItemId> {
        self.enclosing_item_id
    }
}

#[derive(Clone)]
pub struct UnsupportedItem<'pb> {
    pub(crate) name: Rc<str>,
    pub(crate) unique_name: Option<&'pb str>,
    pub(crate) kind: UnsupportedItemKind,
    pub(crate) path: Option<UnsupportedItemPath<'pb>>,
    pub(crate) errors: Vec<Rc<FormattedError>>,
    pub(crate) source_loc: Option<&'pb str>,
    pub(crate) id: ItemId,
    pub(crate) must_bind: bool,
    pub(crate) defining_target: Option<BazelLabel>,
    pub(crate) inline_cpp_source_text: Option<Rc<str>>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> UnsupportedItem<'pb> {
        /// Unlike other AST nodes that borrow from the protobuf memory, `UnsupportedItem` names are
        /// dynamically formatted during Rust code generation. Storing `Rc<str>` here avoids requiring
        /// unsafe string lifetime extensions at the cost of negligible string allocations.
        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn unique_name(&self) -> Option<&'pb str> {
            self.unique_name
        }

        pub fn kind(&self) -> UnsupportedItemKind {
            self.kind
        }

        pub fn path(&self) -> Option<&UnsupportedItemPath<'pb>> {
            self.path.as_ref()
        }

        pub fn errors(&self) -> &[Rc<FormattedError>] {
            self.errors.as_slice()
        }

        pub fn source_loc(&self) -> Option<&'pb str> {
            self.source_loc
        }

        pub fn inline_cpp_source_text(&self) -> Option<&str> {
            self.inline_cpp_source_text.as_deref()
        }

        pub fn id(&self) -> ItemId {
            self.id
        }

        pub fn must_bind(&self) -> bool {
            self.must_bind
        }

        pub fn defining_target(&self) -> Option<&BazelLabel> {
            self.defining_target.as_ref()
        }
    }
}

impl<'pb> UnsupportedItem<'pb> {
    pub fn source_text_as_token_stream(&self) -> Option<proc_macro2::TokenStream> {
        self.inline_cpp_source_text()?.parse::<proc_macro2::TokenStream>().ok()
    }

    pub fn set_inline_cpp_source_text(&mut self, text: Option<Rc<str>>) {
        self.inline_cpp_source_text = text;
    }
}

impl<'pb> GenericItem<'pb> for UnsupportedItem<'pb> {
    fn id(&self) -> ItemId {
        UnsupportedItem::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        self.unique_name()
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        None
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        self.kind
    }
    fn source_loc(&self) -> Option<&'pb str> {
        self.source_loc()
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        None
    }
    fn must_bind(&self) -> bool {
        UnsupportedItem::must_bind(self)
    }
}

impl<'pb> UnsupportedItem<'pb> {
    #[allow(clippy::too_many_arguments)]
    pub fn new_raw(
        name: impl Into<Rc<str>>,
        unique_name: Option<&'pb str>,
        kind: UnsupportedItemKind,
        id: ItemId,
        source_loc: Option<&'pb str>,
        defining_target: Option<BazelLabel>,
        must_bind: bool,
        path: Option<UnsupportedItemPath<'pb>>,
        error: Option<Rc<FormattedError>>,
    ) -> Self {
        Self {
            name: name.into(),
            unique_name,
            errors: error.into_iter().collect(),
            kind,
            path,
            source_loc,
            id,
            must_bind,
            defining_target,
            inline_cpp_source_text: None,
        }
    }
}

#[derive(Clone)]
pub struct Comment<'pb> {
    pub(crate) proto: CommentView<'pb>,
    pub(crate) text: &'pb str,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> Comment<'pb> {
        pub fn text(&self) -> &'pb str {
            self.text
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }
    }
}

impl<'pb> GenericItem<'pb> for Comment<'pb> {
    fn id(&self) -> ItemId {
        Comment::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        None
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        None
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Other
    }
    fn source_loc(&self) -> Option<&'pb str> {
        None
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        None
    }
    fn must_bind(&self) -> bool {
        Comment::must_bind(self)
    }
}

#[derive(Clone)]
pub struct Namespace<'pb> {
    pub(crate) proto: NamespaceView<'pb>,
    pub(crate) cc_name: Identifier<'pb>,
    pub(crate) rs_name: Identifier<'pb>,
    pub(crate) unique_name: &'pb str,
    pub(crate) unknown_attr: Option<&'pb str>,
    pub(crate) owning_target: BazelLabel,
    pub(crate) deprecated: Option<&'pb str>,
    pub(crate) doc_comment: Option<&'pb str>,
    pub(crate) children: Vec<Item<'pb>>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> Namespace<'pb> {
        pub fn cc_name(&self) -> &Identifier<'pb> {
            &self.cc_name
        }

        pub fn rs_name(&self) -> &Identifier<'pb> {
            &self.rs_name
        }

        pub fn unique_name(&self) -> &'pb str {
            self.unique_name
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn canonical_namespace_id(&self) -> ItemId {
            ItemId(self.proto.canonical_namespace_id() as usize)
        }

        /// A human-readable list of attributes that Crubit doesn't understand.
        pub fn unknown_attr(&self) -> Option<&'pb str> {
            self.unknown_attr
        }

        pub fn owning_target(&self) -> &BazelLabel {
            &self.owning_target
        }

        pub fn enclosing_item_id(&self) -> Option<ItemId> {
            Into::<Option<i64>>::into(self.proto.enclosing_item_id_opt()).map(|id| ItemId(id as usize))
        }

        pub fn is_inline(&self) -> bool {
            self.proto.is_inline()
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }

        pub fn deprecated(&self) -> Option<&'pb str> {
            self.deprecated
        }

        pub fn doc_comment(&self) -> Option<&'pb str> {
            self.doc_comment
        }

        pub fn children(&self) -> &[Item<'pb>] {
            &self.children
        }
    }
}

impl<'pb> Namespace<'pb> {
    pub fn children_mut(&mut self) -> &mut Vec<Item<'pb>> {
        &mut self.children
    }

    pub fn set_children(&mut self, children: Vec<Item<'pb>>) {
        self.children = children;
    }
}

impl<'pb> GenericItem<'pb> for Namespace<'pb> {
    fn id(&self) -> ItemId {
        Namespace::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target().clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Namespace
    }
    fn source_loc(&self) -> Option<&'pb str> {
        None
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        self.unknown_attr()
    }
    fn must_bind(&self) -> bool {
        Namespace::must_bind(self)
    }
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        Some(self.cc_name.as_str())
    }
}

#[derive(Clone)]
pub struct UseMod<'pb> {
    pub(crate) proto: UseModView<'pb>,
    pub(crate) path: Rc<str>,
    pub(crate) mod_name: Identifier<'pb>,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> UseMod<'pb> {
        pub fn path(&self) -> &str {
            &self.path
        }

        pub fn mod_name(&self) -> &Identifier<'pb> {
            &self.mod_name
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }
    }
}

impl<'pb> GenericItem<'pb> for UseMod<'pb> {
    fn id(&self) -> ItemId {
        UseMod::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        None
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        None
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Other
    }
    fn source_loc(&self) -> Option<&'pb str> {
        None
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        None
    }
    fn must_bind(&self) -> bool {
        UseMod::must_bind(self)
    }
}

/// A C++ type annotated with CRUBIT_INTERNAL_RUST_TYPE, indicating that Crubit should use the
/// existing Rust type instead of generating a new Rust type. Note that this corresponds to concrete
/// types, meaning non-template types or template instantiations, but not uninstantiated template
/// declarations.
#[derive(Clone)]
pub struct ExistingRustType<'pb> {
    pub(crate) proto: ExistingRustTypeView<'pb>,
    pub(crate) rs_name: &'pb str,
    pub(crate) cc_name: &'pb str,
    pub(crate) unique_name: &'pb str,
    pub(crate) template_args: Vec<TemplateArg>,
    pub(crate) owning_target: BazelLabel,
}

derive_debug_partialeq_eq_hash! {
    impl<'pb> ExistingRustType<'pb> {
        /// The name of the existing Rust type.
        /// Note that it may contain interpolated type parameters, like `RustType<{T}>`.
        /// This means that it's incorrect to directly parse as an Ident.
        pub fn rs_name(&self) -> &'pb str {
            self.rs_name
        }

        pub fn cc_name(&self) -> &'pb str {
            self.cc_name
        }

        pub fn unique_name(&self) -> &'pb str {
            self.unique_name
        }

        /// The template arguments on this instance of the type instantiation (empty is no template
        /// arguments). This list parallels `template_arg_names`.
        pub fn template_args(&self) -> &[TemplateArg] {
            &self.template_args
        }

        pub fn owning_target(&self) -> &BazelLabel {
            &self.owning_target
        }

        pub fn size_align(&self) -> Option<SizeAlign<'pb>> {
            self.proto.has_size_align().then(|| SizeAlign(self.proto.size_align()))
        }

        pub fn is_same_abi(&self) -> bool {
            self.proto.is_same_abi()
        }

        pub fn id(&self) -> ItemId {
            ItemId(self.proto.id() as usize)
        }

        pub fn must_bind(&self) -> bool {
            self.proto.must_bind()
        }

        pub fn impl_debug(&self) -> bool {
            self.proto.impl_debug()
        }
    }
}

impl<'pb> GenericItem<'pb> for ExistingRustType<'pb> {
    fn id(&self) -> ItemId {
        ExistingRustType::id(self)
    }
    fn unique_name(&self) -> Option<&'pb str> {
        Some(self.unique_name())
    }
    fn owning_target(&self) -> Option<BazelLabel> {
        Some(self.owning_target.clone())
    }
    fn unsupported_kind(&self) -> UnsupportedItemKind {
        UnsupportedItemKind::Other
    }
    fn source_loc(&self) -> Option<&'pb str> {
        None
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
        None
    }
    fn must_bind(&self) -> bool {
        ExistingRustType::must_bind(self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Item<'pb> {
    Func(Rc<Func<'pb>>),
    IncompleteRecord(Rc<IncompleteRecord<'pb>>),
    Record(Rc<Record<'pb>>),
    Enum(Rc<Enum<'pb>>),
    Constant(Rc<Constant<'pb>>),
    GlobalVar(Rc<GlobalVar<'pb>>),
    TypeAlias(Rc<TypeAlias<'pb>>),
    UnsupportedItem(Rc<UnsupportedItem<'pb>>),
    Comment(Rc<Comment<'pb>>),
    Namespace(Rc<Namespace<'pb>>),
    UseMod(Rc<UseMod<'pb>>),
    ExistingRustType(Rc<ExistingRustType<'pb>>),
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

impl<'pb> GenericItem<'pb> for Item<'pb> {
    fn id(&self) -> ItemId {
        forward_item! {
            match self {
                _(x) => x.id()
            }
        }
    }
    fn unique_name(&self) -> Option<&'pb str> {
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
    fn source_loc(&self) -> Option<&'pb str> {
        forward_item! {
            match self {
                _(x) => x.source_loc()
            }
        }
    }
    fn unknown_attr(&self) -> Option<&'pb str> {
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
    fn cc_name_as_str(&self) -> Option<&'pb str> {
        forward_item! {
            match self {
                _(x) => x.cc_name_as_str()
            }
        }
    }
}

impl<'pb> Item<'pb> {
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
                unsupported.path.as_ref().and_then(|p| p.enclosing_item_id)
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
    pub fn cc_name_as_str(&self) -> Option<&'pb str> {
        match self {
            Item::Func(func) => match &func.cc_name {
                UnqualifiedIdentifier::Identifier(identifier) => Some(identifier.as_str()),
                _ => None,
            },
            Item::IncompleteRecord(incomplete_record) => Some(incomplete_record.cc_name.as_str()),
            Item::Record(record) => Some(record.cc_name.as_str()),
            Item::Enum(enum_) => Some(enum_.cc_name.as_str()),
            Item::Constant(constant) => Some(constant.cc_name.as_str()),
            Item::GlobalVar(global_var) => Some(global_var.cc_name.as_str()),
            Item::TypeAlias(type_alias) => Some(type_alias.cc_name.as_str()),
            Item::Namespace(namespace) => Some(namespace.cc_name.as_str()),
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
            Item::IncompleteRecord(incomplete_record) => Some(incomplete_record.rs_name.as_str()),
            Item::Record(record) => Some(record.rs_name.as_str()),
            Item::Enum(enum_) => Some(enum_.rs_name.as_str()),
            Item::Constant(constant) => Some(constant.rs_name.as_str()),
            Item::GlobalVar(global_var) => Some(global_var.rs_name.as_str()),
            Item::TypeAlias(type_alias) => Some(type_alias.rs_name.as_str()),
            Item::UnsupportedItem(_) => None,
            Item::Comment(_) => None,
            Item::Namespace(namespace) => Some(namespace.rs_name.as_str()),
            Item::UseMod(use_mod) => Some(use_mod.mod_name.as_str()),
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
    pub fn place_in_nested_module_if_nested_in_record(&self) -> bool {
        match self {
            Item::IncompleteRecord(_)
            | Item::Record(_)
            | Item::GlobalVar(_)
            | Item::TypeAlias(_)
            | Item::Enum(_)
            | Item::Constant(_)
            | Item::UseMod(_)
            | Item::ExistingRustType(_) => true,
            Item::Func(_) | Item::UnsupportedItem(_) | Item::Comment(_) => false,
            Item::Namespace(_) => unreachable!("Found a namespace that's opened inside of a record. This is not valid C++, so this is a bug."),
        }
    }
}

impl<'pb> From<Func<'pb>> for Item<'pb> {
    fn from(func: Func<'pb>) -> Item<'pb> {
        Item::Func(Rc::new(func))
    }
}

impl<'a, 'pb> TryFrom<&'a Item<'pb>> for &'a Rc<Func<'pb>> {
    type Error = Error;
    fn try_from(value: &'a Item<'pb>) -> Result<Self, Self::Error> {
        if let Item::Func(f) = value {
            Ok(f)
        } else {
            bail!("Not a Func: {:#?}", value)
        }
    }
}

impl<'pb> From<Record<'pb>> for Item<'pb> {
    fn from(record: Record<'pb>) -> Item<'pb> {
        Item::Record(Rc::new(record))
    }
}

impl<'a, 'pb> TryFrom<&'a Item<'pb>> for &'a Rc<Record<'pb>> {
    type Error = Error;
    fn try_from(value: &'a Item<'pb>) -> Result<Self, Self::Error> {
        if let Item::Record(r) = value {
            Ok(r)
        } else {
            bail!("Not a Record: {:#?}", value)
        }
    }
}

impl<'pb> From<UnsupportedItem<'pb>> for Item<'pb> {
    fn from(unsupported: UnsupportedItem<'pb>) -> Item<'pb> {
        Item::UnsupportedItem(Rc::new(unsupported))
    }
}

impl<'a, 'pb> TryFrom<&'a Item<'pb>> for &'a Rc<UnsupportedItem<'pb>> {
    type Error = Error;
    fn try_from(value: &'a Item<'pb>) -> Result<Self, Self::Error> {
        if let Item::UnsupportedItem(u) = value {
            Ok(u)
        } else {
            bail!("Not an UnsupportedItem: {:#?}", value)
        }
    }
}

impl<'pb> From<Comment<'pb>> for Item<'pb> {
    fn from(comment: Comment<'pb>) -> Item<'pb> {
        Item::Comment(Rc::new(comment))
    }
}

impl<'a, 'pb> TryFrom<&'a Item<'pb>> for &'a Rc<Comment<'pb>> {
    type Error = Error;
    fn try_from(value: &'a Item<'pb>) -> Result<Self, Self::Error> {
        if let Item::Comment(c) = value {
            Ok(c)
        } else {
            bail!("Not a Comment: {:#?}", value)
        }
    }
}

impl<'pb> From<Namespace<'pb>> for Item<'pb> {
    fn from(ns: Namespace<'pb>) -> Item<'pb> {
        Item::Namespace(Rc::new(ns))
    }
}

impl<'a, 'pb> TryFrom<&'a Item<'pb>> for &'a Rc<Namespace<'pb>> {
    type Error = Error;
    fn try_from(value: &'a Item<'pb>) -> Result<Self, Self::Error> {
        if let Item::Namespace(c) = value {
            Ok(c)
        } else {
            bail!("Not a Namespace: {:#?}", value)
        }
    }
}

impl<'pb> From<ExistingRustType<'pb>> for Item<'pb> {
    fn from(existing_rust_type: ExistingRustType<'pb>) -> Item<'pb> {
        Item::ExistingRustType(Rc::new(existing_rust_type))
    }
}

impl<'a, 'pb> TryFrom<&'a Item<'pb>> for &'a Rc<ExistingRustType<'pb>> {
    type Error = Error;
    fn try_from(value: &'a Item<'pb>) -> Result<Self, Self::Error> {
        if let Item::ExistingRustType(r) = value {
            Ok(r)
        } else {
            bail!("Not an ExistingRustType: {:#?}", value)
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct TreeIR<'pb> {
    pub public_headers: Vec<HeaderName<'pb>>,
    pub current_target: BazelLabel,
    pub crate_root_path: Option<Rc<str>>,
    pub crubit_features: BTreeMap<BazelLabel, crubit_feature::SerializedCrubitFeatures>,
    pub crate_names: BTreeMap<BazelLabel, Ident>,
    pub unstable_rust_features: Vec<String>,
    pub reexported_namespaces: Vec<Rc<str>>,
    pub top_level_items: BTreeMap<BazelLabel, Vec<Item<'pb>>>,
}

/// A custom debug impl that wraps the HashMap in rustfmt-friendly notation.
///
/// See b/272530008.
impl<'pb> Debug for TreeIR<'pb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // BTreeMap has consistent ordering, unlike HashMap, so it's reasonable to rely on a
        // consistent Debug output.
        struct DebugBTreeMap<T>(pub T);

        // Format as `[ (k, v) ]` instead of `map! { k: v }` because rustfmt fails on macros
        // with complicated contents (our nested items) which it cannot verify is valid Rust. This
        // ensures the output is parse-able for rustfmt on test failures. See b/272530008.
        impl<K: Debug, V: Debug> Debug for DebugBTreeMap<&BTreeMap<K, V>> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self.0.iter()).finish()
            }
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
            .field("crubit_features", &DebugBTreeMap(crubit_features))
            .field("crate_names", &DebugBTreeMap(crate_names))
            .field("unstable_rust_features", unstable_rust_features)
            .field("reexported_namespaces", reexported_namespaces)
            .field("top_level_items", &DebugBTreeMap(top_level_items))
            .finish()
    }
}

#[derive(Debug)]
pub struct IR<'pb> {
    tree_ir: TreeIR<'pb>,
    item_id_to_item: HashMap<ItemId, Item<'pb>>,
    lifetimes: HashMap<LifetimeId, LifetimeName>,
    namespace_id_to_number_of_reopened_namespaces: HashMap<ItemId, usize>,
    reopened_namespace_id_to_idx: HashMap<ItemId, usize>,
    function_name_to_functions: HashMap<UnqualifiedIdentifier<'pb>, Vec<Rc<Func<'pb>>>>,
}

impl<'pb> PartialEq for IR<'pb> {
    fn eq(&self, other: &Self) -> bool {
        self.tree_ir == other.tree_ir
            && self.item_id_to_item == other.item_id_to_item
            && self.lifetimes == other.lifetimes
            && self.namespace_id_to_number_of_reopened_namespaces
                == other.namespace_id_to_number_of_reopened_namespaces
            && self.reopened_namespace_id_to_idx == other.reopened_namespace_id_to_idx
            && self.function_name_to_functions == other.function_name_to_functions
    }
}

impl<'pb> Eq for IR<'pb> {}

impl<'pb> IR<'pb> {
    pub fn tree_ir(&self) -> &TreeIR<'pb> {
        &self.tree_ir
    }

    pub fn unstable_rust_features(&self) -> &[String] {
        &self.tree_ir.unstable_rust_features
    }

    pub fn get_decl(&self, id: ItemId) -> Option<&Item<'pb>> {
        self.item_id_to_item.get(&id)
    }

    pub fn items(&self) -> impl Iterator<Item = &Item<'pb>> + '_ {
        let roots = self.tree_ir.top_level_items.values().flat_map(|v| v.iter());
        ItemsIterator::new(roots.collect())
    }

    pub fn lifetimes(&self) -> impl Iterator<Item = (&LifetimeId, &LifetimeName)> {
        self.lifetimes.iter()
    }

    /// Returns the top-level items of a target.
    pub fn top_level_items_in_target(&self, target: &BazelLabel) -> &[Item<'pb>] {
        self.tree_ir.top_level_items.get(target).map(|v| v.as_slice()).unwrap_or_default()
    }

    pub fn top_level_items(&self) -> &[Item<'pb>] {
        self.top_level_items_in_target(self.current_target())
    }

    pub fn reexported_namespaces(&self) -> &[Rc<str>] {
        &self.tree_ir.reexported_namespaces
    }

    pub fn public_headers(&self) -> impl Iterator<Item = &HeaderName<'pb>> {
        self.tree_ir.public_headers.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &Rc<Func<'pb>>> {
        self.items().filter_map(|item| match item {
            Item::Func(func) => Some(func),
            _ => None,
        })
    }

    pub fn type_aliases(&self) -> impl Iterator<Item = &Rc<TypeAlias<'pb>>> {
        self.items().filter_map(|item| match item {
            Item::TypeAlias(type_alias) => Some(type_alias),
            _ => None,
        })
    }

    pub fn records(&self) -> impl Iterator<Item = &Rc<Record<'pb>>> {
        self.items().filter_map(|item| match item {
            Item::Record(record) => Some(record),
            _ => None,
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = &Rc<Enum<'pb>>> {
        self.items().filter_map(|item| match item {
            Item::Enum(enum_item) => Some(enum_item),
            _ => None,
        })
    }

    pub fn constants(&self) -> impl Iterator<Item = &Rc<Constant<'pb>>> {
        self.items().filter_map(|item| match item {
            Item::Constant(constant) => Some(constant),
            _ => None,
        })
    }

    pub fn unsupported_items(&self) -> impl Iterator<Item = &Rc<UnsupportedItem<'pb>>> {
        self.items().filter_map(|item| match item {
            Item::UnsupportedItem(unsupported_item) => Some(unsupported_item),
            _ => None,
        })
    }

    pub fn comments(&self) -> impl Iterator<Item = &Rc<Comment<'pb>>> {
        self.items().filter_map(|item| match item {
            Item::Comment(comment) => Some(comment),
            _ => None,
        })
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &Rc<Namespace<'pb>>> {
        self.items().filter_map(|item| match item {
            Item::Namespace(ns) => Some(ns),
            _ => None,
        })
    }

    pub fn existing_rust_types(&self) -> impl Iterator<Item = &Rc<ExistingRustType<'pb>>> {
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
        function_name: &UnqualifiedIdentifier<'pb>,
    ) -> impl Iterator<Item = &Rc<Func<'pb>>> + '_ {
        self.function_name_to_functions.get(function_name).map_or([].iter(), |v| v.iter())
    }
}

// TODO(jeanpierreda): This should probably be a method on IR accepting a GenericItem,
// and returning the crate name, or similar.

/// Returns Some(crate_ident) if this is an imported crate.
pub fn rs_imported_crate_name(owning_target: &BazelLabel, ir: &IR<'_>) -> Option<Ident> {
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
        let proto = protobuf::proto!(ir_rust_proto::Identifier { identifier: "hello" });
        let identifier = Identifier::try_from(proto.as_view()).unwrap();
        assert_eq!(format!("{identifier:?}"), "\"hello\"");
    }

    #[gtest]
    fn test_unqualified_identifier_debug_print() {
        let proto = protobuf::proto!(ir_rust_proto::Identifier { identifier: "hello" });
        let identifier = Identifier::try_from(proto.as_view()).unwrap();
        assert_eq!(format!("{:?}", UnqualifiedIdentifier::Identifier(identifier)), "\"hello\"");
        assert_eq!(format!("{:?}", UnqualifiedIdentifier::Constructor), "Constructor");
        assert_eq!(format!("{:?}", UnqualifiedIdentifier::Destructor), "Destructor");
    }

    #[gtest]
    fn test_used_headers() {
        let proto = protobuf::proto!(IRProto {
            public_headers: [__ { name: "foo/bar.h" }],
            current_target: "//foo:bar",
        });
        let ir = proto_to_ir(proto.as_view()).unwrap();
        let expected = TreeIR {
            public_headers: vec![HeaderName { name: "foo/bar.h".into() }],
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
        let proto = protobuf::proto!(IRProto { current_target: "//foo:bar" });
        let ir = proto_to_ir(proto.as_view()).unwrap();
        assert_eq!(ir.crate_root_path(), None);
    }

    #[gtest]
    fn test_crate_root_path() {
        let proto = protobuf::proto!(IRProto {
            crate_root_path: "__cc_template_instantiations_rs_api",
            current_target: "//foo:bar",
        });
        let ir = proto_to_ir(proto.as_view()).unwrap();
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
        let proto = protobuf::proto!(IRProto {
            current_target: "//foo:bar",
            top_level_items: [(
                "//foo:bar",
                __ {
                    items: [__ {
                        namespace_decl: __ {
                            cc_name: __ { identifier: "nsA" },
                            rs_name: __ { identifier: "nsA" },
                            unique_name: "nsA",
                            id: 1,
                            canonical_namespace_id: 1,
                            owning_target: "//foo:bar",
                            is_inline: false,
                            must_bind: false,
                            children: [__ {
                                namespace_decl: __ {
                                    cc_name: __ { identifier: "nsB" },
                                    rs_name: __ { identifier: "nsB" },
                                    unique_name: "nsB",
                                    id: 2,
                                    canonical_namespace_id: 2,
                                    owning_target: "//foo:bar",
                                    is_inline: false,
                                    must_bind: false,
                                    children: [__ {
                                        comment: __ { text: "hello", id: 3, must_bind: false }
                                    }]
                                }
                            }]
                        }
                    }]
                }
            )]
        });
        let ir = proto_to_ir(proto.as_view()).unwrap();

        let ids: Vec<_> = ir.items().map(|item| item.id().0).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[gtest]
    fn test_make_ir_redeclarations() {
        let proto = protobuf::proto!(IRProto {
            current_target: "//foo:bar",
            top_level_items: [(
                "//foo:bar",
                __ {
                    items: [
                        __ {
                            namespace_decl: __ {
                                cc_name: __ { identifier: "ns1" },
                                rs_name: __ { identifier: "ns1" },
                                unique_name: "ns1",
                                id: 100,
                                canonical_namespace_id: 100,
                                owning_target: "//foo:bar",
                                is_inline: false,
                                must_bind: false,
                                children: [__ {
                                    comment: __ { text: "hello", id: 200, must_bind: false }
                                }]
                            }
                        },
                        __ {
                            namespace_decl: __ {
                                cc_name: __ { identifier: "ns1" },
                                rs_name: __ { identifier: "ns1" },
                                unique_name: "ns1",
                                id: 101,
                                canonical_namespace_id: 100,
                                owning_target: "//foo:bar",
                                is_inline: false,
                                must_bind: false,
                                children: [__ {
                                    comment: __ { text: "hello", id: 200, must_bind: false }
                                }]
                            }
                        }
                    ]
                }
            )]
        });
        let ir = proto_to_ir(proto.as_view()).unwrap();

        let comment_items: Vec<_> = ir.items().filter(|item| item.id() == ItemId(200)).collect();

        assert_eq!(comment_items.len(), 1);
    }

    #[gtest]
    fn test_proto_crate_names() {
        let proto = protobuf::proto!(IRProto {
            current_target: "//foo:bar",
            crate_names: [("//dep:target", "custom_crate")]
        });
        let ir = proto_to_ir(proto.as_view()).unwrap();
        assert_eq!(
            ir.crate_name(&"//dep:target".into()).map(|i| i.to_string()),
            Some("custom_crate".to_string())
        );
    }

    #[gtest]
    fn test_proto_crate_names_invalid_ident() {
        let proto = protobuf::proto!(IRProto {
            current_target: "//foo:bar",
            crate_names: [("//dep:target", "invalid*crate")]
        });
        let result = proto_to_ir(proto.as_view());
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
        let proto = protobuf::proto!(IRProto {
            current_target: "//foo:bar",
            crate_names: [("//dep:target", "custom_crate")],
        });
        let ir = proto_to_ir(proto.as_view()).unwrap();
        let crate_ident = rs_imported_crate_name(&"//dep:target".into(), &ir).unwrap();
        assert_eq!(crate_ident.to_string(), "custom_crate");
    }

    #[gtest]
    fn test_rs_imported_crate_name_without_custom_name() {
        let proto = protobuf::proto!(IRProto { current_target: "//foo:bar" });
        let ir = proto_to_ir(proto.as_view()).unwrap();
        let crate_ident = rs_imported_crate_name(&"//dep:target".into(), &ir).unwrap();
        assert_eq!(crate_ident.to_string(), "target");
    }
}
