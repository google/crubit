// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Code generator for `#[repr(C)]` IR types shared in-memory between C++ (`ir_generated.h`)
//! and Rust (`ir_generated.rs`).
//!
//! Run `common/golden_update.sh` after modifying the schema in this file.

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;

#[derive(Clone, Debug)]
enum FieldType {
    Unit,
    Bool,
    I32,
    I64,
    U64,
    Usize,
    ItemId,
    LifetimeId,
    BazelLabel,
    Str,
    Bytes,
    Enum(&'static str),
    Struct(&'static str),
    Box(Box<FieldType>),
    Vec(Box<FieldType>),
    Map(Box<FieldType>, Box<FieldType>),
}

impl FieldType {
    fn cc_storage_type(&self) -> String {
        match self {
            Self::Unit => unreachable!("Unit has no C++ storage field"),
            Self::Bool => "bool".to_string(),
            Self::I32 | Self::LifetimeId => "int32_t".to_string(),
            Self::I64 => "int64_t".to_string(),
            Self::U64 | Self::ItemId => "uint64_t".to_string(),
            Self::Usize => "size_t".to_string(),
            Self::Str | Self::BazelLabel => "FfiStr".to_string(),
            Self::Bytes => "FfiVec<uint8_t>".to_string(),
            Self::Enum(path) | Self::Struct(path) => format!("::crubit::ffi::{path}"),
            Self::Box(inner) => format!("FfiBox<{}>", inner.cc_storage_type()),
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
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Usize => "usize".to_string(),
            Self::ItemId => "ItemId".to_string(),
            Self::LifetimeId => "LifetimeId".to_string(),
            Self::BazelLabel => "BazelLabel".to_string(),
            Self::Str => "FfiStr".to_string(),
            Self::Bytes => "FfiVec<u8>".to_string(),
            Self::Enum(path) | Self::Struct(path) => format!("{prefix}{path}"),
            Self::Box(inner) => format!("FfiBox<{}>", inner.rs_storage_type(in_submodule)),
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
            Self::Unit => "()".to_string(),
            Self::Bool => "bool".to_string(),
            Self::I32 => "i32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::Usize => "usize".to_string(),
            Self::ItemId => "ItemId".to_string(),
            Self::LifetimeId => "LifetimeId".to_string(),
            Self::BazelLabel => "&BazelLabel".to_string(),
            Self::Str => "&str".to_string(),
            Self::Bytes => "&[u8]".to_string(),
            Self::Enum(path) => format!("{prefix}{path}"),
            Self::Struct("StatusOrOptionalString") => "Result<Option<&str>, &str>".to_string(),
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
            Self::Unit => unreachable!("Unit is only used in oneofs"),
            Self::Bool => {
                if default_true {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Self::I32 | Self::I64 | Self::U64 | Self::Usize => "0".to_string(),
            Self::ItemId => "ItemId::EMPTY".to_string(),
            Self::LifetimeId => "LifetimeId::EMPTY".to_string(),
            Self::BazelLabel => "BazelLabel::EMPTY".to_string(),
            Self::Str => "FfiStr::EMPTY".to_string(),
            Self::Enum(path) | Self::Struct(path) => format!("{prefix}{path}::EMPTY"),
            Self::Box(_) => "FfiBox::EMPTY".to_string(),
            Self::Vec(_) | Self::Bytes => "FfiVec::EMPTY".to_string(),
            Self::Map(_, _) => "FfiMap::EMPTY".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct EnumVariantDef {
    rs_name: &'static str,
    cc_name: &'static str,
    discriminant: i32,
}

#[derive(Clone, Debug)]
struct EnumDef {
    scope: Option<(&'static str, &'static str)>,
    name: &'static str,
    variants: Vec<EnumVariantDef>,
}

#[derive(Clone, Debug)]
struct UnionVariantDef {
    pascal_name: &'static str,
    field_name: &'static str,
    ty: FieldType,
}

#[derive(Clone, Debug)]
struct UnionDef {
    scope: Option<(&'static str, &'static str)>,
    name: &'static str,
    variants: Vec<UnionVariantDef>,
}

#[derive(Clone, Debug)]
struct FieldDef {
    cc_name: &'static str,
    rs_name: &'static str,
    ty: FieldType,
    optional: bool,
    default_true: bool,
}

#[derive(Clone, Debug)]
struct StructDef {
    scope: Option<(&'static str, &'static str)>,
    name: &'static str,
    fields: Vec<FieldDef>,
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
        FieldType::ItemId
    };
    (LifetimeId) => {
        FieldType::LifetimeId
    };
    (BazelLabel) => {
        FieldType::BazelLabel
    };
    (Str) => {
        FieldType::Str
    };
    (Bytes) => {
        FieldType::Bytes
    };
    (Enum($p:literal)) => {
        FieldType::Enum($p)
    };
    (Struct($p:literal)) => {
        FieldType::Struct($p)
    };
    (Box($inner:tt $( ( $($args:tt)* ) )?)) => {
        FieldType::Box(Box::new(field_ty!($inner $( ( $($args)* ) )?)))
    };
    (Vec($inner:tt $( ( $($args:tt)* ) )?)) => {
        FieldType::Vec(Box::new(field_ty!($inner $( ( $($args)* ) )?)))
    };
    (Map($k:tt $( ( $($kargs:tt)* ) )?, $v:tt $( ( $($vargs:tt)* ) )?)) => {
        FieldType::Map(
            Box::new(field_ty!($k $( ( $($kargs)* ) )?)),
            Box::new(field_ty!($v $( ( $($vargs)* ) )?)),
        )
    };
}

macro_rules! ir_schema_item {
    (
        $enums:ident, $records:ident;
        $(#[scope($ens:literal, parent = $eparent:literal)])?
        enum $ename:ident {
            $( $ers:ident = $ecc:ident ),* $(,)?
        }
    ) => {
        $enums.push(EnumDef {
            scope: None $( .or(Some(($ens, $eparent))) )?,
            name: stringify!($ename),
            variants: [
                $( (stringify!($ers), stringify!($ecc)) ),*
            ]
            .into_iter()
            .enumerate()
            .map(|(i, (rs_name, cc_name))| EnumVariantDef {
                rs_name,
                cc_name,
                discriminant: i as i32,
            })
            .collect(),
        });
    };
    (
        $enums:ident, $records:ident;
        $(#[scope($uns:literal, parent = $uparent:literal)])?
        enum $uname:ident {
            $( $vpascal:ident ( $vfield:ident : $vty:tt $( ( $($vargs:tt)* ) )? ) ),* $(,)?
        }
    ) => {
        $records.push(RecordDef::Union(UnionDef {
            scope: None $( .or(Some(($uns, $uparent))) )?,
            name: stringify!($uname),
            variants: vec![
                $(
                    UnionVariantDef {
                        pascal_name: stringify!($vpascal),
                        field_name: stringify!($vfield),
                        ty: field_ty!($vty $( ( $($vargs)* ) )?),
                    }
                ),*
            ],
        }));
    };
    (
        $enums:ident, $records:ident;
        $(#[scope($sns:literal, parent = $sparent:literal)])?
        struct $sname:ident {
            $(
                $(#[optional $( $opt_unused:tt )?])?
                $(#[default_true $( $dt_unused:tt )?])?
                $fcc:ident : $fty:tt $( ( $($fargs:tt)* ) )?
            ),* $(,)?
        }
    ) => {
        $records.push(RecordDef::Struct(StructDef {
            scope: None $( .or(Some(($sns, $sparent))) )?,
            name: stringify!($sname),
            fields: vec![
                $(
                    {
                        let cc_name = stringify!($fcc);
                        FieldDef {
                            cc_name,
                            rs_name: if cc_name == "type" { "type_" } else { cc_name },
                            ty: field_ty!($fty $( ( $($fargs)* ) )?),
                            optional: false $( || { $( $opt_unused )? true } )?,
                            default_true: false $( || { $( $dt_unused )? true } )?,
                        }
                    }
                ),*
            ],
        }));
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
        schema.resolve_enum_types();
        schema
    }};
}

fn schema() -> Schema {
    ir_schema! {
        enum SafetyAnnotation {
            Unannotated = SAFETY_ANNOTATION_UNANNOTATED,
            DisableUnsafe = SAFETY_ANNOTATION_DISABLE_UNSAFE,
            Unsafe = SAFETY_ANNOTATION_UNSAFE,
        }
        enum PointerTypeKind {
            LValueRef = L_VALUE_REF,
            RValueRef = R_VALUE_REF,
            Nullable = NULLABLE,
            NonNull = NON_NULL,
            Owned = OWNED,
        }
        enum CallingConv {
            C = C_DECL,
            X86FastCall = FAST_CALL,
            X86VectorCall = VECTOR_CALL,
            X86ThisCall = THIS_CALL,
            X86StdCall = STD_CALL,
            Win64 = MS_ABI,
        }
        enum Primitive {
            Bool = BOOL,
            Void = VOID,
            Float = FLOAT,
            Double = DOUBLE,
            Char = CHAR,
            SignedChar = SIGNED_CHAR,
            UnsignedChar = UNSIGNED_CHAR,
            Short = SHORT,
            Int = INT,
            Long = LONG,
            LongLong = LONG_LONG,
            Int128 = INT128,
            UnsignedShort = UNSIGNED_SHORT,
            UnsignedInt = UNSIGNED_INT,
            UnsignedLong = UNSIGNED_LONG,
            UnsignedLongLong = UNSIGNED_LONG_LONG,
            UnsignedInt128 = UNSIGNED_INT128,
            Char16T = CHAR16_T,
            Char32T = CHAR32_T,
            PtrdiffT = PTRDIFF_T,
            IntptrT = INTPTR_T,
            SizeT = SIZE_T,
            UintptrT = UINTPTR_T,
            StdPtrdiffT = STD_PTRDIFF_T,
            StdIntptrT = STD_INTPTR_T,
            StdSizeT = STD_SIZE_T,
            StdUintptrT = STD_UINTPTR_T,
            Int8T = INT8_T,
            Int16T = INT16_T,
            Int32T = INT32_T,
            Int64T = INT64_T,
            StdInt8T = STD_INT8_T,
            StdInt16T = STD_INT16_T,
            StdInt32T = STD_INT32_T,
            StdInt64T = STD_INT64_T,
            Uint8T = UINT8_T,
            Uint16T = UINT16_T,
            Uint32T = UINT32_T,
            Uint64T = UINT64_T,
            StdUint8T = STD_UINT8_T,
            StdUint16T = STD_UINT16_T,
            StdUint32T = STD_UINT32_T,
            StdUint64T = STD_UINT64_T,
        }
        #[scope("instance_method_metadata", parent = "InstanceMethodMetadata")]
        enum ReferenceQualification {
            Unqualified = UNQUALIFIED,
            LValue = L_VALUE,
            RValue = R_VALUE,
        }
        enum AccessSpecifier {
            Public = PUBLIC,
            Protected = PROTECTED,
            Private = PRIVATE,
        }
        enum SpecialMemberFunc {
            Trivial = TRIVIAL,
            NontrivialMembers = NONTRIVIAL_MEMBERS,
            NontrivialUserDefined = NONTRIVIAL_USER_DEFINED,
            Unavailable = UNAVAILABLE,
        }
        enum RecordType {
            Struct = STRUCT,
            Union = UNION,
            Class = CLASS,
        }
        #[scope("bridge_type::callable", parent = "bridge_type::Callable")]
        enum BackingType {
            DynCallable = DYN_CALLABLE,
            AnyInvocable = ANY_INVOCABLE,
        }
        #[scope("bridge_type::callable", parent = "bridge_type::Callable")]
        enum FnTrait {
            Fn = FN,
            FnMut = FN_MUT,
            FnOnce = FN_ONCE,
        }
        enum TraitImplPolarity {
            None = NONE,
            Negative = NEGATIVE,
            Positive = POSITIVE,
        }
        #[scope("unsupported_item", parent = "UnsupportedItem")]
        enum Kind {
            Other = OTHER,
            Func = FUNC,
            GlobalVar = GLOBAL_VAR,
            Struct = STRUCT,
            Union = UNION,
            Class = CLASS,
            Enum = ENUM,
            TypeAlias = TYPE_ALIAS,
            Namespace = NAMESPACE,
            Constructor = CONSTRUCTOR,
        }
        struct HeaderName {
            name: Str,
        }
        struct LifetimeName {
            name: Str,
            id: LifetimeId,
        }
        struct FormattedError {
            fmt: Str,
            message: Str,
        }
        #[scope("cc_type", parent = "CcType")]
        struct PointerType {
            kind: Enum("PointerTypeKind"),
            #[optional]
            lifetime: LifetimeId,
            pointee_type: Box(Struct("CcType")),
        }
        #[scope("cc_type", parent = "CcType")]
        struct FuncPointer {
            non_null: Bool,
            call_conv: Enum("CallingConv"),
            param_and_return_types: Vec(Struct("CcType")),
            lifetime_inputs: Vec(Str),
        }
        #[scope("cc_type", parent = "CcType")]
        enum Variant {
            Primitive(primitive: Enum("Primitive")),
            Pointer(pointer: Struct("cc_type::PointerType")),
            FuncPointer(func_pointer: Struct("cc_type::FuncPointer")),
            Decl(decl: ItemId),
            Error(error: Struct("FormattedError")),
        }
        struct CcType {
            variant: Enum("cc_type::Variant"),
            is_const: Bool,
            is_nonnull: Bool,
            unknown_attr: Str,
            explicit_lifetimes: Vec(Str),
            template_args: Vec(Struct("CcType")),
        }
        struct Identifier {
            identifier: Str,
        }
        struct IntegerConstant {
            is_negative: Bool,
            wrapped_value: U64,
        }
        struct Operator {
            name: Str,
        }
        enum UnqualifiedIdentifier {
            Identifier(ident: Struct("Identifier")),
            Operator(oper: Struct("Operator")),
            Constructor(constructor: Unit),
            Destructor(destructor: Unit),
            ConversionOperator(conversion_operator: Unit),
        }
        struct FuncParam {
            type: Struct("CcType"),
            identifier: Struct("Identifier"),
            clang_lifetime_capture_by: Vec(I32),
            clang_lifetimebound: Bool,
            #[optional]
            unknown_attr: Str,
        }
        struct InstanceMethodMetadata {
            reference: Enum("instance_method_metadata::ReferenceQualification"),
            is_const: Bool,
            is_virtual: Bool,
        }
        #[scope("member_func_semantic", parent = "MemberFuncSemantic")]
        struct Setter {
            type: Struct("CcType"),
            offset: Usize,
        }
        #[scope("member_func_semantic", parent = "MemberFuncSemantic")]
        struct Getter {
            type: Struct("CcType"),
            offset: Usize,
        }
        enum MemberFuncSemantic {
            Setter(setter: Struct("member_func_semantic::Setter")),
            Getter(getter: Struct("member_func_semantic::Getter")),
        }
        struct Func {
            cc_name: Enum("UnqualifiedIdentifier"),
            rs_name: Enum("UnqualifiedIdentifier"),
            unique_name: Str,
            owning_target: BazelLabel,
            mangled_name: Str,
            #[optional]
            doc_comment: Str,
            return_type: Struct("CcType"),
            params: Vec(Struct("FuncParam")),
            lifetime_params: Vec(Struct("LifetimeName")),
            is_inline: Bool,
            is_always_inline: Bool,
            #[optional]
            instance_method_metadata: Struct("InstanceMethodMetadata"),
            is_extern_c: Bool,
            is_noreturn: Bool,
            is_variadic: Bool,
            is_consteval: Bool,
            #[optional]
            nodiscard: Str,
            #[optional]
            deprecated: Str,
            #[optional]
            unknown_attr: Str,
            #[optional]
            call_conv: Enum("CallingConv"),
            is_member_or_descendant_of_class_template: Bool,
            safety_annotation: Enum("SafetyAnnotation"),
            source_loc: Str,
            id: ItemId,
            #[optional]
            enclosing_item_id: ItemId,
            #[optional]
            adl_enclosing_record: ItemId,
            must_bind: Bool,
            #[optional]
            inline_cpp_source_text: Str,
            lifetime_inputs: Vec(Str),
            #[optional]
            semantic: Enum("MemberFuncSemantic"),
            is_pub_crate: Bool,
        }
        enum StatusOrOptionalString {
            None(none: Unit),
            OkValue(ok_value: Str),
            Err(err: Str),
        }
        struct Field {
            #[optional]
            rust_identifier: Struct("Identifier"),
            #[optional]
            cpp_identifier: Struct("Identifier"),
            #[optional]
            doc_comment: Str,
            type: Struct("CcType"),
            access: Enum("AccessSpecifier"),
            offset: Usize,
            size: Usize,
            unknown_attr: Enum("StatusOrOptionalString"),
            is_no_unique_address: Bool,
            is_bitfield: Bool,
            is_inheritable: Bool,
            is_mutable: Bool,
            #[optional]
            deprecated: Str,
        }
        struct BaseClass {
            base_record_id: ItemId,
            #[optional]
            offset: I64,
        }
        struct SizeAlign {
            size: Usize,
            alignment: Usize,
        }
        enum TemplateArg {
            Type(type: Struct("CcType")),
            Bool(bool_value: Bool),
            Int(int_value: I64),
        }
        #[scope("bridge_type", parent = "BridgeType")]
        struct Bridge {
            rust_name: Str,
            abi_rust: Str,
            abi_cpp: Str,
            template_args: Vec(Struct("CcType")),
            #[optional]
            label_hint: Str,
        }
        #[scope("bridge_type", parent = "BridgeType")]
        struct StdOptional {
            inner_type: Struct("CcType"),
        }
        #[scope("bridge_type", parent = "BridgeType")]
        struct StdPair {
            first_type: Struct("CcType"),
            second_type: Struct("CcType"),
        }
        #[scope("bridge_type", parent = "BridgeType")]
        struct ProtoMessageBridge {
            rust_name: Str,
        }
        #[scope("bridge_type", parent = "BridgeType")]
        struct Callable {
            backing_type: Enum("bridge_type::callable::BackingType"),
            fn_trait: Enum("bridge_type::callable::FnTrait"),
            return_type: Struct("CcType"),
            param_types: Vec(Struct("CcType")),
        }
        enum BridgeType {
            Bridge(bridge: Struct("bridge_type::Bridge")),
            StdOptional(std_optional: Struct("bridge_type::StdOptional")),
            StdPair(std_pair: Struct("bridge_type::StdPair")),
            StdString(std_string: Unit),
            ProtoMessageBridge(proto_message_bridge: Struct("bridge_type::ProtoMessageBridge")),
            Callable(callable: Struct("bridge_type::Callable")),
        }
        #[scope("array_value", parent = "ArrayValue")]
        struct IntegerElements {
            elements: Vec(Struct("IntegerConstant")),
            #[optional]
            filler: Struct("IntegerConstant"),
        }
        #[scope("array_value", parent = "ArrayValue")]
        struct ArrayElements {
            elements: Vec(Struct("ArrayValue")),
            #[optional]
            filler: Box(Struct("ArrayValue")),
        }
        #[scope("array_value", parent = "ArrayValue")]
        enum Kind {
            Integers(integer_elements: Struct("array_value::IntegerElements")),
            Arrays(array_elements: Struct("array_value::ArrayElements")),
            StringLiteral(string_literal: Bytes),
        }
        struct ArrayValue {
            kind: Enum("array_value::Kind"),
            length: U64,
        }
        #[scope("constant", parent = "Constant")]
        enum Kind {
            Integer(value: Struct("IntegerConstant")),
            Array(array_value: Struct("ArrayValue")),
            StringView(string_view_value: Bytes),
        }
        struct Constant {
            kind: Enum("constant::Kind"),
            cc_name: Struct("Identifier"),
            rs_name: Struct("Identifier"),
            unique_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            source_loc: Str,
            #[optional]
            unknown_attr: Str,
            #[optional]
            enclosing_item_id: ItemId,
            #[optional]
            type: Struct("CcType"),
            must_bind: Bool,
            #[optional]
            deprecated: Str,
            #[optional]
            doc_comment: Str,
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        struct StdVector {
            element_type: Struct("CcType"),
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        struct StdOptional {
            element_type: Struct("CcType"),
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        struct StdSharedPtr {
            element_type: Struct("CcType"),
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        struct StdUniquePtr {
            element_type: Struct("CcType"),
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        struct AbslSpan {
            element_type: Struct("CcType"),
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        struct C9Co {
            element_type: Struct("CcType"),
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        struct StdAtomic {
            element_type: Struct("CcType"),
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        struct AbslFlatHashMap {
            key_type: Struct("CcType"),
            value_type: Struct("CcType"),
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        struct AbslFlatHashSet {
            element_type: Struct("CcType"),
        }
        #[scope("template_specialization", parent = "TemplateSpecialization")]
        enum Kind {
            StdStringView(std_string_view: Unit),
            StdWStringView(std_w_string_view: Unit),
            StdVector(std_vector: Struct("template_specialization::StdVector")),
            StdOptional(std_optional: Struct("template_specialization::StdOptional")),
            StdSharedPtr(std_shared_ptr: Struct("template_specialization::StdSharedPtr")),
            StdUniquePtr(std_unique_ptr: Struct("template_specialization::StdUniquePtr")),
            AbslSpan(absl_span: Struct("template_specialization::AbslSpan")),
            AbslFlatHashMap(absl_flat_hash_map: Struct("template_specialization::AbslFlatHashMap")),
            AbslFlatHashSet(absl_flat_hash_set: Struct("template_specialization::AbslFlatHashSet")),
            C9Co(c9_co: Struct("template_specialization::C9Co")),
            StdAtomic(std_atomic: Struct("template_specialization::StdAtomic")),
            NonSpecial(non_special: Unit),
        }
        struct TemplateSpecialization {
            defining_target: BazelLabel,
            kind: Enum("template_specialization::Kind"),
        }
        struct TraitDerives {
            clone: Enum("TraitImplPolarity"),
            copy: Enum("TraitImplPolarity"),
            debug: Enum("TraitImplPolarity"),
            send: Enum("TraitImplPolarity"),
            sync: Enum("TraitImplPolarity"),
            custom: Vec(Str),
        }
        struct OwnedPtrConfig {
            owned_ptr_type: Str,
            drop_impl: Str,
        }
        struct Record {
            rs_name: Struct("Identifier"),
            cc_name: Struct("Identifier"),
            unique_name: Str,
            mangled_cc_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            #[optional]
            template_specialization: Struct("TemplateSpecialization"),
            #[optional]
            unknown_attr: Str,
            #[optional]
            doc_comment: Str,
            #[optional]
            bridge_type: Enum("BridgeType"),
            #[optional]
            owned_ptr_config: Struct("OwnedPtrConfig"),
            source_loc: Str,
            unambiguous_public_bases: Vec(Struct("BaseClass")),
            fields: Vec(Struct("Field")),
            lifetime_params: Vec(Struct("LifetimeName")),
            size_align: Struct("SizeAlign"),
            trait_derives: Struct("TraitDerives"),
            is_derived_class: Bool,
            has_virtual_bases: Bool,
            override_alignment: Bool,
            #[optional]
            pack: Usize,
            safety_annotation: Enum("SafetyAnnotation"),
            copy_constructor: Enum("SpecialMemberFunc"),
            move_constructor: Enum("SpecialMemberFunc"),
            destructor: Enum("SpecialMemberFunc"),
            is_rust_movable: Bool,
            is_inheritable: Bool,
            is_abstract: Bool,
            #[optional]
            nodiscard: Str,
            record_type: Enum("RecordType"),
            is_aggregate: Bool,
            is_canonical_alias: Bool,
            #[optional]
            enclosing_item_id: ItemId,
            must_bind: Bool,
            overloads_operator_delete: Bool,
            has_private_or_deleted_operator_delete: Bool,
            lifetime_inputs: Vec(Str),
            impl_debug: Bool,
            has_private_pointer_or_reference_fields: Bool,
            has_template_argument_lifetime: Bool,
            detected_formatter: Bool,
            #[optional]
            deprecated: Str,
            is_thread_safe: Bool,
            is_explicit_class_template_instantiation_definition: Bool,
            children: Vec(Enum("Item")),
        }
        struct IncompleteRecord {
            cc_name: Struct("Identifier"),
            rs_name: Struct("Identifier"),
            unique_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            #[optional]
            unknown_attr: Str,
            record_type: Enum("RecordType"),
            #[optional]
            enclosing_item_id: ItemId,
            must_bind: Bool,
        }
        struct Enumerator {
            identifier: Struct("Identifier"),
            value: Struct("IntegerConstant"),
            #[optional]
            unknown_attr: Str,
            #[optional]
            deprecated: Str,
            #[optional]
            doc_comment: Str,
        }
        struct Enum {
            cc_name: Struct("Identifier"),
            rs_name: Struct("Identifier"),
            unique_name: Str,
            mangled_cc_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            source_loc: Str,
            underlying_type: Struct("CcType"),
            enumerators: Vec(Struct("Enumerator")),
            #[optional]
            unknown_attr: Str,
            #[optional]
            enclosing_item_id: ItemId,
            must_bind: Bool,
            detected_formatter: Bool,
            #[optional]
            nodiscard: Str,
            #[optional]
            deprecated: Str,
            #[optional]
            doc_comment: Str,
            is_incomplete: Bool,
        }
        struct GlobalVar {
            cc_name: Struct("Identifier"),
            rs_name: Struct("Identifier"),
            unique_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            source_loc: Str,
            #[optional]
            unknown_attr: Str,
            #[optional]
            enclosing_item_id: ItemId,
            #[optional]
            mangled_name: Str,
            type: Struct("CcType"),
            must_bind: Bool,
            #[optional]
            deprecated: Str,
            #[optional]
            doc_comment: Str,
        }
        struct TypeAlias {
            cc_name: Struct("Identifier"),
            rs_name: Struct("Identifier"),
            unique_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            #[optional]
            doc_comment: Str,
            #[optional]
            unknown_attr: Str,
            underlying_type: Struct("CcType"),
            source_loc: Str,
            #[optional]
            enclosing_item_id: ItemId,
            must_bind: Bool,
            #[optional]
            deprecated: Str,
            lifetime_inputs: Vec(Str),
        }
        #[scope("unsupported_item", parent = "UnsupportedItem")]
        struct Path {
            ident: Enum("UnqualifiedIdentifier"),
            #[optional]
            enclosing_item_id: ItemId,
        }
        struct UnsupportedItem {
            name: Str,
            #[optional]
            unique_name: Str,
            kind: Enum("unsupported_item::Kind"),
            #[optional]
            path: Struct("unsupported_item::Path"),
            errors: Vec(Struct("FormattedError")),
            #[optional]
            source_loc: Str,
            #[optional]
            inline_cpp_source_text: Str,
            id: ItemId,
            must_bind: Bool,
            #[optional]
            defining_target: BazelLabel,
        }
        struct Comment {
            text: Str,
            id: ItemId,
            must_bind: Bool,
        }
        struct Namespace {
            cc_name: Struct("Identifier"),
            rs_name: Struct("Identifier"),
            unique_name: Str,
            id: ItemId,
            canonical_namespace_id: ItemId,
            #[optional]
            unknown_attr: Str,
            owning_target: BazelLabel,
            #[optional]
            enclosing_item_id: ItemId,
            is_inline: Bool,
            must_bind: Bool,
            #[optional]
            deprecated: Str,
            #[optional]
            doc_comment: Str,
            children: Vec(Enum("Item")),
        }
        struct UseMod {
            path: Str,
            mod_name: Struct("Identifier"),
            id: ItemId,
            must_bind: Bool,
        }
        struct ExistingRustType {
            rs_name: Str,
            cc_name: Str,
            unique_name: Str,
            template_args: Vec(Enum("TemplateArg")),
            owning_target: BazelLabel,
            #[optional]
            size_align: Struct("SizeAlign"),
            is_same_abi: Bool,
            id: ItemId,
            must_bind: Bool,
            impl_debug: Bool,
            #[optional]
            label_hint: Str,
            #[default_true]
            is_trivially_copyable: Bool,
        }
        enum Item {
            Func(func: Box(Struct("Func"))),
            Record(record: Box(Struct("Record"))),
            IncompleteRecord(incomplete_record: Box(Struct("IncompleteRecord"))),
            Enum(enum_decl: Box(Struct("Enum"))),
            Constant(constant: Box(Struct("Constant"))),
            TypeAlias(type_alias: Box(Struct("TypeAlias"))),
            GlobalVar(global_var: Box(Struct("GlobalVar"))),
            UnsupportedItem(unsupported_item: Box(Struct("UnsupportedItem"))),
            Comment(comment: Box(Struct("Comment"))),
            Namespace(namespace_decl: Box(Struct("Namespace"))),
            UseMod(use_mod: Box(Struct("UseMod"))),
            ExistingRustType(existing_rust_type: Box(Struct("ExistingRustType"))),
        }
        struct IRProto {
            public_headers: Vec(Struct("HeaderName")),
            current_target: Str,
            #[optional]
            crate_root_path: Str,
            crubit_features: Map(Str, Vec(Str)),
            unstable_rust_features: Vec(Str),
            reexported_namespaces: Vec(Str),
            top_level_items: Map(Str, Vec(Enum("Item"))),
            crate_names: Map(Str, Str),
        }
        struct GenerateBindingsRequest {
            ir: Struct("IRProto"),
            crubit_support_path_format: Str,
            crubit_support_versioned_path_format: Str,
            clang_format_exe_path: Str,
            rustfmt_exe_path: Str,
            rustfmt_config_path: Str,
            generate_error_report: Bool,
            is_golden_test: Bool,
            kythe_annotations: Bool,
            kythe_default_corpus: Str,
        }
        struct GenerateBindingsResponse {
            rs_api: Str,
            rs_api_impl: Str,
            error_report: Str,
            fatal_errors: Str,
        }
    }
}

impl EnumDef {
    fn cc_qual_name(&self) -> String {
        if let Some((ns, _)) = self.scope {
            format!("{ns}::{}", self.name)
        } else {
            self.name.to_string()
        }
    }
}

impl UnionDef {
    fn cc_qual_name(&self) -> String {
        if let Some((ns, _)) = self.scope {
            format!("{ns}::{}", self.name)
        } else {
            self.name.to_string()
        }
    }
}

impl StructDef {
    fn cc_qual_name(&self) -> String {
        if let Some((ns, _)) = self.scope {
            format!("{ns}::{}", self.name)
        } else {
            self.name.to_string()
        }
    }

    fn has_optional_fields(&self) -> bool {
        self.fields.iter().any(|f| f.optional)
    }
}

impl RecordDef {
    fn scope(&self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Union(u) => u.scope,
            Self::Struct(st) => st.scope,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Union(u) => u.name,
            Self::Struct(st) => st.name,
        }
    }

    fn cc_qual_name(&self) -> String {
        match self {
            Self::Union(u) => u.cc_qual_name(),
            Self::Struct(st) => st.cc_qual_name(),
        }
    }
}

fn align_up(offset: usize, align: usize) -> usize {
    (offset + align - 1) & !(align - 1)
}

impl FieldType {
    fn resolve(&mut self, c_enums: &[EnumDef], records: &[RecordDef]) {
        match self {
            Self::Enum(path) => {
                if c_enums.iter().any(|e| e.cc_qual_name() == *path) {
                    return;
                }
                if records.iter().any(|r| r.cc_qual_name() == *path) {
                    *self = Self::Struct(path);
                    return;
                }
                panic!("Unknown enum type: {path}");
            }
            Self::Box(inner) | Self::Vec(inner) => inner.resolve(c_enums, records),
            Self::Map(k, v) => {
                k.resolve(c_enums, records);
                v.resolve(c_enums, records);
            }
            _ => {}
        }
    }
}

impl Schema {
    fn resolve_enum_types(&mut self) {
        let c_enums = self.enums.clone();
        let records_snapshot = self.records.clone();
        for rec in &mut self.records {
            match rec {
                RecordDef::Union(u) => {
                    for v in &mut u.variants {
                        v.ty.resolve(&c_enums, &records_snapshot);
                    }
                }
                RecordDef::Struct(st) => {
                    for f in &mut st.fields {
                        f.ty.resolve(&c_enums, &records_snapshot);
                    }
                }
            }
        }
    }

    fn is_copy_struct(&self, path: &str) -> bool {
        matches!(path, "IntegerConstant" | "InstanceMethodMetadata" | "BaseClass" | "SizeAlign")
    }

    fn is_field_forward_ref(&self, current_record_idx: usize, ty: &FieldType) -> bool {
        match ty {
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
                .any(|r| matches!(r, RecordDef::Union(u) if u.cc_qual_name() == *path))
        } else {
            false
        }
    }

    fn field_layout_64(&self, ty: &FieldType) -> (usize, usize) {
        match ty {
            FieldType::Unit => (0, 1),
            FieldType::Bool => (1, 1),
            FieldType::I32 | FieldType::LifetimeId | FieldType::Enum(_) => (4, 4),
            FieldType::I64
            | FieldType::U64
            | FieldType::Usize
            | FieldType::ItemId
            | FieldType::Box(_) => (8, 8),
            FieldType::Str | FieldType::BazelLabel => (16, 8),
            FieldType::Vec(_) | FieldType::Bytes | FieldType::Map(_, _) => (24, 8),
            FieldType::Struct(path) => {
                let rec = self
                    .records
                    .iter()
                    .find(|r| r.cc_qual_name() == *path)
                    .unwrap_or_else(|| panic!("Unknown record path: {path}"));
                match rec {
                    RecordDef::Union(u) => self.union_layout_64(u),
                    RecordDef::Struct(st) => {
                        let (size, align, _) = self.struct_layout_64(st);
                        (size, align)
                    }
                }
            }
        }
    }

    fn union_layout_64(&self, u: &UnionDef) -> (usize, usize) {
        let mut union_size = 1usize;
        let mut union_align = 1usize;
        for v in &u.variants {
            let (vsize, valign) = self.field_layout_64(&v.ty);
            union_size = union_size.max(vsize);
            union_align = union_align.max(valign);
        }
        union_size = align_up(union_size, union_align);
        let total_align = 4usize.max(union_align);
        let union_offset = align_up(4, union_align);
        let total_size = align_up(union_offset + union_size, total_align);
        (total_size, total_align)
    }

    fn struct_layout_64(
        &self,
        st: &StructDef,
    ) -> (usize, usize, Vec<(&'static str, &'static str, usize)>) {
        let (mut offset, mut max_align) =
            if st.has_optional_fields() { (8usize, 8usize) } else { (0usize, 1usize) };
        let mut field_offsets = Vec::new();
        for f in &st.fields {
            let (fsize, falign) = self.field_layout_64(&f.ty);
            offset = align_up(offset, falign);
            field_offsets.push((f.cc_name, f.rs_name, offset));
            offset += fsize;
            max_align = max_align.max(falign);
        }
        let total_size = align_up(offset, max_align);
        (total_size, max_align, field_offsets)
    }
}

fn generate_cc_nested_aliases(out: &mut String, schema: &Schema, parent_key: &str) {
    for child_rec in &schema.records {
        if let Some((child_ns, child_parent)) = child_rec.scope() {
            if child_parent == parent_key {
                let cname = child_rec.name();
                writeln!(out, "  using {cname} = ::crubit::ffi::{child_ns}::{cname};").unwrap();
            }
        }
    }
    for child_en in &schema.enums {
        if let Some((child_ns, child_parent)) = child_en.scope {
            if child_parent == parent_key {
                writeln!(
                    out,
                    "  using {} = ::crubit::ffi::{child_ns}::{};",
                    child_en.name, child_en.name
                )
                .unwrap();
                for v in &child_en.variants {
                    writeln!(
                        out,
                        "  static constexpr {} {} = ::crubit::ffi::{child_ns}::{};",
                        child_en.name, v.cc_name, v.cc_name
                    )
                    .unwrap();
                }
            }
        }
    }
}

fn generate_cc(schema: &Schema) -> String {
    let mut out = String::new();
    writeln!(out, "// Automatically @generated by ir_codegen.rs. Do not edit directly.").unwrap();
    writeln!(out, "// To regenerate, run: common/golden_update.sh").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "// clang-format off").unwrap();
    writeln!(out, "#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_").unwrap();
    writeln!(out, "#define CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#include <cstddef>").unwrap();
    writeln!(out, "#include <cstdint>").unwrap();
    writeln!(out, "#include <memory>").unwrap();
    writeln!(out, "#include <new>").unwrap();
    writeln!(out, "#include <type_traits>").unwrap();
    writeln!(out, "#include <utility>").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#include \"third_party/absl/log/check.h\"").unwrap();
    writeln!(out, "#include \"third_party/absl/strings/string_view.h\"").unwrap();
    writeln!(out, "#include \"rs_bindings_from_cc/ir_ffi_support.h\"").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "namespace crubit::ffi {{").unwrap();
    writeln!(
        out,
        "// NOLINTBEGIN(*-readability-class-member-naming,*-readability-constant-naming)"
    )
    .unwrap();
    writeln!(out).unwrap();

    // Pass 1: Forward declarations of structs/unions and definitions of C-style enums.
    for rec in &schema.records {
        if let Some((ns, _)) = rec.scope() {
            writeln!(out, "namespace {ns} {{ struct {}; }}", rec.name()).unwrap();
        } else {
            writeln!(out, "struct {};", rec.name()).unwrap();
        }
    }
    writeln!(out).unwrap();

    for en in &schema.enums {
        if let Some((ns, _)) = en.scope {
            writeln!(out, "namespace {ns} {{").unwrap();
        }
        writeln!(out, "enum {} : int32_t {{", en.name).unwrap();
        for v in &en.variants {
            writeln!(out, "  {} = {},", v.cc_name, v.discriminant).unwrap();
        }
        writeln!(out, "}};").unwrap();
        if let Some((ns, _)) = en.scope {
            writeln!(out, "}}  // namespace {ns}").unwrap();
        }
        writeln!(out).unwrap();
    }

    // Pass 2: Union and Struct definitions.
    for (rec_idx, rec) in schema.records.iter().enumerate() {
        match rec {
            RecordDef::Union(u) => {
                let uname = u.name;
                let parent_key = u.cc_qual_name();
                if let Some((ns, _)) = u.scope {
                    writeln!(out, "namespace {ns} {{").unwrap();
                }
                writeln!(out, "struct {uname} {{").unwrap();
                generate_cc_nested_aliases(&mut out, schema, &parent_key);

                let v0 = &u.variants[0];
                let v0p = v0.pascal_name;
                let v0f = v0.field_name;
                let v0_init = if matches!(v0.ty, FieldType::Unit) {
                    "empty_(0)".to_string()
                } else {
                    format!("{v0f}_()")
                };

                writeln!(out, "  enum Case : uint32_t {{").unwrap();
                for (idx, v) in u.variants.iter().enumerate() {
                    writeln!(out, "    k{} = {},", v.pascal_name, idx).unwrap();
                }
                writeln!(out, "  }};").unwrap();
                writeln!(out, "  {uname}() : tag_(k{v0p}), {v0_init} {{}}").unwrap();
                writeln!(
                    out,
                    "  {uname}(const {uname}& other) : tag_(k{v0p}), {v0_init} {{ CopyFrom(other); }}"
                )
                .unwrap();
                writeln!(
                    out,
                    "  {uname}({uname}&& other) noexcept : tag_(k{v0p}), {v0_init} {{ MoveFrom(std::move(other)); }}"
                )
                .unwrap();
                writeln!(out, "  {uname}& operator=(const {uname}& other) {{").unwrap();
                writeln!(
                    out,
                    "    if (this != &other) {{ {uname} tmp(other); MoveFrom(std::move(tmp)); }}"
                )
                .unwrap();
                writeln!(out, "    return *this;").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out, "  {uname}& operator=({uname}&& other) noexcept {{").unwrap();
                writeln!(
                    out,
                    "    if (this != &other) {{ {uname} tmp(std::move(other)); MoveFrom(std::move(tmp)); }}"
                )
                .unwrap();
                writeln!(out, "    return *this;").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out, "  ~{uname}() {{ Destroy(); }}").unwrap();
                for v in &u.variants {
                    let vf = v.field_name;
                    let vp = v.pascal_name;
                    match &v.ty {
                        FieldType::Unit => {
                            writeln!(
                                out,
                                "  static {uname} {vp}() {{ {uname} res; res.set_{vf}(); return res; }}"
                            )
                            .unwrap();
                        }
                        FieldType::Struct(cc_path) => {
                            let cty = format!("::crubit::ffi::{cc_path}");
                            writeln!(out, "  // NOLINTNEXTLINE(google-explicit-constructor)")
                                .unwrap();
                            writeln!(
                                out,
                                "  {uname}({cty} value) : tag_(k{v0p}), {v0_init} {{ *mutable_{vf}() = std::move(value); }}"
                            )
                            .unwrap();
                        }
                        FieldType::Box(inner) => {
                            let inner_cty = inner.cc_storage_type();
                            writeln!(out, "  // NOLINTNEXTLINE(google-explicit-constructor)")
                                .unwrap();
                            writeln!(
                                out,
                                "  {uname}({inner_cty} value) : tag_(k{v0p}), {v0_init} {{ *mutable_{vf}() = std::move(value); }}"
                            )
                            .unwrap();
                        }
                        FieldType::Enum(_) => {
                            let cty = v.ty.cc_storage_type();
                            writeln!(out, "  // NOLINTNEXTLINE(google-explicit-constructor)")
                                .unwrap();
                            writeln!(
                                out,
                                "  {uname}({cty} value) : tag_(k{v0p}), {v0_init} {{ set_{vf}(value); }}"
                            )
                            .unwrap();
                        }
                        _ => {}
                    }
                }
                writeln!(out, "  void Clear() {{").unwrap();
                writeln!(out, "    Destroy();").unwrap();
                writeln!(out, "    tag_ = k{v0p};").unwrap();
                if !matches!(v0.ty, FieldType::Unit) {
                    let v0_cty = v0.ty.cc_storage_type();
                    writeln!(out, "    new (&{v0f}_) {v0_cty}();").unwrap();
                }
                writeln!(out, "  }}").unwrap();
                writeln!(out, "  Case kind() const {{ return tag_; }}").unwrap();

                for v in &u.variants {
                    let vf = v.field_name;
                    let vp = v.pascal_name;
                    writeln!(out, "  bool has_{vf}() const {{ return tag_ == k{vp}; }}").unwrap();
                    match &v.ty {
                        FieldType::Unit => {
                            writeln!(out, "  void set_{vf}() {{ Destroy(); tag_ = k{vp}; }}")
                                .unwrap();
                        }
                        FieldType::Struct(cc_path) => {
                            let cty = format!("::crubit::ffi::{cc_path}");
                            writeln!(out, "  const {cty}& {vf}() const {{").unwrap();
                            writeln!(out, "    if (has_{vf}()) return {vf}_;").unwrap();
                            writeln!(out, "    static const {cty}& kDefault = *new {cty}();")
                                .unwrap();
                            writeln!(out, "    return kDefault;").unwrap();
                            writeln!(out, "  }}").unwrap();
                            writeln!(out, "  {cty}* mutable_{vf}() {{").unwrap();
                            writeln!(out, "    if (!has_{vf}()) {{").unwrap();
                            writeln!(out, "      Destroy();").unwrap();
                            writeln!(out, "      tag_ = k{vp};").unwrap();
                            writeln!(out, "      new (&{vf}_) {cty}();").unwrap();
                            writeln!(out, "    }}").unwrap();
                            writeln!(out, "    return &{vf}_;").unwrap();
                            writeln!(out, "  }}").unwrap();
                        }
                        FieldType::Box(inner) => {
                            let inner_cty = inner.cc_storage_type();
                            writeln!(out, "  const {inner_cty}& {vf}() const {{").unwrap();
                            writeln!(
                                out,
                                "    if (has_{vf}() && {vf}_.has_value()) return *{vf}_;"
                            )
                            .unwrap();
                            writeln!(
                                out,
                                "    static const {inner_cty}& kDefault = *new {inner_cty}();"
                            )
                            .unwrap();
                            writeln!(out, "    return kDefault;").unwrap();
                            writeln!(out, "  }}").unwrap();
                            writeln!(out, "  {inner_cty}* mutable_{vf}() {{").unwrap();
                            writeln!(out, "    if (!has_{vf}()) {{").unwrap();
                            writeln!(out, "      Destroy();").unwrap();
                            writeln!(out, "      tag_ = k{vp};").unwrap();
                            writeln!(out, "      new (&{vf}_) FfiBox<{inner_cty}>();").unwrap();
                            writeln!(out, "    }}").unwrap();
                            writeln!(out, "    return &{vf}_.get_or_init();").unwrap();
                            writeln!(out, "  }}").unwrap();
                        }
                        FieldType::Str | FieldType::BazelLabel => {
                            writeln!(
                                out,
                                "  absl::string_view {vf}() const {{ return has_{vf}() ? {vf}_.AsView() : absl::string_view(); }}"
                            )
                            .unwrap();
                            writeln!(out, "  void set_{vf}(absl::string_view value) {{").unwrap();
                            writeln!(out, "    FfiStr tmp(value);").unwrap();
                            writeln!(out, "    Destroy();").unwrap();
                            writeln!(out, "    tag_ = k{vp};").unwrap();
                            writeln!(out, "    new (&{vf}_) FfiStr(std::move(tmp));").unwrap();
                            writeln!(out, "  }}").unwrap();
                        }
                        FieldType::Bytes => {
                            writeln!(
                                out,
                                "  absl::string_view {vf}() const {{ return has_{vf}() ? absl::string_view(reinterpret_cast<const char*>({vf}_.data()), {vf}_.size()) : absl::string_view(); }}"
                            )
                            .unwrap();
                            writeln!(out, "  void set_{vf}(absl::string_view value) {{").unwrap();
                            writeln!(
                                out,
                                "    FfiVec<uint8_t> tmp(reinterpret_cast<const uint8_t*>(value.data()), reinterpret_cast<const uint8_t*>(value.data()) + value.size());"
                            )
                            .unwrap();
                            writeln!(out, "    Destroy();").unwrap();
                            writeln!(out, "    tag_ = k{vp};").unwrap();
                            writeln!(out, "    new (&{vf}_) FfiVec<uint8_t>(std::move(tmp));")
                                .unwrap();
                            writeln!(out, "  }}").unwrap();
                        }
                        FieldType::Bool
                        | FieldType::I32
                        | FieldType::I64
                        | FieldType::U64
                        | FieldType::Usize
                        | FieldType::ItemId
                        | FieldType::LifetimeId
                        | FieldType::Enum(_) => {
                            let cty = v.ty.cc_storage_type();
                            writeln!(
                                out,
                                "  {cty} {vf}() const {{ return has_{vf}() ? {vf}_ : static_cast<{cty}>(0); }}"
                            )
                            .unwrap();
                            writeln!(
                                out,
                                "  void set_{vf}({cty} value) {{ Destroy(); tag_ = k{vp}; {vf}_ = value; }}"
                            )
                            .unwrap();
                        }
                        _ => unreachable!("Unsupported union variant type"),
                    }
                }

                writeln!(
                    out,
                    "  friend bool operator==(const {uname}& lhs, const {uname}& rhs) {{"
                )
                .unwrap();
                writeln!(out, "    if (lhs.tag_ != rhs.tag_) return false;").unwrap();
                writeln!(out, "    switch (lhs.tag_) {{").unwrap();
                for v in &u.variants {
                    let vp = v.pascal_name;
                    let vf = v.field_name;
                    if matches!(v.ty, FieldType::Unit) {
                        writeln!(out, "      case k{vp}: return true;").unwrap();
                    } else {
                        writeln!(out, "      case k{vp}: return lhs.{vf}_ == rhs.{vf}_;").unwrap();
                    }
                }
                writeln!(out, "      default: CHECK(false);").unwrap();
                writeln!(out, "    }}").unwrap();
                writeln!(out, "    return false;").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out, " private:").unwrap();
                writeln!(out, "  void Destroy() {{").unwrap();
                writeln!(out, "    switch (tag_) {{").unwrap();
                for v in &u.variants {
                    let vp = v.pascal_name;
                    let vf = v.field_name;
                    if matches!(v.ty, FieldType::Unit) {
                        writeln!(out, "      case k{vp}: break;").unwrap();
                    } else {
                        writeln!(out, "      case k{vp}: std::destroy_at(&{vf}_); break;").unwrap();
                    }
                }
                writeln!(out, "      default: CHECK(false);").unwrap();
                writeln!(out, "    }}").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out, "  void CopyFrom(const {uname}& other) {{").unwrap();
                writeln!(out, "    Destroy();").unwrap();
                writeln!(out, "    tag_ = other.tag_;").unwrap();
                writeln!(out, "    switch (tag_) {{").unwrap();
                for v in &u.variants {
                    let vp = v.pascal_name;
                    let vf = v.field_name;
                    if matches!(v.ty, FieldType::Unit) {
                        writeln!(out, "      case k{vp}: break;").unwrap();
                    } else {
                        let cty = v.ty.cc_storage_type();
                        writeln!(out, "      case k{vp}: new (&{vf}_) {cty}(other.{vf}_); break;")
                            .unwrap();
                    }
                }
                writeln!(out, "      default: CHECK(false);").unwrap();
                writeln!(out, "    }}").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out, "  void MoveFrom({uname}&& other) noexcept {{").unwrap();
                writeln!(out, "    Destroy();").unwrap();
                writeln!(out, "    tag_ = other.tag_;").unwrap();
                writeln!(out, "    switch (tag_) {{").unwrap();
                for v in &u.variants {
                    let vp = v.pascal_name;
                    let vf = v.field_name;
                    if matches!(v.ty, FieldType::Unit) {
                        writeln!(out, "      case k{vp}: break;").unwrap();
                    } else {
                        let cty = v.ty.cc_storage_type();
                        writeln!(
                            out,
                            "      case k{vp}: new (&{vf}_) {cty}(std::move(other.{vf}_)); break;"
                        )
                        .unwrap();
                    }
                }
                writeln!(out, "      default: CHECK(false);").unwrap();
                writeln!(out, "    }}").unwrap();
                writeln!(out, "    other.Clear();").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out, "  Case tag_;").unwrap();
                writeln!(out, "  union {{").unwrap();
                writeln!(out, "    char empty_;").unwrap();
                for v in &u.variants {
                    if !matches!(v.ty, FieldType::Unit) {
                        writeln!(out, "    {} {}_;", v.ty.cc_storage_type(), v.field_name).unwrap();
                    }
                }
                writeln!(out, "  }};").unwrap();
                writeln!(out, "}};").unwrap();
                if let Some((ns, _)) = u.scope {
                    writeln!(out, "}}  // namespace {ns}").unwrap();
                }
                writeln!(out).unwrap();
            }
            RecordDef::Struct(st) => {
                let sname = st.name;
                let parent_key = st.cc_qual_name();
                let has_fwd_ref = schema.has_forward_ref(rec_idx);

                if let Some((ns, _)) = st.scope {
                    writeln!(out, "namespace {ns} {{").unwrap();
                }
                writeln!(out, "struct {sname} {{").unwrap();
                generate_cc_nested_aliases(&mut out, schema, &parent_key);

                let has_single_field_ctor = st.fields.len() == 1
                    && matches!(
                        st.fields[0].ty,
                        FieldType::Str
                            | FieldType::BazelLabel
                            | FieldType::Bool
                            | FieldType::I32
                            | FieldType::I64
                            | FieldType::U64
                            | FieldType::Usize
                            | FieldType::ItemId
                            | FieldType::LifetimeId
                            | FieldType::Enum(_)
                            | FieldType::Struct(_)
                    );
                let has_leading_union_ctor = st.fields.len() > 1
                    && matches!(st.fields[0].cc_name, "variant" | "kind")
                    && schema.is_union_type(&st.fields[0].ty);
                if has_fwd_ref {
                    writeln!(out, "  {sname}();").unwrap();
                    writeln!(out, "  {sname}(const {sname}&);").unwrap();
                    writeln!(out, "  {sname}({sname}&&) noexcept;").unwrap();
                    writeln!(out, "  {sname}& operator=(const {sname}&);").unwrap();
                    writeln!(out, "  {sname}& operator=({sname}&&) noexcept;").unwrap();
                    writeln!(out, "  ~{sname}();").unwrap();
                } else if has_single_field_ctor || has_leading_union_ctor {
                    writeln!(out, "  {sname}() = default;").unwrap();
                }
                if (has_single_field_ctor || has_leading_union_ctor)
                    && !schema.is_field_forward_ref(rec_idx, &st.fields[0].ty)
                {
                    let f = &st.fields[0];
                    let fname = f.cc_name;
                    match &f.ty {
                        FieldType::Str | FieldType::BazelLabel => {
                            writeln!(
                                out,
                                "  explicit {sname}(absl::string_view {fname}) {{ set_{fname}({fname}); }}"
                            )
                            .unwrap();
                        }
                        FieldType::Bool
                        | FieldType::I32
                        | FieldType::I64
                        | FieldType::U64
                        | FieldType::Usize
                        | FieldType::ItemId
                        | FieldType::LifetimeId
                        | FieldType::Enum(_) => {
                            let cty = f.ty.cc_storage_type();
                            writeln!(
                                out,
                                "  explicit {sname}({cty} {fname}) {{ set_{fname}({fname}); }}"
                            )
                            .unwrap();
                        }
                        FieldType::Struct(cc_path) => {
                            let cty = format!("::crubit::ffi::{cc_path}");
                            writeln!(
                                out,
                                "  explicit {sname}({cty} {fname}) {{ *mutable_{fname}() = std::move({fname}); }}"
                            )
                            .unwrap();
                        }
                        _ => unreachable!(),
                    }
                }

                // Regular field accessors
                let mut bit_idx = 0u32;
                for f in &st.fields {
                    let fname = f.cc_name;
                    let is_fwd = schema.is_field_forward_ref(rec_idx, &f.ty);
                    let set_bit = if f.optional {
                        let mask = 1u64 << bit_idx;
                        bit_idx += 1;
                        writeln!(
                            out,
                            "  bool has_{fname}() const {{ return (has_bits_ & 0x{mask:x}ULL) != 0; }}"
                        )
                        .unwrap();
                        if !is_fwd {
                            writeln!(
                                out,
                                "  void clear_{fname}() {{ has_bits_ &= ~0x{mask:x}ULL; {fname}_ = {{}}; }}"
                            )
                            .unwrap();
                        }
                        format!("has_bits_ |= 0x{mask:x}ULL; ")
                    } else {
                        String::new()
                    };
                    match &f.ty {
                        FieldType::Bool
                        | FieldType::I32
                        | FieldType::I64
                        | FieldType::U64
                        | FieldType::Usize
                        | FieldType::ItemId
                        | FieldType::LifetimeId
                        | FieldType::Enum(_) => {
                            let cty = f.ty.cc_storage_type();
                            writeln!(out, "  {cty} {fname}() const {{ return {fname}_; }}")
                                .unwrap();
                            writeln!(
                                out,
                                "  void set_{fname}({cty} value) {{ {set_bit}{fname}_ = value; }}"
                            )
                            .unwrap();
                        }
                        FieldType::Str | FieldType::BazelLabel => {
                            writeln!(
                                out,
                                "  absl::string_view {fname}() const {{ return {fname}_.AsView(); }}"
                            )
                            .unwrap();
                            writeln!(
                                out,
                                "  void set_{fname}(absl::string_view value) {{ {set_bit}{fname}_ = FfiStr(value); }}"
                            )
                            .unwrap();
                        }
                        FieldType::Bytes => {
                            writeln!(
                                out,
                                "  absl::string_view {fname}() const {{ return absl::string_view(reinterpret_cast<const char*>({fname}_.data()), {fname}_.size()); }}"
                            )
                            .unwrap();
                            writeln!(
                                out,
                                "  void set_{fname}(absl::string_view value) {{ {set_bit}{fname}_ = FfiVec<uint8_t>(reinterpret_cast<const uint8_t*>(value.data()), reinterpret_cast<const uint8_t*>(value.data()) + value.size()); }}"
                            )
                            .unwrap();
                        }
                        FieldType::Struct(cc_path) => {
                            let cty = format!("::crubit::ffi::{cc_path}");
                            writeln!(out, "  const {cty}& {fname}() const {{ return {fname}_; }}")
                                .unwrap();
                            writeln!(
                                out,
                                "  {cty}* mutable_{fname}() {{ {set_bit}return &{fname}_; }}"
                            )
                            .unwrap();
                        }
                        FieldType::Box(inner) => {
                            let cty = inner.cc_storage_type();
                            if is_fwd {
                                writeln!(out, "  const {cty}& {fname}() const;").unwrap();
                                writeln!(out, "  {cty}* mutable_{fname}();").unwrap();
                            } else {
                                writeln!(out, "  const {cty}& {fname}() const {{").unwrap();
                                writeln!(out, "    if ({fname}_.has_value()) return *{fname}_;")
                                    .unwrap();
                                writeln!(out, "    static const {cty}& kDefault = *new {cty}();")
                                    .unwrap();
                                writeln!(out, "    return kDefault;").unwrap();
                                writeln!(out, "  }}").unwrap();
                                writeln!(
                                    out,
                                    "  {cty}* mutable_{fname}() {{ {set_bit}return &{fname}_.get_or_init(); }}"
                                )
                                .unwrap();
                            }
                        }
                        FieldType::Vec(inner) => {
                            let cty = f.ty.cc_storage_type();
                            writeln!(out, "  const {cty}& {fname}() const {{ return {fname}_; }}")
                                .unwrap();
                            writeln!(out, "  {cty}* mutable_{fname}() {{ return &{fname}_; }}")
                                .unwrap();
                            writeln!(out, "  void clear_{fname}() {{ {fname}_.Clear(); }}")
                                .unwrap();
                            match inner.as_ref() {
                                FieldType::Str | FieldType::BazelLabel => {
                                    writeln!(
                                        out,
                                        "  void add_{fname}(absl::string_view value) {{ {fname}_.Add(FfiStr(value)); }}"
                                    )
                                    .unwrap();
                                }
                                FieldType::Bool
                                | FieldType::I32
                                | FieldType::I64
                                | FieldType::U64
                                | FieldType::Usize
                                | FieldType::ItemId
                                | FieldType::LifetimeId
                                | FieldType::Enum(_) => {
                                    let elem_ty = inner.cc_storage_type();
                                    writeln!(
                                        out,
                                        "  void add_{fname}({elem_ty} value) {{ {fname}_.Add(value); }}"
                                    )
                                    .unwrap();
                                }
                                _ => {
                                    let elem_ty = inner.cc_storage_type();
                                    if is_fwd {
                                        writeln!(out, "  {elem_ty}* add_{fname}();").unwrap();
                                    } else {
                                        writeln!(
                                            out,
                                            "  {elem_ty}* add_{fname}() {{ return {fname}_.Add(); }}"
                                        )
                                        .unwrap();
                                    }
                                }
                            }
                        }
                        FieldType::Map(_, _) => {
                            let cty = f.ty.cc_storage_type();
                            writeln!(out, "  const {cty}& {fname}() const {{ return {fname}_; }}")
                                .unwrap();
                            writeln!(out, "  {cty}* mutable_{fname}() {{ return &{fname}_; }}")
                                .unwrap();
                        }
                        FieldType::Unit => unreachable!(),
                    }
                }
                assert!(bit_idx <= 64, "Struct {sname} has more than 64 optional fields");

                writeln!(
                    out,
                    "  friend bool operator==(const {sname}&, const {sname}&) = default;"
                )
                .unwrap();

                // Data members (exact #[repr(C)] layout matching Rust)
                if st.has_optional_fields() {
                    writeln!(out, "  uint64_t has_bits_ = 0;").unwrap();
                }
                for f in &st.fields {
                    let fname = f.cc_name;
                    let cty = f.ty.cc_storage_type();
                    if f.default_true {
                        writeln!(out, "  {cty} {fname}_ = true;").unwrap();
                    } else {
                        writeln!(out, "  {cty} {fname}_{{}};").unwrap();
                    }
                }

                writeln!(out, "}};").unwrap();
                if let Some((ns, _)) = st.scope {
                    writeln!(out, "}}  // namespace {ns}").unwrap();
                }
                writeln!(out).unwrap();
            }
        }
    }

    // Pass 3: Out-of-line inline definitions for forward-referenced structs and layout assertions.
    for (rec_idx, rec) in schema.records.iter().enumerate() {
        let qname = rec.cc_qual_name();
        match rec {
            RecordDef::Union(u) => {
                let (osize, oalign) = schema.union_layout_64(u);
                writeln!(out, "static_assert(std::is_standard_layout_v<{qname}>);").unwrap();
                writeln!(
                    out,
                    "static_assert(sizeof(size_t) != 8 || (sizeof({qname}) == {osize} && alignof({qname}) == {oalign}));"
                )
                .unwrap();
            }
            RecordDef::Struct(st) => {
                let sname = st.name;
                if schema.has_forward_ref(rec_idx) {
                    writeln!(out, "inline {qname}::{sname}() = default;").unwrap();
                    writeln!(out, "inline {qname}::{sname}(const {sname}&) = default;").unwrap();
                    writeln!(out, "inline {qname}::{sname}({sname}&&) noexcept = default;")
                        .unwrap();
                    writeln!(out, "inline {qname}& {qname}::operator=(const {sname}&) = default;")
                        .unwrap();
                    writeln!(
                        out,
                        "inline {qname}& {qname}::operator=({sname}&&) noexcept = default;"
                    )
                    .unwrap();
                    writeln!(out, "inline {qname}::~{sname}() = default;").unwrap();

                    let mut bit_idx = 0u32;
                    for f in &st.fields {
                        let set_bit = if f.optional {
                            let mask = 1u64 << bit_idx;
                            bit_idx += 1;
                            format!("has_bits_ |= 0x{mask:x}ULL; ")
                        } else {
                            String::new()
                        };
                        if !schema.is_field_forward_ref(rec_idx, &f.ty) {
                            continue;
                        }
                        let fname = f.cc_name;
                        match &f.ty {
                            FieldType::Box(inner) => {
                                let inner_ty = inner.cc_storage_type();
                                writeln!(
                                    out,
                                    "inline const {inner_ty}& {qname}::{fname}() const {{"
                                )
                                .unwrap();
                                writeln!(out, "  if ({fname}_.has_value()) return *{fname}_;")
                                    .unwrap();
                                writeln!(
                                    out,
                                    "  static const {inner_ty}& kDefault = *new {inner_ty}();"
                                )
                                .unwrap();
                                writeln!(out, "  return kDefault;").unwrap();
                                writeln!(out, "}}").unwrap();
                                writeln!(
                                    out,
                                    "inline {inner_ty}* {qname}::mutable_{fname}() {{ {set_bit}return &{fname}_.get_or_init(); }}"
                                )
                                .unwrap();
                            }
                            FieldType::Vec(inner) => {
                                let elem_ty = inner.cc_storage_type();
                                writeln!(
                                    out,
                                    "inline {elem_ty}* {qname}::add_{fname}() {{ return {fname}_.Add(); }}"
                                )
                                .unwrap();
                            }
                            FieldType::Map(_, _) => {}
                            _ => unreachable!(),
                        }
                    }
                }

                let (ssize, salign, foffsets) = schema.struct_layout_64(st);
                writeln!(out, "static_assert(std::is_standard_layout_v<{qname}>);").unwrap();
                writeln!(
                    out,
                    "static_assert(sizeof(size_t) != 8 || (sizeof({qname}) == {ssize} && alignof({qname}) == {salign}));"
                )
                .unwrap();
                for (cc_fname, _, foff) in foffsets {
                    writeln!(
                        out,
                        "static_assert(sizeof(size_t) != 8 || offsetof({qname}, {cc_fname}_) == {foff});"
                    )
                    .unwrap();
                }
            }
        }
    }

    writeln!(out).unwrap();
    writeln!(out, "// NOLINTEND(*-readability-class-member-naming,*-readability-constant-naming)")
        .unwrap();
    writeln!(out, "}}  // namespace crubit::ffi").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_").unwrap();
    out
}

fn generate_rs_enum(out: &mut String, en: &EnumDef, indent: &str) {
    let ename = en.name;
    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}#[repr(i32)]").unwrap();
    writeln!(
        out,
        "{indent}#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]"
    )
    .unwrap();
    writeln!(out, "{indent}pub enum {ename} {{").unwrap();
    for (idx, v) in en.variants.iter().enumerate() {
        if idx == 0 {
            writeln!(out, "{indent}    #[default]").unwrap();
        }
        writeln!(out, "{indent}    {} = {},", v.rs_name, v.discriminant).unwrap();
    }
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl {ename} {{").unwrap();
    writeln!(out, "{indent}    pub const EMPTY: Self = Self::{};", en.variants[0].rs_name).unwrap();
    writeln!(out, "{indent}    pub const fn from_i32(val: i32) -> Option<Self> {{").unwrap();
    writeln!(out, "{indent}        match val {{").unwrap();
    for v in &en.variants {
        writeln!(out, "{indent}            {} => Some(Self::{}),", v.discriminant, v.rs_name)
            .unwrap();
    }
    writeln!(out, "{indent}            _ => None,").unwrap();
    writeln!(out, "{indent}        }}").unwrap();
    writeln!(out, "{indent}    }}").unwrap();
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();
}

fn generate_rs_union(
    out: &mut String,
    schema: &Schema,
    u: &UnionDef,
    in_submodule: bool,
    indent: &str,
) {
    let uname = u.name;
    let prefix = if in_submodule { "super::" } else { "" };

    let v0 = &u.variants[0];
    let v0p = v0.pascal_name;
    let empty_expr = match &v0.ty {
        FieldType::Unit => format!("Self::{v0p}"),
        _ => format!("Self::{v0p}({})", v0.ty.rs_empty_expr(false, in_submodule)),
    };

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}#[repr(C, u32)]").unwrap();
    if matches!(uname, "UnqualifiedIdentifier" | "StatusOrOptionalString") {
        writeln!(out, "{indent}#[derive(Clone, PartialEq, Eq, Hash)]").unwrap();
    } else {
        writeln!(out, "{indent}#[derive(Debug, Clone, PartialEq, Eq, Hash)]").unwrap();
    }
    writeln!(out, "{indent}pub enum {uname} {{").unwrap();
    for (idx, v) in u.variants.iter().enumerate() {
        let vp = v.pascal_name;
        match &v.ty {
            FieldType::Unit => {
                writeln!(out, "{indent}    {vp} = {idx},").unwrap();
            }
            FieldType::Enum(rs_path) => {
                writeln!(out, "{indent}    {vp}({prefix}{rs_path}) = {idx},").unwrap();
            }
            _ => {
                let sty = v.ty.rs_storage_type(in_submodule);
                writeln!(out, "{indent}    {vp}({sty}) = {idx},").unwrap();
            }
        }
    }
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl Default for {uname} {{").unwrap();
    writeln!(out, "{indent}    fn default() -> Self {{").unwrap();
    writeln!(out, "{indent}        Self::EMPTY").unwrap();
    writeln!(out, "{indent}    }}").unwrap();
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl {prefix}FfiDefault for {uname} {{").unwrap();
    writeln!(out, "{indent}    const EMPTY_REF: &'static Self = &Self::EMPTY;").unwrap();
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl From<{prefix}ProtoDefault> for {uname} {{").unwrap();
    writeln!(out, "{indent}    fn from(_: {prefix}ProtoDefault) -> Self {{").unwrap();
    writeln!(out, "{indent}        Self::EMPTY").unwrap();
    writeln!(out, "{indent}    }}").unwrap();
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    if uname == "StatusOrOptionalString" {
        writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
        writeln!(out, "{indent}impl StatusOrOptionalString {{").unwrap();
        writeln!(out, "{indent}    pub fn as_result(&self) -> Result<Option<&str>, &str> {{")
            .unwrap();
        writeln!(out, "{indent}        match self {{").unwrap();
        writeln!(out, "{indent}            Self::None => Ok(None),").unwrap();
        writeln!(out, "{indent}            Self::OkValue(s) => Ok(Some(s.as_str())),").unwrap();
        writeln!(out, "{indent}            Self::Err(e) => Err(e.as_str()),").unwrap();
        writeln!(out, "{indent}        }}").unwrap();
        writeln!(out, "{indent}    }}").unwrap();
        writeln!(out, "{indent}}}").unwrap();
        writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
        writeln!(out, "{indent}impl core::fmt::Debug for StatusOrOptionalString {{").unwrap();
        writeln!(
            out,
            "{indent}    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{"
        )
        .unwrap();
        writeln!(out, "{indent}        core::fmt::Debug::fmt(&self.as_result(), f)").unwrap();
        writeln!(out, "{indent}    }}").unwrap();
        writeln!(out, "{indent}}}").unwrap();
        writeln!(out).unwrap();
    }

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl {uname} {{").unwrap();
    writeln!(out, "{indent}    pub const EMPTY: Self = {empty_expr};").unwrap();
    for v in &u.variants {
        let vf_clean = v.field_name;
        let vp = v.pascal_name;
        match &v.ty {
            FieldType::Unit => {
                writeln!(
                    out,
                    "{indent}    pub fn set_{vf_clean}(&mut self) {{ *self = Self::{vp}; }}"
                )
                .unwrap();
            }
            FieldType::Enum(rs_path) => {
                let ety = format!("{prefix}{rs_path}");
                writeln!(
                    out,
                    "{indent}    pub fn set_{vf_clean}(&mut self, val: impl Into<{ety}>) {{ *self = Self::{vp}(val.into()); }}"
                )
                .unwrap();
            }
            FieldType::Box(inner) => {
                let sub_ty = inner.rs_storage_type(in_submodule);
                writeln!(
                    out,
                    "{indent}    pub fn set_{vf_clean}(&mut self, val: impl Into<{sub_ty}>) -> &mut {sub_ty} {{ *self = Self::{vp}(FfiBox::new(val.into())); let Self::{vp}(ref mut inner) = *self else {{ unreachable!() }}; inner.as_mut().unwrap() }}"
                )
                .unwrap();
            }
            _ => {
                let sty = v.ty.rs_storage_type(in_submodule);
                writeln!(
                    out,
                    "{indent}    pub fn set_{vf_clean}(&mut self, val: impl Into<{sty}>) -> &mut {sty} {{ *self = Self::{vp}(val.into()); let Self::{vp}(ref mut inner) = *self else {{ unreachable!() }}; inner }}"
                )
                .unwrap();
            }
        }
    }
    writeln!(out, "{indent}}}").unwrap();
    let (osize, oalign) = schema.union_layout_64(u);
    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}#[cfg(target_pointer_width = \"64\")]").unwrap();
    writeln!(
        out,
        "{indent}const _: () = assert!(core::mem::size_of::<{uname}>() == {osize} && core::mem::align_of::<{uname}>() == {oalign});"
    )
    .unwrap();
    writeln!(out).unwrap();
}

fn generate_rs_struct(
    out: &mut String,
    schema: &Schema,
    st: &StructDef,
    in_submodule: bool,
    indent: &str,
) {
    let sname = st.name;
    let prefix = if in_submodule { "super::" } else { "" };
    let has_opt = st.has_optional_fields();
    let is_copy = schema.is_copy_struct(&st.cc_qual_name());

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}#[repr(C)]").unwrap();
    if is_copy {
        writeln!(out, "{indent}#[derive(Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
    } else {
        writeln!(out, "{indent}#[derive(Clone, PartialEq, Eq, Hash)]").unwrap();
    }
    writeln!(out, "{indent}pub struct {sname} {{").unwrap();
    if has_opt {
        writeln!(out, "{indent}    pub has_bits: u64,").unwrap();
    }
    for f in &st.fields {
        writeln!(out, "{indent}    pub {}: {},", f.rs_name, f.ty.rs_storage_type(in_submodule))
            .unwrap();
    }
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    if !matches!(sname, "Identifier" | "Operator" | "CcType") {
        writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
        writeln!(out, "{indent}impl core::fmt::Debug for {sname} {{").unwrap();
        writeln!(
            out,
            "{indent}    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{"
        )
        .unwrap();
        let debug_name = if sname == "Path" { "UnsupportedItemPath" } else { sname };
        writeln!(out, "{indent}        f.debug_struct(\"{debug_name}\")").unwrap();
        for f in &st.fields {
            if sname == "Enum" && f.rs_name == "is_incomplete" {
                continue;
            }
            let fname = f.rs_name;
            if f.optional || (sname == "Enum" && fname == "enumerators") {
                writeln!(out, "{indent}            .field(\"{fname}\", &self.{fname}())").unwrap();
            } else {
                writeln!(out, "{indent}            .field(\"{fname}\", &self.{fname})").unwrap();
            }
        }
        writeln!(out, "{indent}            .finish()").unwrap();
        writeln!(out, "{indent}    }}").unwrap();
        writeln!(out, "{indent}}}").unwrap();
        writeln!(out).unwrap();
    }

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl {prefix}FfiDefault for {sname} {{").unwrap();
    writeln!(out, "{indent}    const EMPTY_REF: &'static Self = &Self::EMPTY;").unwrap();
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl Default for {sname} {{").unwrap();
    writeln!(out, "{indent}    fn default() -> Self {{").unwrap();
    writeln!(out, "{indent}        Self::EMPTY").unwrap();
    writeln!(out, "{indent}    }}").unwrap();
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl From<{prefix}ProtoDefault> for {sname} {{").unwrap();
    writeln!(out, "{indent}    fn from(_: {prefix}ProtoDefault) -> Self {{").unwrap();
    writeln!(out, "{indent}        Self::EMPTY").unwrap();
    writeln!(out, "{indent}    }}").unwrap();
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl From<FfiBox<{sname}>> for {sname} {{").unwrap();
    writeln!(out, "{indent}    fn from(rc: FfiBox<{sname}>) -> Self {{").unwrap();
    writeln!(out, "{indent}        Clone::clone(&*rc)").unwrap();
    writeln!(out, "{indent}    }}").unwrap();
    writeln!(out, "{indent}}}").unwrap();
    writeln!(out).unwrap();

    // impl Struct (EMPTY, getters, and set_* setter methods)
    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}impl {sname} {{").unwrap();
    writeln!(out, "{indent}    pub const EMPTY: Self = Self {{").unwrap();
    if has_opt {
        writeln!(out, "{indent}        has_bits: 0,").unwrap();
    }
    for f in &st.fields {
        writeln!(
            out,
            "{indent}        {}: {},",
            f.rs_name,
            f.ty.rs_empty_expr(f.default_true, in_submodule)
        )
        .unwrap();
    }
    writeln!(out, "{indent}    }};").unwrap();

    let mut bit_idx = 0u32;
    for f in &st.fields {
        let fname = f.rs_name;
        let clean_name = f.cc_name;
        let rty = f.ty.rs_getter_return_type(schema, in_submodule);
        let val_expr = match &f.ty {
            FieldType::Bool
            | FieldType::I32
            | FieldType::I64
            | FieldType::U64
            | FieldType::Usize
            | FieldType::ItemId
            | FieldType::LifetimeId
            | FieldType::Enum(_) => format!("self.{fname}"),
            FieldType::BazelLabel => format!("&self.{fname}"),
            FieldType::Str => format!("self.{fname}.as_str()"),
            FieldType::Bytes | FieldType::Vec(_) => format!("self.{fname}.as_slice()"),
            FieldType::Struct("StatusOrOptionalString") => format!("self.{fname}.as_result()"),
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

        if f.optional {
            let mask = 1u64 << bit_idx;
            bit_idx += 1;
            let empty_expr = f.ty.rs_empty_expr(f.default_true, in_submodule);
            writeln!(out, "{indent}    pub fn has_{clean_name}(&self) -> bool {{").unwrap();
            writeln!(out, "{indent}        (self.has_bits & 0x{mask:x}) != 0").unwrap();
            writeln!(out, "{indent}    }}").unwrap();
            writeln!(out, "{indent}    pub fn clear_{clean_name}(&mut self) {{").unwrap();
            writeln!(out, "{indent}        self.has_bits &= !0x{mask:x};").unwrap();
            writeln!(out, "{indent}        self.{fname} = {empty_expr};").unwrap();
            writeln!(out, "{indent}    }}").unwrap();
            writeln!(out, "{indent}    pub fn {fname}(&self) -> Option<{rty}> {{").unwrap();
            writeln!(out, "{indent}        self.has_{clean_name}().then(|| {val_expr})").unwrap();
            writeln!(out, "{indent}    }}").unwrap();
        } else if sname == "Enum" && fname == "enumerators" {
            writeln!(out, "{indent}    pub fn enumerators(&self) -> Option<&[Enumerator]> {{")
                .unwrap();
            writeln!(
                out,
                "{indent}        (!self.is_incomplete).then_some(self.enumerators.as_slice())"
            )
            .unwrap();
            writeln!(out, "{indent}    }}").unwrap();
        } else {
            writeln!(out, "{indent}    pub fn {fname}(&self) -> {rty} {{").unwrap();
            writeln!(out, "{indent}        {val_expr}").unwrap();
            writeln!(out, "{indent}    }}").unwrap();
        }
    }

    let mut bit_idx = 0u32;
    for f in &st.fields {
        let fname = f.rs_name;
        let clean_name = f.cc_name;
        let sty = f.ty.rs_storage_type(in_submodule);
        let set_bit = if f.optional {
            let mask = 1u64 << bit_idx;
            bit_idx += 1;
            format!("self.has_bits |= 0x{mask:x}; ")
        } else {
            String::new()
        };
        match &f.ty {
            FieldType::Box(inner) => {
                let sub_ty = inner.rs_storage_type(in_submodule);
                writeln!(
                    out,
                    "{indent}    pub fn set_{clean_name}(&mut self, val: impl Into<{sub_ty}>) -> &mut {sub_ty} {{ {set_bit}self.{fname} = FfiBox::new(val.into()); self.{fname}.as_mut().unwrap() }}"
                )
                .unwrap();
            }
            FieldType::Enum(rs_path) => {
                let ety = format!("{prefix}{rs_path}");
                writeln!(
                    out,
                    "{indent}    pub fn set_{clean_name}(&mut self, val: impl Into<{ety}>) {{ {set_bit}self.{fname} = val.into(); }}"
                )
                .unwrap();
            }
            FieldType::Vec(_) | FieldType::Map(_, _) => {
                writeln!(
                    out,
                    "{indent}    pub fn set_{clean_name}(&mut self, val: impl Into<{sty}>) {{ self.{fname} = val.into(); }}"
                )
                .unwrap();
            }
            _ => {
                writeln!(
                    out,
                    "{indent}    pub fn set_{clean_name}(&mut self, val: impl Into<{sty}>) -> &mut {sty} {{ {set_bit}self.{fname} = val.into(); &mut self.{fname} }}"
                )
                .unwrap();
            }
        }
    }
    writeln!(out, "{indent}}}").unwrap();
    let (ssize, salign, foffsets) = schema.struct_layout_64(st);
    writeln!(out, "{indent}#[rustfmt::skip]").unwrap();
    writeln!(out, "{indent}#[cfg(target_pointer_width = \"64\")]").unwrap();
    writeln!(out, "{indent}const _: () = {{").unwrap();
    writeln!(
        out,
        "{indent}    assert!(core::mem::size_of::<{sname}>() == {ssize} && core::mem::align_of::<{sname}>() == {salign});"
    )
    .unwrap();
    for (_, rs_fname, foff) in foffsets {
        writeln!(out, "{indent}    assert!(core::mem::offset_of!({sname}, {rs_fname}) == {foff});")
            .unwrap();
    }
    writeln!(out, "{indent}}};").unwrap();
    writeln!(out).unwrap();
}

fn generate_rs(schema: &Schema) -> String {
    let mut out = String::new();
    writeln!(out, "// Automatically @generated by ir_codegen.rs. Do not edit directly.").unwrap();
    writeln!(out, "// To regenerate, run: common/golden_update.sh").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#![allow(").unwrap();
    writeln!(out, "    dead_code,").unwrap();
    writeln!(out, "    non_camel_case_types,").unwrap();
    writeln!(out, "    unused_imports,").unwrap();
    writeln!(out, "    clippy::large_enum_variant,").unwrap();
    writeln!(out, "    clippy::should_implement_trait,").unwrap();
    writeln!(out, "    clippy::clone_on_copy,").unwrap();
    writeln!(out, "    clippy::unnecessary_lazy_evaluations,").unwrap();
    writeln!(out, "    clippy::explicit_auto_deref").unwrap();
    writeln!(out, ")]").unwrap();
    writeln!(out).unwrap();

    let mut setter_names: BTreeMap<&'static str, &'static str> = BTreeMap::new();
    for rec in &schema.records {
        match rec {
            RecordDef::Union(u) => {
                for v in &u.variants {
                    if v.field_name == "type" {
                        setter_names.insert("r#type", "type");
                        setter_names.insert("type_", "type");
                    } else {
                        setter_names.insert(v.field_name, v.field_name);
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
    writeln!(out, "#[macro_export]").unwrap();
    writeln!(out, "macro_rules! __proto_call_setter {{").unwrap();
    for (rs_ident, clean_name) in setter_names {
        writeln!(
            out,
            "    ($msg:ident, {rs_ident}, $($arg:tt)*) => {{ $msg.set_{clean_name}($($arg)*) }};"
        )
        .unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    writeln!(
        out,
        r#"#[macro_export]
macro_rules! __proto_fields {{
    ($msg:ident $(,)?) => {{}};
    ($msg:ident, $field:ident : __ {{ $($inner:tt)* }} $(, $($rest:tt)*)?) => {{
        {{
            let __sub = $crate::__proto_call_setter!($msg, $field, $crate::ir_ffi_support::ProtoDefault);
            $crate::__proto_fields!(__sub, $($inner)*);
        }}
        $crate::__proto_fields!($msg, $($($rest)*)?);
    }};
    ($msg:ident, $field:ident : [ $( __ {{ $($inner:tt)* }} ),* $(,)? ] $(, $($rest:tt)*)?) => {{
        $(
            {{
                let __elem = $msg.$field.push(Default::default());
                $crate::__proto_fields!(__elem, $($inner)*);
            }}
        )*
        $crate::__proto_fields!($msg, $($($rest)*)?);
    }};
    ($msg:ident, $field:ident : [ $( ( $k:expr, [ $( __ {{ $($inner:tt)* }} ),* $(,)? ] ) ),* $(,)? ] $(, $($rest:tt)*)?) => {{
        $(
            {{
                let __vec = $msg.$field.push(($k).into(), Default::default());
                $(
                    {{
                        let __elem = __vec.push(Default::default());
                        $crate::__proto_fields!(__elem, $($inner)*);
                    }}
                )*
            }}
        )*
        $crate::__proto_fields!($msg, $($($rest)*)?);
    }};
    ($msg:ident, $field:ident : [ $( ( $k:expr, __ {{ $($inner:tt)* }} ) ),* $(,)? ] $(, $($rest:tt)*)?) => {{
        $(
            {{
                let __val = $msg.$field.push(($k).into(), Default::default());
                $crate::__proto_fields!(__val, $($inner)*);
            }}
        )*
        $crate::__proto_fields!($msg, $($($rest)*)?);
    }};
    ($msg:ident, $field:ident : $($subty:ident)::+ {{ $($inner:tt)* }} $(, $($rest:tt)*)?) => {{
        $crate::__proto_call_setter!($msg, $field, $crate::proto!($($subty)::+ {{ $($inner)* }}));
        $crate::__proto_fields!($msg, $($($rest)*)?);
    }};
    ($msg:ident, $field:ident : $val:expr $(, $($rest:tt)*)?) => {{
        $crate::__proto_call_setter!($msg, $field, $val);
        $crate::__proto_fields!($msg, $($($rest)*)?);
    }};
}}

#[macro_export]
macro_rules! proto {{
    ($($ty:ident)::+ {{ $($tt:tt)* }}) => {{{{
        let mut __msg = <$($ty)::+>::EMPTY;
        $crate::__proto_fields!(__msg, $($tt)*);
        __msg
    }}}};
}}
pub use crate::proto;

#[rustfmt::skip]
use crate::ir_ffi_support::{{BazelLabel, FfiBox, FfiDefault, FfiMap, FfiStr, FfiVec, ItemId, LifetimeId, ProtoDefault}};
"#
    )
    .unwrap();

    // Top-level enums
    for en in &schema.enums {
        if en.scope.is_none() {
            generate_rs_enum(&mut out, en, "");
        }
    }

    // Top-level unions and structs
    for rec in &schema.records {
        if rec.scope().is_none() {
            match rec {
                RecordDef::Union(u) => generate_rs_union(&mut out, schema, u, false, ""),
                RecordDef::Struct(st) => generate_rs_struct(&mut out, schema, st, false, ""),
            }
        }
    }

    // Group submodule items by top-level module name
    let mut mod_names: BTreeMap<&'static str, ()> = BTreeMap::new();
    for en in &schema.enums {
        if let Some((ns, _)) = en.scope {
            let top_mod = ns.split("::").next().unwrap();
            mod_names.insert(top_mod, ());
        }
    }
    for rec in &schema.records {
        if let Some((ns, _)) = rec.scope() {
            let top_mod = ns.split("::").next().unwrap();
            mod_names.insert(top_mod, ());
        }
    }

    for (mod_idx, top_mod) in mod_names.keys().enumerate() {
        if mod_idx > 0 {
            writeln!(out).unwrap();
        }
        writeln!(out, "#[rustfmt::skip]").unwrap();
        writeln!(out, "pub mod {top_mod} {{").unwrap();
        writeln!(out, "    use super::*;").unwrap();
        writeln!(out).unwrap();

        // Enums directly in top_mod
        for en in &schema.enums {
            if let Some((ns, _)) = en.scope {
                if ns == *top_mod {
                    generate_rs_enum(&mut out, en, "    ");
                }
            }
        }

        // Nested sub-submodules (e.g. `bridge_type::callable`)
        let mut sub_mods: BTreeMap<&'static str, &'static str> = BTreeMap::new();
        for en in &schema.enums {
            if let Some((ns, _)) = en.scope {
                if let Some(rest) = ns.strip_prefix(&format!("{top_mod}::")) {
                    sub_mods.insert(rest, ns);
                }
            }
        }
        for (sub_mod, full_ns) in sub_mods {
            writeln!(out, "    pub mod {sub_mod} {{").unwrap();
            for en in &schema.enums {
                if let Some((ns, _)) = en.scope {
                    if ns == full_ns {
                        generate_rs_enum(&mut out, en, "        ");
                    }
                }
            }
            writeln!(out, "    }}").unwrap();
            writeln!(out).unwrap();
        }

        // Unions and structs directly in top_mod
        for rec in &schema.records {
            if let Some((ns, _)) = rec.scope() {
                if ns == *top_mod {
                    match rec {
                        RecordDef::Union(u) => {
                            generate_rs_union(&mut out, schema, u, true, "    ");
                        }
                        RecordDef::Struct(st) => {
                            generate_rs_struct(&mut out, schema, st, true, "    ");
                        }
                    }
                }
            }
        }

        writeln!(out, "}}").unwrap();
    }

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Usage: ir_codegen <out_cc_header> <out_rs_file>");
    let s = schema();
    fs::write(&args[1], generate_cc(&s)).expect("Failed to write generated C++ header");
    fs::write(&args[2], generate_rs(&s)).expect("Failed to write generated Rust file");
}
