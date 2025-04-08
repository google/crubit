// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/slot_fingerprint.h"

#include <array>
#include <cstdint>

#include "llvm/include/llvm/ADT/ArrayRef.h"
#include "llvm/include/llvm/ADT/StringRef.h"
#include "llvm/include/llvm/ADT/bit.h"
#include "llvm/include/llvm/Support/MD5.h"

namespace clang::tidy::nullability {

SlotFingerprint fingerprint(llvm::StringRef USR, uint32_t SlotIndex) {
  // MD5 is an arbitrary choice of hash function.
  llvm::MD5 Hash;
  Hash.update(USR);
  Hash.update(llvm::bit_cast<std::array<uint8_t, 4>>(SlotIndex));
  llvm::MD5::MD5Result Result;
  Hash.final(Result);
  return Result.low();
}
}  // namespace clang::tidy::nullability
