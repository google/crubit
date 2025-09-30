// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Dynamically parsed format strings, similar to `absl::Format<'s', ...>`, but using
//! Rust's format-string syntax.

use anyhow::{bail, ensure, Result};
// Note: needs to be Send to be used with clap.
use std::sync::Arc;

/// A (dynamically parsed) format string with NUM_VARIABLES substitutions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Format<const NUM_VARIABLES: usize> {
    fragments: Arc<[Fragment]>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Fragment {
    /// A string fragment.
    String(String),
    /// A variable 0 <= n < NUM_VARIABLES
    Variable(usize),
}

impl<const N: usize> Format<N> {
    /// Parse a format string, using textual names for each variable.
    ///
    /// The order of the variable names in `variable_names` corresponds with the order of
    /// substitutions in the `format()` call later.
    pub fn parse_with_metavars(
        format_string: &str,
        variable_names: &[&'static str; N],
    ) -> Result<Self> {
        let mut fragments = Vec::new();

        let mut processed_string = format_string;

        fn push_str(fragments: &mut Vec<Fragment>, s: &str) {
            if let Some(Fragment::String(last_string)) = fragments.last_mut() {
                last_string.push_str(s)
            } else {
                fragments.push(Fragment::String(s.to_string()));
            }
        }

        while let Some(i) = processed_string.find(&['{', '}']) {
            if processed_string[i..].starts_with("{{") || processed_string[i..].starts_with("}}") {
                push_str(&mut fragments, &processed_string[..=i]);
                processed_string = &processed_string[(i + 2)..];
                continue;
            }
            ensure!(
                !processed_string[i..].starts_with('}'),
                "invalid format string: unmatched `}}`: {format_string:?}"
            );
            push_str(&mut fragments, &processed_string[..i]);
            processed_string = &processed_string[(i + 1)..];
            let Some(end) = processed_string.find('}') else {
                bail!("invalid format string: unmatched `{{`: {format_string:?}");
            };
            let variable_name = processed_string[..end].trim();
            processed_string = &processed_string[(end + 1)..];

            let Some(variable_index) = variable_names.iter().position(|x| *x == variable_name)
            else {
                bail!(
                    "invalid format string: unknown variable `{variable_name}`: {format_string:?}"
                );
            };
            fragments.push(Fragment::Variable(variable_index));
        }
        if !processed_string.is_empty() {
            push_str(&mut fragments, processed_string);
        }
        Ok(Self { fragments: fragments.into() })
    }

    /// Formats a string using the provided substitutions.
    ///
    /// The args correspond to the variable names passed to `parse_with_metavars()`, in the order
    /// in which they were provided.
    ///
    /// For example:
    ///
    /// ```
    /// let format_string = Format::parse_with_metavars(
    ///     "/{arg2}/{arg1}", &["arg1", "arg2"]).unwrap();
    /// assert_eq!(format_string.format(["a", "b"]).as_str(), "b/a");
    /// ```
    pub fn format(&self, args: &[&str; N]) -> String {
        let it = self.fragments.iter().map(|f| match f {
            Fragment::String(s) => s.as_str(),
            Fragment::Variable(i) => args[*i],
        });
        let length = it.clone().map(|s| s.len()).sum();
        let mut result = String::with_capacity(length);
        for fragment in it {
            result.push_str(fragment);
        }
        result
    }
}
