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

static_assert((struct MyOption<Vec3> (*)(float, float, float, bool)) &
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

static_assert((struct MyOption<Vec3> (*)(struct MyOption<Vec3>, float)) &
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

static_assert((struct MyOption<MyI8Struct> (*)()) & ::MakeMyI8Struct);

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

static_assert((struct MyOption<rs_std::SliceRef<const int>> (*)(
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

static_assert((struct MyOption<DefaultEnum> (*)()) &
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

static_assert((struct MyOption<I64Enum> (*)()) &
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

static_assert((struct MyOption<some_namespace::EnumInNamespace> (*)()) &
              ::ReturnsEnumInNamespaceInComposableBridgeType);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<char32_t>) == 16);
static_assert(alignof(class std::initializer_list<char32_t>) == 8);

extern "C" void __rust_thunk__930f72cd__ZNSt16initializer_listIDiEC1Ev(
    class std::initializer_list<char32_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char32_t const*
__rust_thunk__e332b549__ZNKSt16initializer_listIDiE4dataEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->data();
}

static_assert((char32_t const* (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::data);

extern "C" size_t __rust_thunk__b623ee09__ZNKSt16initializer_listIDiE4sizeEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::size);

extern "C" bool __rust_thunk__b7fe421e__ZNKSt16initializer_listIDiE5emptyEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::empty);

extern "C" char32_t const*
__rust_thunk__f28ef40c__ZNKSt16initializer_listIDiE5beginEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->begin();
}

static_assert((char32_t const* (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::begin);

extern "C" char32_t const*
__rust_thunk__a337f936__ZNKSt16initializer_listIDiE3endEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->end();
}

static_assert((char32_t const* (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::end);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<char16_t>) == 16);
static_assert(alignof(class std::initializer_list<char16_t>) == 8);

extern "C" void __rust_thunk__930f72cd__ZNSt16initializer_listIDsEC1Ev(
    class std::initializer_list<char16_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char16_t const*
__rust_thunk__e332b549__ZNKSt16initializer_listIDsE4dataEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->data();
}

static_assert((char16_t const* (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::data);

extern "C" size_t __rust_thunk__b623ee09__ZNKSt16initializer_listIDsE4sizeEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::size);

extern "C" bool __rust_thunk__b7fe421e__ZNKSt16initializer_listIDsE5emptyEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::empty);

extern "C" char16_t const*
__rust_thunk__f28ef40c__ZNKSt16initializer_listIDsE5beginEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->begin();
}

static_assert((char16_t const* (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::begin);

extern "C" char16_t const*
__rust_thunk__a337f936__ZNKSt16initializer_listIDsE3endEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->end();
}

static_assert((char16_t const* (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::end);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<char>) == 16);
static_assert(alignof(class std::initializer_list<char>) == 8);

extern "C" void __rust_thunk__930f72cd__ZNSt16initializer_listIcEC1Ev(
    class std::initializer_list<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" char const*
__rust_thunk__e332b549__ZNKSt16initializer_listIcE4dataEv(
    class std::initializer_list<char> const* __this) {
  return __this->data();
}

static_assert((char const* (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::data);

extern "C" size_t __rust_thunk__b623ee09__ZNKSt16initializer_listIcE4sizeEv(
    class std::initializer_list<char> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::size);

extern "C" bool __rust_thunk__b7fe421e__ZNKSt16initializer_listIcE5emptyEv(
    class std::initializer_list<char> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::empty);

extern "C" char const*
__rust_thunk__f28ef40c__ZNKSt16initializer_listIcE5beginEv(
    class std::initializer_list<char> const* __this) {
  return __this->begin();
}

static_assert((char const* (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::begin);

extern "C" char const* __rust_thunk__a337f936__ZNKSt16initializer_listIcE3endEv(
    class std::initializer_list<char> const* __this) {
  return __this->end();
}

static_assert((char const* (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::end);

static_assert(sizeof(class std::allocator<char32_t>) == 1);
static_assert(alignof(class std::allocator<char32_t>) == 1);

extern "C" void __rust_thunk__874cc001__ZNSt3__u9allocatorIDiEC1Ev(
    class std::allocator<char32_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char32_t* __rust_thunk__f1b14650__ZNSt3__u9allocatorIDiE8allocateEm(
    class std::allocator<char32_t>* __this, size_t __n) {
  return __this->allocate(__n);
}

static_assert((char32_t* (::std::allocator<char32_t>::*)(size_t)) &
              ::std::allocator<char32_t>::allocate);

extern "C" void __rust_thunk__afb010d7__ZNSt3__u9allocatorIDiE10deallocateEPDim(
    class std::allocator<char32_t>* __this, char32_t* __p, size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert((void (::std::allocator<char32_t>::*)(char32_t*, size_t)) &
              ::std::allocator<char32_t>::deallocate);

static_assert(sizeof(class std::allocator<char16_t>) == 1);
static_assert(alignof(class std::allocator<char16_t>) == 1);

extern "C" void __rust_thunk__874cc001__ZNSt3__u9allocatorIDsEC1Ev(
    class std::allocator<char16_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char16_t* __rust_thunk__f1b14650__ZNSt3__u9allocatorIDsE8allocateEm(
    class std::allocator<char16_t>* __this, size_t __n) {
  return __this->allocate(__n);
}

static_assert((char16_t* (::std::allocator<char16_t>::*)(size_t)) &
              ::std::allocator<char16_t>::allocate);

extern "C" void __rust_thunk__afb010d7__ZNSt3__u9allocatorIDsE10deallocateEPDsm(
    class std::allocator<char16_t>* __this, char16_t* __p, size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert((void (::std::allocator<char16_t>::*)(char16_t*, size_t)) &
              ::std::allocator<char16_t>::deallocate);

static_assert(sizeof(class std::allocator<char>) == 1);
static_assert(alignof(class std::allocator<char>) == 1);

extern "C" void __rust_thunk__874cc001__ZNSt3__u9allocatorIcEC1Ev(
    class std::allocator<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" char* __rust_thunk__f1b14650__ZNSt3__u9allocatorIcE8allocateEm(
    class std::allocator<char>* __this, size_t __n) {
  return __this->allocate(__n);
}

static_assert((char* (::std::allocator<char>::*)(size_t)) &
              ::std::allocator<char>::allocate);

extern "C" void __rust_thunk__afb010d7__ZNSt3__u9allocatorIcE10deallocateEPcm(
    class std::allocator<char>* __this, char* __p, size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert((void (::std::allocator<char>::*)(char*, size_t)) &
              ::std::allocator<char>::deallocate);

static_assert(CRUBIT_SIZEOF(class std::pmr::polymorphic_allocator<char32_t>) ==
              8);
static_assert(alignof(class std::pmr::polymorphic_allocator<char32_t>) == 8);

extern "C" void
__rust_thunk__68de76ae__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1Ev(
    class std::pmr::polymorphic_allocator<char32_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__39b21c9e__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1EPNS0_15memory_resourceE(
    class std::pmr::polymorphic_allocator<char32_t>* __this,
    class ::std::__u::pmr::memory_resource* __r) {
  crubit::construct_at(__this, __r);
}

static_assert(CRUBIT_SIZEOF(class std::pmr::polymorphic_allocator<char16_t>) ==
              8);
static_assert(alignof(class std::pmr::polymorphic_allocator<char16_t>) == 8);

extern "C" void
__rust_thunk__68de76ae__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1Ev(
    class std::pmr::polymorphic_allocator<char16_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__39b21c9e__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1EPNS0_15memory_resourceE(
    class std::pmr::polymorphic_allocator<char16_t>* __this,
    class ::std::__u::pmr::memory_resource* __r) {
  crubit::construct_at(__this, __r);
}

static_assert(CRUBIT_SIZEOF(class std::pmr::polymorphic_allocator<char>) == 8);
static_assert(alignof(class std::pmr::polymorphic_allocator<char>) == 8);

extern "C" void
__rust_thunk__68de76ae__ZNSt3__u3pmr21polymorphic_allocatorIcEC1Ev(
    class std::pmr::polymorphic_allocator<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__39b21c9e__ZNSt3__u3pmr21polymorphic_allocatorIcEC1EPNS0_15memory_resourceE(
    class std::pmr::polymorphic_allocator<char>* __this,
    class ::std::__u::pmr::memory_resource* __r) {
  crubit::construct_at(__this, __r);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string<char32_t, std::char_traits<char32_t>,
                                std::pmr::polymorphic_allocator<char32_t>>) ==
    32);
static_assert(
    alignof(
        class std::basic_string<char32_t, std::char_traits<char32_t>,
                                std::pmr::polymorphic_allocator<char32_t>>) ==
    8);

extern "C" void
__rust_thunk__2092f4db__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__9482840f__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__d2b8915c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c5d73420__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_RKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __str,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__84e8d931__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__0ef12977__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_RKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__6a8a26ea__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ESt16initializer_listIDiERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__9e328439__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEED1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__1ce0ecef__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEOS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         class std::basic_string<
             char32_t, std::char_traits<char32_t>,
             std::pmr::polymorphic_allocator<char32_t>>&&)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__687f7d28__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSESt16initializer_listIDiE(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         class std::initializer_list<char32_t>)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__ea8273a8__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEPKDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    char32_t const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         char32_t const*)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" void
__rust_thunk__03fe85db__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE6cbeginEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->cbegin());
}

static_assert(
    (class std::__wrap_iter<const char32_t*> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::cbegin);

extern "C" void
__rust_thunk__04b06d49__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4cendEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->cend());
}

static_assert(
    (class std::__wrap_iter<const char32_t*> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::cend);

extern "C" void
__rust_thunk__1eaf4f3f__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char32_t*>> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::crbegin);

extern "C" void
__rust_thunk__868ef36b__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char32_t*>> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::crend);

extern "C" void
__rust_thunk__36b91357__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7reserveEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  __this->reserve();
}

static_assert(
    (void (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::reserve);

extern "C" bool
__rust_thunk__df51d7b5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5emptyEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string<char32_t, std::char_traits<char32_t>,
                               std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::empty);

extern "C" char32_t const*
__rust_thunk__b06cd177__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5c_strEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  return __this->c_str();
}

static_assert(
    (char32_t const* (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::c_str);

extern "C" void
__rust_thunk__72feb6af__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13get_allocatorEv(
    class std::pmr::polymorphic_allocator<char32_t>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert(
    (class std::pmr::polymorphic_allocator<char32_t> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<
        char32_t, std::char_traits<char32_t>,
        std::pmr::polymorphic_allocator<char32_t>>::get_allocator);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__372f6a76__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    char32_t __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         char32_t)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__a5cc26f4__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSERKS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         class std::basic_string<
             char32_t, std::char_traits<char32_t>,
             std::pmr::polymorphic_allocator<char32_t>> const&)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" void
__rust_thunk__c41d7f8a__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE9push_backEDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    char32_t __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                               std::pmr::polymorphic_allocator<char32_t>>::*)(
        char32_t)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::push_back);

extern "C" void
__rust_thunk__919c3118__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE8pop_backEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  __this->pop_back();
}

static_assert(
    (void (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::pop_back);

extern "C" void
__rust_thunk__89a4566e__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5clearEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  __this->clear();
}

static_assert(
    (void (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::clear);

extern "C" void
__rust_thunk__f97f4b0d__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13shrink_to_fitEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  __this->shrink_to_fit();
}

static_assert(
    (void (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()) &
    ::std::basic_string<
        char32_t, std::char_traits<char32_t>,
        std::pmr::polymorphic_allocator<char32_t>>::shrink_to_fit);

extern "C" void
__rust_thunk__6a0bf3ce__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4swapERS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str) {
  __this->swap(*__str);
}

static_assert(
    (void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                               std::pmr::polymorphic_allocator<char32_t>>::*)(
        class std::basic_string<char32_t, std::char_traits<char32_t>,
                                std::pmr::polymorphic_allocator<char32_t>>&)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::swap);

static_assert(
    CRUBIT_SIZEOF(class std::basic_string<char32_t, std::char_traits<char32_t>,
                                          std::allocator<char32_t>>) == 24);
static_assert(
    alignof(class std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>) == 8);

extern "C" void
__rust_thunk__2092f4db__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__9482840f__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__d2b8915c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c5d73420__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_RKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__84e8d931__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__0ef12977__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_RKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__e0b10d67__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mmRKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str,
    size_t __pos, size_t __n, class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, __n, *__a);
}

extern "C" void
__rust_thunk__b9d3416e__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mRKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str,
    size_t __pos, class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, *__a);
}

extern "C" void
__rust_thunk__6a8a26ea__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ESt16initializer_listIDiERKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__9e328439__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEED1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__1ce0ecef__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEOS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> &
               (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(
                   class std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>&&)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__687f7d28__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSESt16initializer_listIDiE(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> &
               (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(
                   class std::initializer_list<char32_t>)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__ea8273a8__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEPKDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    char32_t const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::allocator<char32_t>>::*)(char32_t const*)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::allocator<char32_t>>::operator=);

extern "C" void
__rust_thunk__03fe85db__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6cbeginEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->cbegin());
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::cbegin);

extern "C" void
__rust_thunk__04b06d49__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4cendEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->cend());
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::cend);

extern "C" void
__rust_thunk__1eaf4f3f__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->crbegin());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const char32_t*>> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::crbegin);

extern "C" void
__rust_thunk__868ef36b__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->crend());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const char32_t*>> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::crend);

extern "C" size_t
__rust_thunk__bceaaa6c__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4sizeEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::size);

extern "C" size_t
__rust_thunk__a429ae4c__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6lengthEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->length();
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::length);

extern "C" size_t
__rust_thunk__c5dc490e__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8max_sizeEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->max_size();
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::max_size);

extern "C" size_t
__rust_thunk__dd906736__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8capacityEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->capacity();
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::capacity);

extern "C" bool
__rust_thunk__df51d7b5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5emptyEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::empty);

extern "C" char32_t const*
__rust_thunk__ff95ff90__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char32_t const& (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                          std::allocator<char32_t>>::*)(size_t)
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::allocator<char32_t>>::operator[]);

extern "C" char32_t*
__rust_thunk__9f614826__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char32_t& (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(size_t)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::allocator<char32_t>>::operator[]);

extern "C" void
__rust_thunk__70859a7f__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6substrEmm(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)(size_t,
                                                                    size_t)
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::substr);

extern "C" char32_t const*
__rust_thunk__b06cd177__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5c_strEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->c_str();
}

