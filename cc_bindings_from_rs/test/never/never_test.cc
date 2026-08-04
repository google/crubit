// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/never/never.h"

#include "gtest/gtest.h"

// Wrapper around EXPECT_DEATH that accommodates Android emulators/devices.
// Android assert and panic messages are routed to logcat (/dev/log) rather than
// stderr, so death tests cannot inspect stderr/stdout for regex matching.
#ifdef __ANDROID__
#define CRUBIT_EXPECT_DEATH(statement, regex) EXPECT_DEATH(statement, "")
#else
#define CRUBIT_EXPECT_DEATH(statement, regex) EXPECT_DEATH(statement, regex)
#endif

namespace {

TEST(NeverTest, NeverReturnDoesNotReturn) {
  CRUBIT_EXPECT_DEATH(never::never_return(), "You can't do that!");
}

TEST(NeverTest, ExternNeverReturnDoesNotReturn) {
  CRUBIT_EXPECT_DEATH(never::extern_never_return(),
                      "You can't do that directly!");
}

TEST(NeverTest, AssocatedFnNeverReturnDoesNotReturn) {
  CRUBIT_EXPECT_DEATH(never::NeverStruct::associated_fn_never_return(),
                      "You can't do that as an associated fn!");
}

TEST(NeverTest, MethodNeverReturnDoesNotReturn) {
  never::NeverStruct never_struct;
  CRUBIT_EXPECT_DEATH(never_struct.method_never_return(),
                      "You can't do that as a method!");
}

}  // namespace
