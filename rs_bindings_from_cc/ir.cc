// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

#include <cstdint>
#include <optional>
#include <string>
#include <utility>

#include "common/check.h"
#include "llvm/ADT/APSInt.h"
#include "llvm/ADT/SmallString.h"

namespace crubit {

namespace {
CcType PointerOrReferenceTo(CcType pointee_type, PointerTypeKind pointer_kind,
                            std::optional<LifetimeId> lifetime) {
  CcType result;
  auto* p = result.mutable_variant()->mutable_pointer();
  p->set_kind(pointer_kind);
  if (lifetime.has_value()) {
    p->set_lifetime(lifetime->value());
  }
  *p->mutable_pointee_type() = std::move(pointee_type);
  return result;
}
}  // namespace

llvm::StringRef PrimitiveSpelling(Primitive primitive) {
  switch (primitive) {
    case Primitive::BOOL:
      return "bool";
    case Primitive::VOID:
      return "void";
    case Primitive::FLOAT:
      return "float";
    case Primitive::DOUBLE:
      return "double";
    case Primitive::CHAR:
      return "char";
    case Primitive::SIGNED_CHAR:
      return "signed char";
    case Primitive::UNSIGNED_CHAR:
      return "unsigned char";
    case Primitive::SHORT:
      return "short";
    case Primitive::INT:
      return "int";
    case Primitive::LONG:
      return "long";
    case Primitive::LONG_LONG:
      return "long long";
    case Primitive::INT128:
      return "__int128";
    case Primitive::UNSIGNED_SHORT:
      return "unsigned short";
    case Primitive::UNSIGNED_INT:
      return "unsigned int";
    case Primitive::UNSIGNED_LONG:
      return "unsigned long";
    case Primitive::UNSIGNED_LONG_LONG:
      return "unsigned long long";
    case Primitive::UNSIGNED_INT128:
      return "unsigned __int128";
    case Primitive::CHAR16_T:
      return "char16_t";
    case Primitive::CHAR32_T:
      return "char32_t";
    case Primitive::PTRDIFF_T:
      return "ptrdiff_t";
    case Primitive::INTPTR_T:
      return "intptr_t";
    case Primitive::SIZE_T:
      return "size_t";
    case Primitive::UINTPTR_T:
      return "uintptr_t";
    case Primitive::STD_PTRDIFF_T:
      return "std::ptrdiff_t";
    case Primitive::STD_INTPTR_T:
      return "std::intptr_t";
    case Primitive::STD_SIZE_T:
      return "std::size_t";
    case Primitive::STD_UINTPTR_T:
      return "std::uintptr_t";
    case Primitive::INT8_T:
      return "int8_t";
    case Primitive::INT16_T:
      return "int16_t";
    case Primitive::INT32_T:
      return "int32_t";
    case Primitive::INT64_T:
      return "int64_t";
    case Primitive::STD_INT8_T:
      return "std::int8_t";
    case Primitive::STD_INT16_T:
      return "std::int16_t";
    case Primitive::STD_INT32_T:
      return "std::int32_t";
    case Primitive::STD_INT64_T:
      return "std::int64_t";
    case Primitive::UINT8_T:
      return "uint8_t";
    case Primitive::UINT16_T:
      return "uint16_t";
    case Primitive::UINT32_T:
      return "uint32_t";
    case Primitive::UINT64_T:
      return "uint64_t";
    case Primitive::STD_UINT8_T:
      return "std::uint8_t";
    case Primitive::STD_UINT16_T:
      return "std::uint16_t";
    case Primitive::STD_UINT32_T:
      return "std::uint32_t";
    case Primitive::STD_UINT64_T:
      return "std::uint64_t";
  }
  return "";
}

CcType CcTypeDecl(ItemId id) {
  CcType result;
  result.mutable_variant()->set_decl(id.value());
  return result;
}

CcType PointerTo(CcType pointee_type, std::optional<LifetimeId> lifetime,
                 bool nullable) {
  return PointerOrReferenceTo(
      std::move(pointee_type),
      nullable ? PointerTypeKind::NULLABLE : PointerTypeKind::NON_NULL,
      lifetime);
}

CcType OwnedPointerTo(CcType pointee_type, std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type), PointerTypeKind::OWNED,
                              lifetime);
}

CcType LValueReferenceTo(CcType pointee_type,
                         std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type),
                              PointerTypeKind::L_VALUE_REF, lifetime);
}

CcType RValueReferenceTo(CcType pointee_type,
                         std::optional<LifetimeId> lifetime) {
  return PointerOrReferenceTo(std::move(pointee_type),
                              PointerTypeKind::R_VALUE_REF, lifetime);
}

llvm::Expected<IntegerConstant> IntegerConstantFromAPValue(
    const llvm::APSInt& value) {
  if (value.getSignificantBits() > 64) {
    llvm::SmallString<128> value_str;
    value.toString(value_str);
    return crubit::InvalidArgumentError(
        llvm::Twine("value is too large to fit in 64 bits: ") +
        value_str.str());
  }
  IntegerConstant result;
  result.set_is_negative(value < 0);
  uint64_t wrapped_value = value.isSigned()
                               ? static_cast<uint64_t>(value.getSExtValue())
                               : value.getZExtValue();
  result.set_wrapped_value(wrapped_value);
  return result;
}

UnqualifiedIdentifier& TranslatedUnqualifiedIdentifier::rs_identifier() {
  if (crubit_rust_name) {
    return *crubit_rust_name;
  }
  return cc_identifier;
}

Identifier& TranslatedIdentifier::rs_identifier() {
  if (crubit_rust_name) {
    return *crubit_rust_name;
  }
  return cc_identifier;
}

FormattedError FormattedError::FromStatus(llvm::Error error) {
  std::string fmt = (llvm::Twine("(unannotated `") +
                     StatusCodeToString(GetStatusCode(error)) + "` status)")
                        .str();
  std::string msg = GetStatusMessage(error);
  llvm::consumeError(std::move(error));
  return FormattedError(fmt, std::move(msg));
}

}  // namespace crubit