static_assert((char32_t const* (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::c_str);

extern "C" void
__rust_thunk__72feb6af__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13get_allocatorEv(
    class std::allocator<char32_t>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert((class std::allocator<char32_t> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::get_allocator);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__372f6a76__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    char32_t __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> &
               (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(char32_t)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__a5cc26f4__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> &
               (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(
                   class std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>> const&)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::operator=);

extern "C" void
__rust_thunk__c41d7f8a__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE9push_backEDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    char32_t __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                               std::allocator<char32_t>>::*)(char32_t)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::allocator<char32_t>>::push_back);

extern "C" void
__rust_thunk__919c3118__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8pop_backEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  __this->pop_back();
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::pop_back);

extern "C" void
__rust_thunk__89a4566e__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5clearEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  __this->clear();
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::clear);

extern "C" void
__rust_thunk__f97f4b0d__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13shrink_to_fitEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  __this->shrink_to_fit();
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::shrink_to_fit);

extern "C" size_t
__rust_thunk__d34fee97__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4copyEPDimm(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this,
    char32_t* __s, size_t __n, size_t __pos) {
  return __this->copy(__s, __n, __pos);
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)(
                  char32_t*, size_t, size_t) const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::copy);

extern "C" void
__rust_thunk__6a0bf3ce__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4swapERS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str) {
  __this->swap(*__str);
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)(
                  class std::basic_string<char32_t, std::char_traits<char32_t>,
                                          std::allocator<char32_t>>&)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::swap);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string<char16_t, std::char_traits<char16_t>,
                                std::pmr::polymorphic_allocator<char16_t>>) ==
    32);
static_assert(
    alignof(
        class std::basic_string<char16_t, std::char_traits<char16_t>,
                                std::pmr::polymorphic_allocator<char16_t>>) ==
    8);

extern "C" void
__rust_thunk__2092f4db__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__9482840f__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__d2b8915c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c5d73420__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_RKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __str,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__84e8d931__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__0ef12977__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_RKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__6a8a26ea__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ESt16initializer_listIDsERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__9e328439__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEED1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__1ce0ecef__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEOS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         class std::basic_string<
             char16_t, std::char_traits<char16_t>,
             std::pmr::polymorphic_allocator<char16_t>>&&)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__687f7d28__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSESt16initializer_listIDsE(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         class std::initializer_list<char16_t>)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__ea8273a8__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEPKDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    char16_t const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         char16_t const*)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" void
__rust_thunk__03fe85db__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE6cbeginEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->cbegin());
}

static_assert(
    (class std::__wrap_iter<const char16_t*> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::cbegin);

extern "C" void
__rust_thunk__04b06d49__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4cendEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->cend());
}

static_assert(
    (class std::__wrap_iter<const char16_t*> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::cend);

extern "C" void
__rust_thunk__1eaf4f3f__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char16_t*>> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::crbegin);

extern "C" void
__rust_thunk__868ef36b__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char16_t*>> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::crend);

extern "C" void
__rust_thunk__36b91357__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7reserveEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  __this->reserve();
}

static_assert(
    (void (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::reserve);

extern "C" bool
__rust_thunk__df51d7b5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5emptyEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string<char16_t, std::char_traits<char16_t>,
                               std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::empty);

extern "C" char16_t const*
__rust_thunk__b06cd177__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5c_strEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  return __this->c_str();
}

static_assert(
    (char16_t const* (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::c_str);

extern "C" void
__rust_thunk__72feb6af__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13get_allocatorEv(
    class std::pmr::polymorphic_allocator<char16_t>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert(
    (class std::pmr::polymorphic_allocator<char16_t> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<
        char16_t, std::char_traits<char16_t>,
        std::pmr::polymorphic_allocator<char16_t>>::get_allocator);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__372f6a76__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    char16_t __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         char16_t)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__a5cc26f4__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSERKS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         class std::basic_string<
             char16_t, std::char_traits<char16_t>,
             std::pmr::polymorphic_allocator<char16_t>> const&)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" void
__rust_thunk__c41d7f8a__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE9push_backEDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    char16_t __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                               std::pmr::polymorphic_allocator<char16_t>>::*)(
        char16_t)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::push_back);

extern "C" void
__rust_thunk__919c3118__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE8pop_backEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  __this->pop_back();
}

static_assert(
    (void (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::pop_back);

extern "C" void
__rust_thunk__89a4566e__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5clearEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  __this->clear();
}

static_assert(
    (void (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::clear);

extern "C" void
__rust_thunk__f97f4b0d__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13shrink_to_fitEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  __this->shrink_to_fit();
}

static_assert(
    (void (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()) &
    ::std::basic_string<
        char16_t, std::char_traits<char16_t>,
        std::pmr::polymorphic_allocator<char16_t>>::shrink_to_fit);

extern "C" void
__rust_thunk__6a0bf3ce__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4swapERS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str) {
  __this->swap(*__str);
}

static_assert(
    (void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                               std::pmr::polymorphic_allocator<char16_t>>::*)(
        class std::basic_string<char16_t, std::char_traits<char16_t>,
                                std::pmr::polymorphic_allocator<char16_t>>&)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::swap);

static_assert(
    CRUBIT_SIZEOF(class std::basic_string<char16_t, std::char_traits<char16_t>,
                                          std::allocator<char16_t>>) == 24);
static_assert(
    alignof(class std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>) == 8);

extern "C" void
__rust_thunk__2092f4db__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__9482840f__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__d2b8915c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c5d73420__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_RKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__84e8d931__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__0ef12977__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_RKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__e0b10d67__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mmRKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str,
    size_t __pos, size_t __n, class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, __n, *__a);
}

extern "C" void
__rust_thunk__b9d3416e__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mRKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str,
    size_t __pos, class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, *__a);
}

extern "C" void
__rust_thunk__6a8a26ea__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ESt16initializer_listIDsERKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__9e328439__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEED1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__1ce0ecef__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEOS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> &
               (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(
                   class std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>&&)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__687f7d28__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSESt16initializer_listIDsE(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> &
               (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(
                   class std::initializer_list<char16_t>)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__ea8273a8__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEPKDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    char16_t const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::allocator<char16_t>>::*)(char16_t const*)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::allocator<char16_t>>::operator=);

extern "C" void
__rust_thunk__03fe85db__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6cbeginEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->cbegin());
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::cbegin);

extern "C" void
__rust_thunk__04b06d49__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4cendEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->cend());
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::cend);

extern "C" void
__rust_thunk__1eaf4f3f__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->crbegin());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const char16_t*>> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::crbegin);

extern "C" void
__rust_thunk__868ef36b__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->crend());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const char16_t*>> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::crend);

extern "C" size_t
__rust_thunk__bceaaa6c__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4sizeEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::size);

extern "C" size_t
__rust_thunk__a429ae4c__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6lengthEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->length();
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::length);

extern "C" size_t
__rust_thunk__c5dc490e__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8max_sizeEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->max_size();
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::max_size);

extern "C" size_t
__rust_thunk__dd906736__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8capacityEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->capacity();
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::capacity);

extern "C" bool
__rust_thunk__df51d7b5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5emptyEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::empty);

extern "C" char16_t const*
__rust_thunk__ff95ff90__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char16_t const& (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                          std::allocator<char16_t>>::*)(size_t)
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::allocator<char16_t>>::operator[]);

extern "C" char16_t*
__rust_thunk__9f614826__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char16_t& (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(size_t)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::allocator<char16_t>>::operator[]);

extern "C" void
__rust_thunk__70859a7f__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6substrEmm(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)(size_t,
                                                                    size_t)
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::substr);

extern "C" char16_t const*
__rust_thunk__b06cd177__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5c_strEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->c_str();
}

static_assert((char16_t const* (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::c_str);

extern "C" void
__rust_thunk__72feb6af__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13get_allocatorEv(
    class std::allocator<char16_t>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert((class std::allocator<char16_t> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::get_allocator);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__372f6a76__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    char16_t __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> &
               (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(char16_t)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__a5cc26f4__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> &
               (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(
                   class std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>> const&)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::operator=);

extern "C" void
__rust_thunk__c41d7f8a__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE9push_backEDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    char16_t __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                               std::allocator<char16_t>>::*)(char16_t)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::allocator<char16_t>>::push_back);

extern "C" void
__rust_thunk__919c3118__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8pop_backEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  __this->pop_back();
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::pop_back);

extern "C" void
__rust_thunk__89a4566e__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5clearEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  __this->clear();
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::clear);

extern "C" void
__rust_thunk__f97f4b0d__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13shrink_to_fitEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  __this->shrink_to_fit();
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::shrink_to_fit);

extern "C" size_t
__rust_thunk__d34fee97__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4copyEPDsmm(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this,
    char16_t* __s, size_t __n, size_t __pos) {
  return __this->copy(__s, __n, __pos);
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)(
                  char16_t*, size_t, size_t) const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::copy);

extern "C" void
__rust_thunk__6a0bf3ce__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4swapERS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str) {
  __this->swap(*__str);
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)(
                  class std::basic_string<char16_t, std::char_traits<char16_t>,
                                          std::allocator<char16_t>>&)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::swap);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string<char, std::char_traits<char>,
                                std::pmr::polymorphic_allocator<char>>) == 32);
static_assert(
    alignof(class std::basic_string<char, std::char_traits<char>,
                                    std::pmr::polymorphic_allocator<char>>) ==
    8);

extern "C" void
__rust_thunk__2092f4db__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1Ev(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__9482840f__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__d2b8915c__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c5d73420__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_RKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const* __str,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__84e8d931__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__0ef12977__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_RKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__6a8a26ea__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ESt16initializer_listIcERKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::initializer_list<char>* __il,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__9e328439__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEED1Ev(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__1ce0ecef__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEOS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(
         class std::basic_string<char, std::char_traits<char>,
                                 std::pmr::polymorphic_allocator<char>>&&)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__687f7d28__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSESt16initializer_listIcE(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::initializer_list<char>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(
         class std::initializer_list<char>)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__ea8273a8__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEPKc(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    char const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(
         char const*)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" void
__rust_thunk__03fe85db__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE6cbeginEv(
    class std::__wrap_iter<const char*>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->cbegin());
}

static_assert(
    (class std::__wrap_iter<const char*> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::cbegin);

extern "C" void
__rust_thunk__04b06d49__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4cendEv(
    class std::__wrap_iter<const char*>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->cend());
}

static_assert(
    (class std::__wrap_iter<const char*> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::cend);

extern "C" void
__rust_thunk__1eaf4f3f__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char*>> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::crbegin);

extern "C" void
__rust_thunk__868ef36b__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char*>> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::crend);

extern "C" void
__rust_thunk__36b91357__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7reserveEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  __this->reserve();
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::reserve);

extern "C" bool
__rust_thunk__df51d7b5__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5emptyEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::empty);

extern "C" char const*
__rust_thunk__b06cd177__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5c_strEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  return __this->c_str();
}

static_assert(
    (char const* (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::c_str);

extern "C" void
__rust_thunk__72feb6af__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13get_allocatorEv(
    class std::pmr::polymorphic_allocator<char>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert(
    (class std::pmr::polymorphic_allocator<char> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::get_allocator);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__372f6a76__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEc(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    char __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(char)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__a5cc26f4__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSERKS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(
         class std::basic_string<
             char, std::char_traits<char>,
             std::pmr::polymorphic_allocator<char>> const&)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" void
__rust_thunk__c41d7f8a__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE9push_backEc(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    char __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)(
        char)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::push_back);

extern "C" void
__rust_thunk__919c3118__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE8pop_backEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  __this->pop_back();
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::pop_back);

extern "C" void
__rust_thunk__89a4566e__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5clearEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  __this->clear();
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::clear);

extern "C" void
__rust_thunk__f97f4b0d__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13shrink_to_fitEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  __this->shrink_to_fit();
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::shrink_to_fit);

extern "C" void
__rust_thunk__6a0bf3ce__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4swapERS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str) {
  __this->swap(*__str);
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)(
        class std::basic_string<char, std::char_traits<char>,
                                std::pmr::polymorphic_allocator<char>>&)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::swap);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<char32_t*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<char32_t*>>) == 8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<char32_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<char32_t*>>* __this,
    class std::__wrap_iter<char32_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEE4baseEv(
    class std::__wrap_iter<char32_t*>* __return,
    class std::reverse_iterator<std::__wrap_iter<char32_t*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<char32_t*> (
                  ::std::reverse_iterator<std::__wrap_iter<char32_t*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<char32_t*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<char16_t*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<char16_t*>>) == 8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<char16_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<char16_t*>>* __this,
    class std::__wrap_iter<char16_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEE4baseEv(
    class std::__wrap_iter<char16_t*>* __return,
    class std::reverse_iterator<std::__wrap_iter<char16_t*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<char16_t*> (
                  ::std::reverse_iterator<std::__wrap_iter<char16_t*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<char16_t*>>::base);

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<std::__wrap_iter<const char32_t*>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<const char32_t*>>) ==
    8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __this,
    class std::__wrap_iter<const char32_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEE4baseEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>> const*
        __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__wrap_iter<const char32_t*> (
        ::std::reverse_iterator<std::__wrap_iter<const char32_t*>>::*)()
         const) &
    ::std::reverse_iterator<std::__wrap_iter<const char32_t*>>::base);

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<std::__wrap_iter<const char16_t*>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<const char16_t*>>) ==
    8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __this,
    class std::__wrap_iter<const char16_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEE4baseEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>> const*
        __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__wrap_iter<const char16_t*> (
        ::std::reverse_iterator<std::__wrap_iter<const char16_t*>>::*)()
         const) &
    ::std::reverse_iterator<std::__wrap_iter<const char16_t*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<const char*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<const char*>>) == 8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __this,
    class std::__wrap_iter<const char*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEE4baseEv(
    class std::__wrap_iter<const char*>* __return,
    class std::reverse_iterator<std::__wrap_iter<const char*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<const char*> (
                  ::std::reverse_iterator<std::__wrap_iter<const char*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<const char*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<const int*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<const int*>>) == 8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const int*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const int*>>* __this,
    class std::__wrap_iter<const int*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEE4baseEv(
    class std::__wrap_iter<const int*>* __return,
    class std::reverse_iterator<std::__wrap_iter<const int*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::reverse_iterator<std::__wrap_iter<const int*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<const int*>>::base);

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<std::__wrap_iter<
            std::basic_string_view<char, std::char_traits<char>>*>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<
                std::basic_string_view<char, std::char_traits<char>>*>>) == 8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEC1ES7_(
    class std::reverse_iterator<std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>>* __this,
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEE4baseEv(
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>* __return,
    class std::reverse_iterator<std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*> (
        ::std::reverse_iterator<std::__wrap_iter<
            std::basic_string_view<char, std::char_traits<char>>*>>::*)()
         const) &
    ::std::reverse_iterator<std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<char*>>) == 8);
