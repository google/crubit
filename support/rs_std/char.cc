// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/char.h"

#include <cstddef>
#include <cstdint>

#include "absl/log/check.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "support/rs_std/str_ref.h"

namespace rs_std {
namespace {

static constexpr uint8_t kContFlag = 0b10000000;

}  // namespace

StrRef char_::encode_utf8(absl::Span<uint8_t> output_buffer) const {
  size_t len = len_utf8();
  CHECK_LE(len, output_buffer.size());
  switch (len) {
    case 1: {
      output_buffer[0] = static_cast<uint8_t>(value_);
      break;
    }
    case 2: {
      output_buffer[0] =
          static_cast<uint8_t>((value_ >> 6) & 0x1f) | 0b11000000;
      output_buffer[1] = static_cast<uint8_t>(value_ & 0x3f) | kContFlag;
      break;
    }
    case 3: {
      output_buffer[0] =
          static_cast<uint8_t>((value_ >> 12) & 0x0f) | 0b11100000;
      output_buffer[1] = static_cast<uint8_t>((value_ >> 6) & 0x3f) | kContFlag;
      output_buffer[2] = static_cast<uint8_t>(value_ & 0x3f) | kContFlag;
      break;
    }
    case 4: {
      output_buffer[0] =
          static_cast<uint8_t>((value_ >> 18) & 0x07) | 0b11110000;
      output_buffer[1] =
          static_cast<uint8_t>((value_ >> 12) & 0x3f) | kContFlag;
      output_buffer[2] = static_cast<uint8_t>((value_ >> 6) & 0x3f) | kContFlag;
      output_buffer[3] = static_cast<uint8_t>(value_ & 0x3f) | kContFlag;
      break;
    }
  }

  // NOTE: sadly, this reinterpret_cast from `uint8_t*` to `const char*` is
  // required and prevents this function from being constexpr.
  return StrRef::FromUtf8Unchecked(absl::string_view(
      reinterpret_cast<const char*>(output_buffer.data()), len));
}

}  // namespace rs_std
