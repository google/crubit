// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn print_link_search<T: AsRef<OsStr>>(s: T) -> io::Result<()> {
    print!("cargo::rustc-link-search=native=");
    io::stdout().write_all(s.as_ref().as_encoded_bytes())?;
    println!();
    Ok(())
}

pub fn print_link_searches<T: AsRef<OsStr>>(paths: impl IntoIterator<Item = T>) -> io::Result<()> {
    for path in paths.into_iter() {
        print_link_search(path)?;
    }
    Ok(())
}

pub fn print_link_libs<T: AsRef<OsStr>>(libs: impl IntoIterator<Item = T>) -> io::Result<()> {
    for lib in libs.into_iter() {
        print!("cargo::rustc-link-lib=");
        io::stdout().write_all(lib.as_ref().as_encoded_bytes())?;
        println!();
    }
    Ok(())
}

#[cfg(unix)]
const LIB_EXTENSION: &str = "a";
#[cfg(windows)]
const LIB_EXTENSION: &str = "lib";

pub fn add_include_path<P: AsRef<Path>>(build: &mut cc::Build, path: P, system: bool) {
    let flag = if system && cfg!(unix) { "-isystem" } else { "-I" };
    build.flag(format!("{flag}{}", path.as_ref().display()));
}

/// Reads paths from an environment variable and validates that all specified paths exist on disk.
/// Paths may be separated by platform path separators (`:` on Unix, `;` on Windows) or commas.
///
/// Emits `cargo::rerun-if-env-changed` for `env_var`.
///
/// # Panics
///
/// Panics if `env_var` is not set in the environment, or if any of the specified paths do not
/// exist.
pub fn get_env_paths(env_var: &str) -> Vec<PathBuf> {
    println!("cargo::rerun-if-env-changed={}", env_var);
    let val = std::env::var(env_var).unwrap_or_else(|_| {
        panic!("\n\nERROR: Required environment variable '{}' is not set.\n\n", env_var);
    });

    val.split(',')
        .flat_map(std::env::split_paths)
        .filter(|path| !path.as_os_str().is_empty())
        .inspect(|path| {
            if !path.exists() {
                panic!(
                    "\n\nERROR: Path '{}' specified in '{}' does not exist.\n\n",
                    path.display(),
                    env_var
                );
            }
        })
        .collect()
}

/// Returns whether `path` is a Windows *import* library rather than a static
/// library.
///
/// On Unix the two kinds of library are told apart by their extension: a
/// shared library is `libfoo.so` or `libfoo.dylib`, so the `.a` filter in
/// [`collect_static_libs`] already excludes it.  Windows has no such
/// distinction.  The import library that describes `foo.dll` is an archive
/// named `foo.lib`, exactly like a static library, and a build of LLVM
/// installs both kinds side by side in the same `lib` directory:
/// `LLVM-C.lib`, `LTO.lib`, `Remarks.lib` and `libclang.lib` describe DLLs,
/// while the couple of hundred archives beside them are static.  Linking an
/// import library by mistake gives the resulting executable a load-time
/// dependency on a DLL that is not part of the toolchain, and lets symbols
/// that the DLL happens to re-export (zlib's, in LLVM's case) win over the
/// static library that was meant to provide them.
///
/// The two are therefore told apart by content.  An import library's members
/// include "short import" records, which begin with an `IMPORT_OBJECT_HEADER`
/// whose first two `u16`s are `IMAGE_FILE_MACHINE_UNKNOWN` (`0x0000`) and the
/// sentinel `0xFFFF`.  That pair cannot begin a COFF object file, where the
/// same two fields are the machine type and the number of sections, so the
/// presence of a single such member is conclusive.  Note that an import
/// library also contains a few ordinary COFF members - the import descriptor
/// and null-thunk objects that the linker generates - so it is the *presence*
/// of a short import record that identifies it, not the absence of COFF
/// members.
///
/// Anything that cannot be opened, or that is not an archive this function
/// understands, is reported as not an import library: it is not a build
/// script's job to diagnose a malformed archive, and the linker's error
/// message will be far more useful than one invented here.  Archive member
/// reading and COFF import object parsing are delegated to the `object` crate
/// ([`object::read::archive::ArchiveFile`] and
/// [`object::read::coff::ImportFile`]).
#[cfg(any(windows, test))]
fn is_import_library(path: &Path) -> bool {
    use object::read::archive::ArchiveFile;
    use object::read::coff::ImportFile;

    let Ok(data) = std::fs::read(path) else {
        return false;
    };
    let Ok(archive) = ArchiveFile::parse(&*data) else {
        return false;
    };
    for member in archive.members() {
        let Ok(member) = member else { continue };
        let Ok(member_data) = member.data(&*data) else { continue };
        if ImportFile::parse(member_data).is_ok() {
            return true;
        }
    }
    false
}

