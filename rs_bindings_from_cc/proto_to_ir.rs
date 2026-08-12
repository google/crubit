// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use super::*;

fn opt_str_to_rc<'a>(
    opt_s: impl Into<Option<&'a ::protobuf::ProtoStr>>,
) -> Result<Option<Rc<str>>> {
    opt_s.into().map(|s| s.to_str().map(Rc::from)).transpose().map_err(Into::into)
}

fn opt_str_to_str<'a>(
    opt_s: impl Into<Option<&'a ::protobuf::ProtoStr>>,
) -> Result<Option<&'a str>> {
    opt_s.into().map(|s| s.to_str()).transpose().map_err(Into::into)
}

impl<'pb> TryFrom<::ir_rust_proto::HeaderNameView<'pb>> for HeaderName<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::HeaderNameView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::IdentifierView<'pb>> for Identifier<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::IdentifierView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::SizeAlignView<'pb>> for SizeAlign {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::SizeAlignView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::LifetimeNameView<'pb>> for LifetimeName<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::LifetimeNameView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::CallingConv> for CcCallingConv {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::CallingConv) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::PointerTypeKind> for PointerTypeKind {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::PointerTypeKind) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::cc_type::PointerTypeView<'_>> for PointerType {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::cc_type::PointerTypeView<'_>) -> Result<Self> {
        Ok(PointerType {
            kind: PointerTypeKind::try_from(proto.kind())?,
            lifetime: Into::<Option<i32>>::into(proto.lifetime_opt()).map(LifetimeId),
            pointee_type: Rc::new(CcType::try_from(proto.pointee_type())?),
        })
    }
}

impl TryFrom<::ir_rust_proto::CcTypeView<'_>> for CcType {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::CcTypeView<'_>) -> Result<Self> {
        let variant = match proto.variant() {
            ::ir_rust_proto::cc_type::VariantOneof::Primitive(p) => {
                let primitive = p.spelling().to_str()?.parse()?;
                CcTypeVariant::Primitive(primitive)
            }
            ::ir_rust_proto::cc_type::VariantOneof::Pointer(p) => {
                CcTypeVariant::Pointer(PointerType::try_from(p)?)
            }
            ::ir_rust_proto::cc_type::VariantOneof::FuncPointer(fp) => {
                let call_conv = CcCallingConv::try_from(fp.call_conv())?;
                CcTypeVariant::FuncPointer {
                    non_null: fp.non_null(),
                    call_conv,
                    param_and_return_types: fp
                        .param_and_return_types()
                        .iter()
                        .map(CcType::try_from)
                        .try_collect()?,
                    lifetime_inputs: fp
                        .lifetime_inputs()
                        .iter()
                        .map(|s| s.to_str().map(Rc::from))
                        .try_collect()?,
                }
            }
            ::ir_rust_proto::cc_type::VariantOneof::Decl(id) => {
                CcTypeVariant::Decl { id: ItemId(id as usize), template_args: None }
            }
            ::ir_rust_proto::cc_type::VariantOneof::Error(err) => {
                CcTypeVariant::Error(FormattedError::try_from(err)?)
            }
            _ => bail!("unmapped VariantOneof: {:?}", proto.variant()),
        };

        Ok(CcType {
            variant,
            is_const: proto.is_const(),
            unknown_attr: Rc::from(proto.unknown_attr().to_str()?),
            explicit_lifetimes: proto
                .explicit_lifetimes()
                .iter()
                .map(|s| s.to_str().map(Rc::from))
                .try_collect()?,
        })
    }
}

impl TryFrom<::ir_rust_proto::FormattedErrorView<'_>> for FormattedError {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::FormattedErrorView<'_>) -> Result<Self> {
        Ok(FormattedError {
            fmt: Rc::from(proto.fmt().to_str()?),
            message: Rc::from(proto.message().to_str()?),
        })
    }
}

