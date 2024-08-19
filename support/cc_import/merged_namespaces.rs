// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use import_internal::{AbsoluteLabel, Mode};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::rc::Rc;
use syn::Ident;

/// Representation of a target's namespace hierarchy, as encoded in the
/// *_namespace.json file.
#[derive(Clone, Debug, Deserialize)]
pub struct JsonNamespaceHierarchy {
    pub label: Rc<str>,
    #[serde(default)]
    pub namespaces: Vec<JsonNamespace>,
}

/// Single C++ namespace, as encoded in the *_namespace.json file.
#[derive(Clone, Debug, Deserialize)]
pub struct JsonNamespace {
    pub name: Rc<str>,
    #[serde(default)]
    pub children: Vec<JsonNamespace>,
}

/// A trie-like data structure that represents the merged namespace hierarchy
/// from all the direct dependencies.
#[derive(Clone, Debug)]
pub struct MergedNamespaceHierarchy {
    /// A map of a top level namespace name to the corresponding
    /// MergedNamespace.
    pub top_level_namespaces: BTreeMap<Rc<str>, MergedNamespace>,
}

/// A single C++ namespace potentially reopened across multiple targets.
#[derive(Clone, Debug)]
pub struct MergedNamespace {
    /// name of the current namespace.
    pub name: Rc<str>,
    /// A map of child namespace name to the corresponding MergedNamespace.
    pub children: BTreeMap<Rc<str>, MergedNamespace>,
    /// A set of targets that reopen this namespace.
    pub labels: BTreeSet<Rc<str>>,
}

impl MergedNamespace {
    /// Creates a MergedNamespace from JsonNamespace.
    pub fn from_json_namespace(label: Rc<str>, json_namespace: &JsonNamespace) -> MergedNamespace {
        let mut merged_children = BTreeMap::new();
        for child in json_namespace.children.iter() {
            merged_children.insert(
                child.name.clone(),
                MergedNamespace::from_json_namespace(label.clone(), child),
            );
        }
        MergedNamespace {
            name: json_namespace.name.clone(),
            children: merged_children,
            labels: BTreeSet::from([label]),
        }
    }

    /// Merges the namespace passed as an argument into the current one.
    pub(crate) fn merge(&mut self, other: MergedNamespace) {
        let MergedNamespace { name, children, mut labels } = other;
        assert!(
            self.name == name,
            "Cannot merge namespaces with different names, got '{}' and '{}'",
            &self.name,
            &name
        );
        self.labels.append(&mut labels);
        for namespace in children.into_values() {
            self.add_child(namespace);
        }
    }

    fn add_child(&mut self, namespace: MergedNamespace) {
        self.labels.append(&mut namespace.labels.iter().cloned().collect());
        match self.children.get_mut(&namespace.name) {
            Some(child_namespace) => {
                let MergedNamespace { name: _, children, labels: _ } = namespace;
                for child in children.into_values() {
                    child_namespace.add_child(child);
                }
            }
            None => {
                self.children.insert(namespace.name.clone(), namespace);
            }
        }
    }
}

impl MergedNamespaceHierarchy {
    /// Creates a MergedNamespaceHierarchy from JsonNamespaceHierarchy.
    pub fn from_json_namespace_hierarchy(jnh: &JsonNamespaceHierarchy) -> MergedNamespaceHierarchy {
        let mut merged_namespaces = BTreeMap::new();
        for top_level_namespace in jnh.namespaces.iter() {
            merged_namespaces.insert(
                top_level_namespace.name.clone(),
                MergedNamespace::from_json_namespace(jnh.label.clone(), top_level_namespace),
            );
        }
        MergedNamespaceHierarchy { top_level_namespaces: merged_namespaces }
    }

    // Merges the namespace hierarchy passed as an argument into the current one.
    pub fn merge(&mut self, other: MergedNamespaceHierarchy) {
        for (name, namespace) in other.top_level_namespaces {
            match self.top_level_namespaces.get_mut(&name) {
                Some(child_namespace) => {
                    child_namespace.merge(namespace);
                }
                None => {
                    self.top_level_namespaces.insert(name, namespace);
                }
            }
        }
    }
}

impl ToTokens for MergedNamespaceHierarchy {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for namespace in self.top_level_namespaces.values() {
            to_use_stmts(namespace, tokens, &mut vec![]);
        }
    }
}

