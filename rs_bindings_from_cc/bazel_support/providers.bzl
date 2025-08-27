# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""All providers involved in C++/Rust interop."""

AdditionalRustSrcsProviderInfo = provider(
    doc = """
The provider that specifies the Rust source files to be included in the Rust crate along with
generated Rust bindings of this C++ target.
""",
    fields = {
        "srcs": "The Rust source files to be included in addition to generated Rust bindings.",
        "namespace_path": "The namespace path for the Rust source files.",
        "deps": "List of DepVariantInfo of other libraries to be linked to this library target. " +
                "These can be either other `rust_library` targets or `cc_library` targets if " +
                "linking a native library.",
        "cc_deps": "List of DepVariantInfo of cc_library targets whose crubit-generated bindings " +
                   "will be linked to this library target.",
    },
)

RustBindingsFromCcInfo = provider(
    doc = ("A provider that contains compile and linking information for the generated" +
           " `.cc` and `.rs` files."),
    fields = {
        "cc_info": ("A CcInfo provider for the implementation of the API projection, " +
                    "or None if this is a real Rust target."),
        "dep_variant_info": ("A DepVariantInfo provider that carries information about the " +
                             "generated bindings (compiled `.rs` file)."),
        "pass_through_dep_variant_infos": ("A depset of DepVariantInfo providers that should be " +
                                           "passed through to downstream bindings in case the " +
                                           "target doesn't get bindings."),
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
        "stl_headers": "The list of STL headers.",
    },
)
