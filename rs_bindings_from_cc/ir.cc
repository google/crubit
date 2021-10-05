// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

#include <string>
#include <vector>

#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

nlohmann::json HeaderName::ToJson() const {
  nlohmann::json result;
  result["name"] = name_;
  return result;
}

nlohmann::json RsType::ToJson() const {
  nlohmann::json result;

  std::vector<nlohmann::json> json_params;
  json_params.reserve(type_params.size());
  for (const RsType& param : type_params) {
    json_params.push_back(param.ToJson());
  }
  result["name"] = name;
  result["type_params"] = std::move(json_params);

  return result;
}

nlohmann::json CcType::ToJson() const {
  nlohmann::json result;

  std::vector<nlohmann::json> json_params;
  json_params.reserve(type_params.size());
  for (const CcType& param : type_params) {
    json_params.push_back(param.ToJson());
  }
  result["name"] = name;
  result["is_const"] = is_const;
  result["type_params"] = std::move(json_params);

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

nlohmann::json Func::ToJson() const {
  std::vector<nlohmann::json> json_params;
  json_params.reserve(params.size());
  for (const FuncParam& param : params) {
    json_params.push_back(param.ToJson());
  }
  nlohmann::json func;
  func["identifier"] = identifier.ToJson();
  func["mangled_name"] = mangled_name;
  func["return_type"] = return_type.ToJson();
  func["params"] = std::move(json_params);
  func["is_inline"] = is_inline;

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
  result["type"] = type.ToJson();
  result["identifier"] = identifier.ToJson();
  result["access"] = AccessToString(access);
  result["offset"] = offset;
  return result;
}

static std::string SpecialMemberDefinitionToString(
    SpecialMemberFunc::Definition def) {
  switch (def) {
    case SpecialMemberFunc::Definition::kTrivial:
      return "Trivial";
    case SpecialMemberFunc::Definition::kNontrivial:
      return "Nontrivial";
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
  std::vector<nlohmann::json> json_fields;
  json_fields.reserve(fields.size());
  for (const Field& field : fields) {
    json_fields.push_back(field.ToJson());
  }

  nlohmann::json record;
  record["identifier"] = identifier.ToJson();
  if (doc_comment) {
    record["doc_comment"] = *doc_comment;
  }
  record["fields"] = std::move(json_fields);
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

nlohmann::json IR::ToJson() const {
  std::vector<nlohmann::json> json_used_headers;
  json_used_headers.reserve(used_headers.size());
  for (const HeaderName& header : used_headers) {
    json_used_headers.push_back(header.ToJson());
  }

  std::vector<nlohmann::json> json_items;
  json_items.reserve(items.size());
  for (const auto& item : items) {
    std::visit([&](auto&& item) { json_items.push_back(item.ToJson()); }, item);
  }

  nlohmann::json result;
  result["used_headers"] = std::move(json_used_headers);
  result["items"] = std::move(json_items);
  return result;
}

}  // namespace rs_bindings_from_cc
