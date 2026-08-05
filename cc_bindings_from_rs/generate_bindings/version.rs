// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub(crate) struct ParsedVersion {
    pub(crate) sanitized_full: String,
    pub(crate) aliases: Vec<String>,
}

struct ManualSemver {
    major: u64,
    minor: u64,
    patch: u64,
    pre: Option<String>,
}

fn parse_manual_semver(version: &str) -> Option<ManualSemver> {
    let mut parts = version.splitn(2, '-');
    let release_part = parts.next()?;
    let pre_part = parts.next();

    let mut release_subparts = release_part.split('.');
    let major_str = release_subparts.next()?;
    let minor_str = release_subparts.next()?;
    let patch_str = release_subparts.next()?;
    if release_subparts.next().is_some() {
        return None;
    }

    let major = major_str.parse::<u64>().ok()?;
    let minor = minor_str.parse::<u64>().ok()?;
    let patch = patch_str.parse::<u64>().ok()?;

    Some(ManualSemver { major, minor, patch, pre: pre_part.map(|s| s.to_string()) })
}

/// Parses a version string into a `ParsedVersion` struct, which contains
/// the sanitized version components and aliases.
///
/// This function attempts to parse the version as a semver string (using
/// `parse_manual_semver`). If successful, it sanitizes the components
/// (collapsing prerelease identifiers to alphanumeric only) and generates
/// version aliases. If parsing fails, it falls back to treating the whole
/// string as a generic version identifier, sanitizing it by replacing
/// non-alphanumeric characters with underscores.
pub(crate) fn parse_and_sanitize_version(version_str: &str) -> ParsedVersion {
    if let Some(v) = parse_manual_semver(version_str) {
        let major = v.major;
        let minor = v.minor;
        let patch = v.patch;

        let pre_str = if let Some(pre) = &v.pre {
            let mut s = String::new();
            for c in pre.chars() {
                if c.is_ascii_alphanumeric() {
                    s.push(c);
                } else if c == '.' {
                    // collapse rc.1 to rc1
                } else {
                    s.push('_');
                }
            }
            format!("_{}", s)
        } else {
            "".to_string()
        };

        let sanitized_full = format!("v{}_{}_{}{}", major, minor, patch, pre_str);

        let mut aliases = vec![];
        if major != 0 {
            aliases.push(format!("v{}", major));
            aliases.push(format!("v{}_{}", major, minor));
        } else if minor != 0 {
            aliases.push(format!("v0_{}", minor));
        }

        ParsedVersion { sanitized_full, aliases }
    } else {
        // Fallback for non-semver versions
        let mut sanitized = String::new();
        if !version_str.starts_with('v')
            || !version_str.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
        {
            sanitized.push('v');
        }
        for c in version_str.chars() {
            if c.is_ascii_alphanumeric() {
                sanitized.push(c);
            } else {
                sanitized.push('_');
            }
        }
        ParsedVersion { sanitized_full: sanitized, aliases: vec![] }
    }
}

/// Sanitizes a crate version string into a valid C++ identifier.
///
/// If the version is semver-compliant, it formats it as `v<major>_<minor>_<patch>[_<prerelease>]`.
/// Otherwise, it replaces non-alphanumeric characters with underscores.
pub(crate) fn sanitize_version(version: &str) -> String {
    parse_and_sanitize_version(version).sanitized_full
}
