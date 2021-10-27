// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/strings/substitute.h"
#include "third_party/absl/types/span.h"

namespace rs_bindings_from_cc {
namespace {

using ::testing::AllOf;
using ::testing::AnyOf;
using ::testing::Each;
using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::Pointee;
using ::testing::Property;
using ::testing::SizeIs;
using ::testing::VariantWith;
using ::testing::status::StatusIs;

template <typename T>
UnqualifiedIdentifier GetName(const T& x) {
  return x.identifier;
}
UnqualifiedIdentifier GetName(const Func& x) { return x.name; }

// Matches an IR node that has the given identifier.
MATCHER_P(IdentifierIs, identifier, "") {
  UnqualifiedIdentifier name = GetName(arg);
  const Identifier* actual = std::get_if<Identifier>(&name);
  if (actual == nullptr) {
    *result_listener << "actual name not an identifier.";
    return false;
  }
  if (actual->Ident() == identifier) return true;

  *result_listener << "actual identifier: '" << actual->Ident() << "'";
  return false;
}

// Matches an IR node that has the given doc comment.
MATCHER_P(DocCommentIs, doc_comment, "") {
  if (arg.doc_comment && *arg.doc_comment == doc_comment) return true;

  *result_listener << "actual doc comment: '"
                   << (arg.doc_comment ? *arg.doc_comment : "<none>") << "'";
  return false;
}

// Matches a Func that has the given mangled name.
MATCHER_P(MangledNameIs, mangled_name, "") {
  if (arg.mangled_name == mangled_name) return true;

  *result_listener << "actual mangled name: '" << arg.mangled_name << "'";
  return false;
}

// Matches a Func that has a return type matching `matcher`.
template <typename Matcher>
auto ReturnType(const Matcher& matcher) {
  return testing::Field("return_type", &Func::return_type, matcher);
}

// Matches a Func that has parameters matching `matchers`.
template <typename... Args>
auto ParamsAre(const Args&... matchers) {
  return testing::Field("params", &Func::params, ElementsAre(matchers...));
}

// Matches a Func that is inline.
MATCHER(IsInline, "") { return arg.is_inline; }

// Matches a FuncParam with a type that matches all given matchers.
template <typename... Args>
auto ParamType(const Args&... matchers) {
  return testing::Field("type", &FuncParam::type, AllOf(matchers...));
}

// Matches a RsType or CcType that has the given name.
MATCHER_P(NameIs, name, "") {
  if (arg.name == name) return true;

  *result_listener << "actual name: '" << arg.name << "'";
  return false;
}

// Matches a MappedType with a CcType that matches all given matchers.
template <typename... Args>
auto CcTypeIs(const Args&... matchers) {
  return testing::Field("cc_type", &MappedType::cc_type, AllOf(matchers...));
}

// Matches a MappedType with a RsType that matches all given matchers.
template <typename... Args>
auto RsTypeIs(const Args&... matchers) {
  return testing::Field("rs_type", &MappedType::rs_type, AllOf(matchers...));
}

// Matches an RsType that has type parameters matching `matchers`.
template <typename... Args>
auto RsTypeParamsAre(const Args&... matchers) {
  return testing::Field("type_params", &RsType::type_params,
                        ElementsAre(matchers...));
}

// Matches a CcType that has type parameters matching `matchers`.
template <typename... Args>
auto CcTypeParamsAre(const Args&... matchers) {
  return testing::Field("type_params", &CcType::type_params,
                        ElementsAre(matchers...));
}

auto IsCcInt() { return AllOf(NameIs("int"), CcTypeParamsAre()); }

auto IsRsInt() { return AllOf(NameIs("i32"), RsTypeParamsAre()); }

// Matches a CcType that is a pointer to a type matching `matcher`.
template <typename Matcher>
auto CcPointsTo(const Matcher& matcher) {
  return AllOf(NameIs("*"), CcTypeParamsAre(matcher));
}

// Matches an RsType that is a pointer to a type matching `matcher`.
template <typename Matcher>
auto RsPointsTo(const Matcher& matcher) {
  return AllOf(NameIs("*mut"), RsTypeParamsAre(matcher));
}

// Matches a MappedType that is void.
MATCHER(IsVoid, "") { return arg.IsVoid(); }

// Matches a MappedType that is an integer.
auto IsInt() { return AllOf(CcTypeIs(IsCcInt()), RsTypeIs(IsRsInt())); }

// Matches a MappedType that is a pointer to integer.
auto IsIntPtr() {
  return AllOf(CcTypeIs(CcPointsTo(IsCcInt())),
               RsTypeIs(RsPointsTo(IsRsInt())));
}

// Matches a MappedType for cc and rs types with no type parameters.
auto IsSimpleType(absl::string_view rs_name, absl::string_view cc_name) {
  return AllOf(CcTypeIs(NameIs(cc_name), CcTypeParamsAre()),
               RsTypeIs(NameIs(rs_name), RsTypeParamsAre()));
}

// Matches a Record that has fields matching `matchers`.
template <typename... Args>
auto FieldsAre(const Args&... matchers) {
  return testing::Field("fields", &Record::fields, ElementsAre(matchers...));
}

// Matches a Record that has the given size.
MATCHER_P(RecordSizeIs, size, "") {
  if (arg.size == size) return true;

  *result_listener << "actual size: " << arg.size;
  return false;
}

// Matches a Record that has the given alignment.
MATCHER_P(AlignmentIs, alignment, "") {
  if (arg.alignment == alignment) return true;

  *result_listener << "actual alignment: " << arg.alignment;
  return false;
}

// Matches a Record with a copy_constructor that matches all given matchers.
template <typename... Args>
auto CopyConstructor(const Args&... matchers) {
  return testing::Field("copy_constructor", &Record::copy_constructor,
                        AllOf(matchers...));
}

// Matches a Record with a move_constructor that matches all given matchers.
template <typename... Args>
auto MoveConstructor(const Args&... matchers) {
  return testing::Field("move_constructor", &Record::move_constructor,
                        AllOf(matchers...));
}

// Matches a Record with a destructor that matches all given matchers.
template <typename... Args>
auto Destructor(const Args&... matchers) {
  return testing::Field("destructor", &Record::destructor, AllOf(matchers...));
}

// Matches a Record which is trivial for calls.
MATCHER(IsTrivialAbi, "") { return arg.is_trivial_abi; }

// Matches a SpecialMemberFunc that has the given definition.
MATCHER_P(DefinitionIs, definition, "") { return arg.definition == definition; }

// Matches a Field that has the given access specifier.
MATCHER_P(AccessIs, access, "") { return arg.access == access; }

// Matches a Field that has the given offset.
MATCHER_P(OffsetIs, offset, "") {
  if (arg.offset == offset) return true;

  *result_listener << "actual offset: " << arg.offset;
  return false;
}

// Matches a Field with a type that matches all given matchers.
template <typename... Args>
auto FieldType(const Args&... matchers) {
  return testing::Field("type", &Field::type, AllOf(matchers...));
}

TEST(AstVisitorTest, Noop) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"// nothing interesting there."}));

  EXPECT_THAT(ir.items, IsEmpty());
  EXPECT_THAT(ir.used_headers,
              ElementsAre(Property(&HeaderName::IncludePath,
                                   "test/testing_header_0.h")));
}

