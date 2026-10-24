// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Code generator for `#[repr(C)]` IR types shared in-memory between C++ (`ir_generated.h`)
//! and Rust (`ir_generated.rs`).
//!
//! Run `common/golden_update.sh` after modifying the schema in this file.

use genco::lang::{c, rust};
use genco::tokens::quoted;
use genco::{quote_in, Tokens};
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

// TODO(b/578040953): Clean this up once genco 0.20 is available. Genco 0.17 doesn't have token
// formatting by default so we've implemented a shim.
fn format_tokens<L>(tokens: &Tokens<L>, is_cc: bool) -> String
where
    L: genco::lang::Lang,
    L::Config: Default,
{
    let step = if is_cc { 2 } else { 4 };
    let mut indent = 0usize;
    let mut out = String::new();
    for line in tokens.to_file_vec().expect("genco formatting to String cannot fail") {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            out.push('\n');
            continue;
        }
        let is_ns_open = is_cc && trimmed.starts_with("namespace ") && trimmed.ends_with('{');
        let is_ns_close = is_cc && trimmed.starts_with("}  // namespace ");
        if !is_ns_close
            && (trimmed.starts_with('}') || trimmed.starts_with(']') || trimmed.starts_with(')'))
        {
            indent = indent.saturating_sub(1);
        }
        if is_cc && trimmed == "private:" {
            for _ in 0..(indent * step).saturating_sub(1) {
                out.push(' ');
            }
        } else {
            let line_indent = if !is_cc && trimmed.starts_with('.') { indent + 1 } else { indent };
            for _ in 0..(line_indent * step) {
                out.push(' ');
            }
        }
        out.push_str(trimmed);
        out.push('\n');
        if !is_ns_open
            && (trimmed.ends_with('{') || trimmed.ends_with('[') || trimmed.ends_with('('))
        {
            indent += 1;
        }
    }
    out
}

fn generate_cc_nested_aliases(out: &mut c::Tokens, schema: &Schema, parent_key: &str) {
    for child_rec in &schema.records {
        if let Some(scope) = child_rec.scope()
            && scope.parent == parent_key
        {
            let child_ns = &scope.ns;
            let cname = child_rec.name();
            quote_in! { *out =>
                using $cname = ::crubit::ffi::$child_ns::$cname;$['\r']
            }
        }
    }
    for child_en in &schema.enums {
        if let Some(scope) = &child_en.item.scope {
            if scope.parent == parent_key {
                let child_ns = &scope.ns;
                let ename = child_en.item.name;
                quote_in! { *out =>
                    using $ename = ::crubit::ffi::$child_ns::$ename;$['\r']
                    $(for v in &child_en.variants =>
                        static constexpr $ename $(&v.cc_name) = ::crubit::ffi::$child_ns::$(&v.cc_name);$['\r']
                    )
                }
            }
        }
    }
}

