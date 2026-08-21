// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "absl/hash/hash.h"

#include <functional>
#include <unordered_map>
#include <unordered_set>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "cc_bindings_from_rs/test/known_traits/hash/rs_hash.h"

namespace crubit {
namespace {

TEST(HashTest, DerivedStructAbslHash) {
  namespace test = rs_hash::derived_struct;
  test::Point p1 = test::create_point(10, 20);
  test::Point p2 = test::create_point(10, 20);
  test::Point p3 = test::create_point(10, 30);

  EXPECT_EQ(absl::Hash<test::Point>()(p1), absl::Hash<test::Point>()(p2));
  EXPECT_NE(absl::Hash<test::Point>()(p1), absl::Hash<test::Point>()(p3));
}

TEST(HashTest, DerivedStructStdHash) {
  namespace test = rs_hash::derived_struct;
  test::Point p1 = test::create_point(10, 20);
  test::Point p2 = test::create_point(10, 20);
  test::Point p3 = test::create_point(10, 30);

  EXPECT_EQ(std::hash<test::Point>()(p1), std::hash<test::Point>()(p2));
  EXPECT_NE(std::hash<test::Point>()(p1), std::hash<test::Point>()(p3));
}

TEST(HashTest, DerivedStructAbslContainer) {
  namespace test = rs_hash::derived_struct;
  absl::flat_hash_set<test::Point> points;
  points.insert(test::create_point(1, 2));
  points.insert(test::create_point(3, 4));

  EXPECT_TRUE(points.contains(test::create_point(1, 2)));
  EXPECT_TRUE(points.contains(test::create_point(3, 4)));
  EXPECT_FALSE(points.contains(test::create_point(5, 6)));

  absl::flat_hash_map<test::Point, int> map;
  map[test::create_point(1, 2)] = 42;
  EXPECT_EQ(map[test::create_point(1, 2)], 42);
}

TEST(HashTest, DerivedStructStdContainer) {
  namespace test = rs_hash::derived_struct;
  std::unordered_set<test::Point> points;
  points.insert(test::create_point(1, 2));
  points.insert(test::create_point(3, 4));

  EXPECT_EQ(points.count(test::create_point(1, 2)), 1);
  EXPECT_EQ(points.count(test::create_point(3, 4)), 1);
  EXPECT_EQ(points.count(test::create_point(5, 6)), 0);

  std::unordered_map<test::Point, int> map;
  map[test::create_point(1, 2)] = 42;
  EXPECT_EQ(map[test::create_point(1, 2)], 42);
}

TEST(HashTest, ExplicitStructHash) {
  namespace test = rs_hash::explicit_struct;
  test::CustomHashStruct c1 = test::create_custom(5);
  test::CustomHashStruct c2 = test::create_custom(5);
  test::CustomHashStruct c3 = test::create_custom(10);

  EXPECT_EQ(absl::Hash<test::CustomHashStruct>()(c1),
            absl::Hash<test::CustomHashStruct>()(c2));
  EXPECT_NE(absl::Hash<test::CustomHashStruct>()(c1),
            absl::Hash<test::CustomHashStruct>()(c3));

  EXPECT_EQ(std::hash<test::CustomHashStruct>()(c1),
            std::hash<test::CustomHashStruct>()(c2));
  EXPECT_NE(std::hash<test::CustomHashStruct>()(c1),
            std::hash<test::CustomHashStruct>()(c3));
}

TEST(HashTest, DerivedEnumHash) {
  namespace test = rs_hash::derived_enum;
  test::Color red = test::create_red();
  test::Color green = test::create_green();
  test::Color blue = test::create_blue();
  test::Color red2 = test::create_red();

  EXPECT_EQ(absl::Hash<test::Color>()(red), absl::Hash<test::Color>()(red2));
  EXPECT_NE(absl::Hash<test::Color>()(red), absl::Hash<test::Color>()(green));
  EXPECT_NE(absl::Hash<test::Color>()(green), absl::Hash<test::Color>()(blue));

  EXPECT_EQ(std::hash<test::Color>()(red), std::hash<test::Color>()(red2));
  EXPECT_NE(std::hash<test::Color>()(red), std::hash<test::Color>()(green));
  EXPECT_NE(std::hash<test::Color>()(green), std::hash<test::Color>()(blue));
}

TEST(HashTest, DerivedTupleStructHash) {
  namespace test = rs_hash::derived_tuple_struct;
  test::TupleStruct t1 = test::create_tuple(10, 20);
  test::TupleStruct t2 = test::create_tuple(10, 20);
  test::TupleStruct t3 = test::create_tuple(20, 10);

  EXPECT_EQ(absl::Hash<test::TupleStruct>()(t1),
            absl::Hash<test::TupleStruct>()(t2));
  EXPECT_NE(absl::Hash<test::TupleStruct>()(t1),
            absl::Hash<test::TupleStruct>()(t3));

  EXPECT_EQ(std::hash<test::TupleStruct>()(t1),
            std::hash<test::TupleStruct>()(t2));
  EXPECT_NE(std::hash<test::TupleStruct>()(t1),
            std::hash<test::TupleStruct>()(t3));
}

}  // namespace
}  // namespace crubit
