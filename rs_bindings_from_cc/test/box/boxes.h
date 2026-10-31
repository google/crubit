// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_BOX_BOXES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_BOX_BOXES_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <utility>

#include "support/rs/box.h"
#include "support/rs/pin_box.h"
#include "support/rs/slice_box.h"
#include "support/rs/str_box.h"

namespace test {

rs::Box<int32_t> MakeBox(int32_t v);
int32_t ReadBox(const rs::Box<int32_t>& b);
void MutateBox(rs::Box<int32_t>& b);
int32_t ConsumeBox(rs::Box<int32_t> b);
rs::Box<int32_t> RoundtripBox(rs::Box<int32_t> b);
int32_t ReadBoxPtr(const rs::Box<int32_t>* b);

struct Point {
  int x;
  int y;
};

rs::Box<Point> MakeBoxedPoint();
int32_t PointX(rs::Box<Point> b);

struct Holder {
  rs::Box<int32_t> b;
};

Holder* MakeHolder();
void DestroyHolder(Holder* h);
const rs::Box<int32_t>* HolderBox(const Holder* h);

// A type that is not Rust-movable (`!Unpin` in Rust) because it has a
// user-defined move constructor. It shares ownership of an `int` so that tests
// can observe destruction through `use_count()`.
struct NonTrivial {
  explicit NonTrivial(std::shared_ptr<int> p) : ptr(std::move(p)) {}
  NonTrivial(const NonTrivial&) = delete;
  NonTrivial& operator=(const NonTrivial&) = delete;
  NonTrivial(NonTrivial&& other) : ptr(std::move(other.ptr)) {}
  NonTrivial& operator=(NonTrivial&& other) {
    ptr = std::move(other.ptr);
    return *this;
  }
  ~NonTrivial() = default;

  std::shared_ptr<int> ptr;
};

rs::PinBox<NonTrivial> MakePinBoxNonTrivial(int val);
// Returns a new box whose `NonTrivial` shares ownership with `b`'s.
rs::PinBox<NonTrivial> SharePinBoxNonTrivial(const rs::PinBox<NonTrivial>& b);
int64_t NonTrivialUseCount(const rs::PinBox<NonTrivial>& b);
void ConsumePinBox(rs::PinBox<NonTrivial> b);
int32_t ReadPinBoxNonTrivial(const rs::PinBox<NonTrivial>& b);

rs::PinBox<int32_t> MakePinBoxUnpin();

rs::SliceBox<int32_t> MakeSliceBox(size_t n);
int32_t SumSliceBox(const rs::SliceBox<int32_t>& b);
size_t ConsumeSliceBox(rs::SliceBox<int32_t> b);

rs::StrBox MakeStrBox();
size_t StrBoxLen(const rs::StrBox& b);
size_t ConsumeStrBox(rs::StrBox b);

// `NonTrivial` is not Rust-movable, so bindings for these functions are
// rejected (the error messages are checked by the `boxes` golden test). They
// exist here to verify that the rest of the crate still builds and runs.
rs::Box<NonTrivial> Rejected();
void TakesRejected(rs::Box<NonTrivial> b);
rs::SliceBox<NonTrivial> RejectedSlice();

}  // namespace test

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_BOX_BOXES_H_
