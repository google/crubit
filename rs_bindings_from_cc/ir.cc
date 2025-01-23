// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

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
#include "absl/types/optional.h"
#include "common/string_type.h"
#include "common/strong_int.h"
#include "clang/AST/Type.h"
#include "llvm/Support/JSON.h"

namespace crubit {

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

llvm::json::Value toJSON(const RsType& type) {
  if (auto* rs_type_named = std::get_if<RsTypeNamed>(&type)) {
    return llvm::json::Object{{"NamedType", rs_type_named->ToJson()}};
  } else if (auto* decl_id = std::get_if<ItemId>(&type)) {
    return llvm::json::Object{
        {"ItemIdType", llvm::json::Object{{"decl_id", decl_id->value()}}}};
  } else {
    return llvm::json::Object{
        {"UnknownAttr",
         llvm::json::Object{
             {"unknown_attr", std::get<UnknownAttr>(type).unknown_attr}}},
    };
  }
}

llvm::json::Value RsTypeNamed::ToJson() const {
  std::vector<llvm::json::Value> type_args_vector;
  type_args_vector.reserve(type_args.size());
  for (const RsType& type : type_args) {
    type_args_vector.push_back(toJSON(type));
  }
  return llvm::json::Object{
      {"name", name},
      {"lifetime_args", lifetime_args},
      {"type_args", type_args_vector},
  };
}

llvm::json::Value CcType::ToJson() const {
  return llvm::json::Object{
      {"name", decl_id.has_value() ? llvm::json::Value(nullptr)
                                   : llvm::json::Value(name)},
      {"is_const", is_const},
      {"type_args", type_args},
      {"decl_id", decl_id},
  };
}

namespace {
enum class ValueCategory { kLvalue, kRvalue };

MappedType PointerOrReferenceTo(
    MappedType pointee_type, absl::string_view cc_ptr_name,
    ValueCategory value_category, std::optional<LifetimeId> lifetime,
    std::optional<clang::RefQualifierKind> ref_qualifier_kind, bool nullable) {
  bool has_lifetime = lifetime.has_value();
  absl::string_view rs_name;
  if (value_category == ValueCategory::kLvalue) {
    if (has_lifetime) {
      if (ref_qualifier_kind.has_value() &&
          ref_qualifier_kind.value() == clang::RefQualifierKind::RQ_RValue) {
        rs_name = pointee_type.cpp_type.is_const ? internal::kRustRvalueRefConst
                                                 : internal::kRustRvalueRefMut;
      } else {
        rs_name = pointee_type.cpp_type.is_const ? internal::kRustRefConst
                                                 : internal::kRustRefMut;
      }
    } else {
      rs_name = pointee_type.cpp_type.is_const ? internal::kRustPtrConst
                                               : internal::kRustPtrMut;
    }
  } else {
    CHECK(has_lifetime);
    rs_name = pointee_type.cpp_type.is_const ? internal::kRustRvalueRefConst
                                             : internal::kRustRvalueRefMut;
  }
  RsTypeNamed rs_type = RsTypeNamed{.name = std::string(rs_name)};
  if (has_lifetime) {
    rs_type.lifetime_args.push_back(*std::move(lifetime));
  }
  rs_type.type_args.push_back(std::move(pointee_type.rs_type));
  if (has_lifetime && nullable) {
    rs_type = RsTypeNamed{.name = "Option", .type_args = {std::move(rs_type)}};
  }
  return MappedType{
      .rs_type = std::move(rs_type),
      .cpp_type = CcType{.name = std::string(cc_ptr_name),
                         .type_args = {std::move(pointee_type.cpp_type)}},
  };
}
}  // namespace

MappedType MappedType::PointerTo(
    MappedType pointee_type, std::optional<LifetimeId> lifetime,
    std::optional<clang::RefQualifierKind> ref_qualifier_kind, bool nullable) {
  return PointerOrReferenceTo(std::move(pointee_type), internal::kCcPtr,
                              ValueCategory::kLvalue, lifetime,
                              ref_qualifier_kind, nullable);
}

MappedType MappedType::LValueReferenceTo(MappedType pointee_type,
                                         std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type), internal::kCcLValueRef,
                              ValueCategory::kLvalue, lifetime,
                              /*ref_qualifier_kind=*/std::nullopt,
                              /*nullable=*/false);
}

MappedType MappedType::RValueReferenceTo(MappedType pointee_type,
                                         LifetimeId lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type), internal::kCcRValueRef,
                              ValueCategory::kRvalue, lifetime,
                              /*ref_qualifier_kind=*/std::nullopt,
                              /*nullable=*/false);
}

