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
    "CcBindingsFromRustInfo",
)
load(
    "//cc_bindings_from_rs/test/bazel/unit_tests/unit_test_helpers:attach_aspect.bzl",
    "ActionsInfo",
    "attach_aspect",
)

def _find_actions_by_mnemonic(env, expected_mnemonic):
    """Searches `target_actions` for all actions with `expected_mnemonic`.

    Args:
      env: A test environment struct received from `analysistest.begin(ctx)`
      expected_mnemonic: string to be compared against `Action.mnemonic`
                         (see https://bazel.build/rules/lib/Action#mnemonic)

    Returns:
      Actions  - The actions with the given mnemonic.
    """
    return [
        a
        for a in analysistest.target_under_test(env)[ActionsInfo].actions
        if a.mnemonic == expected_mnemonic
    ]

def _find_action_by_mnemonic(env, expected_mnemonic):
    """Searches `target_actions` for a single one with `expected_mnemonic`.

    Will throw or assert if there are no matching actions, or if there is
    more than one matching action.

    Args:
      env: A test environment struct received from `analysistest.begin(ctx)`
      expected_mnemonic: string to be compared against `Action.mnemonic`
                         (see https://bazel.build/rules/lib/Action#mnemonic)

    Returns:
      Action  - The action with the given mnemonic.
    """
    matching_actions = _find_actions_by_mnemonic(env, expected_mnemonic)
    if len(matching_actions) != 1:
        fail("Expected exactly one action matching %s, got %d: %r" % (expected_mnemonic, len(matching_actions), matching_actions))
    return matching_actions[0]

def _remove_ext(f):
    """Takes a File and returns its `path` with `extension` removed.

    Args:
      f: File - https://bazel.build/rules/lib/File

    Returns:
      string - the path of `f` with `extension` removed
    """
    f.path.removesuffix(f.extension)

def _has_arg_with_suffix(cmdline, suffix):
    """Returns True if there is an arugment on the command line that ends with `suffix`.

    Args:
      cmdline - List[String]
      suffix - String

    Returns:
      bool - Whether `suffix` appears on the command line as a suffix.
    """
    for arg in cmdline:
        if arg.endswith(suffix):
            return True
    return False

def _has_arg_with_prefix(cmdline, prefix):
    """Returns True if there is an arugment on the command line that starts with `prefix`.

    Args:
      cmdline - List[String]
      prefix - String

    Returns:
      bool - Whether `prefix` appears on the command line as a prefix.
    """
    for arg in cmdline:
        if arg.startswith(prefix):
            return True
    return False

def _has_file_with_name_prefix(files, basename_prefix):
    """Returns true if `files` contains an element that starts with `basename_prefix`.

    Args:
      files - List[File]: https://bazel.build/rules/lib/File
      basename_prefix - String

    Returns:
      bool - Whether a file with the given basename prefix is found.
    """
    matching_files = [f for f in files if f.basename.startswith(basename_prefix)]
    return len(matching_files) > 0

def _header_generation_test_impl(ctx):
    env = analysistest.begin(ctx)

    # Verify that `CcBindingsFromRust` propagates inputs and rustc flags from the
    # target create.
    generate_action = _find_action_by_mnemonic(env, "CcBindingsFromRust")
    generate_action_inputs = generate_action.inputs.to_list()
    asserts.true(
        env,
        "rusty_lib.rs" in [i.basename for i in generate_action_inputs],
        "Expected to find `rusty_lib.rs` in the action inputs, got {}.".format(
            generate_action_inputs,
        ),
    )

    # ":emptylib" is a dependency of the crate for which we are generating bindings.
    # Similarly to how it's output libemptylib-{hash}.{rlib|rmeta} is an input to the `Rustc`
    # compile action, it should also be an input to the `CcBindingsFromRust` bindings generating
    # action.
    asserts.true(
        env,
        _has_file_with_name_prefix(generate_action_inputs, "libemptylib-"),
        "Expected to find `libemptylib-HASH` in the action inputs, got {}.".format(generate_action_inputs),
    )

    # Verify that `CcBindingsFromRust` generates:
    # 1) `generated_header` ("..._cc_api.h")
    # 2) `generated_impl` ("..._cc_api_impl.rs")
    generated_outputs = generate_action.outputs.to_list()
    asserts.equals(env, 2, len(generated_outputs))
    generated_header = generated_outputs[0]
    asserts.equals(env, "rusty_lib_cc_api.h", generated_header.basename)
    generated_impl = generated_outputs[1]
    asserts.equals(env, "rusty_lib_cc_api_impl.rs", generated_impl.basename)

    [rustc_action] = [action for action in _find_actions_by_mnemonic(env, "Rustc") if generated_impl.path in [input.path for input in action.inputs.to_list()]]

    # Extract `rustc_rlib` output (and verify it has `rlib` extension).
    rustc_outputs = rustc_action.outputs.to_list()
    asserts.equals(env, 1, len(rustc_outputs))
    rustc_rlib = rustc_outputs[0]
    asserts.equals(env, "rlib", rustc_rlib.extension)

    # Verify that `cc_info.compilation_context.direct_headers` contains `generated_header`.
    target_under_test = analysistest.target_under_test(env)
    asserts.true(env, CcBindingsFromRustInfo in target_under_test)
    cc_info = target_under_test[CcBindingsFromRustInfo].cc_info
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

def _cmdline_flags_test_impl(ctx):
    env = analysistest.begin(ctx)

    bindings_action = _find_action_by_mnemonic(env, "CcBindingsFromRust")
    cmdline = bindings_action.argv

    asserts.true(
        env,
        _has_arg_with_suffix(cmdline, "rusty_lib.rs"),
        "Expected to find `rusty_lib.rs` on the command line, got {}.".format(cmdline),
    )
    asserts.true(
        env,
        "--crate-type=rlib" in cmdline,
        "Expected to find `--crate-type=rlib` on the command line, got {}.".format(cmdline),
    )
    asserts.true(
        env,
        "-Cpanic=abort" in cmdline,
        "Expected to find `-Cpanic=abort` on the command line, got {}.".format(cmdline),
    )

    # ":emptylib" is a dependency of the crate for which we are generating bindings.
    # Similarly to how we pass `--extern=emptylib` to the command line for the `Rustc`
    # compile action, we should also pass it to the `CcBindingsFromRust` bindings generating action.
    asserts.true(
        env,
        _has_arg_with_prefix(cmdline, "--extern=emptylib"),
        "Expected to find `--extern=emptylib` on the command line, got {}.".format(cmdline),
    )

    return analysistest.end(env)

cmdline_flags_test = analysistest.make(_cmdline_flags_test_impl)

def _tests():
    rust_library(
        name = "rusty_lib",
        srcs = ["rusty_lib.rs", "other_file.rs"],
        deps = [":emptylib"],
        tags = ["manual"],
    )

    rust_library(
        name = "emptylib",
        srcs = ["empty.rs"],
    )
    attach_aspect(name = "rusty_lib_bindings", dep = ":rusty_lib")

    header_generation_test(
        name = "header_generation_test",
        target_under_test = ":rusty_lib_bindings",
    )

    cmdline_flags_test(
        name = "cmdline_flags_test",
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
            ":cmdline_flags_test",
        ],
    )
