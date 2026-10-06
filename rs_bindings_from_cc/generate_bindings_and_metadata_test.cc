// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include <string>
#include <utility>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_replace.h"
#include "absl/strings/str_split.h"
#include "absl/strings/string_view.h"
#include "common/external_binaries.h"
#include "common/ffi_types.h"
#include "common/file_io.h"
#include "common/status_macros.h"
#include "common/test_utils.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/collect_namespaces.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {
namespace {

using ::testing::ElementsAre;
using ::testing::EndsWith;
using ::testing::HasSubstr;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::Pair;
using ::testing::StrEq;

/// Returns a Cmdline for the given header.
Cmdline MakeCmdline(std::string header) {
  auto args = CmdlineArgs{
      .current_target = BazelLabel("//:target"),
      .cc_out = "cc_out",
      .rs_out = "rs_out",
      .ir_out = "ir_out",
      .namespaces_out = "namespaces_out",
      .crubit_support_path_format = "<crubit/support/path/{header}>",
      .clang_format_exe_path = std::string(ClangFormatExePath()),
      .rustfmt_exe_path = std::string(RustfmtExePath()),
      .rustfmt_config_path = "nowhere/rustfmt.toml",
      .is_golden_test = false,
      .public_headers = {HeaderName(header)},
  };
  args.headers_to_targets[args.public_headers[0]] = args.current_target;
  args.target_to_features[args.current_target] = {"supported"};
  absl::StatusOr<Cmdline> cmdline = Cmdline::Create(args);
  CHECK_OK(cmdline);
  return *cmdline;
}

TEST(GenerateBindingsAndMetadataTest, GeneratingIR) {
  Cmdline cmdline = MakeCmdline("a.h");

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs(),
                                  /*virtual_headers_contents_for_testing=*/
                                  {{HeaderName("a.h"), "namespace ns{}"}}));

  ASSERT_EQ(result.ir.public_headers_size(), 1);
  ASSERT_EQ(result.ir.public_headers(0).name(), "a.h");
  ASSERT_EQ(result.error_report, "");

  // Check that IR items have the proper owning target set.
  auto item = get_items_if<ir_proto::Namespace>(result.ir).front();
  ASSERT_EQ(item->owning_target(), "//:target");
}

TEST(GenerateBindingsAndMetadataTest, InstantiationsAreEmptyInNormalMode) {
  Cmdline cmdline = MakeCmdline("a.h");

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs(),
                                  /*virtual_headers_contents_for_testing=*/
                                  {{HeaderName("a.h"), "// empty header"}}));

  ASSERT_THAT(result.instantiations, IsEmpty());
}

absl::StatusOr<absl::flat_hash_map<Identifier, Identifier>>
GetInstantiationsFor(absl::string_view header_content,
                     absl::string_view rust_source) {
  std::string a_rs_path = WriteFileForCurrentTest("a.rs", rust_source);
  CmdlineArgs args = MakeCmdline("a.h").args();
  args.srcs_to_scan_for_instantiations = {a_rs_path};
  args.instantiations_out = "instantiations_out";
  absl::StatusOr<Cmdline> cmdline = Cmdline::Create(args);
  CHECK_OK(cmdline);

  CRUBIT_ASSIGN_OR_RETURN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(
          *cmdline, DefaultClangArgs(),
          /*virtual_headers_contents_for_testing=*/
          {{HeaderName("a.h"), std::string(header_content)}}));

  return std::move(result.instantiations);
}

TEST(GenerateBindingsAndMetadataTest,
     RegularTypeAliasNotPresentInInstantiations) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             template <typename T>
                             class MyTemplate {};

                             using MyFunnyTemplate = MyTemplate<bool>;

                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(
      instantiations,
      ElementsAre(Pair(Identifier("ExpectedTemplate<bool>"),
                       Identifier("__CcTemplateInst16ExpectedTemplateIbE"))));
}

TEST(GenerateBindingsAndMetadataTest,
     ExplicitClassTemplateInstantiationDeclarationsNotPresentInInstantiations) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             template <typename T>
                             class MyTemplate {};

                             extern template class MyTemplate<bool>;

                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(
      instantiations,
      ElementsAre(Pair(Identifier("ExpectedTemplate<bool>"),
                       Identifier("__CcTemplateInst16ExpectedTemplateIbE"))));
}

TEST(GenerateBindingsAndMetadataTest,
     ExplicitClassTemplateInstantiationDefinitionsNotPresentInInstantiations) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             template <typename T>
                             class MyTemplate {};

                             template class MyTemplate<bool>;

                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(
      instantiations,
      ElementsAre(Pair(Identifier("ExpectedTemplate<bool>"),
                       Identifier("__CcTemplateInst16ExpectedTemplateIbE"))));
}

TEST(GenerateBindingsAndMetadataTest,
     RegularRecordsNotPresentInInstantiations) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             struct MyStruct {};

                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(
      instantiations,
      ElementsAre(Pair(Identifier("ExpectedTemplate<bool>"),
                       Identifier("__CcTemplateInst16ExpectedTemplateIbE"))));
}

TEST(GenerateBindingsAndMetadataTest,
     InstantiationsAreGeneratedForCcTemplateMacro) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(
      instantiations,
      ElementsAre(Pair(Identifier("ExpectedTemplate<bool>"),
                       Identifier("__CcTemplateInst16ExpectedTemplateIbE"))));
}

