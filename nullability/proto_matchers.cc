// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/proto_matchers.h"

#include <memory>
#include <ostream>
#include <string>

#include "absl/base/nullability.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/protobuf/message.h"
#include "third_party/protobuf/text_format.h"

namespace clang::tidy::nullability {
namespace {

class EqualsProtoMatcher
    : public testing::MatcherInterface<const proto2::Message &> {
  std::string Expected;

 public:
  EqualsProtoMatcher(llvm::StringRef Expected) : Expected(Expected) {}

  bool MatchAndExplain(
      const proto2::Message &M,
      absl::Nonnull<testing::MatchResultListener *> Listener) const override {
    std::unique_ptr<proto2::Message> Parsed(M.New());
    if (!proto2::TextFormat::ParseFromString(Expected, Parsed.get())) {
      *Listener << "where <<<\n"
                << Expected << "\n>>> doesn't parse as " << M.GetTypeName();
      return false;
    }
    // Compare textual representations.
    std::string PrintedExpected, PrintedActual;
    if (!proto2::TextFormat::PrintToString(*Parsed, &PrintedExpected)) {
      *Listener << "where expected message failed to print!";
      return false;
    }
    if (!proto2::TextFormat::PrintToString(M, &PrintedActual)) {
      *Listener << "where actual message failed to print!";
      return false;
    }
    return testing::ExplainMatchResult(PrintedExpected, PrintedActual,
                                       Listener);
  }

  void DescribeTo(absl::Nonnull<std::ostream *> OS) const override {
    *OS << "equals proto <<<\n" << Expected << "\n>>>";
  }
};

}  // namespace

testing::Matcher<const proto2::Message &> EqualsProto(llvm::StringRef Textual) {
  return testing::MakeMatcher(new EqualsProtoMatcher(Textual));
}

}  // namespace clang::tidy::nullability
