// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

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
#include "common/string_type.h"
#include "common/strong_int.h"
#include "llvm/include/llvm/Support/JSON.h"

namespace crubit {
namespace {
// https://en.cppreference.com/w/cpp/utility/variant/visit
template <typename... Ts>
struct visitor : Ts... {
  using Ts::operator()...;
};
}  // namespace

template <class T>
llvm::json::Value toJSON(const T& t) {
  return t.ToJson();
}

template <typename TTag, typename TInt>
llvm::json::Value toJSON(const crubit::StrongInt<TTag, TInt> strong_int) {
  return llvm::json::Value(strong_int.value());
}

template <typename TTag>
llvm::json::Value toJSON(const crubit::StringType<TTag> string_type) {
  return llvm::json::Value(string_type.value());
}

template <class T>
llvm::json::Value toJSON(const absl::StatusOr<T>& t) {
  if (t.ok()) {
    return llvm::json::Object{{"Ok", *t}};
  }
  return llvm::json::Object{{"Err", std::string(t.status().message())}};
}

llvm::json::Value HeaderName::ToJson() const {
  return llvm::json::Object{
      {"name", name_},
  };
}

llvm::json::Value LifetimeName::ToJson() const {
  return llvm::json::Object{
      {"name", name},
      {"id", id},
  };
}

llvm::json::Value CcType::ToJson() const {
  llvm::json::Object variant_object = std::visit(
      visitor{
          [&](CcType::Primitive primitive) {
            return llvm::json::Object{{"Primitive", primitive.spelling}};
          },
          [&](CcType::PointerType pointer) {
            return llvm::json::Object{
                {"Pointer",
                 llvm::json::Object{
                     {
                         "kind",
                         [&]() -> llvm::json::Value {
                           switch (pointer.kind) {
                             case PointerTypeKind::kLValueRef:
                               return "LValueRef";
                             case PointerTypeKind::kRValueRef:
                               return "RValueRef";
                             case PointerTypeKind::kNullable:
                               return "Nullable";
                             case PointerTypeKind::kNonNull:
                               return "NonNull";
                           }
                         }(),
                     },
                     {"lifetime", pointer.lifetime},
                     {"pointee_type", *pointer.pointee_type},
                 }},
            };
          },
          [&](const CcType::FuncPointer& func_value) {
            std::vector<llvm::json::Value> param_and_return_type_values;
            param_and_return_type_values.reserve(
                func_value.param_and_return_types.size());
            for (const CcType& type : func_value.param_and_return_types) {
              param_and_return_type_values.push_back(type.ToJson());
            }
            return llvm::json::Object{
                {"FuncPointer",
                 llvm::json::Object{
                     {"non_null", func_value.non_null},
                     {
                         "call_conv",
                         [&]() -> llvm::json::Value {
                           switch (func_value.call_conv) {
                             case CallingConv::kC:
                               return "cdecl";
                             case CallingConv::kX86FastCall:
                               return "fastcall";
                             case CallingConv::kX86VectorCall:
                               return "vectorcall";
                             case CallingConv::kX864ThisCall:
                               return "thiscall";
                             case CallingConv::kX86StdCall:
                               return "stdcall";
                             case CallingConv::kWin64:
                               return "ms_abi";
                           }
                         }(),
                     },
                     {"param_and_return_types", param_and_return_type_values},
                 }},
            };
          },
          [&](ItemId id) { return llvm::json::Object{{"Record", id}}; }},
      variant);

  return llvm::json::Object{
      {"variant", std::move(variant_object)},
      {"is_const", is_const},
      {"unknown_attr", unknown_attr},
  };
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

CcType CcType::LValueReferenceTo(CcType pointee_type,
                                 std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type),
                              PointerTypeKind::kLValueRef, lifetime);
}

CcType CcType::RValueReferenceTo(CcType pointee_type, LifetimeId lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type),
                              PointerTypeKind::kRValueRef, lifetime);
}

