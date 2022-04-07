# Various utilities needed by rs_bindings_from_cc

licenses(["notice"])

package(default_visibility = ["//third_party/crubit:__subpackages__"])

cc_library(
    name = "check",
    hdrs = ["check.h"],
    deps = [
        "@absl//base:core_headers",
        "@llvm///llvm:Support",
    ],
)

cc_library(
    name = "strong_int",
    hdrs = ["strong_int.h"],
    deps = [
        "@absl//base:core_headers",
        "@absl//meta:type_traits",
    ],
)

cc_library(
    name = "string_type",
    hdrs = ["string_type.h"],
    deps = [
        "@absl//container:flat_hash_set",
        "@absl//flags:marshalling",
        "@absl//meta:type_traits",
        "@absl//strings",
    ],
)

cc_library(
    name = "status_macros",
    hdrs = ["status_macros.h"],
    deps = [
        "@absl//base:core_headers",
        "@absl//status",
        "@absl//status:statusor",
        "@absl//types:source_location",
    ],
)
