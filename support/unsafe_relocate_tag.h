// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_UNSAFE_RELOCATE_TAG_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_UNSAFE_RELOCATE_TAG_H_

namespace crubit {

// A type tag for constructors to move-construct via a trivial relocation
// operation, or a Rust move, rather than by running the actual logic of a move
// constructor.
//
// A constructor which accepts `(UnsafeRelocateTag, T&& x)` will relocate `x`
// into the new object, leaving `x` in an uninitialized state. The caller must
// not run the destructor of `x` (or otherwise use it) without first
// reinitializing it.
//
// This can be used, for example, by `crubit::Slot` in
// `crubit/support/internal/slot.h` to initialize a value on the stack, and then
// move it into a return value without performing a C++ move operation.
struct UnsafeRelocateTag {};

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_UNSAFE_RELOCATE_TAG_H_
