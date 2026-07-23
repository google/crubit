// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

#include <cstdint>
#include <memory>
#include <optional>
#include <ostream>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/algorithm/container.h"
#include "absl/base/nullability.h"
#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/strings/cord.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/string_type.h"
#include "common/strong_int.h"
#include "llvm/Support/JSON.h"

namespace crubit {

namespace flat_proto = rs_bindings_from_cc::ir_proto::flat;

namespace {
// https://en.cppreference.com/w/cpp/utility/variant/visit
template <typename... Ts>
struct visitor : Ts... {
  using Ts::operator()...;
};
}  // namespace

flat_proto::HeaderName HeaderName::ToFlatProto() const {
  flat_proto::HeaderName proto;
  proto.set_name(name_);
  return proto;
}

flat_proto::LifetimeName LifetimeName::ToFlatProto() const {
  flat_proto::LifetimeName proto;
  proto.set_name(name);
  proto.set_id(id.value());
  return proto;
}

flat_proto::PointerTypeKind ToFlatProto(PointerTypeKind pointer_type_kind) {
  switch (pointer_type_kind) {
    case PointerTypeKind::kLValueRef:
      return flat_proto::L_VALUE_REF;
    case PointerTypeKind::kRValueRef:
      return flat_proto::R_VALUE_REF;
    case PointerTypeKind::kNullable:
      return flat_proto::NULLABLE;
    case PointerTypeKind::kNonNull:
      return flat_proto::NON_NULL;
    case PointerTypeKind::kOwned:
      return flat_proto::OWNED;
  }
}

flat_proto::CallingConv ToFlatProto(CallingConv calling_conv) {
  switch (calling_conv) {
    case CallingConv::kC:
      return flat_proto::C_DECL;
    case CallingConv::kX86FastCall:
      return flat_proto::FAST_CALL;
    case CallingConv::kX86VectorCall:
      return flat_proto::VECTOR_CALL;
    case CallingConv::kX864ThisCall:
      return flat_proto::THIS_CALL;
    case CallingConv::kX86StdCall:
      return flat_proto::STD_CALL;
    case CallingConv::kWin64:
      return flat_proto::MS_ABI;
  }
}

flat_proto::CcType CcType::ToFlatProto() const {
  flat_proto::CcType proto;
  proto.set_is_const(is_const);
  proto.set_unknown_attr(unknown_attr);
  proto.mutable_explicit_lifetimes()->Add(explicit_lifetimes.begin(),
                                          explicit_lifetimes.end());

  std::visit(
      visitor{[&](const CcType::Primitive& primitive) {
                proto.mutable_primitive()->set_spelling(primitive.spelling);
              },
              [&](const CcType::PointerType& pointer) {
                auto* p = proto.mutable_pointer();
                p->set_kind(crubit::ToFlatProto(pointer.kind));
                if (pointer.lifetime) {
                  p->set_lifetime((*pointer.lifetime).value());
                }
                *p->mutable_pointee_type() =
                    pointer.pointee_type->ToFlatProto();
              },
              [&](const CcType::FuncPointer& func_value) {
                auto* f = proto.mutable_func_pointer();
                f->set_non_null(func_value.non_null);
                f->set_call_conv(crubit::ToFlatProto(func_value.call_conv));
                for (const CcType& type : func_value.param_and_return_types) {
                  *f->add_param_and_return_types() = type.ToFlatProto();
                }
                f->mutable_lifetime_inputs()->Add(
                    func_value.lifetime_inputs.begin(),
                    func_value.lifetime_inputs.end());
              },
              [&](ItemId id) { proto.set_decl(id.value()); },
              [&](const FormattedError& error) {
                *proto.mutable_error() = error.ToFlatProto();
              }},
      variant);
  return proto;
}

namespace {
CcType PointerOrReferenceTo(CcType pointee_type, PointerTypeKind pointer_kind,
                            std::optional<LifetimeId> lifetime) {
  return CcType(CcType::PointerType{
      .kind = pointer_kind,
      .lifetime = lifetime,
      .pointee_type = std::make_shared<CcType>(std::move(pointee_type)),
  });
}
}  // namespace

CcType CcType::PointerTo(CcType pointee_type,
                         std::optional<LifetimeId> lifetime, bool nullable) {
  return PointerOrReferenceTo(
      std::move(pointee_type),
      nullable ? PointerTypeKind::kNullable : PointerTypeKind::kNonNull,
      lifetime);
}

CcType CcType::OwnedPointerTo(CcType pointee_type,
                              std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type), PointerTypeKind::kOwned,
                              lifetime);
}

CcType CcType::LValueReferenceTo(CcType pointee_type,
                                 std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type),
                              PointerTypeKind::kLValueRef, lifetime);
}

CcType CcType::RValueReferenceTo(CcType pointee_type,
                                 std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type),
                              PointerTypeKind::kRValueRef, lifetime);
}

flat_proto::Identifier Identifier::ToFlatProto() const {
  flat_proto::Identifier proto;
  proto.set_identifier(identifier_);
  return proto;
}

flat_proto::IntegerConstant IntegerConstant::ToFlatProto() const {
  flat_proto::IntegerConstant proto;
  proto.set_is_negative(is_negative_);
  proto.set_wrapped_value(wrapped_value_);
  return proto;
}

flat_proto::Operator Operator::ToFlatProto() const {
  flat_proto::Operator proto;
  proto.set_name(name_);
  return proto;
}

