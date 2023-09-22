load("@io_bazel_rules_scala//scala:scala.bzl", "scala_library")

scala_library(
    name = "blahblah",
    srcs = glob(["**/*.scala"]),
    resources = ["@//spec"],
    visibility = ["//visibility:public"],
    deps = [
        "@//spec-traits",
        "@maven//:software_amazon_smithy_smithy_codegen_core",
        "@maven//:software_amazon_smithy_smithy_model",
        "@maven//:software_amazon_smithy_smithy_utils",
        "@maven//:com_lihaoyi_os_lib_2_13",
        "@maven//:com_lihaoyi_geny_2_13",
        "@maven//:com_monovore_decline_2_13",
        "@maven//:org_typelevel_cats_core_2_13",
        "@maven//:org_typelevel_cats_kernel_2_13"
    ],
)
