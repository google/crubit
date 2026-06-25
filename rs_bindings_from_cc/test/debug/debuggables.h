// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DEBUG_DEBUGGABLES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DEBUG_DEBUGGABLES_H_

#include "absl/base/no_destructor.h"
#include "absl/base/nullability.h"
#include "rs_bindings_from_cc/test/debug/existing_rust_debuggable.h"
#include "support/annotations.h"

struct CRUBIT_MUST_BIND Debuggable {};

using AliasedDebuggable CRUBIT_MUST_BIND = Debuggable;

struct CRUBIT_MUST_BIND CRUBIT_OVERRIDE_DEBUG(false) OptOut {};

enum class CRUBIT_MUST_BIND Enum { kOne = 1 };

union CRUBIT_MUST_BIND Union {
  int i;
  double d;
};

struct CRUBIT_MUST_BIND Exhaustive {
  int primitive = 0;
  const void* absl_nullable pointer = nullptr;
  void (*absl_nullable function)() = nullptr;
  Enum an_enum = Enum::kOne;
  Union named_union;
  Debuggable debuggable;
  AliasedDebuggable aliased_debuggable;
  existing_rust_debuggable::ExistingRustDebuggable rust_debuggable;
};

struct CRUBIT_MUST_BIND RustKeywords {
  enum class CRUBIT_MUST_BIND Type { kCereal, kFiber };
  Type type = Type::kCereal;
  enum class CRUBIT_MUST_BIND Use { kToEat, kToWear, kToSleep };
  Use use = Use::kToEat;
  double yield = 0;
};

struct CRUBIT_MUST_BIND HasNonDebuggable {
  Debuggable debuggable;
  OptOut non_debuggable;
};

struct CRUBIT_MUST_BIND HasAnonymousUnion {
  enum class CRUBIT_MUST_BIND Tag { kUninit, kInt, kDouble };
  Tag tag = Tag::kUninit;
  union {
    int i;
    double d;
  };
};

class CRUBIT_MUST_BIND Abstract {
 public:
  virtual ~Abstract() = 0;

 protected:
  Abstract() = default;
  Abstract(const Abstract&) = default;
  Abstract& operator=(const Abstract&) = default;
};

inline Abstract::~Abstract() = default;

class CRUBIT_MUST_BIND CRUBIT_OVERRIDE_DEBUG(true) AbstractOptIn {
 public:
  virtual ~AbstractOptIn() = 0;

 protected:
  AbstractOptIn() = default;
  AbstractOptIn(const AbstractOptIn&) = default;
  AbstractOptIn& operator=(const AbstractOptIn&) = default;
};

inline AbstractOptIn::~AbstractOptIn() = default;

CRUBIT_MUST_BIND
inline const AbstractOptIn& abstract_opt_in_instance() {
  class Concrete : public AbstractOptIn {
   public:
    Concrete() = default;
    ~Concrete() override = default;
  };
  static const absl::NoDestructor<Concrete> kConcrete;
  return *kConcrete;
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DEBUG_DEBUGGABLES_H_
