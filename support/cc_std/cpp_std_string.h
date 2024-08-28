// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_CPP_STD_STRING_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_CPP_STD_STRING_H_

#include <cstddef>
#include <memory>
#include <string>
#include <string_view>
#include <utility>

/// A Rust-compatible wrapper around `std::string`, which should be used only
/// for interop with Rust.
class StdString final {
 public:
  explicit StdString(std::string value)
      : value_(std::make_unique<std::string>(std::move(value))) {}
  std::string& value() { return *value_; }
  const std::string& value() const { return *value_; }
  const char* data() const { return value_->data(); }
  size_t size() const { return value_->size(); }

  static inline StdString FromStringView(std::string_view value) {
    return StdString(std::string(value));
  }

 private:
  std::unique_ptr<std::string> value_;
};

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_CPP_STD_STRING_H_