/// Discovers and validates static library archives (`.a` / `.lib`) in directories specified by
/// `env_var`.
///
/// Filters libraries using `include_lib_fn` and emits `cargo::rerun-if-changed` for each included
/// archive.
///
/// On Windows, archives that are import libraries rather than static libraries are excluded, even
/// if `include_lib_fn` accepts them; see [`is_import_library`].
///
/// Returns a tuple `(search_directories, library_names)`:
/// - `search_directories`: directories where the libraries were found, suitable for link search
///   paths.
/// - `library_names`: sorted and deduplicated library names with the file extension (`.a` / `.lib`)
///   and Unix `lib` prefix removed, suitable for `cargo::rustc-link-lib`.
///
/// # Panics
///
/// Panics if `env_var` is not set, if any directory does not exist or cannot be read, if any
/// archive has a non-UTF8 filename, or if no matching library archives are found.
pub fn collect_static_libs<F>(env_var: &str, include_lib_fn: F) -> (Vec<PathBuf>, Vec<OsString>)
where
    F: Fn(&str) -> bool,
{
    const { assert!(cfg!(unix) || cfg!(windows)) };

    let lib_dirs = get_env_paths(env_var);
    let mut libs = Vec::new();

    for dir in &lib_dirs {
        let entries = std::fs::read_dir(dir).unwrap_or_else(|e| {
            panic!(
                "\n\nERROR: Unable to read directory '{}' specified in '{}': {}\n\n",
                dir.display(),
                env_var,
                e
            )
        });

        for entry in entries {
            let Ok(entry) = entry else { continue };
            let Ok(meta) = entry.metadata() else { continue };
            if !meta.is_file() {
                continue;
            }
            let path = entry.path();
            if path.extension() != Some(OsStr::new(LIB_EXTENSION)) {
                continue;
            }
            let Some(stem) = path.file_stem() else {
                continue;
            };
            let stem_str = stem.to_str().unwrap_or_else(|| {
                panic!("Non-UTF8 filename in '{}': {:?}", env_var, path.display())
            });
            let libname = if cfg!(windows) {
                // On Windows, the filename without an extension: `name.lib` => `name`.
                stem_str
            } else {
                // On Unix, drop the lib prefix and the extension: `libname.a` => `name`.
                stem_str.strip_prefix("lib").unwrap_or(stem_str)
            };
            if !include_lib_fn(libname) {
                continue;
            }
            // The extension check above excludes shared libraries on Unix, where they are
            // `.so` / `.dylib` rather than `.a`.  On Windows the import library of a DLL is
            // a `.lib` archive just like a static library, so it has to be recognised by its
            // contents instead.  Checked here, after `include_lib_fn`, so that the scan is
            // only paid for archives that would otherwise be linked.
            #[cfg(windows)]
            if is_import_library(&path) {
                continue;
            }
            println!("cargo::rerun-if-changed={}", path.display());
            libs.push(OsString::from(libname));
        }
    }

    if libs.is_empty() {
        panic!(
            "\n\nERROR: No .{} static library files found in directory specified by '{}' \
             ({:?}).\n\n",
            LIB_EXTENSION, env_var, lib_dirs
        );
    }

    libs.sort_unstable();
    libs.dedup();
    (lib_dirs, libs)
}

pub fn add_source_file<P: AsRef<Path>>(build: &mut cc::Build, path: P) -> io::Result<()> {
    print!("cargo::rerun-if-changed=");
    io::stdout().write_all(path.as_ref().as_os_str().as_encoded_bytes())?;
    println!();
    build.file(path.as_ref());
    Ok(())
}

pub fn print_compiler_deps() {
    println!("cargo::rerun-if-env-changed=CC");
    println!("cargo::rerun-if-env-changed=CXX");
    println!("cargo::rerun-if-env-changed=LD");
    println!("cargo::rerun-if-env-changed=CFLAGS");
    println!("cargo::rerun-if-env-changed=CXXFLAGS");
    println!("cargo::rerun-if-env-changed=LDFLAGS");
}

/// Unit tests for import library detection.
///
/// Note: These tests use standard `#[test]` rather than `googletest` because
/// `crubit_build` is a build helper crate built via Cargo.
#[cfg(test)]
mod tests {
    use super::*;