impl<'pb> TryFrom<::ir_rust_proto::UnqualifiedIdentifierView<'pb>> for UnqualifiedIdentifier<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::UnqualifiedIdentifierView<'pb>) -> Result<Self> {
        match proto.identifier() {
            ::ir_rust_proto::unqualified_identifier::IdentifierOneof::Ident(id) => {
                Ok(UnqualifiedIdentifier::Identifier(Identifier::try_from(id)?))
            }
            ::ir_rust_proto::unqualified_identifier::IdentifierOneof::Oper(op) => {
                Ok(UnqualifiedIdentifier::Operator(Operator::try_from(op)?))
            }
            ::ir_rust_proto::unqualified_identifier::IdentifierOneof::SpecialName(sn) => match sn {
                ::ir_rust_proto::SpecialName::Constructor => Ok(UnqualifiedIdentifier::Constructor),
                ::ir_rust_proto::SpecialName::Destructor => Ok(UnqualifiedIdentifier::Destructor),
                _ => bail!("Unspecified SpecialName"),
            },
            ::ir_rust_proto::unqualified_identifier::IdentifierOneof::ConversionOperator(_) => {
                Ok(UnqualifiedIdentifier::ConversionOperator)
            }
            _ => bail!("unmapped IdentifierOneof: {:?}", proto.identifier()),
        }
    }
}

impl<'pb> TryFrom<::ir_rust_proto::OperatorView<'pb>> for Operator<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::OperatorView<'pb>) -> Result<Self> {
        Ok(Operator { name: proto.name().to_str()? })
    }
}

impl TryFrom<::ir_rust_proto::instance_method_metadata::ReferenceQualification>
    for ReferenceQualification
{
    type Error = Error;
    fn try_from(
        proto: ::ir_rust_proto::instance_method_metadata::ReferenceQualification,
    ) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::InstanceMethodMetadataView<'pb>> for InstanceMethodMetadata {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::InstanceMethodMetadataView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::FuncParamView<'pb>> for FuncParam<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::FuncParamView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::SafetyAnnotation> for SafetyAnnotation {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::SafetyAnnotation) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::MemberFuncSemanticView<'_>> for MemberFuncSemantic {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::MemberFuncSemanticView<'_>) -> Result<Self> {
        match proto.variant() {
            ::ir_rust_proto::member_func_semantic::VariantOneof::Setter(s) => {
                Ok(MemberFuncSemantic::Setter(Setter {
                    type_: CcType::try_from(s.r#type())?,
                    offset: s.offset() as usize,
                }))
            }
            ::ir_rust_proto::member_func_semantic::VariantOneof::Getter(g) => {
                Ok(MemberFuncSemantic::Getter(Getter {
                    type_: CcType::try_from(g.r#type())?,
                    offset: g.offset() as usize,
                }))
            }
            _ => bail!("unmapped VariantOneof: {:?}", proto.variant()),
        }
    }
}

impl<'pb> TryFrom<::ir_rust_proto::FuncView<'pb>> for Func<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::FuncView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::BaseClassView<'pb>> for BaseClass {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::BaseClassView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::AccessSpecifier> for AccessSpecifier {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::AccessSpecifier) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::FieldView<'pb>> for Field<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::FieldView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::TraitImplPolarity> for TraitImplPolarity {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::TraitImplPolarity) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::TraitDerivesView<'pb>> for TraitDerives<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::TraitDerivesView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::SpecialMemberFunc> for SpecialMemberFunc {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::SpecialMemberFunc) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::RecordType> for RecordType {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::RecordType) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::TemplateArgView<'_>> for TemplateArg {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::TemplateArgView<'_>) -> Result<Self> {
        match proto.variant() {
            ::ir_rust_proto::template_arg::VariantOneof::Type(t) => {
                Ok(TemplateArg::Type(CcType::try_from(t)?))
            }
            ::ir_rust_proto::template_arg::VariantOneof::BoolValue(b) => Ok(TemplateArg::Bool(b)),
            ::ir_rust_proto::template_arg::VariantOneof::IntValue(i) => Ok(TemplateArg::Int(i)),
            _ => bail!("unmapped VariantOneof: {:?}", proto.variant()),
        }
    }
}

