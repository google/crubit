// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_cc

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/composable_bridging.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct StructWithBridgeField) == 1);
static_assert(alignof(struct StructWithBridgeField) == 1);
static_assert(CRUBIT_OFFSET_OF(bridge_field, struct StructWithBridgeField) ==
              0);

extern "C" void __rust_thunk___ZN21StructWithBridgeFieldC1Ev(
    struct StructWithBridgeField* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___Z15ReturnCppStructv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(::crubit::CppStructAbi::kSize,
                                     __return_abi_buffer);
  ::crubit::CppStructAbi().Encode(ReturnCppStruct(), __return_encoder);
}

static_assert((struct CppStruct (*)()) & ::ReturnCppStruct);

extern "C" void __rust_thunk___Z13TakeCppStruct9CppStruct(
    const unsigned char* __param_0) {
  ::crubit::Decoder ____param_0_decoder(::crubit::CppStructAbi::kSize,
                                        __param_0);
  TakeCppStruct(::crubit::CppStructAbi().Decode(____param_0_decoder));
}

static_assert((void (*)(struct CppStruct)) & ::TakeCppStruct);

static_assert(CRUBIT_SIZEOF(struct Vec3) == 12);
static_assert(alignof(struct Vec3) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct Vec3) == 0);
static_assert(CRUBIT_OFFSET_OF(y, struct Vec3) == 4);
static_assert(CRUBIT_OFFSET_OF(z, struct Vec3) == 8);

extern "C" void __rust_thunk___ZN4Vec3C1Ev(struct Vec3* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___Z16MakeOptionalVec3fffb(
    unsigned char* __return_abi_buffer, float x, float y, float z,
    bool is_present) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>(
      ::crubit::TransmuteAbi<::Vec3>())
      .Encode(MakeOptionalVec3(x, y, z, is_present), __return_encoder);
}

static_assert((struct MyOption<struct Vec3> (*)(float, float, float, bool)) &
              ::MakeOptionalVec3);

extern "C" void __rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(
    unsigned char* __return_abi_buffer, const unsigned char* v, float factor) {
  ::crubit::Decoder __v_decoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>::kSize, v);
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>(
      ::crubit::TransmuteAbi<::Vec3>())
      .Encode(MapMultiply(::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>(
                              ::crubit::TransmuteAbi<::Vec3>())
                              .Decode(__v_decoder),
                          factor),
              __return_encoder);
}

static_assert((struct MyOption<struct Vec3> (*)(struct MyOption<struct Vec3>,
                                                float)) &
              ::MapMultiply);

extern "C" void __rust_thunk___Z14MakeMyI8Structv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::MyI8Struct>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::MyI8Struct>>(
      ::crubit::TransmuteAbi<::MyI8Struct>())
      .Encode(MakeMyI8Struct(), __return_encoder);
}

static_assert((struct MyOption<struct MyI8Struct> (*)()) & ::MakeMyI8Struct);

static_assert((void (*)(::rs_std::SliceRef<::std::__u::string_view>)) &
              ::InspectStringViews);

extern "C" void __rust_thunk___Z12MaybeVoidPtrv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<void*>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<void*>>(
      ::crubit::TransmuteAbi<void*>())
      .Encode(MaybeVoidPtr(), __return_encoder);
}

static_assert((struct MyOption<void*> (*)()) & ::MaybeVoidPtr);

extern "C" void
__rust_thunk___Z40AcceptsSliceAndReturnsStatusErrorIfEmptyN6rs_std8SliceRefIKiEE(
    unsigned char* __return_abi_buffer, ::rs_std::SliceRef<const int> slice) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<
          ::crubit::TransmuteAbi<::rs_std::SliceRef<const int>>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::rs_std::SliceRef<const int>>>(
      ::crubit::TransmuteAbi<::rs_std::SliceRef<const int>>())
      .Encode(AcceptsSliceAndReturnsStatusErrorIfEmpty(slice),
              __return_encoder);
}