llvm::json::Value Identifier::ToJson() const {
  return llvm::json::Object{
      {"identifier", identifier_},
  };
}

llvm::json::Value IntegerConstant::ToJson() const {
  return llvm::json::Object{
      {"is_negative", is_negative_},
      {"wrapped_value", wrapped_value_},
  };
}

llvm::json::Value Operator::ToJson() const {
  return llvm::json::Object{
      {"name", name_},
  };
}

static std::string SpecialNameToString(SpecialName special_name) {
  switch (special_name) {
    case SpecialName::kDestructor:
      return "Destructor";
    case SpecialName::kConstructor:
      return "Constructor";
  }
}

llvm::json::Value toJSON(const UnqualifiedIdentifier& unqualified_identifier) {
  if (auto* id = std::get_if<Identifier>(&unqualified_identifier)) {
    return llvm::json::Object{
        {"Identifier", *id},
    };
  } else if (auto* op = std::get_if<Operator>(&unqualified_identifier)) {
    return llvm::json::Object{
        {"Operator", *op},
    };
  } else {
    SpecialName special_name = std::get<SpecialName>(unqualified_identifier);
    return llvm::json::Object{
        {SpecialNameToString(special_name), nullptr},
    };
  }
}

llvm::json::Value FuncParam::ToJson() const {
  return llvm::json::Object{
      {"type", type},
      {"identifier", identifier},
      {"unknown_attr", unknown_attr},
  };
}

std::ostream& operator<<(std::ostream& o, const SpecialName& special_name) {
  return o << SpecialNameToString(special_name);
}

UnqualifiedIdentifier& TranslatedUnqualifiedIdentifier::rs_identifier() {
  if (crubit_rust_name.has_value()) {
    return *crubit_rust_name;
  }
  return cc_identifier;
}

Identifier& TranslatedIdentifier::rs_identifier() {
  if (crubit_rust_name.has_value()) {
    return *crubit_rust_name;
  }
  return cc_identifier;
}

llvm::json::Value MemberFuncMetadata::InstanceMethodMetadata::ToJson() const {
  const char* reference_str = nullptr;
  switch (reference) {
    case MemberFuncMetadata::kLValue:
      reference_str = "LValue";
      break;
    case MemberFuncMetadata::kRValue:
      reference_str = "RValue";
      break;
    case MemberFuncMetadata::kUnqualified:
      reference_str = "Unqualified";
      break;
  }

  return llvm::json::Object{
      {"reference", reference_str},
      {"is_const", is_const},
      {"is_virtual", is_virtual},
  };
}

llvm::json::Value MemberFuncMetadata::ToJson() const {
  return llvm::json::Object{
      {"record_id", record_id},
      {"instance_method_metadata", instance_method_metadata},
  };
}

llvm::json::Value TypeMapOverride::ToJson() const {
  llvm::json::Object override{
      {"rs_name", rs_name},
      {"cc_name", cc_name},
      {"type_parameters", type_parameters},
      {"owning_target", owning_target},
      {"is_same_abi", is_same_abi},
      {"id", id},
      {"must_bind", must_bind},
  };
  if (size_align.has_value()) {
    override.insert({"size_align", size_align->ToJson()});
  }

  return llvm::json::Object{
      {"TypeMapOverride", std::move(override)},
  };
}

