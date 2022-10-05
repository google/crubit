// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/token_stream_printer_impl.h"

#include <string>

#include "gtest/gtest.h"
#include "common/ffi_types.h"

namespace {

std::string ClangFormatForTest(const std::string& input) {
  std::string_view input_slice(input.data(), input.size() + 1);
  CHECK_EQ('\0', input_slice.back());

  crubit::FfiU8SliceBox raw_output =
      Crubit_ClangFormat(crubit::MakeFfiU8Slice(input_slice));
  std::string str_output = std::string(raw_output.ptr, raw_output.size);
  crubit::FreeFfiU8SliceBox(raw_output);
  return str_output;
}

TEST(TokenStreamPrinterImplTest, ClangFormat) {
  const char kInput[] = R"(
    int
    foo
    (
    )
    ;
  )";

  const char kExpectedOutput[] = R"(
int foo();
)";

  EXPECT_EQ(kExpectedOutput, ClangFormatForTest(kInput));
}

}  // namespace