static std::string SpecialNameToString(SpecialName special_name) {
  switch (special_name) {
    case SpecialName::kDestructor:
      return "Destructor";
    case SpecialName::kConstructor:
      return "Constructor";
  }
}

flat_proto::SpecialName ToFlatProto(SpecialName special_name) {
  switch (special_name) {
    case SpecialName::kDestructor:
      return flat_proto::DESTRUCTOR;
    case SpecialName::kConstructor:
      return flat_proto::CONSTRUCTOR;
  }
}

flat_proto::UnqualifiedIdentifier ToFlatProto(
    const UnqualifiedIdentifier& unqualified_identifier) {
  flat_proto::UnqualifiedIdentifier proto;
  std::visit(
      visitor{
          [&](const Identifier& id) {
            *proto.mutable_ident() = id.ToFlatProto();
          },
          [&](const Operator& op) { *proto.mutable_oper() = op.ToFlatProto(); },
          [&](const SpecialName& special_name) {
            proto.set_special_name(crubit::ToFlatProto(special_name));
          },
          [&](const ConversionOperator& conversion_operator) {
            proto.mutable_conversion_operator();
          }},
      unqualified_identifier);
  return proto;
}

flat_proto::FuncParam FuncParam::ToFlatProto() const {
  flat_proto::FuncParam proto;
  *proto.mutable_type() = type.ToFlatProto();
  *proto.mutable_identifier() = identifier.ToFlatProto();
  if (unknown_attr) {
    proto.set_unknown_attr(*unknown_attr);
  }
  proto.mutable_clang_lifetime_capture_by()->Add(
      clang_lifetime_capture_by.begin(), clang_lifetime_capture_by.end());
  proto.set_clang_lifetimebound(clang_lifetimebound);
  return proto;
}

std::ostream& operator<<(std::ostream& o, const SpecialName& special_name) {
  return o << SpecialNameToString(special_name);
}

UnqualifiedIdentifier& TranslatedUnqualifiedIdentifier::rs_identifier() {
  if (crubit_rust_name) {
    return *crubit_rust_name;
  }
  return cc_identifier;
}

Identifier& TranslatedIdentifier::rs_identifier() {
  if (crubit_rust_name) {
    return *crubit_rust_name;
  }
  return cc_identifier;
}

flat_proto::InstanceMethodMetadata InstanceMethodMetadata::ToFlatProto() const {
  flat_proto::InstanceMethodMetadata proto;
  switch (reference) {
    case InstanceMethodMetadata::kLValue:
      proto.set_reference(flat_proto::InstanceMethodMetadata::L_VALUE);
      break;
    case InstanceMethodMetadata::kRValue:
      proto.set_reference(flat_proto::InstanceMethodMetadata::R_VALUE);
      break;
    case InstanceMethodMetadata::kUnqualified:
      proto.set_reference(flat_proto::InstanceMethodMetadata::UNQUALIFIED);
      break;
  }
  proto.set_is_const(is_const);
  proto.set_is_virtual(is_virtual);
  return proto;
}

flat_proto::Constant Constant::ToFlatProto() const {
  flat_proto::Constant proto;
  *proto.mutable_value() = value.ToFlatProto();
  *proto.mutable_cc_name() = cc_name.ToFlatProto();
  *proto.mutable_rs_name() = rs_name.ToFlatProto();
  proto.set_unique_name(unique_name);
  proto.set_id(id.value());
  proto.set_owning_target(owning_target.value());
  proto.set_source_loc(source_loc);
  *proto.mutable_type() = type.ToFlatProto();
  if (unknown_attr) proto.set_unknown_attr(*unknown_attr);
  if (enclosing_item_id)
    proto.set_enclosing_item_id(enclosing_item_id->value());
  proto.set_must_bind(must_bind);
  if (deprecated) proto.set_deprecated(*deprecated);
  if (doc_comment) proto.set_doc_comment(*doc_comment);
  return proto;
}

flat_proto::ExistingRustType ExistingRustType::ToFlatProto() const {
  flat_proto::ExistingRustType proto;
  proto.set_rs_name(rs_name);
  proto.set_cc_name(cc_name);
  proto.set_unique_name(unique_name);
  for (const auto& arg : template_args)
    *proto.add_template_args() = arg.ToFlatProto();
  proto.set_owning_target(owning_target.value());
  if (size_align) *proto.mutable_size_align() = size_align->ToFlatProto();
  proto.set_is_same_abi(is_same_abi);
  proto.set_id(id.value());
  proto.set_must_bind(must_bind);
  proto.set_impl_debug(impl_debug);
  return proto;
}

flat_proto::UseMod UseMod::ToFlatProto() const {
  flat_proto::UseMod proto;
  proto.set_path(path);
  *proto.mutable_mod_name() = mod_name.ToFlatProto();
  proto.set_id(id.value());
  proto.set_must_bind(must_bind);
  return proto;
}

static std::string SafetyAnnotationToString(
    SafetyAnnotation safety_annotation) {
  switch (safety_annotation) {
    case SafetyAnnotation::kDisableUnsafe:
      return "DisableUnsafe";
    case SafetyAnnotation::kUnsafe:
      return "Unsafe";
    case SafetyAnnotation::kUnannotated:
      return "Unannotated";
  }
}