fn generate_cc(schema: &Schema) -> String {
    let mut out = c::Tokens::new();
    quote_in! { out =>
        $("// Automatically @generated by ir_codegen.rs. Do not edit directly.")$['\r']
        $("// To regenerate, run: common/golden_update.sh")$['\n']
        $("// clang-format off")$['\r']
        #ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_$['\r']
        #define CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_$['\n']
        #include <cstddef>$['\r']
        #include <cstdint>$['\r']
        #include <memory>$['\r']
        #include <new>$['\r']
        #include <type_traits>$['\r']
        #include <utility>$['\n']
        #include "third_party/absl/base/attributes.h"$['\r']
        #include "common/check.h"$['\r']
        #include "third_party/absl/strings/string_view.h"$['\r']
        #include "rs_bindings_from_cc/ir_ffi_support.h"$['\n']
        namespace crubit::ffi $("{")$['\r']
        $("// NOLINTBEGIN(*-readability-class-member-naming,*-readability-constant-naming)")$['\n']
        $(for rec in &schema.records {
            $(if let Some(ns) = rec.ns() {
                namespace $ns { struct ABSL_ATTRIBUTE_TRIVIAL_ABI $(rec.name()); }$['\r']
            } else {
                struct ABSL_ATTRIBUTE_TRIVIAL_ABI $(rec.name());$['\r']
            })
        })$['\n']
        $(for en in &schema.enums {
            $(if let Some(ns) = en.item.ns() => namespace $ns $("{")$['\r'])
            enum $(en.item.name) : int32_t {$['\r']
                $(for v in &en.variants => $(&v.cc_name) = $(v.discriminant),$['\r'])
            };$['\r']
            $(if let Some(ns) = en.item.ns() => $("}  // namespace ")$ns$['\r'])
            $['\n']
        })
    }

    // Pass 2: Union and Struct definitions.
    for (rec_idx, rec) in schema.records.iter().enumerate() {
        let parent_key = rec.cc_qual_name();
        match rec {
            RecordDef::Union(u) => {
                let uname = u.item.name;
                let v0 = &u.variants[0];
                let v0p = v0.pascal_name;
                let v0f = &v0.field_name;
                let v0_init = if matches!(v0.ty, FieldType::Unit) {
                    "empty_(0)".to_string()
                } else {
                    format!("{v0f}_()")
                };

                quote_in! { out =>
                    $(if let Some(ns) = u.item.ns() => namespace $ns $("{")$['\r'])
                    struct ABSL_ATTRIBUTE_TRIVIAL_ABI $uname {$['\r']
                        $(ref out => generate_cc_nested_aliases(out, schema, &parent_key))
                        enum Case : uint32_t {$['\r']
                            $(for (idx, v) in u.variants.iter().enumerate() =>
                                k$(v.pascal_name) = $idx,$['\r']
                            )
                        };$['\r']
                        $uname() : tag_(k$v0p), payload_() {}$['\r']
                        $uname(const $uname& other) : tag_(k$v0p), payload_() { CopyFrom(other); }$['\r']
                        $uname($uname&& other) noexcept : tag_(k$v0p), payload_() { MoveFrom(std::move(other)); }$['\r']
                        $uname& operator=(const $uname& other) {$['\r']
                            if (this != &other) { $uname tmp(other); MoveFrom(std::move(tmp)); }$['\r']
                            return *this;$['\r']
                        }$['\r']
                        $uname& operator=($uname&& other) noexcept {$['\r']
                            if (this != &other) { MoveFrom(std::move(other)); }$['\r']
                            return *this;$['\r']
                        }$['\r']
                        ~$uname() { Destroy(); }$['\r']
                        $(for v in &u.variants {
                            $(let vf = &v.field_name)
                            $(let vp = v.pascal_name)
                            $(match &v.ty {
                                FieldType::Unit => {
                                    static $uname $vp() { $uname res; res.set_$vf(); return res; }$['\r']
                                }
                                FieldType::Struct(cc_path) => {
                                    $(let cty = &format!("::crubit::ffi::{cc_path}"))
                                    $("// NOLINTNEXTLINE(google-explicit-constructor)")$['\r']
                                    $uname($cty value) : tag_(k$vp), payload_(k$vp) { new (&payload_.$(vf)_) $cty(std::move(value)); }$['\r']
                                }
                                FieldType::Box(inner) => {
                                    $(let inner_cty = &inner.cc_storage_type())
                                    $("// NOLINTNEXTLINE(google-explicit-constructor)")$['\r']
                                    $uname($inner_cty value) : tag_(k$vp), payload_(k$vp) { new (&payload_.$(vf)_) FfiRc<$inner_cty>(std::move(value)); }$['\r']
                                }
                                FieldType::Enum(_) => {
                                    $(let cty = v.ty.cc_storage_type())
                                    $("// NOLINTNEXTLINE(google-explicit-constructor)")$['\r']
                                    $uname($cty value) : tag_(k$vp), payload_(k$vp) { payload_.$(vf)_ = value; }$['\r']
                                }
                                _ => {}
                            })
                        })
                        void Clear() {$['\r']
                            Destroy();$['\r']
                            tag_ = k$v0p;$['\r']
                            $(if !matches!(v0.ty, FieldType::Unit) {
                                $(let v0_cty = v0.ty.cc_storage_type())
                                new (&payload_.$(v0f)_) $v0_cty();$['\r']
                            })
                        }$['\r']
                        Case kind() const { return tag_; }$['\r']
                        $(for v in &u.variants {
                            $(let vf = &v.field_name)
                            $(let vp = v.pascal_name)
                            bool has_$vf() const { return tag_ == k$vp; }$['\r']
                            $(match v.ty.cc_type() {
                                FieldType::Unit => {
                                    void set_$vf() { Destroy(); tag_ = k$vp; }$['\r']
                                }
                                FieldType::Struct(cc_path) => {
                                    $(let cty = &format!("::crubit::ffi::{cc_path}"))
                                    const $cty& $vf() const { return has_$vf() ? payload_.$(vf)_ : DefaultValue<$cty>(); }$['\r']
                                    $cty* mutable_$vf() {$['\r']
                                        if (!has_$vf()) {$['\r']
                                            Destroy();$['\r']
                                            tag_ = k$vp;$['\r']
                                            new (&payload_.$(vf)_) $cty();$['\r']
                                        }$['\r']
                                        return &payload_.$(vf)_;$['\r']
                                    }$['\r']
                                }
                                FieldType::Box(inner) => {
                                    $(let inner_cty = &inner.cc_storage_type())
                                    const $inner_cty& $vf() const { return has_$vf() ? payload_.$(vf)_.get_or_default() : DefaultValue<$inner_cty>(); }$['\r']
                                    $inner_cty* mutable_$vf() {$['\r']
                                        if (!has_$vf()) {$['\r']
                                            Destroy();$['\r']
                                            tag_ = k$vp;$['\r']
                                            new (&payload_.$(vf)_) FfiRc<$inner_cty>();$['\r']
                                        }$['\r']
                                        return &payload_.$(vf)_.get_or_init();$['\r']
                                    }$['\r']
                                }
                                FieldType::Str => {
                                    absl::string_view $vf() const { return has_$vf() ? payload_.$(vf)_.AsView() : absl::string_view(); }$['\r']
                                    void set_$vf(absl::string_view value) {$['\r']
                                        FfiString tmp(value);$['\r']
                                        Destroy();$['\r']
                                        tag_ = k$vp;$['\r']
                                        new (&payload_.$(vf)_) FfiString(std::move(tmp));$['\r']
                                    }$['\r']
                                }
                                FieldType::Bytes => {
                                    absl::string_view $vf() const { return has_$vf() ? absl::string_view(reinterpret_cast<const char*>(payload_.$(vf)_.data()), payload_.$(vf)_.size()) : absl::string_view(); }$['\r']
                                    void set_$vf(absl::string_view value) {$['\r']
                                        FfiVec<uint8_t> tmp(reinterpret_cast<const uint8_t*>(value.data()), reinterpret_cast<const uint8_t*>(value.data()) + value.size());$['\r']
                                        Destroy();$['\r']
                                        tag_ = k$vp;$['\r']
                                        new (&payload_.$(vf)_) FfiVec<uint8_t>(std::move(tmp));$['\r']
                                    }$['\r']
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
                                    $cty $vf() const { return has_$vf() ? payload_.$(vf)_ : $default_val; }$['\r']
                                    void set_$vf($cty value) { Destroy(); tag_ = k$vp; payload_.$(vf)_ = value; }$['\r']
                                }
                                _ => { $(ref _ => unreachable!("Unsupported union variant type")) }
                            })
                        })
                        friend bool operator==(const $uname& lhs, const $uname& rhs) {$['\r']
                            if (lhs.tag_ != rhs.tag_) return false;$['\r']
                            switch (lhs.tag_) {$['\r']
                                $(for v in &u.variants {
                                    $(let vp = v.pascal_name)
                                    $(let vf = &v.field_name)
                                    $(if matches!(v.ty, FieldType::Unit) {
                                        case k$vp: return true;$['\r']
                                    } else {
                                        case k$vp: return lhs.payload_.$(vf)_ == rhs.payload_.$(vf)_;$['\r']
                                    })
                                })
                                default: CRUBIT_CHECK(false);$['\r']
                            }$['\r']
                            return false;$['\r']
                        }$['\r']
                        private:$['\r']
                        void Destroy() {$['\r']
                            switch (tag_) {$['\r']
                                $(for v in &u.variants {
                                    $(let vp = v.pascal_name)
                                    $(let vf = &v.field_name)
                                    $(if matches!(v.ty, FieldType::Unit) {
                                        case k$vp: break;$['\r']
                                    } else {
                                        case k$vp: std::destroy_at(&payload_.$(vf)_); break;$['\r']
                                    })
                                })
                                default: CRUBIT_CHECK(false);$['\r']
                            }$['\r']
                        }$['\r']
                        void CopyFrom(const $uname& other) {$['\r']
                            Destroy();$['\r']
                            tag_ = other.tag_;$['\r']
                            switch (tag_) {$['\r']
                                $(for v in &u.variants {
                                    $(let vp = v.pascal_name)
                                    $(let vf = &v.field_name)
                                    $(if matches!(v.ty, FieldType::Unit) {
                                        case k$vp: break;$['\r']
                                    } else {
                                        $(let cty = v.ty.cc_storage_type())
                                        case k$vp: new (&payload_.$(vf)_) $cty(other.payload_.$(vf)_); break;$['\r']
                                    })
                                })
                                default: CRUBIT_CHECK(false);$['\r']
                            }$['\r']
                        }$['\r']
                        void MoveFrom($uname&& other) noexcept {$['\r']
                            Destroy();$['\r']
                            tag_ = other.tag_;$['\r']
                            switch (tag_) {$['\r']
                                $(for v in &u.variants {
                                    $(let vp = v.pascal_name)
                                    $(let vf = &v.field_name)
                                    $(if matches!(v.ty, FieldType::Unit) {
                                        case k$vp: break;$['\r']
                                    } else {
                                        $(let cty = v.ty.cc_storage_type())
                                        case k$vp: new (&payload_.$(vf)_) $cty(std::move(other.payload_.$(vf)_)); break;$['\r']
                                    })
                                })
                                default: CRUBIT_CHECK(false);$['\r']
                            }$['\r']
                            other.Clear();$['\r']
                        }$['\r']
                        Case tag_;$['\r']
                        union ABSL_ATTRIBUTE_TRIVIAL_ABI Payload {$['\r']
                            Payload() : $v0_init {}$['\r']
                            explicit Payload(Case) : empty_(0) {}$['\r']
                            Payload(const Payload&) {}$['\r']
                            Payload(Payload&&) noexcept {}$['\r']
                            ~Payload() {}$['\r']
                            char empty_;$['\r']
                            $(for v in &u.variants {
                                $(if !matches!(v.ty, FieldType::Unit) =>
                                    $(v.ty.cc_storage_type()) $(&v.field_name)_;$['\r']
                                )
                            })
                        } payload_;$['\r']
                    };$['\r']
                    $(if let Some(ns) = u.item.ns() => $("}  // namespace ")$ns$['\r'])
                    $['\n']
                }
            }
            RecordDef::Struct(st) => {
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

                quote_in! { out =>
                    $(if let Some(ns) = st.item.ns() => namespace $ns $("{")$['\r'])
                    struct ABSL_ATTRIBUTE_TRIVIAL_ABI $sname {$['\r']
                        $(ref out => generate_cc_nested_aliases(out, schema, &parent_key))
                        $(if has_fwd_ref {
                            $sname();$['\r']
                            $sname(const $sname&);$['\r']
                            $sname($sname&&) noexcept;$['\r']
                            $sname& operator=(const $sname&);$['\r']
                            $sname& operator=($sname&&) noexcept;$['\r']
                            ~$sname();$['\r']
                        } else {
                            $(if has_single_field_ctor || has_leading_union_ctor {
                                $sname() = default;$['\r']
                            })
                        })
                        $(if (has_single_field_ctor || has_leading_union_ctor)
                            && !schema.is_field_forward_ref(rec_idx, &st.fields[0].ty)
                        {
                            $(let f = &st.fields[0])
                            $(let fname = f.cc_name)
                            $(match f.ty.cc_type() {
                                FieldType::Str => {
                                    explicit $sname(absl::string_view $fname) { set_$fname($fname); }$['\r']
                                }
                                FieldType::Bool
                                | FieldType::I32
                                | FieldType::U32
                                | FieldType::I64
                                | FieldType::U64
                                | FieldType::Usize
                                | FieldType::Enum(_) => {
                                    $(let cty = f.ty.cc_storage_type())
                                    explicit $sname($cty $fname) { set_$fname($fname); }$['\r']
                                }
                                FieldType::Struct(cc_path) => {
                                    $(let cty = format!("::crubit::ffi::{cc_path}"))
                                    explicit $sname($cty $fname) { *mutable_$fname() = std::move($fname); }$['\r']
                                }
                                _ => { $(ref _ => unreachable!()) }
                            })
                        })
                        $(ref out {
                            // Regular field accessors
                            for f in &st.fields {
                                let fname = f.cc_name;
                                let is_fwd = schema.is_field_forward_ref(rec_idx, &f.ty);
                                let set_bit = f.cc_has_mask();
                                if let Some(hex_mask) = &set_bit {
                                    let clear_val = if f.default_true { "true" } else { "{}" };
                                    quote_in! { *out =>
                                        bool has_$fname() const { return (has_bits_ & $hex_mask) != 0; }$['\r']
                                        $(if is_fwd {
                                            void clear_$fname();$['\r']
                                        } else {
                                            void clear_$fname() { has_bits_ &= ~$hex_mask; $(fname)_ = $clear_val; }$['\r']
                                        })
                                    }
                                }
                                match f.ty.cc_type() {
                                    FieldType::Bool
                                    | FieldType::I32
                                    | FieldType::U32
                                    | FieldType::I64
                                    | FieldType::U64
                                    | FieldType::Usize
                                    | FieldType::Enum(_) => {
                                        let cty = &f.ty.cc_storage_type();
                                        quote_in! { *out =>
                                            $cty $fname() const { return $(fname)_; }$['\r']
                                            void set_$fname($cty value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_ = value; }$['\r']
                                        }
                                    }
                                    FieldType::Str => {
                                        quote_in! { *out =>
                                            absl::string_view $fname() const { return $(fname)_.AsView(); }$['\r']
                                            void set_$fname(absl::string_view value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_ = FfiString(value); }$['\r']
                                        }
                                    }
                                    FieldType::Bytes => {
                                        quote_in! { *out =>
                                            absl::string_view $fname() const { return absl::string_view(reinterpret_cast<const char*>($(fname)_.data()), $(fname)_.size()); }$['\r']
                                            void set_$fname(absl::string_view value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_ = FfiVec<uint8_t>(reinterpret_cast<const uint8_t*>(value.data()), reinterpret_cast<const uint8_t*>(value.data()) + value.size()); }$['\r']
                                        }
                                    }
                                    FieldType::Struct(cc_path) => {
                                        let cty = &format!("::crubit::ffi::{cc_path}");
                                        quote_in! { *out =>
                                            const $cty& $fname() const { return $(fname)_; }$['\r']
                                            $cty* mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_; }$['\r']
                                        }
                                    }
                                    FieldType::Box(inner) => {
                                        let cty = &inner.cc_storage_type();
                                        quote_in! { *out =>
                                            $(if is_fwd {
                                                const $cty& $fname() const;$['\r']
                                                $cty* mutable_$fname();$['\r']
                                            } else {
                                                const $cty& $fname() const { return $(fname)_.get_or_default(); }$['\r']
                                                $cty* mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_.get_or_init(); }$['\r']
                                            })
                                        }
                                    }
                                    FieldType::Vec(inner) => {
                                        let cty = &f.ty.cc_storage_type();
                                        quote_in! { *out =>
                                            const $cty& $fname() const { return $(fname)_; }$['\r']
                                            $cty* mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_; }$['\r']
                                            $(if f.has_bit.is_none() =>
                                                void clear_$fname() { $(fname)_.Clear(); }$['\r']
                                            )
                                            $(match inner.cc_type() {
                                                FieldType::Str => {
                                                    void add_$fname(absl::string_view value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_.Add(FfiString(value)); }$['\r']
                                                }
                                                FieldType::Bool
                                                | FieldType::I32
                                                | FieldType::U32
                                                | FieldType::I64
                                                | FieldType::U64
                                                | FieldType::Usize
                                                | FieldType::Enum(_) => {
                                                    $(let elem_ty = &inner.cc_storage_type())
                                                    void add_$fname($elem_ty value) { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])$(fname)_.Add(value); }$['\r']
                                                }
                                                _ => {
                                                    $(let elem_ty = &inner.cc_storage_type())
                                                    $(if is_fwd {
                                                        $elem_ty* add_$fname();$['\r']
                                                    } else {
                                                        $elem_ty* add_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return $(fname)_.Add(); }$['\r']
                                                    })
                                                }
                                            })
                                        }
                                    }
                                    FieldType::Map(_, _) => {
                                        let cty = &f.ty.cc_storage_type();
                                        quote_in! { *out =>
                                            const $cty& $fname() const { return $(fname)_; }$['\r']
                                            $cty* mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_; }$['\r']
                                        }
                                    }
                                    FieldType::Unit | FieldType::Newtype(_, _) => unreachable!(),
                                }
                            }
                        })
                        friend bool operator==(const $sname&, const $sname&) = default;$['\r']
                        $(if st.has_optional_fields() => uint64_t has_bits_ = 0;$['\r'])
                        $(for f in &st.fields {
                            $(let fname = f.cc_name)
                            $(let cty = f.ty.cc_storage_type())
                            $(if f.default_true {
                                $cty $(fname)_ = true;$['\r']
                            } else {
                                $cty $(fname)_{};$['\r']
                            })
                        })
                    };$['\r']
                    $(if let Some(ns) = st.item.ns() => $("}  // namespace ")$ns$['\r'])
                    $['\n']
                }
            }
        }
    }

    // Pass 3: Out-of-line inline definitions for forward-referenced structs and layout assertions.
    for (rec_idx, rec) in schema.records.iter().enumerate() {
        let qname = &rec.cc_qual_name();
        match rec {
            RecordDef::Union(u) => {
                let layout = schema.union_layout(u);
                quote_in! { out =>
                    static_assert(std::is_standard_layout_v<$qname>);$['\r']
                    static_assert(sizeof(size_t) != 8 || (sizeof($qname) == $(layout.size) && alignof($qname) == $(layout.align)));$['\r']
                }
            }
            RecordDef::Struct(st) => {
                let sname = st.item.name;
                if schema.has_forward_ref(rec_idx) {
                    quote_in! { out =>
                        inline $qname::$sname() = default;$['\r']
                        inline $qname::$sname(const $sname&) = default;$['\r']
                        inline $qname::$sname($sname&&) noexcept = default;$['\r']
                        inline $qname& $qname::operator=(const $sname&) = default;$['\r']
                        inline $qname& $qname::operator=($sname&&) noexcept = default;$['\r']
                        inline $qname::~$sname() = default;$['\r']
                    }

                    for f in &st.fields {
                        if !schema.is_field_forward_ref(rec_idx, &f.ty) {
                            continue;
                        }
                        let set_bit = f.cc_has_mask();
                        let fname = f.cc_name;
                        if let Some(hex_mask) = &set_bit {
                            let clear_val = if f.default_true { "true" } else { "{}" };
                            quote_in! { out =>
                                inline void $qname::clear_$fname() { has_bits_ &= ~$hex_mask; $(fname)_ = $clear_val; }$['\r']
                            }
                        }
                        match &f.ty {
                            FieldType::Box(inner) => {
                                let inner_ty = &inner.cc_storage_type();
                                quote_in! { out =>
                                    inline const $inner_ty& $qname::$fname() const { return $(fname)_.get_or_default(); }$['\r']
                                    inline $inner_ty* $qname::mutable_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return &$(fname)_.get_or_init(); }$['\r']
                                }
                            }
                            FieldType::Vec(inner) => {
                                let elem_ty = &inner.cc_storage_type();
                                quote_in! { out =>
                                    inline $elem_ty* $qname::add_$fname() { $(if let Some(m) = &set_bit => has_bits_ |= $m;$[' '])return $(fname)_.Add(); }$['\r']
                                }
                            }
                            FieldType::Map(_, _) => {}
                            _ => unreachable!(),
                        }
                    }
                }

                let st_layout = schema.struct_layout(st);
                quote_in! { out =>
                    static_assert(std::is_standard_layout_v<$qname>);$['\r']
                    static_assert(sizeof(size_t) != 8 || (sizeof($qname) == $(st_layout.layout.size) && alignof($qname) == $(st_layout.layout.align)));$['\r']
                    $(for foff in st_layout.field_offsets =>
                        static_assert(sizeof(size_t) != 8 || offsetof($qname, $(foff.cc_name)_) == $(foff.offset));$['\r']
                    )
                }
            }
        }
    }

    quote_in! { out =>
        $['\n']
        $("// NOLINTEND(*-readability-class-member-naming,*-readability-constant-naming)")$['\r']
        $("}  // namespace crubit::ffi")$['\n']
        $("#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_")$['\r']
    }
    format_tokens(&out, true)
}

