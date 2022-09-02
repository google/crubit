// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_COLLECT_NAMESPACES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_COLLECT_NAMESPACES_H_

#include <string>

#include "rs_bindings_from_cc/ir.h"
#include "llvm/Support/JSON.h"

namespace crubit {

// Representation of a C++ namespace for JSON serialization.
// This structure differs from the Namespace struct in ir.h in that it only
// stores the names of the namespace children, as it is the only information
// that the cc_import! macro needs in order to be able to merge namespaces
// across targets.
struct NamespaceNode {
  llvm::json::Value ToJson() const;

  std::string name;
  std::vector<NamespaceNode> children;
};

inline std::ostream& operator<<(std::ostream& o, const NamespaceNode& ns) {
  return o << std::string(llvm::formatv("{0:2}", ns.ToJson()));
}

// Representation of all C++ namespaces within the current target.
struct NamespacesHierarchy {
  llvm::json::Value ToJson() const;

  std::vector<NamespaceNode> namespaces;
};

inline std::ostream& operator<<(std::ostream& o,
                                const NamespacesHierarchy& all) {
  return o << std::string(llvm::formatv("{0:2}", all.ToJson()));
}

// Returns the current target's namespace hierarchy in JSON serializable format.
NamespacesHierarchy CollectNamespaces(const IR& ir);

inline std::string NamespacesAsJson(const NamespacesHierarchy& topLevel) {
  return llvm::formatv("{0:2}", topLevel.ToJson());
}
}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_COLLECT_NAMESPACES_H_