flat_proto::SafetyAnnotation ToFlatProto(SafetyAnnotation safety_annotation) {
  switch (safety_annotation) {
    case SafetyAnnotation::kDisableUnsafe:
      return flat_proto::SafetyAnnotation::SAFETY_ANNOTATION_DISABLE_UNSAFE;
    case SafetyAnnotation::kUnsafe:
      return flat_proto::SafetyAnnotation::SAFETY_ANNOTATION_UNSAFE;
    case SafetyAnnotation::kUnannotated:
      return flat_proto::SafetyAnnotation::SAFETY_ANNOTATION_UNANNOTATED;
  }
}

flat_proto::Func Func::ToFlatProto() const {
  flat_proto::Func proto;
  *proto.mutable_cc_name() = crubit::ToFlatProto(cc_name);
  *proto.mutable_rs_name() = crubit::ToFlatProto(rs_name);
  proto.set_unique_name(unique_name);
  proto.set_owning_target(owning_target.value());
  if (doc_comment) proto.set_doc_comment(*doc_comment);
  proto.set_mangled_name(mangled_name);
  *proto.mutable_return_type() = return_type.ToFlatProto();
  for (const auto& p : params) *proto.add_params() = p.ToFlatProto();
  for (const auto& l : lifetime_params)
    *proto.add_lifetime_params() = l.ToFlatProto();
  proto.set_is_inline(is_inline);
  if (instance_method_metadata) {
    *proto.mutable_instance_method_metadata() =
        instance_method_metadata->ToFlatProto();
  }
  proto.set_is_extern_c(is_extern_c);
  proto.set_is_noreturn(is_noreturn);
  proto.set_is_variadic(is_variadic);
  proto.set_is_consteval(is_consteval);
  if (nodiscard) proto.set_nodiscard(*nodiscard);
  if (deprecated) proto.set_deprecated(*deprecated);
  if (unknown_attr) proto.set_unknown_attr(*unknown_attr);
  proto.set_has_c_calling_convention(has_c_calling_convention);
  proto.set_is_member_or_descendant_of_class_template(
      is_member_or_descendant_of_class_template);
  proto.set_safety_annotation(crubit::ToFlatProto(safety_annotation));
  proto.set_source_loc(source_loc);
  proto.set_id(id.value());
  if (enclosing_item_id)
    proto.set_enclosing_item_id(enclosing_item_id->value());
  if (adl_enclosing_record)
    proto.set_adl_enclosing_record(adl_enclosing_record->value());
  proto.set_must_bind(must_bind);
  proto.mutable_lifetime_inputs()->Add(lifetime_inputs.begin(),
                                       lifetime_inputs.end());
  if (inline_cpp_source_text) {
    proto.set_inline_cpp_source_text(*inline_cpp_source_text);
  }
  proto.set_is_compiler_generated(is_compiler_generated);
  return proto;
}

static std::string AccessToString(AccessSpecifier access) {
  switch (access) {
    case kPublic:
      return "Public";
    case kProtected:
      return "Protected";
    case kPrivate:
      return "Private";
  }
}

flat_proto::AccessSpecifier ToFlatProto(AccessSpecifier access) {
  switch (access) {
    case kPublic:
      return flat_proto::PUBLIC;
    case kProtected:
      return flat_proto::PROTECTED;
    case kPrivate:
      return flat_proto::PRIVATE;
  }
}

std::ostream& operator<<(std::ostream& o, const AccessSpecifier& access) {
  return o << AccessToString(access);
}

flat_proto::Field Field::ToFlatProto() const {
  flat_proto::Field proto;
  if (rust_identifier) {
    *proto.mutable_rust_identifier() = rust_identifier->ToFlatProto();
  }
  if (cpp_identifier) {
    *proto.mutable_cpp_identifier() = cpp_identifier->ToFlatProto();
  }
  if (doc_comment) {
    proto.set_doc_comment(*doc_comment);
  }
  *proto.mutable_type() = type.ToFlatProto();
  proto.set_access(crubit::ToFlatProto(access));
  proto.set_offset(offset);
  proto.set_size(size);
  if (unknown_attr.ok()) {
    if (unknown_attr->has_value()) {
      proto.mutable_unknown_attr()->set_ok_value(unknown_attr->value());
    }
  } else {
    proto.mutable_unknown_attr()->set_err(unknown_attr.status().message());
  }
  proto.set_is_no_unique_address(is_no_unique_address);
  proto.set_is_bitfield(is_bitfield);
  proto.set_is_inheritable(is_inheritable);
  proto.set_is_mutable(is_mutable);
  if (deprecated) {
    proto.set_deprecated(*deprecated);
  }
  return proto;
}

flat_proto::SpecialMemberFunc ToFlatProto(SpecialMemberFunc f) {
  switch (f) {
    case SpecialMemberFunc::kTrivial:
      return flat_proto::TRIVIAL;
    case SpecialMemberFunc::kNontrivialMembers:
      return flat_proto::NONTRIVIAL_MEMBERS;
    case SpecialMemberFunc::kNontrivialUserDefined:
      return flat_proto::NONTRIVIAL_USER_DEFINED;
    case SpecialMemberFunc::kUnavailable:
      return flat_proto::UNAVAILABLE;
  }
}

flat_proto::BaseClass BaseClass::ToFlatProto() const {
  flat_proto::BaseClass proto;
  proto.set_base_record_id(base_record_id.value());
  if (offset) {
    proto.set_offset(*offset);
  }
  return proto;
}

static std::string RecordTypeToString(RecordType record_type) {
  switch (record_type) {
    case kStruct:
      return "Struct";
    case kUnion:
      return "Union";
    case kClass:
      return "Class";
  }
}

