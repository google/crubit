// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/collect_namespaces.h"

#include <string_view>
#include <vector>

#include "absl/container/btree_map.h"
#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "llvm/Support/JSON.h"

namespace crubit {
namespace {

// A Trie that stores the namespace hierarchy.
//
// Consider the following namespace structure:
// namespace top_level_one {
// namespace internal {}
// }  // namespace A
//
// namespace top_level_two {
// namespace internal {}
// }
//
// namespace top_level_one {
// namespace inner {}
// }
//
// A Namespace trie allows us to convert the above to:
//
// top_level_one -> [internal, inner]
// top_level_two -> [internal]
//
// Which we can then serialize to JSON.
class NamespaceTrie {
 private:
  struct Node {
    absl::string_view name;
    // A map of child namespace name to the index of the namespace node in the
    // trie_nodes_ vector. We use a btree_map so that at conversion time we get
    // deterministic JSON output.
    absl::btree_map<absl::string_view, int> child_name_to_idx;
  };

  std::vector<Node> trie_nodes_;
  // A map of top level namespace name to the index of the namespace node in the
  // trie_nodes_ vector. We use a btree_map so that at conversion time we get
  // deterministic JSON output.
  absl::btree_map<absl::string_view, int> top_level_name_to_idx_;
  // The current target's label.
  BazelLabel label_;
  // A map of item id to the IR Namespace item. It allows us to look up the
  // children namespace items.
  absl::flat_hash_map<ItemId, const Namespace*>& id_to_namespace_;

  // Creates a node from a Namespace and inserts it into the trie.
  void InsertNode(int parent_idx, const Namespace* ns) {
    auto name = ns->name.Ident();
    auto parent = &trie_nodes_[parent_idx];
    int child_idx;
    if (parent->child_name_to_idx.find(name) ==
        parent->child_name_to_idx.end()) {
      child_idx = trie_nodes_.size();
      parent->child_name_to_idx.insert({name, child_idx});
      // The following line potentially invalidates the "parent" pointer.
      trie_nodes_.push_back({name, {}});
    } else {
      child_idx = parent->child_name_to_idx[name];
    }

    for (auto ns_child_id : ns->child_item_ids) {
      if (id_to_namespace_.find(ns_child_id) == id_to_namespace_.end()) {
        continue;
      }
      auto ns_child = id_to_namespace_[ns_child_id];
      InsertNode(child_idx, ns_child);
    }
  }

  // Converts a trie node into the JSON serializable NamespaceNode.
  NamespaceNode NodeToNamespaceNode(const Node* node) const {
    std::vector<NamespaceNode> namespaces;
    namespaces.reserve(node->child_name_to_idx.size());
    for (const auto& [_, idx] : node->child_name_to_idx) {
      namespaces.push_back(NodeToNamespaceNode(&trie_nodes_[idx]));
    }
    return NamespaceNode{std::string(node->name), std::move(namespaces)};
  }

 public:
  NamespaceTrie(BazelLabel label,
                absl::flat_hash_map<ItemId, const Namespace*>& id_to_namespace)
      : label_(label), id_to_namespace_(id_to_namespace) {}

  NamespaceTrie(NamespaceTrie&) = delete;
  NamespaceTrie& operator=(NamespaceTrie&) = delete;

  // Creates a trie node from the top level namespace and inserts it into the
  // trie.
  void InsertTopLevel(const Namespace* ns) {
    auto name = ns->name.Ident();
    int node_idx;
    if (top_level_name_to_idx_.find(name) == top_level_name_to_idx_.end()) {
      node_idx = trie_nodes_.size();
      top_level_name_to_idx_.insert({name, node_idx});
      trie_nodes_.push_back({name, {}});
    } else {
      node_idx = top_level_name_to_idx_[name];
    }

    for (auto ns_child_id : ns->child_item_ids) {
      if (id_to_namespace_.find(ns_child_id) == id_to_namespace_.end()) {
        continue;
      }
      auto ns_child = id_to_namespace_[ns_child_id];
      InsertNode(node_idx, ns_child);
    }
  }

  // Converts the trie into the JSON serializable NamespacesHierarchy.
  NamespacesHierarchy ToNamespacesHierarchy() {
    std::vector<NamespaceNode> namespaces;
    namespaces.reserve(this->top_level_name_to_idx_.size());
    for (auto& [_, idx] : this->top_level_name_to_idx_) {
      namespaces.push_back(NodeToNamespaceNode(&trie_nodes_[idx]));
    }
    return NamespacesHierarchy{label_, std::move(namespaces)};
  }
};

}  // namespace

// Returns the current target's namespace hierarchy in JSON serializable format.
NamespacesHierarchy CollectNamespaces(const IR& ir) {
  auto all_namespaces = ir.get_items_if<Namespace>();
  absl::flat_hash_map<ItemId, const Namespace*> id_to_namespace;
  for (auto ns : all_namespaces) {
    // We are not interested in namespaces from different targets.
    if (ns->owning_target != ir.current_target) {
      continue;
    }
    id_to_namespace.insert({ns->id, ns});
  }

  NamespaceTrie trie(ir.current_target, id_to_namespace);
  for (auto namespace_id : ir.top_level_item_ids) {
    if (id_to_namespace.count(namespace_id) == 0) {
      continue;
    }
    auto ns = id_to_namespace[namespace_id];
    trie.InsertTopLevel(ns);
  }

  return trie.ToNamespacesHierarchy();
}

llvm::json::Value NamespaceNode::ToJson() const {
  std::vector<llvm::json::Value> json_children;
  json_children.reserve(children.size());
  for (const auto& child : children) {
    json_children.push_back(child.ToJson());
  }

  return llvm::json::Object{
      {"name", name},
      {"children", std::move(json_children)},
  };
}

llvm::json::Value NamespacesHierarchy::ToJson() const {
  std::vector<llvm::json::Value> json_namespaces;
  json_namespaces.reserve(namespaces.size());
  for (const auto& ns : namespaces) {
    json_namespaces.push_back(ns.ToJson());
  }

  return llvm::json::Object{
      {"label", label.value()},
      {"namespaces", std::move(json_namespaces)},
  };
}

}  //  namespace crubit
