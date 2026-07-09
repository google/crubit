load("@bazel_skylib//lib:selects.bzl", "selects")
load("@rules_license//rules:license.bzl", "license")

package(
    default_applicable_licenses = [":license"],
    default_visibility = ["//visibility:private"],
)

license(
    name = "license",
    package_name = "crubit",
)

licenses(["notice"])

exports_files(["LICENSE"])

# LINT.IfChange
# `crubit_supported` (and `supported_platforms`) can be used as a select condition
# to distinguish platforms where Crubit (and rs_bindings_from_cc in particular) is
# supported from those where it is not.

alias(
    name = "crubit_supported",
    actual = ":supported_platforms",
    visibility = ["//visibility:public"],
)
# LINT.ThenChange(//depot/common/bazel_support/toolchains/BUILD)
