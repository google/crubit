# Various utilities needed by rs_bindings_from_cc

licenses(["notice"])

package(default_visibility = ["//rs_bindings_from_cc:__subpackages__"])

cc_library(
    name = "check",
    hdrs = ["check.h"],
    deps = [
        "//third_party/absl/base:core_headers",
        "@llvm///llvm:Support",
    ],
)

cc_library(
    name = "strong_int",
    hdrs = ["strong_int.h"],
    deps = [
        "//third_party/absl/base:core_headers",
        "//third_party/absl/meta:type_traits",
        "//third_party/absl/strings",
    ],
)

cc_library(
    name = "string_type",
    hdrs = ["string_type.h"],
    deps = [
        "//third_party/absl/container:flat_hash_set",
        "//third_party/absl/flags:marshalling",
        "//third_party/absl/meta:type_traits",
        "//third_party/absl/strings",
    ],
)

cc_library(
    name = "status_macros",
    hdrs = ["status_macros.h"],
    deps = [
        "//third_party/absl/base:core_headers",
        "//third_party/absl/status",
        "//third_party/absl/status:statusor",
        "//third_party/absl/types:source_location",
    ],
)
