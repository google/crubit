// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_SUPPORT_RS_STD_ITERATOR_ADAPTER_H_
#define CRUBIT_SUPPORT_RS_STD_ITERATOR_ADAPTER_H_

#include <cstddef>
#include <iterator>
#include <optional>
#include <type_traits>
#include <utility>

#include "support/rs_std/traits.h"

// Implementation note: Forward-declaring `rs::core::iter::Iterator` avoids
// having to `#include` the generated `core` bindings - one concern here is
// that the `#include` path may be different in various Crubit clients.
namespace rs::core::iter {
struct Iterator;
}  // namespace rs::core::iter

namespace rs {

// Sentinel that marks the end of a Rust iterator.
struct IteratorEnd {
 public:
  IteratorEnd() = default;
  IteratorEnd(const IteratorEnd&) = default;
  IteratorEnd& operator=(const IteratorEnd&) = default;
};

// Adapter that wraps a Rust-style iterator and presents it as a C++ iterator.
//
// This type models the `std::input_iterator` concept.
//
// Note that Rust iterators do *not* currently model `std::forward_iterator`,
// although it may be potentially (?) fixable in the future:
// * One reason is that `forward_iterator` requires `IteratorAdapter` to be
//   comparable with itself (`std::sentinel_for<I, I>`) and should have a
//   default constructor (`std::incrementable` => `std::regular` =>
//   `std::semiregular` => `std::default_initializable`).  IIUC there are no
//   hard blockers to address this aspect (just need to expand the API of
//   `IteratorAdapter`).
// * Another reason is that `forward_iterator` requires
//   that the iterator can be copied (`std::incrementable` => `std::regular` =>
//   `std::semiregular` => `std::copyable`), and then both copies can be
//   iterated independently over the same sequence (the "multi-pass guarantee").
//   This can potentially be supported in the future by providing this
//   functionality conditonally - if `TAdaptedIterator` is `Clone` (which
//   presumably would ensure that `next()` doesn't affect the other copy?).
template <typename TAdaptedIterator>
  requires(rs_std::where_v<TAdaptedIterator, rs::core::iter::Iterator>)

class IteratorAdapter {
 private:
  using impl = rs_std::impl<TAdaptedIterator, rs::core::iter::Iterator>;
  static constexpr bool kIsPointerItem =
      ::std::is_pointer_v<typename impl::Item>;

 public:
  using value_type =
      ::std::conditional_t<kIsPointerItem,
                           ::std::remove_pointer_t<typename impl::Item>,
                           typename impl::Item>;
  using difference_type = ::std::ptrdiff_t;

 private:
  static constexpr bool kIsMoveOnly =
      !::std::is_copy_constructible_v<value_type> &&
      ::std::is_move_constructible_v<value_type>;

 public:
  using pointer = void;
  using reference = ::std::conditional_t<
      kIsPointerItem, ::std::add_lvalue_reference_t<value_type>,
      ::std::conditional_t<kIsMoveOnly, value_type, const value_type&>>;
  using iterator_category = ::std::input_iterator_tag;
  using iterator_concept = ::std::input_iterator_tag;

  IteratorAdapter() = delete;
  explicit IteratorAdapter(TAdaptedIterator source)
      : source_(::std::move(source)) {
    static_assert(::std::input_iterator<IteratorAdapter>);
    next();
  }

  // `= default` means that `IteratorAdapter` is copyable if and only if
  // `TAdaptedIterator` is copyable.  Non-copyable iterators can be used with
  // the modern C++ ranges APIs like `std::ranges::copy`, but not with legacy
  // C++ APIs like `std::common_iterator`.
  IteratorAdapter(const IteratorAdapter&) = default;
  IteratorAdapter& operator=(const IteratorAdapter&) = default;

  // `IteratorAdapter` is always movable, because we require early on that
  // `TAdaptedIterator` is movable (requiring the `std::movable` concept).
  IteratorAdapter(IteratorAdapter&&) = default;
  IteratorAdapter& operator=(IteratorAdapter&&) = default;

  // Core Requirements for `std::input_iterator`
  //
  // Implementation notes:
  //
  // * `std::input_iterator` => `std::indirectly_readable` requires that
  //   `this`/`self` is passed by `const&` into `operator*` and `iter_move`
  //
  //     ```
  //     template< class In >
  //         concept __IndirectlyReadableImpl =
  //             requires(const In in) {  /* <= `const`! */
  //             ...
  //              { *in } -> std::same_as<std::iter_reference_t<In>>;
  //              { ranges::iter_move(in) } -> ...;
  //     ```
  // * `operator*` returns `const value_type&` (see also `reference` type alias
  //   definition above), because 1) it is a `const` member function and 2)
  //   conceptually mutating the (short-lived) `current_item_` seems
  //   undesirable.
  // * `iter_move(t)` is case #1 of a "customization point object" documented at
  //   https://en.cppreference.com/cpp/iterator/ranges/iter_move.  It is
  //   needed here, because `std::move(*it)` wouldn't work since it returns
  //   a `const value_type&` and not `value_type&` (the former cannot be moved
  //   out of).  `iter_move` has to move out of `current_item_`, so we mark
  //   `current_item_` as `mutable` to reconcile all of these requirements.
  reference operator*() const {
    if constexpr (kIsPointerItem) {
      return *current_item_.value();
    } else if constexpr (kIsMoveOnly) {
      return ::std::move(*current_item_);
    } else {
      return current_item_.value();
    }
  }
  friend value_type&& iter_move(const IteratorAdapter& self) noexcept {
    if constexpr (kIsPointerItem) {
      return ::std::move(*self.current_item_.value());
    } else {
      return ::std::move(*self.current_item_);
    }
  }
  IteratorAdapter& operator++() {
    next();
    return *this;
  }
  void operator++(int) { next(); }
  bool operator==(IteratorEnd) const { return !current_item_.has_value(); }

 private:
  void next() { current_item_ = impl::next(source_); }

  TAdaptedIterator source_;

  // `mutable` to support `iter_move` taking a `const IteratorAdapter&`.
  mutable ::std::optional<typename impl::Item> current_item_;
};

template <typename TAdaptedIterator>
IteratorAdapter(TAdaptedIterator) -> IteratorAdapter<TAdaptedIterator>;

}  // namespace rs

namespace rs_std {
// Mirrors `impl<I: Iterator + ?Sized> Iterator for &mut I` from the Rust
// standard library, which is needed because we don't automatically generate
// bindings for blanket impls today.
template <typename T>
  requires(rs_std::where_v<T, rs::core::iter::Iterator>)
struct impl<T*, ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item = typename rs_std::impl<T, ::rs::core::iter::Iterator>::Item;
  static ::std::optional<Item> next(T* self) {
    return rs_std::impl<T, ::rs::core::iter::Iterator>::next(*self);
  }
};
}  // namespace rs_std

#endif  // CRUBIT_SUPPORT_RS_STD_ITERATOR_ADAPTER_H_