static_assert((struct MyOption<class rs_std::SliceRef<const int>> (*)(
                  ::rs_std::SliceRef<const int>)) &
              ::AcceptsSliceAndReturnsStatusErrorIfEmpty);

extern "C" void __rust_thunk___Z16ReturnsCStrArrayv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<char const**>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<char const**>>(
      ::crubit::TransmuteAbi<char const**>())
      .Encode(ReturnsCStrArray(), __return_encoder);
}

static_assert((struct MyOption<const char**> (*)()) & ::ReturnsCStrArray);

extern "C" void __rust_thunk___Z40ReturnsDefaultEnumInComposableBridgeTypev(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::DefaultEnum>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::DefaultEnum>>(
      ::crubit::TransmuteAbi<::DefaultEnum>())
      .Encode(ReturnsDefaultEnumInComposableBridgeType(), __return_encoder);
}

static_assert((struct MyOption<enum DefaultEnum> (*)()) &
              ::ReturnsDefaultEnumInComposableBridgeType);

extern "C" void __rust_thunk___Z36ReturnsI64EnumInComposableBridgeTypev(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::I64Enum>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::I64Enum>>(
      ::crubit::TransmuteAbi<::I64Enum>())
      .Encode(ReturnsI64EnumInComposableBridgeType(), __return_encoder);
}

static_assert((struct MyOption<enum I64Enum> (*)()) &
              ::ReturnsI64EnumInComposableBridgeType);

extern "C" void __rust_thunk___Z44ReturnsEnumInNamespaceInComposableBridgeTypev(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<
          ::crubit::TransmuteAbi<::some_namespace::EnumInNamespace>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<
      ::crubit::TransmuteAbi<::some_namespace::EnumInNamespace>>(
      ::crubit::TransmuteAbi<::some_namespace::EnumInNamespace>())
      .Encode(ReturnsEnumInNamespaceInComposableBridgeType(), __return_encoder);
}

static_assert((struct MyOption<enum some_namespace::EnumInNamespace> (*)()) &
              ::ReturnsEnumInNamespaceInComposableBridgeType);

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<class std::__wrap_iter<const int*>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<class std::__wrap_iter<const int*>>) ==
    8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __this,
    class std::__wrap_iter<const int*>* __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEE4baseEv(
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

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<
                            class std::__wrap_iter<class std::basic_string_view<
                                char, struct std::char_traits<char>>*>>) == 8);
static_assert(alignof(class std::reverse_iterator<
                      class std::__wrap_iter<class std::basic_string_view<
                          char, struct std::char_traits<char>>*>>) == 8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEC1Ev(
    class std::reverse_iterator<class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEC1ES7_(
    class std::reverse_iterator<class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*>>*
        __this,
    class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*>*
        __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEE4baseEv(
    class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>* __return,
    class std::reverse_iterator<
        class std::__wrap_iter<class std::basic_string_view<
            char, struct std::char_traits<char>>*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*> (
        ::std::reverse_iterator<
            class std::__wrap_iter<class std::basic_string_view<
                char, struct std::char_traits<char>>*>>::*)() const) &
    ::std::reverse_iterator<class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<class std::__wrap_iter<int*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<class std::__wrap_iter<int*>>) == 8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(
    class std::reverse_iterator<class std::__wrap_iter<int*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(
    class std::reverse_iterator<class std::__wrap_iter<int*>>* __this,
    class std::__wrap_iter<int*>* __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEE4baseEv(
    class std::__wrap_iter<int*>* __return,
    class std::reverse_iterator<class std::__wrap_iter<int*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<int*> (
                  ::std::reverse_iterator<class std::__wrap_iter<int*>>::*)()
                   const) &
              ::std::reverse_iterator<class std::__wrap_iter<int*>>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char*>) == 8);

extern "C" void __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
    class std::reverse_iterator<const char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
    class std::reverse_iterator<const char*>* __this, char const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char const*
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(
    class std::reverse_iterator<const char*> const* __this) {
  return __this->base();
}

static_assert((char const* (::std::reverse_iterator<const char*>::*)() const) &
              ::std::reverse_iterator<const char*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const wchar_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const wchar_t*>) == 8);

extern "C" void __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
    class std::reverse_iterator<const wchar_t*>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const int*>) == 8);
