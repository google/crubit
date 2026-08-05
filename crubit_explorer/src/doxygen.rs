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
use std::str::FromStr;
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
        fs::write(&file_path, bytes)
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
        parse_doxygen_xml(&xml_dir, &mut file_symbols);
    }

    let mut output_files = Vec::new();
    collect_xml_files(temp_dir.path(), &xml_dir, &mut output_files)?;

    Ok(DoxygenResponse {
        xml_output: Some(FileSet { files: output_files }),
        error: None,
        file_symbols: Some(file_symbols),
    })
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

fn parse_doxygen_xml(xml_dir: &Path, file_symbols_map: &mut HashMap<String, SymbolList>) {
    let index_xml_path = xml_dir.join("index.xml");
    let Ok(content) = fs::read_to_string(&index_xml_path) else {
        return;
    };

    let Ok(doc) = roxmltree::Document::parse(&content) else {
        return;
    };

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

        let compound_xml_path = xml_dir.join(format!("{}.xml", refid));
        let compound_content = if compound_xml_path.exists() {
            fs::read_to_string(&compound_xml_path).ok()
        } else {
            None
        };

        let compound_doc =
            compound_content.as_deref().and_then(|c| roxmltree::Document::parse(c).ok());

        let mut compound_file = String::new();
        if let Some(ref c_doc) = compound_doc {
            if let Some(loc) = c_doc
                .descendants()
                .find(|n| n.has_tag_name("compounddef"))
                .and_then(|cdef| cdef.children().find(|n| n.has_tag_name("location")))
            {
                if let Some(f) = loc.attribute("file") {
                    if !f.is_empty() {
                        if let Some(name) = Path::new(f).file_name() {
                            compound_file = name.to_string_lossy().to_string();
                        }
                    }
                }
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
            if let Some(ref c_doc) = compound_doc {
                if let Some(m_def) = c_doc.descendants().find(|n| {
                    n.has_tag_name("memberdef") && n.attribute("id") == Some(&member_refid)
                }) {
                    if let Some(loc) = m_def.children().find(|n| n.has_tag_name("location")) {
                        if let Some(f) = loc.attribute("file") {
                            if !f.is_empty() {
                                if let Some(name) = Path::new(f).file_name() {
                                    member_file = name.to_string_lossy().to_string();
                                }
                            }
                        }
                    }
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
                    description: None,
                });
        }
    }
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
}
