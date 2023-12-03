load("@rules_kotlin//kotlin:jvm.bzl", "kt_jvm_library")

kt_jvm_library(
    name = "codegen",
    srcs = glob(["src/main/kotlin/**/*.kt"]),
    resources = ["@//spec"] + glob(["src/main/resources/**/*"]),
    visibility = ["//visibility:public"],
    deps = [
        "@//spec-traits",
        "@//:kotlin_serialization",
        "@maven//:com_github_agluszak_jsonrpc4kt",
        "@maven//:com_google_code_gson_gson",
        "@maven//:software_amazon_smithy_smithy_codegen_core",
        "@maven//:software_amazon_smithy_smithy_model",
    ],
)
