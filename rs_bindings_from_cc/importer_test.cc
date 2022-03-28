// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <optional>
#include <string>
#include <type_traits>
#include <variant>
#include <vector>

#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "third_party/absl/status/status.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/types/span.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"

namespace rs_bindings_from_cc {
namespace {

using ::testing::AllOf;
using ::testing::AnyOf;
using ::testing::Each;
using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::Pointee;
using ::testing::SizeIs;
using ::testing::VariantWith;
using ::testing::status::StatusIs;

std::optional<ItemId> DeclIdForRecord(const IR& ir, absl::string_view rs_name) {
  for (const Record* record : ir.get_items_if<Record>()) {
    if (record->rs_name == rs_name) {
      return record->id;
    }
  }
  return std::nullopt;
}

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

// Matches an Record node that has the given `rs_name`.
MATCHER_P(RsNameIs, rs_name, "") { return arg.rs_name == rs_name; }

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

// Matches an RsType or CcType that has the given name.
MATCHER_P(NameIs, name, "") {
  if (arg.name == name) return true;

  *result_listener << "actual name: '" << arg.name << "'";
  return false;
}

// Matches an RsType or CcType that has the given decl_id.
MATCHER_P(DeclIdIs, decl_id, "") {
  if (arg.decl_id.hasValue() && *arg.decl_id == decl_id) return true;

  *result_listener << "actual decl_id: ";
  if (arg.decl_id.hasValue()) {
    *result_listener << *arg.decl_id;
  } else {
    *result_listener << "std::nullopt";
  }
  return false;
}

// Matches an RsType or CcType that is const .
MATCHER(IsConst, "") { return arg.is_const; }

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

// Matches an RsType that has type arguments matching `matchers`.
template <typename... Args>
auto RsTypeParamsAre(const Args&... matchers) {
  return testing::Field("type_args", &RsType::type_args,
                        ElementsAre(matchers...));
}

// Matches a CcType that has type arguments matching `matchers`.
template <typename... Args>
auto CcTypeParamsAre(const Args&... matchers) {
  return testing::Field("type_args", &CcType::type_args,
                        ElementsAre(matchers...));
}

auto IsCcInt() { return AllOf(NameIs("int"), CcTypeParamsAre()); }

auto IsRsInt() { return AllOf(NameIs("i32"), RsTypeParamsAre()); }

// Matches a CcType that is a pointer to a type matching `matcher`.
template <typename Matcher>
auto CcPointsTo(const Matcher& matcher) {
  return AllOf(NameIs("*"), CcTypeParamsAre(matcher));
}

template <typename Matcher>
auto CcReferenceTo(const Matcher& matcher) {
  return AllOf(NameIs("&"), CcTypeParamsAre(matcher));
}

// Matches an RsType that is a mutable pointer to a type matching `matcher`.
template <typename Matcher>
auto RsPointsTo(const Matcher& matcher) {
  return AllOf(NameIs("*mut"), RsTypeParamsAre(matcher));
}

// Matches an RsType that is a const pointer to a type matching `matcher`.
template <typename Matcher>
auto RsConstPointsTo(const Matcher& matcher) {
  return AllOf(NameIs("*const"), RsTypeParamsAre(matcher));
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

// Matches a MappedType that is an lvalue reference to integer.
auto IsIntRef() {
  return AllOf(CcTypeIs(CcReferenceTo(IsCcInt())),
               RsTypeIs(RsPointsTo(IsRsInt())));
}

// Matches a MappedType for cc and rs types with no type arguments.
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

// Return the items from `ir` without predefined builtin types.
decltype(IR::items) ItemsWithoutBuiltins(const IR& ir) {
  decltype(IR::items) items;

  for (const auto& item : ir.items) {
    if (const auto* type_alias = std::get_if<TypeAlias>(&item)) {
      if (type_alias->identifier.Ident() == "__builtin_ms_va_list") {
        continue;
      }
    }
    items.push_back(item);
  }

  return items;
}

TEST(ImporterTest, Noop) {
  // Nothing interesting there, but also not empty, so that the header gets
  // generated.
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(" "));

  EXPECT_THAT(ItemsWithoutBuiltins(ir), IsEmpty());
}

TEST(ImporterTest, ErrorOnInvalidInput) {
  ASSERT_THAT(IrFromCc("int foo(); But this is not C++"),
              StatusIs(absl::StatusCode::kInvalidArgument));
}

TEST(ImporterTest, FuncWithVoidReturnType) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc("void Foo();"));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              ElementsAre(VariantWith<Func>(
                  AllOf(IdentifierIs("Foo"), MangledNameIs("_Z3Foov"),
                        ReturnType(IsVoid()), ParamsAre()))));
}

