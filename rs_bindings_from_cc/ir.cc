// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

#include <stdint.h>

#include <optional>
#include <ostream>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/strings/string_view.h"
#include "common/check.h"
#include "common/strong_int.h"
#include "rs_bindings_from_cc/bazel_types.h"
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

llvm::json::Value RsType::ToJson() const {
  return llvm::json::Object{
      {"name", decl_id.has_value() ? llvm::json::Value(nullptr)
                                   : llvm::json::Value(name)},
      {"lifetime_args", lifetime_args},
      {"type_args", type_args},
      {"decl_id", decl_id},
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

MappedType PointerOrReferenceTo(MappedType pointee_type,
                                absl::string_view cc_ptr_name,
                                ValueCategory value_category,
                                std::optional<LifetimeId> lifetime,
                                bool nullable) {
  bool has_lifetime = lifetime.has_value();
  absl::string_view rs_name;
  if (value_category == ValueCategory::kLvalue) {
    if (has_lifetime) {
      rs_name = pointee_type.cc_type.is_const ? internal::kRustRefConst
                                              : internal::kRustRefMut;
    } else {
      rs_name = pointee_type.cc_type.is_const ? internal::kRustPtrConst
                                              : internal::kRustPtrMut;
    }
  } else {
    CRUBIT_CHECK(has_lifetime);
    rs_name = pointee_type.cc_type.is_const ? internal::kRustRvalueRefConst
                                            : internal::kRustRvalueRefMut;
  }
  auto pointer_type =
      MappedType::Simple(std::string(rs_name), std::string(cc_ptr_name));
  if (has_lifetime) {
    pointer_type.rs_type.lifetime_args.push_back(*std::move(lifetime));
  }
  pointer_type.rs_type.type_args.push_back(std::move(pointee_type.rs_type));
  if (has_lifetime && nullable) {
    pointer_type.rs_type =
        RsType{.name = "Option", .type_args = {pointer_type.rs_type}};
  }
  pointer_type.cc_type.type_args.push_back(std::move(pointee_type.cc_type));
  return pointer_type;
}
}  // namespace

MappedType MappedType::PointerTo(MappedType pointee_type,
                                 std::optional<LifetimeId> lifetime,
                                 bool nullable) {
  return PointerOrReferenceTo(std::move(pointee_type), internal::kCcPtr,
                              ValueCategory::kLvalue, lifetime, nullable);
}

MappedType MappedType::LValueReferenceTo(MappedType pointee_type,
                                         std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type), internal::kCcLValueRef,
                              ValueCategory::kLvalue, lifetime,
                              /*nullable=*/false);
}

MappedType MappedType::RValueReferenceTo(MappedType pointee_type,
                                         LifetimeId lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type), internal::kCcRValueRef,
                              ValueCategory::kRvalue, lifetime,
                              /*nullable=*/false);
}

MappedType MappedType::FuncPtr(absl::string_view cc_call_conv,
                               absl::string_view rs_abi,
                               std::optional<LifetimeId> lifetime,
                               MappedType return_type,
                               std::vector<MappedType> param_types) {
  MappedType result = FuncRef(cc_call_conv, rs_abi, lifetime,
                              std::move(return_type), std::move(param_types));

  CRUBIT_CHECK(result.cc_type.name == internal::kCcLValueRef);
  result.cc_type.name = std::string(internal::kCcPtr);

  RsType rs_func_ptr_type = std::move(result.rs_type);
  CRUBIT_CHECK(
      rs_func_ptr_type.name.substr(0, internal::kRustFuncPtr.length()) ==
      internal::kRustFuncPtr);
  result.rs_type =
      RsType{.name = "Option", .type_args = {std::move(rs_func_ptr_type)}};

  return result;
}