static_assert(alignof(class std::reverse_iterator<std::__wrap_iter<char*>>) ==
              8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<char*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<char*>>* __this,
    class std::__wrap_iter<char*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEE4baseEv(
    class std::__wrap_iter<char*>* __return,
    class std::reverse_iterator<std::__wrap_iter<char*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<char*> (
                  ::std::reverse_iterator<std::__wrap_iter<char*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<char*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<int*>>) == 8);
static_assert(alignof(class std::reverse_iterator<std::__wrap_iter<int*>>) ==
              8);

extern "C" void
__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<int*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<int*>>* __this,
    class std::__wrap_iter<int*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEE4baseEv(
    class std::__wrap_iter<int*>* __return,
    class std::reverse_iterator<std::__wrap_iter<int*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<int*> (
                  ::std::reverse_iterator<std::__wrap_iter<int*>>::*)() const) &
              ::std::reverse_iterator<std::__wrap_iter<int*>>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char32_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char32_t*>) == 8);

extern "C" void __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorIPKDiEC1Ev(
    class std::reverse_iterator<const char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorIPKDiEC1ES2_(
    class std::reverse_iterator<const char32_t*>* __this, char32_t const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char32_t const*
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorIPKDiE4baseEv(
    class std::reverse_iterator<const char32_t*> const* __this) {
  return __this->base();
}

static_assert((char32_t const* (::std::reverse_iterator<const char32_t*>::*)()
                   const) &
              ::std::reverse_iterator<const char32_t*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char16_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char16_t*>) == 8);

extern "C" void __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorIPKDsEC1Ev(
    class std::reverse_iterator<const char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorIPKDsEC1ES2_(
    class std::reverse_iterator<const char16_t*>* __this, char16_t const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char16_t const*
__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorIPKDsE4baseEv(
    class std::reverse_iterator<const char16_t*> const* __this) {
  return __this->base();
}

static_assert((char16_t const* (::std::reverse_iterator<const char16_t*>::*)()
                   const) &
              ::std::reverse_iterator<const char16_t*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char8_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char8_t*>) == 8);

extern "C" void __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorIPKDuEC1Ev(
    class std::reverse_iterator<const char8_t*>* __this) {
  crubit::construct_at(__this);
}

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

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<char32_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<char32_t*>) == 8);

