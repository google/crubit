// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_ffi_support.h"

#include <sstream>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "gtest/gtest.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/DenseMapInfo.h"
#include "llvm/ADT/StringRef.h"

namespace {

using ::crubit::ffi::BazelLabel;
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
void crubit_test_roundtrip_bazel_label_map(FfiMap<BazelLabel, FfiString>* map,
                                           FfiMap<BazelLabel, FfiString>* out);
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
  EXPECT_EQ(v.front().AsView(), "first");
  EXPECT_EQ(v.back().AsView(), "third");
  v.emplace_back("temporary");
  EXPECT_EQ(v.back().AsView(), "temporary");
  v.pop_back();
  EXPECT_EQ(v.size(), 2);

  FfiVec<FfiString> span_copy(v.AsSpan());
  EXPECT_EQ(span_copy, v);
  std::vector<FfiString> std_vec = {FfiString("first"), FfiString("third")};
  FfiVec<FfiString> moved_from_std(std::move(std_vec));
  EXPECT_EQ(moved_from_std, v);

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

  // Heterogeneous lookups with FfiString, std::string, and string_view.
  EXPECT_TRUE(out.contains(FfiString("m")));
  EXPECT_TRUE(out.contains(std::string("z")));
  EXPECT_TRUE(out.contains(std::string_view("a")));
  EXPECT_EQ(out.at(FfiString("m")).AsView(), "13");
  EXPECT_EQ(out.at(std::string("z")).AsView(), "26");

  // Non-string key support.
  FfiMap<int, int> int_map;
  int_map[20] = 200;
  int_map[10] = 100;
  EXPECT_TRUE(int_map.contains(10));
  EXPECT_FALSE(int_map.contains(15));
  EXPECT_EQ(int_map.at(20), 200);
  EXPECT_EQ(int_map.entries()[0].first, 10);
  EXPECT_EQ(int_map.entries()[1].first, 20);
}

TEST(IrFfiSupportTest, BazelLabelStripsCanonicalMainRepoPrefix) {
  BazelLabel from_cstr("@@//foo/bar:baz");
  BazelLabel from_str(std::string("@@//foo/bar:baz"));
  BazelLabel from_sv(std::string_view("@@//foo/bar:baz"));
  BazelLabel from_ref(llvm::StringRef("@@//foo/bar:baz"));
  BazelLabel from_ffi(FfiString::FromUtf8("@@//foo/bar:baz"));
  BazelLabel shorthand("//foo/bar:baz");

  for (const BazelLabel& label :
       {from_cstr, from_str, from_sv, from_ref, from_ffi}) {
    EXPECT_EQ(label.value(), "//foo/bar:baz");
    EXPECT_EQ(label.AsView(), "//foo/bar:baz");
    EXPECT_EQ(label.package_name(), "//foo/bar");
    EXPECT_EQ(label.target_name(), "baz");
    EXPECT_EQ(label, shorthand);
    std::ostringstream os;
    os << label;
    EXPECT_EQ(os.str(), "//foo/bar:baz");
  }

  // Non-main-repo prefixes must be preserved verbatim.
  EXPECT_EQ(BazelLabel("@repo//foo:bar").value(), "@repo//foo:bar");
  EXPECT_EQ(BazelLabel("@@repo//foo:bar").value(), "@@repo//foo:bar");
  EXPECT_EQ(BazelLabel("@@").value(), "@@");

  // Shorthand package label matches explicit target name across `@@//` forms.
  EXPECT_EQ(BazelLabel("@@//foo/bar"), BazelLabel("//foo/bar:bar"));
  EXPECT_EQ(BazelLabel("//foo/bar:bar"), "@@//foo/bar");
  EXPECT_EQ(BazelLabel("//foo/bar"), "@@//foo/bar:bar");
}

