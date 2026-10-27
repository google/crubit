// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_ffi_support.h"

#include <utility>
#include <vector>

#include "gtest/gtest.h"

namespace {

using ::crubit::ffi::FfiMap;
using ::crubit::ffi::FfiRc;
using ::crubit::ffi::FfiString;
using ::crubit::ffi::FfiVec;

extern "C" {
void crubit_test_roundtrip_ffi_string(FfiString* s, FfiString* out);
void crubit_test_pass_ffi_string_by_value(FfiString s, FfiString* out);
void crubit_test_roundtrip_ffi_vec(FfiVec<FfiString>* v,
                                   FfiVec<FfiString>* out);
void crubit_test_clone_and_cow_ffi_rc(const FfiRc<FfiString>* shared,
                                      FfiRc<FfiString>* out_clone,
                                      FfiRc<FfiString>* out_mutated);
bool crubit_test_ffi_rc_equals(const FfiRc<FfiString>* lhs,
                               const FfiRc<FfiString>* rhs);
void crubit_test_roundtrip_ffi_map(FfiMap<FfiString, FfiString>* map,
                                   FfiMap<FfiString, FfiString>* out);
}

TEST(IrFfiSupportTest, FfiStringRoundtripAcrossRust) {
  FfiString empty;
  EXPECT_EQ(empty.data(), nullptr);
  EXPECT_EQ(empty.AsView(), "");
  FfiString roundtripped_empty;
  crubit_test_roundtrip_ffi_string(&empty, &roundtripped_empty);
  EXPECT_TRUE(empty.empty());
  EXPECT_EQ(roundtripped_empty.AsView(), "_from_rust");

  FfiString hello("hello");
  FfiString roundtripped;
  crubit_test_roundtrip_ffi_string(&hello, &roundtripped);
  EXPECT_TRUE(hello.empty());
  EXPECT_EQ(roundtripped.AsView(), "hello_from_rust");

  FfiString by_value_out;
  crubit_test_pass_ffi_string_by_value(FfiString("by_val"), &by_value_out);
  EXPECT_EQ(by_value_out.AsView(), "by_val_by_value");
}

TEST(IrFfiSupportTest, FfiVecInsertAndRoundtripAcrossRust) {
  FfiVec<FfiString> v = {FfiString("first"), FfiString("third")};
  FfiVec<FfiString> out;
  crubit_test_roundtrip_ffi_vec(&v, &out);
  EXPECT_TRUE(v.empty());
  ASSERT_EQ(out.size(), 3);
  EXPECT_EQ(out[0].AsView(), "first");
  EXPECT_EQ(out[1].AsView(), "inserted_in_rust");
  EXPECT_EQ(out[2].AsView(), "third");
}

TEST(IrFfiSupportTest, FfiRcCrossLanguageRefCountCowAndNullEquality) {
  FfiRc<FfiString> null_rc;
  FfiRc<FfiString> explicit_default_rc(FfiString(""));
  EXPECT_EQ(null_rc, explicit_default_rc);
  EXPECT_TRUE(crubit_test_ffi_rc_equals(&null_rc, &explicit_default_rc));

  FfiRc<FfiString> shared(FfiString("shared_value"));
  FfiRc<FfiString> rust_clone;
  FfiRc<FfiString> rust_mutated;
  crubit_test_clone_and_cow_ffi_rc(&shared, &rust_clone, &rust_mutated);

  EXPECT_EQ(shared->AsView(), "shared_value");
  EXPECT_EQ(rust_clone->AsView(), "shared_value");
  EXPECT_EQ(rust_mutated->AsView(), "cow_from_rust");

  // Drop original handle; `rust_clone` must keep the shared allocation alive.
  shared.Reset();
  EXPECT_FALSE(shared.has_value());
  EXPECT_EQ(rust_clone->AsView(), "shared_value");
}

TEST(IrFfiSupportTest, FfiRcMultipleClonesAndReset) {
  FfiRc<FfiString> root(FfiString("shared_payload"));
  std::vector<FfiRc<FfiString>> copies;
  copies.reserve(256);
  for (int i = 0; i < 256; ++i) {
    copies.push_back(root);
  }
  for (const auto& c : copies) {
    EXPECT_EQ(c->AsView(), "shared_payload");
  }
  copies.clear();
  EXPECT_EQ(root->AsView(), "shared_payload");
}

TEST(IrFfiSupportTest, FfiMapMaintainsSortedOrderAcrossLanguages) {
  FfiMap<FfiString, FfiString> map;
  map["z"] = FfiString("26");
  map["a"] = FfiString("1");

  ASSERT_EQ(map.size(), 2);
  EXPECT_EQ(map.entries()[0].first.AsView(), "a");
  EXPECT_EQ(map.entries()[1].first.AsView(), "z");

  FfiMap<FfiString, FfiString> out;
  crubit_test_roundtrip_ffi_map(&map, &out);
  EXPECT_TRUE(map.empty());
  ASSERT_EQ(out.size(), 3);
  EXPECT_EQ(out.entries()[0].first.AsView(), "a");
  EXPECT_EQ(out.entries()[0].second.AsView(), "updated_1");
  EXPECT_EQ(out.entries()[1].first.AsView(), "m");
  EXPECT_EQ(out.entries()[1].second.AsView(), "13");
  EXPECT_EQ(out.entries()[2].first.AsView(), "z");
  EXPECT_EQ(out.entries()[2].second.AsView(), "26");

  FfiMap<FfiString, FfiString> same_map;
  same_map["m"] = FfiString("13");
  same_map["z"] = FfiString("26");
  same_map["a"] = FfiString("updated_1");
  EXPECT_EQ(out, same_map);
}

}  // namespace
