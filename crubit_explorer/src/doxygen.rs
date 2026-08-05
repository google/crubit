// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use axum::extract::rejection::JsonRejection;
use axum::extract::Json;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::{self, FromStr};
use tempfile::Builder;

use crate::api::{ErrorDetails, File, FileSet};
use crate::resource_locator::{get_doxyfile_path, get_doxygen_path};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoxygenRequest {
    pub input: FileSet,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolKind {
    Unspecified,
    Class,
    Struct,
    Union,
    Interface,
    Namespace,
    File,
    Group,
    Page,
    Dir,
    Category,
    Exception,
    Protocol,
    Concept,
    Define,
    Property,
    Event,
    Variable,
    Typedef,
    Enum,
    Enumvalue,
    Function,
    Signal,
    Slot,
    Friend,
    Dcl,
    Prototype,
}

impl FromStr for SymbolKind {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "class" => SymbolKind::Class,
            "struct" => SymbolKind::Struct,
            "union" => SymbolKind::Union,
            "interface" => SymbolKind::Interface,
            "namespace" => SymbolKind::Namespace,
            "file" => SymbolKind::File,
            "group" => SymbolKind::Group,
            "page" => SymbolKind::Page,
            "dir" => SymbolKind::Dir,
            "category" => SymbolKind::Category,
            "exception" => SymbolKind::Exception,
            "protocol" => SymbolKind::Protocol,
            "concept" => SymbolKind::Concept,
            "define" => SymbolKind::Define,
            "property" => SymbolKind::Property,
            "event" => SymbolKind::Event,
            "variable" => SymbolKind::Variable,
            "typedef" => SymbolKind::Typedef,
            "enum" => SymbolKind::Enum,
            "enumvalue" => SymbolKind::Enumvalue,
            "function" => SymbolKind::Function,
            "signal" => SymbolKind::Signal,
            "slot" => SymbolKind::Slot,
            "friend" => SymbolKind::Friend,
            "dcl" => SymbolKind::Dcl,
            "prototype" => SymbolKind::Prototype,
            _ => SymbolKind::Unspecified,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub refid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SymbolList {
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoxygenResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_output: Option<FileSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_symbols: Option<HashMap<String, SymbolList>>,
}

pub async fn doxygen_handler(
    payload: Result<Json<DoxygenRequest>, JsonRejection>,
) -> Json<DoxygenResponse> {
    let payload = match payload {
        Ok(Json(p)) => p,
        Err(e) => {
            return Json(DoxygenResponse {
                xml_output: None,
                error: Some(ErrorDetails::new("Failed to parse request", e.to_string())),
                file_symbols: None,
            });
        }
    };

    let result = tokio::task::spawn_blocking(move || execute_doxygen(payload)).await;

    match result {
        Ok(Ok(response)) => Json(response),
        Ok(Err(error)) => {
            Json(DoxygenResponse { xml_output: None, error: Some(error), file_symbols: None })
        }
        Err(e) => Json(DoxygenResponse {
            xml_output: None,
            error: Some(ErrorDetails::new("Internal server error", e.to_string())),
            file_symbols: None,
        }),
    }
}

fn execute_doxygen(payload: DoxygenRequest) -> Result<DoxygenResponse, ErrorDetails> {
    let doxygen_bin = get_doxygen_path()
        .map_err(|e| ErrorDetails::new("Failed to locate doxygen binary", e.to_string()))?;

    let temp_dir = Builder::new()
        .prefix("doxygen_explorer_")
        .tempdir()
        .map_err(|e| ErrorDetails::new("Failed to create temporary directory", e.to_string()))?;

    // Write input files to temp directory
    for file in payload.input.files {
        let file_path = temp_dir.path().join(&file.name);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ErrorDetails::new("Failed to create directory for file", e.to_string())
            })?;
        }
        let bytes = BASE64_STANDARD
            .decode(&file.contents_b64)
            .map_err(|e| ErrorDetails::new("Base64 decode failed", e.to_string()))?;
        let processed_bytes =
            if [".h", ".hpp", ".cc", ".cpp"].iter().any(|ext| file.name.ends_with(ext))
                && let Ok(text) = str::from_utf8(&bytes)
            {
                preprocess_code_for_doxygen(text).into_bytes()
            } else {
                bytes
            };
        fs::write(&file_path, processed_bytes)
            .map_err(|e| ErrorDetails::new("Failed to write temporary file", e.to_string()))?;
    }

    let doxyfile_path = get_doxyfile_path()
        .map_err(|e| ErrorDetails::new("Failed to locate Doxyfile", e.to_string()))?;
    let doxyfile_content = fs::read_to_string(&doxyfile_path)
        .map_err(|e| ErrorDetails::new("Failed to read Doxyfile", e.to_string()))?;

    let mut child = Command::new(&doxygen_bin)
        .arg("-")
        .current_dir(temp_dir.path())
        .env("LC_ALL", "C")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ErrorDetails::new("Failed to start Doxygen process", e.to_string()))?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(doxyfile_content.as_bytes());
    }

    let output = child
        .wait_with_output()
        .map_err(|e| ErrorDetails::new("Doxygen process failed", e.to_string()))?;

    if !output.status.success() {
        return Ok(DoxygenResponse {
            xml_output: None,
            error: Some(ErrorDetails::new(
                "Doxygen failed",
                String::from_utf8_lossy(&output.stderr).to_string(),
            )),
            file_symbols: None,
        });
    }

    let xml_dir = temp_dir.path().join("xml");
    if !xml_dir.exists() {
        return Ok(DoxygenResponse {
            xml_output: None,
            error: Some(ErrorDetails::new(
                "Doxygen failed to generate XML",
                "xml directory not found".to_string(),
            )),
            file_symbols: None,
        });
    }

    let mut file_symbols = HashMap::new();
    let index_xml_path = xml_dir.join("index.xml");
    if index_xml_path.exists() {
        parse_doxygen_xml(&xml_dir, &mut file_symbols)?;
    }

    let mut output_files = Vec::new();
    collect_xml_files(temp_dir.path(), &xml_dir, &mut output_files)?;

    Ok(DoxygenResponse {
        xml_output: Some(FileSet { files: output_files }),
        error: None,
        file_symbols: Some(file_symbols),
    })
}

