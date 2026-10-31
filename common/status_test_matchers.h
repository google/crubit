// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_STATUS_TEST_MATCHERS_H_
#define CRUBIT_COMMON_STATUS_TEST_MATCHERS_H_

#include <ostream>
#include <string>
#include <type_traits>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "common/status.h"
#include "common/status_macros.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace detail {

struct StatusInfo {
  bool is_ok;
  int code;
  std::string code_string;
  std::string message;
};

inline StatusInfo GetStatusInfo(const absl::Status& s) {
  return {s.ok(), static_cast<int>(s.code()),
          absl::StatusCodeToString(s.code()), std::string(s.message())};
}

template <typename T>
inline StatusInfo GetStatusInfo(const absl::StatusOr<T>& s) {
  return GetStatusInfo(s.status());
}

inline StatusInfo GetStatusInfo(const llvm::Error& err) {
  StatusInfo info;
  info.is_ok = !const_cast<llvm::Error&>(err);
  if (!info.is_ok) {
    info.code = static_cast<int>(crubit::GetStatusCode(err));
    info.code_string =
        crubit::StatusCodeToString(static_cast<crubit::StatusCode>(info.code))
            .str();
    info.message = crubit::GetStatusMessage(err);
  } else {
    info.code = 0;
    info.code_string = "OK";
  }
  // For llvm::Error/Expected arguments the matcher receives a const&; inspect
  // via the non-consuming helpers, then (via const_cast) consumeError so a
  // temporary unchecked llvm::Error does not abort when destroyed.
  llvm::consumeError(std::move(const_cast<llvm::Error&>(err)));
  return info;
}

template <typename T>
inline StatusInfo GetStatusInfo(const llvm::Expected<T>& expected) {
  StatusInfo info;
  info.is_ok = (bool)const_cast<llvm::Expected<T>&>(expected);
  if (!info.is_ok) {
    auto err = const_cast<llvm::Expected<T>&>(expected).takeError();
    if (!err) {
      // gMock evaluates a failing matcher twice (once to match, once to
      // explain); the first evaluation already consumed the error.
      info.code = static_cast<int>(crubit::StatusCode::kUnknown);
      info.code_string = "<error already consumed by an earlier match>";
      info.message = info.code_string;
      return info;
    }
    info.code = static_cast<int>(crubit::GetStatusCode(err));
    info.code_string =
        crubit::StatusCodeToString(static_cast<crubit::StatusCode>(info.code))
            .str();
    info.message = crubit::GetStatusMessage(err);
    llvm::consumeError(std::move(err));
  } else {
    info.code = 0;
    info.code_string = "OK";
    // consume success to mark it checked
    llvm::consumeError(const_cast<llvm::Expected<T>&>(expected).takeError());
  }
  return info;
}

template <typename StatusOrType>
class IsOkAndHoldsMatcherImpl
    : public ::testing::MatcherInterface<StatusOrType> {
 public:
  typedef typename std::remove_reference<StatusOrType>::type Decayed;
  typedef decltype(*std::declval<Decayed&>()) value_reference;
  typedef typename std::decay<value_reference>::type value_type;

  template <typename InnerMatcher>
  explicit IsOkAndHoldsMatcherImpl(InnerMatcher&& inner_matcher)
      : inner_matcher_(::testing::SafeMatcherCast<const value_type&>(
            std::forward<InnerMatcher>(inner_matcher))) {}

  void DescribeTo(std::ostream* os) const override {
    *os << "is OK and has a value that ";
    inner_matcher_.DescribeTo(os);
  }

  void DescribeNegationTo(std::ostream* os) const override {
    *os << "isn't OK or has a value that ";
    inner_matcher_.DescribeNegationTo(os);
  }

  bool MatchAndExplain(
      StatusOrType actual_value,
      ::testing::MatchResultListener* result_listener) const override {
    auto info = GetStatusInfo(actual_value);
    if (!info.is_ok) {
      *result_listener << "which has status " << info.code_string << ": "
                       << info.message;
      return false;
    }

    ::testing::StringMatchResultListener inner_listener;
    const bool matches =
        inner_matcher_.MatchAndExplain(*actual_value, &inner_listener);
    const std::string inner_explanation = inner_listener.str();
    if (!inner_explanation.empty()) {
      *result_listener << "which contains value "
                       << ::testing::PrintToString(*actual_value) << ", "
                       << inner_explanation;
    }
    return matches;
  }

 private:
  const ::testing::Matcher<const value_type&> inner_matcher_;
};

template <typename InnerMatcher>
class IsOkAndHoldsMatcher {
 public:
  explicit IsOkAndHoldsMatcher(InnerMatcher inner_matcher)
      : inner_matcher_(std::move(inner_matcher)) {}

  template <typename StatusOrType>
  operator ::testing::Matcher<StatusOrType>() const {  // NOLINT
    return ::testing::Matcher<StatusOrType>(
        new IsOkAndHoldsMatcherImpl<const StatusOrType&>(inner_matcher_));
  }

