load("@rules_jvm_external//:kt_defs.bzl", "kt_jvm_export")

kt_jvm_export(
    name = "bsp4kt",
    srcs = glob(["src/**/*.kt"]),
    plugins = ["@com_github_jetbrains_kotlin//:kotlinx-serialization-compiler-plugin"],
    deps = [
        "@maven//:org_jetbrains_kotlinx_kotlinx_serialization_json",
        "@maven//:com_github_agluszak_jsonrpc4kt",
    ],
      maven_coordinates = "com.jetbrains.bsp:bsp4kt:0.1.0",
)
