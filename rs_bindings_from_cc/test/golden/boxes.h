// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BOXES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BOXES_H_

#include "support/rs/box.h"
#include "support/rs/pin_box.h"
#include "support/rs/slice_box.h"
#include "support/rs/str_box.h"

namespace test {

rs::Box<int> MakeBox(int v);
int ReadBox(const rs::Box<int>& b);
void MutateBox(rs::Box<int>& b);
int ConsumeBox(rs::Box<int> b);
rs::Box<int> RoundtripBox(rs::Box<int> b);
int ReadBoxPtr(const rs::Box<int>* b);

struct Point {
  int x;
  int y;
};

rs::Box<Point> MakeBoxedPoint();
int PointX(rs::Box<Point> b);

struct Holder {
  rs::Box<int> b;
};

Holder* MakeHolder();
void DestroyHolder(Holder* h);
const rs::Box<int>* HolderBox(const Holder* h);

// A type that is not Rust-movable (`!Unpin` in Rust) because it has a
// user-defined move constructor and destructor.
struct NonTrivial {
  explicit NonTrivial(int v) : val(v) {}
  NonTrivial(const NonTrivial&) = delete;
  NonTrivial& operator=(const NonTrivial&) = delete;
  NonTrivial(NonTrivial&& other) : val(other.val) {}
  NonTrivial& operator=(NonTrivial&& other) {
    val = other.val;
    return *this;
  }
  ~NonTrivial() {}

  int val;
};

rs::PinBox<NonTrivial> MakePinBoxNonTrivial(int val);
rs::PinBox<NonTrivial> SharePinBoxNonTrivial(const rs::PinBox<NonTrivial>& b);
long NonTrivialUseCount(const rs::PinBox<NonTrivial>& b);
void ConsumePinBox(rs::PinBox<NonTrivial> b);
int ReadPinBoxNonTrivial(const rs::PinBox<NonTrivial>& b);

rs::PinBox<int> MakePinBoxUnpin();

rs::SliceBox<int> MakeSliceBox(unsigned long n);
int SumSliceBox(const rs::SliceBox<int>& b);
unsigned long ConsumeSliceBox(rs::SliceBox<int> b);

rs::StrBox MakeStrBox();
unsigned long StrBoxLen(const rs::StrBox& b);
unsigned long ConsumeStrBox(rs::StrBox b);

// Rejected/TakesRejected/RejectedSlice exist to assert the !Unpin rejection
// comments.
rs::Box<NonTrivial> Rejected();
void TakesRejected(rs::Box<NonTrivial> b);
rs::SliceBox<NonTrivial> RejectedSlice();

}  // namespace test

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BOXES_H_
