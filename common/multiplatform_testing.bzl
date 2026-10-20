# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""Supporting macro for multiplatform code."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_test",
)

_PLATFORMS = [
    "x86_linux",
    "arm_linux",
    "darwin_x86_64",
    "darwin_arm64",
    "windows_x86_64",
]

# The platform whose `rust_test` target owns the compiled test binary. The targets for the other
# platforms re-run that binary rather than compiling their own copy.
_BASE_PLATFORM = _PLATFORMS[0]

# Test attributes which `multiplatform_rust_test` forwards from the `rust_test` to the per-platform
# wrapper tests, so that all platforms are tagged, sized, and sharded alike.
_FORWARDED_TEST_ATTRS = [
    "flaky",
    "local",
    "shard_count",
    "size",
    "tags",
    "target_compatible_with",
    "timeout",
    "visibility",
]

def _platform_rust_test_impl(ctx):
    test = ctx.attr.test
    test_default_info = test[DefaultInfo]

    # Bazel requires that the `executable` provided by an executable rule be created by the same
    # rule, so we cannot just forward the `DefaultInfo` of the wrapped test; symlink to it instead.
    executable = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.symlink(
        output = executable,
        target_file = test_default_info.files_to_run.executable,
        is_executable = True,
    )

    env = {}
    inherited_env = []
    if RunEnvironmentInfo in test:
        env = dict(test[RunEnvironmentInfo].environment)
        inherited_env = list(test[RunEnvironmentInfo].inherited_environment)
    env["CRUBIT_TEST_PLATFORM"] = ctx.attr.platform

    providers = [
        DefaultInfo(
            executable = executable,
            runfiles = test_default_info.default_runfiles,
        ),
        RunEnvironmentInfo(
            environment = env,
            inherited_environment = inherited_env,
        ),
        coverage_common.instrumented_files_info(ctx, dependency_attributes = ["test"]),
    ]
    if testing.ExecutionInfo in test:
        providers.append(test[testing.ExecutionInfo])
    return providers

_platform_rust_test = rule(
    implementation = _platform_rust_test_impl,
    doc = """Runs an already-compiled `rust_test` with `CRUBIT_TEST_PLATFORM` set to `platform`.

    This lets `multiplatform_rust_test` compile and link its test binary once, rather than once per
    platform: the platform is only read at runtime (see `multiplatform_testing.rs`).
    """,
    attrs = {
        "test": attr.label(
            doc = "The `rust_test` whose binary is run.",
            mandatory = True,
            executable = True,
            cfg = "target",
        ),
        "platform": attr.string(
            doc = "The value of `CRUBIT_TEST_PLATFORM` to run the test with.",
            mandatory = True,
            values = _PLATFORMS,
        ),
    },
    test = True,
)

def multiplatform_rust_test(name, **kwargs):
    """Macro to parameterize a test target by target platform.

    The test binary is compiled and linked only once, by the `rust_test` for `x86_linux`
    (`<name>-x86_linux`). The tests for the remaining platforms are thin wrappers which run that
    same binary with a different value of the `CRUBIT_TEST_PLATFORM` environment variable.

    Args:
      name: The name of the `test_suite` grouping the per-platform tests, which are named
        `<name>-<platform>`.
      **kwargs: Arguments for the underlying `rust_test`.
    """

    visibility = kwargs.get("visibility", None)
    if visibility:
        test_suite_kwargs = {"visibility": visibility}
    else:
        test_suite_kwargs = {}

    # TODO(jeanpierreda): Ideally we'd use `.`, not `-`, but this breaks for non-crate= rust_test targets
    # because they create a crate with `.` in the name. That's illegal.
    native.test_suite(
        name = name,
        tests = [name + "-" + platform for platform in _PLATFORMS],
        **test_suite_kwargs
    )

    base_test_name = name + "-" + _BASE_PLATFORM
    env = kwargs.setdefault("env", {})
    env["CRUBIT_TEST_PLATFORM"] = _BASE_PLATFORM

    # Note: we use `rust_test` rather than `crubit_rust_test` because we want unit tests to
    # run using the current build platform, not the platform built with the updated version
    # of crubit.
    rust_test(
        name = base_test_name,
        **kwargs
    )

    wrapper_kwargs = {attr: kwargs[attr] for attr in _FORWARDED_TEST_ATTRS if attr in kwargs}
    for platform in _PLATFORMS:
        if platform == _BASE_PLATFORM:
            continue
        _platform_rust_test(
            name = name + "-" + platform,
            test = ":" + base_test_name,
            platform = platform,
            **wrapper_kwargs
        )
