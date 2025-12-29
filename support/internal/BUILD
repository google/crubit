load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "bindings_support",
    srcs = ["check_no_mutable_aliasing.cc"],
    hdrs = [
        "check_no_mutable_aliasing.h",
        "cxx20_backports.h",
        "fmt.h",
        "lazy_init.h",
        "memswap.h",
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
    # Using mature Abseil APIs seems okay - we should be able to assume that
    # Crubit users have a version of Abseil that is relatively recent (although
    # we can't rely on an exact version and/or exact absl/base/options.h).
    deps = [
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:has_ostream_operator",
        "@abseil-cpp//absl/types:span",
    ],
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
        "@abseil-cpp//absl/log:check",
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
