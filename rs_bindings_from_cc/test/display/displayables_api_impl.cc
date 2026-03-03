// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/display:displayables
// Features: fmt, supported

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
#include "rs_bindings_from_cc/test/display/displayables.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct CanAbslStringify) == 16);
static_assert(alignof(struct CanAbslStringify) == 8);
static_assert(CRUBIT_OFFSET_OF(value, struct CanAbslStringify) == 0);

extern "C" void __rust_thunk___ZN16CanAbslStringifyC1Ev(
    struct CanAbslStringify* __this) {
  crubit::construct_at(__this);
}

extern "C" bool
__crubit_fmt__16CanAbslStringify___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
    const struct CanAbslStringify& value,
    ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

static_assert(CRUBIT_SIZEOF(struct CanAbslStringifyByFill) == 16);
static_assert(alignof(struct CanAbslStringifyByFill) == 8);
static_assert(CRUBIT_OFFSET_OF(count, struct CanAbslStringifyByFill) == 0);
static_assert(CRUBIT_OFFSET_OF(ch, struct CanAbslStringifyByFill) == 8);

extern "C" void __rust_thunk___ZN22CanAbslStringifyByFillC1Ev(
    struct CanAbslStringifyByFill* __this) {
  crubit::construct_at(__this);
}

extern "C" bool
__crubit_fmt__22CanAbslStringifyByFill___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
    const struct CanAbslStringifyByFill& value,
    ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

static_assert(CRUBIT_SIZEOF(struct CanAbslStringifyByFormat) == 16);
static_assert(alignof(struct CanAbslStringifyByFormat) == 8);
static_assert(CRUBIT_OFFSET_OF(value, struct CanAbslStringifyByFormat) == 0);

extern "C" void __rust_thunk___ZN24CanAbslStringifyByFormatC1Ev(
    struct CanAbslStringifyByFormat* __this) {
  crubit::construct_at(__this);
}

extern "C" bool
__crubit_fmt__24CanAbslStringifyByFormat___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
    const struct CanAbslStringifyByFormat& value,
    ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

static_assert(CRUBIT_SIZEOF(struct CanOstream) == 16);
static_assert(alignof(struct CanOstream) == 8);
static_assert(CRUBIT_OFFSET_OF(value, struct CanOstream) == 0);

extern "C" void __rust_thunk___ZN10CanOstreamC1Ev(struct CanOstream* __this) {
  crubit::construct_at(__this);
}

extern "C" bool
__crubit_fmt__10CanOstream___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
    const struct CanOstream& value,
    ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

static_assert(CRUBIT_SIZEOF(struct CanAbslStringifyAndOstream) == 32);
static_assert(alignof(struct CanAbslStringifyAndOstream) == 8);
static_assert(CRUBIT_OFFSET_OF(stringify, struct CanAbslStringifyAndOstream) ==
              0);
static_assert(CRUBIT_OFFSET_OF(ostream, struct CanAbslStringifyAndOstream) ==
              16);

extern "C" void __rust_thunk___ZN26CanAbslStringifyAndOstreamC1Ev(
    struct CanAbslStringifyAndOstream* __this) {
  crubit::construct_at(__this);
}

extern "C" bool
__crubit_fmt__26CanAbslStringifyAndOstream___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
    const struct CanAbslStringifyAndOstream& value,
    ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

extern "C" bool
__crubit_fmt__DisplayableEnum___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
    const DisplayableEnum& value,
    ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

static_assert(sizeof(struct NotDisplayable) == 1);
static_assert(alignof(struct NotDisplayable) == 1);

extern "C" void __rust_thunk___ZN14NotDisplayableC1Ev(
    struct NotDisplayable* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct TemplatedStringView) == 16);
static_assert(alignof(struct TemplatedStringView) == 8);

extern "C" void
__rust_thunk___ZN19TemplatedStringViewC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE(
    struct TemplatedStringView* __this, absl::string_view* v) {
  crubit::construct_at(__this, std::move(*v));
}

extern "C" bool
__crubit_fmt__19TemplatedStringView___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
    const struct TemplatedStringView& value,
    ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

static_assert(sizeof(struct TemplatedNotDisplayable) == 1);
static_assert(alignof(struct TemplatedNotDisplayable) == 1);

extern "C" void __rust_thunk___ZN23TemplatedNotDisplayableC1Ev(
    struct TemplatedNotDisplayable* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct DisplayInRust) == 32);
static_assert(alignof(struct DisplayInRust) == 8);
static_assert(CRUBIT_OFFSET_OF(cc_value, struct DisplayInRust) == 0);
static_assert(CRUBIT_OFFSET_OF(rust_value, struct DisplayInRust) == 16);

extern "C" void __rust_thunk___ZN13DisplayInRustC1Ev(
    struct DisplayInRust* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    16);
static_assert(
    alignof(class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    8);

#pragma clang diagnostic pop