flat_proto::RecordType ToFlatProto(RecordType record_type) {
  switch (record_type) {
    case kStruct:
      return flat_proto::STRUCT;
    case kUnion:
      return flat_proto::UNION;
    case kClass:
      return flat_proto::CLASS;
  }
}

std::ostream& operator<<(std::ostream& o, const RecordType& record_type) {
  return o << RecordTypeToString(record_type);
}

flat_proto::IncompleteRecord IncompleteRecord::ToFlatProto() const {
  flat_proto::IncompleteRecord proto;
  *proto.mutable_cc_name() = cc_name.ToFlatProto();
  *proto.mutable_rs_name() = rs_name.ToFlatProto();
  proto.set_unique_name(unique_name);
  proto.set_id(id.value());
  proto.set_owning_target(owning_target.value());
  if (unknown_attr) proto.set_unknown_attr(*unknown_attr);
  proto.set_record_type(crubit::ToFlatProto(record_type));
  if (enclosing_item_id)
    proto.set_enclosing_item_id(enclosing_item_id->value());
  proto.set_must_bind(must_bind);
  return proto;
}

flat_proto::SizeAlign SizeAlign::ToFlatProto() const {
  flat_proto::SizeAlign proto;
  proto.set_size(size);
  proto.set_alignment(alignment);
  return proto;
}

flat_proto::BridgeType BridgeType::ToFlatProto() const {
  flat_proto::BridgeType proto;
  std::visit(
      visitor{
          [&](const BridgeType::Bridge& annotation) {
            auto* b = proto.mutable_bridge();
            b->set_rust_name(annotation.rust_name);
            b->set_abi_rust(annotation.abi_rust);
            b->set_abi_cpp(annotation.abi_cpp);
            for (const auto& arg : annotation.template_args) {
              *b->add_template_args() = arg.ToFlatProto();
            }
          },
          [&](const BridgeType::StdOptional& std_optional) {
            *proto.mutable_std_optional()->mutable_inner_type() =
                std_optional.inner_type->ToFlatProto();
          },
          [&](const BridgeType::StdPair& std_pair) {
            auto* p = proto.mutable_std_pair();
            *p->mutable_first_type() = std_pair.first_type->ToFlatProto();
            *p->mutable_second_type() = std_pair.second_type->ToFlatProto();
          },
          [&](const BridgeType::StdString& std_string) {
            // Calling mutable_std_string instantiates the message field to
            // signify its presence
            proto.mutable_std_string();
          },
          [&](const BridgeType::ProtoMessageBridge& proto_message_bridge) {
            proto.mutable_proto_message_bridge()->set_rust_name(
                proto_message_bridge.rust_name);
          },
          [&](const BridgeType::Callable& callable) {
            auto* c = proto.mutable_callable();
            switch (callable.backing_type) {
              case BridgeType::Callable::BackingType::kDynCallable:
                c->set_backing_type(
                    flat_proto::BridgeType::Callable::DYN_CALLABLE);
                break;
              case BridgeType::Callable::BackingType::kAnyInvocable:
                c->set_backing_type(
                    flat_proto::BridgeType::Callable::ANY_INVOCABLE);
                break;
            }
            switch (callable.fn_trait) {
              case BridgeType::Callable::FnTrait::kFn:
                c->set_fn_trait(flat_proto::BridgeType::Callable::FN);
                break;
              case BridgeType::Callable::FnTrait::kFnMut:
                c->set_fn_trait(flat_proto::BridgeType::Callable::FN_MUT);
                break;
              case BridgeType::Callable::FnTrait::kFnOnce:
                c->set_fn_trait(flat_proto::BridgeType::Callable::FN_ONCE);
                break;
            }
            *c->mutable_return_type() = callable.return_type->ToFlatProto();
            for (const auto& param : callable.param_types) {
              *c->add_param_types() = param.ToFlatProto();
            }
          },
      },
      variant);
  return proto;
}

flat_proto::TemplateArg TemplateArg::ToFlatProto() const {
  flat_proto::TemplateArg proto;
  std::visit(
      visitor{[&](const CcType& type) {
                *proto.mutable_type() = type.ToFlatProto();
              },
              [&](bool bool_value) { proto.set_bool_value(bool_value); },
              [&](int64_t int_value) { proto.set_int_value(int_value); }},
      variant);
  return proto;
}

flat_proto::TemplateSpecialization TemplateSpecialization::ToFlatProto() const {
  flat_proto::TemplateSpecialization proto;
  proto.set_defining_target(defining_target.value());
  std::visit(
      visitor{
          [&](const StdStringView&) { proto.mutable_std_string_view(); },
          [&](const StdWStringView&) { proto.mutable_std_w_string_view(); },
          [&](const StdVector& std_vector) {
            *proto.mutable_std_vector()->mutable_element_type() =
                std_vector.element_type.ToFlatProto();
          },
          [&](const StdUniquePtr& std_unique_ptr) {
            *proto.mutable_std_unique_ptr()->mutable_element_type() =
                std_unique_ptr.element_type.ToFlatProto();
          },
          [&](const AbslSpan& absl_span) {
            *proto.mutable_absl_span()->mutable_element_type() =
                absl_span.element_type.ToFlatProto();
          },
          [&](const AbslFlatHashMap& absl_flat_hash_map) {
            auto* msg = proto.mutable_absl_flat_hash_map();
            *msg->mutable_key_type() =
                absl_flat_hash_map.key_type.ToFlatProto();
            *msg->mutable_value_type() =
                absl_flat_hash_map.value_type.ToFlatProto();
          },
          [&](const AbslFlatHashSet& absl_flat_hash_set) {
            auto* msg = proto.mutable_absl_flat_hash_set();
            *msg->mutable_element_type() =
                absl_flat_hash_set.element_type.ToFlatProto();
          },
          [&](const C9Co& c9_co) {
            *proto.mutable_c9_co()->mutable_element_type() =
                c9_co.element_type.ToFlatProto();
          },
          [&](const NonSpecial&) { proto.mutable_non_special(); },
      },
      kind);
  return proto;
}

