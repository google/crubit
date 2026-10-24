// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs/alloc.h"
#include "support/rs/char.h"
#include "support/rs/core.h"  // IWYU pragma: keep
#include "support/rs/dyn_callable.h"
#include "support/rs/dyn_erased_future.h"  // IWYU pragma: keep
#include "support/rs/enum.h"               // IWYU pragma: keep
#include "support/rs/fn.h"
#include "support/rs/fn_ref.h"
#include "support/rs/int.h"
#include "support/rs/internal/enum.h"     // IWYU pragma: keep
#include "support/rs/internal/is_utf8.h"  // IWYU pragma: keep
#include "support/rs/iterator_adapter.h"  // IWYU pragma: keep
#include "support/rs/lossy_formatter_for_bindings.h"  // IWYU pragma: keep
#include "support/rs/option.h"
#include "support/rs/result.h"  // IWYU pragma: keep
#include "support/rs/slice_ref.h"
#include "support/rs/std.h"
#include "support/rs/str_ref.h"
#include "support/rs/traits.h"  // IWYU pragma: keep
#include "support/rs/tuple.h"   // IWYU pragma: keep
#include "support/rs/unit.h"
#include "support/rs/vec.h"    // IWYU pragma: keep
#include "support/rs/waker.h"  // IWYU pragma: keep

int main() {
  auto ch = rs::char_::from_u32('x');
  if (!ch.has_value() || *ch != rs::char_('x')) {
    return 1;
  }

  rs::usize n = 7;
  if (n != rs::usize(7)) {
    return 2;
  }

  const int elems[] = {1, 2, 3};
  rs::SliceRef<const int> slice(elems);
  if (slice.size() != 3u) {
    return 3;
  }

  rs::StrRef str("hello");
  if (str.to_string_view() != "hello") {
    return 4;
  }

  rs::Option<int> opt(42);
  if (!opt.has_value() || *opt != 42) {
    return 5;
  }

  if (!(rs::unit == rs::unit_t{})) {
    return 6;
  }

  int add_one_state = 10;
  auto fn_ref_cb = [&](int x) { return x + add_one_state; };
  rs::FnRef<int(int)> fn_ref(fn_ref_cb);
  if (fn_ref(5) != 15) {
    return 7;
  }

  rs::Fn<int(int)> fn([](int x) { return x * 2; });
  if (fn(6) != 12) {
    return 8;
  }

  rs::DynCallable<int(int) const> dyn_cb;
  if (static_cast<bool>(dyn_cb)) {
    return 9;
  }

  rs::alloc::string::String alloc_str("from alloc");
  if (alloc_str.as_str() != "from alloc") {
    return 10;
  }

  rs::std::string::String std_str("from std");
  if (std_str.as_str() != "from std") {
    return 11;
  }

  return 0;
}