extern "C" void __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPDiEC1Ev(
    class std::__wrap_iter<char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPDiEplEl(
    class std::__wrap_iter<char32_t*>* __return,
    class std::__wrap_iter<char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<char32_t*> (
                  ::std::__wrap_iter<char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char32_t*>::operator+);

extern "C" class std::__wrap_iter<char32_t*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPDiEpLEl(
    class std::__wrap_iter<char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<char32_t*> &
               (::std::__wrap_iter<char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char32_t*>::operator+=);

extern "C" void __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPDiEmiEl(
    class std::__wrap_iter<char32_t*>* __return,
    class std::__wrap_iter<char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<char32_t*> (
                  ::std::__wrap_iter<char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char32_t*>::operator-);

extern "C" class std::__wrap_iter<char32_t*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPDiEmIEl(
    class std::__wrap_iter<char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<char32_t*> &
               (::std::__wrap_iter<char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char32_t*>::operator-=);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<char16_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<char16_t*>) == 8);

extern "C" void __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPDsEC1Ev(
    class std::__wrap_iter<char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPDsEplEl(
    class std::__wrap_iter<char16_t*>* __return,
    class std::__wrap_iter<char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<char16_t*> (
                  ::std::__wrap_iter<char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char16_t*>::operator+);

extern "C" class std::__wrap_iter<char16_t*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPDsEpLEl(
    class std::__wrap_iter<char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<char16_t*> &
               (::std::__wrap_iter<char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char16_t*>::operator+=);

extern "C" void __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPDsEmiEl(
    class std::__wrap_iter<char16_t*>* __return,
    class std::__wrap_iter<char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<char16_t*> (
                  ::std::__wrap_iter<char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char16_t*>::operator-);

extern "C" class std::__wrap_iter<char16_t*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPDsEmIEl(
    class std::__wrap_iter<char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<char16_t*> &
               (::std::__wrap_iter<char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char16_t*>::operator-=);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const char32_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<const char32_t*>) == 8);

extern "C" void __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPKDiEC1Ev(
    class std::__wrap_iter<const char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPKDiEplEl(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::__wrap_iter<const char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char32_t*>::operator+);

extern "C" class std::__wrap_iter<const char32_t*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPKDiEpLEl(
    class std::__wrap_iter<const char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> &
               (::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char32_t*>::operator+=);

extern "C" void __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPKDiEmiEl(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::__wrap_iter<const char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char32_t*>::operator-);

extern "C" class std::__wrap_iter<const char32_t*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPKDiEmIEl(
    class std::__wrap_iter<const char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> &
               (::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char32_t*>::operator-=);

extern "C" char32_t const*
__rust_thunk__6dc0ff60__ZNKSt3__u11__wrap_iterIPKDiEixEl(
    class std::__wrap_iter<const char32_t*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char32_t const& (::std::__wrap_iter<const char32_t*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<const char32_t*>::operator[]);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const char16_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<const char16_t*>) == 8);

extern "C" void __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPKDsEC1Ev(
    class std::__wrap_iter<const char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPKDsEplEl(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::__wrap_iter<const char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char16_t*>::operator+);

extern "C" class std::__wrap_iter<const char16_t*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPKDsEpLEl(
    class std::__wrap_iter<const char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> &
               (::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char16_t*>::operator+=);

extern "C" void __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPKDsEmiEl(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::__wrap_iter<const char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char16_t*>::operator-);

extern "C" class std::__wrap_iter<const char16_t*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPKDsEmIEl(
    class std::__wrap_iter<const char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> &
               (::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char16_t*>::operator-=);

extern "C" char16_t const*
__rust_thunk__6dc0ff60__ZNKSt3__u11__wrap_iterIPKDsEixEl(
    class std::__wrap_iter<const char16_t*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char16_t const& (::std::__wrap_iter<const char16_t*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<const char16_t*>::operator[]);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const char*>) == 8);
static_assert(alignof(class std::__wrap_iter<const char*>) == 8);

extern "C" void __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPKcEC1Ev(
    class std::__wrap_iter<const char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPKcEplEl(
    class std::__wrap_iter<const char*>* __return,
    class std::__wrap_iter<const char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const char*> (
                  ::std::__wrap_iter<const char*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char*>::operator+);

extern "C" class std::__wrap_iter<const char*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPKcEpLEl(
    class std::__wrap_iter<const char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const char*> &
               (::std::__wrap_iter<const char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char*>::operator+=);

extern "C" void __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPKcEmiEl(
    class std::__wrap_iter<const char*>* __return,
    class std::__wrap_iter<const char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const char*> (
                  ::std::__wrap_iter<const char*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char*>::operator-);

extern "C" class std::__wrap_iter<const char*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPKcEmIEl(
    class std::__wrap_iter<const char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const char*> &
               (::std::__wrap_iter<const char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char*>::operator-=);

extern "C" char const* __rust_thunk__6dc0ff60__ZNKSt3__u11__wrap_iterIPKcEixEl(
    class std::__wrap_iter<const char*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char const& (::std::__wrap_iter<const char*>::*)(ptrdiff_t)
                   const) &
              ::std::__wrap_iter<const char*>::operator[]);

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

static_assert(
    CRUBIT_SIZEOF(class std::__wrap_iter<
                  std::basic_string_view<char, std::char_traits<char>>*>) == 8);
static_assert(alignof(class std::__wrap_iter<
                      std::basic_string_view<char, std::char_traits<char>>*>) ==
              8);

extern "C" void
__rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEC1Ev(
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEplEl(
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>* __return,
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert(
    (class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*> (
        ::std::__wrap_iter<
            std::basic_string_view<char, std::char_traits<char>>*>::*)(
        ptrdiff_t) const) &
    ::std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>::operator+);

extern "C" class std::__wrap_iter<
    std::basic_string_view<char, std::char_traits<char>>*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEpLEl(
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert(
    (class std::__wrap_iter<
         std::basic_string_view<char, std::char_traits<char>>*> &
     (::std::__wrap_iter<
         std::basic_string_view<char, std::char_traits<char>>*>::*)(
         ptrdiff_t)) &
    ::std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>::operator+=);

extern "C" void
__rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEmiEl(
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>* __return,
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert(
    (class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*> (
        ::std::__wrap_iter<
            std::basic_string_view<char, std::char_traits<char>>*>::*)(
        ptrdiff_t) const) &
    ::std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>::operator-);

extern "C" class std::__wrap_iter<
    std::basic_string_view<char, std::char_traits<char>>*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEmIEl(
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert(
    (class std::__wrap_iter<
         std::basic_string_view<char, std::char_traits<char>>*> &
     (::std::__wrap_iter<
         std::basic_string_view<char, std::char_traits<char>>*>::*)(
         ptrdiff_t)) &
    ::std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>::operator-=);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<char*>) == 8);
static_assert(alignof(class std::__wrap_iter<char*>) == 8);

extern "C" void __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPcEC1Ev(
    class std::__wrap_iter<char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPcEplEl(
    class std::__wrap_iter<char*>* __return,
    class std::__wrap_iter<char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<char*> (::std::__wrap_iter<char*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<char*>::operator+);

extern "C" class std::__wrap_iter<char*>*
__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPcEpLEl(
    class std::__wrap_iter<char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<char*> &
               (::std::__wrap_iter<char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char*>::operator+=);

extern "C" void __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPcEmiEl(
    class std::__wrap_iter<char*>* __return,
    class std::__wrap_iter<char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<char*> (::std::__wrap_iter<char*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<char*>::operator-);

extern "C" class std::__wrap_iter<char*>*
__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPcEmIEl(
    class std::__wrap_iter<char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<char*> &
               (::std::__wrap_iter<char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char*>::operator-=);

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
    class std::reverse_iterator<std::__wrap_iter<const int*>>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const int*>> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__dc1b110a__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv(
    class std::reverse_iterator<std::__wrap_iter<const int*>>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const int*>> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::rend);

static_assert(
    CRUBIT_SIZEOF(
        class std::span<std::basic_string_view<char, std::char_traits<char>>,
                        18446744073709551615UL>) == 16);
static_assert(
    alignof(
        class std::span<std::basic_string_view<char, std::char_traits<char>>,
                        18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__9ef48370__ZNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEC1Ev(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__eb5fa8a1__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5firstEm(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>* __return,
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert(
    (class std::span<std::basic_string_view<char, std::char_traits<char>>,
                     18446744073709551615UL> (
        ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>::*)(size_t) const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::first);

extern "C" void
__rust_thunk__cb04abd4__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4lastEm(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>* __return,
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert(
    (class std::span<std::basic_string_view<char, std::char_traits<char>>,
                     18446744073709551615UL> (
        ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>::*)(size_t) const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::last);

extern "C" void
__rust_thunk__f8d72bf1__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE7subspanEmm(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>* __return,
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this,
    size_t __offset, size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert(
    (class std::span<std::basic_string_view<char, std::char_traits<char>>,
                     18446744073709551615UL> (
        ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>::*)(size_t, size_t) const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__e58d956f__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4sizeEv(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::span<std::basic_string_view<char, std::char_traits<char>>,
                         18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::size);

extern "C" size_t
__rust_thunk__1a2eb8d0__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE10size_bytesEv(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert(
    (size_t (::std::span<std::basic_string_view<char, std::char_traits<char>>,
                         18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__5eda390c__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5emptyEv(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::span<std::basic_string_view<char, std::char_traits<char>>,
                       18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::empty);

extern "C" ::std::__u::string_view*
__rust_thunk__02898003__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5frontEv(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert(
    (::std::__u::string_view &
     (::std::span<std::basic_string_view<char, std::char_traits<char>>,
                  18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::front);

extern "C" ::std::__u::string_view*
__rust_thunk__f4827081__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4backEv(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert(
    (::std::__u::string_view &
     (::std::span<std::basic_string_view<char, std::char_traits<char>>,
                  18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::back);

extern "C" ::std::__u::string_view*
__rust_thunk__e6274c04__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4dataEv(
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  return __this->data();
}

static_assert(
    (::std::__u::string_view *
     (::std::span<std::basic_string_view<char, std::char_traits<char>>,
                  18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::data);

extern "C" void
__rust_thunk__4d8e3070__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5beginEv(
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>* __return,
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert(
    (class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*> (
        ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__b3c9f034__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE3endEv(
    class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>* __return,
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert(
    (class std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*> (
        ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::end);

extern "C" void
__rust_thunk__3e61f7cc__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE6rbeginEv(
    class std::reverse_iterator<std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>>* __return,
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<
         std::basic_string_view<char, std::char_traits<char>>*>> (
        ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__dc1b110a__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4rendEv(
    class std::reverse_iterator<std::__wrap_iter<
        std::basic_string_view<char, std::char_traits<char>>*>>* __return,
    class std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<
         std::basic_string_view<char, std::char_traits<char>>*>> (
        ::std::span<std::basic_string_view<char, std::char_traits<char>>,
                    18446744073709551615UL>::*)() const) &
    ::std::span<std::basic_string_view<char, std::char_traits<char>>,
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
    class std::reverse_iterator<std::__wrap_iter<int*>>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<int*>> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__dc1b110a__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv(
    class std::reverse_iterator<std::__wrap_iter<int*>>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<int*>> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::rend);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>) ==
    16);
static_assert(
    alignof(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>) ==
    8);

extern "C" void
__rust_thunk__7fd65fbf__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1Ev(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__4e959746__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1EPKDi(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    char32_t const* __s) {
  crubit::construct_at(__this, __s);
}

extern "C" char32_t const*
__rust_thunk__c08a9682__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5beginEv(
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
__rust_thunk__5dcda818__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE3endEv(
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
__rust_thunk__858d955d__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6cbeginEv(
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
__rust_thunk__51f15331__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4cendEv(
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
__rust_thunk__f480140b__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6rbeginEv(
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
__rust_thunk__551fce2a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4rendEv(
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
__rust_thunk__84b7a540__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE7crbeginEv(
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
__rust_thunk__12ead93e__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5crendEv(
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
__rust_thunk__ef59537e__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4sizeEv(
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
__rust_thunk__45ef7262__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6lengthEv(
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
__rust_thunk__6730efad__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE8max_sizeEv(
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
__rust_thunk__30e7ccff__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5emptyEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::empty);

extern "C" char32_t const*
__rust_thunk__899d60e2__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEixEm(
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
__rust_thunk__28a503e2__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE2atEm(
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
__rust_thunk__78239758__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5frontEv(
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
__rust_thunk__f12840fd__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4backEv(
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
__rust_thunk__ec31bd0c__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4dataEv(
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
__rust_thunk__02ccdf16__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_prefixEm(
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
__rust_thunk__9b80b3ee__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_suffixEm(
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
__rust_thunk__424a0b8a__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4swapERS3_(
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
__rust_thunk__5bb68eaf__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4copyEPDimm(
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
__rust_thunk__b263e1c7__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6substrEmm(
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
__rust_thunk__7fd65fbf__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1Ev(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__4e959746__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1EPKDs(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    char16_t const* __s) {
  crubit::construct_at(__this, __s);
}

extern "C" char16_t const*
__rust_thunk__c08a9682__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5beginEv(
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
__rust_thunk__5dcda818__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE3endEv(
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
__rust_thunk__858d955d__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6cbeginEv(
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
__rust_thunk__51f15331__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4cendEv(
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
__rust_thunk__f480140b__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6rbeginEv(
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
__rust_thunk__551fce2a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4rendEv(
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
__rust_thunk__84b7a540__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE7crbeginEv(
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
__rust_thunk__12ead93e__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5crendEv(
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
__rust_thunk__ef59537e__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4sizeEv(
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
__rust_thunk__45ef7262__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6lengthEv(
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
__rust_thunk__6730efad__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE8max_sizeEv(
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
__rust_thunk__30e7ccff__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5emptyEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::empty);

extern "C" char16_t const*
__rust_thunk__899d60e2__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEixEm(
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
__rust_thunk__28a503e2__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE2atEm(
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
__rust_thunk__78239758__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5frontEv(
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
__rust_thunk__f12840fd__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4backEv(
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
__rust_thunk__ec31bd0c__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4dataEv(
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
__rust_thunk__02ccdf16__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_prefixEm(
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
__rust_thunk__9b80b3ee__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_suffixEm(
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
__rust_thunk__424a0b8a__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4swapERS3_(
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
__rust_thunk__5bb68eaf__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4copyEPDsmm(
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
__rust_thunk__b263e1c7__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6substrEmm(
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
__rust_thunk__7fd65fbf__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEC1Ev(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__f480140b__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6rbeginEv(
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
__rust_thunk__551fce2a__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4rendEv(
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
__rust_thunk__84b7a540__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE7crbeginEv(
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
__rust_thunk__12ead93e__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5crendEv(
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
__rust_thunk__ef59537e__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4sizeEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::size);

extern "C" size_t
__rust_thunk__45ef7262__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6lengthEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->length();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::length);

extern "C" size_t
__rust_thunk__6730efad__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE8max_sizeEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->max_size();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::max_size);

extern "C" bool
__rust_thunk__30e7ccff__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5emptyEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::empty);

extern "C" void
__rust_thunk__02ccdf16__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_prefixEm(
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
__rust_thunk__9b80b3ee__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_suffixEm(
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
__rust_thunk__424a0b8a__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4swapERS3_(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __other) {
  __this->swap(*__other);
}

static_assert(
    (void (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>&)) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::swap);

extern "C" void
__rust_thunk__b263e1c7__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6substrEmm(
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
__rust_thunk__33849511__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4findES3_m(
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
__rust_thunk__c3ef4171__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5rfindES3_m(
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
__rust_thunk__b1ff3b00__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13find_first_ofES3_m(
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
__rust_thunk__b3f86d06__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE12find_last_ofES3_m(
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
__rust_thunk__59c574e9__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE17find_first_not_ofES3_m(
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
__rust_thunk__6aa501ed__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE16find_last_not_ofES3_m(
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
__rust_thunk__3815fa04__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE11starts_withES3_(
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
__rust_thunk__ca0823ad__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE9ends_withES3_(
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

static_assert(sizeof(struct std::ranges::views::__elements::__fn<0UL>) == 1);
static_assert(alignof(struct std::ranges::views::__elements::__fn<0UL>) == 1);

extern "C" void
__rust_thunk__49cbb889__ZNSt3__u6ranges5views10__elements4__fnILm0EEC1Ev(
    struct std::ranges::views::__elements::__fn<0UL>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::ranges::views::__elements::__fn<1UL>) == 1);
static_assert(alignof(struct std::ranges::views::__elements::__fn<1UL>) == 1);

extern "C" void
__rust_thunk__49cbb889__ZNSt3__u6ranges5views10__elements4__fnILm1EEC1Ev(
    struct std::ranges::views::__elements::__fn<1UL>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::fpos<__mbstate_t>) == 16);
static_assert(alignof(class std::fpos<__mbstate_t>) == 8);

extern "C" void __rust_thunk__4aca90b4__ZNSt3__u4fposI11__mbstate_tEC1Ex(
    class std::fpos<__mbstate_t>* __this, long long __off) {
  crubit::construct_at(__this, __off);
}

extern "C" class std::fpos<__mbstate_t>*
__rust_thunk__cbfbce58__ZNSt3__u4fposI11__mbstate_tEpLEx(
    class std::fpos<__mbstate_t>* __this, long long __off) {
  return std::addressof(__this->operator+=(__off));
}

static_assert((class std::fpos<__mbstate_t> &
               (::std::fpos<__mbstate_t>::*)(long long)) &
              ::std::fpos<__mbstate_t>::operator+=);

extern "C" void __rust_thunk__348b348c__ZNKSt3__u4fposI11__mbstate_tEplEx(
    class std::fpos<__mbstate_t>* __return,
    class std::fpos<__mbstate_t> const* __this, long long __off) {
  new (__return) auto(__this->operator+(__off));
}

static_assert((class std::fpos<__mbstate_t> (::std::fpos<__mbstate_t>::*)(
                  long long) const) &
              ::std::fpos<__mbstate_t>::operator+);

extern "C" class std::fpos<__mbstate_t>*
__rust_thunk__e3e32cb6__ZNSt3__u4fposI11__mbstate_tEmIEx(
    class std::fpos<__mbstate_t>* __this, long long __off) {
  return std::addressof(__this->operator-=(__off));
}

static_assert((class std::fpos<__mbstate_t> &
               (::std::fpos<__mbstate_t>::*)(long long)) &
              ::std::fpos<__mbstate_t>::operator-=);

extern "C" void __rust_thunk__fab8d593__ZNKSt3__u4fposI11__mbstate_tEmiEx(
    class std::fpos<__mbstate_t>* __return,
    class std::fpos<__mbstate_t> const* __this, long long __off) {
  new (__return) auto(__this->operator-(__off));
}

static_assert((class std::fpos<__mbstate_t> (::std::fpos<__mbstate_t>::*)(
                  long long) const) &
              ::std::fpos<__mbstate_t>::operator-);

static_assert(
    CRUBIT_SIZEOF(struct std::__allocation_result<char32_t*, unsigned long>) ==
    16);
static_assert(
    alignof(struct std::__allocation_result<char32_t*, unsigned long>) == 8);
static_assert(CRUBIT_OFFSET_OF(
                  ptr,
                  struct std::__allocation_result<char32_t*, unsigned long>) ==
              0);
static_assert(CRUBIT_OFFSET_OF(
                  count,
                  struct std::__allocation_result<char32_t*, unsigned long>) ==
              8);

extern "C" void
__rust_thunk__da1528d7__ZNSt3__u19__allocation_resultIPDimEC1ES1_m(
    struct std::__allocation_result<char32_t*, unsigned long>* __this,
    char32_t* __ptr, unsigned long __count) {
  crubit::construct_at(__this, __ptr, __count);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__allocation_result<char16_t*, unsigned long>) ==
    16);
static_assert(
    alignof(struct std::__allocation_result<char16_t*, unsigned long>) == 8);
static_assert(CRUBIT_OFFSET_OF(
                  ptr,
                  struct std::__allocation_result<char16_t*, unsigned long>) ==
              0);
static_assert(CRUBIT_OFFSET_OF(
                  count,
                  struct std::__allocation_result<char16_t*, unsigned long>) ==
              8);

extern "C" void
__rust_thunk__da1528d7__ZNSt3__u19__allocation_resultIPDsmEC1ES1_m(
    struct std::__allocation_result<char16_t*, unsigned long>* __this,
    char16_t* __ptr, unsigned long __count) {
  crubit::construct_at(__this, __ptr, __count);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__allocation_result<char*, unsigned long>) == 16);
static_assert(alignof(struct std::__allocation_result<char*, unsigned long>) ==
              8);
static_assert(CRUBIT_OFFSET_OF(
                  ptr, struct std::__allocation_result<char*, unsigned long>) ==
              0);
static_assert(CRUBIT_OFFSET_OF(
                  count,
                  struct std::__allocation_result<char*, unsigned long>) == 8);

extern "C" void
__rust_thunk__da1528d7__ZNSt3__u19__allocation_resultIPcmEC1ES1_m(
    struct std::__allocation_result<char*, unsigned long>* __this, char* __ptr,
    unsigned long __count) {
  crubit::construct_at(__this, __ptr, __count);
}

static_assert(sizeof(class std::ratio<1000000000000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000000000000L, 1L>) == 1);

extern "C" void
__rust_thunk__1c277543__ZNSt3__u5ratioILl1000000000000000000ELl1EEC1Ev(
    class std::ratio<1000000000000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000000000L, 1L>) == 1);

extern "C" void
__rust_thunk__1c277543__ZNSt3__u5ratioILl1000000000000000ELl1EEC1Ev(
    class std::ratio<1000000000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000000L, 1L>) == 1);

extern "C" void
__rust_thunk__1c277543__ZNSt3__u5ratioILl1000000000000ELl1EEC1Ev(
    class std::ratio<1000000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl1000000000ELl1EEC1Ev(
    class std::ratio<1000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl1000000ELl1EEC1Ev(
    class std::ratio<1000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl1000ELl1EEC1Ev(
    class std::ratio<1000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<100L, 1L>) == 1);
static_assert(alignof(class std::ratio<100L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl100ELl1EEC1Ev(
    class std::ratio<100L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<10L, 1L>) == 1);
static_assert(alignof(class std::ratio<10L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl10ELl1EEC1Ev(
    class std::ratio<10L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000000000000L>) == 1);

extern "C" void
__rust_thunk__1c277543__ZNSt3__u5ratioILl1ELl1000000000000000000EEC1Ev(
    class std::ratio<1L, 1000000000000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000000000L>) == 1);

extern "C" void
__rust_thunk__1c277543__ZNSt3__u5ratioILl1ELl1000000000000000EEC1Ev(
    class std::ratio<1L, 1000000000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000000L>) == 1);

extern "C" void
__rust_thunk__1c277543__ZNSt3__u5ratioILl1ELl1000000000000EEC1Ev(
    class std::ratio<1L, 1000000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl1ELl1000000000EEC1Ev(
    class std::ratio<1L, 1000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl1ELl1000000EEC1Ev(
    class std::ratio<1L, 1000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl1ELl1000EEC1Ev(
    class std::ratio<1L, 1000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 100L>) == 1);
static_assert(alignof(class std::ratio<1L, 100L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl1ELl100EEC1Ev(
    class std::ratio<1L, 100L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 10L>) == 1);
static_assert(alignof(class std::ratio<1L, 10L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl1ELl10EEC1Ev(
    class std::ratio<1L, 10L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1L>) == 1);
static_assert(alignof(class std::ratio<1L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl1ELl1EEC1Ev(
    class std::ratio<1L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<2629746L, 1L>) == 1);
static_assert(alignof(class std::ratio<2629746L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl2629746ELl1EEC1Ev(
    class std::ratio<2629746L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<31556952L, 1L>) == 1);
static_assert(alignof(class std::ratio<31556952L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl31556952ELl1EEC1Ev(
    class std::ratio<31556952L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<3600L, 1L>) == 1);
static_assert(alignof(class std::ratio<3600L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl3600ELl1EEC1Ev(
    class std::ratio<3600L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<604800L, 1L>) == 1);
static_assert(alignof(class std::ratio<604800L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl604800ELl1EEC1Ev(
    class std::ratio<604800L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<60L, 1L>) == 1);
static_assert(alignof(class std::ratio<60L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl60ELl1EEC1Ev(
    class std::ratio<60L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<86400L, 1L>) == 1);
static_assert(alignof(class std::ratio<86400L, 1L>) == 1);

extern "C" void __rust_thunk__1c277543__ZNSt3__u5ratioILl86400ELl1EEC1Ev(
    class std::ratio<86400L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<int, std::ratio<2629746L, 1L>>) ==
    4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<2629746L, 1L>>) == 4);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<2629746L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<int, std::ratio<31556952L, 1L>>) == 4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<31556952L, 1L>>) == 4);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<31556952L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<int, std::ratio<604800L, 1L>>) ==
    4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<604800L, 1L>>) == 4);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<604800L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<int, std::ratio<86400L, 1L>>) ==
    4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<86400L, 1L>>) == 4);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<86400L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long, std::ratio<3600L, 1L>>) ==
    8);
static_assert(
    alignof(class std::chrono::duration<long, std::ratio<3600L, 1L>>) == 8);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEC1Ev(
    class std::chrono::duration<long, std::ratio<3600L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long, std::ratio<60L, 1L>>) == 8);
static_assert(alignof(class std::chrono::duration<long, std::ratio<60L, 1L>>) ==
              8);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEC1Ev(
    class std::chrono::duration<long, std::ratio<60L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<long long, std::ratio<1L, 1000000000L>>) ==
    8);
static_assert(
    alignof(
        class std::chrono::duration<long long, std::ratio<1L, 1000000000L>>) ==
    8);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEC1Ev(
    class std::chrono::duration<long long, std::ratio<1L, 1000000000L>>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<long long, std::ratio<1L, 1000000L>>) == 8);
static_assert(
    alignof(class std::chrono::duration<long long, std::ratio<1L, 1000000L>>) ==
    8);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEC1Ev(
    class std::chrono::duration<long long, std::ratio<1L, 1000000L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<long long, std::ratio<1L, 1000L>>) == 8);
static_assert(
    alignof(class std::chrono::duration<long long, std::ratio<1L, 1000L>>) ==
    8);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEC1Ev(
    class std::chrono::duration<long long, std::ratio<1L, 1000L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long long, std::ratio<1L, 1L>>) ==
    8);
static_assert(
    alignof(class std::chrono::duration<long long, std::ratio<1L, 1L>>) == 8);

extern "C" void
__rust_thunk__ee039b0b__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEC1Ev(
    class std::chrono::duration<long long, std::ratio<1L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::time_point<
            std::chrono::steady_clock,
            std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>) ==
    8);
static_assert(
    alignof(class std::chrono::time_point<
            std::chrono::steady_clock,
            std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>) ==
    8);

extern "C" void
__rust_thunk__854ddc95__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__448e1518__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>* __this,
    class std::chrono::duration<long long, std::ratio<1L, 1000000000L>> const*
        __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::time_point<
                  std::chrono::system_clock,
                  std::chrono::duration<int, std::ratio<86400L, 1L>>>) == 4);
static_assert(alignof(class std::chrono::time_point<
                      std::chrono::system_clock,
                      std::chrono::duration<int, std::ratio<86400L, 1L>>>) ==
              4);

extern "C" void
__rust_thunk__854ddc95__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<int, std::ratio<86400L, 1L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__448e1518__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<int, std::ratio<86400L, 1L>>>* __this,
    class std::chrono::duration<int, std::ratio<86400L, 1L>> const* __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::time_point<
            std::chrono::system_clock,
            std::chrono::duration<long long, std::ratio<1L, 1000000L>>>) == 8);
static_assert(
    alignof(class std::chrono::time_point<
            std::chrono::system_clock,
            std::chrono::duration<long long, std::ratio<1L, 1000000L>>>) == 8);

extern "C" void
__rust_thunk__854ddc95__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__448e1518__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000L>>>* __this,
    class std::chrono::duration<long long, std::ratio<1L, 1000000L>> const*
        __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::time_point<
                  std::chrono::system_clock,
                  std::chrono::duration<long long, std::ratio<1L, 1L>>>) == 8);
static_assert(alignof(class std::chrono::time_point<
                      std::chrono::system_clock,
                      std::chrono::duration<long long, std::ratio<1L, 1L>>>) ==
              8);

extern "C" void
__rust_thunk__854ddc95__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__448e1518__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1L>>>* __this,
    class std::chrono::duration<long long, std::ratio<1L, 1L>> const* __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char32_t, false>) == 4);
static_assert(alignof(struct std::__atomic_base<char32_t, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<char32_t, false>) ==
              0);

extern "C" struct std::__atomic_base<char32_t, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIDiLb0EEaSERKS1_(
    struct std::__atomic_base<char32_t, false>* __this,
    struct std::__atomic_base<char32_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char32_t, false> &
               (::std::__atomic_base<char32_t, false>::*)(
                   struct std::__atomic_base<char32_t, false> const&)) &
              ::std::__atomic_base<char32_t, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIDiLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char32_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char32_t, false>::*)() const) &
              ::std::__atomic_base<char32_t, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIDiLb0EE10notify_oneEv(
    struct std::__atomic_base<char32_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char32_t, false>::*)()) &
              ::std::__atomic_base<char32_t, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIDiLb0EE10notify_allEv(
    struct std::__atomic_base<char32_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char32_t, false>::*)()) &
              ::std::__atomic_base<char32_t, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIDiLb0EEC1Ev(
    struct std::__atomic_base<char32_t, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIDiLb0EEC1EDi(
    struct std::__atomic_base<char32_t, false>* __this, char32_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char16_t, false>) == 2);
static_assert(alignof(struct std::__atomic_base<char16_t, false>) == 2);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<char16_t, false>) ==
              0);

extern "C" struct std::__atomic_base<char16_t, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIDsLb0EEaSERKS1_(
    struct std::__atomic_base<char16_t, false>* __this,
    struct std::__atomic_base<char16_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char16_t, false> &
               (::std::__atomic_base<char16_t, false>::*)(
                   struct std::__atomic_base<char16_t, false> const&)) &
              ::std::__atomic_base<char16_t, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIDsLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char16_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char16_t, false>::*)() const) &
              ::std::__atomic_base<char16_t, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIDsLb0EE10notify_oneEv(
    struct std::__atomic_base<char16_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char16_t, false>::*)()) &
              ::std::__atomic_base<char16_t, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIDsLb0EE10notify_allEv(
    struct std::__atomic_base<char16_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char16_t, false>::*)()) &
              ::std::__atomic_base<char16_t, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIDsLb0EEC1Ev(
    struct std::__atomic_base<char16_t, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIDsLb0EEC1EDs(
    struct std::__atomic_base<char16_t, false>* __this, char16_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char8_t, false>) == 1);
static_assert(alignof(struct std::__atomic_base<char8_t, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<char8_t, false>) == 0);

extern "C" struct std::__atomic_base<char8_t, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIDuLb0EEaSERKS1_(
    struct std::__atomic_base<char8_t, false>* __this,
    struct std::__atomic_base<char8_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char8_t, false> &
               (::std::__atomic_base<char8_t, false>::*)(
                   struct std::__atomic_base<char8_t, false> const&)) &
              ::std::__atomic_base<char8_t, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIDuLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char8_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char8_t, false>::*)() const) &
              ::std::__atomic_base<char8_t, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIDuLb0EE10notify_oneEv(
    struct std::__atomic_base<char8_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char8_t, false>::*)()) &
              ::std::__atomic_base<char8_t, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIDuLb0EE10notify_allEv(
    struct std::__atomic_base<char8_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char8_t, false>::*)()) &
              ::std::__atomic_base<char8_t, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIDuLb0EEC1Ev(
    struct std::__atomic_base<char8_t, false>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::__atomic_base<signed char, false>) == 1);
static_assert(alignof(struct std::__atomic_base<signed char, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<signed char, false>) ==
              0);

extern "C" struct std::__atomic_base<signed char, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIaLb0EEaSERKS1_(
    struct std::__atomic_base<signed char, false>* __this,
    struct std::__atomic_base<signed char, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<signed char, false> &
               (::std::__atomic_base<signed char, false>::*)(
                   struct std::__atomic_base<signed char, false> const&)) &
              ::std::__atomic_base<signed char, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIaLb0EE12is_lock_freeEv(
    struct std::__atomic_base<signed char, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<signed char, false>::*)() const) &
              ::std::__atomic_base<signed char, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIaLb0EE10notify_oneEv(
    struct std::__atomic_base<signed char, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<signed char, false>::*)()) &
              ::std::__atomic_base<signed char, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIaLb0EE10notify_allEv(
    struct std::__atomic_base<signed char, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<signed char, false>::*)()) &
              ::std::__atomic_base<signed char, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIaLb0EEC1Ev(
    struct std::__atomic_base<signed char, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIaLb0EEC1Ea(
    struct std::__atomic_base<signed char, false>* __this, signed char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<bool, false>) == 1);
static_assert(alignof(struct std::__atomic_base<bool, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<bool, false>) ==
              0);

extern "C" struct std::__atomic_base<bool, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIbLb0EEaSERKS1_(
    struct std::__atomic_base<bool, false>* __this,
    struct std::__atomic_base<bool, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<bool, false> &
               (::std::__atomic_base<bool, false>::*)(
                   struct std::__atomic_base<bool, false> const&)) &
              ::std::__atomic_base<bool, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIbLb0EE12is_lock_freeEv(
    struct std::__atomic_base<bool, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<bool, false>::*)() const) &
              ::std::__atomic_base<bool, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIbLb0EE10notify_oneEv(
    struct std::__atomic_base<bool, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<bool, false>::*)()) &
              ::std::__atomic_base<bool, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIbLb0EE10notify_allEv(
    struct std::__atomic_base<bool, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<bool, false>::*)()) &
              ::std::__atomic_base<bool, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIbLb0EEC1Ev(
    struct std::__atomic_base<bool, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIbLb0EEC1Eb(
    struct std::__atomic_base<bool, false>* __this, bool __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char, false>) == 1);
static_assert(alignof(struct std::__atomic_base<char, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<char, false>) ==
              0);

extern "C" struct std::__atomic_base<char, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIcLb0EEaSERKS1_(
    struct std::__atomic_base<char, false>* __this,
    struct std::__atomic_base<char, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char, false> &
               (::std::__atomic_base<char, false>::*)(
                   struct std::__atomic_base<char, false> const&)) &
              ::std::__atomic_base<char, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIcLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char, false>::*)() const) &
              ::std::__atomic_base<char, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIcLb0EE10notify_oneEv(
    struct std::__atomic_base<char, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char, false>::*)()) &
              ::std::__atomic_base<char, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIcLb0EE10notify_allEv(
    struct std::__atomic_base<char, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char, false>::*)()) &
              ::std::__atomic_base<char, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIcLb0EEC1Ev(
    struct std::__atomic_base<char, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIcLb0EEC1Ec(
    struct std::__atomic_base<char, false>* __this, char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<unsigned char, false>) == 1);
static_assert(alignof(struct std::__atomic_base<unsigned char, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned char, false>) == 0);

extern "C" struct std::__atomic_base<unsigned char, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIhLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned char, false>* __this,
    struct std::__atomic_base<unsigned char, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned char, false> &
               (::std::__atomic_base<unsigned char, false>::*)(
                   struct std::__atomic_base<unsigned char, false> const&)) &
              ::std::__atomic_base<unsigned char, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIhLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned char, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned char, false>::*)() const) &
              ::std::__atomic_base<unsigned char, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIhLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned char, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned char, false>::*)()) &
              ::std::__atomic_base<unsigned char, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIhLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned char, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned char, false>::*)()) &
              ::std::__atomic_base<unsigned char, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIhLb0EEC1Ev(
    struct std::__atomic_base<unsigned char, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIhLb0EEC1Eh(
    struct std::__atomic_base<unsigned char, false>* __this,
    unsigned char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<int, false>) == 4);
static_assert(alignof(struct std::__atomic_base<int, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<int, false>) ==
              0);

extern "C" struct std::__atomic_base<int, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIiLb0EEaSERKS1_(
    struct std::__atomic_base<int, false>* __this,
    struct std::__atomic_base<int, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<int, false> &
               (::std::__atomic_base<int, false>::*)(
                   struct std::__atomic_base<int, false> const&)) &
              ::std::__atomic_base<int, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIiLb0EE12is_lock_freeEv(
    struct std::__atomic_base<int, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<int, false>::*)() const) &
              ::std::__atomic_base<int, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIiLb0EE10notify_oneEv(
    struct std::__atomic_base<int, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<int, false>::*)()) &
              ::std::__atomic_base<int, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIiLb0EE10notify_allEv(
    struct std::__atomic_base<int, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<int, false>::*)()) &
              ::std::__atomic_base<int, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIiLb0EEC1Ev(
    struct std::__atomic_base<int, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIiLb0EEC1Ei(
    struct std::__atomic_base<int, false>* __this, int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned int, false>) ==
              4);
static_assert(alignof(struct std::__atomic_base<unsigned int, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned int, false>) == 0);

extern "C" struct std::__atomic_base<unsigned int, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIjLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned int, false>* __this,
    struct std::__atomic_base<unsigned int, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned int, false> &
               (::std::__atomic_base<unsigned int, false>::*)(
                   struct std::__atomic_base<unsigned int, false> const&)) &
              ::std::__atomic_base<unsigned int, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIjLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned int, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned int, false>::*)() const) &
              ::std::__atomic_base<unsigned int, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIjLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned int, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned int, false>::*)()) &
              ::std::__atomic_base<unsigned int, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIjLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned int, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned int, false>::*)()) &
              ::std::__atomic_base<unsigned int, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIjLb0EEC1Ev(
    struct std::__atomic_base<unsigned int, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIjLb0EEC1Ej(
    struct std::__atomic_base<unsigned int, false>* __this, unsigned int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long, false>) == 8);
static_assert(alignof(struct std::__atomic_base<long, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<long, false>) ==
              0);

extern "C" struct std::__atomic_base<long, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIlLb0EEaSERKS1_(
    struct std::__atomic_base<long, false>* __this,
    struct std::__atomic_base<long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long, false> &
               (::std::__atomic_base<long, false>::*)(
                   struct std::__atomic_base<long, false> const&)) &
              ::std::__atomic_base<long, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIlLb0EE12is_lock_freeEv(
    struct std::__atomic_base<long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<long, false>::*)() const) &
              ::std::__atomic_base<long, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIlLb0EE10notify_oneEv(
    struct std::__atomic_base<long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<long, false>::*)()) &
              ::std::__atomic_base<long, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIlLb0EE10notify_allEv(
    struct std::__atomic_base<long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<long, false>::*)()) &
              ::std::__atomic_base<long, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIlLb0EEC1Ev(
    struct std::__atomic_base<long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIlLb0EEC1El(
    struct std::__atomic_base<long, false>* __this, long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long, false>) ==
              8);
static_assert(alignof(struct std::__atomic_base<unsigned long, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned long, false>) == 0);

extern "C" struct std::__atomic_base<unsigned long, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseImLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned long, false>* __this,
    struct std::__atomic_base<unsigned long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned long, false> &
               (::std::__atomic_base<unsigned long, false>::*)(
                   struct std::__atomic_base<unsigned long, false> const&)) &
              ::std::__atomic_base<unsigned long, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseImLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned long, false>::*)() const) &
              ::std::__atomic_base<unsigned long, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseImLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned long, false>::*)()) &
              ::std::__atomic_base<unsigned long, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseImLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned long, false>::*)()) &
              ::std::__atomic_base<unsigned long, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseImLb0EEC1Ev(
    struct std::__atomic_base<unsigned long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseImLb0EEC1Em(
    struct std::__atomic_base<unsigned long, false>* __this,
    unsigned long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<short, false>) == 2);
static_assert(alignof(struct std::__atomic_base<short, false>) == 2);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<short, false>) ==
              0);

extern "C" struct std::__atomic_base<short, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIsLb0EEaSERKS1_(
    struct std::__atomic_base<short, false>* __this,
    struct std::__atomic_base<short, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<short, false> &
               (::std::__atomic_base<short, false>::*)(
                   struct std::__atomic_base<short, false> const&)) &
              ::std::__atomic_base<short, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIsLb0EE12is_lock_freeEv(
    struct std::__atomic_base<short, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<short, false>::*)() const) &
              ::std::__atomic_base<short, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIsLb0EE10notify_oneEv(
    struct std::__atomic_base<short, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<short, false>::*)()) &
              ::std::__atomic_base<short, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIsLb0EE10notify_allEv(
    struct std::__atomic_base<short, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<short, false>::*)()) &
              ::std::__atomic_base<short, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIsLb0EEC1Ev(
    struct std::__atomic_base<short, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIsLb0EEC1Es(
    struct std::__atomic_base<short, false>* __this, short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned short, false>) ==
              2);
static_assert(alignof(struct std::__atomic_base<unsigned short, false>) == 2);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned short, false>) == 0);

extern "C" struct std::__atomic_base<unsigned short, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseItLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned short, false>* __this,
    struct std::__atomic_base<unsigned short, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned short, false> &
               (::std::__atomic_base<unsigned short, false>::*)(
                   struct std::__atomic_base<unsigned short, false> const&)) &
              ::std::__atomic_base<unsigned short, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseItLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned short, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned short, false>::*)() const) &
              ::std::__atomic_base<unsigned short, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseItLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned short, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned short, false>::*)()) &
              ::std::__atomic_base<unsigned short, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseItLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned short, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned short, false>::*)()) &
              ::std::__atomic_base<unsigned short, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseItLb0EEC1Ev(
    struct std::__atomic_base<unsigned short, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseItLb0EEC1Et(
    struct std::__atomic_base<unsigned short, false>* __this,
    unsigned short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<wchar_t, false>) == 4);
static_assert(alignof(struct std::__atomic_base<wchar_t, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<wchar_t, false>) == 0);

extern "C" struct std::__atomic_base<wchar_t, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIwLb0EEaSERKS1_(
    struct std::__atomic_base<wchar_t, false>* __this,
    struct std::__atomic_base<wchar_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<wchar_t, false> &
               (::std::__atomic_base<wchar_t, false>::*)(
                   struct std::__atomic_base<wchar_t, false> const&)) &
              ::std::__atomic_base<wchar_t, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIwLb0EE12is_lock_freeEv(
    struct std::__atomic_base<wchar_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<wchar_t, false>::*)() const) &
              ::std::__atomic_base<wchar_t, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIwLb0EE10notify_oneEv(
    struct std::__atomic_base<wchar_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<wchar_t, false>::*)()) &
              ::std::__atomic_base<wchar_t, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIwLb0EE10notify_allEv(
    struct std::__atomic_base<wchar_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<wchar_t, false>::*)()) &
              ::std::__atomic_base<wchar_t, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIwLb0EEC1Ev(
    struct std::__atomic_base<wchar_t, false>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long long, false>) == 8);
static_assert(alignof(struct std::__atomic_base<long long, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<long long, false>) ==
              0);

extern "C" struct std::__atomic_base<long long, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIxLb0EEaSERKS1_(
    struct std::__atomic_base<long long, false>* __this,
    struct std::__atomic_base<long long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long long, false> &
               (::std::__atomic_base<long long, false>::*)(
                   struct std::__atomic_base<long long, false> const&)) &
              ::std::__atomic_base<long long, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIxLb0EE12is_lock_freeEv(
    struct std::__atomic_base<long long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<long long, false>::*)() const) &
              ::std::__atomic_base<long long, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIxLb0EE10notify_oneEv(
    struct std::__atomic_base<long long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<long long, false>::*)()) &
              ::std::__atomic_base<long long, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIxLb0EE10notify_allEv(
    struct std::__atomic_base<long long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<long long, false>::*)()) &
              ::std::__atomic_base<long long, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIxLb0EEC1Ev(
    struct std::__atomic_base<long long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIxLb0EEC1Ex(
    struct std::__atomic_base<long long, false>* __this, long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long long, false>) == 8);
static_assert(alignof(struct std::__atomic_base<unsigned long long, false>) ==
              8);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned long long, false>) ==
              0);

extern "C" struct std::__atomic_base<unsigned long long, false>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIyLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned long long, false>* __this,
    struct std::__atomic_base<unsigned long long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<unsigned long long, false> &
     (::std::__atomic_base<unsigned long long, false>::*)(
         struct std::__atomic_base<unsigned long long, false> const&)) &
    ::std::__atomic_base<unsigned long long, false>::operator=);

extern "C" bool
__rust_thunk__9825ff7c__ZNKSt3__u13__atomic_baseIyLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned long long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned long long, false>::*)()
                   const) &
              ::std::__atomic_base<unsigned long long, false>::is_lock_free);

extern "C" void
__rust_thunk__62faee67__ZNSt3__u13__atomic_baseIyLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned long long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned long long, false>::*)()) &
              ::std::__atomic_base<unsigned long long, false>::notify_one);

extern "C" void
__rust_thunk__534ac377__ZNSt3__u13__atomic_baseIyLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned long long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned long long, false>::*)()) &
              ::std::__atomic_base<unsigned long long, false>::notify_all);

