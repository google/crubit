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

// Matches a type that is void.
MATCHER(IsVoid, "") { return arg.IsVoid(); }

// Matches a Record that has fields matching `matchers`.
template <typename... Args>
auto FieldsAre(const Args&... matchers) {
  return testing::Field(&Record::fields, ElementsAre(matchers...));
}

// Matches a Field that has the given access specifier.
MATCHER_P(AccessIs, access, "") { return arg.access == access; }

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

  Func func = ir.functions[0];
  EXPECT_EQ(func.identifier.Ident(), "Add");
  EXPECT_EQ(func.mangled_name, "_Z3Addii");
  EXPECT_EQ(func.return_type.rs_name, "i32");
  EXPECT_THAT(func.return_type.type_params, IsEmpty());

  ASSERT_THAT(func.params, SizeIs(2));
  EXPECT_EQ(func.params[0].type.rs_name, "i32");
  EXPECT_THAT(func.params[0].type.type_params, IsEmpty());
  EXPECT_EQ(func.params[0].identifier.Ident(), "a");
  EXPECT_EQ(func.params[1].type.rs_name, "i32");
  EXPECT_THAT(func.params[1].type.type_params, IsEmpty());
  EXPECT_EQ(func.params[1].identifier.Ident(), "b");
}

TEST(AstVisitorTest, TestImportPointerFunc) {
  IR ir = ImportCode({"int* Foo(int* a);"}, {});
  ASSERT_THAT(ir.functions, SizeIs(1));

  Func func = ir.functions[0];

  ASSERT_THAT(func.params, SizeIs(1));
  Type return_type = func.return_type;
  Type param_type = func.params[0].type;

  for (Type type : {return_type, param_type}) {
    EXPECT_EQ(type.rs_name, "*mut");
    EXPECT_EQ(type.cc_name, "*");
    ASSERT_THAT(type.type_params, SizeIs(1));
    const Type& pointee = type.type_params[0];
    EXPECT_EQ(pointee.rs_name, "i32");
    EXPECT_EQ(pointee.cc_name, "int");
    EXPECT_THAT(pointee.type_params, IsEmpty());
  }
}

TEST(AstVisitorTest, Struct) {
  IR ir = ImportCode(
      {"struct SomeStruct { int first_field; int second_field; };"}, {});
  EXPECT_THAT(ir.functions, IsEmpty());

  EXPECT_THAT(ir.records, SizeIs(1));
  Record some_struct = ir.records[0];

  EXPECT_EQ(some_struct.identifier.Ident(), "SomeStruct");

  ASSERT_THAT(some_struct.fields, SizeIs(2));
  Field first = some_struct.fields[0];
  Field second = some_struct.fields[1];

  EXPECT_EQ(first.identifier.Ident(), "first_field");
  EXPECT_EQ(first.type.cc_name, "int");
  EXPECT_EQ(first.type.rs_name, "i32");
  EXPECT_EQ(second.identifier.Ident(), "second_field");
  EXPECT_EQ(second.type.cc_name, "int");
  EXPECT_EQ(second.type.rs_name, "i32");
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
                        "  char f0;"
                        "  short f1;"
                        "  int f2;"
                        "  long f3;"
                        "  unsigned char f4;"
                        "  unsigned short f5;"
                        "  unsigned int f6;"
                        "  unsigned long f7;"
                        "  signed char f8;"
                        "  signed short f9;"
                        "  signed int f10;"
                        "  signed long f11;"
                        "  int8_t f12;"
                        "  int16_t f13;"
                        "  int32_t f14;"
                        "  int64_t f15;"
                        "  uint8_t f16;"
                        "  uint16_t f17;"
                        "  uint32_t f18;"
                        "  uint64_t f19;"
                        "};"},
                       {});
  auto fields = ir.records[0].fields;

  EXPECT_EQ(fields[0].type.rs_name, "i8");
  EXPECT_EQ(fields[0].type.cc_name, "char");
  EXPECT_EQ(fields[1].type.rs_name, "i16");
  EXPECT_EQ(fields[1].type.cc_name, "short");
  EXPECT_EQ(fields[2].type.rs_name, "i32");
  EXPECT_EQ(fields[2].type.cc_name, "int");
  EXPECT_EQ(fields[3].type.rs_name, "i64");
  EXPECT_EQ(fields[3].type.cc_name, "long");

  EXPECT_EQ(fields[4].type.rs_name, "u8");
  EXPECT_EQ(fields[4].type.cc_name, "unsigned char");
  EXPECT_EQ(fields[5].type.rs_name, "u16");
  EXPECT_EQ(fields[5].type.cc_name, "unsigned short");
  EXPECT_EQ(fields[6].type.rs_name, "u32");
  EXPECT_EQ(fields[6].type.cc_name, "unsigned int");
  EXPECT_EQ(fields[7].type.rs_name, "u64");
  EXPECT_EQ(fields[7].type.cc_name, "unsigned long");

  EXPECT_EQ(fields[8].type.rs_name, "i8");
  EXPECT_EQ(fields[8].type.cc_name, "signed char");
  EXPECT_EQ(fields[9].type.rs_name, "i16");
  EXPECT_EQ(fields[9].type.cc_name, "short");
  EXPECT_EQ(fields[10].type.rs_name, "i32");
  EXPECT_EQ(fields[10].type.cc_name, "int");
  EXPECT_EQ(fields[11].type.rs_name, "i64");
  EXPECT_EQ(fields[11].type.cc_name, "long");

  EXPECT_EQ(fields[12].type.rs_name, "i8");
  EXPECT_EQ(fields[12].type.cc_name, "int8_t");
  EXPECT_EQ(fields[13].type.rs_name, "i16");
  EXPECT_EQ(fields[13].type.cc_name, "int16_t");
  EXPECT_EQ(fields[14].type.rs_name, "i32");
  EXPECT_EQ(fields[14].type.cc_name, "int32_t");
  EXPECT_EQ(fields[15].type.rs_name, "i64");
  EXPECT_EQ(fields[15].type.cc_name, "int64_t");

  EXPECT_EQ(fields[16].type.rs_name, "u8");
  EXPECT_EQ(fields[16].type.cc_name, "uint8_t");
  EXPECT_EQ(fields[17].type.rs_name, "u16");
  EXPECT_EQ(fields[17].type.cc_name, "uint16_t");
  EXPECT_EQ(fields[18].type.rs_name, "u32");
  EXPECT_EQ(fields[18].type.cc_name, "uint32_t");
  EXPECT_EQ(fields[19].type.rs_name, "u64");
  EXPECT_EQ(fields[19].type.cc_name, "uint64_t");
}

}  // namespace
}  // namespace rs_bindings_from_cc