TEST(ImporterTest, TwoFuncs) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc("void Foo(); void Bar();"));
  EXPECT_THAT(
      ItemsWithoutBuiltins(ir),
      ElementsAre(
          VariantWith<Func>(AllOf(IdentifierIs("Foo"), MangledNameIs("_Z3Foov"),
                                  ReturnType(IsVoid()), ParamsAre())),
          VariantWith<Func>(AllOf(IdentifierIs("Bar"), MangledNameIs("_Z3Barv"),
                                  ReturnType(IsVoid()), ParamsAre()))));
}

TEST(ImporterTest, TwoFuncsFromTwoHeaders) {
  ASSERT_OK_AND_ASSIGN(
      IR ir, IrFromCc("", BazelLabel{"//two_funcs:one_target"},
                      {HeaderName("test/testing_header_0.h"),
                       HeaderName("test/testing_header_1.h")},
                      {{HeaderName("test/testing_header_0.h"), "void Foo();"},
                       {HeaderName("test/testing_header_1.h"), "void Bar();"}},
                      {
                          {HeaderName("test/testing_header_0.h"),
                           BazelLabel{"//two_funcs:one_target"}},
                          {HeaderName("test/testing_header_1.h"),
                           BazelLabel{"//two_funcs:one_target"}},
                      }));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              ElementsAre(VariantWith<Func>(IdentifierIs("Foo")),
                          VariantWith<Func>(IdentifierIs("Bar"))));
}

TEST(ImporterTest, NonInlineFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc("void Foo() {}"));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              ElementsAre(VariantWith<Func>(
                  AllOf(IdentifierIs("Foo"), Not(IsInline())))));
}

TEST(ImporterTest, InlineFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc("inline void Foo() {}"));
  EXPECT_THAT(
      ItemsWithoutBuiltins(ir),
      ElementsAre(VariantWith<Func>(AllOf(IdentifierIs("Foo"), IsInline()))));
}

TEST(ImporterTest, FuncJustOnce) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc("void Foo(); void Foo();"));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              ElementsAre(VariantWith<Func>(AllOf(IdentifierIs("Foo")))));
}

TEST(ImporterTest, TestImportPointerFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc("int* Foo(int* a);"));

  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              ElementsAre(VariantWith<Func>(AllOf(
                  ReturnType(IsIntPtr()), ParamsAre(ParamType(IsIntPtr()))))));
}

TEST(ImporterTest, TestImportConstStructPointerFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir,
                       IrFromCc("struct S{}; const S* Foo(const S* s);"));

  std::optional<ItemId> decl_id = DeclIdForRecord(ir, "S");
  ASSERT_TRUE(decl_id.has_value());

  auto is_ptr_to_const_s =
      AllOf(CcTypeIs(CcPointsTo(AllOf(DeclIdIs(*decl_id), IsConst()))),
            RsTypeIs(RsConstPointsTo(DeclIdIs(*decl_id))));

  EXPECT_THAT(ir.items, Contains(VariantWith<Func>(AllOf(
                            IdentifierIs("Foo"), ReturnType(is_ptr_to_const_s),
                            ParamsAre(ParamType(is_ptr_to_const_s))))));
}

TEST(ImporterTest, TestImportReferenceFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc("int& Foo(int& a);"));

  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              ElementsAre(VariantWith<Func>(AllOf(
                  ReturnType(IsIntRef()), ParamsAre(ParamType(IsIntRef()))))));
}

