// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_ERROR_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_ERROR_H_

#include <string>
#include <system_error>
#include <utility>

#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Error information for errors that originate in the `lifetime_analysis`
// package.
class LifetimeError : public llvm::ErrorInfo<LifetimeError> {
 public:
  enum class Type {
    ElisionNotEnabled,
    CannotElideOutputLifetimes,
    Other,
  };

  LifetimeError(Type type, std::string message)
      : type_(type), message_(std::move(message)) {}

  Type type() const { return type_; }

  void log(llvm::raw_ostream& OS) const override { OS << message_; }

  std::error_code convertToErrorCode() const override {
    return llvm::inconvertibleErrorCode();
  }

  static char ID;

 private:
  Type type_;
  std::string message_;
};

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_ERROR_H_
