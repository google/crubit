// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/proto_matchers.h"

#include <memory>
#include <new>
#include <ostream>
#include <string>

#include "absl/base/nullability.h"
#include "llvm/include/llvm/ADT/StringRef.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"
#include "google/protobuf/message.h"
#include "google/protobuf/text_format.h"

namespace clang::tidy::nullability {
namespace {

class EqualsProtoMatcher
    : public testing::MatcherInterface<const google::protobuf::Message &> {
  std::string Expected;

 public:
  EqualsProtoMatcher(llvm::StringRef Expected) : Expected(Expected) {}

  EqualsProtoMatcher(const google::protobuf::Message &ExpectedProto) {
    if (!google::protobuf::TextFormat::PrintToString(ExpectedProto, &Expected)) {
      Expected = "Failed to print expected proto!";
    }
  }

  bool MatchAndExplain(
      const google::protobuf::Message &M,
      testing::MatchResultListener *absl_nonnull Listener) const override {
    std::unique_ptr<google::protobuf::Message> Parsed(M.New());
    if (!google::protobuf::TextFormat::ParseFromString(Expected, Parsed.get())) {
      *Listener << "where <<<\n"
                << Expected << "\n>>> doesn't parse as " << M.GetTypeName();
      return false;
    }
    // Compare textual representations.
    std::string PrintedExpected, PrintedActual;
    if (!google::protobuf::TextFormat::PrintToString(*Parsed, &PrintedExpected)) {
      *Listener << "where expected message failed to print!";
      return false;
    }
    if (!google::protobuf::TextFormat::PrintToString(M, &PrintedActual)) {
      *Listener << "where actual message failed to print!";
      return false;
    }
    return testing::ExplainMatchResult(PrintedExpected, PrintedActual,
                                       Listener);
  }

  void DescribeTo(std::ostream *absl_nonnull OS) const override {
    *OS << "equals proto <<<\n" << Expected << "\n>>>";
  }
};

}  // namespace

testing::Matcher<const google::protobuf::Message &> EqualsProto(llvm::StringRef Textual) {
  return testing::MakeMatcher(new EqualsProtoMatcher(Textual));
}

testing::Matcher<const google::protobuf::Message &> EqualsProto(
    const google::protobuf::Message &Expected) {
  return testing::MakeMatcher(new EqualsProtoMatcher(Expected));
}

}  // namespace clang::tidy::nullability
