// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

export interface DoxygenSymbol {
  readonly name: string;
  readonly kind: string;
  readonly refid: string;
  readonly description?: string;
}

export interface DoxygenRequest {
  readonly input: {
    files: Array<{name: string; contentsB64: string}>;
  };
}

export interface DoxygenResponse {
  readonly xmlOutput?: {
    files: Array<{name: string; contentsB64: string}>;
  };
  readonly error?: {
    text: string;
    reason: string;
  };
  readonly fileSymbols?: Record<string, {symbols?: DoxygenSymbol[]}>;
}

export interface FlatSymbolNode {
  readonly name: string;
  readonly fullName: string;
  readonly kind: string;
  readonly refid: string;
  readonly depth: number;
  readonly hasChildren: boolean;
  collapsed: boolean;
  visible: boolean;
}

interface SymbolTreeNode {
  name: string;
  fullName: string;
  kind: string;
  refid: string;
  children: SymbolTreeNode[];
}

export function buildFlatSymbolTree(
  symbols: readonly DoxygenSymbol[],
): FlatSymbolNode[] {
  const rootNodes: SymbolTreeNode[] = [];
  const nodeMap = new Map<string, SymbolTreeNode>();

  function getOrCreateNode(
    fullName: string,
    originalSymbol?: DoxygenSymbol,
  ): SymbolTreeNode {
    const existing = nodeMap.get(fullName);
    if (existing !== undefined) {
      if (originalSymbol && !existing.refid) {
        existing.kind = originalSymbol.kind.toLowerCase();
        existing.refid = originalSymbol.refid;
      }
      return existing;
    }

    const lastColonIdx = fullName.lastIndexOf('::');
    const name =
      lastColonIdx === -1 ? fullName : fullName.substring(lastColonIdx + 2);

    const node: SymbolTreeNode = {
      name,
      fullName,
      kind: originalSymbol ? originalSymbol.kind.toLowerCase() : 'namespace',
      refid: originalSymbol ? originalSymbol.refid : '',
      children: [],
    };

    nodeMap.set(fullName, node);

    if (lastColonIdx === -1) {
      rootNodes.push(node);
    } else {
      const parentFullName = fullName.substring(0, lastColonIdx);
      const parentNode = getOrCreateNode(parentFullName);
      parentNode.children.push(node);
    }

    return node;
  }

  for (const sym of symbols) {
    getOrCreateNode(sym.name, sym);
  }

  for (const node of nodeMap.values()) {
    node.children.sort((a, b) => a.name.localeCompare(b.name));
  }
  rootNodes.sort((a, b) => a.name.localeCompare(b.name));

  return flattenTree(rootNodes);
}

function flattenTree(
  nodes: readonly SymbolTreeNode[],
  depth = 0,
  result: FlatSymbolNode[] = [],
): FlatSymbolNode[] {
  for (const node of nodes) {
    result.push({
      name: node.name,
      fullName: node.fullName,
      kind: node.kind,
      refid: node.refid,
      depth,
      hasChildren: node.children.length > 0,
      collapsed: false,
      visible: true,
    });
    if (node.children.length > 0) {
      flattenTree(node.children, depth + 1, result);
    }
  }
  return result;
}
