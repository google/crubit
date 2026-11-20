// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_CLANG_COMPAT_MACROS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_CLANG_COMPAT_MACROS_H_

// `LLVM_DEV_DATE_GE(YYYYMMDD)` and `LLVM_DEV_DATE_LT(YYYYMMDD)` let Crubit
// build against LLVM versions from before and after a Clang API change.
// `CRUBIT_LLVM_DEV_DATE` is the UTC commit date (git committer date, not author
// date) of the LLVM version that Crubit is built against.
//
// `LLVM_DEV_DATETIME_GE(YYYYMMDDHHMMSS)` and `LLVM_DEV_DATETIME_LT(...)` also
// compare the UTC commit time (24-hour clock).  Use them only when a date is
// not precise enough, i.e. when a Clang API change and an LLVM version that
// Crubit is built against are from the same UTC day.  They compare with
// `CRUBIT_LLVM_DEV_DATETIME`: the commit date and time as one integer in
// YYYYMMDDHHMMSS format.  It is optional: builds that don't define it use the
// time 000000 (midnight), so for them a guard from later on the same day
// selects the older API.  `#if` computes with integers that have at least 64
// bits (https://eel.is/c++draft/cpp.cond), so 14-digit numbers need no suffix.
//
// The guards reject an argument in the wrong format (e.g. a YYYYMMDD date
// given to `LLVM_DEV_DATETIME_GE`) with a "division by zero" compile error.
//
// Crubit supports building against older LLVM versions (e.g. the LLVM version
// pinned by Chromium), not only the newest one.  So do not remove
// `#if LLVM_DEV_DATE_GE(...)` guards (or their `#else` branches) only because
// the newest LLVM no longer needs them.  If a build compiles the wrong branch,
// then fix the `CRUBIT_LLVM_DEV_DATE` (or `CRUBIT_LLVM_DEV_DATETIME`) value of
// that build instead.

// `CRUBIT_INTERNAL_IS_YYYYMMDD(d)` is 1 if `d` has the YYYYMMDD format (and is
// not before 1970), and 0 otherwise.  `CRUBIT_INTERNAL_IS_YYYYMMDDHHMMSS(dt)`
// does the same check for the YYYYMMDDHHMMSS format.
#define CRUBIT_INTERNAL_IS_YYYYMMDD(d)                           \
  ((d) >= 19700101 && (d) <= 99991231 && (d) / 100 % 100 >= 1 && \
   (d) / 100 % 100 <= 12 && (d) % 100 >= 1 && (d) % 100 <= 31)
#define CRUBIT_INTERNAL_IS_YYYYMMDDHHMMSS(dt)                                 \
  (CRUBIT_INTERNAL_IS_YYYYMMDD((dt) / 1000000) && (dt) / 10000 % 100 <= 23 && \
   (dt) / 100 % 100 <= 59 && (dt) % 100 <= 59)

#if !defined(CRUBIT_LLVM_DEV_DATE)
#error "CRUBIT_LLVM_DEV_DATE must be defined when compiling Crubit."
#elif !CRUBIT_INTERNAL_IS_YYYYMMDD(CRUBIT_LLVM_DEV_DATE)
#error "CRUBIT_LLVM_DEV_DATE must be in YYYYMMDD format."
#endif

#if !defined(CRUBIT_LLVM_DEV_DATETIME)
// `LL` prevents an `int` overflow outside of `#if`.
#define CRUBIT_LLVM_DEV_DATETIME ((CRUBIT_LLVM_DEV_DATE) * 1000000LL)
#elif !CRUBIT_INTERNAL_IS_YYYYMMDDHHMMSS(CRUBIT_LLVM_DEV_DATETIME)
#error "CRUBIT_LLVM_DEV_DATETIME must be in YYYYMMDDHHMMSS format."
#elif (CRUBIT_LLVM_DEV_DATETIME) / 1000000 != (CRUBIT_LLVM_DEV_DATE)
#error "CRUBIT_LLVM_DEV_DATETIME must start with CRUBIT_LLVM_DEV_DATE."
#endif

// Expand to 1 for a valid argument.  For an invalid argument, they divide by
// zero, which is a compile error in `#if`.  (`? 1 : 0` avoids a division by a
// `bool` outside of `#if`.)
#define CRUBIT_INTERNAL_REQUIRE_YYYYMMDD(d) \
  (1 / (CRUBIT_INTERNAL_IS_YYYYMMDD(d) ? 1 : 0))
#define CRUBIT_INTERNAL_REQUIRE_YYYYMMDDHHMMSS(dt) \
  (1 / (CRUBIT_INTERNAL_IS_YYYYMMDDHHMMSS(dt) ? 1 : 0))

#define LLVM_DEV_DATE_GE(date) \
  (CRUBIT_INTERNAL_REQUIRE_YYYYMMDD(date) && (CRUBIT_LLVM_DEV_DATE) >= (date))
#define LLVM_DEV_DATE_LT(date) \
  (CRUBIT_INTERNAL_REQUIRE_YYYYMMDD(date) && (CRUBIT_LLVM_DEV_DATE) < (date))
#define LLVM_DEV_DATETIME_GE(datetime)                 \
  (CRUBIT_INTERNAL_REQUIRE_YYYYMMDDHHMMSS(datetime) && \
   (CRUBIT_LLVM_DEV_DATETIME) >= (datetime))
#define LLVM_DEV_DATETIME_LT(datetime)                 \
  (CRUBIT_INTERNAL_REQUIRE_YYYYMMDDHHMMSS(datetime) && \
   (CRUBIT_LLVM_DEV_DATETIME) < (datetime))

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_CLANG_COMPAT_MACROS_H_
