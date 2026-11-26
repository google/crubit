// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/absl_flat_hash_map:isolated

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/absl_flat_hash_map/isolated.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

extern "C" void
__rust_thunk___ZN6crubit4test9GetAtFiveEN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS3_11char_traitsIcEEEELi42EEE(
    ::std::__u::string_view* __return,
    crubit::test::MapWithStringViewValue* map) {
  new (__return) auto(crubit::test::GetAtFive(crubit::UnsafeTakeValue(map)));
}

static_assert(
    (::std::__u::string_view (*)(crubit::test::MapWithStringViewValue)) &
    ::crubit::test::GetAtFive);

extern "C" ::std::__u::string_view const*
__rust_thunk___ZN6crubit4test12GetRefAtFiveERKN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS3_11char_traitsIcEEEELi42EEE(
    crubit::test::MapWithStringViewValue const* map) {
  return std::addressof(crubit::test::GetRefAtFive(*map));
}

static_assert((::std::__u::string_view const& (*)(
                  crubit::test::MapWithStringViewValue const&)) &
              ::crubit::test::GetRefAtFive);

extern "C" void
__rust_thunk___ZN6crubit4test13GetSpanAtFiveEN4absl13flat_hash_mapIiNS1_4SpanIKiEELi42EEE(
    class absl::Span<const int>* __return,
    crubit::test::MapWithSpanValue* map) {
  new (__return) auto(
      crubit::test::GetSpanAtFive(crubit::UnsafeTakeValue(map)));
}

static_assert(
    (class absl::Span<const int> (*)(crubit::test::MapWithSpanValue)) &
    ::crubit::test::GetSpanAtFive);

extern "C" ::rs::SliceRef<const int>
__rust_thunk___ZN6crubit4test17GetSliceRefAtFiveEN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE(
    crubit::test::MapWithSliceRefValue* map) {
  return crubit::test::GetSliceRefAtFive(crubit::UnsafeTakeValue(map));
}

static_assert(
    (::rs::SliceRef<const int> (*)(crubit::test::MapWithSliceRefValue)) &
    ::crubit::test::GetSliceRefAtFive);

static_assert(sizeof(class crubit::test::NoDestructor) == 1);
static_assert(alignof(class crubit::test::NoDestructor) == 1);

extern "C" class crubit::test::NoDestructor*
__rust_thunk___ZN6crubit4test12NoDestructoraSERKS1_(
    class crubit::test::NoDestructor* __this,
    class crubit::test::NoDestructor const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((class crubit::test::NoDestructor &
               (::crubit::test::NoDestructor::*)(
                   class crubit::test::NoDestructor const&)) &
              ::crubit::test::NoDestructor::operator=);

static_assert(sizeof(class crubit::test::NoDelete) == 1);
static_assert(alignof(class crubit::test::NoDelete) == 1);

extern "C" void __rust_thunk___ZN6crubit4test8NoDeleteC1Ev(
    class crubit::test::NoDelete* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class crubit::test::NoMoveNoCopy) == 1);
static_assert(alignof(class crubit::test::NoMoveNoCopy) == 1);

extern "C" void __rust_thunk___ZN6crubit4test12NoMoveNoCopyC1Ev(
    class crubit::test::NoMoveNoCopy* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class crubit::test::CopyOnly) == 1);
static_assert(alignof(class crubit::test::CopyOnly) == 1);

extern "C" void __rust_thunk___ZN6crubit4test8CopyOnlyC1Ev(
    class crubit::test::CopyOnly* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<class std::__wrap_iter<const int*>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<class std::__wrap_iter<const int*>>) ==
    8);

extern "C" void
__rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__fdf38745__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __this,
    class std::__wrap_iter<const int*>* __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

