// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_TEST_HEADERS_FOR_TEST_H_
#define CRUBIT_NULLABILITY_TEST_HEADERS_FOR_TEST_H_

#include "clang/Tooling/Tooling.h"

namespace clang::tidy::nullability {

/// Returns headers containing mock definitions to be used in tests.
/// This includes various standard library headers as well as a header
/// `preamble.h` containing test-specific definitions.
tooling::FileContentMappings headersForTest();

/// See `headersForTest()`, but the result is returned as a `StringMap`.
llvm::StringMap<std::string> headersForTestAsStringMap();

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_TEST_HEADERS_FOR_TEST_H_
