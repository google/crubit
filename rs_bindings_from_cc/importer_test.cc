// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/functional/overload.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "common/status_test_matchers.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"

namespace crubit {
namespace {

using ::testing::AllOf;
using ::testing::AnyOf;
using ::testing::Contains;
using ::testing::Each;
using ::testing::ElementsAre;
using ::testing::Eq;
using ::testing::ExplainMatchResult;
using ::testing::Field;
using ::testing::HasSubstr;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::Pointee;
using ::testing::SizeIs;
using ::testing::UnorderedElementsAre;
using ::testing::VariantWith;

std::optional<ItemId> DeclIdForRecord(const IR& ir, absl::string_view rs_name) {
  for (const Record* record : ir.get_items_if<Record>()) {
    if (record->rs_name.Ident() == rs_name) {
      return record->id;
    }
  }
  return std::nullopt;
}

std::optional<IR::Item> FindItemById(const IR& ir, ItemId id) {
  for (auto item : ir.items) {
    if (auto* record = std::get_if<Record>(&item); record && record->id == id) {
      return item;
    } else if (auto* func = std::get_if<Func>(&item); func && func->id == id) {
      return item;
    } else if (auto* comment = std::get_if<Comment>(&item);
               comment && comment->id == id) {
      return item;
    } else if (auto* unsupported = std::get_if<UnsupportedItem>(&item);
               unsupported && unsupported->id == id) {
      return item;
    } else if (auto* ns = std::get_if<Namespace>(&item); ns && ns->id == id) {
      return item;
    } else if (auto* incomplete = std::get_if<IncompleteRecord>(&item);
               incomplete && incomplete->id == id) {
      return item;
    }
  }
  return std::nullopt;
}

template <typename T>
UnqualifiedIdentifier GetName(const T& x) {
  return x.identifier;
}
UnqualifiedIdentifier GetName(const Func& x) { return x.cc_name; }
UnqualifiedIdentifier GetName(const Namespace& x) { return x.cc_name; }

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
MATCHER_P(RsNameIs, rs_name, "") { return arg.rs_name.Ident() == rs_name; }

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

// Matches a Func that has lifetime parameters matching `matcher`.
template <typename... Args>
auto LifetimeParamsAre(const Args&... matchers) {
  return testing::Field("lifetime_params", &Func::lifetime_params,
                        ElementsAre(matchers...));
}

MATCHER_P(UnknownAttributesAre, val, "") {
  if (arg.unknown_attr == val) return true;

  *result_listener << "actual unknown attributes: '" << arg.unknown_attr << "'";
  return false;
}

template <typename... Args>
auto ExplicitLifetimesAre(const Args&... matchers) {
  return testing::Field("explicit_lifetimes", &CcType::explicit_lifetimes,
                        ElementsAre(matchers...));
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

MATCHER_P(UnsupportedItemNameIs, name, "") {
  if (arg.name == name) return true;

  *result_listener << "actual name: '" << arg.name << "'";
  return false;
}

// Matches a CcType that's a primitive with the given name.
MATCHER_P(IsCcPrimitive, name, "") {
  const auto* primitive = std::get_if<CcType::Primitive>(&arg.variant);
  if (primitive != nullptr || primitive->spelling == name) return true;
  *result_listener << "actual name: '";
  if (primitive != nullptr) {
    *result_listener << primitive->spelling;
  }
  *result_listener << "'";
  return false;
}

// Matches text for comments.
MATCHER_P(TextIs, text, "") {
  if (arg.text == text) return true;

  *result_listener << "actual text: '" << arg.text << "'";
  return false;
}

// Matches an CcType that has the given decl_id.
MATCHER_P(CcDeclIdIs, decl_id, "") {
  const auto* id = std::get_if<ItemId>(&arg.variant);
  if (id != nullptr && *id == decl_id) return true;

  *result_listener << "actual decl_id: ";
  if (id != nullptr) {
    *result_listener << *id;
  } else {
    *result_listener << "std::nullopt";
  }
  return false;
}

// Matches an CcType that is const .
MATCHER(IsConst, "") { return arg.is_const; }

// Matches a CcType pointer with kind `kind`.
MATCHER_P(IsPointerWithKind, kind, "") {
  const auto* pointer = std::get_if<CcType::PointerType>(&arg.variant);
  if (pointer == nullptr) {
    *result_listener << "was not a pointer";
    return false;
  }
  if (pointer->kind == kind) return true;
  *result_listener << "wrong pointer kind";
  return false;
}

// Matches a CcType that is a pointer to a type matching `matcher`.
template <typename Matcher>
auto CcPointsTo(const Matcher& matcher) {
  return Field("variant", &CcType::variant,
               VariantWith<CcType::PointerType>(AllOf(
                   Field("kind", &CcType::PointerType::kind,
                         AnyOf(Eq(PointerTypeKind::kNullable),
                               Eq(PointerTypeKind::kNonNull))),
                   Field("pointee_type", &CcType::PointerType::pointee_type,
                         Pointee(matcher)))));
}

template <typename Matcher>
auto CcReferenceTo(const Matcher& matcher) {
  return Field("variant", &CcType::variant,
               VariantWith<CcType::PointerType>(AllOf(
                   Field("kind", &CcType::PointerType::kind,
                         Eq(PointerTypeKind::kLValueRef)),
                   Field("pointee_type", &CcType::PointerType::pointee_type,
                         Pointee(matcher)))));
}

// Matches a CcType that is void.
MATCHER(IsVoid, "") { return arg.IsVoid(); }

// Recursively tests the provided CcType to check if any lifetimes are set.
MATCHER(HasLifetimes, "") {
  return std::visit(
      absl::Overload{
          [](const CcType::Primitive& primitive) { return false; },
          [&](const CcType::PointerType& pointer) {
            if (pointer.lifetime.has_value()) {
              return true;
            }
            return ExplainMatchResult(HasLifetimes(), *pointer.pointee_type,
                                      result_listener);
          },
          [&](const CcType::FuncPointer& func_pointer) {
            return ExplainMatchResult(Contains(HasLifetimes()),
                                      func_pointer.param_and_return_types,
                                      result_listener);
          },
          // There doesn't appear to be a way to record lifetimes as applied
          // to records accepting lifetime arguments.
          [&](const ItemId& id) { return false; },
      },
      arg.variant);
}

// Matches a CcType that is a pointer to integer.
auto IsIntPtr() { return CcPointsTo(IsCcPrimitive("int")); }

// Matches a CcType that is an lvalue reference to integer.
auto IsIntRef() { return CcReferenceTo(IsCcPrimitive("int")); }

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
      if (type_alias->cc_name.Ident() == "__builtin_ms_va_list") {
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
  IrFromCcOptions options;
  options.extra_source_code_for_testing = " ";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({.extra_source_code_for_testing = " "}));

  EXPECT_THAT(ItemsWithoutBuiltins(ir), IsEmpty());
}

TEST(ImporterTest, ErrorOnInvalidInput) {
  ASSERT_THAT(IrFromCc({"int foo(); But this is not C++"}),
              StatusIs(absl::StatusCode::kInvalidArgument));
}

TEST(ImporterTest, FuncWithVoidReturnType) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo();"}));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              UnorderedElementsAre(VariantWith<Func>(
                  AllOf(IdentifierIs("Foo"), MangledNameIs("_Z3Foov"),
                        ReturnType(IsVoid()), ParamsAre()))));
}

