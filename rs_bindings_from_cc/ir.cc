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
  result["name"] = std::string(name_);
  return result;
}

nlohmann::json Type::ToJson() const {
  nlohmann::json result;
  result["rs_name"] = std::string(rs_name);
  result["cc_name"] = std::string(cc_name);

  return result;
}

nlohmann::json Identifier::ToJson() const {
  nlohmann::json result;
  result["identifier"] = std::string(identifier_);
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
  result["mangled_name"] = std::string(mangled_name);
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
  std::vector<nlohmann::json> used_headers;
  used_headers.reserve(used_headers_.size());
  for (const HeaderName& header : used_headers_) {
    used_headers.push_back(header.ToJson());
  }

  std::vector<nlohmann::json> functions;
  functions.reserve(functions_.size());
  for (const Func& func : functions_) {
    functions.push_back(func.ToJson());
  }

  std::vector<nlohmann::json> records;
  records.reserve(records_.size());
  for (const Record& record : records_) {
    records.push_back(record.ToJson());
  }

  nlohmann::json result;
  result["used_headers"] = std::move(used_headers);
  result["functions"] = std::move(functions);
  result["records"] = std::move(records);
  return result;
}

}  // namespace rs_bindings_from_cc
