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
  result["rs_name"] = std::string(rs_name_);
  result["cc_name"] = std::string(cc_name_);

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

nlohmann::json IR::ToJson() const {
  std::vector<nlohmann::json> used_headers;
  for (const HeaderName& header : used_headers_) {
    used_headers.push_back(header.ToJson());
  }

  std::vector<nlohmann::json> functions;
  for (const Func& func : functions_) {
    functions.push_back(func.ToJson());
  }

  nlohmann::json result;
  result["used_headers"] = used_headers;
  result["functions"] = functions;
  return result;
}

}  // namespace rs_bindings_from_cc
