// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_STATUS_TEST_MATCHERS_H_
#define CRUBIT_COMMON_STATUS_TEST_MATCHERS_H_

#include <string>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "common/status.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace detail {

struct StatusInfo {
  StatusCode code = StatusCode::kOk;
  std::string message;

  bool is_ok() const { return code == StatusCode::kOk; }
  std::string code_string() const { return StatusCodeToString(code).str(); }
};

inline StatusInfo GetStatusInfo(const llvm::Error& err) {
  // Matchers receive `const&`; inspect via the non-consuming helpers, then
  // consume the error so temporary `llvm::Error` values do not abort on
  // destruction.
  StatusCode code = GetStatusCode(err);
  std::string message = code == StatusCode::kOk ? "" : GetStatusMessage(err);
  llvm::consumeError(std::move(const_cast<llvm::Error&>(err)));
  return {code, std::move(message)};
}

template <typename T>
inline StatusInfo GetStatusInfo(const llvm::Expected<T>& expected) {
  auto& mut = const_cast<llvm::Expected<T>&>(expected);
  if (mut) return {};
  llvm::Error err = mut.takeError();
  if (!err) {
    // gMock evaluates a failing matcher twice (once to match, once to explain);
    // the first evaluation already consumed the error.
    return {StatusCode::kUnknown,
            "<error already consumed by an earlier match>"};
  }
  return GetStatusInfo(err);
}

}  // namespace detail

// Matches a `Status`, `StatusOr<>`, `llvm::Error`, or `llvm::Expected<>` whose
// status is OK.
MATCHER(IsOk, negation ? "is not OK" : "is OK") {
  detail::StatusInfo info = detail::GetStatusInfo(arg);
  if (!info.is_ok()) {
    *result_listener << "whose status is " << info.code_string() << ": "
                     << info.message;
    return false;
  }
  return true;
}

// Matches a `StatusOr<>` or `llvm::Expected<>` whose status is OK and whose
// value matches `inner_matcher`.
MATCHER_P(IsOkAndHolds, inner_matcher,
          (negation ? "isn't OK or has a value that "
                    : "is OK and has a value that ") +
              ::testing::PrintToString(inner_matcher)) {
  detail::StatusInfo info = detail::GetStatusInfo(arg);
  if (!info.is_ok()) {
    *result_listener << "which has status " << info.code_string() << ": "
                     << info.message;
    return false;
  }
  return ::testing::ExplainMatchResult(inner_matcher, *arg, result_listener);
}

// Matches a status whose code equals `expected_code` (`crubit::StatusCode`) and
// whose message matches `message_matcher`.
MATCHER_P2(
    StatusIs, expected_code, message_matcher,
    std::string(negation ? "does not have status code " : "has status code ") +
        StatusCodeToString(static_cast<StatusCode>(expected_code)).str() +
        " and a message that " +
        ::testing::DescribeMatcher<const std::string&>(message_matcher,
                                                       false)) {
  detail::StatusInfo info = detail::GetStatusInfo(arg);
  const auto code = static_cast<StatusCode>(expected_code);
  if (info.code != code) {
    *result_listener << "whose canonical status code is not equal to "
                     << StatusCodeToString(code).str() << " (was "
                     << info.code_string() << ")";
    return false;
  }
  if (!::testing::MatcherCast<const std::string&>(message_matcher)
           .Matches(info.message)) {
    *result_listener << "whose error message is wrong (was: \"" << info.message
                     << "\")";
    return false;
  }
  return true;
}

template <typename Code>
auto StatusIs(Code expected_code) {
  return StatusIs(expected_code, ::testing::_);
}

#undef EXPECT_OK
#define EXPECT_OK(expression) EXPECT_THAT(expression, ::crubit::IsOk())

#undef ASSERT_OK
#define ASSERT_OK(expression) ASSERT_THAT(expression, ::crubit::IsOk())

#define CRUBIT_TEST_PASTE_INNER_(X, Y) X##Y
#define CRUBIT_TEST_PASTE_(X, Y) CRUBIT_TEST_PASTE_INNER_(X, Y)

#define CRUBIT_ASSERT_OK_AND_ASSIGN_IMPL_(val, lhs, rexpr) \
  auto val = (rexpr);                                      \
  ASSERT_OK(val);                                          \
  lhs = std::move(*val)

#undef ASSERT_OK_AND_ASSIGN
#define ASSERT_OK_AND_ASSIGN(lhs, rexpr)                                   \
  CRUBIT_ASSERT_OK_AND_ASSIGN_IMPL_(CRUBIT_TEST_PASTE_(_status, __LINE__), \
                                    lhs, rexpr)

}  // namespace crubit

#endif  // CRUBIT_COMMON_STATUS_TEST_MATCHERS_H_
