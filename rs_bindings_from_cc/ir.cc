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

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/strings/cord.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/strong_int.h"

namespace crubit {

namespace flat_proto = rs_bindings_from_cc::ir_proto::flat;

namespace {
// https://en.cppreference.com/w/cpp/utility/variant/visit
template <typename... Ts>
struct visitor : Ts... {
  using Ts::operator()...;
};
}  // namespace

flat_proto::CcType CcType::ToFlatProto() const {
  flat_proto::CcType proto;
  proto.set_is_const(is_const);
  proto.set_unknown_attr(unknown_attr);
  proto.mutable_explicit_lifetimes()->Add(explicit_lifetimes.begin(),
                                          explicit_lifetimes.end());

  std::visit(
      visitor{
          [&](const CcType::Primitive& primitive) {
            proto.mutable_primitive()->set_spelling(primitive.spelling);
          },
          [&](const CcType::PointerType& pointer) {
            auto* p = proto.mutable_pointer();
            p->set_kind(pointer.kind);
            if (pointer.lifetime) {
              p->set_lifetime((*pointer.lifetime).value());
            }
            *p->mutable_pointee_type() = pointer.pointee_type->ToFlatProto();
          },
          [&](const CcType::FuncPointer& func_value) {
            auto* f = proto.mutable_func_pointer();
            f->set_non_null(func_value.non_null);
            f->set_call_conv(func_value.call_conv);
            for (const CcType& type : func_value.param_and_return_types) {
              *f->add_param_and_return_types() = type.ToFlatProto();
            }
            f->mutable_lifetime_inputs()->Add(
                func_value.lifetime_inputs.begin(),
                func_value.lifetime_inputs.end());
          },
          [&](ItemId id) { proto.set_decl(static_cast<int64_t>(id.value())); },
          [&](const FormattedError& error) {
            error.WriteToProto(*proto.mutable_error());
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
      nullable ? PointerTypeKind::NULLABLE : PointerTypeKind::NON_NULL,
      lifetime);
}

CcType CcType::OwnedPointerTo(CcType pointee_type,
                              std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type), PointerTypeKind::OWNED,
                              lifetime);
}

CcType CcType::LValueReferenceTo(CcType pointee_type,
                                 std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type),
                              PointerTypeKind::L_VALUE_REF, lifetime);
}

CcType CcType::RValueReferenceTo(CcType pointee_type,
                                 std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type),
                              PointerTypeKind::R_VALUE_REF, lifetime);
}

void HeaderName::WriteToProto(ir_proto::HeaderName& proto) const {
  proto.set_name(name_);
}

void FormattedError::WriteToProto(ir_proto::FormattedError& proto) const {
  proto.set_fmt(fmt_);
  proto.set_message(message_);
}

void Identifier::WriteToProto(ir_proto::Identifier& proto) const {
  proto.set_identifier(identifier_);
}

void IntegerConstant::WriteToProto(ir_proto::IntegerConstant& proto) const {
  proto.set_is_negative(is_negative_);
  proto.set_wrapped_value(static_cast<int64_t>(wrapped_value_));
}

void Operator::WriteToProto(ir_proto::Operator& proto) const {
  proto.set_name(name_);
}

static std::string SpecialNameToString(SpecialName special_name) {
  switch (special_name) {
    case SpecialName::DESTRUCTOR:
      return "Destructor";
    case SpecialName::CONSTRUCTOR:
      return "Constructor";
    default:
      return "Unspecified";
  }
}

void WriteToProto(const UnqualifiedIdentifier& unqualified_identifier,
                  ir_proto::UnqualifiedIdentifier& proto) {
  std::visit(visitor{[&](const Identifier& id) {
                       id.WriteToProto(*proto.mutable_ident());
                     },
                     [&](const Operator& op) {
                       op.WriteToProto(*proto.mutable_oper());
                     },
                     [&](const SpecialName& special_name) {
                       proto.set_special_name(special_name);
                     },
                     [&](const ConversionOperator& conversion_operator) {
                       proto.mutable_conversion_operator();
                     }},
             unqualified_identifier);
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
            if (annotation.label_hint.has_value()) {
              b->set_label_hint(*annotation.label_hint);
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
          [&](const StdSharedPtr& std_shared_ptr) {
            *proto.mutable_std_shared_ptr()->mutable_element_type() =
                std_shared_ptr.element_type.ToFlatProto();
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

flat_proto::TraitDerives TraitDerives::ToFlatProto() const {
  flat_proto::TraitDerives proto;
  proto.set_clone(clone);
  proto.set_copy(copy);
  proto.set_debug(debug);
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

}  // namespace crubit