 private:
  const InnerMatcher inner_matcher_;
};

template <typename T>
class MonoIsOkMatcherImpl : public ::testing::MatcherInterface<T> {
 public:
  void DescribeTo(std::ostream* os) const override { *os << "is OK"; }
  void DescribeNegationTo(std::ostream* os) const override {
    *os << "is not OK";
  }
  bool MatchAndExplain(T actual_value,
                       ::testing::MatchResultListener*) const override {
    return GetStatusInfo(actual_value).is_ok;
  }
};

class IsOkMatcher {
 public:
  template <typename T>
  operator ::testing::Matcher<T>() const {  // NOLINT
    return ::testing::Matcher<T>(new MonoIsOkMatcherImpl<T>());
  }
};

class StatusIsMatcher {
 public:
  StatusIsMatcher(int expected_code,
                  testing::Matcher<const std::string&> message_matcher)
      : expected_code_(expected_code), message_matcher_(message_matcher) {}

  void DescribeTo(std::ostream* os) const {
    *os << "has status code that is equal to "
        << StatusCodeToString(static_cast<StatusCode>(expected_code_)).str();
    *os << " and has an error message that ";
    message_matcher_.DescribeTo(os);
  }

  void DescribeNegationTo(std::ostream* os) const {
    *os << "has status code that is not equal to "
        << StatusCodeToString(static_cast<StatusCode>(expected_code_)).str();
    *os << " or has an error message that ";
    message_matcher_.DescribeNegationTo(os);
  }

  template <typename StatusType>
  bool MatchAndExplain(const StatusType& status,
                       testing::MatchResultListener* result_listener) const {
    auto info = GetStatusInfo(status);
    if (info.code != expected_code_) {
      *result_listener
          << "whose canonical status code is not equal to "
          << StatusCodeToString(static_cast<StatusCode>(expected_code_)).str()
          << " (was " << info.code_string << ")";
      return false;
    }
    if (!message_matcher_.Matches(info.message)) {
      *result_listener << "whose error message is wrong (was: \""
                       << info.message << "\")";
      return false;
    }
    return true;
  }

 private:
  const int expected_code_;
  const testing::Matcher<const std::string&> message_matcher_;
};

}  // namespace detail

// Returns a gMock matcher that matches a `StatusOr<>` or `llvm::Expected<>`
// whose status is OK and whose value matches the inner matcher.
template <typename InnerMatcher>
detail::IsOkAndHoldsMatcher<typename std::decay<InnerMatcher>::type>
IsOkAndHolds(InnerMatcher&& inner_matcher) {
  return detail::IsOkAndHoldsMatcher<typename std::decay<InnerMatcher>::type>(
      std::forward<InnerMatcher>(inner_matcher));
}

// Returns a gMock matcher that matches a `Status`, `StatusOr<>`, `llvm::Error`
// or `llvm::Expected<>` which is OK.
inline detail::IsOkMatcher IsOk() { return detail::IsOkMatcher(); }

// Status matcher that checks the status code for an expected value. `code` may
// be an `absl::StatusCode` or a `crubit::StatusCode`.
template <typename Code>
inline testing::PolymorphicMatcher<detail::StatusIsMatcher> StatusIs(
    Code code) {
  return testing::MakePolymorphicMatcher(
      detail::StatusIsMatcher(static_cast<int>(code), testing::_));
}

// Status matcher that checks the status code and message for expected values.
template <typename Code, typename MessageMatcher>
inline testing::PolymorphicMatcher<detail::StatusIsMatcher> StatusIs(
    Code code, const MessageMatcher& message) {
  return testing::MakePolymorphicMatcher(detail::StatusIsMatcher(
      static_cast<int>(code),
      testing::MatcherCast<const std::string&>(message)));
}

#undef EXPECT_OK
#define EXPECT_OK(expression) EXPECT_THAT(expression, ::crubit::IsOk())

#undef ASSERT_OK
#define ASSERT_OK(expression) ASSERT_THAT(expression, ::crubit::IsOk())

#define CRUBIT_TEST_PASTE_INNER_(X, Y) X##Y
#define CRUBIT_TEST_PASTE_(X, Y) CRUBIT_TEST_PASTE_INNER_(X, Y)

#undef ASSERT_OK_AND_ASSIGN
#define ASSERT_OK_AND_ASSIGN(lhs, rexpr)                \
  auto CRUBIT_TEST_PASTE_(_status, __LINE__) = (rexpr); \
  ASSERT_OK(CRUBIT_TEST_PASTE_(_status, __LINE__));     \
  lhs = ::crubit::status_macros_internal::TakeValue(    \
      CRUBIT_TEST_PASTE_(_status, __LINE__))

}  // namespace crubit

#endif  // CRUBIT_COMMON_STATUS_TEST_MATCHERS_H_
