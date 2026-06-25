// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bazel/multiple_crate_versions/consumer_v2.h"

#include "absl/strings/str_cat.h"
#include "cc_bindings_from_rs/test/bazel/multiple_crate_versions/v2_test.h"

namespace consumer_v2 {
std::string GetV2String() {
  my_crate::SomeStruct s;
  return absl::StrCat(s);
}
std::string GetV2FreeFunction() {
  return std::string(my_crate::free_function());
}
std::string GetV2Method() {
  my_crate::SomeStruct s;
  return std::string(s.method());
}
std::string GetV2AssocFunction() {
  return std::string(my_crate::SomeStruct::assoc_function());
}
std::string GetV2Clone() {
  my_crate::SomeStruct s;
  my_crate::SomeStruct s2 = s;
  return absl::StrCat(s2);
}
}  // namespace consumer_v2
