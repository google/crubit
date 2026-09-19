// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/annotations.h"

#ifndef CRUBIT_SUPPORT_ANNOTATIONS_INTERNAL_H_
#define CRUBIT_SUPPORT_ANNOTATIONS_INTERNAL_H_

#if defined(__clang__)
#define CRUBIT_TRIVIAL_ABI [[clang::trivial_abi]]
#define CRUBIT_VIEW [[gsl::Pointer]]
#if __has_cpp_attribute(clang::lifetimebound)
#define CRUBIT_LIFETIME_BOUND [[clang::lifetimebound]]
#elif __has_attribute(lifetimebound)
#define CRUBIT_LIFETIME_BOUND __attribute__((lifetimebound))
#else
#define CRUBIT_LIFETIME_BOUND
#endif
#define crubit_nonnull _Nonnull
#define crubit_nullable _Nullable
#else
#define CRUBIT_TRIVIAL_ABI
#define CRUBIT_VIEW
#define CRUBIT_LIFETIME_BOUND
#define crubit_nonnull
#define crubit_nullable
#endif

#if defined(_MSC_VER) && !defined(__clang__)
#define CRUBIT_UNREACHABLE() __assume(false)
#else
#define CRUBIT_UNREACHABLE() __builtin_unreachable()
#endif

// Style waiver granted in crubit.rs-style-waiver-attribute-annotate
#if defined(__clang__) && __has_cpp_attribute(clang::annotate) && \
    __has_cpp_attribute(clang::annotate_type)
#define CRUBIT_INTERNAL_ANNOTATE(...) [[clang::annotate(__VA_ARGS__)]]
#define CRUBIT_INTERNAL_ANNOTATE_TYPE(...) [[clang::annotate_type(__VA_ARGS__)]]
#else
#define CRUBIT_INTERNAL_ANNOTATE(...)
#define CRUBIT_INTERNAL_ANNOTATE_TYPE(...)
#endif

#define CRUBIT_INTERNAL_EXPAND_AND_CONCAT_IMPL(a, b) a##b
#define CRUBIT_INTERNAL_EXPAND_AND_CONCAT(a, b) \
  CRUBIT_INTERNAL_EXPAND_AND_CONCAT_IMPL(a, b)

namespace crubit::rust_type {
// Helper for CRUBIT_INTERNAL_RUST_TYPE. This type should never be used
// directly.
template <typename...>
struct Args {};

// Helper for CRUBIT_INTERNAL_RUST_TYPE. Instantiations of this type can be used
// to represent const-generic arguments in Rust.
//
// Example:
//
// ```cpp
// template <typename T, int N>
// struct CRUBIT_INTERNAL_RUST_TYPE("MyType", T, crubit::rust_type::Const<N>)
// MyType {};
// ```
//
// `MyType<T, 123>` in C++ will then be mapped to `MyType<T, 123>` in Rust.
template <auto>
struct Const {};
}  // namespace crubit::rust_type

// Unsafe: disables bindings, and reinterprets all uses of this type as `t`.
//
// This attribute completely disables automated bindings for the type which it
// appertains to. All uses of that type are replaced with uses of `t`, which
// must be a Rust type which exists and is guaranteed to be available by that
// name.
//
// This can be applied to a struct, class, or enum.
//
// If CRUBIT_INTERNAL_SAME_ABI is also specified, then the Rust
// type will be assumed to be exactly C ABI compatible (not just of identical
// layout) with `t`.
//
// For example, this C++ header:
//
// ```c++
// struct
//   CRUBIT_INTERNAL_RUST_TYPE("char")
//   CRUBIT_INTERNAL_SAME_ABI
//   CharT {
//     std::uint32_t c;
// };
// CharT foo() { return {0};}
// ```
//
// Becomes this Rust interface:
//
// ```rust
// pub fn foo() -> char;  // returns '\0'
// ```
//
// How to spell `t`:
//
// A path in `t` is spelled from the point of view of the Rust crate that
// corresponds to the Bazel target defining the annotated C++ declaration (when
// that target is the one bindings are being generated for, this is the crate
// being generated):
//
//   * `::some_crate::path::To::Type` names a crate explicitly: the first path
//     segment is a crate name, and the path is used exactly as spelled.
//   * `crate::path::To::Type` refers to that crate itself. Use this spelling
//     for items defined in the same target as the annotated declaration,
//     rather than naming that target's crate, so that the path keeps working
//     when the crate is renamed, e.g. by crate name mangling
//     (`use_label_encoded_names_for_deps`).
//   * `path::To::Type` is relative to the root of that same crate, i.e. it is
//     equivalent to `crate::path::To::Type`. Its first path segment names a
//     module, not a crate.
//
// SAFETY:
//   If the type is not layout-compatible with `t`, the behavior is undefined.
#define CRUBIT_INTERNAL_RUST_TYPE(t, ...)                  \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_rust_type", t, \
                           crubit::rust_type::Args<__VA_ARGS__>())

// Unsafe: forces a type to be treated as C abi compatible with its rust
// equivalent.
//
// This only has any effect when used in combination with
// CRUBIT_INTERNAL_RUST_TYPE. For all other types, C ABI compatibility can be
// inferred automatically, and overriding the decision is always a bug.
//
// SAFETY:
//   If the type is not ABI-compatible with its Rust equivalent, the behavior is
//   undefined.
#define CRUBIT_INTERNAL_SAME_ABI \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_same_abi")

// (Optional) The target that owns the Rust type for CRUBIT_INTERNAL_RUST_TYPE.
// Format: "//package:target" or a short-form like "@abseil-cpp//absl/status".
//
// This is only meaningful for a Rust type spelled as an explicit
// `::some_crate::path::To::Type` path (see "How to spell `t`" above): when
// crate name mangling is enabled (e.g. `use_label_encoded_names_for_deps`) and
// `some_crate` matches the target name of the hint, `some_crate` is replaced
// with the mangled crate name of the hint target. It is required when the C++
// type's owning target differs from the target providing its Rust bindings
// (e.g. `absl::StatusOr` in `@abseil-cpp//absl/status:statusor`, whose Rust
// type is provided by `@abseil-cpp//absl/status:status`); otherwise the
// owning target is inferred.
//
// The other two spellings need no hint: their crate is already the one
// corresponding to the target defining the annotated declaration, and Crubit
// spells it with the mangled crate name where necessary. In particular, a
// relative path is *not* rewritten, since its first segment names a module
// rather than a crate.
#define CRUBIT_INTERNAL_RUST_TYPE_LABEL_HINT(label_hint) \
  CRUBIT_INTERNAL_ANNOTATE("crubit_internal_rust_type_label_hint", label_hint)

#define CRUBIT_INTERNAL_RUST_TYPE_WITH_HINT(t, label_hint, ...) \
  CRUBIT_INTERNAL_RUST_TYPE(t, __VA_ARGS__)                     \
  CRUBIT_INTERNAL_RUST_TYPE_LABEL_HINT(label_hint)

#endif  // CRUBIT_SUPPORT_ANNOTATIONS_INTERNAL_H_
