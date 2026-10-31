// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/box/boxes.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

#include "support/rs/box.h"
#include "support/rs/pin_box.h"
#include "support/rs/slice_box.h"
#include "support/rs/str_box.h"

namespace test {

rs::Box<int32_t> MakeBox(int32_t v) { return rs::Box<int32_t>(v); }
int32_t ReadBox(const rs::Box<int32_t>& b) { return *b; }
void MutateBox(rs::Box<int32_t>& b) { *b = 100; }
int32_t ConsumeBox(rs::Box<int32_t> b) { return *b; }
rs::Box<int32_t> RoundtripBox(rs::Box<int32_t> b) {
  return rs::Box<int32_t>(*b);
}
int32_t ReadBoxPtr(const rs::Box<int32_t>* b) { return **b; }

rs::Box<Point> MakeBoxedPoint() { return rs::Box<Point>(Point{1, 2}); }
int32_t PointX(rs::Box<Point> b) { return b->x; }

Holder* MakeHolder() {
  auto h = new Holder{rs::Box<int32_t>(42)};
  return h;
}
void DestroyHolder(Holder* h) { delete h; }
const rs::Box<int32_t>* HolderBox(const Holder* h) { return &h->b; }

rs::PinBox<NonTrivial> MakePinBoxNonTrivial(int val) {
  return rs::make_pin_box<NonTrivial>(std::make_shared<int>(val));
}
rs::PinBox<NonTrivial> SharePinBoxNonTrivial(const rs::PinBox<NonTrivial>& b) {
  return rs::make_pin_box<NonTrivial>(b->ptr);
}
int64_t NonTrivialUseCount(const rs::PinBox<NonTrivial>& b) {
  return b->ptr.use_count();
}
void ConsumePinBox(rs::PinBox<NonTrivial> b) {}
int32_t ReadPinBoxNonTrivial(const rs::PinBox<NonTrivial>& b) {
  return *b->ptr;
}

rs::PinBox<int32_t> MakePinBoxUnpin() { return rs::PinBox<int32_t>(100); }

rs::SliceBox<int32_t> MakeSliceBox(size_t n) {
  std::vector<int32_t> v(n, 42);
  return rs::SliceBox<int32_t>(v);
}
int32_t SumSliceBox(const rs::SliceBox<int32_t>& b) {
  int32_t sum = 0;
  for (size_t i = 0; i < b.size(); ++i) sum += b[i];
  return sum;
}
size_t ConsumeSliceBox(rs::SliceBox<int32_t> b) { return b.size(); }

rs::StrBox MakeStrBox() { return rs::StrBox("hello"); }
size_t StrBoxLen(const rs::StrBox& b) { return b.size(); }
size_t ConsumeStrBox(rs::StrBox b) { return b.size(); }

rs::Box<NonTrivial> Rejected() { __builtin_trap(); }
void TakesRejected(rs::Box<NonTrivial> b) {}
rs::SliceBox<NonTrivial> RejectedSlice() { __builtin_trap(); }

}  // namespace test