extern "C" void
__rust_thunk__6ecbc2e1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEE4baseEv(
    class std::__wrap_iter<const int*>* __return,
    class std::reverse_iterator<class std::__wrap_iter<const int*>> const*
        __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__wrap_iter<const int*> (
        ::std::reverse_iterator<class std::__wrap_iter<const int*>>::*)()
         const) &
    ::std::reverse_iterator<class std::__wrap_iter<const int*>>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char*>) == 8);

extern "C" void __rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
    class std::reverse_iterator<const char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__fdf38745__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
    class std::reverse_iterator<const char*>* __this, char const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char const*
__rust_thunk__6ecbc2e1__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(
    class std::reverse_iterator<const char*> const* __this) {
  return __this->base();
}

static_assert((char const* (::std::reverse_iterator<const char*>::*)() const) &
              ::std::reverse_iterator<const char*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const int*>) == 8);
static_assert(alignof(class std::reverse_iterator<const int*>) == 8);

extern "C" void __rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorIPKiEC1Ev(
    class std::reverse_iterator<const int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__fdf38745__ZNSt3__u16reverse_iteratorIPKiEC1ES2_(
    class std::reverse_iterator<const int*>* __this, int const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" int const*
__rust_thunk__6ecbc2e1__ZNKSt3__u16reverse_iteratorIPKiE4baseEv(
    class std::reverse_iterator<const int*> const* __this) {
  return __this->base();
}

static_assert((int const* (::std::reverse_iterator<const int*>::*)() const) &
              ::std::reverse_iterator<const int*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const wchar_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const wchar_t*>) == 8);

extern "C" void __rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
    class std::reverse_iterator<const wchar_t*>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const int*>) == 8);
static_assert(alignof(class std::__wrap_iter<const int*>) == 8);

