load("//rules/kotlin:junit5.bzl", "kt_test")

kt_test(
    name = "DslTest",
    src = "DslTest.kt",
    classname = "DslTest",
    deps = [
        "//:codegen",
    ],
)

kt_test(
    name = "Bsp4rsTest",
    src = "Bsp4rsTest.kt",
    classname = "Bsp4rsTest",
    deps = [
        "//:codegen",
        "@maven//:software_amazon_smithy_smithy_model",
    ],
)
