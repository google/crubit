// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/lifetime_annotations.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_LIFETIME_ANNOTATIONS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_LIFETIME_ANNOTATIONS_H_

// Shorthand for arbitrary-length lifetime name annotations.
// This can be used like `$(my_lifetime_name)`.
//
// See also the predefined macros for `$static` and single-character
// (e.g. `$a`) lifetimes below.
//
// These annotations communicate the lifetimes of the input types to Crubit.
// For more information on the design, see b/454627672.
//
// TODO(mboehme): We would prefer `$(...)` to be a variadic macro that
// stringizes each of its macro arguments individually. This is possible but
// requires some contortions: https://stackoverflow.com/a/5958315
#if defined(__clang__) && __has_cpp_attribute(clang::annotate_type)
#define $(l) [[clang::annotate_type("lifetime", #l)]]
#else
#define $(l)
#endif

// Shorthand for lifetime parameter annotations on types and functions.
// This can be used like `$lifetime_param("a")` or `$lifetime_param("a", "b")`.
#if defined(__clang__) && __has_cpp_attribute(clang::annotate)
#define $lifetime_param(...) [[clang::annotate("lifetime_params", __VA_ARGS__)]]
#define $lifetime_params(...) \
  [[clang::annotate("lifetime_params", __VA_ARGS__)]]
#else
#define $lifetime_param(...)
#define $lifetime_params(...)
#endif

// Shorthand for a static lifetime annotation.
#define $static $(static)

// `$unknown`: nothing is known about how long the pointee lives or what it is
// tied to.
//
// DECIDED. `$unknown` is the one spelling; `$(unknown)` is only its expansion.
//
// `$unknown` is not a lifetime variable. It is never unified with another
// lifetime name, it has no outlives relation with any lifetime, and it is never
// `'static`. Crubit binds a position written `$unknown` conservatively, and
// never reports an error for it:
//
// - A parameter becomes a raw pointer, or `raw_string_view` for
//   `std::string_view`.
// - A reference result becomes a raw pointer.
// - A record result that carries lifetimes makes the function an `unsafe fn`,
//   with a Safety comment saying that the caller must keep the borrowed data
//   alive (b/589394789).
//
// Writing `$unknown` explicitly produces no warning: it is a deliberate
// statement. In contrast, a lifetime name written on a result that no
// parameter, enclosing record or `$static` binds binds as if unannotated, with
// a warning, since it is probably a mistake (b/589394789); only an explicit
// `$unknown` is unknown.
//
// Records whose lifetimes are unknown (unannotated records that may hold
// pointers) follow a policy that is still OPEN. Unannotated code is never an
// error.
//
// Example:
//
//   // Returns a pointer whose lifetime the caller must track by hand.
//   const Config& $unknown GlobalConfigFor(const Request& r);
#define $unknown $(unknown)

// Reserved names.
//
// DECIDED: the names below are reserved for attributes that Crubit does not
// implement yet. They are written like lifetimes, but they are not lifetimes.
// Crubit ignores a reserved name as if it were absent, and warns
// ("'$solo' is reserved (no effect yet); ignored"). It is never an error. A
// reserved name is removed from every position where it is written, before
// any lifetime is bound:
//
// - On a pointer or reference level, the other names written on that level
//   are kept. If every name on the level is reserved, the level has no
//   explicit lifetime and is elided as usual. It is *not* treated as
//   `$unknown`. So `int& $a $solo` is `int& $a`, `int& $solo` is `int&`, and
//   `int* $solo` is `int*`.
// - In a declared lifetime parameter list (`$lifetime_param("solo")`, or the
//   same `lifetime_params` annotation in any other spelling), the name is
//   dropped before the record's lifetime count is computed. So
//   `struct $lifetime_param("solo") S` declares no lifetime.
//
// The binding of a type written with a reserved name is the binding of the same
// type written without it, with all its properties. For example, `R& $solo`
// (with `R` a record with a lifetime) binds like `R&`. A function that stores a
// borrow of another parameter into such an `R&` is unsound today with or
// without `$solo` (row K3 of the lifetime-safety plan; probe cl/1027824779).
// A fix for K3 covers both.
//
// A user may not use a reserved name as an ordinary lifetime name. What each
// name will mean is described below. The meaning is intended, and the details
// are OPEN.

// `$solo` (RESERVED): the exclusive attribute. For the borrow's duration, this
// pointer is the only live access path to its pointee, and what it reaches is
// disjoint from every other argument. It is what Rust's `&mut` promises and
// what `CMut` does not promise. Intended consequences:
//
// - A store through a `$solo` pointer may be strong: it overwrites the old
//   value and ends the old borrows. Today a store through a parameter is weak
//   (merged).
// - Invalidating calls through it are allowed.
// - Crubit may bind it as `&mut` rather than `CMut`.
// - C++ callers are checked for disjointness at the call; Rust callers get it
//   from `&mut`.
//
// OPEN: whether it is the default for non-const `T&`; whether it is copyable or
// move-only; whether it applies to a whole view or to each pointer level; and
// how it interacts with `$frozen`. Until it is designed, the name is only
// reserved.
#define $solo $(solo)

// Shorthand for a single-character lifetime annotation.
#define $a $(a)
#define $b $(b)
#define $c $(c)
#define $d $(d)
#define $e $(e)
#define $f $(f)
#define $g $(g)
#define $h $(h)
#define $i $(i)
#define $j $(j)
#define $k $(k)
#define $l $(l)
#define $m $(m)
#define $n $(n)
#define $o $(o)
#define $p $(p)
#define $q $(q)
#define $r $(r)
#define $s $(s)
#define $t $(t)
#define $u $(u)
#define $v $(v)
#define $w $(w)
#define $x $(x)
#define $y $(y)
#define $z $(z)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_LIFETIME_ANNOTATIONS_H_
