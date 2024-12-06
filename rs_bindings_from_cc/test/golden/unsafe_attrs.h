// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSAFE_ATTRS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSAFE_ATTRS_H_

static constexpr bool ReturnsTrue() { return true; }
static constexpr bool ReturnsFalse() { return false; }

void TotallySafe();
void TotallyUnsafe(void*);

[[clang::annotate("crubit_override_unsafe", ReturnsTrue())]]
void SafeSignatureButAnnotatedUnsafe();

[[clang::annotate("crubit_override_unsafe", ReturnsFalse())]]
void SafeSignatureButAnnotatedSafe();

[[clang::annotate("crubit_override_unsafe", ReturnsTrue())]]
void UnsafeSignatureButAnnotatedUnsafe(void*);

[[clang::annotate("crubit_override_unsafe", ReturnsFalse())]]
void UnsafeSignatureButAnnotatedSafe(void*);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSAFE_ATTRS_H_