extern "C" void __rust_thunk__83c617d2__ZNSt3__u11__wrap_iterIPKiEC1Ev(
    class std::__wrap_iter<const int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__45d7cb71__ZNKSt3__u11__wrap_iterIPKiEplEl(
    class std::__wrap_iter<const int*>* __return,
    class std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::__wrap_iter<const int*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const int*>::operator+);

extern "C" class std::__wrap_iter<const int*>*
__rust_thunk__c15f65fd__ZNSt3__u11__wrap_iterIPKiEpLEl(
    class std::__wrap_iter<const int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const int*> &
               (::std::__wrap_iter<const int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const int*>::operator+=);

extern "C" void __rust_thunk__ee08d685__ZNKSt3__u11__wrap_iterIPKiEmiEl(
    class std::__wrap_iter<const int*>* __return,
    class std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::__wrap_iter<const int*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const int*>::operator-);

extern "C" class std::__wrap_iter<const int*>*
__rust_thunk__55a37ce5__ZNSt3__u11__wrap_iterIPKiEmIEl(
    class std::__wrap_iter<const int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const int*> &
               (::std::__wrap_iter<const int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const int*>::operator-=);

extern "C" int const* __rust_thunk__f7b43e06__ZNKSt3__u11__wrap_iterIPKiEixEl(
    class std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((int const& (::std::__wrap_iter<const int*>::*)(ptrdiff_t)
                   const) &
              ::std::__wrap_iter<const int*>::operator[]);

static_assert(
    CRUBIT_SIZEOF(class std::span<const int, 18446744073709551615UL>) == 16);
static_assert(alignof(class std::span<const int, 18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__7fa98692__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
    class std::span<const int, 18446744073709551615UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__4e54b5b5__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm(
    class std::span<const int, 18446744073709551615UL>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert((class std::span<const int, 18446744073709551615UL> (
                  ::std::span<const int, 18446744073709551615UL>::*)(size_t)
                   const) &
              ::std::span<const int, 18446744073709551615UL>::first);

extern "C" void
__rust_thunk__0f590027__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm(
    class std::span<const int, 18446744073709551615UL>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert((class std::span<const int, 18446744073709551615UL> (
                  ::std::span<const int, 18446744073709551615UL>::*)(size_t)
                   const) &
              ::std::span<const int, 18446744073709551615UL>::last);

extern "C" void
__rust_thunk__022263c7__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm(
    class std::span<const int, 18446744073709551615UL>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this,
    size_t __offset, size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert((class std::span<const int, 18446744073709551615UL> (
                  ::std::span<const int, 18446744073709551615UL>::*)(size_t,
                                                                     size_t)
                   const) &
              ::std::span<const int, 18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__ea271768__ZNKSt3__u4spanIKiLm18446744073709551615EE4sizeEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::size);

extern "C" size_t
__rust_thunk__7475e8e4__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert((size_t (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__0e958be1__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::empty);

extern "C" int const*
__rust_thunk__b2c6c9fa__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm(
    class std::span<const int, 18446744073709551615UL> const* __this,
    size_t __idx) {
  return std::addressof(__this->operator[](__idx));
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)(
                  size_t) const) &
              ::std::span<const int, 18446744073709551615UL>::operator[]);

extern "C" int const*
__rust_thunk__e6dd0be6__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::front);

extern "C" int const*
__rust_thunk__4df28266__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::back);

extern "C" int const*
__rust_thunk__b265a8aa__ZNKSt3__u4spanIKiLm18446744073709551615EE4dataEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->data();
}

static_assert((int const* (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::data);

extern "C" void
__rust_thunk__626b734c__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv(
    class std::__wrap_iter<const int*>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__09c98b89__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv(
    class std::__wrap_iter<const int*>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::end);

extern "C" void
__rust_thunk__af6f680d__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<const int*>> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__e01cc11e__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<const int*>> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::rend);

static_assert(sizeof(class absl::flat_hash_map<class crubit::test::NoMoveNoCopy,
                                               int, 42>) == 1);
static_assert(
    alignof(
        class absl::flat_hash_map<class crubit::test::NoMoveNoCopy, int, 42>) ==
    1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEC1Ev(
    class absl::flat_hash_map<class crubit::test::NoMoveNoCopy, int, 42>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(class absl::flat_hash_map<struct crubit::test::Bridged, int, 42>) ==
    1);
static_assert(
    alignof(class absl::flat_hash_map<struct crubit::test::Bridged, int, 42>) ==
    1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEC1Ev(
    class absl::flat_hash_map<struct crubit::test::Bridged, int, 42>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>) ==
    1);
static_assert(
    alignof(class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>) ==
    1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEC1Ev(
    class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>* __this) {
  crubit::construct_at(__this);
}

extern "C" size_t
__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>*
        __this) {
  return __this->size();
}

extern "C" size_t
__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>*
        __this) {
  return __this->capacity();
}

extern "C" void
__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>* __this) {
  __this->clear();
}

extern "C" void
__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>* __this,
    size_t count) {
  __this->reserve(count);
}

extern "C" bool
__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>*
        __this,
    const class crubit::test::CopyOnly* key,
    const class crubit::test::CopyOnly** result_key, const int** result_value) {
  auto it = __this->find(*key);
  if (it == __this->end()) {
    return false;
  }
  *result_key = &it->first;
  *result_value = &it->second;
  return true;
}

extern "C" int*
__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>* __this,
    const class crubit::test::CopyOnly* key) {
  auto it = __this->find(*key);
  return it == __this->end() ? nullptr : &it->second;
}

extern "C" bool
__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>* __this,
    class crubit::test::CopyOnly* key, int* value, bool overwrite,
    const class crubit::test::CopyOnly** result_key, int** result_value) {
  auto it = __this->try_emplace(std::move(*key), std::move(*value));
  if (!it.second && overwrite) {
    std::destroy_at(&it.first->second);
    crubit::construct_at(&it.first->second, std::move(*value));
  }
  *result_key = &it.first->first;
  *result_value = &it.first->second;
  return it.second || overwrite;
}

extern "C" bool
__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<class crubit::test::CopyOnly, int, 42>* __this,
    const class crubit::test::CopyOnly* key) {
  return __this->erase(*key) != 0;
}

static_assert(
    sizeof(class absl::flat_hash_map<
           class std::basic_string_view<char, struct std::char_traits<char>>,
           int, 42>) == 1);
static_assert(
    alignof(class absl::flat_hash_map<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            int, 42>) == 1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEC1Ev(
    class absl::flat_hash_map<
        class std::basic_string_view<char, struct std::char_traits<char>>, int,
        42>* __this) {
  crubit::construct_at(__this);
}

extern "C" size_t
__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<
        class std::basic_string_view<char, struct std::char_traits<char>>, int,
        42>* __this) {
  return __this->size();
}

extern "C" size_t
__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<
        class std::basic_string_view<char, struct std::char_traits<char>>, int,
        42>* __this) {
  return __this->capacity();
}

extern "C" void
__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        class std::basic_string_view<char, struct std::char_traits<char>>, int,
        42>* __this) {
  __this->clear();
}