/// Preprocesses C++ source code before feeding it to Doxygen.
///
/// Replaces macros, attributes, and specifiers (such as `CRUBIT_INTERNAL_RUST_TYPE`,
/// `alignas(...)`, `[[...]]`, and `final`) with equal-length whitespace. This prevents Doxygen
/// syntax errors and unexpected symbol output while preserving line numbers and column offsets
/// for source location mapping.
/// TODO: b/540507040 support newer doxygen version so this is no longer needed.
fn preprocess_code_for_doxygen(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    for line in content.lines() {
        let mut cleaned = line.to_string();
        while let Some(start) = cleaned.find("CRUBIT_INTERNAL_RUST_TYPE(") {
            if let Some(end) = cleaned[start..].find(')') {
                let match_len = end + 1;
                cleaned.replace_range(start..start + match_len, &" ".repeat(match_len));
            } else {
                break;
            }
        }
        while let Some(start) = cleaned.find("alignas(") {
            if let Some(end) = cleaned[start..].find(')') {
                let match_len = end + 1;
                cleaned.replace_range(start..start + match_len, &" ".repeat(match_len));
            } else {
                break;
            }
        }
        while let Some(start) = cleaned.find("[[") {
            if let Some(end) = cleaned[start..].find("]]") {
                let match_len = end + 2;
                cleaned.replace_range(start..start + match_len, &" ".repeat(match_len));
            } else {
                break;
            }
        }
        if cleaned.contains("struct ") || cleaned.contains("class ") {
            if let Some(pos) = cleaned.find(" final") {
                cleaned.replace_range(pos..pos + 6, "      ");
            }
        }
        result.push_str(&cleaned);
        result.push('\n');
    }
    result
}

