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

#include "base/integral_types.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/json/src/json.hpp"
#include "util/gtl/labs/string_type.h"
#include "util/intops/strong_int.h"

namespace rs_bindings_from_cc {

template <class T>
static std::vector<nlohmann::json> VectorToJson(const std::vector<T>& v) {
  std::vector<nlohmann::json> result;
  result.reserve(v.size());
  for (const T& t : v) {
    result.push_back(t.ToJson());
  }
  return result;
}

nlohmann::json HeaderName::ToJson() const {
  nlohmann::json result;
  result["name"] = name_;
  return result;
}

nlohmann::json RsType::ToJson() const {
  nlohmann::json result;

  if (decl_id.has_value()) {
    result["decl_id"] = decl_id->value();
  } else {
    result["name"] = name;
  }
  result["type_params"] = VectorToJson(type_params);

  return result;
}

nlohmann::json CcType::ToJson() const {
  nlohmann::json result;

  if (decl_id.has_value()) {
    result["decl_id"] = decl_id->value();
  } else {
    result["name"] = name;
  }
  result["is_const"] = is_const;
  result["type_params"] = VectorToJson(type_params);

  return result;
}

nlohmann::json MappedType::ToJson() const {
  nlohmann::json result;

  result["rs_type"] = rs_type.ToJson();
  result["cc_type"] = cc_type.ToJson();

  return result;
}

nlohmann::json Identifier::ToJson() const {
  nlohmann::json result;
  result["identifier"] = identifier_;
  return result;
}

nlohmann::json FuncParam::ToJson() const {
  nlohmann::json result;
  result["type"] = type.ToJson();
  result["identifier"] = identifier.ToJson();
  return result;
}

static std::string SpecialNameToString(SpecialName special_name) {
  switch (special_name) {
    case SpecialName::kDestructor:
      return "Destructor";
    case SpecialName::kConstructor:
      return "Constructor";
  }
}

std::ostream& operator<<(std::ostream& o, const SpecialName& special_name) {
  return o << SpecialNameToString(special_name);
}

nlohmann::json MemberFuncMetadata::ToJson() const {
  nlohmann::json meta;

  meta["for_type"] = for_type.ToJson();

  if (instance_method_metadata.has_value()) {
    nlohmann::json instance;

    absl::string_view reference;
    switch (instance_method_metadata->reference) {
      case MemberFuncMetadata::kLValue:
        reference = "LValue";
        break;
      case MemberFuncMetadata::kRValue:
        reference = "RValue";
        break;
      case MemberFuncMetadata::kUnqualified:
        reference = "Unqualified";
        break;
    }
    instance["reference"] = reference;
    instance["is_const"] = instance_method_metadata->is_const;
    instance["is_virtual"] = instance_method_metadata->is_virtual;

    meta["instance_method_metadata"] = std::move(instance);
  }

  return meta;
}

nlohmann::json Func::ToJson() const {
  nlohmann::json func;
  if (auto* id = std::get_if<Identifier>(&name)) {
    func["name"]["Identifier"] = id->ToJson();
  } else {
    func["name"][SpecialNameToString(std::get<SpecialName>(name))] = nullptr;
  }
  func["decl_id"] = decl_id.value();
  func["owning_target"] = owning_target.value();
  if (doc_comment) {
    func["doc_comment"] = *doc_comment;
  }
  func["mangled_name"] = mangled_name;
  func["return_type"] = return_type.ToJson();
  func["params"] = VectorToJson(params);
  func["is_inline"] = is_inline;
  if (member_func_metadata.has_value()) {
    func["member_func_metadata"] = member_func_metadata->ToJson();
  }

  nlohmann::json item;
  item["Func"] = std::move(func);
  return item;
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

nlohmann::json Field::ToJson() const {
  nlohmann::json result;

  result["identifier"] = identifier.ToJson();
  if (doc_comment) {
    result["doc_comment"] = *doc_comment;
  }
  result["type"] = type.ToJson();
  result["access"] = AccessToString(access);
  result["offset"] = offset;
  return result;
}

static std::string SpecialMemberDefinitionToString(
    SpecialMemberFunc::Definition def) {
  switch (def) {
    case SpecialMemberFunc::Definition::kTrivial:
      return "Trivial";
    case SpecialMemberFunc::Definition::kNontrivialMembers:
      return "NontrivialMembers";
    case SpecialMemberFunc::Definition::kNontrivialSelf:
      return "NontrivialSelf";
    case SpecialMemberFunc::Definition::kDeleted:
      return "Deleted";
  }
}

std::ostream& operator<<(std::ostream& o,
                         const SpecialMemberFunc::Definition& definition) {
  return o << SpecialMemberDefinitionToString(definition);
}

nlohmann::json SpecialMemberFunc::ToJson() const {
  nlohmann::json result;
  result["definition"] = SpecialMemberDefinitionToString(definition);
  result["access"] = AccessToString(access);
  return result;
}

nlohmann::json Record::ToJson() const {
  nlohmann::json record;
  record["identifier"] = identifier.ToJson();
  record["decl_id"] = decl_id.value();
  record["owning_target"] = owning_target.value();
  if (doc_comment) {
    record["doc_comment"] = *doc_comment;
  }
  record["fields"] = VectorToJson(fields);
  record["size"] = size;
  record["alignment"] = alignment;
  record["copy_constructor"] = copy_constructor.ToJson();
  record["move_constructor"] = move_constructor.ToJson();
  record["destructor"] = destructor.ToJson();
  record["is_trivial_abi"] = is_trivial_abi;

  nlohmann::json item;
  item["Record"] = std::move(record);
  return item;
}
nlohmann::json SourceLoc::ToJson() const {
  nlohmann::json source_loc;
  source_loc["filename"] = filename;
  source_loc["line"] = line;
  source_loc["column"] = column;
  return source_loc;
}

nlohmann::json UnsupportedItem::ToJson() const {
  nlohmann::json unsupported;
  unsupported["name"] = name;
  unsupported["message"] = message;
  unsupported["source_loc"] = source_loc.ToJson();

  nlohmann::json item;
  item["UnsupportedItem"] = std::move(unsupported);
  return item;
}

nlohmann::json Comment::ToJson() const {
  nlohmann::json comment;
  comment["text"] = text;

  nlohmann::json item;
  item["Comment"] = std::move(comment);
  return item;
}

nlohmann::json IR::ToJson() const {
  std::vector<nlohmann::json> json_items;
  json_items.reserve(items.size());
  for (const auto& item : items) {
    std::visit([&](auto&& item) { json_items.push_back(item.ToJson()); }, item);
  }

  nlohmann::json result;
  result["used_headers"] = VectorToJson(used_headers);
  result["current_target"] = current_target.value();
  result["items"] = std::move(json_items);
  return result;
}

}  // namespace rs_bindings_from_cc