extern "C" void
__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        class std::basic_string_view<char, struct std::char_traits<char>>, int,
        42>* __this,
    size_t count) {
  __this->reserve(count);
}

extern "C" bool
__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<
        class std::basic_string_view<char, struct std::char_traits<char>>, int,
        42>* __this,
    const ::std::__u::string_view* key,
    const ::std::__u::string_view** result_key, const int** result_value) {
  auto it = __this->find(*key);
  if (it == __this->end()) {
    return false;
  }
  *result_key = &it->first;
  *result_value = &it->second;
  return true;
}

extern "C" int*
__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        class std::basic_string_view<char, struct std::char_traits<char>>, int,
        42>* __this,
    const ::std::__u::string_view* key) {
  auto it = __this->find(*key);
  return it == __this->end() ? nullptr : &it->second;
}

extern "C" bool
__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        class std::basic_string_view<char, struct std::char_traits<char>>, int,
        42>* __this,
    ::std::__u::string_view* key, int* value, bool overwrite,
    const ::std::__u::string_view** result_key, int** result_value) {
  auto it = __this->try_emplace(std::move(*key), std::move(*value));
  if (!it.second && overwrite) {
    std::destroy_at(&it.first->second);
    crubit::construct_at(&it.first->second, std::move(*value));
  }
  *result_key = &it.first->first;
  *result_value = &it.first->second;
  return it.second || overwrite;
}

extern "C" bool
__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        class std::basic_string_view<char, struct std::char_traits<char>>, int,
        42>* __this,
    const ::std::__u::string_view* key) {
  return __this->erase(*key) != 0;
}

static_assert(sizeof(class absl::flat_hash_map<int, class rs::StrRef, 42>) ==
              1);
static_assert(alignof(class absl::flat_hash_map<int, class rs::StrRef, 42>) ==
              1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN2rs6StrRefELi42EEC1Ev(
    class absl::flat_hash_map<int, class rs::StrRef, 42>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>) ==
    1);
static_assert(
    alignof(
        class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>) ==
    1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEC1Ev(
    class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>* __this) {
  crubit::construct_at(__this);
}

extern "C" size_t
__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>*
        __this) {
  return __this->size();
}

extern "C" size_t
__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>*
        __this) {
  return __this->capacity();
}

extern "C" void
__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>* __this) {
  __this->clear();
}

extern "C" void
__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>* __this,
    size_t count) {
  __this->reserve(count);
}

extern "C" bool
__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>*
        __this,
    const int* key, const int** result_key,
    const ::rs::SliceRef<const int>** result_value) {
  auto it = __this->find(*key);
  if (it == __this->end()) {
    return false;
  }
  *result_key = &it->first;
  *result_value = &it->second;
  return true;
}

extern "C" ::rs::SliceRef<const int>*
__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>* __this,
    const int* key) {
  auto it = __this->find(*key);
  return it == __this->end() ? nullptr : &it->second;
}

