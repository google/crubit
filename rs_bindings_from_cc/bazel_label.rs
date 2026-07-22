// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// A Bazel label, e.g. `//foo:bar`.
#[derive(Debug, Eq, Clone)]
pub struct BazelLabel(pub Rc<str>);

impl BazelLabel {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the target name. E.g. `bar` for `//foo:bar`.
    pub fn target_name(&self) -> &str {
        if let Some((_package, target_name)) = self.0.split_once(':') {
            return target_name;
        }
        if let Some((_, last_package_component)) = self.0.rsplit_once('/') {
            return last_package_component;
        }
        &self.0
    }

    pub fn package_name(&self) -> &str {
        self.0.rsplit_once(':').unwrap_or((&self.0, "")).0
    }

    fn last_package_component(&self) -> &str {
        self.package_name().rsplit_once('/').unwrap_or(("", "")).1
    }

    // TODO(b/216587072): Remove this hacky escaping and use the import! macro once
    // available.
    // For now, use the simple escaping scheme of mapping all invalid characters
    // to underscore, instead of the one similar to `convert_to_cc_identifier`, so
    // that the escaped target name doesn't become longer (rustc currently produces
    // .o artifacts that repeat the target name twice, which can easily cause
    // the path length of artifacts to exceed the limit of the file system.)
    pub fn target_name_escaped(&self) -> String {
        let mut target_name = self.target_name().to_owned();
        if target_name == "core" {
            target_name = "core_".to_owned() + self.last_package_component();
        } else if target_name.starts_with(char::is_numeric) {
            target_name.insert(0, 'n');
        }
        target_name.replace(|c: char| !c.is_ascii_alphanumeric(), "_")
    }

    // Returns the bazel label as a valid C++ identifier, with a leading underscore.
    // Non-alphanumeric characters are escaped as `_xx`, where `xx` is the the byte
    // as hexadecimal.
    //
    // For instance, `//foo` becomes `__2f_2ffoo`.
    pub fn convert_to_cc_identifier(&self) -> String {
        use std::fmt::Write;
        let mut result = "_".to_string();
        result.reserve_exact(self.0.len().checked_mul(2).unwrap_or(self.0.len()));

        // This is yet another escaping scheme... :-/  Compare this with
        // https://github.com/bazelbuild/rules_rust/blob/1f2e6231de29d8fad8d21486f0d16403632700bf/rust/private/utils.bzl#L459-L586
        for b in self.0.bytes() {
            if (b as char).is_ascii_alphanumeric() {
                result.push(b as char);
            } else {
                write!(result, "_{b:02x}").unwrap();
            }
        }
        result.shrink_to_fit();

        #[cfg(debug_assertions)]
        for c in result.chars() {
            debug_assert!(
                c.is_ascii_alphanumeric() || c == '_',
                "invalid result identifier: {result:?}"
            );
        }

        result
    }

    fn components(&self) -> (&str, &str) {
        (self.target_name(), self.package_name())
    }
}

impl PartialEq for BazelLabel {
    fn eq(&self, other: &Self) -> bool {
        self.components() == other.components()
    }
}

impl PartialOrd for BazelLabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BazelLabel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.components().cmp(&other.components())
    }
}

impl Hash for BazelLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.components().hash(state);
    }
}

impl<T: Into<String>> From<T> for BazelLabel {
    fn from(label: T) -> Self {
        Self(label.into().into())
    }
}

impl Display for BazelLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If this isn't actually a known bazel target, stringify for humans as the filename.
        if let Some(s) = self.0.strip_prefix("//_unknown_target:") {
            write!(f, "{}", s)
        } else {
            write!(f, "{}", &*self.0)
        }
    }
}
