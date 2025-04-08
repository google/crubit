// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_PROTO_MATCHERS_H_
#define CRUBIT_NULLABILITY_PROTO_MATCHERS_H_

#include "llvm/include/llvm/ADT/StringRef.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "google/protobuf/message.h"

namespace clang::tidy::nullability {

testing::Matcher<const proto2::Message&> EqualsProto(llvm::StringRef Textual);

testing::Matcher<const proto2::Message&> EqualsProto(
    const proto2::Message& Expected);

}  // namespace clang::tidy::nullability

#endif