TEST(ImporterTest, TwoFuncs) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo(); void Bar();"}));
  EXPECT_THAT(
      ItemsWithoutBuiltins(ir),
      UnorderedElementsAre(
          VariantWith<Func>(AllOf(IdentifierIs("Foo"), MangledNameIs("_Z3Foov"),
                                  ReturnType(IsVoid()), ParamsAre())),
          VariantWith<Func>(AllOf(IdentifierIs("Bar"), MangledNameIs("_Z3Barv"),
                                  ReturnType(IsVoid()), ParamsAre()))));
}

TEST(ImporterTest, TwoFuncsFromTwoHeaders) {
  ASSERT_OK_AND_ASSIGN(
      IR ir,
      IrFromCc({.current_target = BazelLabel{"//two_funcs:one_target"},
                .public_headers = {HeaderName("test/testing_header_0.h"),
                                   HeaderName("test/testing_header_1.h")},
                .virtual_headers_contents_for_testing =
                    {{HeaderName("test/testing_header_0.h"), "void Foo();"},
                     {HeaderName("test/testing_header_1.h"), "void Bar();"}},
                .headers_to_targets = {
                    {HeaderName("test/testing_header_0.h"),
                     BazelLabel{"//two_funcs:one_target"}},
                    {HeaderName("test/testing_header_1.h"),
                     BazelLabel{"//two_funcs:one_target"}},
                }}));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              UnorderedElementsAre(VariantWith<Func>(IdentifierIs("Foo")),
                                   VariantWith<Func>(IdentifierIs("Bar"))));
}

