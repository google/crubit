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

namespace crubit {

namespace detail {
inline const ::absl::Status& GetStatus(const ::absl::Status& status) {
  return status;
}

template <typename T>
inline const ::absl::Status& GetStatus(const ::absl::StatusOr<T>& status) {
  return status.status();
}

// Monomorphic implementation of matcher IsOkAndHolds(m).  StatusOrType is a
// reference to StatusOr<T>.
template <typename StatusOrType>
class IsOkAndHoldsMatcherImpl
    : public ::testing::MatcherInterface<StatusOrType> {
 public:
  typedef
      typename std::remove_reference<StatusOrType>::type::value_type value_type;

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
    if (!actual_value.ok()) {
      *result_listener << "which has status " << actual_value.status();
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

// Implements IsOkAndHolds(m) as a polymorphic matcher.
template <typename InnerMatcher>
class IsOkAndHoldsMatcher {
 public:
  explicit IsOkAndHoldsMatcher(InnerMatcher inner_matcher)
      : inner_matcher_(std::move(inner_matcher)) {}

  // Converts this polymorphic matcher to a monomorphic matcher of the
  // given type.  StatusOrType can be either StatusOr<T> or a
  // reference to StatusOr<T>.
  template <typename StatusOrType>
  operator ::testing::Matcher<StatusOrType>() const {  // NOLINT
    return ::testing::Matcher<StatusOrType>(
        new IsOkAndHoldsMatcherImpl<const StatusOrType&>(inner_matcher_));
  }

 private:
  const InnerMatcher inner_matcher_;
};

// Monomorphic implementation of matcher IsOk() for a given type T.
// T can be Status, StatusOr<>, or a reference to either of them.
template <typename T>
class MonoIsOkMatcherImpl : public ::testing::MatcherInterface<T> {
 public:
  void DescribeTo(std::ostream* os) const override { *os << "is OK"; }
  void DescribeNegationTo(std::ostream* os) const override {
    *os << "is not OK";
  }
  bool MatchAndExplain(T actual_value,
                       ::testing::MatchResultListener*) const override {
    return GetStatus(actual_value).ok();
  }
};

// Implements IsOk() as a polymorphic matcher.
class IsOkMatcher {
 public:
  template <typename T>
  operator ::testing::Matcher<T>() const {  // NOLINT
    return ::testing::Matcher<T>(new MonoIsOkMatcherImpl<T>());
  }
};

class StatusIsMatcher {
 public:
  StatusIsMatcher(absl::StatusCode expected_code,
                  testing::Matcher<const std::string&> message_matcher)
      : expected_code_(expected_code), message_matcher_(message_matcher) {}

  void DescribeTo(std::ostream* os) const {
    *os << "has status code that is equal to "
        << absl::StatusCodeToString(expected_code_);
    *os << " and has an error message that ";
    message_matcher_.DescribeTo(os);
  }

  void DescribeNegationTo(std::ostream* os) const {
    *os << "has status code that is not equal to "
        << absl::StatusCodeToString(expected_code_);
    *os << " or has an error message that ";
    message_matcher_.DescribeNegationTo(os);
  }

  template <typename StatusType>
  bool MatchAndExplain(const StatusType& status,
                       testing::MatchResultListener* result_listener) const {
    if (GetStatus(status).code() != expected_code_) {
      *result_listener << "whose canonical status code is not equal to"
                       << absl::StatusCodeToString(expected_code_);
      return false;
    }
    if (!message_matcher_.Matches(std::string(GetStatus(status).message()))) {
      *result_listener << "whose error message is wrong";
      return false;
    }
    return true;
  }

 private:
  const absl::StatusCode expected_code_;
  const testing::Matcher<const std::string&> message_matcher_;
};

}  // namespace detail

// Returns a gMock matcher that matches a StatusOr<> whose status is
// OK and whose value matches the inner matcher.
template <typename InnerMatcher>
detail::IsOkAndHoldsMatcher<typename std::decay<InnerMatcher>::type>
IsOkAndHolds(InnerMatcher&& inner_matcher) {
  return detail::IsOkAndHoldsMatcher<typename std::decay<InnerMatcher>::type>(
      std::forward<InnerMatcher>(inner_matcher));
}

// Returns a gMock matcher that matches a Status or StatusOr<> which is OK.
inline detail::IsOkMatcher IsOk() { return detail::IsOkMatcher(); }

// Status matcher that checks the StatusCode for an expected value.
inline testing::PolymorphicMatcher<detail::StatusIsMatcher> StatusIs(
    const absl::StatusCode& code) {
  return testing::MakePolymorphicMatcher(
      detail::StatusIsMatcher(code, testing::_));
}

// Status matcher that checks the StatusCode and message for expected values.
template <typename MessageMatcher>
testing::PolymorphicMatcher<detail::StatusIsMatcher> StatusIs(
    const absl::StatusCode& code, const MessageMatcher& message) {
  return testing::MakePolymorphicMatcher(detail::StatusIsMatcher(
      code, testing::MatcherCast<const std::string&>(message)));
}

#define ASSERT_OK(expression) ASSERT_THAT(expression, ::crubit::IsOk())

#define PASTE_INNER(X, Y) X##Y
#define PASTE(X, Y) PASTE_INNER(X, Y)

#define ASSERT_OK_AND_ASSIGN(lhs, rexpr) \
  auto PASTE(_status, __LINE__) = rexpr; \
  ASSERT_OK(PASTE(_status, __LINE__));   \
  lhs = std::move(PASTE(_status, __LINE__)).value();
}  // namespace crubit

#endif  // CRUBIT_COMMON_STATUS_TEST_MATCHERS_H_