impl<'pb> TryFrom<::ir_rust_proto::BridgeTypeView<'pb>> for BridgeType<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::BridgeTypeView<'pb>) -> Result<Self> {
        match proto.variant() {
            ::ir_rust_proto::bridge_type::VariantOneof::Bridge(b) => Ok(BridgeType::Bridge {
                rust_name: b.rust_name().to_str()?,
                abi_rust: b.abi_rust().to_str()?,
                abi_cpp: b.abi_cpp().to_str()?,
                template_args: b.template_args().iter().map(CcType::try_from).try_collect()?,
                label_hint: opt_str_to_str(b.label_hint_opt())?,
            }),
            ::ir_rust_proto::bridge_type::VariantOneof::StdOptional(so) => {
                Ok(BridgeType::StdOptional(CcType::try_from(so.inner_type())?))
            }
            ::ir_rust_proto::bridge_type::VariantOneof::StdPair(sp) => Ok(BridgeType::StdPair(
                CcType::try_from(sp.first_type())?,
                CcType::try_from(sp.second_type())?,
            )),
            ::ir_rust_proto::bridge_type::VariantOneof::StdString(_) => Ok(BridgeType::StdString),
            ::ir_rust_proto::bridge_type::VariantOneof::ProtoMessageBridge(pmb) => {
                Ok(BridgeType::ProtoMessageBridge { rust_name: pmb.rust_name().to_str()? })
            }
            ::ir_rust_proto::bridge_type::VariantOneof::Callable(c) => Ok(BridgeType::Callable {
                backing_type: c.backing_type(),
                fn_trait: c.fn_trait(),
                return_type: CcType::try_from(c.return_type())?,
                param_types: c.param_types().iter().map(CcType::try_from).try_collect()?,
            }),
            _ => bail!("unmapped VariantOneof: {:?}", proto.variant()),
        }
    }
}

impl<'pb> TryFrom<::ir_rust_proto::OwnedPtrConfigView<'pb>> for OwnedPtrConfig<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::OwnedPtrConfigView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::TemplateSpecializationView<'_>> for TemplateSpecialization {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::TemplateSpecializationView<'_>) -> Result<Self> {
        let kind = match proto.kind() {
            ::ir_rust_proto::template_specialization::KindOneof::StdStringView(_) => {
                TemplateSpecializationKind::StdStringView
            }
            ::ir_rust_proto::template_specialization::KindOneof::StdWStringView(_) => {
                TemplateSpecializationKind::StdWStringView
            }
            ::ir_rust_proto::template_specialization::KindOneof::StdVector(v) => {
                TemplateSpecializationKind::StdVector {
                    raw_element_type: CcType::try_from(v.element_type())?,
                }
            }
            ::ir_rust_proto::template_specialization::KindOneof::StdSharedPtr(up) => {
                TemplateSpecializationKind::StdSharedPtr {
                    raw_element_type: CcType::try_from(up.element_type())?,
                }
            }
            ::ir_rust_proto::template_specialization::KindOneof::StdUniquePtr(up) => {
                TemplateSpecializationKind::StdUniquePtr {
                    raw_element_type: CcType::try_from(up.element_type())?,
                }
            }
            ::ir_rust_proto::template_specialization::KindOneof::AbslSpan(as_) => {
                TemplateSpecializationKind::AbslSpan {
                    raw_element_type: CcType::try_from(as_.element_type())?,
                }
            }
            ::ir_rust_proto::template_specialization::KindOneof::AbslFlatHashMap(afhm) => {
                TemplateSpecializationKind::AbslFlatHashMap {
                    raw_key_type: CcType::try_from(afhm.key_type())?,
                    raw_value_type: CcType::try_from(afhm.value_type())?,
                }
            }
            ::ir_rust_proto::template_specialization::KindOneof::AbslFlatHashSet(afhs) => {
                TemplateSpecializationKind::AbslFlatHashSet {
                    raw_element_type: CcType::try_from(afhs.element_type())?,
                }
            }
            ::ir_rust_proto::template_specialization::KindOneof::C9Co(c) => {
                TemplateSpecializationKind::C9Co {
                    raw_element_type: CcType::try_from(c.element_type())?,
                }
            }
            ::ir_rust_proto::template_specialization::KindOneof::NonSpecial(_) => {
                TemplateSpecializationKind::NonSpecial
            }
            _ => bail!("unmapped KindOneof: {:?}", proto.kind()),
        };
        Ok(TemplateSpecialization {
            defining_target: BazelLabel::from(proto.defining_target().to_str()?),
            kind,
        })
    }
}

