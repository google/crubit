// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:string_view

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/string_view.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void
__rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(
    ::std::__u::string_view* s) {
  string_view_sink(std::move(*s));
}

static_assert((void (*)(::std::__u::string_view)) & ::string_view_sink);

extern "C" void
__rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(
    ::std::__u::string_view* __return, ::std::__u::string_view* s) {
  new (__return) auto(string_view_return(std::move(*s)));
}

static_assert((::std::__u::string_view (*)(::std::__u::string_view)) &
              ::string_view_return);

extern "C" void
__rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(
    ::std::__u::string_view* __return, ::std::__u::string_view* a,
    ::std::__u::string_view* b) {
  new (__return) auto(
      ambiguous_string_view_return(std::move(*a), std::move(*b)));
}

static_assert((::std::__u::string_view (*)(::std::__u::string_view,
                                           ::std::__u::string_view)) &
              ::ambiguous_string_view_return);

extern "C" void
__rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(
    ::std::__u::string_view* x) {
  explicit_lifetime_string_view(std::move(*x));
}

static_assert((void (*)(::std::__u::string_view)) &
              ::explicit_lifetime_string_view);

extern "C" void
__rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(
    ::std::__u::string_view* __return, ::std::__u::string_view* x,
    ::std::__u::string_view* y) {
  new (__return) auto(
      unambiguous_string_view_return_annotated(std::move(*x), std::move(*y)));
}

static_assert((::std::__u::string_view (*)(::std::__u::string_view,
                                           ::std::__u::string_view)) &
              ::unambiguous_string_view_return_annotated);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>) ==
    16);
static_assert(
    alignof(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>) ==
    8);

extern "C" void
__rust_thunk__312e1445__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1Ev(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__3debf465__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1EPKDi(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    char32_t const* __s) {
  crubit::construct_at(__this, __s);
}

extern "C" char32_t const*
__rust_thunk__fe39e9b3__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5beginEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->begin();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::begin);

extern "C" char32_t const*
__rust_thunk__f4155dc5__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE3endEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->end();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::end);

extern "C" char32_t const*
__rust_thunk__c6d7d2bf__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6cbeginEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->cbegin();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::cbegin);

extern "C" char32_t const*
__rust_thunk__8f3a9bdb__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4cendEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->cend();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::cend);

extern "C" void
__rust_thunk__6812a118__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6rbeginEv(
    class std::reverse_iterator<const char32_t*>* __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class std::reverse_iterator<const char32_t*> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::rbegin);

extern "C" void
__rust_thunk__fca51d4a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4rendEv(
    class std::reverse_iterator<const char32_t*>* __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class std::reverse_iterator<const char32_t*> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::rend);

extern "C" void
__rust_thunk__bd12e34d__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE7crbeginEv(
    class std::reverse_iterator<const char32_t*>* __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<const char32_t*> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::crbegin);

extern "C" void
__rust_thunk__8982b159__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5crendEv(
    class std::reverse_iterator<const char32_t*>* __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<const char32_t*> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::crend);

extern "C" size_t
__rust_thunk__f154408a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4sizeEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->size();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::size);

extern "C" size_t
__rust_thunk__dd0ece79__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6lengthEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->length();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::length);

extern "C" size_t
__rust_thunk__32e97d7f__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE8max_sizeEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->max_size();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::max_size);

extern "C" bool
__rust_thunk__6ef60d4e__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5emptyEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::empty);

extern "C" char32_t const*
__rust_thunk__3e6d3da1__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEixEm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char32_t const& (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t) const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::operator[]);

extern "C" char32_t const*
__rust_thunk__81d2effd__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE2atEm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this,
    size_t __pos) {
  return std::addressof(__this->at(__pos));
}

static_assert(
    (char32_t const& (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t) const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::at);

extern "C" char32_t const*
__rust_thunk__d11869b0__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5frontEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return std::addressof(__this->front());
}

static_assert(
    (char32_t const& (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::front);

extern "C" char32_t const*
__rust_thunk__fadaf858__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4backEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return std::addressof(__this->back());
}

static_assert(
    (char32_t const& (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::back);

extern "C" char32_t const*
__rust_thunk__b6e5b563__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4dataEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->data();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::data);

extern "C" void
__rust_thunk__569f834a__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_prefixEm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    size_t __n) {
  __this->remove_prefix(__n);
}

static_assert(
    (void (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char32_t,
                             std::char_traits<char32_t>>::remove_prefix);

extern "C" void
__rust_thunk__6036761d__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_suffixEm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    size_t __n) {
  __this->remove_suffix(__n);
}

static_assert(
    (void (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char32_t,
                             std::char_traits<char32_t>>::remove_suffix);

extern "C" void
__rust_thunk__2df55f0d__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4swapERS3_(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>*
        __other) {
  __this->swap(*__other);
}

static_assert(
    (void (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>&)) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::swap);

extern "C" size_t
__rust_thunk__7524d8bf__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4copyEPDimm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this,
    char32_t* __s, size_t __n, size_t __pos) {
  return __this->copy(__s, __n, __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        char32_t*, size_t, size_t) const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::copy);

extern "C" void
__rust_thunk__638f0c15__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6substrEmm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>*
        __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert(
    (class std::basic_string_view<char32_t, std::char_traits<char32_t>> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t, size_t) const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::substr);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<char16_t, std::char_traits<char16_t>>) ==
    16);
static_assert(
    alignof(
        class std::basic_string_view<char16_t, std::char_traits<char16_t>>) ==
    8);

extern "C" void
__rust_thunk__312e1445__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1Ev(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__3debf465__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1EPKDs(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    char16_t const* __s) {
  crubit::construct_at(__this, __s);
}

extern "C" char16_t const*
__rust_thunk__fe39e9b3__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5beginEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->begin();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::begin);

extern "C" char16_t const*
__rust_thunk__f4155dc5__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE3endEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->end();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::end);

