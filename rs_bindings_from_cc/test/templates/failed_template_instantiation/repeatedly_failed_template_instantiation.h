// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_REPEATEDLY_FAILED_TEMPLATE_INSTANTIATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_REPEATEDLY_FAILED_TEMPLATE_INSTANTIATION_H_

// Clang reports the errors of a failed template instantiation only the first
// time the specialization is completed. Afterwards the specialization is left
// with a complete, but ill-formed, definition, and completing it again
// silently succeeds.
//
// Crubit instantiates class template specializations eagerly, so it must not
// let the outcome of that instantiation depend on how often -- and in which
// order -- the specializations happen to be encountered. Otherwise the second
// occurrence gets bindings whose generated `..._rs_api_impl.cc` does not
// compile, because there the very same instantiation is attempted for the
// first time again.

class Incomplete;

template <typename T>
struct NeedsCompleteT final {
  static_assert(sizeof(T) > 0, "T must be complete");
  T* ptr;
};

template <typename T>
struct InstantiableTemplate final {
  T* ptr;
};

using Uninstantiable = NeedsCompleteT<Incomplete>;

// The same non-instantiable specialization, named twice.
inline void TakesUninstantiable1(Uninstantiable*) {}
inline void TakesUninstantiable2(Uninstantiable*) {}

// A specialization that is instantiable only because `Uninstantiable` has
// already been (unsuccessfully) instantiated above.
inline void TakesNestedUninstantiable(NeedsCompleteT<Uninstantiable>*) {}

// Instantiable specializations keep getting bindings, also when named
// repeatedly.
inline void TakesInstantiable1(InstantiableTemplate<Incomplete>*) {}
inline void TakesInstantiable2(InstantiableTemplate<Incomplete>*) {}

// A *pointer* to a non-instantiable specialization must be rejected too.
//
// It is tempting to assume that `Foo<Bad*>` is fine because a pointer to an
// incomplete type is itself complete. It is not: the thunk generated for this
// function instantiates `NeedsCompleteT<Incomplete>` anyway, and the resulting
// `..._rs_api_impl.cc` fails to compile with "invalid application of 'sizeof'
// to an incomplete type".
//
// So this is a build-level regression test: if the instantiability check stops
// following pointer and reference arguments, *this target stops building*.
inline void TakesPointerToUninstantiable(
    InstantiableTemplate<Uninstantiable*>*) {}

// The *outer* specialization is named before the inner one. Completing it
// fails `NeedsCompleteT<IncompleteSeenLate>` for the first time, so Clang's
// errors are attributed to the outer attempt. The inner specialization must
// still be recorded as failed, or it gets bindings when named afterwards.
//
// This only works because the template arguments are actively checked, not
// merely looked up among previously recorded failures.
class IncompleteSeenLate;
inline void TakesNestedFirst(
    NeedsCompleteT<NeedsCompleteT<IncompleteSeenLate>>*) {}
inline void TakesInnerLater(NeedsCompleteT<IncompleteSeenLate>*) {}

// Mirrors the std::variant failure from maps/gmm/snapping more closely: the
// instantiation fails in a *base class* (like std::variant's `_Variant_base`),
// and the later check is a concept check through a pointer (like
// std::reverse_iterator's `bidirectional_iterator` check).
//
// These are deliberately not inline: a function body would force the broken
// types to be completed while parsing this header.
template <typename T>
struct RequiresCompleteBase {
  int x[sizeof(T)];
};

template <typename T>
struct HasFailedBase : RequiresCompleteBase<T> {};

using FailedBaseAlias = HasFailedBase<Incomplete>;

template <typename Ptr>
struct ChecksPointerIncrement {
  static_assert(requires(Ptr p) { ++p; });
};

void TakesConceptCheckOnFailedBase(
    ChecksPointerIncrement<const FailedBaseAlias*>);
void TakesFailedBase(FailedBaseAlias);

// The failed instantiation is first observed outside of the instantiability
// check for specializations: here, while Crubit defines `HoldsBox`'s implicit
// copy constructor, which instantiates `Box<IncompleteInBody>::Box(const
// Box&)`, which in turn instantiates `NeedsCompleteT<IncompleteInBody>` for the
// first time. Both failures must be recorded, or `TakesFailedInImplicitCopy`
// and `Box<IncompleteInBody>`'s copy constructor get bindings afterwards.
class IncompleteInBody;

// Instantiating the class `Box<IncompleteInBody>` is fine. Only the body of its
// copy constructor needs `NeedsCompleteT<IncompleteInBody>` to be complete.
template <typename T>
struct Box final {
  Box() = default;
  Box(const Box& other) : p(other.p) { (void)sizeof(NeedsCompleteT<T>); }
  T* p = nullptr;
};

struct HoldsBox final {
  Box<IncompleteInBody> box;
};

// Named for the first time only after the failed instantiation above.
inline void TakesFailedInImplicitCopy(NeedsCompleteT<IncompleteInBody>*) {}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_REPEATEDLY_FAILED_TEMPLATE_INSTANTIATION_H_