extern "C" void __rust_thunk__d19591c1__ZNSt3__u13__atomic_baseIyLb0EEC1Ev(
    struct std::__atomic_base<unsigned long long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6e42e56__ZNSt3__u13__atomic_baseIyLb0EEC1Ey(
    struct std::__atomic_base<unsigned long long, false>* __this,
    unsigned long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char32_t, true>) == 4);
static_assert(alignof(struct std::__atomic_base<char32_t, true>) == 4);

extern "C" struct std::__atomic_base<char32_t, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIDiLb1EEaSEOS1_(
    struct std::__atomic_base<char32_t, true>* __this,
    struct std::__atomic_base<char32_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char32_t, true> &
               (::std::__atomic_base<char32_t, true>::*)(
                   struct std::__atomic_base<char32_t, true>&&)) &
              ::std::__atomic_base<char32_t, true>::operator=);

extern "C" struct std::__atomic_base<char32_t, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIDiLb1EEaSERKS1_(
    struct std::__atomic_base<char32_t, true>* __this,
    struct std::__atomic_base<char32_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char32_t, true> &
               (::std::__atomic_base<char32_t, true>::*)(
                   struct std::__atomic_base<char32_t, true> const&)) &
              ::std::__atomic_base<char32_t, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIDiLb1EEC1Ev(
    struct std::__atomic_base<char32_t, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIDiLb1EEC1EDi(
    struct std::__atomic_base<char32_t, true>* __this, char32_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char16_t, true>) == 2);
