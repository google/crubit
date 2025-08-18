package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//visibility:public"],
)

environment(name = "non_prod")

environment_group(
    name = "execution_environment",
    defaults = [
        ":non_prod",
    ],
    environments = [
        ":non_prod",
    ],
)
