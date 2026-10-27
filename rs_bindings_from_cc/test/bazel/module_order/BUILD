load("@rules_shell//shell:sh_test.bzl", "sh_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load("//rs_bindings_from_cc/test/golden:golden_test.bzl", "generated_bindings")

package(
    features = ["header_modules"],
)

licenses(["notice"])

# Checks that the bindings don't depend on the order in which precompiled header
# modules are loaded: `:lib_ab` and `:lib_ba` have the same header, but their
# dependencies are in different orders, so their `-fmodule-file` flags are too.
# The two must get the same bindings, except for the target names.
#
# Without header modules (when
# `//rs_bindings_from_cc/bazel_support:use_header_modules` is
# off), both parse the same headers textually in the same order, so the test
# passes trivially.

crubit_test_cc_library(
    name = "lib_ab",
    hdrs = ["lib.h"],
    deps = [
        ":a",
        ":b",
        ":template",
    ],
)

crubit_test_cc_library(
    name = "lib_ba",
    hdrs = ["lib.h"],
    # Do not sort: `:b` must come before `:a`, the reverse of `:lib_ab`.
    deps = [
        ":b",
        ":a",
        ":template",
    ],
)

crubit_test_cc_library(
    name = "a",
    hdrs = ["a.h"],
    deps = [
        ":template",
        "//support:annotations",
    ],
)

crubit_test_cc_library(
    name = "b",
    hdrs = ["b.h"],
    deps = [
        ":template",
        "//support:annotations",
    ],
)

crubit_test_cc_library(
    name = "template",
    hdrs = ["template.h"],
    deps = ["//support:annotations"],
)

generated_bindings(
    name = "lib_ab_bindings",
    cc_library = ":lib_ab",
)

generated_bindings(
    name = "lib_ba_bindings",
    cc_library = ":lib_ba",
)

[
    filegroup(
        name = "%s_%s" % (lib, group),
        srcs = [":%s_bindings" % lib],
        output_group = group,
    )
    for lib in [
        "lib_ab",
        "lib_ba",
    ]
    for group in [
        "rust_file",
        "cc_file",
    ]
]

[
    sh_test(
        name = "same_%s_test" % group,
        srcs = ["same_bindings_test.sh"],
        args = [
            "$(rootpath :lib_ab_%s)" % group,
            "$(rootpath :lib_ba_%s)" % group,
        ],
        data = [
            ":lib_ab_%s" % group,
            ":lib_ba_%s" % group,
        ],
    )
    for group in [
        "rust_file",
        "cc_file",
    ]
]