extern "C" char16_t const*
__rust_thunk__c6d7d2bf__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6cbeginEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->cbegin();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::cbegin);

extern "C" char16_t const*
__rust_thunk__8f3a9bdb__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4cendEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->cend();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::cend);

extern "C" void
__rust_thunk__6812a118__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6rbeginEv(
    class std::reverse_iterator<const char16_t*>* __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class std::reverse_iterator<const char16_t*> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::rbegin);

extern "C" void
__rust_thunk__fca51d4a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4rendEv(
    class std::reverse_iterator<const char16_t*>* __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class std::reverse_iterator<const char16_t*> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::rend);

extern "C" void
__rust_thunk__bd12e34d__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE7crbeginEv(
    class std::reverse_iterator<const char16_t*>* __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<const char16_t*> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::crbegin);

extern "C" void
__rust_thunk__8982b159__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5crendEv(
    class std::reverse_iterator<const char16_t*>* __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<const char16_t*> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::crend);

extern "C" size_t
__rust_thunk__f154408a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4sizeEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->size();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::size);

extern "C" size_t
__rust_thunk__dd0ece79__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6lengthEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->length();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::length);

extern "C" size_t
__rust_thunk__32e97d7f__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE8max_sizeEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->max_size();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::max_size);

extern "C" bool
__rust_thunk__6ef60d4e__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5emptyEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::empty);

extern "C" char16_t const*
__rust_thunk__3e6d3da1__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEixEm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char16_t const& (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t) const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::operator[]);

extern "C" char16_t const*
__rust_thunk__81d2effd__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE2atEm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this,
    size_t __pos) {
  return std::addressof(__this->at(__pos));
}

static_assert(
    (char16_t const& (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t) const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::at);

extern "C" char16_t const*
__rust_thunk__d11869b0__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5frontEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return std::addressof(__this->front());
}

static_assert(
    (char16_t const& (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::front);

extern "C" char16_t const*
__rust_thunk__fadaf858__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4backEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return std::addressof(__this->back());
}

static_assert(
    (char16_t const& (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::back);

extern "C" char16_t const*
__rust_thunk__b6e5b563__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4dataEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->data();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::data);

extern "C" void
__rust_thunk__569f834a__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_prefixEm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    size_t __n) {
  __this->remove_prefix(__n);
}

static_assert(
    (void (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char16_t,
                             std::char_traits<char16_t>>::remove_prefix);

extern "C" void
__rust_thunk__6036761d__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_suffixEm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    size_t __n) {
  __this->remove_suffix(__n);
}

static_assert(
    (void (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char16_t,
                             std::char_traits<char16_t>>::remove_suffix);

extern "C" void
__rust_thunk__2df55f0d__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4swapERS3_(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>*
        __other) {
  __this->swap(*__other);
}

static_assert(
    (void (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        class std::basic_string_view<char16_t, std::char_traits<char16_t>>&)) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::swap);

extern "C" size_t
__rust_thunk__7524d8bf__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4copyEPDsmm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this,
    char16_t* __s, size_t __n, size_t __pos) {
  return __this->copy(__s, __n, __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        char16_t*, size_t, size_t) const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::copy);

extern "C" void
__rust_thunk__638f0c15__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6substrEmm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>*
        __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert(
    (class std::basic_string_view<char16_t, std::char_traits<char16_t>> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t, size_t) const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::substr);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>) ==
    16);
static_assert(
    alignof(class std::basic_string_view<char8_t, std::char_traits<char8_t>>) ==
    8);

extern "C" void
__rust_thunk__312e1445__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEC1Ev(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6812a118__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6rbeginEv(
    class std::reverse_iterator<const char8_t*>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class std::reverse_iterator<const char8_t*> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::rbegin);

extern "C" void
__rust_thunk__fca51d4a__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4rendEv(
    class std::reverse_iterator<const char8_t*>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class std::reverse_iterator<const char8_t*> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::rend);

extern "C" void
__rust_thunk__bd12e34d__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE7crbeginEv(
    class std::reverse_iterator<const char8_t*>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<const char8_t*> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::crbegin);

extern "C" void
__rust_thunk__8982b159__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5crendEv(
    class std::reverse_iterator<const char8_t*>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<const char8_t*> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::crend);

extern "C" size_t
__rust_thunk__f154408a__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4sizeEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::size);

extern "C" size_t
__rust_thunk__dd0ece79__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6lengthEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->length();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::length);

