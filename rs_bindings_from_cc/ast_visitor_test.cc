// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "devtools/cymbal/common/clang_tool.h"
#include "rs_bindings_from_cc/frontend_action.h"
#include "rs_bindings_from_cc/ir.h"
#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/strings/substitute.h"
#include "third_party/absl/types/span.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/FrontendAction.h"

namespace rs_bindings_from_cc {
namespace {

using ::testing::AllOf;
using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::Property;
using ::testing::SizeIs;

// Matches an IR node that has the given identifier.
MATCHER_P(IdentifierIs, identifier, "") {
  if (arg.identifier.Ident() == identifier) return true;

  *result_listener << "actual identifier: '" << arg.identifier.Ident() << "'";
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
  return testing::Field(&Func::return_type, matcher);
}

// Matches a Func that has parameters matching `matchers`.
template <typename... Args>
auto ParamsAre(const Args&... matchers) {
  return testing::Field(&Func::params, ElementsAre(matchers...));
}

// Matches a Func that is inline.
MATCHER(IsInline, "") { return arg.is_inline; }

// Matches a FuncParam with a type that matches all given matchers.
template <typename... Args>
auto ParamType(const Args&... matchers) {
  return testing::Field(&FuncParam::type, AllOf(matchers...));
}

// Matches a Type that has the given Rust name.
MATCHER_P(RsNameIs, rs_name, "") {
  if (arg.rs_name == rs_name) return true;

  *result_listener << "actual rs_name: '" << arg.rs_name << "'";
  return false;
}

// Matches a Type that has the given C++ name.
MATCHER_P(CcNameIs, cc_name, "") {
  if (arg.cc_name == cc_name) return true;

  *result_listener << "actual cc_name: '" << arg.cc_name << "'";
  return false;
}

// Matches a type that has type parameters matching `matchers`.
template <typename... Args>
auto TypeParamsAre(const Args&... matchers) {
  return testing::Field(&Type::type_params, ElementsAre(matchers...));
}

// Matches a type that is void.
MATCHER(IsVoid, "") { return arg.IsVoid(); }

// Matches an integer type.
auto IsInt() {
  return AllOf(RsNameIs("i32"), CcNameIs("int"), TypeParamsAre());
}

// Matches a type that is a pointer to a type matching `matcher`.
template <typename Matcher>
auto PointsTo(const Matcher& matcher) {
  return AllOf(RsNameIs("*mut"), CcNameIs("*"), TypeParamsAre(matcher));
}

// Matches a Record that has fields matching `matchers`.
template <typename... Args>
auto FieldsAre(const Args&... matchers) {
  return testing::Field(&Record::fields, ElementsAre(matchers...));
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
  return testing::Field(&Field::type, AllOf(matchers...));
}

constexpr absl::string_view kVirtualInputPath =
    "ast_visitor_test_virtual_input.cc";

IR ImportCode(absl::Span<const absl::string_view> header_files_contents,
              const std::vector<absl::string_view>& args) {
  std::vector<std::string> headers;
  absl::flat_hash_map<std::string, std::string> file_contents;
  std::string virtual_input_file_content;

  int counter = 0;
  for (const absl::string_view header_content : header_files_contents) {
    std::string filename(
        absl::Substitute("test/testing_header_$0.h", counter++));
    file_contents.insert({filename, std::string(header_content)});
    absl::SubstituteAndAppend(&virtual_input_file_content, "#include \"$0\"\n",
                              filename);
    headers.push_back(std::move(filename));
  }

  file_contents.insert(
      {std::string(kVirtualInputPath), virtual_input_file_content});

  std::vector<std::string> args_as_strings(args.begin(), args.end());
  args_as_strings.push_back("--syntax-only");
  args_as_strings.push_back(std::string(kVirtualInputPath));

  IR ir;
  devtools::cymbal::RunToolWithClangFlagsOnCode(
      args_as_strings, file_contents,
      std::make_unique<rs_bindings_from_cc::FrontendAction>(
          std::vector<absl::string_view>(headers.begin(), headers.end()), ir));
  return ir;
}

TEST(AstVisitorTest, Noop) {
  IR ir = ImportCode({"// nothing interesting there."}, {});
  EXPECT_THAT(ir.functions, IsEmpty());
  EXPECT_THAT(ir.used_headers,
              ElementsAre(Property(&HeaderName::IncludePath,
                                   "test/testing_header_0.h")));
}

TEST(AstVisitorTest, IREmptyOnInvalidInput) {
  IR ir = ImportCode({"int foo(); But this is not C++"}, {});
  EXPECT_THAT(ir.functions, IsEmpty());
}

TEST(AstVisitorTest, FuncWithVoidReturnType) {
  IR ir = ImportCode({"void Foo();"}, {});
  EXPECT_THAT(ir.functions,
              ElementsAre(AllOf(IdentifierIs("Foo"), MangledNameIs("_Z3Foov"),
                                ReturnType(IsVoid()), ParamsAre())));
}

TEST(AstVisitorTest, TwoFuncs) {
  IR ir = ImportCode({"void Foo(); void Bar();"}, {});
  EXPECT_THAT(ir.functions,
              ElementsAre(AllOf(IdentifierIs("Foo"), MangledNameIs("_Z3Foov"),
                                ReturnType(IsVoid()), ParamsAre()),
                          AllOf(IdentifierIs("Bar"), MangledNameIs("_Z3Barv"),
                                ReturnType(IsVoid()), ParamsAre())));
}

TEST(AstVisitorTest, TwoFuncsFromTwoHeaders) {
  IR ir = ImportCode({"void Foo();", "void Bar();"}, {});
  EXPECT_THAT(ir.functions,
              ElementsAre(IdentifierIs("Foo"), IdentifierIs("Bar")));
}

TEST(AstVisitorTest, NonInlineFunc) {
  IR ir = ImportCode({"void Foo() {}"}, {});
  EXPECT_THAT(ir.functions,
              ElementsAre(AllOf(IdentifierIs("Foo"), Not(IsInline()))));
}

TEST(AstVisitorTest, InlineFunc) {
  IR ir = ImportCode({"inline void Foo() {}"}, {});
  EXPECT_THAT(ir.functions,
              ElementsAre(AllOf(IdentifierIs("Foo"), IsInline())));
}

TEST(AstVisitorTest, FuncJustOnce) {
  IR ir = ImportCode({"void Foo(); void Foo();"}, {});
  EXPECT_THAT(ir.functions, ElementsAre(AllOf(IdentifierIs("Foo"))));
}

TEST(AstVisitorTest, FuncParams) {
  IR ir = ImportCode({"int Add(int a, int b);"}, {});
  ASSERT_THAT(ir.functions, SizeIs(1));

  EXPECT_THAT(
      ir.functions,
      ElementsAre(AllOf(
          IdentifierIs("Add"), MangledNameIs("_Z3Addii"), ReturnType(IsInt()),
          ParamsAre(AllOf(ParamType(IsInt()), IdentifierIs("a")),
                    AllOf(ParamType(IsInt()), IdentifierIs("b"))))));
}

TEST(AstVisitorTest, TestImportPointerFunc) {
  IR ir = ImportCode({"int* Foo(int* a);"}, {});

  EXPECT_THAT(ir.functions,
              ElementsAre(AllOf(ReturnType(PointsTo(IsInt())),
                                ParamsAre(ParamType(PointsTo(IsInt()))))));
}

TEST(AstVisitorTest, Struct) {
  IR ir = ImportCode(
      {"struct SomeStruct { int first_field; int second_field; };"}, {});
  EXPECT_THAT(ir.functions, IsEmpty());

  EXPECT_THAT(ir.records,
              ElementsAre(AllOf(
                  IdentifierIs("SomeStruct"), RecordSizeIs(8), AlignmentIs(4),
                  FieldsAre(AllOf(IdentifierIs("first_field"),
                                  FieldType(IsInt()), OffsetIs(0)),
                            AllOf(IdentifierIs("second_field"),
                                  FieldType(IsInt()), OffsetIs(32))))));
}

TEST(AstVisitorTest, MemberVariableAccessSpecifiers) {
  IR ir = ImportCode({R"(
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
  )"},
                     {});

  EXPECT_THAT(ir.functions, IsEmpty());

  EXPECT_THAT(
      ir.records,
      ElementsAre(
          AllOf(
              IdentifierIs("SomeStruct"),
              FieldsAre(
                  AllOf(IdentifierIs("default_access_int"), AccessIs(kPublic)),
                  AllOf(IdentifierIs("public_int"), AccessIs(kPublic)),
                  AllOf(IdentifierIs("protected_int"), AccessIs(kProtected)),
                  AllOf(IdentifierIs("private_int"), AccessIs(kPrivate)))),
          AllOf(IdentifierIs("SomeClass"),
                FieldsAre(AllOf(IdentifierIs("default_access_int"),
                                AccessIs(kPrivate))))));
}

TEST(AstVisitorTest, IntegerTypes) {
  auto ir = ImportCode({"#include <stdint.h>\n"
                        "struct S { "
                        "  char c;"
                        "  short s;"
                        "  int i;"
                        "  long l;"
                        "  long long ll;"
                        "  unsigned char uc;"
                        "  unsigned short us;"
                        "  unsigned int ui;"
                        "  unsigned long ul;"
                        "  unsigned long long ull;"
                        "  signed char sc;"
                        "  signed short ss;"
                        "  signed int si;"
                        "  signed long sl;"
                        "  signed long long sll;"
                        "  int8_t i8;"
                        "  int16_t i16;"
                        "  int32_t i32;"
                        "  int64_t i64;"
                        "  uint8_t u8;"
                        "  uint16_t u16;"
                        "  uint32_t u32;"
                        "  uint64_t u64;"
                        "};"},
                       {});

  EXPECT_THAT(ir.records,
              ElementsAre(FieldsAre(
                  FieldType(RsNameIs("i8"), CcNameIs("char")),
                  FieldType(RsNameIs("i16"), CcNameIs("short")),
                  FieldType(RsNameIs("i32"), CcNameIs("int")),
                  FieldType(RsNameIs("i64"), CcNameIs("long")),
                  FieldType(RsNameIs("i64"), CcNameIs("long long")),

                  FieldType(RsNameIs("u8"), CcNameIs("unsigned char")),
                  FieldType(RsNameIs("u16"), CcNameIs("unsigned short")),
                  FieldType(RsNameIs("u32"), CcNameIs("unsigned int")),
                  FieldType(RsNameIs("u64"), CcNameIs("unsigned long")),
                  FieldType(RsNameIs("u64"), CcNameIs("unsigned long long")),

                  FieldType(RsNameIs("i8"), CcNameIs("signed char")),
                  FieldType(RsNameIs("i16"), CcNameIs("short")),
                  FieldType(RsNameIs("i32"), CcNameIs("int")),
                  FieldType(RsNameIs("i64"), CcNameIs("long")),
                  FieldType(RsNameIs("i64"), CcNameIs("long long")),

                  FieldType(RsNameIs("i8"), CcNameIs("int8_t")),
                  FieldType(RsNameIs("i16"), CcNameIs("int16_t")),
                  FieldType(RsNameIs("i32"), CcNameIs("int32_t")),
                  FieldType(RsNameIs("i64"), CcNameIs("int64_t")),

                  FieldType(RsNameIs("u8"), CcNameIs("uint8_t")),
                  FieldType(RsNameIs("u16"), CcNameIs("uint16_t")),
                  FieldType(RsNameIs("u32"), CcNameIs("uint32_t")),
                  FieldType(RsNameIs("u64"), CcNameIs("uint64_t")))));
}

}  // namespace
}  // namespace rs_bindings_from_cc