impl<'pb> TryFrom<::ir_rust_proto::RecordView<'pb>> for Record<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::RecordView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::IncompleteRecordView<'pb>> for IncompleteRecord<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::IncompleteRecordView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::EnumeratorView<'pb>> for Enumerator<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::EnumeratorView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::EnumView<'pb>> for Enum<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::EnumView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::ConstantView<'pb>> for Constant<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::ConstantView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::GlobalVarView<'pb>> for GlobalVar<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::GlobalVarView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::TypeAliasView<'pb>> for TypeAlias<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::TypeAliasView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl TryFrom<::ir_rust_proto::unsupported_item::Kind> for UnsupportedItemKind {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::unsupported_item::Kind) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::unsupported_item::PathView<'pb>> for UnsupportedItemPath<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::unsupported_item::PathView<'pb>) -> Result<Self> {
        Ok(UnsupportedItemPath {
            ident: UnqualifiedIdentifier::try_from(proto.ident())?,
            enclosing_item_id: Into::<Option<i64>>::into(proto.enclosing_item_id_opt())
                .map(|id| ItemId(id as usize)),
        })
    }
}

impl<'pb> TryFrom<::ir_rust_proto::UnsupportedItemView<'pb>> for UnsupportedItem<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::UnsupportedItemView<'pb>) -> Result<Self> {
        let path =
            proto.has_path().then(|| UnsupportedItemPath::try_from(proto.path())).transpose()?;

        Ok(UnsupportedItem {
            name: Rc::from(proto.name().to_str()?),
            unique_name: opt_str_to_str(proto.unique_name_opt())?,
            kind: UnsupportedItemKind::try_from(proto.kind())?,
            path,
            errors: proto
                .errors()
                .iter()
                .map(|e| FormattedError::try_from(e).map(Rc::new))
                .try_collect()?,
            source_loc: opt_str_to_str(proto.source_loc_opt())?,
            id: ItemId(proto.id() as usize),
            must_bind: proto.must_bind(),
            defining_target: opt_str_to_str(proto.defining_target_opt())?.map(BazelLabel::from),
            inline_cpp_source_text: opt_str_to_rc(proto.inline_cpp_source_text_opt())?,
        })
    }
}

impl<'pb> TryFrom<::ir_rust_proto::CommentView<'pb>> for Comment<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::CommentView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::NamespaceView<'pb>> for Namespace<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::NamespaceView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::UseModView<'pb>> for UseMod<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::UseModView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::ExistingRustTypeView<'pb>> for ExistingRustType<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::ExistingRustTypeView<'pb>) -> Result<Self> {
        proto.validate()?;
        Ok(proto.to_ir())
    }
}