MappedType MappedType::FuncRef(absl::string_view cc_call_conv,
                               absl::string_view rs_abi,
                               std::optional<LifetimeId> lifetime,
                               MappedType return_type,
                               std::vector<MappedType> param_types) {
  std::vector<MappedType> type_args = std::move(param_types);
  type_args.push_back(std::move(return_type));

  std::vector<CcType> cc_type_args;
  std::vector<RsType> rs_type_args;
  cc_type_args.reserve(type_args.size());
  rs_type_args.reserve(type_args.size());
  for (MappedType& type_arg : type_args) {
    cc_type_args.push_back(std::move(type_arg.cc_type));
    rs_type_args.push_back(std::move(type_arg.rs_type));
  }

  CcType cc_func_value_type = CcType{
      .name = absl::StrCat(internal::kCcFuncValue, " ", cc_call_conv),
      .type_args = std::move(cc_type_args),
  };
  CcType cc_func_ref_type = CcType{.name = std::string(internal::kCcLValueRef),
                                   .type_args = {cc_func_value_type}};

  // Rust cannot express a function *value* type, only function pointer types.
  RsType rs_func_ptr_type = RsType{
      .name = absl::StrCat(internal::kRustFuncPtr, " ", rs_abi),
      .type_args = std::move(rs_type_args),
  };
  if (lifetime.has_value())
    rs_func_ptr_type.lifetime_args.push_back(*std::move(lifetime));

  return MappedType{
      .rs_type = std::move(rs_func_ptr_type),
      .cc_type = std::move(cc_func_ref_type),
  };
}

llvm::json::Value MappedType::ToJson() const {
  return llvm::json::Object{
      {"rs_type", rs_type},
      {"cc_type", cc_type},
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
      {"is_explicit_ctor", is_explicit_ctor},
  };
}

llvm::json::Value MemberFuncMetadata::ToJson() const {
  return llvm::json::Object{
      {"record_id", record_id},
      {"instance_method_metadata", instance_method_metadata},
  };
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
      {"has_c_calling_convention", has_c_calling_convention},
      {"is_member_or_descendant_of_class_template",
       is_member_or_descendant_of_class_template},
      {"source_loc", source_loc},
      {"id", id},
      {"enclosing_namespace_id", enclosing_namespace_id},
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
  };

  return llvm::json::Object{
      {"IncompleteRecord", std::move(record)},
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
      {"id", id},
      {"owning_target", owning_target},
      {"doc_comment", doc_comment},
      {"unambiguous_public_bases", unambiguous_public_bases},
      {"fields", fields},
      {"lifetime_params", lifetime_params},
      {"size", size},
      {"alignment", alignment},
      {"is_derived_class", is_derived_class},
      {"override_alignment", override_alignment},
      {"copy_constructor", copy_constructor},
      {"move_constructor", move_constructor},
      {"destructor", destructor},
      {"is_trivial_abi", is_trivial_abi},
      {"is_inheritable", is_inheritable},
      {"is_abstract", is_abstract},
      {"record_type", RecordTypeToString(record_type)},
      {"is_aggregate", is_aggregate},
      {"is_anon_record_with_typedef", is_anon_record_with_typedef},
      {"child_item_ids", std::move(json_item_ids)},
      {"enclosing_namespace_id", enclosing_namespace_id},
  };

  return llvm::json::Object{
      {"Record", std::move(record)},
  };
}

llvm::json::Value Enumerator::ToJson() const {
  return llvm::json::Object{
      {"identifier", identifier},
      {"value", value},
  };
}

llvm::json::Value Enum::ToJson() const {
  llvm::json::Object enum_ir{
      {"identifier", identifier},
      {"id", id},
      {"owning_target", owning_target},
      {"underlying_type", underlying_type},
      {"enumerators", enumerators},
      {"enclosing_namespace_id", enclosing_namespace_id},
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
      {"doc_comment", doc_comment},
      {"underlying_type", underlying_type},
      {"enclosing_namespace_id", enclosing_namespace_id},
  };

  return llvm::json::Object{
      {"TypeAlias", std::move(type_alias)},
  };
}

llvm::json::Value SourceLoc::ToJson() const {
  return llvm::json::Object{
      {"filename", filename},
      {"line", line},
      {"column", column},
  };
}

llvm::json::Value UnsupportedItem::ToJson() const {
  llvm::json::Object unsupported{
      {"name", name},
      {"message", message},
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
      {"owning_target", owning_target},
      {"child_item_ids", std::move(json_item_ids)},
      {"enclosing_namespace_id", enclosing_namespace_id},
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

  std::vector<llvm::json::Value> top_level_ids;
  top_level_ids.reserve(top_level_item_ids.size());
  for (const auto& id : top_level_item_ids) {
    top_level_ids.push_back(id.value());
  }

  return llvm::json::Object{
      {"used_headers", used_headers},
      {"current_target", current_target},
      {"items", std::move(json_items)},
      {"top_level_item_ids", std::move(top_level_ids)},
  };
}

}  // namespace crubit