fn generate_rs_enum(out: &mut rust::Tokens, en: &EnumDef) {
    let ename = en.item.name;
    let v0_rs = en.variants[0].rs_name;
    quote_in! { *out =>
        #[repr(i32)]$['\r']
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]$['\r']
        pub enum $ename {$['\r']
            $(for (idx, v) in en.variants.iter().enumerate() {
                $(if idx == 0 => #[default]$['\r'])
                $(v.rs_name) = $(v.discriminant),$['\r']
            })
        }$['\r']
        impl $ename {$['\r']
            pub const EMPTY: Self = Self::$v0_rs;$['\r']
            pub const fn from_i32(val: i32) -> Option<Self> {$['\r']
                match val {$['\r']
                    $(for v in &en.variants => $(v.discriminant) => Some(Self::$(v.rs_name)),$['\r'])
                    _ => None,$['\r']
                }$['\r']
            }$['\r']
        }$['\n']
    }
}

fn generate_rs_union(out: &mut rust::Tokens, schema: &Schema, u: &UnionDef, in_submodule: bool) {
    let uname = u.item.name;
    let prefix = if in_submodule { "super::" } else { "" };

    let v0 = &u.variants[0];
    let v0p = v0.pascal_name;
    let empty_expr = match &v0.ty {
        FieldType::Unit => format!("Self::{v0p}"),
        _ => format!("Self::{v0p}({})", v0.ty.rs_empty_expr(false, in_submodule)),
    };
    let layout = schema.union_layout(u);

    quote_in! { *out =>
        #[repr(C, u32)]$['\r']
        $(if matches!(uname, "UnqualifiedIdentifier" | "StatusOrOptionalString") {
            #[derive(Clone, PartialEq, Eq, Hash)]$['\r']
        } else {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]$['\r']
        })
        pub enum $uname {$['\r']
            $(for (idx, v) in u.variants.iter().enumerate() {
                $(let vp = v.pascal_name)
                $(match &v.ty {
                    FieldType::Unit => {
                        $vp = $idx,$['\r']
                    }
                    _ => {
                        $(let sty = &v.ty.rs_storage_type(in_submodule))
                        $vp($sty) = $idx,$['\r']
                    }
                })
            })
        }$['\n']
        impl Default for $uname {$['\r']
            fn default() -> Self {$['\r']
                Self::EMPTY$['\r']
            }$['\r']
        }$['\r']
        impl $(prefix)FfiDefault for $uname {$['\r']
            const EMPTY_REF: &'static Self = &Self::EMPTY;$['\r']
        }$['\n']
        $(match uname {
            "UnqualifiedIdentifier" => {
                impl core::fmt::Debug for UnqualifiedIdentifier {$['\r']
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {$['\r']
                        match self {$['\r']
                            Self::Identifier(id) => core::fmt::Debug::fmt(id, f),$['\r']
                            Self::Operator(op) => core::fmt::Debug::fmt(op, f),$['\r']
                            Self::Constructor => f.write_str("Constructor"),$['\r']
                            Self::Destructor => f.write_str("Destructor"),$['\r']
                            Self::ConversionOperator => f.write_str("ConversionOperator"),$['\r']
                        }$['\r']
                    }$['\r']
                }$['\n']
            }
            "StatusOrOptionalString" => {
                impl StatusOrOptionalString {$['\r']
                    pub fn as_result(&self) -> Result<Option<&str>, &str> {$['\r']
                        match self {$['\r']
                            Self::None => Ok(None),$['\r']
                            Self::OkValue(s) => Ok(Some(s.as_str())),$['\r']
                            Self::Err(e) => Err(e.as_str()),$['\r']
                        }$['\r']
                    }$['\r']
                }$['\r']
                impl core::fmt::Debug for StatusOrOptionalString {$['\r']
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {$['\r']
                        core::fmt::Debug::fmt(&self.as_result(), f)$['\r']
                    }$['\r']
                }$['\n']
            }
            _ => {}
        })
        impl $uname {$['\r']
            pub const EMPTY: Self = $empty_expr;$['\r']
            $(for v in &u.variants {
                $(let vf_clean = &v.field_name)
                $(let vp = v.pascal_name)
                $(match &v.ty {
                    FieldType::Unit => {
                        pub fn set_$vf_clean(&mut self) {$['\r']
                            *self = Self::$vp;$['\r']
                        }$['\r']
                    }
                    FieldType::Enum(rs_path) => {
                        $(let ety = &format!("{prefix}{rs_path}"))
                        pub fn set_$vf_clean(&mut self, val: $ety) {$['\r']
                            *self = Self::$vp(val);$['\r']
                        }$['\r']
                    }
                    FieldType::Box(inner) => {
                        $(let sub_ty = &inner.rs_storage_type(in_submodule))
                        pub fn set_$vf_clean(&mut self, val: $sub_ty) -> &mut $sub_ty {$['\r']
                            *self = Self::$vp(FfiRc::new(val));$['\r']
                            let Self::$vp(ref mut inner) = *self else {$['\r']
                                unreachable!()$['\r']
                            };$['\r']
                            inner.make_mut()$['\r']
                        }$['\r']
                    }
                    _ => {
                        $(let sty = &v.ty.rs_storage_type(in_submodule))
                        pub fn set_$vf_clean(&mut self, val: $sty) -> &mut $sty {$['\r']
                            *self = Self::$vp(val);$['\r']
                            let Self::$vp(ref mut inner) = *self else {$['\r']
                                unreachable!()$['\r']
                            };$['\r']
                            inner$['\r']
                        }$['\r']
                    }
                })
            })
        }$['\r']
        #[cfg(target_pointer_width = "64")]$['\r']
        const _: () = {$['\r']
            assert!(core::mem::size_of::<$uname>() == $(layout.size));$['\r']
            assert!(core::mem::align_of::<$uname>() == $(layout.align));$['\r']
        };$['\n']
    }
}