TraitImplPolarity* absl_nullable TraitDerives::Polarity(
    absl::string_view trait) {
  // <internal link> start
  if (trait == "Clone") return &clone;
  if (trait == "Copy") return &copy;
  if (trait == "Debug") return &debug;
  // <internal link> end
  return nullptr;
}

static std::string TraitImplPolarityToString(TraitImplPolarity polarity) {
  switch (polarity) {
    case TraitImplPolarity::kNegative:
      return "Negative";
    case TraitImplPolarity::kNone:
      return "None";
    case TraitImplPolarity::kPositive:
      return "Positive";
  }
}

flat_proto::TraitImplPolarity ToFlatProto(TraitImplPolarity polarity) {
  switch (polarity) {
    case TraitImplPolarity::kNegative:
      return flat_proto::NEGATIVE;
    case TraitImplPolarity::kNone:
      return flat_proto::NONE;
    case TraitImplPolarity::kPositive:
      return flat_proto::POSITIVE;
  }
}

flat_proto::TraitDerives TraitDerives::ToFlatProto() const {
  flat_proto::TraitDerives proto;
  proto.set_clone(crubit::ToFlatProto(clone));
  proto.set_copy(crubit::ToFlatProto(copy));
  proto.set_debug(crubit::ToFlatProto(debug));
  proto.set_send(send);
  proto.set_sync(sync);
  proto.mutable_custom()->Add(custom.begin(), custom.end());
  return proto;
}

flat_proto::OwnedPtrConfig OwnedPtrConfig::ToFlatProto() const {
  flat_proto::OwnedPtrConfig proto;
  proto.set_owned_ptr_type(owned_ptr_type);
  proto.set_drop_impl(drop_impl);
  return proto;
}

flat_proto::Record Record::ToFlatProto() const {
  flat_proto::Record proto;
  *proto.mutable_rs_name() = rs_name.ToFlatProto();
  *proto.mutable_cc_name() = cc_name.ToFlatProto();
  proto.set_unique_name(unique_name);
  proto.set_mangled_cc_name(mangled_cc_name);
  proto.set_id(id.value());
  proto.set_owning_target(owning_target.value());
  if (template_specialization)
    *proto.mutable_template_specialization() =
        template_specialization->ToFlatProto();
  if (unknown_attr) proto.set_unknown_attr(*unknown_attr);
  if (doc_comment) proto.set_doc_comment(*doc_comment);
  if (bridge_type) *proto.mutable_bridge_type() = bridge_type->ToFlatProto();
  if (owned_ptr_config)
    *proto.mutable_owned_ptr_config() = owned_ptr_config->ToFlatProto();
  proto.set_source_loc(source_loc);
  for (const auto& b : unambiguous_public_bases)
    *proto.add_unambiguous_public_bases() = b.ToFlatProto();
  proto.mutable_fields()->Reserve(fields.size());
  for (const auto& f : fields) *proto.add_fields() = f.ToFlatProto();
  for (const auto& l : lifetime_params)
    *proto.add_lifetime_params() = l.ToFlatProto();
  *proto.mutable_size_align() = size_align.ToFlatProto();
  *proto.mutable_trait_derives() = trait_derives.ToFlatProto();
  proto.set_is_derived_class(is_derived_class);
  proto.set_override_alignment(override_alignment);
  proto.set_safety_annotation(crubit::ToFlatProto(safety_annotation));
  proto.set_copy_constructor(crubit::ToFlatProto(copy_constructor));
  proto.set_move_constructor(crubit::ToFlatProto(move_constructor));
  proto.set_destructor(crubit::ToFlatProto(destructor));
  proto.set_is_trivial_abi(is_trivial_abi);
  proto.set_is_inheritable(is_inheritable);
  proto.set_is_abstract(is_abstract);
  if (nodiscard) proto.set_nodiscard(*nodiscard);
  proto.set_record_type(crubit::ToFlatProto(record_type));
  proto.set_is_aggregate(is_aggregate);
  proto.set_is_canonical_alias(is_canonical_alias);
  proto.mutable_children()->Reserve(children.size());
  for (const auto& child : children) {
    *proto.add_children() = crubit::ToFlatProto(*child);
  }
  if (enclosing_item_id)
    proto.set_enclosing_item_id(enclosing_item_id->value());
  proto.set_must_bind(must_bind);
  proto.set_overloads_operator_delete(overloads_operator_delete);
  proto.set_has_private_or_deleted_operator_delete(
      has_private_or_deleted_operator_delete);
  proto.set_impl_debug(impl_debug);
  proto.set_has_private_pointer_or_reference_fields(
      has_private_pointer_or_reference_fields);
  proto.set_detected_formatter(detected_formatter);
  proto.set_is_thread_safe(is_thread_safe);
  proto.mutable_lifetime_inputs()->Add(lifetime_inputs.begin(),
                                       lifetime_inputs.end());
  if (deprecated) proto.set_deprecated(*deprecated);
  proto.set_is_explicit_class_template_instantiation_definition(
      is_explicit_class_template_instantiation_definition);
  return proto;
}