llvm::json::Value UseMod::ToJson() const {
  llvm::json::Object use_mod{
      {"path", path},
      {"mod_name", mod_name},
      {"id", id},
      {"must_bind", must_bind},
  };

  return llvm::json::Object{
      {"UseMod", std::move(use_mod)},
  };
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

llvm::json::Value Func::ToJson() const {
  llvm::json::Object func{
      {"cc_name", cc_name},
      {"rs_name", rs_name},
      {"owning_target", owning_target},
      {"doc_comment", doc_comment},
      {"mangled_name", mangled_name},
      {"return_type", return_type},
      {"params", params},
      {"lifetime_params", lifetime_params},
      {"is_inline", is_inline},
      {"member_func_metadata", member_func_metadata},
      {"is_extern_c", is_extern_c},
      {"is_noreturn", is_noreturn},
      {"nodiscard", nodiscard},
      {"deprecated", deprecated},
      {"has_c_calling_convention", has_c_calling_convention},
      {"is_member_or_descendant_of_class_template",
       is_member_or_descendant_of_class_template},
      {"safety_annotation", SafetyAnnotationToString(safety_annotation)},
      {"source_loc", source_loc},
      {"id", id},
      {"enclosing_item_id", enclosing_item_id},
      {"adl_enclosing_record", adl_enclosing_record},
      {"must_bind", must_bind},
  };

  return llvm::json::Object{
      {"Func", std::move(func)},
  };
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

std::ostream& operator<<(std::ostream& o, const AccessSpecifier& access) {
  return o << AccessToString(access);
}

llvm::json::Value Field::ToJson() const {
  return llvm::json::Object{
      {"identifier", identifier},
      {"doc_comment", doc_comment},
      {"type", type},
      {"access", AccessToString(access)},
      {"offset", offset},
      {"size", size},
      {"unknown_attr", unknown_attr},
      {"is_no_unique_address", is_no_unique_address},
      {"is_bitfield", is_bitfield},
      {"is_inheritable", is_inheritable},
  };
}

llvm::json::Value toJSON(const SpecialMemberFunc& f) {
  switch (f) {
    case SpecialMemberFunc::kTrivial:
      return "Trivial";
    case SpecialMemberFunc::kNontrivialMembers:
      return "NontrivialMembers";
    case SpecialMemberFunc::kNontrivialUserDefined:
      return "NontrivialUserDefined";
    case SpecialMemberFunc::kUnavailable:
      return "Unavailable";
  }
}

llvm::json::Value BaseClass::ToJson() const {
  return llvm::json::Object{
      {"base_record_id", base_record_id},
      {"offset", offset},
  };
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

std::ostream& operator<<(std::ostream& o, const RecordType& record_type) {
  return o << RecordTypeToString(record_type);
}

llvm::json::Value IncompleteRecord::ToJson() const {
  llvm::json::Object record{{"cc_name", cc_name},
                            {"rs_name", rs_name},
                            {"id", id},
                            {"owning_target", owning_target},
                            {"unknown_attr", unknown_attr},
                            {"record_type", RecordTypeToString(record_type)},
                            {"enclosing_item_id", enclosing_item_id},
                            {"must_bind", must_bind}};

  return llvm::json::Object{
      {"IncompleteRecord", std::move(record)},
  };
}

llvm::json::Value SizeAlign::ToJson() const {
  return llvm::json::Object{
      {"size", size},
      {"alignment", alignment},
  };
}

llvm::json::Value BridgeType::ToJson() const {
  return std::visit(
      visitor{
          [&](const BridgeType::Annotation& annotation) {
            return llvm::json::Object{{
                "Annotation",
                llvm::json::Object{
                    {"rust_name", annotation.rust_name},
                    {"rust_to_cpp_converter", annotation.rust_to_cpp_converter},
                    {"cpp_to_rust_converter", annotation.cpp_to_rust_converter},
                },
            }};
          },
          [&](const BridgeType::StdOptional& std_optional) {
            return llvm::json::Object{
                {"StdOptional", std_optional.inner_type->ToJson()}};
          },
          [&](const BridgeType::StdPair& std_pair) {
            return llvm::json::Object{
                {"StdPair", llvm::json::Array{
                                std_pair.first_type->ToJson(),
                                std_pair.second_type->ToJson(),
                            }}};
          },
      },
      variant);
}

llvm::json::Value TemplateArg::ToJson() const {
  return llvm::json::Object{
      {"type", type},
  };
}

llvm::json::Value TemplateSpecialization::ToJson() const {
  return llvm::json::Object{
      {"template_name", template_name},
      {"template_args", template_args},
  };
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

llvm::json::Value TraitDerives::ToJson() const {
  return llvm::json::Object{
      // <internal link> start
      {"clone", TraitImplPolarityToString(clone)},
      {"copy", TraitImplPolarityToString(copy)},
      {"debug", TraitImplPolarityToString(debug)},
      // <internal link> end
      {"send", send},
      {"sync", sync},
      {"custom", custom},
  };
}

llvm::json::Value Record::ToJson() const {
  std::vector<llvm::json::Value> json_item_ids;
  json_item_ids.reserve(child_item_ids.size());
  for (const auto& id : child_item_ids) {
    json_item_ids.push_back(id.value());
  }

  llvm::json::Object record{
      {"rs_name", rs_name},
      {"cc_name", cc_name},
      {"cc_preferred_name", cc_preferred_name},
      {"mangled_cc_name", mangled_cc_name},
      {"id", id},
      {"owning_target", owning_target},
      {"defining_target", defining_target},
      {"template_specialization", template_specialization},
      {"unknown_attr", unknown_attr},
      {"doc_comment", doc_comment},
      {"bridge_type", bridge_type},
      {"source_loc", source_loc},
      {"unambiguous_public_bases", unambiguous_public_bases},
      {"fields", fields},
      {"lifetime_params", lifetime_params},
      {"size_align", size_align.ToJson()},
      {"trait_derives", trait_derives.ToJson()},
      {"is_derived_class", is_derived_class},
      {"override_alignment", override_alignment},
      {"is_unsafe_type", is_unsafe_type},
      {"copy_constructor", copy_constructor},
      {"move_constructor", move_constructor},
      {"destructor", destructor},
      {"is_trivial_abi", is_trivial_abi},
      {"is_inheritable", is_inheritable},
      {"is_abstract", is_abstract},
      {"nodiscard", nodiscard},
      {"record_type", RecordTypeToString(record_type)},
      {"is_aggregate", is_aggregate},
      {"is_anon_record_with_typedef", is_anon_record_with_typedef},
      {"child_item_ids", std::move(json_item_ids)},
      {"enclosing_item_id", enclosing_item_id},
      {"must_bind", must_bind},
  };

  return llvm::json::Object{
      {"Record", std::move(record)},
  };
}

llvm::json::Value Enumerator::ToJson() const {
  return llvm::json::Object{
      {"identifier", identifier},
      {"value", value},
      {"unknown_attr", unknown_attr},
  };
}

llvm::json::Value Enum::ToJson() const {
  llvm::json::Object enum_ir{
      {"cc_name", cc_name},
      {"rs_name", rs_name},
      {"id", id},
      {"owning_target", owning_target},
      {"source_loc", source_loc},
      {"underlying_type", underlying_type},
      {"enumerators", enumerators},
      {"unknown_attr", unknown_attr},
      {"enclosing_item_id", enclosing_item_id},
      {"must_bind", must_bind},
  };

  return llvm::json::Object{
      {"Enum", std::move(enum_ir)},
  };
}

llvm::json::Value GlobalVar::ToJson() const {
  llvm::json::Object var{
      {"cc_name", cc_name},
      {"rs_name", rs_name},
      {"id", id},
      {"owning_target", owning_target},
      {"source_loc", source_loc},
      {"mangled_name", mangled_name},
      {"type", type},
      {"unknown_attr", unknown_attr},
      {"enclosing_item_id", enclosing_item_id},
      {"must_bind", must_bind},
  };

  return llvm::json::Object{
      {"GlobalVar", std::move(var)},
  };
}

llvm::json::Value TypeAlias::ToJson() const {
  llvm::json::Object type_alias{{"cc_name", cc_name},
                                {"rs_name", rs_name},
                                {"id", id},
                                {"owning_target", owning_target},
                                {"unknown_attr", unknown_attr},
                                {"doc_comment", doc_comment},
                                {"underlying_type", underlying_type},
                                {"source_loc", source_loc},
                                {"enclosing_item_id", enclosing_item_id},
                                {"must_bind", must_bind}};

  return llvm::json::Object{
      {"TypeAlias", std::move(type_alias)},
  };
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

llvm::json::Value FormattedError::ToJson() const {
  return llvm::json::Object{
      {"fmt", fmt_},
      {"message", message_},
  };
}

static std::string UnsupportedItemKindToString(UnsupportedItem::Kind kind) {
  switch (kind) {
    case UnsupportedItem::Kind::kValue:
      return "Value";
    case UnsupportedItem::Kind::kType:
      return "Type";
    case UnsupportedItem::Kind::kUnnameable:
      return "Unnameable";
  }
}

llvm::json::Value UnsupportedItem::Path::ToJson() const {
  return llvm::json::Object{
      {"ident", ident},
      {"enclosing_item_id", enclosing_item_id},
  };
}

llvm::json::Value UnsupportedItem::ToJson() const {
  std::vector<llvm::json::Value> json_errors;
  json_errors.reserve(errors.size());
  for (const auto& error : errors) {
    json_errors.push_back(error.ToJson());
  }

  llvm::json::Object unsupported{
      {"name", name},
      {"kind", UnsupportedItemKindToString(kind)},
      {"path", path},
      {"errors", json_errors},
      {"source_loc", source_loc},
      {"id", id},
      {"must_bind", must_bind},
  };

  return llvm::json::Object{
      {"UnsupportedItem", std::move(unsupported)},
  };
}

llvm::json::Value Comment::ToJson() const {
  llvm::json::Object comment{
      {"text", text},
      {"id", id},
      {"must_bind", must_bind},
  };
  comment["id"] = id.value();
  return llvm::json::Object{
      {"Comment", std::move(comment)},
  };
}

llvm::json::Value Namespace::ToJson() const {
  std::vector<llvm::json::Value> json_item_ids;
  json_item_ids.reserve(child_item_ids.size());
  for (const auto& id : child_item_ids) {
    json_item_ids.push_back(id.value());
  }

  llvm::json::Object ns{
      {"cc_name", cc_name},
      {"rs_name", rs_name},
      {"id", id},
      {"canonical_namespace_id", canonical_namespace_id},
      {"unknown_attr", unknown_attr},
      {"owning_target", owning_target},
      {"child_item_ids", std::move(json_item_ids)},
      {"enclosing_item_id", enclosing_item_id},
      {"is_inline", is_inline},
      {"must_bind", must_bind},
  };

  return llvm::json::Object{
      {"Namespace", std::move(ns)},
  };
}

llvm::json::Value IR::ToJson() const {
  std::vector<llvm::json::Value> json_items;
  json_items.reserve(items.size());
  for (const auto& item : items) {
    std::visit([&](auto&& item) { json_items.push_back(item.ToJson()); }, item);
  }
  CHECK_EQ(json_items.size(), items.size());

  std::vector<llvm::json::Value> top_level_ids;
  top_level_ids.reserve(top_level_item_ids.size());
  for (const auto& id : top_level_item_ids) {
    top_level_ids.push_back(id.value());
  }

  llvm::json::Object features_json;
  for (const auto& [target, features] : crubit_features) {
    std::vector<llvm::json::Value> feature_array;
    for (const std::string& feature : features) {
      feature_array.push_back(feature);
    }
    features_json[target.value()] = std::move(feature_array);
  }

  llvm::json::Object result{
      {"public_headers", public_headers},
      {"current_target", current_target},
      {"items", std::move(json_items)},
      {"top_level_item_ids", std::move(top_level_ids)},
      {"crubit_features", std::move(features_json)},
  };
  if (!crate_root_path.empty()) {
    result["crate_root_path"] = crate_root_path;
  }
  return std::move(result);
}

std::string ItemToString(const IR::Item& item) {
  return std::visit(
      [&](auto&& item) { return llvm::formatv("{0}", item.ToJson()); }, item);
}

void SetMustBindItem(IR::Item& item) {
  // All IR::Item variants have a `must_bind` field.
  std::visit([](auto& item_variant) { item_variant.must_bind = true; }, item);
}

}  // namespace crubit
