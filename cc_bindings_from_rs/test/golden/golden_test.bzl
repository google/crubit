# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Golden tests for cc_bindings_from_rs.

`golden_test` generates the bindings for a Rust library on the host and,
optionally, for Android, iOS and Windows target platforms, and diffs them
against goldens checked in next to the library.
"""

load(
    "@rules_rust//rust:defs.bzl",
    rust_library_rule = "rust_library",
)
load("@rules_rust//rust/private:providers.bzl", "CrateInfo")  # buildifier: disable=bzl-visibility
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_cli_flag_aspect_hint.bzl",
    "cc_bindings_from_rust_cli_flag",
)
load("//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl", "cc_bindings_from_rust_library_config")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust_aspect",
)
load(
    "//cc_bindings_from_rs/bazel_support:providers.bzl",
    "GeneratedBindingsInfo",
)
load(
    "//google_internal/build_flavors:crubit_build_flavors_android.bzl",
    "CRUBIT_ANDROID_ABI_TIERS",
    "CRUBIT_ANDROID_PLATFORMS",
    "CRUBIT_TAGS_MAPPING",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "ENABLE_CRUBIT_ASSERTIONS_ATTR",
    "crubit_multiplatform_golden_transition",
)
load(
    "//google_internal/build_flavors:crubit_build_flavors_ios.bzl",
    "CRUBIT_IOS_SIM_PLATFORMS",
    "CRUBIT_IOS_SIM_TAGS_MAPPING",
)
load(
    "//google_internal/build_flavors:crubit_build_flavors_windows.bzl",
    "CRUBIT_WINDOWS_ABI_TIERS",
    "CRUBIT_WINDOWS_PLATFORMS",
    "CRUBIT_WINDOWS_TAGS_MAPPING",
)

# Golden subtests for each `platforms` value of `golden_test`:
# - `archs`: arch name -> the target platform and the ABI tier whose goldens the
#   arch shares (see `_generate_golden_subtest`).
# - `skip_tags`: `no_test_<platform>_<arch>` tag -> the arch it skips.
# The subtests only generate bindings, so they also cover build-only CPUs such
# as Windows arm64. Nothing is linked, so setting `--platforms` is enough:
# unlike `crubit_windows_cc_test`, the Windows subtests don't need the extra
# flags that `--config=windows_<cpu>` sets, such as `--dynamic_mode=off`.
_GOLDEN_PLATFORMS = {
    "android": struct(
        archs = {
            Label(target_cpu).name: struct(
                platform = platform,
                abi_tier = CRUBIT_ANDROID_ABI_TIERS[target_cpu],
            )
            for target_cpu, platform in CRUBIT_ANDROID_PLATFORMS.items()
        },
        skip_tags = {
            tag: Label(target_cpu).name
            for tag, target_cpu in CRUBIT_TAGS_MAPPING.items()
        },
    ),
    "ios": struct(
        archs = {
            arch: struct(platform = platform, abi_tier = "ios")
            for arch, platform in CRUBIT_IOS_SIM_PLATFORMS.items()
        },
        skip_tags = CRUBIT_IOS_SIM_TAGS_MAPPING,
    ),
    "windows": struct(
        archs = {
            cpu: struct(platform = config.platform, abi_tier = CRUBIT_WINDOWS_ABI_TIERS[cpu])
            for cpu, config in CRUBIT_WINDOWS_PLATFORMS.items()
        },
        skip_tags = CRUBIT_WINDOWS_TAGS_MAPPING,
    ),
}

_ALL_SKIP_TAGS = [
    tag
    for platform_config in _GOLDEN_PLATFORMS.values()
    for tag in platform_config.skip_tags
]

def _generate_bindings_impl(ctx):
    rust_library = ctx.attr.rust_library[0]
    if GeneratedBindingsInfo not in rust_library:
        fail("Bindings were not generated for the given rust_library.")
    bindings = rust_library[GeneratedBindingsInfo]
    return OutputGroupInfo(
        h_file = [bindings.h_file],
        rust_file = [bindings.rust_file],
    )

_generate_bindings = rule(
    attrs = ENABLE_CRUBIT_ASSERTIONS_ATTR | {
        # Multi-platform variant of crubit_flavor_transition.
        "rust_library": attr.label(
            providers = [CrateInfo],
            aspects = [cc_bindings_from_rust_aspect],
            cfg = crubit_multiplatform_golden_transition,
        ),
        "target_platform": attr.label(),
        # Synthetic dependency to ensure even a coarse `bazel query` analysis finds a transitive
        # dependency from Crubit tool sources to golden test bindings.
        "_cc_bindings_from_rs_binary": attr.label(
            default = "//cc_bindings_from_rs",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
    },
    implementation = _generate_bindings_impl,
)

def _generate_golden_subtest(
        name,
        basename,
        rust_library,
        tags,
        golden_h,
        golden_rs,
        target_platform = None,
        golden_dir = None,
        abi_tier = None):
    """Instantiates binding generation, output filegroups, and sh_test for a platform configuration."""

    bindings_name = basename + ".generated_bindings"
    _generate_bindings(
        name = bindings_name,
        rust_library = rust_library,
        target_platform = target_platform,
        testonly = True,
    )

    sh_args = []
    if golden_dir:
        sh_args.extend(["--platform", golden_dir])
    if abi_tier:
        sh_args.extend(["--tier", abi_tier])

    data = ["//common:LICENSE_HEADER"]
    owned_files = []

    for output_grp, golden_file in (("h_file", golden_h), ("rust_file", golden_rs)):
        if not golden_file:
            continue
        new_file = "%s.%s" % (basename, output_grp)
        native.filegroup(
            name = new_file,
            srcs = [bindings_name],
            output_group = output_grp,
            testonly = True,
        )

        # Hierarchical golden resolution order:
        # 1. Architecture-specific override (e.g. goldens/android_x86/foo.h)
        # 2. ABI-tier shared golden (e.g. goldens/android_32/foo.h)
        # 3. Default host golden (foo.h)
        read_target = golden_file
        candidates = []
        if golden_dir:
            candidates.append("goldens/%s/%s" % (golden_dir, golden_file))
        if abi_tier and abi_tier != golden_dir:
            candidates.append("goldens/%s/%s" % (abi_tier, golden_file))

        for cand in candidates:
            if native.glob([cand]):
                read_target = cand
                break

        sh_args.extend(["$(location %s)" % read_target, "$(location %s)" % new_file])

        data.extend([read_target, new_file])
        owned_files.append(read_target)

    if not native.package_name().startswith("third_party/crosstool/"):
        native.sh_test(
            name = name,
            srcs = ["//common:golden_test.sh"],
            args = sh_args,
            data = data,
            tags = tags,
            testonly = True,
        )

    native.filegroup(
        name = basename + ".build_cleaner_optout",
        srcs = owned_files,
        tags = ["ignore_srcs"],
        visibility = ["//visibility:private"],
        testonly = True,
    )

def golden_test(
        name,
        rust_library,
        tags = None,
        basename = None,
        golden_h = None,
        golden_rs = None,
        platforms = None,
        kythe_annotations = False):
    """Generates a golden test for `rust_library`.

    Args:
        name: The name of the golden test.
        rust_library: The Rust library whose outputs should be checked.
        tags: The test tags.
        basename: The name to use for generated files.
        golden_h: The generated C++ source code for the bindings.
        golden_rs: The generated Rust source code for the bindings.
        platforms: Additional target platforms to generate subtests for: any of
            "android", "ios" and "windows". Defaults to None.
        kythe_annotations: Whether to generate Kythe annotations.
    """
    if not basename:
        basename = name

    tags = list(tags) if tags else []
    tags.append("crubit_golden_test")

    platforms = platforms or []

    # Turn on annotations if necessary.
    # TODO(jeanpierreda): Move this out to a separate
    # target.
    kythe_annotations_flag = []
    if kythe_annotations:
        kythe_annotations_flag_name = "kythe_annotations_" + rust_library
        cc_bindings_from_rust_cli_flag(
            name = kythe_annotations_flag_name,
            flags = "--kythe-annotations",
        )
        kythe_annotations_flag = [":" + kythe_annotations_flag_name]

    # Since we have patched the rust_library name, we need to keep the original crate
    # name as the namespace name otherwise users get confused.
    top_level_namespace = "top_level_namespace" + rust_library
    cc_bindings_from_rust_library_config(
        name = top_level_namespace,
        namespace = rust_library,
    )
    args = {}
    for key, value in native.existing_rule(rust_library).items():
        if key != "kind" and key != "name" and value:
            args[key] = value
    patched_name = rust_library + "_golden"
    args["name"] = patched_name
    if "aspect_hints" in args:
        args["aspect_hints"] = list(args["aspect_hints"])
    else:
        args["aspect_hints"] = []
    args["aspect_hints"].append(":" + top_level_namespace)
    args["aspect_hints"] += kythe_annotations_flag
    rust_library_rule(
        **args
    )

    # 1. Always generate the host test with the original name.
    _generate_golden_subtest(
        name = name,
        basename = basename,
        rust_library = patched_name,
        tags = tags,
        golden_h = golden_h,
        golden_rs = golden_rs,
    )

    # 2. Generate multi-platform sub-tests for requested platforms.
    # `no_test_<platform>_<arch>` tags only choose which subtests to create, so
    # they are not propagated to the subtests or test_suites.
    subtest_tags = [t for t in tags if t not in _ALL_SKIP_TAGS]

    for platform in platforms:
        if platform not in _GOLDEN_PLATFORMS:
            fail("Unsupported platform for golden_test: %s" % platform)
        platform_config = _GOLDEN_PLATFORMS[platform]
        skipped_archs = [
            platform_config.skip_tags[t]
            for t in tags
            if t in platform_config.skip_tags
        ]

        platform_tests = []
        for arch, arch_config in platform_config.archs.items():
            if arch in skipped_archs:
                continue
            arch_dir = "%s_%s" % (platform, arch)
            subtest_name = "%s_%s" % (name, arch_dir)
            _generate_golden_subtest(
                name = subtest_name,
                basename = "%s_%s" % (basename, arch_dir),
                rust_library = patched_name,
                tags = subtest_tags,
                golden_h = golden_h,
                golden_rs = golden_rs,
                target_platform = arch_config.platform,
                golden_dir = arch_dir,
                abi_tier = arch_config.abi_tier,
            )
            platform_tests.append(":" + subtest_name)

        if platform_tests:
            native.test_suite(
                name = "%s_on_%s" % (name, platform),
                tests = platform_tests,
                tags = subtest_tags,
                visibility = ["//visibility:private"],
            )
