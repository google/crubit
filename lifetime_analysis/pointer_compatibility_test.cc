// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/pointer_compatibility.h"

#include <functional>

#include "gtest/gtest.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "lifetime_annotations/test/run_on_code.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

using clang::ast_matchers::cxxRecordDecl;
using clang::ast_matchers::enumDecl;
using clang::ast_matchers::hasName;
using clang::ast_matchers::match;
using clang::ast_matchers::selectFirst;

clang::QualType getClassType(llvm::StringRef name,
                             const clang::ASTContext& ast_context) {
  return ast_context.getRecordType(selectFirst<clang::CXXRecordDecl>(
      "class", match(cxxRecordDecl(hasName(name)).bind("class"),
                     const_cast<clang::ASTContext&>(ast_context))));
}

clang::QualType getEnumType(llvm::StringRef name,
                            const clang::ASTContext& ast_context) {
  return ast_context.getEnumType(selectFirst<clang::EnumDecl>(
      "enum", match(enumDecl(hasName(name)).bind("enum"),
                    const_cast<clang::ASTContext&>(ast_context))));
}

bool MayPointTo(clang::QualType pointer_type, clang::QualType object_type,
                const clang::ASTContext& ast_context) {
  return clang::tidy::lifetimes::MayPointTo(
      pointer_type, object_type, const_cast<clang::ASTContext&>(ast_context));
}

