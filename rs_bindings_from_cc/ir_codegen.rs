// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Code generator for `#[repr(C)]` IR types shared in-memory between C++ (`ir_generated.h`)
//! and Rust (`ir_generated.rs`).
//!
//! Run `common/golden_update.sh` after modifying the schema in this file.

use genco::fmt;
use genco::lang::{c, rust};
use genco::quote;
use genco::tokens::quoted;
use heck::ToSnakeCase;
use std::collections::BTreeMap;
use std::env;
use std::fs;

#[derive(Clone, Debug)]
enum FieldType {
    Unit,
    Bool,
    I32,
    U32,
    I64,
    U64,
    Usize,
    Str,
    Bytes,
    Newtype(&'static str, Box<FieldType>),
    Enum(&'static str),
    Struct(&'static str),
    Box(Box<FieldType>),
    Vec(Box<FieldType>),
    Map(Box<FieldType>, Box<FieldType>),
}

impl FieldType {
    fn cc_type(&self) -> &FieldType {
        match self {
            Self::Newtype(_, inner) => inner.cc_type(),
            other => other,
        }
    }

    fn cc_storage_type(&self) -> String {
        match self.cc_type() {
            Self::Unit => unreachable!("Unit has no C++ storage field"),
            Self::Bool => "bool".to_string(),
            Self::I32 => "int32_t".to_string(),
            Self::U32 => "uint32_t".to_string(),
            Self::I64 => "int64_t".to_string(),
            Self::U64 => "uint64_t".to_string(),
            Self::Usize => "size_t".to_string(),
            Self::Str => "FfiString".to_string(),
            Self::Bytes => "FfiVec<uint8_t>".to_string(),
            Self::Newtype(_, _) => unreachable!(),
            Self::Enum(path) | Self::Struct(path) => format!("::crubit::ffi::{path}"),
            Self::Box(inner) => format!("FfiRc<{}>", inner.cc_storage_type()),
            Self::Vec(inner) => format!("FfiVec<{}>", inner.cc_storage_type()),
            Self::Map(k, v) => format!("FfiMap<{}, {}>", k.cc_storage_type(), v.cc_storage_type()),
        }
    }

    fn rs_storage_type(&self, in_submodule: bool) -> String {
        let prefix = if in_submodule { "super::" } else { "" };
        match self {
            Self::Unit => unreachable!("Unit has no Rust storage payload"),
            Self::Bool => "bool".to_string(),
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Usize => "usize".to_string(),
            Self::Str => "FfiString".to_string(),
            Self::Bytes => "FfiVec<u8>".to_string(),
            Self::Newtype(name, _) => (*name).to_string(),
            Self::Enum(path) | Self::Struct(path) => format!("{prefix}{path}"),
            Self::Box(inner) => format!("FfiRc<{}>", inner.rs_storage_type(in_submodule)),
            Self::Vec(inner) => format!("FfiVec<{}>", inner.rs_storage_type(in_submodule)),
            Self::Map(k, v) => format!(
                "FfiMap<{}, {}>",
                k.rs_storage_type(in_submodule),
                v.rs_storage_type(in_submodule)
            ),
        }
    }

    fn rs_getter_return_type(&self, schema: &Schema, in_submodule: bool) -> String {
        let prefix = if in_submodule { "super::" } else { "" };
        match self {
            Self::Unit => unreachable!("Unit is only used in unions"),
            Self::Bool => "bool".to_string(),
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Usize => "usize".to_string(),
            Self::Str => "&str".to_string(),
            Self::Bytes => "&[u8]".to_string(),
            Self::Newtype(name, inner) => {
                if schema.is_copy_type(inner) {
                    (*name).to_string()
                } else {
                    format!("&{name}")
                }
            }
            Self::Enum(path) => format!("{prefix}{path}"),
            Self::Struct("StatusOrOptionalString") => "Result<Option<&str>, &str>".to_string(),
            Self::Struct("decl_location::ByteRange") => "(u32, u32)".to_string(),
            Self::Struct(path) => {
                if schema.is_copy_struct(path) {
                    format!("{prefix}{path}")
                } else {
                    format!("&{prefix}{path}")
                }
            }
            Self::Box(inner) => format!("&{}", inner.rs_storage_type(in_submodule)),
            Self::Vec(inner) => format!("&[{}]", inner.rs_storage_type(in_submodule)),
            Self::Map(k, v) => format!(
                "&FfiMap<{}, {}>",
                k.rs_storage_type(in_submodule),
                v.rs_storage_type(in_submodule)
            ),
        }
    }

    fn rs_empty_expr(&self, default_true: bool, in_submodule: bool) -> String {
        let prefix = if in_submodule { "super::" } else { "" };
        match self {
            Self::Unit => unreachable!("Unit is only used in unions"),
            Self::Bool => {
                if default_true {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Self::I32 | Self::U32 | Self::I64 | Self::U64 | Self::Usize => "0".to_string(),
            Self::Str => "FfiString::EMPTY".to_string(),
            Self::Newtype(name, _) => format!("{name}::EMPTY"),
            Self::Enum(path) | Self::Struct(path) => format!("{prefix}{path}::EMPTY"),
            Self::Box(_) => "FfiRc::EMPTY".to_string(),
            Self::Vec(_) | Self::Bytes => "FfiVec::EMPTY".to_string(),
            Self::Map(_, _) => "FfiMap::EMPTY".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Scope {
    ns: String,
    parent: &'static str,
}

#[derive(Clone, Debug)]
pub(crate) struct ScopedName {
    scope: Option<Scope>,
    name: &'static str,
}

impl ScopedName {
    pub(crate) fn new(name: &'static str, parent: Option<&'static str>) -> Self {
        Self {
            scope: parent.map(|p| Scope {
                ns: p.split("::").map(ToSnakeCase::to_snake_case).collect::<Vec<_>>().join("::"),
                parent: p,
            }),
            name,
        }
    }

    fn ns(&self) -> Option<&str> {
        self.scope.as_ref().map(|s| s.ns.as_str())
    }

    fn cc_qual_name(&self) -> String {
        match &self.scope {
            Some(s) => format!("{}::{}", s.ns, self.name),
            None => self.name.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct EnumVariantDef {
    rs_name: &'static str,
    cc_name: String,
    discriminant: i32,
}

#[derive(Clone, Debug)]
struct EnumDef {
    item: ScopedName,
    variants: Vec<EnumVariantDef>,
}

#[derive(Clone, Debug)]
struct UnionVariantDef {
    pascal_name: &'static str,
    field_name: String,
    ty: FieldType,
}

#[derive(Clone, Debug)]
struct UnionDef {
    item: ScopedName,
    variants: Vec<UnionVariantDef>,
}

#[derive(Clone, Debug)]
struct FieldDef {
    cc_name: &'static str,
    rs_name: &'static str,
    ty: FieldType,
    has_bit: Option<u32>,
    default_true: bool,
}

impl FieldDef {
    fn cc_has_mask(&self) -> Option<String> {
        self.has_bit.map(|bit| format!("0x{:x}ULL", 1u64 << bit))
    }

    fn rs_has_mask(&self) -> Option<String> {
        self.has_bit.map(|bit| format!("0x{:x}", 1u64 << bit))
    }
}

#[derive(Clone, Debug)]
struct StructDef {
    item: ScopedName,
    fields: Vec<FieldDef>,
}

impl StructDef {
    pub(crate) fn new(
        item: ScopedName,
        raw_fields: Vec<(&'static str, FieldType, bool, bool)>,
    ) -> Self {
        let sname = item.name;
        let mut bit_idx = 0u32;
        let fields = raw_fields
            .into_iter()
            .map(|(cc_name, ty, optional, default_true)| {
                let has_bit = if optional {
                    assert!(bit_idx < 64, "Struct {sname} has more than 64 optional fields");
                    let bit = bit_idx;
                    bit_idx += 1;
                    Some(bit)
                } else {
                    None
                };
                FieldDef {
                    cc_name,
                    rs_name: if cc_name == "type" { "type_" } else { cc_name },
                    ty,
                    has_bit,
                    default_true,
                }
            })
            .collect();
        Self { item, fields }
    }

    fn has_optional_fields(&self) -> bool {
        self.fields.iter().any(|f| f.has_bit.is_some())
    }
}

#[derive(Clone, Debug)]
enum RecordDef {
    Union(UnionDef),
    Struct(StructDef),
}

struct Schema {
    enums: Vec<EnumDef>,
    records: Vec<RecordDef>,
}

macro_rules! field_ty {
    (Unit) => {
        FieldType::Unit
    };
    (Bool) => {
        FieldType::Bool
    };
    (I32) => {
        FieldType::I32
    };
    (U32) => {
        FieldType::U32
    };
    (I64) => {
        FieldType::I64
    };
    (U64) => {
        FieldType::U64
    };
    (Usize) => {
        FieldType::Usize
    };
    (ItemId) => {
        FieldType::Newtype("ItemId", Box::new(FieldType::U64))
    };
    (LifetimeId) => {
        FieldType::Newtype("LifetimeId", Box::new(FieldType::I32))
    };
    (BazelLabel) => {
        FieldType::Newtype("BazelLabel", Box::new(FieldType::Str))
    };
    (Str) => {
        FieldType::Str
    };
    (Bytes) => {
        FieldType::Bytes
    };
    (Box($($inner:tt)+)) => {
        FieldType::Box(Box::new(field_ty!($($inner)+)))
    };
    (Vec($($inner:tt)+)) => {
        FieldType::Vec(Box::new(field_ty!($($inner)+)))
    };
    (Map($($k:ident)::+, $($v:tt)+)) => {
        FieldType::Map(
            Box::new(field_ty!($($k)::+)),
            Box::new(field_ty!($($v)+)),
        )
    };
    ($p:ident $(:: $sub:ident)*) => {
        FieldType::Struct(concat!(stringify!($p) $(, "::", stringify!($sub))*))
    };
}

macro_rules! union_variant {
    ($vpascal:ident) => {
        UnionVariantDef {
            pascal_name: stringify!($vpascal),
            field_name: heck::ToSnakeCase::to_snake_case(stringify!($vpascal)),
            ty: FieldType::Unit,
        }
    };
    ($vpascal:ident ( $vfield:ident : $($vty:tt)+ )) => {
        UnionVariantDef {
            pascal_name: stringify!($vpascal),
            field_name: stringify!($vfield).to_string(),
            ty: field_ty!($($vty)+),
        }
    };
    ($vpascal:ident ( $($vty:tt)+ )) => {
        UnionVariantDef {
            pascal_name: stringify!($vpascal),
            field_name: heck::ToSnakeCase::to_snake_case(stringify!($vpascal)),
            ty: field_ty!($($vty)+),
        }
    };
}

macro_rules! ir_schema_item {
    (
        $enums:ident, $records:ident;
        $(#[scope($eparent:literal)])?
        enum $ename:ident {
            $( $ers:ident $( = $ecc:ident )? ),* $(,)?
        }
    ) => {
        $enums.push(EnumDef {
            item: ScopedName::new(stringify!($ename), None $( .or(Some($eparent)) )?),
            variants: [
                $(
                    (
                        stringify!($ers),
                        None::<&'static str> $( .or(Some(stringify!($ecc))) )?,
                    )
                ),*
            ]
            .into_iter()
            .enumerate()
            .map(|(i, (rs_name, cc_override))| EnumVariantDef {
                rs_name,
                cc_name: cc_override
                    .map(str::to_string)
                    .unwrap_or_else(|| heck::ToShoutySnakeCase::to_shouty_snake_case(rs_name)),
                discriminant: i as i32,
            })
            .collect(),
        });
    };
    (
        $enums:ident, $records:ident;
        $(#[scope($uparent:literal)])?
        enum $uname:ident {
            $( $vpascal:ident $( ( $($vinner:tt)+ ) )? ),* $(,)?
        }
    ) => {
        $records.push(RecordDef::Union(UnionDef {
            item: ScopedName::new(stringify!($uname), None $( .or(Some($uparent)) )?),
            variants: vec![
                $( union_variant!($vpascal $( ( $($vinner)+ ) )?) ),*
            ],
        }));
    };
    (
        $enums:ident, $records:ident;
        $(#[scope($sparent:literal)])?
        struct $sname:ident {
            $(
                $(#[optional $( $opt_unused:tt )?])?
                $(#[default_true $( $dt_unused:tt )?])?
                $fcc:ident : $($fty:ident)::+ $( ( $($fargs:tt)* ) )?
            ),* $(,)?
        }
    ) => {
        $records.push(RecordDef::Struct(StructDef::new(
            ScopedName::new(stringify!($sname), None $( .or(Some($sparent)) )?),
            vec![
                $(
                    (
                        stringify!($fcc),
                        field_ty!($($fty)::+ $( ( $($fargs)* ) )?),
                        false $( || { $( $opt_unused )? true } )?,
                        false $( || { $( $dt_unused )? true } )?,
                    )
                ),*
            ],
        )));
    };
}

macro_rules! ir_schema {
    (
        $(
            $(#[$($attr:tt)*])*
            $kind:ident $name:ident {
                $($body:tt)*
            }
        )*
    ) => {{
        let mut enums = Vec::new();
        let mut records = Vec::new();
        $(
            ir_schema_item!(enums, records; $(#[$($attr)*])* $kind $name { $($body)* });
        )*
        let mut schema = Schema { enums, records };
        schema.resolve_types();
        schema
    }};
}

mod ir_schema;

impl RecordDef {
    fn item(&self) -> &ScopedName {
        match self {
            Self::Union(u) => &u.item,
            Self::Struct(st) => &st.item,
        }
    }

    fn scope(&self) -> Option<&Scope> {
        self.item().scope.as_ref()
    }

    fn ns(&self) -> Option<&str> {
        self.item().ns()
    }

    fn name(&self) -> &'static str {
        self.item().name
    }

    fn cc_qual_name(&self) -> String {
        self.item().cc_qual_name()
    }
}

fn align_up(offset: usize, align: usize) -> usize {
    (offset + align - 1) & !(align - 1)
}

impl FieldType {
    /// Resolves named structs and enums in the schema.
    ///
    /// This is essentially rudimentary name resolution. In the schema we write "foo::Bar" as a
    /// string literal and it is unclear whether that is a struct or an enum. This function walks
    /// FieldType and resolves the names to either enums or structs, panicking if a name is not
    /// defined.
    fn resolve(&mut self, enum_names: &[String], record_names: &[String]) {
        match self {
            Self::Struct(path) => {
                if enum_names.iter().any(|name| name == *path) {
                    *self = Self::Enum(path);
                    return;
                }
                if record_names.iter().any(|name| name == *path) {
                    return;
                }
                panic!("Unknown schema type: {path}");
            }
            Self::Box(inner) | Self::Vec(inner) | Self::Newtype(_, inner) => {
                inner.resolve(enum_names, record_names)
            }
            Self::Map(k, v) => {
                k.resolve(enum_names, record_names);
                v.resolve(enum_names, record_names);
            }
            _ => {}
        }
    }
}

/// Byte size and alignment on a 64-bit host (`sizeof(size_t) == 8` / `target_pointer_width =
/// "64"`).
#[derive(Copy, Clone, Debug)]
struct Layout {
    size: usize,
    align: usize,
}

#[derive(Copy, Clone, Debug)]
struct FieldOffset {
    cc_name: &'static str,
    rs_name: &'static str,
    offset: usize,
}

#[derive(Clone, Debug)]
struct StructLayout {
    layout: Layout,
    field_offsets: Vec<FieldOffset>,
}

impl Schema {
    /// Resolves named structs and enums in the schema.
    ///
    /// This is essentially rudimentary name resolution. In the schema we write "foo::Bar" as a
    /// string literal and it is unclear whether that is a struct or an enum. This function collects
    /// all names in the schema and uses them to resolve the names in each FieldType.
    fn resolve_types(&mut self) {
        let enum_names: Vec<String> = self.enums.iter().map(|e| e.item.cc_qual_name()).collect();
        let record_names: Vec<String> = self.records.iter().map(RecordDef::cc_qual_name).collect();
        for rec in &mut self.records {
            match rec {
                RecordDef::Union(u) => {
                    for v in &mut u.variants {
                        v.ty.resolve(&enum_names, &record_names);
                    }
                }
                RecordDef::Struct(st) => {
                    for f in &mut st.fields {
                        f.ty.resolve(&enum_names, &record_names);
                    }
                }
            }
        }
    }

    fn is_copy_type(&self, ty: &FieldType) -> bool {
        match ty.cc_type() {
            FieldType::Bool
            | FieldType::I32
            | FieldType::U32
            | FieldType::I64
            | FieldType::U64
            | FieldType::Usize
            | FieldType::Enum(_) => true,
            FieldType::Struct(path) => self.is_copy_struct(path),
            _ => false,
        }
    }

    fn is_copy_struct(&self, path: &str) -> bool {
        let rec = self
            .records
            .iter()
            .find(|r| r.cc_qual_name() == path)
            .unwrap_or_else(|| panic!("Unknown record path: {path}"));
        match rec {
            RecordDef::Union(_) => false,
            RecordDef::Struct(st) => st.fields.iter().all(|f| self.is_copy_type(&f.ty)),
        }
    }

    fn is_field_forward_ref(&self, current_record_idx: usize, ty: &FieldType) -> bool {
        match ty.cc_type() {
            FieldType::Struct(target_path) => {
                let target_idx = self
                    .records
                    .iter()
                    .position(|r| r.cc_qual_name() == *target_path)
                    .unwrap_or_else(|| panic!("Unknown record path: {target_path}"));
                target_idx >= current_record_idx
            }
            FieldType::Box(inner) | FieldType::Vec(inner) => {
                self.is_field_forward_ref(current_record_idx, inner)
            }
            FieldType::Map(k, v) => {
                self.is_field_forward_ref(current_record_idx, k)
                    || self.is_field_forward_ref(current_record_idx, v)
            }
            _ => false,
        }
    }

    fn has_forward_ref(&self, current_record_idx: usize) -> bool {
        match &self.records[current_record_idx] {
            RecordDef::Union(_) => false,
            RecordDef::Struct(st) => {
                st.fields.iter().any(|f| self.is_field_forward_ref(current_record_idx, &f.ty))
            }
        }
    }

    fn is_union_type(&self, ty: &FieldType) -> bool {
        if let FieldType::Struct(path) = ty {
            self.records
                .iter()
                .any(|r| matches!(r, RecordDef::Union(u) if u.item.cc_qual_name() == *path))
        } else {
            false
        }
    }

    fn field_layout(&self, ty: &FieldType) -> Layout {
        match ty.cc_type() {
            FieldType::Unit => Layout { size: 0, align: 1 },
            FieldType::Bool => Layout { size: 1, align: 1 },
            FieldType::I32 | FieldType::U32 | FieldType::Enum(_) => Layout { size: 4, align: 4 },
            FieldType::I64 | FieldType::U64 | FieldType::Usize | FieldType::Box(_) => {
                Layout { size: 8, align: 8 }
            }
            FieldType::Str => Layout { size: 16, align: 8 },
            FieldType::Vec(_) | FieldType::Bytes | FieldType::Map(_, _) => {
                Layout { size: 24, align: 8 }
            }
            FieldType::Newtype(_, _) => unreachable!(),
            FieldType::Struct(path) => {
                let rec = self
                    .records
                    .iter()
                    .find(|r| r.cc_qual_name() == *path)
                    .unwrap_or_else(|| panic!("Unknown record path: {path}"));
                match rec {
                    RecordDef::Union(u) => self.union_layout(u),
                    RecordDef::Struct(st) => self.struct_layout(st).layout,
                }
            }
        }
    }

    fn union_layout(&self, u: &UnionDef) -> Layout {
        let mut union_size = 1usize;
        let mut union_align = 1usize;
        for v in &u.variants {
            let v_layout = self.field_layout(&v.ty);
            union_size = union_size.max(v_layout.size);
            union_align = union_align.max(v_layout.align);
        }
        union_size = align_up(union_size, union_align);
        let total_align = 4usize.max(union_align);
        let union_offset = align_up(4, union_align);
        let total_size = align_up(union_offset + union_size, total_align);
        Layout { size: total_size, align: total_align }
    }

    fn struct_layout(&self, st: &StructDef) -> StructLayout {
        let (mut offset, mut max_align) =
            if st.has_optional_fields() { (8usize, 8usize) } else { (0usize, 1usize) };
        let mut field_offsets = Vec::new();
        for f in &st.fields {
            let f_layout = self.field_layout(&f.ty);
            offset = align_up(offset, f_layout.align);
            field_offsets.push(FieldOffset { cc_name: f.cc_name, rs_name: f.rs_name, offset });
            offset += f_layout.size;
            max_align = max_align.max(f_layout.align);
        }
        let total_size = align_up(offset, max_align);
        StructLayout { layout: Layout { size: total_size, align: max_align }, field_offsets }
    }
}

fn generate_cc_nested_aliases(schema: &Schema, parent_key: &str) -> c::Tokens {
    quote! {
        $(for child_rec in &schema.records join ($['\r']) {
            $(if let Some(scope) = child_rec.scope() {
                $(if scope.parent == parent_key {
                    using $(child_rec.name()) = ::crubit::ffi::$(&scope.ns)::$(child_rec.name());
                })
            })
        })
        $(for child_en in &schema.enums join ($['\r']) {
            $(if let Some(scope) = &child_en.item.scope {
                $(if scope.parent == parent_key {
                    $(let child_ns = &scope.ns)
                    $(let ename = child_en.item.name)
                    using $ename = ::crubit::ffi::$child_ns::$ename;
                    $(for v in &child_en.variants join ($['\r']) =>
                        static constexpr $ename $(&v.cc_name) = ::crubit::ffi::$child_ns::$(&v.cc_name);
                    )
                })
            })
        })
    }
}

fn generate_cc_union(schema: &Schema, u: &UnionDef) -> c::Tokens {
    let parent_key = u.item.cc_qual_name();
    let uname = u.item.name;
    let v0 = &u.variants[0];
    let v0p = v0.pascal_name;
    let v0f = &v0.field_name;
    let v0_init = if matches!(v0.ty, FieldType::Unit) {
        "empty_(0)".to_string()
    } else {
        format!("{v0f}_()")
    };

    quote! {
        $(if let Some(ns) = u.item.ns() => namespace $ns $("{"))
        struct ABSL_ATTRIBUTE_TRIVIAL_ABI $uname {
            $(generate_cc_nested_aliases(schema, &parent_key))
            enum Case : uint32_t {
                $(for (idx, v) in u.variants.iter().enumerate() join ($['\r']) =>
                    k$(v.pascal_name) = $idx,
                )
            };
            $uname() : tag_(k$v0p), payload_() {}
            $uname(const $uname& other) : tag_(k$v0p), payload_() { CopyFrom(other); }
            $uname($uname&& other) noexcept : tag_(k$v0p), payload_() { MoveFrom(std::move(other)); }
            $uname& operator=(const $uname& other) {
                if (this != &other) { $uname tmp(other); MoveFrom(std::move(tmp)); }
                return *this;
            }
            $uname& operator=($uname&& other) noexcept {
                if (this != &other) { MoveFrom(std::move(other)); }
                return *this;
            }
            ~$uname() { Destroy(); }
            $(for v in &u.variants join ($['\r']) {
                $(let vf = &v.field_name)
                $(let vp = v.pascal_name)
                $(match &v.ty {
                    FieldType::Unit => {
                        static $uname $vp() { $uname res; res.set_$vf(); return res; }
                    }
                    FieldType::Struct(cc_path) => {
                        $(let cty = &format!("::crubit::ffi::{cc_path}"))
                        $("// NOLINTNEXTLINE(google-explicit-constructor)")
                        $uname($cty value) : tag_(k$vp), payload_(k$vp) { new (&payload_.$(vf)_) $cty(std::move(value)); }
                    }
                    FieldType::Box(inner) => {
                        $(let inner_cty = &inner.cc_storage_type())
                        $("// NOLINTNEXTLINE(google-explicit-constructor)")
                        $uname($inner_cty value) : tag_(k$vp), payload_(k$vp) { new (&payload_.$(vf)_) FfiRc<$inner_cty>(std::move(value)); }
                    }
                    FieldType::Enum(_) => {
                        $(let cty = v.ty.cc_storage_type())
                        $("// NOLINTNEXTLINE(google-explicit-constructor)")
                        $uname($cty value) : tag_(k$vp), payload_(k$vp) { payload_.$(vf)_ = value; }
                    }
                    _ => {}
                })
            })
            void Clear() {
                Destroy();
                tag_ = k$v0p;
                $(if !matches!(v0.ty, FieldType::Unit) {
                    $(let v0_cty = v0.ty.cc_storage_type())
                    new (&payload_.$(v0f)_) $v0_cty();
                })
            }
            Case kind() const { return tag_; }
            $(for v in &u.variants join ($['\r']) {
                $(let vf = &v.field_name)
                $(let vp = v.pascal_name)
                bool has_$vf() const { return tag_ == k$vp; }
                $(match v.ty.cc_type() {
                    FieldType::Unit => {
                        void set_$vf() { Destroy(); tag_ = k$vp; }
                    }
                    FieldType::Struct(cc_path) => {
                        $(let cty = &format!("::crubit::ffi::{cc_path}"))
                        const $cty& $vf() const { return has_$vf() ? payload_.$(vf)_ : DefaultValue<$cty>(); }
                        $cty* mutable_$vf() {
                            if (!has_$vf()) {
                                Destroy();
                                tag_ = k$vp;
                                new (&payload_.$(vf)_) $cty();
                            }
                            return &payload_.$(vf)_;
                        }
                    }
                    FieldType::Box(inner) => {
                        $(let inner_cty = &inner.cc_storage_type())
                        const $inner_cty& $vf() const { return has_$vf() ? payload_.$(vf)_.get_or_default() : DefaultValue<$inner_cty>(); }
                        $inner_cty* mutable_$vf() {
                            if (!has_$vf()) {
                                Destroy();
                                tag_ = k$vp;
                                new (&payload_.$(vf)_) FfiRc<$inner_cty>();
                            }
                            return &payload_.$(vf)_.get_or_init();
                        }
                    }
                    FieldType::Str => {
                        absl::string_view $vf() const { return has_$vf() ? payload_.$(vf)_.AsView() : absl::string_view(); }
                        void set_$vf(absl::string_view value) {
                            FfiString tmp(value);
                            Destroy();
                            tag_ = k$vp;
                            new (&payload_.$(vf)_) FfiString(std::move(tmp));
                        }
                    }
                    FieldType::Bytes => {
                        absl::string_view $vf() const { return has_$vf() ? absl::string_view(reinterpret_cast<const char*>(payload_.$(vf)_.data()), payload_.$(vf)_.size()) : absl::string_view(); }
                        void set_$vf(absl::string_view value) {
                            FfiVec<uint8_t> tmp(reinterpret_cast<const uint8_t*>(value.data()), reinterpret_cast<const uint8_t*>(value.data()) + value.size());
                            Destroy();
                            tag_ = k$vp;
                            new (&payload_.$(vf)_) FfiVec<uint8_t>(std::move(tmp));
                        }
                    }
                    FieldType::Bool
                    | FieldType::I32
                    | FieldType::U32
                    | FieldType::I64
                    | FieldType::U64
                    | FieldType::Usize
                    | FieldType::Enum(_) => {
                        $(let cty = &v.ty.cc_storage_type())
                        $(let default_val = if matches!(v.ty.cc_type(), FieldType::Bool) { "false".to_string() } else { format!("static_cast<{cty}>(0)") })
                        $cty $vf() const { return has_$vf() ? payload_.$(vf)_ : $default_val; }
                        void set_$vf($cty value) { Destroy(); tag_ = k$vp; payload_.$(vf)_ = value; }
                    }
                    _ => { $(ref _ => unreachable!("Unsupported union variant type")) }
                })
            })
            friend bool operator==(const $uname& lhs, const $uname& rhs) {
                if (lhs.tag_ != rhs.tag_) return false;
                switch (lhs.tag_) {
                    $(for v in &u.variants join ($['\r']) {
                        $(let vp = v.pascal_name)
                        $(let vf = &v.field_name)
                        $(if matches!(v.ty, FieldType::Unit) {
                            case k$vp: return true;
                        } else {
                            case k$vp: return lhs.payload_.$(vf)_ == rhs.payload_.$(vf)_;
                        })
                    })
                    default: CRUBIT_CHECK(false);
                }
                return false;
            }
        $[' ']private:
            void Destroy() {
                switch (tag_) {
                    $(for v in &u.variants join ($['\r']) {
                        $(let vp = v.pascal_name)
                        $(let vf = &v.field_name)
                        $(if matches!(v.ty, FieldType::Unit) {
                            case k$vp: break;
                        } else {
                            case k$vp: std::destroy_at(&payload_.$(vf)_); break;
                        })
                    })
                    default: CRUBIT_CHECK(false);
                }
            }
            void CopyFrom(const $uname& other) {
                Destroy();
                tag_ = other.tag_;
                switch (tag_) {
                    $(for v in &u.variants join ($['\r']) {
                        $(let vp = v.pascal_name)
                        $(let vf = &v.field_name)
                        $(if matches!(v.ty, FieldType::Unit) {
                            case k$vp: break;
                        } else {
                            $(let cty = v.ty.cc_storage_type())
                            case k$vp: new (&payload_.$(vf)_) $cty(other.payload_.$(vf)_); break;
                        })
                    })
                    default: CRUBIT_CHECK(false);
                }
            }
            void MoveFrom($uname&& other) noexcept {
                Destroy();
                tag_ = other.tag_;
                switch (tag_) {
                    $(for v in &u.variants join ($['\r']) {
                        $(let vp = v.pascal_name)
                        $(let vf = &v.field_name)
                        $(if matches!(v.ty, FieldType::Unit) {
                            case k$vp: break;
                        } else {
                            $(let cty = v.ty.cc_storage_type())
                            case k$vp: new (&payload_.$(vf)_) $cty(std::move(other.payload_.$(vf)_)); break;
                        })
                    })
                    default: CRUBIT_CHECK(false);
                }
                other.Clear();
            }
            Case tag_;
            union ABSL_ATTRIBUTE_TRIVIAL_ABI Payload {
                Payload() : $v0_init {}
                explicit Payload(Case) : empty_(0) {}
                Payload(const Payload&) {}
                Payload(Payload&&) noexcept {}
                ~Payload() {}
                char empty_;
                $(for v in &u.variants join ($['\r']) {
                    $(if !matches!(v.ty, FieldType::Unit) =>
                        $(v.ty.cc_storage_type()) $(&v.field_name)_;
                    )
                })
            } payload_;
        };
        $(if let Some(ns) = u.item.ns() => $("}  // namespace ")$ns)
    }
}

fn generate_cc_struct(schema: &Schema, rec_idx: usize, st: &StructDef) -> c::Tokens {
    let parent_key = st.item.cc_qual_name();
    let sname = st.item.name;
    let has_fwd_ref = schema.has_forward_ref(rec_idx);

    let has_single_field_ctor = st.fields.len() == 1
        && matches!(
            st.fields[0].ty.cc_type(),
            FieldType::Str
                | FieldType::Bool
                | FieldType::I32
                | FieldType::U32
                | FieldType::I64
                | FieldType::U64
                | FieldType::Usize
                | FieldType::Enum(_)
                | FieldType::Struct(_)
        );
    let has_leading_union_ctor = st.fields.len() > 1
        && matches!(st.fields[0].cc_name, "variant" | "kind")
        && schema.is_union_type(&st.fields[0].ty);

    quote! {
        $(if let Some(ns) = st.item.ns() => namespace $ns $("{"))
        struct ABSL_ATTRIBUTE_TRIVIAL_ABI $sname {
            $(generate_cc_nested_aliases(schema, &parent_key))
            $(if has_fwd_ref {
                $sname();
                $sname(const $sname&);
                $sname($sname&&) noexcept;
                $sname& operator=(const $sname&);
                $sname& operator=($sname&&) noexcept;
                ~$sname();
            } else {
                $(if has_single_field_ctor || has_leading_union_ctor {
                    $sname() = default;
                })
            })
            $(if (has_single_field_ctor || has_leading_union_ctor)
                && !schema.is_field_forward_ref(rec_idx, &st.fields[0].ty)
            {
                $(let f = &st.fields[0])
                $(let fname = f.cc_name)
                $(match f.ty.cc_type() {
                    FieldType::Str => {
                        explicit $sname(absl::string_view $fname) { set_$fname($fname); }
                    }
                    FieldType::Bool
                    | FieldType::I32
                    | FieldType::U32
                    | FieldType::I64
                    | FieldType::U64
                    | FieldType::Usize
                    | FieldType::Enum(_) => {
                        $(let cty = f.ty.cc_storage_type())
                        explicit $sname($cty $fname) { set_$fname($fname); }
                    }
                    FieldType::Struct(cc_path) => {
                        $(let cty = format!("::crubit::ffi::{cc_path}"))
                        explicit $sname($cty $fname) { *mutable_$fname() = std::move($fname); }
                    }
                    _ => { $(ref _ => unreachable!()) }
                })
            })
            $(for f in &st.fields join ($['\r']) {
                $(let fname = f.cc_name)
                $(let is_fwd = schema.is_field_forward_ref(rec_idx, &f.ty))
                $(let set_bit = f.cc_has_mask())
                $(if let Some(hex_mask) = &set_bit {
                    $(let clear_val = if f.default_true { "true" } else { "{}" })
                    bool has_$fname() const { return (has_bits_ & $hex_mask) != 0; }
                    $(if is_fwd {
                        void clear_$fname();
                    } else {
                        void clear_$fname() { has_bits_ &= ~$hex_mask; $(fname)_ = $clear_val; }
                    })
                })
                $(match f.ty.cc_type() {
                    FieldType::Bool
                    | FieldType::I32
                    | FieldType::U32
                    | FieldType::I64
                    | FieldType::U64
                    | FieldType::Usize
                    | FieldType::Enum(_) => {
                        $(let cty = &f.ty.cc_storage_type())
                        $cty $fname() const { return $(fname)_; }
                        void set_$fname($cty value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_ = value; }
                    }
                    FieldType::Str => {
                        absl::string_view $fname() const { return $(fname)_.AsView(); }
                        void set_$fname(absl::string_view value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_ = FfiString(value); }
                    }
                    FieldType::Bytes => {
                        absl::string_view $fname() const { return absl::string_view(reinterpret_cast<const char*>($(fname)_.data()), $(fname)_.size()); }
                        void set_$fname(absl::string_view value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_ = FfiVec<uint8_t>(reinterpret_cast<const uint8_t*>(value.data()), reinterpret_cast<const uint8_t*>(value.data()) + value.size()); }
                    }
                    FieldType::Struct(cc_path) => {
                        $(let cty = &format!("::crubit::ffi::{cc_path}"))
                        const $cty& $fname() const { return $(fname)_; }
                        $cty* mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_; }
                    }
                    FieldType::Box(inner) => {
                        $(let cty = &inner.cc_storage_type())
                        $(if is_fwd {
                            const $cty& $fname() const;
                            $cty* mutable_$fname();
                        } else {
                            const $cty& $fname() const { return $(fname)_.get_or_default(); }
                            $cty* mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_.get_or_init(); }
                        })
                    }
                    FieldType::Vec(inner) => {
                        $(let cty = &f.ty.cc_storage_type())
                        const $cty& $fname() const { return $(fname)_; }
                        $cty* mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_; }
                        $(if f.has_bit.is_none() =>
                            void clear_$fname() { $(fname)_.Clear(); }
                        )
                        $(match inner.cc_type() {
                            FieldType::Str => {
                                void add_$fname(absl::string_view value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_.Add(FfiString(value)); }
                            }
                            FieldType::Bool
                            | FieldType::I32
                            | FieldType::U32
                            | FieldType::I64
                            | FieldType::U64
                            | FieldType::Usize
                            | FieldType::Enum(_) => {
                                $(let elem_ty = &inner.cc_storage_type())
                                void add_$fname($elem_ty value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_.Add(value); }
                            }
                            _ => {
                                $(let elem_ty = &inner.cc_storage_type())
                                $(if is_fwd {
                                    $elem_ty* add_$fname();
                                } else {
                                    $elem_ty* add_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return $(fname)_.Add(); }
                                })
                            }
                        })
                    }
                    FieldType::Map(_, _) => {
                        $(let cty = &f.ty.cc_storage_type())
                        const $cty& $fname() const { return $(fname)_; }
                        $cty* mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_; }
                    }
                    FieldType::Unit | FieldType::Newtype(_, _) => { $(ref _ => unreachable!()) }
                })
            })
            friend bool operator==(const $sname&, const $sname&) = default;
            $(if st.has_optional_fields() => uint64_t has_bits_ = 0;)
            $(for f in &st.fields join ($['\r']) {
                $(let fname = f.cc_name)
                $(let cty = f.ty.cc_storage_type())
                $(if f.default_true {
                    $cty $(fname)_ = true;
                } else {
                    $cty $(fname)_{};
                })
            })
        };
        $(if let Some(ns) = st.item.ns() => $("}  // namespace ")$ns)
    }
}

fn generate_cc_out_of_line(schema: &Schema, rec_idx: usize, rec: &RecordDef) -> c::Tokens {
    let qname = &rec.cc_qual_name();
    match rec {
        RecordDef::Union(u) => {
            let layout = schema.union_layout(u);
            quote! {
                static_assert(std::is_standard_layout_v<$qname>);
                static_assert(sizeof(size_t) != 8 || (sizeof($qname) == $(layout.size) && alignof($qname) == $(layout.align)));
            }
        }
        RecordDef::Struct(st) => {
            let sname = st.item.name;
            let st_layout = schema.struct_layout(st);
            quote! {
                $(if schema.has_forward_ref(rec_idx) {
                    inline $qname::$sname() = default;
                    inline $qname::$sname(const $sname&) = default;
                    inline $qname::$sname($sname&&) noexcept = default;
                    inline $qname& $qname::operator=(const $sname&) = default;
                    inline $qname& $qname::operator=($sname&&) noexcept = default;
                    inline $qname::~$sname() = default;
                    $(for f in &st.fields join ($['\r']) {
                        $(if schema.is_field_forward_ref(rec_idx, &f.ty) {
                            $(let set_bit = f.cc_has_mask())
                            $(let fname = f.cc_name)
                            $(if let Some(hex_mask) = &set_bit {
                                $(let clear_val = if f.default_true { "true" } else { "{}" })
                                inline void $qname::clear_$fname() { has_bits_ &= ~$hex_mask; $(fname)_ = $clear_val; }
                            })
                            $(match &f.ty {
                                FieldType::Box(inner) => {
                                    $(let inner_ty = &inner.cc_storage_type())
                                    inline const $inner_ty& $qname::$fname() const { return $(fname)_.get_or_default(); }
                                    inline $inner_ty* $qname::mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_.get_or_init(); }
                                }
                                FieldType::Vec(inner) => {
                                    $(let elem_ty = &inner.cc_storage_type())
                                    inline $elem_ty* $qname::add_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return $(fname)_.Add(); }
                                }
                                FieldType::Map(_, _) => {}
                                _ => { $(ref _ => unreachable!()) }
                            })
                        })
                    })
                })
                static_assert(std::is_standard_layout_v<$qname>);
                static_assert(sizeof(size_t) != 8 || (sizeof($qname) == $(st_layout.layout.size) && alignof($qname) == $(st_layout.layout.align)));
                $(for foff in st_layout.field_offsets join ($['\r']) =>
                    static_assert(sizeof(size_t) != 8 || offsetof($qname, $(foff.cc_name)_) == $(foff.offset));
                )
            }
        }
    }
}

fn generate_cc(schema: &Schema) -> String {
    let out: c::Tokens = quote! {
        $("// Automatically @generated by ir_codegen.rs. Do not edit directly.")
        $("// To regenerate, run: common/golden_update.sh")

        $("// clang-format off")
        #ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_
        #define CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_

        #include <cstddef>
        #include <cstdint>
        #include <memory>
        #include <new>
        #include <type_traits>
        #include <utility>

        #include "third_party/absl/base/attributes.h"
        #include "common/check.h"
        #include "third_party/absl/strings/string_view.h"
        #include "rs_bindings_from_cc/ir_ffi_support.h"

        namespace crubit::ffi $("{")
        $("// NOLINTBEGIN(*-readability-class-member-naming,*-readability-constant-naming)")

        $(for rec in &schema.records join ($['\r']) {
            $(if let Some(ns) = rec.ns() {
                namespace $ns { struct ABSL_ATTRIBUTE_TRIVIAL_ABI $(rec.name()); }
            } else {
                struct ABSL_ATTRIBUTE_TRIVIAL_ABI $(rec.name());
            })
        })

        $(for en in &schema.enums join ($['\n']) {
            $(if let Some(ns) = en.item.ns() => namespace $ns $("{"))
            enum $(en.item.name) : int32_t {
                $(for v in &en.variants join ($['\r']) => $(&v.cc_name) = $(v.discriminant),)
            };
            $(if let Some(ns) = en.item.ns() => $("}  // namespace ")$ns)
        })

        $(for (rec_idx, rec) in schema.records.iter().enumerate() join ($['\n']) =>
            $(match rec {
                RecordDef::Union(u) => $(generate_cc_union(schema, u)),
                RecordDef::Struct(st) => $(generate_cc_struct(schema, rec_idx, st)),
            })
        )

        $(for (rec_idx, rec) in schema.records.iter().enumerate() join ($['\r']) =>
            $(generate_cc_out_of_line(schema, rec_idx, rec))
        )

        $("// NOLINTEND(*-readability-class-member-naming,*-readability-constant-naming)")
        $("}  // namespace crubit::ffi")

        $("#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_")
    };

    let mut w = fmt::FmtWriter::new(String::new());
    let fmt = fmt::Config::from_lang::<c::C>().with_indentation(fmt::Indentation::Space(2));
    let config = c::Config::default();
    out.format_file(&mut w.as_formatter(&fmt), &config)
        .expect("genco formatting to String cannot fail");
    w.into_inner()
}

fn generate_rs_enum(en: &EnumDef) -> rust::Tokens {
    let ename = en.item.name;
    let v0_rs = en.variants[0].rs_name;
    quote! {
        #[repr(i32)]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $ename {
            $(for (idx, v) in en.variants.iter().enumerate() join ($['\r']) {
                $(if idx == 0 => #[default])
                $(v.rs_name) = $(v.discriminant),
            })
        }
        impl $ename {
            pub const EMPTY: Self = Self::$v0_rs;
            pub const fn from_i32(val: i32) -> Option<Self> {
                match val {
                    $(for v in &en.variants join ($['\r']) => $(v.discriminant) => Some(Self::$(v.rs_name)),)
                    _ => None,
                }
            }
        }
    }
}

fn generate_rs_union(schema: &Schema, u: &UnionDef, in_submodule: bool) -> rust::Tokens {
    let uname = u.item.name;
    let prefix = if in_submodule { "super::" } else { "" };

    let v0 = &u.variants[0];
    let v0p = v0.pascal_name;
    let empty_expr = match &v0.ty {
        FieldType::Unit => format!("Self::{v0p}"),
        _ => format!("Self::{v0p}({})", v0.ty.rs_empty_expr(false, in_submodule)),
    };
    let layout = schema.union_layout(u);

    quote! {
        #[repr(C, u32)]
        $(if matches!(uname, "UnqualifiedIdentifier" | "StatusOrOptionalString") {
            #[derive(Clone, PartialEq, Eq, Hash)]
        } else {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        })
        pub enum $uname {
            $(for (idx, v) in u.variants.iter().enumerate() join ($['\r']) {
                $(let vp = v.pascal_name)
                $(match &v.ty {
                    FieldType::Unit => {
                        $vp = $idx,
                    }
                    _ => {
                        $(let sty = &v.ty.rs_storage_type(in_submodule))
                        $vp($sty) = $idx,
                    }
                })
            })
        }

        impl Default for $uname {
            fn default() -> Self {
                Self::EMPTY
            }
        }
        impl $(prefix)FfiDefault for $uname {
            const EMPTY_REF: &'static Self = &Self::EMPTY;
        }

        $(match uname {
            "UnqualifiedIdentifier" => {
                impl core::fmt::Debug for UnqualifiedIdentifier {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            Self::Identifier(id) => core::fmt::Debug::fmt(id, f),
                            Self::Operator(op) => core::fmt::Debug::fmt(op, f),
                            Self::Constructor => f.write_str("Constructor"),
                            Self::Destructor => f.write_str("Destructor"),
                            Self::ConversionOperator => f.write_str("ConversionOperator"),
                        }
                    }
                }$['\n']
            }
            "StatusOrOptionalString" => {
                impl StatusOrOptionalString {
                    pub fn as_result(&self) -> Result<Option<&str>, &str> {
                        match self {
                            Self::None => Ok(None),
                            Self::OkValue(s) => Ok(Some(s.as_str())),
                            Self::Err(e) => Err(e.as_str()),
                        }
                    }
                }
                impl core::fmt::Debug for StatusOrOptionalString {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        core::fmt::Debug::fmt(&self.as_result(), f)
                    }
                }$['\n']
            }
            _ => {}
        })
        impl $uname {
            pub const EMPTY: Self = $empty_expr;
            $(for v in &u.variants join ($['\r']) {
                $(let vf_clean = &v.field_name)
                $(let vp = v.pascal_name)
                $(match &v.ty {
                    FieldType::Unit => {
                        pub fn set_$vf_clean(&mut self) {
                            *self = Self::$vp;
                        }
                    }
                    FieldType::Enum(rs_path) => {
                        $(let ety = &format!("{prefix}{rs_path}"))
                        pub fn set_$vf_clean(&mut self, val: $ety) {
                            *self = Self::$vp(val);
                        }
                    }
                    FieldType::Box(inner) => {
                        $(let sub_ty = &inner.rs_storage_type(in_submodule))
                        pub fn set_$vf_clean(&mut self, val: $sub_ty) -> &mut $sub_ty {
                            *self = Self::$vp(FfiRc::new(val));
                            let Self::$vp(ref mut inner) = *self else {
                                unreachable!()
                            };
                            inner.make_mut()
                        }
                    }
                    _ => {
                        $(let sty = &v.ty.rs_storage_type(in_submodule))
                        pub fn set_$vf_clean(&mut self, val: $sty) -> &mut $sty {
                            *self = Self::$vp(val);
                            let Self::$vp(ref mut inner) = *self else {
                                unreachable!()
                            };
                            inner
                        }
                    }
                })
            })
        }
        #[cfg(target_pointer_width = "64")]
        const _: () = {
            assert!(core::mem::size_of::<$uname>() == $(layout.size));
            assert!(core::mem::align_of::<$uname>() == $(layout.align));
        };
    }
}

fn generate_rs_struct(schema: &Schema, st: &StructDef, in_submodule: bool) -> rust::Tokens {
    let sname = st.item.name;
    let prefix = if in_submodule { "super::" } else { "" };
    let has_opt = st.has_optional_fields();
    let is_copy = st.fields.iter().all(|f| schema.is_copy_type(&f.ty));
    let derive_debug = !has_opt && !matches!(sname, "Identifier" | "Operator" | "CcType" | "Path");
    let debug_name = if sname == "Path" { "UnsupportedItemPath" } else { sname };
    let st_layout = schema.struct_layout(st);

    quote! {
        #[repr(C)]
        $(if is_copy {
            #[derive($(if derive_debug => Debug, )Clone, Copy, PartialEq, Eq, Hash)]
        } else {
            #[derive($(if derive_debug => Debug, )Clone, PartialEq, Eq, Hash)]
        })
        pub struct $sname {
            $(if has_opt => pub has_bits: u64,)
            $(for f in &st.fields join ($['\r']) =>
                pub $(f.rs_name): $(f.ty.rs_storage_type(in_submodule)),
            )
        }

        $(match sname {
            "Identifier" => {
                impl core::fmt::Debug for Identifier {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(f, "\"{}\"", self.identifier.as_str())
                    }
                }$['\n']
            }
            "Operator" => {
                impl core::fmt::Debug for Operator {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        let sep = match self.name.as_str().chars().next() {
                            Some(c) if c.is_alphabetic() => " ",
                            _ => "",
                        };
                        write!(f, "\"operator{sep}{}\"", self.name.as_str())
                    }
                }$['\n']
            }
            "CcType" => {
                impl core::fmt::Debug for CcType {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        struct DebugVariant<'a>(&'a CcType);
                        impl core::fmt::Debug for DebugVariant<'_> {
                            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                                match &self.0.variant {
                                    cc_type::Variant::Decl(id) => {
                                        let args = (!self.0.template_args.is_empty())
                                            .then_some(self.0.template_args.as_slice());
                                        f.debug_struct("Decl")
                                            .field("id", id)
                                            .field("template_args", &args)
                                            .finish()
                                    }
                                    cc_type::Variant::FuncPointer(fp) => core::fmt::Debug::fmt(fp, f),
                                    other => core::fmt::Debug::fmt(other, f),
                                }
                            }
                        }
                        f.debug_struct("CcType")
                            .field("variant", &DebugVariant(self))
                            .field("is_const", &self.is_const)
                            .field("is_nonnull", &self.is_nonnull)
                            .field("unknown_attr", &self.unknown_attr)
                            .field("explicit_lifetimes", &self.explicit_lifetimes)
                            .finish()
                    }
                }$['\n']
            }
            _ if !derive_debug => {
                impl core::fmt::Debug for $sname {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct($(quoted(debug_name)))
                            $(for f in &st.fields join ($['\r']) {
                                $(let fname = f.rs_name)
                                $(if f.has_bit.is_some() {
                                    .field($(quoted(fname)), &self.$fname())
                                } else {
                                    .field($(quoted(fname)), &self.$fname)
                                })
                            })
                            .finish()
                    }
                }$['\n']
            }
            _ => {}
        })
        impl $(prefix)FfiDefault for $sname {
            const EMPTY_REF: &'static Self = &Self::EMPTY;
        }
        impl Default for $sname {
            fn default() -> Self {
                Self::EMPTY
            }
        }

        impl $sname {
            pub const EMPTY: Self = Self {
                $(if has_opt => has_bits: 0,)
                $(for f in &st.fields join ($['\r']) =>
                    $(f.rs_name): $(f.ty.rs_empty_expr(f.default_true, in_submodule)),
                )
            };
            $(for f in &st.fields join ($['\r']) {
                $(let fname = f.rs_name)
                $(let clean_name = f.cc_name)
                $(let rty = f.ty.rs_getter_return_type(schema, in_submodule))
                $(let val_expr = match &f.ty {
                    FieldType::Bool
                    | FieldType::I32
                    | FieldType::U32
                    | FieldType::I64
                    | FieldType::U64
                    | FieldType::Usize
                    | FieldType::Enum(_) => format!("self.{fname}"),
                    FieldType::Newtype(_, inner) => {
                        if schema.is_copy_type(inner) {
                            format!("self.{fname}")
                        } else {
                            format!("&self.{fname}")
                        }
                    }
                    FieldType::Str => format!("self.{fname}.as_str()"),
                    FieldType::Bytes | FieldType::Vec(_) => format!("self.{fname}.as_slice()"),
                    FieldType::Struct("StatusOrOptionalString") => format!("self.{fname}.as_result()"),
                    FieldType::Struct("decl_location::ByteRange") => {
                        format!("(self.{fname}.begin, self.{fname}.end)")
                    }
                    FieldType::Struct(path) => {
                        if schema.is_copy_struct(path) {
                            format!("self.{fname}")
                        } else {
                            format!("&self.{fname}")
                        }
                    }
                    FieldType::Box(_) => format!("&*self.{fname}"),
                    FieldType::Map(_, _) => format!("&self.{fname}"),
                    FieldType::Unit => unreachable!(),
                })
                $(if let Some(hex_mask) = &f.rs_has_mask() {
                    $(let empty_expr = &f.ty.rs_empty_expr(f.default_true, in_submodule))
                    pub fn has_$clean_name(&self) -> bool {
                        (self.has_bits & $hex_mask) != 0
                    }
                    pub fn clear_$clean_name(&mut self) {
                        self.has_bits &= !$hex_mask;
                        self.$fname = $empty_expr;
                    }
                    pub fn $fname(&self) -> Option<$rty> {
                        self.has_$clean_name().then(|| $val_expr)
                    }
                } else {
                    pub fn $fname(&self) -> $rty {
                        $val_expr
                    }
                })
            })
            $(for f in &st.fields join ($['\r']) {
                $(let fname = f.rs_name)
                $(let clean_name = f.cc_name)
                $(let sty = &f.ty.rs_storage_type(in_submodule))
                $(let set_bit = f.rs_has_mask())
                $(match &f.ty {
                    FieldType::Box(inner) => {
                        $(let sub_ty = &inner.rs_storage_type(in_submodule))
                        pub fn set_$clean_name(&mut self, val: $sub_ty) -> &mut $sub_ty {
                            $(if let Some(m) = &set_bit => self.has_bits |= $m;)
                            self.$fname = FfiRc::new(val);
                            self.$fname.make_mut()
                        }
                    }
                    FieldType::Enum(_) | FieldType::Vec(_) | FieldType::Map(_, _) => {
                        pub fn set_$clean_name(&mut self, val: $sty) {
                            $(if let Some(m) = &set_bit => self.has_bits |= $m;)
                            self.$fname = val;
                        }
                    }
                    _ => {
                        pub fn set_$clean_name(&mut self, val: $sty) -> &mut $sty {
                            $(if let Some(m) = &set_bit => self.has_bits |= $m;)
                            self.$fname = val;
                            &mut self.$fname
                        }
                    }
                })
            })
        }
        #[cfg(target_pointer_width = "64")]
        const _: () = {
            assert!(core::mem::size_of::<$sname>() == $(st_layout.layout.size));
            assert!(core::mem::align_of::<$sname>() == $(st_layout.layout.align));
            $(for foff in st_layout.field_offsets join ($['\r']) =>
                assert!(core::mem::offset_of!($sname, $(foff.rs_name)) == $(foff.offset));
            )
        };
    }
}

fn generate_rs(schema: &Schema) -> String {
    let mut setter_names: BTreeMap<&str, &str> = BTreeMap::new();
    for rec in &schema.records {
        match rec {
            RecordDef::Union(u) => {
                for v in &u.variants {
                    if v.field_name == "type" {
                        setter_names.insert("r#type", "type");
                        setter_names.insert("type_", "type");
                    } else {
                        setter_names.insert(&v.field_name, &v.field_name);
                    }
                }
            }
            RecordDef::Struct(st) => {
                for f in &st.fields {
                    if f.cc_name == "type" {
                        setter_names.insert("r#type", "type");
                        setter_names.insert("type_", "type");
                    } else {
                        setter_names.insert(f.rs_name, f.cc_name);
                    }
                }
            }
        }
    }

    // Group submodule items by top-level module name and optional nested submodule.
    let mut top_mods: BTreeMap<&str, BTreeMap<&str, &str>> = BTreeMap::new();
    for en in &schema.enums {
        if let Some(ns) = en.item.ns() {
            let top_mod = ns.split("::").next().unwrap();
            let sub_mods = top_mods.entry(top_mod).or_default();
            if let Some(rest) = ns.strip_prefix(&format!("{top_mod}::")) {
                sub_mods.insert(rest, ns);
            }
        }
    }
    for rec in &schema.records {
        if let Some(ns) = rec.ns() {
            let top_mod = ns.split("::").next().unwrap();
            top_mods.entry(top_mod).or_default();
        }
    }

    let out: rust::Tokens = quote! {
        $("// Automatically @generated by ir_codegen.rs. Do not edit directly.")
        $("// To regenerate, run: common/golden_update.sh")

        #![allow(
            dead_code,
            non_camel_case_types,
            unused_imports,
            clippy::large_enum_variant,
            clippy::should_implement_trait,
            clippy::clone_on_copy,
            clippy::unnecessary_lazy_evaluations,
            clippy::explicit_auto_deref
        )]

        #[rustfmt::skip]
        #[macro_export]
        macro_rules! __proto_call_setter {
            $(for (rs_ident, clean_name) in setter_names join ($['\r']) =>
                ($$msg:ident, $rs_ident, $$($$arg:tt)*) => { $$msg.set_$clean_name($$($$arg)*) };
            )
        }

        #[rustfmt::skip]
        #[macro_export]
        macro_rules! __proto_fields {
            ($$msg:ident $$(,)?) => {};
            ($$msg:ident, $$field:ident : __ { $$($$inner:tt)* } $$(, $$($$rest:tt)*)?) => {
                {
                    let __sub = $$crate::ir_generated::__proto_call_setter!($$msg, $$field, Default::default());
                    $$crate::ir_generated::__proto_fields!(__sub, $$($$inner)*);
                }
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);
            };
            ($$msg:ident, $$field:ident : [ $$( __ { $$($$inner:tt)* } ),* $$(,)? ] $$(, $$($$rest:tt)*)?) => {
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, Default::default());
                $$(
                    {
                        let __elem = $$msg.$$field.push(Default::default());
                        $$crate::ir_generated::__proto_fields!(__elem, $$($$inner)*);
                    }
                )*
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);
            };
            ($$msg:ident, $$field:ident : [ $$( ( $$k:expr, [ $$( __ { $$($$inner:tt)* } ),* $$(,)? ] ) ),* $$(,)? ] $$(, $$($$rest:tt)*)?) => {
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, Default::default());
                $$(
                    {
                        let __vec = $$msg.$$field.insert(($$k).into(), Default::default());
                        $$(
                            {
                                let __elem = __vec.push(Default::default());
                                $$crate::ir_generated::__proto_fields!(__elem, $$($$inner)*);
                            }
                        )*
                    }
                )*
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);
            };
            ($$msg:ident, $$field:ident : [ $$( ( $$k:expr, __ { $$($$inner:tt)* } ) ),* $$(,)? ] $$(, $$($$rest:tt)*)?) => {
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, Default::default());
                $$(
                    {
                        let __val = $$msg.$$field.insert(($$k).into(), Default::default());
                        $$crate::ir_generated::__proto_fields!(__val, $$($$inner)*);
                    }
                )*
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);
            };
            ($$msg:ident, $$field:ident : $$($$subty:ident)::+ { $$($$inner:tt)* } $$(, $$($$rest:tt)*)?) => {
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, $$crate::ir_generated::proto!($$($$subty)::+ { $$($$inner)* }));
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);
            };
            ($$msg:ident, $$field:ident : $$val:expr $$(, $$($$rest:tt)*)?) => {
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, ($$val).into());
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);
            };
        }

        #[rustfmt::skip]
        #[macro_export]
        macro_rules! proto {
            ($$($$ty:ident)::+ { $$($$tt:tt)* }) => {{
                let mut __msg = <$$($$ty)::+>::EMPTY;
                $$crate::ir_generated::__proto_fields!(__msg, $$($$tt)*);
                __msg
            }};
        }
        pub use {__proto_call_setter, __proto_fields, proto};

        #[rustfmt::skip]
        mod __defs {
            use crate::ir_ffi_support::{BazelLabel, FfiDefault, FfiMap, FfiRc, FfiString, FfiVec, ItemId, LifetimeId};

            $(for en in schema.enums.iter().filter(|en| en.item.scope.is_none()) join ($['\n']) =>
                $(generate_rs_enum(en))
            )

            $(for rec in schema.records.iter().filter(|rec| rec.scope().is_none()) join ($['\n']) =>
                $(match rec {
                    RecordDef::Union(u) => $(generate_rs_union(schema, u, false)),
                    RecordDef::Struct(st) => $(generate_rs_struct(schema, st, false)),
                })
            )

            $(for (&top_mod, sub_mods) in &top_mods join ($['\n']) {
                pub mod $top_mod {
                    use super::*;

                    $(for en in schema.enums.iter().filter(|en| en.item.ns() == Some(top_mod)) {
                        $(generate_rs_enum(en))$['\n']
                    })
                    $(for (&sub_mod, &full_ns) in sub_mods {
                        pub mod $sub_mod {
                            $(for en in schema.enums.iter().filter(|en| en.item.ns() == Some(full_ns)) join ($['\n']) =>
                                $(generate_rs_enum(en))
                            )
                        }$['\n']
                    })
                    $(for rec in schema.records.iter().filter(|rec| rec.ns() == Some(top_mod)) join ($['\n']) =>
                        $(match rec {
                            RecordDef::Union(u) => $(generate_rs_union(schema, u, true)),
                            RecordDef::Struct(st) => $(generate_rs_struct(schema, st, true)),
                        })
                    )
                }
            })
        }
        #[rustfmt::skip]
        pub use __defs::*;
    };

    out.to_file_string().expect("genco formatting to String cannot fail")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Usage: ir_codegen <out_cc_header> <out_rs_file>");
    let s = ir_schema::schema();
    fs::write(&args[1], generate_cc(&s)).expect("Failed to write generated C++ header");
    fs::write(&args[2], generate_rs(&s)).expect("Failed to write generated Rust file");
}