flat_proto::Enumerator Enumerator::ToFlatProto() const {
  flat_proto::Enumerator proto;
  *proto.mutable_identifier() = identifier.ToFlatProto();
  *proto.mutable_value() = value.ToFlatProto();
  if (unknown_attr) proto.set_unknown_attr(*unknown_attr);
  if (deprecated) proto.set_deprecated(*deprecated);
  if (doc_comment) proto.set_doc_comment(*doc_comment);
  return proto;
}

flat_proto::Enum Enum::ToFlatProto() const {
  flat_proto::Enum proto;
  *proto.mutable_cc_name() = cc_name.ToFlatProto();
  *proto.mutable_rs_name() = rs_name.ToFlatProto();
  proto.set_unique_name(unique_name);
  proto.set_mangled_cc_name(mangled_cc_name);
  proto.set_id(id.value());
  proto.set_owning_target(owning_target.value());
  proto.set_source_loc(source_loc);
  *proto.mutable_underlying_type() = underlying_type.ToFlatProto();
  if (enumerators) {
    proto.mutable_enumerators()->Reserve(enumerators->size());
    for (const auto& e : *enumerators) {
      *proto.add_enumerators() = e.ToFlatProto();
    }
  } else {
    proto.set_is_incomplete(true);
  }
  if (unknown_attr) proto.set_unknown_attr(*unknown_attr);
  if (enclosing_item_id)
    proto.set_enclosing_item_id(enclosing_item_id->value());
  proto.set_must_bind(must_bind);
  proto.set_detected_formatter(detected_formatter);
  if (nodiscard) proto.set_nodiscard(nodiscard.value());
  if (deprecated) proto.set_deprecated(*deprecated);
  if (doc_comment) proto.set_doc_comment(*doc_comment);
  return proto;
}

flat_proto::GlobalVar GlobalVar::ToFlatProto() const {
  flat_proto::GlobalVar proto;
  *proto.mutable_cc_name() = cc_name.ToFlatProto();
  *proto.mutable_rs_name() = rs_name.ToFlatProto();
  proto.set_unique_name(unique_name);
  proto.set_id(id.value());
  proto.set_owning_target(owning_target.value());
  proto.set_source_loc(source_loc);
  if (mangled_name) proto.set_mangled_name(*mangled_name);
  *proto.mutable_type() = type.ToFlatProto();
  if (unknown_attr) proto.set_unknown_attr(*unknown_attr);
  if (enclosing_item_id)
    proto.set_enclosing_item_id(enclosing_item_id->value());
  proto.set_must_bind(must_bind);
  if (deprecated) proto.set_deprecated(*deprecated);
  if (doc_comment) proto.set_doc_comment(*doc_comment);
  return proto;
}

flat_proto::TypeAlias TypeAlias::ToFlatProto() const {
  flat_proto::TypeAlias proto;
  *proto.mutable_cc_name() = cc_name.ToFlatProto();
  *proto.mutable_rs_name() = rs_name.ToFlatProto();
  proto.set_unique_name(unique_name);
  proto.set_id(id.value());
  proto.set_owning_target(owning_target.value());
  if (doc_comment) proto.set_doc_comment(*doc_comment);
  if (unknown_attr) proto.set_unknown_attr(*unknown_attr);
  *proto.mutable_underlying_type() = underlying_type.ToFlatProto();
  proto.set_source_loc(source_loc);
  if (enclosing_item_id)
    proto.set_enclosing_item_id(enclosing_item_id->value());
  proto.set_must_bind(must_bind);
  if (deprecated) proto.set_deprecated(*deprecated);
  proto.mutable_lifetime_inputs()->Add(lifetime_inputs.begin(),
                                       lifetime_inputs.end());
  return proto;
}

FormattedError FormattedError::FromStatus(absl::Status status) {
  std::optional<absl::Cord> fmt_cord =
      status.GetPayload(FormattedError::kFmtPayloadTypeUrl);
  std::string fmt;
  if (fmt_cord) {
    fmt = std::string(*fmt_cord);
  } else {
    fmt = absl::StrCat("(unannotated `",
                       absl::StatusCodeToString(status.code()), "` status)");
  }
  return FormattedError(fmt, std::string(status.message()));
}

flat_proto::FormattedError FormattedError::ToFlatProto() const {
  flat_proto::FormattedError proto;
  proto.set_fmt(fmt_);
  proto.set_message(message_);
  return proto;
}

static std::string UnsupportedItemKindToString(UnsupportedItem::Kind kind) {
  switch (kind) {
    case UnsupportedItem::Kind::kFunc:
      return "Func";
    case UnsupportedItem::Kind::kGlobalVar:
      return "GlobalVar";
    case UnsupportedItem::Kind::kStruct:
      return "Struct";
    case UnsupportedItem::Kind::kUnion:
      return "Union";
    case UnsupportedItem::Kind::kClass:
      return "Class";
    case UnsupportedItem::Kind::kEnum:
      return "Enum";
    case UnsupportedItem::Kind::kTypeAlias:
      return "TypeAlias";
    case UnsupportedItem::Kind::kNamespace:
      return "Namespace";
    case UnsupportedItem::Kind::kConstructor:
      return "Constructor";
    case UnsupportedItem::Kind::kOther:
      return "Other";
  }
}

