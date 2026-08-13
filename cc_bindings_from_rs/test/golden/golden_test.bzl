# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A rule that generates bindings source files for a given Rust library."""

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
    "CRUBIT_ANDROID_PLATFORMS",
    "CRUBIT_TAGS_MAPPING",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_multiplatform_golden_transition",
)

def _generate_bindings_impl(ctx):
    rust_library = ctx.attr.rust_library[0]
    if not GeneratedBindingsInfo in rust_library:
        fail("Bindings were not generated for the given rust_library.")
    bindings = rust_library[GeneratedBindingsInfo]
    return OutputGroupInfo(
        h_file = [bindings.h_file],
        rust_file = [bindings.rust_file],
    )

_generate_bindings = rule(
    attrs = {
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
        golden_dir = None):
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
        sh_args += ["--platform", golden_dir]

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

        # Prefer platform-specific golden if present; otherwise fall back to host golden.
        read_target = golden_file
        if golden_dir:
            best_file = "goldens/%s/%s" % (golden_dir, golden_file)
            if native.glob([best_file]):
                read_target = best_file

        sh_args.extend(["$(location %s)" % read_target, "$(location %s)" % new_file])
        if golden_dir:
            # Pass base filename so golden_test.sh can bootstrap platform goldens under WRITE_GOLDENS=1.
            sh_args.append(golden_file)

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
        platforms = [],
        kythe_annotations = False):
    """Generates a golden test for `rust_library`.

    Args:
        name: The name of the golden test.
        rust_library: The Rust library whose outputs should be checked.
        tags: The test tags.
        basename: The name to use for generated files.
        golden_h: The generated C++ source code for the bindings.
        golden_rs: The generated Rust source code for the bindings.
        platforms: List of additional target platforms to generate tests for (e.g. ["android"]). Defaults to [].
        kythe_annotations: Whether to generate Kythe annotations.
    """
    if not basename:
        basename = name
    if not tags:
        tags = []
    tags.append("crubit_golden_test")

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
    for platform in platforms:
        if platform == "android":
            android_tests = []

            # Skip architectures matching exclusion tags (e.g. no_test_android_x86).
            excluded_cpus = [CRUBIT_TAGS_MAPPING[t] for t in tags if t in CRUBIT_TAGS_MAPPING]

            # Strip exclusion tags so non-excluded subtests aren't filtered out by tag filters.
            subtest_tags = [t for t in tags if t not in CRUBIT_TAGS_MAPPING]
            for target_cpu, platform_label in CRUBIT_ANDROID_PLATFORMS.items():
                if target_cpu in excluded_cpus:
                    continue
                arch_dir = "android_" + Label(target_cpu).name
                subtest_name = "%s_%s" % (name, arch_dir)
                _generate_golden_subtest(
                    name = subtest_name,
                    basename = "%s_%s" % (basename, arch_dir),
                    rust_library = patched_name,
                    tags = subtest_tags,
                    golden_h = golden_h,
                    golden_rs = golden_rs,
                    target_platform = platform_label,
                    golden_dir = arch_dir,
                )
                android_tests.append(":" + subtest_name)

            if android_tests:
                native.test_suite(
                    name = name + "_on_android",
                    tests = android_tests,
                    tags = tags,
                    visibility = ["//visibility:private"],
                )
        else:
            fail("Unsupported platform for golden_test: %s" % platform)