static_assert(alignof(struct std::__atomic_base<char16_t, true>) == 2);

extern "C" struct std::__atomic_base<char16_t, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIDsLb1EEaSEOS1_(
    struct std::__atomic_base<char16_t, true>* __this,
    struct std::__atomic_base<char16_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char16_t, true> &
               (::std::__atomic_base<char16_t, true>::*)(
                   struct std::__atomic_base<char16_t, true>&&)) &
              ::std::__atomic_base<char16_t, true>::operator=);

extern "C" struct std::__atomic_base<char16_t, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIDsLb1EEaSERKS1_(
    struct std::__atomic_base<char16_t, true>* __this,
    struct std::__atomic_base<char16_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char16_t, true> &
               (::std::__atomic_base<char16_t, true>::*)(
                   struct std::__atomic_base<char16_t, true> const&)) &
              ::std::__atomic_base<char16_t, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIDsLb1EEC1Ev(
    struct std::__atomic_base<char16_t, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIDsLb1EEC1EDs(
    struct std::__atomic_base<char16_t, true>* __this, char16_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char8_t, true>) == 1);
static_assert(alignof(struct std::__atomic_base<char8_t, true>) == 1);

extern "C" struct std::__atomic_base<char8_t, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIDuLb1EEaSEOS1_(
    struct std::__atomic_base<char8_t, true>* __this,
    struct std::__atomic_base<char8_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char8_t, true> &
               (::std::__atomic_base<char8_t, true>::*)(
                   struct std::__atomic_base<char8_t, true>&&)) &
              ::std::__atomic_base<char8_t, true>::operator=);

extern "C" struct std::__atomic_base<char8_t, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIDuLb1EEaSERKS1_(
    struct std::__atomic_base<char8_t, true>* __this,
    struct std::__atomic_base<char8_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char8_t, true> &
               (::std::__atomic_base<char8_t, true>::*)(
                   struct std::__atomic_base<char8_t, true> const&)) &
              ::std::__atomic_base<char8_t, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIDuLb1EEC1Ev(
    struct std::__atomic_base<char8_t, true>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::__atomic_base<signed char, true>) == 1);
static_assert(alignof(struct std::__atomic_base<signed char, true>) == 1);

extern "C" struct std::__atomic_base<signed char, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIaLb1EEaSEOS1_(
    struct std::__atomic_base<signed char, true>* __this,
    struct std::__atomic_base<signed char, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<signed char, true> &
               (::std::__atomic_base<signed char, true>::*)(
                   struct std::__atomic_base<signed char, true>&&)) &
              ::std::__atomic_base<signed char, true>::operator=);

extern "C" struct std::__atomic_base<signed char, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIaLb1EEaSERKS1_(
    struct std::__atomic_base<signed char, true>* __this,
    struct std::__atomic_base<signed char, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<signed char, true> &
               (::std::__atomic_base<signed char, true>::*)(
                   struct std::__atomic_base<signed char, true> const&)) &
              ::std::__atomic_base<signed char, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIaLb1EEC1Ev(
    struct std::__atomic_base<signed char, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIaLb1EEC1Ea(
    struct std::__atomic_base<signed char, true>* __this, signed char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char, true>) == 1);
static_assert(alignof(struct std::__atomic_base<char, true>) == 1);

extern "C" struct std::__atomic_base<char, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIcLb1EEaSEOS1_(
    struct std::__atomic_base<char, true>* __this,
    struct std::__atomic_base<char, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char, true> &
               (::std::__atomic_base<char, true>::*)(
                   struct std::__atomic_base<char, true>&&)) &
              ::std::__atomic_base<char, true>::operator=);

extern "C" struct std::__atomic_base<char, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIcLb1EEaSERKS1_(
    struct std::__atomic_base<char, true>* __this,
    struct std::__atomic_base<char, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char, true> &
               (::std::__atomic_base<char, true>::*)(
                   struct std::__atomic_base<char, true> const&)) &
              ::std::__atomic_base<char, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIcLb1EEC1Ev(
    struct std::__atomic_base<char, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIcLb1EEC1Ec(
    struct std::__atomic_base<char, true>* __this, char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<unsigned char, true>) == 1);
static_assert(alignof(struct std::__atomic_base<unsigned char, true>) == 1);

extern "C" struct std::__atomic_base<unsigned char, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIhLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned char, true>* __this,
    struct std::__atomic_base<unsigned char, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned char, true> &
               (::std::__atomic_base<unsigned char, true>::*)(
                   struct std::__atomic_base<unsigned char, true>&&)) &
              ::std::__atomic_base<unsigned char, true>::operator=);

extern "C" struct std::__atomic_base<unsigned char, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIhLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned char, true>* __this,
    struct std::__atomic_base<unsigned char, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned char, true> &
               (::std::__atomic_base<unsigned char, true>::*)(
                   struct std::__atomic_base<unsigned char, true> const&)) &
              ::std::__atomic_base<unsigned char, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIhLb1EEC1Ev(
    struct std::__atomic_base<unsigned char, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIhLb1EEC1Eh(
    struct std::__atomic_base<unsigned char, true>* __this, unsigned char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<int, true>) == 4);
static_assert(alignof(struct std::__atomic_base<int, true>) == 4);

extern "C" struct std::__atomic_base<int, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIiLb1EEaSEOS1_(
    struct std::__atomic_base<int, true>* __this,
    struct std::__atomic_base<int, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<int, true> &
               (::std::__atomic_base<int, true>::*)(
                   struct std::__atomic_base<int, true>&&)) &
              ::std::__atomic_base<int, true>::operator=);

extern "C" struct std::__atomic_base<int, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIiLb1EEaSERKS1_(
    struct std::__atomic_base<int, true>* __this,
    struct std::__atomic_base<int, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<int, true> &
               (::std::__atomic_base<int, true>::*)(
                   struct std::__atomic_base<int, true> const&)) &
              ::std::__atomic_base<int, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIiLb1EEC1Ev(
    struct std::__atomic_base<int, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIiLb1EEC1Ei(
    struct std::__atomic_base<int, true>* __this, int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned int, true>) ==
              4);
static_assert(alignof(struct std::__atomic_base<unsigned int, true>) == 4);

extern "C" struct std::__atomic_base<unsigned int, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIjLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned int, true>* __this,
    struct std::__atomic_base<unsigned int, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned int, true> &
               (::std::__atomic_base<unsigned int, true>::*)(
                   struct std::__atomic_base<unsigned int, true>&&)) &
              ::std::__atomic_base<unsigned int, true>::operator=);

extern "C" struct std::__atomic_base<unsigned int, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIjLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned int, true>* __this,
    struct std::__atomic_base<unsigned int, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned int, true> &
               (::std::__atomic_base<unsigned int, true>::*)(
                   struct std::__atomic_base<unsigned int, true> const&)) &
              ::std::__atomic_base<unsigned int, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIjLb1EEC1Ev(
    struct std::__atomic_base<unsigned int, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIjLb1EEC1Ej(
    struct std::__atomic_base<unsigned int, true>* __this, unsigned int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long, true>) == 8);
static_assert(alignof(struct std::__atomic_base<long, true>) == 8);

extern "C" struct std::__atomic_base<long, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIlLb1EEaSEOS1_(
    struct std::__atomic_base<long, true>* __this,
    struct std::__atomic_base<long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<long, true> &
               (::std::__atomic_base<long, true>::*)(
                   struct std::__atomic_base<long, true>&&)) &
              ::std::__atomic_base<long, true>::operator=);

extern "C" struct std::__atomic_base<long, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIlLb1EEaSERKS1_(
    struct std::__atomic_base<long, true>* __this,
    struct std::__atomic_base<long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long, true> &
               (::std::__atomic_base<long, true>::*)(
                   struct std::__atomic_base<long, true> const&)) &
              ::std::__atomic_base<long, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIlLb1EEC1Ev(
    struct std::__atomic_base<long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIlLb1EEC1El(
    struct std::__atomic_base<long, true>* __this, long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long, true>) ==
              8);
static_assert(alignof(struct std::__atomic_base<unsigned long, true>) == 8);

extern "C" struct std::__atomic_base<unsigned long, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseImLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned long, true>* __this,
    struct std::__atomic_base<unsigned long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned long, true> &
               (::std::__atomic_base<unsigned long, true>::*)(
                   struct std::__atomic_base<unsigned long, true>&&)) &
              ::std::__atomic_base<unsigned long, true>::operator=);

extern "C" struct std::__atomic_base<unsigned long, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseImLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned long, true>* __this,
    struct std::__atomic_base<unsigned long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned long, true> &
               (::std::__atomic_base<unsigned long, true>::*)(
                   struct std::__atomic_base<unsigned long, true> const&)) &
              ::std::__atomic_base<unsigned long, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseImLb1EEC1Ev(
    struct std::__atomic_base<unsigned long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseImLb1EEC1Em(
    struct std::__atomic_base<unsigned long, true>* __this, unsigned long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<short, true>) == 2);
static_assert(alignof(struct std::__atomic_base<short, true>) == 2);

extern "C" struct std::__atomic_base<short, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIsLb1EEaSEOS1_(
    struct std::__atomic_base<short, true>* __this,
    struct std::__atomic_base<short, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<short, true> &
               (::std::__atomic_base<short, true>::*)(
                   struct std::__atomic_base<short, true>&&)) &
              ::std::__atomic_base<short, true>::operator=);

extern "C" struct std::__atomic_base<short, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIsLb1EEaSERKS1_(
    struct std::__atomic_base<short, true>* __this,
    struct std::__atomic_base<short, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<short, true> &
               (::std::__atomic_base<short, true>::*)(
                   struct std::__atomic_base<short, true> const&)) &
              ::std::__atomic_base<short, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIsLb1EEC1Ev(
    struct std::__atomic_base<short, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIsLb1EEC1Es(
    struct std::__atomic_base<short, true>* __this, short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned short, true>) ==
              2);
