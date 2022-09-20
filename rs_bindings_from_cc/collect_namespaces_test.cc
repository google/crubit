// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/collect_namespaces.h"

#include "gmock/gmock.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"

namespace crubit {
namespace {

using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::StrEq;

MATCHER_P(NameIs, name, "") { return arg.name == name; }

TEST(CollectNamespacesTest, Namespaces) {
  absl::string_view file = R"(
    namespace top_level_one {
      namespace inner {}
    }
    namespace top_level_two {}
  )";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  auto namespace_hierarchy = CollectNamespaces(ir);
  auto top_level_namespaces = namespace_hierarchy.namespaces;

  ASSERT_THAT(top_level_namespaces.size(), 2);
  EXPECT_THAT(top_level_namespaces,
              ElementsAre(NameIs("top_level_one"), NameIs("top_level_two")));

  ASSERT_THAT(top_level_namespaces[0].children.size(), 1);
  EXPECT_THAT(top_level_namespaces[0].children[0], NameIs("inner"));

  ASSERT_THAT(namespace_hierarchy.label.value(),
              StrEq("//test:testing_target"));
}

TEST(CollectNamespacesTest, ReopenedNamespaces) {
  absl::string_view file = R"(
    namespace top_level {
      namespace inner {}
    }
    namespace top_level {
      namespace inner {}
    }
  )";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  auto top_level_namespaces = CollectNamespaces(ir).namespaces;

  ASSERT_THAT(top_level_namespaces.size(), 1);
  EXPECT_THAT(top_level_namespaces, ElementsAre(NameIs("top_level")));
  EXPECT_THAT(top_level_namespaces[0].children, ElementsAre(NameIs("inner")));
}

TEST(CollectNamespacesTest, InlineNamespaces) {
  absl::string_view file = R"(
    inline namespace top_level {
      inline namespace inner {}
    }
  )";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  auto top_level_namespaces = CollectNamespaces(ir).namespaces;

  ASSERT_THAT(top_level_namespaces.size(), 1);
  EXPECT_THAT(top_level_namespaces, ElementsAre(NameIs("top_level")));
  EXPECT_THAT(top_level_namespaces[0].children, ElementsAre(NameIs("inner")));
}

TEST(CollectNamespacesTest, AnonymousNamespaces) {
  absl::string_view file = R"(
    namespace {}
  )";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  auto top_level_namespaces = CollectNamespaces(ir).namespaces;

  EXPECT_THAT(top_level_namespaces, IsEmpty());
}

TEST(CollectNamespacesTest, SameNameDifferentParent) {
  absl::string_view file = R"(
    namespace top_level_one {
      namespace middle {
        namespace inner_one {}
      }
    }
    namespace top_level_two {
      namespace middle {
        namespace inner_two {}
      }
    }
  )";
  ASSERT_OK_AND_ASSIGN(IR ir, IrFromCc(file));

  auto top_level_namespaces = CollectNamespaces(ir).namespaces;

  ASSERT_THAT(top_level_namespaces.size(), 2);
  EXPECT_THAT(top_level_namespaces,
              ElementsAre(NameIs("top_level_one"), NameIs("top_level_two")));

  ASSERT_THAT(top_level_namespaces[0].children.size(), 1);
  EXPECT_THAT(top_level_namespaces[0].children, ElementsAre(NameIs("middle")));

  auto should_contain_only_d = top_level_namespaces[0].children[0].children;
  ASSERT_THAT(should_contain_only_d.size(), 1);
  EXPECT_THAT(should_contain_only_d[0], NameIs("inner_one"));
}

}  // namespace
}  // namespace crubit
