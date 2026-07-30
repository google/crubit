// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "net/proto2/contrib/proto_matcher/proto_matcher.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/functional/overload.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/match.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/status_test_matchers.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"

namespace crubit {
namespace {

namespace ir_proto = ::crubit::rs_bindings_from_cc::ir_proto::flat;

using Record = ir_proto::Record;
using Enum = ir_proto::Enum;
using Func = ir_proto::Func;
using UnsupportedItem = ir_proto::UnsupportedItem;
using IncompleteRecord = ir_proto::IncompleteRecord;
using Namespace = ir_proto::Namespace;
using TypeAlias = ir_proto::TypeAlias;
using Constant = ir_proto::Constant;
using GlobalVar = ir_proto::GlobalVar;
using Comment = ir_proto::Comment;
using Item = ir_proto::Item;
using ExistingRustType = ir_proto::ExistingRustType;
using UseMod = ir_proto::UseMod;
using CcType = ir_proto::CcType;
using FuncParam = ir_proto::FuncParam;
using FieldProto = ir_proto::Field;
using FormattedError = ir_proto::FormattedError;

namespace SpecialMemberFunc {
constexpr auto kTrivial = ir_proto::TRIVIAL;
constexpr auto kNontrivialMembers = ir_proto::NONTRIVIAL_MEMBERS;
constexpr auto kNontrivialUserDefined = ir_proto::NONTRIVIAL_USER_DEFINED;
constexpr auto kUnavailable = ir_proto::UNAVAILABLE;
}  // namespace SpecialMemberFunc
namespace PointerTypeKind {
constexpr auto kNullable = ir_proto::NULLABLE;
constexpr auto kNonNull = ir_proto::NON_NULL;
constexpr auto kLValueRef = ir_proto::L_VALUE_REF;
constexpr auto kRValueRef = ir_proto::R_VALUE_REF;
constexpr auto kOwned = ir_proto::OWNED;
}  // namespace PointerTypeKind
using RecordType = ir_proto::RecordType;

using ::testing::AllOf;
using ::testing::AnyOf;
using ::testing::Contains;
using ::testing::Each;
using ::testing::ElementsAre;
using ::testing::Eq;
using ::testing::EqualsProto;
using ::testing::ExplainMatchResult;
using ::testing::Field;
using ::testing::HasSubstr;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::Pointee;
using ::testing::Property;
using ::testing::SizeIs;
using ::testing::UnorderedElementsAre;
using ::testing::proto::Partially;

template <typename T, typename Matcher>
auto VariantWith(Matcher matcher) {
  if constexpr (std::is_same_v<T, Func>) {
    return Property("func", &Item::func, matcher);
  } else if constexpr (std::is_same_v<T, Record>) {
    return Property("record", &Item::record, matcher);
  } else if constexpr (std::is_same_v<T, Enum>) {
    return Property("enum_decl", &Item::enum_decl, matcher);
  } else if constexpr (std::is_same_v<T, UnsupportedItem>) {
    return Property("unsupported_item", &Item::unsupported_item, matcher);
  } else if constexpr (std::is_same_v<T, IncompleteRecord>) {
    return Property("incomplete_record", &Item::incomplete_record, matcher);
  } else if constexpr (std::is_same_v<T, Namespace>) {
    return Property("namespace_decl", &Item::namespace_decl, matcher);
  } else if constexpr (std::is_same_v<T, TypeAlias>) {
    return Property("type_alias", &Item::type_alias, matcher);
  } else if constexpr (std::is_same_v<T, Constant>) {
    return Property("constant", &Item::constant, matcher);
  } else if constexpr (std::is_same_v<T, GlobalVar>) {
    return Property("global_var", &Item::global_var, matcher);
  } else if constexpr (std::is_same_v<T, Comment>) {
    return Property("comment", &Item::comment, matcher);
  } else if constexpr (std::is_same_v<T, ExistingRustType>) {
    return Property("existing_rust_type", &Item::existing_rust_type, matcher);
  } else if constexpr (std::is_same_v<T, UseMod>) {
    return Property("use_mod", &Item::use_mod, matcher);
  }
}

std::optional<int64_t> GetItemId(const ir_proto::Item& item) {
  const auto* refl = item.GetReflection();
  const auto* field =
      refl->GetOneofFieldDescriptor(item, item.GetDescriptor()->oneof_decl(0));
  if (!field) return std::nullopt;
  const auto& sub = refl->GetMessage(item, field);
  const auto* id = sub.GetDescriptor()->FindFieldByName("id");
  return id ? std::optional(sub.GetReflection()->GetInt64(sub, id))
            : std::nullopt;
}

std::optional<ItemId> DeclIdForRecord(const IR& ir, absl::string_view rs_name) {
  for (const Record* record : ir.get_items_if<Record>()) {
    if (record->rs_name().identifier() == rs_name) {
      return ItemId(record->id());
    }
  }
  return std::nullopt;
}

absl::StatusOr<IR> IrFromCcWithRecordImplDebug(
    absl::string_view extra_source_code_for_testing) {
  return IrFromCc(
      {.extra_source_code_for_testing = extra_source_code_for_testing,
       .crubit_features = {
           {BazelLabel{"//test:testing_target"}, {"record_impl_debug"}}}});
}

std::optional<Item> FindItemById(const IR& ir, ItemId id) {
  for (const auto& [target, item_list] : ir.ir_proto.top_level_items()) {
    for (const auto& item : item_list.items()) {
      if (GetItemId(item) == id.value()) return item;
    }
  }
  return std::nullopt;
}

inline absl::string_view GetName(const Func& func) {
  if (func.cc_name().has_ident()) return func.cc_name().ident().identifier();
  return "";
}

inline absl::string_view GetName(const Record& record) {
  return record.cc_name().identifier();
}

inline absl::string_view GetName(const TypeAlias& type_alias) {
  return type_alias.cc_name().identifier();
}

inline absl::string_view GetName(const Namespace& ns) {
  return ns.cc_name().identifier();
}

inline absl::string_view GetName(const ExistingRustType& existing_rust_type) {
  return existing_rust_type.cc_name();
}

inline absl::string_view GetName(const FuncParam& func_param) {
  return func_param.identifier().identifier();
}

// Matches an IR node that has the given identifier.
MATCHER_P(IdentifierIs, identifier, "") {
  absl::string_view name = GetName(arg);
  if (name == identifier) return true;
  *result_listener << "actual identifier: '" << name << "'";
  return false;
}

// Matches a Record node that has the given `rs_name`.
MATCHER_P(RsNameIs, rs_name, "") {
  return arg.rs_name().identifier() == rs_name;
}

// Matches an IR item (Record or TypeAlias) that has the given `cc_name`.
MATCHER_P(CcNameIs, cc_name, "") { return GetName(arg) == cc_name; }

MATCHER(HasDetectedFormatter, "") {
  if (arg.detected_formatter()) return true;
  *result_listener << "detected_formatter is false";
  return false;
}

// Matches a Func that has a return type matching `matcher`.
template <typename Matcher>
auto ReturnType(const Matcher& matcher) {
  return Property("return_type", &Func::return_type, matcher);
}

// Matches a Func that has lifetime parameters matching `matcher`.
template <typename... Args>
auto LifetimeParamsAre(const Args&... matchers) {
  return Property("lifetime_params", &Func::lifetime_params,
                  ElementsAre(matchers...));
}

MATCHER_P(UnknownAttributesAre, val, "") {
  if (arg.unknown_attr() == val) return true;

  *result_listener << "actual unknown attributes: '" << arg.unknown_attr()
                   << "'";
  return false;
}

MATCHER_P(ExplicitLifetimesAreMatcher, matcher, "") {
  return ExplainMatchResult(matcher, arg.explicit_lifetimes(), result_listener);
}

template <typename... Args>
auto ExplicitLifetimesAre(const Args&... matchers) {
  return ExplicitLifetimesAreMatcher(ElementsAre(matchers...));
}

// Matches a Func that has parameters matching `matchers`.
template <typename... Args>
auto ParamsAre(const Args&... matchers) {
  return Property("params", &Func::params, ElementsAre(matchers...));
}

// Matches a Func that is inline.
MATCHER(IsInline, "") { return arg.is_inline(); }

// Matches a FuncParam with a type that matches all given matchers.
template <typename... Args>
auto ParamType(const Args&... matchers) {
  return Property("type", &ir_proto::FuncParam::type, AllOf(matchers...));
}

MATCHER_P(UnsupportedItemNameIs, name, "") {
  if (arg.name() == name) return true;

  *result_listener << "actual name: '" << arg.name() << "'";
  return false;
}

// Matches an UnsupportedItem containing a FormattedError message.
template <typename Matcher>
auto HasErrorMessage(const Matcher& msg_matcher) {
  return Property("errors", &UnsupportedItem::errors,
                  ElementsAre(Property(&FormattedError::message, msg_matcher)));
}

// Matches a CcType that's a primitive with the given name.
MATCHER_P(IsCcPrimitive, name, "") {
  if (arg.has_primitive() && arg.primitive().spelling() == name) return true;

  *result_listener << "actual name: '";
  if (arg.has_primitive()) {
    *result_listener << arg.primitive().spelling();
  }
  *result_listener << "'";
  return false;
}

// Matches text for comments.
MATCHER_P(TextIs, text, "") {
  if (arg.text() == text) return true;

  *result_listener << "actual text: '" << arg.text() << "'";
  return false;
}

// Matches an CcType that has the given decl_id.
MATCHER_P(CcDeclIdIs, decl_id, "") {
  if (arg.has_decl() && arg.decl() == decl_id.value()) return true;

  *result_listener << "actual decl_id: ";
  if (arg.has_decl()) {
    *result_listener << arg.decl();
  } else {
    *result_listener << "std::nullopt";
  }
  return false;
}

// Matches an CcType that is const .
MATCHER(IsConst, "") { return arg.is_const(); }

// Matches a CcType pointer with kind `kind`.
MATCHER_P(IsPointerWithKind, kind, "") {
  if (!arg.has_pointer()) {
    *result_listener << "was not a pointer";
    return false;
  }
  if (arg.pointer().kind() == kind) return true;
  *result_listener << "wrong pointer kind";
  return false;
}

// Matches a CcType that is a pointer to a type matching `matcher`.
template <typename Matcher>
auto CcPointsTo(const Matcher& matcher) {
  return Property("pointer", &CcType::pointer,
                  AllOf(Property("kind", &CcType::PointerType::kind,
                                 AnyOf(Eq(PointerTypeKind::kNullable),
                                       Eq(PointerTypeKind::kNonNull))),
                        Property("pointee_type",
                                 &CcType::PointerType::pointee_type, matcher)));
}

template <typename Matcher>
auto CcReferenceTo(const Matcher& matcher) {
  return Property("pointer", &CcType::pointer,
                  AllOf(Property("kind", &CcType::PointerType::kind,
                                 Eq(PointerTypeKind::kLValueRef)),
                        Property("pointee_type",
                                 &CcType::PointerType::pointee_type, matcher)));
}

// Matches a CcType that is void.
MATCHER(IsVoid, "") {
  return arg.has_primitive() && arg.primitive().spelling() == "void";
}

// Recursively tests the provided CcType to check if any lifetimes are set.
MATCHER(HasLifetimes, "") {
  if (arg.has_pointer()) {
    if (arg.pointer().lifetime() != 0) return true;
    return ExplainMatchResult(HasLifetimes(), arg.pointer().pointee_type(),
                              result_listener);
  } else if (arg.has_func_pointer()) {
    for (const auto& param : arg.func_pointer().param_and_return_types()) {
      if (ExplainMatchResult(HasLifetimes(), param, result_listener))
        return true;
    }
    return false;
  }
  // There doesn't appear to be a way to record lifetimes as applied
  // to records accepting lifetime arguments.
  return false;
}

// Matches a CcType that is a pointer to integer.
auto IsIntPtr() { return CcPointsTo(IsCcPrimitive("int")); }

// Matches a CcType that is an lvalue reference to integer.
auto IsIntRef() { return CcReferenceTo(IsCcPrimitive("int")); }

// Matches a Record that has fields matching `matchers`.
template <typename... Args>
auto FieldsAre(const Args&... matchers) {
  return Property("fields", &Record::fields, ElementsAre(matchers...));
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
  return Property("copy_constructor", &Record::copy_constructor,
                  AllOf(matchers...));
}

// Matches a Record with a move_constructor that matches all given matchers.
template <typename... Args>
auto MoveConstructor(const Args&... matchers) {
  return Property("move_constructor", &Record::move_constructor,
                  AllOf(matchers...));
}

// Matches a Record with a destructor that matches all given matchers.
template <typename... Args>
auto Destructor(const Args&... matchers) {
  return Property("destructor", &Record::destructor, AllOf(matchers...));
}

MATCHER(ImplDebug, "") { return arg.impl_debug(); }

// Matches a Record which is trivial for calls.
MATCHER(IsTrivialAbi, "") { return arg.is_trivial_abi(); }

// Matches a Field that has the given offset.
MATCHER_P(OffsetIs, offset, "") {
  if (arg.offset == offset) return true;

  *result_listener << "actual offset: " << arg.offset;
  return false;
}

MATCHER_P(FieldTypeMatcher, matcher, "") {
  return ExplainMatchResult(matcher, arg.type(), result_listener);
}

// Matches a Field with a type that matches all given matchers.
template <typename... Args>
auto FieldType(const Args&... matchers) {
  return FieldTypeMatcher(AllOf(matchers...));
}

// Return the items from `ir` without predefined builtin types.
std::vector<Item> ItemsWithoutBuiltins(const IR& ir) {
  std::vector<Item> items;
  auto process_item = [&](auto& self, const Item& item) -> void {
    if (item.has_type_alias()) {
      // Skip builtin type aliases like __uint128_t, __builtin_ms_va_list.
      if (absl::StartsWith(item.type_alias().cc_name().identifier(), "__")) {
        return;
      }
    }
    items.push_back(item);
    if (item.has_record()) {
      for (const auto& child : item.record().children()) {
        self(self, child);
      }
    } else if (item.has_namespace_decl()) {
      for (const auto& child : item.namespace_decl().children()) {
        self(self, child);
      }
    }
  };

  for (const auto& [target, item_list] : ir.ir_proto.top_level_items()) {
    for (const auto& item : item_list.items()) {
      process_item(process_item, item);
    }
  }
  return items;
}

std::vector<ir_proto::Item> GetTopLevelItems(const IR& ir) {
  auto it = ir.ir_proto.top_level_items().find(ir.current_target.value());
  if (it == ir.ir_proto.top_level_items().end()) {
    return {};
  }
  return {it->second.items().begin(), it->second.items().end()};
}

TEST(ImporterTest, ProtoMessageBridgeType) {
  absl::string_view file = R"cc(
    namespace proto2 {
    struct MessageLite {};
    struct Message : public MessageLite {};
    }  // namespace proto2
    class MyMessage : public google::protobuf::Message {};
    class MyMessage_Request : public google::protobuf::Message {};
    class MyMessage_Request_Inner : public google::protobuf::Message {};
    class Outer : public google::protobuf::Message {};
    class Outer_Inner_Level2 : public google::protobuf::Message {};
  )cc";
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({file}));