fn generate_rs_struct(out: &mut rust::Tokens, schema: &Schema, st: &StructDef, in_submodule: bool) {
    let sname = st.item.name;
    let prefix = if in_submodule { "super::" } else { "" };
    let has_opt = st.has_optional_fields();
    let is_copy = st.fields.iter().all(|f| schema.is_copy_type(&f.ty));
    let derive_debug = !has_opt && !matches!(sname, "Identifier" | "Operator" | "CcType" | "Path");
    let debug_name = if sname == "Path" { "UnsupportedItemPath" } else { sname };
    let st_layout = schema.struct_layout(st);

    quote_in! { *out =>
        #[repr(C)]$['\r']
        $(if is_copy {
            #[derive($(if derive_debug => Debug, )Clone, Copy, PartialEq, Eq, Hash)]$['\r']
        } else {
            #[derive($(if derive_debug => Debug, )Clone, PartialEq, Eq, Hash)]$['\r']
        })
        pub struct $sname {$['\r']
            $(if has_opt => pub has_bits: u64,$['\r'])
            $(for f in &st.fields =>
                pub $(f.rs_name): $(f.ty.rs_storage_type(in_submodule)),$['\r']
            )
        }$['\n']
        $(match sname {
            "Identifier" => {
                impl core::fmt::Debug for Identifier {$['\r']
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {$['\r']
                        write!(f, "\"{}\"", self.identifier.as_str())$['\r']
                    }$['\r']
                }$['\n']
            }
            "Operator" => {
                impl core::fmt::Debug for Operator {$['\r']
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {$['\r']
                        let sep = match self.name.as_str().chars().next() {$['\r']
                            Some(c) if c.is_alphabetic() => " ",$['\r']
                            _ => "",$['\r']
                        };$['\r']
                        write!(f, "\"operator{sep}{}\"", self.name.as_str())$['\r']
                    }$['\r']
                }$['\n']
            }
            "CcType" => {
                impl core::fmt::Debug for CcType {$['\r']
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {$['\r']
                        struct DebugVariant<'a>(&'a CcType);$['\r']
                        impl core::fmt::Debug for DebugVariant<'_> {$['\r']
                            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {$['\r']
                                match &self.0.variant {$['\r']
                                    cc_type::Variant::Decl(id) => {$['\r']
                                        let args = (!self.0.template_args.is_empty())$['\r']
                                            .then_some(self.0.template_args.as_slice());$['\r']
                                        f.debug_struct("Decl")$['\r']
                                            .field("id", id)$['\r']
                                            .field("template_args", &args)$['\r']
                                            .finish()$['\r']
                                    }$['\r']
                                    cc_type::Variant::FuncPointer(fp) => core::fmt::Debug::fmt(fp, f),$['\r']
                                    other => core::fmt::Debug::fmt(other, f),$['\r']
                                }$['\r']
                            }$['\r']
                        }$['\r']
                        f.debug_struct("CcType")$['\r']
                            .field("variant", &DebugVariant(self))$['\r']
                            .field("is_const", &self.is_const)$['\r']
                            .field("is_nonnull", &self.is_nonnull)$['\r']
                            .field("unknown_attr", &self.unknown_attr)$['\r']
                            .field("explicit_lifetimes", &self.explicit_lifetimes)$['\r']
                            .finish()$['\r']
                    }$['\r']
                }$['\n']
            }
            _ if !derive_debug => {
                impl core::fmt::Debug for $sname {$['\r']
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {$['\r']
                        f.debug_struct($(quoted(debug_name)))$['\r']
                            $(for f in &st.fields {
                                $(let fname = f.rs_name)
                                $(if f.has_bit.is_some() {
                                    .field($(quoted(fname)), &self.$fname())$['\r']
                                } else {
                                    .field($(quoted(fname)), &self.$fname)$['\r']
                                })
                            })
                            .finish()$['\r']
                    }$['\r']
                }$['\n']
            }
            _ => {}
        })
        impl $(prefix)FfiDefault for $sname {$['\r']
            const EMPTY_REF: &'static Self = &Self::EMPTY;$['\r']
        }$['\r']
        impl Default for $sname {$['\r']
            fn default() -> Self {$['\r']
                Self::EMPTY$['\r']
            }$['\r']
        }$['\n']
        impl $sname {$['\r']
            pub const EMPTY: Self = Self {$['\r']
                $(if has_opt => has_bits: 0,$['\r'])
                $(for f in &st.fields =>
                    $(f.rs_name): $(f.ty.rs_empty_expr(f.default_true, in_submodule)),$['\r']
                )
            };$['\r']
            $(ref out {
                for f in &st.fields {
                    let fname = f.rs_name;
                    let clean_name = f.cc_name;
                    let rty = f.ty.rs_getter_return_type(schema, in_submodule);
                    let val_expr = match &f.ty {
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
                    };

                    if let Some(hex_mask) = &f.rs_has_mask() {
                        let empty_expr = &f.ty.rs_empty_expr(f.default_true, in_submodule);
                        quote_in! { *out =>
                            pub fn has_$clean_name(&self) -> bool {$['\r']
                                $("")(self.has_bits & $hex_mask) != 0$['\r']
                            }$['\r']
                            pub fn clear_$clean_name(&mut self) {$['\r']
                                self.has_bits &= !$hex_mask;$['\r']
                                self.$fname = $empty_expr;$['\r']
                            }$['\r']
                            pub fn $fname(&self) -> Option<$rty> {$['\r']
                                self.has_$clean_name().then(|| $val_expr)$['\r']
                            }$['\r']
                        }
                    } else {
                        quote_in! { *out =>
                            pub fn $fname(&self) -> $rty {$['\r']
                                $val_expr$['\r']
                            }$['\r']
                        }
                    }
                }

                for f in &st.fields {
                    let fname = f.rs_name;
                    let clean_name = f.cc_name;
                    let sty = &f.ty.rs_storage_type(in_submodule);
                    let set_bit = f.rs_has_mask();
                    match &f.ty {
                        FieldType::Box(inner) => {
                            let sub_ty = &inner.rs_storage_type(in_submodule);
                            quote_in! { *out =>
                                pub fn set_$clean_name(&mut self, val: $sub_ty) -> &mut $sub_ty {$['\r']
                                    $(if let Some(m) = &set_bit => self.has_bits |= $m;$['\r'])
                                    self.$fname = FfiRc::new(val);$['\r']
                                    self.$fname.make_mut()$['\r']
                                }$['\r']
                            }
                        }
                        FieldType::Enum(_) | FieldType::Vec(_) | FieldType::Map(_, _) => {
                            quote_in! { *out =>
                                pub fn set_$clean_name(&mut self, val: $sty) {$['\r']
                                    $(if let Some(m) = &set_bit => self.has_bits |= $m;$['\r'])
                                    self.$fname = val;$['\r']
                                }$['\r']
                            }
                        }
                        _ => {
                            quote_in! { *out =>
                                pub fn set_$clean_name(&mut self, val: $sty) -> &mut $sty {$['\r']
                                    $(if let Some(m) = &set_bit => self.has_bits |= $m;$['\r'])
                                    self.$fname = val;$['\r']
                                    &mut self.$fname$['\r']
                                }$['\r']
                            }
                        }
                    }
                }
            })
        }$['\r']
        #[cfg(target_pointer_width = "64")]$['\r']
        const _: () = {$['\r']
            assert!(core::mem::size_of::<$sname>() == $(st_layout.layout.size));$['\r']
            assert!(core::mem::align_of::<$sname>() == $(st_layout.layout.align));$['\r']
            $(for foff in st_layout.field_offsets =>
                assert!(core::mem::offset_of!($sname, $(foff.rs_name)) == $(foff.offset));$['\r']
            )
        };$['\n']
    }
}

