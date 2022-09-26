// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use getopts::{Fail, Options};
use std::path::Path;

#[derive(Debug)]
pub struct Cmdline {
    h_out: String,
    rustc_args: Vec<String>,
}

const H_OUT: &str = "h_out";

impl Cmdline {
    pub fn new(args: &[String]) -> Result<Self, Fail> {
        // Ensure that `@file` expansion also covers *our* args.
        let args = rustc_driver::args::arg_expand_all(args);

        let matches = Options::new()
            .reqopt("", H_OUT, "output path for C++ header file with bindings", "FILE")
            .parse(&args)?;
        let h_out =
            matches.opt_str(H_OUT).expect("getopts should enforce presence of --h_out `reqopt`");

        Ok(Self { h_out, rustc_args: matches.free })
    }

    pub fn h_out(&self) -> &Path {
        Path::new(&self.h_out)
    }

    pub fn rustc_args(&self) -> &[String] {
        &self.rustc_args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::Itertools;
    use tempfile::tempdir;

    fn new_cmdline(args: &[&str]) -> Result<Cmdline, Fail> {
        let args = args.iter().map(|s| s.to_string()).collect_vec();
        Cmdline::new(&args)
    }

    #[test]
    fn test_h_out_happy_path() -> Result<(), Fail> {
        let cmdline = new_cmdline(&["--h_out=foo.h"])?;
        assert_eq!(Path::new("foo.h"), cmdline.h_out());
        Ok(())
    }

    #[test]
    fn test_h_out_missing() {
        match new_cmdline(&[]) {
            Err(Fail::OptionMissing(arg)) if arg == H_OUT => (),
            other => panic!("Unexpected success or unrecognized error: {:?}", other),
        }
    }

    #[test]
    fn test_h_out_without_arg() {
        match new_cmdline(&["--h_out"]) {
            Err(Fail::ArgumentMissing(arg)) if arg == H_OUT => (),
            other => panic!("Unexpected success or unrecognized error: {:?}", other),
        }
    }

    #[test]
    fn test_h_out_duplicated() {
        match new_cmdline(&["--h_out=foo.h", "--h_out=bar.h"]) {
            Err(Fail::OptionDuplicated(arg)) if arg == H_OUT => (),
            other => panic!("Unexpected success or unrecognized error: {:?}", other),
        }
    }

    #[test]
    fn test_rustc_args_happy_path() -> Result<(), Fail> {
        // Note that this test would fail without the `--` separator.
        let cmdline = new_cmdline(&["--h_out=foo.h", "--", "test.rs", "--crate-type=lib"])?;
        let rustc_args = cmdline.rustc_args();
        assert!(
            itertools::equal(&["test.rs", "--crate-type=lib"], rustc_args),
            "rustc_args = {:?}",
            rustc_args
        );
        Ok(())
    }

    #[test]
    fn test_here_file() -> anyhow::Result<()> {
        let tmpdir = tempdir()?;
        let tmpfile = tmpdir.path().join("herefile");
        std::fs::write(
            &tmpfile,
            &["--h_out=foo.h", "--", "test.rs", "--crate-type=lib"].join("\n"),
        )?;

        let cmdline = Cmdline::new(&[format!("@{}", tmpfile.display())])?;
        assert_eq!(Path::new("foo.h"), cmdline.h_out());
        let rustc_args = cmdline.rustc_args();
        assert!(
            itertools::equal(&["test.rs", "--crate-type=lib"], rustc_args),
            "rustc_args = {:?}",
            rustc_args
        );
        Ok(())
    }
}
