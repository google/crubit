// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::path::PathBuf;

// Opaque wrapper around the compressed archive bytes.
// Crubit handles this safely without requiring C++ Vec bindings.
#[derive(Clone, Default)]
pub struct Archive {
    data: Vec<u8>,
}

// Simple error struct for clean Crubit Result interop
#[derive(Clone, Default)]
pub struct Error {
    pub message: String,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error { message: err.to_string().into() }
    }
}

pub struct DirIterator {
    stack: Box<dyn Iterator<Item = Result<std::fs::DirEntry, std::io::Error>>>,
    cwd: PathBuf,
    tar_builder: tar::Builder<DeflateEncoder<Vec<u8>>>,
}

impl DirIterator {
    pub fn new(dir_path: &str) -> Result<Self, Error> {
        let path_buf = PathBuf::from(dir_path);
        let current_dir = std::env::current_dir()?;
        let read_dir = std::fs::read_dir(&current_dir.join(path_buf))?;
        let buffer = Vec::new();
        let enc = DeflateEncoder::new(buffer, Compression::default());
        Ok(DirIterator {
            stack: Box::new(read_dir),
            cwd: current_dir,
            tar_builder: tar::Builder::new(enc),
        })
    }

    pub fn take_archive(&mut self) -> Result<Archive, Error> {
        let buffer = Vec::new();
        let enc = DeflateEncoder::new(buffer, Compression::default());
        let builder = tar::Builder::new(enc);
        let builder = std::mem::replace(&mut self.tar_builder, builder);
        let enc = builder.into_inner()?;
        let data = enc.finish()?;
        Ok(Archive { data })
    }
}

impl Iterator for DirIterator {
    type Item = Result<DirEntry, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = loop {
            let entry = self.stack.next()?;
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => break Err(err.into()),
            };
            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(err) => break Err(err.into()),
            };
            if file_type.is_dir() {
                match std::fs::read_dir(entry.path()) {
                    Ok(entries) => {
                        let iter = std::mem::replace(&mut self.stack, Box::new(std::iter::empty()));
                        self.stack = Box::new(iter.chain(entries))
                    }
                    Err(e) => break Err(e.into()),
                }
                continue;
            }
            break std::fs::read_to_string(entry.path()).map_err(|err| err.into()).and_then(
                |contents| {
                    let mut header = tar::Header::new_gnu();
                    header.set_size(contents.len().try_into().unwrap());
                    header.set_cksum();
                    let mut bytes = contents.as_bytes();
                    self.tar_builder.append_data(
                        &mut header,
                        &entry.path().strip_prefix(&self.cwd).map_err(|_| Error {
                            message: format!(
                                "Expected path to be relative to current directory, but it was not"
                            ),
                        })?,
                        &mut bytes,
                    )?;
                    Ok(DirEntry {
                        path: entry.path().display().to_string(),
                        contents: Archive { data: contents.into_bytes() },
                    })
                },
            );
        };
        Some(next)
    }
}

#[derive(Clone, Default)]
pub struct DirEntry {
    pub path: String,
    pub contents: Archive,
}

// 2. Accepts the opaque Archive from C++ and computes the blake3 hash
pub fn hash_archive(archive: &Archive) -> blake3::Hash {
    blake3::hash(&archive.data)
}
