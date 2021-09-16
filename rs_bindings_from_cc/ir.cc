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

nlohmann::json Type::ToJson() const {
  nlohmann::json result;

  std::vector<nlohmann::json> json_params;
  json_params.reserve(type_params.size());
  for (const Type& param : type_params) {
    json_params.push_back(param.ToJson());
  }
  result["rs_name"] = rs_name;
  result["cc_name"] = cc_name;
  result["type_params"] = std::move(json_params);

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

nlohmann::json Field::ToJson() const {
  nlohmann::json result;
  result["type"] = type.ToJson();
  result["identifier"] = identifier.ToJson();
  return result;
}

nlohmann::json Record::ToJson() const {
  std::vector<nlohmann::json> fields;
  fields.reserve(fields_.size());
  for (const Field& field : fields_) {
    fields.push_back(field.ToJson());
  }

  nlohmann::json result;
  result["identifier"] = identifier_.ToJson();
  result["fields"] = std::move(fields);
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
