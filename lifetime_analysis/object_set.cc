// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/object_set.h"

#include <string>
#include <vector>

#include "absl/strings/str_join.h"
#include "lifetime_analysis/object.h"

namespace clang {
namespace tidy {
namespace lifetimes {

std::string ObjectSet::DebugString() const {
  std::vector<std::string> parts;
  for (const Object* object : objects_) {
    parts.push_back(object->DebugString());
  }
  return absl::StrJoin(parts, ", ");
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
