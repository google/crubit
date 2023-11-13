// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/test/headers_for_test.h"

namespace clang::tidy::nullability {

constexpr char kPreamble[] = R"cc(
  enum NullabilityKind {
    NK_nonnull,
    NK_nullable,
    NK_unspecified,
  };

  template <NullabilityKind... NK, typename T>
  void __assert_nullability(const T &);

  template <typename T>
  T value();

  template <typename T>
  using Nullable [[clang::annotate("Nullable")]] = T;

  template <typename T>
  using Nonnull [[clang::annotate("Nonnull")]] = T;

  template <typename T>
  using NullabilityUnknown [[clang::annotate("Nullability_Unspecified")]] = T;
)cc";

constexpr char kNewHeader[] = R"cc(
  namespace std {
  struct nothrow_t {
    explicit nothrow_t() = default;
  };
  extern const nothrow_t nothrow;
  using size_t = decltype(sizeof(int));
  }  // namespace std
  void *operator new(std::size_t size, const std::nothrow_t &) noexcept;
)cc";

tooling::FileContentMappings headersForTest() {
  return {{"preamble.h", kPreamble}, {"new", kNewHeader}};
}

llvm::StringMap<std::string> headersForTestAsStringMap() {
  llvm::StringMap<std::string> Result;
  for (const auto &[Name, Content] : headersForTest()) {
    Result.insert_or_assign(Name, Content);
  }
  return Result;
}

}  // namespace clang::tidy::nullability
