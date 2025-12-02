// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use base64::{prelude::BASE64_STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use token_stream_printer::{fix_provenance_map_postformatting, SubstringProvenanceMap};

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
struct KytheMetadata {
    #[serde(rename = "type")]
    typ: String,
    meta: Vec<KytheMetadataRule>,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
struct KytheMetadataVName {
    corpus: String,
    path: String,
    language: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
struct KytheMetadataRule {
    #[serde(rename = "type")]
    typ: String,
    source_begin: usize,
    source_end: usize,
    target_begin: usize,
    target_end: usize,
    edge: String,
    source_vname: KytheMetadataVName,
}

/// Given a `provenance_map` and a C++ `header` that has possibly been formatted, embed the
/// provenance map in the header with adjusted offsets. Use `default_corpus` if no other corpus
/// applies.
pub fn cc_embed_provenance_map(
    provenance_map: &SubstringProvenanceMap,
    default_corpus: &str,
    mut header: String,
) -> String {
    // NB: the formatter might break the line after the colon.
    let fixed_map =
        fix_provenance_map_postformatting(&header, "// Generated from:", provenance_map);
    let mut sorted_map: Vec<_> = fixed_map.values().collect();
    sorted_map.sort_unstable();
    header.push_str("\n// This file contains Kythe metadata. ");
    let mut metas = Vec::new();
    for entry in sorted_map {
        // TODO: b/460420108, check for a valid path here (and possibly apply vname remapping
        // rules, if it turns out that we want to do that in Crubit and not elsewhere in the
        // pipeline).
        if let (Ok(source_begin), Ok(source_end)) =
            (entry.original_start.parse::<usize>(), entry.original_end.parse::<usize>())
        {
            metas.push(KytheMetadataRule {
                typ: "anchor_anchor".to_owned(),
                source_begin,
                source_end,
                target_begin: entry.formatted_start,
                target_end: entry.formatted_end,
                edge: "/kythe/edge/imputes".to_owned(),
                source_vname: KytheMetadataVName {
                    corpus: default_corpus.to_owned(),
                    path: entry.original_path.clone(),
                    language: "rust".to_owned(),
                },
            });
        }
    }
    let meta = KytheMetadata { typ: "kythe0".to_owned(), meta: metas };
    BASE64_STANDARD.encode_string(serde_json::to_vec(&meta).unwrap(), &mut header);
    header.push('\n');
    header
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    use anyhow::Result;
    use token_stream_printer::SubstringWithProvenance;

    #[gtest]
    fn test_cc_appends_correct_empty_provenance_map() -> Result<()> {
        let header = String::from(
            r#"#ifndef GUARD
#endif"#,
        );
        let empty_map = SubstringProvenanceMap::new();
        let result = cc_embed_provenance_map(&empty_map, "corpus", header);
        // Check for the empty record: {"type":"kythe0","meta":[]}
        assert_eq!(
            result,
            r#"#ifndef GUARD
#endif
// This file contains Kythe metadata. eyJ0eXBlIjoia3l0aGUwIiwibWV0YSI6W119
"#
        );
        Ok(())
    }

    #[gtest]
    fn test_cc_appends_correct_provenance_map() -> Result<()> {
        let header = String::from(
            r#"#ifndef GUARD
// Generated from: a
foo 
// Generated from: b
bar
#endif"#,
        );
        let mut provenance_map = SubstringProvenanceMap::new();
        provenance_map.insert(
            1,
            SubstringWithProvenance {
                substring: "foo".to_owned(),
                original_path: "foo.rs".to_owned(),
                original_start: "1".to_owned(),
                original_end: "2".to_owned(),
                index: 0,
            },
        );
        provenance_map.insert(
            2,
            SubstringWithProvenance {
                substring: "bar".to_owned(),
                original_path: "bar.rs".to_owned(),
                original_start: "3".to_owned(),
                original_end: "4".to_owned(),
                index: 0,
            },
        );
        let result = cc_embed_provenance_map(&provenance_map, "corpus", header);
        let prefix = r#"#ifndef GUARD
// Generated from: a
foo 
// Generated from: b
bar
#endif
// This file contains Kythe metadata. "#;
        assert!(result.starts_with(prefix));
        let encoding = &result[prefix.len()..result.len() - 1];
        let json_str = BASE64_STANDARD.decode(encoding)?;
        let decoded: KytheMetadata = serde_json::from_slice(&json_str)?;
        assert_eq!(
            decoded,
            KytheMetadata {
                typ: "kythe0".to_owned(),
                meta: vec![
                    KytheMetadataRule {
                        typ: "anchor_anchor".to_owned(),
                        source_begin: 1,
                        source_end: 2,
                        target_begin: 35,
                        target_end: 38,
                        edge: "/kythe/edge/imputes".to_owned(),
                        source_vname: KytheMetadataVName {
                            corpus: "corpus".to_owned(),
                            path: "foo.rs".to_owned(),
                            language: "rust".to_owned(),
                        },
                    },
                    KytheMetadataRule {
                        typ: "anchor_anchor".to_owned(),
                        source_begin: 3,
                        source_end: 4,
                        target_begin: 61,
                        target_end: 64,
                        edge: "/kythe/edge/imputes".to_owned(),
                        source_vname: KytheMetadataVName {
                            corpus: "corpus".to_owned(),
                            path: "bar.rs".to_owned(),
                            language: "rust".to_owned(),
                        },
                    },
                ]
            }
        );
        Ok(())
    }
}