extern "C" bool
__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>* __this,
    int* key, ::rs::SliceRef<const int>* value, bool overwrite,
    const int** result_key, ::rs::SliceRef<const int>** result_value) {
  auto it = __this->try_emplace(std::move(*key), std::move(*value));
  if (!it.second && overwrite) {
    std::destroy_at(&it.first->second);
    crubit::construct_at(&it.first->second, std::move(*value));
  }
  *result_key = &it.first->first;
  *result_value = &it.first->second;
  return it.second || overwrite;
}

extern "C" bool
__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class rs::SliceRef<const int>, 42>* __this,
    const int* key) {
  return __this->erase(*key) != 0;
}

static_assert(
    sizeof(
        class absl::flat_hash_map<int, class crubit::test::NoMoveNoCopy, 42>) ==
    1);
static_assert(
    alignof(
        class absl::flat_hash_map<int, class crubit::test::NoMoveNoCopy, 42>) ==
    1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEC1Ev(
    class absl::flat_hash_map<int, class crubit::test::NoMoveNoCopy, 42>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(class absl::flat_hash_map<int, struct crubit::test::Bridged, 42>) ==
    1);
static_assert(
    alignof(class absl::flat_hash_map<int, struct crubit::test::Bridged, 42>) ==
    1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEC1Ev(
    class absl::flat_hash_map<int, struct crubit::test::Bridged, 42>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>) ==
    1);
static_assert(
    alignof(class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>) ==
    1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEC1Ev(
    class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>* __this) {
  crubit::construct_at(__this);
}

extern "C" size_t
__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>*
        __this) {
  return __this->size();
}

extern "C" size_t
__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>*
        __this) {
  return __this->capacity();
}

extern "C" void
__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>* __this) {
  __this->clear();
}

extern "C" void
__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>* __this,
    size_t count) {
  __this->reserve(count);
}

extern "C" bool
__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>*
        __this,
    const int* key, const int** result_key,
    const class crubit::test::CopyOnly** result_value) {
  auto it = __this->find(*key);
  if (it == __this->end()) {
    return false;
  }
  *result_key = &it->first;
  *result_value = &it->second;
  return true;
}

extern "C" class crubit::test::CopyOnly*
__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>* __this,
    const int* key) {
  auto it = __this->find(*key);
  return it == __this->end() ? nullptr : &it->second;
}

extern "C" bool
__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>* __this,
    int* key, class crubit::test::CopyOnly* value, bool overwrite,
    const int** result_key, class crubit::test::CopyOnly** result_value) {
  auto it = __this->try_emplace(std::move(*key), std::move(*value));
  if (!it.second && overwrite) {
    std::destroy_at(&it.first->second);
    crubit::construct_at(&it.first->second, std::move(*value));
  }
  *result_key = &it.first->first;
  *result_value = &it.first->second;
  return it.second || overwrite;
}

extern "C" bool
__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, class crubit::test::CopyOnly, 42>* __this,
    const int* key) {
  return __this->erase(*key) != 0;
}

static_assert(
    sizeof(class absl::flat_hash_map<int, class absl::Span<const int>, 42>) ==
    1);
static_assert(
    alignof(class absl::flat_hash_map<int, class absl::Span<const int>, 42>) ==
    1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEC1Ev(
    class absl::flat_hash_map<int, class absl::Span<const int>, 42>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(class absl::flat_hash_map<
           int,
           class std::basic_string_view<char, struct std::char_traits<char>>,
           42>) == 1);
static_assert(
    alignof(class absl::flat_hash_map<
            int,
            class std::basic_string_view<char, struct std::char_traits<char>>,
            42>) == 1);

extern "C" void
__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEC1Ev(
    class absl::flat_hash_map<
        int, class std::basic_string_view<char, struct std::char_traits<char>>,
        42>* __this) {
  crubit::construct_at(__this);
}

extern "C" size_t
__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<
        int, class std::basic_string_view<char, struct std::char_traits<char>>,
        42>* __this) {
  return __this->size();
}