TEST(ImporterTest, NonInlineFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo() {}"}));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              UnorderedElementsAre(VariantWith<Func>(
                  AllOf(IdentifierIs("Foo"), Not(IsInline())))));
}

TEST(ImporterTest, InlineFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"inline void Foo() {}"}));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              UnorderedElementsAre(
                  VariantWith<Func>(AllOf(IdentifierIs("Foo"), IsInline()))));
}

TEST(ImporterTest, InlineUndefinedFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"inline void Foo();"}));
  EXPECT_THAT(ir.get_items_if<UnsupportedItem>(),
              ElementsAre(Pointee(
                  Field("errors", &UnsupportedItem::errors,
                        ElementsAre(Property(
                            "message", &FormattedError::message,
                            HasSubstr("Inline function is not defined")))))));
}

TEST(ImporterTest, InlineDefinition) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo(); inline void Foo() {}"}));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              UnorderedElementsAre(
                  VariantWith<Func>(AllOf(IdentifierIs("Foo"), IsInline()))));
}

TEST(ImporterTest, FuncJustOnce) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo(); void Foo();"}));
  EXPECT_THAT(
      ItemsWithoutBuiltins(ir),
      UnorderedElementsAre(VariantWith<Func>(AllOf(IdentifierIs("Foo")))));
}

TEST(ImporterTest, TestImportPointerFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"int* Foo(int* a);"}));

  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              UnorderedElementsAre(VariantWith<Func>(AllOf(
                  ReturnType(IsIntPtr()), ParamsAre(ParamType(IsIntPtr()))))));
}

TEST(ImporterTest, TestImportConstStructPointerFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir,
                       IrFromCc({"struct S{}; const S* Foo(const S* s);"}));

  std::optional<ItemId> decl_id = DeclIdForRecord(ir, "S");
  ASSERT_TRUE(decl_id.has_value());

  auto is_ptr_to_const_s = CcPointsTo(AllOf(CcDeclIdIs(*decl_id), IsConst()));

  EXPECT_THAT(ir.items, Contains(VariantWith<Func>(AllOf(
                            IdentifierIs("Foo"), ReturnType(is_ptr_to_const_s),
                            ParamsAre(ParamType(is_ptr_to_const_s))))));
}

TEST(ImporterTest, TestImportReferenceFunc) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"int& Foo(int& a);"}));

  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              UnorderedElementsAre(VariantWith<Func>(AllOf(
                  ReturnType(IsIntRef()), ParamsAre(ParamType(IsIntRef()))))));
}