TEST(ImporterTest, TrivialCopyConstructor) {
  absl::string_view file = R"cc(
    struct Implicit {};
    struct Defaulted {
      Defaulted(const Defaulted&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kTrivial)))));
}

TEST(ImporterTest, NontrivialUserDefinedCopyConstructor) {
  absl::string_view file = R"cc(
    struct NontrivialUserDefined {
      NontrivialUserDefined(const NontrivialUserDefined&);
    };
    struct NontrivialSub : public NontrivialUserDefined {};

    // Despite having a defaulted copy constructor, this is not trivially
    // copyable, because the *first* declaration is not defaulted.
    struct NontrivialUserDefinedDefaulted {
      NontrivialUserDefinedDefaulted(const NontrivialUserDefinedDefaulted&);
    };
    inline NontrivialUserDefinedDefaulted::NontrivialUserDefinedDefaulted(
        const NontrivialUserDefinedDefaulted&) = default;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records,
              Each(Pointee(CopyConstructor(DefinitionIs(
                  SpecialMemberFunc::Definition::kNontrivialUserDefined)))));
}

TEST(ImporterTest, NontrivialMembersCopyConstructor) {
  absl::string_view file = R"cc(
    struct NontrivialUserDefined {
      NontrivialUserDefined(const NontrivialUserDefined&);
    };
    struct MemberImplicit {
      NontrivialUserDefined x;
    };
    struct MemberDefaulted {
      MemberDefaulted(const MemberDefaulted&) = default;
      NontrivialUserDefined x;
    };
    struct Subclass : public MemberImplicit {};
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(
      records,
      Each(Pointee(AnyOf(
          RsNameIs(
              "NontrivialUserDefined"),  // needed to create nontrivial members
          CopyConstructor(DefinitionIs(
              SpecialMemberFunc::Definition::kNontrivialMembers))))));
}

TEST(ImporterTest, DeletedCopyConstructor) {
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
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kDeleted)))));
}

TEST(ImporterTest, PublicCopyConstructor) {
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
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(AccessIs(kPublic)))));
}

TEST(ImporterTest, PrivateCopyConstructor) {
  absl::string_view file = R"cc(
    class Defaulted {
      Defaulted(const Defaulted&) = default;
    };
    struct Section {
     private:
      Section(const Section&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(AccessIs(kPrivate)))));
}

TEST(ImporterTest, TrivialMoveConstructor) {
  absl::string_view file = R"cc(
    struct Implicit {};
    struct Defaulted {
      Defaulted(Defaulted&&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kTrivial)))));
}

TEST(ImporterTest, NontrivialUserDefinedMoveConstructor) {
  absl::string_view file = R"cc(
    struct NontrivialUserDefined {
      NontrivialUserDefined(NontrivialUserDefined&&);
    };
    struct NontrivialSub : public NontrivialUserDefined {};

    // Despite having a defaulted move constructor, this is not trivially
    // movable, because the *first* declaration is not defaulted.
    struct NontrivialUserDefinedDefaulted {
      NontrivialUserDefinedDefaulted(NontrivialUserDefinedDefaulted&&);
    };
    inline NontrivialUserDefinedDefaulted::NontrivialUserDefinedDefaulted(
        NontrivialUserDefinedDefaulted&&) = default;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records,
              Each(Pointee(MoveConstructor(DefinitionIs(
                  SpecialMemberFunc::Definition::kNontrivialUserDefined)))));
}

TEST(ImporterTest, NontrivialMembersMoveConstructor) {
  absl::string_view file = R"cc(
    struct NontrivialUserDefined {
      NontrivialUserDefined(NontrivialUserDefined&&);
    };
    struct MemberImplicit {
      NontrivialUserDefined x;
    };
    struct MemberDefaulted {
      MemberDefaulted(MemberDefaulted&&) = default;
      NontrivialUserDefined x;
    };
    struct Subclass : public MemberImplicit {};
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(
      records,
      Each(Pointee(AnyOf(
          RsNameIs(
              "NontrivialUserDefined"),  // needed to create nontrivial members
          MoveConstructor(DefinitionIs(
              SpecialMemberFunc::Definition::kNontrivialMembers))))));
}