TEST(AstVisitorTest, ErrorOnInvalidInput) {
  ASSERT_THAT(IrFromCc({"int foo(); But this is not C++"}),
              StatusIs(absl::StatusCode::kInvalidArgument));
}

TEST(AstVisitorTest, FuncWithVoidReturnType) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo();"}));
  EXPECT_THAT(ir.items, ElementsAre(VariantWith<Func>(
                            AllOf(IdentifierIs("Foo"), MangledNameIs("_Z3Foov"),
                                  ReturnType(IsVoid()), ParamsAre()))));
}

TEST(AstVisitorTest, TwoFuncs) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo(); void Bar();"}));
  EXPECT_THAT(
      ir.items,
      ElementsAre(
          VariantWith<Func>(AllOf(IdentifierIs("Foo"), MangledNameIs("_Z3Foov"),
                                  ReturnType(IsVoid()), ParamsAre())),
          VariantWith<Func>(AllOf(IdentifierIs("Bar"), MangledNameIs("_Z3Barv"),
                                  ReturnType(IsVoid()), ParamsAre()))));
}

TEST(AstVisitorTest, TwoFuncsFromTwoHeaders) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo();", "void Bar();"}));
  EXPECT_THAT(ir.items, ElementsAre(VariantWith<Func>(IdentifierIs("Foo")),
                                    VariantWith<Func>(IdentifierIs("Bar"))));
}

