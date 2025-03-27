// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/internal/is_utf8.h"

#include <array>
#include <cstdint>

#include "gtest/gtest.h"
#include "fuzztest/fuzztest.h"
#include "absl/flags/flag.h"
#include "absl/strings/string_view.h"
#include "util/textprogressbar/textprogressbar.h"
#include "util/utf8/public/unilib.h"

ABSL_FLAG(bool, check_all_32_bit_values, false,
          "If true, check that IsUtf8() returns the same value as "
          "UniLib::IsStructurallyValid() for all possible 32-bit arguments.");

namespace {

static_assert(rs_std::internal::IsAscii("abc"));
static_assert(rs_std::internal::IsAscii("abc\0"));
static_assert(!rs_std::internal::IsAscii("abc\x80"));
static_assert(!rs_std::internal::IsAscii("abc\xff"));

void ExpectAscii(absl::string_view data) {
  EXPECT_TRUE(rs_std::internal::IsAscii(data));
  EXPECT_TRUE(rs_std::internal::IsUtf8(data));
}
FUZZ_TEST(IsAsciiFuzzTest, ExpectAscii).WithDomains(fuzztest::AsciiString());

static_assert(rs_std::internal::IsUtf8("abc"));
static_assert(rs_std::internal::IsUtf8("abc\0"));
static_assert(rs_std::internal::IsUtf8("👁💖🐶"));
static_assert(rs_std::internal::IsUtf8("abc\xc2\x80"));
static_assert(rs_std::internal::IsUtf8("abc\xe0\xa0\x80"));

template <int... Bytes>
constexpr bool BytesAreUtf8() {
  const std::array<char, sizeof...(Bytes)> bytes_array = {Bytes...};
  return rs_std::internal::IsUtf8(
      absl::string_view(bytes_array.data(), bytes_array.size()));
}

// Test continuation byte as first byte (first byte is 0b10xxxxxx)
static_assert(!BytesAreUtf8<0b10111111>());
// Test too-short code unit sequence
//   first byte is 0b110yyyyy, second byte begins with either 0b0 or 0b11
static_assert(!BytesAreUtf8<0b11000000, 0b00000000>());
static_assert(!BytesAreUtf8<0b11000000, 0b11000000>());
// Test overlong code unit sequence
//   0b11000000 0b10000000 encodes the codepoint 0 using 2 bytes when it should
//   use one.
static_assert(!BytesAreUtf8<0b11000000, 0b10000000>());
// Test reserved code point
static_assert(!BytesAreUtf8<0b11111111>());
// Test surrogate code point
static_assert(!BytesAreUtf8<0b10110000, 0b10111111>());

void ExpectUtf8(absl::string_view data) {
  EXPECT_TRUE(rs_std::internal::IsUtf8(data));
}
FUZZ_TEST(IsUtf8FuzzTest, ExpectUtf8).WithDomains(fuzztest::Utf8String());

// Compare against an existing UTF8 validation function.
void ExpectEqualsExistingUtf8Validation(absl::string_view data) {
  EXPECT_EQ(rs_std::internal::IsUtf8(data), UniLib::IsStructurallyValid(data));
}
FUZZ_TEST(IsUtf8FuzzTest, ExpectEqualsExistingUtf8Validation);

TEST(IsUtf8Test, ExpectEqualsExistingUtf8ValidationForAllPossibleChars) {
  // All possible UTF-8 characters are covered by searching over all possible
  // uint32_t values.
  // See
  // https://randomascii.wordpress.com/2014/01/27/theres-only-four-billion-floatsso-test-them-all/
  //
  // This test takes ~32 seconds to run when built with -opt
  // on my workstation.
  if (!absl::GetFlag(FLAGS_check_all_32_bit_values)) {
    GTEST_SKIP() << "This test is not normally run because it is quite large.";
  }
  util_textprogressbar::TextProgressBar progress(
      "Checking all UTF-8 characters");
  progress.set_max_progress(UINT32_MAX);
  progress.Start();
  uint32_t i = 0;
  constexpr uint32_t kProgressUpdateInterval = UINT32_MAX / 1024;
  while (true) {
    absl::string_view data(reinterpret_cast<const char*>(&i), sizeof(i));
    bool is_utf8 = rs_std::internal::IsUtf8(data);
    bool other_is_utf8 = UniLib::IsStructurallyValid(data);
    // Wrap the assertion in an if statement to avoid generating billions of
    // expectations.
    if (is_utf8 != other_is_utf8) {
      ASSERT_EQ(is_utf8, other_is_utf8);
    }
    // Don't update progress every time to avoid the cost of the indirect call.
    if (i % kProgressUpdateInterval == 0) {
      progress.SetProgress(i);
    }
    if (i == UINT32_MAX) {
      break;
    }
    ++i;
  }
  progress.Finish();
}

}  // namespace