static_assert(alignof(struct std::__atomic_base<unsigned short, true>) == 2);

extern "C" struct std::__atomic_base<unsigned short, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseItLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned short, true>* __this,
    struct std::__atomic_base<unsigned short, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned short, true> &
               (::std::__atomic_base<unsigned short, true>::*)(
                   struct std::__atomic_base<unsigned short, true>&&)) &
              ::std::__atomic_base<unsigned short, true>::operator=);

extern "C" struct std::__atomic_base<unsigned short, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseItLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned short, true>* __this,
    struct std::__atomic_base<unsigned short, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned short, true> &
               (::std::__atomic_base<unsigned short, true>::*)(
                   struct std::__atomic_base<unsigned short, true> const&)) &
              ::std::__atomic_base<unsigned short, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseItLb1EEC1Ev(
    struct std::__atomic_base<unsigned short, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseItLb1EEC1Et(
    struct std::__atomic_base<unsigned short, true>* __this,
    unsigned short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<wchar_t, true>) == 4);
static_assert(alignof(struct std::__atomic_base<wchar_t, true>) == 4);

extern "C" struct std::__atomic_base<wchar_t, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIwLb1EEaSEOS1_(
    struct std::__atomic_base<wchar_t, true>* __this,
    struct std::__atomic_base<wchar_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<wchar_t, true> &
               (::std::__atomic_base<wchar_t, true>::*)(
                   struct std::__atomic_base<wchar_t, true>&&)) &
              ::std::__atomic_base<wchar_t, true>::operator=);

extern "C" struct std::__atomic_base<wchar_t, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIwLb1EEaSERKS1_(
    struct std::__atomic_base<wchar_t, true>* __this,
    struct std::__atomic_base<wchar_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<wchar_t, true> &
               (::std::__atomic_base<wchar_t, true>::*)(
                   struct std::__atomic_base<wchar_t, true> const&)) &
              ::std::__atomic_base<wchar_t, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIwLb1EEC1Ev(
    struct std::__atomic_base<wchar_t, true>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long long, true>) == 8);
static_assert(alignof(struct std::__atomic_base<long long, true>) == 8);

extern "C" struct std::__atomic_base<long long, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIxLb1EEaSEOS1_(
    struct std::__atomic_base<long long, true>* __this,
    struct std::__atomic_base<long long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<long long, true> &
               (::std::__atomic_base<long long, true>::*)(
                   struct std::__atomic_base<long long, true>&&)) &
              ::std::__atomic_base<long long, true>::operator=);

extern "C" struct std::__atomic_base<long long, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIxLb1EEaSERKS1_(
    struct std::__atomic_base<long long, true>* __this,
    struct std::__atomic_base<long long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long long, true> &
               (::std::__atomic_base<long long, true>::*)(
                   struct std::__atomic_base<long long, true> const&)) &
              ::std::__atomic_base<long long, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIxLb1EEC1Ev(
    struct std::__atomic_base<long long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIxLb1EEC1Ex(
    struct std::__atomic_base<long long, true>* __this, long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long long, true>) == 8);
static_assert(alignof(struct std::__atomic_base<unsigned long long, true>) ==
              8);

extern "C" struct std::__atomic_base<unsigned long long, true>*
__rust_thunk__de02b63a__ZNSt3__u13__atomic_baseIyLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned long long, true>* __this,
    struct std::__atomic_base<unsigned long long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned long long, true> &
               (::std::__atomic_base<unsigned long long, true>::*)(
                   struct std::__atomic_base<unsigned long long, true>&&)) &
              ::std::__atomic_base<unsigned long long, true>::operator=);

extern "C" struct std::__atomic_base<unsigned long long, true>*
__rust_thunk__29f8c9c6__ZNSt3__u13__atomic_baseIyLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned long long, true>* __this,
    struct std::__atomic_base<unsigned long long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<unsigned long long, true> &
     (::std::__atomic_base<unsigned long long, true>::*)(
         struct std::__atomic_base<unsigned long long, true> const&)) &
    ::std::__atomic_base<unsigned long long, true>::operator=);

extern "C" void __rust_thunk__5148b2ef__ZNSt3__u13__atomic_baseIyLb1EEC1Ev(
    struct std::__atomic_base<unsigned long long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__23f8d797__ZNSt3__u13__atomic_baseIyLb1EEC1Ey(
    struct std::__atomic_base<unsigned long long, true>* __this,
    unsigned long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::atomic<char8_t>) == 1);
static_assert(alignof(struct std::atomic<char8_t>) == 1);

extern "C" void
__rust_thunk__31a73ad2__ZNSt3__u6atomicIDuEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<char8_t>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::atomic<long>) == 8);
static_assert(alignof(struct std::atomic<long>) == 8);

extern "C" void
__rust_thunk__31a73ad2__ZNSt3__u6atomicIlEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<long>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__7c439097__ZNSt3__u6atomicIlEC1El(
    struct std::atomic<long>* __this, long __d) {
  crubit::construct_at(__this, __d);
}

extern "C" long __rust_thunk__758c5b09__ZNSt3__u6atomicIlEaSEl(
    struct std::atomic<long>* __this, long __d) {
  return __this->operator=(__d);
}

static_assert((long (::std::atomic<long>::*)(long)) &
              ::std::atomic<long>::operator=);

static_assert(CRUBIT_SIZEOF(struct std::atomic<unsigned long>) == 8);
static_assert(alignof(struct std::atomic<unsigned long>) == 8);

extern "C" void
__rust_thunk__31a73ad2__ZNSt3__u6atomicImEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<unsigned long>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__7c439097__ZNSt3__u6atomicImEC1Em(
    struct std::atomic<unsigned long>* __this, unsigned long __d) {
  crubit::construct_at(__this, __d);
}

extern "C" unsigned long __rust_thunk__758c5b09__ZNSt3__u6atomicImEaSEm(
    struct std::atomic<unsigned long>* __this, unsigned long __d) {
  return __this->operator=(__d);
}

static_assert((unsigned long (::std::atomic<unsigned long>::*)(unsigned long)) &
              ::std::atomic<unsigned long>::operator=);

static_assert(CRUBIT_SIZEOF(struct std::atomic<wchar_t>) == 4);
static_assert(alignof(struct std::atomic<wchar_t>) == 4);

extern "C" void
__rust_thunk__31a73ad2__ZNSt3__u6atomicIwEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<wchar_t>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_ios<char, std::char_traits<char>>) == 152);
static_assert(alignof(class std::basic_ios<char, std::char_traits<char>>) == 8);

extern "C" bool
__rust_thunk__6650d9b7__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEEntEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->operator!();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::operator!);

extern "C" unsigned int
__rust_thunk__6a8b5925__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE7rdstateEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->rdstate();
}

static_assert(
    (unsigned int (::std::basic_ios<char, std::char_traits<char>>::*)() const) &
    ::std::basic_ios<char, std::char_traits<char>>::rdstate);

extern "C" void
__rust_thunk__476e70db__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEE5clearEj(
    class std::basic_ios<char, std::char_traits<char>>* __this,
    unsigned int __state) {
  __this->clear(__state);
}

static_assert(
    (void (::std::basic_ios<char, std::char_traits<char>>::*)(unsigned int)) &
    ::std::basic_ios<char, std::char_traits<char>>::clear);

extern "C" void
__rust_thunk__021850a7__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEE8setstateEj(
    class std::basic_ios<char, std::char_traits<char>>* __this,
    unsigned int __state) {
  __this->setstate(__state);
}

static_assert(
    (void (::std::basic_ios<char, std::char_traits<char>>::*)(unsigned int)) &
    ::std::basic_ios<char, std::char_traits<char>>::setstate);

extern "C" bool
__rust_thunk__3618c461__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE4goodEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->good();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::good);

extern "C" bool
__rust_thunk__ddea5929__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE3eofEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->eof();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::eof);

extern "C" bool
__rust_thunk__382d904b__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE4failEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->fail();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::fail);

extern "C" bool
__rust_thunk__3c531447__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE3badEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->bad();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::bad);

extern "C" void
__rust_thunk__6709eb5a__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEED1Ev(
    class std::basic_ios<char, std::char_traits<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_ios<char, std::char_traits<char>>*
__rust_thunk__7bc0d408__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEE7copyfmtERKS3_(
    class std::basic_ios<char, std::char_traits<char>>* __this,
    class std::basic_ios<char, std::char_traits<char>> const* __rhs) {
  return std::addressof(__this->copyfmt(*__rhs));
}

static_assert((class std::basic_ios<char, std::char_traits<char>> &
               (::std::basic_ios<char, std::char_traits<char>>::*)(
                   class std::basic_ios<char, std::char_traits<char>> const&)) &
              ::std::basic_ios<char, std::char_traits<char>>::copyfmt);

extern "C" void
__rust_thunk__5bd0175c__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEEC1EPNS_15basic_streambufIcS2_EE(
    class std::basic_ios<char, std::char_traits<char>>* __this,
    class std::basic_streambuf<char, std::char_traits<char>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__9d148f95__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEE5imbueERKNS_6localeE(
    class ::std::__u::locale* __return,
    class std::basic_ios<char, std::char_traits<char>>* __this,
    class ::std::__u::locale const* __loc) {
  new (__return) auto(__this->imbue(*__loc));
}

static_assert((class ::std::__u::locale (
                  ::std::basic_ios<char, std::char_traits<char>>::*)(
                  class ::std::__u::locale const&)) &
              ::std::basic_ios<char, std::char_traits<char>>::imbue);

extern "C" char
__rust_thunk__f4751b7d__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE6narrowEcc(
    class std::basic_ios<char, std::char_traits<char>> const* __this, char __c,
    char __dfault) {
  return __this->narrow(__c, __dfault);
}

static_assert((char (::std::basic_ios<char, std::char_traits<char>>::*)(char,
                                                                        char)
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::narrow);

extern "C" char
__rust_thunk__a396a179__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE5widenEc(
    class std::basic_ios<char, std::char_traits<char>> const* __this,
    char __c) {
  return __this->widen(__c);
}

static_assert((char (::std::basic_ios<char, std::char_traits<char>>::*)(char)
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::widen);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u9basic_iosIcNS_11char_traitsIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acomposable_5fbridging_5fcc(
    class std::basic_ios<char, std::char_traits<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_ios<wchar_t, std::char_traits<wchar_t>>) ==
    152);
static_assert(
    alignof(class std::basic_ios<wchar_t, std::char_traits<wchar_t>>) == 8);

extern "C" bool
__rust_thunk__6650d9b7__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEEntEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->operator!();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::operator!);

extern "C" unsigned int
__rust_thunk__6a8b5925__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE7rdstateEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->rdstate();
}

static_assert(
    (unsigned int (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
         const) &
    ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::rdstate);

extern "C" void
__rust_thunk__476e70db__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEE5clearEj(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned int __state) {
  __this->clear(__state);
}

static_assert((void (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)(
                  unsigned int)) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::clear);

extern "C" void
__rust_thunk__021850a7__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEE8setstateEj(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned int __state) {
  __this->setstate(__state);
}

static_assert((void (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)(
                  unsigned int)) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::setstate);

extern "C" bool
__rust_thunk__3618c461__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE4goodEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->good();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::good);

extern "C" bool
__rust_thunk__ddea5929__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE3eofEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->eof();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::eof);

extern "C" bool
__rust_thunk__382d904b__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE4failEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->fail();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::fail);

extern "C" bool
__rust_thunk__3c531447__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE3badEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->bad();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::bad);

extern "C" void
__rust_thunk__6709eb5a__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEED1Ev(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_ios<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__7bc0d408__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEE7copyfmtERKS3_(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __rhs) {
  return std::addressof(__this->copyfmt(*__rhs));
}

static_assert(
    (class std::basic_ios<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)(
         class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const&)) &
    ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::copyfmt);

extern "C" void
__rust_thunk__5bd0175c__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEEC1EPNS_15basic_streambufIwS2_EE(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__9d148f95__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEE5imbueERKNS_6localeE(
    class ::std::__u::locale* __return,
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this,
    class ::std::__u::locale const* __loc) {
  new (__return) auto(__this->imbue(*__loc));
}

static_assert((class ::std::__u::locale (
                  ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)(
                  class ::std::__u::locale const&)) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::imbue);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u9basic_iosIwNS_11char_traitsIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acomposable_5fbridging_5fcc(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_streambuf<char, std::char_traits<char>>) ==
    64);
static_assert(
    alignof(class std::basic_streambuf<char, std::char_traits<char>>) == 8);

extern "C" void
__rust_thunk__c35c4af4__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEED1Ev(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__2212eb43__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE8pubimbueERKNS_6localeE(
    class ::std::__u::locale* __return,
    class std::basic_streambuf<char, std::char_traits<char>>* __this,
    class ::std::__u::locale const* __loc) {
  new (__return) auto(__this->pubimbue(*__loc));
}

static_assert((class ::std::__u::locale (
                  ::std::basic_streambuf<char, std::char_traits<char>>::*)(
                  class ::std::__u::locale const&)) &
              ::std::basic_streambuf<char, std::char_traits<char>>::pubimbue);

extern "C" void
__rust_thunk__c28eab23__ZNKSt3__u15basic_streambufIcNS_11char_traitsIcEEE6getlocEv(
    class ::std::__u::locale* __return,
    class std::basic_streambuf<char, std::char_traits<char>> const* __this) {
  new (__return) auto(__this->getloc());
}

static_assert((class ::std::__u::locale (
                  ::std::basic_streambuf<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_streambuf<char, std::char_traits<char>>::getloc);

extern "C" class std::basic_streambuf<char, std::char_traits<char>>*
__rust_thunk__79fe303a__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE9pubsetbufEPcl(
    class std::basic_streambuf<char, std::char_traits<char>>* __this, char* __s,
    ptrdiff_t __n) {
  return __this->pubsetbuf(__s, __n);
}

static_assert((class std::basic_streambuf<char, std::char_traits<char>> *
               (::std::basic_streambuf<char, std::char_traits<char>>::*)(
                   char*, ptrdiff_t)) &
              ::std::basic_streambuf<char, std::char_traits<char>>::pubsetbuf);

extern "C" void
__rust_thunk__1e5a4aa5__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE10pubseekoffExNS_8ios_base7seekdirEj(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_streambuf<char, std::char_traits<char>>* __this,
    long long __off, ::std::__u::ios_base::seekdir __way,
    unsigned int __which) {
  new (__return) auto(__this->pubseekoff(__off, __way, __which));
}

static_assert((class std::fpos<__mbstate_t> (
                  ::std::basic_streambuf<char, std::char_traits<char>>::*)(
                  long long, ::std::__u::ios_base::seekdir, unsigned int)) &
              ::std::basic_streambuf<char, std::char_traits<char>>::pubseekoff);

extern "C" void
__rust_thunk__e79eed6a__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE10pubseekposENS_4fposI11__mbstate_tEEj(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_streambuf<char, std::char_traits<char>>* __this,
    class std::fpos<__mbstate_t>* __sp, unsigned int __which) {
  new (__return) auto(__this->pubseekpos(std::move(*__sp), __which));
}

static_assert((class std::fpos<__mbstate_t> (
                  ::std::basic_streambuf<char, std::char_traits<char>>::*)(
                  class std::fpos<__mbstate_t>, unsigned int)) &
              ::std::basic_streambuf<char, std::char_traits<char>>::pubseekpos);

extern "C" int
__rust_thunk__6fea3a38__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE7pubsyncEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->pubsync();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::pubsync);

extern "C" ptrdiff_t
__rust_thunk__f08eef7e__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE8in_availEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->in_avail();
}

static_assert(
    (ptrdiff_t (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::in_avail);

extern "C" int
__rust_thunk__54348959__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE6snextcEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->snextc();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::snextc);

extern "C" int
__rust_thunk__31d816b4__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE6sbumpcEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->sbumpc();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sbumpc);

extern "C" int
__rust_thunk__02a71ece__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE5sgetcEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->sgetc();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sgetc);

