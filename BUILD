load("@rules_license//rules:license.bzl", "license")
load("//gws/tools/gwsq/v3:gwsq_test.bzl", "gwsq_test")

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//visibility:private"],
)

license(
    name = "license",
    package_name = "crubit",
)

licenses(["notice"])

exports_files(["LICENSE"])

gwsq_test(
    name = "crubit_gwsq_test",
    src = "crubit.gwsq",
)