static_assert(alignof(class std::__wrap_iter<const int*>) == 8);

extern "C" void __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPKiEC1Ev(
    class std::__wrap_iter<const int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPKiEplEl(
    class std::__wrap_iter<const int*>* __return,
    class std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::__wrap_iter<const int*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const int*>::operator+);

extern "C" class std::__wrap_iter<const int*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPKiEpLEl(
    class std::__wrap_iter<const int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const int*> &
               (::std::__wrap_iter<const int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const int*>::operator+=);

extern "C" void __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPKiEmiEl(
    class std::__wrap_iter<const int*>* __return,
    class std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::__wrap_iter<const int*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const int*>::operator-);

extern "C" class std::__wrap_iter<const int*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPKiEmIEl(
    class std::__wrap_iter<const int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const int*> &
               (::std::__wrap_iter<const int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const int*>::operator-=);

extern "C" int const* __rust_thunk__6dc0ff60__ZNKSt3__u11__wrap_iterIPKiEixEl(
    class std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((int const& (::std::__wrap_iter<const int*>::*)(ptrdiff_t)
                   const) &
              ::std::__wrap_iter<const int*>::operator[]);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<class std::basic_string_view<
                                char, struct std::char_traits<char>>*>) == 8);
static_assert(alignof(class std::__wrap_iter<class std::basic_string_view<
                          char, struct std::char_traits<char>>*>) == 8);

extern "C" void
__rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEC1Ev(
    class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEplEl(
    class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>* __return,
    class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert(
    (class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*> (
        ::std::__wrap_iter<class std::basic_string_view<
            char, struct std::char_traits<char>>*>::*)(ptrdiff_t) const) &
    ::std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>::operator+);

extern "C" class std::__wrap_iter<
    class std::basic_string_view<char, struct std::char_traits<char>>*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEpLEl(
    class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert(
    (class std::__wrap_iter<
         class std::basic_string_view<char, struct std::char_traits<char>>*> &
     (::std::__wrap_iter<class std::basic_string_view<
          char, struct std::char_traits<char>>*>::*)(ptrdiff_t)) &
    ::std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>::operator+=);

extern "C" void
__rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEmiEl(
    class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>* __return,
    class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert(
    (class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*> (
        ::std::__wrap_iter<class std::basic_string_view<
            char, struct std::char_traits<char>>*>::*)(ptrdiff_t) const) &
    ::std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>::operator-);

extern "C" class std::__wrap_iter<
    class std::basic_string_view<char, struct std::char_traits<char>>*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEmIEl(
    class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert(
    (class std::__wrap_iter<
         class std::basic_string_view<char, struct std::char_traits<char>>*> &
     (::std::__wrap_iter<class std::basic_string_view<
          char, struct std::char_traits<char>>*>::*)(ptrdiff_t)) &
    ::std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>::operator-=);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<int*>) == 8);
static_assert(alignof(class std::__wrap_iter<int*>) == 8);

extern "C" void __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPiEC1Ev(
    class std::__wrap_iter<int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPiEplEl(
    class std::__wrap_iter<int*>* __return,
    class std::__wrap_iter<int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<int*> (::std::__wrap_iter<int*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<int*>::operator+);

extern "C" class std::__wrap_iter<int*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPiEpLEl(
    class std::__wrap_iter<int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<int*> &
               (::std::__wrap_iter<int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<int*>::operator+=);

extern "C" void __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPiEmiEl(
    class std::__wrap_iter<int*>* __return,
    class std::__wrap_iter<int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<int*> (::std::__wrap_iter<int*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<int*>::operator-);

extern "C" class std::__wrap_iter<int*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPiEmIEl(
    class std::__wrap_iter<int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<int*> &
               (::std::__wrap_iter<int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<int*>::operator-=);

static_assert(
    CRUBIT_SIZEOF(class std::span<const int, 18446744073709551615UL>) == 16);
static_assert(alignof(class std::span<const int, 18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__9ef48370__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
    class std::span<const int, 18446744073709551615UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__eb5fa8a1__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm(
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
__rust_thunk__cb04abd4__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm(
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
__rust_thunk__f8d72bf1__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm(
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
__rust_thunk__e58d956f__ZNKSt3__u4spanIKiLm18446744073709551615EE4sizeEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::size);

extern "C" size_t
__rust_thunk__1a2eb8d0__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert((size_t (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__5eda390c__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::empty);

extern "C" int const*
__rust_thunk__0b050f23__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm(
    class std::span<const int, 18446744073709551615UL> const* __this,
    size_t __idx) {
  return std::addressof(__this->operator[](__idx));
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)(
                  size_t) const) &
              ::std::span<const int, 18446744073709551615UL>::operator[]);

extern "C" int const*
__rust_thunk__02898003__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::front);

extern "C" int const*
__rust_thunk__f4827081__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::back);

extern "C" int const*
__rust_thunk__e6274c04__ZNKSt3__u4spanIKiLm18446744073709551615EE4dataEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->data();
}

static_assert((int const* (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::data);

extern "C" void
__rust_thunk__4d8e3070__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv(
    class std::__wrap_iter<const int*>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__b3c9f034__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv(
    class std::__wrap_iter<const int*>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::end);

extern "C" void
__rust_thunk__3e61f7cc__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<const int*>> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__dc1b110a__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<const int*>> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::rend);

static_assert(
    CRUBIT_SIZEOF(
        class std::span<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            18446744073709551615UL>) == 16);
static_assert(
    alignof(class std::span<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__9ef48370__ZNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEC1Ev(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__eb5fa8a1__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5firstEm(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>* __return,
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert(
    (class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> (
        ::std::span<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            18446744073709551615UL>::*)(size_t) const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::first);

extern "C" void
__rust_thunk__cb04abd4__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4lastEm(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>* __return,
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert(
    (class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> (
        ::std::span<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            18446744073709551615UL>::*)(size_t) const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::last);

extern "C" void
__rust_thunk__f8d72bf1__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE7subspanEmm(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>* __return,
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this,
    size_t __offset, size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert(
    (class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> (
        ::std::span<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            18446744073709551615UL>::*)(size_t, size_t) const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__e58d956f__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4sizeEv(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::span<
             class std::basic_string_view<char, struct std::char_traits<char>>,
             18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::size);

extern "C" size_t
__rust_thunk__1a2eb8d0__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE10size_bytesEv(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert(
    (size_t (::std::span<
             class std::basic_string_view<char, struct std::char_traits<char>>,
             18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__5eda390c__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5emptyEv(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::span<
           class std::basic_string_view<char, struct std::char_traits<char>>,
           18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::empty);

extern "C" ::std::__u::string_view*
__rust_thunk__02898003__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5frontEv(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert(
    (::std::__u::string_view &
     (::std::span<
         class std::basic_string_view<char, struct std::char_traits<char>>,
         18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::front);

extern "C" ::std::__u::string_view*
__rust_thunk__f4827081__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4backEv(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert(
    (::std::__u::string_view &
     (::std::span<
         class std::basic_string_view<char, struct std::char_traits<char>>,
         18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::back);

extern "C" ::std::__u::string_view*
__rust_thunk__e6274c04__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4dataEv(
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  return __this->data();
}

static_assert(
    (::std::__u::string_view *
     (::std::span<
         class std::basic_string_view<char, struct std::char_traits<char>>,
         18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::data);

extern "C" void
__rust_thunk__4d8e3070__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5beginEv(
    class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>* __return,
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert(
    (class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*> (
        ::std::span<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__b3c9f034__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE3endEv(
    class std::__wrap_iter<class std::basic_string_view<
        char, struct std::char_traits<char>>*>* __return,
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert(
    (class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*> (
        ::std::span<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::end);

extern "C" void
__rust_thunk__3e61f7cc__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE6rbeginEv(
    class std::reverse_iterator<class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*>>*
        __return,
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class std::reverse_iterator<class std::__wrap_iter<
         class std::basic_string_view<char, struct std::char_traits<char>>*>> (
        ::std::span<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__dc1b110a__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4rendEv(
    class std::reverse_iterator<class std::__wrap_iter<
        class std::basic_string_view<char, struct std::char_traits<char>>*>>*
        __return,
    class std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class std::reverse_iterator<class std::__wrap_iter<
         class std::basic_string_view<char, struct std::char_traits<char>>*>> (
        ::std::span<
            class std::basic_string_view<char, struct std::char_traits<char>>,
            18446744073709551615UL>::*)() const) &
    ::std::span<
        class std::basic_string_view<char, struct std::char_traits<char>>,
        18446744073709551615UL>::rend);

static_assert(CRUBIT_SIZEOF(class std::span<int, 18446744073709551615UL>) ==
              16);
static_assert(alignof(class std::span<int, 18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__9ef48370__ZNSt3__u4spanIiLm18446744073709551615EEC1Ev(
    class std::span<int, 18446744073709551615UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__eb5fa8a1__ZNKSt3__u4spanIiLm18446744073709551615EE5firstEm(
    class std::span<int, 18446744073709551615UL>* __return,
    class std::span<int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert((class std::span<int, 18446744073709551615UL> (
                  ::std::span<int, 18446744073709551615UL>::*)(size_t) const) &
              ::std::span<int, 18446744073709551615UL>::first);

extern "C" void
__rust_thunk__cb04abd4__ZNKSt3__u4spanIiLm18446744073709551615EE4lastEm(
    class std::span<int, 18446744073709551615UL>* __return,
    class std::span<int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert((class std::span<int, 18446744073709551615UL> (
                  ::std::span<int, 18446744073709551615UL>::*)(size_t) const) &
              ::std::span<int, 18446744073709551615UL>::last);

extern "C" void
__rust_thunk__f8d72bf1__ZNKSt3__u4spanIiLm18446744073709551615EE7subspanEmm(
    class std::span<int, 18446744073709551615UL>* __return,
    class std::span<int, 18446744073709551615UL> const* __this, size_t __offset,
    size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert((class std::span<int, 18446744073709551615UL> (
                  ::std::span<int, 18446744073709551615UL>::*)(size_t, size_t)
                   const) &
              ::std::span<int, 18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__e58d956f__ZNKSt3__u4spanIiLm18446744073709551615EE4sizeEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::size);

extern "C" size_t
__rust_thunk__1a2eb8d0__ZNKSt3__u4spanIiLm18446744073709551615EE10size_bytesEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert((size_t (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__5eda390c__ZNKSt3__u4spanIiLm18446744073709551615EE5emptyEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::empty);

extern "C" int*
__rust_thunk__02898003__ZNKSt3__u4spanIiLm18446744073709551615EE5frontEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert((int& (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::front);

extern "C" int*
__rust_thunk__f4827081__ZNKSt3__u4spanIiLm18446744073709551615EE4backEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert((int& (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::back);

extern "C" int*
__rust_thunk__e6274c04__ZNKSt3__u4spanIiLm18446744073709551615EE4dataEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return __this->data();
}

static_assert((int* (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::data);

extern "C" void
__rust_thunk__4d8e3070__ZNKSt3__u4spanIiLm18446744073709551615EE5beginEv(
    class std::__wrap_iter<int*>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert((class std::__wrap_iter<int*> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__b3c9f034__ZNKSt3__u4spanIiLm18446744073709551615EE3endEv(
    class std::__wrap_iter<int*>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert((class std::__wrap_iter<int*> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::end);

extern "C" void
__rust_thunk__3e61f7cc__ZNKSt3__u4spanIiLm18446744073709551615EE6rbeginEv(
    class std::reverse_iterator<class std::__wrap_iter<int*>>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<int*>> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__dc1b110a__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv(
    class std::reverse_iterator<class std::__wrap_iter<int*>>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<int*>> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::rend);

#pragma clang diagnostic pop