extern "C" ptrdiff_t
__rust_thunk__53d0187f__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE5sgetnEPcl(
    class std::basic_streambuf<char, std::char_traits<char>>* __this, char* __s,
    ptrdiff_t __n) {
  return __this->sgetn(__s, __n);
}

static_assert(
    (ptrdiff_t (::std::basic_streambuf<char, std::char_traits<char>>::*)(
        char*, ptrdiff_t)) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sgetn);

extern "C" int
__rust_thunk__acd7170e__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE9sputbackcEc(
    class std::basic_streambuf<char, std::char_traits<char>>* __this,
    char __c) {
  return __this->sputbackc(__c);
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)(char)) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sputbackc);

extern "C" int
__rust_thunk__99917863__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE7sungetcEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->sungetc();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sungetc);

extern "C" int
__rust_thunk__946bb05a__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE5sputcEc(
    class std::basic_streambuf<char, std::char_traits<char>>* __this,
    char __c) {
  return __this->sputc(__c);
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)(char)) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sputc);

extern "C" ptrdiff_t
__rust_thunk__10a5b8b9__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE5sputnEPKcl(
    class std::basic_streambuf<char, std::char_traits<char>>* __this,
    char const* __s, ptrdiff_t __n) {
  return __this->sputn(__s, __n);
}

static_assert(
    (ptrdiff_t (::std::basic_streambuf<char, std::char_traits<char>>::*)(
        char const*, ptrdiff_t)) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sputn);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u15basic_streambufIcNS_11char_traitsIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acomposable_5fbridging_5fcc(
    class std::basic_streambuf<char, std::char_traits<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>) == 64);
static_assert(
    alignof(class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>) ==
    8);

extern "C" void
__rust_thunk__c35c4af4__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEED1Ev(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__2212eb43__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE8pubimbueERKNS_6localeE(
    class ::std::__u::locale* __return,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this,
    class ::std::__u::locale const* __loc) {
  new (__return) auto(__this->pubimbue(*__loc));
}

static_assert(
    (class ::std::__u::locale (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)(
        class ::std::__u::locale const&)) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::pubimbue);

extern "C" void
__rust_thunk__c28eab23__ZNKSt3__u15basic_streambufIwNS_11char_traitsIwEEE6getlocEv(
    class ::std::__u::locale* __return,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>> const*
        __this) {
  new (__return) auto(__this->getloc());
}

static_assert(
    (class ::std::__u::locale (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()
         const) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::getloc);

extern "C" void
__rust_thunk__1e5a4aa5__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE10pubseekoffExNS_8ios_base7seekdirEj(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this,
    long long __off, ::std::__u::ios_base::seekdir __way,
    unsigned int __which) {
  new (__return) auto(__this->pubseekoff(__off, __way, __which));
}

static_assert(
    (class std::fpos<__mbstate_t> (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)(
        long long, ::std::__u::ios_base::seekdir, unsigned int)) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::pubseekoff);

extern "C" void
__rust_thunk__e79eed6a__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE10pubseekposENS_4fposI11__mbstate_tEEj(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::fpos<__mbstate_t>* __sp, unsigned int __which) {
  new (__return) auto(__this->pubseekpos(std::move(*__sp), __which));
}

static_assert(
    (class std::fpos<__mbstate_t> (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)(
        class std::fpos<__mbstate_t>, unsigned int)) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::pubseekpos);

extern "C" int
__rust_thunk__6fea3a38__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE7pubsyncEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->pubsync();
}

static_assert(
    (int (::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::pubsync);

extern "C" ptrdiff_t
__rust_thunk__f08eef7e__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE8in_availEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->in_avail();
}

static_assert(
    (ptrdiff_t (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::in_avail);

extern "C" unsigned int
__rust_thunk__54348959__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE6snextcEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->snextc();
}

static_assert(
    (unsigned int (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::snextc);

extern "C" unsigned int
__rust_thunk__31d816b4__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE6sbumpcEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->sbumpc();
}

static_assert(
    (unsigned int (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::sbumpc);

extern "C" unsigned int
__rust_thunk__02a71ece__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE5sgetcEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->sgetc();
}

static_assert(
    (unsigned int (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::sgetc);

extern "C" unsigned int
__rust_thunk__99917863__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE7sungetcEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->sungetc();
}

static_assert(
    (unsigned int (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::sungetc);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u15basic_streambufIwNS_11char_traitsIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acomposable_5fbridging_5fcc(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_ostream<char, std::char_traits<char>>) ==
    160);
static_assert(alignof(class std::basic_ostream<char, std::char_traits<char>>) ==
              8);

extern "C" void
__rust_thunk__ded2d464__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEC1EPNS_15basic_streambufIcS2_EE(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    class std::basic_streambuf<char, std::char_traits<char>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__3c50a7b4__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEED1Ev(
    class std::basic_ostream<char, std::char_traits<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__0eef479b__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPFRS3_S4_E(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    crubit::type_identity_t<
        class std::basic_ostream<char, std::char_traits<char>>&(
            class std::basic_ostream<char, std::char_traits<char>>&)>* __pf) {
  return std::addressof(__this->operator<<(__pf));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(
         crubit::type_identity_t<
             class std::basic_ostream<char, std::char_traits<char>>&(
                 class std::basic_ostream<char, std::char_traits<char>>&)>*)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__f926b7c3__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPFRNS_9basic_iosIcS2_EES6_E(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    crubit::type_identity_t<class std::basic_ios<char, std::char_traits<char>>&(
        class std::basic_ios<char, std::char_traits<char>>&)>* __pf) {
  return std::addressof(__this->operator<<(__pf));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(
         crubit::type_identity_t<
             class std::basic_ios<char, std::char_traits<char>>&(
                 class std::basic_ios<char, std::char_traits<char>>&)>*)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__fc7838b8__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPFRNS_8ios_baseES5_E(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    crubit::type_identity_t<
        class ::std::__u::ios_base&(class ::std::__u::ios_base&)>* __pf) {
  return std::addressof(__this->operator<<(__pf));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(
                   crubit::type_identity_t<class ::std::__u::ios_base&(
                       class ::std::__u::ios_base&)>*)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__fe4ea326__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEb(
    class std::basic_ostream<char, std::char_traits<char>>* __this, bool __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(bool)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__e407b0a5__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEs(
    class std::basic_ostream<char, std::char_traits<char>>* __this, short __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(short)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__4ed8ec73__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEt(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    unsigned short __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(unsigned short)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__4e7d35aa__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEi(
    class std::basic_ostream<char, std::char_traits<char>>* __this, int __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(int)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__0e9c7430__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEj(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    unsigned int __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(unsigned int)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__1bc3773d__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEl(
    class std::basic_ostream<char, std::char_traits<char>>* __this, long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(long)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__c82d3f12__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEm(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    unsigned long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(unsigned long)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__c80e84e8__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEx(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    long long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(long long)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__4b414195__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEy(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    unsigned long long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(
                   unsigned long long)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__d19af565__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEf(
    class std::basic_ostream<char, std::char_traits<char>>* __this, float __f) {
  return std::addressof(__this->operator<<(__f));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(float)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__7a9a2fc0__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEd(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    double __f) {
  return std::addressof(__this->operator<<(__f));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(double)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__093d1099__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPKv(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    void const* __p) {
  return std::addressof(__this->operator<<(__p));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(void const*)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__a13617a7__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPNS_15basic_streambufIcS2_EE(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    class std::basic_streambuf<char, std::char_traits<char>>* __sb) {
  return std::addressof(__this->operator<<(__sb));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(
                   class std::basic_streambuf<char, std::char_traits<char>>*)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__18a69841__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEE3putEc(
    class std::basic_ostream<char, std::char_traits<char>>* __this, char __c) {
  return std::addressof(__this->put(__c));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(char)) &
              ::std::basic_ostream<char, std::char_traits<char>>::put);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__3888f3b6__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEE5writeEPKcl(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    char const* __s, ptrdiff_t __n) {
  return std::addressof(__this->write(__s, __n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(
                   char const*, ptrdiff_t)) &
              ::std::basic_ostream<char, std::char_traits<char>>::write);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__dd0d6a89__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEE5flushEv(
    class std::basic_ostream<char, std::char_traits<char>>* __this) {
  return std::addressof(__this->flush());
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)()) &
              ::std::basic_ostream<char, std::char_traits<char>>::flush);

extern "C" void
__rust_thunk__f786a186__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEE5tellpEv(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_ostream<char, std::char_traits<char>>* __this) {
  new (__return) auto(__this->tellp());
}

static_assert((class std::fpos<__mbstate_t> (
                  ::std::basic_ostream<char, std::char_traits<char>>::*)()) &
              ::std::basic_ostream<char, std::char_traits<char>>::tellp);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acomposable_5fbridging_5fcc(
    class std::basic_ostream<char, std::char_traits<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>) == 160);
static_assert(
    alignof(class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>) == 8);

extern "C" void
__rust_thunk__ded2d464__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEEC1EPNS_15basic_streambufIwS2_EE(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__3c50a7b4__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEED1Ev(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__0eef479b__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPFRS3_S4_E(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    crubit::type_identity_t<
        class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>&(
            class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>&)>*
        __pf) {
  return std::addressof(__this->operator<<(__pf));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(
         crubit::type_identity_t<
             class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>&(
                 class std::basic_ostream<wchar_t,
                                          std::char_traits<wchar_t>>&)>*)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__f926b7c3__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPFRNS_9basic_iosIwS2_EES6_E(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    crubit::type_identity_t<
        class std::basic_ios<wchar_t, std::char_traits<wchar_t>>&(
            class std::basic_ios<wchar_t, std::char_traits<wchar_t>>&)>* __pf) {
  return std::addressof(__this->operator<<(__pf));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(
         crubit::type_identity_t<class std::basic_ios<
             wchar_t, std::char_traits<wchar_t>>&(
             class std::basic_ios<wchar_t, std::char_traits<wchar_t>>&)>*)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__fc7838b8__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPFRNS_8ios_baseES5_E(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    crubit::type_identity_t<
        class ::std::__u::ios_base&(class ::std::__u::ios_base&)>* __pf) {
  return std::addressof(__this->operator<<(__pf));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(
         crubit::type_identity_t<
             class ::std::__u::ios_base&(class ::std::__u::ios_base&)>*)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__fe4ea326__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEb(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    bool __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(bool)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__e407b0a5__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEs(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    short __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(short)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__4ed8ec73__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEt(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned short __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(
         unsigned short)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__4e7d35aa__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEi(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    int __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(int)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__0e9c7430__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEj(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned int __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(
         unsigned int)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__1bc3773d__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEl(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(long)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__c82d3f12__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEm(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(
         unsigned long)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__c80e84e8__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEx(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    long long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(long long)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__4b414195__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEy(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned long long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(
         unsigned long long)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__d19af565__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEf(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    float __f) {
  return std::addressof(__this->operator<<(__f));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(float)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__7a9a2fc0__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEd(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    double __f) {
  return std::addressof(__this->operator<<(__f));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(double)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__093d1099__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPKv(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    void const* __p) {
  return std::addressof(__this->operator<<(__p));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(
         void const*)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__a13617a7__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPNS_15basic_streambufIwS2_EE(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __sb) {
  return std::addressof(__this->operator<<(__sb));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(
         class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>*)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__dd0d6a89__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEE5flushEv(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this) {
  return std::addressof(__this->flush());
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::flush);

extern "C" void
__rust_thunk__f786a186__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEE5tellpEv(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this) {
  new (__return) auto(__this->tellp());
}

static_assert(
    (class std::fpos<__mbstate_t> (
        ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::tellp);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u13basic_ostreamIwNS_11char_traitsIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acomposable_5fbridging_5fcc(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(sizeof(struct std::placeholders::__ph<10>) == 1);
static_assert(alignof(struct std::placeholders::__ph<10>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi10EEC1Ev(
    struct std::placeholders::__ph<10>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<1>) == 1);
static_assert(alignof(struct std::placeholders::__ph<1>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi1EEC1Ev(
    struct std::placeholders::__ph<1>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<2>) == 1);
static_assert(alignof(struct std::placeholders::__ph<2>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi2EEC1Ev(
    struct std::placeholders::__ph<2>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<3>) == 1);
static_assert(alignof(struct std::placeholders::__ph<3>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi3EEC1Ev(
    struct std::placeholders::__ph<3>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<4>) == 1);
static_assert(alignof(struct std::placeholders::__ph<4>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi4EEC1Ev(
    struct std::placeholders::__ph<4>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<5>) == 1);
static_assert(alignof(struct std::placeholders::__ph<5>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi5EEC1Ev(
    struct std::placeholders::__ph<5>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<6>) == 1);
static_assert(alignof(struct std::placeholders::__ph<6>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi6EEC1Ev(
    struct std::placeholders::__ph<6>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<7>) == 1);
static_assert(alignof(struct std::placeholders::__ph<7>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi7EEC1Ev(
    struct std::placeholders::__ph<7>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<8>) == 1);
static_assert(alignof(struct std::placeholders::__ph<8>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi8EEC1Ev(
    struct std::placeholders::__ph<8>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<9>) == 1);
static_assert(alignof(struct std::placeholders::__ph<9>) == 1);

extern "C" void __rust_thunk__68083cb0__ZNSt3__u12placeholders4__phILi9EEC1Ev(
    struct std::placeholders::__ph<9>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
