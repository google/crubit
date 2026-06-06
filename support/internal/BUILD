load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("//build_integration:build_integration.bzl", "bazel_to_builds")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "bindings_support",
    srcs = ["check_no_mutable_aliasing.cc"],
    hdrs = [
        "check.h",
        "check_no_mutable_aliasing.h",
        "cxx20_backports.h",
        "fmt.h",
        "memswap.h",
        "move_assign.h",
        "offsetof.h",
        "sizeof.h",
        "slot.h",
    ],
    compatible_with = ["//buildenv/target:non_prod"],
    copts = ["-Wshadow"],
    visibility = [
        "//visibility:public",
    ],
    # It is important to be thoughtful when adding new dependencies for
    # `bindings_support` (and possibly other targets in this BUILD file).
    deps = [],
)

cc_test(
    name = "check_no_mutable_aliasing_test",
    srcs = ["check_no_mutable_aliasing_test.cc"],
    deps = [
        ":bindings_support",
        "//testing/base/public:gunit_main",
    ],
)

crubit_cc_test(
    name = "memswap_test",
    srcs = ["memswap_test.cc"],
    deps = [
        ":bindings_support",
        "//testing/base/public:gunit_main",
    ],
)

crubit_cc_test(
    name = "offsetof_test",
    srcs = ["offsetof_test.cc"],
    deps = [
        ":bindings_support",
        "//testing/base/public:gunit_main",
    ],
)

crubit_cc_test(
    name = "slot_test",
    srcs = ["slot_test.cc"],
    deps = [
        ":bindings_support",
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/log:absl_check",
    ],
)

crubit_cc_test(
    name = "sizeof_test",
    srcs = ["sizeof_test.cc"],
    deps = [
        ":bindings_support",
        "//testing/base/public:gunit_main",
    ],
)

bazel_to_builds(
    name = "generate_builds",
    cmake = "CMakeLists.txt",
    gn = "BUILD.gn",
    targets = [
        ":bindings_support",
    ],
)