TEST(AstVisitorTest, NonInlineFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo() {}"}));
  EXPECT_THAT(ir.items, ElementsAre(VariantWith<Func>(
                            AllOf(IdentifierIs("Foo"), Not(IsInline())))));
}

TEST(AstVisitorTest, InlineFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"inline void Foo() {}"}));
  EXPECT_THAT(
      ir.items,
      ElementsAre(VariantWith<Func>(AllOf(IdentifierIs("Foo"), IsInline()))));
}

TEST(AstVisitorTest, FuncJustOnce) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo(); void Foo();"}));
  EXPECT_THAT(ir.items,
              ElementsAre(VariantWith<Func>(AllOf(IdentifierIs("Foo")))));
}

TEST(AstVisitorTest, TestImportPointerFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"int* Foo(int* a);"}));

  EXPECT_THAT(ir.items,
              ElementsAre(VariantWith<Func>(AllOf(
                  ReturnType(IsIntPtr()), ParamsAre(ParamType(IsIntPtr()))))));
}

TEST(AstVisitorTest, Struct) {
  ASSERT_OK_AND_ASSIGN(
      IR ir,
      IrFromCc({"struct SomeStruct { int first_field; int second_field; };"}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records,
              ElementsAre(Pointee(AllOf(
                  IdentifierIs("SomeStruct"), RecordSizeIs(8), AlignmentIs(4),
                  FieldsAre(AllOf(IdentifierIs("first_field"),
                                  FieldType(IsInt()), OffsetIs(0)),
                            AllOf(IdentifierIs("second_field"),
                                  FieldType(IsInt()), OffsetIs(32)))))));
}

TEST(AstVisitorTest, TrivialCopyConstructor) {
  absl::string_view file = R"cc(
    struct Implicit {};
    struct Defaulted {
      Defaulted(const Defaulted&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kTrivial)))));
}

TEST(AstVisitorTest, NontrivialSelfCopyConstructor) {
  absl::string_view file = R"cc(
    struct NontrivialSelf {
      NontrivialSelf(const NontrivialSelf&);
    };
    struct NontrivialSub : public NontrivialSelf {};

    // Despite having a defaulted copy constructor, this is not trivially
    // copyable, because the *first* declaration is not defaulted.
    struct NontrivialSelfDefaulted {
      NontrivialSelfDefaulted(const NontrivialSelfDefaulted&);
    };
    inline NontrivialSelfDefaulted::NontrivialSelfDefaulted(
        const NontrivialSelfDefaulted&) = default;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kNontrivialSelf)))));
}

TEST(AstVisitorTest, NontrivialMembersCopyConstructor) {
  absl::string_view file = R"cc(
    struct NontrivialSelf {
      NontrivialSelf(const NontrivialSelf&);
    };
    struct MemberImplicit {
      NontrivialSelf x;
    };
    struct MemberDefaulted {
      MemberDefaulted(const MemberDefaulted&) = default;
      NontrivialSelf x;
    };
    struct Subclass : public MemberImplicit {};
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(records,
              Each(Pointee(AnyOf(
                  IdentifierIs(
                      "NontrivialSelf"),  // needed to create nontrivial members
                  CopyConstructor(DefinitionIs(
                      SpecialMemberFunc::Definition::kNontrivialMembers))))));
}

