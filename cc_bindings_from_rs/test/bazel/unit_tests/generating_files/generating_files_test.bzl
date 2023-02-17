# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for the main generated files of cc_bindings_from_rs."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)

def _find_action_by_mnemonic(env, expected_mnemonic):
    """Searches `target_actions` for a single one with `expected_mnemonic`.

    Will throw or assert if there are no matching actions, or if there is
    more than one matching action.

    Args:
      env: A test environment struct received from `analysistest.begin(ctx)`
      expected_mnemonic: string to be compared against `Action.mnemonic`
                         (see https://bazel.build/rules/lib/Action#mnemonic)

    Returns:
      Action  - the path of `f` with `extension` removed
    """
    matching_actions = [
        a
        for a in analysistest.target_actions(env)
        if a.mnemonic == expected_mnemonic
    ]
    asserts.equals(env, 1, len(matching_actions))
    return matching_actions[0]

def _remove_ext(f):
    """Takes a File and returns its `path` with `extension` removed.

    Args:
      f: File - https://bazel.build/rules/lib/File

    Returns:
      string - the path of `f` with `extension` removed
    """
    f.path.removesuffix(f.extension)

def _header_generation_test_impl(ctx):
    env = analysistest.begin(ctx)

    # Verify that `CcBindingsFromRust` propagates inputs and rustc flags from the
    # target create.
    generate_action = _find_action_by_mnemonic(env, "CcBindingsFromRust")
    asserts.true(env, "rusty_lib_crate_root.rs" in [i.basename for i in generate_action.inputs.to_list()])
    generate_cmdline = " ".join(generate_action.argv)
    asserts.true(env, "rusty_lib_crate_root.rs" in generate_cmdline)
    asserts.true(env, "--crate-type rlib" in generate_cmdline)
    asserts.true(env, "--codegen panic=abort" in generate_cmdline)

    # Verify that `CcBindingsFromRust` generates:
    # 1) `generated_header` ("..._cc_api.h")
    # 2) `generated_impl` ("..._cc_api_impl.rs")
    generated_outputs = generate_action.outputs.to_list()
    asserts.equals(env, 2, len(generated_outputs))
    generated_header = generated_outputs[0]
    asserts.equals(env, "rusty_lib_cc_api.h", generated_header.basename)
    generated_impl = generated_outputs[1]
    asserts.equals(env, "rusty_lib_cc_api_impl.rs", generated_impl.basename)

    # Verify that `generated_impl` is an input for `rustc_action`.
    rustc_action = _find_action_by_mnemonic(env, "Rustc")
    rustc_input_paths = [i.path for i in rustc_action.inputs.to_list()]
    asserts.true(env, generated_impl.path in rustc_input_paths)

    # Extract `rustc_rlib` output (and verify it has `rlib` extension).
    rustc_outputs = rustc_action.outputs.to_list()
    asserts.equals(env, 1, len(rustc_outputs))
    rustc_rlib = rustc_outputs[0]
    asserts.equals(env, "rlib", rustc_rlib.extension)

    # Verify that `cc_info.compilation_context.direct_headers` contains `generated_header`.
    target_under_test = analysistest.target_under_test(env)
    asserts.true(env, CcInfo in target_under_test)
    cc_info = target_under_test[CcInfo]
    asserts.true(env, len(cc_info.compilation_context.direct_headers) == 1)
    cc_info_header = cc_info.compilation_context.direct_headers[0]
    asserts.equals(env, generated_header, cc_info_header)

    # Verify that `cc_info.linker_input.linker_inputs` contains `rustc_rlib`.
    cc_info_links_rustc_output = False
    for linker_input in cc_info.linking_context.linker_inputs.to_list():
        static_libs = [lib.static_library for lib in linker_input.libraries if lib.static_library]
        for static_lib in static_libs:
            # Using `_remove_ext` because `rustc_rlib` has `.rlib`
            # extension, but `static_library` has `.a` extension.
            if _remove_ext(rustc_rlib) == _remove_ext(static_lib):
                cc_info_links_rustc_output = True
    asserts.true(env, cc_info_links_rustc_output)

    return analysistest.end(env)

header_generation_test = analysistest.make(_header_generation_test_impl)

def _tests():
    rust_library(
        name = "rusty_lib",
        srcs = ["rusty_lib_crate_root.rs"],
        tags = ["manual"],
    )

    cc_bindings_from_rust(
        name = "rusty_lib_bindings",
        crate = ":rusty_lib",
        tags = ["manual"],
    )

    header_generation_test(
        name = "header_generation_test",
        target_under_test = ":rusty_lib_bindings",
    )

def generating_files_test(name):
    """Sets up generating_files_test test suite.

    Args:
      name: name of the test suite"""
    _tests()

    native.test_suite(
        name = name,
        tests = [
            ":header_generation_test",
        ],
    )