fn to_use_stmts(
    namespace: &MergedNamespace,
    tokens: &mut TokenStream,
    outer_namespaces: &mut Vec<Ident>,
) {
    let module_name = Ident::new(&namespace.name, Span::call_site());

    let mut inner_tokens = TokenStream::new();
    for label_literal in namespace.labels.iter() {
        let span = Span::call_site();
        let label = AbsoluteLabel::parse(label_literal, &span).expect("Couldn't parse label");
        // TODO(rosica): Use mangled name for the crate.
        let crate_name = &label.crate_name(&Mode::NoRenaming);
        let crate_ident = Ident::new(crate_name, span);
        inner_tokens.append_all(quote! {
            pub use #crate_ident::#(#outer_namespaces::)*#module_name::*;
        });
    }
    outer_namespaces.push(module_name);
    for inner in namespace.children.values() {
        to_use_stmts(inner, &mut inner_tokens, outer_namespaces);
    }
    let module_name = outer_namespaces.pop();

    tokens.extend(quote! {
        pub mod #module_name {
            #inner_tokens
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use std::ops::Deref;

    #[gtest]
    fn from_json() {
        let json = r#"{
            "label": "//foo/bar:baz",
            "namespaces": [
                {
                    "name": "top_level",
                    "children": []
                }
            ]
        }"#;

        let ns: JsonNamespaceHierarchy = serde_json::from_str(json).unwrap();
        assert_eq!(&*ns.label, "//foo/bar:baz");
        assert_eq!(ns.namespaces.len(), 1);
        assert_eq!(&*ns.namespaces[0].name, "top_level");
    }

    #[gtest]
    fn test_merge_namespace_hierarchies() {
        let hierarchy_one: JsonNamespaceHierarchy = serde_json::from_str(
            r#"{
            "label": "//foo/bar:baz",
            "namespaces": [
                {
                    "name": "top_level_1",
                    "children": []
                }
            ]
        }"#,
        )
        .unwrap();

        let hierarchy_two: JsonNamespaceHierarchy = serde_json::from_str(
            r#"{
            "label": "//foo/bar:xyz",
            "namespaces": [
                {
                    "name": "top_level_1",
                    "children": []
                },
                {
                    "name": "top_level_2",
                    "children": []
                }
            ]
        }"#,
        )
        .unwrap();

        let mut merged_hierarchy =
            MergedNamespaceHierarchy::from_json_namespace_hierarchy(&hierarchy_one);
        let merged_hierarchy_two =
            MergedNamespaceHierarchy::from_json_namespace_hierarchy(&hierarchy_two);

        merged_hierarchy.merge(merged_hierarchy_two);
        let MergedNamespaceHierarchy { top_level_namespaces } = merged_hierarchy;

        assert_eq!(top_level_namespaces.len(), 2);

        let MergedNamespace { name, children: _, labels } =
            top_level_namespaces.get("top_level_1").unwrap();

        assert_eq!(&**name, "top_level_1");
        assert_eq!(
            labels.iter().map(Deref::deref).collect::<Vec<_>>(),
            ["//foo/bar:baz", "//foo/bar:xyz"]
        );

        let MergedNamespace { name, children: _, labels } =
            top_level_namespaces.get("top_level_2").unwrap();

        assert_eq!(&**name, "top_level_2");
        assert_eq!(labels.iter().map(Deref::deref).collect::<Vec<_>>(), ["//foo/bar:xyz"]);
    }

    #[gtest]
    #[should_panic(expected = "Cannot merge namespaces with different names, got 'a' and 'b'")]
    fn test_merge_different_namespaces() {
        let mut namespace_one = MergedNamespace::from_json_namespace(
            "//label1".into(),
            &serde_json::from_str(
                r#"{
            "name": "a",
            "children": []
        }"#,
            )
            .unwrap(),
        );

        let namespace_two = MergedNamespace::from_json_namespace(
            "//label2".into(),
            &serde_json::from_str(
                r#"{
            "name": "b",
            "children": []
        }"#,
            )
            .unwrap(),
        );

        namespace_one.merge(namespace_two);
    }

    #[gtest]
    fn test_merge_namespaces() {
        let json_namespace_one: JsonNamespace = serde_json::from_str(
            r#"{
            "name": "a",
            "children": [
                {
                    "name": "b",
                    "children": []
                },
                {
                    "name": "c",
                    "children": []
                }
            ]
        }"#,
        )
        .unwrap();

        let json_namespace_two: JsonNamespace = serde_json::from_str(
            r#"{
            "name": "a",
            "children": [
                {
                    "name": "c",
                    "children": [
                        {
                            "name": "d",
                            "children": []
                        }
                    ]
                },
                {
                    "name": "d",
                    "children": []
                }
            ]
        }"#,
        )
        .unwrap();

        let mut merged_namespace =
            MergedNamespace::from_json_namespace("//:label1".into(), &json_namespace_one);
        let merged_namespace_two =
            MergedNamespace::from_json_namespace("//:label2".into(), &json_namespace_two);

        merged_namespace.merge(merged_namespace_two);
        let MergedNamespace { name, children, labels } = merged_namespace;

        assert_eq!(&*name, "a");
        assert_eq!(labels.iter().map(Deref::deref).collect::<Vec<_>>(), ["//:label1", "//:label2"]);
        assert_eq!(children.keys().map(Deref::deref).collect::<Vec<_>>(), ["b", "c", "d"]);

        let MergedNamespace { name: _, children: _, labels: b_labels } = children.get("b").unwrap();
        // Namespace b only exists in the first target
        assert_eq!(b_labels.iter().map(Deref::deref).collect::<Vec<_>>(), ["//:label1"]);

        let MergedNamespace { name: _, children: children_c, labels: c_labels } =
            children.get("c").unwrap();
        // Namespace c exists in both targets, however its child d only exists in the
        // second target
        assert_eq!(
            c_labels.iter().map(Deref::deref).collect::<Vec<_>>(),
            ["//:label1", "//:label2"]
        );
        let MergedNamespace { name: _, children: _, labels: c_child_labels } =
            children_c.get("d").unwrap();
        assert_eq!(c_child_labels.iter().map(Deref::deref).collect::<Vec<_>>(), ["//:label2"]);

        let MergedNamespace { name: _, children: _, labels: d_labels } = children.get("d").unwrap();
        // Namespace d only exists in the second target
        assert_eq!(d_labels.iter().map(Deref::deref).collect::<Vec<_>>(), ["//:label2"]);
    }

    #[gtest]
    fn test_to_tokens() {
        let hierarchy_one: JsonNamespaceHierarchy = serde_json::from_str(
            r#"{
            "label": "//foo/bar:baz",
            "namespaces": [
                {
                    "name": "top_level_1",
                    "children": [
                        {
                            "name": "reopened",
                            "children": [
                                {
                                    "name": "baz_specific",
                                    "children": []
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#,
        )
        .unwrap();

        let hierarchy_two: JsonNamespaceHierarchy = serde_json::from_str(
            r#"{
            "label": "//foo/bar:xyz",
            "namespaces": [
                {
                    "name": "top_level_1",
                    "children": [
                        {
                            "name": "xyz_specific",
                            "children": []
                        },
                        {
                            "name": "reopened",
                            "children": []
                        }
                    ]
                },
                {
                    "name": "top_level_2",
                    "children": []
                }
            ]
        }"#,
        )
        .unwrap();

        let mut merged_hierarchy =
            MergedNamespaceHierarchy::from_json_namespace_hierarchy(&hierarchy_one);
        let merged_hierarchy_two =
            MergedNamespaceHierarchy::from_json_namespace_hierarchy(&hierarchy_two);

        merged_hierarchy.merge(merged_hierarchy_two);

        token_stream_matchers::assert_rs_matches!(
            quote! {#merged_hierarchy},
            quote! {
                pub mod top_level_1 {
                    pub use baz::top_level_1::*;
                    pub use xyz::top_level_1::*;
                    pub mod reopened {
                        pub use baz::top_level_1::reopened::*;
                        pub mod baz_specific {
                            pub use baz::top_level_1::reopened::baz_specific::*;
                        }
                    }
                    pub mod xyz_specific {
                        pub use xyz::top_level_1::xyz_specific::*;
                    }
                }
                pub mod top_level_2 {
                    pub use xyz::top_level_2::*;
                }

            }
        );
    }
}
