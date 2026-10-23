// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! An mdBook preprocessor that expands docs-style `live-snippet` blocks.
//!
//! See `lib.rs` for details. mdBook invokes this binary as
//! `mdbook-live-snippet supports <renderer>` to check renderer support, and
//! otherwise passes `[context, book]` as JSON on stdin and expects the
//! processed book as JSON on stdout.

use std::io::{Read, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some("supports") {
        // The output is plain Markdown, so every renderer is supported.
        return ExitCode::SUCCESS;
    }
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut input) {
        eprintln!("mdbook-live-snippet: failed to read stdin: {e}");
        return ExitCode::FAILURE;
    }
    match mdbook_live_snippet::run(&input) {
        Ok(output) => {
            if let Err(e) = std::io::stdout().write_all(output.as_bytes()) {
                eprintln!("mdbook-live-snippet: failed to write stdout: {e}");
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("mdbook-live-snippet: {e:#}");
            ExitCode::FAILURE
        }
    }
}