extern "C" size_t
__rust_thunk__32e97d7f__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE8max_sizeEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->max_size();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::max_size);

extern "C" bool
__rust_thunk__6ef60d4e__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5emptyEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::empty);

extern "C" void
__rust_thunk__569f834a__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_prefixEm(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this,
    size_t __n) {
  __this->remove_prefix(__n);
}

static_assert(
    (void (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::remove_prefix);

extern "C" void
__rust_thunk__6036761d__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_suffixEm(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this,
    size_t __n) {
  __this->remove_suffix(__n);
}

static_assert(
    (void (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::remove_suffix);

extern "C" void
__rust_thunk__2df55f0d__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4swapERS3_(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __other) {
  __this->swap(*__other);
}

static_assert(
    (void (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>&)) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::swap);

extern "C" void
__rust_thunk__638f0c15__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6substrEmm(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert(
    (class std::basic_string_view<char8_t, std::char_traits<char8_t>> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(size_t,
                                                                         size_t)
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::substr);

extern "C" size_t
__rust_thunk__87ddb218__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4findES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::find);

extern "C" size_t
__rust_thunk__4f57f330__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5rfindES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->rfind(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::rfind);

extern "C" size_t
__rust_thunk__6de94bc4__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13find_first_ofES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find_first_of(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::find_first_of);

extern "C" size_t
__rust_thunk__455614d0__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE12find_last_ofES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find_last_of(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::find_last_of);

extern "C" size_t
__rust_thunk__4256accd__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE17find_first_not_ofES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find_first_not_of(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::find_first_not_of);

extern "C" size_t
__rust_thunk__0dc198fc__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE16find_last_not_ofES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find_last_not_of(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::find_last_not_of);

extern "C" bool
__rust_thunk__5381cdc4__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE11starts_withES3_(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s) {
  return __this->starts_with(std::move(*__s));
}

static_assert(
    (bool (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>)
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::starts_with);

extern "C" bool
__rust_thunk__a5f3bd39__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE9ends_withES3_(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s) {
  return __this->ends_with(std::move(*__s));
}

static_assert(
    (bool (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>)
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::ends_with);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char32_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char32_t*>) == 8);

extern "C" void __rust_thunk__046f2d88__ZNSt3__u16reverse_iteratorIPKDiEC1Ev(
    class std::reverse_iterator<const char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9d7a113a__ZNSt3__u16reverse_iteratorIPKDiEC1ES2_(
    class std::reverse_iterator<const char32_t*>* __this, char32_t const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char32_t const*
__rust_thunk__74bf6f6f__ZNKSt3__u16reverse_iteratorIPKDiE4baseEv(
    class std::reverse_iterator<const char32_t*> const* __this) {
  return __this->base();
}

static_assert((char32_t const* (::std::reverse_iterator<const char32_t*>::*)()
                   const) &
              ::std::reverse_iterator<const char32_t*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char16_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char16_t*>) == 8);

extern "C" void __rust_thunk__046f2d88__ZNSt3__u16reverse_iteratorIPKDsEC1Ev(
    class std::reverse_iterator<const char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9d7a113a__ZNSt3__u16reverse_iteratorIPKDsEC1ES2_(
    class std::reverse_iterator<const char16_t*>* __this, char16_t const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char16_t const*
__rust_thunk__74bf6f6f__ZNKSt3__u16reverse_iteratorIPKDsE4baseEv(
    class std::reverse_iterator<const char16_t*> const* __this) {
  return __this->base();
}

static_assert((char16_t const* (::std::reverse_iterator<const char16_t*>::*)()
                   const) &
              ::std::reverse_iterator<const char16_t*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char8_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char8_t*>) == 8);

extern "C" void __rust_thunk__046f2d88__ZNSt3__u16reverse_iteratorIPKDuEC1Ev(
    class std::reverse_iterator<const char8_t*>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char*>) == 8);

extern "C" void __rust_thunk__046f2d88__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
    class std::reverse_iterator<const char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9d7a113a__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
    class std::reverse_iterator<const char*>* __this, char const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char const*
__rust_thunk__74bf6f6f__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(
    class std::reverse_iterator<const char*> const* __this) {
  return __this->base();
}

static_assert((char const* (::std::reverse_iterator<const char*>::*)() const) &
              ::std::reverse_iterator<const char*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const wchar_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const wchar_t*>) == 8);

extern "C" void __rust_thunk__046f2d88__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
    class std::reverse_iterator<const wchar_t*>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