TEST(AstVisitorTest, DeletedCopyConstructor) {
  absl::string_view file = R"cc(
    struct Deleted {
      Deleted(const Deleted&) = delete;
    };
    struct DeletedByMember {
      Deleted x;
    };
    struct DeletedByCtorDef {
      DeletedByCtorDef(DeletedByCtorDef&&) {}
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kDeleted)))));
}

TEST(AstVisitorTest, PublicCopyConstructor) {
  absl::string_view file = R"cc(
    class Implicit {};
    struct Defaulted {
      Defaulted(const Defaulted&) = default;
    };
    class Section {
     public:
      Section(const Section&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(AccessIs(kPublic)))));
}

TEST(AstVisitorTest, PrivateCopyConstructor) {
  absl::string_view file = R"cc(
    class Defaulted {
      Defaulted(const Defaulted&) = default;
    };
    struct Section {
     private:
      Section(const Section&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(AccessIs(kPrivate)))));
}

TEST(AstVisitorTest, TrivialMoveConstructor) {
  absl::string_view file = R"cc(
    struct Implicit {};
    struct Defaulted {
      Defaulted(Defaulted&&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kTrivial)))));
}

TEST(AstVisitorTest, NontrivialSelfMoveConstructor) {
  absl::string_view file = R"cc(
    struct NontrivialSelf {
      NontrivialSelf(NontrivialSelf&&);
    };
    struct NontrivialSub : public NontrivialSelf {};

    // Despite having a defaulted move constructor, this is not trivially
    // movable, because the *first* declaration is not defaulted.
    struct NontrivialSelfDefaulted {
      NontrivialSelfDefaulted(NontrivialSelfDefaulted&&);
    };
    inline NontrivialSelfDefaulted::NontrivialSelfDefaulted(
        NontrivialSelfDefaulted&&) = default;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kNontrivialSelf)))));
}

TEST(AstVisitorTest, NontrivialMembersMoveConstructor) {
  absl::string_view file = R"cc(
    struct NontrivialSelf {
      NontrivialSelf(NontrivialSelf&&);
    };
    struct MemberImplicit {
      NontrivialSelf x;
    };
    struct MemberDefaulted {
      MemberDefaulted(MemberDefaulted&&) = default;
      NontrivialSelf x;
    };
    struct Subclass : public MemberImplicit {};
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(records,
              Each(Pointee(AnyOf(
                  IdentifierIs(
                      "NontrivialSelf"),  // needed to create nontrivial members
                  MoveConstructor(DefinitionIs(
                      SpecialMemberFunc::Definition::kNontrivialMembers))))));
}

TEST(AstVisitorTest, DeletedMoveConstructor) {
  absl::string_view file = R"cc(
    struct Deleted {
      Deleted(Deleted&&) = delete;
    };
    struct DeletedByMember {
      Deleted x;
    };
    struct SuppressedByCtorDef {
      SuppressedByCtorDef(const SuppressedByCtorDef&) {}
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kDeleted)))));
}

TEST(AstVisitorTest, PublicMoveConstructor) {
  absl::string_view file = R"cc(
    class Implicit {};
    struct Defaulted {
      Defaulted(Defaulted&&) = default;
    };
    class Section {
     public:
      Section(Section&&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(AccessIs(kPublic)))));
}

TEST(AstVisitorTest, PrivateMoveConstructor) {
  absl::string_view file = R"cc(
    class Defaulted {
      Defaulted(Defaulted&&) = default;
    };
    struct Section {
     private:
      Section(Section&&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(AccessIs(kPrivate)))));
}

