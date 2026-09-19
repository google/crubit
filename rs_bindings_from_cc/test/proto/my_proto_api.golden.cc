// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/proto:my_proto_api

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/fmt.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"
#include "support/rs_std/lossy_formatter_for_bindings.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/proto/my_proto_api.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___ZN4test11MakeRequestEl(
    unsigned char* __return_abi_buffer, int64_t num) {
  ::crubit::Encoder __return_encoder(
      ::crubit::BoxedAbi<my_package::MyMessage_Request>::kSize,
      __return_abi_buffer);
  ::crubit::BoxedAbi<my_package::MyMessage_Request>().Encode(
      test::MakeRequest(num), __return_encoder);
}

static_assert((class my_package::MyMessage_Request (*)(int64_t)) &
              ::test::MakeRequest);

extern "C" void __rust_thunk___ZN4test11ReturnValueEv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::BoxedAbi<my_package::MyMessage>::kSize, __return_abi_buffer);
  ::crubit::BoxedAbi<my_package::MyMessage>().Encode(test::ReturnValue(),
                                                     __return_encoder);
}

static_assert((class my_package::MyMessage (*)()) & ::test::ReturnValue);

extern "C" int64_t
__rust_thunk___ZN4test16ExtractFromValueEN10my_package9MyMessageE(
    const unsigned char* msg) {
  ::crubit::Decoder __msg_decoder(
      ::crubit::BoxedAbi<my_package::MyMessage>::kSize, msg);
  return test::ExtractFromValue(
      ::crubit::BoxedAbi<my_package::MyMessage>().Decode(__msg_decoder));
}

static_assert((int64_t (*)(class my_package::MyMessage)) &
              ::test::ExtractFromValue);

extern "C" int64_t
__rust_thunk___ZN4test19ExtractFromConstPtrEPKN10my_package9MyMessageE(
    my_package::MyMessage const* msg) {
  return test::ExtractFromConstPtr(msg);
}

static_assert((int64_t (*)(my_package::MyMessage const*)) &
              ::test::ExtractFromConstPtr);

extern "C" int64_t
__rust_thunk___ZN4test19ExtractFromConstRefERKN10my_package9MyMessageE(
    my_package::MyMessage const* msg) {
  return test::ExtractFromConstRef(*msg);
}

static_assert((int64_t (*)(my_package::MyMessage const&)) &
              ::test::ExtractFromConstRef);

extern "C" int64_t
__rust_thunk___ZN4test21ExtractFromMutablePtrEPN10my_package9MyMessageE(
    my_package::MyMessage* msg) {
  return test::ExtractFromMutablePtr(msg);
}

static_assert((int64_t (*)(my_package::MyMessage*)) &
              ::test::ExtractFromMutablePtr);

extern "C" int64_t
__rust_thunk___ZN4test21ExtractFromMutableRefERN10my_package9MyMessageE(
    my_package::MyMessage* msg) {
  return test::ExtractFromMutableRef(*msg);
}

static_assert((int64_t (*)(my_package::MyMessage&)) &
              ::test::ExtractFromMutableRef);

extern "C" my_package::MyMessage* __rust_thunk___ZN4test12GetMutMsgPtrEv() {
  return test::GetMutMsgPtr();
}

static_assert((my_package::MyMessage * (*)()) & ::test::GetMutMsgPtr);

extern "C" my_package::MyMessage const*
__rust_thunk___ZN4test14GetConstMsgPtrEv() {
  return test::GetConstMsgPtr();
}

static_assert((my_package::MyMessage const* (*)()) & ::test::GetConstMsgPtr);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<char32_t>) == 16);
static_assert(alignof(class std::initializer_list<char32_t>) == 8);

extern "C" void __rust_thunk__17e454af__ZNSt16initializer_listIDiEC1Ev(
    class std::initializer_list<char32_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char32_t const*
__rust_thunk__d29b6e8a__ZNKSt16initializer_listIDiE4dataEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->data();
}

static_assert((char32_t const* (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::data);

extern "C" size_t __rust_thunk__13c1b761__ZNKSt16initializer_listIDiE4sizeEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::size);

extern "C" bool __rust_thunk__13fffd14__ZNKSt16initializer_listIDiE5emptyEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::empty);

extern "C" char32_t const*
__rust_thunk__db02d08a__ZNKSt16initializer_listIDiE5beginEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->begin();
}

static_assert((char32_t const* (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::begin);

extern "C" char32_t const*
__rust_thunk__6ba87a85__ZNKSt16initializer_listIDiE3endEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->end();
}

static_assert((char32_t const* (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::end);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<char16_t>) == 16);
static_assert(alignof(class std::initializer_list<char16_t>) == 8);

extern "C" void __rust_thunk__17e454af__ZNSt16initializer_listIDsEC1Ev(
    class std::initializer_list<char16_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char16_t const*
__rust_thunk__d29b6e8a__ZNKSt16initializer_listIDsE4dataEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->data();
}

static_assert((char16_t const* (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::data);

extern "C" size_t __rust_thunk__13c1b761__ZNKSt16initializer_listIDsE4sizeEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::size);

extern "C" bool __rust_thunk__13fffd14__ZNKSt16initializer_listIDsE5emptyEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::empty);

extern "C" char16_t const*
__rust_thunk__db02d08a__ZNKSt16initializer_listIDsE5beginEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->begin();
}

static_assert((char16_t const* (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::begin);

extern "C" char16_t const*
__rust_thunk__6ba87a85__ZNKSt16initializer_listIDsE3endEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->end();
}

static_assert((char16_t const* (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::end);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<
                            absl::crc_internal::CrcCordState::PrefixCrc>) ==
              16);
static_assert(alignof(class std::initializer_list<
                      absl::crc_internal::CrcCordState::PrefixCrc>) == 8);

extern "C" void
__rust_thunk__17e454af__ZNSt16initializer_listIN4absl12crc_internal12CrcCordState9PrefixCrcEEC1Ev(
    class std::initializer_list<absl::crc_internal::CrcCordState::PrefixCrc>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" size_t
__rust_thunk__13c1b761__ZNKSt16initializer_listIN4absl12crc_internal12CrcCordState9PrefixCrcEE4sizeEv(
    class std::initializer_list<
        absl::crc_internal::CrcCordState::PrefixCrc> const* __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::initializer_list<
             absl::crc_internal::CrcCordState::PrefixCrc>::*)() const) &
    ::std::initializer_list<absl::crc_internal::CrcCordState::PrefixCrc>::size);

extern "C" bool
__rust_thunk__13fffd14__ZNKSt16initializer_listIN4absl12crc_internal12CrcCordState9PrefixCrcEE5emptyEv(
    class std::initializer_list<
        absl::crc_internal::CrcCordState::PrefixCrc> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<
                     absl::crc_internal::CrcCordState::PrefixCrc>::*)() const) &
              ::std::initializer_list<
                  absl::crc_internal::CrcCordState::PrefixCrc>::empty);

static_assert(
    CRUBIT_SIZEOF(class std::initializer_list<std::basic_string<
                      char, std::char_traits<char>, std::allocator<char>>>) ==
    16);
static_assert(
    alignof(class std::initializer_list<std::basic_string<
                char, std::char_traits<char>, std::allocator<char>>>) == 8);

extern "C" void
__rust_thunk__17e454af__ZNSt16initializer_listINSt3__u12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEEC1Ev(
    class std::initializer_list<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>> const*
__rust_thunk__d29b6e8a__ZNKSt16initializer_listINSt3__u12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEE4dataEv(
    class std::initializer_list<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>> const* __this) {
  return __this->data();
}

static_assert(
    (class std::basic_string<
        char, std::char_traits<char>,
        std::allocator<char>> const* (::std::
                                          initializer_list<std::basic_string<
                                              char, std::char_traits<char>,
                                              std::allocator<char>>>::*)()
         const) &
    ::std::initializer_list<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>>::data);

extern "C" size_t
__rust_thunk__13c1b761__ZNKSt16initializer_listINSt3__u12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEE4sizeEv(
    class std::initializer_list<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>> const* __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::initializer_list<std::basic_string<
                 char, std::char_traits<char>, std::allocator<char>>>::*)()
         const) &
    ::std::initializer_list<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>>::size);

extern "C" bool
__rust_thunk__13fffd14__ZNKSt16initializer_listINSt3__u12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEE5emptyEv(
    class std::initializer_list<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>> const* __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::initializer_list<std::basic_string<
               char, std::char_traits<char>, std::allocator<char>>>::*)()
         const) &
    ::std::initializer_list<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>>::empty);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>> const*
__rust_thunk__db02d08a__ZNKSt16initializer_listINSt3__u12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEE5beginEv(
    class std::initializer_list<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>> const* __this) {
  return __this->begin();
}

static_assert(
    (class std::basic_string<
        char, std::char_traits<char>,
        std::allocator<char>> const* (::std::
                                          initializer_list<std::basic_string<
                                              char, std::char_traits<char>,
                                              std::allocator<char>>>::*)()
         const) &
    ::std::initializer_list<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>>::begin);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>> const*
__rust_thunk__6ba87a85__ZNKSt16initializer_listINSt3__u12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEEE3endEv(
    class std::initializer_list<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>> const* __this) {
  return __this->end();
}

static_assert(
    (class std::basic_string<
        char, std::char_traits<char>,
        std::allocator<char>> const* (::std::
                                          initializer_list<std::basic_string<
                                              char, std::char_traits<char>,
                                              std::allocator<char>>>::*)()
         const) &
    ::std::initializer_list<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>>::end);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<char>) == 16);
static_assert(alignof(class std::initializer_list<char>) == 8);