TEST(PointerCompatibilityTest, MayPointTo) {
  runOnCodeWithLifetimeHandlers(
      "class Base {};"
      "class Derived : public Base {};"
      "class Unrelated {};"
      "enum SignedEnum {};"
      "enum UnsignedEnum : unsigned {};"
      "enum LongEnum : long {};",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        auto pointer_to = [&ast_context](clang::QualType type) {
          return ast_context.getPointerType(type);
        };

        clang::QualType void_type = ast_context.VoidTy;
        clang::QualType char_type = ast_context.CharTy;
        clang::QualType signed_char_type = ast_context.SignedCharTy;
        clang::QualType unsigned_char_type = ast_context.UnsignedCharTy;
        clang::QualType int_type = ast_context.IntTy;
        clang::QualType unsigned_int_type = ast_context.UnsignedIntTy;
        clang::QualType long_type = ast_context.LongTy;
        clang::QualType bool_type = ast_context.BoolTy;

        clang::QualType base_type = getClassType("Base", ast_context);
        clang::QualType derived_type = getClassType("Derived", ast_context);
        clang::QualType unrelated_type = getClassType("Unrelated", ast_context);
        clang::QualType signed_enum_type =
            getEnumType("SignedEnum", ast_context);
        clang::QualType unsigned_enum_type =
            getEnumType("UnsignedEnum", ast_context);
        clang::QualType long_enum_type = getEnumType("LongEnum", ast_context);

        // Trivial case: A pointer can point to its exact pointee type.
        EXPECT_TRUE(MayPointTo(pointer_to(base_type), base_type, ast_context));

        // void pointers and character pointers may point to anything.
        EXPECT_TRUE(MayPointTo(pointer_to(void_type), base_type, ast_context));
        EXPECT_TRUE(MayPointTo(pointer_to(char_type), base_type, ast_context));
        EXPECT_TRUE(
            MayPointTo(pointer_to(signed_char_type), base_type, ast_context));
        EXPECT_TRUE(
            MayPointTo(pointer_to(unsigned_char_type), base_type, ast_context));

        // But an int pointer may not point at an unrelated type.
        EXPECT_FALSE(MayPointTo(pointer_to(int_type), base_type, ast_context));

        // We also allow a void pointer to be converted back to any other
        // pointer type, but we don't allow the same for character pointers.
        EXPECT_TRUE(MayPointTo(pointer_to(base_type), void_type, ast_context));
        EXPECT_FALSE(MayPointTo(pointer_to(base_type), char_type, ast_context));
        EXPECT_FALSE(
            MayPointTo(pointer_to(base_type), signed_char_type, ast_context));
        EXPECT_FALSE(
            MayPointTo(pointer_to(base_type), unsigned_char_type, ast_context));

        // A signed integer pointer may point to the unsigned variant of the
        // integer type and vice versa.
        EXPECT_TRUE(
            MayPointTo(pointer_to(int_type), unsigned_int_type, ast_context));
        EXPECT_TRUE(
            MayPointTo(pointer_to(unsigned_int_type), int_type, ast_context));

        // An enum pointer may point to any enum that has the same underlying
        // type or to its underlying type (ignoring signedness in both cases).
        // Signed enum:
        EXPECT_TRUE(MayPointTo(pointer_to(signed_enum_type), signed_enum_type,
                               ast_context));
        EXPECT_TRUE(MayPointTo(pointer_to(signed_enum_type), unsigned_enum_type,
                               ast_context));
        EXPECT_TRUE(
            MayPointTo(pointer_to(signed_enum_type), int_type, ast_context));
        EXPECT_TRUE(MayPointTo(pointer_to(signed_enum_type), unsigned_int_type,
                               ast_context));
        EXPECT_TRUE(
            MayPointTo(pointer_to(int_type), signed_enum_type, ast_context));
        EXPECT_TRUE(MayPointTo(pointer_to(unsigned_int_type), signed_enum_type,
                               ast_context));
        // Unsigned enum:
        EXPECT_TRUE(MayPointTo(pointer_to(unsigned_enum_type), signed_enum_type,
                               ast_context));
        EXPECT_TRUE(MayPointTo(pointer_to(unsigned_enum_type),
                               unsigned_enum_type, ast_context));
        EXPECT_TRUE(
            MayPointTo(pointer_to(unsigned_enum_type), int_type, ast_context));
        EXPECT_TRUE(MayPointTo(pointer_to(unsigned_enum_type),
                               unsigned_int_type, ast_context));
        EXPECT_TRUE(
            MayPointTo(pointer_to(int_type), unsigned_enum_type, ast_context));
        EXPECT_TRUE(MayPointTo(pointer_to(unsigned_int_type),
                               unsigned_enum_type, ast_context));
        // Underlying types of different width are not compatible:
        EXPECT_FALSE(MayPointTo(pointer_to(long_enum_type), signed_enum_type,
                                ast_context));
        EXPECT_FALSE(
            MayPointTo(pointer_to(long_type), signed_enum_type, ast_context));

        // A bool pointer may point to bool. This is a regression test for an
        // assertion failure that we were getting because
        // Type::isUnsignedIntegerType() considers `bool` to be an unsigned
        // integer type.
        EXPECT_TRUE(MayPointTo(pointer_to(bool_type), bool_type, ast_context));

        // But an integer pointer may not point at an integer of a different
        // size.
        EXPECT_FALSE(MayPointTo(pointer_to(long_type), int_type, ast_context));
        EXPECT_FALSE(MayPointTo(pointer_to(int_type), long_type, ast_context));

        // A pointer to a base class may point to an object of the derived
        // class, and vice versa. However, a pointer to a class type may not
        // point to an object of a class unrelated by inheritance.
        EXPECT_TRUE(
            MayPointTo(pointer_to(base_type), derived_type, ast_context));
        EXPECT_TRUE(
            MayPointTo(pointer_to(derived_type), base_type, ast_context));
        EXPECT_FALSE(
            MayPointTo(pointer_to(base_type), unrelated_type, ast_context));

        // A pointer to const may point at a non-const object (unsurprisingly),
        // but we also allow the opposite. IOW, in propagating pointees through
        // a function, we assume the function may cast away const.
        // As a side note, this is consistent with the "strict aliasing" rules,
        // which the pointer's pointee type and the dynamic type of the object
        // to be similar by C++'s definition of similar.
        EXPECT_TRUE(MayPointTo(pointer_to(base_type.withConst()), base_type,
                               ast_context));
        EXPECT_TRUE(MayPointTo(pointer_to(base_type), base_type.withConst(),
                               ast_context));

        // Likewise, a pointer to volatile may point at a non-volatile object
        // and vice versa.
        EXPECT_TRUE(MayPointTo(pointer_to(base_type.withVolatile()), base_type,
                               ast_context));
        EXPECT_TRUE(MayPointTo(pointer_to(base_type), base_type.withVolatile(),
                               ast_context));

        // We also allow points-to relationships that would be disallowed by
        // invariance. The example below is equivalent to the following:
        // int **pp;
        // const int ***ppp = &pp;
        // The code above doesn't compile, but the strict aliasing rules permit
        // this type of aliasing.
        EXPECT_TRUE(
            MayPointTo(pointer_to(pointer_to(pointer_to(int_type.withConst()))),
                       pointer_to(pointer_to(int_type)), ast_context));
      },
      {});
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
