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

using ::testing::IsEmpty;
using ::testing::SizeIs;

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
  ASSERT_THAT(ir.used_headers, SizeIs(1));
  EXPECT_EQ(ir.used_headers[0].IncludePath(), "test/testing_header_0.h");
}

TEST(AstVisitorTest, IREmptyOnInvalidInput) {
  IR ir = ImportCode({"int foo(); But this is not C++"}, {});
  EXPECT_THAT(ir.functions, IsEmpty());
}

TEST(AstVisitorTest, FuncWithVoidReturnType) {
  IR ir = ImportCode({"void Foo();"}, {});
  ASSERT_THAT(ir.functions, SizeIs(1));
  Func func = ir.functions[0];
  EXPECT_EQ(func.identifier.Ident(), "Foo");
  EXPECT_EQ(func.mangled_name, "_Z3Foov");
  EXPECT_TRUE(func.return_type.IsVoid());
  EXPECT_THAT(func.params, IsEmpty());
}

TEST(AstVisitorTest, TwoFuncs) {
  IR ir = ImportCode({"void Foo(); void Bar();"}, {});
  ASSERT_THAT(ir.functions, SizeIs(2));

  Func foo = ir.functions[0];
  EXPECT_EQ(foo.identifier.Ident(), "Foo");
  EXPECT_EQ(foo.mangled_name, "_Z3Foov");
  EXPECT_TRUE(foo.return_type.IsVoid());
  EXPECT_THAT(foo.params, IsEmpty());

  Func bar = ir.functions[1];
  EXPECT_EQ(bar.identifier.Ident(), "Bar");
  EXPECT_EQ(bar.mangled_name, "_Z3Barv");
  EXPECT_TRUE(bar.return_type.IsVoid());
  EXPECT_THAT(bar.params, IsEmpty());
}

TEST(AstVisitorTest, TwoFuncsFromTwoHeaders) {
  IR ir = ImportCode({"void Foo();", "void Bar();"}, {});
  ASSERT_THAT(ir.functions, SizeIs(2));
  Func foo = ir.functions[0];
  EXPECT_EQ(foo.identifier.Ident(), "Foo");
  Func bar = ir.functions[1];
  EXPECT_EQ(bar.identifier.Ident(), "Bar");
}

TEST(AstVisitorTest, NonInlineFunc) {
  IR ir = ImportCode({"void Foo() {}"}, {});
  ASSERT_THAT(ir.functions, SizeIs(1));
  Func func = ir.functions[0];
  EXPECT_EQ(func.identifier.Ident(), "Foo");
  EXPECT_FALSE(func.is_inline);
}

TEST(AstVisitorTest, InlineFunc) {
  IR ir = ImportCode({"inline void Foo() {}"}, {});
  ASSERT_THAT(ir.functions, SizeIs(1));
  Func func = ir.functions[0];
  EXPECT_EQ(func.identifier.Ident(), "Foo");
  EXPECT_TRUE(func.is_inline);
}

TEST(AstVisitorTest, FuncJustOnce) {
  IR ir = ImportCode({"void Foo(); void Foo();"}, {});
  ASSERT_THAT(ir.functions, SizeIs(1));
  Func func = ir.functions[0];
  EXPECT_EQ(func.identifier.Ident(), "Foo");
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
  EXPECT_THAT(ir.functions, SizeIs(0));

  EXPECT_THAT(ir.records, SizeIs(1));
  Record some_struct = ir.records[0];

  EXPECT_EQ(some_struct.Ident().Ident(), "SomeStruct");

  ASSERT_THAT(some_struct.Fields(), SizeIs(2));
  Field first = some_struct.Fields()[0];
  Field second = some_struct.Fields()[1];

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
  EXPECT_THAT(ir.functions, SizeIs(0));
  ASSERT_THAT(ir.records, SizeIs(2));

  Record some_struct = ir.records[0];
  EXPECT_EQ(some_struct.Ident().Ident(), "SomeStruct");
  ASSERT_THAT(some_struct.Fields(), SizeIs(4));
  Field field0 = some_struct.Fields()[0];
  EXPECT_EQ(field0.identifier.Ident(), "default_access_int");
  EXPECT_EQ(field0.access, kPublic);
  Field field1 = some_struct.Fields()[1];
  EXPECT_EQ(field1.identifier.Ident(), "public_int");
  EXPECT_EQ(field1.access, kPublic);
  Field field2 = some_struct.Fields()[2];
  EXPECT_EQ(field2.identifier.Ident(), "protected_int");
  EXPECT_EQ(field2.access, kProtected);
  Field field3 = some_struct.Fields()[3];
  EXPECT_EQ(field3.identifier.Ident(), "private_int");
  EXPECT_EQ(field3.access, kPrivate);

  Record some_class = ir.records[1];
  EXPECT_EQ(some_class.Ident().Ident(), "SomeClass");
  ASSERT_THAT(some_class.Fields(), SizeIs(1));
  field0 = some_class.Fields()[0];
  EXPECT_EQ(field0.identifier.Ident(), "default_access_int");
  EXPECT_EQ(field0.access, kPrivate);
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
  auto fields = ir.records[0].Fields();

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
