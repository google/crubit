# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""All providers involved in C++/Rust interop."""

RustBindingsFromCcInfo = provider(
    doc = ("A provider that contains compile and linking information for the generated" +
           " `.cc` and `.rs` files."),
    fields = {
        "cc_info": ("A CcInfo provider for the implementation of the API projection, " +
                    "or None if this is a real Rust target."),
        "dep_variant_info": ("A DepVariantInfo provider that carries information from the " +
                             "compiled `.rs` file."),
        "target_args": ("A depset of strings, each one representing a mapping of target " +
                        "to its per-target arguments (headers, features) in json format:\n\n" +
                        "{'t': <target>, 'h': [<header>], 'f': [<feature>]}"),
        "namespaces": ("A json file containing the namespace hierarchy for the target we " +
                       "are generating bindings for, or None."),
    },
)

RustToolchainHeadersInfo = provider(
    doc = "A provider that contains all toolchain C++ headers",
    fields = {"headers": "depset"},
)

GeneratedBindingsInfo = provider(
    doc = "A provider that contains the generated C++ and Rust source files.",
    fields = {
        "cc_file": "The generated C++ source file.",
        "rust_file": "The generated Rust source file.",
        "namespaces_file": "The generated namespace hierarchy in JSON format.",
    },
)

DepsForBindingsInfo = provider(
    doc = """A provider that serves to pass on dependencies needed when compiling the generated
          Rust and C++ files.""",
    fields = {
        "deps_for_rs_file": "list[DepVariantInfo]",
        "deps_for_cc_file": "list[CcInfo]",
    },
)

RustBindingsFromCcToolchainInfo = provider(
    doc = """A provider for platform-specific data, provided as a toolchain.""",
    fields = {
        "binary": "The label for the rs_bindings_from_cc binary",
        "builtin_headers": "The list of clang builtin headers.",
        "is_on_demand": "Whether this is a dynamically built binary or not",
    },
)