fn collect_xml_files(
    base_dir: &Path,
    current_dir: &Path,
    output_files: &mut Vec<File>,
) -> Result<(), ErrorDetails> {
    let entries = fs::read_dir(current_dir)
        .map_err(|e| ErrorDetails::new("Failed to read XML directory", e.to_string()))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_xml_files(base_dir, &path, output_files)?;
        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("xml") {
            if let Ok(bytes) = fs::read(&path) {
                if let Ok(rel_path) = path.strip_prefix(base_dir) {
                    output_files.push(File {
                        name: rel_path.to_string_lossy().to_string(),
                        contents_b64: BASE64_STANDARD.encode(&bytes),
                    });
                }
            }
        }
    }
    Ok(())
}

fn parse_doxygen_xml(
    xml_dir: &Path,
    file_symbols_map: &mut HashMap<String, SymbolList>,
) -> Result<(), ErrorDetails> {
    let index_xml_path = xml_dir.join("index.xml");
    let content = fs::read_to_string(&index_xml_path)
        .map_err(|e| ErrorDetails::new("Failed to read index.xml", e.to_string()))?;

    let doc = roxmltree::Document::parse(&content)
        .map_err(|e| ErrorDetails::new("Failed to parse index.xml", e.to_string()))?;

    for compound in doc.descendants().filter(|n| n.has_tag_name("compound")) {
        let refid = compound.attribute("refid").unwrap_or("").to_string();
        let kind = compound.attribute("kind").unwrap_or("").to_string();
        let compound_name = compound
            .children()
            .find(|n| n.has_tag_name("name"))
            .and_then(|n| n.text())
            .unwrap_or("")
            .trim()
            .to_string();

        let compound_xml_path = xml_dir.join(format!("{refid}.xml"));
        let compound_content = fs::read_to_string(&compound_xml_path).map_err(|e| {
            ErrorDetails::new(format!("Failed to read compound XML: {refid}.xml"), e.to_string())
        })?;
        let compound_doc = roxmltree::Document::parse(&compound_content).map_err(|e| {
            ErrorDetails::new(format!("Failed to parse compound XML: {refid}.xml"), e.to_string())
        })?;

        let mut compound_file = String::new();
        let mut compound_line = None;
        if let Some(loc) = compound_doc
            .descendants()
            .find(|n| n.has_tag_name("compounddef"))
            .and_then(|cdef| cdef.children().find(|n| n.has_tag_name("location")))
        {
            if let Some(f) = loc.attribute("file")
                && !f.is_empty()
                && let Some(name) = Path::new(f).file_name()
            {
                compound_file = name.to_string_lossy().to_string();
            }
            if let Some(l) = loc.attribute("line") {
                compound_line = Some(l.parse::<u32>().map_err(|e| {
                    ErrorDetails::new(
                        "Failed to parse line number",
                        format!("Invalid line number '{l}' for compound '{refid}': {e}"),
                    )
                })?);
            }
        }

        file_symbols_map
            .entry(compound_file.clone())
            .or_insert_with(|| SymbolList { symbols: Vec::new() })
            .symbols
            .push(Symbol {
                name: compound_name.clone(),
                kind: kind.parse().unwrap_or(SymbolKind::Unspecified),
                refid: refid.clone(),
                line: compound_line,
                description: None,
            });

        for member in compound.children().filter(|n| n.has_tag_name("member")) {
            let member_refid = member.attribute("refid").unwrap_or("").to_string();
            let member_kind = member.attribute("kind").unwrap_or("").to_string();
            let member_name = member
                .children()
                .find(|n| n.has_tag_name("name"))
                .and_then(|n| n.text())
                .unwrap_or("")
                .trim()
                .to_string();

            let mut member_file = compound_file.clone();
            let mut member_line = None;
            if let Some(m_def) = compound_doc
                .descendants()
                .find(|n| n.has_tag_name("memberdef") && n.attribute("id") == Some(&member_refid))
                && let Some(loc) = m_def.children().find(|n| n.has_tag_name("location"))
            {
                if let Some(f) = loc.attribute("file")
                    && !f.is_empty()
                    && let Some(name) = Path::new(f).file_name()
                {
                    member_file = name.to_string_lossy().to_string();
                }
                if let Some(l) = loc.attribute("line") {
                    member_line = Some(l.parse::<u32>().map_err(|e| {
                        ErrorDetails::new(
                            "Failed to parse line number",
                            format!("Invalid line number '{l}' for member '{member_refid}': {e}"),
                        )
                    })?);
                }
            }

            let full_name = if compound_name.is_empty() {
                member_name
            } else {
                format!("{}::{}", compound_name, member_name)
            };

            file_symbols_map
                .entry(member_file)
                .or_insert_with(|| SymbolList { symbols: Vec::new() })
                .symbols
                .push(Symbol {
                    name: full_name,
                    kind: member_kind.parse().unwrap_or(SymbolKind::Unspecified),
                    refid: member_refid,
                    line: member_line,
                    description: None,
                });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_symbol_kind_from_str() {
        expect_eq!("class".parse::<SymbolKind>(), Ok(SymbolKind::Class));
        expect_eq!("FUNCTION".parse::<SymbolKind>(), Ok(SymbolKind::Function));
        expect_eq!("unknown".parse::<SymbolKind>(), Ok(SymbolKind::Unspecified));
    }

    #[gtest]
    fn test_parse_roxmltree_simple() {
        let xml = r#"<doxygenindex><compound refid="class_foo" kind="class"><name>Foo</name></compound></doxygenindex>"#;
        let doc = roxmltree::Document::parse(xml).expect("XML parsing failed");
        let compound = doc.descendants().find(|n| n.has_tag_name("compound")).unwrap();
        expect_eq!(compound.attribute("refid"), Some("class_foo"));
        expect_eq!(compound.attribute("kind"), Some("class"));
        let name_node = compound.children().find(|n| n.has_tag_name("name")).unwrap();
        expect_eq!(name_node.text().unwrap().trim(), "Foo");
    }

    #[gtest]
    fn test_get_doxygen_path() {
        let p = get_doxygen_path().expect("doxygen path should be resolved");
        expect_true!(p.exists(), "doxygen binary path {:?} must exist", p);
    }

    #[gtest]
    fn test_get_doxyfile_path() {
        let p = get_doxyfile_path().expect("Doxyfile path should be resolved");
        expect_true!(p.exists(), "Doxyfile path {:?} must exist", p);
    }

    #[gtest]
    fn test_parse_doxygen_xml_location() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_xml = r#"<doxygenindex>
            <compound refid="class_foo" kind="class">
                <name>Foo</name>
                <member refid="class_foo_1a1" kind="function">
                    <name>bar</name>
                </member>
            </compound>
        </doxygenindex>"#;
        fs::write(temp_dir.path().join("index.xml"), index_xml).unwrap();

        let class_xml = r#"<doxygen>
            <compounddef id="class_foo" kind="class">
                <compoundname>Foo</compoundname>
                <location file="foo.h" line="10" column="1"/>
                <sectiondef kind="public-func">
                    <memberdef id="class_foo_1a1" kind="function">
                        <name>bar</name>
                        <location file="foo.h" line="15" column="5"/>
                    </memberdef>
                </sectiondef>
            </compounddef>
        </doxygen>"#;
        fs::write(temp_dir.path().join("class_foo.xml"), class_xml).unwrap();

        let mut file_symbols = HashMap::new();
        parse_doxygen_xml(temp_dir.path(), &mut file_symbols).unwrap();

        expect_true!(file_symbols.contains_key("foo.h"));
        let symbols = &file_symbols["foo.h"].symbols;
        let foo_sym = symbols.iter().find(|s| s.name == "Foo").unwrap();
        expect_eq!(foo_sym.line, Some(10));
        let bar_sym = symbols.iter().find(|s| s.name == "Foo::bar").unwrap();
        expect_eq!(bar_sym.line, Some(15));
    }

    #[gtest]
    fn test_parse_doxygen_xml_invalid_line() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_xml = r#"<doxygenindex>
            <compound refid="class_foo" kind="class">
                <name>Foo</name>
            </compound>
        </doxygenindex>"#;
        fs::write(temp_dir.path().join("index.xml"), index_xml).unwrap();

        let class_xml = r#"<doxygen>
            <compounddef id="class_foo" kind="class">
                <compoundname>Foo</compoundname>
                <location file="foo.h" line="invalid_line_number"/>
            </compounddef>
        </doxygen>"#;
        fs::write(temp_dir.path().join("class_foo.xml"), class_xml).unwrap();

        let mut file_symbols = HashMap::new();
        let result = parse_doxygen_xml(temp_dir.path(), &mut file_symbols);
        expect_true!(result.is_err());
    }

    #[gtest]
    fn test_parse_doxygen_xml_missing_index_xml() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut file_symbols = HashMap::new();
        let result = parse_doxygen_xml(temp_dir.path(), &mut file_symbols);
        expect_true!(result.is_err());
        let err = result.unwrap_err();
        expect_eq!(err.text, "Failed to read index.xml");
    }

    #[gtest]
    fn test_parse_doxygen_xml_malformed_index_xml() {
        let temp_dir = tempfile::tempdir().unwrap();
        fs::write(temp_dir.path().join("index.xml"), "<doxygenindex><unclosed_tag>").unwrap();

        let mut file_symbols = HashMap::new();
        let result = parse_doxygen_xml(temp_dir.path(), &mut file_symbols);
        expect_true!(result.is_err());
        let err = result.unwrap_err();
        expect_eq!(err.text, "Failed to parse index.xml");
    }

    #[gtest]
    fn test_parse_doxygen_xml_missing_compound_xml() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_xml = r#"<doxygenindex>
            <compound refid="class_foo" kind="class">
                <name>Foo</name>
            </compound>
        </doxygenindex>"#;
        fs::write(temp_dir.path().join("index.xml"), index_xml).unwrap();

        let mut file_symbols = HashMap::new();
        let result = parse_doxygen_xml(temp_dir.path(), &mut file_symbols);
        expect_true!(result.is_err());
        let err = result.unwrap_err();
        expect_eq!(err.text, "Failed to read compound XML: class_foo.xml");
    }

    #[gtest]
    fn test_parse_doxygen_xml_malformed_compound_xml() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_xml = r#"<doxygenindex>
            <compound refid="class_foo" kind="class">
                <name>Foo</name>
            </compound>
        </doxygenindex>"#;
        fs::write(temp_dir.path().join("index.xml"), index_xml).unwrap();
        fs::write(temp_dir.path().join("class_foo.xml"), "<doxygen><invalid>").unwrap();

        let mut file_symbols = HashMap::new();
        let result = parse_doxygen_xml(temp_dir.path(), &mut file_symbols);
        expect_true!(result.is_err());
        let err = result.unwrap_err();
        expect_eq!(err.text, "Failed to parse compound XML: class_foo.xml");
    }

    #[gtest]
    fn test_preprocess_code_for_doxygen() {
        let input = "CRUBIT_INTERNAL_RUST_TYPE(i32) alignas(4) [[nodiscard]] class Foo final {};";
        let processed = preprocess_code_for_doxygen(input);
        expect_eq!(processed.len(), input.len() + 1);
        expect_false!(processed.contains("CRUBIT_INTERNAL_RUST_TYPE"));
        expect_false!(processed.contains("alignas"));
        expect_false!(processed.contains("[[nodiscard]]"));
        expect_false!(processed.contains("final"));
        expect_true!(processed.contains("class Foo"));
    }
}