extern "C" void __rust_thunk__17e454af__ZNSt16initializer_listIcEC1Ev(
    class std::initializer_list<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" char const*
__rust_thunk__d29b6e8a__ZNKSt16initializer_listIcE4dataEv(
    class std::initializer_list<char> const* __this) {
  return __this->data();
}

static_assert((char const* (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::data);

extern "C" size_t __rust_thunk__13c1b761__ZNKSt16initializer_listIcE4sizeEv(
    class std::initializer_list<char> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::size);

extern "C" bool __rust_thunk__13fffd14__ZNKSt16initializer_listIcE5emptyEv(
    class std::initializer_list<char> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::empty);

extern "C" char const*
__rust_thunk__db02d08a__ZNKSt16initializer_listIcE5beginEv(
    class std::initializer_list<char> const* __this) {
  return __this->begin();
}

static_assert((char const* (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::begin);

extern "C" char const* __rust_thunk__6ba87a85__ZNKSt16initializer_listIcE3endEv(
    class std::initializer_list<char> const* __this) {
  return __this->end();
}

static_assert((char const* (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::end);

static_assert(sizeof(class std::allocator<char32_t>) == 1);
static_assert(alignof(class std::allocator<char32_t>) == 1);

extern "C" void __rust_thunk__e95e64c8__ZNSt3__u9allocatorIDiEC1Ev(
    class std::allocator<char32_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char32_t* __rust_thunk__3f554b47__ZNSt3__u9allocatorIDiE8allocateEm(
    class std::allocator<char32_t>* __this, size_t __n) {
  return __this->allocate(__n);
}

static_assert((char32_t* (::std::allocator<char32_t>::*)(size_t)) &
              ::std::allocator<char32_t>::allocate);

extern "C" void __rust_thunk__2e7db5c6__ZNSt3__u9allocatorIDiE10deallocateEPDim(
    class std::allocator<char32_t>* __this, char32_t* __p, size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert((void (::std::allocator<char32_t>::*)(char32_t*, size_t)) &
              ::std::allocator<char32_t>::deallocate);

static_assert(sizeof(class std::allocator<char16_t>) == 1);
static_assert(alignof(class std::allocator<char16_t>) == 1);

extern "C" void __rust_thunk__e95e64c8__ZNSt3__u9allocatorIDsEC1Ev(
    class std::allocator<char16_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char16_t* __rust_thunk__3f554b47__ZNSt3__u9allocatorIDsE8allocateEm(
    class std::allocator<char16_t>* __this, size_t __n) {
  return __this->allocate(__n);
}

static_assert((char16_t* (::std::allocator<char16_t>::*)(size_t)) &
              ::std::allocator<char16_t>::allocate);

extern "C" void __rust_thunk__2e7db5c6__ZNSt3__u9allocatorIDsE10deallocateEPDsm(
    class std::allocator<char16_t>* __this, char16_t* __p, size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert((void (::std::allocator<char16_t>::*)(char16_t*, size_t)) &
              ::std::allocator<char16_t>::deallocate);

static_assert(sizeof(class std::allocator<
                     const absl::crc_internal::CrcCordState::PrefixCrc*>) == 1);
static_assert(alignof(class std::allocator<
                      const absl::crc_internal::CrcCordState::PrefixCrc*>) ==
              1);

extern "C" void
__rust_thunk__e95e64c8__ZNSt3__u9allocatorIPKN4absl12crc_internal12CrcCordState9PrefixCrcEEC1Ev(
    class std::allocator<const absl::crc_internal::CrcCordState::PrefixCrc*>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(
        class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc*>) ==
    1);
static_assert(
    alignof(
        class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc*>) ==
    1);

extern "C" void
__rust_thunk__e95e64c8__ZNSt3__u9allocatorIPN4absl12crc_internal12CrcCordState9PrefixCrcEEC1Ev(
    class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc*>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::allocator<char>) == 1);
static_assert(alignof(class std::allocator<char>) == 1);

extern "C" void __rust_thunk__e95e64c8__ZNSt3__u9allocatorIcEC1Ev(
    class std::allocator<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" char* __rust_thunk__3f554b47__ZNSt3__u9allocatorIcE8allocateEm(
    class std::allocator<char>* __this, size_t __n) {
  return __this->allocate(__n);
}

static_assert((char* (::std::allocator<char>::*)(size_t)) &
              ::std::allocator<char>::allocate);

extern "C" void __rust_thunk__2e7db5c6__ZNSt3__u9allocatorIcE10deallocateEPcm(
    class std::allocator<char>* __this, char* __p, size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert((void (::std::allocator<char>::*)(char*, size_t)) &
              ::std::allocator<char>::deallocate);

static_assert(sizeof(class std::allocator<wchar_t>) == 1);
static_assert(alignof(class std::allocator<wchar_t>) == 1);

extern "C" void __rust_thunk__e95e64c8__ZNSt3__u9allocatorIwEC1Ev(
    class std::allocator<wchar_t>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::pmr::polymorphic_allocator<char32_t>) ==
              8);
static_assert(alignof(class std::pmr::polymorphic_allocator<char32_t>) == 8);

extern "C" void
__rust_thunk__e421440d__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1Ev(
    class std::pmr::polymorphic_allocator<char32_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__8f3acb6f__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1EPNS0_15memory_resourceE(
    class std::pmr::polymorphic_allocator<char32_t>* __this,
    class ::std::__u::pmr::memory_resource* __r) {
  crubit::construct_at(__this, __r);
}

static_assert(CRUBIT_SIZEOF(class std::pmr::polymorphic_allocator<char16_t>) ==
              8);
static_assert(alignof(class std::pmr::polymorphic_allocator<char16_t>) == 8);

extern "C" void
__rust_thunk__e421440d__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1Ev(
    class std::pmr::polymorphic_allocator<char16_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__8f3acb6f__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1EPNS0_15memory_resourceE(
    class std::pmr::polymorphic_allocator<char16_t>* __this,
    class ::std::__u::pmr::memory_resource* __r) {
  crubit::construct_at(__this, __r);
}

static_assert(CRUBIT_SIZEOF(class std::pmr::polymorphic_allocator<char>) == 8);
static_assert(alignof(class std::pmr::polymorphic_allocator<char>) == 8);

extern "C" void
__rust_thunk__e421440d__ZNSt3__u3pmr21polymorphic_allocatorIcEC1Ev(
    class std::pmr::polymorphic_allocator<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__8f3acb6f__ZNSt3__u3pmr21polymorphic_allocatorIcEC1EPNS0_15memory_resourceE(
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
__rust_thunk__d008406c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__75a09692__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__e1e8d2d4__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c7de2848__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_RKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __str,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__ebd64662__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__c3fe6773__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_RKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__777b2769__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ESt16initializer_listIDiERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__4a055ff2__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEED1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__f9b8d382__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEOS6_(
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
__rust_thunk__eccb0c67__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSESt16initializer_listIDiE(
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
__rust_thunk__aff68b53__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEPKDi(
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
__rust_thunk__8905b633__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE6cbeginEv(
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
__rust_thunk__3cdcafb2__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4cendEv(
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
__rust_thunk__df70e385__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7crbeginEv(
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
__rust_thunk__75d71c98__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5crendEv(
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
__rust_thunk__33354839__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7reserveEv(
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
__rust_thunk__dfa7b50c__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5emptyEv(
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
__rust_thunk__0bfb3ace__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5c_strEv(
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
__rust_thunk__68fabb4c__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13get_allocatorEv(
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
__rust_thunk__5f7e0a27__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEDi(
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
__rust_thunk__78e24cdd__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSERKS6_(
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
__rust_thunk__35487a7d__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE9push_backEDi(
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
__rust_thunk__a5480a66__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE8pop_backEv(
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
__rust_thunk__1866e889__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5clearEv(
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
__rust_thunk__11bc2261__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13shrink_to_fitEv(
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
__rust_thunk__e6243ab9__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4swapERS6_(
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
__rust_thunk__d008406c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__75a09692__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__e1e8d2d4__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c7de2848__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_RKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__ebd64662__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__c3fe6773__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_RKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__061d70c7__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mmRKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str,
    size_t __pos, size_t __n, class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, __n, *__a);
}

extern "C" void
__rust_thunk__4554c544__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mRKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str,
    size_t __pos, class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, *__a);
}

extern "C" void
__rust_thunk__777b2769__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ESt16initializer_listIDiERKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__4a055ff2__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEED1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__f9b8d382__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEOS5_(
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
__rust_thunk__eccb0c67__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSESt16initializer_listIDiE(
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
__rust_thunk__aff68b53__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEPKDi(
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
__rust_thunk__8905b633__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6cbeginEv(
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
__rust_thunk__3cdcafb2__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4cendEv(
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
__rust_thunk__df70e385__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE7crbeginEv(
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
__rust_thunk__75d71c98__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5crendEv(
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
__rust_thunk__c79b467e__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4sizeEv(
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
__rust_thunk__6504e1c0__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6lengthEv(
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
__rust_thunk__20ef6d85__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8max_sizeEv(
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
__rust_thunk__596387c5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8capacityEv(
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
__rust_thunk__dfa7b50c__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5emptyEv(
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
__rust_thunk__a4b19a8a__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm(
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
__rust_thunk__93439e15__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm(
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
__rust_thunk__ae57e806__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6substrEmm(
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
__rust_thunk__0bfb3ace__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5c_strEv(
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
__rust_thunk__68fabb4c__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13get_allocatorEv(
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
__rust_thunk__5f7e0a27__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEDi(
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
__rust_thunk__78e24cdd__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSERKS5_(
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
__rust_thunk__35487a7d__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE9push_backEDi(
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
__rust_thunk__a5480a66__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8pop_backEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  __this->pop_back();
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::pop_back);

extern "C" void
__rust_thunk__1866e889__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5clearEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  __this->clear();
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::clear);

extern "C" void
__rust_thunk__11bc2261__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13shrink_to_fitEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  __this->shrink_to_fit();
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::shrink_to_fit);

extern "C" size_t
__rust_thunk__a3a1778b__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4copyEPDimm(
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
__rust_thunk__e6243ab9__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4swapERS5_(
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
__rust_thunk__d008406c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__75a09692__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__e1e8d2d4__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c7de2848__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_RKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __str,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__ebd64662__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__c3fe6773__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_RKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__777b2769__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ESt16initializer_listIDsERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__4a055ff2__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEED1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__f9b8d382__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEOS6_(
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
__rust_thunk__eccb0c67__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSESt16initializer_listIDsE(
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
__rust_thunk__aff68b53__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEPKDs(
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
__rust_thunk__8905b633__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE6cbeginEv(
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
__rust_thunk__3cdcafb2__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4cendEv(
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
__rust_thunk__df70e385__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7crbeginEv(
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
__rust_thunk__75d71c98__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5crendEv(
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
__rust_thunk__33354839__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7reserveEv(
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
__rust_thunk__dfa7b50c__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5emptyEv(
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
__rust_thunk__0bfb3ace__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5c_strEv(
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
__rust_thunk__68fabb4c__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13get_allocatorEv(
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
__rust_thunk__5f7e0a27__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEDs(
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
__rust_thunk__78e24cdd__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSERKS6_(
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
__rust_thunk__35487a7d__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE9push_backEDs(
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
__rust_thunk__a5480a66__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE8pop_backEv(
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
__rust_thunk__1866e889__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5clearEv(
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
__rust_thunk__11bc2261__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13shrink_to_fitEv(
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
__rust_thunk__e6243ab9__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4swapERS6_(
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
__rust_thunk__d008406c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__75a09692__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__e1e8d2d4__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c7de2848__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_RKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__ebd64662__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__c3fe6773__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_RKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__061d70c7__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mmRKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str,
    size_t __pos, size_t __n, class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, __n, *__a);
}

extern "C" void
__rust_thunk__4554c544__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mRKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str,
    size_t __pos, class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, *__a);
}

extern "C" void
__rust_thunk__777b2769__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ESt16initializer_listIDsERKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__4a055ff2__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEED1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__f9b8d382__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEOS5_(
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
__rust_thunk__eccb0c67__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSESt16initializer_listIDsE(
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
__rust_thunk__aff68b53__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEPKDs(
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
__rust_thunk__8905b633__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6cbeginEv(
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
__rust_thunk__3cdcafb2__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4cendEv(
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
__rust_thunk__df70e385__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE7crbeginEv(
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
__rust_thunk__75d71c98__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5crendEv(
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
__rust_thunk__c79b467e__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4sizeEv(
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
__rust_thunk__6504e1c0__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6lengthEv(
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
__rust_thunk__20ef6d85__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8max_sizeEv(
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
__rust_thunk__596387c5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8capacityEv(
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
__rust_thunk__dfa7b50c__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5emptyEv(
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
__rust_thunk__a4b19a8a__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm(
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
__rust_thunk__93439e15__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm(
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
__rust_thunk__ae57e806__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6substrEmm(
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
__rust_thunk__0bfb3ace__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5c_strEv(
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
__rust_thunk__68fabb4c__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13get_allocatorEv(
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
__rust_thunk__5f7e0a27__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEDs(
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
__rust_thunk__78e24cdd__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSERKS5_(
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
__rust_thunk__35487a7d__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE9push_backEDs(
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
__rust_thunk__a5480a66__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8pop_backEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  __this->pop_back();
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::pop_back);

extern "C" void
__rust_thunk__1866e889__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5clearEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  __this->clear();
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::clear);

extern "C" void
__rust_thunk__11bc2261__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13shrink_to_fitEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  __this->shrink_to_fit();
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::shrink_to_fit);

extern "C" size_t
__rust_thunk__a3a1778b__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4copyEPDsmm(
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
__rust_thunk__e6243ab9__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4swapERS5_(
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
__rust_thunk__d008406c__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1Ev(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__75a09692__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__e1e8d2d4__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__c7de2848__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_RKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const* __str,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__ebd64662__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__c3fe6773__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_RKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__777b2769__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ESt16initializer_listIcERKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::initializer_list<char>* __il,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__4a055ff2__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEED1Ev(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__f9b8d382__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEOS6_(
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
__rust_thunk__eccb0c67__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSESt16initializer_listIcE(
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
__rust_thunk__aff68b53__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEPKc(
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
__rust_thunk__8905b633__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE6cbeginEv(
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
__rust_thunk__3cdcafb2__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4cendEv(
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
__rust_thunk__df70e385__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7crbeginEv(
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
__rust_thunk__75d71c98__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5crendEv(
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
__rust_thunk__33354839__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7reserveEv(
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
__rust_thunk__dfa7b50c__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5emptyEv(
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
__rust_thunk__0bfb3ace__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5c_strEv(
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
__rust_thunk__68fabb4c__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13get_allocatorEv(
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
__rust_thunk__5f7e0a27__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEc(
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
__rust_thunk__78e24cdd__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSERKS6_(
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
__rust_thunk__35487a7d__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE9push_backEc(
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
__rust_thunk__a5480a66__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE8pop_backEv(
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
__rust_thunk__1866e889__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5clearEv(
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
__rust_thunk__11bc2261__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13shrink_to_fitEv(
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
__rust_thunk__e6243ab9__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4swapERS6_(
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
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<char32_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<char32_t*>>* __this,
    class std::__wrap_iter<char32_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEE4baseEv(
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
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<char16_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<char16_t*>>* __this,
    class std::__wrap_iter<char16_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEE4baseEv(
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
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __this,
    class std::__wrap_iter<const char32_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEE4baseEv(
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
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __this,
    class std::__wrap_iter<const char16_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEE4baseEv(
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
    CRUBIT_SIZEOF(
        class std::reverse_iterator<std::__wrap_iter<const std::basic_string<
            char, std::char_traits<char>, std::allocator<char>>*>>) == 8);
static_assert(
    alignof(
        class std::reverse_iterator<std::__wrap_iter<const std::basic_string<
            char, std::char_traits<char>, std::allocator<char>>*>>) == 8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEEEC1ESA_(
    class std::reverse_iterator<std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>>* __this,
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEEE4baseEv(
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>* __return,
    class std::reverse_iterator<std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__wrap_iter<const std::basic_string<
         char, std::char_traits<char>, std::allocator<char>>*> (
        ::std::reverse_iterator<std::__wrap_iter<const std::basic_string<
            char, std::char_traits<char>, std::allocator<char>>*>>::*)()
         const) &
    ::std::reverse_iterator<std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<const char*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<const char*>>) == 8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __this,
    class std::__wrap_iter<const char*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEE4baseEv(
    class std::__wrap_iter<const char*>* __return,
    class std::reverse_iterator<std::__wrap_iter<const char*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<const char*> (
                  ::std::reverse_iterator<std::__wrap_iter<const char*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<const char*>>::base);

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<std::__wrap_iter<std::basic_string<
            char, std::char_traits<char>, std::allocator<char>>*>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<std::basic_string<
                char, std::char_traits<char>, std::allocator<char>>*>>) == 8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEEEC1ES9_(
    class std::reverse_iterator<std::__wrap_iter<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>>* __this,
    class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEEE4baseEv(
    class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>*>* __return,
    class std::reverse_iterator<std::__wrap_iter<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>*> (
        ::std::reverse_iterator<std::__wrap_iter<std::basic_string<
            char, std::char_traits<char>, std::allocator<char>>*>>::*)()
         const) &
    ::std::reverse_iterator<std::__wrap_iter<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<char*>>) == 8);
static_assert(alignof(class std::reverse_iterator<std::__wrap_iter<char*>>) ==
              8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<char*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<char*>>* __this,
    class std::__wrap_iter<char*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEE4baseEv(
    class std::__wrap_iter<char*>* __return,
    class std::reverse_iterator<std::__wrap_iter<char*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<char*> (
                  ::std::reverse_iterator<std::__wrap_iter<char*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<char*>>::base);

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>>) == 8);
static_assert(
    alignof(
        class std::reverse_iterator<std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>>) == 8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_14__map_iteratorINS_15__tree_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISD_PvEElEEEEEC1Ev(
    class std::reverse_iterator<std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_14__map_iteratorINS_15__tree_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISD_PvEElEEEEEC1ESJ_(
    class std::reverse_iterator<std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>>* __this,
    class std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_14__map_iteratorINS_15__tree_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISD_PvEElEEEEE4baseEv(
    class std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __return,
    class std::reverse_iterator<std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__map_iterator<std::__tree_iterator<
         std::__value_type<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>,
         std::__tree_node<
             std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                               tcmalloc::MallocExtension::Property>,
             void*>*,
         long>> (
        ::std::reverse_iterator<std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>>::*)() const) &
    ::std::reverse_iterator<std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__deque_iterator<
                      absl::crc_internal::CrcCordState::PrefixCrc,
                      const absl::crc_internal::CrcCordState::PrefixCrc*,
                      const absl::crc_internal::CrcCordState::PrefixCrc&,
                      const absl::crc_internal::CrcCordState::PrefixCrc* const*,
                      long, 0L>>) == 16);
static_assert(
    alignof(class std::reverse_iterator<std::__deque_iterator<
                absl::crc_internal::CrcCordState::PrefixCrc,
                const absl::crc_internal::CrcCordState::PrefixCrc*,
                const absl::crc_internal::CrcCordState::PrefixCrc&,
                const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
                0L>>) == 8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPKS5_RS6_PKS7_lLl0EEEEC1Ev(
    class std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPKS5_RS6_PKS7_lLl0EEEEC1ESB_(
    class std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>>*
        __this,
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
        __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPKS5_RS6_PKS7_lLl0EEEE4baseEv(
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
        __return,
    class std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
        0L>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L> (
        ::std::reverse_iterator<std::__deque_iterator<
            absl::crc_internal::CrcCordState::PrefixCrc,
            const absl::crc_internal::CrcCordState::PrefixCrc*,
            const absl::crc_internal::CrcCordState::PrefixCrc&,
            const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
            0L>>::*)() const) &
    ::std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
        0L>>::base);

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<std::__deque_iterator<
            absl::crc_internal::CrcCordState::PrefixCrc,
            absl::crc_internal::CrcCordState::PrefixCrc*,
            absl::crc_internal::CrcCordState::PrefixCrc&,
            absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>>) == 16);
static_assert(
    alignof(class std::reverse_iterator<std::__deque_iterator<
                absl::crc_internal::CrcCordState::PrefixCrc,
                absl::crc_internal::CrcCordState::PrefixCrc*,
                absl::crc_internal::CrcCordState::PrefixCrc&,
                absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>>) ==
    8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPS5_RS5_PS6_lLl0EEEEC1Ev(
    class std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        absl::crc_internal::CrcCordState::PrefixCrc*,
        absl::crc_internal::CrcCordState::PrefixCrc&,
        absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPS5_RS5_PS6_lLl0EEEEC1ES9_(
    class std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        absl::crc_internal::CrcCordState::PrefixCrc*,
        absl::crc_internal::CrcCordState::PrefixCrc&,
        absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>>* __this,
    class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPS5_RS5_PS6_lLl0EEEE4baseEv(
    class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L>* __return,
    class std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        absl::crc_internal::CrcCordState::PrefixCrc*,
        absl::crc_internal::CrcCordState::PrefixCrc&,
        absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>> const*
        __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                 absl::crc_internal::CrcCordState::PrefixCrc*,
                                 absl::crc_internal::CrcCordState::PrefixCrc&,
                                 absl::crc_internal::CrcCordState::PrefixCrc**,
                                 long, 0L> (
        ::std::reverse_iterator<std::__deque_iterator<
            absl::crc_internal::CrcCordState::PrefixCrc,
            absl::crc_internal::CrcCordState::PrefixCrc*,
            absl::crc_internal::CrcCordState::PrefixCrc&,
            absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>>::*)()
         const) &
    ::std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        absl::crc_internal::CrcCordState::PrefixCrc*,
        absl::crc_internal::CrcCordState::PrefixCrc&,
        absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<
                  std::__map_const_iterator<std::__tree_const_iterator<
                      std::__value_type<
                          std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
                      std::__tree_node<
                          std::__value_type<
                              std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
                          void*>*,
                      long>>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<
            std::__map_const_iterator<std::__tree_const_iterator<
                std::__value_type<
                    std::basic_string<char, std::char_traits<char>,
                                      std::allocator<char>>,
                    tcmalloc::MallocExtension::Property>,
                std::__tree_node<
                    std::__value_type<
                        std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                        tcmalloc::MallocExtension::Property>,
                    void*>*,
                long>>>) == 8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorINS_20__map_const_iteratorINS_21__tree_const_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISD_PvEElEEEEEC1Ev(
    class std::reverse_iterator<
        std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorINS_20__map_const_iteratorINS_21__tree_const_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISD_PvEElEEEEEC1ESJ_(
    class std::reverse_iterator<
        std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>>* __this,
    class std::__map_const_iterator<std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorINS_20__map_const_iteratorINS_21__tree_const_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISD_PvEElEEEEE4baseEv(
    class std::__map_const_iterator<std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __return,
    class std::reverse_iterator<
        std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__map_const_iterator<std::__tree_const_iterator<
         std::__value_type<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>,
         std::__tree_node<
             std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                               tcmalloc::MallocExtension::Property>,
             void*>*,
         long>> (
        ::std::reverse_iterator<
            std::__map_const_iterator<std::__tree_const_iterator<
                std::__value_type<
                    std::basic_string<char, std::char_traits<char>,
                                      std::allocator<char>>,
                    tcmalloc::MallocExtension::Property>,
                std::__tree_node<
                    std::__value_type<
                        std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                        tcmalloc::MallocExtension::Property>,
                    void*>*,
                long>>>::*)() const) &
    ::std::reverse_iterator<
        std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char32_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char32_t*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPKDiEC1Ev(
    class std::reverse_iterator<const char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorIPKDiEC1ES2_(
    class std::reverse_iterator<const char32_t*>* __this, char32_t const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char32_t const*
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorIPKDiE4baseEv(
    class std::reverse_iterator<const char32_t*> const* __this) {
  return __this->base();
}

static_assert((char32_t const* (::std::reverse_iterator<const char32_t*>::*)()
                   const) &
              ::std::reverse_iterator<const char32_t*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char16_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char16_t*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPKDsEC1Ev(
    class std::reverse_iterator<const char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorIPKDsEC1ES2_(
    class std::reverse_iterator<const char16_t*>* __this, char16_t const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char16_t const*
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorIPKDsE4baseEv(
    class std::reverse_iterator<const char16_t*> const* __this) {
  return __this->base();
}

static_assert((char16_t const* (::std::reverse_iterator<const char16_t*>::*)()
                   const) &
              ::std::reverse_iterator<const char16_t*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char8_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char8_t*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPKDuEC1Ev(
    class std::reverse_iterator<const char8_t*>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<
                  const proto2::internal::ThreadSafeArenaStats::BlockStats*>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<
            const proto2::internal::ThreadSafeArenaStats::BlockStats*>) == 8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPKN6proto28internal20ThreadSafeArenaStats10BlockStatsEEC1Ev(
    class std::reverse_iterator<
        const proto2::internal::ThreadSafeArenaStats::BlockStats*>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const signed char*>) ==
              8);
static_assert(alignof(class std::reverse_iterator<const signed char*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPKaEC1Ev(
    class std::reverse_iterator<const signed char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorIPKaEC1ES2_(
    class std::reverse_iterator<const signed char*>* __this,
    signed char const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" signed char const*
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorIPKaE4baseEv(
    class std::reverse_iterator<const signed char*> const* __this) {
  return __this->base();
}

static_assert((signed char const* (
                  ::std::reverse_iterator<const signed char*>::*)() const) &
              ::std::reverse_iterator<const signed char*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
    class std::reverse_iterator<const char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
    class std::reverse_iterator<const char*>* __this, char const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char const*
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(
    class std::reverse_iterator<const char*> const* __this) {
  return __this->base();
}

static_assert((char const* (::std::reverse_iterator<const char*>::*)() const) &
              ::std::reverse_iterator<const char*>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<const unsigned char*>) == 8);
static_assert(alignof(class std::reverse_iterator<const unsigned char*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPKhEC1Ev(
    class std::reverse_iterator<const unsigned char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorIPKhEC1ES2_(
    class std::reverse_iterator<const unsigned char*>* __this,
    unsigned char const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" unsigned char const*
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorIPKhE4baseEv(
    class std::reverse_iterator<const unsigned char*> const* __this) {
  return __this->base();
}

static_assert((unsigned char const* (
                  ::std::reverse_iterator<const unsigned char*>::*)() const) &
              ::std::reverse_iterator<const unsigned char*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const wchar_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const wchar_t*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
    class std::reverse_iterator<const wchar_t*>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<
                  proto2::internal::ThreadSafeArenaStats::BlockStats*>) == 8);
static_assert(alignof(class std::reverse_iterator<
                      proto2::internal::ThreadSafeArenaStats::BlockStats*>) ==
              8);

extern "C" void
__rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPN6proto28internal20ThreadSafeArenaStats10BlockStatsEEC1Ev(
    class std::reverse_iterator<
        proto2::internal::ThreadSafeArenaStats::BlockStats*>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<signed char*>) == 8);
static_assert(alignof(class std::reverse_iterator<signed char*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPaEC1Ev(
    class std::reverse_iterator<signed char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorIPaEC1ES1_(
    class std::reverse_iterator<signed char*>* __this, signed char* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" signed char*
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorIPaE4baseEv(
    class std::reverse_iterator<signed char*> const* __this) {
  return __this->base();
}

static_assert((signed char* (::std::reverse_iterator<signed char*>::*)()
                   const) &
              ::std::reverse_iterator<signed char*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<char*>) == 8);
static_assert(alignof(class std::reverse_iterator<char*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPcEC1Ev(
    class std::reverse_iterator<char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorIPcEC1ES1_(
    class std::reverse_iterator<char*>* __this, char* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char* __rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorIPcE4baseEv(
    class std::reverse_iterator<char*> const* __this) {
  return __this->base();
}

static_assert((char* (::std::reverse_iterator<char*>::*)() const) &
              ::std::reverse_iterator<char*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<unsigned char*>) == 8);
static_assert(alignof(class std::reverse_iterator<unsigned char*>) == 8);

extern "C" void __rust_thunk__c18b6d95__ZNSt3__u16reverse_iteratorIPhEC1Ev(
    class std::reverse_iterator<unsigned char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__c64c0184__ZNSt3__u16reverse_iteratorIPhEC1ES1_(
    class std::reverse_iterator<unsigned char*>* __this, unsigned char* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" unsigned char*
__rust_thunk__2051bbba__ZNKSt3__u16reverse_iteratorIPhE4baseEv(
    class std::reverse_iterator<unsigned char*> const* __this) {
  return __this->base();
}

static_assert((unsigned char* (::std::reverse_iterator<unsigned char*>::*)()
                   const) &
              ::std::reverse_iterator<unsigned char*>::base);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<char32_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<char32_t*>) == 8);

extern "C" void __rust_thunk__36717f6b__ZNSt3__u11__wrap_iterIPDiEC1Ev(
    class std::__wrap_iter<char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__cc39332c__ZNKSt3__u11__wrap_iterIPDiEplEl(
    class std::__wrap_iter<char32_t*>* __return,
    class std::__wrap_iter<char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<char32_t*> (
                  ::std::__wrap_iter<char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char32_t*>::operator+);

extern "C" class std::__wrap_iter<char32_t*>*
__rust_thunk__6b083007__ZNSt3__u11__wrap_iterIPDiEpLEl(
    class std::__wrap_iter<char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<char32_t*> &
               (::std::__wrap_iter<char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char32_t*>::operator+=);

extern "C" void __rust_thunk__05594df0__ZNKSt3__u11__wrap_iterIPDiEmiEl(
    class std::__wrap_iter<char32_t*>* __return,
    class std::__wrap_iter<char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<char32_t*> (
                  ::std::__wrap_iter<char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char32_t*>::operator-);

extern "C" class std::__wrap_iter<char32_t*>*
__rust_thunk__581d232d__ZNSt3__u11__wrap_iterIPDiEmIEl(
    class std::__wrap_iter<char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<char32_t*> &
               (::std::__wrap_iter<char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char32_t*>::operator-=);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<char16_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<char16_t*>) == 8);

extern "C" void __rust_thunk__36717f6b__ZNSt3__u11__wrap_iterIPDsEC1Ev(
    class std::__wrap_iter<char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__cc39332c__ZNKSt3__u11__wrap_iterIPDsEplEl(
    class std::__wrap_iter<char16_t*>* __return,
    class std::__wrap_iter<char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<char16_t*> (
                  ::std::__wrap_iter<char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char16_t*>::operator+);

extern "C" class std::__wrap_iter<char16_t*>*
__rust_thunk__6b083007__ZNSt3__u11__wrap_iterIPDsEpLEl(
    class std::__wrap_iter<char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<char16_t*> &
               (::std::__wrap_iter<char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char16_t*>::operator+=);

extern "C" void __rust_thunk__05594df0__ZNKSt3__u11__wrap_iterIPDsEmiEl(
    class std::__wrap_iter<char16_t*>* __return,
    class std::__wrap_iter<char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<char16_t*> (
                  ::std::__wrap_iter<char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char16_t*>::operator-);

extern "C" class std::__wrap_iter<char16_t*>*
__rust_thunk__581d232d__ZNSt3__u11__wrap_iterIPDsEmIEl(
    class std::__wrap_iter<char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<char16_t*> &
               (::std::__wrap_iter<char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char16_t*>::operator-=);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const char32_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<const char32_t*>) == 8);

extern "C" void __rust_thunk__36717f6b__ZNSt3__u11__wrap_iterIPKDiEC1Ev(
    class std::__wrap_iter<const char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__cc39332c__ZNKSt3__u11__wrap_iterIPKDiEplEl(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::__wrap_iter<const char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char32_t*>::operator+);

extern "C" class std::__wrap_iter<const char32_t*>*
__rust_thunk__6b083007__ZNSt3__u11__wrap_iterIPKDiEpLEl(
    class std::__wrap_iter<const char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> &
               (::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char32_t*>::operator+=);

extern "C" void __rust_thunk__05594df0__ZNKSt3__u11__wrap_iterIPKDiEmiEl(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::__wrap_iter<const char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char32_t*>::operator-);

extern "C" class std::__wrap_iter<const char32_t*>*
__rust_thunk__581d232d__ZNSt3__u11__wrap_iterIPKDiEmIEl(
    class std::__wrap_iter<const char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> &
               (::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char32_t*>::operator-=);

extern "C" char32_t const*
__rust_thunk__dea06742__ZNKSt3__u11__wrap_iterIPKDiEixEl(
    class std::__wrap_iter<const char32_t*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char32_t const& (::std::__wrap_iter<const char32_t*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<const char32_t*>::operator[]);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const char16_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<const char16_t*>) == 8);

extern "C" void __rust_thunk__36717f6b__ZNSt3__u11__wrap_iterIPKDsEC1Ev(
    class std::__wrap_iter<const char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__cc39332c__ZNKSt3__u11__wrap_iterIPKDsEplEl(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::__wrap_iter<const char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char16_t*>::operator+);

extern "C" class std::__wrap_iter<const char16_t*>*
__rust_thunk__6b083007__ZNSt3__u11__wrap_iterIPKDsEpLEl(
    class std::__wrap_iter<const char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> &
               (::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char16_t*>::operator+=);

extern "C" void __rust_thunk__05594df0__ZNKSt3__u11__wrap_iterIPKDsEmiEl(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::__wrap_iter<const char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char16_t*>::operator-);

extern "C" class std::__wrap_iter<const char16_t*>*
__rust_thunk__581d232d__ZNSt3__u11__wrap_iterIPKDsEmIEl(
    class std::__wrap_iter<const char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> &
               (::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char16_t*>::operator-=);

extern "C" char16_t const*
__rust_thunk__dea06742__ZNKSt3__u11__wrap_iterIPKDsEixEl(
    class std::__wrap_iter<const char16_t*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char16_t const& (::std::__wrap_iter<const char16_t*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<const char16_t*>::operator[]);

static_assert(
    CRUBIT_SIZEOF(class std::__wrap_iter<const std::basic_string<
                      char, std::char_traits<char>, std::allocator<char>>*>) ==
    8);
static_assert(
    alignof(class std::__wrap_iter<const std::basic_string<
                char, std::char_traits<char>, std::allocator<char>>*>) == 8);

extern "C" void
__rust_thunk__36717f6b__ZNSt3__u11__wrap_iterIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEC1Ev(
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__cc39332c__ZNKSt3__u11__wrap_iterIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEplEl(
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>* __return,
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert(
    (class std::__wrap_iter<const std::basic_string<
         char, std::char_traits<char>, std::allocator<char>>*> (
        ::std::__wrap_iter<const std::basic_string<
            char, std::char_traits<char>, std::allocator<char>>*>::*)(ptrdiff_t)
         const) &
    ::std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>::operator+);

extern "C" class std::__wrap_iter<const std::basic_string<
    char, std::char_traits<char>, std::allocator<char>>*>*
__rust_thunk__6b083007__ZNSt3__u11__wrap_iterIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEpLEl(
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert(
    (class std::__wrap_iter<const std::basic_string<
         char, std::char_traits<char>, std::allocator<char>>*> &
     (::std::__wrap_iter<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>*>::*)(
         ptrdiff_t)) &
    ::std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>::operator+=);

extern "C" void
__rust_thunk__05594df0__ZNKSt3__u11__wrap_iterIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEmiEl(
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>* __return,
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert(
    (class std::__wrap_iter<const std::basic_string<
         char, std::char_traits<char>, std::allocator<char>>*> (
        ::std::__wrap_iter<const std::basic_string<
            char, std::char_traits<char>, std::allocator<char>>*>::*)(ptrdiff_t)
         const) &
    ::std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>::operator-);

extern "C" class std::__wrap_iter<const std::basic_string<
    char, std::char_traits<char>, std::allocator<char>>*>*
__rust_thunk__581d232d__ZNSt3__u11__wrap_iterIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEmIEl(
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert(
    (class std::__wrap_iter<const std::basic_string<
         char, std::char_traits<char>, std::allocator<char>>*> &
     (::std::__wrap_iter<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>*>::*)(
         ptrdiff_t)) &
    ::std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>::operator-=);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>> const*
__rust_thunk__dea06742__ZNKSt3__u11__wrap_iterIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEixEl(
    class std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*> const* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert(
    (class std::basic_string<
        char, std::char_traits<char>,
        std::allocator<char>> const& (::std::
                                          __wrap_iter<const std::basic_string<
                                              char, std::char_traits<char>,
                                              std::allocator<char>>*>::*)(
        ptrdiff_t) const) &
    ::std::__wrap_iter<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>::operator[]);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const char*>) == 8);
static_assert(alignof(class std::__wrap_iter<const char*>) == 8);

extern "C" void __rust_thunk__36717f6b__ZNSt3__u11__wrap_iterIPKcEC1Ev(
    class std::__wrap_iter<const char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__cc39332c__ZNKSt3__u11__wrap_iterIPKcEplEl(
    class std::__wrap_iter<const char*>* __return,
    class std::__wrap_iter<const char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const char*> (
                  ::std::__wrap_iter<const char*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char*>::operator+);

extern "C" class std::__wrap_iter<const char*>*
__rust_thunk__6b083007__ZNSt3__u11__wrap_iterIPKcEpLEl(
    class std::__wrap_iter<const char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const char*> &
               (::std::__wrap_iter<const char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char*>::operator+=);

extern "C" void __rust_thunk__05594df0__ZNKSt3__u11__wrap_iterIPKcEmiEl(
    class std::__wrap_iter<const char*>* __return,
    class std::__wrap_iter<const char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const char*> (
                  ::std::__wrap_iter<const char*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char*>::operator-);

extern "C" class std::__wrap_iter<const char*>*
__rust_thunk__581d232d__ZNSt3__u11__wrap_iterIPKcEmIEl(
    class std::__wrap_iter<const char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const char*> &
               (::std::__wrap_iter<const char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char*>::operator-=);

extern "C" char const* __rust_thunk__dea06742__ZNKSt3__u11__wrap_iterIPKcEixEl(
    class std::__wrap_iter<const char*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char const& (::std::__wrap_iter<const char*>::*)(ptrdiff_t)
                   const) &
              ::std::__wrap_iter<const char*>::operator[]);

static_assert(
    CRUBIT_SIZEOF(class std::__wrap_iter<std::basic_string<
                      char, std::char_traits<char>, std::allocator<char>>*>) ==
    8);
static_assert(
    alignof(class std::__wrap_iter<std::basic_string<
                char, std::char_traits<char>, std::allocator<char>>*>) == 8);

extern "C" void
__rust_thunk__36717f6b__ZNSt3__u11__wrap_iterIPNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEC1Ev(
    class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__cc39332c__ZNKSt3__u11__wrap_iterIPNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEplEl(
    class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>*>* __return,
    class std::__wrap_iter<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert(
    (class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>*> (
        ::std::__wrap_iter<std::basic_string<
            char, std::char_traits<char>, std::allocator<char>>*>::*)(ptrdiff_t)
         const) &
    ::std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                         std::allocator<char>>*>::operator+);

extern "C" class std::__wrap_iter<
    std::basic_string<char, std::char_traits<char>, std::allocator<char>>*>*
__rust_thunk__6b083007__ZNSt3__u11__wrap_iterIPNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEpLEl(
    class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert(
    (class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>*> &
     (::std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                           std::allocator<char>>*>::*)(
         ptrdiff_t)) &
    ::std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                         std::allocator<char>>*>::operator+=);

extern "C" void
__rust_thunk__05594df0__ZNKSt3__u11__wrap_iterIPNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEmiEl(
    class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>*>* __return,
    class std::__wrap_iter<std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*> const* __this,
    ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert(
    (class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>*> (
        ::std::__wrap_iter<std::basic_string<
            char, std::char_traits<char>, std::allocator<char>>*>::*)(ptrdiff_t)
         const) &
    ::std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                         std::allocator<char>>*>::operator-);

extern "C" class std::__wrap_iter<
    std::basic_string<char, std::char_traits<char>, std::allocator<char>>*>*
__rust_thunk__581d232d__ZNSt3__u11__wrap_iterIPNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEmIEl(
    class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>*>* __this,
    ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert(
    (class std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>*> &
     (::std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                           std::allocator<char>>*>::*)(
         ptrdiff_t)) &
    ::std::__wrap_iter<std::basic_string<char, std::char_traits<char>,
                                         std::allocator<char>>*>::operator-=);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<char*>) == 8);
static_assert(alignof(class std::__wrap_iter<char*>) == 8);

extern "C" void __rust_thunk__36717f6b__ZNSt3__u11__wrap_iterIPcEC1Ev(
    class std::__wrap_iter<char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__cc39332c__ZNKSt3__u11__wrap_iterIPcEplEl(
    class std::__wrap_iter<char*>* __return,
    class std::__wrap_iter<char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<char*> (::std::__wrap_iter<char*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<char*>::operator+);

extern "C" class std::__wrap_iter<char*>*
__rust_thunk__6b083007__ZNSt3__u11__wrap_iterIPcEpLEl(
    class std::__wrap_iter<char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<char*> &
               (::std::__wrap_iter<char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char*>::operator+=);

extern "C" void __rust_thunk__05594df0__ZNKSt3__u11__wrap_iterIPcEmiEl(
    class std::__wrap_iter<char*>* __return,
    class std::__wrap_iter<char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<char*> (::std::__wrap_iter<char*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<char*>::operator-);

extern "C" class std::__wrap_iter<char*>*
__rust_thunk__581d232d__ZNSt3__u11__wrap_iterIPcEmIEl(
    class std::__wrap_iter<char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<char*> &
               (::std::__wrap_iter<char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char*>::operator-=);

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
__rust_thunk__4270c2fe__ZNSt3__u19__allocation_resultIPDimEC1ES1_m(
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
__rust_thunk__4270c2fe__ZNSt3__u19__allocation_resultIPDsmEC1ES1_m(
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
__rust_thunk__4270c2fe__ZNSt3__u19__allocation_resultIPcmEC1ES1_m(
    struct std::__allocation_result<char*, unsigned long>* __this, char* __ptr,
    unsigned long __count) {
  crubit::construct_at(__this, __ptr, __count);
}

static_assert(
    sizeof(class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>) ==
    1);
static_assert(
    alignof(
        class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>) ==
    1);

extern "C" void
__rust_thunk__e95e64c8__ZNSt3__u9allocatorIN4absl12crc_internal12CrcCordState9PrefixCrcEEC1Ev(
    class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(class std::allocator<std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>>) == 1);
static_assert(
    alignof(class std::allocator<std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>>) ==
    1);

extern "C" void
__rust_thunk__e95e64c8__ZNSt3__u9allocatorINS_12basic_stringIcNS_11char_traitsIcEENS0_IcEEEEEC1Ev(
    class std::allocator<std::basic_string<char, std::char_traits<char>,
                                           std::allocator<char>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>>*
__rust_thunk__3f554b47__ZNSt3__u9allocatorINS_12basic_stringIcNS_11char_traitsIcEENS0_IcEEEEE8allocateEm(
    class std::allocator<std::basic_string<char, std::char_traits<char>,
                                           std::allocator<char>>>* __this,
    size_t __n) {
  return __this->allocate(__n);
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::allocator<char>> *
     (::std::allocator<std::basic_string<char, std::char_traits<char>,
                                         std::allocator<char>>>::*)(size_t)) &
    ::std::allocator<std::basic_string<char, std::char_traits<char>,
                                       std::allocator<char>>>::allocate);

extern "C" void
__rust_thunk__2e7db5c6__ZNSt3__u9allocatorINS_12basic_stringIcNS_11char_traitsIcEENS0_IcEEEEE10deallocateEPS5_m(
    class std::allocator<std::basic_string<char, std::char_traits<char>,
                                           std::allocator<char>>>* __this,
    class std::basic_string<char, std::char_traits<char>, std::allocator<char>>*
        __p,
    size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert(
    (void (::std::allocator<std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>>::*)(
        class std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>*,
        size_t)) &
    ::std::allocator<std::basic_string<char, std::char_traits<char>,
                                       std::allocator<char>>>::deallocate);

static_assert(
    sizeof(class std::allocator<
           std::pair<const std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>,
                     tcmalloc::MallocExtension::Property>>) == 1);
static_assert(
    alignof(class std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>) == 1);

extern "C" void
__rust_thunk__e95e64c8__ZNSt3__u9allocatorINS_4pairIKNS_12basic_stringIcNS_11char_traitsIcEENS0_IcEEEEN8tcmalloc15MallocExtension8PropertyEEEEC1Ev(
    class std::allocator<
        std::pair<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                  tcmalloc::MallocExtension::Property>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_iostream<wchar_t, std::char_traits<wchar_t>>) == 176);
static_assert(
    alignof(class std::basic_iostream<wchar_t, std::char_traits<wchar_t>>) ==
    8);

extern "C" void
__rust_thunk__0f78ccf8__ZNSt3__u14basic_iostreamIwNS_11char_traitsIwEEEC1EPNS_15basic_streambufIwS2_EE(
    class std::basic_iostream<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__5279c318__ZNSt3__u14basic_iostreamIwNS_11char_traitsIwEEED1Ev(
    class std::basic_iostream<wchar_t, std::char_traits<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u14basic_iostreamIwNS_11char_traitsIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_iostream<wchar_t, std::char_traits<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                             std::allocator<wchar_t>>) == 104);
static_assert(
    alignof(class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                       std::allocator<wchar_t>>) == 8);

extern "C" void
__rust_thunk__eedb4ac5__ZNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEED1Ev(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__ec6a4c9f__ZNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEC1Ev(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__4c22f3da__ZNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEC1Ej(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __this,
    unsigned int __wch) {
  crubit::construct_at(__this, __wch);
}

extern "C" void
__rust_thunk__70f2a896__ZNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEC1ERKS4_(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __this,
    class std::allocator<wchar_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__9adff16b__ZNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEC1EjRKS4_(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __this,
    unsigned int __wch, class std::allocator<wchar_t> const* __a) {
  crubit::construct_at(__this, __wch, *__a);
}

extern "C" void
__rust_thunk__727af024__ZNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEC1EOS5_(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __this,
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __rhs) {
  crubit::construct_at(__this, std::move(*__rhs));
}

extern "C" void
__rust_thunk__cc22688d__ZNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEC1EOS5_RKS4_(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __this,
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __rhs,
    class std::allocator<wchar_t> const* __a) {
  crubit::construct_at(__this, std::move(*__rhs), *__a);
}

extern "C" void
__rust_thunk__18338933__ZNKSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEE13get_allocatorEv(
    class std::allocator<wchar_t>* __return,
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>> const* __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert((class std::allocator<wchar_t> (
                  ::std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                         std::allocator<wchar_t>>::*)() const) &
              ::std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                     std::allocator<wchar_t>>::get_allocator);

extern "C" class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                      std::allocator<wchar_t>>*
__rust_thunk__3ae8c9e5__ZNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEaSEOS5_(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __this,
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __rhs) {
  return std::addressof(__this->operator=(std::move(*__rhs)));
}

static_assert(
    (class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                std::allocator<wchar_t>> &
     (::std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                             std::allocator<wchar_t>>::*)(
         class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                    std::allocator<wchar_t>>&&)) &
    ::std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                           std::allocator<wchar_t>>::operator=);

extern "C" void
__rust_thunk__5cb68783__ZNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEE4swapERS5_(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __this,
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* __rhs) {
  __this->swap(*__rhs);
}

static_assert((void (::std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                            std::allocator<wchar_t>>::*)(
                  class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                             std::allocator<wchar_t>>&)) &
              ::std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                     std::allocator<wchar_t>>::swap);

extern "C" void
__rust_thunk__c653e20f__ZNKSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEE4viewEv(
    ::std::__u::wstring_view* __return,
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>> const* __this) {
  new (__return) auto(__this->view());
}

static_assert((::std::__u::wstring_view (
                  ::std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                         std::allocator<wchar_t>>::*)() const) &
              ::std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                     std::allocator<wchar_t>>::view);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                       std::allocator<wchar_t>>) == 272);
static_assert(
    alignof(class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                           std::allocator<wchar_t>>) == 8);

extern "C" void
__rust_thunk__865ed8c3__ZNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEED1Ev(
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__5704dad6__ZNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1Ev(
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__5f8da0c5__ZNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1Ej(
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    unsigned int __wch) {
  crubit::construct_at(__this, __wch);
}

extern "C" void
__rust_thunk__1cca9be9__ZNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1EjRKS4_(
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    unsigned int __wch, class std::allocator<wchar_t> const* __a) {
  crubit::construct_at(__this, __wch, *__a);
}

extern "C" void
__rust_thunk__4286c151__ZNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1EOS5_(
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __rhs) {
  crubit::construct_at(__this, std::move(*__rhs));
}

extern "C" class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                          std::allocator<wchar_t>>*
__rust_thunk__ef6c2483__ZNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEaSEOS5_(
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __rhs) {
  return std::addressof(__this->operator=(std::move(*__rhs)));
}

static_assert(
    (class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                    std::allocator<wchar_t>> &
     (::std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                 std::allocator<wchar_t>>::*)(
         class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                        std::allocator<wchar_t>>&&)) &
    ::std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>::operator=);

extern "C" void
__rust_thunk__dabc3e01__ZNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEE4swapERS5_(
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __rhs) {
  __this->swap(*__rhs);
}

static_assert(
    (void (::std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                      std::allocator<wchar_t>>::*)(
        class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                       std::allocator<wchar_t>>&)) &
    ::std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>::swap);

extern "C" class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                      std::allocator<wchar_t>>*
__rust_thunk__b85a7068__ZNKSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEE5rdbufEv(
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>> const* __this) {
  return __this->rdbuf();
}

static_assert((class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                          std::allocator<wchar_t>> *
               (::std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                           std::allocator<wchar_t>>::*)()
                   const) &
              ::std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                         std::allocator<wchar_t>>::rdbuf);

extern "C" void
__rust_thunk__6bc76e8b__ZNKSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEE4viewEv(
    ::std::__u::wstring_view* __return,
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>> const* __this) {
  new (__return) auto(__this->view());
}

static_assert((::std::__u::wstring_view (
                  ::std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                             std::allocator<wchar_t>>::*)()
                   const) &
              ::std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                         std::allocator<wchar_t>>::view);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_istringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                       std::allocator<wchar_t>>) == 264);
static_assert(
    alignof(class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                           std::allocator<wchar_t>>) == 8);

extern "C" void
__rust_thunk__19c8811b__ZNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEED1Ev(
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__50329082__ZNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1Ev(
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__a55a42ca__ZNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1Ej(
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    unsigned int __wch) {
  crubit::construct_at(__this, __wch);
}

extern "C" void
__rust_thunk__13107d82__ZNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1EjRKS4_(
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    unsigned int __wch, class std::allocator<wchar_t> const* __a) {
  crubit::construct_at(__this, __wch, *__a);
}

extern "C" void
__rust_thunk__e71b908c__ZNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1EOS5_(
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __rhs) {
  crubit::construct_at(__this, std::move(*__rhs));
}

extern "C" class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                          std::allocator<wchar_t>>*
__rust_thunk__d47dd03c__ZNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEaSEOS5_(
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __rhs) {
  return std::addressof(__this->operator=(std::move(*__rhs)));
}

static_assert(
    (class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                    std::allocator<wchar_t>> &
     (::std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                 std::allocator<wchar_t>>::*)(
         class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                        std::allocator<wchar_t>>&&)) &
    ::std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>::operator=);

extern "C" void
__rust_thunk__dbbac04c__ZNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEE4swapERS5_(
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __this,
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* __rhs) {
  __this->swap(*__rhs);
}

static_assert(
    (void (::std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                      std::allocator<wchar_t>>::*)(
        class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                       std::allocator<wchar_t>>&)) &
    ::std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                               std::allocator<wchar_t>>::swap);

extern "C" class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                      std::allocator<wchar_t>>*
__rust_thunk__e5ea186e__ZNKSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEE5rdbufEv(
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>> const* __this) {
  return __this->rdbuf();
}

static_assert((class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                          std::allocator<wchar_t>> *
               (::std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                           std::allocator<wchar_t>>::*)()
                   const) &
              ::std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                         std::allocator<wchar_t>>::rdbuf);

extern "C" void
__rust_thunk__de3b3363__ZNKSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEE4viewEv(
    ::std::__u::wstring_view* __return,
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>> const* __this) {
  new (__return) auto(__this->view());
}

static_assert((::std::__u::wstring_view (
                  ::std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                             std::allocator<wchar_t>>::*)()
                   const) &
              ::std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                         std::allocator<wchar_t>>::view);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                      std::allocator<wchar_t>>) == 280);
static_assert(
    alignof(class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                          std::allocator<wchar_t>>) == 8);

extern "C" void
__rust_thunk__60e5c291__ZNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEED1Ev(
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__1674e7ab__ZNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1Ev(
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__fed9ac62__ZNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1Ej(
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __this,
    unsigned int __wch) {
  crubit::construct_at(__this, __wch);
}

extern "C" void
__rust_thunk__6fbc12f6__ZNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1EjRKS4_(
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __this,
    unsigned int __wch, class std::allocator<wchar_t> const* __a) {
  crubit::construct_at(__this, __wch, *__a);
}

extern "C" void
__rust_thunk__4b955580__ZNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEC1EOS5_(
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __this,
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __rhs) {
  crubit::construct_at(__this, std::move(*__rhs));
}

extern "C" class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                         std::allocator<wchar_t>>*
__rust_thunk__56175ff1__ZNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEaSEOS5_(
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __this,
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __rhs) {
  return std::addressof(__this->operator=(std::move(*__rhs)));
}

static_assert(
    (class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                   std::allocator<wchar_t>> &
     (::std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                std::allocator<wchar_t>>::*)(
         class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                       std::allocator<wchar_t>>&&)) &
    ::std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                              std::allocator<wchar_t>>::operator=);

extern "C" void
__rust_thunk__4cf9ff75__ZNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEE4swapERS5_(
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __this,
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* __rhs) {
  __this->swap(*__rhs);
}

static_assert(
    (void (::std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                     std::allocator<wchar_t>>::*)(
        class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                      std::allocator<wchar_t>>&)) &
    ::std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                              std::allocator<wchar_t>>::swap);

extern "C" class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                      std::allocator<wchar_t>>*
__rust_thunk__b026324e__ZNKSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEE5rdbufEv(
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>> const* __this) {
  return __this->rdbuf();
}

static_assert((class std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>,
                                          std::allocator<wchar_t>> *
               (::std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                          std::allocator<wchar_t>>::*)()
                   const) &
              ::std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                        std::allocator<wchar_t>>::rdbuf);

extern "C" void
__rust_thunk__ed9d55bd__ZNKSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEE4viewEv(
    ::std::__u::wstring_view* __return,
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>> const* __this) {
  new (__return) auto(__this->view());
}

static_assert((::std::__u::wstring_view (
                  ::std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                            std::allocator<wchar_t>>::*)()
                   const) &
              ::std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                        std::allocator<wchar_t>>::view);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_stringstream<wchar_t, std::char_traits<wchar_t>,
                                  std::allocator<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(CRUBIT_SIZEOF(class std::fpos<__mbstate_t>) == 16);
static_assert(alignof(class std::fpos<__mbstate_t>) == 8);

extern "C" void __rust_thunk__ca8cb3ad__ZNSt3__u4fposI11__mbstate_tEC1Ex(
    class std::fpos<__mbstate_t>* __this, long long __off) {
  crubit::construct_at(__this, __off);
}

extern "C" class std::fpos<__mbstate_t>*
__rust_thunk__04b7a048__ZNSt3__u4fposI11__mbstate_tEpLEx(
    class std::fpos<__mbstate_t>* __this, long long __off) {
  return std::addressof(__this->operator+=(__off));
}

static_assert((class std::fpos<__mbstate_t> &
               (::std::fpos<__mbstate_t>::*)(long long)) &
              ::std::fpos<__mbstate_t>::operator+=);

extern "C" void __rust_thunk__3a58c6da__ZNKSt3__u4fposI11__mbstate_tEplEx(
    class std::fpos<__mbstate_t>* __return,
    class std::fpos<__mbstate_t> const* __this, long long __off) {
  new (__return) auto(__this->operator+(__off));
}

static_assert((class std::fpos<__mbstate_t> (::std::fpos<__mbstate_t>::*)(
                  long long) const) &
              ::std::fpos<__mbstate_t>::operator+);

extern "C" class std::fpos<__mbstate_t>*
__rust_thunk__7cfa5761__ZNSt3__u4fposI11__mbstate_tEmIEx(
    class std::fpos<__mbstate_t>* __this, long long __off) {
  return std::addressof(__this->operator-=(__off));
}

static_assert((class std::fpos<__mbstate_t> &
               (::std::fpos<__mbstate_t>::*)(long long)) &
              ::std::fpos<__mbstate_t>::operator-=);

extern "C" void __rust_thunk__4bb7a208__ZNKSt3__u4fposI11__mbstate_tEmiEx(
    class std::fpos<__mbstate_t>* __return,
    class std::fpos<__mbstate_t> const* __this, long long __off) {
  new (__return) auto(__this->operator-(__off));
}

static_assert((class std::fpos<__mbstate_t> (::std::fpos<__mbstate_t>::*)(
                  long long) const) &
              ::std::fpos<__mbstate_t>::operator-);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>) ==
    16);
static_assert(
    alignof(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>) ==
    8);

extern "C" void
__rust_thunk__d11d3542__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1Ev(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c6c20ad4__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1EPKDi(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    char32_t const* __s) {
  crubit::construct_at(__this, __s);
}

extern "C" char32_t const*
__rust_thunk__9c02dba6__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5beginEv(
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
__rust_thunk__2400a8f6__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE3endEv(
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
__rust_thunk__719cca6b__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6cbeginEv(
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
__rust_thunk__0b859ab1__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4cendEv(
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
__rust_thunk__df48401c__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6rbeginEv(
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
__rust_thunk__5dbcc965__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4rendEv(
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
__rust_thunk__e0acf9a4__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE7crbeginEv(
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
__rust_thunk__66d598ab__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5crendEv(
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
__rust_thunk__43729707__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4sizeEv(
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
__rust_thunk__56929600__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6lengthEv(
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
__rust_thunk__b57842f5__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE8max_sizeEv(
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
__rust_thunk__7f34e5fe__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5emptyEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::empty);

extern "C" char32_t const*
__rust_thunk__ea428023__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEixEm(
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
__rust_thunk__64d54e07__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE2atEm(
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
__rust_thunk__5ab9e12d__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5frontEv(
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
__rust_thunk__7d44597a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4backEv(
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
__rust_thunk__59d24f91__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4dataEv(
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
__rust_thunk__38e48106__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_prefixEm(
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
__rust_thunk__4b120dc1__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_suffixEm(
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
__rust_thunk__9b1a4141__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4swapERS3_(
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
__rust_thunk__7a337b7c__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4copyEPDimm(
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
__rust_thunk__afc7f054__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6substrEmm(
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
__rust_thunk__d11d3542__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1Ev(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c6c20ad4__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1EPKDs(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    char16_t const* __s) {
  crubit::construct_at(__this, __s);
}

extern "C" char16_t const*
__rust_thunk__9c02dba6__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5beginEv(
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
__rust_thunk__2400a8f6__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE3endEv(
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
__rust_thunk__719cca6b__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6cbeginEv(
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
__rust_thunk__0b859ab1__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4cendEv(
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
__rust_thunk__df48401c__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6rbeginEv(
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
__rust_thunk__5dbcc965__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4rendEv(
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
__rust_thunk__e0acf9a4__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE7crbeginEv(
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
__rust_thunk__66d598ab__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5crendEv(
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
__rust_thunk__43729707__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4sizeEv(
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
__rust_thunk__56929600__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6lengthEv(
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
__rust_thunk__b57842f5__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE8max_sizeEv(
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
__rust_thunk__7f34e5fe__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5emptyEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::empty);

extern "C" char16_t const*
__rust_thunk__ea428023__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEixEm(
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
__rust_thunk__64d54e07__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE2atEm(
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
__rust_thunk__5ab9e12d__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5frontEv(
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
__rust_thunk__7d44597a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4backEv(
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
__rust_thunk__59d24f91__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4dataEv(
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
__rust_thunk__38e48106__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_prefixEm(
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
__rust_thunk__4b120dc1__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_suffixEm(
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
__rust_thunk__9b1a4141__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4swapERS3_(
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
__rust_thunk__7a337b7c__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4copyEPDsmm(
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
__rust_thunk__afc7f054__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6substrEmm(
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
__rust_thunk__d11d3542__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEC1Ev(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__df48401c__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6rbeginEv(
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
__rust_thunk__5dbcc965__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4rendEv(
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
__rust_thunk__e0acf9a4__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE7crbeginEv(
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
__rust_thunk__66d598ab__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5crendEv(
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
__rust_thunk__43729707__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4sizeEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::size);

extern "C" size_t
__rust_thunk__56929600__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6lengthEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->length();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::length);

extern "C" size_t
__rust_thunk__b57842f5__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE8max_sizeEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->max_size();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::max_size);

extern "C" bool
__rust_thunk__7f34e5fe__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5emptyEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::empty);

extern "C" void
__rust_thunk__38e48106__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_prefixEm(
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
__rust_thunk__4b120dc1__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_suffixEm(
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
__rust_thunk__9b1a4141__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4swapERS3_(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __other) {
  __this->swap(*__other);
}

static_assert(
    (void (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>&)) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::swap);

extern "C" void
__rust_thunk__afc7f054__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6substrEmm(
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
__rust_thunk__dc37b19c__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4findES3_m(
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
__rust_thunk__fea04da2__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5rfindES3_m(
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
__rust_thunk__35698774__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13find_first_ofES3_m(
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
__rust_thunk__fd6ec060__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE12find_last_ofES3_m(
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
__rust_thunk__a2aae637__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE17find_first_not_ofES3_m(
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
__rust_thunk__620c4384__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE16find_last_not_ofES3_m(
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
__rust_thunk__f83e8917__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE11starts_withES3_(
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
__rust_thunk__9dedcb9b__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE9ends_withES3_(
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

static_assert(sizeof(class std::ratio<1000000000000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000000000000L, 1L>) == 1);

extern "C" void
__rust_thunk__36026963__ZNSt3__u5ratioILl1000000000000000000ELl1EEC1Ev(
    class std::ratio<1000000000000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000000000L, 1L>) == 1);

extern "C" void
__rust_thunk__36026963__ZNSt3__u5ratioILl1000000000000000ELl1EEC1Ev(
    class std::ratio<1000000000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000000L, 1L>) == 1);

extern "C" void
__rust_thunk__36026963__ZNSt3__u5ratioILl1000000000000ELl1EEC1Ev(
    class std::ratio<1000000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl1000000000ELl1EEC1Ev(
    class std::ratio<1000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl1000000ELl1EEC1Ev(
    class std::ratio<1000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl1000ELl1EEC1Ev(
    class std::ratio<1000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<100L, 1L>) == 1);
static_assert(alignof(class std::ratio<100L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl100ELl1EEC1Ev(
    class std::ratio<100L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<10L, 1L>) == 1);
static_assert(alignof(class std::ratio<10L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl10ELl1EEC1Ev(
    class std::ratio<10L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000000000000L>) == 1);

extern "C" void
__rust_thunk__36026963__ZNSt3__u5ratioILl1ELl1000000000000000000EEC1Ev(
    class std::ratio<1L, 1000000000000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000000000L>) == 1);

extern "C" void
__rust_thunk__36026963__ZNSt3__u5ratioILl1ELl1000000000000000EEC1Ev(
    class std::ratio<1L, 1000000000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000000L>) == 1);

extern "C" void
__rust_thunk__36026963__ZNSt3__u5ratioILl1ELl1000000000000EEC1Ev(
    class std::ratio<1L, 1000000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl1ELl1000000000EEC1Ev(
    class std::ratio<1L, 1000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl1ELl1000000EEC1Ev(
    class std::ratio<1L, 1000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl1ELl1000EEC1Ev(
    class std::ratio<1L, 1000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 100L>) == 1);
static_assert(alignof(class std::ratio<1L, 100L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl1ELl100EEC1Ev(
    class std::ratio<1L, 100L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 10L>) == 1);
static_assert(alignof(class std::ratio<1L, 10L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl1ELl10EEC1Ev(
    class std::ratio<1L, 10L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1L>) == 1);
static_assert(alignof(class std::ratio<1L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl1ELl1EEC1Ev(
    class std::ratio<1L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<2629746L, 1L>) == 1);
static_assert(alignof(class std::ratio<2629746L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl2629746ELl1EEC1Ev(
    class std::ratio<2629746L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<31556952L, 1L>) == 1);
static_assert(alignof(class std::ratio<31556952L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl31556952ELl1EEC1Ev(
    class std::ratio<31556952L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<3600L, 1L>) == 1);
static_assert(alignof(class std::ratio<3600L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl3600ELl1EEC1Ev(
    class std::ratio<3600L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<604800L, 1L>) == 1);
static_assert(alignof(class std::ratio<604800L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl604800ELl1EEC1Ev(
    class std::ratio<604800L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<60L, 1L>) == 1);
static_assert(alignof(class std::ratio<60L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl60ELl1EEC1Ev(
    class std::ratio<60L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<86400L, 1L>) == 1);
static_assert(alignof(class std::ratio<86400L, 1L>) == 1);

extern "C" void __rust_thunk__36026963__ZNSt3__u5ratioILl86400ELl1EEC1Ev(
    class std::ratio<86400L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<int, std::ratio<2629746L, 1L>>) ==
    4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<2629746L, 1L>>) == 4);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<2629746L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<int, std::ratio<31556952L, 1L>>) == 4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<31556952L, 1L>>) == 4);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<31556952L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<int, std::ratio<604800L, 1L>>) ==
    4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<604800L, 1L>>) == 4);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<604800L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<int, std::ratio<86400L, 1L>>) ==
    4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<86400L, 1L>>) == 4);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<86400L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>) ==
    8);
static_assert(
    alignof(
        class std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>) ==
    8);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIlNS_5ratioILl1ELl1000000000000000EEEEC1Ev(
    class std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long, std::ratio<1L, 1L>>) == 8);
static_assert(alignof(class std::chrono::duration<long, std::ratio<1L, 1L>>) ==
              8);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIlNS_5ratioILl1ELl1EEEEC1Ev(
    class std::chrono::duration<long, std::ratio<1L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long, std::ratio<3600L, 1L>>) ==
    8);
static_assert(
    alignof(class std::chrono::duration<long, std::ratio<3600L, 1L>>) == 8);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEC1Ev(
    class std::chrono::duration<long, std::ratio<3600L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long, std::ratio<60L, 1L>>) == 8);
static_assert(alignof(class std::chrono::duration<long, std::ratio<60L, 1L>>) ==
              8);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEC1Ev(
    class std::chrono::duration<long, std::ratio<60L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>) ==
    16);
static_assert(
    alignof(
        class std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>) ==
    16);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationInNS_5ratioILl1ELl1000000000EEEEC1Ev(
    class std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>*
        __this) {
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
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEC1Ev(
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
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEC1Ev(
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
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEC1Ev(
    class std::chrono::duration<long long, std::ratio<1L, 1000L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long long, std::ratio<1L, 1L>>) ==
    8);
static_assert(
    alignof(class std::chrono::duration<long long, std::ratio<1L, 1L>>) == 8);

extern "C" void
__rust_thunk__5c6f0de0__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEC1Ev(
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
__rust_thunk__8e509e3d__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__814243de__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1ERKS6_(
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
__rust_thunk__8e509e3d__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<int, std::ratio<86400L, 1L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__814243de__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1ERKS6_(
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
__rust_thunk__8e509e3d__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__814243de__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1ERKS6_(
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
__rust_thunk__8e509e3d__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__814243de__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1L>>>* __this,
    class std::chrono::duration<long long, std::ratio<1L, 1L>> const* __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::time_point<
                  std::chrono::local_t,
                  std::chrono::duration<int, std::ratio<86400L, 1L>>>) == 4);
static_assert(alignof(class std::chrono::time_point<
                      std::chrono::local_t,
                      std::chrono::duration<int, std::ratio<86400L, 1L>>>) ==
              4);

extern "C" void
__rust_thunk__8e509e3d__ZNSt3__u6chrono10time_pointINS0_7local_tENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::local_t,
        std::chrono::duration<int, std::ratio<86400L, 1L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__814243de__ZNSt3__u6chrono10time_pointINS0_7local_tENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::local_t,
        std::chrono::duration<int, std::ratio<86400L, 1L>>>* __this,
    class std::chrono::duration<int, std::ratio<86400L, 1L>> const* __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::time_point<
                  std::chrono::local_t,
                  std::chrono::duration<long long, std::ratio<1L, 1L>>>) == 8);
static_assert(alignof(class std::chrono::time_point<
                      std::chrono::local_t,
                      std::chrono::duration<long long, std::ratio<1L, 1L>>>) ==
              8);

extern "C" void
__rust_thunk__8e509e3d__ZNSt3__u6chrono10time_pointINS0_7local_tENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::local_t,
        std::chrono::duration<long long, std::ratio<1L, 1L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__814243de__ZNSt3__u6chrono10time_pointINS0_7local_tENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::local_t,
        std::chrono::duration<long long, std::ratio<1L, 1L>>>* __this,
    class std::chrono::duration<long long, std::ratio<1L, 1L>> const* __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::time_point<
            std::filesystem::_FilesystemClock,
            std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>) ==
    16);
static_assert(
    alignof(class std::chrono::time_point<
            std::filesystem::_FilesystemClock,
            std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>) ==
    16);

extern "C" void
__rust_thunk__8e509e3d__ZNSt3__u6chrono10time_pointINS_10filesystem16_FilesystemClockENS0_8durationInNS_5ratioILl1ELl1000000000EEEEEEC1Ev(
    class std::chrono::time_point<
        std::filesystem::_FilesystemClock,
        std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__814243de__ZNSt3__u6chrono10time_pointINS_10filesystem16_FilesystemClockENS0_8durationInNS_5ratioILl1ELl1000000000EEEEEEC1ERKS7_(
    class std::chrono::time_point<
        std::filesystem::_FilesystemClock,
        std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>* __this,
    class std::chrono::duration<__int128, std::ratio<1L, 1000000000L>> const*
        __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char32_t, false>) == 4);
static_assert(alignof(struct std::__atomic_base<char32_t, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<char32_t, false>) ==
              0);

extern "C" struct std::__atomic_base<char32_t, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIDiLb0EEaSERKS1_(
    struct std::__atomic_base<char32_t, false>* __this,
    struct std::__atomic_base<char32_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char32_t, false> &
               (::std::__atomic_base<char32_t, false>::*)(
                   struct std::__atomic_base<char32_t, false> const&)) &
              ::std::__atomic_base<char32_t, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIDiLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char32_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char32_t, false>::*)() const) &
              ::std::__atomic_base<char32_t, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIDiLb0EE10notify_oneEv(
    struct std::__atomic_base<char32_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char32_t, false>::*)()) &
              ::std::__atomic_base<char32_t, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIDiLb0EE10notify_allEv(
    struct std::__atomic_base<char32_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char32_t, false>::*)()) &
              ::std::__atomic_base<char32_t, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIDiLb0EEC1Ev(
    struct std::__atomic_base<char32_t, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIDiLb0EEC1EDi(
    struct std::__atomic_base<char32_t, false>* __this, char32_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char16_t, false>) == 2);
static_assert(alignof(struct std::__atomic_base<char16_t, false>) == 2);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<char16_t, false>) ==
              0);

extern "C" struct std::__atomic_base<char16_t, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIDsLb0EEaSERKS1_(
    struct std::__atomic_base<char16_t, false>* __this,
    struct std::__atomic_base<char16_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char16_t, false> &
               (::std::__atomic_base<char16_t, false>::*)(
                   struct std::__atomic_base<char16_t, false> const&)) &
              ::std::__atomic_base<char16_t, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIDsLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char16_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char16_t, false>::*)() const) &
              ::std::__atomic_base<char16_t, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIDsLb0EE10notify_oneEv(
    struct std::__atomic_base<char16_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char16_t, false>::*)()) &
              ::std::__atomic_base<char16_t, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIDsLb0EE10notify_allEv(
    struct std::__atomic_base<char16_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char16_t, false>::*)()) &
              ::std::__atomic_base<char16_t, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIDsLb0EEC1Ev(
    struct std::__atomic_base<char16_t, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIDsLb0EEC1EDs(
    struct std::__atomic_base<char16_t, false>* __this, char16_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char8_t, false>) == 1);
static_assert(alignof(struct std::__atomic_base<char8_t, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<char8_t, false>) == 0);

extern "C" struct std::__atomic_base<char8_t, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIDuLb0EEaSERKS1_(
    struct std::__atomic_base<char8_t, false>* __this,
    struct std::__atomic_base<char8_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char8_t, false> &
               (::std::__atomic_base<char8_t, false>::*)(
                   struct std::__atomic_base<char8_t, false> const&)) &
              ::std::__atomic_base<char8_t, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIDuLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char8_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char8_t, false>::*)() const) &
              ::std::__atomic_base<char8_t, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIDuLb0EE10notify_oneEv(
    struct std::__atomic_base<char8_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char8_t, false>::*)()) &
              ::std::__atomic_base<char8_t, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIDuLb0EE10notify_allEv(
    struct std::__atomic_base<char8_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char8_t, false>::*)()) &
              ::std::__atomic_base<char8_t, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIDuLb0EEC1Ev(
    struct std::__atomic_base<char8_t, false>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__atomic_base<
                  absl::base_internal::PerThreadSynch::State, false>) == 4);
static_assert(alignof(struct std::__atomic_base<
                      absl::base_internal::PerThreadSynch::State, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(
                  __a_,
                  struct std::__atomic_base<
                      absl::base_internal::PerThreadSynch::State, false>) == 0);

extern "C" struct std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                                     false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIN4absl13base_internal14PerThreadSynch5StateELb0EEaSERKS5_(
    struct std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                              false>* __this,
    struct std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                              false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                               false> &
     (::std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                           false>::*)(
         struct std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                                   false> const&)) &
    ::std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                         false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIN4absl13base_internal14PerThreadSynch5StateELb0EE12is_lock_freeEv(
    struct std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                              false> const* __this) {
  return __this->is_lock_free();
}

static_assert(
    (bool (::std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                                false>::*)() const) &
    ::std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                         false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIN4absl13base_internal14PerThreadSynch5StateELb0EE10notify_oneEv(
    struct std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                              false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<
                     absl::base_internal::PerThreadSynch::State, false>::*)()) &
              ::std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                                   false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIN4absl13base_internal14PerThreadSynch5StateELb0EE10notify_allEv(
    struct std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                              false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<
                     absl::base_internal::PerThreadSynch::State, false>::*)()) &
              ::std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                                   false>::notify_all);

extern "C" void
__rust_thunk__713765c9__ZNSt3__u13__atomic_baseIN4absl13base_internal14PerThreadSynch5StateELb0EEC1Ev(
    struct std::__atomic_base<absl::base_internal::PerThreadSynch::State,
                              false>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::__atomic_base<
                     absl::base_internal::ThreadIdentity::WaitState, false>) ==
              1);
static_assert(alignof(struct std::__atomic_base<
                      absl::base_internal::ThreadIdentity::WaitState, false>) ==
              1);
static_assert(CRUBIT_OFFSET_OF(
                  __a_,
                  struct std::__atomic_base<
                      absl::base_internal::ThreadIdentity::WaitState, false>) ==
              0);

extern "C" struct std::__atomic_base<
    absl::base_internal::ThreadIdentity::WaitState, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIN4absl13base_internal14ThreadIdentity9WaitStateELb0EEaSERKS5_(
    struct std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                              false>* __this,
    struct std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                              false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                               false> &
     (::std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                           false>::*)(
         struct std::__atomic_base<
             absl::base_internal::ThreadIdentity::WaitState, false> const&)) &
    ::std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                         false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIN4absl13base_internal14ThreadIdentity9WaitStateELb0EE12is_lock_freeEv(
    struct std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                              false> const* __this) {
  return __this->is_lock_free();
}

static_assert(
    (bool (::std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                                false>::*)() const) &
    ::std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                         false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIN4absl13base_internal14ThreadIdentity9WaitStateELb0EE10notify_oneEv(
    struct std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                              false>* __this) {
  __this->notify_one();
}

static_assert(
    (void (::std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                                false>::*)()) &
    ::std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                         false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIN4absl13base_internal14ThreadIdentity9WaitStateELb0EE10notify_allEv(
    struct std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                              false>* __this) {
  __this->notify_all();
}

static_assert(
    (void (::std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                                false>::*)()) &
    ::std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                         false>::notify_all);

extern "C" void
__rust_thunk__713765c9__ZNSt3__u13__atomic_baseIN4absl13base_internal14ThreadIdentity9WaitStateELb0EEC1Ev(
    struct std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState,
                              false>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<
                            absl::flags_internal::MaskedPointer, false>) == 8);
static_assert(alignof(struct std::__atomic_base<
                      absl::flags_internal::MaskedPointer, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(
                  __a_,
                  struct std::__atomic_base<absl::flags_internal::MaskedPointer,
                                            false>) == 0);

extern "C" struct std::__atomic_base<absl::flags_internal::MaskedPointer,
                                     false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIN4absl14flags_internal13MaskedPointerELb0EEaSERKS4_(
    struct std::__atomic_base<absl::flags_internal::MaskedPointer, false>*
        __this,
    struct std::__atomic_base<absl::flags_internal::MaskedPointer, false> const*
        __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<absl::flags_internal::MaskedPointer, false> &
     (::std::__atomic_base<absl::flags_internal::MaskedPointer, false>::*)(
         struct std::__atomic_base<absl::flags_internal::MaskedPointer,
                                   false> const&)) &
    ::std::__atomic_base<absl::flags_internal::MaskedPointer,
                         false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIN4absl14flags_internal13MaskedPointerELb0EE12is_lock_freeEv(
    struct std::__atomic_base<absl::flags_internal::MaskedPointer, false> const*
        __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<absl::flags_internal::MaskedPointer,
                                          false>::*)() const) &
              ::std::__atomic_base<absl::flags_internal::MaskedPointer,
                                   false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIN4absl14flags_internal13MaskedPointerELb0EE10notify_oneEv(
    struct std::__atomic_base<absl::flags_internal::MaskedPointer, false>*
        __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<absl::flags_internal::MaskedPointer,
                                          false>::*)()) &
              ::std::__atomic_base<absl::flags_internal::MaskedPointer,
                                   false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIN4absl14flags_internal13MaskedPointerELb0EE10notify_allEv(
    struct std::__atomic_base<absl::flags_internal::MaskedPointer, false>*
        __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<absl::flags_internal::MaskedPointer,
                                          false>::*)()) &
              ::std::__atomic_base<absl::flags_internal::MaskedPointer,
                                   false>::notify_all);

extern "C" void
__rust_thunk__713765c9__ZNSt3__u13__atomic_baseIN4absl14flags_internal13MaskedPointerELb0EEC1Ev(
    struct std::__atomic_base<absl::flags_internal::MaskedPointer, false>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<
                            const char* (*(*)(const proto2::DescriptorPool*,
                                              const google::protobuf::Message*, int))(
                                const char*, proto2::internal::ParseContext*),
                            false>) == 8);
static_assert(alignof(struct std::__atomic_base<
                      const char* (*(*)(const proto2::DescriptorPool*,
                                        const google::protobuf::Message*, int))(
                          const char*, proto2::internal::ParseContext*),
                      false>) == 8);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<
                            const char* (*(*)(const proto2::DescriptorPool*,
                                              const google::protobuf::Message*, int))(
                                const char*, proto2::internal::ParseContext*),
                            false>) == 0);

extern "C" struct std::__atomic_base<
    const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                      int))(const char*, proto2::internal::ParseContext*),
    false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIPFPFPKcS2_PN6proto28internal12ParseContextEEPKNS3_14DescriptorPoolEPKNS3_7MessageEiELb0EEaSERKSH_(
    struct std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false>* __this,
    struct std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<
         const char* (*(*)(const proto2::DescriptorPool*,
                           const google::protobuf::Message*,
                           int))(const char*, proto2::internal::ParseContext*),
         false> &
     (::std::__atomic_base<const char* (*(*)(const proto2::DescriptorPool*,
                                             const google::protobuf::Message*, int))(
                               const char*, proto2::internal::ParseContext*),
                           false>::*)(
         struct std::__atomic_base<
             const char* (*(*)(const proto2::DescriptorPool*,
                               const google::protobuf::Message*, int))(
                 const char*, proto2::internal::ParseContext*),
             false> const&)) &
    ::std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIPFPFPKcS2_PN6proto28internal12ParseContextEEPKNS3_14DescriptorPoolEPKNS3_7MessageEiELb0EE12is_lock_freeEv(
    struct std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false> const* __this) {
  return __this->is_lock_free();
}

static_assert(
    (bool (
        ::std::__atomic_base<const char* (*(*)(const proto2::DescriptorPool*,
                                               const google::protobuf::Message*, int))(
                                 const char*, proto2::internal::ParseContext*),
                             false>::*)() const) &
    ::std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIPFPFPKcS2_PN6proto28internal12ParseContextEEPKNS3_14DescriptorPoolEPKNS3_7MessageEiELb0EE10notify_oneEv(
    struct std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false>* __this) {
  __this->notify_one();
}

static_assert(
    (void (
        ::std::__atomic_base<const char* (*(*)(const proto2::DescriptorPool*,
                                               const google::protobuf::Message*, int))(
                                 const char*, proto2::internal::ParseContext*),
                             false>::*)()) &
    ::std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIPFPFPKcS2_PN6proto28internal12ParseContextEEPKNS3_14DescriptorPoolEPKNS3_7MessageEiELb0EE10notify_allEv(
    struct std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false>* __this) {
  __this->notify_all();
}

static_assert(
    (void (
        ::std::__atomic_base<const char* (*(*)(const proto2::DescriptorPool*,
                                               const google::protobuf::Message*, int))(
                                 const char*, proto2::internal::ParseContext*),
                             false>::*)()) &
    ::std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false>::notify_all);

extern "C" void
__rust_thunk__713765c9__ZNSt3__u13__atomic_baseIPFPFPKcS2_PN6proto28internal12ParseContextEEPKNS3_14DescriptorPoolEPKNS3_7MessageEiELb0EEC1Ev(
    struct std::__atomic_base<
        const char* (*(*)(const proto2::DescriptorPool*, const google::protobuf::Message*,
                          int))(const char*, proto2::internal::ParseContext*),
        false>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                           proto2::HookLocation::Value),
                                  false>) == 8);
static_assert(
    alignof(struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                               proto2::HookLocation::Value),
                                      false>) == 8);
static_assert(
    CRUBIT_OFFSET_OF(
        __a_, struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                                 proto2::HookLocation::Value),
                                        false>) == 0);

extern "C" struct std::__atomic_base<
    void (*)(const google::protobuf::MessageLite&, proto2::HookLocation::Value), false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIPFvRKN6proto211MessageLiteENS1_12HookLocation5ValueEELb0EEaSERKS9_(
    struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                       proto2::HookLocation::Value),
                              false>* __this,
    struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                       proto2::HookLocation::Value),
                              false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                        proto2::HookLocation::Value),
                               false> &
     (::std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                    proto2::HookLocation::Value),
                           false>::*)(
         struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                            proto2::HookLocation::Value),
                                   false> const&)) &
    ::std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                  proto2::HookLocation::Value),
                         false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIPFvRKN6proto211MessageLiteENS1_12HookLocation5ValueEELb0EE12is_lock_freeEv(
    struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                       proto2::HookLocation::Value),
                              false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                                   proto2::HookLocation::Value),
                                          false>::*)() const) &
              ::std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                            proto2::HookLocation::Value),
                                   false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIPFvRKN6proto211MessageLiteENS1_12HookLocation5ValueEELb0EE10notify_oneEv(
    struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                       proto2::HookLocation::Value),
                              false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                                   proto2::HookLocation::Value),
                                          false>::*)()) &
              ::std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                            proto2::HookLocation::Value),
                                   false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIPFvRKN6proto211MessageLiteENS1_12HookLocation5ValueEELb0EE10notify_allEv(
    struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                       proto2::HookLocation::Value),
                              false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                                   proto2::HookLocation::Value),
                                          false>::*)()) &
              ::std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                            proto2::HookLocation::Value),
                                   false>::notify_all);

extern "C" void
__rust_thunk__713765c9__ZNSt3__u13__atomic_baseIPFvRKN6proto211MessageLiteENS1_12HookLocation5ValueEELb0EEC1Ev(
    struct std::__atomic_base<void (*)(const google::protobuf::MessageLite&,
                                       proto2::HookLocation::Value),
                              false>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__atomic_base<
                  const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>*,
                  false>) == 8);
static_assert(alignof(struct std::__atomic_base<
                      const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>*,
                      false>) == 8);
static_assert(
    CRUBIT_OFFSET_OF(__a_,
                     struct std::__atomic_base<
                         const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>*,
                         false>) == 0);

extern "C" struct std::__atomic_base<
    const std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>>*,
    false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EEaSERKS9_(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>*,
        false>* __this,
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>*,
        false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<
         const std::basic_string<char, std::char_traits<char>,
                                 std::allocator<char>>*,
         false> &
     (::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>*,
                           false>::*)(
         struct std::__atomic_base<
             const std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>*,
             false> const&)) &
    ::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>*,
                         false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EE12is_lock_freeEv(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>*,
        false> const* __this) {
  return __this->is_lock_free();
}

static_assert(
    (bool (::std::__atomic_base<
           const std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>>*,
           false>::*)() const) &
    ::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>*,
                         false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EE10notify_oneEv(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>*,
        false>* __this) {
  __this->notify_one();
}

static_assert(
    (void (::std::__atomic_base<
           const std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>>*,
           false>::*)()) &
    ::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>*,
                         false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EE10notify_allEv(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>*,
        false>* __this) {
  __this->notify_all();
}

static_assert(
    (void (::std::__atomic_base<
           const std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>>*,
           false>::*)()) &
    ::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>*,
                         false>::notify_all);

extern "C" void
__rust_thunk__713765c9__ZNSt3__u13__atomic_baseIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EEC1Ev(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>*,
        false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EEC1ES8_(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>*,
        false>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const* __d) {
  crubit::construct_at(__this, __d);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__atomic_base<
                  const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>**,
                  false>) == 8);
static_assert(alignof(struct std::__atomic_base<
                      const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>**,
                      false>) == 8);
static_assert(
    CRUBIT_OFFSET_OF(__a_,
                     struct std::__atomic_base<
                         const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>**,
                         false>) == 0);

extern "C" struct std::__atomic_base<
    const std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>>**,
    false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIPPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EEaSERKSA_(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>**,
        false>* __this,
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>**,
        false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<
         const std::basic_string<char, std::char_traits<char>,
                                 std::allocator<char>>**,
         false> &
     (::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>**,
                           false>::*)(
         struct std::__atomic_base<
             const std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>**,
             false> const&)) &
    ::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>**,
                         false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIPPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EE12is_lock_freeEv(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>**,
        false> const* __this) {
  return __this->is_lock_free();
}

static_assert(
    (bool (::std::__atomic_base<
           const std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>>**,
           false>::*)() const) &
    ::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>**,
                         false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIPPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EE10notify_oneEv(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>**,
        false>* __this) {
  __this->notify_one();
}

static_assert(
    (void (::std::__atomic_base<
           const std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>>**,
           false>::*)()) &
    ::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>**,
                         false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIPPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EE10notify_allEv(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>**,
        false>* __this) {
  __this->notify_all();
}

static_assert(
    (void (::std::__atomic_base<
           const std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>>**,
           false>::*)()) &
    ::std::__atomic_base<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>**,
                         false>::notify_all);

extern "C" void
__rust_thunk__713765c9__ZNSt3__u13__atomic_baseIPPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EEC1Ev(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>**,
        false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIPPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEELb0EEC1ES9_(
    struct std::__atomic_base<
        const std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>>**,
        false>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const** __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<void*, false>) == 8);
static_assert(alignof(struct std::__atomic_base<void*, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<void*, false>) ==
              0);

extern "C" struct std::__atomic_base<void*, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIPvLb0EEaSERKS2_(
    struct std::__atomic_base<void*, false>* __this,
    struct std::__atomic_base<void*, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<void*, false> &
               (::std::__atomic_base<void*, false>::*)(
                   struct std::__atomic_base<void*, false> const&)) &
              ::std::__atomic_base<void*, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIPvLb0EE12is_lock_freeEv(
    struct std::__atomic_base<void*, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<void*, false>::*)() const) &
              ::std::__atomic_base<void*, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIPvLb0EE10notify_oneEv(
    struct std::__atomic_base<void*, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<void*, false>::*)()) &
              ::std::__atomic_base<void*, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIPvLb0EE10notify_allEv(
    struct std::__atomic_base<void*, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<void*, false>::*)()) &
              ::std::__atomic_base<void*, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIPvLb0EEC1Ev(
    struct std::__atomic_base<void*, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIPvLb0EEC1ES1_(
    struct std::__atomic_base<void*, false>* __this, void* __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<signed char, false>) == 1);
static_assert(alignof(struct std::__atomic_base<signed char, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<signed char, false>) ==
              0);

extern "C" struct std::__atomic_base<signed char, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIaLb0EEaSERKS1_(
    struct std::__atomic_base<signed char, false>* __this,
    struct std::__atomic_base<signed char, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<signed char, false> &
               (::std::__atomic_base<signed char, false>::*)(
                   struct std::__atomic_base<signed char, false> const&)) &
              ::std::__atomic_base<signed char, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIaLb0EE12is_lock_freeEv(
    struct std::__atomic_base<signed char, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<signed char, false>::*)() const) &
              ::std::__atomic_base<signed char, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIaLb0EE10notify_oneEv(
    struct std::__atomic_base<signed char, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<signed char, false>::*)()) &
              ::std::__atomic_base<signed char, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIaLb0EE10notify_allEv(
    struct std::__atomic_base<signed char, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<signed char, false>::*)()) &
              ::std::__atomic_base<signed char, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIaLb0EEC1Ev(
    struct std::__atomic_base<signed char, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIaLb0EEC1Ea(
    struct std::__atomic_base<signed char, false>* __this, signed char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<bool, false>) == 1);
static_assert(alignof(struct std::__atomic_base<bool, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<bool, false>) ==
              0);

extern "C" struct std::__atomic_base<bool, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIbLb0EEaSERKS1_(
    struct std::__atomic_base<bool, false>* __this,
    struct std::__atomic_base<bool, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<bool, false> &
               (::std::__atomic_base<bool, false>::*)(
                   struct std::__atomic_base<bool, false> const&)) &
              ::std::__atomic_base<bool, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIbLb0EE12is_lock_freeEv(
    struct std::__atomic_base<bool, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<bool, false>::*)() const) &
              ::std::__atomic_base<bool, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIbLb0EE10notify_oneEv(
    struct std::__atomic_base<bool, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<bool, false>::*)()) &
              ::std::__atomic_base<bool, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIbLb0EE10notify_allEv(
    struct std::__atomic_base<bool, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<bool, false>::*)()) &
              ::std::__atomic_base<bool, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIbLb0EEC1Ev(
    struct std::__atomic_base<bool, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIbLb0EEC1Eb(
    struct std::__atomic_base<bool, false>* __this, bool __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char, false>) == 1);
static_assert(alignof(struct std::__atomic_base<char, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<char, false>) ==
              0);

extern "C" struct std::__atomic_base<char, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIcLb0EEaSERKS1_(
    struct std::__atomic_base<char, false>* __this,
    struct std::__atomic_base<char, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char, false> &
               (::std::__atomic_base<char, false>::*)(
                   struct std::__atomic_base<char, false> const&)) &
              ::std::__atomic_base<char, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIcLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char, false>::*)() const) &
              ::std::__atomic_base<char, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIcLb0EE10notify_oneEv(
    struct std::__atomic_base<char, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char, false>::*)()) &
              ::std::__atomic_base<char, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIcLb0EE10notify_allEv(
    struct std::__atomic_base<char, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char, false>::*)()) &
              ::std::__atomic_base<char, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIcLb0EEC1Ev(
    struct std::__atomic_base<char, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIcLb0EEC1Ec(
    struct std::__atomic_base<char, false>* __this, char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<unsigned char, false>) == 1);
static_assert(alignof(struct std::__atomic_base<unsigned char, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned char, false>) == 0);

extern "C" struct std::__atomic_base<unsigned char, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIhLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned char, false>* __this,
    struct std::__atomic_base<unsigned char, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned char, false> &
               (::std::__atomic_base<unsigned char, false>::*)(
                   struct std::__atomic_base<unsigned char, false> const&)) &
              ::std::__atomic_base<unsigned char, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIhLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned char, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned char, false>::*)() const) &
              ::std::__atomic_base<unsigned char, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIhLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned char, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned char, false>::*)()) &
              ::std::__atomic_base<unsigned char, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIhLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned char, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned char, false>::*)()) &
              ::std::__atomic_base<unsigned char, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIhLb0EEC1Ev(
    struct std::__atomic_base<unsigned char, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIhLb0EEC1Eh(
    struct std::__atomic_base<unsigned char, false>* __this,
    unsigned char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<int, false>) == 4);
static_assert(alignof(struct std::__atomic_base<int, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<int, false>) ==
              0);

extern "C" struct std::__atomic_base<int, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIiLb0EEaSERKS1_(
    struct std::__atomic_base<int, false>* __this,
    struct std::__atomic_base<int, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<int, false> &
               (::std::__atomic_base<int, false>::*)(
                   struct std::__atomic_base<int, false> const&)) &
              ::std::__atomic_base<int, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIiLb0EE12is_lock_freeEv(
    struct std::__atomic_base<int, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<int, false>::*)() const) &
              ::std::__atomic_base<int, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIiLb0EE10notify_oneEv(
    struct std::__atomic_base<int, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<int, false>::*)()) &
              ::std::__atomic_base<int, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIiLb0EE10notify_allEv(
    struct std::__atomic_base<int, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<int, false>::*)()) &
              ::std::__atomic_base<int, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIiLb0EEC1Ev(
    struct std::__atomic_base<int, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIiLb0EEC1Ei(
    struct std::__atomic_base<int, false>* __this, int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned int, false>) ==
              4);
static_assert(alignof(struct std::__atomic_base<unsigned int, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned int, false>) == 0);

extern "C" struct std::__atomic_base<unsigned int, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIjLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned int, false>* __this,
    struct std::__atomic_base<unsigned int, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned int, false> &
               (::std::__atomic_base<unsigned int, false>::*)(
                   struct std::__atomic_base<unsigned int, false> const&)) &
              ::std::__atomic_base<unsigned int, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIjLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned int, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned int, false>::*)() const) &
              ::std::__atomic_base<unsigned int, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIjLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned int, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned int, false>::*)()) &
              ::std::__atomic_base<unsigned int, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIjLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned int, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned int, false>::*)()) &
              ::std::__atomic_base<unsigned int, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIjLb0EEC1Ev(
    struct std::__atomic_base<unsigned int, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIjLb0EEC1Ej(
    struct std::__atomic_base<unsigned int, false>* __this, unsigned int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long, false>) == 8);
static_assert(alignof(struct std::__atomic_base<long, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<long, false>) ==
              0);

extern "C" struct std::__atomic_base<long, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIlLb0EEaSERKS1_(
    struct std::__atomic_base<long, false>* __this,
    struct std::__atomic_base<long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long, false> &
               (::std::__atomic_base<long, false>::*)(
                   struct std::__atomic_base<long, false> const&)) &
              ::std::__atomic_base<long, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIlLb0EE12is_lock_freeEv(
    struct std::__atomic_base<long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<long, false>::*)() const) &
              ::std::__atomic_base<long, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIlLb0EE10notify_oneEv(
    struct std::__atomic_base<long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<long, false>::*)()) &
              ::std::__atomic_base<long, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIlLb0EE10notify_allEv(
    struct std::__atomic_base<long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<long, false>::*)()) &
              ::std::__atomic_base<long, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIlLb0EEC1Ev(
    struct std::__atomic_base<long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIlLb0EEC1El(
    struct std::__atomic_base<long, false>* __this, long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long, false>) ==
              8);
static_assert(alignof(struct std::__atomic_base<unsigned long, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned long, false>) == 0);

extern "C" struct std::__atomic_base<unsigned long, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseImLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned long, false>* __this,
    struct std::__atomic_base<unsigned long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned long, false> &
               (::std::__atomic_base<unsigned long, false>::*)(
                   struct std::__atomic_base<unsigned long, false> const&)) &
              ::std::__atomic_base<unsigned long, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseImLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned long, false>::*)() const) &
              ::std::__atomic_base<unsigned long, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseImLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned long, false>::*)()) &
              ::std::__atomic_base<unsigned long, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseImLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned long, false>::*)()) &
              ::std::__atomic_base<unsigned long, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseImLb0EEC1Ev(
    struct std::__atomic_base<unsigned long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseImLb0EEC1Em(
    struct std::__atomic_base<unsigned long, false>* __this,
    unsigned long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<short, false>) == 2);
static_assert(alignof(struct std::__atomic_base<short, false>) == 2);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<short, false>) ==
              0);

extern "C" struct std::__atomic_base<short, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIsLb0EEaSERKS1_(
    struct std::__atomic_base<short, false>* __this,
    struct std::__atomic_base<short, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<short, false> &
               (::std::__atomic_base<short, false>::*)(
                   struct std::__atomic_base<short, false> const&)) &
              ::std::__atomic_base<short, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIsLb0EE12is_lock_freeEv(
    struct std::__atomic_base<short, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<short, false>::*)() const) &
              ::std::__atomic_base<short, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIsLb0EE10notify_oneEv(
    struct std::__atomic_base<short, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<short, false>::*)()) &
              ::std::__atomic_base<short, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIsLb0EE10notify_allEv(
    struct std::__atomic_base<short, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<short, false>::*)()) &
              ::std::__atomic_base<short, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIsLb0EEC1Ev(
    struct std::__atomic_base<short, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIsLb0EEC1Es(
    struct std::__atomic_base<short, false>* __this, short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned short, false>) ==
              2);
static_assert(alignof(struct std::__atomic_base<unsigned short, false>) == 2);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned short, false>) == 0);

extern "C" struct std::__atomic_base<unsigned short, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseItLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned short, false>* __this,
    struct std::__atomic_base<unsigned short, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned short, false> &
               (::std::__atomic_base<unsigned short, false>::*)(
                   struct std::__atomic_base<unsigned short, false> const&)) &
              ::std::__atomic_base<unsigned short, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseItLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned short, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned short, false>::*)() const) &
              ::std::__atomic_base<unsigned short, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseItLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned short, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned short, false>::*)()) &
              ::std::__atomic_base<unsigned short, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseItLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned short, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned short, false>::*)()) &
              ::std::__atomic_base<unsigned short, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseItLb0EEC1Ev(
    struct std::__atomic_base<unsigned short, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseItLb0EEC1Et(
    struct std::__atomic_base<unsigned short, false>* __this,
    unsigned short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<wchar_t, false>) == 4);
static_assert(alignof(struct std::__atomic_base<wchar_t, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<wchar_t, false>) == 0);

extern "C" struct std::__atomic_base<wchar_t, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIwLb0EEaSERKS1_(
    struct std::__atomic_base<wchar_t, false>* __this,
    struct std::__atomic_base<wchar_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<wchar_t, false> &
               (::std::__atomic_base<wchar_t, false>::*)(
                   struct std::__atomic_base<wchar_t, false> const&)) &
              ::std::__atomic_base<wchar_t, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIwLb0EE12is_lock_freeEv(
    struct std::__atomic_base<wchar_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<wchar_t, false>::*)() const) &
              ::std::__atomic_base<wchar_t, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIwLb0EE10notify_oneEv(
    struct std::__atomic_base<wchar_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<wchar_t, false>::*)()) &
              ::std::__atomic_base<wchar_t, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIwLb0EE10notify_allEv(
    struct std::__atomic_base<wchar_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<wchar_t, false>::*)()) &
              ::std::__atomic_base<wchar_t, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIwLb0EEC1Ev(
    struct std::__atomic_base<wchar_t, false>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long long, false>) == 8);
static_assert(alignof(struct std::__atomic_base<long long, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<long long, false>) ==
              0);

extern "C" struct std::__atomic_base<long long, false>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIxLb0EEaSERKS1_(
    struct std::__atomic_base<long long, false>* __this,
    struct std::__atomic_base<long long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long long, false> &
               (::std::__atomic_base<long long, false>::*)(
                   struct std::__atomic_base<long long, false> const&)) &
              ::std::__atomic_base<long long, false>::operator=);

extern "C" bool
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIxLb0EE12is_lock_freeEv(
    struct std::__atomic_base<long long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<long long, false>::*)() const) &
              ::std::__atomic_base<long long, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIxLb0EE10notify_oneEv(
    struct std::__atomic_base<long long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<long long, false>::*)()) &
              ::std::__atomic_base<long long, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIxLb0EE10notify_allEv(
    struct std::__atomic_base<long long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<long long, false>::*)()) &
              ::std::__atomic_base<long long, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIxLb0EEC1Ev(
    struct std::__atomic_base<long long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIxLb0EEC1Ex(
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
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIyLb0EEaSERKS1_(
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
__rust_thunk__7a2b8b61__ZNKSt3__u13__atomic_baseIyLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned long long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned long long, false>::*)()
                   const) &
              ::std::__atomic_base<unsigned long long, false>::is_lock_free);

extern "C" void
__rust_thunk__d2298d60__ZNSt3__u13__atomic_baseIyLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned long long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned long long, false>::*)()) &
              ::std::__atomic_base<unsigned long long, false>::notify_one);

extern "C" void
__rust_thunk__528680cd__ZNSt3__u13__atomic_baseIyLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned long long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned long long, false>::*)()) &
              ::std::__atomic_base<unsigned long long, false>::notify_all);

extern "C" void __rust_thunk__713765c9__ZNSt3__u13__atomic_baseIyLb0EEC1Ev(
    struct std::__atomic_base<unsigned long long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__73ab5274__ZNSt3__u13__atomic_baseIyLb0EEC1Ey(
    struct std::__atomic_base<unsigned long long, false>* __this,
    unsigned long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char32_t, true>) == 4);
static_assert(alignof(struct std::__atomic_base<char32_t, true>) == 4);

extern "C" struct std::__atomic_base<char32_t, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIDiLb1EEaSEOS1_(
    struct std::__atomic_base<char32_t, true>* __this,
    struct std::__atomic_base<char32_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char32_t, true> &
               (::std::__atomic_base<char32_t, true>::*)(
                   struct std::__atomic_base<char32_t, true>&&)) &
              ::std::__atomic_base<char32_t, true>::operator=);

extern "C" struct std::__atomic_base<char32_t, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIDiLb1EEaSERKS1_(
    struct std::__atomic_base<char32_t, true>* __this,
    struct std::__atomic_base<char32_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char32_t, true> &
               (::std::__atomic_base<char32_t, true>::*)(
                   struct std::__atomic_base<char32_t, true> const&)) &
              ::std::__atomic_base<char32_t, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIDiLb1EEC1Ev(
    struct std::__atomic_base<char32_t, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIDiLb1EEC1EDi(
    struct std::__atomic_base<char32_t, true>* __this, char32_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char16_t, true>) == 2);
static_assert(alignof(struct std::__atomic_base<char16_t, true>) == 2);

extern "C" struct std::__atomic_base<char16_t, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIDsLb1EEaSEOS1_(
    struct std::__atomic_base<char16_t, true>* __this,
    struct std::__atomic_base<char16_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char16_t, true> &
               (::std::__atomic_base<char16_t, true>::*)(
                   struct std::__atomic_base<char16_t, true>&&)) &
              ::std::__atomic_base<char16_t, true>::operator=);

extern "C" struct std::__atomic_base<char16_t, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIDsLb1EEaSERKS1_(
    struct std::__atomic_base<char16_t, true>* __this,
    struct std::__atomic_base<char16_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char16_t, true> &
               (::std::__atomic_base<char16_t, true>::*)(
                   struct std::__atomic_base<char16_t, true> const&)) &
              ::std::__atomic_base<char16_t, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIDsLb1EEC1Ev(
    struct std::__atomic_base<char16_t, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIDsLb1EEC1EDs(
    struct std::__atomic_base<char16_t, true>* __this, char16_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char8_t, true>) == 1);
static_assert(alignof(struct std::__atomic_base<char8_t, true>) == 1);

extern "C" struct std::__atomic_base<char8_t, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIDuLb1EEaSEOS1_(
    struct std::__atomic_base<char8_t, true>* __this,
    struct std::__atomic_base<char8_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char8_t, true> &
               (::std::__atomic_base<char8_t, true>::*)(
                   struct std::__atomic_base<char8_t, true>&&)) &
              ::std::__atomic_base<char8_t, true>::operator=);

extern "C" struct std::__atomic_base<char8_t, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIDuLb1EEaSERKS1_(
    struct std::__atomic_base<char8_t, true>* __this,
    struct std::__atomic_base<char8_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char8_t, true> &
               (::std::__atomic_base<char8_t, true>::*)(
                   struct std::__atomic_base<char8_t, true> const&)) &
              ::std::__atomic_base<char8_t, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIDuLb1EEC1Ev(
    struct std::__atomic_base<char8_t, true>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::__atomic_base<signed char, true>) == 1);
static_assert(alignof(struct std::__atomic_base<signed char, true>) == 1);

extern "C" struct std::__atomic_base<signed char, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIaLb1EEaSEOS1_(
    struct std::__atomic_base<signed char, true>* __this,
    struct std::__atomic_base<signed char, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<signed char, true> &
               (::std::__atomic_base<signed char, true>::*)(
                   struct std::__atomic_base<signed char, true>&&)) &
              ::std::__atomic_base<signed char, true>::operator=);

extern "C" struct std::__atomic_base<signed char, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIaLb1EEaSERKS1_(
    struct std::__atomic_base<signed char, true>* __this,
    struct std::__atomic_base<signed char, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<signed char, true> &
               (::std::__atomic_base<signed char, true>::*)(
                   struct std::__atomic_base<signed char, true> const&)) &
              ::std::__atomic_base<signed char, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIaLb1EEC1Ev(
    struct std::__atomic_base<signed char, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIaLb1EEC1Ea(
    struct std::__atomic_base<signed char, true>* __this, signed char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char, true>) == 1);
static_assert(alignof(struct std::__atomic_base<char, true>) == 1);

extern "C" struct std::__atomic_base<char, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIcLb1EEaSEOS1_(
    struct std::__atomic_base<char, true>* __this,
    struct std::__atomic_base<char, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char, true> &
               (::std::__atomic_base<char, true>::*)(
                   struct std::__atomic_base<char, true>&&)) &
              ::std::__atomic_base<char, true>::operator=);

extern "C" struct std::__atomic_base<char, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIcLb1EEaSERKS1_(
    struct std::__atomic_base<char, true>* __this,
    struct std::__atomic_base<char, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char, true> &
               (::std::__atomic_base<char, true>::*)(
                   struct std::__atomic_base<char, true> const&)) &
              ::std::__atomic_base<char, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIcLb1EEC1Ev(
    struct std::__atomic_base<char, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIcLb1EEC1Ec(
    struct std::__atomic_base<char, true>* __this, char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<unsigned char, true>) == 1);
static_assert(alignof(struct std::__atomic_base<unsigned char, true>) == 1);

extern "C" struct std::__atomic_base<unsigned char, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIhLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned char, true>* __this,
    struct std::__atomic_base<unsigned char, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned char, true> &
               (::std::__atomic_base<unsigned char, true>::*)(
                   struct std::__atomic_base<unsigned char, true>&&)) &
              ::std::__atomic_base<unsigned char, true>::operator=);

extern "C" struct std::__atomic_base<unsigned char, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIhLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned char, true>* __this,
    struct std::__atomic_base<unsigned char, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned char, true> &
               (::std::__atomic_base<unsigned char, true>::*)(
                   struct std::__atomic_base<unsigned char, true> const&)) &
              ::std::__atomic_base<unsigned char, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIhLb1EEC1Ev(
    struct std::__atomic_base<unsigned char, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIhLb1EEC1Eh(
    struct std::__atomic_base<unsigned char, true>* __this, unsigned char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<int, true>) == 4);
static_assert(alignof(struct std::__atomic_base<int, true>) == 4);

extern "C" struct std::__atomic_base<int, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIiLb1EEaSEOS1_(
    struct std::__atomic_base<int, true>* __this,
    struct std::__atomic_base<int, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<int, true> &
               (::std::__atomic_base<int, true>::*)(
                   struct std::__atomic_base<int, true>&&)) &
              ::std::__atomic_base<int, true>::operator=);

extern "C" struct std::__atomic_base<int, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIiLb1EEaSERKS1_(
    struct std::__atomic_base<int, true>* __this,
    struct std::__atomic_base<int, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<int, true> &
               (::std::__atomic_base<int, true>::*)(
                   struct std::__atomic_base<int, true> const&)) &
              ::std::__atomic_base<int, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIiLb1EEC1Ev(
    struct std::__atomic_base<int, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIiLb1EEC1Ei(
    struct std::__atomic_base<int, true>* __this, int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned int, true>) ==
              4);
static_assert(alignof(struct std::__atomic_base<unsigned int, true>) == 4);

extern "C" struct std::__atomic_base<unsigned int, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIjLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned int, true>* __this,
    struct std::__atomic_base<unsigned int, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned int, true> &
               (::std::__atomic_base<unsigned int, true>::*)(
                   struct std::__atomic_base<unsigned int, true>&&)) &
              ::std::__atomic_base<unsigned int, true>::operator=);

extern "C" struct std::__atomic_base<unsigned int, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIjLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned int, true>* __this,
    struct std::__atomic_base<unsigned int, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned int, true> &
               (::std::__atomic_base<unsigned int, true>::*)(
                   struct std::__atomic_base<unsigned int, true> const&)) &
              ::std::__atomic_base<unsigned int, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIjLb1EEC1Ev(
    struct std::__atomic_base<unsigned int, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIjLb1EEC1Ej(
    struct std::__atomic_base<unsigned int, true>* __this, unsigned int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long, true>) == 8);
static_assert(alignof(struct std::__atomic_base<long, true>) == 8);

extern "C" struct std::__atomic_base<long, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIlLb1EEaSEOS1_(
    struct std::__atomic_base<long, true>* __this,
    struct std::__atomic_base<long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<long, true> &
               (::std::__atomic_base<long, true>::*)(
                   struct std::__atomic_base<long, true>&&)) &
              ::std::__atomic_base<long, true>::operator=);

extern "C" struct std::__atomic_base<long, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIlLb1EEaSERKS1_(
    struct std::__atomic_base<long, true>* __this,
    struct std::__atomic_base<long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long, true> &
               (::std::__atomic_base<long, true>::*)(
                   struct std::__atomic_base<long, true> const&)) &
              ::std::__atomic_base<long, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIlLb1EEC1Ev(
    struct std::__atomic_base<long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIlLb1EEC1El(
    struct std::__atomic_base<long, true>* __this, long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long, true>) ==
              8);
static_assert(alignof(struct std::__atomic_base<unsigned long, true>) == 8);

extern "C" struct std::__atomic_base<unsigned long, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseImLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned long, true>* __this,
    struct std::__atomic_base<unsigned long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned long, true> &
               (::std::__atomic_base<unsigned long, true>::*)(
                   struct std::__atomic_base<unsigned long, true>&&)) &
              ::std::__atomic_base<unsigned long, true>::operator=);

extern "C" struct std::__atomic_base<unsigned long, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseImLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned long, true>* __this,
    struct std::__atomic_base<unsigned long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned long, true> &
               (::std::__atomic_base<unsigned long, true>::*)(
                   struct std::__atomic_base<unsigned long, true> const&)) &
              ::std::__atomic_base<unsigned long, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseImLb1EEC1Ev(
    struct std::__atomic_base<unsigned long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseImLb1EEC1Em(
    struct std::__atomic_base<unsigned long, true>* __this, unsigned long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<short, true>) == 2);
static_assert(alignof(struct std::__atomic_base<short, true>) == 2);

extern "C" struct std::__atomic_base<short, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIsLb1EEaSEOS1_(
    struct std::__atomic_base<short, true>* __this,
    struct std::__atomic_base<short, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<short, true> &
               (::std::__atomic_base<short, true>::*)(
                   struct std::__atomic_base<short, true>&&)) &
              ::std::__atomic_base<short, true>::operator=);

extern "C" struct std::__atomic_base<short, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIsLb1EEaSERKS1_(
    struct std::__atomic_base<short, true>* __this,
    struct std::__atomic_base<short, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<short, true> &
               (::std::__atomic_base<short, true>::*)(
                   struct std::__atomic_base<short, true> const&)) &
              ::std::__atomic_base<short, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIsLb1EEC1Ev(
    struct std::__atomic_base<short, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIsLb1EEC1Es(
    struct std::__atomic_base<short, true>* __this, short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned short, true>) ==
              2);
static_assert(alignof(struct std::__atomic_base<unsigned short, true>) == 2);

extern "C" struct std::__atomic_base<unsigned short, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseItLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned short, true>* __this,
    struct std::__atomic_base<unsigned short, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned short, true> &
               (::std::__atomic_base<unsigned short, true>::*)(
                   struct std::__atomic_base<unsigned short, true>&&)) &
              ::std::__atomic_base<unsigned short, true>::operator=);

extern "C" struct std::__atomic_base<unsigned short, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseItLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned short, true>* __this,
    struct std::__atomic_base<unsigned short, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned short, true> &
               (::std::__atomic_base<unsigned short, true>::*)(
                   struct std::__atomic_base<unsigned short, true> const&)) &
              ::std::__atomic_base<unsigned short, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseItLb1EEC1Ev(
    struct std::__atomic_base<unsigned short, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseItLb1EEC1Et(
    struct std::__atomic_base<unsigned short, true>* __this,
    unsigned short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<wchar_t, true>) == 4);
static_assert(alignof(struct std::__atomic_base<wchar_t, true>) == 4);

extern "C" struct std::__atomic_base<wchar_t, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIwLb1EEaSEOS1_(
    struct std::__atomic_base<wchar_t, true>* __this,
    struct std::__atomic_base<wchar_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<wchar_t, true> &
               (::std::__atomic_base<wchar_t, true>::*)(
                   struct std::__atomic_base<wchar_t, true>&&)) &
              ::std::__atomic_base<wchar_t, true>::operator=);

extern "C" struct std::__atomic_base<wchar_t, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIwLb1EEaSERKS1_(
    struct std::__atomic_base<wchar_t, true>* __this,
    struct std::__atomic_base<wchar_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<wchar_t, true> &
               (::std::__atomic_base<wchar_t, true>::*)(
                   struct std::__atomic_base<wchar_t, true> const&)) &
              ::std::__atomic_base<wchar_t, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIwLb1EEC1Ev(
    struct std::__atomic_base<wchar_t, true>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long long, true>) == 8);
static_assert(alignof(struct std::__atomic_base<long long, true>) == 8);

extern "C" struct std::__atomic_base<long long, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIxLb1EEaSEOS1_(
    struct std::__atomic_base<long long, true>* __this,
    struct std::__atomic_base<long long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<long long, true> &
               (::std::__atomic_base<long long, true>::*)(
                   struct std::__atomic_base<long long, true>&&)) &
              ::std::__atomic_base<long long, true>::operator=);

extern "C" struct std::__atomic_base<long long, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIxLb1EEaSERKS1_(
    struct std::__atomic_base<long long, true>* __this,
    struct std::__atomic_base<long long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long long, true> &
               (::std::__atomic_base<long long, true>::*)(
                   struct std::__atomic_base<long long, true> const&)) &
              ::std::__atomic_base<long long, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIxLb1EEC1Ev(
    struct std::__atomic_base<long long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIxLb1EEC1Ex(
    struct std::__atomic_base<long long, true>* __this, long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long long, true>) == 8);
static_assert(alignof(struct std::__atomic_base<unsigned long long, true>) ==
              8);

extern "C" struct std::__atomic_base<unsigned long long, true>*
__rust_thunk__5eb817c9__ZNSt3__u13__atomic_baseIyLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned long long, true>* __this,
    struct std::__atomic_base<unsigned long long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned long long, true> &
               (::std::__atomic_base<unsigned long long, true>::*)(
                   struct std::__atomic_base<unsigned long long, true>&&)) &
              ::std::__atomic_base<unsigned long long, true>::operator=);

extern "C" struct std::__atomic_base<unsigned long long, true>*
__rust_thunk__90e56429__ZNSt3__u13__atomic_baseIyLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned long long, true>* __this,
    struct std::__atomic_base<unsigned long long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<unsigned long long, true> &
     (::std::__atomic_base<unsigned long long, true>::*)(
         struct std::__atomic_base<unsigned long long, true> const&)) &
    ::std::__atomic_base<unsigned long long, true>::operator=);

extern "C" void __rust_thunk__7dd02c93__ZNSt3__u13__atomic_baseIyLb1EEC1Ev(
    struct std::__atomic_base<unsigned long long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__f6c4bfad__ZNSt3__u13__atomic_baseIyLb1EEC1Ey(
    struct std::__atomic_base<unsigned long long, true>* __this,
    unsigned long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::atomic<char8_t>) == 1);
static_assert(alignof(struct std::atomic<char8_t>) == 1);

extern "C" void
__rust_thunk__89ad414c__ZNSt3__u6atomicIDuEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<char8_t>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        struct std::atomic<absl::base_internal::PerThreadSynch::State>) == 4);
static_assert(
    alignof(struct std::atomic<absl::base_internal::PerThreadSynch::State>) ==
    4);

extern "C" void
__rust_thunk__89ad414c__ZNSt3__u6atomicIN4absl13base_internal14PerThreadSynch5StateEEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<absl::base_internal::PerThreadSynch::State>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(
        struct std::atomic<absl::base_internal::ThreadIdentity::WaitState>) ==
    1);
static_assert(
    alignof(
        struct std::atomic<absl::base_internal::ThreadIdentity::WaitState>) ==
    1);

extern "C" void
__rust_thunk__89ad414c__ZNSt3__u6atomicIN4absl13base_internal14ThreadIdentity9WaitStateEEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<absl::base_internal::ThreadIdentity::WaitState>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(struct std::atomic<absl::flags_internal::MaskedPointer>) ==
    8);
static_assert(
    alignof(struct std::atomic<absl::flags_internal::MaskedPointer>) == 8);

extern "C" void
__rust_thunk__89ad414c__ZNSt3__u6atomicIN4absl14flags_internal13MaskedPointerEEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<absl::flags_internal::MaskedPointer>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::atomic<long>) == 8);
static_assert(alignof(struct std::atomic<long>) == 8);

extern "C" void
__rust_thunk__89ad414c__ZNSt3__u6atomicIlEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<long>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__183e645a__ZNSt3__u6atomicIlEC1El(
    struct std::atomic<long>* __this, long __d) {
  crubit::construct_at(__this, __d);
}

extern "C" long __rust_thunk__47d9800a__ZNSt3__u6atomicIlEaSEl(
    struct std::atomic<long>* __this, long __d) {
  return __this->operator=(__d);
}

static_assert((long (::std::atomic<long>::*)(long)) &
              ::std::atomic<long>::operator=);

static_assert(CRUBIT_SIZEOF(struct std::atomic<unsigned long>) == 8);
static_assert(alignof(struct std::atomic<unsigned long>) == 8);

extern "C" void
__rust_thunk__89ad414c__ZNSt3__u6atomicImEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<unsigned long>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__183e645a__ZNSt3__u6atomicImEC1Em(
    struct std::atomic<unsigned long>* __this, unsigned long __d) {
  crubit::construct_at(__this, __d);
}

extern "C" unsigned long __rust_thunk__47d9800a__ZNSt3__u6atomicImEaSEm(
    struct std::atomic<unsigned long>* __this, unsigned long __d) {
  return __this->operator=(__d);
}

static_assert((unsigned long (::std::atomic<unsigned long>::*)(unsigned long)) &
              ::std::atomic<unsigned long>::operator=);

static_assert(CRUBIT_SIZEOF(struct std::atomic<wchar_t>) == 4);
static_assert(alignof(struct std::atomic<wchar_t>) == 4);

extern "C" void
__rust_thunk__89ad414c__ZNSt3__u6atomicIwEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<wchar_t>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(struct std::atomic<const std::basic_string<
                      char, std::char_traits<char>, std::allocator<char>>*>) ==
    8);
static_assert(
    alignof(struct std::atomic<const std::basic_string<
                char, std::char_traits<char>, std::allocator<char>>*>) == 8);

extern "C" void
__rust_thunk__91b712d2__ZNSt3__u6atomicIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEC1Ev(
    struct std::atomic<const std::basic_string<
        char, std::char_traits<char>, std::allocator<char>>*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__2bc9e05f__ZNSt3__u6atomicIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEC1ES8_(
    struct std::atomic<const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>*>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const* __d) {
  crubit::construct_at(__this, __d);
}

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::allocator<char>> const*
__rust_thunk__c77a2c17__ZNSt3__u6atomicIPKNS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEEaSES8_(
    struct std::atomic<const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>*>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const* __d) {
  return __this->operator=(__d);
}

static_assert(
    (class std::basic_string<
        char, std::char_traits<char>,
        std::allocator<char>> const* (::std::
                                          atomic<const std::basic_string<
                                              char, std::char_traits<char>,
                                              std::allocator<char>>*>::*)(
        class std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>> const*)) &
    ::std::atomic<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>*>::operator=);

static_assert(
    CRUBIT_SIZEOF(class std::basic_ios<char, std::char_traits<char>>) == 152);
static_assert(alignof(class std::basic_ios<char, std::char_traits<char>>) == 8);

extern "C" bool
__rust_thunk__7fcb6ff7__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEEntEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->operator!();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::operator!);

extern "C" unsigned int
__rust_thunk__4ad05a9d__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE7rdstateEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->rdstate();
}

static_assert(
    (unsigned int (::std::basic_ios<char, std::char_traits<char>>::*)() const) &
    ::std::basic_ios<char, std::char_traits<char>>::rdstate);

extern "C" void
__rust_thunk__c8027e9e__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEE5clearEj(
    class std::basic_ios<char, std::char_traits<char>>* __this,
    unsigned int __state) {
  __this->clear(__state);
}

static_assert(
    (void (::std::basic_ios<char, std::char_traits<char>>::*)(unsigned int)) &
    ::std::basic_ios<char, std::char_traits<char>>::clear);

extern "C" void
__rust_thunk__5481019e__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEE8setstateEj(
    class std::basic_ios<char, std::char_traits<char>>* __this,
    unsigned int __state) {
  __this->setstate(__state);
}

static_assert(
    (void (::std::basic_ios<char, std::char_traits<char>>::*)(unsigned int)) &
    ::std::basic_ios<char, std::char_traits<char>>::setstate);

extern "C" bool
__rust_thunk__341dfb3e__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE4goodEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->good();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::good);

extern "C" bool
__rust_thunk__1b58fe95__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE3eofEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->eof();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::eof);

extern "C" bool
__rust_thunk__73e66355__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE4failEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->fail();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::fail);

extern "C" bool
__rust_thunk__1a2a8798__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE3badEv(
    class std::basic_ios<char, std::char_traits<char>> const* __this) {
  return __this->bad();
}

static_assert((bool (::std::basic_ios<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::bad);

extern "C" void
__rust_thunk__db50351d__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEED1Ev(
    class std::basic_ios<char, std::char_traits<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_ios<char, std::char_traits<char>>*
__rust_thunk__82316d38__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEE7copyfmtERKS3_(
    class std::basic_ios<char, std::char_traits<char>>* __this,
    class std::basic_ios<char, std::char_traits<char>> const* __rhs) {
  return std::addressof(__this->copyfmt(*__rhs));
}

static_assert((class std::basic_ios<char, std::char_traits<char>> &
               (::std::basic_ios<char, std::char_traits<char>>::*)(
                   class std::basic_ios<char, std::char_traits<char>> const&)) &
              ::std::basic_ios<char, std::char_traits<char>>::copyfmt);

extern "C" void
__rust_thunk__360ef3bd__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEEC1EPNS_15basic_streambufIcS2_EE(
    class std::basic_ios<char, std::char_traits<char>>* __this,
    class std::basic_streambuf<char, std::char_traits<char>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__a7d7fda5__ZNSt3__u9basic_iosIcNS_11char_traitsIcEEE5imbueERKNS_6localeE(
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
__rust_thunk__1b1ae90f__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE6narrowEcc(
    class std::basic_ios<char, std::char_traits<char>> const* __this, char __c,
    char __dfault) {
  return __this->narrow(__c, __dfault);
}

static_assert((char (::std::basic_ios<char, std::char_traits<char>>::*)(char,
                                                                        char)
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::narrow);

extern "C" char
__rust_thunk__ff9e1e0f__ZNKSt3__u9basic_iosIcNS_11char_traitsIcEEE5widenEc(
    class std::basic_ios<char, std::char_traits<char>> const* __this,
    char __c) {
  return __this->widen(__c);
}

static_assert((char (::std::basic_ios<char, std::char_traits<char>>::*)(char)
                   const) &
              ::std::basic_ios<char, std::char_traits<char>>::widen);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u9basic_iosIcNS_11char_traitsIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_ios<char, std::char_traits<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_ios<wchar_t, std::char_traits<wchar_t>>) ==
    152);
static_assert(
    alignof(class std::basic_ios<wchar_t, std::char_traits<wchar_t>>) == 8);

extern "C" bool
__rust_thunk__7fcb6ff7__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEEntEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->operator!();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::operator!);

extern "C" unsigned int
__rust_thunk__4ad05a9d__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE7rdstateEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->rdstate();
}

static_assert(
    (unsigned int (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
         const) &
    ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::rdstate);

extern "C" void
__rust_thunk__c8027e9e__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEE5clearEj(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned int __state) {
  __this->clear(__state);
}

static_assert((void (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)(
                  unsigned int)) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::clear);

extern "C" void
__rust_thunk__5481019e__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEE8setstateEj(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned int __state) {
  __this->setstate(__state);
}

static_assert((void (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)(
                  unsigned int)) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::setstate);

extern "C" bool
__rust_thunk__341dfb3e__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE4goodEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->good();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::good);

extern "C" bool
__rust_thunk__1b58fe95__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE3eofEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->eof();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::eof);

extern "C" bool
__rust_thunk__73e66355__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE4failEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->fail();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::fail);

extern "C" bool
__rust_thunk__1a2a8798__ZNKSt3__u9basic_iosIwNS_11char_traitsIwEEE3badEv(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>> const* __this) {
  return __this->bad();
}

static_assert((bool (::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::*)()
                   const) &
              ::std::basic_ios<wchar_t, std::char_traits<wchar_t>>::bad);

extern "C" void
__rust_thunk__db50351d__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEED1Ev(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_ios<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__82316d38__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEE7copyfmtERKS3_(
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
__rust_thunk__360ef3bd__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEEC1EPNS_15basic_streambufIwS2_EE(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__a7d7fda5__ZNSt3__u9basic_iosIwNS_11char_traitsIwEEE5imbueERKNS_6localeE(
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
__crubit_operator_delete____CcTemplateInstNSt3__u9basic_iosIwNS_11char_traitsIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_ios<wchar_t, std::char_traits<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(sizeof(struct std::less<void>) == 1);
static_assert(alignof(struct std::less<void>) == 1);

extern "C" void __rust_thunk__49d12de0__ZNSt3__u4lessIvEC1Ev(
    struct std::less<void>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_streambuf<char, std::char_traits<char>>) ==
    64);
static_assert(
    alignof(class std::basic_streambuf<char, std::char_traits<char>>) == 8);

extern "C" void
__rust_thunk__da1f4ced__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEED1Ev(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__5139c4f7__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE8pubimbueERKNS_6localeE(
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
__rust_thunk__523e4952__ZNKSt3__u15basic_streambufIcNS_11char_traitsIcEEE6getlocEv(
    class ::std::__u::locale* __return,
    class std::basic_streambuf<char, std::char_traits<char>> const* __this) {
  new (__return) auto(__this->getloc());
}

static_assert((class ::std::__u::locale (
                  ::std::basic_streambuf<char, std::char_traits<char>>::*)()
                   const) &
              ::std::basic_streambuf<char, std::char_traits<char>>::getloc);

extern "C" class std::basic_streambuf<char, std::char_traits<char>>*
__rust_thunk__7bad4052__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE9pubsetbufEPcl(
    class std::basic_streambuf<char, std::char_traits<char>>* __this, char* __s,
    ptrdiff_t __n) {
  return __this->pubsetbuf(__s, __n);
}

static_assert((class std::basic_streambuf<char, std::char_traits<char>> *
               (::std::basic_streambuf<char, std::char_traits<char>>::*)(
                   char*, ptrdiff_t)) &
              ::std::basic_streambuf<char, std::char_traits<char>>::pubsetbuf);

extern "C" void
__rust_thunk__539e86af__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE10pubseekoffExNS_8ios_base7seekdirEj(
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
__rust_thunk__e6188b0b__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE10pubseekposENS_4fposI11__mbstate_tEEj(
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
__rust_thunk__b67961a2__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE7pubsyncEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->pubsync();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::pubsync);

extern "C" ptrdiff_t
__rust_thunk__12d745df__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE8in_availEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->in_avail();
}

static_assert(
    (ptrdiff_t (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::in_avail);

extern "C" int
__rust_thunk__0a228bdb__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE6snextcEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->snextc();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::snextc);

extern "C" int
__rust_thunk__d6c055be__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE6sbumpcEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->sbumpc();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sbumpc);

extern "C" int
__rust_thunk__ff02a1cf__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE5sgetcEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->sgetc();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sgetc);

extern "C" ptrdiff_t
__rust_thunk__b941cb57__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE5sgetnEPcl(
    class std::basic_streambuf<char, std::char_traits<char>>* __this, char* __s,
    ptrdiff_t __n) {
  return __this->sgetn(__s, __n);
}

static_assert(
    (ptrdiff_t (::std::basic_streambuf<char, std::char_traits<char>>::*)(
        char*, ptrdiff_t)) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sgetn);

extern "C" int
__rust_thunk__dae93f89__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE9sputbackcEc(
    class std::basic_streambuf<char, std::char_traits<char>>* __this,
    char __c) {
  return __this->sputbackc(__c);
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)(char)) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sputbackc);

extern "C" int
__rust_thunk__759cf391__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE7sungetcEv(
    class std::basic_streambuf<char, std::char_traits<char>>* __this) {
  return __this->sungetc();
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)()) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sungetc);

extern "C" int
__rust_thunk__d8d96a58__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE5sputcEc(
    class std::basic_streambuf<char, std::char_traits<char>>* __this,
    char __c) {
  return __this->sputc(__c);
}

static_assert(
    (int (::std::basic_streambuf<char, std::char_traits<char>>::*)(char)) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sputc);

extern "C" ptrdiff_t
__rust_thunk__445532a7__ZNSt3__u15basic_streambufIcNS_11char_traitsIcEEE5sputnEPKcl(
    class std::basic_streambuf<char, std::char_traits<char>>* __this,
    char const* __s, ptrdiff_t __n) {
  return __this->sputn(__s, __n);
}

static_assert(
    (ptrdiff_t (::std::basic_streambuf<char, std::char_traits<char>>::*)(
        char const*, ptrdiff_t)) &
    ::std::basic_streambuf<char, std::char_traits<char>>::sputn);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u15basic_streambufIcNS_11char_traitsIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
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
__rust_thunk__da1f4ced__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEED1Ev(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__5139c4f7__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE8pubimbueERKNS_6localeE(
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
__rust_thunk__523e4952__ZNKSt3__u15basic_streambufIwNS_11char_traitsIwEEE6getlocEv(
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
__rust_thunk__539e86af__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE10pubseekoffExNS_8ios_base7seekdirEj(
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
__rust_thunk__e6188b0b__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE10pubseekposENS_4fposI11__mbstate_tEEj(
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
__rust_thunk__b67961a2__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE7pubsyncEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->pubsync();
}

static_assert(
    (int (::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::pubsync);

extern "C" ptrdiff_t
__rust_thunk__12d745df__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE8in_availEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->in_avail();
}

static_assert(
    (ptrdiff_t (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::in_avail);

extern "C" unsigned int
__rust_thunk__0a228bdb__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE6snextcEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->snextc();
}

static_assert(
    (unsigned int (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::snextc);

extern "C" unsigned int
__rust_thunk__d6c055be__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE6sbumpcEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->sbumpc();
}

static_assert(
    (unsigned int (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::sbumpc);

extern "C" unsigned int
__rust_thunk__ff02a1cf__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE5sgetcEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->sgetc();
}

static_assert(
    (unsigned int (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::sgetc);

extern "C" unsigned int
__rust_thunk__759cf391__ZNSt3__u15basic_streambufIwNS_11char_traitsIwEEE7sungetcEv(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->sungetc();
}

static_assert(
    (unsigned int (
        ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>::sungetc);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u15basic_streambufIwNS_11char_traitsIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_ostream<char, std::char_traits<char>>) ==
    160);
static_assert(alignof(class std::basic_ostream<char, std::char_traits<char>>) ==
              8);

extern "C" void
__rust_thunk__24e510b8__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEC1EPNS_15basic_streambufIcS2_EE(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    class std::basic_streambuf<char, std::char_traits<char>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__ec8b2c71__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEED1Ev(
    class std::basic_ostream<char, std::char_traits<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__b173275d__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPFRS3_S4_E(
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
__rust_thunk__c1da4757__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPFRNS_9basic_iosIcS2_EES6_E(
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
__rust_thunk__64fcdad9__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPFRNS_8ios_baseES5_E(
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
__rust_thunk__d0664ba3__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEb(
    class std::basic_ostream<char, std::char_traits<char>>* __this, bool __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(bool)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__3e019978__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEs(
    class std::basic_ostream<char, std::char_traits<char>>* __this, short __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(short)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__35430b22__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEt(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    unsigned short __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(unsigned short)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__e555b1e4__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEi(
    class std::basic_ostream<char, std::char_traits<char>>* __this, int __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(int)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__aba0b457__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEj(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    unsigned int __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(unsigned int)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__507824e4__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEl(
    class std::basic_ostream<char, std::char_traits<char>>* __this, long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(long)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__640d0342__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEm(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    unsigned long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(unsigned long)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__aa197065__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEx(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    long long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(long long)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__8c07ff7a__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEy(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    unsigned long long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(
                   unsigned long long)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__09e45f0f__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEf(
    class std::basic_ostream<char, std::char_traits<char>>* __this, float __f) {
  return std::addressof(__this->operator<<(__f));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(float)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__01c0297d__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEd(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    double __f) {
  return std::addressof(__this->operator<<(__f));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(double)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__00b390c5__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPKv(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    void const* __p) {
  return std::addressof(__this->operator<<(__p));
}

static_assert(
    (class std::basic_ostream<char, std::char_traits<char>> &
     (::std::basic_ostream<char, std::char_traits<char>>::*)(void const*)) &
    ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__2af0431d__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEElsEPNS_15basic_streambufIcS2_EE(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    class std::basic_streambuf<char, std::char_traits<char>>* __sb) {
  return std::addressof(__this->operator<<(__sb));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(
                   class std::basic_streambuf<char, std::char_traits<char>>*)) &
              ::std::basic_ostream<char, std::char_traits<char>>::operator<<);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__ce431882__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEE3putEc(
    class std::basic_ostream<char, std::char_traits<char>>* __this, char __c) {
  return std::addressof(__this->put(__c));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(char)) &
              ::std::basic_ostream<char, std::char_traits<char>>::put);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__974b1293__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEE5writeEPKcl(
    class std::basic_ostream<char, std::char_traits<char>>* __this,
    char const* __s, ptrdiff_t __n) {
  return std::addressof(__this->write(__s, __n));
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)(
                   char const*, ptrdiff_t)) &
              ::std::basic_ostream<char, std::char_traits<char>>::write);

extern "C" class std::basic_ostream<char, std::char_traits<char>>*
__rust_thunk__df64ec1f__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEE5flushEv(
    class std::basic_ostream<char, std::char_traits<char>>* __this) {
  return std::addressof(__this->flush());
}

static_assert((class std::basic_ostream<char, std::char_traits<char>> &
               (::std::basic_ostream<char, std::char_traits<char>>::*)()) &
              ::std::basic_ostream<char, std::char_traits<char>>::flush);

extern "C" void
__rust_thunk__a60c6fa9__ZNSt3__u13basic_ostreamIcNS_11char_traitsIcEEE5tellpEv(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_ostream<char, std::char_traits<char>>* __this) {
  new (__return) auto(__this->tellp());
}

static_assert((class std::fpos<__mbstate_t> (
                  ::std::basic_ostream<char, std::char_traits<char>>::*)()) &
              ::std::basic_ostream<char, std::char_traits<char>>::tellp);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_ostream<char, std::char_traits<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>) == 160);
static_assert(
    alignof(class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>) == 8);

extern "C" void
__rust_thunk__24e510b8__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEEC1EPNS_15basic_streambufIwS2_EE(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__ec8b2c71__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEED1Ev(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__b173275d__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPFRS3_S4_E(
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
__rust_thunk__c1da4757__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPFRNS_9basic_iosIwS2_EES6_E(
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
__rust_thunk__64fcdad9__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPFRNS_8ios_baseES5_E(
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
__rust_thunk__d0664ba3__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEb(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    bool __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(bool)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__3e019978__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEs(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    short __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(short)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__35430b22__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEt(
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
__rust_thunk__e555b1e4__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEi(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    int __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(int)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__aba0b457__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEj(
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
__rust_thunk__507824e4__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEl(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(long)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__640d0342__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEm(
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
__rust_thunk__aa197065__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEx(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    long long __n) {
  return std::addressof(__this->operator<<(__n));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(long long)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__8c07ff7a__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEy(
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
__rust_thunk__09e45f0f__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEf(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    float __f) {
  return std::addressof(__this->operator<<(__f));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(float)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__01c0297d__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEd(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this,
    double __f) {
  return std::addressof(__this->operator<<(__f));
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)(double)) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::operator<<);

extern "C" class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__00b390c5__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPKv(
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
__rust_thunk__2af0431d__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEElsEPNS_15basic_streambufIwS2_EE(
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
__rust_thunk__df64ec1f__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEE5flushEv(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this) {
  return std::addressof(__this->flush());
}

static_assert(
    (class std::basic_ostream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::flush);

extern "C" void
__rust_thunk__a60c6fa9__ZNSt3__u13basic_ostreamIwNS_11char_traitsIwEEE5tellpEv(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* __this) {
  new (__return) auto(__this->tellp());
}

static_assert(
    (class std::fpos<__mbstate_t> (
        ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_ostream<wchar_t, std::char_traits<wchar_t>>::tellp);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u13basic_ostreamIwNS_11char_traitsIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_ostream<wchar_t, std::char_traits<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_istream<char, std::char_traits<char>>) ==
    168);
static_assert(alignof(class std::basic_istream<char, std::char_traits<char>>) ==
              8);

extern "C" void
__rust_thunk__156a6547__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEEC1EPNS_15basic_streambufIcS2_EE(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    class std::basic_streambuf<char, std::char_traits<char>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__b10e28dd__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEED1Ev(
    class std::basic_istream<char, std::char_traits<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__31ec4fc1__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsEPFRS3_S4_E(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    crubit::type_identity_t<
        class std::basic_istream<char, std::char_traits<char>>&(
            class std::basic_istream<char, std::char_traits<char>>&)>* __pf) {
  return std::addressof(__this->operator>>(__pf));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(
         crubit::type_identity_t<
             class std::basic_istream<char, std::char_traits<char>>&(
                 class std::basic_istream<char, std::char_traits<char>>&)>*)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__0b94a289__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsEPFRNS_9basic_iosIcS2_EES6_E(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    crubit::type_identity_t<class std::basic_ios<char, std::char_traits<char>>&(
        class std::basic_ios<char, std::char_traits<char>>&)>* __pf) {
  return std::addressof(__this->operator>>(__pf));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(
         crubit::type_identity_t<
             class std::basic_ios<char, std::char_traits<char>>&(
                 class std::basic_ios<char, std::char_traits<char>>&)>*)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__a0862367__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsEPFRNS_8ios_baseES5_E(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    crubit::type_identity_t<
        class ::std::__u::ios_base&(class ::std::__u::ios_base&)>* __pf) {
  return std::addressof(__this->operator>>(__pf));
}

static_assert((class std::basic_istream<char, std::char_traits<char>> &
               (::std::basic_istream<char, std::char_traits<char>>::*)(
                   crubit::type_identity_t<class ::std::__u::ios_base&(
                       class ::std::__u::ios_base&)>*)) &
              ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__b74acc6a__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsEPNS_15basic_streambufIcS2_EE(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    class std::basic_streambuf<char, std::char_traits<char>>* __sb) {
  return std::addressof(__this->operator>>(__sb));
}

static_assert((class std::basic_istream<char, std::char_traits<char>> &
               (::std::basic_istream<char, std::char_traits<char>>::*)(
                   class std::basic_streambuf<char, std::char_traits<char>>*)) &
              ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__261f6ae6__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERb(
    class std::basic_istream<char, std::char_traits<char>>* __this, bool* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert((class std::basic_istream<char, std::char_traits<char>> &
               (::std::basic_istream<char, std::char_traits<char>>::*)(bool&)) &
              ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__0c02f92a__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERs(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    short* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(short&)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__17f2a69b__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERt(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    unsigned short* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(unsigned short&)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__0a73ff1c__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERi(
    class std::basic_istream<char, std::char_traits<char>>* __this, int* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert((class std::basic_istream<char, std::char_traits<char>> &
               (::std::basic_istream<char, std::char_traits<char>>::*)(int&)) &
              ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__eb54069a__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERj(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    unsigned int* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(unsigned int&)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__342c26b4__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERl(
    class std::basic_istream<char, std::char_traits<char>>* __this, long* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert((class std::basic_istream<char, std::char_traits<char>> &
               (::std::basic_istream<char, std::char_traits<char>>::*)(long&)) &
              ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__4c4f1a02__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERm(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    unsigned long* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(unsigned long&)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__09a724ac__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERx(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    long long* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(long long&)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__1bacc017__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERy(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    unsigned long long* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert((class std::basic_istream<char, std::char_traits<char>> &
               (::std::basic_istream<char, std::char_traits<char>>::*)(
                   unsigned long long&)) &
              ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__c99e2491__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERf(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    float* __f) {
  return std::addressof(__this->operator>>(*__f));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(float&)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__9dcbd738__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERd(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    double* __f) {
  return std::addressof(__this->operator>>(*__f));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(double&)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__653efca7__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEErsERPv(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    void** __p) {
  return std::addressof(__this->operator>>(*__p));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(void*&)) &
    ::std::basic_istream<char, std::char_traits<char>>::operator>>);

extern "C" ptrdiff_t
__rust_thunk__628fc601__ZNKSt3__u13basic_istreamIcNS_11char_traitsIcEEE6gcountEv(
    class std::basic_istream<char, std::char_traits<char>> const* __this) {
  return __this->gcount();
}

static_assert(
    (ptrdiff_t (::std::basic_istream<char, std::char_traits<char>>::*)()
         const) &
    ::std::basic_istream<char, std::char_traits<char>>::gcount);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__8a6e9ccb__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEE6ignoreEli(
    class std::basic_istream<char, std::char_traits<char>>* __this,
    ptrdiff_t __n, int __dlm) {
  return std::addressof(__this->ignore(__n, __dlm));
}

static_assert(
    (class std::basic_istream<char, std::char_traits<char>> &
     (::std::basic_istream<char, std::char_traits<char>>::*)(ptrdiff_t, int)) &
    ::std::basic_istream<char, std::char_traits<char>>::ignore);

extern "C" int
__rust_thunk__f238704d__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEE4peekEv(
    class std::basic_istream<char, std::char_traits<char>>* __this) {
  return __this->peek();
}

static_assert((int (::std::basic_istream<char, std::char_traits<char>>::*)()) &
              ::std::basic_istream<char, std::char_traits<char>>::peek);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__cd9d8c7a__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEE4readEPcl(
    class std::basic_istream<char, std::char_traits<char>>* __this, char* __s,
    ptrdiff_t __n) {
  return std::addressof(__this->read(__s, __n));
}

static_assert((class std::basic_istream<char, std::char_traits<char>> &
               (::std::basic_istream<char, std::char_traits<char>>::*)(
                   char*, ptrdiff_t)) &
              ::std::basic_istream<char, std::char_traits<char>>::read);

extern "C" ptrdiff_t
__rust_thunk__729b57c0__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEE8readsomeEPcl(
    class std::basic_istream<char, std::char_traits<char>>* __this, char* __s,
    ptrdiff_t __n) {
  return __this->readsome(__s, __n);
}

static_assert(
    (ptrdiff_t (::std::basic_istream<char, std::char_traits<char>>::*)(
        char*, ptrdiff_t)) &
    ::std::basic_istream<char, std::char_traits<char>>::readsome);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__37b5b698__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEE7putbackEc(
    class std::basic_istream<char, std::char_traits<char>>* __this, char __c) {
  return std::addressof(__this->putback(__c));
}

static_assert((class std::basic_istream<char, std::char_traits<char>> &
               (::std::basic_istream<char, std::char_traits<char>>::*)(char)) &
              ::std::basic_istream<char, std::char_traits<char>>::putback);

extern "C" class std::basic_istream<char, std::char_traits<char>>*
__rust_thunk__2ac3a89d__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEE5ungetEv(
    class std::basic_istream<char, std::char_traits<char>>* __this) {
  return std::addressof(__this->unget());
}

static_assert((class std::basic_istream<char, std::char_traits<char>> &
               (::std::basic_istream<char, std::char_traits<char>>::*)()) &
              ::std::basic_istream<char, std::char_traits<char>>::unget);

extern "C" int
__rust_thunk__785cfc5e__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEE4syncEv(
    class std::basic_istream<char, std::char_traits<char>>* __this) {
  return __this->sync();
}

static_assert((int (::std::basic_istream<char, std::char_traits<char>>::*)()) &
              ::std::basic_istream<char, std::char_traits<char>>::sync);

extern "C" void
__rust_thunk__7392644d__ZNSt3__u13basic_istreamIcNS_11char_traitsIcEEE5tellgEv(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_istream<char, std::char_traits<char>>* __this) {
  new (__return) auto(__this->tellg());
}

static_assert((class std::fpos<__mbstate_t> (
                  ::std::basic_istream<char, std::char_traits<char>>::*)()) &
              ::std::basic_istream<char, std::char_traits<char>>::tellg);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u13basic_istreamIcNS_11char_traitsIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_istream<char, std::char_traits<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_istream<wchar_t, std::char_traits<wchar_t>>) == 168);
static_assert(
    alignof(class std::basic_istream<wchar_t, std::char_traits<wchar_t>>) == 8);

extern "C" void
__rust_thunk__156a6547__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEEC1EPNS_15basic_streambufIwS2_EE(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__b10e28dd__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEED1Ev(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__31ec4fc1__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsEPFRS3_S4_E(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    crubit::type_identity_t<
        class std::basic_istream<wchar_t, std::char_traits<wchar_t>>&(
            class std::basic_istream<wchar_t, std::char_traits<wchar_t>>&)>*
        __pf) {
  return std::addressof(__this->operator>>(__pf));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
         crubit::type_identity_t<
             class std::basic_istream<wchar_t, std::char_traits<wchar_t>>&(
                 class std::basic_istream<wchar_t,
                                          std::char_traits<wchar_t>>&)>*)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__0b94a289__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsEPFRNS_9basic_iosIwS2_EES6_E(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    crubit::type_identity_t<
        class std::basic_ios<wchar_t, std::char_traits<wchar_t>>&(
            class std::basic_ios<wchar_t, std::char_traits<wchar_t>>&)>* __pf) {
  return std::addressof(__this->operator>>(__pf));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
         crubit::type_identity_t<class std::basic_ios<
             wchar_t, std::char_traits<wchar_t>>&(
             class std::basic_ios<wchar_t, std::char_traits<wchar_t>>&)>*)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__a0862367__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsEPFRNS_8ios_baseES5_E(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    crubit::type_identity_t<
        class ::std::__u::ios_base&(class ::std::__u::ios_base&)>* __pf) {
  return std::addressof(__this->operator>>(__pf));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
         crubit::type_identity_t<
             class ::std::__u::ios_base&(class ::std::__u::ios_base&)>*)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__b74acc6a__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsEPNS_15basic_streambufIwS2_EE(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>* __sb) {
  return std::addressof(__this->operator>>(__sb));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
         class std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>*)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__261f6ae6__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERb(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    bool* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(bool&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__0c02f92a__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERs(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    short* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(short&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__17f2a69b__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERt(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned short* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
         unsigned short&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__0a73ff1c__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERi(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    int* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(int&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__eb54069a__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERj(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned int* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
         unsigned int&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__342c26b4__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERl(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    long* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(long&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__4c4f1a02__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERm(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned long* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
         unsigned long&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__09a724ac__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERx(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    long long* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
         long long&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__1bacc017__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERy(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    unsigned long long* __n) {
  return std::addressof(__this->operator>>(*__n));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
         unsigned long long&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__c99e2491__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERf(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    float* __f) {
  return std::addressof(__this->operator>>(*__f));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(float&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__9dcbd738__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERd(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    double* __f) {
  return std::addressof(__this->operator>>(*__f));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(double&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__653efca7__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEErsERPv(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    void** __p) {
  return std::addressof(__this->operator>>(*__p));
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(void*&)) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::operator>>);

extern "C" ptrdiff_t
__rust_thunk__628fc601__ZNKSt3__u13basic_istreamIwNS_11char_traitsIwEEE6gcountEv(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>> const*
        __this) {
  return __this->gcount();
}

static_assert(
    (ptrdiff_t (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)()
         const) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::gcount);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__8a6e9ccb__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEE6ignoreElj(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this,
    ptrdiff_t __n, unsigned int __dlm) {
  return std::addressof(__this->ignore(__n, __dlm));
}

static_assert((class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
               (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)(
                   ptrdiff_t, unsigned int)) &
              ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::ignore);

extern "C" unsigned int
__rust_thunk__f238704d__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEE4peekEv(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->peek();
}

static_assert(
    (unsigned int (
        ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::peek);

extern "C" class std::basic_istream<wchar_t, std::char_traits<wchar_t>>*
__rust_thunk__2ac3a89d__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEE5ungetEv(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this) {
  return std::addressof(__this->unget());
}

static_assert(
    (class std::basic_istream<wchar_t, std::char_traits<wchar_t>> &
     (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::unget);

extern "C" int
__rust_thunk__785cfc5e__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEE4syncEv(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this) {
  return __this->sync();
}

static_assert(
    (int (::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::sync);

extern "C" void
__rust_thunk__7392644d__ZNSt3__u13basic_istreamIwNS_11char_traitsIwEEE5tellgEv(
    class std::fpos<__mbstate_t>* __return,
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* __this) {
  new (__return) auto(__this->tellg());
}

static_assert(
    (class std::fpos<__mbstate_t> (
        ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::*)()) &
    ::std::basic_istream<wchar_t, std::char_traits<wchar_t>>::tellg);

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u13basic_istreamIwNS_11char_traitsIwEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_istream<wchar_t, std::char_traits<wchar_t>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_iostream<char, std::char_traits<char>>) ==
    176);
static_assert(
    alignof(class std::basic_iostream<char, std::char_traits<char>>) == 8);

extern "C" void
__rust_thunk__0f78ccf8__ZNSt3__u14basic_iostreamIcNS_11char_traitsIcEEEC1EPNS_15basic_streambufIcS2_EE(
    class std::basic_iostream<char, std::char_traits<char>>* __this,
    class std::basic_streambuf<char, std::char_traits<char>>* __sb) {
  crubit::construct_at(__this, __sb);
}

extern "C" void
__rust_thunk__62bd60d4__ZNSt3__u14basic_iostreamIcNS_11char_traitsIcEEED1Ev(
    class std::basic_iostream<char, std::char_traits<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u14basic_iostreamIcNS_11char_traitsIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_iostream<char, std::char_traits<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_stringbuf<char, std::char_traits<char>,
                                             std::allocator<char>>) == 104);
static_assert(alignof(class std::basic_stringbuf<char, std::char_traits<char>,
                                                 std::allocator<char>>) == 8);

extern "C" void
__rust_thunk__ec6a4c9f__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEC1Ev(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__4c22f3da__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEC1Ej(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this,
    unsigned int __wch) {
  crubit::construct_at(__this, __wch);
}

extern "C" void
__rust_thunk__6ebfa393__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEC1ERKNS_12basic_stringIcS2_S4_EEj(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const* __s,
    unsigned int __wch) {
  crubit::construct_at(__this, *__s, __wch);
}

extern "C" void
__rust_thunk__70f2a896__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEC1ERKS4_(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this,
    class std::allocator<char> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__9adff16b__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EjRKS4_(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this,
    unsigned int __wch, class std::allocator<char> const* __a) {
  crubit::construct_at(__this, __wch, *__a);
}

extern "C" void
__rust_thunk__b30848f0__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EONS_12basic_stringIcS2_S4_EEj(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>, std::allocator<char>>*
        __s,
    unsigned int __wch) {
  crubit::construct_at(__this, std::move(*__s), __wch);
}

extern "C" void
__rust_thunk__727af024__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EOS5_(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this,
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __rhs) {
  crubit::construct_at(__this, std::move(*__rhs));
}

extern "C" void
__rust_thunk__cc22688d__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EOS5_RKS4_(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this,
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __rhs,
    class std::allocator<char> const* __a) {
  crubit::construct_at(__this, std::move(*__rhs), *__a);
}

extern "C" class std::basic_stringbuf<char, std::char_traits<char>,
                                      std::allocator<char>>*
__rust_thunk__88091907__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEaSEOS5_(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this,
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __rhs) {
  return std::addressof(__this->operator=(std::move(*__rhs)));
}

static_assert((class std::basic_stringbuf<char, std::char_traits<char>,
                                          std::allocator<char>> &
               (::std::basic_stringbuf<char, std::char_traits<char>,
                                       std::allocator<char>>::*)(
                   class std::basic_stringbuf<char, std::char_traits<char>,
                                              std::allocator<char>>&&)) &
              ::std::basic_stringbuf<char, std::char_traits<char>,
                                     std::allocator<char>>::operator=);

extern "C" void
__rust_thunk__04b65a89__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEE4swapERS5_(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this,
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __rhs) {
  __this->swap(*__rhs);
}

static_assert((void (::std::basic_stringbuf<char, std::char_traits<char>,
                                            std::allocator<char>>::*)(
                  class std::basic_stringbuf<char, std::char_traits<char>,
                                             std::allocator<char>>&)) &
              ::std::basic_stringbuf<char, std::char_traits<char>,
                                     std::allocator<char>>::swap);

extern "C" void
__rust_thunk__18338933__ZNKSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEE13get_allocatorEv(
    class std::allocator<char>* __return,
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>> const* __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert((class std::allocator<char> (
                  ::std::basic_stringbuf<char, std::char_traits<char>,
                                         std::allocator<char>>::*)() const) &
              ::std::basic_stringbuf<char, std::char_traits<char>,
                                     std::allocator<char>>::get_allocator);

extern "C" void
__rust_thunk__c653e20f__ZNKSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEE4viewEv(
    ::std::__u::string_view* __return,
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>> const* __this) {
  new (__return) auto(__this->view());
}

static_assert((::std::__u::string_view (
                  ::std::basic_stringbuf<char, std::char_traits<char>,
                                         std::allocator<char>>::*)() const) &
              ::std::basic_stringbuf<char, std::char_traits<char>,
                                     std::allocator<char>>::view);

extern "C" void
__rust_thunk__4040fe20__ZNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_stringbuf<char, std::char_traits<char>,
                               std::allocator<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_stringstream<char, std::char_traits<char>,
                                                std::allocator<char>>) == 280);
static_assert(
    alignof(class std::basic_stringstream<char, std::char_traits<char>,
                                          std::allocator<char>>) == 8);

extern "C" void
__rust_thunk__1674e7ab__ZNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1Ev(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__fed9ac62__ZNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1Ej(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __this,
    unsigned int __wch) {
  crubit::construct_at(__this, __wch);
}

extern "C" void
__rust_thunk__c85d7660__ZNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1ERKNS_12basic_stringIcS2_S4_EEj(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const* __s,
    unsigned int __wch) {
  crubit::construct_at(__this, *__s, __wch);
}

extern "C" void
__rust_thunk__6fbc12f6__ZNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EjRKS4_(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __this,
    unsigned int __wch, class std::allocator<char> const* __a) {
  crubit::construct_at(__this, __wch, *__a);
}

extern "C" void
__rust_thunk__df2d8133__ZNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EONS_12basic_stringIcS2_S4_EEj(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>, std::allocator<char>>*
        __s,
    unsigned int __wch) {
  crubit::construct_at(__this, std::move(*__s), __wch);
}

extern "C" void
__rust_thunk__4b955580__ZNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EOS5_(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __this,
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __rhs) {
  crubit::construct_at(__this, std::move(*__rhs));
}

extern "C" class std::basic_stringstream<char, std::char_traits<char>,
                                         std::allocator<char>>*
__rust_thunk__56175ff1__ZNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEaSEOS5_(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __this,
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __rhs) {
  return std::addressof(__this->operator=(std::move(*__rhs)));
}

static_assert((class std::basic_stringstream<char, std::char_traits<char>,
                                             std::allocator<char>> &
               (::std::basic_stringstream<char, std::char_traits<char>,
                                          std::allocator<char>>::*)(
                   class std::basic_stringstream<char, std::char_traits<char>,
                                                 std::allocator<char>>&&)) &
              ::std::basic_stringstream<char, std::char_traits<char>,
                                        std::allocator<char>>::operator=);

extern "C" void
__rust_thunk__4cf9ff75__ZNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEE4swapERS5_(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __this,
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __rhs) {
  __this->swap(*__rhs);
}

static_assert((void (::std::basic_stringstream<char, std::char_traits<char>,
                                               std::allocator<char>>::*)(
                  class std::basic_stringstream<char, std::char_traits<char>,
                                                std::allocator<char>>&)) &
              ::std::basic_stringstream<char, std::char_traits<char>,
                                        std::allocator<char>>::swap);

extern "C" class std::basic_stringbuf<char, std::char_traits<char>,
                                      std::allocator<char>>*
__rust_thunk__b026324e__ZNKSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEE5rdbufEv(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>> const* __this) {
  return __this->rdbuf();
}

static_assert((class std::basic_stringbuf<char, std::char_traits<char>,
                                          std::allocator<char>> *
               (::std::basic_stringstream<char, std::char_traits<char>,
                                          std::allocator<char>>::*)() const) &
              ::std::basic_stringstream<char, std::char_traits<char>,
                                        std::allocator<char>>::rdbuf);

extern "C" void
__rust_thunk__ed9d55bd__ZNKSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEE4viewEv(
    ::std::__u::string_view* __return,
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>> const* __this) {
  new (__return) auto(__this->view());
}

static_assert((::std::__u::string_view (
                  ::std::basic_stringstream<char, std::char_traits<char>,
                                            std::allocator<char>>::*)() const) &
              ::std::basic_stringstream<char, std::char_traits<char>,
                                        std::allocator<char>>::view);

extern "C" void
__rust_thunk__1b7f66b0__ZNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_stringstream<char, std::char_traits<char>,
                                  std::allocator<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_ostringstream<char, std::char_traits<char>,
                                                 std::allocator<char>>) == 264);
static_assert(
    alignof(class std::basic_ostringstream<char, std::char_traits<char>,
                                           std::allocator<char>>) == 8);

extern "C" void
__rust_thunk__50329082__ZNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1Ev(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__a55a42ca__ZNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1Ej(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    unsigned int __wch) {
  crubit::construct_at(__this, __wch);
}

extern "C" void
__rust_thunk__57702383__ZNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1ERKNS_12basic_stringIcS2_S4_EEj(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const* __s,
    unsigned int __wch) {
  crubit::construct_at(__this, *__s, __wch);
}

extern "C" void
__rust_thunk__13107d82__ZNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EjRKS4_(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    unsigned int __wch, class std::allocator<char> const* __a) {
  crubit::construct_at(__this, __wch, *__a);
}

extern "C" void
__rust_thunk__787e7f06__ZNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EONS_12basic_stringIcS2_S4_EEj(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>, std::allocator<char>>*
        __s,
    unsigned int __wch) {
  crubit::construct_at(__this, std::move(*__s), __wch);
}

extern "C" void
__rust_thunk__e71b908c__ZNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EOS5_(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __rhs) {
  crubit::construct_at(__this, std::move(*__rhs));
}

extern "C" class std::basic_ostringstream<char, std::char_traits<char>,
                                          std::allocator<char>>*
__rust_thunk__d47dd03c__ZNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEaSEOS5_(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __rhs) {
  return std::addressof(__this->operator=(std::move(*__rhs)));
}

static_assert((class std::basic_ostringstream<char, std::char_traits<char>,
                                              std::allocator<char>> &
               (::std::basic_ostringstream<char, std::char_traits<char>,
                                           std::allocator<char>>::*)(
                   class std::basic_ostringstream<char, std::char_traits<char>,
                                                  std::allocator<char>>&&)) &
              ::std::basic_ostringstream<char, std::char_traits<char>,
                                         std::allocator<char>>::operator=);

extern "C" void
__rust_thunk__dbbac04c__ZNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEE4swapERS5_(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __rhs) {
  __this->swap(*__rhs);
}

static_assert((void (::std::basic_ostringstream<char, std::char_traits<char>,
                                                std::allocator<char>>::*)(
                  class std::basic_ostringstream<char, std::char_traits<char>,
                                                 std::allocator<char>>&)) &
              ::std::basic_ostringstream<char, std::char_traits<char>,
                                         std::allocator<char>>::swap);

extern "C" class std::basic_stringbuf<char, std::char_traits<char>,
                                      std::allocator<char>>*
__rust_thunk__e5ea186e__ZNKSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEE5rdbufEv(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>> const* __this) {
  return __this->rdbuf();
}

static_assert((class std::basic_stringbuf<char, std::char_traits<char>,
                                          std::allocator<char>> *
               (::std::basic_ostringstream<char, std::char_traits<char>,
                                           std::allocator<char>>::*)() const) &
              ::std::basic_ostringstream<char, std::char_traits<char>,
                                         std::allocator<char>>::rdbuf);

extern "C" void
__rust_thunk__de3b3363__ZNKSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEE4viewEv(
    ::std::__u::string_view* __return,
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>> const* __this) {
  new (__return) auto(__this->view());
}

static_assert((::std::__u::string_view (
                  ::std::basic_ostringstream<char, std::char_traits<char>,
                                             std::allocator<char>>::*)()
                   const) &
              ::std::basic_ostringstream<char, std::char_traits<char>,
                                         std::allocator<char>>::view);

extern "C" void
__rust_thunk__487e7824__ZNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_ostringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(class std::basic_istringstream<char, std::char_traits<char>,
                                                 std::allocator<char>>) == 272);
static_assert(
    alignof(class std::basic_istringstream<char, std::char_traits<char>,
                                           std::allocator<char>>) == 8);

extern "C" void
__rust_thunk__5704dad6__ZNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1Ev(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__5f8da0c5__ZNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1Ej(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    unsigned int __wch) {
  crubit::construct_at(__this, __wch);
}

extern "C" void
__rust_thunk__cc4aa253__ZNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1ERKNS_12basic_stringIcS2_S4_EEj(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const* __s,
    unsigned int __wch) {
  crubit::construct_at(__this, *__s, __wch);
}

extern "C" void
__rust_thunk__1cca9be9__ZNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EjRKS4_(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    unsigned int __wch, class std::allocator<char> const* __a) {
  crubit::construct_at(__this, __wch, *__a);
}

extern "C" void
__rust_thunk__3c33631c__ZNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EONS_12basic_stringIcS2_S4_EEj(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>, std::allocator<char>>*
        __s,
    unsigned int __wch) {
  crubit::construct_at(__this, std::move(*__s), __wch);
}

extern "C" void
__rust_thunk__4286c151__ZNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEC1EOS5_(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __rhs) {
  crubit::construct_at(__this, std::move(*__rhs));
}

extern "C" class std::basic_istringstream<char, std::char_traits<char>,
                                          std::allocator<char>>*
__rust_thunk__ef6c2483__ZNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEaSEOS5_(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __rhs) {
  return std::addressof(__this->operator=(std::move(*__rhs)));
}

static_assert((class std::basic_istringstream<char, std::char_traits<char>,
                                              std::allocator<char>> &
               (::std::basic_istringstream<char, std::char_traits<char>,
                                           std::allocator<char>>::*)(
                   class std::basic_istringstream<char, std::char_traits<char>,
                                                  std::allocator<char>>&&)) &
              ::std::basic_istringstream<char, std::char_traits<char>,
                                         std::allocator<char>>::operator=);

extern "C" void
__rust_thunk__dabc3e01__ZNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEE4swapERS5_(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this,
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __rhs) {
  __this->swap(*__rhs);
}

static_assert((void (::std::basic_istringstream<char, std::char_traits<char>,
                                                std::allocator<char>>::*)(
                  class std::basic_istringstream<char, std::char_traits<char>,
                                                 std::allocator<char>>&)) &
              ::std::basic_istringstream<char, std::char_traits<char>,
                                         std::allocator<char>>::swap);

extern "C" class std::basic_stringbuf<char, std::char_traits<char>,
                                      std::allocator<char>>*
__rust_thunk__b85a7068__ZNKSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEE5rdbufEv(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>> const* __this) {
  return __this->rdbuf();
}

static_assert((class std::basic_stringbuf<char, std::char_traits<char>,
                                          std::allocator<char>> *
               (::std::basic_istringstream<char, std::char_traits<char>,
                                           std::allocator<char>>::*)() const) &
              ::std::basic_istringstream<char, std::char_traits<char>,
                                         std::allocator<char>>::rdbuf);

extern "C" void
__rust_thunk__6bc76e8b__ZNKSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEE4viewEv(
    ::std::__u::string_view* __return,
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>> const* __this) {
  new (__return) auto(__this->view());
}

static_assert((::std::__u::string_view (
                  ::std::basic_istringstream<char, std::char_traits<char>,
                                             std::allocator<char>>::*)()
                   const) &
              ::std::basic_istringstream<char, std::char_traits<char>,
                                         std::allocator<char>>::view);

extern "C" void
__rust_thunk__c1ce9b2b__ZNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__crubit_operator_delete____CcTemplateInstNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fproto_3amy_5fproto_5fapi(
    class std::basic_istringstream<char, std::char_traits<char>,
                                   std::allocator<char>>* ptr) {
  delete ptr;
}

static_assert(
    CRUBIT_SIZEOF(struct std::array<
                  proto2::internal::ThreadSafeArenaStats::BlockStats, 16UL>) ==
    512);
static_assert(
    alignof(struct std::array<
            proto2::internal::ThreadSafeArenaStats::BlockStats, 16UL>) == 8);
static_assert(
    CRUBIT_OFFSET_OF(
        __elems_,
        struct std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                          16UL>) == 0);

extern "C" void
__rust_thunk__56dd02cc__ZNSt3__u5arrayIN6proto28internal20ThreadSafeArenaStats10BlockStatsELm16EEC1Ev(
    struct std::array<proto2::internal::ThreadSafeArenaStats::BlockStats, 16UL>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6d848da9__ZNKSt3__u5arrayIN6proto28internal20ThreadSafeArenaStats10BlockStatsELm16EE7crbeginEv(
    class std::reverse_iterator<
        const proto2::internal::ThreadSafeArenaStats::BlockStats*>* __return,
    struct std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                      16UL> const* __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<
        const proto2::internal::ThreadSafeArenaStats::BlockStats*> (
        ::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                     16UL>::*)() const) &
    ::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                 16UL>::crbegin);

extern "C" void
__rust_thunk__d7789f65__ZNKSt3__u5arrayIN6proto28internal20ThreadSafeArenaStats10BlockStatsELm16EE5crendEv(
    class std::reverse_iterator<
        const proto2::internal::ThreadSafeArenaStats::BlockStats*>* __return,
    struct std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                      16UL> const* __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<
        const proto2::internal::ThreadSafeArenaStats::BlockStats*> (
        ::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                     16UL>::*)() const) &
    ::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                 16UL>::crend);

extern "C" size_t
__rust_thunk__2d784c45__ZNKSt3__u5arrayIN6proto28internal20ThreadSafeArenaStats10BlockStatsELm16EE4sizeEv(
    struct std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                      16UL> const* __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                          16UL>::*)() const) &
    ::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                 16UL>::size);

extern "C" size_t
__rust_thunk__353ad7a8__ZNKSt3__u5arrayIN6proto28internal20ThreadSafeArenaStats10BlockStatsELm16EE8max_sizeEv(
    struct std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                      16UL> const* __this) {
  return __this->max_size();
}

static_assert(
    (size_t (::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                          16UL>::*)() const) &
    ::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                 16UL>::max_size);

extern "C" bool
__rust_thunk__4b0b0325__ZNKSt3__u5arrayIN6proto28internal20ThreadSafeArenaStats10BlockStatsELm16EE5emptyEv(
    struct std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                      16UL> const* __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                        16UL>::*)() const) &
    ::std::array<proto2::internal::ThreadSafeArenaStats::BlockStats,
                 16UL>::empty);

static_assert(sizeof(struct std::array<signed char, 7UL>) == 7);
static_assert(alignof(struct std::array<signed char, 7UL>) == 1);
static_assert(CRUBIT_OFFSET_OF(__elems_, struct std::array<signed char, 7UL>) ==
              0);

extern "C" void __rust_thunk__56dd02cc__ZNSt3__u5arrayIaLm7EEC1Ev(
    struct std::array<signed char, 7UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__edcaa890__ZNSt3__u5arrayIaLm7EE4fillERKa(
    struct std::array<signed char, 7UL>* __this, signed char const* __u) {
  __this->fill(*__u);
}

static_assert((void (::std::array<signed char, 7UL>::*)(signed char const&)) &
              ::std::array<signed char, 7UL>::fill);

extern "C" void __rust_thunk__af1e912a__ZNSt3__u5arrayIaLm7EE4swapERS1_(
    struct std::array<signed char, 7UL>* __this,
    struct std::array<signed char, 7UL>* __a) {
  __this->swap(*__a);
}

static_assert((void (::std::array<signed char, 7UL>::*)(
                  struct std::array<signed char, 7UL>&)) &
              ::std::array<signed char, 7UL>::swap);

extern "C" signed char const*
__rust_thunk__b208ad1e__ZNKSt3__u5arrayIaLm7EE6cbeginEv(
    struct std::array<signed char, 7UL> const* __this) {
  return __this->cbegin();
}

static_assert((signed char const* (::std::array<signed char, 7UL>::*)() const) &
              ::std::array<signed char, 7UL>::cbegin);

extern "C" signed char const*
__rust_thunk__42d44e98__ZNKSt3__u5arrayIaLm7EE4cendEv(
    struct std::array<signed char, 7UL> const* __this) {
  return __this->cend();
}

static_assert((signed char const* (::std::array<signed char, 7UL>::*)() const) &
              ::std::array<signed char, 7UL>::cend);

extern "C" void __rust_thunk__6d848da9__ZNKSt3__u5arrayIaLm7EE7crbeginEv(
    class std::reverse_iterator<const signed char*>* __return,
    struct std::array<signed char, 7UL> const* __this) {
  new (__return) auto(__this->crbegin());
}

static_assert((class std::reverse_iterator<const signed char*> (
                  ::std::array<signed char, 7UL>::*)() const) &
              ::std::array<signed char, 7UL>::crbegin);

extern "C" void __rust_thunk__d7789f65__ZNKSt3__u5arrayIaLm7EE5crendEv(
    class std::reverse_iterator<const signed char*>* __return,
    struct std::array<signed char, 7UL> const* __this) {
  new (__return) auto(__this->crend());
}

static_assert((class std::reverse_iterator<const signed char*> (
                  ::std::array<signed char, 7UL>::*)() const) &
              ::std::array<signed char, 7UL>::crend);

extern "C" size_t __rust_thunk__2d784c45__ZNKSt3__u5arrayIaLm7EE4sizeEv(
    struct std::array<signed char, 7UL> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::array<signed char, 7UL>::*)() const) &
              ::std::array<signed char, 7UL>::size);

extern "C" size_t __rust_thunk__353ad7a8__ZNKSt3__u5arrayIaLm7EE8max_sizeEv(
    struct std::array<signed char, 7UL> const* __this) {
  return __this->max_size();
}

static_assert((size_t (::std::array<signed char, 7UL>::*)() const) &
              ::std::array<signed char, 7UL>::max_size);

extern "C" bool __rust_thunk__4b0b0325__ZNKSt3__u5arrayIaLm7EE5emptyEv(
    struct std::array<signed char, 7UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::array<signed char, 7UL>::*)() const) &
              ::std::array<signed char, 7UL>::empty);

extern "C" signed char* __rust_thunk__ac9d0de6__ZNSt3__u5arrayIaLm7EEixEm(
    struct std::array<signed char, 7UL>* __this, size_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((signed char& (::std::array<signed char, 7UL>::*)(size_t)) &
              ::std::array<signed char, 7UL>::operator[]);

extern "C" signed char const*
__rust_thunk__1f73e0f0__ZNKSt3__u5arrayIaLm7EEixEm(
    struct std::array<signed char, 7UL> const* __this, size_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((signed char const& (::std::array<signed char, 7UL>::*)(size_t)
                   const) &
              ::std::array<signed char, 7UL>::operator[]);

static_assert(sizeof(struct std::array<char, 7UL>) == 7);
static_assert(alignof(struct std::array<char, 7UL>) == 1);
static_assert(CRUBIT_OFFSET_OF(__elems_, struct std::array<char, 7UL>) == 0);

extern "C" void __rust_thunk__56dd02cc__ZNSt3__u5arrayIcLm7EEC1Ev(
    struct std::array<char, 7UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__edcaa890__ZNSt3__u5arrayIcLm7EE4fillERKc(
    struct std::array<char, 7UL>* __this, char const* __u) {
  __this->fill(*__u);
}

static_assert((void (::std::array<char, 7UL>::*)(char const&)) &
              ::std::array<char, 7UL>::fill);

extern "C" void __rust_thunk__af1e912a__ZNSt3__u5arrayIcLm7EE4swapERS1_(
    struct std::array<char, 7UL>* __this, struct std::array<char, 7UL>* __a) {
  __this->swap(*__a);
}

static_assert(
    (void (::std::array<char, 7UL>::*)(struct std::array<char, 7UL>&)) &
    ::std::array<char, 7UL>::swap);

extern "C" char const* __rust_thunk__b208ad1e__ZNKSt3__u5arrayIcLm7EE6cbeginEv(
    struct std::array<char, 7UL> const* __this) {
  return __this->cbegin();
}

static_assert((char const* (::std::array<char, 7UL>::*)() const) &
              ::std::array<char, 7UL>::cbegin);

extern "C" char const* __rust_thunk__42d44e98__ZNKSt3__u5arrayIcLm7EE4cendEv(
    struct std::array<char, 7UL> const* __this) {
  return __this->cend();
}

static_assert((char const* (::std::array<char, 7UL>::*)() const) &
              ::std::array<char, 7UL>::cend);

extern "C" void __rust_thunk__6d848da9__ZNKSt3__u5arrayIcLm7EE7crbeginEv(
    class std::reverse_iterator<const char*>* __return,
    struct std::array<char, 7UL> const* __this) {
  new (__return) auto(__this->crbegin());
}

static_assert((class std::reverse_iterator<const char*> (
                  ::std::array<char, 7UL>::*)() const) &
              ::std::array<char, 7UL>::crbegin);

extern "C" void __rust_thunk__d7789f65__ZNKSt3__u5arrayIcLm7EE5crendEv(
    class std::reverse_iterator<const char*>* __return,
    struct std::array<char, 7UL> const* __this) {
  new (__return) auto(__this->crend());
}

static_assert((class std::reverse_iterator<const char*> (
                  ::std::array<char, 7UL>::*)() const) &
              ::std::array<char, 7UL>::crend);

extern "C" size_t __rust_thunk__2d784c45__ZNKSt3__u5arrayIcLm7EE4sizeEv(
    struct std::array<char, 7UL> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::array<char, 7UL>::*)() const) &
              ::std::array<char, 7UL>::size);

extern "C" size_t __rust_thunk__353ad7a8__ZNKSt3__u5arrayIcLm7EE8max_sizeEv(
    struct std::array<char, 7UL> const* __this) {
  return __this->max_size();
}

static_assert((size_t (::std::array<char, 7UL>::*)() const) &
              ::std::array<char, 7UL>::max_size);

extern "C" bool __rust_thunk__4b0b0325__ZNKSt3__u5arrayIcLm7EE5emptyEv(
    struct std::array<char, 7UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::array<char, 7UL>::*)() const) &
              ::std::array<char, 7UL>::empty);

extern "C" char* __rust_thunk__ac9d0de6__ZNSt3__u5arrayIcLm7EEixEm(
    struct std::array<char, 7UL>* __this, size_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char& (::std::array<char, 7UL>::*)(size_t)) &
              ::std::array<char, 7UL>::operator[]);

extern "C" char const* __rust_thunk__1f73e0f0__ZNKSt3__u5arrayIcLm7EEixEm(
    struct std::array<char, 7UL> const* __this, size_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char const& (::std::array<char, 7UL>::*)(size_t) const) &
              ::std::array<char, 7UL>::operator[]);

static_assert(sizeof(struct std::array<unsigned char, 7UL>) == 7);
static_assert(alignof(struct std::array<unsigned char, 7UL>) == 1);
static_assert(CRUBIT_OFFSET_OF(__elems_,
                               struct std::array<unsigned char, 7UL>) == 0);

extern "C" void __rust_thunk__56dd02cc__ZNSt3__u5arrayIhLm7EEC1Ev(
    struct std::array<unsigned char, 7UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__edcaa890__ZNSt3__u5arrayIhLm7EE4fillERKh(
    struct std::array<unsigned char, 7UL>* __this, unsigned char const* __u) {
  __this->fill(*__u);
}

static_assert(
    (void (::std::array<unsigned char, 7UL>::*)(unsigned char const&)) &
    ::std::array<unsigned char, 7UL>::fill);

extern "C" void __rust_thunk__af1e912a__ZNSt3__u5arrayIhLm7EE4swapERS1_(
    struct std::array<unsigned char, 7UL>* __this,
    struct std::array<unsigned char, 7UL>* __a) {
  __this->swap(*__a);
}

static_assert((void (::std::array<unsigned char, 7UL>::*)(
                  struct std::array<unsigned char, 7UL>&)) &
              ::std::array<unsigned char, 7UL>::swap);

extern "C" unsigned char const*
__rust_thunk__b208ad1e__ZNKSt3__u5arrayIhLm7EE6cbeginEv(
    struct std::array<unsigned char, 7UL> const* __this) {
  return __this->cbegin();
}

static_assert((unsigned char const* (::std::array<unsigned char, 7UL>::*)()
                   const) &
              ::std::array<unsigned char, 7UL>::cbegin);

extern "C" unsigned char const*
__rust_thunk__42d44e98__ZNKSt3__u5arrayIhLm7EE4cendEv(
    struct std::array<unsigned char, 7UL> const* __this) {
  return __this->cend();
}

static_assert((unsigned char const* (::std::array<unsigned char, 7UL>::*)()
                   const) &
              ::std::array<unsigned char, 7UL>::cend);

extern "C" void __rust_thunk__6d848da9__ZNKSt3__u5arrayIhLm7EE7crbeginEv(
    class std::reverse_iterator<const unsigned char*>* __return,
    struct std::array<unsigned char, 7UL> const* __this) {
  new (__return) auto(__this->crbegin());
}

static_assert((class std::reverse_iterator<const unsigned char*> (
                  ::std::array<unsigned char, 7UL>::*)() const) &
              ::std::array<unsigned char, 7UL>::crbegin);

extern "C" void __rust_thunk__d7789f65__ZNKSt3__u5arrayIhLm7EE5crendEv(
    class std::reverse_iterator<const unsigned char*>* __return,
    struct std::array<unsigned char, 7UL> const* __this) {
  new (__return) auto(__this->crend());
}

static_assert((class std::reverse_iterator<const unsigned char*> (
                  ::std::array<unsigned char, 7UL>::*)() const) &
              ::std::array<unsigned char, 7UL>::crend);

extern "C" size_t __rust_thunk__2d784c45__ZNKSt3__u5arrayIhLm7EE4sizeEv(
    struct std::array<unsigned char, 7UL> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::array<unsigned char, 7UL>::*)() const) &
              ::std::array<unsigned char, 7UL>::size);

extern "C" size_t __rust_thunk__353ad7a8__ZNKSt3__u5arrayIhLm7EE8max_sizeEv(
    struct std::array<unsigned char, 7UL> const* __this) {
  return __this->max_size();
}

static_assert((size_t (::std::array<unsigned char, 7UL>::*)() const) &
              ::std::array<unsigned char, 7UL>::max_size);

extern "C" bool __rust_thunk__4b0b0325__ZNKSt3__u5arrayIhLm7EE5emptyEv(
    struct std::array<unsigned char, 7UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::array<unsigned char, 7UL>::*)() const) &
              ::std::array<unsigned char, 7UL>::empty);

extern "C" unsigned char* __rust_thunk__ac9d0de6__ZNSt3__u5arrayIhLm7EEixEm(
    struct std::array<unsigned char, 7UL>* __this, size_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((unsigned char& (::std::array<unsigned char, 7UL>::*)(size_t)) &
              ::std::array<unsigned char, 7UL>::operator[]);

extern "C" unsigned char const*
__rust_thunk__1f73e0f0__ZNKSt3__u5arrayIhLm7EEixEm(
    struct std::array<unsigned char, 7UL> const* __this, size_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert(
    (unsigned char const& (::std::array<unsigned char, 7UL>::*)(size_t) const) &
    ::std::array<unsigned char, 7UL>::operator[]);

static_assert(sizeof(struct std::placeholders::__ph<10>) == 1);
static_assert(alignof(struct std::placeholders::__ph<10>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi10EEC1Ev(
    struct std::placeholders::__ph<10>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<1>) == 1);
static_assert(alignof(struct std::placeholders::__ph<1>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi1EEC1Ev(
    struct std::placeholders::__ph<1>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<2>) == 1);
static_assert(alignof(struct std::placeholders::__ph<2>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi2EEC1Ev(
    struct std::placeholders::__ph<2>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<3>) == 1);
static_assert(alignof(struct std::placeholders::__ph<3>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi3EEC1Ev(
    struct std::placeholders::__ph<3>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<4>) == 1);
static_assert(alignof(struct std::placeholders::__ph<4>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi4EEC1Ev(
    struct std::placeholders::__ph<4>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<5>) == 1);
static_assert(alignof(struct std::placeholders::__ph<5>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi5EEC1Ev(
    struct std::placeholders::__ph<5>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<6>) == 1);
static_assert(alignof(struct std::placeholders::__ph<6>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi6EEC1Ev(
    struct std::placeholders::__ph<6>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<7>) == 1);
static_assert(alignof(struct std::placeholders::__ph<7>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi7EEC1Ev(
    struct std::placeholders::__ph<7>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<8>) == 1);
static_assert(alignof(struct std::placeholders::__ph<8>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi8EEC1Ev(
    struct std::placeholders::__ph<8>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<9>) == 1);
static_assert(alignof(struct std::placeholders::__ph<9>) == 1);

extern "C" void __rust_thunk__8fc4c420__ZNSt3__u12placeholders4__phILi9EEC1Ev(
    struct std::placeholders::__ph<9>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>) == 48);
static_assert(
    alignof(class std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>) == 8);

extern "C" void
__rust_thunk__e7b3fd31__ZNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcED1Ev(
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>*
        __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__81204f90__ZNKSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcE3argEm(
    class std::basic_format_arg<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __return,
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>,
        char> const* __this,
    size_t __id) {
  new (__return) auto(__this->arg(__id));
}

static_assert(
    (class std::basic_format_arg<std::basic_format_context<
         std::back_insert_iterator<std::__format::__output_buffer<char>>,
         char>> (
        ::std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>::*)(size_t) const) &
    ::std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>,
        char>::arg);

extern "C" void
__rust_thunk__eeae3c0b__ZNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcE6localeEv(
    class ::std::__u::locale* __return,
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>*
        __this) {
  new (__return) auto(__this->locale());
}

static_assert(
    (class ::std::__u::locale (
        ::std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>::*)()) &
    ::std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>,
        char>::locale);

extern "C" void
__rust_thunk__aa8a24ed__ZNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcE3outEv(
    class std::back_insert_iterator<std::__format::__output_buffer<char>>*
        __return,
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>*
        __this) {
  new (__return) auto(__this->out());
}

static_assert(
    (class std::back_insert_iterator<std::__format::__output_buffer<char>> (
        ::std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>::*)()) &
    ::std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>,
        char>::out);

extern "C" void
__rust_thunk__e45b2234__ZNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcE10advance_toES5_(
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>*
        __this,
    class std::back_insert_iterator<std::__format::__output_buffer<char>>*
        __it) {
  __this->advance_to(std::move(*__it));
}

static_assert(
    (void (::std::basic_format_context<
           std::back_insert_iterator<std::__format::__output_buffer<char>>,
           char>::*)(
        class std::back_insert_iterator<
            std::__format::__output_buffer<char>>)) &
    ::std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>,
        char>::advance_to);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>) == 48);
static_assert(
    alignof(class std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>) == 8);

extern "C" void
__rust_thunk__e7b3fd31__ZNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwED1Ev(
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__81204f90__ZNKSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwE3argEm(
    class std::basic_format_arg<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __return,
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t> const* __this,
    size_t __id) {
  new (__return) auto(__this->arg(__id));
}

static_assert(
    (class std::basic_format_arg<std::basic_format_context<
         std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
         wchar_t>> (
        ::std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>::*)(size_t) const) &
    ::std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>::arg);

extern "C" void
__rust_thunk__eeae3c0b__ZNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwE6localeEv(
    class ::std::__u::locale* __return,
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>* __this) {
  new (__return) auto(__this->locale());
}

static_assert(
    (class ::std::__u::locale (
        ::std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>::*)()) &
    ::std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>::locale);

extern "C" void
__rust_thunk__aa8a24ed__ZNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwE3outEv(
    class std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>*
        __return,
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>* __this) {
  new (__return) auto(__this->out());
}

static_assert(
    (class std::back_insert_iterator<std::__format::__output_buffer<wchar_t>> (
        ::std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>::*)()) &
    ::std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>::out);

extern "C" void
__rust_thunk__e45b2234__ZNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwE10advance_toES5_(
    class std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>* __this,
    class std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>*
        __it) {
  __this->advance_to(std::move(*__it));
}

static_assert(
    (void (::std::basic_format_context<
           std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
           wchar_t>::*)(
        class std::back_insert_iterator<
            std::__format::__output_buffer<wchar_t>>)) &
    ::std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>::advance_to);

static_assert(
    CRUBIT_SIZEOF(
        class std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>) == 16);
static_assert(
    alignof(class std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>) == 8);

extern "C" void
__rust_thunk__93e9e612__ZNSt3__u19__basic_node_handleINS_11__tree_nodeINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPvEENS6_INS_4pairIKS8_SB_EEEENS_27__map_node_handle_specificsEEC1Ev(
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__c43e58c7__ZNSt3__u19__basic_node_handleINS_11__tree_nodeINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPvEENS6_INS_4pairIKS8_SB_EEEENS_27__map_node_handle_specificsEEC1EOSK_(
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>* __this,
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>* __other) {
  crubit::construct_at(__this, std::move(*__other));
}

extern "C" class std::__basic_node_handle<
    std::__tree_node<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        void*>,
    std::allocator<
        std::pair<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                  tcmalloc::MallocExtension::Property>>,
    std::__map_node_handle_specifics>*
__rust_thunk__1a552d01__ZNSt3__u19__basic_node_handleINS_11__tree_nodeINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPvEENS6_INS_4pairIKS8_SB_EEEENS_27__map_node_handle_specificsEEaSEOSK_(
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>* __this,
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>* __other) {
  return std::addressof(__this->operator=(std::move(*__other)));
}

static_assert(
    (class std::__basic_node_handle<
         std::__tree_node<
             std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                               tcmalloc::MallocExtension::Property>,
             void*>,
         std::allocator<
             std::pair<const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>,
         std::__map_node_handle_specifics> &
     (::std::__basic_node_handle<
         std::__tree_node<
             std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                               tcmalloc::MallocExtension::Property>,
             void*>,
         std::allocator<
             std::pair<const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>,
         std::__map_node_handle_specifics>::*)(
         class std::__basic_node_handle<
             std::__tree_node<
                 std::__value_type<
                     std::basic_string<char, std::char_traits<char>,
                                       std::allocator<char>>,
                     tcmalloc::MallocExtension::Property>,
                 void*>,
             std::allocator<
                 std::pair<const std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>>,
             std::__map_node_handle_specifics>&&)) &
    ::std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>::operator=);

extern "C" void
__rust_thunk__e6eab5d9__ZNKSt3__u19__basic_node_handleINS_11__tree_nodeINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPvEENS6_INS_4pairIKS8_SB_EEEENS_27__map_node_handle_specificsEE13get_allocatorEv(
    class std::allocator<
        std::pair<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                  tcmalloc::MallocExtension::Property>>* __return,
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics> const* __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert(
    (class std::allocator<
        std::pair<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                  tcmalloc::MallocExtension::Property>> (
        ::std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>::*)() const) &
    ::std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>::get_allocator);

extern "C" bool
__rust_thunk__3dfdbf7a__ZNKSt3__u19__basic_node_handleINS_11__tree_nodeINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPvEENS6_INS_4pairIKS8_SB_EEEENS_27__map_node_handle_specificsEE5emptyEv(
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics> const* __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::__basic_node_handle<
           std::__tree_node<
               std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
               void*>,
           std::allocator<
               std::pair<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                         tcmalloc::MallocExtension::Property>>,
           std::__map_node_handle_specifics>::*)() const) &
    ::std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>::empty);

extern "C" void
__rust_thunk__43c78e07__ZNSt3__u19__basic_node_handleINS_11__tree_nodeINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPvEENS6_INS_4pairIKS8_SB_EEEENS_27__map_node_handle_specificsEE4swapERSK_(
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>* __this,
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>* __other) {
  __this->swap(*__other);
}

static_assert(
    (void (::std::__basic_node_handle<
           std::__tree_node<
               std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
               void*>,
           std::allocator<
               std::pair<const std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                         tcmalloc::MallocExtension::Property>>,
           std::__map_node_handle_specifics>::*)(
        class std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>&)) &
    ::std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>::swap);

extern "C" void
__rust_thunk__a39e08ca__ZNSt3__u19__basic_node_handleINS_11__tree_nodeINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPvEENS6_INS_4pairIKS8_SB_EEEENS_27__map_node_handle_specificsEED1Ev(
    class std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>* __this) {
  std::destroy_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__insert_return_type<
                  std::__map_iterator<std::__tree_iterator<
                      std::__value_type<
                          std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
                      std::__tree_node<
                          std::__value_type<
                              std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
                          void*>*,
                      long>>,
                  std::__basic_node_handle<
                      std::__tree_node<
                          std::__value_type<
                              std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
                          void*>,
                      std::allocator<std::pair<
                          const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
                      std::__map_node_handle_specifics>>) == 32);
static_assert(
    alignof(struct std::__insert_return_type<
            std::__map_iterator<std::__tree_iterator<
                std::__value_type<
                    std::basic_string<char, std::char_traits<char>,
                                      std::allocator<char>>,
                    tcmalloc::MallocExtension::Property>,
                std::__tree_node<
                    std::__value_type<
                        std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                        tcmalloc::MallocExtension::Property>,
                    void*>*,
                long>>,
            std::__basic_node_handle<
                std::__tree_node<
                    std::__value_type<
                        std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                        tcmalloc::MallocExtension::Property>,
                    void*>,
                std::allocator<std::pair<
                    const std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                    tcmalloc::MallocExtension::Property>>,
                std::__map_node_handle_specifics>>) == 8);
static_assert(
    CRUBIT_OFFSET_OF(
        position,
        struct std::__insert_return_type<
            std::__map_iterator<std::__tree_iterator<
                std::__value_type<
                    std::basic_string<char, std::char_traits<char>,
                                      std::allocator<char>>,
                    tcmalloc::MallocExtension::Property>,
                std::__tree_node<
                    std::__value_type<
                        std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                        tcmalloc::MallocExtension::Property>,
                    void*>*,
                long>>,
            std::__basic_node_handle<
                std::__tree_node<
                    std::__value_type<
                        std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                        tcmalloc::MallocExtension::Property>,
                    void*>,
                std::allocator<std::pair<
                    const std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                    tcmalloc::MallocExtension::Property>>,
                std::__map_node_handle_specifics>>) == 0);
static_assert(
    CRUBIT_OFFSET_OF(
        inserted,
        struct std::__insert_return_type<
            std::__map_iterator<std::__tree_iterator<
                std::__value_type<
                    std::basic_string<char, std::char_traits<char>,
                                      std::allocator<char>>,
                    tcmalloc::MallocExtension::Property>,
                std::__tree_node<
                    std::__value_type<
                        std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                        tcmalloc::MallocExtension::Property>,
                    void*>*,
                long>>,
            std::__basic_node_handle<
                std::__tree_node<
                    std::__value_type<
                        std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                        tcmalloc::MallocExtension::Property>,
                    void*>,
                std::allocator<std::pair<
                    const std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                    tcmalloc::MallocExtension::Property>>,
                std::__map_node_handle_specifics>>) == 8);
static_assert(
    CRUBIT_OFFSET_OF(
        node, struct std::__insert_return_type<
                  std::__map_iterator<std::__tree_iterator<
                      std::__value_type<
                          std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
                      std::__tree_node<
                          std::__value_type<
                              std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
                          void*>*,
                      long>>,
                  std::__basic_node_handle<
                      std::__tree_node<
                          std::__value_type<
                              std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
                          void*>,
                      std::allocator<std::pair<
                          const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
                      std::__map_node_handle_specifics>>) == 16);

extern "C" void
__rust_thunk__1a637f57__ZNSt3__u20__insert_return_typeINS_14__map_iteratorINS_15__tree_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISD_PvEElEEEENS_19__basic_node_handleISG_NS7_INS_4pairIKS9_SC_EEEENS_27__map_node_handle_specificsEEEEC1Ev(
    struct std::__insert_return_type<
        std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>,
        std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__1a637f57__ZNSt3__u20__insert_return_typeINS_14__map_iteratorINS_15__tree_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISD_PvEElEEEENS_19__basic_node_handleISG_NS7_INS_4pairIKS9_SC_EEEENS_27__map_node_handle_specificsEEEEC1EOSR_(
    struct std::__insert_return_type<
        std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>,
        std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>>* __this,
    struct std::__insert_return_type<
        std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>,
        std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>>* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct std::__insert_return_type<
    std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>,
    std::__basic_node_handle<
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>,
        std::__map_node_handle_specifics>>*
__rust_thunk__1a637f57__ZNSt3__u20__insert_return_typeINS_14__map_iteratorINS_15__tree_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISD_PvEElEEEENS_19__basic_node_handleISG_NS7_INS_4pairIKS9_SC_EEEENS_27__map_node_handle_specificsEEEEaSEOSR_(
    struct std::__insert_return_type<
        std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>,
        std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>>* __this,
    struct std::__insert_return_type<
        std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>,
        std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert(
    (struct std::__insert_return_type<
         std::__map_iterator<std::__tree_iterator<
             std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                               tcmalloc::MallocExtension::Property>,
             std::__tree_node<
                 std::__value_type<
                     std::basic_string<char, std::char_traits<char>,
                                       std::allocator<char>>,
                     tcmalloc::MallocExtension::Property>,
                 void*>*,
             long>>,
         std::__basic_node_handle<
             std::__tree_node<
                 std::__value_type<
                     std::basic_string<char, std::char_traits<char>,
                                       std::allocator<char>>,
                     tcmalloc::MallocExtension::Property>,
                 void*>,
             std::allocator<
                 std::pair<const std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>>,
             std::__map_node_handle_specifics>> &
     (::std::__insert_return_type<
         std::__map_iterator<std::__tree_iterator<
             std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                               tcmalloc::MallocExtension::Property>,
             std::__tree_node<
                 std::__value_type<
                     std::basic_string<char, std::char_traits<char>,
                                       std::allocator<char>>,
                     tcmalloc::MallocExtension::Property>,
                 void*>*,
             long>>,
         std::__basic_node_handle<
             std::__tree_node<
                 std::__value_type<
                     std::basic_string<char, std::char_traits<char>,
                                       std::allocator<char>>,
                     tcmalloc::MallocExtension::Property>,
                 void*>,
             std::allocator<
                 std::pair<const std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>>,
             std::__map_node_handle_specifics>>::*)(
         struct std::__insert_return_type<
             std::__map_iterator<std::__tree_iterator<
                 std::__value_type<
                     std::basic_string<char, std::char_traits<char>,
                                       std::allocator<char>>,
                     tcmalloc::MallocExtension::Property>,
                 std::__tree_node<
                     std::__value_type<
                         std::basic_string<char, std::char_traits<char>,
                                           std::allocator<char>>,
                         tcmalloc::MallocExtension::Property>,
                     void*>*,
                 long>>,
             std::__basic_node_handle<
                 std::__tree_node<
                     std::__value_type<
                         std::basic_string<char, std::char_traits<char>,
                                           std::allocator<char>>,
                         tcmalloc::MallocExtension::Property>,
                     void*>,
                 std::allocator<std::pair<
                     const std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>,
                     tcmalloc::MallocExtension::Property>>,
                 std::__map_node_handle_specifics>>&&)) &
    ::std::__insert_return_type<
        std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>,
        std::__basic_node_handle<
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>,
            std::allocator<
                std::pair<const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>,
            std::__map_node_handle_specifics>>::operator=);

static_assert(
    CRUBIT_SIZEOF(
        class std::variant<
            std::variant<unsigned long, long, unsigned int, int, bool>,
            std::variant<unsigned long, long, double>, absl::Span<const char>,
            std::variant<unsigned int, int, float>>) == 32);
static_assert(
    alignof(class std::variant<
            std::variant<unsigned long, long, unsigned int, int, bool>,
            std::variant<unsigned long, long, double>, absl::Span<const char>,
            std::variant<unsigned int, int, float>>) == 8);

extern "C" bool
__rust_thunk__0f615caa__ZNKSt3__u7variantIJNS0_IJmljibEEENS0_IJmldEEEN4absl4SpanIKcEENS0_IJjifEEEEE22valueless_by_exceptionEv(
    class std::variant<
        std::variant<unsigned long, long, unsigned int, int, bool>,
        std::variant<unsigned long, long, double>, absl::Span<const char>,
        std::variant<unsigned int, int, float>> const* __this) {
  return __this->valueless_by_exception();
}

static_assert(
    (bool (::std::variant<
           std::variant<unsigned long, long, unsigned int, int, bool>,
           std::variant<unsigned long, long, double>, absl::Span<const char>,
           std::variant<unsigned int, int, float>>::*)() const) &
    ::std::variant<
        std::variant<unsigned long, long, unsigned int, int, bool>,
        std::variant<unsigned long, long, double>, absl::Span<const char>,
        std::variant<unsigned int, int, float>>::valueless_by_exception);

extern "C" size_t
__rust_thunk__5531e42b__ZNKSt3__u7variantIJNS0_IJmljibEEENS0_IJmldEEEN4absl4SpanIKcEENS0_IJjifEEEEE5indexEv(
    class std::variant<
        std::variant<unsigned long, long, unsigned int, int, bool>,
        std::variant<unsigned long, long, double>, absl::Span<const char>,
        std::variant<unsigned int, int, float>> const* __this) {
  return __this->index();
}

static_assert(
    (size_t (::std::variant<
             std::variant<unsigned long, long, unsigned int, int, bool>,
             std::variant<unsigned long, long, double>, absl::Span<const char>,
             std::variant<unsigned int, int, float>>::*)() const) &
    ::std::variant<std::variant<unsigned long, long, unsigned int, int, bool>,
                   std::variant<unsigned long, long, double>,
                   absl::Span<const char>,
                   std::variant<unsigned int, int, float>>::index);

static_assert(CRUBIT_SIZEOF(class std::variant<unsigned int, int, float>) == 8);
static_assert(alignof(class std::variant<unsigned int, int, float>) == 4);

extern "C" bool
__rust_thunk__0f615caa__ZNKSt3__u7variantIJjifEE22valueless_by_exceptionEv(
    class std::variant<unsigned int, int, float> const* __this) {
  return __this->valueless_by_exception();
}

static_assert((bool (::std::variant<unsigned int, int, float>::*)() const) &
              ::std::variant<unsigned int, int, float>::valueless_by_exception);

extern "C" size_t __rust_thunk__5531e42b__ZNKSt3__u7variantIJjifEE5indexEv(
    class std::variant<unsigned int, int, float> const* __this) {
  return __this->index();
}

static_assert((size_t (::std::variant<unsigned int, int, float>::*)() const) &
              ::std::variant<unsigned int, int, float>::index);

static_assert(CRUBIT_SIZEOF(class std::variant<unsigned long, long, double>) ==
              16);
static_assert(alignof(class std::variant<unsigned long, long, double>) == 8);

extern "C" bool
__rust_thunk__0f615caa__ZNKSt3__u7variantIJmldEE22valueless_by_exceptionEv(
    class std::variant<unsigned long, long, double> const* __this) {
  return __this->valueless_by_exception();
}

static_assert(
    (bool (::std::variant<unsigned long, long, double>::*)() const) &
    ::std::variant<unsigned long, long, double>::valueless_by_exception);

extern "C" size_t __rust_thunk__5531e42b__ZNKSt3__u7variantIJmldEE5indexEv(
    class std::variant<unsigned long, long, double> const* __this) {
  return __this->index();
}

static_assert((size_t (::std::variant<unsigned long, long, double>::*)()
                   const) &
              ::std::variant<unsigned long, long, double>::index);

static_assert(CRUBIT_SIZEOF(class std::variant<unsigned long, long,
                                               unsigned int, int, bool>) == 16);
static_assert(
    alignof(class std::variant<unsigned long, long, unsigned int, int, bool>) ==
    8);

extern "C" bool
__rust_thunk__0f615caa__ZNKSt3__u7variantIJmljibEE22valueless_by_exceptionEv(
    class std::variant<unsigned long, long, unsigned int, int, bool> const*
        __this) {
  return __this->valueless_by_exception();
}

static_assert(
    (bool (::std::variant<unsigned long, long, unsigned int, int, bool>::*)()
         const) &
    ::std::variant<unsigned long, long, unsigned int, int,
                   bool>::valueless_by_exception);

extern "C" size_t __rust_thunk__5531e42b__ZNKSt3__u7variantIJmljibEE5indexEv(
    class std::variant<unsigned long, long, unsigned int, int, bool> const*
        __this) {
  return __this->index();
}

static_assert(
    (size_t (::std::variant<unsigned long, long, unsigned int, int, bool>::*)()
         const) &
    ::std::variant<unsigned long, long, unsigned int, int, bool>::index);

static_assert(CRUBIT_SIZEOF(class std::back_insert_iterator<
                            std::__format::__output_buffer<char>>) == 8);
static_assert(alignof(class std::back_insert_iterator<
                      std::__format::__output_buffer<char>>) == 8);

extern "C" void
__rust_thunk__19249dfe__ZNSt3__u20back_insert_iteratorINS_8__format15__output_bufferIcEEEC1ERS3_(
    class std::back_insert_iterator<std::__format::__output_buffer<char>>*
        __this,
    class std::__format::__output_buffer<char>* __x) {
  crubit::construct_at(__this, *__x);
}

extern "C" class std::back_insert_iterator<
    std::__format::__output_buffer<char>>*
__rust_thunk__dfcf8768__ZNSt3__u20back_insert_iteratorINS_8__format15__output_bufferIcEEEaSERKc(
    class std::back_insert_iterator<std::__format::__output_buffer<char>>*
        __this,
    char const* __value) {
  return std::addressof(__this->operator=(*__value));
}

static_assert(
    (class std::back_insert_iterator<std::__format::__output_buffer<char>> &
     (::std::back_insert_iterator<std::__format::__output_buffer<char>>::*)(
         char const&)) &
    ::std::back_insert_iterator<
        std::__format::__output_buffer<char>>::operator=);

extern "C" class std::back_insert_iterator<
    std::__format::__output_buffer<char>>*
__rust_thunk__141882f4__ZNSt3__u20back_insert_iteratorINS_8__format15__output_bufferIcEEEaSEOc(
    class std::back_insert_iterator<std::__format::__output_buffer<char>>*
        __this,
    char* __value) {
  return std::addressof(__this->operator=(std::move(*__value)));
}

static_assert(
    (class std::back_insert_iterator<std::__format::__output_buffer<char>> &
     (::std::back_insert_iterator<std::__format::__output_buffer<char>>::*)(
         char&&)) &
    ::std::back_insert_iterator<
        std::__format::__output_buffer<char>>::operator=);

static_assert(CRUBIT_SIZEOF(class std::back_insert_iterator<
                            std::__format::__output_buffer<wchar_t>>) == 8);
static_assert(alignof(class std::back_insert_iterator<
                      std::__format::__output_buffer<wchar_t>>) == 8);

extern "C" void
__rust_thunk__19249dfe__ZNSt3__u20back_insert_iteratorINS_8__format15__output_bufferIwEEEC1ERS3_(
    class std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>*
        __this,
    class std::__format::__output_buffer<wchar_t>* __x) {
  crubit::construct_at(__this, *__x);
}

static_assert(sizeof(struct std::ranges::views::__elements::__fn<0UL>) == 1);
static_assert(alignof(struct std::ranges::views::__elements::__fn<0UL>) == 1);

extern "C" void
__rust_thunk__43f1c07c__ZNSt3__u6ranges5views10__elements4__fnILm0EEC1Ev(
    struct std::ranges::views::__elements::__fn<0UL>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::ranges::views::__elements::__fn<1UL>) == 1);
static_assert(alignof(struct std::ranges::views::__elements::__fn<1UL>) == 1);

extern "C" void
__rust_thunk__43f1c07c__ZNSt3__u6ranges5views10__elements4__fnILm1EEC1Ev(
    struct std::ranges::views::__elements::__fn<1UL>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::__format::__output_buffer<char>) == 40);
static_assert(alignof(class std::__format::__output_buffer<char>) == 8);

extern "C" void
__rust_thunk__f09909b6__ZNSt3__u8__format15__output_bufferIcEC1EPcmPFvRS2_mE(
    class std::__format::__output_buffer<char>* __this, char* __ptr,
    size_t __capacity,
    crubit::type_identity_t<void(class std::__format::__output_buffer<char>&,
                                 size_t)>* __function) {
  crubit::construct_at(__this, __ptr, __capacity, __function);
}

static_assert(CRUBIT_SIZEOF(class std::__format::__output_buffer<wchar_t>) ==
              40);
static_assert(alignof(class std::__format::__output_buffer<wchar_t>) == 8);

static_assert(CRUBIT_SIZEOF(class std::basic_format_parse_context<char>) == 40);
static_assert(alignof(class std::basic_format_parse_context<char>) == 8);

extern "C" void
__rust_thunk__55e36245__ZNSt3__u26basic_format_parse_contextIcEC1ENS_17basic_string_viewIcNS_11char_traitsIcEEEEm(
    class std::basic_format_parse_context<char>* __this,
    ::std::__u::string_view* __fmt, size_t __num_args) {
  crubit::construct_at(__this, std::move(*__fmt), __num_args);
}

extern "C" char const*
__rust_thunk__ee14e5b2__ZNKSt3__u26basic_format_parse_contextIcE5beginEv(
    class std::basic_format_parse_context<char> const* __this) {
  return __this->begin();
}

static_assert((char const* (::std::basic_format_parse_context<char>::*)()
                   const) &
              ::std::basic_format_parse_context<char>::begin);

extern "C" char const*
__rust_thunk__b3c10e3c__ZNKSt3__u26basic_format_parse_contextIcE3endEv(
    class std::basic_format_parse_context<char> const* __this) {
  return __this->end();
}

static_assert((char const* (::std::basic_format_parse_context<char>::*)()
                   const) &
              ::std::basic_format_parse_context<char>::end);

extern "C" void
__rust_thunk__554fab17__ZNSt3__u26basic_format_parse_contextIcE10advance_toEPKc(
    class std::basic_format_parse_context<char>* __this, char const* __it) {
  __this->advance_to(__it);
}

static_assert((void (::std::basic_format_parse_context<char>::*)(char const*)) &
              ::std::basic_format_parse_context<char>::advance_to);

extern "C" size_t
__rust_thunk__fcaf6909__ZNSt3__u26basic_format_parse_contextIcE11next_arg_idEv(
    class std::basic_format_parse_context<char>* __this) {
  return __this->next_arg_id();
}

static_assert((size_t (::std::basic_format_parse_context<char>::*)()) &
              ::std::basic_format_parse_context<char>::next_arg_id);

extern "C" void
__rust_thunk__50b38b67__ZNSt3__u26basic_format_parse_contextIcE12check_arg_idEm(
    class std::basic_format_parse_context<char>* __this, size_t __id) {
  __this->check_arg_id(__id);
}

static_assert((void (::std::basic_format_parse_context<char>::*)(size_t)) &
              ::std::basic_format_parse_context<char>::check_arg_id);

static_assert(CRUBIT_SIZEOF(class std::basic_format_parse_context<wchar_t>) ==
              40);
static_assert(alignof(class std::basic_format_parse_context<wchar_t>) == 8);

extern "C" void
__rust_thunk__55e36245__ZNSt3__u26basic_format_parse_contextIwEC1ENS_17basic_string_viewIwNS_11char_traitsIwEEEEm(
    class std::basic_format_parse_context<wchar_t>* __this,
    ::std::__u::wstring_view* __fmt, size_t __num_args) {
  crubit::construct_at(__this, std::move(*__fmt), __num_args);
}

extern "C" size_t
__rust_thunk__fcaf6909__ZNSt3__u26basic_format_parse_contextIwE11next_arg_idEv(
    class std::basic_format_parse_context<wchar_t>* __this) {
  return __this->next_arg_id();
}

static_assert((size_t (::std::basic_format_parse_context<wchar_t>::*)()) &
              ::std::basic_format_parse_context<wchar_t>::next_arg_id);

extern "C" void
__rust_thunk__50b38b67__ZNSt3__u26basic_format_parse_contextIwE12check_arg_idEm(
    class std::basic_format_parse_context<wchar_t>* __this, size_t __id) {
  __this->check_arg_id(__id);
}

static_assert((void (::std::basic_format_parse_context<wchar_t>::*)(size_t)) &
              ::std::basic_format_parse_context<wchar_t>::check_arg_id);

static_assert(
    CRUBIT_SIZEOF(
        class std::__basic_format_arg_value<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>>) == 16);
static_assert(
    alignof(class std::__basic_format_arg_value<std::basic_format_context<
                std::back_insert_iterator<std::__format::__output_buffer<char>>,
                char>>) == 16);

extern "C" void
__rust_thunk__d5b41767__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Ev(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__099741e8__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Eb(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    bool __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__e9f59c6f__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Ec(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    char __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__d8e86d49__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Ei(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    int __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__2c7ba9bf__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Ej(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    unsigned int __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__1d6dd3c3__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Ex(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    long long __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__0636eb76__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Ey(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    unsigned long long __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__94bbb0b4__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1En(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    __int128 __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__e18b1bae__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Eo(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    unsigned __int128 __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__cf7f1ea7__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Ef(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    float __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__03ab6cfe__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Ed(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    double __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__533cf185__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1EPKc(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    char const* __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__331a72a1__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1ENS_17basic_string_viewIcNS_11char_traitsIcEEEE(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    ::std::__u::string_view* __value) {
  crubit::construct_at(__this, std::move(*__value));
}

extern "C" void
__rust_thunk__5cd2447d__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1EPKv(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this,
    void const* __value) {
  crubit::construct_at(__this, __value);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::__basic_format_arg_value<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>>) == 16);
static_assert(
    alignof(
        class std::__basic_format_arg_value<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>>) == 16);

extern "C" void
__rust_thunk__d5b41767__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Ev(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__099741e8__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Eb(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    bool __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__d8e86d49__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Ei(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    int __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__2c7ba9bf__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Ej(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    unsigned int __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__1d6dd3c3__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Ex(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    long long __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__0636eb76__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Ey(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    unsigned long long __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__94bbb0b4__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1En(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    __int128 __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__e18b1bae__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Eo(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    unsigned __int128 __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__cf7f1ea7__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Ef(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    float __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__03ab6cfe__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Ed(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    double __value) {
  crubit::construct_at(__this, __value);
}

extern "C" void
__rust_thunk__331a72a1__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1ENS_17basic_string_viewIwNS_11char_traitsIwEEEE(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    ::std::__u::wstring_view* __value) {
  crubit::construct_at(__this, std::move(*__value));
}

extern "C" void
__rust_thunk__5cd2447d__ZNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1EPKv(
    class std::__basic_format_arg_value<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this,
    void const* __value) {
  crubit::construct_at(__this, __value);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_format_arg<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>>) == 32);
static_assert(
    alignof(class std::basic_format_arg<std::basic_format_context<
                std::back_insert_iterator<std::__format::__output_buffer<char>>,
                char>>) == 16);
static_assert(
    CRUBIT_OFFSET_OF(
        __value_,
        class std::basic_format_arg<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>>) == 0);
static_assert(
    CRUBIT_OFFSET_OF(
        __type_,
        class std::basic_format_arg<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>>) == 16);

extern "C" void
__rust_thunk__ebdd93c9__ZNSt3__u16basic_format_argINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEC1Ev(
    class std::basic_format_arg<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_format_arg<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>>) == 32);
static_assert(
    alignof(
        class std::basic_format_arg<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>>) == 16);
static_assert(
    CRUBIT_OFFSET_OF(
        __value_,
        class std::basic_format_arg<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>>) == 0);
static_assert(
    CRUBIT_OFFSET_OF(
        __type_,
        class std::basic_format_arg<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>>) == 16);

extern "C" void
__rust_thunk__ebdd93c9__ZNSt3__u16basic_format_argINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEC1Ev(
    class std::basic_format_arg<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_format_args<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>>) == 24);
static_assert(
    alignof(class std::basic_format_args<std::basic_format_context<
                std::back_insert_iterator<std::__format::__output_buffer<char>>,
                char>>) == 8);

extern "C" void
__rust_thunk__92f04904__ZNKSt3__u17basic_format_argsINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEE3getEm(
    class std::basic_format_arg<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>*
        __return,
    class std::basic_format_args<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>,
        char>> const* __this,
    size_t __id) {
  new (__return) auto(__this->get(__id));
}

static_assert(
    (class std::basic_format_arg<std::basic_format_context<
         std::back_insert_iterator<std::__format::__output_buffer<char>>,
         char>> (
        ::std::basic_format_args<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<char>>,
            char>>::*)(size_t) const) &
    ::std::basic_format_args<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<char>>,
        char>>::get);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_format_args<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>>) == 24);
static_assert(
    alignof(
        class std::basic_format_args<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>>) == 8);

extern "C" void
__rust_thunk__92f04904__ZNKSt3__u17basic_format_argsINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEE3getEm(
    class std::basic_format_arg<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>* __return,
    class std::basic_format_args<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>> const* __this,
    size_t __id) {
  new (__return) auto(__this->get(__id));
}

static_assert(
    (class std::basic_format_arg<std::basic_format_context<
         std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
         wchar_t>> (
        ::std::basic_format_args<std::basic_format_context<
            std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
            wchar_t>>::*)(size_t) const) &
    ::std::basic_format_args<std::basic_format_context<
        std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>,
        wchar_t>>::get);

static_assert(
    CRUBIT_SIZEOF(class std::__deque_iterator<
                  absl::crc_internal::CrcCordState::PrefixCrc,
                  const absl::crc_internal::CrcCordState::PrefixCrc*,
                  const absl::crc_internal::CrcCordState::PrefixCrc&,
                  const absl::crc_internal::CrcCordState::PrefixCrc* const*,
                  long, 0L>) == 16);
static_assert(alignof(class std::__deque_iterator<
                      absl::crc_internal::CrcCordState::PrefixCrc,
                      const absl::crc_internal::CrcCordState::PrefixCrc*,
                      const absl::crc_internal::CrcCordState::PrefixCrc&,
                      const absl::crc_internal::CrcCordState::PrefixCrc* const*,
                      long, 0L>) == 8);

extern "C" void
__rust_thunk__3bf86844__ZNSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPKS4_RS5_PKS6_lLl0EEC1Ev(
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" class std::__deque_iterator<
    absl::crc_internal::CrcCordState::PrefixCrc,
    const absl::crc_internal::CrcCordState::PrefixCrc*,
    const absl::crc_internal::CrcCordState::PrefixCrc&,
    const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
__rust_thunk__1f5f01a6__ZNSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPKS4_RS5_PKS6_lLl0EEpLEl(
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
        __this,
    long __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert(
    (class std::__deque_iterator<
         absl::crc_internal::CrcCordState::PrefixCrc,
         const absl::crc_internal::CrcCordState::PrefixCrc*,
         const absl::crc_internal::CrcCordState::PrefixCrc&,
         const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L> &
     (::std::__deque_iterator<
         absl::crc_internal::CrcCordState::PrefixCrc,
         const absl::crc_internal::CrcCordState::PrefixCrc*,
         const absl::crc_internal::CrcCordState::PrefixCrc&,
         const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
         0L>::*)(long)) &
    ::std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
        0L>::operator+=);

extern "C" class std::__deque_iterator<
    absl::crc_internal::CrcCordState::PrefixCrc,
    const absl::crc_internal::CrcCordState::PrefixCrc*,
    const absl::crc_internal::CrcCordState::PrefixCrc&,
    const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
__rust_thunk__ed62cab2__ZNSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPKS4_RS5_PKS6_lLl0EEmIEl(
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
        __this,
    long __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert(
    (class std::__deque_iterator<
         absl::crc_internal::CrcCordState::PrefixCrc,
         const absl::crc_internal::CrcCordState::PrefixCrc*,
         const absl::crc_internal::CrcCordState::PrefixCrc&,
         const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L> &
     (::std::__deque_iterator<
         absl::crc_internal::CrcCordState::PrefixCrc,
         const absl::crc_internal::CrcCordState::PrefixCrc*,
         const absl::crc_internal::CrcCordState::PrefixCrc&,
         const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
         0L>::*)(long)) &
    ::std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
        0L>::operator-=);

extern "C" void
__rust_thunk__5575fecc__ZNKSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPKS4_RS5_PKS6_lLl0EEplEl(
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
        __return,
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
        0L> const* __this,
    long __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert(
    (class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L> (
        ::std::__deque_iterator<
            absl::crc_internal::CrcCordState::PrefixCrc,
            const absl::crc_internal::CrcCordState::PrefixCrc*,
            const absl::crc_internal::CrcCordState::PrefixCrc&,
            const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
            0L>::*)(long) const) &
    ::std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
        0L>::operator+);

extern "C" void
__rust_thunk__2251170c__ZNKSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPKS4_RS5_PKS6_lLl0EEmiEl(
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
        __return,
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
        0L> const* __this,
    long __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert(
    (class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L> (
        ::std::__deque_iterator<
            absl::crc_internal::CrcCordState::PrefixCrc,
            const absl::crc_internal::CrcCordState::PrefixCrc*,
            const absl::crc_internal::CrcCordState::PrefixCrc&,
            const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
            0L>::*)(long) const) &
    ::std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long,
        0L>::operator-);

static_assert(
    CRUBIT_SIZEOF(class std::__deque_iterator<
                  absl::crc_internal::CrcCordState::PrefixCrc,
                  absl::crc_internal::CrcCordState::PrefixCrc*,
                  absl::crc_internal::CrcCordState::PrefixCrc&,
                  absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>) ==
    16);
static_assert(
    alignof(class std::__deque_iterator<
            absl::crc_internal::CrcCordState::PrefixCrc,
            absl::crc_internal::CrcCordState::PrefixCrc*,
            absl::crc_internal::CrcCordState::PrefixCrc&,
            absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>) == 8);

extern "C" void
__rust_thunk__3bf86844__ZNSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPS4_RS4_PS5_lLl0EEC1Ev(
    class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L>* __this) {
  crubit::construct_at(__this);
}

extern "C" class std::__deque_iterator<
    absl::crc_internal::CrcCordState::PrefixCrc,
    absl::crc_internal::CrcCordState::PrefixCrc*,
    absl::crc_internal::CrcCordState::PrefixCrc&,
    absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>*
__rust_thunk__1f5f01a6__ZNSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPS4_RS4_PS5_lLl0EEpLEl(
    class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L>* __this,
    long __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert(
    (class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                 absl::crc_internal::CrcCordState::PrefixCrc*,
                                 absl::crc_internal::CrcCordState::PrefixCrc&,
                                 absl::crc_internal::CrcCordState::PrefixCrc**,
                                 long, 0L> &
     (::std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                              absl::crc_internal::CrcCordState::PrefixCrc*,
                              absl::crc_internal::CrcCordState::PrefixCrc&,
                              absl::crc_internal::CrcCordState::PrefixCrc**,
                              long, 0L>::*)(long)) &
    ::std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                            absl::crc_internal::CrcCordState::PrefixCrc*,
                            absl::crc_internal::CrcCordState::PrefixCrc&,
                            absl::crc_internal::CrcCordState::PrefixCrc**, long,
                            0L>::operator+=);

extern "C" class std::__deque_iterator<
    absl::crc_internal::CrcCordState::PrefixCrc,
    absl::crc_internal::CrcCordState::PrefixCrc*,
    absl::crc_internal::CrcCordState::PrefixCrc&,
    absl::crc_internal::CrcCordState::PrefixCrc**, long, 0L>*
__rust_thunk__ed62cab2__ZNSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPS4_RS4_PS5_lLl0EEmIEl(
    class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L>* __this,
    long __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert(
    (class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                 absl::crc_internal::CrcCordState::PrefixCrc*,
                                 absl::crc_internal::CrcCordState::PrefixCrc&,
                                 absl::crc_internal::CrcCordState::PrefixCrc**,
                                 long, 0L> &
     (::std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                              absl::crc_internal::CrcCordState::PrefixCrc*,
                              absl::crc_internal::CrcCordState::PrefixCrc&,
                              absl::crc_internal::CrcCordState::PrefixCrc**,
                              long, 0L>::*)(long)) &
    ::std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                            absl::crc_internal::CrcCordState::PrefixCrc*,
                            absl::crc_internal::CrcCordState::PrefixCrc&,
                            absl::crc_internal::CrcCordState::PrefixCrc**, long,
                            0L>::operator-=);

extern "C" void
__rust_thunk__5575fecc__ZNKSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPS4_RS4_PS5_lLl0EEplEl(
    class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L>* __return,
    class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L> const* __this,
    long __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert(
    (class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                 absl::crc_internal::CrcCordState::PrefixCrc*,
                                 absl::crc_internal::CrcCordState::PrefixCrc&,
                                 absl::crc_internal::CrcCordState::PrefixCrc**,
                                 long, 0L> (
        ::std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L>::*)(long) const) &
    ::std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                            absl::crc_internal::CrcCordState::PrefixCrc*,
                            absl::crc_internal::CrcCordState::PrefixCrc&,
                            absl::crc_internal::CrcCordState::PrefixCrc**, long,
                            0L>::operator+);

extern "C" void
__rust_thunk__2251170c__ZNKSt3__u16__deque_iteratorIN4absl12crc_internal12CrcCordState9PrefixCrcEPS4_RS4_PS5_lLl0EEmiEl(
    class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L>* __return,
    class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L> const* __this,
    long __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert(
    (class std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                 absl::crc_internal::CrcCordState::PrefixCrc*,
                                 absl::crc_internal::CrcCordState::PrefixCrc&,
                                 absl::crc_internal::CrcCordState::PrefixCrc**,
                                 long, 0L> (
        ::std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                                absl::crc_internal::CrcCordState::PrefixCrc*,
                                absl::crc_internal::CrcCordState::PrefixCrc&,
                                absl::crc_internal::CrcCordState::PrefixCrc**,
                                long, 0L>::*)(long) const) &
    ::std::__deque_iterator<absl::crc_internal::CrcCordState::PrefixCrc,
                            absl::crc_internal::CrcCordState::PrefixCrc*,
                            absl::crc_internal::CrcCordState::PrefixCrc&,
                            absl::crc_internal::CrcCordState::PrefixCrc**, long,
                            0L>::operator-);

static_assert(
    CRUBIT_SIZEOF(
        class std::deque<
            absl::crc_internal::CrcCordState::PrefixCrc,
            std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>) ==
    48);
static_assert(
    alignof(class std::deque<
            absl::crc_internal::CrcCordState::PrefixCrc,
            std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>) == 8);

extern "C" void
__rust_thunk__21ddddd2__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1Ev(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__0398fd18__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEED1Ev(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__216e5f72__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1ERKS6_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc> const*
        __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" class std::deque<
    absl::crc_internal::CrcCordState::PrefixCrc,
    std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>*
__rust_thunk__960c9bd9__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEaSESt16initializer_listIS4_E(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::initializer_list<absl::crc_internal::CrcCordState::PrefixCrc>*
        __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert(
    (class std::deque<
         absl::crc_internal::CrcCordState::PrefixCrc,
         std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> &
     (::std::deque<
         absl::crc_internal::CrcCordState::PrefixCrc,
         std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)(
         class std::initializer_list<
             absl::crc_internal::CrcCordState::PrefixCrc>)) &
    ::std::deque<absl::crc_internal::CrcCordState::PrefixCrc,
                 std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::
    operator=);

extern "C" void
__rust_thunk__06a1a864__ZNKSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE6cbeginEv(
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
        __return,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __this) {
  new (__return) auto(__this->cbegin());
}

static_assert(
    (class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L> (
        ::std::deque<
            absl::crc_internal::CrcCordState::PrefixCrc,
            std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()
         const) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::cbegin);

extern "C" void
__rust_thunk__58b84732__ZNKSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE4cendEv(
    class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>*
        __return,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __this) {
  new (__return) auto(__this->cend());
}

static_assert(
    (class std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L> (
        ::std::deque<
            absl::crc_internal::CrcCordState::PrefixCrc,
            std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()
         const) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::cend);

extern "C" void
__rust_thunk__6c91d4ce__ZNKSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE7crbeginEv(
    class std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>>*
        __return,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<std::__deque_iterator<
         absl::crc_internal::CrcCordState::PrefixCrc,
         const absl::crc_internal::CrcCordState::PrefixCrc*,
         const absl::crc_internal::CrcCordState::PrefixCrc&,
         const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>> (
        ::std::deque<
            absl::crc_internal::CrcCordState::PrefixCrc,
            std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()
         const) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::crbegin);

extern "C" void
__rust_thunk__206894cc__ZNKSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE5crendEv(
    class std::reverse_iterator<std::__deque_iterator<
        absl::crc_internal::CrcCordState::PrefixCrc,
        const absl::crc_internal::CrcCordState::PrefixCrc*,
        const absl::crc_internal::CrcCordState::PrefixCrc&,
        const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>>*
        __return,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<std::__deque_iterator<
         absl::crc_internal::CrcCordState::PrefixCrc,
         const absl::crc_internal::CrcCordState::PrefixCrc*,
         const absl::crc_internal::CrcCordState::PrefixCrc&,
         const absl::crc_internal::CrcCordState::PrefixCrc* const*, long, 0L>> (
        ::std::deque<
            absl::crc_internal::CrcCordState::PrefixCrc,
            std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()
         const) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::crend);

extern "C" size_t
__rust_thunk__45903bf0__ZNKSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE4sizeEv(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::deque<
             absl::crc_internal::CrcCordState::PrefixCrc,
             std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()
         const) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::size);

extern "C" size_t
__rust_thunk__185bef01__ZNKSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE8max_sizeEv(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __this) {
  return __this->max_size();
}

static_assert(
    (size_t (::std::deque<
             absl::crc_internal::CrcCordState::PrefixCrc,
             std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()
         const) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::max_size);

extern "C" bool
__rust_thunk__93a81268__ZNKSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE5emptyEv(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::deque<
           absl::crc_internal::CrcCordState::PrefixCrc,
           std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()
         const) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::empty);

extern "C" void
__rust_thunk__4096210f__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1Em(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    size_t __n) {
  crubit::construct_at(__this, __n);
}

extern "C" void
__rust_thunk__a24d09ae__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1EmRKS6_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    size_t __n,
    class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc> const*
        __a) {
  crubit::construct_at(__this, __n, *__a);
}

extern "C" void
__rust_thunk__d34b7161__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1ERKS7_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __c) {
  crubit::construct_at(__this, *__c);
}

extern "C" void
__rust_thunk__341d0f00__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1ERKS7_RKS6_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const* __c,
    class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc> const*
        __a) {
  crubit::construct_at(__this, *__c, *__a);
}

extern "C" class std::deque<
    absl::crc_internal::CrcCordState::PrefixCrc,
    std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>*
__rust_thunk__629e1887__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEaSERKS7_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __c) {
  return std::addressof(__this->operator=(*__c));
}

static_assert(
    (class std::deque<
         absl::crc_internal::CrcCordState::PrefixCrc,
         std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> &
     (::std::deque<
         absl::crc_internal::CrcCordState::PrefixCrc,
         std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)(
         class std::deque<absl::crc_internal::CrcCordState::PrefixCrc,
                          std::allocator<absl::crc_internal::CrcCordState::
                                             PrefixCrc>> const&)) &
    ::std::deque<absl::crc_internal::CrcCordState::PrefixCrc,
                 std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::
    operator=);

extern "C" void
__rust_thunk__3ee2138f__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1ESt16initializer_listIS4_E(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::initializer_list<absl::crc_internal::CrcCordState::PrefixCrc>*
        __il) {
  crubit::construct_at(__this, std::move(*__il));
}

extern "C" void
__rust_thunk__abe8dbaa__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1ESt16initializer_listIS4_ERKS6_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::initializer_list<absl::crc_internal::CrcCordState::PrefixCrc>*
        __il,
    class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc> const*
        __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__326a558f__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1EOS7_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __c) {
  crubit::construct_at(__this, std::move(*__c));
}

extern "C" void
__rust_thunk__b8fd8f69__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEC1EOS7_RKS6_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __c,
    class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc> const*
        __a) {
  crubit::construct_at(__this, std::move(*__c), *__a);
}

extern "C" class std::deque<
    absl::crc_internal::CrcCordState::PrefixCrc,
    std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>*
__rust_thunk__5445989e__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEEaSEOS7_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __c) {
  return std::addressof(__this->operator=(std::move(*__c)));
}

static_assert(
    (class std::deque<
         absl::crc_internal::CrcCordState::PrefixCrc,
         std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> &
     (::std::deque<
         absl::crc_internal::CrcCordState::PrefixCrc,
         std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)(
         class std::deque<
             absl::crc_internal::CrcCordState::PrefixCrc,
             std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>&&)) &
    ::std::deque<absl::crc_internal::CrcCordState::PrefixCrc,
                 std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::
    operator=);

extern "C" void
__rust_thunk__7e42ac72__ZNKSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE13get_allocatorEv(
    class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>* __return,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>> const*
        __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert(
    (class std::allocator<absl::crc_internal::CrcCordState::PrefixCrc> (
        ::std::deque<
            absl::crc_internal::CrcCordState::PrefixCrc,
            std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()
         const) &
    ::std::deque<absl::crc_internal::CrcCordState::PrefixCrc,
                 std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::
        get_allocator);

extern "C" void
__rust_thunk__bc2e69fb__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE13shrink_to_fitEv(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this) {
  __this->shrink_to_fit();
}

static_assert(
    (void (::std::deque<
           absl::crc_internal::CrcCordState::PrefixCrc,
           std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()) &
    ::std::deque<absl::crc_internal::CrcCordState::PrefixCrc,
                 std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::
        shrink_to_fit);

extern "C" void
__rust_thunk__5225ebff__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE9pop_frontEv(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this) {
  __this->pop_front();
}

static_assert(
    (void (::std::deque<
           absl::crc_internal::CrcCordState::PrefixCrc,
           std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()) &
    ::std::deque<absl::crc_internal::CrcCordState::PrefixCrc,
                 std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::
        pop_front);

extern "C" void
__rust_thunk__8ee570ab__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE8pop_backEv(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this) {
  __this->pop_back();
}

static_assert(
    (void (::std::deque<
           absl::crc_internal::CrcCordState::PrefixCrc,
           std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::pop_back);

extern "C" void
__rust_thunk__f2d571dd__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE4swapERS7_(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this,
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __c) {
  __this->swap(*__c);
}

static_assert(
    (void (::std::deque<
           absl::crc_internal::CrcCordState::PrefixCrc,
           std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)(
        class std::deque<
            absl::crc_internal::CrcCordState::PrefixCrc,
            std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>&)) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::swap);

extern "C" void
__rust_thunk__d82bfdb3__ZNSt3__u5dequeIN4absl12crc_internal12CrcCordState9PrefixCrcENS_9allocatorIS4_EEE5clearEv(
    class std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>* __this) {
  __this->clear();
}

static_assert(
    (void (::std::deque<
           absl::crc_internal::CrcCordState::PrefixCrc,
           std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::*)()) &
    ::std::deque<
        absl::crc_internal::CrcCordState::PrefixCrc,
        std::allocator<absl::crc_internal::CrcCordState::PrefixCrc>>::clear);

static_assert(
    CRUBIT_SIZEOF(
        class std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>) == 8);
static_assert(
    alignof(class std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>) == 8);

extern "C" void
__rust_thunk__27b3ace6__ZNSt3__u15__tree_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISB_PvEElEC1Ev(
    class std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>) == 8);
static_assert(
    alignof(class std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>) == 8);

extern "C" void
__rust_thunk__da0482fd__ZNSt3__u21__tree_const_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISB_PvEElEC1Ev(
    class std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__42010ae9__ZNSt3__u21__tree_const_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISB_PvEElEC1ENS_15__tree_iteratorISB_SF_lEE(
    class std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>* __this,
    class std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>* __p) {
  crubit::construct_at(__this, std::move(*__p));
}

static_assert(
    CRUBIT_SIZEOF(
        class std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>) == 8);
static_assert(
    alignof(
        class std::__map_iterator<std::__tree_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>) == 8);

extern "C" void
__rust_thunk__830c8f89__ZNSt3__u14__map_iteratorINS_15__tree_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISC_PvEElEEEC1Ev(
    class std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__42c93070__ZNSt3__u14__map_iteratorINS_15__tree_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISC_PvEElEEEC1ESH_(
    class std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __this,
    class std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>* __i) {
  crubit::construct_at(__this, std::move(*__i));
}

static_assert(
    CRUBIT_SIZEOF(
        class std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>) == 8);
static_assert(
    alignof(
        class std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>) == 8);

extern "C" void
__rust_thunk__8b37e083__ZNSt3__u20__map_const_iteratorINS_21__tree_const_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISC_PvEElEEEC1Ev(
    class std::__map_const_iterator<std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__aaf06744__ZNSt3__u20__map_const_iteratorINS_21__tree_const_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISC_PvEElEEEC1ESH_(
    class std::__map_const_iterator<std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __this,
    class std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>* __i) {
  crubit::construct_at(__this, std::move(*__i));
}

extern "C" void
__rust_thunk__736f3317__ZNSt3__u20__map_const_iteratorINS_21__tree_const_iteratorINS_12__value_typeINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyEEEPNS_11__tree_nodeISC_PvEElEEEC1ENS_14__map_iteratorINS_15__tree_iteratorISC_SG_lEEEE(
    class std::__map_const_iterator<std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __this,
    class std::__map_iterator<std::__tree_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __i) {
  crubit::construct_at(__this, std::move(*__i));
}

static_assert(
    CRUBIT_SIZEOF(
        class std::map<std::basic_string<char, std::char_traits<char>,
                                         std::allocator<char>>,
                       tcmalloc::MallocExtension::Property, std::less<void>,
                       std::allocator<std::pair<
                           const std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>>>) == 24);
static_assert(
    alignof(
        class std::map<std::basic_string<char, std::char_traits<char>,
                                         std::allocator<char>>,
                       tcmalloc::MallocExtension::Property, std::less<void>,
                       std::allocator<std::pair<
                           const std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>>>) == 8);

extern "C" void
__rust_thunk__d0ad50ba__ZNSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEEC1Ev(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__e26df825__ZNSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEEC1ERKSB_(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __this,
    struct std::less<void> const* __comp) {
  crubit::construct_at(__this, *__comp);
}

extern "C" void
__rust_thunk__bd0c2744__ZNSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEEC1ERKSB_RKSF_(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __this,
    struct std::less<void> const* __comp,
    class std::allocator<
        std::pair<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                  tcmalloc::MallocExtension::Property>> const* __a) {
  crubit::construct_at(__this, *__comp, *__a);
}

extern "C" void
__rust_thunk__af6dd28b__ZNSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEEC1EOSG_(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __this,
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __m) {
  crubit::construct_at(__this, std::move(*__m));
}

extern "C" class std::map<
    std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
    tcmalloc::MallocExtension::Property, std::less<void>,
    std::allocator<
        std::pair<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                  tcmalloc::MallocExtension::Property>>>*
__rust_thunk__536b1db2__ZNSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEEaSEOSG_(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __this,
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __m) {
  return std::addressof(__this->operator=(std::move(*__m)));
}

static_assert(
    (class std::map<
         std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
         tcmalloc::MallocExtension::Property, std::less<void>,
         std::allocator<
             std::pair<const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>> &
     (::std::map<
         std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
         tcmalloc::MallocExtension::Property, std::less<void>,
         std::allocator<
             std::pair<const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)(
         class std::map<
             std::basic_string<char, std::char_traits<char>,
                               std::allocator<char>>,
             tcmalloc::MallocExtension::Property, std::less<void>,
             std::allocator<
                 std::pair<const std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>>>&&)) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::operator=);

extern "C" void
__rust_thunk__a1e3596c__ZNSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEEC1ERKSF_(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __this,
    class std::allocator<
        std::pair<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                  tcmalloc::MallocExtension::Property>> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__70f9648f__ZNSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEED1Ev(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __this) {
  std::destroy_at(__this);
}

extern "C" void
__rust_thunk__92f1b816__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE6cbeginEv(
    class std::__map_const_iterator<std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __return,
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this) {
  new (__return) auto(__this->cbegin());
}

static_assert(
    (class std::__map_const_iterator<std::__tree_const_iterator<
         std::__value_type<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>,
         std::__tree_node<
             std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                               tcmalloc::MallocExtension::Property>,
             void*>*,
         long>> (
        ::std::map<std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>,
                   tcmalloc::MallocExtension::Property, std::less<void>,
                   std::allocator<std::pair<
                       const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)() const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::cbegin);

extern "C" void
__rust_thunk__0e7fe790__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE4cendEv(
    class std::__map_const_iterator<std::__tree_const_iterator<
        std::__value_type<std::basic_string<char, std::char_traits<char>,
                                            std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>,
        std::__tree_node<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            void*>*,
        long>>* __return,
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this) {
  new (__return) auto(__this->cend());
}

static_assert(
    (class std::__map_const_iterator<std::__tree_const_iterator<
         std::__value_type<std::basic_string<char, std::char_traits<char>,
                                             std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>,
         std::__tree_node<
             std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                 std::allocator<char>>,
                               tcmalloc::MallocExtension::Property>,
             void*>*,
         long>> (
        ::std::map<std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>,
                   tcmalloc::MallocExtension::Property, std::less<void>,
                   std::allocator<std::pair<
                       const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)() const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::cend);

extern "C" void
__rust_thunk__53c9712f__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE7crbeginEv(
    class std::reverse_iterator<
        std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>>* __return,
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<
        std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>> (
        ::std::map<std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>,
                   tcmalloc::MallocExtension::Property, std::less<void>,
                   std::allocator<std::pair<
                       const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)() const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::crbegin);

extern "C" void
__rust_thunk__97d3ce35__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE5crendEv(
    class std::reverse_iterator<
        std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>>* __return,
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<
        std::__map_const_iterator<std::__tree_const_iterator<
            std::__value_type<std::basic_string<char, std::char_traits<char>,
                                                std::allocator<char>>,
                              tcmalloc::MallocExtension::Property>,
            std::__tree_node<std::__value_type<
                                 std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                                 tcmalloc::MallocExtension::Property>,
                             void*>*,
            long>>> (
        ::std::map<std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>,
                   tcmalloc::MallocExtension::Property, std::less<void>,
                   std::allocator<std::pair<
                       const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)() const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::crend);

extern "C" bool
__rust_thunk__2a068df0__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE5emptyEv(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::map<std::basic_string<char, std::char_traits<char>,
                                        std::allocator<char>>,
                      tcmalloc::MallocExtension::Property, std::less<void>,
                      std::allocator<std::pair<
                          const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>>::*)() const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::empty);

extern "C" size_t
__rust_thunk__adff7610__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE4sizeEv(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this) {
  return __this->size();
}

static_assert(
    (size_t (
        ::std::map<std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>,
                   tcmalloc::MallocExtension::Property, std::less<void>,
                   std::allocator<std::pair<
                       const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)() const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::size);

extern "C" size_t
__rust_thunk__97f245d4__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE8max_sizeEv(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this) {
  return __this->max_size();
}

static_assert(
    (size_t (
        ::std::map<std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>,
                   tcmalloc::MallocExtension::Property, std::less<void>,
                   std::allocator<std::pair<
                       const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)() const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::max_size);

extern "C" void
__rust_thunk__d5fdcabf__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE13get_allocatorEv(
    class std::allocator<
        std::pair<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                  tcmalloc::MallocExtension::Property>>* __return,
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert(
    (class std::allocator<
        std::pair<const std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>,
                  tcmalloc::MallocExtension::Property>> (
        ::std::map<std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>,
                   tcmalloc::MallocExtension::Property, std::less<void>,
                   std::allocator<std::pair<
                       const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)() const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::get_allocator);

extern "C" void
__rust_thunk__99e8b19d__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE8key_compEv(
    struct std::less<void>* __return,
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this) {
  new (__return) auto(__this->key_comp());
}

static_assert(
    (struct std::less<void> (
        ::std::map<std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>,
                   tcmalloc::MallocExtension::Property, std::less<void>,
                   std::allocator<std::pair<
                       const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)() const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::key_comp);

extern "C" void
__rust_thunk__c7bc8eb1__ZNSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE5clearEv(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __this) {
  __this->clear();
}

static_assert(
    (void (::std::map<std::basic_string<char, std::char_traits<char>,
                                        std::allocator<char>>,
                      tcmalloc::MallocExtension::Property, std::less<void>,
                      std::allocator<std::pair<
                          const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>>::*)()) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::clear);

extern "C" void
__rust_thunk__4c60938d__ZNSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE4swapERSG_(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __this,
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>* __m) {
  __this->swap(*__m);
}

static_assert(
    (void (::std::map<std::basic_string<char, std::char_traits<char>,
                                        std::allocator<char>>,
                      tcmalloc::MallocExtension::Property, std::less<void>,
                      std::allocator<std::pair<
                          const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>>::*)(
        class std::map<std::basic_string<char, std::char_traits<char>,
                                         std::allocator<char>>,
                       tcmalloc::MallocExtension::Property, std::less<void>,
                       std::allocator<std::pair<
                           const std::basic_string<char, std::char_traits<char>,
                                                   std::allocator<char>>,
                           tcmalloc::MallocExtension::Property>>>&)) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::swap);

extern "C" size_t
__rust_thunk__9bd8e3c3__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE5countERSD_(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const* __k) {
  return __this->count(*__k);
}

static_assert(
    (size_t (
        ::std::map<std::basic_string<char, std::char_traits<char>,
                                     std::allocator<char>>,
                   tcmalloc::MallocExtension::Property, std::less<void>,
                   std::allocator<std::pair<
                       const std::basic_string<char, std::char_traits<char>,
                                               std::allocator<char>>,
                       tcmalloc::MallocExtension::Property>>>::*)(
        class std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>> const&) const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::count);

extern "C" bool
__rust_thunk__9acdd65c__ZNKSt3__u3mapINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEEN8tcmalloc15MallocExtension8PropertyENS_4lessIvEENS4_INS_4pairIKS6_S9_EEEEE8containsERSD_(
    class std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>> const* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::allocator<char>> const* __k) {
  return __this->contains(*__k);
}

static_assert(
    (bool (::std::map<std::basic_string<char, std::char_traits<char>,
                                        std::allocator<char>>,
                      tcmalloc::MallocExtension::Property, std::less<void>,
                      std::allocator<std::pair<
                          const std::basic_string<char, std::char_traits<char>,
                                                  std::allocator<char>>,
                          tcmalloc::MallocExtension::Property>>>::*)(
        class std::basic_string<char, std::char_traits<char>,
                                std::allocator<char>> const&) const) &
    ::std::map<
        std::basic_string<char, std::char_traits<char>, std::allocator<char>>,
        tcmalloc::MallocExtension::Property, std::less<void>,
        std::allocator<
            std::pair<const std::basic_string<char, std::char_traits<char>,
                                              std::allocator<char>>,
                      tcmalloc::MallocExtension::Property>>>::contains);

#pragma clang diagnostic pop
