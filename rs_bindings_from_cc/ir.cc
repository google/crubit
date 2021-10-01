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
  nlohmann::json result;
  result["identifier"] = identifier.ToJson();
  result["mangled_name"] = mangled_name;
  result["return_type"] = return_type.ToJson();
  result["params"] = std::move(json_params);
  result["is_inline"] = is_inline;
  return result;
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

nlohmann::json Field::ToJson() const {
  nlohmann::json result;
  result["type"] = type.ToJson();
  result["identifier"] = identifier.ToJson();
  result["access"] = AccessToString(access);
  result["offset"] = offset;
  return result;
}

nlohmann::json Record::ToJson() const {
  std::vector<nlohmann::json> json_fields;
  json_fields.reserve(fields.size());
  for (const Field& field : fields) {
    json_fields.push_back(field.ToJson());
  }

  nlohmann::json result;
  result["identifier"] = identifier.ToJson();
  result["fields"] = std::move(json_fields);
  result["size"] = size;
  result["alignment"] = alignment;
  result["is_trivial_abi"] = is_trivial_abi;
  return result;
}

nlohmann::json IR::ToJson() const {
  std::vector<nlohmann::json> json_used_headers;
  json_used_headers.reserve(used_headers.size());
  for (const HeaderName& header : used_headers) {
    json_used_headers.push_back(header.ToJson());
  }

  std::vector<nlohmann::json> json_functions;
  json_functions.reserve(functions.size());
  for (const Func& func : functions) {
    json_functions.push_back(func.ToJson());
  }

  std::vector<nlohmann::json> json_records;
  json_records.reserve(records.size());
  for (const Record& record : records) {
    json_records.push_back(record.ToJson());
  }

  nlohmann::json result;
  result["used_headers"] = std::move(json_used_headers);
  result["functions"] = std::move(json_functions);
  result["records"] = std::move(json_records);
  return result;
}

}  // namespace rs_bindings_from_cc