MappedType MappedType::FuncPtr(absl::string_view cc_call_conv,
                               absl::string_view rs_abi,
                               std::optional<LifetimeId> lifetime,
                               MappedType return_type,
                               std::vector<MappedType> param_types) {
  MappedType result = FuncRef(cc_call_conv, rs_abi, lifetime,
                              std::move(return_type), std::move(param_types));

  CHECK_EQ(result.cpp_type.name, internal::kCcLValueRef);
  result.cpp_type.name = std::string(internal::kCcPtr);

  RsType rs_func_ptr_type = std::move(result.rs_type);
  const RsTypeNamed* named = std::get_if<RsTypeNamed>(&rs_func_ptr_type);
  CHECK(named != nullptr &&
        named->name.substr(0, internal::kRustFuncPtr.length()) ==
            internal::kRustFuncPtr);
  result.rs_type =
      RsTypeNamed{.name = "Option", .type_args = {std::move(rs_func_ptr_type)}};

  return result;
}

MappedType MappedType::FuncRef(absl::string_view cc_call_conv,
                               absl::string_view rs_abi,
                               std::optional<LifetimeId> lifetime,
                               MappedType return_type,
                               std::vector<MappedType> param_types) {
  std::vector<MappedType> type_args = std::move(param_types);
  type_args.push_back(std::move(return_type));

  std::vector<CcType> cpp_type_args;
  std::vector<RsType> rs_type_args;
  cpp_type_args.reserve(type_args.size());
  rs_type_args.reserve(type_args.size());
  for (MappedType& type_arg : type_args) {
    cpp_type_args.push_back(std::move(type_arg.cpp_type));
    rs_type_args.push_back(std::move(type_arg.rs_type));
  }

  CcType cc_func_value_type = CcType{
      .name = absl::StrCat(internal::kCcFuncValue, " ", cc_call_conv),
      .type_args = std::move(cpp_type_args),
  };
  CcType cc_func_ref_type = CcType{.name = std::string(internal::kCcLValueRef),
                                   .type_args = {cc_func_value_type}};

  // Rust cannot express a function *value* type, only function pointer types.
  RsTypeNamed rs_func_ptr_type = RsTypeNamed{
      .name = absl::StrCat(internal::kRustFuncPtr, " ", rs_abi),
      .type_args = std::move(rs_type_args),
  };
  if (lifetime.has_value())
    rs_func_ptr_type.lifetime_args.push_back(*std::move(lifetime));

  return MappedType{
      .rs_type = std::move(rs_func_ptr_type),
      .cpp_type = std::move(cc_func_ref_type),
  };
}

llvm::json::Value MappedType::ToJson() const {
  return llvm::json::Object{
      {"rs_type", rs_type},
      {"cpp_type", cpp_type},
  };
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
      {"name", name},
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
  llvm::json::Object record{
      {"cc_name", cc_name},
      {"rs_name", rs_name},
      {"id", id},
      {"owning_target", owning_target},
      {"unknown_attr", unknown_attr},
      {"record_type", RecordTypeToString(record_type)},
      {"enclosing_item_id", enclosing_item_id},
  };

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

llvm::json::Value BridgeTypeInfo::ToJson() const {
  return llvm::json::Object{
      {"bridge_type", bridge_type},
      {"rust_to_cpp_converter", rust_to_cpp_converter},
      {"cpp_to_rust_converter", cpp_to_rust_converter},
  };
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

absl::Nullable<TraitImplPolarity*> TraitDerives::Polarity(
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
      {"bridge_type_info", bridge_type_info},
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
      {"identifier", identifier},
      {"id", id},
      {"owning_target", owning_target},
      {"source_loc", source_loc},
      {"underlying_type", underlying_type},
      {"enumerators", enumerators},
      {"unknown_attr", unknown_attr},
      {"enclosing_item_id", enclosing_item_id},
  };

  return llvm::json::Object{
      {"Enum", std::move(enum_ir)},
  };
}

llvm::json::Value TypeAlias::ToJson() const {
  llvm::json::Object type_alias{
      {"identifier", identifier},
      {"id", id},
      {"owning_target", owning_target},
      {"unknown_attr", unknown_attr},
      {"doc_comment", doc_comment},
      {"underlying_type", underlying_type},
      {"source_loc", source_loc},
      {"enclosing_item_id", enclosing_item_id},
  };

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
  };

  return llvm::json::Object{
      {"UnsupportedItem", std::move(unsupported)},
  };
}

llvm::json::Value Comment::ToJson() const {
  llvm::json::Object comment{
      {"text", text},
      {"id", id},
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
      {"name", name},
      {"id", id},
      {"canonical_namespace_id", canonical_namespace_id},
      {"unknown_attr", unknown_attr},
      {"owning_target", owning_target},
      {"child_item_ids", std::move(json_item_ids)},
      {"enclosing_item_id", enclosing_item_id},
      {"is_inline", is_inline},
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

}  // namespace crubit