impl<'pb> TryFrom<::ir_rust_proto::ItemView<'pb>> for Item<'pb> {
    type Error = Error;
    fn try_from(proto: ::ir_rust_proto::ItemView<'pb>) -> Result<Self> {
        match proto.item() {
            ::ir_rust_proto::item::ItemOneof::Func(f) => {
                Ok(Item::Func(Rc::new(Func::try_from(f)?)))
            }
            ::ir_rust_proto::item::ItemOneof::Record(r) => {
                Ok(Item::Record(Rc::new(Record::try_from(r)?)))
            }
            ::ir_rust_proto::item::ItemOneof::IncompleteRecord(ir) => {
                Ok(Item::IncompleteRecord(Rc::new(IncompleteRecord::try_from(ir)?)))
            }
            ::ir_rust_proto::item::ItemOneof::EnumDecl(e) => {
                Ok(Item::Enum(Rc::new(Enum::try_from(e)?)))
            }
            ::ir_rust_proto::item::ItemOneof::Constant(c) => {
                Ok(Item::Constant(Rc::new(Constant::try_from(c)?)))
            }
            ::ir_rust_proto::item::ItemOneof::TypeAlias(ta) => {
                Ok(Item::TypeAlias(Rc::new(TypeAlias::try_from(ta)?)))
            }
            ::ir_rust_proto::item::ItemOneof::GlobalVar(gv) => {
                Ok(Item::GlobalVar(Rc::new(GlobalVar::try_from(gv)?)))
            }
            ::ir_rust_proto::item::ItemOneof::UnsupportedItem(ui) => {
                Ok(Item::UnsupportedItem(Rc::new(UnsupportedItem::try_from(ui)?)))
            }
            ::ir_rust_proto::item::ItemOneof::Comment(c) => {
                Ok(Item::Comment(Rc::new(Comment::try_from(c)?)))
            }
            ::ir_rust_proto::item::ItemOneof::NamespaceDecl(nd) => {
                Ok(Item::Namespace(Rc::new(Namespace::try_from(nd)?)))
            }
            ::ir_rust_proto::item::ItemOneof::UseMod(um) => {
                Ok(Item::UseMod(Rc::new(UseMod::try_from(um)?)))
            }
            ::ir_rust_proto::item::ItemOneof::ExistingRustType(ert) => {
                Ok(Item::ExistingRustType(Rc::new(ExistingRustType::try_from(ert)?)))
            }
            _ => bail!("unmapped ItemOneof: {:?}", proto.item()),
        }
    }
}

pub fn proto_to_ir<'pb>(proto: ::ir_rust_proto::IRProtoView<'pb>) -> Result<IR<'pb>> {
    let public_headers = proto.public_headers().iter().map(HeaderName::try_from).try_collect()?;

    let crate_root_path = opt_str_to_rc(proto.crate_root_path_opt())?;

    let crubit_features = proto
        .crubit_features()
        .iter()
        .map(|(target, feature_set)| {
            let mut features = flagset::FlagSet::<CrubitFeature>::default();
            for feature_str in feature_set.features().iter() {
                let feature_name = feature_str.to_str()?;
                let feature_flags = crubit_feature::named_features(feature_name.as_bytes())
                    .with_context(|| format!("Invalid Crubit feature name: {:?}", feature_name))?;
                features |= feature_flags;
            }
            Ok::<_, Error>((
                BazelLabel::from(target.to_str()?),
                crubit_feature::SerializedCrubitFeatures::resolved(features),
            ))
        })
        .try_collect()?;

    let unstable_rust_features = proto
        .unstable_rust_features()
        .iter()
        .map(|s| s.to_str().map(String::from))
        .try_collect()?;

    let reexported_namespaces = proto
        .reexported_namespaces()
        .iter()
        .map(|s| s.to_str().map(std::rc::Rc::from))
        .try_collect()?;

    let crate_names = proto
        .crate_names()
        .iter()
        .map(|(target, name)| {
            let name_str = name.to_str()?;
            let ident = try_make_rs_ident(name_str)
                .with_context(|| format!("Invalid crate name identifier: {:?}", name_str))?;
            Ok::<_, Error>((BazelLabel::from(target.to_str()?), ident))
        })
        .try_collect()?;

    let top_level_items = proto
        .top_level_items()
        .iter()
        .map(|(target, item_list)| {
            let items = item_list.items().iter().map(Item::try_from).try_collect()?;
            Ok::<_, Error>((BazelLabel::from(target.to_str()?), items))
        })
        .try_collect()?;

    let tree_ir = TreeIR {
        public_headers,
        current_target: BazelLabel::from(proto.current_target().to_str()?),
        crate_root_path,
        crubit_features,
        crate_names,
        unstable_rust_features,
        reexported_namespaces,
        top_level_items,
    };
    Ok(super::make_ir(tree_ir))
}