TEST(AstVisitorTest, TrivialDestructor) {
  absl::string_view file = R"cc(
    struct Implicit {};
    struct Defaulted {
      ~Defaulted() = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(Destructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kTrivial)))));
}

TEST(AstVisitorTest, NontrivialSelfDestructor) {
  absl::string_view file = R"cc(
    struct NontrivialSelf {
      ~NontrivialSelf();
    };
    struct NontrivialSub : public NontrivialSelf {};

    // Despite having a defaulted destructor, this is not trivially
    // destructible, because the *first* declaration is not defaulted.
    struct NontrivialSelfDefaulted {
      ~NontrivialSelfDefaulted();
    };
    inline NontrivialSelfDefaulted::~NontrivialSelfDefaulted() = default;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(Destructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kNontrivialSelf)))));
}

TEST(AstVisitorTest, NontrivialMembersDestructor) {
  absl::string_view file = R"cc(
    struct NontrivialSelf {
      ~NontrivialSelf();
    };
    struct MemberImplicit {
      NontrivialSelf x;
    };
    struct MemberDefaulted {
      MemberDefaulted(MemberDefaulted&&) = default;
      NontrivialSelf x;
    };
    struct Subclass : public MemberImplicit {};
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(records,
              Each(Pointee(AnyOf(
                  IdentifierIs(
                      "NontrivialSelf"),  // needed to create nontrivial members
                  Destructor(DefinitionIs(
                      SpecialMemberFunc::Definition::kNontrivialMembers))))));
}

TEST(AstVisitorTest, DeletedDestructor) {
  absl::string_view file = R"cc(
    struct Deleted {
      ~Deleted() = delete;
    };
    struct DeletedByMember {
      Deleted x;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(Destructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kDeleted)))));
}

TEST(AstVisitorTest, PublicDestructor) {
  absl::string_view file = R"cc(
    class Implicit {};
    struct Defaulted {
      ~Defaulted() = default;
    };
    class Section {
     public:
      ~Section() = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(Destructor(AccessIs(kPublic)))));
}

TEST(AstVisitorTest, PrivateDestructor) {
  absl::string_view file = R"cc(
    class Defaulted {
      ~Defaulted() = default;
    };
    struct Section {
     private:
      ~Section() = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(Destructor(AccessIs(kPrivate)))));
}

TEST(AstVisitorTest, TrivialAbi) {
  absl::string_view file = R"cc(
    struct Empty {};
    struct Defaulted {
      Defaulted(const Defaulted&) = default;
    };
    struct [[clang::trivial_abi]] Nontrivial {
      Nontrivial(const Nontrivial&) {}
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(IsTrivialAbi())));
}

TEST(AstVisitorTest, NotTrivialAbi) {
  absl::string_view file = R"cc(
    struct Nontrivial {
      Nontrivial(const Nontrivial&) {}
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(1));
  EXPECT_THAT(records, Each(Pointee(Not(IsTrivialAbi()))));
}

TEST(AstVisitorTest, MemberVariableAccessSpecifiers) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({R"(
    struct SomeStruct {
      int default_access_int;
    public:
      int public_int;
    protected:
      int protected_int;
    private:
      int private_int;
    };

    class SomeClass {
      int default_access_int;
    };
  )"}));

  std::vector<Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(
      records,
      ElementsAre(
          Pointee(AllOf(
              IdentifierIs("SomeStruct"),
              FieldsAre(
                  AllOf(IdentifierIs("default_access_int"), AccessIs(kPublic)),
                  AllOf(IdentifierIs("public_int"), AccessIs(kPublic)),
                  AllOf(IdentifierIs("protected_int"), AccessIs(kProtected)),
                  AllOf(IdentifierIs("private_int"), AccessIs(kPrivate))))),
          Pointee(AllOf(IdentifierIs("SomeClass"),
                        FieldsAre(AllOf(IdentifierIs("default_access_int"),
                                        AccessIs(kPrivate)))))));
}

}  // namespace
}  // namespace rs_bindings_from_cc
