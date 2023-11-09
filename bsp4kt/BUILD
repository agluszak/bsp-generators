load("@rules_jvm_external//:kt_defs.bzl", "kt_jvm_export")

kt_jvm_export(
    name = "bsp4kt",
    srcs = glob(["src/**/*.kt"]),
    maven_coordinates = "org.jetbrains.bsp:bsp4kt:0.1.0",
    visibility = ["//visibility:public"],
    deps = [
        "//:kotlin_serialization",
        "@maven//:com_github_agluszak_jsonrpc4kt",
        "@maven//:org_jetbrains_kotlinx_kotlinx_serialization_json",
    ],
)