TEST(IrFfiSupportTest, DenseMapInfoSentinelsAndOperations) {
  using StringInfo = llvm::DenseMapInfo<FfiString>;
  EXPECT_FALSE(StringInfo::isEqual(StringInfo::getEmptyKey(), FfiString("")));
  EXPECT_FALSE(
      StringInfo::isEqual(StringInfo::getTombstoneKey(), FfiString("")));
  EXPECT_FALSE(StringInfo::isEqual(StringInfo::getEmptyKey(),
                                   StringInfo::getTombstoneKey()));
  EXPECT_TRUE(StringInfo::isEqual(StringInfo::getEmptyKey(),
                                  StringInfo::getEmptyKey()));
  EXPECT_TRUE(StringInfo::isEqual(StringInfo::getTombstoneKey(),
                                  StringInfo::getTombstoneKey()));

  using LabelInfo = llvm::DenseMapInfo<BazelLabel>;
  EXPECT_FALSE(LabelInfo::isEqual(LabelInfo::getEmptyKey(), BazelLabel("")));
  EXPECT_FALSE(
      LabelInfo::isEqual(LabelInfo::getTombstoneKey(), BazelLabel("")));
  EXPECT_FALSE(LabelInfo::isEqual(LabelInfo::getEmptyKey(),
                                  LabelInfo::getTombstoneKey()));
  EXPECT_TRUE(
      LabelInfo::isEqual(LabelInfo::getEmptyKey(), LabelInfo::getEmptyKey()));
  EXPECT_TRUE(LabelInfo::isEqual(LabelInfo::getTombstoneKey(),
                                 LabelInfo::getTombstoneKey()));

  llvm::DenseMap<FfiString, int> str_map;
  str_map[FfiString("")] = 1;
  str_map[FfiString("a")] = 2;
  str_map[FfiString("b")] = 3;
  EXPECT_EQ(str_map.lookup(FfiString("")), 1);
  str_map.erase(FfiString("a"));
  EXPECT_FALSE(str_map.contains(FfiString("a")));
  EXPECT_EQ(str_map.lookup(FfiString("")), 1);
  EXPECT_EQ(str_map.lookup(FfiString("b")), 3);
  llvm::DenseMap<FfiString, int> str_map_copy = str_map;
  EXPECT_EQ(str_map_copy.lookup(FfiString("")), 1);
  EXPECT_EQ(str_map_copy.lookup(FfiString("b")), 3);

  llvm::DenseMap<BazelLabel, int> label_map;
  label_map[BazelLabel("")] = 10;
  for (int i = 0; i < 64; ++i) {
    label_map[BazelLabel("//pkg:t" + std::to_string(i))] = i;
  }
  label_map[BazelLabel("//foo/bar:bar")] = 99;
  for (int i = 0; i < 32; ++i) {
    EXPECT_TRUE(label_map.erase(BazelLabel("//pkg:t" + std::to_string(i))));
  }
  EXPECT_EQ(label_map.lookup(BazelLabel("")), 10);
  EXPECT_EQ(label_map.lookup(BazelLabel("@@//foo/bar")), 99);
  EXPECT_EQ(label_map.lookup(BazelLabel(FfiString::FromUtf8("@@//foo/bar"))),
            99);
  for (int i = 32; i < 64; ++i) {
    EXPECT_EQ(label_map.lookup(BazelLabel("//pkg:t" + std::to_string(i))), i);
  }
  llvm::DenseMap<BazelLabel, int> label_map_copy = label_map;
  EXPECT_EQ(label_map_copy.lookup(BazelLabel("")), 10);
  EXPECT_EQ(label_map_copy.lookup(BazelLabel("@@//foo/bar")), 99);
}

TEST(IrFfiSupportTest, BazelLabelFfiMapRoundtripAcrossRust) {
  FfiMap<BazelLabel, FfiString> map;
  map["@@//z/pkg:beta"] = FfiString("2");
  map["//a/pkg:alpha"] = FfiString("1");
  map[BazelLabel(FfiString::FromUtf8("@@//b/foo"))] = FfiString("3");

  FfiMap<BazelLabel, FfiString> out;
  crubit_test_roundtrip_bazel_label_map(&map, &out);
  EXPECT_TRUE(map.empty());
  EXPECT_EQ(out.at("//a/pkg:alpha").AsView(), "1");
  EXPECT_EQ(out.at("//z/pkg:beta").AsView(), "2");
  EXPECT_EQ(out.at("@@//b/foo:foo").AsView(), "3");
  EXPECT_EQ(out.at("//rust/pkg:added").AsView(), "4");
}

}  // namespace