TEST(ImporterTest, DeletedMoveConstructor) {
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
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kDeleted)))));
}

TEST(ImporterTest, PublicMoveConstructor) {
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
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(AccessIs(kPublic)))));
}

TEST(ImporterTest, PrivateMoveConstructor) {
  absl::string_view file = R"cc(
    class Defaulted {
      Defaulted(Defaulted&&) = default;
    };
    struct Section {
     private:
      Section(Section&&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(AccessIs(kPrivate)))));
}

TEST(ImporterTest, TrivialDestructor) {
  absl::string_view file = R"cc(
    struct Implicit {};
    struct Defaulted {
      ~Defaulted() = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(Destructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kTrivial)))));
}

TEST(ImporterTest, NontrivialUserDefinedDestructor) {
  absl::string_view file = R"cc(
    struct NontrivialUserDefined {
      ~NontrivialUserDefined();
    };
    struct NontrivialSub : public NontrivialUserDefined {};

    // Despite having a defaulted destructor, this is not trivially
    // destructible, because the destructor is virtual.
    struct VirtualDestructor {
      virtual ~VirtualDestructor() = default;
    };

    // Despite having a defaulted destructor, this is not trivially
    // destructible, because the *first* declaration is not defaulted.
    struct NontrivialUserDefinedDefaulted {
      ~NontrivialUserDefinedDefaulted();
    };
    inline NontrivialUserDefinedDefaulted::~NontrivialUserDefinedDefaulted() =
        default;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(records,
              Each(Pointee(Destructor(DefinitionIs(
                  SpecialMemberFunc::Definition::kNontrivialUserDefined)))));
}

TEST(ImporterTest, NontrivialMembersDestructor) {
  absl::string_view file = R"cc(
    struct NontrivialUserDefined {
      ~NontrivialUserDefined();
    };
    struct MemberImplicit {
      NontrivialUserDefined x;
    };
    struct MemberDefaulted {
      MemberDefaulted(MemberDefaulted&&) = default;
      NontrivialUserDefined x;
    };
    struct Subclass : public MemberImplicit {};
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(
      records,
      Each(Pointee(AnyOf(
          RsNameIs(
              "NontrivialUserDefined"),  // needed to create nontrivial members
          Destructor(DefinitionIs(
              SpecialMemberFunc::Definition::kNontrivialMembers))))));
}

TEST(ImporterTest, DeletedDestructor) {
  absl::string_view file = R"cc(
    struct Deleted {
      ~Deleted() = delete;
    };
    struct DeletedByMember {
      Deleted x;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(Destructor(DefinitionIs(
                           SpecialMemberFunc::Definition::kDeleted)))));
}

TEST(ImporterTest, PublicDestructor) {
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
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(Destructor(AccessIs(kPublic)))));
}

TEST(ImporterTest, PrivateDestructor) {
  absl::string_view file = R"cc(
    class Defaulted {
      ~Defaulted() = default;
    };
    struct Section {
     private:
      ~Section() = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(Destructor(AccessIs(kPrivate)))));
}

TEST(ImporterTest, TrivialAbi) {
  absl::string_view file = R"cc(
    struct Empty {};
    struct Defaulted {
      Defaulted(const Defaulted&) = default;
    };
    struct [[clang::trivial_abi]] Nontrivial {
      Nontrivial(const Nontrivial&) {}
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(IsTrivialAbi())));
}

TEST(ImporterTest, NotTrivialAbi) {
  absl::string_view file = R"cc(
    struct Nontrivial {
      Nontrivial(const Nontrivial&) {}
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(1));
  EXPECT_THAT(records, Each(Pointee(Not(IsTrivialAbi()))));
}

}  // namespace
}  // namespace rs_bindings_from_cc
