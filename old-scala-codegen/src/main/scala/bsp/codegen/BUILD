load("@io_bazel_rules_scala//scala:scala.bzl", "scala_library")

scala_library(
    name = "old-scala-codegen",
    srcs = glob(["**/*.scala"]),
    resources = ["@//spec"],
    visibility = ["//visibility:public"],
    deps = [
        "@//spec-traits",
        "@maven//:software_amazon_smithy_smithy_codegen_core",
        "@maven//:software_amazon_smithy_smithy_model",
    ],
)
