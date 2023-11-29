load("@io_bazel_rules_kotlin//kotlin:kotlin.bzl", "kt_jvm_binary")


kt_jvm_binary(
    name = "kotlin_json_roundtrip",
    main_class = "org.jetbrains.bsp.tests.KotlinJsonRoundtripKt",
    visibility = ["//visibility:public"],
    srcs = glob(["src/main/kotlin/**/*.kt"]),
    deps = [
        "@maven//:com_google_code_gson_gson",
        "//bsp4kt"
    ],
)

load("//rules/kotlin:junit5.bzl", "kt_test")

kt_test(
    name = "serialization_test",
    src = "src/test/kotlin/SerializationTest.kt",
    classname = "SerializationTest",
    data = [
        "//tests:kotlin_json_roundtrip"
    ],
    deps = [
        "@bazel_tools//tools/java/runfiles:runfiles",
    ],
)
