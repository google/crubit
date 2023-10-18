// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Production of fingerprints identifying individual nullability slots, e.g. the
// nullability of an int* type of a particular function parameter, or the
// outermost nullability of an int*** return type of a function.

#ifndef CRUBIT_NULLABILITY_INFERENCE_SLOT_FINGERPRINT_H_
#define CRUBIT_NULLABILITY_INFERENCE_SLOT_FINGERPRINT_H_

#include <cstdint>

#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {

// A SlotFingerprint is a lossy representation of a symbol's USR and a slot
// within it. We use this to determine whether nullability has been inferred in
// previous rounds. Using only the hash value makes it feasible to fit all
// inference results in memory, and collisions are still sufficiently unlikely
// (we expect ~100M inference targets).
using SlotFingerprint = uint64_t;

SlotFingerprint fingerprint(llvm::StringRef USR, uint32_t SlotIndex);
}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_SLOT_FINGERPRINT_H_
