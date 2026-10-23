// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_ffi_support.h"

#include <thread>  // NOLINT(build/c++11)
#include <vector>

#include "gtest/gtest.h"

namespace {

using ::crubit::ffi::FfiArc;
using ::crubit::ffi::FfiMap;
using ::crubit::ffi::FfiStr;
using ::crubit::ffi::FfiVec;

extern "C" {
void crubit_test_roundtrip_ffi_str(FfiStr* s, FfiStr* out);
void crubit_test_roundtrip_ffi_vec(FfiVec<FfiStr>* v, FfiVec<FfiStr>* out);
void crubit_test_clone_and_cow_ffi_arc(const FfiArc<FfiStr>* shared,
                                       FfiArc<FfiStr>* out_clone,
                                       FfiArc<FfiStr>* out_mutated);
bool crubit_test_ffi_arc_equals(const FfiArc<FfiStr>* lhs,
                                const FfiArc<FfiStr>* rhs);
void crubit_test_roundtrip_ffi_map(FfiMap<FfiStr, FfiStr>* map,
                                   FfiMap<FfiStr, FfiStr>* out);
}

TEST(IrFfiSupportTest, FfiStrRoundtripAcrossRust) {
  FfiStr empty;
  FfiStr roundtripped_empty;
  crubit_test_roundtrip_ffi_str(&empty, &roundtripped_empty);
  EXPECT_TRUE(empty.empty());
  EXPECT_EQ(roundtripped_empty.AsView(), "_from_rust");

  FfiStr hello("hello");
  FfiStr roundtripped;
  crubit_test_roundtrip_ffi_str(&hello, &roundtripped);
  EXPECT_TRUE(hello.empty());
  EXPECT_EQ(roundtripped.AsView(), "hello_from_rust");
}

TEST(IrFfiSupportTest, FfiVecInsertAndRoundtripAcrossRust) {
  FfiVec<FfiStr> v = {FfiStr("first"), FfiStr("third")};
  FfiVec<FfiStr> out;
  crubit_test_roundtrip_ffi_vec(&v, &out);
  EXPECT_TRUE(v.empty());
  ASSERT_EQ(out.size(), 3);
  EXPECT_EQ(out[0].AsView(), "first");
  EXPECT_EQ(out[1].AsView(), "inserted_in_rust");
  EXPECT_EQ(out[2].AsView(), "third");
}

TEST(IrFfiSupportTest, FfiArcCrossLanguageRefCountCowAndNullEquality) {
  FfiArc<FfiStr> null_arc;
  FfiArc<FfiStr> explicit_default_arc(FfiStr(""));
  EXPECT_EQ(null_arc, explicit_default_arc);
  EXPECT_TRUE(crubit_test_ffi_arc_equals(&null_arc, &explicit_default_arc));

  FfiArc<FfiStr> shared(FfiStr("shared_value"));
  FfiArc<FfiStr> rust_clone;
  FfiArc<FfiStr> rust_mutated;
  crubit_test_clone_and_cow_ffi_arc(&shared, &rust_clone, &rust_mutated);

  EXPECT_EQ(shared->AsView(), "shared_value");
  EXPECT_EQ(rust_clone->AsView(), "shared_value");
  EXPECT_EQ(rust_mutated->AsView(), "cow_from_rust");

  // Drop original handle; `rust_clone` must keep the shared allocation alive.
  shared.Reset();
  EXPECT_FALSE(shared.has_value());
  EXPECT_EQ(rust_clone->AsView(), "shared_value");
}

TEST(IrFfiSupportTest, FfiArcConcurrentCloneAndReset) {
  FfiArc<FfiStr> root(FfiStr("concurrent_payload"));
  std::vector<std::thread> threads;
  threads.reserve(4);
  for (int t = 0; t < 4; ++t) {
    threads.emplace_back([root]() {
      std::vector<FfiArc<FfiStr>> copies;
      copies.reserve(256);
      for (int i = 0; i < 256; ++i) {
        copies.push_back(root);
      }
      for (const auto& c : copies) {
        EXPECT_EQ(c->AsView(), "concurrent_payload");
      }
    });
  }
  for (auto& th : threads) {
    th.join();
  }
  EXPECT_EQ(root->AsView(), "concurrent_payload");
}

TEST(IrFfiSupportTest, FfiMapMaintainsSortedOrderAcrossLanguages) {
  FfiMap<FfiStr, FfiStr> map;
  map["z"] = "26";
  map["a"] = "1";

  ASSERT_EQ(map.size(), 2);
  EXPECT_EQ(map.entries()[0].first.AsView(), "a");
  EXPECT_EQ(map.entries()[1].first.AsView(), "z");

  FfiMap<FfiStr, FfiStr> out;
  crubit_test_roundtrip_ffi_map(&map, &out);
  EXPECT_TRUE(map.empty());
  ASSERT_EQ(out.size(), 3);
  EXPECT_EQ(out.entries()[0].first.AsView(), "a");
  EXPECT_EQ(out.entries()[0].second.AsView(), "updated_1");
  EXPECT_EQ(out.entries()[1].first.AsView(), "m");
  EXPECT_EQ(out.entries()[1].second.AsView(), "13");
  EXPECT_EQ(out.entries()[2].first.AsView(), "z");
  EXPECT_EQ(out.entries()[2].second.AsView(), "26");

  FfiMap<FfiStr, FfiStr> same_map;
  same_map["m"] = "13";
  same_map["z"] = "26";
  same_map["a"] = "updated_1";
  EXPECT_EQ(out, same_map);
}

}  // namespace
