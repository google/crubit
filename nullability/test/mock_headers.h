// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file defines mock headers for use in nullability tests.

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_TEST_MOCK_HEADERS_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_TEST_MOCK_HEADERS_H_

#include <utility>

#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability::test {

llvm::ArrayRef<std::pair<llvm::StringRef, llvm::StringRef>> getMockHeaders();

}  // namespace clang::tidy::nullability::test

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_TEST_MOCK_HEADERS_H_