  EXPECT_THAT(
      ir.get_items_if<Record>(),
      AllOf(Contains(Pointee(Partially(EqualsProto(R"pb(
              rs_name { identifier: "MyMessage" }
              bridge_type { proto_message_bridge { rust_name: "MyMessage" } }
            )pb")))),
            Contains(Pointee(Partially(EqualsProto(R"pb(
              rs_name { identifier: "MyMessage_Request" }
              bridge_type {
                proto_message_bridge { rust_name: "my_message::Request" }
              }
            )pb")))),
            Contains(Pointee(Partially(EqualsProto(R"pb(
              rs_name { identifier: "MyMessage_Request_Inner" }
              bridge_type {
                proto_message_bridge { rust_name: "my_message::request::Inner" }
              }
            )pb")))),
            Contains(Pointee(Partially(EqualsProto(R"pb(
              rs_name { identifier: "Outer" }
              bridge_type { proto_message_bridge { rust_name: "Outer" } }
            )pb")))),
            Contains(Pointee(Partially(EqualsProto(R"pb(
              rs_name { identifier: "Outer_Inner_Level2" }
              bridge_type {
                proto_message_bridge { rust_name: "outer::Inner_Level2" }
              }
            )pb"))))));
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
              UnorderedElementsAre(VariantWith<Func>(Partially(EqualsProto(R"pb(
                cc_name { ident { identifier: "Foo" } }
                mangled_name: "_Z3Foov"
                return_type { primitive { spelling: "void" } }
                params: []
              )pb")))));
}

TEST(ImporterTest, TwoFuncs) {
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({"void Foo(); void Bar();"}));
  EXPECT_THAT(
      ItemsWithoutBuiltins(ir),
      UnorderedElementsAre(VariantWith<Func>(Partially(EqualsProto(R"pb(
                             cc_name { ident { identifier: "Foo" } }
                             mangled_name: "_Z3Foov"
                             return_type { primitive { spelling: "void" } }
                             params: []
                           )pb"))),
                           VariantWith<Func>(Partially(EqualsProto(R"pb(
                             cc_name { ident { identifier: "Bar" } }
                             mangled_name: "_Z3Barv"
                             return_type { primitive { spelling: "void" } }
                             params: []
                           )pb")))));
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
              ElementsAre(Pointee(HasErrorMessage(
                  HasSubstr("Inline function is not defined")))));
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

  EXPECT_THAT(ItemsWithoutBuiltins(ir),
              Contains(VariantWith<Func>(
                  AllOf(IdentifierIs("Foo"), ReturnType(is_ptr_to_const_s),
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

  EXPECT_THAT(
      GetTopLevelItems(ir),
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

  EXPECT_THAT(
      GetTopLevelItems(ir),
      ElementsAre(VariantWith<Record>(RsNameIs("ForwardDeclaredStruct")),
                  VariantWith<Record>(RsNameIs("Struct")),
                  VariantWith<IncompleteRecord>(
                      RsNameIs("ForwardDeclaredStructWithNoDefinition"))));
}

TEST(ImporterTest, DuplicateForwardDeclarations) {
  absl::string_view file = R"cc(
    struct ForwardDeclaredStructWithNoDefinition;
    struct ForwardDeclaredStructWithNoDefinition;
  )cc";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc({file}));

  EXPECT_THAT(GetTopLevelItems(ir),
              ElementsAre(VariantWith<IncompleteRecord>(
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

  std::vector<Item> items(records[0]->children().begin(),
                          records[0]->children().end());

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
  const TypeAlias* unsupported_b = nullptr;
  for (auto unsupported_item : ir.get_items_if<UnsupportedItem>()) {
    if (unsupported_item->name() == "A") {
      unsupported_a = unsupported_item;
    }
  }
  for (auto type_alias : ir.get_items_if<TypeAlias>()) {
    if (type_alias->cc_name().identifier() == "B") {
      unsupported_b = type_alias;
    }
  }
  ASSERT_TRUE(unsupported_a != nullptr);
  ASSERT_TRUE(unsupported_b != nullptr);
  ASSERT_TRUE(unsupported_b->underlying_type().has_error());
  EXPECT_THAT(
      unsupported_a->errors(),
      Contains(Property("message", &FormattedError::message,
                        HasSubstr("Class templates are not yet supported"))));
  EXPECT_THAT(
      unsupported_b->underlying_type().error().message(),
      HasSubstr("Unsupported type 'A<NoMethod>': Failed to complete template "
                "specialization type A<NoMethod>: template belongs to target "
                "//test:testing_target, which does not support Crubit."));
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

TEST(ImporterTest, DetectsFormatterAsAbslStringify) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir, IrFromCc({R"cc(
                               struct ByRef {
                                 template <typename Sink>
                                 friend void AbslStringify(Sink&,
                                                           const ByRef&) {}
                               };
                               struct ByValue {
                                 template <typename Sink>
                                 friend void AbslStringify(Sink&, ByValue) {}
                               };
                               struct NoFormatter {};
                             )cc"}));
  EXPECT_THAT(
      ir.get_items_if<Record>(),
      AllOf(
          Contains(Pointee(AllOf(RsNameIs("ByRef"), HasDetectedFormatter()))),
          Contains(Pointee(AllOf(RsNameIs("ByValue"), HasDetectedFormatter()))),
          Contains(Pointee(
              AllOf(RsNameIs("NoFormatter"), Not(HasDetectedFormatter()))))));
}

TEST(ImporterTest, DetectsFormatterAsOstream) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir,
      IrFromCc(  //
          {R"cc(
             namespace std {
             template <typename T>
             struct char_traits {};
             template <typename T, typename Traits = char_traits<T>>
             struct basic_ostream {};
             using ostream = basic_ostream<char>;
             }  // namespace std

             struct ByRef {
               friend std::ostream& operator<<(std::ostream& out, const ByRef&) {
                 return out;
               }
             };
             struct ByValue {
               friend std::ostream& operator<<(std::ostream& out, ByValue) { return out; }
             };
             struct NoFormatter {};
           )cc"}));
  EXPECT_THAT(
      ir.get_items_if<Record>(),
      AllOf(
          Contains(Pointee(AllOf(RsNameIs("ByRef"), HasDetectedFormatter()))),
          Contains(Pointee(AllOf(RsNameIs("ByValue"), HasDetectedFormatter()))),
          Contains(Pointee(
              AllOf(RsNameIs("NoFormatter"), Not(HasDetectedFormatter()))))));
}

TEST(ImporterTest, DetectsFormatterAsPrinterOfBase) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir, IrFromCc({R"cc(
                               struct Base {
                                 template <typename Sink>
                                 friend void AbslStringify(Sink&, const Base&) {
                                 }
                               };
                               struct Derived : Base {};
                             )cc"}));
  EXPECT_THAT(
      ir.get_items_if<Record>(),
      Contains(Pointee(AllOf(RsNameIs("Derived"), HasDetectedFormatter()))));
}

TEST(ImporterTest, DetectsFormatterAsPrinterInCrtpBase) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir, IrFromCc({R"cc(
                               template <typename This>
                               struct Base {
                                 template <typename Sink>
                                 friend void AbslStringify(Sink&, const This&) {
                                 }
                               };
                               struct Derived : private Base<Derived> {};
                             )cc"}));
  EXPECT_THAT(
      ir.get_items_if<Record>(),
      Contains(Pointee(AllOf(RsNameIs("Derived"), HasDetectedFormatter()))));
}

TEST(ImporterTest, DetectsEnumFormatter) {
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCc({R"cc(
                                                enum class Foo {
                                                  kFoo,
                                                };
                                                template <typename Sink>
                                                void AbslStringify(Sink&, Foo) {
                                                }
                                              )cc"}));
  EXPECT_THAT(
      ir.get_items_if<Enum>(),
      Contains(Pointee(AllOf(RsNameIs("Foo"), HasDetectedFormatter()))));
}

TEST(ImporterTest, DoesNotDetectAbslStringifyMemberFunctionAsFormatter) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir, IrFromCc({R"cc(
                               struct Foo {
                                 template <typename Sink>
                                 static void AbslStringify(Sink&, const Foo&) {}
                               };
                             )cc"}));
  EXPECT_THAT(
      ir.get_items_if<Record>(),
      Contains(Pointee(AllOf(RsNameIs("Foo"), Not(HasDetectedFormatter())))));
}

TEST(ImporterTest, DoesNotDetectOperatorLeftShiftWrongTypesAsFormatter) {
  ASSERT_OK_AND_ASSIGN(  //
      const IR ir,       //
      IrFromCc({R"cc(
                  struct Foo {
                    friend Foo& operator<<(Foo& foo, int) { return foo; }
                  };
                )cc"}));
  EXPECT_THAT(
      ir.get_items_if<Record>(),
      Contains(Pointee(AllOf(RsNameIs("Foo"), Not(HasDetectedFormatter())))));
}

TEST(ImporterTest, OverridesDisplayForRecord) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir,
      IrFromCc({R"cc(
                  template <bool>
                  struct enable_if {};

                  template <>
                  struct enable_if<true> {
                    using type = void;
                  };

                  template <bool b>
                  using enable_if_t = typename enable_if<b>::type;

                  template <bool b>
                  struct [[clang::annotate("crubit_override_display", b)]]
                  MaybeFormattable {
                    // Use SFINAE so that `AbslStringify` isn't as easily
                    // detectable.
                    template <typename Sink, bool sfinae_b = b>
                    friend enable_if_t<sfinae_b> AbslStringify(
                        Sink& sink, const MaybeFormattable&) {}
                  };
                  struct NotFormattable : MaybeFormattable<false> {};
                  struct Formattable : MaybeFormattable<true> {};
                )cc"}));
  EXPECT_THAT(ir.get_items_if<Record>(),
              AllOf(Contains(Pointee(AllOf(RsNameIs("NotFormattable"),
                                           Not(HasDetectedFormatter())))),
                    Contains(Pointee(AllOf(RsNameIs("Formattable"),
                                           HasDetectedFormatter())))));
}

TEST(ImporterTest, OverridesDisplayForEnum) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir,
      IrFromCc({R"cc(
                  namespace std {
                  template <typename T, typename Traits>
                  struct basic_ostream {};
                  }  // namespace std

                  enum class [[clang::annotate("crubit_override_display",
                                               true)]] Foo {
                    kFoo,
                  };
                  // Make this generic so that `operator<<` isn't as easily
                  // detectable.
                  template <typename T, typename Traits>
                  auto& operator<<(std::basic_ostream<T, Traits>& out, Foo) {
                    return out;
                  }
                )cc"}));
  EXPECT_THAT(
      ir.get_items_if<Enum>(),
      Contains(Pointee(AllOf(RsNameIs("Foo"), HasDetectedFormatter()))));
}

TEST(ImporterTest, OverrideDisplayInconsistent) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir,  //
      IrFromCc(     //
          {R"cc(
             struct [[clang::annotate("crubit_override_display",
                                      true)]] Inconsistent;
             struct [[clang::annotate("crubit_override_display", false)]] Inconsistent {};
           )cc"}));
  EXPECT_THAT(ir.get_items_if<UnsupportedItem>(),
              ElementsAre(Pointee(AllOf(
                  UnsupportedItemNameIs("Inconsistent"),
                  HasErrorMessage(HasSubstr("crubit_override_display"))))));
}

TEST(ImporterTest, OverrideDisplayMissingArgs) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir,
      IrFromCc({R"cc(
                  struct [[clang::annotate("crubit_override_display")]]
                  MissingArgs {};
                )cc"}));
  EXPECT_THAT(
      ir.get_items_if<UnsupportedItem>(),
      ElementsAre(Pointee(AllOf(UnsupportedItemNameIs("MissingArgs"),
                                HasErrorMessage(HasSubstr("argument"))))));
}

TEST(ImporterTest, OverrideDisplayMultipleArgs) {
  ASSERT_OK_AND_ASSIGN(const IR ir,
                       IrFromCc({R"cc(
                                   struct [[clang::annotate(
                                       "crubit_override_display", true, false)]]
                                   MultipleArgs {};
                                 )cc"}));
  EXPECT_THAT(
      ir.get_items_if<UnsupportedItem>(),
      ElementsAre(Pointee(AllOf(UnsupportedItemNameIs("MultipleArgs"),
                                HasErrorMessage(HasSubstr("argument"))))));
}

TEST(ImporterTest, OverrideDisplayWrongArgType) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir,
      IrFromCc({R"cc(
                  struct [[clang::annotate("crubit_override_display", "foo")]]
                  WrongArgType {};
                )cc"}));
  EXPECT_THAT(ir.get_items_if<UnsupportedItem>(),
              ElementsAre(Pointee(AllOf(UnsupportedItemNameIs("WrongArgType"),
                                        HasErrorMessage(HasSubstr("bool"))))));
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

TEST(ImporterTest, ImplDebugDefaultTrue) {
  ASSERT_OK_AND_ASSIGN(const IR ir,
                       IrFromCcWithRecordImplDebug("struct S {};"));
  EXPECT_THAT(ir.get_items_if<Record>(),
              ElementsAre(Pointee(AllOf(RsNameIs("S"), ImplDebug()))));
}

TEST(ImporterTest, ImplDebugOverrideFalseIsFalse) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir, IrFromCcWithRecordImplDebug(R"cc(
        struct [[clang::annotate("crubit_override_debug", false)]] S {};
      )cc"));
  EXPECT_THAT(ir.get_items_if<Record>(),
              ElementsAre(Pointee(AllOf(RsNameIs("S"), Not(ImplDebug())))));
}

TEST(ImporterTest, ImplDebugOverrideTrueIsTrue) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir, IrFromCcWithRecordImplDebug(R"cc(
        struct [[clang::annotate("crubit_override_debug", true)]] S {};
      )cc"));
  EXPECT_THAT(ir.get_items_if<Record>(),
              ElementsAre(Pointee(AllOf(RsNameIs("S"), ImplDebug()))));
}

TEST(ImporterTest, ImplDebugUnexpectedArgsMissing) {
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCcWithRecordImplDebug(R"cc(
                         struct [[clang::annotate("crubit_override_debug")]] S {
                         };
                       )cc"));
  EXPECT_THAT(ir.get_items_if<Record>(), IsEmpty());
  EXPECT_THAT(ir.get_items_if<UnsupportedItem>(),
              ElementsAre(Pointee(
                  AllOf(UnsupportedItemNameIs("S"),
                        HasErrorMessage(HasSubstr("crubit_override_debug"))))));
}

TEST(ImporterTest, ImplDebugUnexpectedArgsTooMany) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir, IrFromCcWithRecordImplDebug(R"cc(
        struct [[clang::annotate("crubit_override_debug", true, false)]] S {};
      )cc"));
  EXPECT_THAT(ir.get_items_if<Record>(), IsEmpty());
  EXPECT_THAT(ir.get_items_if<UnsupportedItem>(),
              ElementsAre(Pointee(
                  AllOf(UnsupportedItemNameIs("S"),
                        HasErrorMessage(HasSubstr("crubit_override_debug"))))));
}

TEST(ImporterTest, ImplDebugUnexpectedArgsWrongType) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir, IrFromCcWithRecordImplDebug(R"cc(
        struct [[clang::annotate("crubit_override_debug", "true")]] S {};
      )cc"));
  EXPECT_THAT(ir.get_items_if<Record>(), IsEmpty());
  EXPECT_THAT(ir.get_items_if<UnsupportedItem>(),
              ElementsAre(Pointee(AllOf(
                  UnsupportedItemNameIs("S"),
                  HasErrorMessage(HasSubstr("must evaluate to a bool"))))));
}
TEST(ImporterTest, ExistingRustTypeWithoutImplDebug) {
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCcWithRecordImplDebug(R"cc(
                         struct [[clang::annotate("crubit_internal_rust_type",
                                                  "::my_crate::NoDebug")]] S {};
                       )cc"));
  EXPECT_THAT(ir.get_items_if<ExistingRustType>(),
              ElementsAre(Pointee(AllOf(CcNameIs("S"), Not(ImplDebug())))));
}

TEST(ImporterTest, ExistingRustTypeWithImplDebug) {
  ASSERT_OK_AND_ASSIGN(const IR ir, IrFromCcWithRecordImplDebug(R"cc(
                         namespace rs::core::fmt {
                         struct Debug;
                         }  // namespace rs::core::fmt

                         namespace rs_std {
                         template <typename T, typename Trait>
                         struct impl;
                         }  // namespace rs_std

                         struct [[clang::annotate("crubit_internal_rust_type",
                                                  "::my_crate::HasDebug")]] S {
                         };

                         namespace rs_std {
                         template <>
                         struct impl<S, ::rs::core::fmt::Debug> {
                           static constexpr bool kIsImplemented = true;
                         };
                         }  // namespace rs_std
                       )cc"));
  EXPECT_THAT(ir.get_items_if<ExistingRustType>(),
              ElementsAre(Pointee(AllOf(CcNameIs("S"), ImplDebug()))));
}

TEST(ImporterTest, RecordTruncatesAndHashesRustNameWhenOver160Chars) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir,
      IrFromCc(
          {R"cc(
             struct
                 LongStructName_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789 {
             };
           )cc"}));

  EXPECT_THAT(
      ir.get_items_if<Record>(),
      ElementsAre(Pointee(AllOf(
          CcNameIs(
              "LongStructName_123456789_123456789_123456789_123456789_"
              "123456789_"
              "123456789_123456789_123456789_123456789_123456789_123456789_"
              "123456789_123456789_123456789_123456789"),
          RsNameIs(
              "LongStructName_123456789_123456789_123456789_123456789_"
              "123456789_"
              "123456789_123456789_123456789_123456789_123456789_123456789_"
              "123456789_12345678_983f62f5b703d944")))));
}

TEST(ImporterTest, TypeAliasTruncatesAndHashesRustNameWhenOver160Chars) {
  ASSERT_OK_AND_ASSIGN(
      const IR ir,
      IrFromCc(
          {R"cc(
             using LongAliasName_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789 =
                 int;
           )cc"}));

  EXPECT_THAT(
      ItemsWithoutBuiltins(ir),
      ElementsAre(VariantWith<TypeAlias>(AllOf(
          CcNameIs(
              "LongAliasName_123456789_123456789_123456789_123456789_123456789_"
              "123456789_123456789_123456789_123456789_123456789_123456789_"
              "123456789_123456789_123456789_123456789"),
          RsNameIs(
              "LongAliasName_123456789_123456789_123456789_123456789_123456789_"
              "123456789_123456789_123456789_123456789_123456789_123456789_"
              "123456789_123456789_b5317e69b85ef233")))));
}
}  // namespace
}  // namespace crubit