TEST(ImporterTest, TrivialCopyConstructor) {
  absl::string_view file = R"cc(
    struct Implicit {};
    struct Defaulted {
      Defaulted(const Defaulted&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records,
              Each(Pointee(CopyConstructor(SpecialMemberFunc::kTrivial))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(CopyConstructor(
                           SpecialMemberFunc::kNontrivialUserDefined))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(
      records,
      Each(Pointee(AnyOf(
          RsNameIs(
              "NontrivialUserDefined"),  // needed to create nontrivial members
          CopyConstructor(SpecialMemberFunc::kNontrivialMembers)))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records,
              Each(Pointee(CopyConstructor(SpecialMemberFunc::kUnavailable))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records,
              Each(Pointee(CopyConstructor(SpecialMemberFunc::kTrivial))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records,
              Each(Pointee(CopyConstructor(SpecialMemberFunc::kUnavailable))));
}

TEST(ImporterTest, TrivialMoveConstructor) {
  absl::string_view file = R"cc(
    struct Implicit {};
    struct Defaulted {
      Defaulted(Defaulted&&) = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records,
              Each(Pointee(MoveConstructor(SpecialMemberFunc::kTrivial))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(MoveConstructor(
                           SpecialMemberFunc::kNontrivialUserDefined))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(
      records,
      Each(Pointee(AnyOf(
          RsNameIs(
              "NontrivialUserDefined"),  // needed to create nontrivial members
          MoveConstructor(SpecialMemberFunc::kNontrivialMembers)))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records,
              Each(Pointee(MoveConstructor(SpecialMemberFunc::kUnavailable))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records,
              Each(Pointee(MoveConstructor(SpecialMemberFunc::kTrivial))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records,
              Each(Pointee(MoveConstructor(SpecialMemberFunc::kUnavailable))));
}

TEST(ImporterTest, TrivialDestructor) {
  absl::string_view file = R"cc(
    struct Implicit {};
    struct Defaulted {
      ~Defaulted() = default;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records, Each(Pointee(Destructor(SpecialMemberFunc::kTrivial))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(
      records,
      Each(Pointee(Destructor(SpecialMemberFunc::kNontrivialUserDefined))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));
  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(4));
  EXPECT_THAT(
      records,
      Each(Pointee(AnyOf(
          RsNameIs(
              "NontrivialUserDefined"),  // needed to create nontrivial members
          Destructor(SpecialMemberFunc::kNontrivialMembers)))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records,
              Each(Pointee(Destructor(SpecialMemberFunc::kUnavailable))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(3));
  EXPECT_THAT(records, Each(Pointee(Destructor(SpecialMemberFunc::kTrivial))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(2));
  EXPECT_THAT(records,
              Each(Pointee(Destructor(SpecialMemberFunc::kUnavailable))));
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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

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
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  EXPECT_THAT(records, SizeIs(1));
  EXPECT_THAT(records, Each(Pointee(Not(IsTrivialAbi()))));
}

TEST(ImporterTest, TopLevelItemIds) {
  absl::string_view file = R"cc(
    struct ForwardDeclaration;
    struct TopLevelStruct {};
    // Top level comment

    // Function comment
    void top_level_func();
    namespace top_level_namespace {
    struct Nested {};
    // free nested comment

    // nested_func comment
    void nested_func();
    }  // namespace top_level_namespace
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<IR::Item> items;
  for (const auto& id : ir.top_level_item_ids.at(ir.current_target)) {
    auto item = FindItemById(ir, id);
    ASSERT_TRUE(item.has_value());
    items.push_back(*item);
  }

  EXPECT_THAT(
      items,
      ElementsAre(
          VariantWith<IncompleteRecord>(RsNameIs("ForwardDeclaration")),
          VariantWith<Record>(RsNameIs("TopLevelStruct")),
          VariantWith<Comment>(TextIs("Top level comment")),
          VariantWith<Func>(IdentifierIs("top_level_func")),
          VariantWith<Namespace>(IdentifierIs("top_level_namespace")),
          VariantWith<Comment>(TextIs("namespace top_level_namespace"))));
}

TEST(ImporterTest, ForwardDeclarationAndDefinition) {
  absl::string_view file = R"cc(
    struct ForwardDeclaredStruct;
    struct ForwardDeclaredStruct {};
    struct Struct {};
    struct Struct;
    struct ForwardDeclaredStructWithNoDefinition;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<IR::Item> items;
  for (const auto& id : ir.top_level_item_ids.at(ir.current_target)) {
    auto item = FindItemById(ir, id);
    items.push_back(*item);
  }

  EXPECT_THAT(
      items, ElementsAre(VariantWith<Record>(RsNameIs("ForwardDeclaredStruct")),
                         VariantWith<Record>(RsNameIs("Struct")),
                         VariantWith<IncompleteRecord>(RsNameIs(
                             "ForwardDeclaredStructWithNoDefinition"))));
}

TEST(ImporterTest, DuplicateForwardDeclarations) {
  absl::string_view file = R"cc(
    struct ForwardDeclaredStructWithNoDefinition;
    struct ForwardDeclaredStructWithNoDefinition;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  std::vector<IR::Item> items;
  for (const auto& id : ir.top_level_item_ids.at(ir.current_target)) {
    auto item = FindItemById(ir, id);
    items.push_back(*item);
  }

  EXPECT_THAT(items, ElementsAre(VariantWith<IncompleteRecord>(
                         RsNameIs("ForwardDeclaredStructWithNoDefinition"))));
}

TEST(ImporterTest, RecordItemIds) {
  absl::string_view file = R"cc(
    struct TopLevelStruct {
      // A free comment

      // foo comment
      int foo;

      int bar();
      struct Nested {};
      int baz();
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  std::vector<const Record*> records = ir.get_items_if<Record>();
  ASSERT_EQ(records.size(), 2);

  std::vector<IR::Item> items;
  for (const auto& id : records[0]->child_item_ids) {
    auto item = FindItemById(ir, id);
    ASSERT_TRUE(item.has_value());
    items.push_back(*item);
  }

  EXPECT_THAT(items,
              AllOf(Contains(VariantWith<Comment>(TextIs("A free comment"))),
                    Contains(VariantWith<Func>(IdentifierIs("bar"))),
                    Contains(VariantWith<Record>(RsNameIs("Nested"))),
                    Contains(VariantWith<Func>(IdentifierIs("baz")))));
}

TEST(ImporterTest, FailedClassTemplateMethod) {
  absl::string_view file = R"cc(
    struct NoMethod final {};
    template <typename T>
    struct A final {
      auto CallMethod(T t) { return t.method(); }
    };
    using B = A<NoMethod>;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  const UnsupportedItem* unsupported_a = nullptr;
  const UnsupportedItem* unsupported_b = nullptr;
  for (auto unsupported_item : ir.get_items_if<UnsupportedItem>()) {
    if (unsupported_item->name == "A") {
      unsupported_a = unsupported_item;
    } else if (unsupported_item->name == "B") {
      unsupported_b = unsupported_item;
    }
  }
  ASSERT_TRUE(unsupported_a != nullptr);
  ASSERT_TRUE(unsupported_b != nullptr);
  EXPECT_THAT(unsupported_a->errors,
              Contains(testing::Property(
                  "message", &FormattedError::message,
                  HasSubstr("Class templates are not supported yet"))));
  EXPECT_THAT(
      unsupported_b->errors,
      Contains(testing::Property(
          "message", &FormattedError::message,
          HasSubstr(
              "Unsupported type 'A<NoMethod>': Failed to complete template "
              "specialization type A<NoMethod>: template belongs to target "
              "//test:testing_target, which does not support Crubit."))));
}

TEST(ImporterTest, CrashRepro_FunctionTypeAlias) {
  absl::string_view file = R"cc(
    using Callback = void(const int&);
    void SetHook(Callback* cb);
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
}

TEST(ImporterTest, CrashRepro_DecltypeInvolvingTemplate) {
  absl::string_view file = R"cc(
    template <class T>
    struct A {};
    struct B {
      A<int> a;
    };
    template <class Trait>
    struct C {
      static decltype(Trait::a) Func();
    };
    // Note that to trigger the crash, we specifically require the following:
    // - `C::Func()` needs to be static.
    // - We need to call `C` function on a variable `c` (we don't crash if we
    //   call `C::Func()`.
    // - `c` needs to be a parameter (we don't crash if it is a local variable).
    void Func(C<B> c) { c.Func(); }
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
}

TEST(ImporterTest, CrashRepro_AutoInvolvingTemplate) {
  absl::string_view file = R"cc(
    template <class T>
    struct Template {};
    auto Func() { return Template<int>{}; }
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));
}

absl::StatusOr<IR> IrFromCcWithAssumedLifetimes(absl::string_view program) {
  auto full_program = absl::StrCat(R"cc(
#define $(l) [[clang::annotate_type("lifetime", #l)]]
#define $a $(a)
#define $b $(b)
#define LIFETIME_PARAMS(...) [[clang::annotate("lifetime_params", __VA_ARGS__)]]
                                   )cc",
                                   program);
  BazelLabel test_target{"//test:testing_target"};
  return IrFromCc(IrFromCcOptions{
      .extra_source_code_for_testing = full_program,
      .crubit_features = {{test_target, {"assume_lifetimes"}}}});
}

TEST(ImporterTest, AssumedLifetimesCapturesRawFunctionParameterLifetime) {
  absl::string_view file = R"cc(
    void f(int& $a x);
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCcWithAssumedLifetimes(file));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              UnorderedElementsAre(VariantWith<Func>(AllOf(
                  LifetimeParamsAre(), IdentifierIs("f"), ReturnType(IsVoid()),
                  ParamsAre(ParamType(
                      AllOf(ExplicitLifetimesAre("a"), UnknownAttributesAre(""),
                            Not(HasLifetimes()), IsIntRef())))))));
}

TEST(ImporterTest, AssumedLifetimesCapturesRawFunctionParameterLifetimes) {
  absl::string_view file = R"cc(
    void f(int& $a $b x);
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCcWithAssumedLifetimes(file));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              UnorderedElementsAre(VariantWith<Func>(AllOf(
                  LifetimeParamsAre(), IdentifierIs("f"), ReturnType(IsVoid()),
                  ParamsAre(ParamType(AllOf(
                      ExplicitLifetimesAre("a", "b"), UnknownAttributesAre(""),
                      Not(HasLifetimes()), IsIntRef())))))));
}

TEST(ImporterTest,
     AssumedLifetimesCapturesRawFunctionParameterLifetimesSingleAnnotation) {
  absl::string_view file = R"cc(
    void f(int& [[clang::annotate_type("lifetime", "aa", "bb", "cc")]] x);
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCcWithAssumedLifetimes(file));
  EXPECT_THAT(
      ItemsWithoutBuiltins(ir),
      UnorderedElementsAre(VariantWith<Func>(AllOf(
          LifetimeParamsAre(), IdentifierIs("f"), ReturnType(IsVoid()),
          ParamsAre(ParamType(AllOf(ExplicitLifetimesAre("aa", "bb", "cc"),
                                    UnknownAttributesAre(""),
                                    Not(HasLifetimes()), IsIntRef())))))));
}

TEST(ImporterTest,
     AssumedLifetimesCapturesRawFunctionParameterLifetimesMultipleAnnotations) {
  absl::string_view file = R"cc(
    void f(int& [[clang::annotate_type("lifetime", "a", "b")]]
           [[clang::annotate_type("lifetime", "c", "d")]] x);
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCcWithAssumedLifetimes(file));
  EXPECT_THAT(
      ItemsWithoutBuiltins(ir),
      UnorderedElementsAre(VariantWith<Func>(AllOf(
          LifetimeParamsAre(), IdentifierIs("f"), ReturnType(IsVoid()),
          ParamsAre(ParamType(AllOf(ExplicitLifetimesAre("a", "b", "c", "d"),
                                    UnknownAttributesAre(""),
                                    Not(HasLifetimes()), IsIntRef())))))));
}

TEST(ImporterTest, AssumedLifetimesCapturesImplicitThisLifetime) {
  absl::string_view file = R"cc(
    struct S {
      int* $b f() $a;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCcWithAssumedLifetimes(file));
  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              Contains(VariantWith<Func>(AllOf(
                  IdentifierIs("f"), ReturnType(ExplicitLifetimesAre("b")),
                  ParamsAre(AllOf(
                      IdentifierIs("__this"),
                      ParamType(AllOf(
                          ExplicitLifetimesAre("a"), Not(HasLifetimes()),
                          IsPointerWithKind(PointerTypeKind::kNonNull)))))))));
}

TEST(ImporterTest, AssumedLifetimesCapturesImplicitThisLifetimeRvalueRef) {
  absl::string_view file = R"cc(
    struct S {
      int* $b f() && $a;
    };
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCcWithAssumedLifetimes(file));
  EXPECT_THAT(
      ItemsWithoutBuiltins(ir),
      Contains(VariantWith<Func>(
          AllOf(IdentifierIs("f"), ReturnType(ExplicitLifetimesAre("b")),
                ParamsAre(AllOf(
                    IdentifierIs("__this"),
                    ParamType(AllOf(
                        ExplicitLifetimesAre("a"), Not(HasLifetimes()),
                        IsPointerWithKind(PointerTypeKind::kRValueRef)))))))));
}
}  // namespace
}  // namespace crubit
