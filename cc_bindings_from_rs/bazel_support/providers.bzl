# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""Providers used/generated by cc_bindings_from_rs."""

CcBindingsFromRustInfo = provider(
    doc = ("A provider that contains compile and linking information for the generated" +
           " `.rs` and `.h` files."),
    fields = {
        "cc_info": "A CcInfo provider for the C++ API.",
        # TODO(b/271857814): A `CRATE_NAME` might not be globally unique - the
        # key needs to also cover a "hash" of the crate version and compilation
        # flags.
        "crate_key": "String with a crate key to use in --other-crate-bindings",
        "headers": "A list of C++ headers which correspond to this crate.",
        "features": "A list of features enabled for the bindings for this crate.",
        "configuration": "A CcBindingsFromRustLibraryConfigInfo provider.",
    },
)

GeneratedBindingsInfo = provider(
    doc = "A provider that contains the generated C++ and Rust files.",
    fields = {
        "h_file": "The generated C++ header file.",
        "rust_file": "The generated Rust source file.",
    },
)

CcBindingsFromRustToolchainInfo = provider(
    doc = """A provider for platform-specific data, provided as a toolchain.""",
    fields = {
        "binary": "The label for the cc_bindings_from_rs binary",
    },
)
