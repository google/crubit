// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_MEMCPY_MOVABLE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_MEMCPY_MOVABLE_H_

#include "support/annotations.h"

namespace crubit::test {

// Tests that a class annotated with `CRUBIT_UNSAFE_MEMCPY_MOVABLE` is
// moved via memcpy in Rust bindings even if it has non-trivial move operations.
class CRUBIT_UNSAFE_MEMCPY_MOVABLE MemcpyMovableClass {
 public:
  MemcpyMovableClass() : unused_(5) {}
  MemcpyMovableClass(const MemcpyMovableClass&) {}
  MemcpyMovableClass(MemcpyMovableClass&&) {}
  MemcpyMovableClass& operator=(const MemcpyMovableClass&) { return *this; }
  MemcpyMovableClass& operator=(MemcpyMovableClass&&) { return *this; }

 private:
  [[maybe_unused]] int unused_;
};

class NonMemcpyMovableClass {
 public:
  NonMemcpyMovableClass() : unused_(5) {}
  NonMemcpyMovableClass(const NonMemcpyMovableClass&) {}
  NonMemcpyMovableClass(NonMemcpyMovableClass&&) {}
  NonMemcpyMovableClass& operator=(const NonMemcpyMovableClass&) {
    return *this;
  }
  NonMemcpyMovableClass& operator=(NonMemcpyMovableClass&&) { return *this; }

 private:
  [[maybe_unused]] int unused_;
};

inline MemcpyMovableClass ReturnsMemcpyMovable() {
  return MemcpyMovableClass();
}
inline NonMemcpyMovableClass ReturnsNonMemcpyMovable() {
  return NonMemcpyMovableClass();
}

inline void AcceptsMemcpyMovable(MemcpyMovableClass) {}
inline void AcceptsNonMemcpyMovable(NonMemcpyMovableClass) {}

// Uncomment to see bindings generation error:
//   Dynamic classes (classes with virtual functions or bases) are not movable
//   via memcpy.
// class CRUBIT_UNSAFE_MEMCPY_MOVABLE IllegallyMemcpyMovable {
//  public:
//   virtual ~IllegallyMemcpyMovable() = default;
// };

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_MEMCPY_MOVABLE_H_
