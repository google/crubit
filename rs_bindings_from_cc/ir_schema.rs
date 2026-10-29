// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::{
    EnumDef, EnumVariantDef, FieldType, RecordDef, Schema, ScopedName, StructDef, UnionDef,
    UnionVariantDef,
};

pub(crate) fn schema() -> Schema {
    ir_schema! {
        enum SafetyAnnotation {
            Unannotated = SAFETY_ANNOTATION_UNANNOTATED,
            DisableUnsafe = SAFETY_ANNOTATION_DISABLE_UNSAFE,
            Unsafe = SAFETY_ANNOTATION_UNSAFE,
        }
        enum PointerTypeKind {
            LValueRef,
            RValueRef,
            Nullable,
            NonNull,
            Owned,
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
            Bool,
            Void,
            Float,
            Double,
            Char,
            SignedChar,
            UnsignedChar,
            Short,
            Int,
            Long,
            LongLong,
            Int128,
            UnsignedShort,
            UnsignedInt,
            UnsignedLong,
            UnsignedLongLong,
            UnsignedInt128,
            Char16T,
            Char32T,
            PtrdiffT,
            IntptrT,
            SizeT,
            UintptrT,
            StdPtrdiffT,
            StdIntptrT,
            StdSizeT,
            StdUintptrT,
            Int8T,
            Int16T,
            Int32T,
            Int64T,
            StdInt8T,
            StdInt16T,
            StdInt32T,
            StdInt64T,
            Uint8T,
            Uint16T,
            Uint32T,
            Uint64T,
            StdUint8T,
            StdUint16T,
            StdUint32T,
            StdUint64T,
        }
        #[scope("InstanceMethodMetadata")]
        enum ReferenceQualification {
            Unqualified,
            LValue,
            RValue,
        }
        enum AccessSpecifier {
            Public,
            Protected,
            Private,
        }
        enum SpecialMemberFunc {
            Trivial,
            NontrivialMembers,
            NontrivialUserDefined,
            Unavailable,
        }
        enum RecordType {
            Struct,
            Union,
            Class,
        }
        #[scope("bridge_type::Callable")]
        enum BackingType {
            DynCallable,
            AnyInvocable,
        }
        #[scope("bridge_type::Callable")]
        enum FnTrait {
            Fn,
            FnMut,
            FnOnce,
        }
        enum TraitImplPolarity {
            None,
            Negative,
            Positive,
        }
        #[scope("UnsupportedItem")]
        enum Kind {
            Other,
            Func,
            GlobalVar,
            Struct,
            Union,
            Class,
            Enum,
            TypeAlias,
            Namespace,
            Constructor,
        }
        struct HeaderName {
            name: Str,
        }
        struct LifetimeName {
            name: Str,
            id: LifetimeId,
        }
        struct SourceLocation {
            path: Str,
            line: U32,
        }
        #[scope("DeclLocation")]
        struct ByteRange {
            begin: U32,
            end: U32,
        }
        struct DeclLocation {
            #[optional]
            spelling: SourceLocation,
            #[optional]
            expansion: SourceLocation,
            #[optional]
            name_range: decl_location::ByteRange,
        }
        struct FormattedError {
            fmt: Str,
            message: Str,
        }
        #[scope("CcType")]
        struct PointerType {
            kind: PointerTypeKind,
            #[optional]
            lifetime: LifetimeId,
            pointee_type: Box(CcType),
        }
        #[scope("CcType")]
        struct FuncPointer {
            non_null: Bool,
            call_conv: CallingConv,
            param_and_return_types: Vec(CcType),
            lifetime_inputs: Vec(Str),
        }
        #[scope("CcType")]
        struct ConstantArrayType {
            num_elements: Usize,
            element_type: Box(CcType),
        }
        #[scope("CcType")]
        enum Variant {
            Primitive(Primitive),
            Pointer(cc_type::PointerType),
            FuncPointer(cc_type::FuncPointer),
            Decl(ItemId),
            Error(FormattedError),
            ConstantArray(cc_type::ConstantArrayType),
        }
        struct CcType {
            variant: cc_type::Variant,
            is_const: Bool,
            is_nonnull: Bool,
            unknown_attr: Str,
            explicit_lifetimes: Vec(Str),
            template_args: Vec(CcType),
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
            Identifier(ident: Identifier),
            Operator(oper: Operator),
            Constructor,
            Destructor,
            ConversionOperator,
        }
        struct FuncParam {
            type: CcType,
            identifier: Identifier,
            clang_lifetime_capture_by: Vec(I32),
            clang_lifetimebound: Bool,
            #[optional]
            unknown_attr: Str,
        }
        struct InstanceMethodMetadata {
            reference: instance_method_metadata::ReferenceQualification,
            is_const: Bool,
            is_virtual: Bool,
        }
        #[scope("MemberFuncSemantic")]
        struct Setter {
            type: CcType,
            offset: Usize,
        }
        #[scope("MemberFuncSemantic")]
        struct Getter {
            type: CcType,
            offset: Usize,
        }
        enum MemberFuncSemantic {
            Setter(member_func_semantic::Setter),
            Getter(member_func_semantic::Getter),
        }
        struct Func {
            cc_name: UnqualifiedIdentifier,
            rs_name: UnqualifiedIdentifier,
            unique_name: Str,
            owning_target: BazelLabel,
            mangled_name: Str,
            #[optional]
            doc_comment: Str,
            return_type: CcType,
            params: Vec(FuncParam),
            lifetime_params: Vec(LifetimeName),
            is_inline: Bool,
            is_always_inline: Bool,
            #[optional]
            instance_method_metadata: InstanceMethodMetadata,
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
            call_conv: CallingConv,
            is_member_or_descendant_of_class_template: Bool,
            safety_annotation: SafetyAnnotation,
            #[optional]
            decl_loc: DeclLocation,
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
            semantic: MemberFuncSemantic,
            is_pub_crate: Bool,
        }
        enum StatusOrOptionalString {
            None,
            OkValue(Str),
            Err(Str),
        }
        struct Field {
            #[optional]
            rust_identifier: Identifier,
            #[optional]
            cpp_identifier: Identifier,
            #[optional]
            doc_comment: Str,
            type: CcType,
            access: AccessSpecifier,
            offset: Usize,
            size: Usize,
            unknown_attr: StatusOrOptionalString,
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
            Type(CcType),
            Bool(bool_value: Bool),
            Int(int_value: I64),
        }
        #[scope("BridgeType")]
        struct Bridge {
            rust_name: Str,
            abi_rust: Str,
            abi_cpp: Str,
            template_args: Vec(CcType),
            #[optional]
            label_hint: Str,
        }
        #[scope("BridgeType")]
        struct StdOptional {
            inner_type: CcType,
        }
        #[scope("BridgeType")]
        struct StdPair {
            first_type: CcType,
            second_type: CcType,
        }
        #[scope("BridgeType")]
        struct ProtoMessageBridge {
            rust_name: Str,
        }
        #[scope("BridgeType")]
        struct Callable {
            backing_type: bridge_type::callable::BackingType,
            fn_trait: bridge_type::callable::FnTrait,
            return_type: CcType,
            param_types: Vec(CcType),
        }
        enum BridgeType {
            Bridge(bridge_type::Bridge),
            StdOptional(bridge_type::StdOptional),
            StdPair(bridge_type::StdPair),
            StdString,
            ProtoMessageBridge(bridge_type::ProtoMessageBridge),
            Callable(bridge_type::Callable),
        }
        #[scope("ArrayValue")]
        struct IntegerElements {
            elements: Vec(IntegerConstant),
            #[optional]
            filler: IntegerConstant,
        }
        #[scope("ArrayValue")]
        struct ArrayElements {
            elements: Vec(ArrayValue),
            #[optional]
            filler: Box(ArrayValue),
        }
        #[scope("ArrayValue")]
        enum Kind {
            Integers(integer_elements: array_value::IntegerElements),
            Arrays(array_elements: array_value::ArrayElements),
            StringLiteral(Bytes),
        }
        struct ArrayValue {
            kind: array_value::Kind,
            length: U64,
        }
        #[scope("Constant")]
        enum Kind {
            Integer(value: IntegerConstant),
            Array(array_value: ArrayValue),
            StringView(string_view_value: Bytes),
        }
        struct Constant {
            kind: constant::Kind,
            cc_name: Identifier,
            rs_name: Identifier,
            unique_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            #[optional]
            decl_loc: DeclLocation,
            #[optional]
            unknown_attr: Str,
            #[optional]
            enclosing_item_id: ItemId,
            #[optional]
            type: CcType,
            must_bind: Bool,
            #[optional]
            deprecated: Str,
            #[optional]
            doc_comment: Str,
        }
        #[scope("TemplateSpecialization")]
        struct StdVector {
            element_type: CcType,
        }
        #[scope("TemplateSpecialization")]
        struct StdOptional {
            element_type: CcType,
        }
        #[scope("TemplateSpecialization")]
        struct StdSharedPtr {
            element_type: CcType,
        }
        #[scope("TemplateSpecialization")]
        struct StdUniquePtr {
            element_type: CcType,
        }
        #[scope("TemplateSpecialization")]
        struct AbslSpan {
            element_type: CcType,
        }
        #[scope("TemplateSpecialization")]
        struct C9Co {
            element_type: CcType,
        }
        #[scope("TemplateSpecialization")]
        struct StdAtomic {
            element_type: CcType,
        }
        #[scope("TemplateSpecialization")]
        struct AbslFlatHashMap {
            key_type: CcType,
            value_type: CcType,
        }
        #[scope("TemplateSpecialization")]
        struct AbslFlatHashSet {
            element_type: CcType,
        }
        #[scope("TemplateSpecialization")]
        enum Kind {
            StdStringView,
            StdWStringView,
            StdVector(template_specialization::StdVector),
            StdOptional(template_specialization::StdOptional),
            StdSharedPtr(template_specialization::StdSharedPtr),
            StdUniquePtr(template_specialization::StdUniquePtr),
            AbslSpan(template_specialization::AbslSpan),
            AbslFlatHashMap(template_specialization::AbslFlatHashMap),
            AbslFlatHashSet(template_specialization::AbslFlatHashSet),
            C9Co(template_specialization::C9Co),
            StdAtomic(template_specialization::StdAtomic),
            NonSpecial,
        }
        struct TemplateSpecialization {
            defining_target: BazelLabel,
            kind: template_specialization::Kind,
        }
        struct TraitDerives {
            clone: TraitImplPolarity,
            copy: TraitImplPolarity,
            debug: TraitImplPolarity,
            send: TraitImplPolarity,
            sync: TraitImplPolarity,
            custom: Vec(Str),
        }
        struct OwnedPtrConfig {
            owned_ptr_type: Str,
            drop_impl: Str,
        }
        struct Record {
            rs_name: Identifier,
            cc_name: Identifier,
            unique_name: Str,
            mangled_cc_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            #[optional]
            template_specialization: TemplateSpecialization,
            #[optional]
            unknown_attr: Str,
            #[optional]
            doc_comment: Str,
            #[optional]
            bridge_type: BridgeType,
            #[optional]
            owned_ptr_config: OwnedPtrConfig,
            #[optional]
            decl_loc: DeclLocation,
            unambiguous_public_bases: Vec(BaseClass),
            fields: Vec(Field),
            lifetime_params: Vec(LifetimeName),
            size_align: SizeAlign,
            trait_derives: TraitDerives,
            is_derived_class: Bool,
            has_virtual_bases: Bool,
            override_alignment: Bool,
            #[optional]
            pack: Usize,
            safety_annotation: SafetyAnnotation,
            copy_constructor: SpecialMemberFunc,
            move_constructor: SpecialMemberFunc,
            destructor: SpecialMemberFunc,
            is_rust_movable: Bool,
            is_inheritable: Bool,
            is_abstract: Bool,
            #[optional]
            nodiscard: Str,
            record_type: RecordType,
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
            children: Vec(Item),
            #[optional]
            mangled_rs_name: Str,
        }
        struct IncompleteRecord {
            cc_name: Identifier,
            rs_name: Identifier,
            unique_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            #[optional]
            unknown_attr: Str,
            record_type: RecordType,
            #[optional]
            enclosing_item_id: ItemId,
            must_bind: Bool,
        }
        struct Enumerator {
            identifier: Identifier,
            value: IntegerConstant,
            #[optional]
            unknown_attr: Str,
            #[optional]
            deprecated: Str,
            #[optional]
            doc_comment: Str,
        }
        struct Enum {
            cc_name: Identifier,
            rs_name: Identifier,
            unique_name: Str,
            mangled_cc_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            #[optional]
            decl_loc: DeclLocation,
            underlying_type: CcType,
            #[optional]
            enumerators: Vec(Enumerator),
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
        }
        struct GlobalVar {
            cc_name: Identifier,
            rs_name: Identifier,
            unique_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            #[optional]
            decl_loc: DeclLocation,
            #[optional]
            unknown_attr: Str,
            #[optional]
            enclosing_item_id: ItemId,
            #[optional]
            mangled_name: Str,
            type: CcType,
            must_bind: Bool,
            #[optional]
            deprecated: Str,
            #[optional]
            doc_comment: Str,
        }
        struct TypeAlias {
            cc_name: Identifier,
            rs_name: Identifier,
            unique_name: Str,
            id: ItemId,
            owning_target: BazelLabel,
            #[optional]
            doc_comment: Str,
            #[optional]
            unknown_attr: Str,
            underlying_type: CcType,
            #[optional]
            decl_loc: DeclLocation,
            #[optional]
            enclosing_item_id: ItemId,
            must_bind: Bool,
            #[optional]
            deprecated: Str,
            lifetime_inputs: Vec(Str),
            names_template_instantiation: Bool,
        }
        #[scope("UnsupportedItem")]
        struct Path {
            ident: UnqualifiedIdentifier,
            #[optional]
            enclosing_item_id: ItemId,
        }
        struct UnsupportedItem {
            name: Str,
            #[optional]
            unique_name: Str,
            kind: unsupported_item::Kind,
            #[optional]
            path: unsupported_item::Path,
            errors: Vec(FormattedError),
            #[optional]
            decl_loc: DeclLocation,
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
            cc_name: Identifier,
            rs_name: Identifier,
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
            children: Vec(Item),
        }
        struct UseMod {
            path: Str,
            mod_name: Identifier,
            id: ItemId,
            must_bind: Bool,
        }
        struct ExistingRustType {
            rs_name: Str,
            cc_name: Str,
            unique_name: Str,
            template_args: Vec(TemplateArg),
            owning_target: BazelLabel,
            #[optional]
            size_align: SizeAlign,
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
            Func(Box(Func)),
            Record(Box(Record)),
            IncompleteRecord(Box(IncompleteRecord)),
            Enum(enum_decl: Box(Enum)),
            Constant(Box(Constant)),
            TypeAlias(Box(TypeAlias)),
            GlobalVar(Box(GlobalVar)),
            UnsupportedItem(Box(UnsupportedItem)),
            Comment(Box(Comment)),
            Namespace(namespace_decl: Box(Namespace)),
            UseMod(Box(UseMod)),
            ExistingRustType(Box(ExistingRustType)),
        }
        struct IRProto {
            public_headers: Vec(HeaderName),
            current_target: Str,
            #[optional]
            crate_root_path: Str,
            crubit_features: Map(Str, Vec(Str)),
            unstable_rust_features: Vec(Str),
            reexported_namespaces: Vec(Str),
            top_level_items: Map(Str, Vec(Item)),
            crate_names: Map(Str, Str),
        }
        struct GenerateBindingsRequest {
            ir: IRProto,
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
