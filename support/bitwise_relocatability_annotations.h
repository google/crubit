// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/bitwise_relocatability_annotations.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_BITWISE_RELOCATABILITY_ANNOTATIONS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_BITWISE_RELOCATABILITY_ANNOTATIONS_H_

// Bitwise-relocatability (Rust-movability) annotations. See
// crubit.rs/cpp/cookbook#bitwise_relocatable.
// `support/annotations.h` includes this header. SWIG users: see the comment at
// the top of `support/annotations.h`.
//
// Keep this header small: it is included by widely used C++ headers (e.g.
// Chromium's `url/gurl.h`).

#include "support/annotate_internal.h"  // IWYU pragma: export

// Asserts that a C++ record type is bitwise relocatable, making it Rust-movable
// (Unpin) in the generated bindings even if it has a user-provided move
// constructor or destructor.
//
// By applying this annotation, the author guarantees that Crubit's automated
// checks are sound for this type. Crubit verifies that every base class and
// non-static data member is also bitwise relocatable. However, the author must
// verify the following checklist, which Crubit cannot check:
// (a) The move constructor and destructor are equivalent to `memcpy` plus
//     forgetting the old object (no self-pointers, no registration of `this` in
//     side tables, no address-keyed caches).
// (b) Nothing poisons bytes of `*this` under ASan.
// (c) No address-dependent pointer encoding is used (`__ptrauth`, tagged PFP).
// (d) The properties hold on every standard library the type is built against
//     (or use `CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX`).
//
// If a member is not bitwise relocatable, bindings generation for the owning
// target fails with a hard error.
#define CRUBIT_UNSAFE_BITWISE_RELOCATABLE \
  CRUBIT_INTERNAL_ANNOTATE("crubit_unsafe_bitwise_relocatable")

// Marks a C++ record type as never bitwise relocatable, making it stay
// pinned (!Unpin) in the generated Rust bindings even if it is trivially
// copyable or annotated with [[clang::trivial_abi]].
//
// This is useful for types whose address is registered elsewhere or which must
// not be memcpy-moved despite having trivial special members. Applying this
// along with CRUBIT_UNSAFE_BITWISE_RELOCATABLE is a hard error.
#define CRUBIT_NEVER_BITWISE_RELOCATABLE \
  CRUBIT_INTERNAL_ANNOTATE("crubit_never_bitwise_relocatable")

// Like `CRUBIT_UNSAFE_BITWISE_RELOCATABLE`, but if a member is not bitwise
// relocatable, the generator silently leaves the type pinned instead of
// failing. Useful for class templates whose bitwise relocatability depends on
// their template arguments.
#define CRUBIT_UNSAFE_BITWISE_RELOCATABLE_IF_ELIGIBLE \
  CRUBIT_INTERNAL_ANNOTATE("crubit_unsafe_bitwise_relocatable_if_eligible")

// Treats a single non-static data member as bitwise relocatable, bypassing the
// check on its type. Useful for offset pointers and similar types where the
// author knows bitwise relocation is safe despite the member's C++ type.
#define CRUBIT_UNSAFE_BITWISE_RELOCATABLE_FIELD \
  CRUBIT_INTERNAL_ANNOTATE("crubit_unsafe_bitwise_relocatable_field")

// Treats this `std::string` (`std::basic_string<...>`) field as
// bitwise-relocatable. Valid only when the whole program is built against
// libc++ (basic_string holds no self-pointer there, unlike libstdc++/MSVC STL).
// `rs_bindings_from_cc` fails with an error if the field's type is not
// `std::basic_string` (arrays of it are fine) or if the translation unit is not
// compiled against libc++ (`_LIBCPP_VERSION`). The binding's doc comment
// records the libc++ dependence.
// Note the known gap that `std::optional<std::string>` members are not covered.
#define CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX \
  CRUBIT_INTERNAL_ANNOTATE(                                  \
      "crubit_bitwise_relocatable_std_string_only_on_libcxx")

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_BITWISE_RELOCATABILITY_ANNOTATIONS_H_
