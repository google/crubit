// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Vocabulary library for multi-platform tests which use cross-compilation.

use proc_macro2::TokenStream;
use quote::quote;
use std::sync::LazyLock;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Platform {
    X86Linux,
    ArmLinux,
    X86MacOS,
    ArmMacOS,
    X86Windows,
}

impl Platform {
    pub fn target_triple(self) -> &'static str {
        match self {
            Platform::X86Linux => "x86_64-unknown-linux-gnu",
            Platform::ArmLinux => "aarch64-unknown-linux-gnu",
            Platform::X86MacOS => "x86_64-apple-darwin",
            Platform::ArmMacOS => "arm64-apple-darwin",
            Platform::X86Windows => "x86_64-pc-windows-msvc",
        }
    }

    /// Returns whether the platform uses the Microsoft C++ ABI rather than the
    /// Itanium one.  The two disagree about name mangling, record layout, and
    /// the underlying type of an unfixed `enum`, so some test expectations
    /// have to differ.
    pub fn uses_msvc_cxx_abi(self) -> bool {
        matches!(self, Platform::X86Windows)
    }

    /// Returns the spelling of the `no_unique_address` attribute.
    ///
    /// Clang ignores `[[no_unique_address]]` when it targets the Microsoft C++
    /// ABI, where the attribute is spelled `[[msvc::no_unique_address]]`.
    pub fn no_unique_address_attr(self) -> &'static str {
        if self.uses_msvc_cxx_abi() {
            "[[msvc::no_unique_address]]"
        } else {
            "[[no_unique_address]]"
        }
    }

    /// Returns the `::ffi_11` type and constructor that Crubit uses for an
    /// unscoped C++ `enum` that has no fixed underlying type and only
    /// non-negative enumerators.  Clang infers `unsigned int` for such an
    /// `enum` under the Itanium C++ ABI, but `int` under the Microsoft C++ ABI.
    pub fn unfixed_nonnegative_enum_ffi_type(self) -> (TokenStream, TokenStream) {
        if self.uses_msvc_cxx_abi() {
            (quote! { c_int }, quote! { new_c_int })
        } else {
            (quote! { c_uint }, quote! { new_c_uint })
        }
    }

    /// Returns whether Clang supports the `vectorcall` calling convention on
    /// the platform.  `vectorcall` is only available on x86 targets (at least
    /// on ones currently supported by Crubit;  I hear that UEFI/x86 doesn't
    /// support `vectorcall`).
    pub fn supports_vectorcall(self) -> bool {
        matches!(self, Platform::X86Linux | Platform::X86MacOS | Platform::X86Windows)
    }

    /// Returns the prefix that Clang adds to the mangled name of a function
    /// declared with an asm label (e.g. `int f() asm("foo");`).
    ///
    /// On targets where symbol names get a global prefix (e.g. `_` on Mach-O),
    /// Clang adds a `'\u{1}'` prefix to tell LLVM not to add the global prefix
    /// to the asm label.
    pub fn asm_label_mangled_name_prefix(self) -> &'static str {
        match self {
            Platform::X86MacOS | Platform::ArmMacOS => "\u{1}",
            Platform::X86Linux | Platform::ArmLinux | Platform::X86Windows => "",
        }
    }
}

/// Returns the platform the current test is running for with
/// multiplatform_rust_test.
pub fn test_platform() -> Platform {
    *TEST_PLATFORM.as_ref().unwrap()
}

static TEST_PLATFORM: LazyLock<Result<Platform, String>> = LazyLock::new(|| {
    let env = std::env::var("CRUBIT_TEST_PLATFORM")
        .map_err(|_| "multiplatform tests must use `multiplatform_rust_test`.".to_string())?;
    let platform = match env.as_str() {
        "x86_linux" => Platform::X86Linux,
        "arm_linux" => Platform::ArmLinux,
        "darwin_x86_64" => Platform::X86MacOS,
        "darwin_arm64" => Platform::ArmMacOS,
        "windows_x86_64" => Platform::X86Windows,
        _ => return Err(format!("Unknown platform: {env}")),
    };
    Ok(platform)
});