    fn create_archive_file(dir: &Path, filename: &str, members: &[(&[u8], &[u8])]) -> PathBuf {
        let path = dir.join(filename);
        let mut archive = Vec::new();
        archive.extend_from_slice(b"!<arch>\n");
        for (name, data) in members {
            let mut header = [b' '; 60];
            let name_len = name.len().min(16);
            header[..name_len].copy_from_slice(&name[..name_len]);
            let size_str = format!("{}", data.len());
            header[48..48 + size_str.len()].copy_from_slice(size_str.as_bytes());
            header[58..60].copy_from_slice(b"`\n");
            archive.extend_from_slice(&header);
            archive.extend_from_slice(data);
            if data.len() % 2 != 0 {
                archive.push(b'\n');
            }
        }
        std::fs::write(&path, archive).unwrap();
        path
    }

    fn create_short_import_data(symbol: &str, dll: &str) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_le_bytes()); // sig1: IMAGE_FILE_MACHINE_UNKNOWN
        data.extend_from_slice(&0xFFFFu16.to_le_bytes()); // sig2: IMPORT_OBJECT_HDR_SIG2
        data.extend_from_slice(&0u16.to_le_bytes()); // version
        data.extend_from_slice(&0x8664u16.to_le_bytes()); // machine: AMD64
        data.extend_from_slice(&0u32.to_le_bytes()); // time_date_stamp
        let data_len = (symbol.len() + 1 + dll.len() + 1) as u32;
        data.extend_from_slice(&data_len.to_le_bytes()); // size_of_data
        data.extend_from_slice(&0u16.to_le_bytes()); // ordinal_or_hint
        data.extend_from_slice(&0u16.to_le_bytes()); // name_type: IMPORT_OBJECT_NAME
        data.extend_from_slice(symbol.as_bytes());
        data.push(0);
        data.extend_from_slice(dll.as_bytes());
        data.push(0);
        data
    }

    #[test] // allow_core_test (see mod tests doc comment)
    fn test_is_import_library_detects_short_import() {
        let temp_dir = std::env::temp_dir().join("crubit_test_import_lib");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let short_import_data = create_short_import_data("AddInts", "foo.dll");
        let path = create_archive_file(&temp_dir, "test.lib", &[(b"foo.dll/", &short_import_data)]);
        let result = is_import_library(&path);
        let _ = std::fs::remove_dir_all(&temp_dir);
        assert!(result);
    }

    #[test] // allow_core_test (see mod tests doc comment)
    fn test_is_import_library_ignores_regular_coff() {
        let temp_dir = std::env::temp_dir().join("crubit_test_coff_lib");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let coff_data = [0x64, 0x86, 0x01, 0x00]; // IMAGE_FILE_MACHINE_AMD64, 1 section
        let path = create_archive_file(&temp_dir, "test.lib", &[(b"foo.obj/", &coff_data)]);
        let result = is_import_library(&path);
        let _ = std::fs::remove_dir_all(&temp_dir);
        assert!(!result);
    }

    #[test] // allow_core_test (see mod tests doc comment)
    fn test_is_import_library_skips_bookkeeping_symbols() {
        let temp_dir = std::env::temp_dir().join("crubit_test_bookkeeping_lib");
        std::fs::create_dir_all(&temp_dir).unwrap();
        // Symbol table member `/` starting with 0x0000FFFF (65535 symbols).
        let symtab_data = [0x00, 0x00, 0xFF, 0xFF];
        let coff_data = [0x64, 0x86, 0x01, 0x00];
        let path = create_archive_file(
            &temp_dir,
            "test.lib",
            &[(b"/", &symtab_data), (b"foo.obj/", &coff_data)],
        );
        let result = is_import_library(&path);
        let _ = std::fs::remove_dir_all(&temp_dir);
        assert!(!result);
    }

    #[test] // allow_core_test (see mod tests doc comment)
    fn test_is_import_library_finds_short_import_after_coff_members() {
        let temp_dir = std::env::temp_dir().join("crubit_test_mixed_lib");
        std::fs::create_dir_all(&temp_dir).unwrap();
        let coff_data = [0x64, 0x86, 0x01, 0x00];
        let short_import_data = create_short_import_data("AddInts", "foo.dll");
        let path = create_archive_file(
            &temp_dir,
            "test.lib",
            &[(b"null_thunk.obj/", &coff_data), (b"foo.dll/", &short_import_data)],
        );
        let result = is_import_library(&path);
        let _ = std::fs::remove_dir_all(&temp_dir);
        assert!(result);
    }
}