fn generate_rs(schema: &Schema) -> String {
    let mut out = rust::Tokens::new();

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

    quote_in! { out =>
        $("// Automatically @generated by ir_codegen.rs. Do not edit directly.")$['\r']
        $("// To regenerate, run: common/golden_update.sh")$['\n']
        #![allow($['\r']
            dead_code,$['\r']
            non_camel_case_types,$['\r']
            unused_imports,$['\r']
            clippy::large_enum_variant,$['\r']
            clippy::should_implement_trait,$['\r']
            clippy::clone_on_copy,$['\r']
            clippy::unnecessary_lazy_evaluations,$['\r']
            clippy::explicit_auto_deref$['\r']
        )]$['\n']
        #[rustfmt::skip]$['\r']
        #[macro_export]$['\r']
        macro_rules! __proto_call_setter {$['\r']
            $(for (rs_ident, clean_name) in setter_names =>
                ($$msg:ident, $rs_ident, $$($$arg:tt)*) => { $$msg.set_$clean_name($$($$arg)*) };$['\r']
            )
        }$['\n']
        #[rustfmt::skip]$['\r']
        #[macro_export]$['\r']
        macro_rules! __proto_fields {$['\r']
            $("")($$msg:ident $$(,)?) => {};$['\r']
            $("")($$msg:ident, $$field:ident : __ { $$($$inner:tt)* } $$(, $$($$rest:tt)*)?) => {$['\r']
                {$['\r']
                    let __sub = $$crate::ir_generated::__proto_call_setter!($$msg, $$field, Default::default());$['\r']
                    $$crate::ir_generated::__proto_fields!(__sub, $$($$inner)*);$['\r']
                }$['\r']
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);$['\r']
            };$['\r']
            $("")($$msg:ident, $$field:ident : [ $$( __ { $$($$inner:tt)* } ),* $$(,)? ] $$(, $$($$rest:tt)*)?) => {$['\r']
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, Default::default());$['\r']
                $$($['\r']
                    {$['\r']
                        let __elem = $$msg.$$field.push(Default::default());$['\r']
                        $$crate::ir_generated::__proto_fields!(__elem, $$($$inner)*);$['\r']
                    }$['\r']
                )*$['\r']
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);$['\r']
            };$['\r']
            $("")($$msg:ident, $$field:ident : [ $$( ( $$k:expr, [ $$( __ { $$($$inner:tt)* } ),* $$(,)? ] ) ),* $$(,)? ] $$(, $$($$rest:tt)*)?) => {$['\r']
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, Default::default());$['\r']
                $$($['\r']
                    {$['\r']
                        let __vec = $$msg.$$field.insert(($$k).into(), Default::default());$['\r']
                        $$($['\r']
                            {$['\r']
                                let __elem = __vec.push(Default::default());$['\r']
                                $$crate::ir_generated::__proto_fields!(__elem, $$($$inner)*);$['\r']
                            }$['\r']
                        )*$['\r']
                    }$['\r']
                )*$['\r']
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);$['\r']
            };$['\r']
            $("")($$msg:ident, $$field:ident : [ $$( ( $$k:expr, __ { $$($$inner:tt)* } ) ),* $$(,)? ] $$(, $$($$rest:tt)*)?) => {$['\r']
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, Default::default());$['\r']
                $$($['\r']
                    {$['\r']
                        let __val = $$msg.$$field.insert(($$k).into(), Default::default());$['\r']
                        $$crate::ir_generated::__proto_fields!(__val, $$($$inner)*);$['\r']
                    }$['\r']
                )*$['\r']
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);$['\r']
            };$['\r']
            $("")($$msg:ident, $$field:ident : $$($$subty:ident)::+ { $$($$inner:tt)* } $$(, $$($$rest:tt)*)?) => {$['\r']
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, $$crate::ir_generated::proto!($$($$subty)::+ { $$($$inner)* }));$['\r']
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);$['\r']
            };$['\r']
            $("")($$msg:ident, $$field:ident : $$val:expr $$(, $$($$rest:tt)*)?) => {$['\r']
                $$crate::ir_generated::__proto_call_setter!($$msg, $$field, ($$val).into());$['\r']
                $$crate::ir_generated::__proto_fields!($$msg, $$($$($$rest)*)?);$['\r']
            };$['\r']
        }$['\n']
        #[rustfmt::skip]$['\r']
        #[macro_export]$['\r']
        macro_rules! proto {$['\r']
            $("")($$($$ty:ident)::+ { $$($$tt:tt)* }) => {{$['\r']
                let mut __msg = <$$($$ty)::+>::EMPTY;$['\r']
                $$crate::ir_generated::__proto_fields!(__msg, $$($$tt)*);$['\r']
                __msg$['\r']
            }};$['\r']
        }$['\r']
        pub use {__proto_call_setter, __proto_fields, proto};$['\n']
        #[rustfmt::skip]$['\r']
        mod __defs {$['\r']
            use crate::ir_ffi_support::{BazelLabel, FfiDefault, FfiMap, FfiRc, FfiString, FfiVec, ItemId, LifetimeId};$['\n']
            $(ref out {
                // Top-level enums
                for en in &schema.enums {
                    if en.item.scope.is_none() {
                        generate_rs_enum(out, en);
                    }
                }

                // Top-level unions and structs
                for rec in &schema.records {
                    if rec.scope().is_none() {
                        match rec {
                            RecordDef::Union(u) => generate_rs_union(out, schema, u, false),
                            RecordDef::Struct(st) => generate_rs_struct(out, schema, st, false),
                        }
                    }
                }

                // Group submodule items by top-level module name
                let mut mod_names: BTreeMap<&str, ()> = BTreeMap::new();
                for en in &schema.enums {
                    if let Some(ns) = en.item.ns() {
                        let top_mod = ns.split("::").next().unwrap();
                        mod_names.insert(top_mod, ());
                    }
                }
                for rec in &schema.records {
                    if let Some(ns) = rec.ns() {
                        let top_mod = ns.split("::").next().unwrap();
                        mod_names.insert(top_mod, ());
                    }
                }

                for (mod_idx, &top_mod) in mod_names.keys().enumerate() {
                    if mod_idx > 0 {
                        out.line();
                    }
                    let mut sub_mods: BTreeMap<&str, &str> = BTreeMap::new();
                    for en in &schema.enums {
                        if let Some(ns) = en.item.ns() {
                            if let Some(rest) = ns.strip_prefix(&format!("{top_mod}::")) {
                                sub_mods.insert(rest, ns);
                            }
                        }
                    }

                    quote_in! { *out =>
                        pub mod $top_mod {$['\r']
                            use super::*;$['\n']
                            $(ref out {
                                // Enums directly in top_mod
                                for en in &schema.enums {
                                    if en.item.ns() == Some(top_mod) {
                                        generate_rs_enum(out, en);
                                    }
                                }

                                // Nested sub-submodules (e.g. `bridge_type::callable`)
                                for (&sub_mod, &full_ns) in &sub_mods {
                                    quote_in! { *out =>
                                        pub mod $sub_mod {$['\r']
                                            $(ref out {
                                                for en in &schema.enums {
                                                    if en.item.ns() == Some(full_ns) {
                                                        generate_rs_enum(out, en);
                                                    }
                                                }
                                            })
                                        }$['\n']
                                    }
                                }

                                // Unions and structs directly in top_mod
                                for rec in &schema.records {
                                    if rec.ns() == Some(top_mod) {
                                        match rec {
                                            RecordDef::Union(u) => {
                                                generate_rs_union(out, schema, u, true);
                                            }
                                            RecordDef::Struct(st) => {
                                                generate_rs_struct(out, schema, st, true);
                                            }
                                        }
                                    }
                                }
                            })
                        }$['\r']
                    }
                }
            })
        }$['\r']
        #[rustfmt::skip]$['\r']
        pub use __defs::*;$['\r']
    }

    format_tokens(&out, false)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Usage: ir_codegen <out_cc_header> <out_rs_file>");
    let s = ir_schema::schema();
    fs::write(&args[1], generate_cc(&s)).expect("Failed to write generated C++ header");
    fs::write(&args[2], generate_rs(&s)).expect("Failed to write generated Rust file");
}