flat_proto::UnsupportedItem::Kind ToFlatProto(UnsupportedItem::Kind kind) {
  switch (kind) {
    case UnsupportedItem::Kind::kFunc:
      return flat_proto::UnsupportedItem::FUNC;
    case UnsupportedItem::Kind::kGlobalVar:
      return flat_proto::UnsupportedItem::GLOBAL_VAR;
    case UnsupportedItem::Kind::kStruct:
      return flat_proto::UnsupportedItem::STRUCT;
    case UnsupportedItem::Kind::kUnion:
      return flat_proto::UnsupportedItem::UNION;
    case UnsupportedItem::Kind::kClass:
      return flat_proto::UnsupportedItem::CLASS;
    case UnsupportedItem::Kind::kEnum:
      return flat_proto::UnsupportedItem::ENUM;
    case UnsupportedItem::Kind::kTypeAlias:
      return flat_proto::UnsupportedItem::TYPE_ALIAS;
    case UnsupportedItem::Kind::kNamespace:
      return flat_proto::UnsupportedItem::NAMESPACE;
    case UnsupportedItem::Kind::kConstructor:
      return flat_proto::UnsupportedItem::CONSTRUCTOR;
    case UnsupportedItem::Kind::kOther:
      return flat_proto::UnsupportedItem::OTHER;
  }
}

flat_proto::UnsupportedItem::Path UnsupportedItem::Path::ToFlatProto() const {
  flat_proto::UnsupportedItem::Path proto;
  *proto.mutable_ident() = crubit::ToFlatProto(ident);
  if (enclosing_item_id)
    proto.set_enclosing_item_id(enclosing_item_id->value());
  return proto;
}

flat_proto::UnsupportedItem UnsupportedItem::ToFlatProto() const {
  flat_proto::UnsupportedItem proto;
  proto.set_name(name);
  if (!unique_name.empty()) proto.set_unique_name(unique_name);
  proto.set_kind(crubit::ToFlatProto(kind));
  if (path) *proto.mutable_path() = path->ToFlatProto();
  for (const auto& error : errors) *proto.add_errors() = error.ToFlatProto();
  proto.set_source_loc(source_loc);
  proto.set_id(id.value());
  proto.set_must_bind(must_bind);
  if (defining_target) proto.set_defining_target(defining_target->value());
  if (inline_cpp_source_text) {
    proto.set_inline_cpp_source_text(*inline_cpp_source_text);
  }
  proto.set_is_compiler_generated(is_compiler_generated);
  return proto;
}

flat_proto::Comment Comment::ToFlatProto() const {
  flat_proto::Comment proto;
  proto.set_text(text);
  proto.set_id(id.value());
  proto.set_must_bind(must_bind);
  return proto;
}

flat_proto::Namespace Namespace::ToFlatProto() const {
  flat_proto::Namespace proto;
  *proto.mutable_cc_name() = cc_name.ToFlatProto();
  *proto.mutable_rs_name() = rs_name.ToFlatProto();
  proto.set_unique_name(unique_name);
  proto.set_id(id.value());
  proto.set_canonical_namespace_id(canonical_namespace_id.value());
  if (unknown_attr) proto.set_unknown_attr(*unknown_attr);
  proto.set_owning_target(owning_target.value());
  proto.mutable_children()->Reserve(children.size());
  for (const auto& child : children) {
    *proto.add_children() = crubit::ToFlatProto(*child);
  }
  if (enclosing_item_id)
    proto.set_enclosing_item_id(enclosing_item_id->value());
  proto.set_is_inline(is_inline);
  proto.set_must_bind(must_bind);
  if (deprecated) proto.set_deprecated(*deprecated);
  if (doc_comment) proto.set_doc_comment(*doc_comment);
  return proto;
}

flat_proto::Item ToFlatProto(const IR::Item& item) {
  flat_proto::Item proto;
  std::visit(
      visitor{
          [&](const Func& i) { *proto.mutable_func() = i.ToFlatProto(); },
          [&](const Record& i) { *proto.mutable_record() = i.ToFlatProto(); },
          [&](const IncompleteRecord& i) {
            *proto.mutable_incomplete_record() = i.ToFlatProto();
          },
          [&](const Enum& i) { *proto.mutable_enum_decl() = i.ToFlatProto(); },
          [&](const Constant& i) {
            *proto.mutable_constant() = i.ToFlatProto();
          },
          [&](const TypeAlias& i) {
            *proto.mutable_type_alias() = i.ToFlatProto();
          },
          [&](const GlobalVar& i) {
            *proto.mutable_global_var() = i.ToFlatProto();
          },
          [&](const UnsupportedItem& i) {
            *proto.mutable_unsupported_item() = i.ToFlatProto();
          },
          [&](const Comment& i) { *proto.mutable_comment() = i.ToFlatProto(); },
          [&](const Namespace& i) {
            *proto.mutable_namespace_decl() = i.ToFlatProto();
          },
          [&](const UseMod& i) { *proto.mutable_use_mod() = i.ToFlatProto(); },
          [&](const ExistingRustType& i) {
            *proto.mutable_existing_rust_type() = i.ToFlatProto();
          }},
      item.as_variant());
  return proto;
}

