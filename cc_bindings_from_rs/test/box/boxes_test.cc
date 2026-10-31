// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

namespace rs::std {}

#include "cc_bindings_from_rs/test/box/boxes.h"

#include <cstddef>
#include <cstdint>
#include <optional>
#include <span>
#include <string_view>
#include <utility>

#include "support/movable.h"
#include "gtest/gtest.h"

namespace {

TEST(BoxTest, BasicPrimitives) {
  rs::Box<int32_t> b = boxes::make_box(42);
  EXPECT_EQ(boxes::read_box(b), 42);
  boxes::mutate_box(b);
  EXPECT_EQ(boxes::read_box(b), 43);

  // Returned `rs::Movable<rs::Box<T>>` passes directly to by-value parameters.
  EXPECT_EQ(boxes::consume_box(boxes::make_box(99)), 99);

  // `rs::make_box` returns `rs::Movable<rs::Box<T>>` for C++-constructed boxes.
  int val = boxes::consume_box(rs::make_box<int32_t>(43));
  EXPECT_EQ(val, 43);

  rs::Box<int32_t> b3 = boxes::roundtrip_box(rs::make_box(10));
  EXPECT_EQ(boxes::read_box(b3), 10);

  // `Movable<Box<T>>` forwards `*` directly and exposes `as_box()` for
  // `&Box<T>` / `&mut Box<T>` Rust functions.
  auto mb = boxes::make_box(7);
  EXPECT_EQ(*mb, 7);
  boxes::mutate_box(mb.as_box());
  EXPECT_EQ(boxes::read_box(mb.as_box()), 8);
}

TEST(BoxTest, PointAndHolder) {
  rs::Box<boxes::Point> p = boxes::make_boxed_point();
  EXPECT_EQ(p->x, 10);
  EXPECT_EQ(p->y, 20);

  // Single-dereference field access on `Movable<Box<Point>>`.
  EXPECT_EQ(boxes::make_boxed_point()->x, 10);
  EXPECT_EQ(rs::make_box(boxes::Point{.x = 3, .y = 4})->y, 4);

  int x = boxes::point_x(rs::make_box(boxes::Point{.x = 10, .y = 20}));
  EXPECT_EQ(x, 10);

  // Holder has Clone+Default so it's a regular movable C++ struct
  boxes::Holder h = boxes::make_holder();
  EXPECT_EQ(boxes::holder_value(h), 42);

  // UnmovableHolder is returned wrapped in `rs::Movable<T>`, which both
  // converts implicitly to `UnmovableHolder` and works with `rs::make_box`.
  boxes::UnmovableHolder uh = boxes::make_unmovable_holder();
  EXPECT_EQ(*uh.b, 43);

  auto boxed_uh = rs::make_box(boxes::make_unmovable_holder());
  EXPECT_EQ(*boxed_uh->b, 43);
}

TEST(BoxTest, PinBox) {
  auto pb = boxes::make_pin_box();
  EXPECT_EQ(*pb, 100);
  EXPECT_EQ(boxes::read_pin_box(pb.as_pin_box()), 100);

  EXPECT_EQ(boxes::make_pin_box_not_unpin()->val, 200);
  rs::PinBox<boxes::NotUnpin> not_unpin = boxes::make_pin_box_not_unpin();
  EXPECT_EQ(not_unpin->val, 200);

  rs::Box<boxes::NotUnpin> plain_box = boxes::make_box_not_unpin();
  EXPECT_EQ(plain_box->val, 300);
}

TEST(BoxTest, SliceBox) {
  rs::SliceBox<int32_t> sb = boxes::make_slice_box(5);
  EXPECT_EQ(boxes::sum_slice_box(sb), 5);

  EXPECT_EQ(sb.as_slice().size(), 5);
  std::span<int32_t> span = sb.as_slice();
  for (int32_t v : span) {
    EXPECT_EQ(v, 1);
  }

  // C++-created slice box
  int32_t values[] = {1, 2, 3};
  auto sb2 = rs::Movable<rs::SliceBox<int32_t>>(
      std::in_place, std::span<const int32_t>(values));
  size_t n = boxes::consume_slice_box(std::move(sb2));
  EXPECT_EQ(n, 3);
}

TEST(BoxTest, StrBox) {
  rs::StrBox sb = boxes::make_str_box();
  EXPECT_EQ(boxes::str_box_len(sb), 11);

  std::string_view sv = static_cast<std::string_view>(sb);
  EXPECT_EQ(sv, "hello world");

  // C++-created str box (compile-time checked literal via StrRef, and runtime
  // checked std::string_view via StrBox::FromUtf8).
  rs::StrBox literal_box("foo");
  EXPECT_EQ(boxes::str_box_len(literal_box), 3);
  EXPECT_EQ(boxes::consume_str_box(
                rs::Movable<rs::StrBox>(std::in_place, rs::StrRef("foo"))),
            3);

  std::string_view dynamic_utf8 = "bar!";
  std::optional<rs::Movable<rs::StrBox>> sb2 =
      rs::StrBox::FromUtf8(dynamic_utf8);
  ASSERT_TRUE(sb2.has_value());
  EXPECT_EQ(boxes::consume_str_box(*std::move(sb2)), 4);
}

TEST(BoxTest, DropTracking) {
  std::optional<boxes::Tracked> handle;
  {
    rs::Box<boxes::Tracked> tracked_box = boxes::make_tracked_box();
    handle.emplace(boxes::clone_handle(*tracked_box));
    EXPECT_EQ(boxes::tracked_strong_count(*handle), 2);
  }
  EXPECT_EQ(boxes::tracked_strong_count(*handle), 1);

  // Prove Rust frees C++-allocated memory
  {
    // A Box<Tracked> created in C++ and dropped in Rust
    auto cpp_box = rs::make_box(boxes::clone_handle(*handle));
    EXPECT_EQ(boxes::tracked_strong_count(*handle), 2);
    boxes::consume_tracked(std::move(cpp_box));
    EXPECT_EQ(boxes::tracked_strong_count(*handle), 1);
  }
}

TEST(BoxTest, OptionBox) {
  static_assert(sizeof(rs::Option<rs::Box<int32_t>>) == sizeof(void*));

  rs::Option<rs::Box<int32_t>> opt_none = boxes::make_opt_box(false);
  EXPECT_FALSE(opt_none.has_value());
  EXPECT_EQ(boxes::consume_opt_box(std::move(opt_none)), -1);

  rs::Option<rs::Box<int32_t>> opt_some = boxes::make_opt_box(true);
  EXPECT_TRUE(opt_some.has_value());
  EXPECT_EQ(boxes::consume_opt_box(std::move(opt_some)), 42);
  // The C++ move constructor takes the payload, leaving `None` behind, so the
  // box is not destroyed twice.
  EXPECT_FALSE(opt_some.has_value());  // NOLINT(bugprone-use-after-move)

  rs::Option<rs::Box<int32_t>> cpp_none = std::nullopt;
  EXPECT_EQ(boxes::consume_opt_box(std::move(cpp_none)), -1);

  rs::Option<rs::Box<int32_t>> cpp_some = rs::make_box(1337);
  EXPECT_EQ(boxes::opt_box_value(&cpp_some.value()), 1337);
  EXPECT_EQ(boxes::consume_opt_box(std::move(cpp_some)), 1337);
}

TEST(BoxTest, OptionPinBox) {
  auto opt_none = boxes::make_opt_pin_box(false);
  EXPECT_FALSE(opt_none.has_value());

  auto opt_some = boxes::make_opt_pin_box(true);
  EXPECT_TRUE(opt_some.has_value());
  EXPECT_EQ(opt_some.value()->val, 123);
}

TEST(BoxTest, OptionBoxSlice) {
  auto opt_none = boxes::make_opt_box_slice(false);
  EXPECT_FALSE(opt_none.has_value());

  auto opt_some = boxes::make_opt_box_slice(true);
  EXPECT_TRUE(opt_some.has_value());
  EXPECT_EQ(opt_some.value().size(), 3);
}

TEST(BoxTest, OptionBoxStr) {
  auto opt_none = boxes::make_opt_box_str(false);
  EXPECT_FALSE(opt_none.has_value());

  auto opt_some = boxes::make_opt_box_str(true);
  EXPECT_TRUE(opt_some.has_value());
  EXPECT_EQ(opt_some.value().size(), 5);
}

}  // namespace