extern "C" size_t
__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<
        int, class std::basic_string_view<char, struct std::char_traits<char>>,
        42>* __this) {
  return __this->capacity();
}

extern "C" void
__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        int, class std::basic_string_view<char, struct std::char_traits<char>>,
        42>* __this) {
  __this->clear();
}

extern "C" void
__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        int, class std::basic_string_view<char, struct std::char_traits<char>>,
        42>* __this,
    size_t count) {
  __this->reserve(count);
}

extern "C" bool
__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<
        int, class std::basic_string_view<char, struct std::char_traits<char>>,
        42>* __this,
    const int* key, const int** result_key,
    const ::std::__u::string_view** result_value) {
  auto it = __this->find(*key);
  if (it == __this->end()) {
    return false;
  }
  *result_key = &it->first;
  *result_value = &it->second;
  return true;
}

extern "C" ::std::__u::string_view*
__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        int, class std::basic_string_view<char, struct std::char_traits<char>>,
        42>* __this,
    const int* key) {
  auto it = __this->find(*key);
  return it == __this->end() ? nullptr : &it->second;
}

extern "C" bool
__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        int, class std::basic_string_view<char, struct std::char_traits<char>>,
        42>* __this,
    int* key, ::std::__u::string_view* value, bool overwrite,
    const int** result_key, ::std::__u::string_view** result_value) {
  auto it = __this->try_emplace(std::move(*key), std::move(*value));
  if (!it.second && overwrite) {
    std::destroy_at(&it.first->second);
    crubit::construct_at(&it.first->second, std::move(*value));
  }
  *result_key = &it.first->first;
  *result_value = &it.first->second;
  return it.second || overwrite;
}

extern "C" bool
__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<
        int, class std::basic_string_view<char, struct std::char_traits<char>>,
        42>* __this,
    const int* key) {
  return __this->erase(*key) != 0;
}

static_assert(sizeof(class absl::flat_hash_map<int, unsigned long, 42>) == 1);
static_assert(alignof(class absl::flat_hash_map<int, unsigned long, 42>) == 1);

extern "C" void __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIimLi42EEC1Ev(
    class absl::flat_hash_map<int, unsigned long, 42>* __this) {
  crubit::construct_at(__this);
}

extern "C" size_t
__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<int, unsigned long, 42>* __this) {
  return __this->size();
}

extern "C" size_t
__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<int, unsigned long, 42>* __this) {
  return __this->capacity();
}

extern "C" void
__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, unsigned long, 42>* __this) {
  __this->clear();
}

extern "C" void
__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, unsigned long, 42>* __this, size_t count) {
  __this->reserve(count);
}

extern "C" bool
__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    const class absl::flat_hash_map<int, unsigned long, 42>* __this,
    const int* key, const int** result_key,
    const unsigned long** result_value) {
  auto it = __this->find(*key);
  if (it == __this->end()) {
    return false;
  }
  *result_key = &it->first;
  *result_value = &it->second;
  return true;
}

extern "C" unsigned long*
__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, unsigned long, 42>* __this, const int* key) {
  auto it = __this->find(*key);
  return it == __this->end() ? nullptr : &it->second;
}

extern "C" bool
__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, unsigned long, 42>* __this, int* key,
    unsigned long* value, bool overwrite, const int** result_key,
    unsigned long** result_value) {
  auto it = __this->try_emplace(std::move(*key), std::move(*value));
  if (!it.second && overwrite) {
    std::destroy_at(&it.first->second);
    crubit::construct_at(&it.first->second, std::move(*value));
  }
  *result_key = &it.first->first;
  *result_value = &it.first->second;
  return it.second || overwrite;
}

extern "C" bool
__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
    class absl::flat_hash_map<int, unsigned long, 42>* __this, const int* key) {
  return __this->erase(*key) != 0;
}

#pragma clang diagnostic pop