// We explicitly call .Reserve() on large AST subset Protobuf collections.
// This is necessary for proto generation because `RepeatedPtrField` can
// re-allocate memory continuously for large item counts, unlike standard C++
// containers.
void IR::ToFlatProto(flat_proto::IRProto* proto) const {
  proto->mutable_public_headers()->Reserve(public_headers.size());
  for (const auto& h : public_headers)
    *proto->add_public_headers() = h.ToFlatProto();
  proto->set_current_target(current_target.value());
  // Flat items list is deprecated and empty in serialization.

  for (const auto& [target, items] : top_level_items) {
    auto& list = (*proto->mutable_top_level_items())[target.value()];
    list.mutable_items()->Reserve(items.size());
    for (const auto& item : items) {
      *list.add_items() = crubit::ToFlatProto(*item);
    }
  }
  if (!crate_root_path.empty()) proto->set_crate_root_path(crate_root_path);
  for (const auto& [target, features] : crubit_features) {
    auto& set = (*proto->mutable_crubit_features())[target.value()];
    std::vector<std::string> sorted_features(features.begin(), features.end());
    absl::c_sort(sorted_features);
    set.mutable_features()->Add(sorted_features.begin(), sorted_features.end());
  }
  for (const auto& [target, name] : crate_names) {
    (*proto->mutable_crate_names())[target.value()] = name;
  }
  proto->mutable_unstable_rust_features()->Add(unstable_rust_features.begin(),
                                               unstable_rust_features.end());
  proto->mutable_reexported_namespaces()->Add(reexported_namespaces.begin(),
                                              reexported_namespaces.end());
}

std::string ItemToString(const IR::Item& item) {
  return ToFlatProto(item).ShortDebugString();
}

void SetMustBindItem(IR::Item& item) {
  // All IR::Item variants have a `must_bind` field.
  std::visit([](auto& item_variant) { item_variant.must_bind = true; },
             item.as_variant());
}

ItemId Item::id() const {
  return std::visit([](const auto& val) { return val.id; }, as_variant());
}

std::vector<ItemId> IR::top_level_item_ids(const BazelLabel& target) const {
  std::vector<ItemId> ids;
  auto it = top_level_items.find(target);
  if (it != top_level_items.end()) {
    ids.reserve(it->second.size());
    for (const auto& item : it->second) {
      ids.push_back(item->id());
    }
  }
  return ids;
}

std::vector<ItemId> Record::child_item_ids() const {
  std::vector<ItemId> ids;
  ids.reserve(children.size());
  for (const auto& child : children) {
    ids.push_back(child->id());
  }
  return ids;
}

std::vector<ItemId> Namespace::child_item_ids() const {
  std::vector<ItemId> ids;
  ids.reserve(children.size());
  for (const auto& child : children) {
    ids.push_back(child->id());
  }
  return ids;
}

// Produces a nested IR which inlines child items on namespaces and records to
// replace the legacy representation (an array of item IDs).
// See crubit.rs-better-ir for more context.
void IR::BuildTree(
    absl::flat_hash_map<BazelLabel, std::vector<ItemId>> top_level_item_ids,
    absl::flat_hash_map<ItemId, std::vector<ItemId>> child_item_ids) {
  top_level_items.clear();
  absl::flat_hash_map<ItemId, std::shared_ptr<Item>> item_map;
  item_map.reserve(items.size());

  for (const auto& item : items) {
    ItemId id =
        std::visit([](const auto& val) { return val.id; }, item.as_variant());
    item_map[id] = std::make_shared<Item>(item);
  }

  for (auto& item : items) {
    std::visit(visitor{[&](Record& r) {
                         if (auto it = child_item_ids.find(r.id);
                             it != child_item_ids.end()) {
                           r.children.reserve(it->second.size());
                           for (const auto& child_id : it->second) {
                             if (auto item_it = item_map.find(child_id);
                                 item_it != item_map.end()) {
                               r.children.push_back(item_it->second);
                             }
                           }
                         }
                       },
                       [&](Namespace& ns) {
                         if (auto it = child_item_ids.find(ns.id);
                             it != child_item_ids.end()) {
                           ns.children.reserve(it->second.size());
                           for (const auto& child_id : it->second) {
                             if (auto item_it = item_map.find(child_id);
                                 item_it != item_map.end()) {
                               ns.children.push_back(item_it->second);
                             }
                           }
                         }
                       },
                       [](auto& other) {}},
               item.as_variant());
  }

  for (auto& [id, shared_item] : item_map) {
    std::visit(visitor{[&](Record& r) {
                         if (auto it = child_item_ids.find(r.id);
                             it != child_item_ids.end()) {
                           r.children.reserve(it->second.size());
                           for (const auto& child_id : it->second) {
                             if (auto item_it = item_map.find(child_id);
                                 item_it != item_map.end()) {
                               r.children.push_back(item_it->second);
                             }
                           }
                         }
                       },
                       [&](Namespace& ns) {
                         if (auto it = child_item_ids.find(ns.id);
                             it != child_item_ids.end()) {
                           ns.children.reserve(it->second.size());
                           for (const auto& child_id : it->second) {
                             if (auto item_it = item_map.find(child_id);
                                 item_it != item_map.end()) {
                               ns.children.push_back(item_it->second);
                             }
                           }
                         }
                       },
                       [](auto& other) {}},
               shared_item->as_variant());
  }

  for (const auto& [target, item_ids] : top_level_item_ids) {
    auto& list = top_level_items[target];
    list.reserve(item_ids.size());
    for (const auto& id : item_ids) {
      if (auto it = item_map.find(id); it != item_map.end()) {
        list.push_back(it->second);
      }
    }
  }
}

}  // namespace crubit
