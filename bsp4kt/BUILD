load("@io_bazel_rules_kotlin//kotlin:jvm.bzl", "kt_jvm_library")

kt_jvm_library(
    name = "bsp4kt",
    srcs = glob(["src/**/*.kt"]),
    plugins = ["@com_github_jetbrains_kotlin//:kotlinx-serialization-compiler-plugin"],
    deps = [
        "@maven//:org_jetbrains_kotlinx_kotlinx_serialization_json",
        "@maven//:com_github_agluszak_jsonrpc4kt",
    ],
)