TEST(GenerateBindingsAndMetadataTest,
     InstantiationsNamespaceIsNotEmittedAsRustModule) {
  std::string a_rs_path =
      WriteFileForCurrentTest("a.rs", "cc_template!{ExpectedTemplate<bool>}");
  for (bool lazy_import_alien_decls : {false, true}) {
    SCOPED_TRACE(lazy_import_alien_decls ? "lazy_import_alien_decls=true"
                                         : "lazy_import_alien_decls=false");
    CmdlineArgs args = MakeCmdline("a.h").args();
    args.srcs_to_scan_for_instantiations = {a_rs_path};
    args.instantiations_out = "instantiations_out";
    args.lazy_import_alien_decls = lazy_import_alien_decls;
    args.check_importer_invariants = true;
    ASSERT_OK_AND_ASSIGN(Cmdline cmdline, Cmdline::Create(args));

    ASSERT_OK_AND_ASSIGN(
        BindingsAndMetadata result,
        GenerateBindingsAndMetadata(cmdline, DefaultClangArgs(),
                                    /*virtual_headers_contents_for_testing=*/
                                    {{HeaderName("a.h"), R"cc(
                                        template <typename T>
                                        class ExpectedTemplate {};
                                      )cc"}}));

    // The instantiation itself is still collected...
    EXPECT_THAT(
        result.instantiations,
        ElementsAre(Pair(Identifier("ExpectedTemplate<bool>"),
                         Identifier("__CcTemplateInst16ExpectedTemplateIbE"))));
    // ...but the synthetic namespace that holds the instantiation aliases is
    // an implementation detail and must not show up as a public module or in
    // the namespaces metadata.
    EXPECT_THAT(result.rs_api,
                Not(HasSubstr("mod __cc_template_instantiations")));
    EXPECT_THAT(NamespacesAsJson(result.namespaces),
                Not(HasSubstr("__cc_template_instantiations")));
  }
}

TEST(GenerateBindingsAndMetadataTest, NamespacesJsonGenerated) {
  constexpr absl::string_view kHeaderContent = R"(
    namespace top_level_1 {
      namespace middle {
        namespace inner_1 {}
      }
      namespace middle {
        namespace inner_2 {}
      }
    }

    namespace top_level_2 {
      namespace inner_3 {}
    }

    namespace top_level_1 {}
  )";
  constexpr absl::string_view kExpected = R"({
  "label": "//:target",
  "namespaces": [
    {
      "children": [
        {
          "children": [
            {
              "children": [],
              "name": "inner_1"
            },
            {
              "children": [],
              "name": "inner_2"
            }
          ],
          "name": "middle"
        }
      ],
      "name": "top_level_1"
    },
    {
      "children": [
        {
          "children": [],
          "name": "inner_3"
        }
      ],
      "name": "top_level_2"
    }
  ]
})";

  Cmdline cmdline = MakeCmdline("a.h");
  ASSERT_OK_AND_ASSIGN(BindingsAndMetadata result,
                       GenerateBindingsAndMetadata(
                           cmdline, DefaultClangArgs(),
                           /*virtual_headers_contents_for_testing=*/
                           {{HeaderName("a.h"), std::string(kHeaderContent)}}));

  ASSERT_THAT(NamespacesAsJson(result.namespaces), StrEq(kExpected));
}

TEST(GenerateBindingsAndMetadataTest,
     ExtraCppSrcsNotIncludedTwiceWhenPassedViaMinusInclude) {
  std::string extra_cc_path =
      WriteFileForCurrentTest("extra.cc", "struct UnguardedStruct {};");
  CmdlineArgs args = MakeCmdline("a.h").args();
  args.extra_cpp_srcs = {extra_cc_path};
  absl::StatusOr<Cmdline> cmdline = Cmdline::Create(args);
  CHECK_OK(cmdline);

  std::vector<std::string> clang_args = DefaultClangArgs();
  clang_args.push_back("-include");
  clang_args.push_back(extra_cc_path);

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(*cmdline, std::move(clang_args),
                                  /*virtual_headers_contents_for_testing=*/
                                  {{HeaderName("a.h"), "// empty header"}}));
  ASSERT_EQ(result.error_report, "");
}

TEST(GenerateBindingsAndMetadataTest, DepfileOmitsInMemoryFiles) {
  WriteFileForCurrentTest("included.h", "struct Included {};");
  std::string public_h =
      WriteFileForCurrentTest("public.h", "#include \"included.h\"\n");
  std::string depfile_path = absl::StrCat(testing::TempDir(), "/rs_api.d");
  std::vector<std::string> clang_args = DefaultClangArgs();
  clang_args.insert(clang_args.end(),
                    {"-MD", "-MF", depfile_path, "-MT", "path/to/rs_api.rs"});

  // Both `in_memory.h` and the main file synthesized by `IrFromCc` exist only
  // in Clang's in-memory file system.
  Cmdline cmdline = MakeCmdline("in_memory.h");
  ASSERT_OK(GenerateBindingsAndMetadata(
                cmdline, std::move(clang_args),
                /*virtual_headers_contents_for_testing=*/
                {{HeaderName("in_memory.h"),
                  absl::StrCat("#include \"", public_h, "\"\n")}})
                .status());

  ASSERT_OK_AND_ASSIGN(std::string depfile, GetFileContents(depfile_path));
  // Clang wraps long depfile lines using `\` + newline continuations.
  std::vector<std::string> depfile_tokens =
      absl::StrSplit(absl::StrReplaceAll(depfile, {{"\\\n", " "}}),
                     absl::ByAnyChar(" \n"), absl::SkipEmpty());
  EXPECT_THAT(depfile_tokens,
              ElementsAre("path/to/rs_api.rs:", EndsWith("/public.h"),
                          EndsWith("/included.h")));
}

}  // namespace
}  // namespace crubit
